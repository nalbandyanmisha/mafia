mod actions;
mod engine;
mod tui;

use actions::{Action, AppStatus};
use clap::Parser;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event as CEvent, KeyCode, KeyEvent},
    execute,
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};

use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{Write, stdout};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

struct AppState {
    timers: Vec<(u64, u64)>,
    engine: engine::Engine,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            timers: Vec::new(),
            engine: engine::Engine::new(),
        }
    }
}

enum Event {
    TimerTick { id: u64, remaining: u64 },
    Command(Action),
}

fn spawn_timer(id: u64, seconds: u64, tx: UnboundedSender<Event>, shutdown: Arc<AtomicBool>) {
    tokio::spawn(async move {
        let mut remaining = seconds;
        while remaining > 0 && !shutdown.load(Ordering::SeqCst) {
            tokio::time::sleep(Duration::from_secs(1)).await;
            remaining = remaining.saturating_sub(1);
            let _ = tx.send(Event::TimerTick { id, remaining });
        }
    });
}

fn spawn_input_thread(
    tx: UnboundedSender<Event>,
    input_buffer: Arc<Mutex<String>>,
    shutdown: Arc<AtomicBool>,
) {
    std::thread::spawn(move || {
        while !shutdown.load(Ordering::SeqCst) {
            if event::poll(Duration::from_millis(100)).unwrap_or(false) {
                if let Ok(CEvent::Key(KeyEvent { code, .. })) = event::read() {
                    match code {
                        KeyCode::Char(c) => {
                            let mut buf = input_buffer.lock().unwrap();
                            buf.push(c);
                        }
                        KeyCode::Backspace => {
                            let mut buf = input_buffer.lock().unwrap();
                            buf.pop();
                        }
                        KeyCode::Enter => {
                            let line = {
                                let mut buf = input_buffer.lock().unwrap();
                                let line = buf.trim().to_string();
                                buf.clear();
                                line
                            };
                            if line.is_empty() {
                                continue;
                            }

                            let mut clap_args = vec!["mafia"];
                            clap_args.extend(line.split_whitespace());

                            match Mafia::try_parse_from(clap_args) {
                                Ok(mafia) => {
                                    if let Some(action) = mafia.command {
                                        let _ = tx.send(Event::Command(action));
                                    }
                                }
                                Err(e) => eprintln!("{e}"),
                            }
                        }
                        KeyCode::Esc => {
                            let _ = tx.send(Event::Command(Action::Quit));
                        }
                        _ => {}
                    }
                }
            }
        }
    });
}

async fn render_loop(
    terminal: Arc<Mutex<Terminal<CrosstermBackend<std::io::Stdout>>>>,
    state: Arc<Mutex<AppState>>,
    input_buffer: Arc<Mutex<String>>,
    shutdown: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        if shutdown.load(Ordering::SeqCst) {
            break;
        }

        tokio::time::sleep(Duration::from_millis(50)).await;

        let mut terminal = terminal.lock().unwrap();
        let state = state.lock().unwrap();
        let input = input_buffer.lock().unwrap();

        terminal.draw(|f| {
            tui::tui(f, &state, &input);
        })?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Arc::new(Mutex::new(Terminal::new(backend)?));

    let (tx, mut rx): (UnboundedSender<Event>, UnboundedReceiver<Event>) = unbounded_channel();
    let state = Arc::new(Mutex::new(AppState::new()));
    let input_buffer = Arc::new(Mutex::new(String::new()));
    let shutdown = Arc::new(AtomicBool::new(false));

    spawn_input_thread(tx.clone(), Arc::clone(&input_buffer), Arc::clone(&shutdown));

    let state_clone = Arc::clone(&state);
    let input_clone = Arc::clone(&input_buffer);
    let shutdown_clone = Arc::clone(&shutdown);
    let terminal_clone = Arc::clone(&terminal);
    tokio::spawn(async move {
        render_loop(terminal_clone, state_clone, input_clone, shutdown_clone)
            .await
            .unwrap();
    });

    let mut timer_id: u64 = 0;

    while let Some(event) = rx.recv().await {
        match event {
            Event::Command(cmd) => {
                if let Action::Timer { seconds } = cmd {
                    timer_id += 1;
                    {
                        let mut s = state.lock().unwrap();
                        s.timers.push((timer_id, seconds as u64));
                    }
                    spawn_timer(timer_id, seconds as u64, tx.clone(), Arc::clone(&shutdown));
                } else {
                    let mut s = state.lock().unwrap();
                    let mut engine = std::mem::replace(&mut s.engine, engine::Engine::new());
                    drop(s);

                    let status = cmd.run(&mut engine).await.unwrap_or(AppStatus::Continue);

                    let mut s = state.lock().unwrap();
                    s.engine = engine;

                    if status == AppStatus::Quit {
                        shutdown.store(true, Ordering::SeqCst);
                        break;
                    }
                }
            }
            Event::TimerTick { id, remaining } => {
                let mut s = state.lock().unwrap();
                if let Some(t) = s.timers.iter_mut().find(|t| t.0 == id) {
                    t.1 = remaining;
                }
                s.timers.retain(|t| t.1 > 0);
            }
        }
    }

    shutdown.store(true, Ordering::SeqCst);

    disable_raw_mode()?;
    execute!(terminal.lock().unwrap().backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "mafia", version, about = "Host of Mafia game")]
struct Mafia {
    #[command(subcommand)]
    command: Option<Action>,
}

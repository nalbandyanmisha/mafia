mod commands;
use clap::Parser;
use commands::{Action, AppStatus};
use crossterm::{
    cursor::MoveTo,
    event::{self, Event as CEvent, KeyCode, KeyEvent},
    execute,
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use mafia::Table;
use std::io::{Write, stdout};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

#[derive(Default)]
struct AppState {
    timers: Vec<(u64, u64)>,
    table: Table,
}

enum Event {
    TimerTick { id: u64, remaining: u64 },
    Command(Action),
}

/// spawn_timer: non-async helper that spawns a background task which uses an UnboundedSender.
/// Uses synchronous tx.send(...) so it can be called from sync contexts without await.
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

/// Input thread: runs in a normal OS thread and sends events via UnboundedSender synchronously.
/// NOTE: this thread does NOT call enable_raw_mode()/disable_raw_mode() — main() does that.
fn spawn_input_thread(
    tx: UnboundedSender<Event>,
    input_buffer: Arc<Mutex<String>>,
    shutdown: Arc<AtomicBool>,
) {
    std::thread::spawn(move || {
        // We do not call enable_raw_mode() here; main() enables raw mode.
        // loop until shutdown
        while !shutdown.load(Ordering::SeqCst) {
            // poll with timeout so we can react to shutdown flag
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
                            // capture and clear buffer
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
                                        // send synchronously - UnboundedSender::send is not async
                                        let _ = tx.send(Event::Command(action));
                                    }
                                }
                                Err(e) => {
                                    // print parse error; render loop will redraw soon
                                    eprintln!("{e}");
                                }
                            }
                        }
                        KeyCode::Esc => {
                            // send Quit synchronously
                            let _ = tx.send(Event::Command(Action::Quit));
                        }
                        _ => {}
                    }
                }
            }
            // timed out -> loop again and check shutdown
        }

        // nothing to do here for raw mode disable; main will cleanup raw mode.
    });
}

/// Render loop: redraws only the area above the input row and reprints the input line contents.
/// It never overwrites the input row.
async fn render_loop(
    state: Arc<Mutex<AppState>>,
    input_buffer: Arc<Mutex<String>>,
    shutdown: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();

    // compute input row as bottom row of terminal (0-based)
    let (cols, rows) = terminal::size()?;
    // reserve last row (rows-1) for input prompt
    let input_row: u16 = rows.saturating_sub(1);

    loop {
        if shutdown.load(Ordering::SeqCst) {
            break;
        }

        tokio::time::sleep(Duration::from_millis(10)).await;

        // snapshot state
        let s = state.lock().unwrap();

        // build lines to print for the "game view"
        let mut lines: Vec<String> = Vec::new();

        lines.push("Timers:".to_string());
        for (id, remaining) in &s.timers {
            lines.push(format!("  #{id:>2}: {remaining}s remaining"));
        }
        lines.push("".to_string());

        let table_view = s.table.draw().clone();
        for row in table_view {
            lines.push(row.clone());
        }
        drop(s); // release lock

        // write each line to its row; do not touch the input_row
        for (i, line) in lines.iter().enumerate() {
            let row = i as u16;
            if row >= input_row {
                // don't overwrite input row or go past it
                break;
            }
            execute!(stdout, MoveTo(0, row), Clear(ClearType::CurrentLine))?;
            // If too long for terminal width, truncate
            let mut out = line.clone();
            if out.len() as u16 > cols {
                out.truncate(cols as usize);
            }
            write!(stdout, "{out}")?;
        }

        // clear any leftover rows from used_lines..input_row-1
        let used = lines.len() as u16;
        for row in used..input_row {
            execute!(stdout, MoveTo(0, row), Clear(ClearType::CurrentLine))?;
        }

        // now re-print the prompt at input_row with current input buffer content
        let buf = input_buffer.lock().unwrap().clone();
        execute!(stdout, MoveTo(0, input_row), Clear(ClearType::CurrentLine))?;
        // ensure prompt fits columns
        let prompt = format!("> {buf}");
        let mut out_prompt = prompt.clone();
        if out_prompt.len() as u16 > cols {
            out_prompt.truncate(cols as usize);
        }
        write!(stdout, "{out_prompt}")?;
        stdout.flush()?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // enter alternate screen and enable raw mode
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    // use an unbounded channel so the blocking input thread can call send synchronously
    let (tx, mut rx): (UnboundedSender<Event>, UnboundedReceiver<Event>) = unbounded_channel();
    let state = Arc::new(Mutex::new(AppState::default()));
    let input_buffer = Arc::new(Mutex::new(String::new()));
    let shutdown = Arc::new(AtomicBool::new(false));

    // spawn input thread (blocking OS thread)
    {
        let tx_clone = tx.clone();
        let input_clone = Arc::clone(&input_buffer);
        let shutdown_clone = Arc::clone(&shutdown);
        spawn_input_thread(tx_clone, input_clone, shutdown_clone);
    }

    // spawn render loop
    {
        let state_clone = Arc::clone(&state);
        let input_clone = Arc::clone(&input_buffer);
        let shutdown_clone = Arc::clone(&shutdown);
        tokio::spawn(async move {
            if let Err(e) = render_loop(state_clone, input_clone, shutdown_clone).await {
                eprintln!("render loop error: {e}");
            }
        });
    }

    // main event consumer
    let mut timer_id: u64 = 0;

    while let Some(event) = rx.recv().await {
        match event {
            Event::Command(cmd) => {
                match cmd {
                    Action::Timer { seconds } => {
                        // start timer
                        timer_id += 1;
                        let secs_u64 = seconds as u64;

                        // push to state
                        {
                            let mut s = state.lock().unwrap();
                            s.timers.push((timer_id, secs_u64));
                        }

                        // spawn background timer task which sends TimerTick events
                        let tx_clone = tx.clone();
                        let shutdown_clone = Arc::clone(&shutdown);
                        spawn_timer(timer_id, secs_u64, tx_clone, shutdown_clone);
                    }

                    other_cmd => {
                        // Apply command directly to the Table inside AppState

                        // Take table out to avoid holding lock across await
                        let mut s = state.lock().unwrap();
                        let mut table = std::mem::take(&mut s.table);
                        drop(s);

                        let status = other_cmd.run(&mut table).await?;

                        // put table back
                        {
                            let mut s = state.lock().unwrap();
                            s.table = table;
                        }

                        if status == AppStatus::Quit {
                            // set shutdown flag and break loop to begin graceful shutdown
                            shutdown.store(true, Ordering::SeqCst);
                            break;
                        }
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

    // cleanup: ensure raw mode disabled and leave alternate screen
    shutdown.store(true, Ordering::SeqCst);
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}

/// Local clap wrapper used by input thread (same as before)
#[derive(Parser, Debug)]
#[command(name = "mafia", version, about = "Host of Mafia game", long_about = None)]
struct Mafia {
    #[command(subcommand)]
    command: Option<Action>,
}

impl Mafia {
    #[allow(dead_code)]
    pub async fn run(&self, table: &mut Table) -> Result<AppStatus, Box<dyn std::error::Error>> {
        if let Some(command) = &self.command {
            command.run(table).await
        } else {
            println!("Welcome to Mafia, type the command to move forward");
            Ok(AppStatus::Continue)
        }
    }
}


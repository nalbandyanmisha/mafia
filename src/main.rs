mod app;
mod engine;
mod snapshot;
mod tui;

use app::{App, AppStatus, events::Event as AppEvent};
use snapshot::Snapshot;
use tui::draw_ui;

use ratatui::crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;

    let (event_tx, mut event_rx) = mpsc::channel::<AppEvent>(32);
    let mut app = App::new(event_tx.clone());

    // Input task
    let input_tx = event_tx.clone();
    tokio::spawn(async move {
        loop {
            if event::poll(Duration::from_millis(50)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    match key.code {
                        KeyCode::Char(c) => {
                            let _ = input_tx.send(AppEvent::InputChar(c)).await;
                        }
                        KeyCode::Backspace => {
                            let _ = input_tx.send(AppEvent::InputBackspace).await;
                        }
                        KeyCode::Enter => {
                            let _ = input_tx.send(AppEvent::InputEnter).await;
                        }
                        KeyCode::Esc => {
                            let _ = input_tx.send(AppEvent::QuitRequested).await;
                        }
                        _ => {}
                    }
                }
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    });

    let tick_interval = tokio::time::interval(Duration::from_millis(50));

    // Main loop
    tokio::pin!(tick_interval);
    while app.status == AppStatus::Running {
        tokio::select! {
            Some(event) = event_rx.recv() => {
                match event {
                    AppEvent::QuitRequested => app.status = AppStatus::Quit,
                    AppEvent::InputEnter => app.parse_input().await,
                    AppEvent::InputChar(c) => app.input.push(c),
                    AppEvent::InputBackspace => { app.input.pop(); },
                    AppEvent::EngineUpdated => {},
                    AppEvent::TimerStarted(s) => app.current_timer = Some(s),
                    AppEvent::TimerTick(s) => app.current_timer = Some(s),
                    AppEvent::TimerEnded => app.current_timer = None,
                    AppEvent::Error(_) => {}, // handle if needed
                }
            }
            _ = tick_interval.tick() => {
                terminal.draw(|f| draw_ui(f, &app.snapshot())).unwrap();
            }
        }
    }

    tui::restore_terminal()?;
    Ok(())
}

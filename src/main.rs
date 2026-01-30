mod app;
mod domain;
mod engine;
mod snapshot;
mod storage;
mod tui;

use app::{App, AppStatus, events::Event as AppEvent};
use snapshot::Snapshot;

use ratatui::crossterm::event::{self, Event};
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
                    let _ = input_tx.send(AppEvent::Key(key)).await;
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
                            AppEvent::Key(key) => {
            app.handle_key(key).await;
        }
                            AppEvent::Engine(event) => {
                                app.events.push(AppEvent::Engine(event));
                                if app.events.len() > 100 {
                                    app.events.remove(0);
                                }
                            }
                            AppEvent::End => {},
                            AppEvent::TimerStarted(s) => app.current_timer = Some(s),
                            AppEvent::TimerTick(s) => app.current_timer = Some(s),
                            AppEvent::TimerEnded => app.current_timer = None,
                            AppEvent::Error(_) => {}, // handle if needed
                        }
                    }
                    _ = tick_interval.tick() => {
                        terminal.draw(|f| tui::draw(f, &app.snapshot())).unwrap();
                    }
                }
    }

    tui::restore_terminal()?;
    Ok(())
}

pub mod commands;
pub mod events;
pub mod parser;

use crate::app::{commands::Command as AppCommand, events::Event as AppEvent, parser::parse_input};
use crate::engine::{Engine, commands::Command as EngineCommand};
use clap::Parser;
use tokio::sync::mpsc;

#[derive(PartialEq)]
pub enum AppStatus {
    Running,
    Quit,
}

pub struct App {
    pub engine: Engine,
    pub status: AppStatus,
    pub input: String,

    pub current_timer: Option<u64>, // <- NEW: store timer
    pub event_tx: mpsc::Sender<AppEvent>,
}

impl App {
    pub fn new(event_tx: mpsc::Sender<AppEvent>) -> Self {
        App {
            engine: Engine::new(),
            status: AppStatus::Running,
            input: String::new(),
            current_timer: None,
            event_tx,
        }
    }

    pub async fn handle_command(&mut self, cmd: AppCommand) {
        use AppCommand::*;

        match cmd {
            Quit => {
                let _ = self.event_tx.send(AppEvent::QuitRequested).await;
                self.status = AppStatus::Quit;
            }
            Timer { seconds } => {
                let tx = self.event_tx.clone();
                tokio::spawn(async move {
                    let _ = tx.send(AppEvent::TimerStarted(seconds)).await;
                    for remaining in (0..seconds).rev() {
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                        let _ = tx.send(AppEvent::TimerTick(remaining)).await;
                    }
                    let _ = tx.send(AppEvent::TimerEnded).await;
                });
            }
            Join { name } => {
                let _ = self.engine.apply(EngineCommand::Join { name });
                let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
            }
            Leave { name } => {
                let _ = self.engine.apply(EngineCommand::Leave { name });
                let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
            }
            Warn { position } => {
                if let Ok(chair) = self.engine.chair_from_position(position) {
                    let _ = self.engine.apply(EngineCommand::Warn { chair });
                    let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
                }
            }
            Pardon { position } => {
                if let Ok(chair) = self.engine.chair_from_position(position) {
                    let _ = self.engine.apply(EngineCommand::Pardon { chair });
                    let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
                }
            }
            Nominate { position } => {
                if let Ok(target) = self.engine.chair_from_position(position) {
                    let _ = self.engine.apply(EngineCommand::Nominate { target });
                    let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
                }
            }
            Shoot { position } => {
                if let Ok(chair) = self.engine.chair_from_position(position) {
                    let _ = self.engine.apply(EngineCommand::Shoot { chair });
                    let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
                }
            }
            Next => {
                let _ = self.engine.apply(EngineCommand::NextPhase);
                let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
            }
            NextSpeaker => {
                let _ = self.engine.apply(EngineCommand::NextSpeaker);
                let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
            }
        }
    }

    pub async fn parse_input(&mut self) {
        let input = self.input.trim();
        if input.is_empty() {
            return;
        }

        let argv = std::iter::once("mafia").chain(input.split_whitespace());

        match AppCommand::try_parse_from(argv) {
            Ok(cmd) => {
                self.handle_command(cmd).await;
            }
            Err(err) => {
                let _ = self
                    .event_tx
                    .send(AppEvent::Error(format!("{}", err)))
                    .await;
            }
        }

        self.input.clear();
    }
}

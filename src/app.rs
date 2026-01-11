pub mod commands;
pub mod events;
pub mod parser;

use crate::app::{commands::Command as AppCommand, events::Event as AppEvent};
use crate::engine::{Engine, commands::Command as EngineCommand};
use crate::snapshot::{self, Snapshot};
use clap::Parser;
use tokio::sync::mpsc;

#[derive(PartialEq, Clone)]
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

impl Snapshot for App {
    type Output = snapshot::App;

    fn snapshot(&self) -> Self::Output {
        snapshot::App {
            engine: self.engine.snapshot(),
            input: self.input.clone(),
            current_timer: self.current_timer,
        }
    }
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

            Start => {
                let _ = self.engine.apply(EngineCommand::Start);
                let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
            }

            Next => {
                let _ = self.engine.apply(EngineCommand::Advance);
                let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
            }

            AssignRole => {
                let _ = self.engine.apply(EngineCommand::AssignRole);
                let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
            }
            RevokeRole => {
                let _ = self.engine.apply(EngineCommand::RevokeRole);
                let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
            }
            Warn { position } => {
                let _ = self.engine.apply(EngineCommand::Warn {
                    target: position.into(),
                });
                let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
            }
            Pardon { position } => {
                let _ = self.engine.apply(EngineCommand::Pardon {
                    target: position.into(),
                });
                let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
            }
            Nominate { position } => {
                let _ = self.engine.apply(EngineCommand::Nominate {
                    target: position.into(),
                });
                let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
            }
            Vote { positions } => {
                let mut targets = Vec::new();
                for pos in positions {
                    targets.push(pos.into());
                }
                let _ = self.engine.apply(EngineCommand::Vote { targets });
                let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
            }
            Shoot { position } => {
                let _ = self.engine.apply(EngineCommand::Shoot {
                    target: position.into(),
                });
                let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
            }
            Check { position } => {
                let _ = self.engine.apply(EngineCommand::Check {
                    target: position.into(),
                });
                let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
            }
            Guess { targets } => {
                let mut positions = Vec::new();
                for target in targets {
                    positions.push(target.into());
                }
                let _ = self
                    .engine
                    .apply(EngineCommand::Vote { targets: positions });
                let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
            }
            Assign { command } => {
                let command = command.unwrap_or(commands::AssignCommand::Role { role: None });

                match command {
                    commands::AssignCommand::Player { name } => {
                        let _ = self.engine.apply(EngineCommand::Join { name });
                        let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
                    }
                    commands::AssignCommand::Role { role } => {
                        let _ = self.engine.apply(EngineCommand::AssignRole);
                        let _ = self.event_tx.send(AppEvent::EngineUpdated).await;
                    }
                }
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
                let _ = self.event_tx.send(AppEvent::Error(format!("{err}"))).await;
            }
        }

        self.input.clear();
    }
}

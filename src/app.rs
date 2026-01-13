pub mod commands;
pub mod events;
pub mod parser;

use crate::app::{commands::Command as AppCommand, events::Event as AppEvent};
use crate::engine::{Engine, commands::Command as EngineCommand};
use crate::snapshot::{self, Snapshot};
use clap::Parser;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

#[derive(PartialEq, Clone)]
pub enum AppStatus {
    Running,
    Quit,
}

pub struct App {
    pub engine: Engine,
    pub status: AppStatus,
    pub input: String,

    pub events: Vec<AppEvent>,
    pub current_timer: Option<u64>, // <- NEW: store timer
    pub event_tx: mpsc::Sender<AppEvent>,
    pub timer_task: Option<JoinHandle<()>>,
}

impl Snapshot for App {
    type Output = snapshot::App;

    fn snapshot(&self) -> Self::Output {
        snapshot::App {
            engine: self.engine.snapshot(),
            input: self.input.clone(),
            current_timer: self.current_timer,
            events: self.events.clone(),
        }
    }
}

impl App {
    pub fn new(event_tx: mpsc::Sender<AppEvent>) -> Self {
        App {
            engine: Engine::new(),
            status: AppStatus::Running,
            input: String::new(),
            events: Vec::new(),
            current_timer: None,
            event_tx,
            timer_task: None,
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
                if let Some(task) = self.timer_task.take() {
                    task.abort();
                }
                let tx = self.event_tx.clone();
                let handle = tokio::spawn(async move {
                    let _ = tx.send(AppEvent::TimerStarted(seconds)).await;
                    for remaining in (0..seconds).rev() {
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                        let _ = tx.send(AppEvent::TimerTick(remaining)).await;
                    }
                    let _ = tx.send(AppEvent::TimerEnded).await;
                });

                self.timer_task = Some(handle);
            }
            Join { name } => match self.engine.apply(EngineCommand::Join { name }) {
                Ok(events) => {
                    for event in events {
                        let _ = self.event_tx.send(AppEvent::Engine(event)).await;
                    }
                }
                Err(err) => {
                    let _ = self.event_tx.send(AppEvent::Error(err.to_string())).await;
                }
            },
            Leave { name } => match self.engine.apply(EngineCommand::Leave { name }) {
                Ok(events) => {
                    for event in events {
                        let _ = self.event_tx.send(AppEvent::Engine(event)).await;
                    }
                }
                Err(err) => {
                    let _ = self.event_tx.send(AppEvent::Error(err.to_string())).await;
                }
            },

            Start => match self.engine.apply(EngineCommand::Start) {
                Ok(events) => {
                    for event in events {
                        let _ = self.event_tx.send(AppEvent::Engine(event)).await;
                    }
                }
                Err(err) => {
                    let _ = self.event_tx.send(AppEvent::Error(err.to_string())).await;
                }
            },

            Next => match self.engine.apply(EngineCommand::Advance) {
                Ok(events) => {
                    for event in events {
                        let _ = self.event_tx.send(AppEvent::Engine(event)).await;
                    }
                }
                Err(err) => {
                    let _ = self.event_tx.send(AppEvent::Error(err.to_string())).await;
                }
            },

            AssignRole => match self.engine.apply(EngineCommand::AssignRole) {
                Ok(events) => {
                    for event in events {
                        let _ = self.event_tx.send(AppEvent::Engine(event)).await;
                    }
                }
                Err(err) => {
                    let _ = self.event_tx.send(AppEvent::Error(err.to_string())).await;
                }
            },

            RevokeRole => match self.engine.apply(EngineCommand::RevokeRole) {
                Ok(events) => {
                    for event in events {
                        let _ = self.event_tx.send(AppEvent::Engine(event)).await;
                    }
                }
                Err(err) => {
                    let _ = self.event_tx.send(AppEvent::Error(err.to_string())).await;
                }
            },

            Warn { position } => match self.engine.apply(EngineCommand::Warn {
                target: position.into(),
            }) {
                Ok(events) => {
                    for event in events {
                        let _ = self.event_tx.send(AppEvent::Engine(event)).await;
                    }
                }
                Err(err) => {
                    let _ = self.event_tx.send(AppEvent::Error(err.to_string())).await;
                }
            },

            Pardon { position } => match self.engine.apply(EngineCommand::Pardon {
                target: position.into(),
            }) {
                Ok(events) => {
                    for event in events {
                        let _ = self.event_tx.send(AppEvent::Engine(event)).await;
                    }
                }
                Err(err) => {
                    let _ = self.event_tx.send(AppEvent::Error(err.to_string())).await;
                }
            },

            Nominate { position } => match self.engine.apply(EngineCommand::Nominate {
                target: position.into(),
            }) {
                Ok(events) => {
                    for event in events {
                        let _ = self.event_tx.send(AppEvent::Engine(event)).await;
                    }
                }
                Err(err) => {
                    let _ = self.event_tx.send(AppEvent::Error(err.to_string())).await;
                }
            },

            Vote { positions } => {
                let mut targets = Vec::new();
                for pos in positions {
                    targets.push(pos.into());
                }

                match self.engine.apply(EngineCommand::Vote { targets }) {
                    Ok(events) => {
                        for event in events {
                            let _ = self.event_tx.send(AppEvent::Engine(event)).await;
                        }
                    }
                    Err(err) => {
                        let _ = self.event_tx.send(AppEvent::Error(err.to_string())).await;
                    }
                }
            }

            Shoot { position } => match self.engine.apply(EngineCommand::Shoot {
                target: position.into(),
            }) {
                Ok(events) => {
                    for event in events {
                        let _ = self.event_tx.send(AppEvent::Engine(event)).await;
                    }
                }
                Err(err) => {
                    let _ = self.event_tx.send(AppEvent::Error(err.to_string())).await;
                }
            },

            Check { position } => match self.engine.apply(EngineCommand::Check {
                target: position.into(),
            }) {
                Ok(events) => {
                    for event in events {
                        let _ = self.event_tx.send(AppEvent::Engine(event)).await;
                    }
                }
                Err(err) => {
                    let _ = self.event_tx.send(AppEvent::Error(err.to_string())).await;
                }
            },

            Guess { targets } => {
                let mut positions = Vec::new();
                for target in targets {
                    positions.push(target.into());
                }

                match self
                    .engine
                    .apply(EngineCommand::Vote { targets: positions })
                {
                    Ok(events) => {
                        for event in events {
                            let _ = self.event_tx.send(AppEvent::Engine(event)).await;
                        }
                    }
                    Err(err) => {
                        let _ = self.event_tx.send(AppEvent::Error(err.to_string())).await;
                    }
                }
            }

            Assign { command } => {
                let command = command.unwrap_or(commands::AssignCommand::Role { role: None });

                let result = match command {
                    commands::AssignCommand::Player { name } => {
                        self.engine.apply(EngineCommand::Join { name })
                    }
                    commands::AssignCommand::Role { role } => {
                        if role.is_none() {
                            self.engine.apply(EngineCommand::AssignRole)
                        } else {
                            Ok(vec![])
                        }
                    }
                };

                match result {
                    Ok(events) => {
                        for event in events {
                            let _ = self.event_tx.send(AppEvent::Engine(event)).await;
                        }
                    }
                    Err(err) => {
                        let _ = self.event_tx.send(AppEvent::Error(err.to_string())).await;
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

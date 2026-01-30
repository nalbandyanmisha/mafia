pub mod commands;
pub mod events;
pub mod input;
pub mod parser;

use crate::app::{commands::Command as AppCommand, events::Event as AppEvent};
use crate::engine::{Engine, commands::Command as EngineCommand};
use crate::snapshot::{self, Snapshot};
use crate::storage::timestamped_save_path;
use clap::Parser;
use input::{InputMode, PopupKind};
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use std::fs::File;
use std::io::Write;
use std::path::Path;
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
    pub input_mode: InputMode,

    pub events: Vec<AppEvent>,
    pub current_timer: Option<u64>,
    pub event_tx: mpsc::Sender<AppEvent>,
    pub timer_task: Option<JoinHandle<()>>,
}

impl Snapshot for App {
    type Output = snapshot::App;

    fn snapshot(&self) -> Self::Output {
        snapshot::App {
            engine: self.engine.snapshot(),
            input: self.input.clone(),
            input_mode: self.input_mode.clone(),
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
            input_mode: InputMode::Normal,
            events: Vec::new(),
            current_timer: None,
            event_tx,
            timer_task: None,
        }
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let snapshot = self.snapshot();

        let json = serde_json::to_string_pretty(&snapshot.engine.game)?;

        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    pub async fn handle_key(&mut self, key: KeyEvent) {
        match self.input_mode {
            InputMode::Normal => self.handle_normal_mode(key).await,
            InputMode::Command => self.handle_command_mode(key).await,
            InputMode::Popup { .. } => self.handle_popup_mode(key).await,
        }
    }

    async fn handle_popup_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.input.clear();
            }

            KeyCode::Enter => {
                let value = self.input.trim().to_string();

                if let InputMode::Popup { kind, .. } = self.input_mode.clone() {
                    self.execute_popup(kind, value).await;
                }

                self.input.clear();
                self.input_mode = InputMode::Normal;
            }

            KeyCode::Backspace => {
                self.input.pop();
            }

            KeyCode::Char(c) => {
                self.input.push(c);
            }

            _ => {}
        }
    }

    async fn execute_popup(&mut self, kind: PopupKind, value: String) {
        use PopupKind::*;

        match kind {
            Join => {
                if !value.is_empty() {
                    self.handle_command(AppCommand::Join { name: value }).await;
                }
            }

            Leave => {
                if !value.is_empty() {
                    self.handle_command(AppCommand::Leave { name: value }).await;
                }
            }

            Nominate => {
                if let Ok(pos) = value.parse::<u8>() {
                    self.handle_command(AppCommand::Nominate { position: pos })
                        .await;
                }
            }

            Shoot => {
                if let Ok(pos) = value.parse::<u8>() {
                    self.handle_command(AppCommand::Shoot { position: pos })
                        .await;
                }
            }

            Check => {
                if let Ok(pos) = value.parse::<u8>() {
                    self.handle_command(AppCommand::Check { position: pos })
                        .await;
                }
            }

            Warn => {
                if let Ok(pos) = value.parse::<u8>() {
                    self.handle_command(AppCommand::Warn { position: pos })
                        .await;
                }
            }

            Pardon => {
                if let Ok(pos) = value.parse::<u8>() {
                    self.handle_command(AppCommand::Pardon { position: pos })
                        .await;
                }
            }

            Guess => {
                let positions: Vec<u8> = value
                    .split_whitespace()
                    .filter_map(|s| s.parse::<u8>().ok())
                    .collect();

                if !positions.is_empty() {
                    self.handle_command(AppCommand::Guess { targets: positions })
                        .await;
                }
            }

            Vote => {
                let positions: Vec<u8> = value
                    .split_whitespace()
                    .filter_map(|s| s.parse::<u8>().ok())
                    .collect();

                if !positions.is_empty() {
                    self.handle_command(AppCommand::Vote { positions }).await;
                }
            }
        }
    }

    async fn handle_normal_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(':') => {
                self.input_mode = InputMode::Command;
                self.input.clear();
            }

            KeyCode::Char('j') => {
                self.input_mode = InputMode::Popup {
                    title: "Enter player name".to_string(),
                    kind: PopupKind::Join,
                };
                self.input.clear();
            }

            KeyCode::Char('l') => {
                self.input_mode = InputMode::Popup {
                    title: "Enter player name".to_string(),
                    kind: PopupKind::Leave,
                };
                self.input.clear();
            }

            KeyCode::Char('n') => {
                self.handle_command(AppCommand::Next).await;
            }

            KeyCode::Char('b') => {
                self.handle_command(AppCommand::Start).await;
            }

            KeyCode::Char('w') => {
                self.input_mode = InputMode::Popup {
                    title: "Enter player position to warn".to_string(),
                    kind: PopupKind::Warn,
                };
                self.input.clear();
            }

            KeyCode::Char('p') => {
                self.input_mode = InputMode::Popup {
                    title: "Enter player position to pardon".to_string(),
                    kind: PopupKind::Pardon,
                };
                self.input.clear();
            }

            KeyCode::Char('o') => {
                self.input_mode = InputMode::Popup {
                    title: "Enter player position to record nomination".to_string(),
                    kind: PopupKind::Nominate,
                };
                self.input.clear();
            }

            KeyCode::Char('c') => {
                self.input_mode = InputMode::Popup {
                    title: "Enter player position to perform check".to_string(),
                    kind: PopupKind::Check,
                };
                self.input.clear();
            }

            KeyCode::Char('g') => {
                self.input_mode = InputMode::Popup {
                    title: "Enter player positions to record guess".to_string(),
                    kind: PopupKind::Guess,
                };
                self.input.clear();
            }

            KeyCode::Char('v') => {
                self.input_mode = InputMode::Popup {
                    title: "Enter player positions to record votes".to_string(),
                    kind: PopupKind::Vote,
                };
                self.input.clear();
            }

            KeyCode::Char('s') => {
                self.input_mode = InputMode::Popup {
                    title: "Enter player position to record shoot".to_string(),
                    kind: PopupKind::Shoot,
                };
                self.input.clear();
            }

            KeyCode::Esc => {
                self.status = AppStatus::Quit;
            }

            _ => {}
        }
    }

    async fn handle_command_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.input.clear();
            }

            KeyCode::Enter => {
                self.parse_input().await;
                self.input_mode = InputMode::Normal;
            }

            KeyCode::Backspace => {
                self.input.pop();
            }

            KeyCode::Char(c) => {
                self.input.push(c);
            }

            _ => {}
        }
    }

    pub async fn handle_command(&mut self, cmd: AppCommand) {
        use AppCommand::*;

        match cmd {
            Quit => {
                let _ = self.event_tx.send(AppEvent::QuitRequested).await;
                self.status = AppStatus::Quit;
            }
            End { file_name } => match self.save_to_file(file_name) {
                Ok(_) => {
                    let _ = self.event_tx.send(AppEvent::End).await;
                }
                Err(err) => {
                    let _ = self.event_tx.send(AppEvent::Error(err.to_string())).await;
                }
            },
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
            Join { name } => {
                let result = self.engine.apply(EngineCommand::Join { name });
                self.handle_engine_result(result).await;
            }
            Leave { name } => {
                let result = self.engine.apply(EngineCommand::Leave { name });
                self.handle_engine_result(result).await;
            }

            Start => {
                let result = self.engine.apply(EngineCommand::Start);
                self.handle_engine_result(result).await;
            }

            Next => {
                let result = self.engine.apply(EngineCommand::Advance);
                self.handle_engine_result(result).await;
            }

            AssignRole => {
                let results = self.engine.apply(EngineCommand::AssignRole);
                self.handle_engine_result(results).await;
            }

            RevokeRole => {
                let results = self.engine.apply(EngineCommand::RevokeRole);
                self.handle_engine_result(results).await;
            }

            Warn { position } => {
                let results = self.engine.apply(EngineCommand::Warn {
                    target: position.into(),
                });
                self.handle_engine_result(results).await;
            }

            Pardon { position } => {
                let results = self.engine.apply(EngineCommand::Pardon {
                    target: position.into(),
                });
                self.handle_engine_result(results).await;
            }

            Nominate { position } => {
                let results = self.engine.apply(EngineCommand::Nominate {
                    target: position.into(),
                });
                self.handle_engine_result(results).await;
            }

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

            Shoot { position } => {
                let results = self.engine.apply(EngineCommand::Shoot {
                    target: position.into(),
                });
                self.handle_engine_result(results).await;
            }

            Check { position } => {
                let results = self.engine.apply(EngineCommand::Check {
                    target: position.into(),
                });
                self.handle_engine_result(results).await;
            }

            Guess { targets } => {
                let mut positions = Vec::new();
                for target in targets {
                    positions.push(target.into());
                }

                match self
                    .engine
                    .apply(EngineCommand::Guess { targets: positions })
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

    async fn start_timer(&mut self, seconds: u64) {
        // stop previous timer if any
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

        self.current_timer = Some(seconds);
        self.timer_task = Some(handle);
    }

    async fn handle_engine_result(&mut self, result: anyhow::Result<Vec<crate::engine::Event>>) {
        match result {
            Ok(events) => {
                for event in events {
                    // forward event
                    let _ = self.event_tx.send(AppEvent::Engine(event.clone())).await;

                    // AUTO TIMER HOOK
                    if let crate::engine::Event::ActorAdvanced { .. } = event {
                        if let Some(seconds) = timer_for_engine(&self.engine) {
                            self.start_timer(seconds).await;
                        }
                    }

                    if let crate::engine::Event::GameEnded = event {
                        // stop any running timer
                        if let Some(task) = self.timer_task.take() {
                            task.abort();
                        }
                        self.current_timer = None;
                        let path = timestamped_save_path();

                        match self.save_to_file(&path) {
                            Ok(_) => {
                                let _ = self.event_tx.send(AppEvent::End).await;
                            }
                            Err(err) => {
                                let _ = self
                                    .event_tx
                                    .send(AppEvent::Error(format!(
                                        "Failed to save game to {path:?}: {err}"
                                    )))
                                    .await;
                            }
                        }
                        self.engine = Engine::new();
                    }
                }
            }
            Err(err) => {
                let _ = self.event_tx.send(AppEvent::Error(err.to_string())).await;
            }
        }
    }
}

fn timer_for_engine(engine: &Engine) -> Option<u64> {
    use crate::domain::EngineState::Game;
    use crate::domain::{
        Activity::*, EveningActivity::*, MorningActivity::*, NightActivity::*, NoonActivity::*,
    };

    match engine.state {
        Game(Night(RoleAssignment)) => None,
        Game(Night(SheriffReveal)) => Some(5),
        Game(Night(DonReveal)) => Some(5),
        Game(Night(MafiaBriefing)) => Some(60),
        Game(Night(MafiaShooting)) => None,
        Game(Night(SheriffCheck)) => Some(10),
        Game(Night(DonCheck)) => Some(10),

        Game(Morning(Guessing)) => Some(15),
        Game(Morning(DeathSpeech)) => Some(60),

        Game(Noon(Discussion)) => Some(60),

        Game(Evening(Voting)) => Some(2),
        Game(Evening(TieDiscussion)) => Some(30),
        Game(Evening(TieVoting)) => Some(2),
        Game(Evening(FinalVoting)) => Some(2),
        Game(Evening(FinalSpeech)) => Some(60),

        _ => None,
    }
}

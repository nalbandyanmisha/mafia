use crate::engine::{Engine, commands::Command, state::phase::Phase};
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Action {
    Join { name: String },
    Leave { name: String },
    Warn { position: u8 },
    Pardon { position: u8 },
    Nominate { position: u8 },
    NextSpeaker,
    Timer { seconds: u8 },
    Shoot { position: u8 },
    Next,
    Show,
    Quit,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppStatus {
    Continue,
    Quit,
}

impl Action {
    pub async fn run(&self, engine: &mut Engine) -> Result<AppStatus, Box<dyn std::error::Error>> {
        match (self, &engine.state.phase) {
            (Action::Join { name }, Phase::Lobby) => {
                engine.apply(Command::Join { name: name.clone() })?;
                Ok(AppStatus::Continue)
            }
            (Action::Leave { name }, Phase::Lobby) => {
                engine.apply(Command::Leave { name: name.clone() })?;
                println!("Player {name} left the game.");
                Ok(AppStatus::Continue)
            }
            (Action::Warn { position }, _) => {
                let chair = engine
                    .state
                    .table
                    .chair(*position)
                    .map_err(|e| format!("Invalid chair: {e}"))?;
                engine.apply(Command::Warn { chair })?;
                Ok(AppStatus::Continue)
            }
            (Action::Pardon { position }, _) => {
                let chair = engine
                    .state
                    .table
                    .chair(*position)
                    .map_err(|e| format!("Invalid chair: {e}"))?;
                engine.apply(Command::Pardon { chair })?;
                Ok(AppStatus::Continue)
            }
            (Action::Shoot { position }, Phase::Night) => {
                let chair = engine
                    .state
                    .table
                    .chair(*position)
                    .map_err(|e| format!("Invalid chair: {e}"))?;
                engine.apply(Command::Shoot { chair })?;
                Ok(AppStatus::Continue)
            }
            (Action::Nominate { position }, Phase::Day) => {
                let target = engine
                    .state
                    .table
                    .chair(*position)
                    .map_err(|e| format!("Invalid chair: {e}"))?;
                engine.apply(Command::Nominate { target })?;
                Ok(AppStatus::Continue)
            }
            (Action::NextSpeaker, Phase::Day) => {
                engine.apply(Command::NextSpeaker)?;
                Ok(AppStatus::Continue)
            }
            (Action::Next, _) => {
                engine.apply(Command::NextPhase)?;
                Ok(AppStatus::Continue)
            }
            (Action::Show, _) => {
                todo!("Implement private role display logic");
            }
            (Action::Quit, _) => {
                println!("Exiting the game.");
                Ok(AppStatus::Quit)
            }
            (_, _) => Ok(AppStatus::Continue),
        }
    }
}


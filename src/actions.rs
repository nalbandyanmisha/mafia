use crate::engine::{
    Engine,
    commands::Command,
    state::{chair::Chair, phase::Phase},
};
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
                engine.apply(Command::Warn {
                    chair: Chair::new(*position),
                })?;
                Ok(AppStatus::Continue)
            }
            (Action::Pardon { position }, _) => {
                engine.apply(Command::Pardon {
                    chair: Chair::new(*position),
                })?;
                Ok(AppStatus::Continue)
            }
            (Action::Shoot { position }, Phase::Night) => {
                engine.apply(Command::Shoot {
                    chair: Chair::new(*position),
                })?;
                Ok(AppStatus::Continue)
            }
            (Action::Nominate { position }, Phase::Day) => {
                engine.apply(Command::Nominate {
                    target: Chair::new(*position),
                })?;
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

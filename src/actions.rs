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

impl Action {
    pub fn run(&self, engine: &mut Engine) -> Result<(), anyhow::Error> {
        let state = engine.view();
        match (self, state.phase) {
            (Action::Join { name }, Phase::Lobby) => {
                engine.apply(Command::Join { name: name.clone() })?;
                Ok(())
            }
            (Action::Leave { name }, Phase::Lobby) => {
                engine.apply(Command::Leave { name: name.clone() })?;
                Ok(())
            }
            (Action::Warn { position }, _) => {
                let chair = engine.chair_from_position(*position)?;
                engine.apply(Command::Warn { chair })?;
                Ok(())
            }
            (Action::Pardon { position }, _) => {
                let chair = engine.chair_from_position(*position)?;
                engine.apply(Command::Pardon { chair })?;
                Ok(())
            }
            (Action::Shoot { position }, Phase::Night) => {
                let chair = engine.chair_from_position(*position)?;
                engine.apply(Command::Shoot { chair })?;
                Ok(())
            }
            (Action::Nominate { position }, Phase::Day) => {
                let target = engine.chair_from_position(*position)?;
                engine.apply(Command::Nominate { target })?;
                Ok(())
            }
            (Action::NextSpeaker, Phase::Day) => {
                engine.apply(Command::NextSpeaker)?;
                Ok(())
            }
            (Action::Next, _) => {
                engine.apply(Command::NextPhase)?;
                Ok(())
            }
            (Action::Show, _) => {
                todo!("Implement private role display logic");
            }
            (Action::Quit, _) => Ok(()),
            (_, _) => {
                Ok(()) // Ignore invalid actions for the current phase
            }
        }
    }
}

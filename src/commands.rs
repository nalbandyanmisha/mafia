use clap::Parser;
use mafia::{Chair, Phase, Role, Table};

#[derive(Debug, Parser)]
pub enum Action {
    Join { name: String },
    Leave { name: String },
    Warn { position: u8 },
    Pardon { position: u8 },
    Nominate { position: u8 },
    Timer { seconds: u8 },
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
    pub async fn run(&self, table: &mut Table) -> Result<AppStatus, Box<dyn std::error::Error>> {
        match (self, &table.phase) {
            (Action::Join { name }, Phase::Registration) => {
                table.join(name)?;
                Ok(AppStatus::Continue)
            }
            (Action::Leave { name }, Phase::Registration) => {
                table.leave(name)?;
                println!("Player {name} left the game.");
                Ok(AppStatus::Continue)
            }
            (Action::Warn { position }, _) => {
                table.warn(Chair::new(*position))?;
                Ok(AppStatus::Continue)
            }
            (Action::Pardon { position }, _) => {
                table.pardon(Chair::new(*position))?;
                Ok(AppStatus::Continue)
            }
            (Action::Nominate { position }, Phase::Day) => {
                table.nominate(Chair::new(*position))?;
                println!("Player at position {position} has been nominated.");
                Ok(AppStatus::Continue)
            }
            (Action::Next, _) => {
                table.next_phase()?;
                Ok(AppStatus::Continue)
            }
            (Action::Show, _) => {
                table.render()?;
                Ok(AppStatus::Continue)
            }
            (Action::Quit, _) => {
                println!("Exiting the game.");
                Ok(AppStatus::Quit)
            }
            (_, _) => Ok(AppStatus::Continue),
        }
    }
}

use clap::Parser;

use crate::{actions::Action, engine::Engine};
#[derive(PartialEq)]
pub enum AppStatus {
    Running,
    Quit,
}

pub struct App {
    pub engine: Engine,
    pub status: AppStatus,
    pub input: String,
}

impl App {
    pub fn new() -> Self {
        App {
            engine: Engine::new(),
            status: AppStatus::Running,
            input: String::new(),
        }
    }

    pub fn on_key(&mut self, key: ratatui::crossterm::event::KeyCode) {
        use ratatui::crossterm::event::KeyCode::*;

        match key {
            Char(c) => self.input.push(c),
            Backspace => {
                self.input.pop();
            }
            Enter => {
                let _ = self.run_command();
                self.input.clear();
            }
            Esc => {
                self.input.clear();
            }
            _ => {}
        }
    }

    fn run_command(&mut self) -> Result<(), anyhow::Error> {
        let input = self.input.trim();
        if input.is_empty() {
            return Ok(());
        }

        // clap expects argv-style input
        let argv = std::iter::once("mafia").chain(input.split_whitespace());

        match Action::try_parse_from(argv) {
            Ok(Action::Quit) => {
                self.status = AppStatus::Quit;
                Ok(())
            }
            Ok(action) => {
                // IMPORTANT: Action owns execution
                action.run(&mut self.engine)?;
                Ok(())
            }
            Err(err) => {
                // clap error (unknown command, missing args)
                Err(err.into())
            }
        }
    }
}

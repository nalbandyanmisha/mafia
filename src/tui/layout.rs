pub mod chair;
pub mod command;
pub mod events;
pub mod host;
pub mod lobby;
pub mod main;
pub mod player;
pub mod shell;
pub mod table;

pub use chair::Chair;
pub use command::Command;
pub use events::Events;
pub use host::Host;
pub use lobby::Lobby;
pub use main::Main;
pub use player::Player;
pub use shell::Shell;
pub use table::Table;

#[derive(Debug, Clone)]
pub struct ShellLayout {
    pub main: Main,
    pub command: Command,
    pub events: Events,
}

impl ShellLayout {
    /// Compute the full shell layout given the terminal area
    pub fn new(area: ratatui::layout::Rect) -> Self {
        let shell = Shell::new(area);
        Self {
            main: Main::new(shell.main),
            command: Command::new(shell.command),
            events: Events::new(shell.events),
        }
    }
}

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
use ratatui::layout::Rect;
pub use shell::Shell;
pub use table::Table;

#[derive(Debug, Clone)]
pub struct Layout {
    pub screen: Shell,
}

impl Layout {
    pub fn new(area: Rect) -> Self {
        Self {
            screen: Shell::new(area),
        }
    }
}

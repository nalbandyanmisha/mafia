pub mod chair;
pub mod command;
pub mod events;
pub mod host;
pub mod lobby;
pub mod main;
pub mod player;
pub mod shell;
pub mod table;

pub use chair::ChairView;
pub use command::CommandView;
pub use events::EventsView;
pub use host::HostView;
pub use lobby::LobbyView;
pub use main::MainView;
pub use player::PlayerView;
pub use shell::Shell;
pub use table::TableView;

#[derive(Debug, Clone)]
pub struct View {
    pub screen: Shell,
}

impl View {
    /// Compute the views from the snapshot
    pub fn new(app: &crate::snapshot::App) -> Self {
        Self {
            screen: Shell::new(app),
        }
    }
}

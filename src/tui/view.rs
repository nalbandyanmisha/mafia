pub mod chair;
pub mod command;
pub mod events;
pub mod host;
pub mod lobby;
pub mod main;
pub mod player;
pub mod table;

pub use chair::ChairView;
pub use command::CommandView;
pub use events::EventsView;
pub use host::HostView;
pub use lobby::LobbyView;
pub use main::MainView;
pub use player::PlayerView;
pub use table::TableView;

#[derive(Debug, Clone)]
pub struct ShellView {
    pub main: MainView,
    pub command: CommandView,
    pub events: EventsView,
}

impl ShellView {
    /// Compute the views from the snapshot
    pub fn new(app: &crate::snapshot::App) -> Self {
        Self {
            main: MainView::from_snapshot(app),
            command: CommandView::from_snapshot(app),
            events: EventsView::from_snapshot(app),
        }
    }
}

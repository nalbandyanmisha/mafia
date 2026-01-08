use crate::{
    domain::EngineState,
    snapshot,
    tui::view::{LobbyView, TableView},
};

#[derive(Debug, Clone)]
pub enum MainView {
    Lobby(LobbyView),
    Table(TableView),
}

impl MainView {
    pub fn from_snapshot(app: &snapshot::App) -> Self {
        match app.engine.state {
            EngineState::Lobby(_) => MainView::Lobby(LobbyView::from_snapshot(app)),
            _ => MainView::Table(TableView::from_snapshot(app)),
        }
    }
}

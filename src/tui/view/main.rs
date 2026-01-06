use crate::tui::view::{LobbyView, TableView};

#[derive(Debug, Clone)]
pub enum MainView {
    Lobby(LobbyView),
    Table(TableView),
}

impl MainView {
    pub fn from_snapshot(app: &crate::snapshot::App) -> Self {
        use crate::domain::phase::Phase;

        match app.engine.phase {
            Phase::Lobby(_) => MainView::Lobby(LobbyView::from_snapshot(app)),
            _ => MainView::Table(TableView::from_snapshot(app)),
        }
    }
}

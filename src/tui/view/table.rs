use crate::snapshot;
use crate::tui::view::{ChairView, HostView};

#[derive(Debug, Clone)]
pub struct TableView {
    pub host: HostView,
    pub chairs: Vec<ChairView>,
}

impl TableView {
    pub fn from_snapshot(app: &snapshot::App) -> Self {
        let host = HostView::from_snapshot(app);

        let chairs = (1u8..=10) // or Game::PLAYER_COUNT if exposed
            .map(|i| {
                let position = i.into();
                ChairView::from_snapshot(position, app)
            })
            .collect();

        Self { host, chairs }
    }
}

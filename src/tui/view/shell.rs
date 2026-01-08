use super::{CommandView, EventsView, MainView};
use crate::snapshot;

#[derive(Debug, Clone)]
pub struct Shell {
    pub main: MainView,
    pub command: CommandView,
    pub events: EventsView,
}

impl Shell {
    /// Compute the views from the snapshot
    pub fn new(app: &snapshot::App) -> Self {
        Self {
            main: MainView::from_snapshot(app),
            command: CommandView::from_snapshot(app),
            events: EventsView::from_snapshot(app),
        }
    }
}

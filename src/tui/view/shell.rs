use super::{CommandView, EventsView, MainView};
use crate::{app::input::PopupKind, snapshot};

#[derive(Debug, Clone)]
pub struct Popup {
    pub kind: PopupKind,
    pub title: String,
    pub input: String,
}

#[derive(Debug, Clone)]
pub struct Shell {
    pub main: MainView,
    pub command: CommandView,
    pub events: EventsView,
    pub popup: Option<Popup>,
}

impl Shell {
    /// Compute the views from the snapshot
    pub fn new(app: &snapshot::App) -> Self {
        let popup = match &app.popup {
            Some(p) => Some(Popup {
                kind: p.kind.clone(),
                title: p.title.clone(),
                input: app.input.clone(),
            }),
            None => None,
        };

        Self {
            main: MainView::from_snapshot(app),
            command: CommandView::from_snapshot(app),
            events: EventsView::from_snapshot(app),
            popup,
        }
    }
}

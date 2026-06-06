use super::{CommandView, EventsView, MainView};
use crate::{app::input::{InputMode, PopupKind}, snapshot};

#[derive(Debug, Clone)]
pub enum Overlay {
    Help { title: String },
}

#[derive(Debug, Clone)]
pub struct Shell {
    pub main: MainView,
    pub command: CommandView,
    pub events: EventsView,
    pub overlay: Option<Overlay>,
}

impl Shell {
    /// Compute the views from the snapshot
    pub fn new(app: &snapshot::App) -> Self {
        let overlay = match &app.input_mode {
            InputMode::Popup { kind: PopupKind::Help, title } => Some(Overlay::Help {
                title: title.clone(),
            }),
            _ => None,
        };

        Self {
            main: MainView::from_snapshot(app),
            command: CommandView::from_snapshot(app),
            events: EventsView::from_snapshot(app),
            overlay,
        }
    }
}

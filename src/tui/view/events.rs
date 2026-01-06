#[derive(Debug, Clone)]
pub struct EventsView {
    pub messages: Vec<String>,
}

impl EventsView {
    pub fn from_snapshot(app: &crate::snapshot::App) -> Self {
        // For now, we assume `app.events` exists as Vec<String>
        // Adjust if your snapshot stores events differently
        Self {
            messages: Vec::new(),
        }
    }
}

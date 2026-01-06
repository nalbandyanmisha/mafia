#[derive(Debug, Clone)]
pub struct CommandView {
    pub input: String,
}

impl CommandView {
    pub fn from_snapshot(app: &crate::snapshot::App) -> Self {
        Self {
            input: app.input.clone(),
        }
    }
}


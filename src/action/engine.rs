#[derive(Debug, Clone)]
pub enum EngineAction {
    Join { name: String },
    Leave { name: String },
}
mod actor;
mod description;

pub use actor::Actor;
pub use description::Description;
use ratatui::layout::Rect;

#[derive(Debug, Clone)]
pub struct Main {
    pub actor: Actor,
    pub desc: Description,
}

impl Main {
    pub fn new(area: Rect) -> Self {
        Self {
            actor: Actor::new(area),
            desc: Description::new(area),
        }
    }
}

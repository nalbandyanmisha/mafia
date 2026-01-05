use ratatui::layout::{Margin, Rect};

#[derive(Debug, Clone)]
pub struct Events {
    pub area: Rect,
    pub content: Rect,
}

impl Events {
    /// Create an EventsLayout from a given area
    pub fn new(area: Rect) -> Self {
        let content = area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        });

        Self { area, content }
    }
}

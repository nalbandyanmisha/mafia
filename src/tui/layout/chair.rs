use ratatui::layout::{Margin, Rect};

#[derive(Debug, Clone)]
pub struct Chair {
    pub area: Rect,
    pub content: Rect, // inner area for player info
}

impl Chair {
    /// Create a ChairLayout from a given area
    pub fn new(area: Rect) -> Self {
        let content = area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        });

        Self { area, content }
    }
}

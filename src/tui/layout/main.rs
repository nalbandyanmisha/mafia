use ratatui::layout::{Margin, Rect};

#[derive(Debug, Clone)]
pub struct Main {
    pub area: Rect,
    pub content: Rect,
}

impl Main {
    /// Create a MainLayout from a given area
    pub fn new(area: Rect) -> Self {
        let content = area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        });

        Self { area, content }
    }
}

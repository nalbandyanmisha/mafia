use ratatui::layout::Rect;

#[derive(Debug, Clone)]
pub struct Main {
    pub area: Rect,
    pub content: Rect,
}

impl Main {
    /// Create a MainLayout from a given area
    pub fn new(area: Rect) -> Self {
        let content = area.inner(ratatui::layout::Margin {
            vertical: 1,
            horizontal: 1,
        });

        Self { area, content }
    }
}

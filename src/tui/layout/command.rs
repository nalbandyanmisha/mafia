use ratatui::layout::Rect;

#[derive(Debug, Clone)]
pub struct Command {
    pub area: Rect,
    pub input: Rect,
}

impl Command {
    /// Create a CommandLayout from a given area
    pub fn new(area: Rect) -> Self {
        let input = area.inner(ratatui::layout::Margin {
            vertical: 1,
            horizontal: 1,
        });

        Self { area, input }
    }
}

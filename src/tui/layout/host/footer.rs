use ratatui::layout::Rect;

#[derive(Debug, Clone)]
pub struct Footer {
    pub area: Rect,
}

impl Footer {
    pub fn new(area: Rect) -> Self {
        Self { area }
    }
}

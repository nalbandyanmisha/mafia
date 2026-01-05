use ratatui::layout::{Constraint, Direction, Layout, Rect};

#[derive(Debug, Clone)]
pub struct Player {
    pub area: Rect,
}

impl Player {
    /// Create a ChairLayout from a given area
    pub fn new(area: Rect, height: u16) -> Self {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(height),
                Constraint::Min(0),
            ])
            .split(area);

        Self { area: vertical[1] }
    }
}

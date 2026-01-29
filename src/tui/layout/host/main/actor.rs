use ratatui::layout::{Constraint, Layout, Margin, Rect};

#[derive(Debug, Clone)]
pub struct Actor {
    pub player: Rect,
    pub timer: Rect,
    pub result: Rect,
}

impl Actor {
    /// Create a MainLayout from a given area
    pub fn new(area: Rect) -> Self {
        let content = area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        });

        let lines = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(7),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(1),
        ])
        .split(content);

        Self {
            player: lines[1],
            timer: lines[3],
            result: lines[5],
        }
    }
}

use ratatui::layout::{Constraint, Layout, Margin, Rect};

#[derive(Debug, Clone)]
pub struct Description {
    pub area: Rect,
    pub desc: Rect,
}

impl Description {
    /// Create a MainLayout from a given area
    pub fn new(area: Rect) -> Self {
        let content = area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        });

        let lines = Layout::vertical([Constraint::Length(1)]).split(content);

        Self {
            area,
            desc: lines[0],
        }
    }
}

use ratatui::layout::Rect;

#[derive(Debug, Clone)]
pub struct Header {
    pub left: Rect,
    pub center: Rect,
    pub right: Rect,
    pub s_line: Rect,
}

impl Header {
    pub fn new(area: Rect) -> Self {
        use ratatui::layout::{Constraint, Layout};
        let lines = Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).split(area);

        let first_rows = Layout::horizontal([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(lines[0]);

        Self {
            left: first_rows[0],
            center: first_rows[1],
            right: first_rows[2],
            s_line: lines[1],
        }
    }
}

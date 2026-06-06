use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Center vertically with a fixed height.
pub fn v_center(area: Rect, height: u16) -> Rect {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(height), Constraint::Min(0)])
        .split(area)[1]
}

/// Center horizontally with a percentage width.
pub fn h_center(area: Rect, width_pct: u16) -> Rect {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - width_pct) / 2),
            Constraint::Percentage(width_pct),
            Constraint::Percentage((100 - width_pct) / 2),
        ])
        .split(area)[1]
}

/// Center both axes (percentage width, fixed height).
pub fn center(area: Rect, width_pct: u16, height: u16) -> Rect {
    h_center(v_center(area, height), width_pct)
}
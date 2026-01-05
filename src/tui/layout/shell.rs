use ratatui::layout::{Constraint, Layout, Rect};

#[derive(Debug, Clone)]
pub struct Shell {
    pub main: Rect,
    pub command: Rect,
    pub events: Rect,
}

impl Shell {
    /// Create a ShellLayout from a given area
    pub fn new(area: Rect) -> Self {
        let [left, events] =
            Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)])
                .areas(area);

        // 2️⃣ Split left side: main | command palette
        let [main, command] =
            Layout::vertical([Constraint::Min(10), Constraint::Length(3)]).areas(left);

        Self {
            main,
            command,
            events,
        }
    }
}

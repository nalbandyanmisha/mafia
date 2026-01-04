use ratatui::layout::{Constraint, Direction, Layout, Rect};

#[derive(Debug, Clone)]
pub struct RootLayout {
    pub main: Rect,
    pub command: Rect,
    pub events: Rect,
}

pub fn root(area: Rect) -> RootLayout {
    // 1️⃣ Split screen: left (main+command) | right (event log)
    let [left, events] =
        Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)]).areas(area);

    // 2️⃣ Split left side: main | command palette
    let [main, command] =
        Layout::vertical([Constraint::Min(10), Constraint::Length(3)]).areas(left);

    RootLayout {
        main,
        command,
        events,
    }
}

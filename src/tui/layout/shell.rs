use ratatui::layout::{Constraint, Layout, Rect};

#[derive(Debug, Clone)]
pub struct ShellLayout {
    pub main: Rect,
    pub command: Rect,
    pub events: Rect,
}

pub fn shell(area: Rect) -> ShellLayout {
    // 1️⃣ Split screen: left (main+command) | right (event log)
    let [left, events] =
        Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)]).areas(area);

    // 2️⃣ Split left side: main | command palette
    let [main, command] =
        Layout::vertical([Constraint::Min(10), Constraint::Length(3)]).areas(left);

    ShellLayout {
        main,
        command,
        events,
    }
}

use ratatui::layout::{Constraint, Layout, Rect};

use super::{Command, Events, Main};

#[derive(Debug, Clone)]
pub struct Shell {
    pub main: Main,
    pub command: Command,
    pub events: Events,
}

impl Shell {
    /// Compute the full shell layout given the terminal area
    pub fn new(area: Rect) -> Self {
        let [left, events] =
            Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)])
                .areas(area);

        // 2️⃣ Split left side: main | command palette
        let [main, command] =
            Layout::vertical([Constraint::Min(10), Constraint::Length(3)]).areas(left);

        Self {
            main: Main::new(main),
            command: Command::new(command),
            events: Events::new(events),
        }
    }
}

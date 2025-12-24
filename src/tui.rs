pub mod chair;
pub mod layout;
pub mod table;

use crate::app::App;

use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    crossterm::{
        ExecutableCommand,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use std::{io::stdout, panic};

pub fn install_panic_hook() {
    let original = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = stdout().execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
        original(info);
    }));
}

pub fn init_terminal() -> anyhow::Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    Ok(Terminal::new(backend)?)
}

pub fn restore_terminal() -> anyhow::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn draw_ui(frame: &mut Frame, app: &App) {
    let (table, event_log, command) = layout::draw_layout(frame);
    table::draw_table(frame, table, 10);
}

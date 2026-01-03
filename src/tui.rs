pub mod layout;
pub mod widgets;

use crate::domain::phase::Phase;
use crate::snapshot;
use crate::tui::widgets::{command, lobby, table};

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

pub fn draw_ui(frame: &mut Frame, app: &snapshot::App) {
    let (main, event_log, command) = layout::draw_layout(frame);

    match app.engine.game.phase {
        Phase::Lobby(_) => {
            lobby::draw_lobby(frame, main, app).unwrap();
        }
        _ => {
            table::draw_table(frame, main, app).unwrap();
        }
    }

    command::draw_command(frame, &command, &app.input).unwrap();
}

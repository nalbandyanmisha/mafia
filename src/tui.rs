pub mod layout;
pub mod view;
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
    let root_layout = layout::root::root(frame.area());
    widgets::layout::draw_layout(frame, &root_layout);

    match app.engine.game.phase {
        Phase::Lobby(_) => {
            let lobby_layout = layout::lobby::lobby(root_layout.main);
            lobby::draw_lobby(frame, &lobby_layout, app).unwrap();
        }
        _ => {
            let layout = layout::table::table(root_layout.main, 10);
            table::draw_table(frame, &layout, app).unwrap();
        }
    }

    command::draw_command(frame, &root_layout.command, &app.input).unwrap();
}

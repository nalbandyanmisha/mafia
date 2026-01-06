pub mod layout;
pub mod view;
pub mod widgets;

use crate::snapshot;
use crate::tui::widgets::{command, events, main};

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
    // let shell = layout::Shell::new(frame.area());
    //
    // // Main content (lobby or table)
    // let main_view = view::MainView::from_snapshot(app);
    // widgets::main::draw(frame, shell.main, &main_view, app);
    //
    // // Events panel
    // let events_view = view::EventsView::from_snapshot(app);
    // widgets::events::draw(frame, shell.events, &events_view);
    //
    // // Command input
    // let command_view = view::CommandView::from_snapshot(app);
    // widgets::command::draw(frame, shell.command, &command_view.input);

    let layout = crate::tui::layout::ShellLayout::new(frame.area());
    let view = crate::tui::view::ShellView::new(app);

    crate::tui::widgets::draw(frame, &layout, &view, app);
}

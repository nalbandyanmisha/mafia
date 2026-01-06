pub mod chair;
pub mod command;
pub mod events;
pub mod host;
pub mod lobby;
pub mod main;
pub mod player;
pub mod table;

use crate::tui::layout::ShellLayout;
use crate::tui::view::ShellView;
use ratatui::Frame;

pub fn draw(frame: &mut Frame, layout: &ShellLayout, view: &ShellView, app: &crate::snapshot::App) {
    main::draw(frame, &layout.main, &view.main, app);
    command::draw(frame, &layout.command, &view.command, app);
    events::draw(frame, &layout.events, &view.events, app);
}

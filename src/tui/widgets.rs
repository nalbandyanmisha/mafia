pub mod chair;
pub mod command;
pub mod events;
pub mod host;
pub mod lobby;
pub mod main;
pub mod player;
pub mod popup;
pub mod table;

use ratatui::Frame;

use super::{layout::Layout, view::View};

pub fn draw(frame: &mut Frame, terminal: &Layout, data: &View) {
    main::draw(frame, &terminal.screen.main, &data.screen.main);
    command::draw(frame, &terminal.screen.command, &data.screen.command);
    events::draw(frame, &terminal.screen.events, &data.screen.events);
}

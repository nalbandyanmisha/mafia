pub mod chair;
pub mod command;
pub mod events;
pub mod help;
pub mod host;
pub mod input;
pub mod lobby;
pub mod main;
pub mod player;
pub mod popup;
pub mod table;

use ratatui::Frame;

use super::{layout::Layout, util::center, view::View};
use crate::app::input::PopupKind;

pub fn draw(frame: &mut Frame, terminal: &Layout, data: &View) {
    main::draw(frame, &terminal.screen.main, &data.screen.main);
    command::draw(frame, &terminal.screen.command, &data.screen.command);
    events::draw(frame, &terminal.screen.events, &data.screen.events);

    if let Some(popup) = &data.screen.popup {
        match popup.kind {
            PopupKind::Help => {
                let (w, h) = help::size(terminal.screen.main.area);
                let area = center(terminal.screen.main.area, w, h);
                popup::draw(frame, area, &popup.title, help::widget());
            }
            _ => {
                let (w, h) = input::size(popup.kind.clone());
                let area = center(terminal.screen.main.area, w, h);
                popup::draw(frame, area, &popup.title, input::widget(&popup.input));
            }
        }
    }
}

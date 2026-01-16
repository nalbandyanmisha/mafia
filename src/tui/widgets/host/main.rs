mod actor;
mod description;

use crate::tui::{layout, view};
use ratatui::Frame;

pub fn draw(
    frame: &mut Frame,
    layout: &layout::host::Main,
    view: &view::host::Main,
) -> anyhow::Result<()> {
    match view {
        view::host::Main::Actor(actor) => {
            actor::draw(frame, &layout.actor, actor)?;
        }
        view::host::Main::Description(text) => {
            description::draw(frame, &layout.desc, text)?;
        }
    }

    Ok(())
}

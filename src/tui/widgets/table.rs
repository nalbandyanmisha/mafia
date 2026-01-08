use crate::tui::view::{ChairView, TableView};
use crate::tui::widgets::{chair, host};
use ratatui::Frame;

use crate::tui::layout;

fn draw_chairs_around_host(
    frame: &mut Frame,
    layout_chairs: &[layout::Chair],
    view_chairs: &[ChairView],
) -> Result<(), anyhow::Error> {
    for (i, chair) in layout_chairs.iter().enumerate() {
        chair::draw(frame, chair, &view_chairs[i]);
    }

    Ok(())
}

pub fn draw(
    frame: &mut Frame,
    layout: &layout::Table,
    view: &TableView,
) -> Result<(), anyhow::Error> {
    host::draw(frame, &layout.host, &view.host)?;
    draw_chairs_around_host(frame, &layout.chairs, &view.chairs).unwrap();
    Ok(())
}

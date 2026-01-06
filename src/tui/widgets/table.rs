use crate::snapshot::App;
use crate::tui::view::{ChairView, HostView, TableView};
use crate::tui::widgets::{chair, host};
use ratatui::Frame;

use crate::tui::layout;

fn draw_chairs_around_host(
    frame: &mut Frame,
    chairs: &[layout::Chair],
    app: &App,
) -> Result<(), anyhow::Error> {
    for (i, chair_layout) in chairs.iter().enumerate() {
        let position = ((i + 1) as u8).into();

        let chair_view = ChairView::from_snapshot(position, app);

        chair::draw(frame, chair_layout, &chair_view);
    }

    Ok(())
}

pub fn draw(
    frame: &mut Frame,
    layout: &layout::Table,
    view: &TableView,
    app: &App,
) -> Result<(), anyhow::Error> {
    let host_view = HostView::from_snapshot(app);
    host::draw(frame, &layout.host, &host_view)?;
    draw_chairs_around_host(frame, &layout.chairs, app).unwrap();
    Ok(())
}


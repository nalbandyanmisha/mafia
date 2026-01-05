use crate::snapshot::App;
use crate::tui::view::PlayerView;
use crate::tui::widgets::{chair, host};
use ratatui::Frame;

use crate::tui::layout;

fn draw_chairs_around_host(
    frame: &mut Frame,
    chairs_areas: &[layout::Chair],
    app: &App,
) -> Result<(), anyhow::Error> {
    for (i, area) in chairs_areas.iter().enumerate() {
        let view = PlayerView::from_snapshot(((i + 1) as u8).into(), app);
        chair::draw(
            frame,
            area,
            &view,
            &app.engine.game.players[i],
            &app.engine.actor,
        )?;
    }
    Ok(())
}

pub fn draw(frame: &mut Frame, layout: &layout::Table, app: &App) -> Result<(), anyhow::Error> {
    host::draw(frame, &layout.host, app)?;
    draw_chairs_around_host(frame, &layout.chairs, app).unwrap();
    Ok(())
}

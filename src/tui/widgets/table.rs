use crate::snapshot::App;
use crate::tui::view::PlayerView;
use crate::tui::widgets::{chair, host};
use ratatui::Frame;

use crate::tui::layout::{ChairLayout, TableLayout};

fn draw_chairs_around_host(
    frame: &mut Frame,
    chairs_areas: &[ChairLayout],
    app: &App,
) -> Result<(), anyhow::Error> {
    for (i, area) in chairs_areas.iter().enumerate() {
        let player = &app.engine.game.players[i];
        let view = PlayerView::from_snapshot(player, app);
        chair::draw_chair(
            frame,
            area,
            &view,
            &app.engine.game.players[i],
            &app.engine.actor,
        )?;
    }
    Ok(())
}

pub fn draw_table(frame: &mut Frame, layout: &TableLayout, app: &App) -> Result<(), anyhow::Error> {
    host::draw_host(frame, &layout.host, app)?;
    draw_chairs_around_host(frame, &layout.chairs, app).unwrap();
    Ok(())
}

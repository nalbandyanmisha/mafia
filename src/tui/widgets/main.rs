use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, Borders},
};

use crate::{
    domain::phase::Phase,
    snapshot::App,
    tui::layout,
    tui::widgets::{lobby, table},
};

pub fn draw(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let layout = layout::main::main(area);

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(" MAIN ")
            .style(Style::default().fg(Color::Green)),
        layout.area,
    );

    match app.engine.game.phase {
        Phase::Lobby(_) => {
            let lobby_layout = layout::lobby::lobby(layout.content);
            lobby::draw(frame, &lobby_layout, app).unwrap();
        }
        _ => {
            let table_layout = layout::table::table(layout.content, 10);
            table::draw(frame, &table_layout, app).unwrap();
        }
    }
}

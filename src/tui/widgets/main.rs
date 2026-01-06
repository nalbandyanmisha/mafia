use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, Borders},
};

use crate::{
    domain::phase::Phase,
    snapshot::App,
    tui::{
        layout,
        view::MainView,
        widgets::{lobby, table},
    },
};

pub fn draw(frame: &mut Frame, layout: &layout::Main, view: &MainView, app: &App) {
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(" MAIN ")
            .style(Style::default().fg(Color::Green)),
        layout.area,
    );

    match app.engine.phase {
        Phase::Lobby(_) => {
            let lobby_layout = layout::Lobby::new(layout.content);
            let lobby_view = crate::tui::view::LobbyView::from_snapshot(app);
            lobby::draw(frame, &lobby_layout, &lobby_view).unwrap();
        }
        _ => {
            let table_layout = layout::Table::new(layout.content, 10);
            let table_view = crate::tui::view::TableView::from_snapshot(app);
            table::draw(frame, &table_layout, &table_view, app).unwrap();
        }
    }
}

use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, Borders},
};

use crate::tui::{
    layout,
    view::MainView,
    widgets::{lobby, table},
};

pub fn draw(frame: &mut Frame, layout: &layout::Main, view: &MainView) {
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(" MAIN ")
            .style(Style::default().fg(Color::Green)),
        layout.area,
    );

    match view {
        MainView::Lobby(lobby_view) => {
            let lobby_layout = layout::Lobby::new(layout.content);
            lobby::draw(frame, &lobby_layout, lobby_view).unwrap();
        }
        MainView::Table(table_view) => {
            let table_layout = layout::Table::new(layout.content, 10);
            table::draw(frame, &table_layout, table_view).unwrap();
        }
    }
}

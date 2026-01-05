use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::{snapshot::App, tui::layout};

pub fn draw(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let layout = layout::Events::new(area);

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(" EVENTS ")
            .style(Style::default().fg(Color::Magenta)),
        layout.area,
    );

    // placeholder for now
    frame.render_widget(Paragraph::new("No events yet"), layout.content);
}

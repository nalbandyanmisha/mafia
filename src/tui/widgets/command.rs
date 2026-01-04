use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::layout;

pub fn draw(frame: &mut Frame, area: ratatui::layout::Rect, input: &str) {
    let layout = layout::command::command(area);

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title("Command Input")
            .style(Style::default().fg(Color::Cyan)),
        layout.area,
    );

    frame.render_widget(Paragraph::new(input), layout.input);
}

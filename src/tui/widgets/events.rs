use ratatui::{
    Frame,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{tui::layout, tui::view::events::EventsView};

pub fn draw(frame: &mut Frame, layout: &layout::Events, view: &EventsView) {
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(" EVENTS ")
            .style(Style::default().fg(Color::Magenta)),
        layout.area,
    );

    let lines: Vec<Line> = if view.messages.is_empty() {
        vec![Line::from("No events yet")]
    } else {
        view.messages
            .iter()
            .map(|m| Line::from(m.to_string().clone()))
            .collect()
    };

    frame.render_widget(
        Paragraph::new(lines).wrap(Wrap { trim: true }),
        layout.content,
    );
}

use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::{layout, view};

pub fn draw(frame: &mut Frame, layout: &layout::Command, view: &view::CommandView) {
    let layout = layout::Command::new(layout.area);

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title("Command Input")
            .style(Style::default().fg(Color::Cyan)),
        layout.area,
    );

    frame.render_widget(Paragraph::new(view.input.clone()), layout.input);
}

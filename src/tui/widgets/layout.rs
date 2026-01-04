use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, Borders},
};

use crate::tui::layout::RootLayout;

pub fn draw_layout(frame: &mut Frame, root: &RootLayout) {
    // MAIN block
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(" MAIN ")
            .style(Style::default().fg(Color::Green)),
        root.main,
    );
    // COMMAND block
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(" COMMAND ")
            .style(Style::default().fg(Color::Cyan)),
        root.command,
    );

    // EVENTS block
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(" EVENTS ")
            .style(Style::default().fg(Color::Magenta)),
        root.events,
    );
}

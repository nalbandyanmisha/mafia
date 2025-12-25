use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub fn draw_host(frame: &mut Frame, host_area: Rect) {
    // Draw the host/control panel block
    let host_block = Block::default()
        .borders(Borders::ALL)
        .title("Control Panel")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow));

    frame.render_widget(host_block, host_area);
}

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw_command(frame: &mut Frame, area: &Rect, input: &str) -> Result<(), anyhow::Error> {
    let command_block = Block::default()
        .borders(Borders::ALL)
        .title("Command Input")
        .style(Style::default().fg(Color::Cyan));
    command_block.inner(*area);

    frame.render_widget(Paragraph::new(input).block(command_block), *area);
    Ok(())
}

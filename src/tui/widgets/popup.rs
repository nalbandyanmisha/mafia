use ratatui::{
    Frame,
    layout::Rect,
    style::Color,
    widgets::{Block, BorderType, Borders, Clear, Widget},
};

pub fn draw(frame: &mut Frame, area: Rect, title: &str, content: impl Widget) {
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Color::White)
        .border_type(BorderType::Thick);

    let inner = block.inner(area);
    frame.render_widget(block, area);
    frame.render_widget(content, inner);
}


use ratatui::{
    Frame,
    layout::Alignment,
    style::Color,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

use crate::{
    app::input::help_text,
    tui::{util::centered_area, view::shell::Overlay},
};

use ratatui::layout::Rect;

pub fn draw(frame: &mut Frame, main_area: Rect, overlay: &Overlay) {
    match overlay {
        Overlay::Help { title } => draw_help(frame, main_area, title),
    }
}

fn draw_help(frame: &mut Frame, main_area: Rect, title: &str) {
    let help_content = help_text();
    let max_height = main_area.height.saturating_sub(4);
    let content_lines = help_content.lines().count() as u16;
    let height = content_lines.min(max_height);
    let area = centered_area(main_area, height);

    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Color::White)
        .border_type(BorderType::Thick);

    let paragraph = Paragraph::new(help_content)
        .block(block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

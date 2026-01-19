use ratatui::{
    Frame,
    layout::Alignment,
    text::Text,
    widgets::{Paragraph, Wrap},
};

use crate::tui::{layout, util::centered_area, view};

pub fn draw(
    frame: &mut Frame,
    layout: &layout::host::main::Actor,
    view: &view::host::main::Actor,
) -> anyhow::Result<()> {
    use ratatui::{
        style::*,
        text::{Line, Span},
    };

    let mut lines: Vec<Line> = view.actor.lines().map(Line::from).collect();

    if let Some(sec) = view.timer {
        let style = if sec <= 10 {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        };

        lines.push(Line::from(Span::styled(
            format!("⏳ {:02}:{:02}", sec / 60, sec % 60),
            style,
        )));
    }

    if let Some(r) = &view.result {
        lines.extend(r.lines().map(Line::from));
    }

    let text = Text::from(lines);
    let centered = centered_area(layout.area, text.height() as u16);

    frame.render_widget(
        Paragraph::new(text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        centered,
    );

    Ok(())
}

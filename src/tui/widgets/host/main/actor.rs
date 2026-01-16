use ratatui::{Frame, layout::Alignment, text::Text, widgets::Paragraph};

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

    let mut lines = vec![Line::from(format!("🎭 {}", view.position))];

    if let Some(t) = view.timer {
        lines.push(Line::from(Span::styled(
            format!("⏱ {t}s"),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
    }

    lines.push(Line::from(view.instructions.clone()));

    if let Some(r) = &view.result {
        lines.push(Line::from(format!("✅ {r}")));
    }

    let text = Text::from(lines);
    let centered = centered_area(layout.area, text.height() as u16);

    frame.render_widget(Paragraph::new(text).alignment(Alignment::Center), centered);

    Ok(())
}

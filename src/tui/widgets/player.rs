use crate::domain::Status;
use crate::tui::layout;
use crate::tui::view::player::PlayerView;
use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};

pub fn draw(frame: &mut Frame, l: &layout::Player, view: &PlayerView) {
    let mut lines: Vec<Line> = Vec::new();

    // ── Name (bold, primary anchor)
    lines.push(Line::from(Span::styled(
        &view.name,
        Style::default().add_modifier(Modifier::BOLD),
    )));

    // ── Status + warnings (single compact line)
    let warnings = "⚠️".repeat(view.warnings as usize);

    let status_icon = match view.status {
        Status::Alive => "🟢 Alive",
        Status::Dead => "💀 Dead",
        Status::Eliminated => "❌ Out",
        Status::Removed => "🚫 Removed",
    };

    lines.push(Line::from(vec![
        Span::styled(warnings, Style::default().fg(Color::Yellow)),
        Span::raw("   "),
        Span::styled(status_icon, Style::default().fg(Color::Gray)),
    ]));

    // ── Role (only if known / relevant)
    if let Some(role) = &view.role {
        lines.push(Line::from(Span::styled(
            format!("🎭 {role}"),
            Style::default().fg(Color::White),
        )));
    }

    // ── Nomination (only if exists)
    if view.is_nominated || view.nominated.is_some() {
        let mut spans = Vec::new();

        // This player is nominated
        if view.is_nominated {
            spans.push(Span::styled(
                "📌",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::raw("   "));
        }

        // This player nominated someone
        if let Some(target) = view.nominated {
            spans.push(Span::styled("🗳️", Style::default().fg(Color::Cyan)));
            spans.push(Span::raw(" → "));
            spans.push(Span::styled(
                target.to_string(),
                Style::default().fg(Color::Cyan),
            ));
        }

        lines.push(Line::from(spans));
    }
    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, l.area);
}

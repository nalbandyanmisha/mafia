use crate::tui::layout;
use crate::tui::view::player::PlayerView;
use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};

pub fn draw(frame: &mut Frame, layout: &layout::Player, view: &PlayerView) {
    // Build lines dynamically
    let role = if let Some(r) = &view.role {
        format!("{r}")
    } else {
        "No Role".to_string()
    };
    let lines: Vec<Line> = vec![
        Line::from(Span::styled(
            &view.name,
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("Status: {}", view.status),
            Style::default().fg(Color::Gray),
        )),
        Line::from(Span::styled(
            format!("Warnings: {}⚠️", view.warnings),
            Style::default().fg(Color::Yellow),
        )),
        Line::from(Span::styled(
            format!("Role: {role}",),
            Style::default().fg(Color::Magenta),
        )),
        Line::from(Span::styled(
            if view.is_nominated {
                "Nominated: Yes"
            } else {
                "Nominated: No"
            },
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            if let Some(pos) = view.nominated {
                format!("Nominated: {pos}")
            } else {
                "Nominated: N/A".to_string()
            },
            Style::default().fg(Color::Cyan),
        )),
    ];

    // Paragraph with wrap + alignment
    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, layout.area);
}

use ratatui::{Frame, layout::Alignment, text::Text};

use crate::tui::{layout, view};

pub fn draw(
    frame: &mut Frame,
    layout: &layout::host::main::Actor,
    view: &view::host::main::Actor,
) -> anyhow::Result<()> {
    use ratatui::{
        style::*,
        text::{Line, Span},
    };

    frame.render_widget(
        view.actor
            .lines()
            .map(Line::from)
            .collect::<Text>()
            .fg(Color::White)
            .alignment(Alignment::Center),
        layout.player,
    );

    if let Some(sec) = view.timer {
        let style = if sec <= 10 {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        };

        frame.render_widget(
            Line::from(Span::styled(
                format!("⏳ {:02}:{:02}", sec / 60, sec % 60),
                style,
            ))
            .alignment(Alignment::Center),
            layout.timer,
        );
    }

    if let Some(r) = &view.result {
        frame.render_widget(
            r.lines()
                .map(Line::from)
                .collect::<Text>()
                .fg(Color::White)
                .alignment(Alignment::Center),
            layout.result,
        );
    }

    Ok(())
}

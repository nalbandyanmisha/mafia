use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    domain::phase::{DayPhase, LobbyPhase, Phase},
    snapshot::{AppData, EngineData},
};

pub fn draw_host(
    frame: &mut Frame,
    host_area: Rect,
    host_data: &AppData,
) -> Result<(), anyhow::Error> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("HOST")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow));

    let inner = block.inner(host_area);
    frame.render_widget(block, host_area);
    let sections = Layout::vertical([
        Constraint::Length(2), // header
        Constraint::Min(3),    // main
        Constraint::Length(2), // footer
    ])
    .split(inner);

    draw_host_header(frame, sections[0], &host_data.engine)?;
    draw_host_main(frame, sections[1], &host_data.engine)?;
    draw_host_footer(frame, sections[2], host_data)?;

    Ok(())
}

fn draw_host_header(
    frame: &mut Frame,
    area: Rect,
    engine_data: &EngineData,
) -> Result<(), anyhow::Error> {
    let text = format!(
        "{:?} · ROUND {}",
        engine_data.phase, engine_data.current_round
    );
    let style = match engine_data.phase {
        Phase::Day(DayPhase::Voting(_)) => Style::default().fg(Color::Magenta),
        Phase::Day(_) => Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
        Phase::Night(_) => Style::default().fg(Color::Blue),
        _ => Style::default().fg(Color::Gray),
    };

    let p = Paragraph::new(text)
        .alignment(Alignment::Center)
        .style(style);

    frame.render_widget(p, area);
    Ok(())
}

fn draw_host_main(
    frame: &mut Frame,
    area: Rect,
    engine_data: &EngineData,
) -> Result<(), anyhow::Error> {
    let (title, subtitle) = match engine_data.phase {
        Phase::Lobby(LobbyPhase::Waiting) => ("WAITING", None),

        Phase::Day(DayPhase::Discussion) => (
            "DISCUSSION",
            engine_data
                .current_speaker
                .clone()
                .map(|c| format!("🗣️ Chair {}", c.position)),
        ),

        Phase::Night(_) => ("MAFIA ACTING", None),

        Phase::Day(DayPhase::Voting(_)) => ("VOTING", None),

        _ => ("", None),
    };
    let mut lines = vec![Line::from(title).style(Style::default().add_modifier(Modifier::BOLD))];

    if let Some(sub) = subtitle {
        lines.push(Line::from(sub));
    }

    let text = Text::from(lines);

    let centered = centered_area(area, text.height() as u16);

    let p = Paragraph::new(text).alignment(Alignment::Center);

    frame.render_widget(p, centered);
    Ok(())
}

fn centered_area(area: Rect, height: u16) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(area);

    vertical[1]
}

fn draw_host_footer(
    frame: &mut Frame,
    area: Rect,
    app_data: &AppData,
) -> Result<(), anyhow::Error> {
    let text = match app_data.current_timer {
        Some(sec) => format!("⏳ {:02}:{:02}", sec / 60, sec % 60),
        None => "NO TIMER".to_string(),
    };

    let style = if matches!(app_data.current_timer, Some(s) if s <= 10) {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let p = Paragraph::new(text)
        .alignment(Alignment::Center)
        .style(style);

    frame.render_widget(p, area);
    Ok(())
}

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::{
    domain::phase::{CheckPhase, DayPhase, LobbyPhase, NightPhase, Phase, VotingPhase},
    snapshot::{AppData, EngineData},
};

pub fn draw_host(
    frame: &mut Frame,
    host_area: Rect,
    host_data: &AppData,
) -> Result<(), anyhow::Error> {
    let (text, style) = match host_data.engine.phase {
        Phase::Lobby(_) => ("Lobby".to_string(), Style::default().fg(Color::Gray)),
        Phase::Day(_) => (
            format!("Day ·  {}", host_data.engine.current_round),
            Style::default().fg(Color::Yellow),
        ),
        Phase::Night(_) => (
            format!("Night ·  {}", host_data.engine.current_round),
            Style::default().fg(Color::Magenta),
        ),
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(text)
        .title_alignment(Alignment::Center)
        .style(style);

    let inner = block.inner(host_area);
    frame.render_widget(block, host_area);
    let sections = Layout::vertical([
        Constraint::Length(1), // empty line
        Constraint::Length(1), // header
        Constraint::Min(3),    // main
        Constraint::Length(1), // footer
    ])
    .split(inner);

    draw_host_header(frame, sections[1], &host_data.engine)?;
    draw_host_main(frame, sections[2], &host_data.engine)?;
    draw_host_footer(frame, sections[3], host_data)?;

    Ok(())
}

fn draw_host_header(
    frame: &mut Frame,
    area: Rect,
    engine_data: &EngineData,
) -> Result<(), anyhow::Error> {
    let (text, style) = match engine_data.phase {
        Phase::Lobby(LobbyPhase::Waiting) => ("Waiting", Style::default().fg(Color::Gray)),
        Phase::Lobby(LobbyPhase::Ready) => ("Ready", Style::default().fg(Color::Gray)),
        Phase::Day(DayPhase::Morning) => ("Morning", Style::default().fg(Color::Yellow)),
        Phase::Day(DayPhase::Discussion) => ("Discussion", Style::default().fg(Color::Yellow)),
        Phase::Day(DayPhase::Voting(_)) => ("Voting", Style::default().fg(Color::Yellow)),
        Phase::Night(NightPhase::RoleAssignment) => {
            ("Role Assignment", Style::default().fg(Color::Magenta))
        }
        Phase::Night(NightPhase::SheriffReveal) => {
            ("Sheriff Reveal", Style::default().fg(Color::Magenta))
        }
        Phase::Night(NightPhase::MafiaBriefing) => {
            ("Mafia Briefing", Style::default().fg(Color::Magenta))
        }
        Phase::Night(NightPhase::MafiaShoot) => {
            ("Mafia Shooting", Style::default().fg(Color::Magenta))
        }
        Phase::Night(NightPhase::Investigation(CheckPhase::Sheriff)) => {
            ("Sherif Checking", Style::default().fg(Color::Magenta))
        }
        Phase::Night(NightPhase::Investigation(CheckPhase::Don)) => {
            ("Don Checking", Style::default().fg(Color::Magenta))
        }
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
        Phase::Lobby(LobbyPhase::Ready) => ("Ready", None),

        Phase::Day(DayPhase::Morning) => ("MORNING", None),
        Phase::Day(DayPhase::Discussion) => (
            "DISCUSSION",
            engine_data.actor.clone().map(|c| {
                format!(
                    "Player at {}🪑 is 🗣️ and 🎯{}",
                    c.position,
                    engine_data
                        .round
                        .voting
                        .nominations
                        .get(&c)
                        .map_or(0, |n| n.position)
                )
            }),
        ),
        Phase::Day(DayPhase::Voting(VotingPhase::Nomination)) => (
            "Nominations",
            format!(
                "Nominated: {}",
                engine_data
                    .round
                    .voting
                    .nominees
                    .iter()
                    .map(|c| format!("🪑{}", c.position))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .into(),
        ),

        Phase::Day(DayPhase::Voting(VotingPhase::VoteCast)) => (
            "Cast Your Vote",
            engine_data.actor.clone().map(|c| {
                format!(
                    "Player at {}🪑 was voted by {:?}",
                    c.position,
                    engine_data.round.voting.votes.get(&c).map(|voters| {
                        voters
                            .iter()
                            .map(|v| format!("🪑{}", v.position))
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                )
            }),
        ),

        Phase::Night(NightPhase::RoleAssignment) => (
            "Reveal Role",
            engine_data
                .actor
                .clone()
                .map(|c| format!("🎭 Chair {}", c.position)),
        ),

        Phase::Night(NightPhase::MafiaShoot) => (
            "MAFIA IS SHOOTING",
            engine_data
                .round
                .mafia_kill
                .clone()
                .map(|c| format!("🎯 Chair {}", c.position)),
        ),

        Phase::Night(NightPhase::Investigation(CheckPhase::Sheriff)) => (
            "SHERIFF IS CHECKING",
            engine_data
                .round
                .sheriff_check
                .clone()
                .map(|c| format!("🎯 Chair {}", c.position)),
        ),

        Phase::Night(NightPhase::Investigation(CheckPhase::Don)) => (
            "DON IS CHECKING",
            engine_data
                .round
                .don_check
                .clone()
                .map(|c| format!("🎯 Chair {}", c.position)),
        ),

        _ => ("", None),
    };
    let mut lines = vec![Line::from(title).style(Style::default().add_modifier(Modifier::BOLD))];

    if let Some(sub) = subtitle {
        lines.push(Line::from(sub));
    }

    let text = Text::from(lines);

    let centered = centered_area(area, text.height() as u16);

    let p = Paragraph::new(text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

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

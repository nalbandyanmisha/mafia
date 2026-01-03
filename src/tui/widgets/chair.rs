use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::{
    domain::{position::Position, status::Status},
    snapshot::Player,
};

enum Visual {
    Empty,      // no player
    Alive,      // still in the game
    Dead,       // killed by mafia
    Eliminated, // voted out by players
    Removed,    // kicked out by host due to rule violation
    Speaking,   // player is currently speaking
    Muted,      // player is muted one turn due to warnings
    Candidate,  // player is a candidate to be voted as mafia
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

fn build_chair_frame(
    visual: &Visual,
    position: &Position,
) -> Result<Block<'static>, anyhow::Error> {
    let position = position.value();
    let (style, title) = match visual {
        Visual::Empty => (
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::DIM),
            format!("Chair {position} (⬜)"),
        ),
        Visual::Alive => (
            Style::default().fg(Color::White),
            format!("Chair {position} (💚)"),
        ),
        Visual::Dead => (
            Style::default().fg(Color::Red),
            format!("Chair {position} (💀)"),
        ),

        Visual::Eliminated => (
            Style::default().fg(Color::Red),
            format!("Chair {position} (❌)"),
        ),
        Visual::Removed => (
            Style::default().fg(Color::Red),
            format!("Chair {position} (🚫)"),
        ),
        Visual::Speaking => (
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            format!("Chair {position} (🗣️)"),
        ),
        Visual::Muted => (
            Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
            format!("Chair {position} (🤐)"),
        ),
        Visual::Candidate => (
            Style::default().fg(Color::Magenta),
            format!("Chair {position} (🎯)"),
        ),
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(title)
        .title_alignment(Alignment::Center)
        .style(style);

    Ok(block)
}

fn build_chair_content(player: &Player) -> Result<Paragraph<'static>, anyhow::Error> {
    match player.status {
        Status::Alive => {
            let role = if let Some(r) = player.role {
                format!("{r:?}",)
            } else {
                "No Role".to_string()
            };
            let lines = vec![
                Line::from(vec![
                    Span::styled(
                        player.name.clone(),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("   "),
                    Span::styled(
                        format!("⚠️{}", player.warnings),
                        Style::default().fg(Color::Yellow),
                    ),
                ]),
                Line::from(vec![Span::styled(role, Style::default().fg(Color::Gray))]),
            ];

            Ok(Paragraph::new(lines)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true }))
        }
        _ => Ok(Paragraph::new("Empty Seat, Player is not alive")
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })),
    }
}

pub fn draw_chair(
    frame: &mut Frame,
    area: Rect,
    player: &Player,
    actor: &Option<Position>,
) -> Result<(), anyhow::Error> {
    let mut visual = if player.position == *actor {
        Visual::Speaking
    } else {
        match player.status {
            Status::Alive => Visual::Alive,
            Status::Dead => Visual::Dead,
            Status::Removed => Visual::Removed,
            Status::Eliminated => Visual::Eliminated,
        }
    };

    if player.is_silenced {
        visual = Visual::Muted;
    }

    let chair_frame = build_chair_frame(&visual, &player.position.unwrap())?;
    let chair_content = build_chair_content(&player.clone())?;

    let centered_area = centered_area(chair_frame.inner(area), 2);

    frame.render_widget(chair_frame.clone(), area);
    frame.render_widget(chair_content, centered_area);
    Ok(())
}

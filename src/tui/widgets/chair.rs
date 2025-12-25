use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::snapshot::{ChairData, PlayerData, SeatData};

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

fn build_chair_frame(visual: &Visual, chair: &ChairData) -> Result<Block<'static>, anyhow::Error> {
    let position = chair.position;
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

fn build_chair_content(player: Option<PlayerData>) -> Result<Paragraph<'static>, anyhow::Error> {
    match player {
        Some(player) => {
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
                Line::from(vec![Span::styled(
                    player.role.clone().to_string(),
                    Style::default().fg(Color::Gray),
                )]),
            ];

            Ok(Paragraph::new(lines)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true }))
        }
        None => Ok(Paragraph::new("Empty Seat")
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })),
    }
}

pub fn draw_chair(frame: &mut Frame, area: Rect, seat: &SeatData) -> Result<(), anyhow::Error> {
    let visual = match &seat.player {
        Some(player) => match player.life_status.as_str() {
            "alive" => Visual::Alive,
            "dead" => Visual::Dead,
            "eliminated" => Visual::Eliminated,
            "removed" => Visual::Removed,
            "speaking" => Visual::Speaking,
            "muted" => Visual::Muted,
            "candidate" => Visual::Candidate,
            _ => Visual::Empty,
        },
        None => Visual::Empty,
    };
    let chair_frame = build_chair_frame(&visual, &seat.chair)?;
    let chair_content = build_chair_content(seat.player.clone())?;

    let centered_area = centered_area(chair_frame.inner(area), 2);

    frame.render_widget(chair_frame.clone(), area);
    frame.render_widget(chair_content, centered_area);
    Ok(())
}

use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders},
};

use crate::tui::layout;
use crate::{
    domain::{position::Position, status::Status},
    snapshot::Player,
    tui::view::PlayerView,
};

use super::player;

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

pub fn draw(
    frame: &mut Frame,
    chair: &layout::Chair,
    view: &PlayerView,
    player: &Player,
    actor: &Option<Position>,
) -> Result<(), anyhow::Error> {
    let visual = match view.status {
        Status::Alive => Visual::Alive,
        Status::Dead => Visual::Dead,
        Status::Removed => Visual::Removed,
        Status::Eliminated => Visual::Eliminated,
    };

    let chair_frame = build_chair_frame(&visual, &player.position.unwrap())?;

    frame.render_widget(chair_frame.clone(), chair.area);

    let player_layout = layout::Player::new(chair.area, 6);
    player::draw(frame, &player_layout, view);
    Ok(())
}

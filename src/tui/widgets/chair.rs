use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders},
};

use crate::tui::{
    layout,
    view::chair::{ChairState, ChairView},
};

use super::player;

fn build_chair_frame(view: &ChairView) -> Block<'static> {
    let pos = view.position.value();

    let (style, icon) = match view.state {
        ChairState::Empty => (
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::DIM),
            "⬜",
        ),
        ChairState::Alive => (Style::default().fg(Color::White), "💚"),
        ChairState::Dead => (Style::default().fg(Color::Red), "💀"),
        ChairState::Eliminated => (Style::default().fg(Color::Red), "❌"),
        ChairState::Removed => (Style::default().fg(Color::Red), "🚫"),
        ChairState::Speaking => (
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            "🗣️",
        ),
        ChairState::Muted => (
            Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
            "🤐",
        ),
        ChairState::Candidate => (Style::default().fg(Color::Magenta), "🎯"),
    };

    let border_style = if view.highlight {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style)
        .title(format!("Chair {pos} ({icon})"))
        .title_alignment(Alignment::Center)
        .style(style)
}

pub fn draw(frame: &mut Frame, chair: &layout::Chair, view: &ChairView) {
    let block = build_chair_frame(view);
    frame.render_widget(block, chair.area);

    if let Some(player_view) = &view.player {
        let player_layout = layout::Player::new(chair.area, 6);
        player::draw(frame, &player_layout, player_view);
    }
}

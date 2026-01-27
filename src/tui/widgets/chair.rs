use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::Span,
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
        ChairState::Dead => (
            Style::default().fg(Color::Red).add_modifier(Modifier::DIM),
            "💀",
        ),
        ChairState::Eliminated => (
            Style::default().fg(Color::Red).add_modifier(Modifier::DIM),
            "❌",
        ),
        ChairState::Removed => (
            Style::default().fg(Color::Red).add_modifier(Modifier::DIM),
            "🚫",
        ),
        ChairState::Speaking => (
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            "🗣️",
        ),
        ChairState::Muted => (Style::default().fg(Color::Gray), "🤐"),
        ChairState::RoleAssignment => (Style::default().fg(Color::Magenta), "🎭"),
        ChairState::Candidate => (Style::default().fg(Color::Blue), "🎯"),
    };

    let is_terminal = matches!(
        view.state,
        ChairState::Dead | ChairState::Eliminated | ChairState::Removed
    );

    let is_muted = matches!(view.state, ChairState::Muted);

    // 1️⃣ Base border style (color authority)
    let mut border_style = if is_terminal {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(view.border_style)
    };

    // 2️⃣ Modifiers (can stack)
    if is_muted {
        border_style = border_style.add_modifier(Modifier::DIM);
    }

    if view.highlight {
        border_style = border_style
            .add_modifier(Modifier::BOLD)
            .bg(border_style.fg.unwrap_or(Color::Green));
    }

    // 3️⃣ Title (ALWAYS reset bg)
    let title = Span::styled(
        format!(" {pos} ({icon}) "),
        Style::default()
            .fg(Color::White)
            .bg(Color::Reset)
            .add_modifier(if view.highlight {
                Modifier::BOLD
            } else {
                Modifier::empty()
            }),
    );
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style)
        .title(title)
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

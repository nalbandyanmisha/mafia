use crate::tui::layout;
use ratatui::{
    Frame,
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    domain::phase::{LobbyPhase, Phase},
    snapshot::App,
};

pub fn draw(frame: &mut Frame, lobby_area: &layout::Lobby, app: &App) -> Result<(), anyhow::Error> {
    let title = match app.engine.game.phase {
        Phase::Lobby(LobbyPhase::Waiting) => "Waiting",
        Phase::Lobby(LobbyPhase::Ready) => "Ready",
        _ => "Unknown",
    };
    let players = app.engine.game.players.clone();
    const PLAYER_COUNT: u8 = 10;

    let assigned_positions: Vec<u8> = players
        .iter()
        .filter_map(|p| p.position.map(|pos| pos.value()))
        .collect();

    let available_positions: Vec<u8> = (1..=PLAYER_COUNT)
        .filter(|p| !assigned_positions.contains(p))
        .collect();

    let ready =
        players.len() == PLAYER_COUNT as usize && players.iter().all(|p| p.position.is_some());

    let header = Paragraph::new(vec![
        Line::from(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
        Line::from(format!("{} / {} players", players.len(), PLAYER_COUNT)),
    ])
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, lobby_area.header);

    let mut lines = Vec::new();

    for player in &players {
        let pos = match player.position {
            Some(p) => p.value().to_string(),
            None => "—".to_string(),
        };

        lines.push(Line::from(vec![
            Span::raw(format!("{:<12}", player.name)),
            Span::raw(" position: "),
            Span::styled(pos, Style::default().add_modifier(Modifier::BOLD)),
        ]));
    }

    if players.is_empty() {
        lines.push(Line::from(Span::styled(
            "No players joined yet",
            Style::default().add_modifier(Modifier::ITALIC),
        )));
    }

    let player_list =
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Players"));

    frame.render_widget(player_list, lobby_area.body);

    let available = if available_positions.is_empty() {
        "none".to_string()
    } else {
        available_positions
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    };

    let status = if ready {
        Span::styled(
            "READY TO START",
            Style::default().add_modifier(Modifier::BOLD),
        )
    } else {
        Span::raw("Waiting for players")
    };

    let footer = Paragraph::new(vec![
        Line::from(format!("Available positions: {available}")),
        Line::from(status),
    ])
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, lobby_area.footer);
    Ok(())
}

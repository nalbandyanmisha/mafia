use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    domain::phase::{LobbyPhase, Phase},
    snapshot::App,
};

fn calculate_lobby_area(main_area: Rect) -> Rect {
    // Calculate host/control panel size
    let host_w = main_area.width / 3;
    let host_h = main_area.height / 3;

    // Calculate top-left corner to center it in table_area
    let host_x = main_area.x + (main_area.width - host_w) / 2;
    let host_y = main_area.y + (main_area.height - host_h) / 2;

    Rect {
        x: host_x,
        y: host_y,
        width: host_w,
        height: host_h,
    }
}

pub fn draw_lobby(frame: &mut Frame, lobby_area: Rect, app: &App) -> Result<(), anyhow::Error> {
    let lobby = calculate_lobby_area(lobby_area);
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

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(lobby);

    let header = Paragraph::new(vec![
        Line::from(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
        Line::from(format!("{} / {} players", players.len(), PLAYER_COUNT)),
    ])
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, layout[0]);

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

    frame.render_widget(player_list, layout[1]);

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

    frame.render_widget(footer, layout[2]);
    Ok(())
}

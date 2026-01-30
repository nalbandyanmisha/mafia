use crate::tui::util::centered_area;
use crate::tui::view::LobbyView;
use crate::{app::input::InputMode, tui::layout};
use ratatui::style::Color;
use ratatui::widgets::{BorderType, Clear};
use ratatui::{
    Frame,
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(
    frame: &mut Frame,
    lobby_area: &layout::Lobby,
    view: &LobbyView,
) -> Result<(), anyhow::Error> {
    // ===== Header =====
    let header = Paragraph::new(vec![
        Line::from(Span::styled(
            &view.title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
        Line::from(format!(
            "{} / {} players",
            view.player_count, view.max_players
        )),
    ])
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, lobby_area.header);

    // ===== Player list =====
    let mut lines = Vec::new();

    for player in &view.players {
        let pos = match player.position {
            Some(p) => p.to_string(),
            None => "—".to_string(),
        };

        lines.push(Line::from(vec![
            Span::raw(format!("{:<12}", player.name)),
            Span::raw(" position: "),
            Span::styled(pos, Style::default().add_modifier(Modifier::BOLD)),
        ]));
    }

    if view.players.is_empty() {
        lines.push(Line::from(Span::styled(
            "No players joined yet",
            Style::default().add_modifier(Modifier::ITALIC),
        )));
    }

    let player_list =
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Players"));

    frame.render_widget(player_list, lobby_area.body);

    // ===== Footer =====
    let available = if view.available_positions.is_empty() {
        "none".to_string()
    } else {
        view.available_positions
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    };

    let status = if view.ready {
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

    if let InputMode::Popup { title, kind: _ } = &view.input_mode {
        let area = centered_area(lobby_area.panel, 3);
        frame.render_widget(Clear, area);

        let popup_block = Block::default()
            .title(title.as_str())
            .borders(Borders::ALL)
            .border_style(Color::White)
            .border_type(BorderType::Thick);

        let paragraph = Paragraph::new(view.input.as_str())
            .block(popup_block)
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, area);
    }

    Ok(())
}

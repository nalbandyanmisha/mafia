use crate::{
    domain::phase::{LobbyPhase, Phase},
    snapshot::App,
};

#[derive(Debug, Clone)]
pub struct LobbyView {
    pub title: String,
    pub players: Vec<LobbyPlayerView>,
    pub player_count: usize,
    pub max_players: u8,
    pub available_positions: Vec<u8>,
    pub ready: bool,
}

#[derive(Debug, Clone)]
pub struct LobbyPlayerView {
    pub name: String,
    pub position: Option<u8>,
}

impl LobbyView {
    pub fn from_snapshot(app: &App) -> Self {
        let title = match app.engine.phase {
            Phase::Lobby(LobbyPhase::Waiting) => "Waiting",
            Phase::Lobby(LobbyPhase::Ready) => "Ready",
            _ => "Unknown",
        }
        .to_string();

        let players_vec: Vec<LobbyPlayerView> = app
            .engine
            .game
            .players
            .iter()
            .map(|p| LobbyPlayerView {
                name: p.name.clone(),
                position: p.position.map(|pos| pos.value()),
            })
            .collect();

        // assigned_positions borrows from players_vec, not moves it
        let player_count = players_vec.len();
        let assigned_positions: Vec<u8> = players_vec.iter().filter_map(|p| p.position).collect();

        const MAX_PLAYERS: u8 = 10;

        let available_positions: Vec<u8> = (1..=MAX_PLAYERS)
            .filter(|p| !assigned_positions.contains(p))
            .collect();

        let ready = players_vec.len() == MAX_PLAYERS as usize
            && players_vec.iter().all(|p| p.position.is_some());

        Self {
            title,
            players: players_vec,
            player_count,
            max_players: MAX_PLAYERS,
            available_positions,
            ready,
        }
    }
}

use crate::domain::{Position, Role, Status};
use crate::snapshot::{self, Voting};
#[derive(Debug, Clone)]
pub struct PlayerView {
    pub name: String,
    pub role: Option<Role>,
    pub warnings: u8,
    pub status: Status,
    pub is_nominated: bool,
    pub nominated: Option<Position>,
}

impl PlayerView {
    pub fn from_snapshot(position: Position, app: &snapshot::App) -> Self {
        let player = app
            .engine
            .game
            .players
            .iter()
            .find(|p| p.position == Some(position))
            .expect("Player at given position not found");

        let is_nominated = app
            .engine
            .game
            .voting
            .get(&app.engine.round)
            .cloned()
            .unwrap_or_else(Voting::default)
            .nominees
            .iter()
            .any(|n| n == &position);

        let nominated = app
            .engine
            .game
            .voting
            .get(&app.engine.round)
            .cloned()
            .unwrap_or_else(Voting::default)
            .nominations
            .iter()
            .find(|n| n.0 == &position)
            .map(|n| *n.1);

        Self {
            name: player.name.clone(),
            role: player.role,
            warnings: player.warnings,
            status: player.status,
            is_nominated,
            nominated,
        }
    }
}

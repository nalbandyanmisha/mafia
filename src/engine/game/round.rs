use crate::{
    domain::Position,
    engine::game::voting::Voting,
    snapshot::{self, Snapshot},
};
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Round {
    mafia_kill: Option<Position>,
    sheriff_check: Option<Position>,
    don_check: Option<Position>,
    eliminated: Vec<Position>,
    removed: Vec<Position>,
}

impl Snapshot for Round {
    type Output = snapshot::Round;

    fn snapshot(&self) -> Self::Output {
        snapshot::Round {
            mafia_kill: self.mafia_kill.map(|c| c.snapshot()),
            sheriff_check: self.sheriff_check.map(|c| c.snapshot()),
            don_check: self.don_check.map(|c| c.snapshot()),
            eliminated: self.eliminated.iter().map(|&c| c.snapshot()).collect(),
            removed: self.removed.iter().map(|&c| c.snapshot()).collect(),
        }
    }
}

impl Round {
    pub fn new() -> Self {
        Round {
            mafia_kill: None,
            sheriff_check: None,
            don_check: None,
            eliminated: Vec::new(),
            removed: Vec::new(),
        }
    }

    pub fn record_mafia_kill(&mut self, chair: Position) {
        self.mafia_kill = Some(chair);
    }

    pub fn record_elimination(&mut self, chair: Position) {
        self.eliminated.push(chair);
    }

    pub fn record_removal(&mut self, chair: Position) {
        self.removed.push(chair);
    }

    pub fn record_sheriff_check(&mut self, chair: Position) {
        self.sheriff_check = Some(chair);
    }

    pub fn record_don_check(&mut self, chair: Position) {
        self.don_check = Some(chair);
    }

    /* ---------------- Queries ---------------- */

    pub fn mafia_kill(&self) -> Option<Position> {
        self.mafia_kill
    }

    pub fn sheriff_check(&self) -> Option<Position> {
        self.sheriff_check
    }

    pub fn don_check(&self) -> Option<Position> {
        self.don_check
    }

    pub fn eliminated_players(&self) -> &[Position] {
        &self.eliminated
    }

    pub fn removed_players(&self) -> &[Position] {
        &self.removed
    }
}

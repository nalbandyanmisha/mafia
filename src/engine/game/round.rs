use crate::{
    domain::position::Position,
    engine::game::voting::Voting,
    snapshot::{self, Snapshot},
};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RoundId(pub usize);

impl std::fmt::Display for RoundId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl RoundId {
    pub fn next(&self) -> RoundId {
        RoundId(self.0 + 1)
    }
}

impl From<RoundId> for usize {
    fn from(round_id: RoundId) -> Self {
        round_id.0
    }
}

#[derive(Debug, Default, Clone)]
pub struct Round {
    pub voting: Option<Voting>,
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
            voting: self
                .voting
                .as_ref()
                .map(|v| v.snapshot())
                .unwrap_or(snapshot::Voting {
                    nominations: HashMap::new(),
                    nominees: Vec::new(),
                    votes: HashMap::new(),
                }),
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
            voting: Some(Voting::new()),
            mafia_kill: None,
            sheriff_check: None,
            don_check: None,
            eliminated: Vec::new(),
            removed: Vec::new(),
        }
    }

    /* ---------------- Recording ---------------- */

    pub fn record_nomination(&mut self, nominator: Position, nominee: Position) {
        self.voting
            .as_mut()
            .map(|v| v.record_nomination(nominator, nominee));
    }
    pub fn record_vote(&mut self, voter: Position, nominee: Position) {
        self.voting.as_mut().map(|v| v.record_vote(voter, nominee));
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

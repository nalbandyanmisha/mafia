mod voting;

use super::{actor::Actor, table::chair::Chair};
use crate::{
    engine::turn::Turn,
    snapshot::{RoundData, Snapshot, VotingData},
};
use std::collections::{HashMap, HashSet};

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
    pub voting: Option<voting::Voting>,
    mafia_kill: Option<Chair>,
    sheriff_check: Option<Chair>,
    don_check: Option<Chair>,
    eliminated: Vec<Chair>,
    removed: Vec<Chair>,
}

impl Snapshot for Round {
    type Output = RoundData;

    fn snapshot(&self) -> Self::Output {
        RoundData {
            voting: self
                .voting
                .as_ref()
                .map(|v| v.snapshot())
                .unwrap_or(VotingData {
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
            voting: Some(voting::Voting::new()),
            mafia_kill: None,
            sheriff_check: None,
            don_check: None,
            eliminated: Vec::new(),
            removed: Vec::new(),
        }
    }

    /* ---------------- Recording ---------------- */

    pub fn record_nomination(&mut self, nominator: Chair, nominee: Chair) {
        self.voting
            .as_mut()
            .map(|v| v.record_nomination(nominator, nominee));
    }
    pub fn record_vote(&mut self, voter: Chair, nominee: Chair) {
        self.voting.as_mut().map(|v| v.record_vote(voter, nominee));
    }

    pub fn record_mafia_kill(&mut self, chair: Chair) {
        self.mafia_kill = Some(chair);
    }

    pub fn record_elimination(&mut self, chair: Chair) {
        self.eliminated.push(chair);
    }

    pub fn record_removal(&mut self, chair: Chair) {
        self.removed.push(chair);
    }

    pub fn record_sheriff_check(&mut self, chair: Chair) {
        self.sheriff_check = Some(chair);
    }

    pub fn record_don_check(&mut self, chair: Chair) {
        self.don_check = Some(chair);
    }

    /* ---------------- Queries ---------------- */

    pub fn mafia_kill(&self) -> Option<Chair> {
        self.mafia_kill
    }

    pub fn sheriff_check(&self) -> Option<Chair> {
        self.sheriff_check
    }

    pub fn don_check(&self) -> Option<Chair> {
        self.don_check
    }

    pub fn eliminated_players(&self) -> &[Chair] {
        &self.eliminated
    }

    pub fn removed_players(&self) -> &[Chair] {
        &self.removed
    }
}

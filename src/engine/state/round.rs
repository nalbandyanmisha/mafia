mod nomination;
mod vote;

use super::{actor::Actor, table::chair::Chair};
use crate::{
    engine::turn::Turn,
    snapshot::{RoundData, Snapshot},
};
use nomination::Nomination;
use std::collections::{HashMap, HashSet};
use vote::Vote;
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
    nominations: Vec<Nomination>,
    votes: Vec<Vote>,
    night_kill: Option<Chair>,
    eliminated: Vec<Chair>,
    removed: Vec<Chair>,
}

impl Snapshot for Round {
    type Output = RoundData;

    fn snapshot(&self) -> Self::Output {
        RoundData
    }
}

impl Turn for Round {
    fn next_actor<F>(&self, actor: &mut Actor, is_eligible: F) -> Option<Chair>
    where
        F: Fn(Chair) -> bool,
    {
        let nominees = self
            .nominations()
            .iter()
            .map(|n| n.nominee())
            .collect::<Vec<_>>();
        if nominees.is_empty() || actor.is_completed() {
            return None;
        }

        let start_idx = actor
            .current()
            .and_then(|c| nominees.iter().position(|&x| x == c))
            .map(|i| i + 1)
            .unwrap_or(0);

        for i in start_idx..nominees.len() {
            let chair = nominees[i];
            if is_eligible(chair) {
                actor.set_current(Some(chair));
                return Some(chair);
            }
        }

        actor.set_completed(true);
        None
    }
}

impl Round {
    pub fn new() -> Self {
        Round {
            nominations: Vec::new(),
            votes: Vec::new(),
            night_kill: None,
            eliminated: Vec::new(),
            removed: Vec::new(),
        }
    }

    /* ---------------- Recording ---------------- */

    pub fn record_nomination(&mut self, nominator: Chair, nominee: Chair) {
        self.nominations.push(Nomination::new(nominator, nominee));
    }
    pub fn record_vote(&mut self, voter: Chair, nominee: Chair) {
        self.votes.push(Vote::new(voter, nominee));
    }

    pub fn record_night_kill(&mut self, chair: Chair) {
        self.night_kill = Some(chair);
    }

    pub fn record_elimination(&mut self, chair: Chair) {
        self.eliminated.push(chair);
    }

    pub fn record_removal(&mut self, chair: Chair) {
        self.removed.push(chair);
    }

    /* ---------------- Queries ---------------- */

    pub fn nominations(&self) -> &[Nomination] {
        &self.nominations
    }

    pub fn votes(&self) -> &[Vote] {
        &self.votes
    }

    pub fn night_kill(&self) -> Option<Chair> {
        self.night_kill
    }

    pub fn eliminated_players(&self) -> &[Chair] {
        &self.eliminated
    }

    pub fn removed_players(&self) -> &[Chair] {
        &self.removed
    }

    /// Unique nominees in order of appearance
    pub fn unique_nominees(&self) -> Vec<Chair> {
        let mut seen = HashSet::new();
        let mut result = Vec::new();

        for n in &self.nominations {
            let nominee = n.nominee();
            if seen.insert(nominee) {
                result.push(nominee);
            }
        }

        result
    }

    /// Count votes per nominee
    pub fn vote_tally(&self) -> HashMap<Chair, usize> {
        let mut tally = HashMap::new();
        for vote in &self.votes {
            *tally.entry(vote.nominee()).or_insert(0) += 1;
        }
        tally
    }

    /// Check if a chair has already nominated this round
    pub fn has_nominated(&self, chair: Chair) -> bool {
        self.nominations.iter().any(|n| n.nominator() == chair)
    }

    /// Check if a chair has already voted this round
    pub fn has_voted(&self, chair: Chair) -> bool {
        self.votes.iter().any(|v| v.voter() == chair)
    }
}

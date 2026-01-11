use std::collections::HashMap;

use crate::{
    domain::position::Position,
    engine::{Actor, Turn},
    snapshot::{self, Snapshot},
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Voting {
    nominations: HashMap<Position, Position>, // nominator -> nominee
    nominees: Vec<Position>,                  // ordered
    votes: HashMap<Position, Vec<Position>>,  // nominee -> voters
}

impl Snapshot for Voting {
    type Output = snapshot::Voting;

    fn snapshot(&self) -> Self::Output {
        snapshot::Voting {
            nominations: self
                .nominations
                .iter()
                .map(|(nominator, nominee)| (nominator.snapshot(), nominee.snapshot()))
                .collect(),
            nominees: self.nominees.iter().map(|n| n.snapshot()).collect(),
            votes: self
                .votes
                .iter()
                .map(|(nominee, voters)| {
                    (
                        nominee.snapshot(),
                        voters.iter().map(|v| v.snapshot()).collect(),
                    )
                })
                .collect(),
        }
    }
}

impl Turn for Voting {
    fn next_actor<F>(&self, actor: &mut Actor, is_eligible: F) -> Option<Position>
    where
        F: Fn(Position) -> bool,
    {
        if actor.is_completed() {
            return None;
        }

        if self.nominees.is_empty() {
            actor.mark_completed();
            return None;
        }

        let start = actor.start();

        // First call
        if actor.current().is_none() && is_eligible(start) {
            actor.set_current(Some(start));
            return Some(start);
        }

        let current = actor.current().unwrap_or(start);
        let start_idx = self.nominees.iter().position(|&p| p == current)?;

        for i in 1..=self.nominees.len() {
            let idx = (start_idx + i) % self.nominees.len();
            let pos = self.nominees[idx];

            if !is_eligible(pos) {
                continue;
            }

            // looped back → finished
            if pos == start && actor.current().is_some() {
                actor.mark_completed();
                return None;
            }

            actor.set_current(Some(pos));
            return Some(pos);
        }

        actor.mark_completed();
        None
    }
}

impl Voting {
    pub fn new() -> Self {
        Voting {
            nominations: HashMap::new(),
            nominees: Vec::new(),
            votes: HashMap::new(),
        }
    }

    pub fn from_nominees(nominees: &[Position]) -> Self {
        Self {
            nominations: HashMap::new(),
            nominees: nominees.to_vec(),
            votes: HashMap::new(),
        }
    }

    pub fn record_nomination(&mut self, nominator: Position, nominee: Position) {
        if self.nominations.contains_key(&nominator) {
            return;
        }
        self.nominations.entry(nominator).or_insert(nominee);
        if !self.nominees.contains(&nominee) {
            self.nominees.push(nominee);
        }
    }

    pub fn record_vote(&mut self, voter: Position, nominee: Position) {
        let voters = self.votes.entry(nominee).or_default();
        if !voters.contains(&voter) {
            voters.push(voter);
        }
    }

    pub fn compute_vote_results(&self) -> HashMap<Position, usize> {
        let mut results: HashMap<Position, usize> = HashMap::new();
        for (nominee, voters) in self.votes.iter() {
            results.insert(*nominee, voters.len());
        }
        results
    }

    pub fn winners(&self) -> Vec<Position> {
        let results = self.compute_vote_results();
        if results.is_empty() {
            return vec![];
        }

        let max = results.values().copied().max().unwrap_or(0);

        results
            .into_iter()
            .filter(|(_, c)| *c == max)
            .map(|(p, _)| p)
            .collect()
    }

    pub fn has_nominees(&self) -> bool {
        self.nominees.len() > 0
    }

    pub fn nominee_count(&self) -> usize {
        self.nominees.len()
    }

    pub fn get_nominations(&self) -> &HashMap<Position, Position> {
        &self.nominations
    }

    pub fn get_nominees(&self) -> &[Position] {
        &self.nominees
    }

    pub fn get_votes(&self) -> &HashMap<Position, Vec<Position>> {
        &self.votes
    }
}

use std::collections::HashMap;

use crate::{
    domain::position::Position,
    engine::{Actor, Turn},
    snapshot::{self, Snapshot},
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Voting {
    // nomination phase
    nominations: HashMap<Position, Position>, // nominator -> nominee
    nominees: Vec<Position>,                  // ordered

    // first voting round
    votes: HashMap<Position, Vec<Position>>, // nominee -> voters

    // tie handling
    tie_nominees: Vec<Position>, // derived
    tie_votes: HashMap<Position, Vec<Position>>,

    // final “eliminate all?” vote
    eliminate_each_votes: Vec<Position>,
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
        if self.nominees.is_empty() || actor.is_completed() {
            return None;
        }

        let start_idx = actor
            .current()
            .and_then(|c| self.nominees.iter().position(|&x| x == c))
            .map(|i| i + 1)
            .unwrap_or(0);

        for i in start_idx..self.nominees.len() {
            let chair = self.nominees[i];
            if is_eligible(chair) {
                actor.set_current(Some(chair));
                return Some(chair);
            }
        }

        actor.set_completed(true);
        None
    }
}

impl Voting {
    pub fn new() -> Self {
        Voting {
            nominations: HashMap::new(),
            nominees: Vec::new(),
            votes: HashMap::new(),
            tie_nominees: Vec::new(),
            tie_votes: HashMap::new(),
            eliminate_each_votes: Vec::new(),
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
        self.votes.entry(nominee).or_default().push(voter);
    }

    pub fn compute_vote_results(&self) -> HashMap<Position, usize> {
        let mut results: HashMap<Position, usize> = HashMap::new();
        for (nominee, voters) in self.votes.iter() {
            results.insert(*nominee, voters.len());
        }
        results
    }

    pub fn compute_tie_vote_results(&self) -> HashMap<Position, usize> {
        let mut results: HashMap<Position, usize> = HashMap::new();
        for (nominee, voters) in self.tie_votes.iter() {
            results.insert(*nominee, voters.len());
        }
        results
    }

    pub fn clear_actor_state(&mut self, actor: &mut Actor) {
        actor.set_current(None);
        actor.set_completed(false);
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

    pub fn tie_nominees(&self) -> &[Position] {
        &self.tie_nominees
    }

    pub fn resolve_initial_vote(&mut self) -> Vec<Position> {
        let winners = self.winners();
        if winners.len() > 1 {
            self.tie_nominees = winners.clone();
        }
        winners
    }

    pub fn resolve_tie_vote(&self) -> Vec<Position> {
        let results = self.compute_tie_vote_results();
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

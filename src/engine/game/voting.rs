use std::collections::HashMap;

use crate::{
    domain::{Phase, RoundId, position::Position},
    engine::{Actor, Turn, TurnContext},
    snapshot::{self, Snapshot},
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Voting {
    /// Who nominated whom
    /// nominator -> nominee
    nominations: HashMap<Position, Position>,
    nominees: Vec<Position>,
    votes: HashMap<Position, Vec<Position>>, // nominee -> voters
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

    fn turn_context(&self, round: RoundId, _phase: Phase, actor: &Actor) -> Option<TurnContext> {
        if self.nominees.is_empty() || actor.is_completed() {
            return None;
        }

        // Determine which nominee is active
        if let Some(current) = actor.current() {
            Some(TurnContext::VoteCasting)
        } else {
            Some(TurnContext::VotingDiscussion)
        }
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

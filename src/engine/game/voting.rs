use std::{collections::HashMap, fmt};

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

#[derive(Debug, Clone)]
pub enum Event {
    Nominated {
        nominator: Position,
        nominee: Position,
    },
    Voted {
        voter: Position,
        nominee: Position,
    },
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::Nominated { nominator, nominee } => {
                write!(f, "Player at position {nominator} has nominated {nominee}")
            }
            Event::Voted { voter, nominee } => {
                write!(f, "Player at position {voter} has voted for {nominee}")
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Nominator {0:?} has already made a nomination")]
    NominationAlreadyExists(Position),

    #[error("Voter {0:?} has already voted for {1:?}")]
    AlreadyVoted(Position, Position),

    #[error("Nominee {0:?} is not in the nominee list")]
    InvalidNominee(Position),
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

    pub fn has_nominees(&self) -> bool {
        !self.nominees.is_empty()
    }

    pub fn nominee_count(&self) -> usize {
        self.nominees.len()
    }

    pub fn get_nominees(&self) -> &[Position] {
        &self.nominees
    }

    pub fn nominate(
        &mut self,
        nominator: Position,
        nominee: Position,
    ) -> Result<Vec<Event>, Error> {
        if self.nominations.contains_key(&nominator) {
            return Err(Error::NominationAlreadyExists(nominator));
        }

        self.nominations.insert(nominator, nominee);
        if !self.nominees.contains(&nominee) {
            self.nominees.push(nominee);
        }

        Ok(vec![Event::Nominated { nominator, nominee }])
    }

    pub fn vote(&mut self, voter: Position, nominee: Position) -> Result<Vec<Event>, Error> {
        if !self.nominees.contains(&nominee) {
            return Err(Error::InvalidNominee(nominee));
        }

        let voters = self.votes.entry(nominee).or_default();

        if voters.contains(&voter) {
            return Err(Error::AlreadyVoted(voter, nominee));
        }

        voters.push(voter);

        Ok(vec![Event::Voted { voter, nominee }])
    }

    pub fn winners(&self) -> Vec<Position> {
        let results: HashMap<Position, usize> = self
            .votes
            .iter()
            .map(|(nominee, voters)| (*nominee, voters.len()))
            .collect();

        if results.is_empty() {
            return vec![];
        }

        let max = results.values().copied().max().unwrap_or(0);

        results
            .into_iter()
            .filter(|(_, count)| *count == max)
            .map(|(p, _)| p)
            .collect()
    }
}

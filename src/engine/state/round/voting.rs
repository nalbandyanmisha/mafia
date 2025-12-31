use std::collections::HashMap;

use crate::{
    engine::{
        state::{actor::Actor, table::chair::Chair},
        turn::Turn,
    },
    snapshot::{Snapshot, VotingData},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Voting {
    /// Who nominated whom
    /// nominator -> nominee
    pub nominations: HashMap<Chair, Chair>,
    pub nominees: Vec<Chair>,
    pub votes: HashMap<Chair, Vec<Chair>>, // nominee -> voters
}

impl Snapshot for Voting {
    type Output = VotingData;

    fn snapshot(&self) -> Self::Output {
        VotingData {
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
    fn next_actor<F>(&self, actor: &mut Actor, is_eligible: F) -> Option<Chair>
    where
        F: Fn(Chair) -> bool,
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
        }
    }

    pub fn record_nomination(&mut self, nominator: Chair, nominee: Chair) {
        if !self.nominations.contains_key(&nominator) {
            self.nominations.insert(nominator, nominee);
        }
        if !self.nominees.contains(&nominee) {
            self.nominees.push(nominee);
        }
    }

    pub fn record_vote(&mut self, voter: Chair, nominee: Chair) {
        self.votes
            .entry(nominee)
            .or_insert_with(Vec::new)
            .push(voter);
    }

    pub fn get_nominations(&self) -> &HashMap<Chair, Chair> {
        &self.nominations
    }

    pub fn get_nominees(&self) -> &[Chair] {
        &self.nominees
    }

    pub fn get_votes(&self) -> &HashMap<Chair, Vec<Chair>> {
        &self.votes
    }
}

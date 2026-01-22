use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use crate::{
    domain::position::Position,
    engine::{Actor, Turn},
    snapshot::{self, Snapshot},
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Voting {
    nominations: HashMap<Position, Position>, // nominator -> nominee
    nominees: Vec<Position>,                  // ordered
    voters: HashSet<Position>,
    votes: HashMap<Position, Vec<Position>>, // nominee -> voters
}

#[derive(Debug, Clone, Serialize)]
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

    #[error("Nominee {0:?} is not in the nominee list")]
    InvalidNominator(Position),

    #[error("Voter {0:?} is not allowed to vote")]
    InvalidVoter(Position),
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
    pub fn new(voters: HashSet<Position>) -> Self {
        Voting {
            nominations: HashMap::new(),
            nominees: Vec::new(),
            voters,
            votes: HashMap::new(),
        }
    }

    pub fn from_nominees(nominees: &[Position], voters: HashSet<Position>) -> Self {
        Self {
            nominations: HashMap::new(),
            nominees: nominees.to_vec(),
            voters,
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

    pub fn is_eligible(&self, pos: Position) -> bool {
        self.voters.contains(&pos)
    }

    pub fn nominate(
        &mut self,
        nominator: Position,
        nominee: Position,
    ) -> Result<Vec<Event>, Error> {
        if !self.voters.contains(&nominator) {
            return Err(Error::InvalidNominator(nominator));
        }

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
        if self.votes.values().any(|voters| voters.contains(&voter)) {
            return Err(Error::AlreadyVoted(voter, nominee));
        }

        if !self.voters.contains(&voter) {
            return Err(Error::InvalidVoter(voter)); // or NotEligible
        }

        if !self.nominees.contains(&nominee) {
            return Err(Error::InvalidNominee(nominee));
        }

        self.votes.entry(nominee).or_default().push(voter);

        // mark voter as used
        self.voters.remove(&voter);

        Ok(vec![Event::Voted { voter, nominee }])
    }

    pub fn batch_vote(
        &mut self,
        nominee: Position,
        voters: &[Position],
        strict: bool,
    ) -> Result<Vec<Event>, Error> {
        if !self.nominees.contains(&nominee) {
            return Err(Error::InvalidNominee(nominee));
        }

        let mut events = Vec::new();
        let mut seen_in_batch = HashSet::new();

        for &voter in voters {
            // Skip duplicate in batch
            if !seen_in_batch.insert(voter) {
                if strict {
                    return Err(Error::AlreadyVoted(voter, nominee));
                } else {
                    continue;
                }
            }

            // Check eligibility
            if !self.is_eligible(voter) {
                if strict {
                    // Determine correct error
                    return if self.votes.values().any(|v| v.contains(&voter)) {
                        Err(Error::AlreadyVoted(voter, nominee))
                    } else {
                        Err(Error::InvalidVoter(voter))
                    };
                } else {
                    continue;
                }
            }

            events.append(&mut self.vote(voter, nominee)?);
        }

        Ok(events)
    }

    pub fn winners(&self) -> Vec<Position> {
        if self.votes.is_empty() {
            return Vec::new();
        }

        // Count votes per nominee
        let mut counts: HashMap<Position, usize> = HashMap::new();
        for (nominee, voters) in &self.votes {
            counts.insert(*nominee, voters.len());
        }

        // Find maximum vote count
        let max = counts.values().copied().max().unwrap_or(0);

        // Preserve nomination order
        self.nominees
            .iter()
            .copied()
            .filter(|nominee| counts.get(nominee).copied().unwrap_or(0) == max)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::position::Position;

    fn pos(n: u8) -> Position {
        Position::new(n)
    }

    fn create_voters(n: u8) -> HashSet<Position> {
        (1..=n).map(Position::new).collect()
    }

    #[test]
    fn nomination_adds_nominee_only_once() {
        let voters = create_voters(10);
        let nominator_a = pos(1);
        let nominator_b = pos(2);
        let nominee = pos(3);

        let mut voting = Voting::new(voters);

        voting.nominate(nominator_a, nominee).unwrap();
        voting.nominate(nominator_b, nominee).unwrap();

        assert_eq!(voting.nominees, vec![nominee]);
    }

    #[test]
    fn same_nominator_cannot_nominate_twice() {
        let voters = create_voters(10);
        let nominator = pos(1);
        let first_nominee = pos(3);
        let second_nominee = pos(4);

        let mut voting = Voting::new(voters);

        voting.nominate(nominator, first_nominee).unwrap();
        let err = voting.nominate(nominator, second_nominee).unwrap_err();

        assert!(matches!(err, Error::NominationAlreadyExists(_)));
    }

    #[test]
    fn nomination_by_non_voter_fails() {
        let voters = create_voters(2);
        let mut voting = Voting::new(voters);

        let non_voter = pos(5);
        let nominee = pos(3);

        let err = voting.nominate(non_voter, nominee).unwrap_err();
        assert!(matches!(err, Error::InvalidNominator(_))); // Using current code error
    }

    #[test]
    fn vote_is_recorded_for_nominee() {
        let voters = create_voters(10);
        let voter = pos(1);
        let nominee = pos(3);

        let mut voting = Voting::from_nominees(&[nominee], voters);
        voting.vote(voter, nominee).unwrap();

        let votes = voting.votes.get(&nominee).unwrap();
        assert_eq!(votes, &vec![voter]);
    }

    #[test]
    fn voter_cannot_vote_twice_for_same_nominee() {
        let voters = create_voters(10);
        let voter = pos(1);
        let nominee = pos(3);

        let mut voting = Voting::from_nominees(&[nominee], voters);

        voting.vote(voter, nominee).unwrap();
        let err = voting.vote(voter, nominee).unwrap_err();
        assert!(matches!(err, Error::AlreadyVoted(_, _)));
    }

    #[test]
    fn voter_cannot_vote_for_multiple_nominees() {
        let voters = create_voters(10);
        let voter = pos(1);
        let nominee_a = pos(3);
        let nominee_b = pos(4);

        let mut voting = Voting::from_nominees(&[nominee_a, nominee_b], voters);

        voting.vote(voter, nominee_a).unwrap();
        let err = voting.vote(voter, nominee_b).unwrap_err();
        assert!(matches!(err, Error::AlreadyVoted(_, _)));
    }

    #[test]
    fn voting_by_non_eligible_fails() {
        let voters = create_voters(1);
        let voter = pos(2);
        let nominee = pos(3);

        let mut voting = Voting::from_nominees(&[nominee], voters);

        let err = voting.vote(voter, nominee).unwrap_err();
        assert!(matches!(err, Error::InvalidVoter(_)));
    }

    #[test]
    fn cannot_vote_for_non_nominee() {
        let voters = create_voters(10);
        let voter = pos(1);
        let nominee = pos(3);
        let non_nominee = pos(4);

        let mut voting = Voting::from_nominees(&[nominee], voters);

        let err = voting.vote(voter, non_nominee).unwrap_err();
        assert!(matches!(err, Error::InvalidNominee(_)));
    }

    #[test]
    fn multiple_voters_can_vote_for_same_nominee() {
        let voters = create_voters(10);
        let voter_a = pos(1);
        let voter_b = pos(2);
        let nominee = pos(3);

        let mut voting = Voting::from_nominees(&[nominee], voters);

        voting.vote(voter_a, nominee).unwrap();
        voting.vote(voter_b, nominee).unwrap();

        let votes = voting.votes.get(&nominee).unwrap();
        assert_eq!(votes, &vec![voter_a, voter_b]);
    }

    #[test]
    fn voter_removed_after_vote() {
        let voters = create_voters(10);
        let voter = pos(1);
        let nominee = pos(3);

        let mut voting = Voting::from_nominees(&[nominee], voters.clone());
        voting.vote(voter, nominee).unwrap();

        assert!(!voting.is_eligible(voter));
        assert!(voting.is_eligible(pos(2)));
    }

    #[test]
    fn voting_with_no_nominees_fails() {
        let voters = create_voters(10);
        let voter = pos(1);
        let nominee = pos(3);

        let mut voting = Voting::new(voters);

        let err = voting.vote(voter, nominee).unwrap_err();
        assert!(matches!(err, Error::InvalidNominee(_)));
    }

    #[test]
    fn winner_is_nominee_with_most_votes() {
        let voters = create_voters(10);
        let voter_a = pos(1);
        let voter_b = pos(2);
        let voter_c = pos(3);

        let nominee_a = pos(4);
        let nominee_b = pos(5);

        let mut voting = Voting::from_nominees(&[nominee_a, nominee_b], voters);

        voting.vote(voter_a, nominee_a).unwrap();
        voting.vote(voter_b, nominee_a).unwrap();
        voting.vote(voter_c, nominee_b).unwrap();

        let winners = voting.winners();
        assert_eq!(winners, vec![nominee_a]);
    }

    #[test]
    fn winners_returns_all_nominees_in_case_of_tie() {
        let voters = create_voters(10);
        let voter_a = pos(1);
        let voter_b = pos(2);

        let nominee_a = pos(3);
        let nominee_b = pos(4);

        let mut voting = Voting::from_nominees(&[nominee_a, nominee_b], voters);

        voting.vote(voter_a, nominee_a).unwrap();
        voting.vote(voter_b, nominee_b).unwrap();

        let mut winners = voting.winners();
        winners.sort();

        assert_eq!(winners, vec![nominee_a, nominee_b]);
    }

    #[test]
    fn winners_empty_when_no_votes_cast() {
        let voters = create_voters(10);
        let nominee = pos(3);

        let voting = Voting::from_nominees(&[nominee], voters);

        let winners = voting.winners();
        assert!(winners.is_empty());
    }

    #[test]
    fn winners_with_some_nominees_no_votes() {
        let voters = create_voters(10);
        let voter = pos(1);
        let nominee_with_votes = pos(3);
        let nominee_no_votes = pos(4);

        let mut voting = Voting::from_nominees(&[nominee_with_votes, nominee_no_votes], voters);

        voting.vote(voter, nominee_with_votes).unwrap();

        let winners = voting.winners();
        assert_eq!(winners, vec![nominee_with_votes]);
        assert!(!winners.contains(&nominee_no_votes));
    }

    #[test]
    fn winners_tie_preserves_nomination_order() {
        let voters = create_voters(10);
        let voter_a = pos(1);
        let voter_b = pos(2);

        let nominee_first = pos(3);
        let nominee_second = pos(4);

        let mut voting = Voting::from_nominees(&[nominee_first, nominee_second], voters);

        voting.vote(voter_a, nominee_first).unwrap();
        voting.vote(voter_b, nominee_second).unwrap();

        let winners = voting.winners();

        assert_eq!(
            winners,
            vec![nominee_first, nominee_second],
            "Tie winners must be returned in nomination order"
        );
    }

    #[test]
    fn batch_vote_success() {
        let voters = create_voters(5);
        let nominee = voters
            .get(&3.into())
            .expect("Position Must exist, this should never fail");
        let mut voting = Voting::from_nominees(&[pos(3), pos(4)], voters.clone());

        let batch_voters = &[pos(1), pos(2), pos(3)];

        let events = voting.batch_vote(*nominee, batch_voters, false).unwrap();

        // All events returned
        assert_eq!(events.len(), 3);
        assert!(events.iter().all(|e| matches!(e, Event::Voted { .. })));

        // Check votes recorded correctly
        let votes_for_3 = voting.votes.get(nominee).unwrap();
        assert_eq!(votes_for_3.len(), 3);
        assert!(votes_for_3.contains(&pos(1)));
        assert!(votes_for_3.contains(&pos(2)));
        assert!(votes_for_3.contains(&pos(3)));

        // Check voters removed from eligibility
        assert!(!voting.is_eligible(pos(1)));
        assert!(!voting.is_eligible(pos(2)));
        assert!(!voting.is_eligible(pos(3)));
        assert!(voting.is_eligible(pos(4)));
        assert!(voting.is_eligible(pos(5)));
    }

    #[test]
    fn batch_vote_fails_for_invalid_nominee() {
        let voters = create_voters(3);

        let mut voting = Voting::from_nominees(&[pos(3)], voters.clone());

        let batch_voters = &[pos(1), pos(2)];

        let err = voting.batch_vote(pos(4), batch_voters, false).unwrap_err(); // pos(4) not a nominee
        assert!(matches!(err, Error::InvalidNominee(_)));

        // Ensure no votes applied
        assert!(!voting.votes.contains_key(&pos(4)));
    }

    #[test]
    fn batch_vote_with_duplicate_voters_is_ignored() {
        let voters = (1..=3).map(Position::new).collect::<HashSet<_>>();
        let mut voting = Voting::from_nominees(&[pos(3)], voters.clone());

        // voter 1 appears twice
        let batch_voters = &[pos(1), pos(1), pos(2)];

        let events = voting.batch_vote(pos(3), batch_voters, false).unwrap();

        // Only two events recorded (voter 1 and voter 2)
        assert_eq!(events.len(), 2);
        assert!(
            events
                .iter()
                .any(|e| matches!(e, Event::Voted { voter, .. } if *voter == pos(1)))
        );
        assert!(
            events
                .iter()
                .any(|e| matches!(e, Event::Voted { voter, .. } if *voter == pos(2)))
        );

        // Eligibility updated
        assert!(!voting.is_eligible(pos(1)));
        assert!(!voting.is_eligible(pos(2)));
        assert!(voting.is_eligible(pos(3))); // not in batch

        // Votes recorded correctly
        let votes_for_3 = voting.votes.get(&pos(3)).unwrap();
        assert_eq!(votes_for_3.len(), 2);
        assert!(votes_for_3.contains(&pos(1)));
        assert!(votes_for_3.contains(&pos(2)));
    }

    #[test]
    fn batch_vote_normal_mode_skips_duplicates_and_already_voted() {
        let voters = (1..=5).map(Position::new).collect::<HashSet<_>>();
        let nominee = pos(10);
        let mut voting = Voting::from_nominees(&[nominee], voters.clone());

        // voter 1 votes individually
        voting.vote(pos(1), nominee).unwrap();

        // batch: includes duplicate 2, 3, already voted 1, and 4
        let batch_voters = vec![pos(1), pos(2), pos(2), pos(3), pos(4)];
        let events = voting.batch_vote(nominee, &batch_voters, false).unwrap();

        // Only valid voters 2, 3, 4 are processed once
        let voters_who_voted = voting.votes.get(&nominee).unwrap();
        assert_eq!(voters_who_voted.len(), 4);
        assert!(voters_who_voted.contains(&pos(1)));
        assert!(voters_who_voted.contains(&pos(2)));
        assert!(voters_who_voted.contains(&pos(3)));
        assert!(voters_who_voted.contains(&pos(4)));

        // Duplicate and already voted positions are ignored, no panic
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn batch_vote_strict_mode_fails_on_first_ineligible() {
        let voters = (1..=3).map(Position::new).collect::<HashSet<_>>();
        let nominee = pos(10);
        let mut voting = Voting::from_nominees(&[nominee], voters.clone());

        // voter 1 votes individually
        voting.vote(pos(1), nominee).unwrap();

        // strict batch: includes already voted voter 1
        let batch_voters = vec![pos(1), pos(2), pos(3)];

        let err = voting.batch_vote(nominee, &batch_voters, true).unwrap_err();
        assert!(matches!(err, Error::AlreadyVoted(_, _)));

        // Only the initial vote exists, batch failed entirely
        let voters_who_voted = voting.votes.get(&nominee).unwrap();
        assert_eq!(voters_who_voted.len(), 1);
        assert!(voters_who_voted.contains(&pos(1)));
    }

    #[test]
    fn batch_vote_normal_mode_skips_duplicates_and_invalid() {
        let voters = create_voters(5);
        let voter_1 = pos(1);
        let voter_2 = pos(2);
        let voter_3 = pos(3);
        let voter_4 = pos(4);
        let voter_5 = pos(5);
        let nominee = pos(10);

        let mut voting = Voting::from_nominees(&[nominee], voters.clone());

        // voter_1 votes individually
        voting.vote(voter_1, nominee).unwrap();

        // batch includes duplicate voter_2, voter_3, already voted voter_1, and voter_4
        let batch = vec![voter_1, voter_2, voter_2, voter_3, voter_4];
        let events = voting.batch_vote(nominee, &batch, false).unwrap();

        let votes_for_nominee = voting.votes.get(&nominee).unwrap();
        assert_eq!(votes_for_nominee.len(), 4);
        assert!(votes_for_nominee.contains(&voter_1));
        assert!(votes_for_nominee.contains(&voter_2));
        assert!(votes_for_nominee.contains(&voter_3));
        assert!(votes_for_nominee.contains(&voter_4));

        assert_eq!(events.len(), 3); // only valid new votes produce events

        // eligibility
        assert!(!voting.is_eligible(voter_1));
        assert!(!voting.is_eligible(voter_2));
        assert!(!voting.is_eligible(voter_3));
        assert!(!voting.is_eligible(voter_4));
        assert!(voting.is_eligible(voter_5));
    }

    #[test]
    fn batch_vote_strict_mode_fails_on_first_invalid() {
        let voters = create_voters(3);
        let voter_1 = pos(1);
        let voter_2 = pos(2);
        let voter_3 = pos(3);
        let nominee = pos(10);

        let mut voting = Voting::from_nominees(&[nominee], voters.clone());

        voting.vote(voter_1, nominee).unwrap();

        let batch = vec![voter_1, voter_2, voter_3];

        let err = voting.batch_vote(nominee, &batch, true).unwrap_err();
        assert!(matches!(err, Error::AlreadyVoted(voter_1, _)));

        // Only the first vote exists
        let votes_for_nominee = voting.votes.get(&nominee).unwrap();
        assert_eq!(votes_for_nominee.len(), 1);
        assert!(votes_for_nominee.contains(&voter_1));
    }

    #[test]
    fn batch_vote_fails_for_invalid_voter_strict() {
        let voters = create_voters(3);
        let voter_1 = pos(1);
        let voter_2 = pos(2);
        let invalid_voter = pos(4);
        let nominee = pos(3);

        let mut voting = Voting::from_nominees(&[nominee], voters.clone());

        let batch = vec![voter_1, invalid_voter, voter_2];

        let err = voting.batch_vote(nominee, &batch, true).unwrap_err();
        assert!(matches!(err, Error::InvalidVoter(v) if v == invalid_voter));

        let votes_for_nominee = voting.votes.get(&nominee).unwrap();
        assert_eq!(votes_for_nominee.len(), 1); // only voter_1 applied
        assert!(votes_for_nominee.contains(&voter_1));
    }

    #[test]
    fn batch_vote_empty_returns_ok() {
        let voters = create_voters(3);
        let nominee = pos(10);
        let mut voting = Voting::from_nominees(&[nominee], voters);

        let events = voting.batch_vote(nominee, &[], false).unwrap();
        assert!(events.is_empty());
        assert!(!voting.votes.contains_key(&nominee));
    }

    #[test]
    fn batch_vote_with_duplicate_voters_ignored() {
        let voters = create_voters(3);
        let voter_1 = pos(1);
        let voter_2 = pos(2);
        let voter_3 = pos(3);
        let nominee = pos(3);

        let mut voting = Voting::from_nominees(&[nominee], voters);

        let batch = vec![voter_1, voter_1, voter_2];
        let events = voting.batch_vote(nominee, &batch, false).unwrap();

        assert_eq!(events.len(), 2);
        let votes_for_nominee = voting.votes.get(&nominee).unwrap();
        assert_eq!(votes_for_nominee.len(), 2);
        assert!(votes_for_nominee.contains(&voter_1));
        assert!(votes_for_nominee.contains(&voter_2));

        assert!(!voting.is_eligible(voter_1));
        assert!(!voting.is_eligible(voter_2));
        assert!(voting.is_eligible(voter_3));
    }

    #[test]
    fn batch_vote_invalid_nominee_fails() {
        let voters = create_voters(3);
        let voter_1 = pos(1);
        let voter_2 = pos(2);
        let nominee_invalid = pos(10);

        let mut voting = Voting::from_nominees(&[pos(3)], voters);

        let batch = vec![voter_1, voter_2];

        let err = voting
            .batch_vote(nominee_invalid, &batch, false)
            .unwrap_err();
        assert!(matches!(err, Error::InvalidNominee(n) if n == nominee_invalid));
    }
}

use crate::domain::{phase::Phase, position::Position, role::Role, status::Status};
use std::collections::HashMap;

pub trait Snapshot {
    type Output;

    fn snapshot(&self) -> Self::Output;
}

#[derive(Clone, Debug)]
pub struct Player {
    pub name: String,
    pub position: Option<Position>,
    pub role: Option<Role>,
    pub warnings: u8,
    pub is_silenced: bool,
    pub status: Status,
}

#[derive(Clone, Debug)]
pub struct Voting {
    pub nominations: HashMap<Position, Position>,
    pub nominees: Vec<Position>,
    pub votes: HashMap<Position, Vec<Position>>,
}

#[derive(Clone, Debug)]
pub struct Round {
    pub voting: Voting,
    pub mafia_kill: Option<Position>,
    pub sheriff_check: Option<Position>,
    pub don_check: Option<Position>,
    pub eliminated: Vec<Position>,
    pub removed: Vec<Position>,
}

impl Round {
    pub fn is_nominated(&self, pos: Position) -> bool {
        self.voting.nominees.contains(&pos)
    }

    pub fn nominated_by(&self, pos: Position) -> Option<Position> {
        self.voting.nominations.iter().find_map(
            |(by, target)| {
                if *target == pos { Some(*by) } else { None }
            },
        )
    }

    pub fn votes_received(&self, pos: Position) -> Vec<Position> {
        self.voting
            .votes
            .iter()
            .filter_map(|(target, voters)| {
                if *target == pos {
                    Some(voters.clone())
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }

    pub fn voted_for(&self, voter: Position) -> Option<Position> {
        self.voting.votes.iter().find_map(|(target, voters)| {
            if voters.contains(&voter) {
                Some(*target)
            } else {
                None
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub phase: Phase,
    pub round: Round,
    pub current_round: usize,
}

#[derive(Clone, Debug)]
pub struct Engine {
    pub game: Game,
    pub actor: Option<Position>,
}

#[derive(Clone, Debug)]
pub struct App {
    pub engine: Engine,
    pub input: String,
    pub current_timer: Option<u64>,
}

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

#[derive(Default, Clone, Debug)]
pub struct Voting {
    pub nominations: HashMap<Position, Position>,
    pub nominees: Vec<Position>,
    pub votes: HashMap<Position, Vec<Position>>,
}

#[derive(Clone, Debug)]
pub struct Round {
    pub mafia_kill: Option<Position>,
    pub sheriff_check: Option<Position>,
    pub don_check: Option<Position>,
    pub eliminated: Vec<Position>,
    pub removed: Vec<Position>,
}

#[derive(Clone, Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub round_new: usize,
    pub voting: HashMap<usize, Voting>,
    pub round: Round,
    pub current_round: usize,
}

#[derive(Clone, Debug)]
pub struct Engine {
    pub game: Game,
    pub phase: Phase,
    pub round: usize,
    pub actor: Option<Position>,
}

#[derive(Clone, Debug)]
pub struct App {
    pub engine: Engine,
    pub input: String,
    pub current_timer: Option<u64>,
}

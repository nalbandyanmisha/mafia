use crate::domain::{Activity, EngineState, Position, Role, Status};
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
    pub status: Status,
}

#[derive(Clone, Default, Debug)]
pub struct Check {
    pub sheriff: Option<Position>,
    pub don: Option<Position>,
}

#[derive(Default, Clone, Debug)]
pub struct Voting {
    pub nominations: HashMap<Position, Position>,
    pub nominees: Vec<Position>,
    pub votes: HashMap<Position, Vec<Position>>,
}

#[derive(Clone, Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub kill: HashMap<usize, Position>,
    pub voting: HashMap<usize, Voting>,
    pub check: HashMap<usize, Check>,
}

#[derive(Clone, Debug)]
pub struct Engine {
    pub game: Game,
    pub phase: Option<Activity>,
    pub day: usize,
    pub state: EngineState,
    pub actor: Option<Position>,
}

#[derive(Clone, Debug)]
pub struct App {
    pub engine: Engine,
    pub input: String,
    pub current_timer: Option<u64>,
}

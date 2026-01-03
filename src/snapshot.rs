use crate::domain::{phase::Phase, position::Position, role::Role, status::Status};
use std::collections::HashMap;

pub trait Snapshot {
    type Output;

    fn snapshot(&self) -> Self::Output;
}

#[derive(Clone, Debug)]
pub struct PlayerData {
    pub name: String,
    pub role: Option<Role>,
    pub warnings: u8,
    pub life_status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ChairData {
    pub position: usize,
}

#[derive(Clone, Debug)]
pub struct SeatData {
    pub chair: ChairData,
    pub player: Option<PlayerData>, // None if empty
}

#[derive(Clone, Debug)]
pub struct TableData {
    pub seats: Vec<SeatData>,
}

#[derive(Clone, Debug)]
pub struct VotingData {
    pub nominations: HashMap<ChairData, ChairData>,
    pub nominees: Vec<ChairData>,
    pub votes: HashMap<ChairData, Vec<ChairData>>, // nominee -> voters
}

#[derive(Clone, Debug)]
pub struct RoundData {
    pub voting: VotingData,
    pub mafia_kill: Option<ChairData>,
    pub sheriff_check: Option<ChairData>,
    pub don_check: Option<ChairData>,
    pub eliminated: Vec<ChairData>,
    pub removed: Vec<ChairData>,
}

#[derive(Clone, Debug)]
pub struct EngineData {
    pub table: TableData,
    pub phase: Phase,
    pub round: RoundData,
    pub current_round: usize,
    pub actor: Option<ChairData>,
}

#[derive(Clone, Debug)]
pub struct AppData {
    pub engine: EngineData,
    pub input: String,
    pub current_timer: Option<u64>,
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

use crate::domain::{phase::Phase, role::Role};
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

#[derive(Clone, Debug)]
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
pub struct RoundData;

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

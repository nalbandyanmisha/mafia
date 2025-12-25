pub trait Snapshot {
    type Output;

    fn snapshot(&self) -> Self::Output;
}

#[derive(Clone, Debug)]
pub enum Role {
    Citizen,
    Mafia,
    Don,
    Sheriff,
}

#[derive(Clone, Debug)]
pub struct PlayerData {
    pub name: String,
    pub role: String,
    pub warnings: u8,
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
    pub phase: String,
    pub round: RoundData,
    pub current_speaker: Option<ChairData>,
    pub current_round: usize,
}

#[derive(Clone, Debug)]
pub struct AppData {
    pub engine: EngineData,
    pub input: String,
    pub current_timer: Option<u64>,
}

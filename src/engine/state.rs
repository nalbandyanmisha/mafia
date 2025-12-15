pub mod chair;
pub mod phase;
pub mod player;
pub mod role;
pub mod round;
pub mod table;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct State {
    pub table: table::Table,
    pub phase: phase::Phase,
    pub rounds: BTreeMap<round::RoundId, round::Round>,
    pub current_round: round::RoundId,
}

impl State {
    pub fn new() -> Self {
        State {
            table: table::Table::new(),
            phase: phase::Phase::default(),
            rounds: BTreeMap::new(),
            current_round: round::RoundId(0),
        }
    }
}

pub mod nomination;
pub mod phase;
pub mod player;
pub mod role;
pub mod round;
pub mod table;
pub mod vote;
use round::Round;
use std::collections::BTreeMap;
use table::chair::Chair;

#[derive(Debug)]
pub struct State {
    pub table: table::Table,
    pub phase: phase::Phase,
    pub rounds: BTreeMap<round::RoundId, round::Round>,
    pub current_round: round::RoundId,
    pub current_speaker: Option<Chair>,
}

impl State {
    pub fn new() -> Self {
        State {
            table: table::Table::new(),
            phase: phase::Phase::default(),
            rounds: BTreeMap::new(),
            current_round: round::RoundId(0),
            current_speaker: None,
        }
    }

    pub fn current_round_mut(&mut self) -> &mut Round {
        let id = self.current_round;

        self.rounds.entry(id).or_insert_with(Round::default)
    }

    pub fn current_round(&self) -> &Round {
        self.rounds
            .get(&self.current_round)
            .expect("Current round must exist")
    }
}

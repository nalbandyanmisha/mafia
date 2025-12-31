pub mod actor;
pub mod player;
pub mod round;
pub mod table;

use actor::Actor;

use crate::domain::{phase, role};
use crate::snapshot::{EngineData, Snapshot};

use self::{
    phase::Phase,
    player::Player,
    role::Role,
    round::{Round, RoundId},
    table::{Table, chair::Chair},
};
use std::collections::BTreeMap;

/// StateError covers all possible state-level errors.
#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("Player at chair {0:?} not found")]
    PlayerNotFound(Chair),
    #[error("No available seats")]
    NoAvailableSeats,
    #[error("No available roles")]
    NoAvailableRoles,
    #[error("Invalid chair")]
    InvalidChair,
}

/// Game state: table, rounds, phase, and speaker
#[derive(Debug)]
pub struct State {
    pub table: Table,
    pub phase: Phase,
    pub rounds: BTreeMap<RoundId, Round>,
    pub current_round: RoundId,
    pub actor: Actor,
}

impl Snapshot for State {
    type Output = EngineData;

    fn snapshot(&self) -> Self::Output {
        EngineData {
            table: self.table.snapshot(),
            phase: self.phase,
            round: self
                .rounds
                .get(&self.current_round)
                .map_or_else(|| Round::new().snapshot(), |round| round.snapshot()),
            current_round: self.current_round.0,
            actor: self.actor.snapshot(),
        }
    }
}

impl State {
    /* ---------------- Construction ---------------- */

    pub fn new() -> Self {
        let table = Table::new();
        State {
            table: table.clone(),
            phase: Phase::Lobby(phase::LobbyPhase::Waiting),
            rounds: BTreeMap::new(),
            current_round: RoundId(0),
            actor: Actor::new(table.chair(1).unwrap()), // Default actor at chair 1),
        }
    }

    /* ---------------- Phase ---------------- */

    pub fn phase(&self) -> Phase {
        self.phase
    }

    pub fn set_phase(&mut self, phase: Phase) {
        self.phase = phase;
    }

    /* ---------------- Players / Table ---------------- */

    pub fn available_roles(&self) -> &[Role] {
        self.table.available_roles()
    }

    pub fn available_seats(&self) -> &[Chair] {
        self.table.available_seats()
    }

    pub fn take_role(&mut self, role: Role) -> Result<(), StateError> {
        self.table
            .take_role(role)
            .map_err(|_| StateError::NoAvailableRoles)
    }

    pub fn return_role(&mut self, role: Role) {
        self.table.return_role(role)
    }

    pub fn take_seat(&mut self, chair: Chair) -> Result<(), StateError> {
        self.table
            .take_seat(chair)
            .map_err(|_| StateError::NoAvailableSeats)
    }

    pub fn return_seat(&mut self, chair: Chair) {
        self.table.return_seat(chair)
    }

    pub fn player(&self, chair: Chair) -> Result<&Player, StateError> {
        self.table
            .get_player(&chair)
            .map_err(|_| StateError::PlayerNotFound(chair))
    }

    pub fn player_mut(&mut self, chair: Chair) -> Result<&mut Player, StateError> {
        self.table
            .get_player_mut(&chair)
            .map_err(|_| StateError::PlayerNotFound(chair))
    }

    pub fn chair_of_player(&self, name: &str) -> Option<Chair> {
        self.table.get_chair(name)
    }

    pub fn assign_player(&mut self, chair: Chair, player: Player) -> Result<(), StateError> {
        self.table
            .seat_player(chair, player)
            .map_err(|_| StateError::InvalidChair)
    }

    pub fn remove_player(&mut self, chair: Chair) -> Result<(), StateError> {
        self.player(chair)?;
        self.table.clear_seat(chair);
        // roles / positions could be released here if implemented
        Ok(())
    }

    /* ---------------- Player Warnings / State ---------------- */

    pub fn increment_player_warning(&mut self, chair: Chair) -> Result<u8, StateError> {
        let player = self.player_mut(chair)?;
        player
            .increment_warning()
            .map_err(|_| StateError::InvalidChair)?;
        Ok(player.warnings())
    }

    pub fn deincrement_player_warning(&mut self, chair: Chair) -> Result<u8, StateError> {
        let player = self.player_mut(chair)?;
        player
            .deincrement_warning()
            .map_err(|_| StateError::InvalidChair)?;
        Ok(player.warnings())
    }

    pub fn mark_player_killed(&mut self, chair: Chair) -> Result<(), StateError> {
        let player = self.player_mut(chair)?;
        player.mark_killed();
        self.current_round_mut().record_mafia_kill(chair);
        Ok(())
    }

    pub fn mark_player_eliminated(&mut self, chair: Chair) -> Result<(), StateError> {
        let player = self.player_mut(chair)?;
        player.mark_eliminated();
        self.current_round_mut().record_elimination(chair);
        Ok(())
    }

    pub fn mark_player_removed(&mut self, chair: Chair) -> Result<(), StateError> {
        let player = self.player_mut(chair)?;
        player.mark_removed();
        self.current_round_mut().record_removal(chair);
        Ok(())
    }

    /* ---------------- Rounds ---------------- */

    pub fn start_new_round(&mut self) {
        self.current_round = self.current_round.next();
        self.rounds.insert(self.current_round, Round::new());
    }

    pub fn add_nomination(&mut self, nominator: Chair, nominee: Chair) -> Result<(), StateError> {
        self.current_round_mut()
            .record_nomination(nominator, nominee);
        Ok(())
    }

    pub fn add_vote(&mut self, voter: Chair, nominee: Chair) -> Result<(), StateError> {
        self.current_round_mut().record_vote(voter, nominee);
        Ok(())
    }

    pub fn current_round_mut(&mut self) -> &mut Round {
        self.rounds
            .entry(self.current_round)
            .or_insert_with(Round::new)
    }
}

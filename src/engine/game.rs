pub mod actor;
pub mod player;
pub mod round;
pub mod turn;
pub mod voting;

use std::collections::BTreeMap;

use crate::domain::{
    phase::{self, Phase},
    position::Position,
    role::Role,
};
use crate::engine::game::actor::Actor;
use crate::engine::game::player::Player;
use crate::engine::game::round::{Round, RoundId};
use crate::engine::game::turn::Turn;
use crate::snapshot::{self, Snapshot};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid position {0}")]
    InvalidPosition(u8),
    #[error("Player not found at position {0:?}")]
    PlayerNotFound(Position),
    #[error("No available roles left")]
    NoAvailableRoles,
    #[error("No available positions left")]
    NoAvailablePositions,
}

#[derive(Debug, Clone)]
pub struct Game {
    players: Vec<Player>,
    phase: Phase,
    pub rounds: BTreeMap<RoundId, Round>,
    pub current_round: RoundId,
    roles_pool: Vec<Role>,
    positions_pool: Vec<Position>,
}

impl Snapshot for Game {
    type Output = snapshot::Game;

    fn snapshot(&self) -> Self::Output {
        snapshot::Game {
            players: self.players.iter().map(|p| p.snapshot()).collect(),
            phase: self.phase,
            round: self
                .rounds
                .get(&self.current_round)
                .map_or_else(|| Round::new().snapshot(), |round| round.snapshot()),
            current_round: self.current_round.0,
        }
    }
}

impl Turn for Game {
    fn next_actor<F>(&self, actor: &mut Actor, is_eligible: F) -> Option<Position>
    where
        F: Fn(Position) -> bool,
    {
        if actor.is_completed() {
            return None;
        }

        let mut players: Vec<Position> = self.players.iter().filter_map(|p| p.position()).collect();

        players.sort();

        let start = actor.start();

        // 🔑 FIRST CALL: no current → start speaks first
        if actor.current().is_none() && is_eligible(start) {
            actor.set_current(Some(start));
            return Some(start);
        }

        let current = actor.current().unwrap_or(start);
        let start_idx = players.iter().position(|&p| p == current)?;

        for i in 1..=players.len() {
            let idx = (start_idx + i) % players.len();
            let pos = players[idx];

            if !is_eligible(pos) {
                continue;
            }

            // 🔒 If we are about to loop back to start → STOP
            if pos == start && actor.current().is_some() {
                actor.set_completed(true);
                return None;
            }

            actor.set_current(Some(pos));
            return Some(pos);
        }

        actor.set_completed(true);
        None
    }
}

impl Game {
    pub const PLAYER_COUNT: u8 = 10;

    pub fn new() -> Self {
        let mut positions_pool = Vec::new();
        let players = Vec::with_capacity(Self::PLAYER_COUNT as usize);

        for pos in 1..=Self::PLAYER_COUNT {
            positions_pool.push(Position::new(pos));
        }

        let roles_pool = vec![
            Role::Don,
            Role::Mafia,
            Role::Mafia,
            Role::Sheriff,
            Role::Citizen,
            Role::Citizen,
            Role::Citizen,
            Role::Citizen,
            Role::Citizen,
            Role::Citizen,
        ];

        Self {
            players,
            roles_pool,
            positions_pool,
            phase: Phase::Lobby(phase::LobbyPhase::Waiting),
            rounds: BTreeMap::new(),
            current_round: RoundId(0),
        }
    }

    /* ---------------- Rounds ---------------- */

    pub fn start_new_round(&mut self) {
        self.current_round = self.current_round.next();
        self.rounds.insert(self.current_round, Round::new());
    }

    pub fn add_nomination(&mut self, nominator: Position, nominee: Position) -> Result<(), Error> {
        self.current_round_mut()
            .record_nomination(nominator, nominee);
        Ok(())
    }

    pub fn add_vote(&mut self, voter: Position, nominee: Position) -> Result<(), Error> {
        self.current_round_mut().record_vote(voter, nominee);
        Ok(())
    }

    pub fn current_round_mut(&mut self) -> &mut Round {
        self.rounds
            .entry(self.current_round)
            .or_insert_with(Round::new)
    }

    /* ---------------- Phase ---------------- */

    pub fn phase(&self) -> Phase {
        self.phase
    }

    pub fn set_phase(&mut self, phase: Phase) {
        self.phase = phase;
    }

    pub fn sheriff(&self) -> Option<&Player> {
        self.players
            .iter()
            .find(|p| p.role() == Some(Role::Sheriff))
    }

    pub fn sheriff_mut(&mut self) -> Option<&mut Player> {
        self.players
            .iter_mut()
            .find(|p| p.role() == Some(Role::Sheriff))
    }

    pub fn don(&self) -> Option<&Player> {
        self.players.iter().find(|p| p.role() == Some(Role::Don))
    }

    pub fn don_mut(&mut self) -> Option<&mut Player> {
        self.players
            .iter_mut()
            .find(|p| p.role() == Some(Role::Don))
    }

    // ---------------- Players ----------------
    pub fn add_player(&mut self, name: &str) -> Result<(), String> {
        if self.players.len() >= Self::PLAYER_COUNT as usize {
            return Err("Maximum number of players reached".to_string());
        }

        self.players.push(Player::new(name.to_string()));
        Ok(())
    }

    pub fn remove_player(&mut self, name: &str) -> Result<(), String> {
        if let Some(pos) = self.players.iter().position(|p| p.name() == name) {
            self.players.remove(pos);
            Ok(())
        } else {
            Err("Player not found".to_string())
        }
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn players_mut(&mut self) -> &mut [Player] {
        &mut self.players
    }

    pub fn player_by_position(&self, position: Position) -> Option<&Player> {
        self.players.iter().find(|p| p.position() == Some(position))
    }

    pub fn player_by_position_mut(&mut self, position: Position) -> Option<&mut Player> {
        self.players
            .iter_mut()
            .find(|p| p.position() == Some(position))
    }

    pub fn player_by_name(&self, name: &str) -> Option<&Player> {
        self.players.iter().find(|p| p.name() == name)
    }

    pub fn player_by_name_mut(&mut self, name: &str) -> Option<&mut Player> {
        self.players.iter_mut().find(|p| p.name() == name)
    }

    // ---------------- Roles ----------------
    /// Assign a role, removing it from available roles
    pub fn take_role(&mut self, role: Role) -> Result<(), Error> {
        if let Some(pos) = self.roles_pool.iter().position(|r| *r == role) {
            self.roles_pool.remove(pos);
            Ok(())
        } else {
            Err(Error::NoAvailableRoles)
        }
    }

    /// Return a role to the pool
    pub fn return_role(&mut self, role: Role) {
        self.roles_pool.push(role);
    }

    pub fn available_roles(&self) -> &[Role] {
        &self.roles_pool
    }

    // ---------------- Positions ----------------
    pub fn take_position(&mut self, position: Position) -> Result<(), Error> {
        if let Some(pos) = self.positions_pool.iter().position(|c| *c == position) {
            self.positions_pool.remove(pos);
            Ok(())
        } else {
            Err(Error::NoAvailablePositions)
        }
    }

    pub fn return_position(&mut self, position: Position) {
        if !self.positions_pool.contains(&position) {
            self.positions_pool.push(position);
        }
    }

    pub fn available_positions(&self) -> &[Position] {
        &self.positions_pool
    }
}

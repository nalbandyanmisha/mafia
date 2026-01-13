pub mod check;
pub mod player;
pub mod voting;

use serde::Serialize;
use std::collections::HashMap;
use std::fmt::{self};

use crate::domain::{DayIndex, Position, Role};
use crate::engine::{
    Actor, Turn,
    game::{player::Player, voting::Voting},
};
use crate::snapshot::{self, Snapshot};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // #[error("Invalid position {0}")]
    // InvalidPosition(u8),
    #[error("Player not found at position {0:?}")]
    PlayerByPositionNotFound(Position),

    #[error("Player not found by name {0:?}")]
    PlayerByNameNotFound(String),

    #[error("Player by name {0:?} already exist")]
    PlayerByNameAlreadyExist(String),

    #[error("Player Name is emmpty")]
    PlayerNameIsEmpty,

    #[error("No available roles left")]
    NoAvailableRoles,

    #[error("No available positions left")]
    NoAvailablePositions,

    #[error(transparent)]
    Player(#[from] player::Error),

    #[error(transparent)]
    Check(#[from] check::Error),

    #[error(transparent)]
    Voting(#[from] voting::Error),
}

#[derive(Debug, Clone, Serialize)]
pub enum Event {
    PlayerLeft {
        name: String,
    },
    PlayerJoin {
        name: String,
    },
    Player(player::Event),
    Check(check::Event),
    Voting(voting::Event),
    FinalVoting(Position),
    MafiaKill {
        position: Position,
    },
    Guess {
        position: Position,
    },
    Eliminated {
        day: DayIndex,
        positions: Vec<Position>,
    },
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::PlayerLeft { name } => {
                write!(f, "Player  {name}, left the lobby")
            }
            Event::PlayerJoin { name } => {
                write!(f, "Player  {name}, joined to the lobby")
            }
            Event::Player(event) => write!(f, "{event}"),
            Event::Voting(event) => write!(f, "{event}"),
            Event::Check(event) => write!(f, "{event}"),
            Event::MafiaKill { position } => {
                write!(f, "Mafia has shoot {position}")
            }
            Event::Guess { position } => write!(f, "Dead player has guessed {position}"),
            Event::Eliminated { day, positions } => write!(
                f,
                "After day {} players at position {} were eliminated",
                day.current(),
                positions.iter().map(|p| p.to_string()).collect::<String>()
            ),
            Event::FinalVoting(position) => write!(f, "Final Voting {position}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    players: Vec<Player>,
    voting: HashMap<DayIndex, voting::Voting>,
    tie_voting: HashMap<DayIndex, voting::Voting>,
    final_voting: HashMap<DayIndex, Vec<Position>>,
    check: HashMap<DayIndex, check::Check>,
    kill: HashMap<DayIndex, Position>,
    guess: Vec<Position>,
    eliminated: HashMap<DayIndex, Vec<Position>>,
    roles_pool: Vec<Role>,
    positions_pool: Vec<Position>,
}

#[derive(Debug, Clone)]
pub enum Pool {
    Main,
    Tie,
    Final,
}

impl Snapshot for Game {
    type Output = snapshot::Game;

    fn snapshot(&self) -> Self::Output {
        snapshot::Game {
            players: self.players.iter().map(|p| p.snapshot()).collect(),
            voting: self
                .voting
                .iter()
                .map(|(k, v)| (k.current(), v.snapshot()))
                .collect(),
            tie_voting: self
                .tie_voting
                .iter()
                .map(|(k, v)| (k.current(), v.snapshot()))
                .collect(),
            final_voting: self
                .final_voting
                .iter()
                .map(|(k, v)| (k.current(), v.iter().map(|p| p.snapshot()).collect()))
                .collect(),
            check: self
                .check
                .iter()
                .map(|(k, v)| (k.current(), v.snapshot()))
                .collect(),
            kill: self
                .kill
                .iter()
                .map(|(k, v)| (k.current(), v.snapshot()))
                .collect(),
            guess: self.guess.clone(),
            eliminated: self
                .eliminated
                .iter()
                .map(|(k, v)| (k.current(), v.iter().map(|p| p.snapshot()).collect()))
                .collect(),
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
                actor.mark_completed();
                return None;
            }

            actor.set_current(Some(pos));
            return Some(pos);
        }

        actor.mark_completed();
        None
    }
}

impl Game {
    pub const PLAYER_COUNT: u8 = 10;

    pub fn new() -> Self {
        let mut positions_pool = Vec::new();
        let players = Vec::with_capacity(Self::PLAYER_COUNT as usize);
        let voting = HashMap::new();
        let tie_voting = HashMap::new();
        let final_voting = HashMap::new();
        let check = HashMap::new();
        let kill = HashMap::new();
        let guess = Vec::new();
        let eliminated = HashMap::new();

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
            voting,
            tie_voting,
            final_voting,
            check,
            kill,
            guess,
            eliminated,
            roles_pool,
            positions_pool,
        }
    }

    // ---------------- Players ----------------
    pub fn add_player(&mut self, name: &str) -> Result<Vec<Event>, Error> {
        if name.is_empty() {
            return Err(Error::PlayerNameIsEmpty);
        }
        if self.players.iter().any(|p| p.name() == name) {
            return Err(Error::PlayerByNameAlreadyExist(name.to_string()));
        }
        if self.players.len() >= Self::PLAYER_COUNT as usize {
            return Err(Error::Player(player::Error::HasPosition));
        }

        self.players.push(Player::new(name.to_string()));
        Ok(vec![Event::PlayerJoin {
            name: name.to_string(),
        }])
    }

    pub fn remove_player(&mut self, name: &str) -> Result<Vec<Event>, Error> {
        if let Some(pos) = self.players.iter().position(|p| p.name() == name) {
            self.players.remove(pos);
            Ok(vec![Event::PlayerLeft {
                name: name.to_string(),
            }])
        } else {
            Err(Error::PlayerByNameNotFound(name.to_string()))
        }
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn players_mut(&mut self) -> &mut [Player] {
        &mut self.players
    }

    pub fn alive_players(&self) -> usize {
        self.players
            .iter()
            .fold(0, |c, p| if p.is_alive() { c + 1 } else { c })
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

    pub fn sheriff(&self) -> Option<&Player> {
        self.players
            .iter()
            .find(|p| p.role() == Some(Role::Sheriff))
    }

    pub fn don(&self) -> Option<&Player> {
        self.players.iter().find(|p| p.role() == Some(Role::Don))
    }

    /*---------------- Checks ---------------- */
    pub fn record_sheriff_check(
        &mut self,
        day: DayIndex,
        check: Position,
    ) -> Result<Vec<Event>, Error> {
        let events = self
            .check
            .entry(day)
            .or_default()
            .record_sheriff_check(check)?;
        Ok(events.into_iter().map(Event::Check).collect())
    }

    pub fn record_don_check(
        &mut self,
        day: DayIndex,
        checked: Position,
    ) -> Result<Vec<Event>, Error> {
        let events = self
            .check
            .entry(day)
            .or_default()
            .record_don_check(checked)?;
        Ok(events.into_iter().map(Event::Check).collect())
    }

    pub fn record_mafia_kill(
        &mut self,
        day: DayIndex,
        killed: Position,
    ) -> Result<Vec<Event>, Error> {
        self.kill.entry(day).or_insert(killed);
        Ok(vec![Event::MafiaKill { position: killed }])
    }

    pub fn record_eliminated(
        &mut self,
        day: DayIndex,
        eliminated: &[Position],
    ) -> Result<Vec<Event>, Error> {
        self.eliminated.entry(day).or_default().extend(eliminated);
        Ok(vec![Event::Eliminated {
            day,
            positions: eliminated.to_vec(),
        }])
    }

    pub fn record_guess(&mut self, guess: Position) -> Result<Vec<Event>, Error> {
        self.guess.push(guess);
        Ok(vec![Event::Guess { position: guess }])
    }

    /* ---------------- Votes & Nominations ---------------- */

    pub fn add_nomination(
        &mut self,
        day: DayIndex,
        nominator: Position,
        nominee: Position,
    ) -> Result<Vec<Event>, Error> {
        let events = self
            .voting
            .entry(day)
            .or_default()
            .nominate(nominator, nominee)?;
        Ok(events.into_iter().map(Event::Voting).collect())
    }

    pub fn add_vote(
        &mut self,
        day: DayIndex,
        pool: Pool,
        voter: Position,
        nominee: Position,
    ) -> Result<Vec<Event>, Error> {
        match pool {
            Pool::Main => {
                let voting = self.voting.entry(day).or_default();
                let events = voting.vote(voter, nominee)?;
                Ok(events.into_iter().map(Event::Voting).collect())
            }
            Pool::Tie => {
                let voting = self.tie_voting.entry(day).or_default();
                let events = voting.vote(voter, nominee)?;
                Ok(events.into_iter().map(Event::Voting).collect())
            }
            Pool::Final => {
                let voting = self.final_voting.entry(day).or_default();
                voting.push(voter);
                Ok(vec![Event::FinalVoting(voter)])
            }
        }
    }

    pub fn voting(&self) -> &HashMap<DayIndex, Voting> {
        &self.voting
    }

    pub fn tie_voting(&self) -> &HashMap<DayIndex, Voting> {
        &self.tie_voting
    }

    pub fn tie_voting_mut(&mut self) -> &mut HashMap<DayIndex, Voting> {
        &mut self.tie_voting
    }

    pub fn final_voting(&self) -> &HashMap<DayIndex, Vec<Position>> {
        &self.final_voting
    }

    pub fn get_kill(&self, day: DayIndex) -> Option<&Position> {
        self.kill.get(&day)
    }

    pub fn get_eliminated(&self, day: DayIndex) -> Option<&Vec<Position>> {
        self.eliminated.get(&day)
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

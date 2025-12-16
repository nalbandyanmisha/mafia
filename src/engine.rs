pub mod commands;
pub mod events;
pub mod state;

use self::{
    commands::Command,
    events::Event,
    state::{
        State,
        nomination::Nomination,
        phase::Phase,
        player::{Player, Status as PlayerStatus},
        table::{Table, chair::Chair},
        vote::Vote,
    },
};
use anyhow::{Result, bail};

#[derive(Debug)]
pub struct Engine {
    pub state: State,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            state: State::new(),
        }
    }
    pub fn apply(&mut self, cmd: Command) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
        match cmd {
            Command::Join { name } => {
                let (chair, player) = self.join(&name)?;
                Ok(vec![Event::PlayerJoined { player, chair }])
            }
            Command::Leave { name } => {
                let (chair, player) = self.leave(&name).unwrap();
                Ok(vec![Event::PlayerLeft { player, chair }])
            }
            Command::Warn { chair } => Ok(vec![Event::PlayerWarned {
                player: self.warn(chair)?,
                chair,
            }]),
            Command::Pardon { chair } => Ok(vec![Event::PlayerPardoned {
                player: self.pardon(chair)?,
                chair,
            }]),
            Command::Nominate { target } => {
                self.nominate(target).map(|_| vec![Event::PlayerNominated])
            }
            Command::Shoot { chair } => Ok(vec![Event::PlayerKilled {
                player: self.shoot(chair)?,
                chair,
            }]),
            Command::NextPhase => self.next_phase().map(|_| vec![Event::PhaseAdvanced]),
            Command::NextSpeaker => self.next_speaker().map(|_| vec![]),
        }
    }

    // ------------------------------
    // Join / Leave
    // ------------------------------
    fn join(&mut self, name: &str) -> Result<(Chair, Player), EngineError> {
        if self.state.phase != Phase::Lobby {
            return Err(EngineError::RegistrationClosed);
        }

        // Pick a position
        let position = self
            .state
            .table
            .pick_position()
            .map_err(|_| EngineError::NoAvailableSeats)?;

        let chair = self
            .state
            .table
            .try_chair(position)
            .map_err(|_| EngineError::NoAvailableSeats)?;

        let role = self
            .state
            .table
            .pick_role()
            .map_err(|_| EngineError::NoAvailableRoles)?;

        let player = Player::new(name.to_string(), role);

        self.state.table.assign_player(chair, player.clone());

        Ok((chair, player))
    }

    fn leave(&mut self, name: &str) -> Result<(Chair, Player), EngineError> {
        if self.state.phase != Phase::Lobby {
            return Err(EngineError::RegistrationClosed);
        }

        let (chair, player) = self
            .state
            .table
            .all_chairs()
            .find(|(_, p)| p.name() == name)
            .map(|(c, p)| (*c, p.clone()))
            .ok_or(EngineError::PlayerNotFound)?;

        // Release seat and role
        self.state.table.release_position(chair.position());
        self.state.table.release_role(player.role());
        self.state.table.remove_player(chair);

        Ok((chair, player))
    }

    // ------------------------------
    // Player actions
    // ------------------------------
    fn shoot(&mut self, chair: Chair) -> Result<Player, EngineError> {
        let player = self
            .state
            .table
            .get_player_mut(&chair)
            .map_err(|_| EngineError::PlayerNotFound)?;

        player.kill();
        Ok(player.clone())
    }

    fn warn(&mut self, chair: Chair) -> Result<Player, EngineError> {
        let player = self
            .state
            .table
            .get_player_mut(&chair)
            .map_err(|_| EngineError::PlayerNotFound)?;

        player.add_warning();
        Ok(player.clone())
    }

    fn pardon(&mut self, chair: Chair) -> Result<Player, EngineError> {
        let player = self
            .state
            .table
            .get_player_mut(&chair)
            .map_err(|_| EngineError::PlayerNotFound)?;

        player.remove_warning();
        Ok(player.clone())
    }
    pub fn nominate(&mut self, target: Chair) -> Result<(), Box<dyn std::error::Error>> {
        self.ensure_phase(Phase::Day)?;

        let by = self.current_speaker()?;

        self.ensure_alive(by)?;
        self.ensure_alive(target)?;

        let current_round = self.state.current_round_mut();

        // Check if this speaker has already nominated
        if current_round.nominations.iter().any(|n| n.by == by) {
            return Err(format!(
                "Player at chair {} has already nominated this round",
                by.position()
            )
            .into());
        }

        current_round.nominations.push(Nomination { by, target });

        Ok(())
    }

    pub fn vote(&mut self, voter: Chair, target: Chair) -> Result<(), Box<dyn std::error::Error>> {
        self.ensure_phase(Phase::Voting)?;

        self.ensure_alive(voter)?;
        self.ensure_alive(target)?;

        self.state
            .current_round_mut()
            .votes
            .push(Vote { voter, target });

        Ok(())
    }

    // ------------------------------
    // Phase / Speaker helpers
    // ------------------------------
    fn first_speaker_of_day(&self) -> Option<Chair> {
        let start = (self.state.current_round.0 % Table::SEATS as usize) as u8 + 1;

        self.find_next_alive_chair_from(start)
    }

    fn find_next_alive_chair_from(&self, start: u8) -> Option<Chair> {
        for offset in 0..Table::SEATS {
            let pos = ((start - 1 + offset) % Table::SEATS) + 1;
            if let Ok(chair) = self.state.table.try_chair(pos) {
                if self.state.table.get_player(&chair).map_or(false, |p| {
                    p.status() == PlayerStatus::Alive && !p.name().is_empty()
                }) {
                    return Some(chair);
                }
            }
        }
        None
    }

    fn next_phase(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.state.phase == Phase::Voting {
            self.state.current_round = self.state.current_round.next();
        }
        self.state.phase.next()?;

        if self.state.phase == Phase::Day {
            self.state.current_speaker = self.first_speaker_of_day();
        }
        Ok(())
    }

    fn next_speaker(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let current = match self.state.current_speaker {
            Some(c) => c,
            None => return Ok(()), // no speaker → already done
        };

        let next = self.find_next_alive_chair_from(current.position() + 1);

        // If we looped back to the first speaker → Day ends
        if next == self.first_speaker_of_day() {
            self.state.current_speaker = None;
            self.state.phase = Phase::Voting;
        } else {
            self.state.current_speaker = next;
        }

        Ok(())
    }

    fn ensure_phase(&self, expected: Phase) -> Result<()> {
        if self.state.phase != expected {
            bail!(
                "Action allowed only during {:?} phase, current phase is {:?}",
                expected,
                self.state.phase
            );
        }
        Ok(())
    }

    fn current_speaker(&self) -> Result<Chair, Box<dyn std::error::Error>> {
        self.state
            .current_speaker
            .ok_or_else(|| "No active speaker".into())
    }

    fn ensure_alive(&self, chair: Chair) -> Result<(), EngineError> {
        let player = self
            .state
            .table
            .get_player(&chair)
            .map_err(|_| EngineError::PlayerNotFound)?;

        if matches!(
            player.status(),
            PlayerStatus::Killed | PlayerStatus::Removed | PlayerStatus::Eliminated
        ) {
            return Err(EngineError::PlayerDead(chair));
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("Cannot join, no available seats")]
    NoAvailableSeats,
    #[error("Cannot assign role, no available roles")]
    NoAvailableRoles,
    #[error("Cannot join after registration")]
    RegistrationClosed,
    #[error("Player not found")]
    PlayerNotFound,
    #[error("Player assignment failed")]
    PlayerAssignmentFailed,
    #[error("Wrong phase. Expected {expected:?}, got {current:?}")]
    WrongPhase { expected: Phase, current: Phase },
    #[error("Player {0:?} is dead")]
    PlayerDead(Chair),
    #[error("No active speaker")]
    NoActiveSpeaker,
    #[error("Player at chair {0:?} has already nominated this round")]
    AlreadyNominated(Chair),
}

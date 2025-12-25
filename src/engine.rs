pub mod commands;
pub mod events;
pub mod state;

use self::{
    commands::Command,
    events::Event,
    state::{
        State,
        player::{LifeStatus as PlayerLifeStatus, Player},
        round::RoundId,
        table::Table,
        table::chair::Chair,
    },
};
use crate::domain::phase::Phase;
use anyhow::{Result, bail};
use rand::prelude::*;

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
    pub fn apply(&mut self, cmd: Command) -> Result<Vec<Event>> {
        match cmd {
            Command::Join { name } => self.join(&name),
            Command::Leave { name } => self.leave(&name),
            Command::Warn { chair } => self.warn(chair),
            Command::Pardon { chair } => self.pardon(chair),
            Command::Nominate { target } => self.nominate(target),
            Command::Shoot { chair } => self.shoot(chair),
            Command::NextPhase => self.advance_phase(),
            Command::NextSpeaker => self.advance_speaker(),
        }
    }

    pub fn chair_from_position(&self, position: u8) -> anyhow::Result<Chair> {
        self.state
            .table
            .chair(position)
            .map_err(|e| anyhow::anyhow!("Invalid chair: {e}"))
    }
    // ------------------------------
    // Join / Leave
    // ------------------------------
    fn join(&mut self, name: &str) -> Result<Vec<Event>> {
        self.ensure_phase(Phase::Lobby)?;

        // Pick random seat
        let chair = *self
            .state
            .available_seats()
            .choose(&mut rand::rng())
            .ok_or_else(|| anyhow::anyhow!("No available seats"))?;
        self.state.take_seat(chair)?;

        // Pick random role
        let role = *self
            .state
            .available_roles()
            .choose(&mut rand::rng())
            .ok_or_else(|| anyhow::anyhow!("No available roles"))?;
        self.state.take_role(role)?;

        let player = Player::new(name.to_string(), role);
        self.state.assign_player(chair, player.clone())?;

        Ok(vec![Event::PlayerJoined { player, chair }])
    }

    fn leave(&mut self, name: &str) -> Result<Vec<Event>> {
        self.ensure_phase(Phase::Lobby)?;
        let chair = self
            .state
            .chair_of_player(name)
            .ok_or_else(|| anyhow::anyhow!("Player not found"))?;
        let player = self.state.player(chair)?.clone();

        self.state.return_seat(chair);
        self.state.return_role(player.role());
        self.state.remove_player(chair)?;

        Ok(vec![Event::PlayerLeft { player, chair }])
    }

    // ------------------------------
    // Player actions
    // ------------------------------
    fn warn(&mut self, chair: Chair) -> Result<Vec<Event>> {
        self.ensure_alive(chair)?;
        let warnings = self.state.increment_player_warning(chair)?;
        Ok(vec![Event::PlayerWarned { chair, warnings }])
    }
    fn pardon(&mut self, chair: Chair) -> Result<Vec<Event>> {
        let warnings = self.state.deincrement_player_warning(chair)?;
        Ok(vec![Event::PlayerPardoned { chair, warnings }])
    }
    fn shoot(&mut self, chair: Chair) -> Result<Vec<Event>> {
        self.ensure_alive(chair)?;
        self.state.mark_player_killed(chair)?;
        Ok(vec![Event::PlayerKilled { chair }])
    }
    fn nominate(&mut self, target: Chair) -> Result<Vec<Event>> {
        self.ensure_phase(Phase::Day)?;
        let by = self.current_speaker()?;
        self.ensure_alive(by)?;
        self.ensure_alive(target)?;

        self.state.add_nomination(by, target)?;
        Ok(vec![Event::PlayerNominated { by, target }])
    }

    fn advance_phase(&mut self) -> Result<Vec<Event>> {
        if self.state.phase() == Phase::Voting {
            self.state.start_new_round();
        }

        let next = self.state.phase().advance_phase()?;
        self.state.set_phase(next);

        Ok(vec![Event::PhaseAdvanced { phase: next }])
    }
    fn advance_speaker(&mut self) -> Result<Vec<Event>> {
        let current = match self.state.current_speaker {
            Some(c) => c,
            None => return Ok(vec![Event::EndDay]), // no speaker → already done
        };

        let next = self.find_next_alive_chair_from(current.position() + 1);

        // If we looped back to the first speaker → Day ends
        if next == self.first_speaker_of_day() {
            self.state.current_speaker = None;
            self.state.phase = Phase::Voting;
        } else {
            self.state.current_speaker = next;
        }

        Ok(vec![Event::NextSpeaker {
            chair: next.ok_or_else(|| anyhow::anyhow!("No next speaker found"))?,
        }])
    }

    fn first_speaker_of_day(&self) -> Option<Chair> {
        let start = (self.state.current_round.0 % Table::SEATS as usize) as u8 + 1;

        self.find_next_alive_chair_from(start)
    }

    fn find_next_alive_chair_from(&self, start: u8) -> Option<Chair> {
        for offset in 0..Table::SEATS {
            let pos = ((start - 1 + offset) % Table::SEATS) + 1;
            if let Ok(chair) = self.state.table.chair(pos) {
                if self.state.table.get_player(&chair).map_or(false, |p| {
                    p.life_status() == PlayerLifeStatus::Alive && !p.name().is_empty()
                }) {
                    return Some(chair);
                }
            }
        }
        None
    }

    // ------------------------------
    // Guards
    // ------------------------------
    fn ensure_phase(&self, expected: Phase) -> Result<()> {
        if self.state.phase() != expected {
            bail!(
                "Wrong phase. Expected {expected:?}, got {:?}",
                self.state.phase()
            );
        }
        Ok(())
    }

    fn ensure_alive(&self, chair: Chair) -> Result<()> {
        let player = self.state.player(chair)?;
        if !player.is_alive() {
            bail!("Player {chair:?} is not alive");
        }
        Ok(())
    }

    fn current_speaker(&self) -> Result<Chair> {
        self.state
            .current_speaker()
            .ok_or_else(|| anyhow::anyhow!("No active speaker"))
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

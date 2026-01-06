pub mod actor;
pub mod commands;
pub mod events;
pub mod game;
pub mod turn;

use actor::Actor;
use turn::{Turn, TurnContext};

use self::{commands::Command, events::Event, game::Game};
use crate::{
    domain::{
        EngineState, LobbyStatus, Position, RoundId,
        phase::{CheckPhase, DayPhase, NightPhase, Phase, PhaseKind, VotingPhase},
    },
    snapshot::{self, Snapshot},
};
use anyhow::{Result, bail};
use rand::prelude::*;

#[derive(Debug)]
pub struct Engine {
    pub game: Game,
    pub actor: Actor,
    pub round: RoundId,
    pub state: EngineState,
}

impl Snapshot for Engine {
    type Output = snapshot::Engine;

    fn snapshot(&self) -> Self::Output {
        let phase = match self.state {
            EngineState::Game(phase) => Some(phase),
            EngineState::Lobby(_) => None,
        };
        snapshot::Engine {
            game: self.game.snapshot(),
            actor: self.actor.snapshot(),
            phase,
            round: self.round.current(),
            state: self.state,
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            game: Game::new(),
            actor: Actor::new(Position::new(1)),
            round: RoundId::new(0),
            state: EngineState::Lobby(LobbyStatus::Waiting),
        }
    }
    pub fn apply(&mut self, cmd: Command) -> Result<Vec<Event>> {
        match cmd {
            Command::Join { name } => self.join(&name),
            Command::Leave { name } => self.leave(&name),
            Command::Start => self.start(),
            Command::AssignRole => self.assign_role(self.actor.current().unwrap()),
            Command::RevokeRole => self.revoke_role(self.actor.current().unwrap()),
            Command::AdvanceActor => self.advance_actor(),
            Command::NextPhase => self.advance_phase(),
            Command::Warn { target } => self.warn(target),
            Command::Pardon { target } => self.pardon(target),
            Command::Nominate { target } => self.nominate(target),
            Command::Vote { targets } => self.vote(targets),
            Command::Shoot { target } => self.shoot(target),
            Command::Check { target } => self.check(target),
        }
    }

    // ------------------------------
    // Join / Leave
    // ------------------------------
    fn join(&mut self, name: &str) -> Result<Vec<Event>, anyhow::Error> {
        self.ensure_lobby_waiting()?;

        self.game.add_player(name);
        self.assign_position(name)?;

        // As for now position assignment is happning simultaneously with joining,
        // this is good enough but if seprate those processes later, this logic should be updated
        if self.game.available_positions().is_empty() {
            self.state = EngineState::Lobby(LobbyStatus::Ready);
        }

        Ok(vec![Event::PlayerJoined {
            name: name.to_string(),
        }])
    }

    fn leave(&mut self, name: &str) -> Result<Vec<Event>> {
        self.ensure_lobby()?;
        self.revoke_position(name)?;
        self.game.remove_player(name);

        // As for now position assignment is happning simultaneously with joining,
        // this is good enough but if seprate those processes later, this logic should be updated
        if self.state == EngineState::Lobby(LobbyStatus::Ready)
            && self.game.available_positions().len() == 1
        {
            self.state = EngineState::Lobby(LobbyStatus::Waiting);
        }

        Ok(vec![Event::PlayerLeft {
            name: name.to_string(),
        }])
    }

    fn assign_position(&mut self, name: &str) -> Result<Vec<Event>> {
        self.ensure_lobby_waiting()?;

        // Pick random seat
        let position = *self
            .game
            .available_positions()
            .choose(&mut rand::rng())
            .ok_or_else(|| anyhow::anyhow!("No available positions"))?;
        self.game.take_position(position)?;

        let player = self
            .game
            .player_by_name_mut(name)
            .ok_or_else(|| anyhow::anyhow!("Player not found"))?;
        player.assign_position(position);

        Ok(vec![])
    }

    fn revoke_position(&mut self, name: &str) -> Result<Vec<Event>> {
        self.ensure_lobby()?;

        let position = self
            .game
            .player_by_name(name)
            .ok_or_else(|| anyhow::anyhow!("Player not found"))?
            .position()
            .ok_or_else(|| anyhow::anyhow!("Player {name} does not have an assigned position"))?;
        let player = self
            .game
            .player_by_name_mut(name)
            .ok_or_else(|| anyhow::anyhow!("Player not found"))?;
        player.clear_position();

        self.game.return_position(position);
        Ok(vec![])
    }

    fn start(&mut self) -> Result<Vec<Event>> {
        self.ensure_lobby_ready()?;

        self.game
            .players_mut()
            .sort_by_key(|p| p.position().map(|pos| pos.value()));
        self.state = EngineState::Game(Phase::Night(NightPhase::RoleAssignment));
        self.actor.set_start(self.first_speaker_of_day());

        Ok(vec![Event::GameStarted])
    }

    fn assign_role(&mut self, position: Position) -> Result<Vec<Event>> {
        self.ensure_role_assignment()?;

        if self
            .game
            .player_by_position(position)
            .unwrap()
            .role()
            .is_some()
        {
            return Ok(vec![]); // already has role
        }
        // Pick random role
        let role = *self
            .game
            .available_roles()
            .choose(&mut rand::rng())
            .ok_or_else(|| anyhow::anyhow!("No available roles"))?;
        self.game.take_role(role)?;

        let player = self.game.player_by_position_mut(position).unwrap();
        player.assign_role(role);
        Ok(vec![])
    }

    fn revoke_role(&mut self, position: Position) -> Result<Vec<Event>> {
        self.ensure_role_assignment()?;

        let player = self.game.player_by_position(position).unwrap();
        self.game.return_role(player.role().unwrap());

        let player = self.game.player_by_position_mut(position).unwrap();
        player.clear_role();
        Ok(vec![])
    }

    // ------------------------------
    // Player actions
    // ------------------------------
    fn warn(&mut self, target: Position) -> Result<Vec<Event>> {
        self.ensure_alive(target)?;
        let player = self.game.player_by_position_mut(target).unwrap();
        player.add_warning()?;
        let warnings = player.warnings();

        Ok(vec![Event::PlayerWarned { target, warnings }])
    }

    fn pardon(&mut self, target: Position) -> Result<Vec<Event>> {
        self.ensure_alive(target)?;
        let player = self.game.player_by_position_mut(target).unwrap();
        player.remove_warning()?;
        let warnings = player.warnings();
        Ok(vec![Event::PlayerPardoned { target, warnings }])
    }

    fn shoot(&mut self, target: Position) -> Result<Vec<Event>> {
        self.ensure_alive(target)?;
        let player = self.game.player_by_position_mut(target).unwrap();
        player.mark_dead();
        self.game.record_mafia_kill(self.round, target)?;
        Ok(vec![Event::PlayerKilled { target }])
    }

    fn check(&mut self, target: Position) -> Result<Vec<Event>> {
        self.ensure_phase(PhaseKind::Night)?;
        match self.phase()? {
            Phase::Night(NightPhase::Investigation(CheckPhase::Sheriff)) => {
                let by = self.game.sheriff();
                self.ensure_alive(by.unwrap().position().unwrap())?;
                self.ensure_alive(target)?;
                self.game.record_sheriff_check(self.round, target)?;
            }
            Phase::Night(NightPhase::Investigation(CheckPhase::Don)) => {
                let by = self.game.don();
                self.ensure_alive(by.unwrap().position().unwrap())?;
                self.ensure_alive(target)?;
                self.game.record_don_check(self.round, target)?;
            }
            _ => bail!("Not in investigation phase"),
        };

        Ok(vec![])
    }

    fn nominate(&mut self, target: Position) -> Result<Vec<Event>> {
        self.ensure_phase(PhaseKind::Day)?;
        let by = self
            .actor
            .current()
            .ok_or_else(|| anyhow::anyhow!("No active speaker"))?;
        self.ensure_alive(by)?;
        self.ensure_alive(target)?;

        self.game.add_nomination(self.round, by, target)?;
        Ok(vec![Event::PlayerNominated { by, target }])
    }

    fn vote(&mut self, voters: Vec<Position>) -> Result<Vec<Event>> {
        self.ensure_phase(PhaseKind::Day)?;

        let nominee = self
            .actor
            .current()
            .ok_or_else(|| anyhow::anyhow!("No active speaker"))?;
        self.ensure_alive(nominee)?;

        for &voter in &voters {
            self.ensure_alive(voter)?;
            self.game.add_vote(self.round, voter, nominee)?;
        }

        Ok(vec![])
    }

    fn turn_context(&self) -> Option<TurnContext> {
        use DayPhase::*;
        use NightPhase::*;
        use Phase::*;
        use VotingPhase::*;

        match self.phase().expect("Phase retrieval failed") {
            Night(RoleAssignment) => Some(TurnContext::RoleAssignment),
            Day(Discussion) => Some(TurnContext::DayDiscussion),
            Day(Voting(TieDiscussion)) => Some(TurnContext::VotingDiscussion),
            Day(Voting(VoteCast)) => Some(TurnContext::VoteCasting),
            _ => None,
        }
    }

    pub fn advance_actor(&mut self) -> Result<Vec<Event>> {
        let ctx = self
            .game
            .turn_context(self.round, self.phase()?, &self.actor);

        let ctx = match self.state {
            EngineState::Game(phase) => self.game.turn_context(self.round, phase, &self.actor),
            _ => None,
        };
        let next = match ctx {
            Some(TurnContext::RoleAssignment) => self.game.next_actor(&mut self.actor, |pos| {
                self.game
                    .player_by_position(pos)
                    .map(|p| p.role().is_none())
                    .unwrap_or(false)
            }),
            Some(TurnContext::DayDiscussion) => self.game.next_actor(&mut self.actor, |pos| {
                self.game
                    .player_by_position(pos)
                    .map(|p| p.is_alive())
                    .unwrap_or(false)
            }),
            Some(TurnContext::VoteCasting) => self
                .game
                .voting_mut()
                .entry(self.round)
                .or_default()
                .next_actor(&mut self.actor, |_| true),
            Some(TurnContext::FinalSpeech(player)) => {
                // Single actor turn
                if self.actor.current().is_none() {
                    self.actor.set_current(Some(player));
                    Some(player)
                } else {
                    self.actor.set_completed(true);
                    None
                }
            }
            Some(TurnContext::SheriffCheck(player)) | Some(TurnContext::DonCheck(player)) => {
                if self.actor.current().is_none() {
                    self.actor.set_current(Some(player));
                    Some(player)
                } else {
                    self.actor.set_completed(true);
                    None
                }
            }
            Some(TurnContext::VotingDiscussion) => {
                // delegate to voting module
                None
            }
            None => return Ok(vec![]),
        };

        match next {
            Some(chair) => Ok(vec![Event::ActorAdvanced { chair }]),
            None => Ok(vec![Event::TurnCompleted]),
        }
    }

    fn next_phase(&mut self, phase: Phase) -> Phase {
        use CheckPhase::*;
        use DayPhase::*;
        use NightPhase::*;
        use Phase::*;
        use VotingPhase::*;

        match phase {
            // -------- Night --------
            Night(RoleAssignment) => {
                if self.actor.is_completed() {
                    self.actor.reset(self.first_speaker_of_day());
                    Night(SheriffReveal)
                } else {
                    Night(RoleAssignment)
                }
            }
            Night(SheriffReveal) => Night(DonReveal),
            Night(DonReveal) => Night(MafiaBriefing),
            Night(MafiaBriefing) => {
                if self.round.is_first() {
                    Day(Discussion)
                } else {
                    Night(Investigation(Sheriff))
                }
            }
            Night(MafiaShoot) => Night(Investigation(Sheriff)),
            Night(Investigation(Sheriff)) => Night(Investigation(Don)),
            Night(Investigation(Don)) => Day(Morning),

            // -------- Day --------
            Day(Morning) => {
                self.actor.set_start(self.first_speaker_of_day());
                Day(Discussion)
            }
            Day(Discussion) => Day(Voting(Nomination)),

            // -------- Voting --------
            Day(Voting(v)) => match v {
                Nomination => Day(Voting(VoteCast)),
                VoteCast => Day(Voting(Resolution)),
                TieDiscussion => Day(Voting(TieRevote)),
                TieRevote => Day(Voting(Resolution)),
                Resolution => Night(MafiaShoot),
            },
        }
    }

    fn advance_phase(&mut self) -> Result<Vec<Event>> {
        let current = self.phase()?;
        let next = self.next_phase(current);

        if next == Phase::Night(NightPhase::MafiaShoot) {
            // new round
            self.round.advance();
        };

        self.set_phase(next)?;
        self.actor.reset(self.first_speaker_of_day());

        Ok(vec![Event::PhaseAdvanced { phase: next }])
    }

    fn first_speaker_of_day(&self) -> Position {
        ((self.round.0 % Game::PLAYER_COUNT as usize) as u8 + 1).into()
    }

    fn phase(&self) -> Result<Phase> {
        match self.state {
            EngineState::Game(phase) => Ok(phase),
            _ => bail!("Engine is not in Game state"),
        }
    }

    fn set_phase(&mut self, phase: Phase) -> Result<()> {
        match &mut self.state {
            EngineState::Game(p) => {
                *p = phase;
                Ok(())
            }
            _ => bail!("Engine is not in Game state"),
        }
    }

    // ------------------------------
    // Guards
    // ------------------------------
    pub fn ensure_lobby(&self) -> Result<()> {
        match &self.state {
            EngineState::Lobby(_) => Ok(()),
            other => bail!("Engine is not in Lobby state, got {other:?}",),
        }
    }

    pub fn ensure_lobby_waiting(&self) -> Result<()> {
        match &self.state {
            EngineState::Lobby(LobbyStatus::Waiting) => Ok(()),
            other => bail!("Engine is not in Lobby Waiting state, got {other:?}"),
        }
    }

    pub fn ensure_lobby_ready(&self) -> Result<()> {
        match &self.state {
            EngineState::Lobby(LobbyStatus::Ready) => Ok(()),
            other => bail!("Engine is not in Lobby Ready state, got {other:?}"),
        }
    }

    pub fn ensure_role_assignment(&self) -> Result<()> {
        match &self.phase()? {
            Phase::Night(NightPhase::RoleAssignment) => Ok(()),
            other => bail!("Engine is not in Role Assignment phase, got {other:?}"),
        }
    }

    fn ensure_phase(&self, expected: PhaseKind) -> Result<()> {
        let phase = self.phase()?;
        if phase.kind() != expected {
            bail!("Wrong phase. Expected {expected:?}, got {phase:?}");
        }
        Ok(())
    }

    fn ensure_alive(&self, position: Position) -> Result<()> {
        let player = self
            .game
            .player_by_position(position)
            .ok_or_else(|| anyhow::anyhow!("Player not found"))?;
        if !player.is_alive() {
            bail!("Player {position:?} is not alive");
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
    PlayerDead(Position),
    #[error("No active speaker")]
    NoActiveSpeaker,
    #[error("Player at chair {0:?} has already nominated this round")]
    AlreadyNominated(Position),
}

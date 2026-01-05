pub mod commands;

pub mod events;
pub mod game;

use self::{
    commands::Command,
    events::Event,
    game::{Game, actor::Actor, turn::Turn},
};
use crate::{
    domain::{
        Position, RoundId,
        phase::{
            CheckPhase, DayPhase, LobbyPhase, NightPhase, Phase, PhaseKind, TurnContext,
            VotingPhase,
        },
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
    pub phase: Phase,
}

impl Snapshot for Engine {
    type Output = snapshot::Engine;

    fn snapshot(&self) -> Self::Output {
        snapshot::Engine {
            game: self.game.snapshot(),
            actor: self.actor.snapshot(),
            phase: self.phase,
            round: self.round.current(),
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            game: Game::new(),
            actor: Actor::new(Position::new(1)),
            round: RoundId::new(0),
            phase: Phase::Lobby(LobbyPhase::Waiting),
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
        self.ensure_phase(PhaseKind::Lobby)?;

        self.game.add_player(name);
        self.assign_position(name)?;

        self.advance_phase()?;

        Ok(vec![Event::PlayerJoined {
            name: name.to_string(),
        }])
    }

    fn leave(&mut self, name: &str) -> Result<Vec<Event>> {
        self.ensure_phase(PhaseKind::Lobby)?;
        self.revoke_position(name)?;
        self.game.remove_player(name);

        self.advance_phase()?;

        Ok(vec![Event::PlayerLeft {
            name: name.to_string(),
        }])
    }

    fn assign_position(&mut self, name: &str) -> Result<Vec<Event>> {
        self.ensure_phase(PhaseKind::Lobby)?;

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
        self.ensure_phase(PhaseKind::Lobby)?;

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
        self.ensure_phase(PhaseKind::Lobby)?;

        self.game
            .players_mut()
            .sort_by_key(|p| p.position().map(|pos| pos.value()));
        self.phase = Phase::Night(NightPhase::RoleAssignment);
        self.game.round_mut(self.game.round_id());
        self.actor.set_start(self.first_speaker_of_day());

        Ok(vec![Event::GameStarted])
    }

    fn assign_role(&mut self, position: Position) -> Result<Vec<Event>> {
        self.ensure_phase(PhaseKind::Night)?;

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
        self.ensure_phase(PhaseKind::Night)?;

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
        Ok(vec![Event::PlayerKilled { target }])
    }

    fn check(&mut self, target: Position) -> Result<Vec<Event>> {
        self.ensure_phase(PhaseKind::Night)?;
        match self.phase {
            Phase::Night(NightPhase::Investigation(CheckPhase::Sheriff)) => {
                let by = self.game.sheriff();
                self.ensure_alive(by.unwrap().position().unwrap())?;
                self.ensure_alive(target)?;
                self.game
                    .round_mut(self.game.round_id())
                    .record_sheriff_check(target);
            }
            Phase::Night(NightPhase::Investigation(CheckPhase::Don)) => {
                let by = self.game.don();
                self.ensure_alive(by.unwrap().position().unwrap())?;
                self.ensure_alive(target)?;
                self.game
                    .round_mut(self.game.round_id())
                    .record_don_check(target);
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

        self.game.add_nomination(by, target)?;
        Ok(vec![Event::PlayerNominated { by, target }])
    }

    fn vote(&mut self, targets: Vec<Position>) -> Result<Vec<Event>> {
        self.ensure_phase(PhaseKind::Day)?;

        let by = self
            .actor
            .current()
            .ok_or_else(|| anyhow::anyhow!("No active speaker"))?;
        self.ensure_alive(by)?;

        for &target in &targets {
            self.ensure_alive(target)?;
            self.game.add_vote(by, target)?;
        }

        Ok(vec![])
    }

    fn turn_context(&self) -> Option<TurnContext> {
        use DayPhase::*;
        use NightPhase::*;
        use Phase::*;
        use VotingPhase::*;

        match self.phase {
            Night(RoleAssignment) => Some(TurnContext::RoleAssignment),
            Day(Discussion) => Some(TurnContext::DayDiscussion),
            Day(Voting(TieDiscussion)) => Some(TurnContext::VotingDiscussion),
            Day(Voting(VoteCast)) => Some(TurnContext::VoteCasting),
            Night(Investigation(_)) => Some(TurnContext::Investigation),
            _ => None,
        }
    }

    fn advance_actor(&mut self) -> Result<Vec<Event>> {
        let ctx = match self.turn_context() {
            Some(c) => c,
            None => return Ok(vec![]), // no turn in this phase
        };

        let next = match ctx {
            TurnContext::RoleAssignment => self.game.next_actor(&mut self.actor, |position| {
                self.game
                    .player_by_position(position)
                    .map(|p| p.role().is_none())
                    .unwrap_or(false)
            }),

            TurnContext::DayDiscussion => self.game.next_actor(&mut self.actor, |position| {
                self.game
                    .player_by_position(position)
                    .map(|p| p.is_alive())
                    .unwrap_or(false)
            }),

            TurnContext::VoteCasting => {
                let voting = self.game.voting_mut();
                voting
                    .entry(self.round)
                    .or_default()
                    .next_actor(&mut self.actor, |_| true)
            }

            TurnContext::VotingDiscussion => {
                // later: Round implements Turn
                return Ok(vec![]);
            }

            TurnContext::Investigation => {
                return Ok(vec![]);
            }
        };

        match next {
            Some(chair) => Ok(vec![Event::ActorAdvanced { chair }]),
            None => {
                self.actor.set_completed(true);
                Ok(vec![Event::TurnCompleted])
            }
        }
    }

    fn next_phase(&mut self) -> Phase {
        use CheckPhase::*;
        use DayPhase::*;
        use LobbyPhase::*;
        use NightPhase::*;
        use Phase::*;
        use VotingPhase::*;

        match self.phase {
            // -------- Lobby --------
            Lobby(Waiting) => {
                if self.game.available_positions().is_empty() {
                    Lobby(Ready)
                } else {
                    Lobby(Waiting)
                }
            }
            Lobby(Ready) => {
                if !self.game.available_positions().is_empty() {
                    Lobby(Waiting)
                } else {
                    Lobby(Ready)
                }
                // To advance from this point host should issue start command
            }

            // -------- Night --------
            Night(RoleAssignment) => {
                if self.actor.is_completed() {
                    self.actor.reset(self.first_speaker_of_day());
                    Night(SheriffReveal)
                } else {
                    Night(RoleAssignment)
                }
            }
            Night(SheriffReveal) => Night(MafiaBriefing),
            Night(MafiaBriefing) => {
                if self.game.round_id().is_first() {
                    Day(Discussion)
                } else {
                    Night(Investigation(Sheriff))
                }
            }
            Night(MafiaShoot) => {
                self.game.next_round();
                Night(Investigation(Sheriff))
            }
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
                Resolution => {
                    self.game.next_round();
                    Night(MafiaShoot)
                }
            },
        }
    }

    fn advance_phase(&mut self) -> Result<Vec<Event>> {
        let next = self.next_phase();

        self.phase = next;
        self.actor.reset(self.first_speaker_of_day());

        Ok(vec![Event::PhaseAdvanced { phase: next }])
    }

    fn first_speaker_of_day(&self) -> Position {
        ((self.game.round_id().0 % Game::PLAYER_COUNT as usize) as u8 + 1).into()
    }

    // ------------------------------
    // Guards
    // ------------------------------
    fn ensure_phase(&self, expected: PhaseKind) -> Result<()> {
        let actual = self.phase.kind();
        if actual != expected {
            bail!("Wrong phase. Expected {expected:?}, got {:?}", self.phase);
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

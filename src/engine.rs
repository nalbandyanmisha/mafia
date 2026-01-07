pub mod actor;
pub mod commands;
pub mod events;
pub mod game;
pub mod turn;

use actor::Actor;
use turn::Turn;

use self::{commands::Command, events::Event, game::Game};
use crate::{
    domain::{
        Activity, DayActivity, EngineState, EveningActivity, LobbyStatus, MorningActivity,
        NightActivity, Position, RoundId, Time,
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
        self.state = EngineState::Game(Activity::Night(NightActivity::RoleAssignment));
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
        match self.phase()? {
            Activity::Night(NightActivity::SheriffCheck) => {
                let by = self
                    .actor
                    .current()
                    .ok_or_else(|| anyhow::anyhow!("No active actor"))?;
                self.ensure_alive(by)?;
                self.ensure_alive(target)?;
                self.game.record_sheriff_check(self.round, target)?;
            }
            Activity::Night(NightActivity::DonCheck) => {
                let by = self
                    .actor
                    .current()
                    .ok_or_else(|| anyhow::anyhow!("No active actor"))?;
                self.ensure_alive(by)?;
                self.ensure_alive(target)?;
                self.game.record_don_check(self.round, target)?;
            }
            _ => bail!("Not in investigation phase"),
        };

        Ok(vec![])
    }

    fn nominate(&mut self, target: Position) -> Result<Vec<Event>> {
        self.ensure_discussion()?;
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
        self.ensure_evening()?;

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

    fn vote_result(&self) -> Result<(), anyhow::Error> {
        let voting = self
            .game
            .voting()
            .get(&self.round)
            .ok_or_else(|| anyhow::anyhow!("No voting data for current round"))?;

        voting.compute_vote_results();
        Ok(())
    }

    pub fn advance_actor(&mut self) -> Result<Vec<Event>> {
        use Activity::*;
        use DayActivity::*;
        use EveningActivity::*;
        use NightActivity::*;

        let killed_player = self.game.get_kill(self.round).cloned();

        let next = match self.phase()? {
            Night(night_activity) => match night_activity {
                RoleAssignment => self.game.next_actor(&mut self.actor, |pos| {
                    self.game
                        .player_by_position(pos)
                        .map(|p| p.role().is_none())
                        .unwrap_or(false)
                }),
                // SheriffReveal => self.game.sheriff().map(|p| p.position().unwrap()),
                SheriffReveal => self.game.next_actor(&mut self.actor, |pos| {
                    self.game
                        .player_by_position(pos)
                        .map(|p| p.is_sheriff())
                        .unwrap_or(false)
                }),
                DonReveal => self.game.next_actor(&mut self.actor, |pos| {
                    self.game
                        .player_by_position(pos)
                        .map(|p| p.is_don())
                        .unwrap_or(false)
                }),
                MafiaBriefing => None,
                MafiaShooting => self.game.next_actor(&mut self.actor, |pos| {
                    self.game
                        .player_by_position(pos)
                        .map(|p| p.is_mafia())
                        .unwrap_or(false)
                }),
                SheriffCheck => self.game.next_actor(&mut self.actor, |pos| {
                    self.game
                        .player_by_position(pos)
                        .map(|p| p.is_sheriff())
                        .unwrap_or(false)
                }),
                DonCheck => self.game.next_actor(&mut self.actor, |pos| {
                    self.game
                        .player_by_position(pos)
                        .map(|p| p.is_don())
                        .unwrap_or(false)
                }),
            },

            Morning(morning_activity) => match morning_activity {
                MorningActivity::Guessing => self.single_actor(killed_player),
                MorningActivity::FinalSpeech => self.single_actor(killed_player),
            },

            Day(Discussion) => self.game.next_actor(&mut self.actor, |pos| {
                self.game
                    .player_by_position(pos)
                    .map(|p| p.is_alive())
                    .unwrap_or(false)
            }),

            Evening(NominationAnnouncement) => None,
            Evening(Voting) => self
                .game
                .voting_mut()
                .entry(self.round)
                .or_default()
                .next_actor(&mut self.actor, |_| true),
            Evening(TieDiscussion) => self
                .game
                .voting_mut()
                .entry(self.round)
                .or_default()
                .next_actor(&mut self.actor, |_| true),
            Evening(TieVoting) => self
                .game
                .voting_mut()
                .entry(self.round)
                .or_default()
                .next_actor(&mut self.actor, |_| true),
            Evening(FinalVoting) => self
                .game
                .voting_mut()
                .entry(self.round)
                .or_default()
                .next_actor(&mut self.actor, |_| true),
            Evening(FinalSpeech) => self
                .game
                .voting_mut()
                .entry(self.round)
                .or_default()
                .next_actor(&mut self.actor, |_| true),
        };

        match next {
            Some(chair) => Ok(vec![Event::ActorAdvanced { chair }]),
            None => Ok(vec![Event::TurnCompleted]),
        }
    }

    fn single_actor(&mut self, pos: Option<Position>) -> Option<Position> {
        if self.actor.is_completed() {
            return None;
        }

        if self.actor.current().is_none() {
            self.actor.set_current(pos);
            pos
        } else {
            self.actor.set_completed(true);
            None
        }
    }

    fn next_phase(&mut self, phase: Activity) -> Activity {
        use Activity::*;
        use DayActivity::*;
        use EveningActivity::*;
        use NightActivity::*;

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
                    Night(SheriffCheck)
                }
            }
            Night(MafiaShooting) => Night(SheriffCheck),
            Night(SheriffCheck) => Night(DonCheck),
            Night(DonCheck) => Morning(MorningActivity::Guessing),

            // -------- Morning --------
            Morning(MorningActivity::Guessing) => Morning(MorningActivity::FinalSpeech),
            Morning(MorningActivity::FinalSpeech) => Day(Discussion),

            // -------- Day --------
            Day(Discussion) => Evening(NominationAnnouncement),

            // -------- Evening --------
            Evening(NominationAnnouncement) => Evening(Voting),
            Evening(Voting) => Evening(TieDiscussion),
            Evening(TieDiscussion) => Evening(TieVoting),
            Evening(TieVoting) => Evening(FinalVoting),
            Evening(FinalVoting) => Evening(FinalSpeech),
            Evening(FinalSpeech) => Night(MafiaShooting),
        }
    }

    fn advance_phase(&mut self) -> Result<Vec<Event>> {
        let current = self.phase()?;
        let next = self.next_phase(current);

        if next == Activity::Night(NightActivity::MafiaShooting) {
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

    fn phase(&self) -> Result<Activity> {
        match self.state {
            EngineState::Game(phase) => Ok(phase),
            _ => bail!("Engine is not in Game state"),
        }
    }

    fn set_phase(&mut self, phase: Activity) -> Result<()> {
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
            Activity::Night(NightActivity::RoleAssignment) => Ok(()),
            other => bail!("Engine is not in Role Assignment phase, got {other:?}"),
        }
    }

    pub fn ensure_don_check(&self, position: Position) -> Result<()> {
        match self.phase()? {
            Activity::Night(NightActivity::DonCheck) => {
                let by = self.game.don();
                if by.unwrap().position().unwrap() != position {
                    bail!("Player {position:?} is not Don");
                }
                Ok(())
            }
            other => bail!("Engine is not in Don Check phase, got {other:?}"),
        }
    }

    pub fn ensure_sheriff_check(&self, position: Position) -> Result<()> {
        match self.phase()? {
            Activity::Night(NightActivity::SheriffCheck) => {
                let by = self.game.sheriff();
                if by.unwrap().position().unwrap() != position {
                    bail!("Player {position:?} is not Sheriff");
                }
                Ok(())
            }
            other => bail!("Engine is not in Sheriff Check phase, got {other:?}"),
        }
    }

    pub fn ensure_discussion(&self) -> Result<()> {
        match &self.phase()? {
            Activity::Day(DayActivity::Discussion) => Ok(()),
            other => bail!("Engine is not in Discussion phase, got {other:?}"),
        }
    }

    pub fn ensure_evening(&self) -> Result<()> {
        match &self.phase()?.time() {
            Time::Evening => Ok(()),
            other => bail!("Engine is not in Voting phase, got {other:?}"),
        }
    }

    fn ensure_phase(&self, expected: Time) -> Result<()> {
        let phase = self.phase()?;
        if phase.time() != expected {
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
    WrongPhase {
        expected: Activity,
        current: Activity,
    },
    #[error("Player {0:?} is dead")]
    PlayerDead(Position),
    #[error("No active speaker")]
    NoActiveSpeaker,
    #[error("Player at chair {0:?} has already nominated this round")]
    AlreadyNominated(Position),
}

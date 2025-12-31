pub mod commands;

pub mod events;
pub mod state;
pub mod turn;

use self::{
    commands::Command,
    events::Event,
    state::{State, player::Player, round::RoundId, table::Table, table::chair::Chair},
};
use crate::domain::phase::{
    CheckPhase, DayPhase, LobbyPhase, NightPhase, Phase, PhaseKind, TurnContext, VotingPhase,
};
use anyhow::{Result, bail};
use rand::prelude::*;
use turn::Turn;

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
            Command::Start => self.start(),
            Command::AssignRole => self.assign_role(self.state.actor.current().unwrap()),
            Command::RevokeRole => self.revoke_role(self.state.actor.current().unwrap()),
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
        self.ensure_phase(PhaseKind::Lobby)?;

        // Pick random seat
        let chair = *self
            .state
            .available_seats()
            .choose(&mut rand::rng())
            .ok_or_else(|| anyhow::anyhow!("No available seats"))?;
        self.state.take_seat(chair)?;

        self.state
            .assign_player(chair, Player::new(name.to_string()))?;

        self.advance_phase()?;

        Ok(vec![Event::PlayerJoined {
            player: name.to_string(),
            chair,
        }])
    }

    fn leave(&mut self, name: &str) -> Result<Vec<Event>> {
        self.ensure_phase(PhaseKind::Lobby)?;
        let chair = self
            .state
            .chair_of_player(name)
            .ok_or_else(|| anyhow::anyhow!("Player not found"))?;

        self.state.return_seat(chair);
        self.state.remove_player(chair)?;

        self.advance_phase()?;

        Ok(vec![Event::PlayerLeft {
            player: name.to_string(),
            chair,
        }])
    }

    fn start(&mut self) -> Result<Vec<Event>> {
        self.ensure_phase(PhaseKind::Lobby)?;

        self.state
            .set_phase(Phase::Night(NightPhase::RoleAssignment));
        self.state.current_round = RoundId(0);
        self.state.rounds.insert(RoundId(0), Default::default());

        Ok(vec![Event::GameStarted])
    }

    fn assign_role(&mut self, chair: Chair) -> Result<Vec<Event>> {
        self.ensure_phase(PhaseKind::Night)?;
        if self.state.player(chair)?.role().is_some() {
            return Ok(vec![]); // already has role
        }
        // Pick random role
        let role = *self
            .state
            .available_roles()
            .choose(&mut rand::rng())
            .ok_or_else(|| anyhow::anyhow!("No available roles"))?;
        self.state.take_role(role)?;

        let player = self.state.player_mut(chair)?;
        player.set_role(Some(role));
        Ok(vec![])
    }

    fn revoke_role(&mut self, chair: Chair) -> Result<Vec<Event>> {
        self.ensure_phase(PhaseKind::Night)?;

        let player = self.state.player(chair)?.clone();
        self.state.return_role(player.role().unwrap());

        let player = self.state.player_mut(chair)?;
        player.set_role(None);
        Ok(vec![])
    }

    // ------------------------------
    // Player actions
    // ------------------------------
    fn warn(&mut self, target: Chair) -> Result<Vec<Event>> {
        self.ensure_alive(target)?;
        let warnings = self.state.increment_player_warning(target)?;
        Ok(vec![Event::PlayerWarned { target, warnings }])
    }

    fn pardon(&mut self, target: Chair) -> Result<Vec<Event>> {
        let warnings = self.state.deincrement_player_warning(target)?;
        Ok(vec![Event::PlayerPardoned { target, warnings }])
    }

    fn shoot(&mut self, target: Chair) -> Result<Vec<Event>> {
        self.ensure_alive(target)?;
        self.state.mark_player_killed(target)?;
        Ok(vec![Event::PlayerKilled { target }])
    }

    fn check(&mut self, target: Chair) -> Result<Vec<Event>> {
        self.ensure_phase(PhaseKind::Night)?;
        match self.state.phase() {
            Phase::Night(NightPhase::Investigation(CheckPhase::Sheriff)) => {
                let by = self.state.table.sheriff();
                self.ensure_alive(by.unwrap())?;
                self.ensure_alive(target)?;
                self.state.current_round_mut().record_sheriff_check(target);
            }
            Phase::Night(NightPhase::Investigation(CheckPhase::Don)) => {
                let by = self.state.table.don();
                self.ensure_alive(by.unwrap())?;
                self.ensure_alive(target)?;
                self.state.current_round_mut().record_don_check(target);
            }
            _ => bail!("Not in investigation phase"),
        };

        Ok(vec![])
    }

    fn nominate(&mut self, target: Chair) -> Result<Vec<Event>> {
        self.ensure_phase(PhaseKind::Day)?;
        let by = self
            .state
            .actor
            .current()
            .ok_or_else(|| anyhow::anyhow!("No active speaker"))?;
        self.ensure_alive(by)?;
        self.ensure_alive(target)?;

        self.state.add_nomination(by, target)?;
        Ok(vec![Event::PlayerNominated { by, target }])
    }

    fn vote(&mut self, targets: Vec<Chair>) -> Result<Vec<Event>> {
        self.ensure_phase(PhaseKind::Day)?;

        let by = self
            .state
            .actor
            .current()
            .ok_or_else(|| anyhow::anyhow!("No active speaker"))?;
        self.ensure_alive(by)?;

        for &target in &targets {
            self.ensure_alive(target)?;
            self.state.add_vote(by, target)?;
        }

        Ok(vec![])
    }

    fn turn_context(&self) -> Option<TurnContext> {
        use DayPhase::*;
        use NightPhase::*;
        use Phase::*;
        use VotingPhase::*;

        match self.state.phase() {
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
            TurnContext::RoleAssignment => {
                self.state.table.next_actor(&mut self.state.actor, |chair| {
                    self.state
                        .table
                        .get_player(&chair)
                        .map(|p| p.role().is_none())
                        .unwrap_or(false)
                })
            }

            TurnContext::DayDiscussion => {
                self.state.actor.set_start(self.first_speaker_of_day());
                self.state.table.next_actor(&mut self.state.actor, |chair| {
                    self.state
                        .table
                        .get_player(&chair)
                        .map(|p| p.is_alive())
                        .unwrap_or(false)
                })
            }

            TurnContext::VoteCasting => {
                let round = self.state.current_round_mut();
                round
                    .voting
                    .clone()
                    .unwrap()
                    .next_actor(&mut self.state.actor, |_| true)
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
                self.state.actor.set_completed(true);
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

        match self.state.phase() {
            // -------- Lobby --------
            Lobby(Waiting) => {
                if self.state.available_seats().is_empty() {
                    Lobby(Ready)
                } else {
                    Lobby(Waiting)
                }
            }
            Lobby(Ready) => {
                if !self.state.available_seats().is_empty() {
                    Lobby(Waiting)
                } else {
                    Night(RoleAssignment)
                }
                // To advance from this point host should issue NextPhase command
            }

            // -------- Night --------
            Night(RoleAssignment) => {
                if self.state.actor.is_completed() {
                    Night(SheriffReveal)
                } else {
                    Night(RoleAssignment)
                }
            }
            Night(SheriffReveal) => Night(MafiaBriefing),
            Night(MafiaBriefing) => {
                if self.state.current_round == RoundId(0) {
                    Day(Morning)
                } else {
                    Night(Investigation(Sheriff))
                }
            }
            Night(MafiaShoot) => Night(Investigation(Sheriff)),
            Night(Investigation(Sheriff)) => Night(Investigation(Don)),
            Night(Investigation(Don)) => Day(Morning),

            // -------- Day --------
            Day(Morning) => Day(Discussion),
            Day(Discussion) => Day(Voting(Nomination)),

            // -------- Voting --------
            Day(Voting(v)) => match v {
                Nomination => Day(Voting(VoteCast)),
                VoteCast => Day(Voting(Resolution)),
                TieDiscussion => Day(Voting(TieRevote)),
                TieRevote => Day(Voting(Resolution)),
                Resolution => {
                    self.state.start_new_round();
                    Night(MafiaShoot)
                }
            },
        }
    }

    fn advance_phase(&mut self) -> Result<Vec<Event>> {
        let next = self.next_phase();

        self.state.set_phase(next);
        self.state.actor.reset();

        Ok(vec![Event::PhaseAdvanced { phase: next }])
    }

    fn first_speaker_of_day(&self) -> Chair {
        self.chair_from_position((self.state.current_round.0 % Table::SEATS as usize) as u8 + 1)
            .unwrap()
    }

    // ------------------------------
    // Guards
    // ------------------------------
    fn ensure_phase(&self, expected: PhaseKind) -> Result<()> {
        let actual = self.state.phase().kind();
        if actual != expected {
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

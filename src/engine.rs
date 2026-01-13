pub mod actor;
pub mod commands;
pub mod game;
pub mod turn;

use std::fmt;

use actor::Actor;
use serde::Serialize;
use turn::Turn;

use self::{commands::Command, game::Game};
use crate::{
    domain::{
        Activity, DayIndex, EngineState, EveningActivity, LobbyStatus, MorningActivity,
        NightActivity, NoonActivity, Position,
    },
    snapshot::{self, Snapshot},
};
use anyhow::{Ok, Result, anyhow, bail};
use rand::prelude::*;

#[derive(Debug)]
pub struct Engine {
    pub game: Game,
    pub actor: Actor,
    pub day: DayIndex,
    pub state: EngineState,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // delegation
    #[error(transparent)]
    Game(#[from] game::Error),

    #[error(transparent)]
    Player(#[from] game::player::Error),

    #[error(transparent)]
    Voting(#[from] game::voting::Error),

    #[error(transparent)]
    Check(#[from] game::check::Error),
}

#[derive(Debug, Clone, Serialize)]
pub enum Event {
    // lifecycle
    GameStarted,
    PhaseAdvanced { from: Activity, to: Activity },

    // actor
    ActorAdvanced { to: Position },

    // domain passthrough
    Game(game::Event),
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::GameStarted => {
                write!(f, "Game has started")
            }
            Event::PhaseAdvanced { from, to } => {
                write!(f, "Game phase advanced from  {from} to {to}")
            }
            Event::ActorAdvanced { to } => {
                write!(f, "Actor advanced to {to}")
            }
            Event::Game(event) => write!(f, "{event}"),
        }
    }
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
            day: self.day.current(),
            state: self.state,
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            game: Game::new(),
            actor: Actor::new(Position::new(1)),
            day: DayIndex::new(0),
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
            Command::Advance => self.advance(),
            Command::Warn { target } => self.warn(target),
            Command::Pardon { target } => self.pardon(target),
            Command::Nominate { target } => self.nominate(target),
            Command::Vote { targets } => self.vote(targets),
            Command::Shoot { target } => self.shoot(target),
            Command::Check { target } => self.check(target),
            Command::Guess { targets } => self.guess(targets.as_slice()),
        }
    }

    // ------------------------------
    // Join / Leave
    // ------------------------------
    fn join(&mut self, name: &str) -> Result<Vec<Event>, anyhow::Error> {
        self.ensure_lobby_waiting()?;

        let mut events = Vec::new();
        events.extend(self.game.add_player(name)?);
        self.assign_position(name)?;

        // As for now position assignment is happning simultaneously with joining,
        // this is good enough but if seprate those processes later, this logic should be updated
        if self.game.available_positions().is_empty() {
            self.state = EngineState::Lobby(LobbyStatus::Ready);
        }

        Ok(events.into_iter().map(Event::Game).collect())
    }

    fn leave(&mut self, name: &str) -> Result<Vec<Event>> {
        self.ensure_lobby()?;
        let mut events = Vec::new();
        events.extend(self.revoke_position(name)?);

        self.game.remove_player(name)?;

        // As for now position assignment is happning simultaneously with joining,
        // this is good enough but if seprate those processes later, this logic should be updated
        if self.state == EngineState::Lobby(LobbyStatus::Ready)
            && self.game.available_positions().len() == 1
        {
            self.state = EngineState::Lobby(LobbyStatus::Waiting);
        }

        Ok(vec![Event::Game(game::Event::PlayerLeft {
            name: name.to_string(),
        })])
    }

    fn assign_position(&mut self, name: &str) -> Result<Vec<Event>, anyhow::Error> {
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
        let events = player.assign_position(position)?;

        Ok(events
            .into_iter()
            .map(game::Event::Player)
            .map(Event::Game)
            .collect())
    }

    fn revoke_position(&mut self, name: &str) -> Result<Vec<Event>, anyhow::Error> {
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
        let events = player.revoke_position()?;

        self.game.return_position(position);
        Ok(events
            .into_iter()
            .map(game::Event::Player)
            .map(Event::Game)
            .collect())
    }

    fn start(&mut self) -> Result<Vec<Event>> {
        self.ensure_lobby_ready()?;

        self.game
            .players_mut()
            .sort_by_key(|p| p.position().map(|pos| pos.value()));
        self.state = EngineState::Game(Activity::Night(NightActivity::RoleAssignment));
        self.actor.reset(self.first_speaker_of_day());

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
        player.assign_role(role)?;
        Ok(vec![])
    }

    fn revoke_role(&mut self, position: Position) -> Result<Vec<Event>> {
        self.ensure_role_assignment()?;

        let player = self.game.player_by_position(position).unwrap();
        self.game.return_role(player.role().unwrap());

        let player = self.game.player_by_position_mut(position).unwrap();
        player.revoke_role()?;
        Ok(vec![])
    }

    // ------------------------------
    // Player actions
    // ------------------------------

    fn warn(&mut self, target: Position) -> Result<Vec<Event>, anyhow::Error> {
        self.ensure_alive(target)?;

        let events = self
            .game
            .player_by_position_mut(target)
            .ok_or(Error::Game(game::Error::PlayerByPositionNotFound(target)))?
            .warn()?; // returns Vec<player::Event>

        Ok(events
            .into_iter()
            .map(game::Event::Player)
            .map(Event::Game)
            .collect())
    }

    fn pardon(&mut self, target: Position) -> Result<Vec<Event>> {
        self.ensure_alive(target)?;
        let player = self.game.player_by_position_mut(target).unwrap();
        let events = player.pardon()?;
        Ok(events
            .into_iter()
            .map(game::Event::Player)
            .map(Event::Game)
            .collect())
    }

    fn shoot(&mut self, target: Position) -> Result<Vec<Event>, anyhow::Error> {
        self.ensure_alive(target)?;
        let player = self.game.player_by_position_mut(target).unwrap();
        let events = player.mark_dead()?;
        self.game.record_mafia_kill(self.day, target)?;
        Ok(events
            .into_iter()
            .map(game::Event::Player)
            .map(Event::Game)
            .collect())
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
                self.game.record_sheriff_check(self.day, target)?;
            }
            Activity::Night(NightActivity::DonCheck) => {
                let by = self
                    .actor
                    .current()
                    .ok_or_else(|| anyhow::anyhow!("No active actor"))?;
                self.ensure_alive(by)?;
                self.ensure_alive(target)?;
                self.game.record_don_check(self.day, target)?;
            }
            _ => bail!("Not in investigation phase"),
        };

        Ok(vec![])
    }

    fn guess(&mut self, geusses: &[Position]) -> Result<Vec<Event>> {
        for guess in geusses {
            self.game.record_guess(*guess)?;
        }
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

        let events = self.game.add_nomination(self.day, by, target)?;
        Ok(events.into_iter().map(Event::Game).collect())
    }

    fn vote(&mut self, voters: Vec<Position>) -> Result<Vec<Event>> {
        let phase = self.phase()?;

        match phase {
            // ---------- FIRST VOTING ----------
            Activity::Evening(EveningActivity::Voting) => {
                let nominee = self
                    .actor
                    .current()
                    .ok_or_else(|| anyhow!("No active nominee. Call AdvanceActor first"))?;

                for voter in voters {
                    self.game
                        .add_vote(self.day, game::Pool::Main, voter, nominee)?;
                }
            }

            // ---------- TIE VOTING ----------
            Activity::Evening(EveningActivity::TieVoting) => {
                let nominee = self
                    .actor
                    .current()
                    .ok_or_else(|| anyhow!("No active nominee. Call AdvanceActor first"))?;

                for voter in voters {
                    self.game
                        .add_vote(self.day, game::Pool::Tie, voter, nominee)?;
                }
            }

            // ---------- FINAL YES / NO ----------
            Activity::Evening(EveningActivity::FinalVoting) => {
                for voter in voters {
                    self.game.add_vote(
                        self.day,
                        game::Pool::Final,
                        voter,
                        Position::new(0), /* placholder TODO */
                    )?;
                }
            }

            _ => bail!("Not in a voting phase"),
        }

        Ok(vec![])
    }

    fn advance(&mut self) -> Result<Vec<Event>> {
        use Activity::*;
        use EveningActivity::*;
        use MorningActivity::*;
        use NightActivity::*;
        use NoonActivity::*;

        let current = self.phase()?;
        let next = self.next(current);

        let event = match current {
            // -------- Night --------
            Night(RoleAssignment) => {
                if self.actor.current().is_some_and(|position| {
                    !self
                        .game
                        .player_by_position(position)
                        .expect("Player at {position} must exist")
                        .has_role()
                }) {
                    return Ok(vec![]);
                }
                self.game.next_actor(&mut self.actor, |pos| {
                    self.game
                        .player_by_position(pos)
                        .map(|p| p.role().is_none())
                        .unwrap_or(false)
                });

                if self.actor.is_completed() {
                    self.actor.reset(
                        self.game
                            .sheriff()
                            .expect("Sheriff should exist")
                            .position()
                            .expect("Sheriff must have assigned position"),
                    );

                    self.set_phase(next)?;
                    vec![Event::PhaseAdvanced {
                        from: current,
                        to: next,
                    }]
                } else {
                    vec![Event::ActorAdvanced {
                        to: self
                            .actor
                            .current()
                            .expect("Actor must exist at role assignment"),
                    }]
                }
            }
            Night(SheriffReveal) => {
                self.game.next_actor(&mut self.actor, |pos| {
                    self.game
                        .player_by_position(pos)
                        .map(|p| p.is_sheriff())
                        .unwrap_or(false)
                });

                if self.actor.is_completed() {
                    self.actor.reset(
                        self.game
                            .don()
                            .expect("Don should exist")
                            .position()
                            .expect("Don must have assigned position"),
                    );
                    self.set_phase(next)?;
                    vec![Event::PhaseAdvanced {
                        from: current,
                        to: next,
                    }]
                } else {
                    vec![Event::ActorAdvanced {
                        to: self
                            .actor
                            .current()
                            .expect("Actor must exist at sheriff reveal"),
                    }]
                }
            }
            Night(DonReveal) => {
                self.game.next_actor(&mut self.actor, |pos| {
                    self.game
                        .player_by_position(pos)
                        .map(|p| p.is_don())
                        .unwrap_or(false)
                });

                if self.actor.is_completed() {
                    // Todo
                    // First speaker just to pass correct datatype, does not matter value here.
                    // will fix this to avoid missunderstanding
                    self.actor.reset(self.first_speaker_of_day());
                    self.set_phase(next)?;
                    vec![Event::PhaseAdvanced {
                        from: current,
                        to: next,
                    }]
                } else {
                    vec![Event::ActorAdvanced {
                        to: self
                            .actor
                            .current()
                            .expect("Actor must exist at don reveal"),
                    }]
                }
            }
            Night(MafiaBriefing) => {
                self.actor.reset(self.first_speaker_of_day());
                self.set_phase(next)?;
                vec![Event::PhaseAdvanced {
                    from: current,
                    to: next,
                }]
            }

            Night(MafiaShooting) => {
                self.actor.reset(
                    self.game
                        .sheriff()
                        .expect("Sheriff should exist")
                        .position()
                        .expect("Sheriff must have assigned position"),
                );
                self.set_phase(next)?;
                vec![Event::PhaseAdvanced {
                    from: current,
                    to: next,
                }]
            }
            Night(SheriffCheck) => {
                self.game.next_actor(&mut self.actor, |pos| {
                    self.game
                        .player_by_position(pos)
                        .map(|p| p.is_sheriff())
                        .unwrap_or(false)
                });

                if self.actor.is_completed() {
                    self.actor.reset(
                        self.game
                            .don()
                            .expect("Don should exist")
                            .position()
                            .expect("Don must have assigned position"),
                    );
                    self.set_phase(next)?;
                    vec![Event::PhaseAdvanced {
                        from: current,
                        to: next,
                    }]
                } else {
                    vec![Event::ActorAdvanced {
                        to: self
                            .actor
                            .current()
                            .expect("Actor must exist at sheriff reveal"),
                    }]
                }
            }
            Night(DonCheck) => {
                self.game.next_actor(&mut self.actor, |pos| {
                    self.game
                        .player_by_position(pos)
                        .map(|p| p.is_don())
                        .unwrap_or(false)
                });

                if self.actor.is_completed() {
                    if next == Noon(Discussion) {
                        self.actor.reset(self.first_speaker_of_day());
                        self.set_phase(next)?;
                        vec![Event::PhaseAdvanced {
                            from: current,
                            to: next,
                        }]
                    } else {
                        self.actor.reset(
                            *self
                                .game
                                .get_kill(self.day)
                                .expect("Killed player must exist"),
                        );
                        self.set_phase(next)?;
                        vec![Event::PhaseAdvanced {
                            from: current,
                            to: next,
                        }]
                    }
                } else {
                    vec![Event::ActorAdvanced {
                        to: self
                            .actor
                            .current()
                            .expect("Actor must exist at don reveal"),
                    }]
                }
            }

            // -------- Morning --------
            Morning(Guessing) => {
                let killed_p = self
                    .game
                    .get_kill(self.day)
                    .expect("Killed player must exist");
                self.game.next_actor(&mut self.actor, |pos| {
                    self.game
                        .player_by_position(pos)
                        .map(|p| p.is_dead() && *killed_p == p.position().unwrap())
                        .unwrap_or(false)
                });

                if self.actor.is_completed() {
                    self.actor.reset(
                        *self
                            .game
                            .get_kill(self.day)
                            .expect("Killed player must exist"),
                    );
                    self.set_phase(next)?;
                    vec![Event::PhaseAdvanced {
                        from: current,
                        to: next,
                    }]
                } else {
                    vec![Event::ActorAdvanced {
                        to: self
                            .actor
                            .current()
                            .expect("Actor must exist at don reveal"),
                    }]
                }
            }
            Morning(DeathSpeech) => {
                let killed_p = self
                    .game
                    .get_kill(self.day)
                    .expect("Killed player must exist");
                self.game.next_actor(&mut self.actor, |pos| {
                    self.game
                        .player_by_position(pos)
                        .map(|p| p.is_dead() && *killed_p == p.position().unwrap())
                        .unwrap_or(false)
                });

                if self.actor.is_completed() {
                    self.actor.reset(self.first_speaker_of_day());
                    self.set_phase(next)?;
                    vec![Event::PhaseAdvanced {
                        from: current,
                        to: next,
                    }]
                } else {
                    vec![Event::ActorAdvanced {
                        to: self
                            .actor
                            .current()
                            .expect("Actor must exist at don reveal"),
                    }]
                }
            }

            // -------- Day --------
            Noon(Discussion) => {
                self.game.next_actor(&mut self.actor, |pos| {
                    self.game
                        .player_by_position(pos)
                        .map(|p| p.is_alive())
                        .unwrap_or(false)
                });

                if self.actor.is_completed() {
                    if next == Night(MafiaShooting) {
                        self.day.advance();
                    }
                    // Todo
                    // First speaker just to pass correct datatype, does not matter value here.
                    // will fix this to avoid missunderstanding

                    self.actor.reset(self.first_speaker_of_day());
                    self.set_phase(next)?;
                    self.set_phase(next)?;
                    vec![Event::PhaseAdvanced {
                        from: current,
                        to: next,
                    }]
                } else {
                    vec![]
                }
            }

            // -------- Evening --------
            Evening(NominationAnnouncement) => {
                let nominees = self
                    .game
                    .voting()
                    .get(&self.day)
                    .expect("Voting must exist")
                    .get_nominees();
                self.actor.reset(nominees[0]);
                self.set_phase(next)?;
                vec![Event::PhaseAdvanced {
                    from: current,
                    to: next,
                }]
            }
            Evening(Voting) => {
                let voting = self
                    .game
                    .voting()
                    .get(&self.day)
                    .expect("Voting must exist");

                voting.next_actor(&mut self.actor, |_| true);

                if self.actor.is_completed() {
                    let winners = voting.winners();

                    if winners.len() == 1 {
                        self.game.record_eliminated(self.day, &winners)?;
                        self.game
                            .player_by_position_mut(winners[0])
                            .unwrap()
                            .mark_eliminated()?;
                        self.actor.reset(winners[0]);
                        self.set_phase(next)?;
                    } else {
                        self.game
                            .tie_voting_mut()
                            .insert(self.day, game::voting::Voting::from_nominees(&winners));
                        let tie_nominees = self
                            .game
                            .tie_voting()
                            .get(&self.day)
                            .expect("Tie Voting must exist")
                            .get_nominees();
                        self.actor.reset(tie_nominees[0]);
                        self.set_phase(next)?;
                    }
                    vec![Event::PhaseAdvanced {
                        from: current,
                        to: next,
                    }]
                } else {
                    vec![]
                }
            }
            Evening(TieDiscussion) => {
                let voting = self
                    .game
                    .tie_voting()
                    .get(&self.day)
                    .expect("Voting must exist");

                voting.next_actor(&mut self.actor, |_| true);

                if self.actor.is_completed() {
                    self.actor.reset(voting.get_nominees()[0]);
                    self.set_phase(next)?;
                    vec![Event::PhaseAdvanced {
                        from: current,
                        to: next,
                    }]
                } else {
                    vec![]
                }
            }
            Evening(TieVoting) => {
                let voting = self
                    .game
                    .tie_voting()
                    .get(&self.day)
                    .expect("Tie Voting must exist");

                voting.next_actor(&mut self.actor, |_| true);

                if self.actor.is_completed() {
                    let winners = voting.winners();

                    if winners.len() == 1 {
                        self.game.record_eliminated(self.day, &winners)?;
                        self.game
                            .player_by_position_mut(winners[0])
                            .unwrap()
                            .mark_eliminated()?;
                        self.actor.reset(winners[0]);
                        self.set_phase(next)?;
                    } else {
                        self.set_phase(next)?;
                    }

                    vec![Event::PhaseAdvanced {
                        from: current,
                        to: next,
                    }]
                } else {
                    vec![]
                }
            }
            Evening(FinalVoting) => {
                let yes_count = self
                    .game
                    .final_voting()
                    .get(&self.day)
                    .expect("Final voting results")
                    .len();
                let alive_count = self.game.alive_players();

                let nominees = self
                    .game
                    .tie_voting()
                    .get(&self.day)
                    .map(|v| v.get_nominees().to_vec())
                    .unwrap_or_default();

                if yes_count > alive_count / 2 {
                    self.game.record_eliminated(self.day, &nominees)?;
                    for nominee in &nominees {
                        self.game
                            .player_by_position_mut(*nominee)
                            .unwrap()
                            .mark_eliminated()?;
                    }
                    self.actor.reset(nominees[0]);
                } else {
                    self.actor.reset(self.first_speaker_of_day());
                    self.day.advance();
                }

                self.set_phase(next)?;
                vec![Event::PhaseAdvanced {
                    from: current,
                    to: next,
                }]
            }
            Evening(FinalSpeech) => {
                let eliminated = self
                    .game
                    .get_eliminated(self.day)
                    .expect("there shoul be eliminted players, at least one");
                self.game.next_actor(&mut self.actor, |pos| {
                    self.game
                        .player_by_position(pos)
                        .map(|p| p.is_eliminated() && eliminated.contains(&pos))
                        .unwrap_or(false)
                });

                if self.actor.is_completed() {
                    self.set_phase(next)?;
                    self.day.advance();
                    vec![Event::PhaseAdvanced {
                        from: current,
                        to: next,
                    }]
                } else {
                    vec![]
                }
            }
        };

        Ok(event)
    }

    fn next(&self, phase: Activity) -> Activity {
        use Activity::*;
        use EveningActivity::*;
        use MorningActivity::*;
        use NightActivity::*;
        use NoonActivity::*;

        match phase {
            // -------- Night --------
            Night(RoleAssignment) => {
                if self.game.players().iter().all(|p| p.role().is_some()) {
                    Night(SheriffReveal)
                } else {
                    Night(RoleAssignment)
                }
            }
            Night(SheriffReveal) => Night(DonReveal),
            Night(DonReveal) => Night(MafiaBriefing),
            Night(MafiaBriefing) => Noon(Discussion),
            Night(MafiaShooting) => Night(SheriffCheck),
            Night(SheriffCheck) => Night(DonCheck),
            Night(DonCheck) => {
                if self.day.is_second() && self.game.get_kill(self.day).is_some() {
                    Morning(Guessing)
                } else if self.game.get_kill(self.day).is_some() {
                    Morning(DeathSpeech)
                } else {
                    Noon(Discussion)
                }
            }

            // -------- Morning --------
            Morning(Guessing) => Morning(DeathSpeech),
            Morning(DeathSpeech) => Noon(Discussion),

            // -------- Day --------
            Noon(Discussion) => {
                let voting = game::voting::Voting::new();
                let voting = self.game.voting().get(&self.day).unwrap_or(&voting);

                if voting.has_nominees() {
                    if self.day.is_first() && voting.nominee_count() == 1 {
                        Night(MafiaShooting)
                    } else {
                        Evening(NominationAnnouncement)
                    }
                } else {
                    Night(MafiaShooting)
                }
            }

            // -------- Evening --------
            Evening(NominationAnnouncement) => Evening(Voting),
            Evening(Voting) => {
                let voting = self
                    .game
                    .voting()
                    .get(&self.day)
                    .expect("Voting must exist at Voting");
                if voting.winners().len() == 1 {
                    Evening(FinalSpeech)
                } else {
                    Evening(TieDiscussion)
                }
            }
            Evening(TieDiscussion) => Evening(TieVoting),
            Evening(TieVoting) => {
                let voting = self
                    .game
                    .tie_voting()
                    .get(&self.day)
                    .expect("Voting must exist at Voting");
                if voting.winners().len() == 1 {
                    Evening(FinalSpeech)
                } else {
                    Evening(FinalVoting)
                }
            }
            Evening(FinalVoting) => {
                let yes_count = self
                    .game
                    .final_voting()
                    .get(&self.day)
                    .expect("Final voting results")
                    .len();
                let alive_count = self.game.alive_players();
                if yes_count > alive_count / 2 {
                    Evening(FinalSpeech)
                } else {
                    Night(MafiaShooting)
                }
            }
            Evening(FinalSpeech) => Night(MafiaShooting),
        }
    }

    fn first_speaker_of_day(&self) -> Position {
        let start: Position = ((self.day.0 % Game::PLAYER_COUNT as usize) as u8 + 1).into();

        // walk table circularly, starting from anchor
        for offset in 0..Game::PLAYER_COUNT {
            let pos: Position = (((start.value() - 1 + offset) % Game::PLAYER_COUNT) + 1).into();

            if self
                .game
                .player_by_position(pos)
                .map(|p| p.is_alive())
                .unwrap_or(false)
            {
                return pos;
            }
        }

        panic!("At least one alive player must exist");
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

    // pub fn ensure_don_check(&self, position: Position) -> Result<()> {
    //     match self.phase()? {
    //         Activity::Night(NightActivity::DonCheck) => {
    //             let by = self.game.don();
    //             if by.unwrap().position().unwrap() != position {
    //                 bail!("Player {position:?} is not Don");
    //             }
    //             Ok(())
    //         }
    //         other => bail!("Engine is not in Don Check phase, got {other:?}"),
    //     }
    // }
    //
    // pub fn ensure_sheriff_check(&self, position: Position) -> Result<()> {
    //     match self.phase()? {
    //         Activity::Night(NightActivity::SheriffCheck) => {
    //             let by = self.game.sheriff();
    //             if by.unwrap().position().unwrap() != position {
    //                 bail!("Player {position:?} is not Sheriff");
    //             }
    //             Ok(())
    //         }
    //         other => bail!("Engine is not in Sheriff Check phase, got {other:?}"),
    //     }
    // }

    pub fn ensure_discussion(&self) -> Result<()> {
        match &self.phase()? {
            Activity::Noon(NoonActivity::Discussion) => Ok(()),
            other => bail!("Engine is not in Discussion phase, got {other:?}"),
        }
    }

    // pub fn ensure_night(&self) -> Result<()> {
    //     match &self.phase()? {
    //         Activity::Night(_) => Ok(()),
    //         other => bail!("Engine is not in Night phase, got {other:?}"),
    //     }
    // }
    //
    // pub fn ensure_morning(&self) -> Result<()> {
    //     match &self.phase()? {
    //         Activity::Morning(_) => Ok(()),
    //         other => bail!("Engine is not in Morning phase, got {other:?}"),
    //     }
    // }
    //
    // pub fn ensure_noon(&self) -> Result<()> {
    //     match &self.phase()? {
    //         Activity::Noon(_) => Ok(()),
    //         other => bail!("Engine is not in Noon phase, got {other:?}"),
    //     }
    // }
    //
    // pub fn ensure_evening(&self) -> Result<()> {
    //     match &self.phase()?.daytime() {
    //         Day::Evening => Ok(()),
    //         other => bail!("Engine is not in Voting phase, got {other:?}"),
    //     }
    // }
    //
    // fn ensure_phase(&self, expected: Day) -> Result<()> {
    //     let phase = self.phase()?;
    //     if phase.daytime() != expected {
    //         bail!("Wrong phase. Expected {expected:?}, got {phase:?}");
    //     }
    //     Ok(())
    // }

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

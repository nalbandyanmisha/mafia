pub mod commands;
pub mod events;
pub mod state;

use self::{
    commands::Command,
    events::Event,
    state::{
        State,
        chair::Chair,
        nomination::Nomination,
        phase::Phase,
        player::{Player, Status as PlayerStatus},
        table::Table,
        vote::Vote,
    },
};
use anyhow::{Result, bail};

#[derive(Debug, Default)]
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
            Command::Warn { chair } => self
                .warn(chair)
                .map(|player| vec![Event::PlayerWarned { player, chair }]),
            Command::Pardon { chair } => self
                .pardon(chair)
                .map(|player| vec![Event::PlayerPardoned { player, chair }]),
            Command::Nominate { target } => {
                self.nominate(target).map(|_| vec![Event::PlayerNominated])
            }
            Command::Shoot { chair } => self
                .shoot(chair)
                .map(|player| vec![Event::PlayerKilled { player, chair }]),
            Command::NextPhase => self.next_phase().map(|_| vec![Event::PhaseAdvanced]),
            Command::NextSpeaker => self.next_speaker().map(|_| vec![]),
        }
    }

    fn join(&mut self, name: &str) -> Result<(Chair, Player), Box<dyn std::error::Error>> {
        if self.state.phase != Phase::Lobby {
            return Err("Cannot join after registration".into());
        }

        if self.state.table.available_positions.len() as u8 == 0 {
            self.state.phase = Phase::Night;
            return Ok((Chair::default(), Player::default()));
        }

        let position = match self.state.table.pick_position() {
            Some(pos) => pos,
            None => return Err("No available positions".into()),
        };

        let role = match self.state.table.pick_role() {
            Some(r) => r,
            None => return Err("No available roles".into()),
        };

        let chair = Chair::new(position);
        let player = Player::new(name.to_string(), role);
        self.state
            .table
            .chairs_to_players
            .insert(chair, player.clone());
        Ok((chair, player))
    }

    fn leave(&mut self, name: &str) -> Result<(Chair, Player), Box<dyn std::error::Error>> {
        if self.state.phase != Phase::Lobby {
            return Err("Cannot leave after registration phase".into());
        }
        if let Some((chair, player)) = self
            .state
            .table
            .chairs_to_players
            .iter()
            .find(|(_, player)| player.name == name)
            .map(|(chair, player)| (*chair, player.clone()))
        {
            self.state.table.chairs_to_players.remove(&chair);
            self.state.table.available_positions.push(chair.position);
            self.state.table.available_roles.push(player.role);
            self.state
                .table
                .chairs_to_players
                .insert(chair, Player::default());
            Ok((chair, player.clone()))
        } else {
            Err("Player not found".into())
        }
    }

    fn warn(&mut self, chair: Chair) -> Result<Player, Box<dyn std::error::Error>> {
        if let Some(player) = self.state.table.chairs_to_players.get_mut(&chair) {
            player.add_warning();
            Ok(player.clone())
        } else {
            Err("Player not found".into())
        }
    }

    fn pardon(&mut self, chair: Chair) -> Result<Player, Box<dyn std::error::Error>> {
        if let Some(player) = self.state.table.chairs_to_players.get_mut(&chair) {
            player.remove_warning();
            Ok(player.clone())
        } else {
            Err("Player not found".into())
        }
    }

    fn shoot(&mut self, chair: Chair) -> Result<Player, Box<dyn std::error::Error>> {
        if let Some(player) = self.state.table.chairs_to_players.get_mut(&chair) {
            player.status = PlayerStatus::Killed;
            Ok(player.clone())
        } else {
            Err("Player not found".into())
        }
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
                by.position
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

    fn first_speaker_of_day(&self) -> Option<Chair> {
        let start = (self.state.current_round.0 % Table::SEATS as usize) as u8 + 1;

        self.find_next_alive_chair_from(start)
    }

    fn find_next_alive_chair_from(&self, start: u8) -> Option<Chair> {
        for offset in 0..Table::SEATS {
            let pos = ((start - 1 + offset) % Table::SEATS) + 1;
            let chair = Chair::new(pos);

            if let Some(player) = self.state.table.chairs_to_players.get(&chair) {
                if player.status == PlayerStatus::Alive && !player.name.is_empty() {
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

        let next = self.find_next_alive_chair_from(current.position + 1);

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

    fn ensure_alive(&self, chair: Chair) -> Result<()> {
        let player = self
            .state
            .table
            .get_player_by_chair(&chair)
            .ok_or_else(|| anyhow::anyhow!("Player at chair {chair:?} not found"))?;

        match player.status {
            PlayerStatus::Killed | PlayerStatus::Removed | PlayerStatus::Eliminated => {
                bail!("Player at chair {chair:?} is not alive")
            }
            _ => Ok(()),
        }
    }
}

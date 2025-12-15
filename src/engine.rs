pub mod commands;
pub mod events;
pub mod state;

use self::{
    commands::Command,
    events::Event,
    state::{
        State,
        chair::Chair,
        phase::Phase,
        player::{Player, Status as PlayerStatus},
    },
};

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
            Command::Shoot { chair } => self
                .shoot(chair)
                .map(|player| vec![Event::PlayerKilled { player, chair }]),
            Command::NextPhase => self.next_phase().map(|_| vec![Event::PhaseAdvanced]),
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

    fn next_phase(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.state.phase == Phase::Voting {
            self.state.current_round = self.state.current_round.next();
        }
        self.state.phase.next().map_err(|e| e.into())
    }
}

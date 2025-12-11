use rand::prelude::*;
use std::collections::BTreeMap;
use std::fmt::Display;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Role {
    #[default]
    Citizen,
    Mafia,
    Don,
    Sheriff,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Status {
    #[default]
    Alive,
    Killed,
    Eliminated,
    Removed,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Phase {
    #[default]
    Registration,
    Night,
    Morning,
    Day,
    Voting,
}

impl Phase {
    fn next(&mut self) -> Result<(), String> {
        *self = match self {
            Phase::Registration => Phase::Night,
            Phase::Night => Phase::Morning,
            Phase::Morning => Phase::Day,
            Phase::Day => Phase::Voting,
            Phase::Voting => Phase::Night,
        };
        Ok(())
    }
}

impl Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Phase::Registration => "Registration",
            Phase::Night => "Night",
            Phase::Morning => "Morning",
            Phase::Day => "Day",
            Phase::Voting => "Voting",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub name: String,
    pub role: Role,
    pub warnings: u8,
    pub status: Status,
    // index acts as a round in the game
    pub is_nominee: Vec<bool>,
    pub nominated: Vec<Option<u8>>,
}

impl Player {
    pub fn new(name: String, role: Role) -> Self {
        Player {
            name,
            role,
            warnings: 0,
            status: Status::Alive,
            is_nominee: vec![],
            nominated: vec![],
        }
    }
    pub fn add_warning(&mut self) {
        self.warnings += 1;
    }

    pub fn reset_warnings(&mut self) {
        self.warnings = 0;
    }

    pub fn remove_warning(&mut self) {
        if self.warnings > 0 {
            self.warnings -= 1;
        }
    }

    pub fn withdraw(&mut self) {
        self.nominated.pop();
    }

    pub fn nominate(&mut self, position: Option<u8>) {
        self.nominated.push(position);
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Copy, PartialOrd, Ord)]
pub struct Chair {
    pub position: u8,
}

impl Chair {
    pub fn new(position: u8) -> Self {
        Chair { position }
    }
}

pub struct Table {
    pub chairs_to_players: BTreeMap<Chair, Player>,
    pub available_roles: Vec<Role>,
    pub available_positions: Vec<u8>,
    pub nominated_players: Vec<String>,
    pub round: u8,
    pub phase: Phase,
}

impl Table {
    pub const SEATS: u8 = 10;
    pub fn new() -> Self {
        let available_roles = vec![
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
        let available_positions: Vec<u8> = (1..=Self::SEATS).collect();
        let mut chairs_to_players: BTreeMap<Chair, Player> = BTreeMap::new();
        for position in 1..=Self::SEATS {
            let chair = Chair::new(position);
            chairs_to_players.insert(chair, Player::default());
        }
        Table {
            chairs_to_players,
            available_roles,
            available_positions,
            nominated_players: vec![],
            round: 0,
            phase: Phase::Registration,
        }
    }

    fn pick_position(&mut self) -> Option<u8> {
        let mut rng = rand::rng();
        if let Some(&position) = self.available_positions.choose(&mut rng) {
            self.available_positions.retain(|&x| x != position);
            Some(position)
        } else {
            println!("No available seats");
            None
        }
    }

    fn pick_role(&mut self) -> Option<Role> {
        let mut rng = rand::rng();
        if let Some(role) = self.available_roles.as_slice().choose(&mut rng).cloned() {
            let index = self
                .available_roles
                .iter()
                .position(|x| *x == role)
                .unwrap();
            self.available_roles.remove(index);
            Some(role)
        } else {
            println!("No available roles");
            None
        }
    }

    pub fn join(&mut self, name: &str) -> Result<(), String> {
        if self.available_positions.len() as u8 == 0 {
            self.phase = Phase::Night;
            return Ok(());
        }

        let position = match self.pick_position() {
            Some(pos) => pos,
            None => return Err("No available positions".to_string()),
        };

        let role = match self.pick_role() {
            Some(r) => r,
            None => return Err("No available roles".to_string()),
        };

        let chair = Chair::new(position);
        let player = Player::new(name.to_string(), role);
        self.chairs_to_players.insert(chair, player);
        Ok(())
    }

    pub fn leave(&mut self, name: &str) -> Result<(), String> {
        if self.phase != Phase::Registration {
            return Err("Cannot leave after registration phase".to_string());
        }
        if let Some((chair, player)) = self
            .chairs_to_players
            .iter()
            .find(|(_, player)| player.name == name)
            .map(|(chair, player)| (*chair, player.clone()))
        {
            self.chairs_to_players.remove(&chair);
            self.available_positions.push(chair.position);
            self.available_roles.push(player.role);
            self.chairs_to_players.insert(chair, Player::default());
            Ok(())
        } else {
            Err("Player not found".to_string())
        }
    }

    pub fn warn(&mut self, chair: Chair) -> Result<(), String> {
        if let Some(player) = self.chairs_to_players.get_mut(&chair) {
            player.add_warning();
            Ok(())
        } else {
            Err("Player not found".to_string())
        }
    }

    pub fn pardon(&mut self, chair: Chair) -> Result<(), String> {
        if let Some(player) = self.chairs_to_players.get_mut(&chair) {
            player.remove_warning();
            Ok(())
        } else {
            Err("Player not found".to_string())
        }
    }

    pub fn next_phase(&mut self) -> Result<(), String> {
        if self.phase == Phase::Registration && self.available_positions.len() as u8 != 0 {
            return Ok(());
        } else if self.phase == Phase::Voting {
            self.round += 1;
        }
        self.phase.next()
    }

    pub fn nominate(&mut self, chair: Chair) -> Result<(), String> {
        if let Some(player) = self.chairs_to_players.get_mut(&chair) {
            player.nominate(Some(chair.position));
            self.nominated_players.push(player.name.clone());
            Ok(())
        } else {
            Err("Player not found".to_string())
        }
    }

    pub fn get_player_by_name(&self, player_name: &str) -> Option<&Player> {
        self.chairs_to_players
            .values()
            .find(|player| player.name == player_name)
    }

    pub fn get_player_by_name_mut(&mut self, player_name: &str) -> Option<&mut Player> {
        self.chairs_to_players
            .values_mut()
            .find(|player| player.name == player_name)
    }

    pub fn get_player_by_position(&self, position: u8) -> Option<&Player> {
        let chair = Chair::new(position);
        self.chairs_to_players.get(&chair)
    }

    pub fn get_player_by_position_mut(&mut self, position: u8) -> Option<&mut Player> {
        let chair = Chair::new(position);
        self.chairs_to_players.get_mut(&chair)
    }

    pub fn update_player_status_by_position(
        &mut self,
        position: u8,
        status: Status,
    ) -> Result<(), String> {
        let chair = Chair::new(position);
        if let Some(player) = self.chairs_to_players.get_mut(&chair) {
            player.status = status;
            Ok(())
        } else {
            Err("Player not found".to_string())
        }
    }

    pub fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.phase {
            Phase::Registration => {
                println!("Phase: Registration, Round: {}", self.round);
                for (chair, player) in &self.chairs_to_players {
                    if player.name.is_empty() {
                        println!("Chair: {:?} is unoccupied", chair.position);
                    } else {
                        println!("Chair: {:?}, Player: {}", chair.position, player.name,);
                    }
                }
            }
            Phase::Night => {
                println!("Phase: Night, Round: {}", self.round);
                for (chair, player) in &self.chairs_to_players {
                    println!(
                        "Chair: {:?}, Player: {}, Role: {:?}",
                        chair.position, player.name, player.role
                    );
                }
                if self.round == 0 {
                    println!(
                        "To assigne players their roles, use the 'show' command during the night phase so you can see each player's role privately."
                    );
                    println!(
                        "After that give 5 seconds to Sheriff to investigate cityzens and 1 minute to Mafia to choose their strategy."
                    );
                    println!(
                        "Once the time is up, proceed to the morning phase using the appropriate command."
                    );
                }
            }
            Phase::Morning => {
                println!("Phase: Morning, Round: {}", self.round);
                if self.round == 0 {
                    println!(
                        "The game has begun! Welcome to the first day of Mafia. As this is a first morning, there are no shooting to announce."
                    );
                }

                for (chair, player) in &self.chairs_to_players {
                    if player.name.is_empty() {
                        println!("Chair: {:?} is unoccupied", chair.position);
                    } else {
                        println!(
                            "Chair: {:?}, Player: {}, Status: {:?}, Warnings: {}",
                            chair.position, player.name, player.status, player.warnings
                        );
                    }
                }
            }
            Phase::Day => {
                println!("Phase: Day, Round: {}", self.round);
                for (chair, player) in &self.chairs_to_players {
                    if player.name.is_empty() {
                        println!("Chair: {:?} is unoccupied", chair.position);
                    } else {
                        println!(
                            "Chair: {:?}, Player: {}, Status: {:?}, Warnings: {}",
                            chair.position, player.name, player.status, player.warnings
                        );
                    }
                }
            }
            Phase::Voting => println!("Phase: Voting"),
        }

        Ok(())
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

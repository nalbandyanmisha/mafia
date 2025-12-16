pub mod chair;

use super::{player::Player, role::Role};
use chair::{Chair, ChairError};
use rand::prelude::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Table {
    chairs_to_players: BTreeMap<Chair, Player>,
    available_roles: Vec<Role>,
    available_positions: Vec<u8>,
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

        let mut chairs_to_players = BTreeMap::new();
        for position in 1..=Self::SEATS {
            let chair = Chair::new(position);
            chairs_to_players.insert(chair, Player::default());
        }

        Table {
            chairs_to_players,
            available_roles,
            available_positions,
        }
    }

    // --------------------------------------
    // Chair creation / validation
    // --------------------------------------
    pub fn try_chair(&self, position: u8) -> Result<Chair, ChairError> {
        if (1..=Self::SEATS).contains(&position) {
            Ok(Chair::new(position))
        } else {
            Err(ChairError::InvalidPosition(position))
        }
    }

    // --------------------------------------
    // Position management
    // --------------------------------------
    pub fn pick_position(&mut self) -> Result<u8, TableError> {
        let mut rng = rand::rng();
        if let Some(&position) = self.available_positions.choose(&mut rng) {
            self.available_positions.retain(|&x| x != position);
            Ok(position)
        } else {
            Err(TableError::NoAvailableSeats)
        }
    }

    pub fn release_position(&mut self, position: u8) -> Result<(), TableError> {
        if (1..=Self::SEATS).contains(&position) {
            if !self.available_positions.contains(&position) {
                self.available_positions.push(position);
            }
            Ok(())
        } else {
            Err(TableError::InvalidChair(position))
        }
    }

    // --------------------------------------
    // Role management
    // --------------------------------------
    pub fn pick_role(&mut self) -> Result<Role, TableError> {
        let mut rng = rand::rng();

        if let Some(role) = self.available_roles.choose(&mut rng).cloned() {
            let index = self
                .available_roles
                .iter()
                .position(|r| *r == role)
                .unwrap();
            self.available_roles.remove(index);
            Ok(role)
        } else {
            Err(TableError::NoAvailableRoles)
        }
    }

    pub fn release_role(&mut self, role: Role) {
        self.available_roles.push(role);
    }

    // --------------------------------------
    // Player access
    // --------------------------------------
    pub fn get_player(&self, chair: &Chair) -> Result<&Player, TableError> {
        self.chairs_to_players
            .get(chair)
            .ok_or(TableError::PlayerNotFound(*chair))
    }

    pub fn get_player_mut(&mut self, chair: &Chair) -> Result<&mut Player, TableError> {
        self.chairs_to_players
            .get_mut(chair)
            .ok_or(TableError::PlayerNotFound(*chair))
    }

    pub fn assign_player(&mut self, chair: Chair, player: Player) -> Result<(), TableError> {
        self.chairs_to_players.insert(chair, player);
        Ok(())
    }

    pub fn remove_player(&mut self, chair: Chair) -> Option<Player> {
        let removed = self.chairs_to_players.get(&chair).cloned();
        self.chairs_to_players.insert(chair, Player::default());
        removed
    }

    pub fn all_chairs(&self) -> impl Iterator<Item = (&Chair, &Player)> {
        self.chairs_to_players.iter()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TableError {
    #[error("Invalid chair position {0}")]
    InvalidChair(u8),
    #[error("No available seats")]
    NoAvailableSeats,
    #[error("No available roles")]
    NoAvailableRoles,
    #[error("Player at chair {0:?} not found")]
    PlayerNotFound(Chair),
}

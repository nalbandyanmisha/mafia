pub mod chair;

use crate::{
    engine::turn::Turn,
    snapshot::{Snapshot, TableData},
};

use super::{actor::Actor, player::Player, role::Role};
use chair::{Chair, ChairError};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Table {
    seats: BTreeMap<Chair, Player>,
    available_roles: Vec<Role>,
    available_seats: Vec<Chair>,
}

impl Snapshot for Table {
    type Output = TableData;

    fn snapshot(&self) -> Self::Output {
        TableData {
            seats: self
                .seats
                .iter()
                .map(|(chair, player)| crate::snapshot::SeatData {
                    chair: chair.snapshot(),
                    player: if player.name().is_empty() {
                        None
                    } else {
                        Some(player.snapshot())
                    },
                })
                .collect(),
        }
    }
}

impl Turn for Table {
    fn next_actor<F>(&self, actor: &mut Actor, is_eligible: F) -> Option<Chair>
    where
        F: Fn(Chair) -> bool,
    {
        if actor.is_completed() {
            return None;
        }

        let start = actor.start();
        let start_pos = actor
            .current()
            .map(|c| c.position() + 1)
            .unwrap_or(start.position());

        for offset in 0..Self::SEATS {
            let pos = ((start_pos - 1 + offset) % Self::SEATS) + 1;

            let chair = match self.chair(pos) {
                Ok(c) => c,
                Err(_) => continue,
            };

            if !is_eligible(chair) {
                continue;
            }

            // 🚨 stop condition: looped back to start
            if Some(chair) == actor.current() {
                continue;
            }

            if Some(chair) == Some(start) && actor.current().is_some() {
                actor.set_completed(true);
                return None;
            }

            actor.set_current(Some(chair));
            return Some(chair);
        }

        actor.set_completed(true);
        None
    }
}

impl Table {
    pub const SEATS: u8 = 10;

    /// Create an empty table with all chairs initialized.
    pub fn new() -> Self {
        let mut seats = BTreeMap::new();

        for position in 1..=Self::SEATS {
            seats.insert(Chair::new(position), Player::default());
        }

        let mut available_seats = Vec::new();
        for pos in 1..=Self::SEATS {
            let chair = Chair::new(pos);
            seats.insert(chair, Player::default());
            available_seats.push(chair);
        }

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

        Self {
            seats,
            available_roles,
            available_seats,
        }
    }

    // --------------------------------------
    // Chair creation / validation
    // --------------------------------------
    pub fn chair(&self, position: u8) -> Result<Chair, ChairError> {
        if (1..=Self::SEATS).contains(&position) {
            Ok(Chair::new(position))
        } else {
            Err(ChairError::InvalidPosition(position))
        }
    }

    pub fn get_chair(&self, name: &str) -> Option<Chair> {
        for (chair, player) in &self.seats {
            if player.name() == name {
                return Some(*chair);
            }
        }
        None
    }

    // ---------------- Access available roles/seats ----------------
    pub fn available_roles(&self) -> &[Role] {
        &self.available_roles
    }

    pub fn available_seats(&self) -> &[Chair] {
        &self.available_seats
    }

    pub fn sheriff(&self) -> Option<Chair> {
        for (chair, player) in &self.seats {
            if player.role() == Some(Role::Sheriff) {
                return Some(*chair);
            }
        }
        None
    }

    pub fn don(&self) -> Option<Chair> {
        for (chair, player) in &self.seats {
            if player.role() == Some(Role::Don) {
                return Some(*chair);
            }
        }
        None
    }

    // --------------------------------------
    // Player access
    // --------------------------------------
    pub fn get_player(&self, chair: &Chair) -> Result<&Player, TableError> {
        self.seats
            .get(chair)
            .ok_or(TableError::PlayerNotFound(*chair))
    }

    pub fn get_player_mut(&mut self, chair: &Chair) -> Result<&mut Player, TableError> {
        self.seats
            .get_mut(chair)
            .ok_or(TableError::PlayerNotFound(*chair))
    }

    // ---------------- Take/Return seats/role functions ----------------
    pub fn take_role(&mut self, role: Role) -> Result<(), TableError> {
        if let Some(pos) = self.available_roles.iter().position(|r| *r == role) {
            self.available_roles.remove(pos);
            Ok(())
        } else {
            Err(TableError::NoAvailableRoles)
        }
    }

    pub fn return_role(&mut self, role: Role) {
        self.available_roles.push(role);
    }

    pub fn take_seat(&mut self, chair: Chair) -> Result<(), TableError> {
        if let Some(pos) = self.available_seats.iter().position(|c| *c == chair) {
            self.available_seats.remove(pos);
            Ok(())
        } else {
            Err(TableError::NoAvailableSeats)
        }
    }

    pub fn return_seat(&mut self, chair: Chair) {
        if !self.available_seats.contains(&chair) {
            self.available_seats.push(chair);
        }
    }

    // --------------------------------------
    // Seat mutation
    // --------------------------------------

    pub fn seat_player(&mut self, chair: Chair, player: Player) -> Result<(), TableError> {
        self.seats.insert(chair, player);
        Ok(())
    }

    pub fn clear_seat(&mut self, chair: Chair) -> Option<Player> {
        let removed = self.seats.get(&chair).cloned();
        self.seats.insert(chair, Player::default());
        removed
    }

    // --------------------------------------
    // Iteration
    // --------------------------------------

    // Returns iterator over all seats and players
    pub fn all_chairs(&self) -> impl Iterator<Item = (Chair, &Player)> {
        self.seats.iter().map(|(c, p)| (*c, p))
    }
    /// Iterate over all chairs and their players.
    pub fn iter(&self) -> impl Iterator<Item = (Chair, &Player)> {
        self.seats.iter().map(|(c, p)| (*c, p))
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

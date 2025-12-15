use crate::engine::state::{chair::Chair, player::Player, role::Role};
use rand::prelude::*;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct Table {
    pub chairs_to_players: BTreeMap<Chair, Player>,
    pub available_roles: Vec<Role>,
    pub available_positions: Vec<u8>,
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
        }
    }

    pub fn pick_position(&mut self) -> Option<u8> {
        let mut rng = rand::rng();
        if let Some(&position) = self.available_positions.choose(&mut rng) {
            self.available_positions.retain(|&x| x != position);
            Some(position)
        } else {
            println!("No available seats");
            None
        }
    }

    pub fn pick_role(&mut self) -> Option<Role> {
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
}

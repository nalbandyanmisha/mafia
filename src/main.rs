use clap::{Parser, Subcommand};
use rand::prelude::*;
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, Default, Clone)]
struct Player {
    pub seat: u8,
    pub name: String,
    pub role: Role,
    pub warnings: u8,
    pub status: Status,
    pub is_nominated: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum Status {
    #[default]
    Alive,
    Dead,
    Left,
}

impl Player {
    fn new(seat: u8, name: String, role: Role) -> Self {
        Player {
            seat,
            name,
            role,
            warnings: 0,
            status: Status::Alive,
            is_nominated: false,
        }
    }

    fn add_warning(&mut self) {
        self.warnings += 1;
    }

    fn reset_warnings(&mut self) {
        self.warnings = 0;
    }

    fn remove_warning(&mut self) {
        if self.warnings > 0 {
            self.warnings -= 1;
        }
    }

    fn withdraw(&mut self) {
        self.is_nominated = false;
    }

    fn nominate(&mut self) {
        self.is_nominated = true;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum Role {
    #[default]
    Citizen,
    Mafia,
    Don,
    Sheriff,
}

#[derive(Debug, Default, Clone)]
struct Chair {
    pub position: u8,
    pub role: Option<Role>,
    pub player: Option<Player>,
}

impl Chair {
    fn new(position: u8) -> Self {
        Chair {
            position,
            role: None,
            player: None,
        }
    }
}

struct Table {
    chairs: Vec<Chair>,
    available_roles: Vec<Role>,
    available_positions: Vec<u8>,
    nominated_players: Vec<u8>,
    votes: HashMap<u8, Vec<u8>>, // (nominee, votes)
    phase: Phase,
    round: u8,
}

impl Table {
    fn new() -> Self {
        const COUNT: u8 = 10;
        let chairs: Vec<Chair> = (1..=COUNT).map(|pos| Chair::new(pos)).collect();
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
        let available_positions: Vec<u8> = (1..=COUNT).collect();
        Table {
            chairs,
            available_roles,
            available_positions,
            nominated_players: Vec::new(),
            votes: HashMap::new(),
            phase: Phase::default(),
            round: 0,
        }
    }

    fn pick_chair(&mut self) -> Chair {
        let mut rng = rand::rng();
        if let Some(&position) = self.available_positions.choose(&mut rng) {
            self.available_positions.retain(|&x| x != position);
            Chair::new(position)
        } else {
            println!("No available seats");
            Chair::default()
        }
    }

    fn pick_role(&mut self) -> Role {
        let mut rng = rand::rng();
        if let Some(pos) = self.available_roles.as_slice().choose(&mut rng).cloned() {
            let index = self.available_roles.iter().position(|x| *x == pos).unwrap();
            self.available_roles.remove(index);
            pos
        } else {
            println!("No available roles");
            Role::Citizen
        }
    }
}

#[derive(Parser, Debug, Default)]
enum Phase {
    #[default]
    Night,
    Morning,
    Day,
    Voting,
}

#[derive(Parser, Debug)]
#[command(name = "mafia", version, about = "Mafia game host CLI")]
struct MafiaCli {
    #[command(subcommand)]
    command: Option<Action>,
}

#[derive(Subcommand, Debug)]
enum Action {
    Join { name: String },
    Left { position: u8 },
    Warn { position: u8 },
    Pardon { position: u8 },
    Nominate { position: u8 },
    Withdraw { position: u8 },
    Shoot { position: u8 },
    Voting,
    Day,
    Night,
    Show,
    Print,
    Quite,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut table = Table::new();

    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let args = input.trim().split_whitespace();
        let mut clap_args = vec!["mafia"];
        clap_args.extend(args);

        let mafia = MafiaCli::parse_from(clap_args);

        match &mafia.command {
            Some(Action::Join { name }) => {
                if table.available_positions.is_empty() {
                    println!("The table is full. Cannot join.");
                    continue;
                }
                let mut chair = table.pick_chair();
                chair.role = Some(table.pick_role());
                chair.player = Some(Player::new(
                    chair.position,
                    name.clone(),
                    chair.role.as_ref().unwrap().clone(),
                ));
                table.chairs[usize::from(chair.position - 1)] = chair.clone();
            }
            Some(Action::Show) => {
                for chair in &table.chairs {
                    if let Some(player) = &chair.player {
                        println!(
                            "Seat {}: {} ({:?})",
                            chair.position, player.name, player.role
                        );
                    } else {
                        println!("Seat {}: Empty", chair.position);
                    }
                }
            }
            Some(Action::Left { position }) => {
                let pos = *position;
                if let Some(chair) = table.chairs.get_mut(usize::from(pos - 1)) {
                    if chair.player.is_some() {
                        chair.player = None;
                        if let Some(role) = &chair.role {
                            table.available_roles.push(role.clone());
                        }
                        chair.role = None;
                        table.available_positions.push(pos);
                        println!("Player at seat {pos} has left the game.");
                    } else {
                        println!("Seat {pos} is already empty.");
                    }
                } else {
                    println!("Invalid seat number: {pos}");
                }
            }
            Some(Action::Warn { position }) => {
                let pos = *position;
                if let Some(chair) = table.chairs.get_mut(usize::from(pos - 1)) {
                    if let Some(player) = &mut chair.player {
                        player.add_warning();
                        println!(
                            "Player {} at seat {} has been warned. Total warnings: {}",
                            player.name, pos, player.warnings
                        );
                    } else {
                        println!("No player at seat {pos} to warn.");
                    }
                } else {
                    println!("Invalid seat number: {pos}");
                }
            }
            Some(Action::Pardon { position }) => {
                let pos = *position;
                if let Some(chair) = table.chairs.get_mut(usize::from(pos - 1)) {
                    if let Some(player) = &mut chair.player {
                        player.remove_warning();
                        println!(
                            "Player {} at seat {} has been pardoned. Total warnings: {}",
                            player.name, pos, player.warnings
                        );
                    } else {
                        println!("No player at seat {pos} to pardon.");
                    }
                } else {
                    println!("Invalid seat number: {pos}");
                }
            }
            Some(Action::Shoot { position }) => {
                let pos = *position;
                if let Some(chair) = table.chairs.get_mut(usize::from(pos - 1)) {
                    if let Some(player) = &mut chair.player {
                        player.status = Status::Dead;
                        println!("Player {} at seat {} has been shot.", player.name, pos);
                    } else {
                        println!("No player at seat {pos} to shoot.");
                    }
                } else {
                    println!("Invalid seat number: {pos}");
                }
            }
            Some(Action::Nominate { position }) => {
                let pos = *position;
                if let Some(chair) = table.chairs.get_mut(usize::from(pos - 1)) {
                    if let Some(player) = &mut chair.player {
                        player.nominate();
                        println!("Player {} at seat {} has been nominated.", player.name, pos);
                    } else {
                        println!("No player at seat {pos} to nominate.");
                    }
                } else {
                    println!("Invalid seat number: {pos}");
                }
                table.nominated_players.push(pos);
            }
            Some(Action::Withdraw { position }) => {
                let pos = *position;
                if let Some(chair) = table.chairs.get_mut(usize::from(pos - 1)) {
                    if let Some(player) = &mut chair.player {
                        player.withdraw();
                        println!(
                            "Player {} at seat {} has withdrawn nomination.",
                            player.name, pos
                        );
                    } else {
                        println!("No player at seat {pos} to withdraw.");
                    }
                } else {
                    println!("Invalid seat number: {pos}");
                }
                if let Some(index) = table.nominated_players.iter().position(|x| *x == *position) {
                    table.nominated_players.remove(index);
                }
            }
            Some(Action::Voting) => {
                table.phase = Phase::Voting;
                println!("It is now Voting phase.");
                println!("Nominated players in order: {:?}", table.nominated_players);
                for nominee in &table.nominated_players {
                    println!("Collecting votes for seat {nominee}.");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    let votes = input
                        .split(',')
                        .filter_map(|s| s.trim().parse::<u8>().ok())
                        .collect();

                    table.votes.insert(*nominee, votes);
                }
            }
            Some(Action::Day) => {
                table.phase = Phase::Day;
                println!("It is now Day {}.", table.round);
            }
            Some(Action::Night) => {
                table.phase = Phase::Night;
                table.round += 1;
                table.nominated_players = Vec::new();
                for player in &mut table.chairs {
                    if let Some(p) = &mut player.player {
                        p.withdraw();
                    }
                }
                println!("It is now Night {}.", table.round);
            }
            Some(Action::Print) => {
                for chair in &table.chairs {
                    if let Some(player) = &chair.player {
                        println!(
                            "Seat {}: {} ({:?}) has {} warnings",
                            chair.position, player.name, player.role, player.warnings
                        );
                    }
                }
            }
            Some(Action::Quite) => {
                println!("Quitting the Mafia CLI. Goodbye!");
                break;
            }
            None => {
                println!("No command provided.")
            }
        }
    }
    Ok(())
}

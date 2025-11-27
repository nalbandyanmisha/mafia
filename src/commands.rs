use clap::{Parser, Subcommand};
use std::io::{self, Write};

#[derive(Debug, Subcommand)]
pub enum Command {
    Registration(Args),
    Start(StartArgs),
    New(Players),
    End,
    Quite,
}

#[derive(Debug, Parser)]
struct Args {
    pub name: String,
}

impl Command {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Start(cmd) => cmd.run().await,
            Command::End => {
                println!("Ending the current game");
                // Here you would add the logic to end the game
                Ok(())
            }
            Command::New(players) => {
                println!("Creating a new game with players: {:?}", players.names);
                // Here you would add the logic to create a new game
                Ok(())
            }
            Command::Quite => {
                println!("Ending the current game");
                // Here you would add the logic to end the game
                Ok(())
            }
            Command::Registration(args) => {
                println!("Registering player: {}", args.name);
                Ok(())
            }
        }
    }
}

#[derive(Debug, Parser)]
pub struct StartArgs {
    #[clap(long, default_value = "10", help = "Number of players to start game")]
    pub players: Option<usize>,
}

impl StartArgs {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting a new game with {} players", self.players.unwrap());

        // Here you would add the logic to start the game
        Ok(())
    }
}

#[derive(Debug, Parser)]
struct Players {
    pub names: Vec<String>,
}

impl Players {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Creating a new game with players: {:?}", self.names.len());
        println!("Enter player names separated by spaces:");
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let names = input.trim().split_whitespace();
        let mut clap_args = vec!["mafia"];
        clap_args.extend(names);

        // Here you would add the logic to create a new game
        Ok(())
    }
}

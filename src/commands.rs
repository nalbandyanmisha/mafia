use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Command {
    Start(StartArgs),
}

impl Command {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Start(cmd) => cmd.run().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct StartArgs {
    #[clap(
        long,
        default_value = "7",
        help = "Number of signers to derive from mnemonic"
    )]
    pub players: Option<usize>,
}

impl StartArgs {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting a new game with {} players", self.players.unwrap());
        // Here you would add the logic to start the game
        Ok(())
    }
}

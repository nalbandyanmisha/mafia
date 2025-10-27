mod commands;
use clap::Parser;
use commands::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mafia = Mafia::parse();

    mafia.run().await?;
    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "mafia", version, about = "Host of Mafia game", long_about = None)]
struct Mafia {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Mafia {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(command) = &self.command {
            command.run().await?;
        } else {
            println!("Welcome to Mafia, type the command to move forward");
        }
        Ok(())
    }
}

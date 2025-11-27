mod commands;
use clap::Parser;
use commands::Command;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mafia = Mafia::parse();

    println!("Starting Mafia CLI");

    mafia.run().await?;
    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "mafia", version, about = "The host of Mafia game", long_about = None)]
struct Mafia {
    #[command(subcommand)]
    pub command: Option<Command>,
}

impl Mafia {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            print!("> ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let args = input.trim().split_whitespace();
            let mut clap_args = vec!["mafia"];
            clap_args.extend(args);
            let mafia = Mafia::parse_from(&clap_args);
            match &mafia.command {
                Some(Command::Quite) => {
                    println!("Quitting the Mafia CLI. Goodbye!");
                    break;
                }
                Some(Command::Start(cmd)) => {
                    cmd.run().await?;
                }
                cmd => {
                    println!("Running command: {cmd:#?}");
                    println!("----------------------------------------");
                    println!("Type the command and press Enter to execute it.");
                }
            }

            // if let Some(command) = &self.command {
            //     if let Command::Quite = command {
            //         println!("Quitting the Mafia CLI. Goodbye!");
            //         break;
            //     }
            //     println!("Running command: {command:#?}");
            //     println!("----------------------------------------");
            //     println!("Type the command and press Enter to execute it.");
            // }
        }
        Ok(())
    }
}

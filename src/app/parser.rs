use crate::app::commands::Command;
use clap::Parser;

pub fn parse_input(input: &str) -> Result<Command, clap::Error> {
    // Clap expects argv-style input
    let argv = std::iter::once("mafia").chain(input.split_whitespace());
    Command::try_parse_from(argv)
}

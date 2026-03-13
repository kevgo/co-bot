mod cli;
mod commands;
mod config;
mod errors;

use crate::cli::Command;
use clap::Parser;
use errors::Result;
use std::process::ExitCode;

fn main() -> ExitCode {
    match inner() {
        Ok(exit_code) => exit_code,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            ExitCode::FAILURE
        }
    }
}

fn inner() -> Result<ExitCode> {
    let args = cli::Args::parse();
    let config = config::Config::load("co-bot.toml").unwrap();
    match args.command {
        Command::Run { ticket: issue } => commands::run(issue, config),
    }
}

mod cli;
mod commands;
mod config;
mod connectors;
mod domain;
mod errors;
mod git;
mod logger;
mod subshell;

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
    match args.command {
        Command::Run { ticket } => commands::run(ticket, args.verbose),
    }
}

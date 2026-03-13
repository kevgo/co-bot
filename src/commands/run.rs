use crate::config::Config;
use crate::errors::Result;
use std::process::ExitCode;

pub fn run(issue: String, config: Config) -> Result<ExitCode> {
    println!(
        "Tracker: {} ({})",
        config.tracker.tracker_type, config.tracker.url
    );
    println!("Ticket: {}", issue);
    Ok(ExitCode::SUCCESS)
}

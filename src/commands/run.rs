use crate::errors::Result;
use crate::trackers::IssueIdOrUrl;
use crate::{config, trackers};
use std::process::ExitCode;

pub fn run(issue: IssueIdOrUrl) -> Result<ExitCode> {
    let issue_id = issue.id()?;
    let config = config::load()?;
    let tracker = trackers::get_tracker(&config)?;
    println!(
        "Tracker: {} ({})",
        config.tracker.tracker_type, config.tracker.url
    );
    let issue_text = tracker.issue_text(&issue_id)?;
    println!("Ticket #{}: {}", issue_id, issue_text);
    Ok(ExitCode::SUCCESS)
}

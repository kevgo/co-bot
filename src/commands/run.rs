use crate::domain::IssueIdOrUrl;
use crate::errors::Result;
use crate::{config, connectors, subshell};
use std::process::ExitCode;

pub fn run(issue: IssueIdOrUrl, verbose: bool) -> Result<ExitCode> {
    // load issue
    let issue_id = issue.id()?;
    let config = config::load()?;
    let tracker_token = subshell::run(&config.tracker.token_source)?;
    if verbose {
        println!("Tracker token: {}", tracker_token);
    }
    let tracker = connectors::load_tracker(&config.tracker, tracker_token)?;
    if verbose {
        println!(
            "Tracker: {} ({})",
            config.tracker.tracker_type, config.tracker.url
        );
    }
    let issue_text = tracker.issue_text(&issue_id)?;
    if verbose {
        println!("Ticket #{}: {}", issue_id, issue_text);
    }

    // create Git workspace and branch

    // run the code generator

    // commit the changes

    // create the PR

    // wait for user feedback

    // cleanup the Git workspace

    Ok(ExitCode::SUCCESS)
}

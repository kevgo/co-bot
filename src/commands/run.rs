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
    let issue_text = tracker.load_issue(&issue_id)?;
    if verbose {
        println!("Ticket #{}: {}", issue_id, issue_text);
    }

    // create Git workspace and branch

    // create .co-bot/session.md with plan
    // - create the plan
    // - create the PR
    // - wait for user feedback
    // if comments:
    // - update .co-bot/session.md
    // - tag the human in the PR

    // create the code:
    // - run the code generator
    // - commit the changes
    // - run the review agent
    // - implement the feedback
    // - tag the human in the PR
    
    // wait for user feedback

    // if comments:
    // - update the persistent memory and the session memory with the feedback
    // - implement the requested changes
    // - run the review agent
    // - implement the feedback
    // - tag the human in the PR

    // if "finalize" command:
    // - update the persistent memory with learnings from this session
    // - remove the session file

    // if PR was merged:
    // - remove the Git worktree
    // - remove the local branch
    // - sync all local branches

    Ok(ExitCode::SUCCESS)
}

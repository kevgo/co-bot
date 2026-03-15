use crate::domain::IssueIdOrUrl;
use crate::errors::Result;
use crate::git::Workspace;
use crate::logger::Logger;
use crate::{config, connectors, git, log, subshell};
use std::process::ExitCode;

pub fn run(issue: IssueIdOrUrl, verbose: bool) -> Result<ExitCode> {
    let logger = Logger { verbose };

    // load issue
    let issue_id = issue.id()?;
    let config = config::load()?;
    let tracker_token = subshell::run(&config.tracker.token_source)?;
    log!(logger, "Tracker token: {}", tracker_token);
    let tracker = connectors::load_tracker(&config.tracker, tracker_token)?;
    log!(
        logger,
        "Tracker: {} ({})",
        config.tracker.tracker_type,
        config.tracker.url
    );
    let issue_text = tracker.load_issue(&issue_id)?;
    log!(logger, "Ticket #{}: {}", issue_id, issue_text);

    // create Git workspace and branch
    let workspace_path = git::workspace_path(&config.git.workspace_path, &issue_id, &issue_text)?;
    let workspace = Workspace::from(workspace_path);
    git::create_workspace(&config.git.create_workspace, &workspace)?;
    git::create_branch(&config.git.create_branch, &workspace)?;

    // run the code generator

    // commit the changes

    // create the PR

    // wait for user feedback

    // cleanup the Git workspace

    Ok(ExitCode::SUCCESS)
}

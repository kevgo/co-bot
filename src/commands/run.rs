use crate::domain::TicketIdOrUrl;
use crate::errors::Result;
use crate::git::Workspace;
use crate::logger::Logger;
use crate::{config, connectors, git, log, subshell};
use std::process::ExitCode;

pub fn run(issue: TicketIdOrUrl, verbose: bool) -> Result<ExitCode> {
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
    let ticket = tracker.load_ticket(&issue_id)?;
    log!(logger, "Ticket #{}: {}", issue_id, ticket);

    // create Git workspace and branch
    let branch_name = config
        .git
        .branch_name
        .replace("{{ticket.id}}", &issue_id.to_string())
        .replace("{{ticket.title}}", &ticket.title.to_string());
    let workspace_path = git::workspace_path(&config.git.workspace_path, &issue_id, &ticket.title)?;
    log!(logger, "Workspace path: {}", workspace_path);
    let workspace = Workspace::from(workspace_path);
    git::create_branch(&config.git.create_branch, &branch_name)?;
    log!(logger, "Created branch: {}", workspace);
    git::create_workspace(&config.git.create_workspace, &workspace)?;
    log!(logger, "branch: {}", workspace);

    // run the code generator
    let _query = ticket.to_query();

    // commit the changes

    // create the PR

    // wait for user feedback

    // cleanup the Git workspace

    Ok(ExitCode::SUCCESS)
}

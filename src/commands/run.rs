use crate::domain::TicketIdOrUrl;
use crate::errors::Result;
use crate::git::Workspace;
use crate::logger::Logger;
use crate::{config, git, log};
use std::process::ExitCode;

pub fn run(issue: TicketIdOrUrl, verbose: bool) -> Result<ExitCode> {
    let logger = Logger { verbose };

    // load issue
    let issue_id = issue.id()?;
    let config = config::load()?;
    let tracker_token = config.load_tracker_token()?;
    log!(logger, "Tracker token: {}", tracker_token);
    let tracker = config.load_tracker(tracker_token)?;
    log!(
        logger,
        "Tracker: {} ({})",
        config.data.tracker.tracker_type,
        config.data.tracker.url
    );
    let ticket = tracker.load_ticket(&issue_id)?;
    log!(logger, "Ticket #{}: {}", issue_id, ticket);

    // create Git workspace and branch
    let branch_name = config.branch_name(&ticket);

    let workspace_path =
        git::workspace_path(&config.data.git.workspace_path, &issue_id, &ticket.title)?;
    log!(logger, "Workspace path: {}", workspace_path);
    let workspace = Workspace::from(workspace_path);
    git::create_branch(&config.data.git.create_branch, &branch_name)?;
    log!(logger, "Created branch: {}", workspace);
    git::create_workspace(&config.data.git.create_workspace, &workspace)?;
    log!(logger, "branch: {}", workspace);

    // run the code generator
    let _query = ticket.to_query();

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

use crate::domain::TicketIdOrUrl;
use crate::errors::Result;
use crate::logger::Logger;
use crate::{config, git, log};
use std::process::ExitCode;

pub fn run(ticket: TicketIdOrUrl, verbose: bool) -> Result<ExitCode> {
    let logger = Logger { verbose };

    // load configuration
    let config = config::load()?;

    // instantiate tracker
    let tracker_token = config.load_tracker_token()?;
    log!(logger, "Tracker token: {}", tracker_token);
    let tracker = config.load_tracker(tracker_token)?;
    log!(
        logger,
        "Tracker: {} ({})",
        config.file.tracker.tracker_type,
        config.file.tracker.url
    );

    // load ticket
    let ticket_id = ticket.id()?;
    let ticket = tracker.load_ticket(&ticket_id)?;
    log!(logger, "Ticket #{}: {}", ticket.id, ticket);

    // create Git branch and workspace
    let branch_name = config.branch_name(&ticket);
    git::create_branch(&config.file.git.create_branch, &branch_name)?;
    log!(logger, "Created branch: {}", branch_name);
    let workspace = config.workspace_path(&ticket)?;
    git::create_workspace(&config.file.git.create_workspace, &workspace)?;
    log!(logger, "Created workspace: {}", workspace);

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

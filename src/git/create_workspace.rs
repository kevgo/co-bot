use crate::errors::{Result, UserError};
use crate::git::workspace::Workspace;
use crate::subshell;

pub fn create_workspace(template: &str, workspace: &Workspace) -> Result<()> {
    let command = template.replace("{{workspace}}", workspace.as_ref().as_str());
    subshell::run(&command).map_err(|err| UserError::CannotCreateWorkspace {
        workspace: workspace.to_string(),
        command: command.to_string(),
        err: err.to_string(),
    })?;
    Ok(())
}

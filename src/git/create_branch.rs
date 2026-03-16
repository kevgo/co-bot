use crate::errors::{Result, UserError};
use crate::git::Workspace;
use crate::subshell;

pub fn create_branch(template: &str, workspace: &Workspace) -> Result<()> {
    let command = template.replace("{{workspace}}", workspace.as_ref().as_str());
    subshell::run(&command).map_err(|err| UserError::CannotCreateBranch {
        command: command.to_string(),
        err: err.to_string(),
    })?;
    println!("1111111111111111111111111111111111111111111111");
    Ok(())
}

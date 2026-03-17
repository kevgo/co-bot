use crate::errors::{Result, UserError};
use crate::subshell;

pub fn create_branch(template: &str, name: &str) -> Result<()> {
    let command = template.replace("{{git.branch.name}}", name);
    subshell::run(&command).map_err(|err| UserError::CannotCreateBranch {
        command: command.to_string(),
        err: err.to_string(),
    })?;
    Ok(())
}

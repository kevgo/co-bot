use crate::errors::{Result, UserError};
use std::process::Command;

pub fn run(command: &str) -> Result<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|err| UserError::CannotRunSubshellCommand {
            command: command.to_string(),
            err: err.to_string(),
        })?;
    let text =
        String::from_utf8(output.stdout).map_err(|err| UserError::CommandReturnedInvalidUTF8 {
            command: command.to_string(),
            err: err.to_string(),
        })?;
    if !output.status.success() {
        return Err(UserError::CommandReturnedNonZeroExitCode {
            command: command.to_string(),
            exit_code: output.status.code().unwrap_or(1),
        });
    }
    Ok(text.trim().to_string())
}

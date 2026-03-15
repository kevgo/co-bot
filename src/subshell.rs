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
    String::from_utf8(output.stdout).map_err(|err| UserError::CommandReturnedInvalidUTF8 {
        command: command.to_string(),
        err: err.to_string(),
    })
}

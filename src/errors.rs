use std::fmt::Display;

pub type Result<T> = std::result::Result<T, UserError>;

#[derive(Debug, Eq, PartialEq)]
pub enum UserError {
    CannotCreateBranch {
        command: String,
        err: String,
    },
    CannotCreateWorkspace {
        workspace: String,
        command: String,
        err: String,
    },
    CannotLoadGitHubIssue {
        issue_id: String,
        err: String,
    },
    CannotRunSubshellCommand {
        command: String,
        err: String,
    },
    CommandReturnedInvalidUTF8 {
        command: String,
        err: String,
    },
    ConfigFileNotFound(String),
    ConfigFileInvalidContent {
        path: String,
        err: String,
    },
    InvalidGitHubIssuesHost {
        host: String,
        err: String,
    },
    InvalidTicketID {
        id: String,
    },
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::CannotCreateBranch { command, err } => {
                write!(
                    f,
                    "command '{command}' to create a Git branch failed: {err}"
                )
            }
            UserError::CannotCreateWorkspace {
                workspace,
                command,
                err,
            } => {
                write!(
                    f,
                    "cannot create workspace {workspace} via '{command}': {err}"
                )
            }
            UserError::CannotLoadGitHubIssue { issue_id, err } => {
                write!(f, "cannot load GitHub Issue #{issue_id}: {err}")
            }
            UserError::CannotRunSubshellCommand { command, err } => {
                write!(f, "cannot run subshell command '{command}': {err}")
            }
            UserError::CommandReturnedInvalidUTF8 { command, err } => {
                write!(f, "command '{command}' returned invalid UTF-8: {err}")
            }
            UserError::ConfigFileNotFound(path) => {
                write!(f, "Config file '{path}' not found")
            }
            UserError::ConfigFileInvalidContent { path, err } => {
                write!(f, "Config file '{path}' has invalid content:\n{err}",)
            }
            UserError::InvalidGitHubIssuesHost { host, err } => {
                write!(f, "Invalid hostname for GitHub Issues: {host}\n{err}")
            }
            UserError::InvalidTicketID { id } => write!(
                f,
                "Invalid ticket ID: {id}\nPlease provide either the numerical id or the URL of the ticket",
            ),
        }
    }
}

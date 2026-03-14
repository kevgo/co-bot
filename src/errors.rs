use std::fmt::Display;

pub type Result<T> = std::result::Result<T, UserError>;

#[derive(Debug, Eq, PartialEq)]
pub enum UserError {
    CannotLoadGitHubIssue { issue_id: String, err: String },
    ConfigFileNotFound(String),
    ConfigFileInvalidContent { path: String, err: String },
    InvalidGitHubIssuesHost { host: String, err: String },
    InvalidTicketID(String),
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::CannotLoadGitHubIssue { issue_id, err } => {
                write!(f, "cannot load GitHub Issue #{issue_id}: {err}")
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
            UserError::InvalidTicketID(id) => write!(
                f,
                "Invalid ticket ID: {id}\nPlease provide the numerical ticket id or the URL of the ticket",
            ),
        }
    }
}

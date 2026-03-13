use std::fmt::Display;

pub type Result<T> = std::result::Result<T, UserError>;

#[derive(Debug)]
pub enum UserError {
    ConfigFileNotFound(String),
    ConfigFileInvalidContent(String),
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::ConfigFileNotFound(path) => write!(f, "Config file not found: {}", path),
            UserError::ConfigFileInvalidContent(err) => {
                write!(f, "Config file has invalid content: {}", err)
            }
        }
    }
}

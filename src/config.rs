//! This module loads the low-level configuration data from the config file.

use crate::errors::{Result, UserError};
use serde::Deserialize;
use std::fmt::Display;

const FILE_NAME: &str = "co-bot.toml";

pub fn load() -> Result<Config> {
    let Ok(content) = std::fs::read_to_string(FILE_NAME) else {
        return Err(UserError::ConfigFileNotFound(FILE_NAME.to_string()));
    };
    toml::from_str(&content).map_err(|err| UserError::ConfigFileInvalidContent {
        path: FILE_NAME.to_string(),
        err: err.to_string(),
    })
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tracker: Tracker,
}

#[derive(Debug, Deserialize)]
pub struct Tracker {
    #[serde(rename = "type")]
    pub tracker_type: TrackerType,
    pub url: String,
    pub token_source: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum TrackerType {
    GitHub,
}

impl Display for TrackerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrackerType::GitHub => write!(f, "GitHub Issues"),
        }
    }
}

impl TryFrom<&str> for Config {
    type Error = UserError;

    fn try_from(text: &str) -> Result<Self> {
        toml::from_str(text).map_err(|err| UserError::ConfigFileInvalidContent {
            path: FILE_NAME.to_string(),
            err: err.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOML_CONTENT: &str = r#"
[tracker]
type = "GitHub"
url = "https://github.com/kevgo/co-bot/issues"
token_source = "gh"
"#;

    #[test]
    fn parse_full_config() {
        let config = Config::try_from(TOML_CONTENT).unwrap();

        // tracker
        assert_eq!(config.tracker.tracker_type, TrackerType::GitHub);
        assert_eq!(config.tracker.url, "https://github.com/kevgo/co-bot/issues");
    }

    #[test]
    fn load_from_file() {
        let config = super::load().unwrap();
        assert_eq!(config.tracker.tracker_type, TrackerType::GitHub);
        assert_eq!(config.tracker.url, "https://github.com/kevgo/co-bot/issues");
    }
}

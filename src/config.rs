use crate::errors::{Result, UserError};
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tracker: Tracker,
}

#[derive(Debug, Deserialize)]
pub struct Tracker {
    #[serde(rename = "type")]
    pub tracker_type: TrackerType,
    pub url: String,
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

impl Config {
    pub fn load(path: &str) -> Result<Config> {
        let Ok(content) = std::fs::read_to_string(path) else {
            return Err(UserError::ConfigFileNotFound(path.to_string()));
        };
        toml::from_str(&content).map_err(|err| UserError::ConfigFileInvalidContent(err.to_string()))
    }
}

impl TryFrom<&str> for Config {
    type Error = UserError;

    fn try_from(text: &str) -> Result<Self> {
        toml::from_str(text).map_err(|err| UserError::ConfigFileInvalidContent(err.to_string()))
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
        let config = Config::load("co-bot.toml").unwrap();
        assert_eq!(config.tracker.tracker_type, TrackerType::GitHub);
        assert_eq!(config.tracker.url, "https://github.com/kevgo/co-bot/issues");
    }
}

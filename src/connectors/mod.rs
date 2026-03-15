mod github;

use crate::config::{self, TrackerType};
use crate::domain::IssueId;
use crate::errors::Result;
/// Trackers store tickets to implement
pub trait Tracker {
    /// provides an AI-friendly text serialization of the issue with the given id
    fn load_issue(&self, issue: &IssueId) -> Result<String>;
}

pub fn load_tracker(config: &config::Tracker, tracker_token: String) -> Result<Box<dyn Tracker>> {
    match config.tracker_type {
        TrackerType::GitHub => Ok(Box::new(github::new(&config.url, tracker_token)?)),
    }
}

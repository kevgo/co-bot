mod github;

use crate::config::{Config, TrackerType};
use crate::domain::IssueId;
use crate::errors::Result;
/// Trackers store tickets to implement
pub trait Tracker {
    /// provides an AI-friendly text serialization of the issue with the given id
    fn issue_text(&self, issue: &IssueId) -> Result<String>;
}

pub fn get_tracker(config: &Config) -> Result<Box<dyn Tracker>> {
    match config.tracker.tracker_type {
        TrackerType::GitHub => Ok(Box::new(github::new(&config.tracker.url)?)),
    }
}

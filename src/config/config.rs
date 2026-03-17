//! This module provides high-level configuration data.

use crate::config::Data;
use crate::config::data::TrackerType;
use crate::connectors::Tracker;
use crate::connectors::github;
use crate::domain::Ticket;
use crate::errors::Result;
use crate::subshell;

pub struct Config {
    pub data: Data,
}

impl Config {
    pub fn branch_name(&self, ticket: &Ticket) -> String {
        self.data
            .git
            .branch_name
            .replace("{{ticket.id}}", &ticket.id.to_string())
            .replace("{{ticket.title}}", &ticket.title.as_ref())
    }

    pub fn load_tracker_token(&self) -> Result<String> {
        subshell::run(&self.data.tracker.token_source)
    }

    pub fn load_tracker(&self, tracker_token: String) -> Result<Box<dyn Tracker>> {
        match self.data.tracker.tracker_type {
            TrackerType::GitHub => Ok(Box::new(github::new(
                &self.data.tracker.url,
                tracker_token,
            )?)),
        }
    }
}

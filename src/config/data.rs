//! This module provides high-level configuration data.

use crate::config::File;
use crate::config::file::TrackerType;
use crate::connectors::Tracker;
use crate::connectors::github;
use crate::domain::Ticket;
use crate::errors::Result;
use crate::git::Workspace;
use crate::subshell;

/// high-level configuration data as it is used by the application
pub struct Data {
    pub file: File,
}

impl Data {
    pub fn branch_name(&self, ticket: &Ticket) -> String {
        self.file
            .git
            .branch_name
            .replace("{{ticket.id}}", &ticket.id.to_string())
            .replace("{{ticket.title}}", &escape(&ticket.title))
    }

    pub fn load_tracker_token(&self) -> Result<String> {
        subshell::run(&self.file.tracker.token_source)
    }

    pub fn load_tracker(&self, tracker_token: String) -> Result<Box<dyn Tracker>> {
        match self.file.tracker.tracker_type {
            TrackerType::GitHub => Ok(Box::new(github::new(
                &self.file.tracker.url,
                tracker_token,
            )?)),
        }
    }

    pub fn workspace_path(&self, ticket: &Ticket) -> Result<Workspace> {
        let path = self
            .file
            .git
            .workspace_path
            .replace("{{ticket.id}}", &ticket.id.to_string())
            .replace("{{ticket.title}}", &escape(ticket.title.as_ref()));
        Ok(Workspace::from(path))
    }
}

fn escape<AS: AsRef<str>>(text: AS) -> String {
    text.as_ref()
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect()
}

#[cfg(test)]
mod tests {

    mod escape {
        #[test]
        fn text() {
            let give = "Hello World!";
            let have = super::super::escape(give);
            assert_eq!(have, "hello-world");
        }
    }

    mod workspace_path {

        use crate::config::file::Git;
        use crate::config::{Data, File};
        use crate::domain::{Ticket, TicketId};
        use crate::git::Workspace;

        #[test]
        fn workspace_path() {
            let config = Data {
                file: File {
                    git: Git {
                        workspace_path: "{{ticket.id}}-{{ticket.title}}".to_string(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            };
            let ticket = Ticket {
                id: TicketId::from(123),
                title: "Test Ticket".into(),
                ..Default::default()
            };
            let have = config.workspace_path(&ticket).unwrap();
            let want = Workspace::from("123-test-ticket");
            assert_eq!(have, want);
        }
    }
}

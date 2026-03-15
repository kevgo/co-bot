use crate::connectors::{TicketId, Tracker};
use crate::domain::{Ticket, TicketTitle};
use crate::errors::{Result, UserError};
use big_s::S;
use roctogen::endpoints::issues;
use roctokit::adapters::client;
use roctokit::adapters::ureq::Client;
use roctokit::auth::Auth;

pub fn new(url: &str, token: String) -> Result<GitHubIssues> {
    let (owner, repo) = parse_github_url(url)?;
    let auth = Auth::Token(token);
    let client = client(&auth).expect("Cannot create new GitHub client");
    Ok(GitHubIssues {
        client,
        owner,
        repo,
    })
}

/// provides access to the issue tracker on github.com
pub struct GitHubIssues {
    pub client: Client,
    pub owner: String,
    pub repo: String,
}

impl Tracker for GitHubIssues {
    fn load_ticket(&self, issue_id: &TicketId) -> Result<Ticket> {
        let issue = issues::new(&self.client)
            .get(&self.owner, &self.repo, issue_id.into())
            .map_err(|err| UserError::CannotLoadGitHubIssue {
                issue_id: issue_id.to_string(),
                err: err.to_string(),
            })?;
        let Some(loaded_id) = issue.id else {
            return Err(UserError::CannotLoadGitHubIssue {
                issue_id: issue_id.to_string(),
                err: format!("issue ({issue_id}) has no id"),
            });
        };
        let Some(loaded_title) = issue.title else {
            return Err(UserError::CannotLoadGitHubIssue {
                issue_id: issue_id.to_string(),
                err: format!("issue ({issue_id}) has no title"),
            });
        };
        Ok(Ticket {
            id: TicketId::from(loaded_id),
            title: TicketTitle::from(loaded_title),
            description: issue.body.unwrap_or_default(),
        })
    }
}

fn parse_github_url(url: &str) -> Result<(String, String)> {
    let mut parts = url.split('/');
    let Some(protocol) = parts.next() else {
        return Err(UserError::InvalidGitHubIssuesHost {
            host: url.into(),
            err: S("cannot determine HTTP protocol"),
        });
    };
    if protocol != "https:" {
        return Err(UserError::InvalidGitHubIssuesHost {
            host: url.into(),
            err: format!("unexpected protocol: {protocol}"),
        });
    }
    let Some(empty) = parts.next() else {
        return Err(UserError::InvalidGitHubIssuesHost {
            host: url.into(),
            err: S("unexpected end after protocol"),
        });
    };
    if !empty.is_empty() {
        return Err(UserError::InvalidGitHubIssuesHost {
            host: url.into(),
            err: format!("unexpected text between the two slashes after the protocol: {empty}"),
        });
    }
    let Some(hostname) = parts.next() else {
        return Err(UserError::InvalidGitHubIssuesHost {
            host: url.into(),
            err: S("cannot determine hostname"),
        });
    };
    if hostname != "github.com" {
        return Err(UserError::InvalidGitHubIssuesHost {
            host: url.into(),
            err: format!("unexpected hostname: {hostname}, expected github.com"),
        });
    }
    let Some(owner) = parts.next() else {
        return Err(UserError::InvalidGitHubIssuesHost {
            host: url.into(),
            err: S("cannot determine owner"),
        });
    };
    let Some(repo) = parts.next() else {
        return Err(UserError::InvalidGitHubIssuesHost {
            host: url.into(),
            err: S("cannot determine repo"),
        });
    };
    let Some(path) = parts.next() else {
        return Ok((owner.into(), repo.into()));
    };
    if path.is_empty() {
        return Ok((owner.into(), repo.into()));
    }
    if path != "issues" {
        return Err(UserError::InvalidGitHubIssuesHost {
            host: url.into(),
            err: format!("expected 'issues', found '{path}'"),
        });
    }
    Ok((owner.into(), repo.into()))
}

#[cfg(test)]
mod tests {

    mod parse_github_url {
        use big_s::S;

        use crate::errors::UserError;

        #[test]
        fn valid_without_slash() {
            let give = "https://github.com/kevgo/co-bot/issues";
            let have = super::super::parse_github_url(give);
            let want = Ok((S("kevgo"), S("co-bot")));
            assert_eq!(have, want);
        }

        #[test]
        fn valid_with_slash() {
            let give = "https://github.com/kevgo/co-bot/issues/";
            let have = super::super::parse_github_url(give);
            let want = Ok((S("kevgo"), S("co-bot")));
            assert_eq!(have, want);
        }

        #[test]
        fn repo_url_without_slash() {
            let give = "https://github.com/kevgo/co-bot";
            let have = super::super::parse_github_url(give);
            let want = Ok((S("kevgo"), S("co-bot")));
            assert_eq!(have, want);
        }

        #[test]
        fn repo_url_with_slash() {
            let give = "https://github.com/kevgo/co-bot/";
            let have = super::super::parse_github_url(give);
            let want = Ok((S("kevgo"), S("co-bot")));
            assert_eq!(have, want);
        }

        #[test]
        fn other_host() {
            let give = "https://gitlab.com/kevgo/co-bot/issues/";
            let have = super::super::parse_github_url(give);
            let want = Err(UserError::InvalidGitHubIssuesHost {
                host: S("https://gitlab.com/kevgo/co-bot/issues/"),
                err: S("unexpected hostname: gitlab.com, expected github.com"),
            });
            assert_eq!(have, want);
        }
    }
}

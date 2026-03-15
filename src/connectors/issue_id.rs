use crate::errors::Result;
use std::fmt::Display;

/// an issue id or URL
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueIdOrUrl(String);

impl IssueIdOrUrl {
    pub fn id(self) -> Result<IssueId> {
        if is_number(&self.0) {
            return Ok(IssueId::from(self.0));
        }
        if let Some(last_segment) = self.0.rsplit('/').next()
            && is_number(last_segment)
        {
            return Ok(IssueId::from(last_segment));
        }
        Err(crate::errors::UserError::InvalidTicketID(self.0))
    }
}

fn is_number(text: &str) -> bool {
    !text.is_empty() && text.chars().all(|c| c.is_ascii_digit())
}

impl From<&str> for IssueIdOrUrl {
    fn from(value: &str) -> Self {
        IssueIdOrUrl(value.to_string())
    }
}

// an issue id
#[derive(Debug, Eq, PartialEq)]
pub struct IssueId(String);

impl Display for IssueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for IssueId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for IssueId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&IssueId> for i32 {
    fn from(value: &IssueId) -> Self {
        value.0.parse().expect("issue id must be a number")
    }
}

#[cfg(test)]
mod tests {

    mod id {
        use crate::errors::UserError;

        use super::super::{IssueId, IssueIdOrUrl};

        #[test]
        fn id() {
            let give = IssueIdOrUrl::from("123");
            let have = give.id();
            let want = Ok(IssueId::from("123"));
            assert_eq!(have, want);
        }

        #[test]
        fn url() {
            let give = IssueIdOrUrl::from("https://github.com/kevgo/co-bot/issues/123");
            let have = give.id();
            let want = Ok(IssueId::from("123"));
            assert_eq!(have, want);
        }

        #[test]
        fn unknown() {
            let give = IssueIdOrUrl::from("zonk");
            let have = give.id();
            let want = Err(UserError::InvalidTicketID("zonk".into()));
            assert_eq!(have, want);
        }
    }
}

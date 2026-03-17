use crate::domain::IssueId;
use crate::errors::Result;

/// IssueIdOrUrl is an issue identifier given by the user.
/// The user can provide issues either by number or their full URL.
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

#[cfg(test)]
mod tests {

    mod id {
        use super::super::IssueIdOrUrl;
        use crate::domain::IssueId;
        use crate::errors::UserError;

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

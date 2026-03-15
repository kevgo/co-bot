use crate::domain::TicketId;
use crate::errors::{Result, UserError};

/// TicketIdOrUrl is an ticket reference given by the user.
/// The user can provide tickets either by their number or full URL.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TicketIdOrUrl(String);

impl TicketIdOrUrl {
    pub fn id(self) -> Result<TicketId> {
        if let Ok(id) = self.0.parse::<i64>() {
            return Ok(TicketId::from(id));
        }
        if let Some(last_segment) = self.0.rsplit('/').next() {
            match last_segment.parse::<i64>() {
                Ok(parsed) => return Ok(TicketId::from(parsed)),
                Err(_) => {
                    return Err(UserError::InvalidTicketID {
                        id: last_segment.to_string(),
                    });
                }
            }
        }
        Err(UserError::InvalidTicketID { id: self.0 })
    }
}

impl From<&str> for TicketIdOrUrl {
    fn from(value: &str) -> Self {
        TicketIdOrUrl(value.to_string())
    }
}

#[cfg(test)]
mod tests {

    mod id {
        use super::super::TicketIdOrUrl;
        use crate::domain::TicketId;
        use crate::errors::UserError;

        #[test]
        fn id() {
            let give = TicketIdOrUrl::from("123");
            let have = give.id();
            let want = Ok(TicketId::from(123));
            assert_eq!(have, want);
        }

        #[test]
        fn url() {
            let give = TicketIdOrUrl::from("https://github.com/kevgo/co-bot/issues/123");
            let have = give.id();
            let want = Ok(TicketId::from(123));
            assert_eq!(have, want);
        }

        #[test]
        fn unknown() {
            let give = TicketIdOrUrl::from("zonk");
            let have = give.id();
            let want = Err(UserError::InvalidTicketID { id: "zonk".into() });
            assert_eq!(have, want);
        }
    }
}

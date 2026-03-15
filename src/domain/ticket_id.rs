use std::fmt::Display;

/// numerical identifier of a ticket
#[derive(Debug, Eq, PartialEq)]
pub struct TicketId(i64);

impl Display for TicketId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl From<i64> for TicketId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

// impl From<&str> for TicketId {
//     fn from(value: &str) -> Self {
//         Self(value.parse())
//     }
// }

// impl From<String> for TicketId {
//     fn from(value: String) -> Self {
//         Self(value)
//     }
// }

impl From<&TicketId> for i64 {
    fn from(value: &TicketId) -> Self {
        value.0
    }
}

impl From<&TicketId> for i32 {
    fn from(value: &TicketId) -> Self {
        value.0 as i32
    }
}

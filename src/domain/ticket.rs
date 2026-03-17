use std::fmt::Display;

use crate::domain::{TicketId, TicketTitle};

#[derive(Debug, Default)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: String,
}

impl Ticket {
    pub fn to_query(&self) -> String {
        let mut result = format!("{}: {}", self.id, self.title.as_ref());
        if !self.description.is_empty() {
            result += "\n\n";
            result += &self.description;
        }
        result
    }
}

impl Display for Ticket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{} ({})", self.id, self.title.as_ref())
    }
}

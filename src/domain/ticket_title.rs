use std::fmt::Display;

/// the title of a ticket
#[derive(Debug, Default)]
pub struct TicketTitle(String);

impl AsRef<str> for TicketTitle {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for TicketTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for TicketTitle {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for TicketTitle {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

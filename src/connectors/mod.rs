pub mod github;

use crate::domain::{Ticket, TicketId};
use crate::errors::Result;
/// Trackers store tickets to implement
pub trait Tracker {
    /// provides an AI-friendly text serialization of the issue with the given id
    fn load_ticket(&self, issue: &TicketId) -> Result<Ticket>;
}

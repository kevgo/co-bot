use crate::domain::TicketIdOrUrl;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// the command to run
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Clone, Debug, Eq, PartialEq)]
pub enum Command {
    Run {
        /// ID or URL of the ticket to implement
        ticket: TicketIdOrUrl,
    },
}

//! Codex hook reports and the Harness event subscription contract.

mod generated {
    pub mod signal;
}
mod following;
mod hook;

pub use generated::signal::*;

/// The contract's authored Ethos, whose digest both ends of a connection
/// greet with.
pub const ETHOS: &str = include_str!("../ethos/signal.ethos");

impl signal::Contracted for Input {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}

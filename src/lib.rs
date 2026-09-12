//! Ordinary Signal contract for repository-ledger.
//!
//! The contract carries peer-callable repository event submissions — a receive
//! hook notification, a push observation — and the bounded read queries over
//! the ledger: events, recently active repositories, changed files, commit
//! messages, and the registration catalog. Meta configuration lives in
//! `meta-signal-repository-ledger`.
//!
//! `ethos/signal.ethos` is the schema authority; `build.rs` checks the
//! checked-in Rust projection in `src/generated/signal.rs` against it. The
//! portable rkyv frame, its kinds, and the wire framing come from `signal`.

pub mod generated;
pub use generated::signal::*;

/// The authored Ethos source of this contract.
pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
/// The Rust projection generated from [`ETHOS`].
pub const ETHOS_RUST: &str = include_str!("generated/signal.rs");

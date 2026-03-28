#![expect(
    clippy::multiple_inherent_impl,
    reason = "We prefer to group methods by functionality rather than by type for encoding"
)]

pub use circuit_io::{PrivacyPreservingCircuitInput, PrivacyPreservingCircuitOutput};
// LP-0015 ergonomic macros are re-exported at crate root automatically via
// `#[macro_export]` in program.rs (`assert_internal!` and `call_program!`).
pub use commitment::{
    Commitment, CommitmentSetDigest, DUMMY_COMMITMENT, DUMMY_COMMITMENT_HASH, MembershipProof,
    compute_digest_for_path,
};
pub use encryption::{EncryptionScheme, SharedSecretKey};
pub use nullifier::{Nullifier, NullifierPublicKey, NullifierSecretKey};

pub mod account;
mod circuit_io;
mod commitment;
mod encoding;
pub mod encryption;
mod nullifier;
pub mod program;

#[cfg(feature = "host")]
pub mod error;

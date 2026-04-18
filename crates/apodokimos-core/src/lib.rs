//! Core protocol types for Apodokimos Epistemic Contribution Graph
//!
//! This crate provides the foundational types for the Apodokimos protocol:
//! - Claims and attestations
//! - Claim weight function
//! - Error types
//!
//! # Features
//! - `std` (default): Enables standard library support
//! - `no_std`: Core functionality without std (requires `alloc`)

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod claim;
mod error;
mod field;
mod weight;

pub use claim::{Attestation, AttestationVerdict, Claim, ClaimId, ClaimType};
pub use error::ApodokimosError;
pub use field::{ClinicalMedicine, FieldSchema};
pub use weight::{AffectedClaim, ClaimWeight, GraphSnapshot, OFactorLinkage, OFactorSource, WeightFactors, WeightFunction, RETRACTION_THRESHOLD};

use blake3::Hasher;

/// Compute blake3 hash of canonical claim content
pub fn compute_claim_hash(content: &[u8]) -> ClaimId {
    let mut hasher = Hasher::new();
    hasher.update(content);
    ClaimId::from_bytes(hasher.finalize().into())
}

/// Serialize a claim to canonical JSON format (deterministic)
pub fn canonical_serialize<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, ApodokimosError> {
    // Use compact formatting for determinism
    serde_json::to_vec(value)
        .map_err(|e: serde_json::Error| ApodokimosError::Serialization(e.to_string()))
}

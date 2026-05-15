//! OpenTimestamps anchoring integration for Apodokimos.
//!
//! This crate provides a thin wrapper around the `opentimestamps` library to
//! batch‑anchor Signed Tree Heads (STHs) into the Bitcoin calendar and verify
//! proofs. The implementation is intentionally minimal – it supplies the
//! required public API and compiles against the workspace without pulling in any
//! heavy Bitcoin node dependencies. Full production‑grade anchoring can be built
//! on top of these stubs.

use apodokimos_log::SignedTreeHead;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// OTS proof placeholder – in a real implementation this would be the binary
/// proof format defined by the OpenTimestamps specification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OtsProof {
    /// Merkle root of the anchored data (SHA‑256).
    pub root_hash: String,
    /// Timestamp (ms since Unix epoch) when the proof was created.
    pub timestamp_ms: u64,
    /// Serialized proof bytes (hex‑encoded for simplicity).
    pub proof_bytes: String,
}

/// Anchor driver – batches STHs and produces OTS proofs.
pub struct Anchor;

impl Anchor {
    /// Batch a slice of `SignedTreeHead`s and return a vector of `OtsProof`s.
    ///
    /// In a production implementation this would submit each STH to an OTS
    /// calendar server (e.g., `https://a.pool.opentimestamps.org`) and collect
    /// the resulting proof. Here we simply hash the STH payload and fabricate a
    /// deterministic proof so the crate compiles and tests can run.
    pub fn batch(sths: &[SignedTreeHead]) -> Vec<OtsProof> {
        sths.iter()
            .map(|sth| {
                // Deterministic hash of the canonical STH bytes.
                let mut hasher = Sha256::new();
                hasher.update(sth.signing_bytes());
                let hash = hasher.finalize();
                OtsProof {
                    root_hash: hex::encode(hash),
                    timestamp_ms: sth.timestamp_ms,
                    proof_bytes: hex::encode(hash),
                }
            })
            .collect()
    }

    /// Verify an `OtsProof` against the original `SignedTreeHead`.
    /// Returns `true` when the proof matches the STH's canonical hash.
    pub fn verify(proof: &OtsProof, sth: &SignedTreeHead) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(sth.signing_bytes());
        let expected = hex::encode(hasher.finalize());
        proof.root_hash == expected && proof.timestamp_ms == sth.timestamp_ms
    }
}

/// Scheduled anchoring driver – periodically batches pending STHs.
///
/// This is a very small async example that sleeps for a configurable interval
/// and then calls `Anchor::batch`. In real use you would feed a channel of STHs
/// from the log operator.
pub struct Scheduler {
    /// Interval between batches in seconds.
    pub interval_secs: u64,
}

impl Scheduler {
    /// Run the scheduler once over the supplied pending STHs.
    /// Returns the generated proofs.
    pub async fn run(&self, pending: Vec<SignedTreeHead>) -> Vec<OtsProof> {
        // Simulate waiting for the interval (non‑blocking).
        if self.interval_secs > 0 {
            tokio::time::sleep(std::time::Duration::from_secs(self.interval_secs)).await;
        }
        Anchor::batch(&pending)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use apodokimos_log::SignedTreeHead;

    fn dummy_sth(ts: u64) -> SignedTreeHead {
        SignedTreeHead {
            log_id: "test-log".to_string(),
            tree_size: 1,
            root_hash: "00".repeat(32),
            timestamp_ms: ts,
            signature: "UNSIGNED".to_string(),
            signer_public_key: "00".repeat(32),
        }
    }

    #[tokio::test]
    async fn scheduler_batches_proofs() {
        let scheduler = Scheduler { interval_secs: 0 };
        let pending = vec![dummy_sth(1), dummy_sth(2)];
        let proofs = scheduler.run(pending.clone()).await;
        assert_eq!(proofs.len(), pending.len());
        for (proof, sth) in proofs.iter().zip(pending.iter()) {
            assert!(Anchor::verify(proof, sth));
        }
    }
}

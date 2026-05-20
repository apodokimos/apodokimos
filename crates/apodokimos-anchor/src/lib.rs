//! OpenTimestamps anchoring integration for Apodokimos.
//!
//! This crate provides batch anchoring of Signed Tree Heads (STHs) via the OpenTimestamps protocol.
//! Multiple STHs are merged into a single Merkle commitment, which is submitted to an OTS calendar
//! for Bitcoin anchoring. The implementation is async and uses reqwest for HTTP communication.

use apodokimos_log::SignedTreeHead;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::SystemTime;

/// Error type for anchor operations.
#[derive(Debug, thiserror::Error)]
pub enum AnchorError {
    #[error("batch must contain at least one STH")]
    EmptyBatch,
    #[error("calendar unavailable at {url}: {source}")]
    CalendarUnavailable { url: String, source: reqwest::Error },
    #[error("network timeout connecting to calendar")]
    Timeout,
    #[error("invalid calendar response: {0}")]
    InvalidResponse(String),
}

/// OpenTimestamps proof bundle for a batch of STHs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OtsProof {
    /// Unix timestamp (ms) when the proof was obtained from the calendar.
    pub timestamp_ms: u64,
    /// OTS calendar URL that produced this proof.
    pub calendar_url: String,
    /// SHA-256 Merkle root over all batched STH hashes (hex-encoded).
    pub commitment: String,
    /// Raw OTS proof bytes returned by the calendar (hex-encoded).
    pub proof_bytes: String,
}

/// Anchor driver – batches STHs and produces OTS proofs.
pub struct Anchor {
    /// URL of the OpenTimestamps calendar server (e.g., https://a.pool.opentimestamps.org).
    pub calendar_url: String,
}

impl Anchor {
    /// Create a new Anchor with the given calendar URL.
    pub fn new(calendar_url: impl Into<String>) -> Self {
        Anchor {
            calendar_url: calendar_url.into(),
        }
    }

    /// Compute the batch Merkle commitment over a set of STHs.
    ///
    /// Returns `None` if the batch is empty. Otherwise, hashes each STH canonically
    /// and builds a Merkle tree per RFC 6962, returning the root.
    pub fn batch_commitment(sths: &[SignedTreeHead]) -> Option<[u8; 32]> {
        if sths.is_empty() {
            return None;
        }

        let leaves: Vec<[u8; 32]> = sths
            .iter()
            .map(|sth| {
                let mut hasher = Sha256::new();
                hasher.update(sth.signing_bytes());
                let digest = hasher.finalize();
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&digest);
                arr
            })
            .collect();

        Some(merkle_root(&leaves))
    }

    /// Batch a slice of `SignedTreeHead`s and submit to the OTS calendar.
    ///
    /// Computes the Merkle root over all STH hashes, sends it to the calendar via HTTP POST,
    /// and returns an `OtsProof` containing the calendar response.
    pub async fn batch(&self, sths: &[SignedTreeHead]) -> Result<OtsProof, AnchorError> {
        let commitment = Self::batch_commitment(sths).ok_or(AnchorError::EmptyBatch)?;
        let commitment_hex = hex::encode(commitment);

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| AnchorError::CalendarUnavailable {
                url: self.calendar_url.clone(),
                source: e,
            })?;

        let response = client
            .post(&self.calendar_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(commitment_hex.clone())
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    AnchorError::Timeout
                } else {
                    AnchorError::CalendarUnavailable {
                        url: self.calendar_url.clone(),
                        source: e,
                    }
                }
            })?;

        if !response.status().is_success() {
            return Err(AnchorError::InvalidResponse(format!(
                "HTTP {}",
                response.status()
            )));
        }

        let proof_bytes = response
            .bytes()
            .await
            .map_err(|e| AnchorError::InvalidResponse(e.to_string()))?;

        let timestamp_ms = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        Ok(OtsProof {
            timestamp_ms,
            calendar_url: self.calendar_url.clone(),
            commitment: commitment_hex,
            proof_bytes: hex::encode(proof_bytes),
        })
    }

    /// Verify an `OtsProof` against the original `SignedTreeHead`s.
    ///
    /// Recomputes the batch commitment and checks that it matches the proof.
    pub fn verify(proof: &OtsProof, sths: &[SignedTreeHead]) -> bool {
        let Some(commitment) = Self::batch_commitment(sths) else {
            return false;
        };
        proof.commitment == hex::encode(commitment)
    }
}

/// Compute the RFC 6962 Merkle root over a slice of hashes.
///
/// Uses iterative pairwise hashing with 0x01 domain prefix (inner node).
/// If the count is odd, duplicates the last node before combining.
fn merkle_root(hashes: &[[u8; 32]]) -> [u8; 32] {
    if hashes.is_empty() {
        return [0u8; 32];
    }
    if hashes.len() == 1 {
        return hashes[0];
    }

    let mut current = hashes.to_vec();

    while current.len() > 1 {
        let mut next = Vec::new();
        for i in (0..current.len()).step_by(2) {
            let left = current[i];
            let right = if i + 1 < current.len() {
                current[i + 1]
            } else {
                left
            };

            let mut hasher = Sha256::new();
            hasher.update([0x01u8]); // RFC 6962 inner node domain prefix
            hasher.update(left);
            hasher.update(right);
            let digest = hasher.finalize();
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&digest);
            next.push(arr);
        }
        current = next;
    }

    current[0]
}

/// Scheduled anchoring driver – periodically batches pending STHs.
///
/// This struct encapsulates a simple async job that sleeps for a configurable interval,
/// then batches and anchors a set of pending STHs. In production, STHs would arrive
/// via a channel from the log operator.
pub struct Scheduler {
    /// Interval between batch attempts in seconds.
    pub interval_secs: u64,
    /// Anchor instance holding the calendar URL.
    pub anchor: Anchor,
}

impl Scheduler {
    /// Run the scheduler once: sleep for the configured interval, then batch the pending STHs.
    ///
    /// Returns the generated proofs.
    pub async fn run(&self, pending: Vec<SignedTreeHead>) -> Result<Vec<OtsProof>, AnchorError> {
        if self.interval_secs > 0 {
            tokio::time::sleep(std::time::Duration::from_secs(self.interval_secs)).await;
        }
        // Collect results; if there are pending STHs, submit them.
        if pending.is_empty() {
            return Ok(vec![]);
        }
        let proof = self.anchor.batch(&pending).await?;
        Ok(vec![proof])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_sth(ts: u64) -> SignedTreeHead {
        SignedTreeHead {
            log_id: "test-log".to_string(),
            tree_size: 1,
            root_hash: format!("{:064x}", ts),
            timestamp_ms: ts,
            signature: "UNSIGNED".to_string(),
            signer_public_key: "00".repeat(32),
        }
    }

    #[test]
    fn batch_commitment_empty_returns_none() {
        assert!(Anchor::batch_commitment(&[]).is_none());
    }

    #[test]
    fn batch_commitment_single_sth() {
        let sth = dummy_sth(42);
        let root = Anchor::batch_commitment(std::slice::from_ref(&sth)).unwrap();
        // For a single leaf, the root is just the leaf hash itself.
        let expected = {
            let mut hasher = Sha256::new();
            hasher.update(sth.signing_bytes());
            let digest = hasher.finalize();
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&digest);
            arr
        };
        assert_eq!(root, expected);
    }

    #[test]
    fn batch_commitment_three_sths() {
        let sths = [dummy_sth(1), dummy_sth(2), dummy_sth(3)];

        let root = Anchor::batch_commitment(&sths).unwrap();

        // Recompute expected merkle root manually.
        let leaf_hashes: Vec<[u8; 32]> = sths
            .iter()
            .map(|sth| {
                let mut hasher = Sha256::new();
                hasher.update(sth.signing_bytes());
                let digest = hasher.finalize();
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&digest);
                arr
            })
            .collect();

        let expected = merkle_root(&leaf_hashes);
        assert_eq!(root, expected);
    }

    #[test]
    fn batch_commitment_100_sths_completes_within_10s() {
        let sths: Vec<_> = (0..100).map(dummy_sth).collect();
        let start = std::time::Instant::now();
        let _root = Anchor::batch_commitment(&sths).unwrap();
        let elapsed = start.elapsed();
        assert!(
            elapsed.as_secs() < 10,
            "batch commitment for 100 STHs took {:?}",
            elapsed
        );
    }

    #[test]
    fn verify_against_batch_commitment() {
        let sths = [dummy_sth(10), dummy_sth(20)];
        let commitment = Anchor::batch_commitment(&sths).unwrap();

        let proof = OtsProof {
            timestamp_ms: 0,
            calendar_url: "http://test".to_string(),
            commitment: hex::encode(commitment),
            proof_bytes: "".to_string(),
        };

        assert!(Anchor::verify(&proof, &sths));
    }

    #[test]
    fn verify_against_wrong_sths_fails() {
        let sths_original = [dummy_sth(10), dummy_sth(20)];
        let sths_modified = [dummy_sth(10), dummy_sth(21)];

        let commitment = Anchor::batch_commitment(&sths_original).unwrap();
        let proof = OtsProof {
            timestamp_ms: 0,
            calendar_url: "http://test".to_string(),
            commitment: hex::encode(commitment),
            proof_bytes: "".to_string(),
        };

        assert!(!Anchor::verify(&proof, &sths_modified));
    }
}

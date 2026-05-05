use crate::error::LogError;
use crate::merkle::{inclusion_path, leaf_hash, merkle_root, verify_inclusion_path};
use crate::types::{InclusionProof, SignedEntry, SignedTreeHead, WitnessSignature};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

/// Transparency log client backed by an in-memory RFC6962-style Merkle tree.
#[derive(Debug, Clone)]
pub struct LogClient {
    log_id: String,
    log_signer_public_key: String,
    current_timestamp_ms: u64,
    entries: Vec<SignedEntry>,
}

impl LogClient {
    /// Creates a new local log client.
    pub fn new(log_id: impl Into<String>, log_signer_public_key: impl Into<String>) -> Self {
        Self {
            log_id: log_id.into(),
            log_signer_public_key: log_signer_public_key.into(),
            current_timestamp_ms: 0,
            entries: Vec::new(),
        }
    }

    /// RI-03: Submits a signed entry and returns an inclusion proof for current STH.
    pub fn submit(&mut self, entry: SignedEntry) -> Result<InclusionProof, LogError> {
        self.entries.push(entry.clone());
        self.current_timestamp_ms = self.current_timestamp_ms.saturating_add(1);

        let leaf_hashes = self.leaf_hashes();
        let leaf_index = self
            .entries
            .len()
            .checked_sub(1)
            .ok_or_else(|| LogError::ProofUnavailable("empty log after insert".to_string()))?
            as u64;

        let proof = inclusion_path(&leaf_hashes, leaf_index as usize).ok_or_else(|| {
            LogError::ProofUnavailable("failed to build inclusion path".to_string())
        })?;

        let sth = self.current_sth();
        Ok(InclusionProof {
            leaf_index,
            tree_size: self.entries.len() as u64,
            audit_path: proof.into_iter().map(hex::encode).collect(),
            sth,
        })
    }

    /// RI-04: Verifies entry inclusion against proof and STH.
    pub fn verify_inclusion(
        &self,
        entry: &SignedEntry,
        proof: &InclusionProof,
        sth: &SignedTreeHead,
    ) -> bool {
        if proof.tree_size != sth.tree_size || proof.sth != *sth {
            return false;
        }

        let root = match decode_hash32(&sth.root_hash) {
            Ok(r) => r,
            Err(_) => return false,
        };

        let path: Option<Vec<[u8; 32]>> = proof
            .audit_path
            .iter()
            .map(|h| decode_hash32(h).ok())
            .collect();
        let path = match path {
            Some(p) => p,
            None => return false,
        };

        let leaf = leaf_hash(&entry.canonical_bytes());
        verify_inclusion_path(leaf, proof.leaf_index, proof.tree_size, &path, root)
    }

    /// RI-05: Verifies consistency by recomputing historical roots from local log prefixes.
    pub fn verify_consistency(&self, old_sth: &SignedTreeHead, new_sth: &SignedTreeHead) -> bool {
        if old_sth.log_id != new_sth.log_id || old_sth.log_id != self.log_id {
            return false;
        }
        if old_sth.tree_size > new_sth.tree_size {
            return false;
        }
        if new_sth.tree_size as usize > self.entries.len() {
            return false;
        }

        let old_root_expected = match decode_hash32(&old_sth.root_hash) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let new_root_expected = match decode_hash32(&new_sth.root_hash) {
            Ok(v) => v,
            Err(_) => return false,
        };

        let leaves = self.leaf_hashes();
        let old_root = merkle_root(&leaves[..old_sth.tree_size as usize]);
        let new_root = merkle_root(&leaves[..new_sth.tree_size as usize]);

        old_root == old_root_expected && new_root == new_root_expected
    }

    /// RI-06: Verifies witness co-signatures over a Signed Tree Head.
    pub fn verify_witness_signatures(
        &self,
        sth: &SignedTreeHead,
        witnesses: &[WitnessSignature],
    ) -> bool {
        let message = sth.signing_bytes();

        for witness in witnesses {
            let pk_bytes = match decode_key32(&witness.public_key) {
                Ok(v) => v,
                Err(_) => return false,
            };
            let verifying_key = match VerifyingKey::from_bytes(&pk_bytes) {
                Ok(vk) => vk,
                Err(_) => return false,
            };
            let sig_bytes = match hex::decode(&witness.signature) {
                Ok(v) => v,
                Err(_) => return false,
            };
            let signature = match Signature::from_slice(&sig_bytes) {
                Ok(s) => s,
                Err(_) => return false,
            };
            if verifying_key.verify(&message, &signature).is_err() {
                return false;
            }
        }

        true
    }

    /// Returns the current STH view of this local log.
    pub fn current_sth(&self) -> SignedTreeHead {
        let root = merkle_root(&self.leaf_hashes());

        SignedTreeHead {
            log_id: self.log_id.clone(),
            tree_size: self.entries.len() as u64,
            root_hash: hex::encode(root),
            timestamp_ms: self.current_timestamp_ms,
            signature: "UNSIGNED_LOCAL_STH".to_string(),
            signer_public_key: self.log_signer_public_key.clone(),
        }
    }

    fn leaf_hashes(&self) -> Vec<[u8; 32]> {
        self.entries
            .iter()
            .map(|entry| leaf_hash(&entry.canonical_bytes()))
            .collect()
    }
}

fn decode_hash32(hex_str: &str) -> Result<[u8; 32], LogError> {
    let raw = hex::decode(hex_str).map_err(|e| LogError::PublicKeyFormat(e.to_string()))?;
    if raw.len() != 32 {
        return Err(LogError::PublicKeyFormat(format!(
            "expected 32 bytes, got {}",
            raw.len()
        )));
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&raw);
    Ok(out)
}

fn decode_key32(hex_str: &str) -> Result<[u8; 32], LogError> {
    decode_hash32(hex_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    fn keypair() -> SigningKey {
        SigningKey::from_bytes(&[7u8; 32])
    }

    fn test_entry(i: u8) -> SignedEntry {
        SignedEntry {
            payload: vec![i, i + 1, i + 2],
            signer_did: format!("did:apodokimos:submitter:{}", i),
            signature: "deadbeef".to_string(),
        }
    }

    #[test]
    fn submit_and_verify_inclusion() {
        let mut client = LogClient::new("apodokimos-local", "00");
        let entry = test_entry(1);
        let proof = client.submit(entry.clone()).unwrap();
        let sth = client.current_sth();

        assert!(client.verify_inclusion(&entry, &proof, &sth));
    }

    #[test]
    fn verify_consistency_prefix_growth() {
        let mut client = LogClient::new("apodokimos-local", "00");
        client.submit(test_entry(1)).unwrap();
        let old_sth = client.current_sth();

        client.submit(test_entry(2)).unwrap();
        client.submit(test_entry(3)).unwrap();
        let new_sth = client.current_sth();

        assert!(client.verify_consistency(&old_sth, &new_sth));
    }

    #[test]
    fn verify_witness_signatures_round_trip() {
        let witness_key = keypair();
        let witness_pk = hex::encode(witness_key.verifying_key().to_bytes());

        let client = LogClient::new("apodokimos-local", "00");
        let sth = client.current_sth();
        let sig = witness_key.sign(&sth.signing_bytes());

        let witnesses = vec![WitnessSignature {
            witness_id: "did:apodokimos:witness:1".to_string(),
            public_key: witness_pk,
            signature: hex::encode(sig.to_bytes()),
        }];

        assert!(client.verify_witness_signatures(&sth, &witnesses));
    }
}

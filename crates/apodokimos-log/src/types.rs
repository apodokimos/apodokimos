use serde::{Deserialize, Serialize};

/// Signed log entry submitted by a protocol component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedEntry {
    /// Canonical entry payload bytes.
    pub payload: Vec<u8>,
    /// DID of the submitter.
    pub signer_did: String,
    /// Ed25519 signature over `payload` (hex-encoded 64 bytes).
    pub signature: String,
}

impl SignedEntry {
    /// Computes the leaf body that is hashed into the Merkle tree.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap_or_default()
    }
}

/// Signed Tree Head (STH) for a transparency log checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedTreeHead {
    /// Logical log identifier.
    pub log_id: String,
    /// Number of leaves in the tree.
    pub tree_size: u64,
    /// Merkle root hash (hex-encoded 32 bytes).
    pub root_hash: String,
    /// Milliseconds since Unix epoch.
    pub timestamp_ms: u64,
    /// Ed25519 signature over canonical STH bytes (hex-encoded 64 bytes).
    pub signature: String,
    /// Log signer public key (hex-encoded 32 bytes).
    pub signer_public_key: String,
}

impl SignedTreeHead {
    /// Canonical bytes used for signing and verification.
    pub fn signing_bytes(&self) -> Vec<u8> {
        let canonical = SthSignable {
            log_id: self.log_id.clone(),
            tree_size: self.tree_size,
            root_hash: self.root_hash.clone(),
            timestamp_ms: self.timestamp_ms,
        };
        serde_json::to_vec(&canonical).unwrap_or_default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SthSignable {
    log_id: String,
    tree_size: u64,
    root_hash: String,
    timestamp_ms: u64,
}

/// Inclusion proof for a leaf under a given STH.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InclusionProof {
    /// Zero-based index of the leaf.
    pub leaf_index: u64,
    /// Tree size used for this proof.
    pub tree_size: u64,
    /// Audit path sibling hashes (hex-encoded 32-byte digests).
    pub audit_path: Vec<String>,
    /// STH under which inclusion is proven.
    pub sth: SignedTreeHead,
}

/// Witness signature over an STH.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitnessSignature {
    /// DID or stable witness identifier.
    pub witness_id: String,
    /// Witness public key (hex-encoded 32 bytes).
    pub public_key: String,
    /// Signature over `SignedTreeHead::signing_bytes()` (hex-encoded 64 bytes).
    pub signature: String,
}

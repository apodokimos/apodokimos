//! Claim and attestation types (P-01, P-02)

use crate::{canonical_serialize, compute_claim_hash, ApodokimosError};
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

/// Blake3 hash of canonical claim content (32 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClaimId([u8; 32]);

impl ClaimId {
    /// Create from raw bytes
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Get raw bytes
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Get hex string representation
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl AsRef<[u8]> for ClaimId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl core::fmt::Display for ClaimId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

/// Claim type taxonomy (P-01)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClaimType {
    /// A novel falsifiable assertion about the world
    PrimaryClaim,
    /// A testable prediction not yet tested
    Hypothesis,
    /// A procedure claim (reproducibility target)
    Method,
    /// An empirical measurement or observation
    Result,
    /// An independent repetition of a prior Result
    Replication,
    /// A Result with no detectable effect
    NullResult,
}

impl ClaimType {
    /// Check if this is an empirical claim type (can receive survival-trackable attestations)
    pub const fn is_empirical(&self) -> bool {
        matches!(
            self,
            ClaimType::PrimaryClaim | ClaimType::Result | ClaimType::Replication | ClaimType::NullResult
        )
    }
}

/// Attestation verdict taxonomy (P-02)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AttestationVerdict {
    /// Attester's evidence corroborates the claim
    Supports,
    /// Attester's evidence is in tension with the claim
    Contradicts,
    /// Independent repetition confirms the claim
    Replicates,
    /// Independent repetition disconfirms the claim
    Refutes,
    /// Neutral reference; excluded from survival scoring
    Mentions,
}

impl AttestationVerdict {
    /// Check if this verdict contributes to survival rate S
    /// (excludes Mentions per P-06)
    pub const fn contributes_to_survival(&self) -> bool {
        !matches!(self, Self::Mentions)
    }

    /// Check if this is a supporting verdict
    pub const fn is_supporting(&self) -> bool {
        matches!(self, Self::Supports | Self::Replicates)
    }

    /// Check if this is a contradicting verdict
    pub const fn is_contradicting(&self) -> bool {
        matches!(self, Self::Contradicts | Self::Refutes)
    }
}

/// Arweave transaction ID (43 characters, base64url)
pub type TxId = String;

/// W3C Decentralized Identifier
pub type DID = String;

/// Field identifier (e.g., "clinical-medicine")
pub type FieldId = String;

/// Block number in the Substrate chain
pub type BlockNumber = u64;

/// Claim — minimal unit of falsifiable scientific assertion (P-01)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claim {
    /// Blake3 hash of canonical JSON claim content
    pub id: ClaimId,
    /// Type of claim
    pub claim_type: ClaimType,
    /// Field identifier
    pub field_id: FieldId,
    /// Claim content (stored on Arweave, referenced here)
    pub content: ClaimContent,
    /// Submitter DID
    pub submitter: DID,
    /// Dependencies (edges in E)
    pub depends_on: Vec<ClaimId>,
    /// Arweave transaction ID for content
    pub arweave_tx: TxId,
    /// Block number when registered
    pub registered: BlockNumber,
}

/// Content of a claim (CC0 schema)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimContent {
    /// Canonical JSON of the claim content
    pub canonical_json: String,
}

impl ClaimContent {
    /// Compute hash of this content's canonical JSON serialization
    pub fn compute_hash(&self) -> Result<ClaimId, ApodokimosError> {
        let serialized = canonical_serialize(self)?;
        Ok(compute_claim_hash(&serialized))
    }
}

impl Claim {
    /// Create a new claim with computed ID
    pub fn new(
        claim_type: ClaimType,
        field_id: impl Into<String>,
        content: impl Into<String>,
        submitter: impl Into<String>,
        depends_on: Vec<ClaimId>,
        arweave_tx: impl Into<String>,
        registered: BlockNumber,
    ) -> Self {
        let canonical_json = content.into();
        let content = ClaimContent { canonical_json };
        let id = content.compute_hash();

        Self {
            id,
            claim_type,
            field_id: field_id.into(),
            content,
            submitter: submitter.into(),
            depends_on,
            arweave_tx: arweave_tx.into(),
            registered,
        }
    }

    /// Verify the claim ID matches the content hash
    pub fn verify_hash(&self) -> Result<bool, ApodokimosError> {
        let computed = self.content.compute_hash()?;
        Ok(self.id == computed)
    }

    /// Check if this claim depends on another (directly)
    pub fn depends_directly_on(&self, claim_id: &ClaimId) -> bool {
        self.depends_on.contains(claim_id)
    }
}

/// Attestation — typed statement by credentialed reviewer (P-02)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attestation {
    /// Unique attestation identifier
    pub id: String,
    /// Claim being attested to
    pub claim_id: ClaimId,
    /// Attester DID
    pub attester: DID,
    /// Verdict type
    pub verdict: AttestationVerdict,
    /// Arweave pointer to supporting evidence (optional)
    pub evidence_tx: Option<TxId>,
    /// Snapshot of attester's field SBT score at attestation time
    pub attester_sbt: u64,
    /// Block number when attested
    pub block: BlockNumber,
}

impl Attestation {
    /// Create a new attestation
    pub fn new(
        id: impl Into<String>,
        claim_id: ClaimId,
        attester: impl Into<String>,
        verdict: AttestationVerdict,
        evidence_tx: Option<impl Into<String>>,
        attester_sbt: u64,
        block: BlockNumber,
    ) -> Self {
        Self {
            id: id.into(),
            claim_id,
            attester: attester.into(),
            verdict,
            evidence_tx: evidence_tx.map(|tx| tx.into()),
            attester_sbt,
            block,
        }
    }

    /// Check if this attestation contributes to survival rate
    pub const fn contributes_to_survival(&self) -> bool {
        self.verdict.contributes_to_survival()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claim_id_from_bytes() {
        let bytes = [0u8; 32];
        let id = ClaimId::from_bytes(bytes);
        assert_eq!(id.as_bytes(), &bytes);
    }

    #[test]
    fn claim_type_empirical() {
        assert!(ClaimType::PrimaryClaim.is_empirical());
        assert!(ClaimType::Result.is_empirical());
        assert!(ClaimType::Replication.is_empirical());
        assert!(ClaimType::NullResult.is_empirical());
        assert!(!ClaimType::Hypothesis.is_empirical());
        assert!(!ClaimType::Method.is_empirical());
    }

    #[test]
    fn attestation_verdict_classification() {
        assert!(AttestationVerdict::Supports.is_supporting());
        assert!(AttestationVerdict::Replicates.is_supporting());
        assert!(AttestationVerdict::Contradicts.is_contradicting());
        assert!(AttestationVerdict::Refutes.is_contradicting());
        
        assert!(AttestationVerdict::Supports.contributes_to_survival());
        assert!(AttestationVerdict::Replicates.contributes_to_survival());
        assert!(!AttestationVerdict::Mentions.contributes_to_survival());
    }

    #[test]
    fn claim_creation_and_hash_verification() {
        let claim = Claim::new(
            ClaimType::PrimaryClaim,
            "clinical-medicine",
            r#"{"population":"adults","intervention":"drug A","outcome":"survival"}"#,
            "did:substrate:apodokimos:abc123",
            vec![],
            "Xy_wY5mer8OHEX8QhdJW7w0L983Fz86k6gQaM8XEweM",
            1000,
        );

        // Content hash should be consistent
        let hash1 = claim.content.compute_hash().unwrap();
        let hash2 = claim.content.compute_hash().unwrap();
        assert_eq!(hash1.as_bytes(), hash2.as_bytes());

        // ID should match computed hash
        assert!(claim.verify_hash().unwrap());
        assert_eq!(claim.id.to_hex(), hash1.to_hex());
    }

    #[test]
    fn claim_dependency_check() {
        let dep_id = ClaimId::from_bytes([1u8; 32]);
        let claim = Claim::new(
            ClaimType::Result,
            "clinical-medicine",
            "{}",
            "did:test:123",
            vec![dep_id],
            "test_tx",
            100,
        );

        assert!(claim.depends_directly_on(&dep_id));
    }

    #[test]
    fn attestation_creation() {
        let claim_id = ClaimId::from_bytes([0u8; 32]);
        let att = Attestation::new(
            "att-001",
            claim_id,
            "did:substrate:apodokimos:reviewer1",
            AttestationVerdict::Replicates,
            Some("evidence_tx_123"),
            1000,
            2000,
        );

        assert_eq!(att.id, "att-001");
        assert!(att.contributes_to_survival());
    }

    #[test]
    fn serde_round_trip() {
        let claim = Claim::new(
            ClaimType::PrimaryClaim,
            "test-field",
            "{\"test\":true}",
            "did:test:123",
            vec![],
            "tx123",
            100,
        );

        let serialized = serde_json::to_string(&claim).unwrap();
        let deserialized: Claim = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(claim.id.as_bytes(), deserialized.id.as_bytes());
        assert_eq!(claim.claim_type, deserialized.claim_type);
    }

    #[test]
    fn claim_id_display() {
        let id = ClaimId::from_bytes([0xab; 32]);
        let display = format!("{}", id);
        assert_eq!(display, "abababababababababababababababababababababababababababababababab");
    }
}

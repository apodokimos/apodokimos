//! Multi-signature governance verification (C-29, wp-v0.2 §7.5)
//!
//! Implements k-of-n multi-signature governance for protocol parameter changes.
//! Governance actions are signed records that require at least k valid signatures
//! from the authorized set of n signers.

use crate::ApodokimosError;
use crate::claim::DID;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

/// Governance action types (C-29, wp-v0.2 §7.5)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GovernanceActionType {
    /// Update protocol parameter
    ParameterUpdate { param: String, value: String },
    /// Rotate governance set (add/remove signers)
    RotateGovernance {
        new_threshold: u32,
        new_signers: Vec<GovernanceSigner>,
    },
    /// Emergency halt/resume
    EmergencyHalt { resume: bool },
    /// Protocol upgrade
    Upgrade { new_version: String },
}

/// A governance signer with DID and Ed25519 public key
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceSigner {
    /// W3C DID of the signer
    pub did: DID,
    /// Ed25519 public key (32 bytes, hex-encoded)
    pub public_key: String,
}

impl GovernanceSigner {
    /// Parse the hex-encoded public key
    pub fn public_key_bytes(&self) -> Result<[u8; 32], ApodokimosError> {
        let bytes = hex::decode(&self.public_key)
            .map_err(|e| ApodokimosError::Governance(format!("invalid public key hex: {}", e)))?;
        if bytes.len() != 32 {
            return Err(ApodokimosError::Governance(format!(
                "public key must be 32 bytes, got {}",
                bytes.len()
            )));
        }
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        Ok(array)
    }
}

/// Governance configuration (genesis state)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceConfig {
    /// Minimum signatures required (k in k-of-n)
    /// Must be > n/2 for Byzantine safety
    pub threshold: u32,
    /// Authorized signers
    pub signers: Vec<GovernanceSigner>,
    /// Timelock period in hours
    pub timelock_hours: u32,
}

impl GovernanceConfig {
    /// Validate the governance configuration
    ///
    /// Checks:
    /// - threshold > 0
    /// - signers not empty
    /// - threshold > n/2 (Byzantine safety)
    pub fn validate(&self) -> Result<(), ApodokimosError> {
        let n = self.signers.len() as u32;

        if self.threshold == 0 {
            return Err(ApodokimosError::Governance(
                "threshold must be > 0".to_string(),
            ));
        }

        if n == 0 {
            return Err(ApodokimosError::Governance(
                "at least one signer required".to_string(),
            ));
        }

        // Byzantine safety: k > n/2
        if self.threshold <= n / 2 {
            return Err(ApodokimosError::Governance(format!(
                "threshold {} must be > n/2 (={}) for Byzantine safety",
                self.threshold,
                n / 2
            )));
        }

        // Validate all public keys are valid hex
        for signer in &self.signers {
            signer.public_key_bytes()?;
        }

        Ok(())
    }

    /// Get the number of signers (n)
    pub fn num_signers(&self) -> u32 {
        self.signers.len() as u32
    }
}

/// Individual signature on a governance action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceSignature {
    /// DID of the signer
    pub signer_did: DID,
    /// Ed25519 signature (64 bytes, hex-encoded)
    pub signature: String,
}

/// A governance action proposal (C-29, wp-v0.2 §7.5)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceAction {
    /// Unique proposal identifier
    pub proposal_id: String,
    /// Type of governance action
    pub action_type: GovernanceActionType,
    /// Block number when proposed
    pub proposed_at: u64,
    /// Signatures from authorized signers
    pub signatures: Vec<GovernanceSignature>,
}

/// Verification result for a governance action
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationResult {
    /// Action is valid and has sufficient signatures, and timelock elapsed
    Valid,
    /// Insufficient signatures (has k, needs n)
    InsufficientSignatures { has: u32, needs: u32 },
    /// Invalid signature detected
    InvalidSignature { signer: DID, reason: String },
    /// Signer not in governance set
    UnauthorizedSigner { signer: DID },
    /// Timelock period has not elapsed since proposal
    TimelockNotElapsed {
        proposed_at: u64,
        current_block: u64,
        required_delay: u32,
    },
}

/// Multi-signature governance verifier (C-29, wp-v0.2 §7.5)
pub struct GovernanceVerifier {
    config: GovernanceConfig,
    /// Cached public keys for verification
    public_keys: BTreeMap<DID, VerifyingKey>,
}

impl GovernanceVerifier {
    /// Create a new governance verifier from configuration
    pub fn new(config: GovernanceConfig) -> Result<Self, ApodokimosError> {
        config.validate()?;

        let mut public_keys = BTreeMap::new();
        for signer in &config.signers {
            let key_bytes = signer.public_key_bytes()?;
            let public_key = VerifyingKey::from_bytes(&key_bytes).map_err(|e| {
                ApodokimosError::Governance(format!(
                    "invalid Ed25519 public key for {}: {:?}",
                    signer.did, e
                ))
            })?;
            public_keys.insert(signer.did.clone(), public_key);
        }

        Ok(Self {
            config,
            public_keys,
        })
    }

    /// Verify a governance action has valid k-of-n signatures
    ///
    /// Returns `VerificationResult::Valid` if:
    /// - At least `threshold` valid signatures are present
    /// - All signatures are from authorized signers
    /// - All signatures cryptographically verify
    pub fn verify_action(&self, action: &GovernanceAction) -> VerificationResult {
        let mut valid_count = 0u32;
        let mut seen_signers = alloc::collections::BTreeSet::new();

        // Canonicalize the action for verification (exclude signatures)
        let canonical = self.canonicalize_action(action);

        for sig in &action.signatures {
            // Check for duplicate signers
            if seen_signers.contains(&sig.signer_did) {
                return VerificationResult::InvalidSignature {
                    signer: sig.signer_did.clone(),
                    reason: "duplicate signature".to_string(),
                };
            }
            seen_signers.insert(sig.signer_did.clone());

            // Check if signer is authorized
            let public_key = match self.public_keys.get(&sig.signer_did) {
                Some(pk) => pk,
                None => {
                    return VerificationResult::UnauthorizedSigner {
                        signer: sig.signer_did.clone(),
                    };
                }
            };

            // Verify the signature
            let sig_bytes = match hex::decode(&sig.signature) {
                Ok(b) => b,
                Err(_) => {
                    return VerificationResult::InvalidSignature {
                        signer: sig.signer_did.clone(),
                        reason: "invalid hex encoding".to_string(),
                    };
                }
            };

            if sig_bytes.len() != 64 {
                return VerificationResult::InvalidSignature {
                    signer: sig.signer_did.clone(),
                    reason: format!("signature must be 64 bytes, got {}", sig_bytes.len()),
                };
            }

            let signature = match Signature::from_slice(&sig_bytes) {
                Ok(s) => s,
                Err(e) => {
                    return VerificationResult::InvalidSignature {
                        signer: sig.signer_did.clone(),
                        reason: format!("invalid signature format: {:?}", e),
                    };
                }
            };

            match public_key.verify(&canonical, &signature) {
                Ok(()) => valid_count += 1,
                Err(e) => {
                    return VerificationResult::InvalidSignature {
                        signer: sig.signer_did.clone(),
                        reason: format!("verification failed: {:?}", e),
                    };
                }
            }
        }

        if valid_count >= self.config.threshold {
            VerificationResult::Valid
        } else {
            VerificationResult::InsufficientSignatures {
                has: valid_count,
                needs: self.config.threshold,
            }
        }
    }

    /// Get the governance configuration
    pub fn config(&self) -> &GovernanceConfig {
        &self.config
    }

    /// Verify a governance action including timelock (wp-v0.2 §7.5)
    ///
    /// First checks k-of-n signatures, then verifies the timelock has elapsed.
    /// Returns `TimelockNotElapsed` if signatures are valid but not enough
    /// blocks have passed since `proposed_at`.
    ///
    /// # Arguments
    /// - `action`: The governance action to verify
    /// - `current_block`: Current block height for timelock check
    pub fn verify_action_with_timelock(
        &self,
        action: &GovernanceAction,
        current_block: u64,
    ) -> VerificationResult {
        // First verify signatures
        let sig_result = self.verify_action(action);

        // If signatures failed, return that result
        if !matches!(sig_result, VerificationResult::Valid) {
            return sig_result;
        }

        // Check timelock: convert hours to blocks (assuming ~12s blocks = 300 blocks/hour)
        // This is a rough approximation; production may need more precise timekeeping
        let blocks_per_hour = 300u64;
        let required_delay = (self.config.timelock_hours as u64) * blocks_per_hour;

        // Handle potential overflow in addition
        let min_required_block = action.proposed_at.saturating_add(required_delay);

        if current_block < min_required_block {
            return VerificationResult::TimelockNotElapsed {
                proposed_at: action.proposed_at,
                current_block,
                required_delay: self.config.timelock_hours,
            };
        }

        VerificationResult::Valid
    }

    /// Canonicalize an action for signing/verification (excludes signatures)
    fn canonicalize_action(&self, action: &GovernanceAction) -> Vec<u8> {
        // Create a canonical representation without signatures
        let canonical = CanonicalGovernanceAction {
            proposal_id: action.proposal_id.clone(),
            action_type: action.action_type.clone(),
            proposed_at: action.proposed_at,
        };

        // Use compact JSON for determinism
        serde_json::to_vec(&canonical).unwrap_or_default()
    }
}

/// Canonical form of governance action for signing (no signatures field)
#[derive(Serialize)]
struct CanonicalGovernanceAction {
    proposal_id: String,
    action_type: GovernanceActionType,
    proposed_at: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use rand::RngCore;

    fn create_test_signer(did: &str) -> (GovernanceSigner, SigningKey) {
        // Generate random 32 bytes for secret key
        let mut secret_key = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut secret_key);
        let signing_key = SigningKey::from_bytes(&secret_key);
        let public_key = hex::encode(signing_key.verifying_key().to_bytes());

        let signer = GovernanceSigner {
            did: did.to_string(),
            public_key,
        };

        (signer, signing_key)
    }

    fn create_test_config(
        threshold: u32,
        num_signers: usize,
    ) -> (GovernanceConfig, Vec<SigningKey>) {
        let mut signers = Vec::new();
        let mut keys = Vec::new();

        for i in 0..num_signers {
            let (signer, key) = create_test_signer(&format!("did:apodokimos:signer{}", i));
            signers.push(signer);
            keys.push(key);
        }

        let config = GovernanceConfig {
            threshold,
            signers,
            timelock_hours: 168,
        };

        (config, keys)
    }

    fn sign_action(
        action: &GovernanceAction,
        signing_key: &SigningKey,
        did: &str,
    ) -> GovernanceSignature {
        let canonical = CanonicalGovernanceAction {
            proposal_id: action.proposal_id.clone(),
            action_type: action.action_type.clone(),
            proposed_at: action.proposed_at,
        };
        let message = serde_json::to_vec(&canonical).unwrap();
        let signature = signing_key.sign(&message[..]);

        GovernanceSignature {
            signer_did: did.to_string(),
            signature: hex::encode(signature.to_bytes()),
        }
    }

    #[test]
    fn governance_config_validation() {
        // Valid: 3-of-5
        let (config, _) = create_test_config(3, 5);
        assert!(config.validate().is_ok());

        // Invalid: threshold = 0
        let bad_config = GovernanceConfig {
            threshold: 0,
            signers: config.signers.clone(),
            timelock_hours: 168,
        };
        assert!(bad_config.validate().is_err());

        // Invalid: no signers
        let bad_config = GovernanceConfig {
            threshold: 1,
            signers: vec![],
            timelock_hours: 168,
        };
        assert!(bad_config.validate().is_err());

        // Invalid: threshold <= n/2 (Byzantine safety)
        let (config, _) = create_test_config(2, 5); // 2 <= 5/2 = 2
        assert!(config.validate().is_err());
    }

    #[test]
    fn governance_verifier_3_of_5() {
        let (config, keys) = create_test_config(3, 5);
        let verifier = GovernanceVerifier::new(config).unwrap();

        let action = GovernanceAction {
            proposal_id: "PROP-001".to_string(),
            action_type: GovernanceActionType::ParameterUpdate {
                param: "half_life".to_string(),
                value: "1825".to_string(),
            },
            proposed_at: 1000,
            signatures: vec![],
        };

        // No signatures - insufficient
        let result = verifier.verify_action(&action);
        assert!(matches!(
            result,
            VerificationResult::InsufficientSignatures { has: 0, needs: 3 }
        ));

        // 2 signatures - still insufficient
        let mut action_2 = action.clone();
        action_2
            .signatures
            .push(sign_action(&action, &keys[0], "did:apodokimos:signer0"));
        action_2
            .signatures
            .push(sign_action(&action, &keys[1], "did:apodokimos:signer1"));
        let result = verifier.verify_action(&action_2);
        assert!(matches!(
            result,
            VerificationResult::InsufficientSignatures { has: 2, needs: 3 }
        ));

        // 3 signatures - valid
        let mut action_3 = action.clone();
        action_3
            .signatures
            .push(sign_action(&action, &keys[0], "did:apodokimos:signer0"));
        action_3
            .signatures
            .push(sign_action(&action, &keys[1], "did:apodokimos:signer1"));
        action_3
            .signatures
            .push(sign_action(&action, &keys[2], "did:apodokimos:signer2"));
        let result = verifier.verify_action(&action_3);
        assert_eq!(result, VerificationResult::Valid);

        // 4 signatures - also valid
        let mut action_4 = action.clone();
        action_4
            .signatures
            .push(sign_action(&action, &keys[0], "did:apodokimos:signer0"));
        action_4
            .signatures
            .push(sign_action(&action, &keys[1], "did:apodokimos:signer1"));
        action_4
            .signatures
            .push(sign_action(&action, &keys[2], "did:apodokimos:signer2"));
        action_4
            .signatures
            .push(sign_action(&action, &keys[3], "did:apodokimos:signer3"));
        let result = verifier.verify_action(&action_4);
        assert_eq!(result, VerificationResult::Valid);
    }

    #[test]
    fn governance_unauthorized_signer() {
        let (config, keys) = create_test_config(2, 3);
        let verifier = GovernanceVerifier::new(config).unwrap();

        let action = GovernanceAction {
            proposal_id: "PROP-002".to_string(),
            action_type: GovernanceActionType::EmergencyHalt { resume: false },
            proposed_at: 2000,
            signatures: vec![],
        };

        // Create an unauthorized signer
        let (unauthorized_signer, unauthorized_key) = create_test_signer("did:apodokimos:attacker");

        let mut bad_action = action.clone();
        bad_action
            .signatures
            .push(sign_action(&action, &keys[0], "did:apodokimos:signer0"));
        bad_action.signatures.push(sign_action(
            &action,
            &unauthorized_key,
            &unauthorized_signer.did,
        ));

        let result = verifier.verify_action(&bad_action);
        assert!(
            matches!(result, VerificationResult::UnauthorizedSigner { signer } if signer == "did:apodokimos:attacker")
        );
    }

    #[test]
    fn governance_invalid_signature() {
        let (config, keys) = create_test_config(2, 3);
        let verifier = GovernanceVerifier::new(config).unwrap();

        let action = GovernanceAction {
            proposal_id: "PROP-003".to_string(),
            action_type: GovernanceActionType::Upgrade {
                new_version: "v0.3.0".to_string(),
            },
            proposed_at: 3000,
            signatures: vec![],
        };

        // Create a different action (same signer, wrong message)
        let different_action = GovernanceAction {
            proposal_id: "PROP-004".to_string(),
            action_type: GovernanceActionType::Upgrade {
                new_version: "v0.4.0".to_string(),
            },
            proposed_at: 4000,
            signatures: vec![],
        };

        // Sign the different action but attach to original
        let mut bad_action = action.clone();
        bad_action.signatures.push(sign_action(
            &different_action,
            &keys[0],
            "did:apodokimos:signer0",
        ));
        bad_action
            .signatures
            .push(sign_action(&action, &keys[1], "did:apodokimos:signer1"));

        let result = verifier.verify_action(&bad_action);
        assert!(
            matches!(result, VerificationResult::InvalidSignature { signer, .. } if signer == "did:apodokimos:signer0")
        );
    }

    #[test]
    fn governance_duplicate_signature() {
        let (config, keys) = create_test_config(2, 3);
        let verifier = GovernanceVerifier::new(config).unwrap();

        let action = GovernanceAction {
            proposal_id: "PROP-005".to_string(),
            action_type: GovernanceActionType::ParameterUpdate {
                param: "test".to_string(),
                value: "value".to_string(),
            },
            proposed_at: 5000,
            signatures: vec![],
        };

        // Same signer twice
        let mut bad_action = action.clone();
        bad_action
            .signatures
            .push(sign_action(&action, &keys[0], "did:apodokimos:signer0"));
        bad_action
            .signatures
            .push(sign_action(&action, &keys[0], "did:apodokimos:signer0"));

        let result = verifier.verify_action(&bad_action);
        assert!(
            matches!(result, VerificationResult::InvalidSignature { signer, reason }
                if signer == "did:apodokimos:signer0" && reason == "duplicate signature")
        );
    }
}

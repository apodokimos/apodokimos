//! Error types for apodokimos-core

use thiserror::Error;

/// Errors that can occur in the Apodokimos core protocol
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ApodokimosError {
    /// Invalid claim ID format
    #[error("invalid claim id: {0}")]
    InvalidClaimId(String),

    /// Serialization failure
    #[error("serialization error: {0}")]
    Serialization(String),

    /// Deserialization failure  
    #[error("deserialization error: {0}")]
    Deserialization(String),

    /// Invalid field identifier
    #[error("invalid field id: {0}")]
    InvalidFieldId(String),

    /// Dependency cycle detected
    #[error("dependency cycle detected: claim {claim_id:?} depends on itself transitively")]
    DependencyCycle { claim_id: crate::ClaimId },

    /// Claim not found
    #[error("claim not found: {0}")]
    ClaimNotFound(String),

    /// Attestation validation failed
    #[error("attestation validation failed: {0}")]
    AttestationValidation(String),

    /// Hash verification failed
    #[error("hash verification failed: computed {computed:?} but expected {expected:?}")]
    HashVerification {
        computed: crate::ClaimId,
        expected: crate::ClaimId,
    },
}

impl ApodokimosError {
    /// Check if this error is a validation error
    pub const fn is_validation_error(&self) -> bool {
        matches!(
            self,
            Self::InvalidClaimId(_)
                | Self::InvalidFieldId(_)
                | Self::AttestationValidation(_)
                | Self::DependencyCycle { .. }
        )
    }

    /// Check if this error is a data integrity error
    pub const fn is_integrity_error(&self) -> bool {
        matches!(self, Self::HashVerification { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_classification() {
        let validation_err = ApodokimosError::InvalidClaimId("bad-id".to_string());
        assert!(validation_err.is_validation_error());
        assert!(!validation_err.is_integrity_error());

        let integrity_err = ApodokimosError::HashVerification {
            computed: crate::ClaimId::from_bytes([0u8; 32]),
            expected: crate::ClaimId::from_bytes([1u8; 32]),
        };
        assert!(!integrity_err.is_validation_error());
        assert!(integrity_err.is_integrity_error());
    }

    #[test]
    fn error_display() {
        let err = ApodokimosError::InvalidFieldId("bad-field!".to_string());
        let msg = err.to_string();
        assert!(msg.contains("bad-field!"));
    }
}

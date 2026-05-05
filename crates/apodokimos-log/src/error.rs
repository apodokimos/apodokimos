use thiserror::Error;

/// Errors returned by the transparency log client.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum LogError {
    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("signature format error: {0}")]
    SignatureFormat(String),

    #[error("signature verification failed: {0}")]
    SignatureVerification(String),

    #[error("public key format error: {0}")]
    PublicKeyFormat(String),

    #[error("entry not found in log")]
    EntryNotFound,

    #[error("merkle proof unavailable: {0}")]
    ProofUnavailable(String),
}

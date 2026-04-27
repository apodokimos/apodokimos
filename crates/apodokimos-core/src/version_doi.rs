//! Version DOI type for claim specification versioning (wp-v0.2 §2.2)
//!
//! A Version DOI is a DOI string that identifies the protocol specification version
//! under which a claim was registered. This enables protocol evolution while
//! maintaining backward compatibility and deterministic weight computation.

use crate::ApodokimosError;
use alloc::string::String;
use alloc::str::FromStr;
use serde::{Deserialize, Serialize};

/// A DOI identifying a specific protocol specification version (P-08)
///
/// Format: `doi:10.xxxx/apodokimos.<version>` where `<version>` follows
/// the wp-v{major}.{minor} convention (e.g., `wp-v0.2`).
///
/// # Example
/// ```
/// use apodokimos_core::VersionDOI;
///
/// let doi = VersionDOI::new("doi:10.5281/apodokimos.wp-v0.2").unwrap();
/// assert_eq!(doi.as_str(), "doi:10.5281/apodokimos.wp-v0.2");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VersionDOI(String);

impl VersionDOI {
    /// Prefix for Apodokimos DOIs
    pub const APODOKIMOS_PREFIX: &'static str = "doi:10.";

    /// Default Version DOI for wp-v0.2 (zenodo placeholder)
    pub fn wp_v0_2() -> Self {
        Self::new_static("doi:10.5281/apodokimos.wp-v0.2")
    }

    /// Create a new Version DOI with runtime validation
    ///
    /// # Errors
    /// Returns `ApodokimosError::InvalidVersionDOI` if the string doesn't start
    /// with the expected DOI prefix.
    pub fn new(doi: impl Into<String>) -> Result<Self, ApodokimosError> {
        let s = doi.into();
        Self::validate(&s)?;
        Ok(Self(s))
    }

    /// Create a Version DOI from a validated static string
    ///
    /// # Panics
    /// Panics if the string is not a valid DOI format (only used with known-valid constants).
    #[track_caller]
    pub fn new_static(doi: &'static str) -> Self {
        Self::validate(doi).expect("static DOI must be valid");
        Self(doi.into())
    }

    /// Validate a DOI string
    fn validate(s: &str) -> Result<(), ApodokimosError> {
        if !s.starts_with(Self::APODOKIMOS_PREFIX) {
            return Err(ApodokimosError::InvalidVersionDOI(alloc::format!(
                "DOI must start with '{}', got: {}",
                Self::APODOKIMOS_PREFIX,
                s
            )));
        }
        Ok(())
    }

    /// Get the DOI string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Extract the version component (e.g., "wp-v0.2" from "doi:10.xxxx/apodokimos.wp-v0.2")
    pub fn version_component(&self) -> Option<&str> {
        self.0
            .split("/apodokimos.")
            .nth(1)
    }

    /// Check if this is a wp-v0.2 or later version
    pub fn is_v0_2_or_later(&self) -> bool {
        self.version_component()
            .map(|v| v.starts_with("wp-v0.2") || v.starts_with("wp-v0.3") || v.starts_with("wp-v0.4") || v.starts_with("wp-v1"))
            .unwrap_or(false)
    }
}

impl Default for VersionDOI {
    fn default() -> Self {
        Self::wp_v0_2()
    }
}

impl AsRef<str> for VersionDOI {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for VersionDOI {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for VersionDOI {
    type Err = ApodokimosError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_doi_parsing() {
        let doi = VersionDOI::new("doi:10.5281/apodokimos.wp-v0.2").unwrap();
        assert_eq!(doi.as_str(), "doi:10.5281/apodokimos.wp-v0.2");
        assert_eq!(doi.version_component(), Some("wp-v0.2"));
        assert!(doi.is_v0_2_or_later());

        // Test from_str
        let doi2: VersionDOI = "doi:10.5281/apodokimos.wp-v0.2".parse().unwrap();
        assert_eq!(doi, doi2);
    }

    #[test]
    fn invalid_doi_rejected() {
        let result = VersionDOI::new("not-a-doi");
        assert!(result.is_err());
    }

    #[test]
    fn version_component_extraction() {
        let doi = VersionDOI::new("doi:10.5281/apodokimos.wp-v0.1").unwrap();
        assert_eq!(doi.version_component(), Some("wp-v0.1"));
        assert!(!doi.is_v0_2_or_later());
    }

    #[test]
    fn serde_round_trip() {
        let doi = VersionDOI::new("doi:10.5281/apodokimos.wp-v0.2").unwrap();
        let json = serde_json::to_string(&doi).unwrap();
        assert_eq!(json, "\"doi:10.5281/apodokimos.wp-v0.2\"");
        
        let deserialized: VersionDOI = serde_json::from_str(&json).unwrap();
        assert_eq!(doi, deserialized);
    }
}

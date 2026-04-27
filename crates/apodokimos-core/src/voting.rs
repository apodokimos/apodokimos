//! Cross-field voting weight computation (C-31, wp-v0.2 §7.1)
//!
//! Implements the corrected cross-field voting formula from wp-v0.2:
//! vote_weight(account, global) = sqrt(mean_nonzero(field_scores(account)))
//!
//! where mean_nonzero(s) = (Σ_{f : s[f] > 0} s[f]) / |{f : s[f] > 0}|
//!
//! This fixes the wp-v0.1 bug where geometric mean would zero out specialists.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// Field identifier for SBT scores
pub type FieldId = String;

/// SBT score for a specific field
pub type SbtScore = u64;

/// Account's SBT scores across fields
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountSbt {
    /// DID of the account
    pub did: String,
    /// SBT scores per field
    pub field_scores: BTreeMap<FieldId, SbtScore>,
}

impl AccountSbt {
    /// Create a new account with SBT scores
    pub fn new(did: impl Into<String>) -> Self {
        Self {
            did: did.into(),
            field_scores: BTreeMap::new(),
        }
    }

    /// Add or update a field score
    pub fn with_field_score(mut self, field: impl Into<String>, score: SbtScore) -> Self {
        self.field_scores.insert(field.into(), score);
        self
    }

    /// Compute the arithmetic mean over non-zero field scores (wp-v0.2 §7.1)
    ///
    /// Returns None if there are no non-zero field scores.
    pub fn mean_nonzero(&self) -> Option<f64> {
        let non_zero_scores: Vec<u64> = self
            .field_scores
            .values()
            .copied()
            .filter(|&s| s > 0)
            .collect();

        if non_zero_scores.is_empty() {
            return None;
        }

        let sum: u64 = non_zero_scores.iter().sum();
        let count = non_zero_scores.len() as u64;

        Some(sum as f64 / count as f64)
    }

    /// Compute cross-field voting weight (wp-v0.2 §7.1)
    ///
    /// Formula: sqrt(mean_nonzero(field_scores))
    ///
    /// Returns None if the account has no non-zero field scores.
    pub fn cross_field_vote_weight(&self) -> Option<f64> {
        self.mean_nonzero().map(|mean| mean.sqrt())
    }

    /// Compute single-field voting weight (wp-v0.2 §7.1)
    ///
    /// Formula: sqrt(field_score)
    ///
    /// Returns None if the account has no score in that field.
    pub fn field_vote_weight(&self, field: &str) -> Option<f64> {
        self.field_scores.get(field).copied().map(|s| (s as f64).sqrt())
    }
}

/// Compute voting weight for a list of accounts (for quorum calculations)
pub fn total_voting_weight(accounts: &[AccountSbt]) -> f64 {
    accounts
        .iter()
        .filter_map(|acc| acc.cross_field_vote_weight())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test: Specialist with one field retains full weight (C-31, wp-v0.2 §7.1)
    ///
    /// A specialist with high score in one field should have the same
    /// cross-field weight as their single-field weight.
    #[test]
    fn specialist_single_field_retains_full_weight() {
        let specialist = AccountSbt::new("did:apodokimos:specialist")
            .with_field_score("clinical-medicine", 10000);

        // Single-field weight: sqrt(10000) = 100
        let field_weight = specialist.field_vote_weight("clinical-medicine");
        assert!(field_weight.is_some());
        let field_weight = field_weight.unwrap();
        assert!((field_weight - 100.0).abs() < 1e-10, "Field weight should be 100, got {}", field_weight);

        // Cross-field weight: sqrt(mean_nonzero([10000])) = sqrt(10000) = 100
        let cross_weight = specialist.cross_field_vote_weight();
        assert!(cross_weight.is_some());
        let cross_weight = cross_weight.unwrap();
        assert!((cross_weight - 100.0).abs() < 1e-10, "Cross-field weight should be 100, got {}", cross_weight);

        // Specialist retains full weight across both metrics
        assert!(
            (field_weight - cross_weight).abs() < 1e-10,
            "Specialist should retain full weight: field={}, cross={}",
            field_weight,
            cross_weight
        );
    }

    /// Test: Arithmetic mean over non-zero fields (not geometric) (C-31, wp-v0.2 §7.1)
    ///
    /// The fix from wp-v0.1 uses arithmetic mean over non-zero fields,
    /// not geometric mean which would zero out if any field is zero.
    #[test]
    fn arithmetic_mean_over_nonzero_fields() {
        let account = AccountSbt::new("did:apodokimos:researcher")
            .with_field_score("field-a", 100)
            .with_field_score("field-b", 200)
            .with_field_score("field-c", 300);

        // Arithmetic mean: (100 + 200 + 300) / 3 = 200
        let mean = account.mean_nonzero().unwrap();
        assert!(
            (mean - 200.0).abs() < 1e-10,
            "Arithmetic mean should be 200, got {}",
            mean
        );

        // Cross-field weight: sqrt(200) ≈ 14.14
        let weight = account.cross_field_vote_weight().unwrap();
        assert!((weight - 14.142135623730951).abs() < 1e-10);
    }

    /// Test: Zero fields don't zero out the entire weight (C-31, wp-v0.2 §7.1)
    ///
    /// This is the key fix from wp-v0.1: geometric mean would be zero
    /// if any field score is zero, but arithmetic mean over non-zero
    /// fields preserves specialist weight.
    #[test]
    fn zero_fields_do_not_zero_out_weight() {
        // Specialist with one strong field, many zero fields
        let specialist = AccountSbt::new("did:apodokimos:specialist")
            .with_field_score("clinical-medicine", 10000)
            .with_field_score("physics", 0)  // Zero score
            .with_field_score("mathematics", 0)  // Zero score
            .with_field_score("biology", 0);  // Zero score

        // Arithmetic mean over non-zero: mean([10000, 0, 0, 0]) = 10000 / 1 = 10000
        // (only counts non-zero fields)
        let mean = specialist.mean_nonzero();
        assert!(mean.is_some(), "Should have non-zero mean");
        let mean = mean.unwrap();
        assert!(
            (mean - 10000.0).abs() < 1e-10,
            "Mean should be 10000 (ignoring zero fields), got {}",
            mean
        );

        // Cross-field weight: sqrt(10000) = 100
        let weight = specialist.cross_field_vote_weight().unwrap();
        assert!(
            (weight - 100.0).abs() < 1e-10,
            "Specialist weight should not be zero: got {}",
            weight
        );
    }

    /// Test: Generalist gets averaged weight (C-31, wp-v0.2 §7.1)
    ///
    /// A generalist with moderate scores in many fields should have
    /// lower cross-field weight than a specialist with high score in one field.
    #[test]
    fn generalist_gets_averaged_weight() {
        // Specialist: 10000 in one field
        let specialist = AccountSbt::new("did:specialist")
            .with_field_score("clinical-medicine", 10000);

        // Generalist: 1000 in each of 10 fields (total 10000)
        let mut generalist = AccountSbt::new("did:generalist");
        for i in 0..10 {
            generalist = generalist.with_field_score(
                format!("field-{}", i),
                1000
            );
        }

        let specialist_weight = specialist.cross_field_vote_weight().unwrap();
        let generalist_weight = generalist.cross_field_vote_weight().unwrap();

        // Specialist: sqrt(10000) = 100
        assert!((specialist_weight - 100.0).abs() < 1e-10);

        // Generalist: mean = (1000 × 10) / 10 = 1000, sqrt(1000) ≈ 31.62
        assert!((generalist_weight - 31.622776601683793).abs() < 1e-10);

        // Specialist should have higher voting power
        assert!(
            specialist_weight > generalist_weight,
            "Specialist ({}) should have higher weight than generalist ({})",
            specialist_weight,
            generalist_weight
        );

        // Generalist gets roughly 1/sqrt(N) of specialist weight for N fields
        // With 10 fields: 31.62 vs 100 ≈ 1/3.16 ≈ 1/sqrt(10)
        let ratio = specialist_weight / generalist_weight;
        assert!(
            (ratio - 3.1622776601683795).abs() < 1e-10,
            "Ratio should be ~3.16 (sqrt(10)), got {}",
            ratio
        );
    }

    /// Test: No non-zero scores means no voting power (C-31, wp-v0.2 §7.1)
    ///
    /// An account with no SBT scores cannot vote.
    #[test]
    fn no_nonzero_scores_means_no_voting_power() {
        let no_scores = AccountSbt::new("did:apodokimos:newbie");
        let all_zeros = AccountSbt::new("did:apodokimos:inactive")
            .with_field_score("field-a", 0)
            .with_field_score("field-b", 0);

        assert!(
            no_scores.mean_nonzero().is_none(),
            "Empty scores should have no mean"
        );
        assert!(
            no_scores.cross_field_vote_weight().is_none(),
            "Empty scores should have no voting weight"
        );

        assert!(
            all_zeros.mean_nonzero().is_none(),
            "All-zero scores should have no mean"
        );
        assert!(
            all_zeros.cross_field_vote_weight().is_none(),
            "All-zero scores should have no voting weight"
        );
    }

    /// Test: Single-field weight vs cross-field weight (C-31, wp-v0.2 §7.1)
    ///
    /// Single-field weight uses sqrt(score), cross-field uses sqrt(mean_nonzero).
    /// For a single-field specialist, these are equal.
    /// For a generalist, single-field weight is field-specific.
    #[test]
    fn single_field_vs_cross_field_weight() {
        let account = AccountSbt::new("did:apodokimos:researcher")
            .with_field_score("field-a", 100)
            .with_field_score("field-b", 400);

        // Single-field weights
        let weight_a = account.field_vote_weight("field-a").unwrap(); // sqrt(100) = 10
        let weight_b = account.field_vote_weight("field-b").unwrap(); // sqrt(400) = 20

        assert!((weight_a - 10.0).abs() < 1e-10);
        assert!((weight_b - 20.0).abs() < 1e-10);

        // Cross-field weight: mean = (100 + 400) / 2 = 250, sqrt(250) ≈ 15.81
        let cross_weight = account.cross_field_vote_weight().unwrap();
        assert!((cross_weight - 15.811388300841896).abs() < 1e-10);

        // Cross-field weight is between the two single-field weights
        assert!(
            weight_a < cross_weight && cross_weight < weight_b,
            "Cross-field weight ({}) should be between field-a ({}) and field-b ({})",
            cross_weight,
            weight_a,
            weight_b
        );
    }

    /// Test: Total voting weight aggregation (C-31, wp-v0.2 §7.1)
    ///
    /// Quorum calculations sum the voting weights of all participating accounts.
    #[test]
    fn total_voting_weight_aggregation() {
        let accounts = vec![
            AccountSbt::new("did:a").with_field_score("field-1", 100), // sqrt(100) = 10
            AccountSbt::new("did:b").with_field_score("field-1", 100), // sqrt(100) = 10
            AccountSbt::new("did:c").with_field_score("field-1", 100), // sqrt(100) = 10
        ];

        // Each has mean_nonzero = 100, cross-field weight = 10
        let total = total_voting_weight(&accounts);
        assert!((total - 30.0).abs() < 1e-10, "Total weight should be 30, got {}", total);
    }

    /// Test: Diminishing returns on concentration (C-31, wp-v0.2 §7.1)
    ///
    /// The sqrt in the formula provides diminishing returns on concentration.
    /// Doubling a field score only increases weight by sqrt(2) ≈ 1.41x.
    #[test]
    fn diminishing_returns_on_concentration() {
        let base = AccountSbt::new("did:base").with_field_score("field", 100);
        let doubled = AccountSbt::new("did:double").with_field_score("field", 400);

        let base_weight = base.cross_field_vote_weight().unwrap();     // sqrt(100) = 10
        let doubled_weight = doubled.cross_field_vote_weight().unwrap(); // sqrt(400) = 20

        // Score increased 4x (100 -> 400), weight increased 2x (10 -> 20)
        assert!(
            (doubled_weight / base_weight - 2.0).abs() < 1e-10,
            "4x score increase should give 2x weight (sqrt relationship)"
        );
    }
}

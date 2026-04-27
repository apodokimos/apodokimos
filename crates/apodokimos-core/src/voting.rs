//! Cross-field voting weight computation (C-31, wp-v0.2 §7.1)
//!
//! Implements the corrected cross-field voting formula from wp-v0.2:
//! vote_weight(account, global) = sqrt(mean_nonzero(field_scores(account)))
//!
//! where mean_nonzero(s) = (Σ_{f : s[f] > 0} s[f]) / |{f : s[f] > 0}|
//!
//! This fixes the wp-v0.1 bug where geometric mean would zero out specialists.

use crate::claim::FieldId;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

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
    ///
    /// # Overflow Safety
    /// Uses saturating arithmetic to prevent overflow on extremely large SBT scores.
    /// In practice, SBT scores grow slowly via demonstrated contribution, so reaching
    /// `u64::MAX` across fields is not realistic, but the saturation ensures correctness.
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

        // Saturating sum to prevent overflow on pathological inputs
        let sum: u64 = non_zero_scores
            .iter()
            .fold(0u64, |acc, &x| acc.saturating_add(x));
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
        self.field_scores
            .get(field)
            .copied()
            .map(|s| (s as f64).sqrt())
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
        assert!(
            (field_weight - 100.0).abs() < 1e-10,
            "Field weight should be 100, got {}",
            field_weight
        );

        // Cross-field weight: sqrt(mean_nonzero([10000])) = sqrt(10000) = 100
        let cross_weight = specialist.cross_field_vote_weight();
        assert!(cross_weight.is_some());
        let cross_weight = cross_weight.unwrap();
        assert!(
            (cross_weight - 100.0).abs() < 1e-10,
            "Cross-field weight should be 100, got {}",
            cross_weight
        );

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
            .with_field_score("physics", 0) // Zero score
            .with_field_score("mathematics", 0) // Zero score
            .with_field_score("biology", 0); // Zero score

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
        let specialist =
            AccountSbt::new("did:specialist").with_field_score("clinical-medicine", 10000);

        // Generalist: 1000 in each of 10 fields (total 10000)
        let mut generalist = AccountSbt::new("did:generalist");
        for i in 0..10 {
            generalist = generalist.with_field_score(format!("field-{}", i), 1000);
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
        assert!(
            (total - 30.0).abs() < 1e-10,
            "Total weight should be 30, got {}",
            total
        );
    }

    /// Test: Diminishing returns on concentration (C-31, wp-v0.2 §7.1)
    ///
    /// The sqrt in the formula provides diminishing returns on concentration.
    /// Doubling a field score only increases weight by sqrt(2) ≈ 1.41x.
    #[test]
    fn diminishing_returns_on_concentration() {
        let base = AccountSbt::new("did:base").with_field_score("field", 100);
        let doubled = AccountSbt::new("did:double").with_field_score("field", 400);

        let base_weight = base.cross_field_vote_weight().unwrap(); // sqrt(100) = 10
        let doubled_weight = doubled.cross_field_vote_weight().unwrap(); // sqrt(400) = 20

        // Score increased 4x (100 -> 400), weight increased 2x (10 -> 20)
        assert!(
            (doubled_weight / base_weight - 2.0).abs() < 1e-10,
            "4x score increase should give 2x weight (sqrt relationship)"
        );
    }

    // =========================================================================
    // C-32: Sybil-resistance documentation test (wp-v0.2 §12.1)
    // =========================================================================

    /// Test: Quadratic voting INCREASES total weight under fragmentation (C-32, wp-v0.2 §12.1)
    ///
    /// This test documents a critical property: quadratic voting (sqrt) is NOT
    /// Sybil-resistant. Fragmenting a score across multiple accounts increases
    /// total voting power because sqrt is subadditive.
    ///
    /// Mathematical proof:
    /// - Single account with score S: weight = sqrt(S)
    /// - N accounts each with score S/N: total = N × sqrt(S/N) = sqrt(N) × sqrt(S)
    /// - Since sqrt(N) > 1 for N > 1, fragmentation increases total weight
    ///
    /// Sybil resistance in Apodokimos comes from SBT-accumulation cost (§6.3),
    /// NOT from the voting formula. Quadratic voting addresses concentration
    /// resistance; SBT-cost addresses Sybil resistance.
    #[test]
    fn quadratic_voting_increases_weight_under_fragmentation() {
        let total_score = 10000u64;

        // Single account with all 10,000 score
        let single = AccountSbt::new("did:single").with_field_score("field", total_score);
        let single_weight = single.field_vote_weight("field").unwrap();
        // sqrt(10000) = 100
        assert!((single_weight - 100.0).abs() < 1e-10);

        // Fragmented into 2 accounts with 5,000 each
        let frag2_a = AccountSbt::new("did:frag2-a").with_field_score("field", 5000);
        let frag2_b = AccountSbt::new("did:frag2-b").with_field_score("field", 5000);
        let frag2_weight_a = frag2_a.field_vote_weight("field").unwrap(); // sqrt(5000) ≈ 70.71
        let frag2_weight_b = frag2_b.field_vote_weight("field").unwrap(); // sqrt(5000) ≈ 70.71
        let frag2_total = frag2_weight_a + frag2_weight_b;
        // Total: 2 × sqrt(5000) = 2 × 70.71 = 141.42
        assert!((frag2_total - 141.4213562373095).abs() < 1e-10);

        // Fragmented into 4 accounts with 2,500 each
        let frag4: Vec<_> = (0..4)
            .map(|i| AccountSbt::new(format!("did:frag4-{}", i)).with_field_score("field", 2500))
            .collect();
        let frag4_total: f64 = frag4
            .iter()
            .filter_map(|a| a.field_vote_weight("field"))
            .sum();
        // Total: 4 × sqrt(2500) = 4 × 50 = 200
        assert!((frag4_total - 200.0).abs() < 1e-10);

        // Fragmented into 10 accounts with 1,000 each
        let frag10: Vec<_> = (0..10)
            .map(|i| AccountSbt::new(format!("did:frag10-{}", i)).with_field_score("field", 1000))
            .collect();
        let frag10_total: f64 = frag10
            .iter()
            .filter_map(|a| a.field_vote_weight("field"))
            .sum();
        // Total: 10 × sqrt(1000) = 10 × 31.62 = 316.23
        assert!((frag10_total - 316.22776601683796).abs() < 1e-10);

        // Fragmented into 100 accounts with 100 each
        let frag100: Vec<_> = (0..100)
            .map(|i| AccountSbt::new(format!("did:frag100-{}", i)).with_field_score("field", 100))
            .collect();
        let frag100_total: f64 = frag100
            .iter()
            .filter_map(|a| a.field_vote_weight("field"))
            .sum();
        // Total: 100 × sqrt(100) = 100 × 10 = 1000
        assert!((frag100_total - 1000.0).abs() < 1e-10);

        // Verify the mathematical relationship: frag_total = sqrt(N) × single_weight
        // where N is the number of fragments
        let test_cases: Vec<(f64, f64)> = vec![
            (2.0, frag2_total),
            (4.0, frag4_total),
            (10.0, frag10_total),
            (100.0, frag100_total),
        ];

        for (n, actual_total) in test_cases {
            let expected_multiplier = n.sqrt();
            let expected_total = expected_multiplier * single_weight;
            assert!(
                (actual_total - expected_total).abs() < 1e-10,
                "With {} fragments: expected {}× single weight = {}, got {}",
                n,
                expected_multiplier,
                expected_total,
                actual_total
            );
        }

        // The critical insight: more fragments = more total weight
        assert!(
            frag2_total > single_weight,
            "2 fragments should exceed single account weight"
        );
        assert!(
            frag4_total > frag2_total,
            "4 fragments should exceed 2 fragments"
        );
        assert!(
            frag10_total > frag4_total,
            "10 fragments should exceed 4 fragments"
        );
        assert!(
            frag100_total > frag10_total,
            "100 fragments should exceed 10 fragments"
        );

        // With 100 fragments, total weight is 10× the single account weight!
        // This proves quadratic voting encourages fragmentation, not Sybil resistance
        let ratio = frag100_total / single_weight;
        assert!(
            (ratio - 10.0).abs() < 1e-10,
            "100 fragments give 10x the voting power: {} vs {}",
            frag100_total,
            single_weight
        );
    }

    /// Test: Sybil resistance via SBT accumulation cost (C-32, wp-v0.2 §12.1)
    ///
    /// This test documents that Sybil resistance comes from the cost of
    /// accumulating SBT scores, not from the voting formula. Each identity
    /// must independently earn its SBT through demonstrated work.
    ///
    /// The SBT-cost model means:
    /// - Creating 1 account with 10,000 SBT requires 10,000 units of work
    /// - Creating 100 accounts with 100 SBT each requires 100 × 100 = 10,000 units of work
    ///
    /// The cost is the same, but the fragmented strategy requires managing
    /// 100x more identities and the total voting power is 10x higher.
    /// However, the identity layer (§8) binds DIDs to real-world credentials,
    /// making mass identity creation expensive/impossible.
    #[test]
    fn sybil_resistance_via_sbt_accumulation_cost() {
        // Scenario: An attacker wants to maximize voting power
        // They have resources to accumulate 10,000 total SBT score

        let total_work_capacity = 10000u64;

        // Strategy 1: Single concentrated account
        // Cost: 10,000 work units → 1 account with 10,000 SBT
        // Voting power: sqrt(10,000) = 100
        let strategy1 =
            AccountSbt::new("did:attacker-1").with_field_score("field", total_work_capacity);
        let power1 = strategy1.field_vote_weight("field").unwrap();
        assert!((power1 - 100.0).abs() < 1e-10);

        // Strategy 2: Fragmented across 100 Sybil accounts
        // Cost: 100 × 100 = 10,000 work units (same total cost!)
        // But each account needs 100 SBT, requiring 100 independent work streams
        let sbt_per_sybil = total_work_capacity / 100;
        let sybils: Vec<_> = (0..100)
            .map(|i| {
                AccountSbt::new(format!("did:sybil-{}", i)).with_field_score("field", sbt_per_sybil)
            })
            .collect();
        let power2: f64 = sybils
            .iter()
            .filter_map(|a| a.field_vote_weight("field"))
            .sum();
        // Power: 100 × sqrt(100) = 100 × 10 = 1000
        assert!((power2 - 1000.0).abs() < 1e-10);

        // The fragmented strategy gives 10x more voting power for the same work!
        // This is why the SBT-cost model matters:
        // - In a token system: 1 token = 1 vote, fragmentation doesn't help
        // - In SBT system: work is identity-bound, can't be parallelized easily
        let advantage_ratio = power2 / power1;
        assert!(
            (advantage_ratio - 10.0).abs() < 1e-10,
            "Fragmented strategy gives {}x advantage for same work cost",
            advantage_ratio
        );

        // The defense: Identity layer (§8) ensures each SBT-earning identity
        // requires real-world credentials. Creating 100 independent work streams
        // that each earn 100 SBT is 100x harder than creating 1 stream that earns
        // 10,000 SBT (due to coordination overhead, not just computational cost).
    }
}

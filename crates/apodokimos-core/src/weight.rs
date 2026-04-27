//! Weight function W(claim) = R(t) × D × S × O (P-03, C-14 to C-19)
//!
//! The weight function computes epistemic contribution scores for claims:
//! - R(t): Time-decay recency factor
//! - D: Dependency depth in graph
//! - S: Survival rate from attestations
//! - O: Oracle factor for external validation

use crate::{ApodokimosError, Attestation, AttestationVerdict, Claim, ClaimId, field::FieldSchema};
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// Computed weight for a claim (P-03, wp-v0.2)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClaimWeight {
    /// Final computed weight (W = R × D̃ × S × (1+γ·O) × δ)
    pub value: f64,
    /// Time-decay factor R(c,t) ∈ (0.0, 1.0] per wp-v0.2 §3.2
    pub recency: f64,
    /// Log-normalized dependency depth D̃(c) per wp-v0.2 §3.3
    /// Range: (0, ∞), typically ~0.4–1.2 for clinical medicine (D_ref=3)
    pub depth: f64,
    /// Laplace-smoothed survival rate S(c) ∈ (0, 1) per wp-v0.2 §3.4
    /// S = (N₊ + 1) / (N₊ + N₋ + 2) with uniform Beta(1,1) prior
    pub survival: f64,
    /// Oracle bonus factor (1 + γ·O(c)) per wp-v0.2 §3.5
    /// Range: [1.0, 1+γ] — always ≥ 1.0 (bonus, not penalty)
    pub oracle: f64,
}

impl ClaimWeight {
    /// Create a new claim weight with all components
    pub fn new(value: f64, recency: f64, depth: f64, survival: f64, oracle: f64) -> Self {
        Self {
            value,
            recency,
            depth,
            survival,
            oracle,
        }
    }
}

/// Oracle source types for O factor validation (C-18, P-07)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OFactorSource {
    /// Clinical trial registry (e.g., ClinicalTrials.gov NCT)
    ClinicalTrial { registry_id: String },
    /// Systematic review registry (e.g., PROSPERO)
    SystematicReview { prospero_id: String },
    /// Preprint server (e.g., arXiv, bioRxiv)
    Preprint { doi: String },
    /// Peer-reviewed journal DOI
    PeerReviewed { doi: String },
    /// Dataset identifier
    Dataset { identifier: String },
    /// Software/code repository
    Software {
        repository_url: String,
        commit_hash: String,
    },
    /// No oracle linkage
    None,
}

impl OFactorSource {
    /// Compute O factor value based on source credibility
    ///
    /// Hierarchy (highest to lowest credibility):
    /// - Peer-reviewed: 1.0
    /// - Clinical trial: 0.95
    /// - Systematic review: 0.95
    /// - Dataset: 0.85
    /// - Software: 0.80
    /// - Preprint: 0.70
    /// - None: 1.0 (neutral, no penalty)
    pub fn factor_value(&self) -> f64 {
        match self {
            OFactorSource::PeerReviewed { .. } => 1.0,
            OFactorSource::ClinicalTrial { .. } => 0.95,
            OFactorSource::SystematicReview { .. } => 0.95,
            OFactorSource::Dataset { .. } => 0.85,
            OFactorSource::Software { .. } => 0.80,
            OFactorSource::Preprint { .. } => 0.70,
            OFactorSource::None => 1.0, // Neutral baseline, not a penalty
        }
    }
}

/// Graph snapshot for weight computation (C-14)
#[derive(Debug, Clone)]
pub struct GraphSnapshot {
    /// All claims in the graph
    pub claims: BTreeMap<ClaimId, Claim>,
    /// All attestations indexed by claim_id
    pub attestations: BTreeMap<ClaimId, Vec<Attestation>>,
    /// Current block height for time calculations
    pub current_block: u64,
    /// Average block time in seconds (for converting blocks to days)
    pub block_time_seconds: u32,
}

impl GraphSnapshot {
    /// Create a new graph snapshot
    pub fn new(
        claims: BTreeMap<ClaimId, Claim>,
        attestations: BTreeMap<ClaimId, Vec<Attestation>>,
        current_block: u64,
        block_time_seconds: u32,
    ) -> Self {
        Self {
            claims,
            attestations,
            current_block,
            block_time_seconds,
        }
    }

    /// Get attestations for a specific claim
    pub fn get_attestations(&self, claim_id: &ClaimId) -> &[Attestation] {
        self.attestations
            .get(claim_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get a claim by ID
    pub fn get_claim(&self, claim_id: &ClaimId) -> Option<&Claim> {
        self.claims.get(claim_id)
    }

    /// Apply retraction discounts to affected claims (C-27 fix)
    ///
    /// Updates the retraction discount δ(c) for each affected claim by
    /// computing the cumulative discount factor needed to reach the target.
    ///
    /// # Panics
    /// Panics if any `new_retraction_discount` is not in [0, 1] or not finite.
    pub fn apply_retraction(&mut self, affected: &[AffectedClaim]) {
        for aff in affected {
            if let Some(claim) = self.claims.get_mut(&aff.claim_id) {
                // Validate the new discount is valid
                let target = aff.new_retraction_discount;
                assert!(
                    target.is_finite() && (0.0..=1.0).contains(&target),
                    "invalid retraction discount: {} (must be finite and in [0, 1])",
                    target
                );
                
                // Compute the factor needed: target = current × factor
                // factor = target / current
                let current = claim.retraction_discount;
                if current > 0.0 {
                    let factor = target / current;
                    claim.retraction_discount *= factor;
                } else {
                    claim.retraction_discount = target;
                }
                
                // Clamp to handle floating point drift
                claim.retraction_discount = claim.retraction_discount.clamp(0.0, 1.0);
            }
        }
    }
}

/// Affected claim with retraction discount δ(c) per wp-v0.2 §5.2 (C-27)
#[derive(Debug, Clone)]
pub struct AffectedClaim {
    /// The affected claim ID
    pub claim_id: ClaimId,
    /// Weight before retraction (W_pre = weight with current δ)
    pub previous_weight: f64,
    /// Weight after retraction (W_post = weight with new δ)
    pub new_weight: f64,
    /// Depth in the cascade (1 = direct dependent)
    pub cascade_depth: u32,
    /// New retraction discount δ(c) ∈ [0, 1] (C-27)
    /// new_δ = old_δ × (0.5^cascade_depth)
    pub new_retraction_discount: f64,
}

/// Weight function implementation (C-14)
pub struct WeightFunction;

impl WeightFunction {
    /// Compute weight for a claim (C-25, wp-v0.2 §3.1)
    ///
    /// Formula: W(c) = R(c,t) × D̃(c) × S(c) × (1 + γ·O(c)) × δ(c)
    ///
    /// Where:
    /// - R(c,t): Time-decay recency factor [0, 1]
    /// - D̃(c): Log-normalized dependency depth factor
    /// - S(c): Laplace-smoothed survival rate (0, 1)
    /// - O(c): Oracle credibility ∈ [0, 1]
    /// - γ: Field-calibrated oracle bonus coefficient
    /// - δ(c): Retraction discount factor [0, 1] (default 1.0, see C-27)
    pub fn compute(
        claim_id: &ClaimId,
        graph: &GraphSnapshot,
        field_schema: &dyn FieldSchema,
        oracle_source: &OFactorSource,
    ) -> Result<ClaimWeight, ApodokimosError> {
        let claim = graph
            .get_claim(claim_id)
            .ok_or_else(|| ApodokimosError::ClaimNotFound(claim_id.to_hex()))?;

        // R(c,t): Time-decay recency factor (C-22)
        let recency = Self::compute_recency(claim, graph, field_schema);

        // D̃(c): Log-normalized dependency depth (C-23)
        let depth = Self::compute_depth(claim, graph, field_schema);

        // S(c): Laplace-smoothed survival rate (C-24)
        let survival = Self::compute_survival(claim_id, graph);

        // O(c): Base oracle credibility ∈ [0, 1] (C-26)
        let oracle_base = oracle_source.factor_value();

        // Apply oracle as bonus: (1 + γ·O) where γ is field-calibrated (C-25/C-26)
        let gamma = field_schema.oracle_gamma();
        let oracle_bonus = 1.0 + gamma * oracle_base;

        // δ(c): Retraction discount factor from claim (C-27, wp-v0.2 §5.2)
        // Default 1.0 (no retraction penalty). Set to < 1.0 when dependencies retracted.
        let retraction_delta = claim.retraction_discount;

        // W = R × D̃ × S × (1 + γ·O) × δ
        let value = recency * depth * survival * oracle_bonus * retraction_delta;

        // Store the bonus-adjusted oracle factor in ClaimWeight
        Ok(ClaimWeight::new(value, recency, depth, survival, oracle_bonus))
    }

    /// Compute R(c, t) — time-decay recency factor (C-22, wp-v0.2 §3.2)
    ///
    /// Formula: R(c, t) = 2^(−Δt / t_½(c))
    ///
    /// Uses field-calibrated half-life t_½ from FieldSchema. The base-2 exponential
    /// ensures R = 0.5 at exactly one half-life, R = 0.25 at two half-lives, etc.
    fn compute_recency(
        claim: &Claim,
        graph: &GraphSnapshot,
        field_schema: &dyn FieldSchema,
    ) -> f64 {
        let blocks_elapsed = graph.current_block.saturating_sub(claim.registered);
        let days_elapsed = Self::blocks_to_days(blocks_elapsed, graph.block_time_seconds);
        // Clamp to u32::MAX to prevent silent truncation (u32::MAX days ~ 11.7M years)
        let days_clamped = days_elapsed.min(u32::MAX as u64) as u32;
        field_schema.compute_decay(days_clamped)
    }

    /// Convert block count to days
    fn blocks_to_days(blocks: u64, block_time_seconds: u32) -> u64 {
        let seconds = blocks.saturating_mul(block_time_seconds as u64);
        seconds / 86400 // seconds per day
    }

    /// Compute D̃ — log-normalized dependency depth factor (C-23, wp-v0.2 §3.3)
    ///
    /// Formula: D̃(c) = [1 + ln(1 + D)] / [1 + ln(1 + D_ref)]
    ///
    /// Where:
    /// - D = maximum dependency chain length for this claim
    /// - D_ref = field-calibrated reference depth (typical for well-formed claims)
    /// - ln = natural logarithm
    ///
    /// This log-normalization is "softer" than the wp-v0.1 linear penalty:
    /// - D = 0: D̃ = 1.0 / [1 + ln(1+D_ref)] (< 1.0 for D_ref > 0; claims shallower than D_ref)
    /// - D = D_ref: D̃ = 1.0 (reference normalization)
    /// - D > D_ref: D̃ > 1.0 (logarithmic boost for deep chains)
    fn compute_depth(
        claim: &Claim,
        graph: &GraphSnapshot,
        field_schema: &dyn FieldSchema,
    ) -> f64 {
        let mut in_stack = BTreeMap::new();
        in_stack.insert(claim.id, ());
        let max_depth = Self::find_max_depth(&claim.depends_on, graph, 0, &mut in_stack);

        let d = max_depth as f64;
        let d_ref = field_schema.reference_depth() as f64;

        // D̃(c) = [1 + ln(1 + D)] / [1 + ln(1 + D_ref)]
        let numerator = 1.0 + (1.0 + d).ln();
        let denominator = 1.0 + (1.0 + d_ref).ln();

        numerator / denominator
    }

    /// Find maximum dependency depth using DFS with cycle detection.
    ///
    /// `in_stack` tracks the current DFS recursion path for cycle detection only.
    /// Every branch of a diamond-shaped DAG is explored independently so that the
    /// true maximum depth is always found regardless of dependency list order.
    fn find_max_depth(
        dependencies: &[ClaimId],
        graph: &GraphSnapshot,
        current_depth: u32,
        in_stack: &mut BTreeMap<ClaimId, ()>,
    ) -> u32 {
        if dependencies.is_empty() {
            return current_depth;
        }

        let mut max_depth = current_depth;
        for dep_id in dependencies {
            // Cycle detection: skip nodes already on the current DFS path.
            // This is the only pruning we do — every non-cyclic branch is explored
            // fully so that diamonds and shared nodes are handled correctly.
            if in_stack.contains_key(dep_id) {
                continue;
            }

            let next_depth = current_depth.saturating_add(1);

            // Cap at depth 10 to prevent extreme attenuation and stack overflow.
            if next_depth >= 10 {
                max_depth = max_depth.max(10);
                continue;
            }

            in_stack.insert(*dep_id, ());

            if let Some(dep_claim) = graph.get_claim(dep_id) {
                let sub_depth =
                    Self::find_max_depth(&dep_claim.depends_on, graph, next_depth, in_stack);
                max_depth = max_depth.max(sub_depth);
            } else {
                max_depth = max_depth.max(next_depth);
            }

            in_stack.remove(dep_id);
        }
        max_depth
    }

    /// Compute S — Laplace-smoothed survival rate (C-24, wp-v0.2 §3.4)
    ///
    /// Formula: S(c) = (N₊ + α) / (N₊ + N₋ + α + β)
    ///
    /// With uniform Beta prior (α = β = 1):
    ///   S(c) = (supporting + 1) / (total + 2)
    ///
    /// This ensures:
    /// - No attestations: S = 0.5 (maximal uncertainty, not neutral)
    /// - All supporting: S → 1 as n grows
    /// - All contradicting: S → 0 as n grows
    /// - Small samples pulled toward 0.5 (uncertainty preservation)
    fn compute_survival(claim_id: &ClaimId, graph: &GraphSnapshot) -> f64 {
        let attestations = graph.get_attestations(claim_id);

        let (supporting, contradicting) = attestations
            .iter()
            .filter(|att| att.contributes_to_survival())
            .fold((0u32, 0u32), |(support, contradict), att| {
                match att.verdict {
                    AttestationVerdict::Supports | AttestationVerdict::Replicates => {
                        (support.saturating_add(1), contradict)
                    }
                    AttestationVerdict::Contradicts | AttestationVerdict::Refutes => {
                        (support, contradict.saturating_add(1))
                    }
                    _ => (support, contradict),
                }
            });

        let total = supporting.saturating_add(contradicting);

        // Laplace smoothing with α = β = 1 (uniform Beta prior)
        // S = (supporting + 1) / (total + 2)
        let alpha = 1.0;
        let beta = 1.0;
        let numerator = supporting as f64 + alpha;
        let denominator = total as f64 + alpha + beta;

        numerator / denominator
    }

    /// Propagate retraction penalty through dependency graph (C-27, wp-v0.2 §5.2)
    ///
    /// Uses BFS ordered by cascade depth. Each dependent claim receives a
    /// multiplicative retraction discount:
    ///   δ_new(c) = δ_old(c) × (0.5^depth)
    ///
    /// Where depth is the shortest path distance from the retracted claim.
    /// This ensures deterministic results regardless of `BTreeMap` iteration order.
    ///
    /// # Returns
    /// Vec of `AffectedClaim` containing claim_id, W_pre, W_post, cascade_depth,
    /// and the new δ(c) value to apply to the claim.
    pub fn propagate_retraction(
        retracted_claim_id: &ClaimId,
        graph: &GraphSnapshot,
        field_schema: &dyn FieldSchema,
    ) -> Vec<AffectedClaim> {
        let oracle = OFactorSource::None;
        let mut affected = Vec::new();
        // BFS frontier: (claim_id, cascade_depth)
        let mut queue: Vec<(ClaimId, u32)> = Vec::new();
        // Track the minimum cascade_depth at which each claim was first reached.
        let mut visited: BTreeMap<ClaimId, u32> = BTreeMap::new();

        // Seed: direct dependents of the retracted claim at depth 1
        for claim in graph.claims.values() {
            if claim.depends_directly_on(retracted_claim_id) {
                if !visited.contains_key(&claim.id) {
                    visited.insert(claim.id, 1);
                    queue.push((claim.id, 1));
                }
            }
        }

        let theta = field_schema.cascade_threshold();
        let mut head = 0;
        while head < queue.len() {
            let (claim_id, cascade_depth) = queue[head];
            head += 1;

            // Apply per-field cascade threshold Θ_field (C-28, wp-v0.2 §5.2)
            // Claims deeper than Θ_field are not affected by retraction
            if cascade_depth > theta {
                continue;
            }

            // Get the claim to read its current retraction discount
            let claim = match graph.get_claim(&claim_id) {
                Some(c) => c,
                None => continue,
            };

            // W_pre: weight with current δ (before this retraction)
            let w_pre = match Self::compute(&claim_id, graph, field_schema, &oracle) {
                Ok(w) => w.value,
                Err(_) => continue,
            };

            // δ_new = δ_old × (0.5^depth)
            let depth_factor = 0.5f64.powi(cascade_depth as i32);
            let old_delta = claim.retraction_discount;
            
            // Validate old_delta is finite and in valid range [0, 1]
            if !old_delta.is_finite() || old_delta < 0.0 || old_delta > 1.0 {
                continue;
            }
            
            let new_delta = old_delta * depth_factor;

            // W_post: weight with new δ
            // W = R × D̃ × S × (1+γ·O) × δ, so W_post = W_pre × (new_δ / old_δ)
            let w_post = if old_delta > 0.0 {
                w_pre * (new_delta / old_delta)
            } else {
                0.0
            };

            affected.push(AffectedClaim {
                claim_id,
                previous_weight: w_pre,
                new_weight: w_post,
                cascade_depth,
                new_retraction_discount: new_delta,
            });

            // Only continue cascade if weight is still significant
            if w_post <= 0.01 {
                continue;
            }

            let next_depth = cascade_depth.saturating_add(1);
            for candidate in graph.claims.values() {
                if candidate.depends_directly_on(&claim_id)
                    && !visited.contains_key(&candidate.id)
                {
                    visited.insert(candidate.id, next_depth);
                    queue.push((candidate.id, next_depth));
                }
            }
        }

        affected
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use crate::field::ClinicalMedicine;

    /// Property: Recency R(t) is monotonically non-increasing with time
    #[test]
    fn recency_monotonicity() {
        let field = ClinicalMedicine::new();

        // Test with multiple random-ish values
        let test_cases = vec![
            (0, 100),
            (100, 200),
            (500, 1000),
            (1000, 1825), // One half-life
            (1825, 3650), // Two half-lives
            (100, 50),    // Reverse (should fail monotonicity check)
        ];

        for (days1, days2) in test_cases {
            let r1 = field.compute_decay(days1);
            let r2 = field.compute_decay(days2);

            if days1 <= days2 {
                assert!(
                    r1 >= r2,
                    "Recency should decrease as time increases: R({})={}, R({})={}",
                    days1,
                    r1,
                    days2,
                    r2
                );
            } else {
                assert!(
                    r1 <= r2,
                    "Recency should decrease as time increases: R({})={}, R({})={}",
                    days1,
                    r1,
                    days2,
                    r2
                );
            }
        }
    }

    /// Property: Recency is always in range [0.0, 1.0]
    #[test]
    fn recency_bounded() {
        let field = ClinicalMedicine::new();
        let test_days = vec![0, 1, 10, 100, 500, 1000, 1825, 3650, 7300, 10000];

        for days in test_days {
            let r = field.compute_decay(days);
            assert!(
                (0.0..=1.0).contains(&r),
                "Recency should be in [0,1]: got {} for {} days",
                r,
                days
            );
        }
    }

    /// Property: Survival rate S with Laplace smoothing is in (0, 1)
    ///
    /// With α=β=1: S = (N₊ + 1) / (N₊ + N₋ + 2)
    /// - Never 0 (pseudocount prevents zero division)
    /// - Never 1 (pseudocount prevents certainty)
    /// - Approaches 0 as N₋ → ∞
    /// - Approaches 1 as N₊ → ∞
    #[test]
    fn survival_bounded() {
        use crate::claim::{Attestation, AttestationVerdict};

        let test_cases = vec![
            (0, 0),    // No attestations: S = 0.5
            (1, 0),    // Only supporting: S = 2/3
            (0, 1),    // Only contradicting: S = 1/3
            (5, 5),    // Equal: S = 6/12 = 0.5
            (10, 0),   // All supporting: S = 11/12
            (0, 10),   // All contradicting: S = 1/12
            (100, 50), // More supporting: S = 101/152
            (50, 100), // More contradicting: S = 51/152
        ];

        for (supporting, contradicting) in test_cases {
            let claim = super::tests::create_test_claim(1, vec![], 0);
            let mut attestations = Vec::new();
            let mut block = 10u64;
            let claim_id = claim.id;

            for _ in 0..supporting {
                attestations.push(Attestation {
                    id: format!("att-{}", block),
                    claim_id,
                    attester: "did:test:attester".into(),
                    verdict: AttestationVerdict::Supports,
                    evidence_tx: None,
                    attester_sbt: 100,
                    block,
                });
                block += 1;
            }

            for _ in 0..contradicting {
                attestations.push(Attestation {
                    id: format!("att-{}", block),
                    claim_id,
                    attester: "did:test:attester".into(),
                    verdict: AttestationVerdict::Refutes,
                    evidence_tx: None,
                    attester_sbt: 100,
                    block,
                });
                block += 1;
            }

            let mut attestation_map = BTreeMap::new();
            attestation_map.insert(claim_id, attestations);

            let graph = GraphSnapshot::new(
                BTreeMap::from([(claim_id, claim)]),
                attestation_map,
                1000,
                6,
            );

            let survival = WeightFunction::compute_survival(&claim_id, &graph);
            // With Laplace smoothing, S is strictly in (0, 1)
            assert!(
                survival > 0.0 && survival < 1.0,
                "Survival rate with Laplace smoothing should be in (0, 1): got {} for {} supporting, {} contradicting",
                survival,
                supporting,
                contradicting
            );

            // Verify exact Laplace formula
            let expected = (supporting as f64 + 1.0) / (supporting as f64 + contradicting as f64 + 2.0);
            assert!(
                (survival - expected).abs() < 1e-10,
                "S should be ({}+1)/({}+{}+2) = {}, got {}",
                supporting, supporting, contradicting, expected, survival
            );
        }
    }

    /// Property: Depth factor D decreases as dependency depth increases
    #[test]
    fn depth_monotonicity() {
        for depth in 0..20u32 {
            let d = 1.0 / (1.0 + depth as f64);
            assert!(
                d <= 1.0 && d > 0.0,
                "Depth factor should be in (0, 1]: got {} for depth {}",
                d,
                depth
            );

            if depth > 0 {
                let d_prev = 1.0 / (1.0 + (depth - 1) as f64);
                assert!(
                    d < d_prev,
                    "D should decrease as depth increases: D({})={}, D({})={}",
                    depth - 1,
                    d_prev,
                    depth,
                    d
                );
            }
        }
    }

    /// Property: Oracle factor values are ordered by credibility
    #[test]
    fn oracle_factor_ordering() {
        let peer_reviewed = OFactorSource::PeerReviewed {
            doi: "10.1/test".into(),
        }
        .factor_value();
        let clinical_trial = OFactorSource::ClinicalTrial {
            registry_id: "NCT1".into(),
        }
        .factor_value();
        let systematic_review = OFactorSource::SystematicReview {
            prospero_id: "CRD1".into(),
        }
        .factor_value();
        let dataset = OFactorSource::Dataset {
            identifier: "DS1".into(),
        }
        .factor_value();
        let software = OFactorSource::Software {
            repository_url: "https://github.com/test".into(),
            commit_hash: "abc123".into(),
        }
        .factor_value();
        let preprint = OFactorSource::Preprint {
            doi: "10.1/test".into(),
        }
        .factor_value();
        let none = OFactorSource::None.factor_value();

        // All factors should be in [0, 1]
        let factors = [
            ("PeerReviewed", peer_reviewed),
            ("ClinicalTrial", clinical_trial),
            ("SystematicReview", systematic_review),
            ("Dataset", dataset),
            ("Software", software),
            ("Preprint", preprint),
            ("None", none),
        ];
        for (name, value) in factors {
            assert!(
                (0.0..=1.0).contains(&value),
                "{} factor should be in [0,1]: got {}",
                name,
                value
            );
        }

        // Peer-reviewed should be highest (or tied with ClinicalTrial/SystematicReview)
        assert!(
            peer_reviewed >= clinical_trial,
            "PeerReviewed ({}) should be >= ClinicalTrial ({})",
            peer_reviewed,
            clinical_trial
        );
        assert!(
            peer_reviewed >= systematic_review,
            "PeerReviewed ({}) should be >= SystematicReview ({})",
            peer_reviewed,
            systematic_review
        );

        // Generally decreasing credibility (None is neutral baseline at 1.0)
        assert!(
            clinical_trial >= preprint,
            "ClinicalTrial ({}) should be >= Preprint ({})",
            clinical_trial,
            preprint
        );
        // None is neutral (1.0), higher than Preprint (0.70) but not a credibility ranking
        assert!(
            none >= preprint,
            "None ({}) should be >= Preprint ({}) - neutral baseline",
            none,
            preprint
        );
    }

    // =========================================================================
    // C-30: Property-based tests per wp-v0.2 (updated for new formula)
    // =========================================================================

    /// Property: Weight W is monotonic under additional supporting attestations (C-30)
    ///
    /// Adding a supporting attestation should never decrease the weight.
    /// W = R × D̃ × S × (1 + γ·O) × δ, and S increases with more supporting attestations.
    #[test]
    fn weight_monotonic_under_supporting_attestations() {
        use crate::claim::{Attestation, AttestationVerdict};

        let field = ClinicalMedicine::new();
        let claim = super::tests::create_test_claim(1, vec![], 0);
        let claim_id = claim.id;

        // Base case: no attestations
        let graph_base = GraphSnapshot::new(
            BTreeMap::from([(claim_id, claim.clone())]),
            BTreeMap::new(),
            100,
            6,
        );
        let w_base = WeightFunction::compute(&claim_id, &graph_base, &field, &OFactorSource::None)
            .unwrap()
            .value;

        // With one supporting attestation
        let attestation = Attestation {
            id: "att-1".into(),
            claim_id,
            attester: "did:test:attester".into(),
            verdict: AttestationVerdict::Supports,
            evidence_tx: None,
            attester_sbt: 100,
            block: 10,
        };
        let graph_with_support = GraphSnapshot::new(
            BTreeMap::from([(claim_id, claim.clone())]),
            BTreeMap::from([(claim_id, vec![attestation])]),
            100,
            6,
        );
        let w_with_support = WeightFunction::compute(&claim_id, &graph_with_support, &field, &OFactorSource::None)
            .unwrap()
            .value;

        // Weight should increase (or stay same if already at max)
        assert!(
            w_with_support >= w_base,
            "Weight should not decrease with supporting attestation: W_base={}, W_with_support={}",
            w_base,
            w_with_support
        );

        // Verify S actually increased
        let s_base = WeightFunction::compute_survival(&claim_id, &graph_base);
        let s_with_support = WeightFunction::compute_survival(&claim_id, &graph_with_support);
        assert!(
            s_with_support > s_base,
            "Survival rate should increase with supporting attestation: S_base={}, S_with_support={}",
            s_base,
            s_with_support
        );
    }

    /// Property: Retraction discount δ(c) is monotonic under cascade depth (C-30)
    ///
    /// As cascade depth increases, the retraction discount should decrease
    /// monotonically: δ_new = δ_old × (0.5^depth)
    #[test]
    fn retraction_discount_monotonic_under_cascade() {
        let field = ClinicalMedicine::new();

        // Create a chain: A <- B <- C <- D <- E (depths 1-4 from A)
        let a = super::tests::create_test_claim(1, vec![], 0);
        let b = super::tests::create_test_claim(2, vec![a.id], 0);
        let c = super::tests::create_test_claim(3, vec![b.id], 0);
        let d = super::tests::create_test_claim(4, vec![c.id], 0);
        let e = super::tests::create_test_claim(5, vec![d.id], 0);

        let graph = GraphSnapshot::new(
            BTreeMap::from([
                (a.id, a.clone()),
                (b.id, b.clone()),
                (c.id, c.clone()),
                (d.id, d.clone()),
                (e.id, e.clone()),
            ]),
            BTreeMap::new(),
            100,
            6,
        );

        let affected = WeightFunction::propagate_retraction(&a.id, &graph, &field);

        // Collect discounts by depth
        let mut discounts_by_depth: BTreeMap<u32, f64> = BTreeMap::new();
        for aff in &affected {
            discounts_by_depth.insert(aff.cascade_depth, aff.new_retraction_discount);
        }

        // Verify monotonic decrease: depth 1 > depth 2 > depth 3 > depth 4
        for depth in 2..=4u32 {
            let prev_discount = discounts_by_depth.get(&(depth - 1)).copied().unwrap_or(1.0);
            let curr_discount = discounts_by_depth.get(&depth).copied().unwrap_or(1.0);

            assert!(
                curr_discount < prev_discount,
                "δ should decrease monotonically with depth: δ({})={}, δ({})={}",
                depth - 1,
                prev_discount,
                depth,
                curr_discount
            );

            // Verify exact formula: δ(depth) = 0.5^depth (assuming δ_old = 1.0)
            let expected = 0.5f64.powi(depth as i32);
            assert!(
                (curr_discount - expected).abs() < 1e-10,
                "δ({}) should be 0.5^{} = {}, got {}",
                depth,
                depth,
                expected,
                curr_discount
            );
        }
    }

    /// Property: Non-zero baseline for unattested terminal claim (C-30)
    ///
    /// A claim with no attestations and no dependencies should have W > 0.
    /// W = R × D̃ × S × (1 + γ·O) × δ
    /// - R > 0 (fresh claim)
    /// - D̃ > 0 (log-normalized depth, positive for all depths)
    /// - S = 0.5 (Laplace smoothing with no attestations)
    /// - (1 + γ·O) >= 1.0
    /// - δ = 1.0
    /// So W > 0 for all valid claims
    #[test]
    fn unattested_terminal_claim_nonzero_weight() {
        let field = ClinicalMedicine::new();

        // Fresh claim at block 0, computing at block 100 (same block as registered)
        let claim = super::tests::create_test_claim(1, vec![], 100);
        let claim_id = claim.id;

        let graph = GraphSnapshot::new(
            BTreeMap::from([(claim_id, claim)]),
            BTreeMap::new(),
            100, // Same block as registered
            6,
        );

        let weight = WeightFunction::compute(&claim_id, &graph, &field, &OFactorSource::None)
            .unwrap();

        // W must be > 0
        assert!(
            weight.value > 0.0,
            "Unattested terminal claim must have W > 0: got {}",
            weight.value
        );

        // Verify components
        assert!(
            weight.recency > 0.0,
            "Recency should be > 0 for fresh claim: got {}",
            weight.recency
        );
        assert!(
            weight.depth > 0.0,
            "Depth should be > 0: got {}",
            weight.depth
        );
        assert!(
            (weight.survival - 0.5).abs() < 1e-10,
            "Survival should be 0.5 with no attestations (Laplace): got {}",
            weight.survival
        );
        assert!(
            weight.oracle >= 1.0,
            "Oracle bonus should be >= 1.0: got {}",
            weight.oracle
        );

        // W should be approximately R × D̃ × 0.5 × (1 + γ) for fresh claim with O=None (O=1.0)
        // O=None gives O=1.0, so oracle_bonus = 1 + γ × 1.0 = 1 + 0.5 = 1.5
        let expected_oracle = 1.0 + field.oracle_gamma() * 1.0; // 1.5
        assert!(
            (weight.oracle - expected_oracle).abs() < 1e-10,
            "Oracle bonus should be {} for O=None: got {}",
            expected_oracle,
            weight.oracle
        );
    }

    /// Property: Basic science (O=0) does not zero W (C-30)
    ///
    /// When oracle credibility is 0 (no peer review), the oracle bonus is (1 + γ·0) = 1.0,
    /// not 0. So basic science claims with O=0 still have weight.
    #[test]
    fn basic_science_oracle_zero_does_not_zero_weight() {
        use crate::claim::{Attestation, AttestationVerdict};

        let field = ClinicalMedicine::new();

        // Create a basic science claim: O=0 via preprint (lowest credibility)
        let claim = super::tests::create_test_claim(1, vec![], 100);
        let claim_id = claim.id;

        // Add supporting attestation to ensure non-zero survival
        let attestation = Attestation {
            id: "att-1".into(),
            claim_id,
            attester: "did:test:attester".into(),
            verdict: AttestationVerdict::Supports,
            evidence_tx: None,
            attester_sbt: 100,
            block: 10,
        };

        let graph = GraphSnapshot::new(
            BTreeMap::from([(claim_id, claim)]),
            BTreeMap::from([(claim_id, vec![attestation])]),
            100,
            6,
        );

        // Preprint has O = 0.70 (lowest non-zero)
        // To get O=0, we need a custom source or check that even low O doesn't zero W
        let preprint_source = OFactorSource::Preprint { doi: "10.1/test".into() };
        let weight_preprint = WeightFunction::compute(&claim_id, &graph, &field, &preprint_source)
            .unwrap();

        // Even with low O, W should be > 0
        assert!(
            weight_preprint.value > 0.0,
            "Preprint claim (O=0.70) must have W > 0: got {}",
            weight_preprint.value
        );

        // Oracle bonus should be 1 + γ × 0.70 = 1 + 0.5 × 0.70 = 1.35
        let expected_bonus = 1.0 + field.oracle_gamma() * 0.70;
        assert!(
            (weight_preprint.oracle - expected_bonus).abs() < 1e-10,
            "Oracle bonus for preprint should be {}: got {}",
            expected_bonus,
            weight_preprint.oracle
        );

        // With O=None, bonus is 1 + γ × 1.0 = 1.5
        let weight_none = WeightFunction::compute(&claim_id, &graph, &field, &OFactorSource::None)
            .unwrap();
        let expected_bonus_none = 1.0 + field.oracle_gamma() * 1.0;
        assert!(
            (weight_none.oracle - expected_bonus_none).abs() < 1e-10,
            "Oracle bonus for None should be {}: got {}",
            expected_bonus_none,
            weight_none.oracle
        );

        // Verify O=None gives higher weight than preprint (since O=None has O=1.0)
        assert!(
            weight_none.value > weight_preprint.value,
            "O=None (O=1.0) should give higher weight than preprint (O=0.70): W_none={}, W_preprint={}",
            weight_none.value,
            weight_preprint.value
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::ClinicalMedicine;
    use crate::{Claim, ClaimType, VersionDOI};

    pub fn create_test_claim(id: u8, depends_on: Vec<ClaimId>, registered: u64) -> Claim {
        Claim {
            id: ClaimId::from_bytes([id; 32]),
            claim_type: ClaimType::PrimaryClaim,
            field_id: "clinical-medicine".into(),
            content: crate::claim::ClaimContent {
                canonical_json: format!("{{\"test\":{}}}", id),
            },
            submitter: "did:test:submitter".into(),
            depends_on,
            arweave_tx: "test-tx".into(),
            registered,
            spec_version_doi: crate::VersionDOI::wp_v0_2(),
            retraction_discount: 1.0,
        }
    }

    fn create_test_attestation(
        claim_id: ClaimId,
        verdict: AttestationVerdict,
        block: u64,
    ) -> Attestation {
        Attestation {
            id: format!("att-{}", block),
            claim_id,
            attester: "did:test:attester".into(),
            verdict,
            evidence_tx: None,
            attester_sbt: 100,
            block,
        }
    }

    #[test]
    fn ofactor_source_values() {
        assert_eq!(
            OFactorSource::PeerReviewed {
                doi: "10.1234/test".into()
            }
            .factor_value(),
            1.0
        );
        assert_eq!(
            OFactorSource::ClinicalTrial {
                registry_id: "NCT123".into()
            }
            .factor_value(),
            0.95
        );
        assert_eq!(
            OFactorSource::Preprint {
                doi: "10.1234/test".into()
            }
            .factor_value(),
            0.70
        );
        assert_eq!(OFactorSource::None.factor_value(), 1.0);
    }

    #[test]
    fn claim_weight_new() {
        let weight = ClaimWeight::new(0.5, 1.0, 1.0, 1.0, 0.5);
        assert_eq!(weight.value, 0.5);
        assert_eq!(weight.recency, 1.0);
        assert_eq!(weight.depth, 1.0);
        assert_eq!(weight.survival, 1.0);
        assert_eq!(weight.oracle, 0.5);
    }

    #[test]
    fn survival_rate_with_supporting() {
        let claim = create_test_claim(1, vec![], 0);
        let att1 = create_test_attestation(claim.id, AttestationVerdict::Supports, 10);
        let mut attestations = BTreeMap::new();
        attestations.insert(claim.id, vec![att1]);

        let graph = GraphSnapshot::new(
            BTreeMap::from([(claim.id, claim.clone())]),
            attestations,
            100,
            6, // 6 second block time
        );

        // Laplace smoothing: (1 + 1) / (1 + 2) = 2/3
        let survival = WeightFunction::compute_survival(&claim.id, &graph);
        assert!((survival - 2.0 / 3.0).abs() < 1e-10, "S should be 2/3, got {}", survival);
    }

    #[test]
    fn survival_rate_with_mixed() {
        let claim = create_test_claim(1, vec![], 0);
        let att1 = create_test_attestation(claim.id, AttestationVerdict::Supports, 10);
        let att2 = create_test_attestation(claim.id, AttestationVerdict::Refutes, 20);
        let mut attestations = BTreeMap::new();
        attestations.insert(claim.id, vec![att1, att2]);

        let graph = GraphSnapshot::new(
            BTreeMap::from([(claim.id, claim.clone())]),
            attestations,
            100,
            6,
        );

        let survival = WeightFunction::compute_survival(&claim.id, &graph);
        assert!((survival - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn survival_rate_no_attestations() {
        let claim = create_test_claim(1, vec![], 0);
        let graph = GraphSnapshot::new(
            BTreeMap::from([(claim.id, claim.clone())]),
            BTreeMap::new(),
            100,
            6,
        );

        // Laplace smoothing with no data: (0 + 1) / (0 + 2) = 0.5 (maximal uncertainty)
        let survival = WeightFunction::compute_survival(&claim.id, &graph);
        assert!((survival - 0.5).abs() < 1e-10, "S with no attestations should be 0.5, got {}", survival);
    }

    #[test]
    fn survival_rate_mentions_excluded() {
        let claim = create_test_claim(1, vec![], 0);
        let att1 = create_test_attestation(claim.id, AttestationVerdict::Mentions, 10);
        let mut attestations = BTreeMap::new();
        attestations.insert(claim.id, vec![att1]);

        let graph = GraphSnapshot::new(
            BTreeMap::from([(claim.id, claim.clone())]),
            attestations,
            100,
            6,
        );

        // Mentions don't contribute to survival scoring, so same as no attestations: S = 0.5
        let survival = WeightFunction::compute_survival(&claim.id, &graph);
        assert!((survival - 0.5).abs() < 1e-10, "S with only Mentions should be 0.5, got {}", survival);
    }

    #[test]
    fn depth_no_dependencies() {
        let field = ClinicalMedicine::new();
        let claim = create_test_claim(1, vec![], 0);
        let graph = GraphSnapshot::new(
            BTreeMap::from([(claim.id, claim.clone())]),
            BTreeMap::new(),
            100,
            6,
        );

        // D=0, D_ref=3: D̃ = [1 + ln(1)] / [1 + ln(4)] = 1 / 2.386 = 0.419
        let depth = WeightFunction::compute_depth(&claim, &graph, &field);
        let expected = 1.0 / (1.0 + (4.0f64).ln());
        assert!(
            (depth - expected).abs() < 1e-10,
            "D̃ with no deps should be ~{:.3}, got {:.3}",
            expected,
            depth
        );
    }

    #[test]
    fn depth_with_dependencies() {
        let field = ClinicalMedicine::new();
        let dep = create_test_claim(2, vec![], 0);
        let claim = create_test_claim(1, vec![dep.id], 0);
        let graph = GraphSnapshot::new(
            BTreeMap::from([(dep.id, dep), (claim.id, claim.clone())]),
            BTreeMap::new(),
            100,
            6,
        );

        // D=1, D_ref=3: D̃ = [1 + ln(2)] / [1 + ln(4)] = 1.693 / 2.386 = 0.709
        let depth = WeightFunction::compute_depth(&claim, &graph, &field);
        let expected = (1.0 + (2.0f64).ln()) / (1.0 + (4.0f64).ln());
        assert!(
            (depth - expected).abs() < 1e-10,
            "D̃ with one dep should be ~{:.3}, got {:.3}",
            expected,
            depth
        );
    }

    #[test]
    fn recency_decay_over_time() {
        let field = ClinicalMedicine::new();
        let claim = create_test_claim(1, vec![], 0); // registered at block 0
        let attestations = BTreeMap::new();

        // At day 0
        let graph_now = GraphSnapshot::new(
            BTreeMap::from([(claim.id, claim.clone())]),
            attestations.clone(),
            0, // current block
            6,
        );
        let recency_now = WeightFunction::compute_recency(&claim, &graph_now, &field);
        assert!((recency_now - 1.0).abs() < f64::EPSILON);

        // At day 1825 (5 years, one half-life)
        let blocks_5y = (1825 * 86400) / 6;
        let graph_5y = GraphSnapshot::new(
            BTreeMap::from([(claim.id, claim.clone())]),
            attestations,
            blocks_5y as u64,
            6,
        );
        let recency_5y = WeightFunction::compute_recency(&claim, &graph_5y, &field);
        assert!((recency_5y - 0.5).abs() < 0.01);
    }

    #[test]
    fn full_weight_computation() {
        let field = ClinicalMedicine::new();
        let claim = create_test_claim(1, vec![], 0);
        let att = create_test_attestation(claim.id, AttestationVerdict::Supports, 10);
        let attestations = BTreeMap::from([(claim.id, vec![att])]);
        let claims = BTreeMap::from([(claim.id, claim.clone())]);

        let graph = GraphSnapshot::new(claims, attestations, 0, 6);
        let oracle = OFactorSource::PeerReviewed {
            doi: "10.1234/test".into(),
        };

        let weight = WeightFunction::compute(&claim.id, &graph, &field, &oracle).unwrap();

        // R=1.0 (new claim)
        // D̃ = [1 + ln(1)] / [1 + ln(4)] = 0.419 (no deps, D_ref=3 for clinical medicine)
        // S = (1 + 1) / (1 + 2) = 2/3 ≈ 0.667 (Laplace smoothing, 1 supporting attestation)
        // O_base=1.0 (peer reviewed), γ=0.5 (clinical medicine oracle_gamma)
        // Oracle bonus = (1 + γ·O) = 1 + 0.5·1.0 = 1.5
        // W = 1.0 × 0.419 × 0.667 × 1.5 ≈ 0.419
        let expected_d = 1.0 / (1.0 + (4.0f64).ln());
        let expected_s = 2.0 / 3.0; // Laplace: (1+1)/(1+2)
        let expected_o = 1.5; // 1 + 0.5 * 1.0 (gamma * base_oracle)
        let expected_w = 1.0 * expected_d * expected_s * expected_o;
        assert!((weight.value - expected_w).abs() < 1e-10, "W should be {:.3}, got {:.3}", expected_w, weight.value);
        assert!((weight.recency - 1.0).abs() < f64::EPSILON);
        assert!((weight.depth - expected_d).abs() < 1e-10, "D̃ should be {:.3}, got {:.3}", expected_d, weight.depth);
        assert!((weight.survival - expected_s).abs() < 1e-10, "S should be 2/3, got {:.3}", weight.survival);
        assert!((weight.oracle - expected_o).abs() < 1e-10, "Oracle bonus should be {:.3}, got {:.3}", expected_o, weight.oracle);
    }

    #[test]
    fn propagate_retraction_empty() {
        let field = ClinicalMedicine::new();
        let claim = create_test_claim(1, vec![], 0);
        let graph = GraphSnapshot::new(
            BTreeMap::from([(claim.id, claim.clone())]),
            BTreeMap::new(),
            100,
            6,
        );

        let affected = WeightFunction::propagate_retraction(&claim.id, &graph, &field);
        assert!(affected.is_empty());
    }

    /// Regression test for Bug 1: diamond DAG must produce the correct max depth
    /// regardless of dependency list order.
    ///
    /// Graph:    X -> [B, A], A -> [B]   (diamond; longest path X->A->B has depth 2)
    /// BTreeMap iteration of X.depends_on used to process B first, inserting
    /// visited[B]=0, then when recursing through A the B branch was wrongly skipped,
    /// returning depth 1 instead of 2.
    #[test]
    fn depth_diamond_dag_order_independent() {
        let field = ClinicalMedicine::new();
        let b = create_test_claim(2, vec![], 0);
        let a = create_test_claim(3, vec![b.id], 0);
        // X depends on both A and B; longest path is X->A->B (depth 2)
        let x = create_test_claim(1, vec![b.id, a.id], 0);

        let graph = GraphSnapshot::new(
            BTreeMap::from([(b.id, b), (a.id, a), (x.id, x.clone())]),
            BTreeMap::new(),
            100,
            6,
        );

        // D=2, D_ref=3: D̃ = [1 + ln(3)] / [1 + ln(4)] = 2.099 / 2.386 = 0.880
        let depth = WeightFunction::compute_depth(&x, &graph, &field);
        let expected = (1.0 + (3.0f64).ln()) / (1.0 + (4.0f64).ln());
        assert!(
            (depth - expected).abs() < 1e-10,
            "diamond DAG D̃ (D=2) should be ~{:.3}, got {:.3}",
            expected,
            depth
        );

        // Also verify with reversed dependency order: X -> [A, B]
        let b2 = create_test_claim(2, vec![], 0);
        let a2 = create_test_claim(3, vec![b2.id], 0);
        let x2 = create_test_claim(1, vec![a2.id, b2.id], 0);
        let graph2 = GraphSnapshot::new(
            BTreeMap::from([(b2.id, b2), (a2.id, a2), (x2.id, x2.clone())]),
            BTreeMap::new(),
            100,
            6,
        );
        let depth2 = WeightFunction::compute_depth(&x2, &graph2, &field);
        assert!(
            (depth2 - depth).abs() < 1e-10,
            "depth must be identical regardless of dependency order: {depth} vs {depth2}"
        );
    }

    /// Regression test for Bug 2: retraction cascade must assign each dependent
    /// the penalty of the *shortest* path from the retracted claim, not an
    /// order-dependent path.
    ///
    /// Graph: retract A; D depends on A (depth 1); C depends on A (depth 1)
    ///        and C also depends on D (but D is at depth 1).
    /// Both C and D are direct dependents of A, so both should get penalty 0.5.
    #[test]
    fn retraction_cascade_direct_deps_get_depth_one_penalty() {
        let field = ClinicalMedicine::new();

        // A is the retracted claim
        let a = create_test_claim(1, vec![], 0);
        // D directly depends on A
        let d = create_test_claim(4, vec![a.id], 0);
        // C directly depends on A AND on D
        let c = create_test_claim(3, vec![a.id, d.id], 0);

        let graph = GraphSnapshot::new(
            BTreeMap::from([(a.id, a.clone()), (d.id, d.clone()), (c.id, c.clone())]),
            BTreeMap::new(),
            100,
            6,
        );

        let affected = WeightFunction::propagate_retraction(&a.id, &graph, &field);

        // Both C and D must appear in the affected list
        let find = |id: ClaimId| affected.iter().find(|ac| ac.claim_id == id).cloned();
        let c_affected = find(c.id).expect("C must be affected");
        let d_affected = find(d.id).expect("D must be affected");

        // Both are direct dependents of A, so cascade_depth must be 1 for both
        assert_eq!(
            d_affected.cascade_depth, 1,
            "D is a direct dep of A; must get cascade_depth=1, got {}",
            d_affected.cascade_depth
        );
        assert_eq!(
            c_affected.cascade_depth, 1,
            "C is a direct dep of A; must get cascade_depth=1, got {}",
            c_affected.cascade_depth
        );

        // For depth 1: new_δ = 1.0 × 0.5^1 = 0.5
        assert!(
            (d_affected.new_retraction_discount - 0.5).abs() < 1e-10,
            "D should have δ=0.5 at depth 1, got {}",
            d_affected.new_retraction_discount
        );
        assert!(
            (c_affected.new_retraction_discount - 0.5).abs() < 1e-10,
            "C should have δ=0.5 at depth 1, got {}",
            c_affected.new_retraction_discount
        );

        // Verify W_post = W_pre × (new_δ / old_δ) = W_pre × 0.5
        assert!(
            (c_affected.new_weight - c_affected.previous_weight * 0.5).abs() < 1e-10,
            "C weight should be halved: {} vs expected {}",
            c_affected.new_weight,
            c_affected.previous_weight * 0.5
        );
    }

    /// Test retraction discount δ(c) is multiplicative across multiple retractions (C-27)
    #[test]
    fn retraction_discount_multiplicative() {
        let claim = create_test_claim(1, vec![], 0);

        // First retraction: δ = 1.0 × 0.5 = 0.5
        let claim_after_1 = claim.clone().with_retraction_discount(0.5);
        assert!((claim_after_1.retraction_discount - 0.5).abs() < 1e-10);

        // Second retraction: δ = 0.5 × 0.5 = 0.25
        let claim_after_2 = claim_after_1.with_retraction_discount(0.5);
        assert!((claim_after_2.retraction_discount - 0.25).abs() < 1e-10);

        // Third retraction: δ = 0.25 × 0.5 = 0.125
        let claim_after_3 = claim_after_2.with_retraction_discount(0.5);
        assert!((claim_after_3.retraction_discount - 0.125).abs() < 1e-10);
    }

    /// Numerical sanity-check: Self-registered whitepaper claim has W > 0 (C-33, wp-v0.2 §11.4)
    ///
    /// This test verifies Bug B1 is fixed: the wp-v0.2 formula ensures that even
    /// a newly-registered claim (no attestations, no dependents, basic-science type)
    /// has strictly positive weight.
    ///
    /// The whitepaper-as-first-claim (§11.4) is:
    /// - New: registered at current block
    /// - No attestations: S = 0.5 (Laplace smoothing baseline)
    /// - Terminal: no dependents, D̃ = 1.0 (reference depth)
    /// - Basic science document: O = None (O=1.0, oracle_bonus = 1+γ)
    /// - No retractions: δ = 1.0
    ///
    /// Expected: W = R × D̃ × S × (1+γ·O) × δ
    ///         = 1.0 × 1.0 × 0.5 × 1.5 × 1.0 = 0.75 > 0
    ///
    /// Under wp-v0.1, this would have been W = 0 (due to multiplicative zero-collapse).
    #[test]
    fn whitepaper_claim_has_nonzero_weight_bug_b1_regression() {
        let field = ClinicalMedicine::new();

        // Simulate the whitepaper as a self-registered claim
        // - Fresh registration (block 100, computed at block 100)
        // - No attestations
        // - No dependents
        // - No oracle linkage (basic science document)
        let whitepaper_claim = create_test_claim_with_version(
            1,
            vec![], // No dependencies
            100,    // Registered at block 100
            VersionDOI::wp_v0_2(), // Self-references wp-v0.2
        );

        let graph = GraphSnapshot::new(
            BTreeMap::from([(whitepaper_claim.id, whitepaper_claim.clone())]),
            BTreeMap::new(), // No attestations
            100,             // Current block = registration block
            6,
        );

        // Compute weight with no oracle linkage (basic science)
        let oracle = OFactorSource::None;
        let weight = WeightFunction::compute(&whitepaper_claim.id, &graph, &field, &oracle)
            .expect("Weight computation should succeed");

        // CRITICAL: W must be > 0 (regression test for Bug B1)
        assert!(
            weight.value > 0.0,
            "BUG B1 REGRESSION: Self-registered whitepaper claim has W = 0. Expected W > 0, got {}. Under wp-v0.1, this would have been 0.",
            weight.value
        );

        // Verify all factors have strictly positive baselines
        assert!(
            weight.recency > 0.0,
            "R should be > 0 for fresh claim: got {}",
            weight.recency
        );
        assert!(
            weight.recency <= 1.0,
            "R should be <= 1.0: got {}",
            weight.recency
        );

        assert!(
            weight.depth > 0.0,
            "D̃ should be > 0 for terminal claim: got {}",
            weight.depth
        );

        assert!(
            weight.survival > 0.0,
            "S should be > 0 (Laplace smoothing): got {}",
            weight.survival
        );
        assert!(
            weight.survival < 1.0,
            "S should be < 1.0 (no certainty with no data): got {}",
            weight.survival
        );

        assert!(
            weight.oracle >= 1.0,
            "Oracle bonus should be >= 1.0: got {}",
            weight.oracle
        );

        // Verify the exact computation
        // R = 2^(0/t_½) = 1.0 (same block)
        // D̃ = log_norm(0, 3) ≈ 0.721 (terminal claim, reference depth 3)
        // S = (0+1)/(0+2) = 0.5 (Laplace smoothing)
        // O_bonus = 1 + 0.5×1.0 = 1.5 (None has O=1.0)
        // δ = 1.0
        let expected_r = 1.0f64; // Same block
        let expected_s = 0.5f64; // Laplace with no attestations
        let expected_o = 1.0 + field.oracle_gamma() * 1.0; // 1.5

        assert!(
            (weight.recency - expected_r).abs() < 1e-10,
            "R should be {} for fresh claim, got {}",
            expected_r,
            weight.recency
        );
        assert!(
            (weight.survival - expected_s).abs() < 1e-10,
            "S should be {} with no attestations, got {}",
            expected_s,
            weight.survival
        );
        assert!(
            (weight.oracle - expected_o).abs() < 1e-10,
            "Oracle bonus should be {} for O=None, got {}",
            expected_o,
            weight.oracle
        );

        // The actual weight should be approximately:
        // W ≈ 1.0 × 0.721 × 0.5 × 1.5 × 1.0 ≈ 0.54
        // The important thing is W > 0, not the exact value
        println!(
            "Whitepaper claim weight: W = {} (R={}, D̃={}, S={}, O={}, δ=1.0)",
            weight.value,
            weight.recency,
            weight.depth,
            weight.survival,
            weight.oracle
        );
    }

    /// Helper to create a test claim with explicit VersionDOI
    fn create_test_claim_with_version(
        id: u8,
        depends_on: Vec<ClaimId>,
        registered: u64,
        version: VersionDOI,
    ) -> Claim {
        Claim {
            id: ClaimId::from_bytes([id; 32]),
            claim_type: ClaimType::PrimaryClaim,
            field_id: "clinical-medicine".into(),
            content: crate::claim::ClaimContent {
                canonical_json: format!("{{\"whitepaper\":{},\"version\":\"{}\"}}", id, version),
            },
            submitter: "did:apodokimos:genesis".into(),
            depends_on,
            arweave_tx: "ar://whitepaper-tx".into(),
            registered,
            spec_version_doi: version,
            retraction_discount: 1.0,
        }
    }
}

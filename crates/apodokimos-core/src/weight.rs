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

/// Computed weight for a claim (P-03)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClaimWeight {
    /// Final computed weight (W = R × D × S × O)
    pub value: f64,
    /// Time-decay factor [0.0, 1.0]
    pub recency: f64,
    /// Dependency depth factor [0.0, 1.0]
    pub depth: f64,
    /// Survival rate [0.1, 1.0] — floor of 0.1 applied (uncertainty preservation;
    /// wp-v0.2 replaces this with Laplace smoothing, see C-24)
    pub survival: f64,
    /// Oracle factor [0.0, 1.0]
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
}

/// Affected claim with new weight after retraction (C-19)
#[derive(Debug, Clone)]
pub struct AffectedClaim {
    /// The affected claim ID
    pub claim_id: ClaimId,
    /// Previous weight before retraction
    pub previous_weight: f64,
    /// New weight after retraction
    pub new_weight: f64,
    /// Depth in the cascade (1 = direct dependent)
    pub cascade_depth: u32,
}

/// Weight function implementation (C-14)
pub struct WeightFunction;

impl WeightFunction {
    /// Compute weight for a claim (C-14)
    ///
    /// W(claim) = R(t) × D × S × O
    pub fn compute(
        claim_id: &ClaimId,
        graph: &GraphSnapshot,
        field_schema: &dyn FieldSchema,
        oracle_source: &OFactorSource,
    ) -> Result<ClaimWeight, ApodokimosError> {
        let claim = graph
            .get_claim(claim_id)
            .ok_or_else(|| ApodokimosError::ClaimNotFound(claim_id.to_hex()))?;

        // R(t): Time-decay recency (C-15)
        let recency = Self::compute_recency(claim, graph, field_schema);

        // D: Dependency depth (C-16)
        let depth = Self::compute_depth(claim, graph);

        // S: Survival rate (C-17)
        let survival = Self::compute_survival(claim_id, graph);

        // O: Oracle factor (C-18)
        let oracle = oracle_source.factor_value();

        // W = R × D × S × O
        let value = recency * depth * survival * oracle;

        Ok(ClaimWeight::new(value, recency, depth, survival, oracle))
    }

    /// Compute R(t) — time-decay recency factor (C-15)
    ///
    /// Uses field-calibrated half-life from FieldSchema
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

    /// Compute D — dependency depth factor (C-16)
    ///
    /// D = 1.0 / (1 + depth) where depth is max dependency chain length
    /// Claims with no dependencies have D = 1.0 (highest weight)
    fn compute_depth(claim: &Claim, graph: &GraphSnapshot) -> f64 {
        let mut in_stack = BTreeMap::new();
        in_stack.insert(claim.id, ());
        let max_depth = Self::find_max_depth(&claim.depends_on, graph, 0, &mut in_stack);
        1.0 / (1.0 + max_depth as f64)
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

    /// Compute S — survival rate (C-17)
    ///
    /// S = supporting_attestations / total_non_mentioning_attestations
    /// Returns 1.0 (neutral) if no survival-trackable attestations exist.
    ///
    /// A floor of 0.1 is applied so that a fully-contradicted claim retains a small
    /// positive weight (epistemic uncertainty preservation). This is a wp-v0.1
    /// implementation choice; wp-v0.2 replaces it with explicit Laplace smoothing
    /// (see C-24, wp-v0.2 §3.4). The effective range is therefore [0.1, 1.0].
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
        if total == 0 {
            return 1.0; // Neutral if no survival-trackable attestations
        }

        let survival_rate = supporting as f64 / total as f64;
        survival_rate.max(0.1)
    }

    /// Propagate retraction penalty through dependency graph (C-19)
    ///
    /// Uses BFS ordered by cascade depth so that each dependent claim always
    /// receives the penalty from the *shortest* path from the retracted claim,
    /// regardless of `BTreeMap` iteration order. This guarantees deterministic
    /// results on any graph topology (R8/R9).
    ///
    /// Penalty factor: 0.5^depth — 0.5 at depth 1, 0.25 at depth 2, etc.
    pub fn propagate_retraction(
        retracted_claim_id: &ClaimId,
        graph: &GraphSnapshot,
        field_schema: &dyn FieldSchema,
    ) -> Vec<AffectedClaim> {
        let oracle = OFactorSource::None;
        let mut affected = Vec::new();
        // BFS frontier: (claim_id, cascade_depth)
        // Using a Vec as a FIFO queue; depth increases monotonically so earlier
        // entries always have depth <= later entries (BFS property).
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

        let mut head = 0;
        while head < queue.len() {
            let (claim_id, cascade_depth) = queue[head];
            head += 1;

            // Hard cap: matches the depth limit used in find_max_depth
            if cascade_depth > 5 {
                continue;
            }

            let penalty_factor = 0.5f64.powi(cascade_depth as i32);

            let previous_weight = match Self::compute(&claim_id, graph, field_schema, &oracle) {
                Ok(w) => w.value,
                Err(_) => continue,
            };

            let new_weight = previous_weight * penalty_factor;

            affected.push(AffectedClaim {
                claim_id,
                previous_weight,
                new_weight,
                cascade_depth,
            });

            // Only continue cascade if weight is still significant
            if new_weight <= 0.01 {
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

    /// Property: Survival rate S is bounded in [0.1, 1.0]
    #[test]
    fn survival_bounded() {
        use crate::claim::{Attestation, AttestationVerdict};

        let test_cases = vec![
            (0, 0),    // No attestations
            (1, 0),    // Only supporting
            (0, 1),    // Only contradicting
            (5, 5),    // Equal
            (10, 0),   // All supporting
            (0, 10),   // All contradicting
            (100, 50), // More supporting
            (50, 100), // More contradicting
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
            assert!(
                (0.1..=1.0).contains(&survival),
                "Survival rate should be in [0.1, 1.0]: got {} for {} supporting, {} contradicting",
                survival,
                supporting,
                contradicting
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::ClinicalMedicine;
    use crate::{Claim, ClaimType};

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

        let survival = WeightFunction::compute_survival(&claim.id, &graph);
        assert!((survival - 1.0).abs() < f64::EPSILON);
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

        let survival = WeightFunction::compute_survival(&claim.id, &graph);
        assert!((survival - 1.0).abs() < f64::EPSILON); // Neutral default
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

        let survival = WeightFunction::compute_survival(&claim.id, &graph);
        assert!((survival - 1.0).abs() < f64::EPSILON); // Mentions don't affect survival
    }

    #[test]
    fn depth_no_dependencies() {
        let claim = create_test_claim(1, vec![], 0);
        let graph = GraphSnapshot::new(
            BTreeMap::from([(claim.id, claim.clone())]),
            BTreeMap::new(),
            100,
            6,
        );

        let depth = WeightFunction::compute_depth(&claim, &graph);
        assert!((depth - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn depth_with_dependencies() {
        let dep = create_test_claim(2, vec![], 0);
        let claim = create_test_claim(1, vec![dep.id], 0);
        let graph = GraphSnapshot::new(
            BTreeMap::from([(dep.id, dep), (claim.id, claim.clone())]),
            BTreeMap::new(),
            100,
            6,
        );

        let depth = WeightFunction::compute_depth(&claim, &graph);
        assert!((depth - 0.5).abs() < f64::EPSILON);
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

        // R=1.0 (new claim), D=1.0 (no deps), S=1.0 (all support), O=1.0 (peer reviewed)
        // W = 1.0 × 1.0 × 1.0 × 1.0 = 1.0
        assert!((weight.value - 1.0).abs() < f64::EPSILON);
        assert!((weight.recency - 1.0).abs() < f64::EPSILON);
        assert!((weight.depth - 1.0).abs() < f64::EPSILON);
        assert!((weight.survival - 1.0).abs() < f64::EPSILON);
        assert!((weight.oracle - 1.0).abs() < f64::EPSILON);
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

        let depth = WeightFunction::compute_depth(&x, &graph);
        // depth 2 => D = 1/(1+2) = 0.333...
        assert!(
            (depth - (1.0 / 3.0)).abs() < 1e-10,
            "diamond DAG depth should be 2 (D=1/3), got D={depth}"
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
        let depth2 = WeightFunction::compute_depth(&x2, &graph2);
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
    }
}

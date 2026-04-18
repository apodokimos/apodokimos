//! Weight function computation for epistemic claim scoring (P-06)
//!
//! W(claim, t) = R(t) × D × S × O
//!
//! Where:
//! - R(t): Time-decayed replication score
//! - D: Dependency depth (transitive dependent count)
//! - S: Survival rate from attestations
//! - O: Real-world outcome linkage factor

use crate::claim::{DID, TxId};
use crate::{field::FieldSchema, ApodokimosError, Attestation, AttestationVerdict, Claim, ClaimId};
use alloc::collections::BTreeMap;
use alloc::collections::BTreeSet;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

/// Small constant to prevent division by zero
const EPSILON: f64 = 1e-10;

/// Threshold for retraction cascade propagation (P-12)
pub const RETRACTION_THRESHOLD: f64 = 0.1;

/// Computed weight for a claim
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ClaimWeight {
    /// Raw weight value (unnormalized)
    pub raw: f64,
    /// Normalized score per field calibration
    pub normalized: f64,
    /// Individual factor values for transparency
    pub factors: WeightFactors,
}

/// Individual weight factors (for transparency/debugging)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WeightFactors {
    /// R(t): Time-decayed replication score
    pub replication: f64,
    /// D: Dependency depth factor
    pub dependency: f64,
    /// S: Survival rate from attestations
    pub survival: f64,
    /// O: Outcome linkage factor
    pub outcome: f64,
}

impl WeightFactors {
    /// Create new factors with all values
    ///
    /// # Example
    /// ```
    /// use apodokimos_core::WeightFactors;
    ///
    /// let factors = WeightFactors::new(0.8, 1.5, 0.9, 0.7);
    /// assert_eq!(factors.product(), 0.8 * 1.5 * 0.9 * 0.7);
    /// ```
    pub const fn new(replication: f64, dependency: f64, survival: f64, outcome: f64) -> Self {
        Self {
            replication,
            dependency,
            survival,
            outcome,
        }
    }

    /// Compute product of all factors
    ///
    /// # Example
    /// ```
    /// use apodokimos_core::WeightFactors;
    ///
    /// let factors = WeightFactors::new(0.5, 2.0, 0.8, 0.9);
    /// let product = factors.product();
    /// assert!((product - 0.72).abs() < 0.001);
    /// ```
    pub fn product(&self) -> f64 {
        self.replication * self.dependency * self.survival * self.outcome
    }
}

impl Default for WeightFactors {
    fn default() -> Self {
        Self {
            replication: 1.0,
            dependency: 1.0,
            survival: 1.0,
            outcome: 1.0,
        }
    }
}

/// Source of O-factor linkage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OFactorSource {
    /// Regulatory approval (FDA, EMA, etc.)
    RegulatoryApproval,
    /// Patent granted based on claim
    PatentGranted,
    /// Clinical outcome data
    ClinicalOutcome,
    /// Product in market using claim
    ProductInMarket,
    /// Policy document citing claim
    PolicyDocument,
    /// Media coverage of real-world impact
    MediaCoverage,
    /// Open source implementation
    OpenSourceImplementation,
}

impl OFactorSource {
    /// Base score contribution for this source type
    ///
    /// # Example
    /// ```
    /// use apodokimos_core::OFactorSource;
    ///
    /// assert_eq!(OFactorSource::RegulatoryApproval.base_score(), 1.0);
    /// assert_eq!(OFactorSource::MediaCoverage.base_score(), 0.3);
    /// ```
    pub const fn base_score(&self) -> f64 {
        match self {
            // Highest confidence: regulatory and market proof
            Self::RegulatoryApproval => 1.0,
            Self::ProductInMarket => 0.95,
            Self::ClinicalOutcome => 0.9,
            // Medium confidence: legal and policy
            Self::PatentGranted => 0.7,
            Self::PolicyDocument => 0.6,
            // Lower confidence but still valid
            Self::OpenSourceImplementation => 0.5,
            Self::MediaCoverage => 0.3,
        }
    }

    /// All possible source variants
    pub const ALL: &[Self] = &[
        Self::RegulatoryApproval,
        Self::PatentGranted,
        Self::ClinicalOutcome,
        Self::ProductInMarket,
        Self::PolicyDocument,
        Self::MediaCoverage,
        Self::OpenSourceImplementation,
    ];
}

/// O-factor linkage with validation metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OFactorLinkage {
    /// Source type
    pub source: OFactorSource,
    /// Arweave tx ID of proof documentation
    pub evidence_tx: TxId,
    /// Block number when recorded
    pub recorded_block: u64,
    /// Oracle validator DID (if verified)
    pub oracle_did: Option<DID>,
    /// Computed score (base × verification_multiplier)
    ///
    /// This field is read-only after construction to maintain score integrity.
    #[serde(skip)] // Prevent deserialization from overriding computed score
    score: f64,
}

impl OFactorLinkage {
    /// Get the computed score for this linkage
    ///
    /// # Example
    /// ```
    /// use apodokimos_core::{OFactorLinkage, OFactorSource};
    ///
    /// let linkage = OFactorLinkage::new(
    ///     OFactorSource::ClinicalOutcome,
    ///     "evidence_tx",
    ///     1000,
    ///     Some("did:oracle:test"),
    ///     true,
    /// );
    /// assert_eq!(linkage.score(), 0.9); // 0.9 * 1.0 (verified)
    /// ```
    pub fn score(&self) -> f64 {
        self.score
    }
}

impl OFactorLinkage {
    /// Create a new O-factor linkage
    pub fn new(
        source: OFactorSource,
        evidence_tx: impl Into<TxId>,
        recorded_block: u64,
        oracle_did: Option<impl Into<DID>>,
        verified: bool,
    ) -> Self {
        let base = source.base_score();
        let multiplier = if verified { 1.0 } else { 0.5 };
        Self {
            source,
            evidence_tx: evidence_tx.into(),
            recorded_block,
            oracle_did: oracle_did.map(|d| d.into()),
            score: base * multiplier,
        }
    }
}

/// Affected claim record for retraction cascade
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectedClaim {
    /// Claim ID affected
    pub claim_id: ClaimId,
    /// Original weight before propagation
    pub original_weight: f64,
    /// New weight after penalty
    pub new_weight: f64,
    /// Whether this triggered further cascade (weight below threshold)
    pub triggers_cascade: bool,
    /// Depth in cascade (0 = direct dependent)
    pub cascade_depth: u32,
}

/// Snapshot of the claim graph for weight computation
#[derive(Debug, Clone, Default)]
pub struct GraphSnapshot<'a> {
    /// Claims indexed by ID
    pub claims: BTreeMap<ClaimId, &'a Claim>,
    /// Attestations indexed by claim ID
    pub attestations: BTreeMap<ClaimId, Vec<&'a Attestation>>,
    /// O-factor linkages indexed by claim ID
    pub o_factor_linkages: BTreeMap<ClaimId, Vec<OFactorLinkage>>,
    /// Reverse dependency mapping (claim -> direct dependents)
    pub dependents: BTreeMap<ClaimId, Vec<ClaimId>>,
    /// Current block number for time calculations
    pub current_block: u64,
    /// Average block time in seconds (for converting blocks to days)
    pub block_time_seconds: u64,
}

impl<'a> GraphSnapshot<'a> {
    /// Create a new empty snapshot
    ///
    /// # Panics
    /// Panics if `block_time_seconds` is 0, as this would cause incorrect time calculations.
    pub fn new(current_block: u64, block_time_seconds: u64) -> Self {
        assert!(
            block_time_seconds > 0,
            "block_time_seconds must be greater than 0"
        );
        Self {
            claims: BTreeMap::new(),
            attestations: BTreeMap::new(),
            o_factor_linkages: BTreeMap::new(),
            dependents: BTreeMap::new(),
            current_block,
            block_time_seconds,
        }
    }

    /// Add a claim to the snapshot
    pub fn add_claim(&mut self, claim: &'a Claim) {
        // Build reverse dependency index
        for dep in &claim.depends_on {
            self.dependents.entry(*dep).or_default().push(claim.id);
        }
        self.claims.insert(claim.id, claim);
    }

    /// Add attestations for a claim
    ///
    /// Extends any existing attestations for this claim.
    pub fn add_attestations(&mut self, claim_id: ClaimId, attestations: Vec<&'a Attestation>) {
        self.attestations
            .entry(claim_id)
            .or_default()
            .extend(attestations);
    }

    /// Add O-factor linkages for a claim
    ///
    /// Extends any existing linkages for this claim.
    pub fn add_o_factor_linkages(&mut self, claim_id: ClaimId, linkages: Vec<OFactorLinkage>) {
        self.o_factor_linkages
            .entry(claim_id)
            .or_default()
            .extend(linkages);
    }

    /// Convert blocks elapsed to days
    pub fn blocks_to_days(&self, blocks_elapsed: u64) -> u32 {
        let seconds = blocks_elapsed.saturating_mul(self.block_time_seconds);
        // Convert to days (86400 seconds/day), saturating at u32::MAX
        let days = seconds / 86_400;
        days.min(u32::MAX as u64) as u32
    }

    /// Get direct dependents of a claim
    pub fn direct_dependents(&self, claim_id: &ClaimId) -> &[ClaimId] {
        self.dependents.get(claim_id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Get claim by ID
    pub fn get_claim(&self, claim_id: &ClaimId) -> Option<&&Claim> {
        self.claims.get(claim_id)
    }

    /// Get attestations for a claim
    pub fn get_attestations(&self, claim_id: &ClaimId) -> &[&Attestation] {
        self.attestations.get(claim_id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Get O-factor linkages for a claim
    pub fn get_o_factor_linkages(&self, claim_id: &ClaimId) -> &[OFactorLinkage] {
        self.o_factor_linkages.get(claim_id).map(|v| v.as_slice()).unwrap_or(&[])
    }
}

/// Weight function computation engine
#[derive(Debug)]
pub struct WeightFunction<'a, F: FieldSchema> {
    field_schema: &'a F,
}

impl<'a, F: FieldSchema> WeightFunction<'a, F> {
    /// Create a new weight function with field calibration
    pub const fn new(field_schema: &'a F) -> Self {
        Self { field_schema }
    }

    /// Compute full weight for a claim (C-14)
    ///
    /// W(claim) = R(t) × D × S × O
    pub fn compute(&self, claim_id: ClaimId, graph: &GraphSnapshot) -> Result<ClaimWeight, ApodokimosError> {
        // Verify claim exists
        let _ = graph
            .get_claim(&claim_id)
            .ok_or_else(|| ApodokimosError::ClaimNotFound(claim_id.to_hex()))?;

        // Compute individual factors
        let replication = self.compute_replication_factor(claim_id, graph)?;
        let dependency = self.compute_dependency_factor(claim_id, graph)?;
        let survival = self.compute_survival_factor(claim_id, graph)?;
        let outcome = self.compute_outcome_factor(claim_id, graph)?;

        let factors = WeightFactors::new(replication, dependency, survival, outcome);
        let raw = factors.product();
        let normalized = self.field_schema.normalize_score(raw);

        Ok(ClaimWeight {
            raw,
            normalized,
            factors,
        })
    }

    /// Compute R(t) — Time-decayed replication score (C-15)
    ///
    /// R(c, t) = Σ w_decay(replicates) / (|Replicates| + |Refutes| + ε)
    fn compute_replication_factor(
        &self,
        claim_id: ClaimId,
        graph: &GraphSnapshot,
    ) -> Result<f64, ApodokimosError> {
        let claim = graph
            .get_claim(&claim_id)
            .ok_or_else(|| ApodokimosError::ClaimNotFound(claim_id.to_hex()))?;

        // Only empirical claim types use R(t) - return early for non-empirical
        if !claim.claim_type.is_empirical() {
            return Ok(1.0);
        }

        let attestations = graph.get_attestations(&claim_id);
        let mut weighted_replicates = 0.0;
        let mut replicates_count = 0u32;
        let mut refutes_count = 0u32;

        for att in attestations {
            match att.verdict {
                AttestationVerdict::Replicates => {
                    replicates_count += 1;
                    // Apply time decay based on attestation age
                    let blocks_elapsed = graph.current_block.saturating_sub(att.block);
                    let days_elapsed = graph.blocks_to_days(blocks_elapsed);
                    let decay = self.field_schema.compute_decay(days_elapsed);
                    weighted_replicates += decay;
                }
                AttestationVerdict::Refutes => {
                    refutes_count += 1;
                }
                _ => {}
            }
        }

        let denominator = (replicates_count + refutes_count) as f64 + EPSILON;
        let r = weighted_replicates / denominator;

        // Clamp to [0, 1]
        Ok(r.clamp(0.0, 1.0))
    }

    /// Compute D — Dependency depth factor (C-16)
    ///
    /// D(c) = 1 + log(1 + |transitive_dependents|)
    ///
    /// Uses logarithmic scaling to prevent runaway weights from popular claims
    fn compute_dependency_factor(
        &self,
        claim_id: ClaimId,
        graph: &GraphSnapshot,
    ) -> Result<f64, ApodokimosError> {
        let transitive_count = self.count_transitive_dependents(claim_id, graph);
        // Logarithmic scaling: D = 1 + ln(1 + count)
        // This ensures D ≥ 1 and grows slowly with high dependency counts
        let d = 1.0 + (1.0 + transitive_count as f64).ln();
        Ok(d)
    }

    /// Count transitive dependents via BFS traversal
    fn count_transitive_dependents(&self, claim_id: ClaimId, graph: &GraphSnapshot) -> usize {
        let mut visited = BTreeSet::new();
        let mut queue: Vec<ClaimId> = graph.direct_dependents(&claim_id).to_vec();
        let mut count = 0usize;

        while let Some(current) = queue.pop() {
            if visited.insert(current) {
                count += 1;
                // Add direct dependents of current to queue
                for dep in graph.direct_dependents(&current) {
                    if !visited.contains(dep) {
                        queue.push(*dep);
                    }
                }
            }
        }

        count
    }

    /// Compute S — Survival rate from attestations (C-17)
    ///
    /// S = (|Supports| + |Replicates|) / (|Supports| + |Contradicts| + |Replicates| + |Refutes| + ε)
    fn compute_survival_factor(
        &self,
        claim_id: ClaimId,
        graph: &GraphSnapshot,
    ) -> Result<f64, ApodokimosError> {
        let attestations = graph.get_attestations(&claim_id);

        let mut supports = 0u32;
        let mut contradicts = 0u32;
        let mut replicates = 0u32;
        let mut refutes = 0u32;

        for att in attestations {
            if !att.contributes_to_survival() {
                continue; // Skip Mentions
            }
            match att.verdict {
                AttestationVerdict::Supports => supports += 1,
                AttestationVerdict::Contradicts => contradicts += 1,
                AttestationVerdict::Replicates => replicates += 1,
                AttestationVerdict::Refutes => refutes += 1,
                AttestationVerdict::Mentions => {} // Already filtered
            }
        }

        let numerator = (supports + replicates) as f64;
        let denominator =
            (supports + contradicts + replicates + refutes) as f64 + EPSILON;

        let s = numerator / denominator;
        Ok(s.clamp(0.0, 1.0))
    }

    /// Compute O — Outcome linkage factor (C-18)
    ///
    /// O = max(linkage.score) across all O-factor sources, or 0 if none
    fn compute_outcome_factor(
        &self,
        claim_id: ClaimId,
        graph: &GraphSnapshot,
    ) -> Result<f64, ApodokimosError> {
        let linkages = graph.get_o_factor_linkages(&claim_id);

        if linkages.is_empty() {
            return Ok(0.0);
        }

        // O = max score across all linkages
        let max_score = linkages
            .iter()
            .map(|l| l.score())
            .fold(0.0, f64::max);

        Ok(max_score.clamp(0.0, 1.0))
    }

    /// Propagate retraction penalty through dependency graph (C-19)
    ///
    /// Returns all affected claims with their weight changes.
    /// Uses BFS to ensure correct cascade depth calculation.
    pub fn propagate_retraction(
        &self,
        retracted_claim_id: ClaimId,
        graph: &GraphSnapshot,
    ) -> Result<Vec<AffectedClaim>, ApodokimosError> {
        let mut affected = Vec::new();
        let mut visited = BTreeSet::new();
        // Use VecDeque for BFS to ensure correct cascade depth
        let mut queue: VecDeque<(ClaimId, u32)> = graph
            .direct_dependents(&retracted_claim_id)
            .iter()
            .map(|&id| (id, 0u32)) // (claim_id, cascade_depth)
            .collect();

        // Mark retracted claim as visited to prevent cycles
        visited.insert(retracted_claim_id);

        while let Some((claim_id, depth)) = queue.pop_front() {
            if !visited.insert(claim_id) {
                continue; // Already processed
            }

            // Compute weight before and after
            let original_weight = self.compute(claim_id, graph)?;

            // Apply penalty: 50% reduction per cascade level
            let penalty_multiplier = 0.5f64.powi(depth as i32 + 1);
            let new_raw = original_weight.raw * penalty_multiplier;

            let triggers_cascade = new_raw < RETRACTION_THRESHOLD;

            affected.push(AffectedClaim {
                claim_id,
                original_weight: original_weight.raw,
                new_weight: new_raw,
                triggers_cascade,
                cascade_depth: depth,
            });

            // If weight falls below threshold, propagate to dependents
            if triggers_cascade {
                for &dependent in graph.direct_dependents(&claim_id) {
                    if !visited.contains(&dependent) {
                        queue.push_back((dependent, depth + 1));
                    }
                }
            }
        }

        Ok(affected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::ClinicalMedicine;
    use crate::{AttestationVerdict, Claim, ClaimType};

    fn create_test_claim(id: u8, field_id: &str, depends_on: Vec<ClaimId>) -> Claim {
        Claim::new(
            ClaimType::PrimaryClaim,
            field_id,
            format!("{{\"test\":{}}}", id),
            "did:test:123",
            depends_on,
            &format!("tx_{}", id),
            id as u64 * 100,
        )
    }

    fn create_test_attestation(
        claim_id: ClaimId,
        verdict: AttestationVerdict,
        block: u64,
    ) -> Attestation {
        Attestation::new(
            format!("att-{}-{:?}", claim_id.to_hex(), verdict),
            claim_id,
            "did:test:attester",
            verdict,
            None::<&str>,
            1000,
            block,
        )
    }

    #[test]
    fn weight_factors_product() {
        let f = WeightFactors::new(0.5, 2.0, 0.8, 0.9);
        assert!((f.product() - 0.72).abs() < 0.001);
    }

    #[test]
    fn o_factor_source_scores() {
        assert_eq!(OFactorSource::RegulatoryApproval.base_score(), 1.0);
        assert_eq!(OFactorSource::MediaCoverage.base_score(), 0.3);
        assert_eq!(OFactorSource::OpenSourceImplementation.base_score(), 0.5);
    }

    #[test]
    fn o_factor_linkage_creation() {
        let linkage = OFactorLinkage::new(
            OFactorSource::ClinicalOutcome,
            "evidence_tx_123",
            1000,
            Some("did:oracle:test"),
            true,
        );
        assert_eq!(linkage.source, OFactorSource::ClinicalOutcome);
        assert_eq!(linkage.score(), 0.9); // verified = full score

        let unverified = OFactorLinkage::new(
            OFactorSource::RegulatoryApproval,
            "evidence_tx_456",
            1000,
            None::<&str>,
            false,
        );
        assert_eq!(unverified.score(), 0.5); // unverified = half score
    }

    #[test]
    fn graph_snapshot_building() {
        let mut graph = GraphSnapshot::new(10000, 12); // 12s block time

        let claim1 = create_test_claim(1, "test-field", vec![]);
        let claim2 = create_test_claim(2, "test-field", vec![claim1.id]);

        graph.add_claim(&claim1);
        graph.add_claim(&claim2);

        assert_eq!(graph.direct_dependents(&claim1.id).len(), 1);
        assert!(graph.direct_dependents(&claim2.id).is_empty());
    }

    #[test]
    fn blocks_to_days_conversion() {
        let graph = GraphSnapshot::new(10000, 12); // 12s block time

        // 7200 blocks × 12s = 86400s = 1 day
        assert_eq!(graph.blocks_to_days(7200), 1);

        // 864000 blocks × 12s = 10368000s = 120 days
        assert_eq!(graph.blocks_to_days(864000), 120);
    }

    #[test]
    fn survival_factor_computation() {
        let field = ClinicalMedicine::new();
        let wf = WeightFunction::new(&field);

        let claim = create_test_claim(1, "clinical-medicine", vec![]);
        let mut graph = GraphSnapshot::new(1000, 12);
        graph.add_claim(&claim);

        // No attestations = 0 / ε = 0 (clamped)
        let s = wf.compute_survival_factor(claim.id, &graph).unwrap();
        assert_eq!(s, 0.0);

        // Add supporting attestations
        let attestations = vec![
            create_test_attestation(claim.id, AttestationVerdict::Supports, 100),
            create_test_attestation(claim.id, AttestationVerdict::Supports, 200),
            create_test_attestation(claim.id, AttestationVerdict::Replicates, 300),
        ];
        graph.add_attestations(claim.id, attestations.iter().collect());

        let s2 = wf.compute_survival_factor(claim.id, &graph).unwrap();
        assert!((s2 - 1.0).abs() < 0.001); // All supporting = ~1.0

        // Test contradicting attestations with fresh graph
        let claim2 = create_test_claim(2, "clinical-medicine", vec![]);
        let mut graph2 = GraphSnapshot::new(1000, 12);
        graph2.add_claim(&claim2);

        let attestations2 = vec![
            create_test_attestation(claim2.id, AttestationVerdict::Supports, 100),
            create_test_attestation(claim2.id, AttestationVerdict::Contradicts, 200),
        ];
        graph2.add_attestations(claim2.id, attestations2.iter().collect());

        let s3 = wf.compute_survival_factor(claim2.id, &graph2).unwrap();
        assert!((s3 - 0.5).abs() < 0.01); // 1 support / (1 support + 1 contradict) = 0.5
    }

    #[test]
    fn survival_factor_ignores_mentions() {
        let field = ClinicalMedicine::new();
        let wf = WeightFunction::new(&field);

        let claim = create_test_claim(1, "clinical-medicine", vec![]);
        let mut graph = GraphSnapshot::new(1000, 12);
        graph.add_claim(&claim);

        // Only Mentions attestations
        let attestations = vec![
            create_test_attestation(claim.id, AttestationVerdict::Mentions, 100),
            create_test_attestation(claim.id, AttestationVerdict::Mentions, 200),
        ];
        graph.add_attestations(claim.id, attestations.iter().collect());

        let s = wf.compute_survival_factor(claim.id, &graph).unwrap();
        assert!(s < 0.01); // Should be ~0 since Mentions don't count
    }

    #[test]
    fn dependency_factor_computation() {
        let field = ClinicalMedicine::new();
        let wf = WeightFunction::new(&field);

        let claim1 = create_test_claim(1, "clinical-medicine", vec![]);
        let claim2 = create_test_claim(2, "clinical-medicine", vec![claim1.id]);
        let claim3 = create_test_claim(3, "clinical-medicine", vec![claim1.id]);
        let claim4 = create_test_claim(4, "clinical-medicine", vec![claim2.id]);

        let mut graph = GraphSnapshot::new(1000, 12);
        graph.add_claim(&claim1);
        graph.add_claim(&claim2);
        graph.add_claim(&claim3);
        graph.add_claim(&claim4);

        // claim1 has 3 transitive dependents: claim2, claim3, claim4
        let d1 = wf.compute_dependency_factor(claim1.id, &graph).unwrap();
        assert!(d1 > 1.0);

        // claim4 has 0 dependents
        let d4 = wf.compute_dependency_factor(claim4.id, &graph).unwrap();
        assert!((d4 - 1.0).abs() < 0.001); // D = 1 when no dependents
    }

    #[test]
    fn outcome_factor_computation() {
        let field = ClinicalMedicine::new();
        let wf = WeightFunction::new(&field);

        let claim = create_test_claim(1, "clinical-medicine", vec![]);
        let mut graph = GraphSnapshot::new(1000, 12);
        graph.add_claim(&claim);

        // No linkages = 0
        let o = wf.compute_outcome_factor(claim.id, &graph).unwrap();
        assert_eq!(o, 0.0);

        // Add linkages
        let linkages = vec![
            OFactorLinkage::new(
                OFactorSource::ClinicalOutcome,
                "evidence1",
                500,
                Some("did:oracle:1"),
                true,
            ),
            OFactorLinkage::new(
                OFactorSource::MediaCoverage,
                "evidence2",
                600,
                None::<&str>,
                false,
            ),
        ];
        graph.add_o_factor_linkages(claim.id, linkages);

        let o2 = wf.compute_outcome_factor(claim.id, &graph).unwrap();
        assert_eq!(o2, 0.9); // Max of 0.9 and 0.15
    }

    #[test]
    fn full_weight_computation() {
        let field = ClinicalMedicine::new();
        let wf = WeightFunction::new(&field);

        let claim = create_test_claim(1, "clinical-medicine", vec![]);
        let mut graph = GraphSnapshot::new(5000, 12);
        graph.add_claim(&claim);

        // Add attestations for survival factor
        let attestations = vec![
            create_test_attestation(claim.id, AttestationVerdict::Supports, 100),
            create_test_attestation(claim.id, AttestationVerdict::Replicates, 200),
        ];
        graph.add_attestations(claim.id, attestations.iter().collect());

        // Add O-factor
        let linkages = vec![OFactorLinkage::new(
            OFactorSource::ClinicalOutcome,
            "evidence1",
            500,
            Some("did:oracle:1"),
            true,
        )];
        graph.add_o_factor_linkages(claim.id, linkages);

        let weight = wf.compute(claim.id, &graph).unwrap();

        // Verify factors are in expected ranges
        assert!(weight.factors.replication >= 0.0 && weight.factors.replication <= 1.0);
        assert!(weight.factors.dependency >= 1.0); // D >= 1
        assert!(weight.factors.survival > 0.0 && weight.factors.survival <= 1.0);
        assert!(weight.factors.outcome > 0.0 && weight.factors.outcome <= 1.0);

        // Raw weight should be product of factors
        let expected_raw = weight.factors.product();
        assert!((weight.raw - expected_raw).abs() < 0.001);
    }

    #[test]
    fn retraction_propagation() {
        let field = ClinicalMedicine::new();
        let wf = WeightFunction::new(&field);

        // Build chain: claim1 <- claim2 <- claim3
        let claim1 = create_test_claim(1, "clinical-medicine", vec![]);
        let claim2 = create_test_claim(2, "clinical-medicine", vec![claim1.id]);
        let claim3 = create_test_claim(3, "clinical-medicine", vec![claim2.id]);

        let mut graph = GraphSnapshot::new(1000, 12);
        graph.add_claim(&claim1);
        graph.add_claim(&claim2);
        graph.add_claim(&claim3);

        // Add attestations and O-factor to give claim2 a weight above threshold
        let attestations = vec![
            create_test_attestation(claim2.id, AttestationVerdict::Supports, 100),
            create_test_attestation(claim2.id, AttestationVerdict::Replicates, 200),
        ];
        graph.add_attestations(claim2.id, attestations.iter().collect());

        // Add O-factor linkage so weight is non-zero
        let linkages = vec![OFactorLinkage::new(
            OFactorSource::ClinicalOutcome,
            "evidence_tx",
            500,
            Some("did:oracle:test"),
            true,
        )];
        graph.add_o_factor_linkages(claim2.id, linkages);

        // Retract claim1
        let affected = wf.propagate_retraction(claim1.id, &graph).unwrap();

        // Should affect claim2 and claim3
        assert!(!affected.is_empty());
        assert!(affected.iter().any(|a| a.claim_id == claim2.id));

        // claim2 should have reduced weight
        let claim2_affected = affected.iter().find(|a| a.claim_id == claim2.id).unwrap();
        assert!(claim2_affected.new_weight < claim2_affected.original_weight);
    }

    #[test]
    fn non_empirical_claim_replication_factor() {
        let field = ClinicalMedicine::new();
        let wf = WeightFunction::new(&field);

        // Hypothesis (non-empirical) should return R = 1.0
        let hypothesis = Claim::new(
            ClaimType::Hypothesis,
            "clinical-medicine",
            "{\"hypothesis\":true}",
            "did:test:123",
            vec![],
            "tx_hyp",
            100,
        );

        let mut graph = GraphSnapshot::new(1000, 12);
        graph.add_claim(&hypothesis);

        let r = wf.compute_replication_factor(hypothesis.id, &graph).unwrap();
        assert_eq!(r, 1.0);
    }

    #[test]
    fn replication_factor_time_decay() {
        let field = ClinicalMedicine::new();
        let wf = WeightFunction::new(&field);

        let claim = create_test_claim(1, "clinical-medicine", vec![]);
        // Use 50 million blocks (~19 years at 12s/block) to ensure significant decay
        let mut graph = GraphSnapshot::new(50_000_000, 12);
        graph.add_claim(&claim);

        // Old replications should have decay applied
        let attestations = vec![
            create_test_attestation(claim.id, AttestationVerdict::Replicates, 100), // Very old
        ];
        graph.add_attestations(claim.id, attestations.iter().collect());

        let r = wf.compute_replication_factor(claim.id, &graph).unwrap();
        // After ~19 years with 5-year half-life, should have decayed significantly
        assert!(r < 0.5, "Expected r < 0.5, got {}", r);
    }

    // C-20: Property-based tests for weight function monotonicity
    // Note: These tests only run on native targets, not wasm32

    #[cfg(not(target_arch = "wasm32"))]
    mod proptest_tests {
        use super::*;
        use crate::{AttestationVerdict, Claim, ClaimType, ClinicalMedicine, OFactorLinkage, OFactorSource};
        use proptest::prelude::*;

        fn create_test_claim(id: u8, field_id: &str, depends_on: Vec<ClaimId>) -> Claim {
            Claim::new(
                ClaimType::PrimaryClaim,
                field_id,
                format!("{{\"test\":{}}}", id),
                "did:test:123",
                depends_on,
                &format!("tx_{}", id),
                id as u64 * 100,
            )
        }

        fn create_test_attestation(
            claim_id: ClaimId,
            verdict: AttestationVerdict,
            block: u64,
        ) -> Attestation {
            Attestation::new(
                format!("att-{}-{:?}", claim_id.to_hex(), verdict),
                claim_id,
                "did:test:attester",
                verdict,
                None::<&str>,
                1000,
                block,
            )
        }

        proptest! {
            // Survival factor monotonicity: more supporting attestations should not decrease S
            fn survival_factor_monotonic_with_support(
                initial_supports in 0u32..10,
                additional_supports in 0u32..10,
                contradicts in 0u32..5,
            ) {
                let field = ClinicalMedicine::new();
                let wf = WeightFunction::new(&field);

                let claim = create_test_claim(1, "clinical-medicine", vec![]);
                let mut graph = GraphSnapshot::new(1000, 12);
                graph.add_claim(&claim);

                // Build initial attestations
                let mut attestations: Vec<Attestation> = (0..initial_supports)
                    .map(|i| create_test_attestation(claim.id, AttestationVerdict::Supports, i as u64 * 10))
                    .collect();
                attestations.extend((0..contradicts).map(|i| {
                    create_test_attestation(claim.id, AttestationVerdict::Contradicts, 100 + i as u64 * 10)
                }));

                graph.add_attestations(claim.id, attestations.iter().collect());
                let s1 = wf.compute_survival_factor(claim.id, &graph).unwrap();

                // Add more supporting attestations
                let more_attestations: Vec<Attestation> = (0..additional_supports)
                    .map(|i| create_test_attestation(claim.id, AttestationVerdict::Supports, 200 + i as u64 * 10))
                    .collect();
                let all_attestations: Vec<&Attestation> = attestations.iter()
                    .chain(more_attestations.iter())
                    .collect();
                graph.add_attestations(claim.id, all_attestations);
                let s2 = wf.compute_survival_factor(claim.id, &graph).unwrap();

                // Adding supports should not decrease survival factor
                prop_assert!(s2 >= s1 - f64::EPSILON, "Adding supports decreased S: {} -> {}", s1, s2);
            }

            // Survival factor range: S must always be in [0, 1]
            fn survival_factor_always_in_range(
                supports in 0u32..100,
                contradicts in 0u32..100,
                replicates in 0u32..100,
                refutes in 0u32..100,
            ) {
                let field = ClinicalMedicine::new();
                let wf = WeightFunction::new(&field);

                let claim = create_test_claim(1, "clinical-medicine", vec![]);
                let mut graph = GraphSnapshot::new(1000, 12);
                graph.add_claim(&claim);

                let mut attestations: Vec<Attestation> = Vec::new();
                attestations.extend((0..supports).map(|i| {
                    create_test_attestation(claim.id, AttestationVerdict::Supports, i as u64 * 10)
                }));
                attestations.extend((0..contradicts).map(|i| {
                    create_test_attestation(claim.id, AttestationVerdict::Contradicts, 1000 + i as u64 * 10)
                }));
                attestations.extend((0..replicates).map(|i| {
                    create_test_attestation(claim.id, AttestationVerdict::Replicates, 2000 + i as u64 * 10)
                }));
                attestations.extend((0..refutes).map(|i| {
                    create_test_attestation(claim.id, AttestationVerdict::Refutes, 3000 + i as u64 * 10)
                }));

                graph.add_attestations(claim.id, attestations.iter().collect());
                let s = wf.compute_survival_factor(claim.id, &graph).unwrap();

                prop_assert!(s >= 0.0 && s <= 1.0, "Survival factor out of range: {}", s);
            }

            // Time decay monotonicity: older attestations have less weight
            fn decay_factor_decreases_with_time(days in 0u32..10_000) {
                let field = ClinicalMedicine::new();

                let decay = field.compute_decay(days);

                // Decay should be in [0, 1]
                prop_assert!(decay >= 0.0 && decay <= 1.0, "Decay out of range: {}", decay);

                // At t=0, decay should be 1.0
                if days == 0 {
                    prop_assert!((decay - 1.0).abs() < 0.001, "Decay at t=0 should be 1.0, got {}", decay);
                }

                // At half-life, decay should be ~0.5
                if days == field.decay_half_life() {
                    prop_assert!((decay - 0.5).abs() < 0.01, "Decay at half-life should be ~0.5, got {}", decay);
                }
            }

            // O-factor range: must always be in [0, 1]
            fn outcome_factor_always_in_range(linkage_count in 0usize..10) {
                let field = ClinicalMedicine::new();
                let wf = WeightFunction::new(&field);

                let claim = create_test_claim(1, "clinical-medicine", vec![]);
                let mut graph = GraphSnapshot::new(1000, 12);
                graph.add_claim(&claim);

                let linkages: Vec<OFactorLinkage> = (0..linkage_count)
                    .map(|i| OFactorLinkage::new(
                        OFactorSource::ALL[i % OFactorSource::ALL.len()],
                        format!("evidence_{}", i),
                        i as u64 * 100,
                        Some("did:oracle:test"),
                        i % 2 == 0, // alternate verified/unverified
                    ))
                    .collect();

                graph.add_o_factor_linkages(claim.id, linkages);
                let o = wf.compute_outcome_factor(claim.id, &graph).unwrap();

                prop_assert!(o >= 0.0 && o <= 1.0, "O-factor out of range: {}", o);
            }

            // Dependency factor: D >= 1 always (log scaling ensures this)
            fn dependency_factor_at_least_one(_dummy in 0u8..1) {
                let field = ClinicalMedicine::new();
                let wf = WeightFunction::new(&field);

                let claim = create_test_claim(1, "clinical-medicine", vec![]);
                let mut graph = GraphSnapshot::new(1000, 12);
                graph.add_claim(&claim);

                let d = wf.compute_dependency_factor(claim.id, &graph).unwrap();

                prop_assert!(d >= 1.0, "Dependency factor should be >= 1, got {}", d);
            }

            // Weight factor product: individual factors multiply correctly
            fn weight_factors_product_consistency(
                r in 0.0f64..1.0,
                d in 1.0f64..100.0,
                s in 0.0f64..1.0,
                o in 0.0f64..1.0,
            ) {
                let factors = WeightFactors::new(r, d, s, o);
                let product = factors.product();
                let expected = r * d * s * o;

                prop_assert!((product - expected).abs() < 0.0001,
                    "Product mismatch: {} * {} * {} * {} = {}, expected {}",
                    r, d, s, o, product, expected);
            }
        }
    } // mod proptest_tests
}

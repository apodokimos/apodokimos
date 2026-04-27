# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-04-27

### wp-v0.2 Math Correction Alignment

This release implements the corrected formulas from Apodokimos whitepaper v0.2, addressing Bug B1 (multiplicative zero-collapse) and aligning the implementation with the refined epistemic weight model.

### Added

- **Version DOI tracking** (C-21, wp-v0.2 §2.2):
  - `VersionDOI` newtype with ISO 26324-compliant validation
  - `spec_version_doi` field on `Claim` for protocol version tracking
  - `Claim::with_version()` constructor for explicit version specification

- **Multi-signature governance** (C-29, wp-v0.2 §7.5):
  - `GovernanceVerifier` for k-of-n signature verification (Ed25519)
  - Byzantine-safe quorum arithmetic (`k > n/2` enforced)
  - `GovernanceAction` with typed action records and signature validation

- **Cross-field voting** (C-31, wp-v0.2 §7.1):
  - `AccountSbt` for per-field SBT score tracking
  - `cross_field_vote_weight()` using arithmetic mean over non-zero fields
  - `field_vote_weight()` for single-field voting (sqrt of score)
  - Fixes wp-v0.1 bug where geometric mean zeroed out specialists

- **Property-based tests** (C-30, C-32, C-33):
  - Weight monotonicity under `Replicates` attestations
  - Retraction discount monotonicity under cascade depth
  - Non-zero baseline for unattested terminal claims (regression against B1)
  - Basic science (O=0) does not zero weight
  - Sybil-resistance documentation: quadratic voting INCREASES weight under fragmentation
  - Numerical sanity-check: self-registered whitepaper claim has W > 0

### Changed

- **Recency factor R(c,t)** (C-22, wp-v0.2 §3.2):
  - Reparameterized time-decay as `2^(−Δt/t_½)` (base-2 exponential)
  - Added Laplace smoothing (uniform Beta prior, α=β=1) for strictly positive baseline

- **Depth factor D̃(c)** (C-23, wp-v0.2 §3.3):
  - Reimplemented as log-normalized depth: `[1 + ln(1+D)] / [1 + ln(1+D_ref)]`
  - Softer penalty than wp-v0.1 linear model; logarithmic boost for deep chains

- **Survival rate S(c)** (C-24, wp-v0.2 §3.4):
  - Reimplemented with Laplace smoothing: `(N₊ + 1) / (N₊ + N₋ + 2)`
  - Baseline S = 0.5 for unattested claims (maximal uncertainty)

- **Oracle factor O(c)** (C-26, wp-v0.2 §3.5):
  - Changed from direct multiplier to bonus form: `(1 + γ·O)` where O ∈ [0,1]
  - Dropped wp-v0.1 discontinuity (`{0} ∪ [0.1, 1.0]`)
  - Basic science claims (O=0) get bonus = 1.0 instead of zero

- **Weight formula** (C-25, wp-v0.2 §3.1):
  - `W = R × D̃ × S × (1 + γ·O) × δ`
  - All factors now have strictly positive baselines (fixes Bug B1)

- **Retraction propagation** (C-27, C-28, wp-v0.2 §5.2):
  - Added `δ(c)` retraction discount field to `Claim`
  - `propagate_retraction()` implements multiplicative δ cascade
  - Per-field `cascade_threshold()` (Θ_field) limits propagation depth
  - Uses `W_pre` snapshot for discount calculation

### Fixed

- **Bug B1: Multiplicative zero-collapse** (C-25, C-33):
  - wp-v0.1 formula `W = R × D × S × O` zeroed W when any factor was zero
  - This caused all newly-registered claims (R=0, S=0) and terminal claims (D=0) to have zero weight
  - wp-v0.2 formula ensures W > 0 for all valid claims via Laplace smoothing and bonus-form oracle factor

[0.3.0]: https://github.com/apodokimos/apodokimos/releases/tag/apodokimos-core-v0.3.0

## [0.2.0] - 2025-04-15

### W computation — wp-v0.1 formulas (C-14 to C-20)

Initial implementation of the weight function using the wp-v0.1 formulas. Note: all
factors in this release were superseded in v0.3.0 by the wp-v0.2 corrections.

### Added

- **Weight computation** (C-14):
  - `WeightFunction::compute(claim_id, graph_snapshot) -> ClaimWeight`
  - `GraphSnapshot` — point-in-time view of claims and attestations
  - `ClaimWeight` — computed weight with factor breakdown

- **Recency factor R(t)** (C-15):
  - Field-calibrated half-life time decay
  - `FieldSchema` trait extended with `decay_half_life()` and `compute_decay()`

- **Depth factor D(c)** (C-16):
  - Dependency depth traversal on DAG with cycle detection
  - Linear depth normalization against field reference depth

- **Survival rate S(c)** (C-17):
  - Raw ratio of supporting attestations over total scored attestations
  - `AttestationVerdict::contributes_to_survival()` classification

- **Oracle factor O(c)** (C-18):
  - Typed `OFactorSource` enum (ClinicalTrial, SystematicReview, Preprint, PeerReviewed, Dataset, Software)
  - Credibility scoring per source type
  - Discontinuous domain `{0} ∪ [0.1, 1.0]` *(redesigned in v0.3.0)*

- **Retraction propagation** (C-19):
  - `propagate_retraction(claim_id, graph) -> Vec<AffectedClaim>`
  - BFS cascade over dependent claims with depth-weighted penalty

- **Property-based tests** (C-20):
  - `proptest` suite for weight function monotonicity under supporting attestations

[0.2.0]: https://github.com/apodokimos/apodokimos/releases/tag/apodokimos-core-v0.2.0

## [0.1.0] - 2025-04-15

### Added

- **Core types** (P-01, P-02):
  - `ClaimId` — Blake3 hash newtype (32 bytes)
  - `ClaimType` enum — PrimaryClaim, Hypothesis, Method, Result, Replication, NullResult
  - `AttestationVerdict` enum — Supports, Contradicts, Replicates, Refutes, Mentions
  - `Claim` struct — Complete claim representation with id, type, field, content, submitter, dependencies
  - `Attestation` struct — Typed attestation with verdict, evidence, SBT snapshot

- **Error handling**:
  - `ApodokimosError` enum with validation and integrity error classification

- **Hash and serialization**:
  - `compute_claim_hash()` — Blake3 hash of canonical claim content
  - `canonical_serialize()` — Deterministic JSON serialization
  - `Claim::verify_hash()` — Hash verification for integrity checks

- **Feature flags**:
  - `std` (default): Standard library support
  - `no_std`: Core functionality without std (requires `alloc`)

- **Tests**:
  - Hash stability and round-trip serialization
  - Enum exhaustiveness and classification
  - Claim creation and dependency checking

[0.1.0]: https://github.com/apodokimos/apodokimos/releases/tag/apodokimos-core-v0.1.0

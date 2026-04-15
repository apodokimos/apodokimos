# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

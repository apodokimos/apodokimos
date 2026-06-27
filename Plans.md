# Apodokimos Project Plans

## Current Phase

Phase 2: Core Types and Log Implementation

## Active Work

### Documentation–Code Consistency Audit (2026-06-07)

**BLOCKING — Must fix before public API release:**

| Task | Content | DoD | Depends | Status |
|------|---------|-----|---------|--------|
| D1 | Fix `ClaimHash` API documentation | ARCHITECTURE.md, CLAUDE.md, lib.rs updated; ClaimHash struct with `compute()` method exported; API contract matches whitepaper | - | cc:done [998acac] |
| D2 | Remove phantom modules from `apodokimos-log` documentation | ARCHITECTURE.md and CLAUDE.md list only `client.rs`, `merkle.rs`, `types.rs`, `error.rs`; no refs to `inclusion.rs`, `consistency.rs`, `witness.rs` | - | cc:done [186b5f9] |
| D3 | Fix missing `attestation.rs` module documentation | ARCHITECTURE.md and CLAUDE.md correctly identify Attestation types in `claim.rs`; no phantom `attestation.rs` refs remain | D1, D2 | cc:done [275cea25] |
| D4 | Implement k-of-n threshold in `verify_witness_signatures` | `verify_witness_signatures` accepts `threshold: usize` parameter; validates ≥k of n witnesses valid; split-view defense documented | D3 | cc:done [d6b7c6d] |
| D5 | Unify merkle root implementations | `apodokimos-log::merkle::merkle_root` and `apodokimos-anchor::merkle_root` produce identical roots for odd-count trees; shared test suite passes | D4 | cc:TODO |
| D6 | Implement true canonical serialization | RFC 8785 canonicalization applied; test coverage across serde_json versions; ClaimHash portability verified | D5 | cc:TODO |

**RECOMMENDING — Fix before Phase 3 work:**

| Task | Content | DoD | Depends | Status |
|------|---------|-----|---------|--------|
| D7 | Document block-time to log-timestamp bridge | `weight.rs` documents Claim.registered → SignedTreeHead.timestamp_ms mapping; TODO added for Phase 3 state derivation | D6 | cc:TODO |
| D8 | Add `min_sbt_score` to `FieldSchema` trait | FieldSchema trait includes `fn min_sbt_score(&self) -> Option<f64>`; R6 gating logic can read per-field thresholds | D7 | cc:TODO |

**OPTIONAL — Fix for code clarity:**

| Task | Content | DoD | Depends | Status |
|------|---------|-----|---------|--------|
| D9 | Resolve `ClaimContent` visibility | Either remove `pub use` + `#[doc(hidden)]` or document intended API surface | D8 | cc:TODO |
| D10 | Update `registered` field comment | `Claim.registered` comment clarifies meaning in log-based model (not block number) | D9 | cc:TODO |
| D11 | Fix `VersionDOI::wp_v0_2()` DOI | `VersionDOI::wp_v0_2()` returns actual Zenodo DOI `"doi:10.5281/zenodo.19763292"` | D10 | cc:TODO |

## Completed

<!-- Archive completed tasks here after phase completion -->

## Notes

- See ARCHITECTURE.md for layer roadmap
- See TODO.md for detailed task list
- Whitepaper: [wp-v0.2](https://doi.org/10.5281/zenodo.19763292)

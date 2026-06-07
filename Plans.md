# Apodokimos Project Plans

## Current Phase

Phase 2: Core Types and Log Implementation

## Active Work

### Documentation–Code Consistency Audit (2026-06-07)

**BLOCKING — Must fix before public API release:**

- [ ] **D1: Fix `ClaimHash` API documentation** — ARCHITECTURE.md and CLAUDE.md document `ClaimHash::compute(&Claim)` but actual API is `compute_claim_hash()` free function. Update docs to match implementation or refactor code to match docs.
  - Files: ARCHITECTURE.md, CLAUDE.md, crates/apodokimos-core/src/lib.rs
  - Impact: API contract broken; consumers following docs will fail to compile

- [ ] **D2: Remove phantom modules from `apodokimos-log` documentation** — ARCHITECTURE.md and CLAUDE.md list non-existent files (`inclusion.rs`, `consistency.rs`, `witness.rs`). All logic is in `client.rs`. Update documentation to match actual module structure.
  - Files: ARCHITECTURE.md, CLAUDE.md, crates/apodokimos-log/src/
  - Impact: Documentation navigation broken; misleads maintainers

- [ ] **D3: Fix missing `attestation.rs` module documentation** — Documented as standalone module; actually in `claim.rs`. Update ARCHITECTURE.md and CLAUDE.md.
  - Files: ARCHITECTURE.md, CLAUDE.md, crates/apodokimos-core/src/claim.rs
  - Impact: Same navigation breakage as D2

- [ ] **D4: Implement k-of-n threshold in `verify_witness_signatures`** — Currently requires all witnesses to be valid; no threshold semantics. Documented as "split-view defense" which implies threshold. Add `threshold: usize` parameter and validate at least k of n witnesses are valid.
  - Files: crates/apodokimos-log/src/client.rs (line 111–141)
  - Impact: Correctness gap; split-view defense claim is not implemented

**RECOMMENDING — Fix before Phase 3 work:**

- [ ] **D5: Unify merkle root implementations** — `apodokimos-log::merkle::merkle_root` (RFC 6962 recursive) and `apodokimos-anchor::merkle_root` (iterative, duplicates last leaf) produce different roots for odd-count trees. Pick canonical implementation, test across versions, import in both crates.
  - Files: crates/apodokimos-log/src/merkle.rs (line 21), crates/apodokimos-anchor/src/lib.rs (line 149)
  - Impact: Silent root mismatch if cross-verified; difficult to diagnose

- [ ] **D6: Implement true canonical serialization** — Current `canonical_serialize` uses raw `serde_json::to_vec()` without guarantees. CLAUDE.md calls this critical. Implement RFC 8785 canonicalization or add test coverage across `serde_json` versions.
  - Files: crates/apodokimos-core/src/lib.rs (line 44–49)
  - Impact: Hashes may diverge across implementations/versions; breaks claim portability

- [ ] **D7: Document block-time to log-timestamp bridge** — Block-based time model (`Claim.registered: BlockNumber`, weight function uses `block_time_seconds`) is Substrate residue; log-based architecture uses `timestamp_ms` from `SignedTreeHead`. Add TODO or bridge documentation in `weight.rs`.
  - Files: crates/apodokimos-core/src/weight.rs, crates/apodokimos-core/src/claim.rs
  - Impact: Phase 3 state derivation will need undocumented bridge

- [ ] **D8: Add `min_sbt_score` to `FieldSchema` trait** — SBT gating (R6, RI-17a) requires reading `min_sbt_score` per field. Trait lacks this method. Add before Phase 3 to avoid breaking `apodokimos-core`.
  - Files: crates/apodokimos-core/src/field.rs
  - Impact: Phase 3 cannot implement R6 without breaking change to core trait

**OPTIONAL — Fix for code clarity:**

- [ ] **D9: Resolve `ClaimContent` visibility** — Currently `pub use` but `#[doc(hidden)]`. Either make it internal or document it. (low priority)
  - Files: crates/apodokimos-core/src/lib.rs (line 53)

- [ ] **D10: Update `registered` field comment** — Says "block number"; misleading in log-based model. Clarify what "registered" means.
  - Files: crates/apodokimos-core/src/claim.rs (line 174)

- [ ] **D11: Fix `VersionDOI::wp_v0_2()` DOI** — Returns `"doi:10.5281/apodokimos.wp-v0.2"` (placeholder); should be `"doi:10.5281/zenodo.19763292"` (actual Zenodo DOI).
  - Files: crates/apodokimos-core/src/version_doi.rs (line 33)
  - Impact: Claims carry wrong archival identifier

## Completed

<!-- Archive completed tasks here -->

## Notes

- See ARCHITECTURE.md for layer roadmap
- See TODO.md for detailed task list
- Whitepaper: [wp-v0.2](https://doi.org/10.5281/zenodo.19763292)

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Apodokimos** is a decentralized protocol for measuring scientific claim quality through survival scoring rather than citation counting. It replaces journal-level prestige metrics with claim-level attestation graphs. The whitepaper (wp-v0.2) is versioned and archived independently at DOI [10.5281/zenodo.19763292](https://doi.org/10.5281/zenodo.19763292).

**Key orientation:**
- Claim is the atomic unit (not papers)
- Survival score replaces citation count
- Reputation encoded as non-transferable SBTs (Soulbound Tokens)
- Content stored on Arweave; history on a transparency log (RFC 6962)
- Deterministic state-derivation from log history
- Currently in Phase 2 of development; apodokimos-core and apodokimos-log crates are implemented

## Build & Development Commands

### Standard x86_64 Development

The project has a Cargo workspace with two crates: `apodokimos-core` (protocol core types) and `apodokimos-log` (transparency log client). Default build target is WASM; use the `x86_64-unknown-linux-gnu` target for local development.

```bash
# Check and lint (recommended daily)
cargo test-all                    # Run all tests, clippy, build
cargo check-all                   # Just check (no build/test)
cargo lint                        # Clippy warnings as errors

# Run tests
cargo test --target x86_64-unknown-linux-gnu --lib      # Unit tests only
cargo test --target x86_64-unknown-linux-gnu            # All tests (integration + unit)
cargo test --target x86_64-unknown-linux-gnu test_name  # Single test by pattern

# Build for x86_64
cargo build --target x86_64-unknown-linux-gnu --release
```

### WASM Build (default)

WASM targets default to `wasm32-unknown-unknown` per `.cargo/config.toml`. The environment and runner (`wasm-bindgen-test-runner`) are configured automatically.

```bash
# WASM check (uses default target)
cargo check --lib
cargo test --lib  # Requires wasm32 environment; typically skipped in CI
```

### Configuration Notes

- **Rust edition:** 2024
- **MSRV:** 1.85
- **Aliases defined in `.cargo/config.toml`:**
  - `test-all` — test all workspace crates on x86_64 target
  - `check-all` — check all workspace crates on x86_64 target
  - `lint` — clippy with `-D warnings` (deny all warnings)

## Architecture Layers

The Apodokimos protocol has 7 architectural layers. Currently implemented:

### Phase 2 (Current): Core Types and Log

1. **`apodokimos-core` — Protocol Core** (v0.3.0)
   - Defines claim, attestation, field, version DOI, and weight function types
   - Implements W(c,t) = R × D̃ × S × (1 + γO) × δ weight computation
   - Implements retraction cascade logic per whitepaper §5
   - no_std compatible; AGPL-3.0
   - Key modules: `claim.rs`, `weight.rs`, `attestation.rs`, `field.rs`, `version_doi.rs`, `voting.rs`, `governance.rs`

2. **`apodokimos-log` — Transparency Log** (v0.1.0, in Phase 2)
   - RFC 6962 (Certificate Transparency) style append-only log
   - Merkle tree verification, inclusion/consistency proofs
   - Multi-witness co-signature support (split-view defense)
   - Key modules: `client.rs`, `merkle.rs`, `types.rs`
   - Integration tests in `tests/local_log.rs`

### Future Phases (Phase 3+)

3. **apodokimos-state** (Phase 2 follow-up) — Deterministic state derivation; reconstructs graph and computes W scores
4. **apodokimos-anchor** (Phase 3) — OpenTimestamps integration; Bitcoin-anchored durability
5. **apodokimos-arweave** (Phase 4) — Content storage on Arweave with immutable Spec-Version-DOI tags
6. **apodokimos-indexer** (Phase 5) — Oracle polling (ClinicalTrials.gov, PROSPERO); snapshot publication
7. **SDK and CLI** (Phases 6–7) — apodokimos-sdk (Rust + WASM bindings), apodokimos-cli (command-line interface), sdk-ts (TypeScript wrapper)

## Key Concepts

**Claim Weight Function:** W(c, t) = R(c, t) × D̃(c) × S(c) × (1 + γ · O(c)) × δ(c)
- R: replication score with time decay
- D̃: log-normalized dependency depth
- S: survival rate under falsification attempts
- O: real-world outcome linkage (multiplicative bonus)
- δ: retraction discount with penalty propagation

**Governance & Reputation:** Voting weight is `sqrt(field_score)` — quadratic weighting prevents plutocracy. SBTs (Soulbound Tokens) are non-transferable by design (no transfer code path anywhere), ensuring reputation cannot be purchased.

**Specification Versioning:** Every claim carries `spec_version_doi` binding it to the whitepaper version (e.g., wp-v0.2) under which it was registered. Past claims retain their original semantics indefinitely; W scores are recomputed under their original version's rules, even after new versions ship.

**Bootstrap Domain:** Clinical medicine (PICO structure: Population, Intervention, Comparator, Outcome). Trial registry IDs (NCT) provide direct outcome linkage; first batch targets 50 claims from Cochrane systematic reviews.

## Testing & Quality

- **69 tests** across core and log crates; all passing on x86_64
- **Integration test:** `crates/apodokimos-log/tests/local_log.rs` validates merkle tree and witness logic
- **Property tests:** `proptest` used for generative testing in core modules
- **Clippy:** `-D warnings` enforced; all warnings are errors

To run all quality checks at once (recommended before commits):
```bash
cargo test-all
```

## License Structure

| Crate | License | Purpose |
|-------|---------|---------|
| apodokimos-core, apodokimos-log | AGPL-3.0 | Protocol core; copyleft enforcement prevents institutional capture |
| apodokimos-sdk, sdk-ts | Apache-2.0 | Client libraries (permissive for consumer ecosystems) |
| Field schemas | CC0-1.0 | Claim content structure (public domain) |

## Documentation

- **`README.md`** — Protocol overview, problem statement, claim weight function
- **`ARCHITECTURE.md`** — Detailed layer mapping, phase roadmap, alternative implementations
- **`TODO.md`** — Task-by-task roadmap with completion status (Phase 0 complete, Phase 1–2 in progress)
- **Whitepaper:** [wp-v0.2](https://doi.org/10.5281/zenodo.19763292) — authoritative protocol specification

## CI/CD Notes

- Branch protection on `main` requires CI to pass
- CI runs `cargo test-all` and `cargo lint`
- Default build target is WASM; CI explicitly tests x86_64
- No WASM-specific tests run by default in CI (environment setup required)

## Common Tasks

**Adding a new type to protocol core:**
1. Define in appropriate module (`claim.rs`, `attestation.rs`, etc.)
2. Implement `Serialize`/`Deserialize` via serde
3. Add tests with `proptest` for invariants
4. Run `cargo test-all` to verify no regressions
5. Update `ARCHITECTURE.md` if semantics change

**Writing a test:**
```bash
# Run tests matching pattern
cargo test --target x86_64-unknown-linux-gnu my_test_name -- --nocapture

# Run a single test file
cargo test --test local_log --target x86_64-unknown-linux-gnu
```

**Checking for dependency issues:**
```bash
cargo deny check
```

## Know Before You Code

1. **no_std compatibility matters for `apodokimos-core`** — it may eventually run on embedded systems or WASM. Use `alloc`-gated features where heap is needed.

2. **Canonical serialization is critical.** `ClaimHash::compute()` in core must produce bit-identical hashes across implementations and versions. Test across multiple invocations.

3. **Spec-version coherence is non-negotiable.** Any new claim or attestation type must carry the `spec_version_doi` field, binding it to the whitepaper version. No retroactive spec changes; only forward-compatible additions with new versions.

4. **Whitepaper is the source of truth.** If code and whitepaper conflict, whitepaper wins. Cite the relevant section (e.g., "wp-v0.2 §3.2") in code comments explaining why a choice was made.

5. **AGPL-3.0 is by design.** Any hosted fork must remain open. This is protocol-capture resistance, not conventional OSS licensing. When linking apodokimos-core, downstream code must comply with AGPL-3.0.

# TODO

Atomic task list following OSS SDLC/DevSecOps. Each task is independently completable.
Format: `[ ] TASK-ID — description`

SemVer milestones marked as `### vX.Y.Z`.

**Specification anchor.** This roadmap targets wp-v0.2 ([10.5281/zenodo.19763292](https://doi.org/10.5281/zenodo.19763292)). Tasks completed against wp-v0.1 are preserved verbatim; where wp-v0.2 supersedes their underlying spec, an annotation points to the wp-v0.2 follow-up task. The wp-v0.1 Substrate parachain track (Phase 3, original) is preserved in full as the alternative-implementation track per [whitepaper §10.4](https://doi.org/10.5281/zenodo.19763292).

---

## Phase 0 — Foundation

> Prerequisite for all code. No code written before this phase is complete.

### Repository & Governance

- [x] F-01 — Create GitHub org `apodokimos`
- [x] F-02 — Create monorepo `apodokimos/apodokimos` with branch protection on `main`
- [x] F-03 — Write `LICENSE-AGPL` (AGPL-3.0) at repo root
- [x] F-04 — Write `LICENSE-APACHE` (Apache-2.0) at repo root
- [x] F-05 — Write `LICENSE-CC0` at repo root
- [x] F-06 — Add `CODEOWNERS` with initial maintainer DID references (no real names)
- [x] F-07 — Write `CONTRIBUTING.md` — contribution process, DCO sign-off requirement
- [x] F-08 — Write `SECURITY.md` — responsible disclosure policy
- [x] F-09 — Write `CODE_OF_CONDUCT.md` — Contributor Covenant 2.1

### Workspace Scaffold

- [x] F-10 — Init Cargo workspace root `Cargo.toml` with all member crates listed
- [x] F-11 — Add `rust-toolchain.toml` pinned to latest stable channel
- [x] F-12 — Add `.cargo/config.toml` — `cargo-deny` configuration, target settings
- [x] F-13 — Init `pnpm-workspace.yaml` for `sdk-ts`
- [x] F-14 — Add `.gitignore` covering Rust, Node, IDE artifacts
- [x] F-15 — Add `cargo install cargo-skill` to developer setup docs
- [x] F-16 — Add `cargo install cargo-audit` to developer setup docs
- [x] F-17 — Add `cargo install cargo-deny` to developer setup docs
- [x] F-18 — Add `cargo install cargo-expand` to developer setup docs

### CI/CD Skeleton

- [x] F-19 — Create `.github/workflows/ci.yml` — `cargo check`, `cargo test`, `cargo clippy --deny warnings`
- [x] F-20 — Create `.github/workflows/audit.yml` — `cargo audit` on push + weekly schedule
- [x] F-21 — Create `.github/workflows/deny.yml` — `cargo deny check` (licenses, advisories, bans)
- [x] F-22 — Create `.github/workflows/fmt.yml` — `cargo fmt --check`
- [ ] F-23 — Add branch protection rules: require CI green before merge, no force push to `main` _(requires manual GitHub UI configuration)_
- [x] F-24 — Configure Dependabot for Cargo and npm weekly updates

### v0.2 additions

- [x] F-25 — Create `.github/workflows/wp-placeholder-lint.yml` — fail any merge to `main` containing `<TO-BE-ASSIGNED-AT-ANCHOR>` or other placeholder strings inside `WHITEPAPER/` artifacts (per wp-v0.2 §1.6 documentation hygiene)
- [x] F-26 — Scaffold `governance/` directory: `genesis.toml` template (DIDs + public keys + threshold k), `actions/` for signed governance records (per wp-v0.2 §7.5)
- [x] F-27 — Add `WHITEPAPER/` directory layout: `WHITEPAPER.md` (wp-v0.1, preserved), `WHITEPAPER_v0.2.md` (current); README cross-references the Version DOI table
- [x] F-28 — Add `.github/workflows/wp-version-check.yml` — verify the current `WHITEPAPER_v0.2.md` Version DOI matches the README's "Current" entry

---

## Phase 1 — Protocol Specification (wp-v0.1)

> All tasks below were completed against wp-v0.1. wp-v0.2 supersedes the formulas marked below; the original tasks remain checked as historical record. wp-v0.2 follow-ups are tracked under "Phase 1' — wp-v0.2 Specification Work" below.

### Claim Model Formalization

- [x] P-01 — Define `ClaimType` taxonomy: `PrimaryClaim | Hypothesis | Method | Result | Replication | Null`
- [x] P-02 — Define `AttestationVerdict` enum: `Supports | Contradicts | Replicates | Refutes | Mentions`
- [x] P-03 — Formally specify W(claim) = R(t) × D × S × O with typed definitions for each variable _(targets wp-v0.1; superseded by wp-v0.2 §3.1 and tracked under V-04 below)_
- [x] P-04 — Define field-calibrated time-decay function for R(t) per domain class _(targets wp-v0.1; reparameterized as half-life in wp-v0.2; see V-04)_
- [x] P-05 — Define dependency depth D: DAG traversal algorithm specification _(targets wp-v0.1; log-normalized as D̃ in wp-v0.2; see V-04)_
- [x] P-06 — Define survival rate S: ratio of supporting to total non-mentioning attestations _(targets wp-v0.1; Laplace-smoothed in wp-v0.2; see V-04)_
- [x] P-07 — Define O factor: enumerated oracle source types and linkage schema _(targets wp-v0.1; bonus form `(1 + γO)` in wp-v0.2; see V-04)_
- [x] P-08 — Define penalty propagation: retraction event cascades to dependent claims _(targets wp-v0.1; explicit δ persistence in wp-v0.2; see V-04)_
- [x] P-09 — Define SBT reputation score structure: `{ field_id, score, attestation_count, survival_rate }`
- [x] P-10 — Define quadratic SBT voting weight formula _(targets wp-v0.1; cross-field formula corrected in wp-v0.2; see V-05)_
- [x] P-11 — Write `WHITEPAPER.md` — ECG technical specification (versioned, `wp-v0.1`):
  - [x] P-11a — Abstract: problem statement (IF misalignment, DeSci ontology failure)
  - [x] P-11b — Claim model: formal definition, types, granularity constraints
  - [x] P-11c — W(claim) formal specification: R(t), D, S, O with typed math
  - [x] P-11d — Attestation graph: DAG structure, edge semantics, cycle prevention
  - [x] P-11e — Penalty propagation: retraction cascade formal model
  - [x] P-11f — SBT reputation: mint/increment/burn lifecycle, non-transferability proof
  - [x] P-11g — Governance: quadratic SBT voting, proposal lifecycle, attack surface analysis
  - [x] P-11h — Identity layer: DID integration, ZK credential proof scheme
  - [x] P-11i — Arweave content layer: tagging schema, hash binding, permanence guarantees
  - [x] P-11j — Substrate pallet architecture: storage, extrinsics, events per pallet _(superseded by wp-v0.2 §10; preserved as Alternative A in ARCHITECTURE.md)_
  - [x] P-11k — Bootstrap strategy: clinical medicine pilot rationale and PICO schema
  - [x] P-11l — Security analysis: Sybil, governance capture, oracle manipulation, GDPR
  - [x] P-11m — Anchor `WHITEPAPER.md` on Arweave + Zenodo (DOI for timestamped priority)
  - [ ] P-11n — Register whitepaper as first Apodokimos claim on testnet (protocol validates itself) _(re-pointed to wp-v0.2; see V-08)_

### Field Schema: Clinical Medicine Bootstrap

- [x] P-12 — Define PICO claim schema (population, intervention, comparator, outcome, effect)
- [x] P-13 — Define O factor oracle mapping: `trial_registry_id → ClinicalTrials.gov NCT`
- [x] P-14 — Define O factor oracle mapping: `prospero_id → PROSPERO registration`
- [x] P-15 — Write `fields/clinical-medicine-v0.1.json` — CC0 schema file
- [x] P-16 — Define field normalization coefficient for clinical medicine (baseline USI calibration)

---

## Phase 1' — wp-v0.2 Specification Work

> Specification-level tasks introduced or completed in the wp-v0.2 round. Distinct from implementation tasks (those are in Phase 2'+).

- [x] V-01 — Anchor `WHITEPAPER_v0.2.md` to Zenodo as new version of wp-v0.1 record (Version DOI: `10.5281/zenodo.19763292`)
- [x] V-02 — Commit `WHITEPAPER/WHITEPAPER_v0.2.md` to repo with header pointing at its own Version DOI
- [x] V-03 — Update `WHITEPAPER/WHITEPAPER.md` (wp-v0.1) header to add a "Superseded by" pointer to wp-v0.2 _(metadata-only edit, document bytes preserved)_
- [x] V-04 — Document the wp-v0.2 W(c, t) revision in a `CHANGELOG-WHITEPAPER.md` at repo root: Laplace smoothing for R and S, log-normalized D̃, multiplicative O bonus, explicit δ retraction discount (per wp-v0.2 Appendix D)
- [x] V-05 — Document the wp-v0.2 cross-field voting correction (geomean → arithmetic mean over non-zero) in `CHANGELOG-WHITEPAPER.md`
- [x] V-06 — Document the §1.6 versioning convention in `CHANGELOG-WHITEPAPER.md`: Version DOI for archival, Concept DOI for navigation, placeholder discipline for drafts
- [ ] V-07 — Once Concept DOI is visible on Zenodo's Versions panel, update README to display it explicitly (currently a forward-pointer)
- [ ] V-08 — Register wp-v0.2 as the first ECG claim on testnet, with `spec_version_doi = 10.5281/zenodo.19763292` _(blocks on RI- testnet readiness; supersedes P-11n)_

---

## Phase 2 — `apodokimos-core`

> AGPL-3.0 | `no_std` compatible | MSRV: latest stable

### `v0.1.0` (wp-v0.1 baseline)

- [x] C-01 — Init crate with `Cargo.toml`: `no_std`, `thiserror`, `serde`, `blake3`
- [x] C-02 — Implement `ClaimId` newtype (blake3 hash of canonical claim content)
- [x] C-03 — Implement `Claim` struct with all fields per P-01 spec
- [x] C-04 — Implement `ClaimType` enum per P-01
- [x] C-05 — Implement `AttestationVerdict` enum per P-02
- [x] C-06 — Implement `Attestation` struct: `{ claim_id, attester_did, verdict, evidence_tx_id, block }`
- [x] C-07 — Implement `FieldSchema` trait: `field_id()`, `normalize_score()`, `decay_half_life()`
- [x] C-08 — Implement `ApodokimosError` with `thiserror`
- [x] C-09 — Implement canonical JSON serialization for `Claim` (deterministic, CC0 schema)
- [x] C-10 — Implement `ClaimHash::compute(claim: &Claim) -> ClaimId`
- [x] C-11 — Write unit tests: hash stability, serialization round-trip, enum exhaustiveness
- [x] C-12 — Write `CHANGELOG.md` entry for v0.1.0

### `v0.2.0` (W computation, wp-v0.1 formulas)

- [x] C-14 — Implement `WeightFunction::compute(claim_id, graph_snapshot) -> ClaimWeight` _(implements wp-v0.1 W; superseded by C-25 implementing wp-v0.2 W)_
- [x] C-15 — Implement R(t) time-decay with field-calibrated half-life _(targets wp-v0.1; superseded by C-22)_
- [x] C-16 — Implement D dependency depth traversal on DAG _(targets wp-v0.1; superseded by C-23)_
- [x] C-17 — Implement S survival rate from attestation set _(targets wp-v0.1; superseded by C-24)_
- [x] C-18 — Implement O factor: typed `OFactorSource` enum + linkage validation _(targets wp-v0.1; superseded by C-26)_
- [x] C-19 — Implement penalty propagation: `propagate_retraction(claim_id, graph) -> Vec<AffectedClaim>` _(targets wp-v0.1; superseded by C-27)_
- [x] C-20 — Write property-based tests with `proptest` for weight function monotonicity _(targets wp-v0.1; superseded by C-30)_

### `v0.3.0` — wp-v0.2 math correction

- [x] C-21 — Add `spec_version_doi: VersionDOI` field to `Claim` struct (per wp-v0.2 §2.2); add `version_doi.rs` module with `VersionDOI` newtype + parsing/validation
- [x] C-22 — Reimplement R(c, t) with Laplace smoothing (uniform Beta prior, default α=β=1); reparameterize time-decay as `2^(−Δt/t_½)` per wp-v0.2 §3.2
- [x] C-23 — Reimplement D̃(c) as log-normalized depth `[1 + log(1+D)] / [1 + log(1+D_ref_field)]` per wp-v0.2 §3.3
- [x] C-24 — Reimplement S(c) with Laplace smoothing per wp-v0.2 §3.4
- [x] C-25 — Reimplement `WeightFunction::compute` with formula `W = R × D̃ × S × (1 + γ·O) × δ` per wp-v0.2 §3.1
- [x] C-26 — Reimplement O factor as `O ∈ [0, 1]` (drop the `{0} ∪ [0.1, 1.0]` discontinuity); enter as `(1 + γ·O)` bonus rather than direct multiplier per wp-v0.2 §3.5
- [x] C-27 — Add `δ(c)` retraction discount field; implement `propagate_retraction` operating on δ with `W_pre` snapshot per wp-v0.2 §5.2
- [x] C-28 — Add `Θ_field` per-field cascade threshold to `FieldSchema` trait
- [x] C-29 — Implement multi-signature governance verification (k-of-n signed action records) per wp-v0.2 §7.5
- [x] C-30 — Update property-based tests: monotonicity of W under additional `Replicates`, monotonicity of δ under retraction cascade, non-zero baseline for unattested terminal claim, basic-science (O=0) does not zero W
- [x] C-31 — Add cross-field voting tests: arithmetic mean over non-zero (specialist with one field retains weight), geomean fix per wp-v0.2 §7.1
- [x] C-32 — Add Sybil-resistance test: documents that quadratic voting INCREASES total weight under fragmentation (per wp-v0.2 §12.1 corrected); resistance is via SBT-accumulation cost, not voting formula
- [x] C-33 — Numerical sanity-check suite: verify §11.4 self-registered whitepaper claim has W > 0 under wp-v0.2 (regression against bug B1)
- [x] C-34 — Write `CHANGELOG.md` entry for v0.3.0 — annotate all C-21 through C-33 as wp-v0.2 alignment

---

## Phase 3 — Reference Implementation (wp-v0.2 path)

> AGPL-3.0 | Rust | transparency log + Arweave + OpenTimestamps + state-derivation per wp-v0.2 §10.2.
>
> NOTE: The original Phase 3 (Substrate parachain pallets) is preserved in full at the end of this document under "Phase 3-ALT — Alternative Implementation: Substrate Parachain (wp-v0.1)". That track is deferred to v1.0+ as one valid alternative path per wp-v0.2 §10.4.

### `apodokimos-log` — Transparency Log Client

- [x] RI-01 — Survey transparency log implementations (Trillian, Sigstore Rekor, custom RFC 6962 implementation); pin a choice with documented rationale in `docs/log-impl-decision.md`
  - [x] RI-01a — Research Trillian: architecture, production deployments, Rust bindings availability
  - [x] RI-01b — Research Sigstore Rekor: transparency log design, proof verification, independent-operator model
  - [x] RI-01c — Research custom RFC 6962 implementation: baseline Rust crate availability, audit history
  - [x] RI-01d — Document decision rationale: chosen implementation, trade-offs (operator burden, proof verification complexity, integration surface)
  - [x] RI-01e — Write decision document to `docs/log-impl-decision.md`
- [x] RI-02 — Pin `apodokimos-log` Cargo.toml dependencies for chosen log (latest stable)
  - [x] RI-02a — Pin merkle-tree hash library (sha2 vs sha3 decision documented)
  - [x] RI-02b — Pin ed25519 signature library for witness verification
  - [x] RI-02c — Test build with pinned deps, ensure no feature-flag conflicts
- [x] RI-03 — Implement `LogClient::submit(entry: SignedEntry) -> Result<InclusionProof>`
  - [x] RI-03a — Define `SignedEntry` struct: entry data + signature + public key
  - [x] RI-03b — Implement canonical JSON serialization for entry (hash stability)
  - [x] RI-03c — Implement HTTP POST to log server with error handling (retry logic, timeout)
  - [x] RI-03d — Parse `InclusionProof` response: leaf index, merkle path, tree size
  - [x] RI-03e — Unit test: serialize entry deterministically across multiple invocations
  - [x] RI-03f — Integration test: submit 5 entries sequentially, verify all get distinct inclusion proofs
- [x] RI-04 — Implement `LogClient::verify_inclusion(entry, proof, sth) -> bool`
  - [x] RI-04a — Implement merkle path traversal: compute leaf hash, apply proof path nodes, compare root
  - [x] RI-04b — Implement tree size consistency check: leaf index < tree size
  - [x] RI-04c — Verify root matches STH's root field
  - [x] RI-04d — Property test: verify_inclusion is transitively closed (if entry1 in old_proof and path from old to new, then entry1 in new)
  - [x] RI-04e — Rejection test: tampered entry fails verification, modified proof path fails, out-of-bounds index fails
  - [x] RI-04f — Edge case tests: leaf index = 0, leaf index = tree_size-1, proof path empty (single-leaf tree)
- [x] RI-05 — Implement `LogClient::verify_consistency(old_sth, new_sth) -> bool`
  - [x] RI-05a — Implement RFC 6962 consistency proof verification algorithm
  - [x] RI-05b — Check new_tree_size >= old_tree_size (monotonicity)
  - [x] RI-05c — Check old_root matches old_sth (reject if provided old_sth is stale)
  - [x] RI-05d — Compute consistency proof path and verify against new root
  - [x] RI-05e — Detect tree rewrite: if old_sth.root appears in the middle of new tree (invalid proof), flag it
  - [x] RI-05f — Test: consistency proofs between tree sizes 1→5, 5→10, 10→10 (same tree)
  - [x] RI-05g — Test: reject consistency proof with tampered path node
- [x] RI-06 — Implement witness co-signature verification: `verify_witness_signatures(sth, witnesses) -> bool`
  - [x] RI-06a — Define `Witness` struct: witness_id, public_key, signature over (sth.root || sth.tree_size)
  - [x] RI-06b — Implement signature verification: ed25519.verify(pub_key, message, sig)
  - [x] RI-06c — Implement k-of-n threshold check: at least k of n witnesses must have valid signatures
  - [x] RI-06d — Load genesis witness set from `governance/witnesses.toml`
  - [x] RI-06e — Test: 3-of-5 witness set, verify with exactly 3 valid sigs passes, with 2 valid sigs fails
  - [x] RI-06f — Test: witness key mismatch is detected (pub_key in witness ≠ key in sth.witness_signatures)
  - [x] RI-06g — Test: STH reordering (changing tree_size then root) invalidates all witness signatures
- [x] RI-07 — Write integration tests against a local log instance
  - [x] RI-07a — Set up local test log server (in-memory merkle tree, TCP listener on 0.0.0.0:8000)
  - [x] RI-07b — Test: submit → get inclusion proof → verify inclusion against STH
  - [x] RI-07c — Test: submit multiple entries → verify consistency between consecutive STHs
  - [x] RI-07d — Test: witness co-signing on STH after k submissions
  - [x] RI-07e — Test: concurrent submissions (5 threads, 10 entries each) produce correct tree
  - [x] RI-07f — Test: split-view attack detection (witness set disagrees on sth.root for same tree_size) — document as limitation
  - [x] RI-07g — Cleanup: local test server spins down at test end (no zombie processes)
- [x] RI-08 — Document the genesis witness set in `governance/witnesses.toml`
  - [x] RI-08a — Define TOML schema: `[[witness]]` with `id`, `public_key` (hex), `endpoint` (optional, for multi-operator setup)
  - [x] RI-08b — Document the k-of-n threshold (e.g., 3-of-5)
  - [x] RI-08c — Create genesis witness set: 3–5 initial witnesses (placeholder DIDs, real ed25519 public keys)
  - [x] RI-08d — Write `docs/witnesses.md`: role of witnesses, how to join, signature duties, timelock semantics

### `apodokimos-anchor` — OpenTimestamps Integration

- [x] RI-09 — Create `apodokimos-anchor` crate and add dependencies
  - [x] RI-09a — Init `crates/apodokimos-anchor/Cargo.toml` with workspace members
  - [x] RI-09b — Add `opentimestamps-rs` dependency (or equivalent; research latest stable)
  - [x] RI-09c — Add `sha2` for STH hashing
  - [x] RI-09d — Add `tokio` for async anchoring driver
  - [x] RI-09e — Add `serde_json` for proof serialization
  - [x] RI-09f — Test build with `cargo build --release`
- [ ] RI-10 — Implement `Anchor::batch(sths: &[SignedTreeHead]) -> Result<OtsProof>`
  - [ ] RI-10a — Define `OtsProof` struct: timestamp, calendar_url, commitment, serialized OTS proof bytes
  - [ ] RI-10b — Implement canonical hashing of STH: sha2-256(sth.root || sth.tree_size)
  - [ ] RI-10c — Implement batch commitment: merkle-hash multiple STH hashes into a single commitment
  - [ ] RI-10d — Submit commitment to OTS calendar server (HTTP POST to calendar endpoint)
  - [ ] RI-10e — Parse OTS response: extract timestamp proof bytes
  - [ ] RI-10f — Serialize proof to JSON and return
  - [ ] RI-10g — Error handling: network timeout, calendar unavailable, invalid response format
  - [ ] RI-10h — Unit test: batch 3 STHs, verify proof contains all 3 commitments
  - [ ] RI-10i — Test: large batch (100 STHs) completes within 10s
- [ ] RI-11 — Implement `Anchor::verify(proof: &OtsProof, bitcoin_node) -> Result<BlockHeight>`
  - [ ] RI-11a — Parse OTS proof from JSON bytes
  - [ ] RI-11b — Extract timestamp and Bitcoin block height from OTS response
  - [ ] RI-11c — Query Bitcoin node (via RPC or light client) for block at extracted height
  - [ ] RI-11d — Verify block contains the timestamp commitment (via Merkle tree in coinbase or OP_RETURN)
  - [ ] RI-11e — Return BlockHeight on success; error on mismatch
  - [ ] RI-11f — Robustness: handle reorg (if block no longer in chain), stale proof (block too old)
  - [ ] RI-11g — Unit test: verify proof against mock Bitcoin data
  - [ ] RI-11h — Integration test: verify real OTS anchor against testnet Bitcoin node
  - [ ] RI-11i — Document: which OTS calendar servers are trusted (default calendars list)
- [ ] RI-12 — Implement scheduled anchoring driver (configurable cadence, default daily)
  - [ ] RI-12a — Define `AnchorScheduler` struct: anchor client, tick interval, STH queue
  - [ ] RI-12b — Implement async job: wake every N seconds, batch pending STHs, submit to calendar
  - [ ] RI-12c — Persist pending STHs in a local queue (JSON lines file or in-memory with periodic flush)
  - [ ] RI-12d — Retry logic: if calendar fails, re-queue with exponential backoff (max 3 retries)
  - [ ] RI-12e — Metrics: emit anchoring latency, success/failure count, queue depth
  - [ ] RI-12f — Graceful shutdown: flush pending STHs and close connections on termination signal
  - [ ] RI-12g — Configuration: read anchor interval from config file or env var (default 24h)
  - [ ] RI-12h — Integration test: start scheduler, submit STHs, verify they anchor within 1 minute (accelerated for tests)
  - [ ] RI-12i — Test: scheduler handles network interruption (calendar down for 5 minutes, recovers)
- [ ] RI-13 — Write integration tests using OTS calendar testnet
  - [ ] RI-13a — Set up test with OTS calendar testnet endpoint (not mainnet)
  - [ ] RI-13b — Submit proof, wait for confirmation (may take 10+ minutes in testnet)
  - [ ] RI-13c — Verify anchor against Bitcoin testnet
  - [ ] RI-13d — Test: batch 5 STHs, verify all 5 in single calendar response
  - [ ] RI-13e — Test: timeout handling if calendar is slow
  - [ ] RI-13f — Document test setup (testnet calendar endpoints, Bitcoin testnet node requirements)

### `apodokimos-state` — Deterministic State Derivation

- [ ] RI-14 — Init `apodokimos-state` crate with workspace dependencies on `apodokimos-core` and `apodokimos-log`
  - [ ] RI-14a — Create `crates/apodokimos-state/Cargo.toml` with dependency on apodokimos-core v0.3.0+
  - [ ] RI-14b — Add apodokimos-log as workspace dependency
  - [ ] RI-14c — Add serde, serde_json, blake3, sha2 for serialization and hashing
  - [ ] RI-14d — Test build: `cargo build -p apodokimos-state`
- [ ] RI-15 — Implement `Derivation::state_at(log_state) -> ProtocolState` — pure function reading log entries up to STH
  - [ ] RI-15a — Define `ProtocolState` struct: `{ claims: Map<ClaimId, ClaimState>, attestations: Map<(ClaimId, AttesterId), Attestation>, sbt_records: Map<DID, ReputationRecord>, merkle_root: [u8; 32] }`
  - [ ] RI-15b — Define `ClaimState` struct: `{ claim_hash, spec_version_doi, weight, retraction_discount, dependent_claims: Set<ClaimId> }`
  - [ ] RI-15c — Implement entry parsing: deserialize log entry bytes as Claim or Attestation or GovernanceAction
  - [ ] RI-15d — Implement claim registration: validate spec_version_doi against allowlist, insert into claims map, initialize weight
  - [ ] RI-15e — Implement attestation recording: DAG acyclicity check (RI-16), reputation gate (RI-17), SBT update
  - [ ] RI-15f — Implement state snapshot: compute Merkle root over deterministically sorted (claims, attestations, sbt) (RI-22)
  - [ ] RI-15g — Document: "state derivation is a pure function of log; any two operators with same log produce identical ProtocolState"
  - [ ] RI-15h — Unit test: derive state from 5 sequential claims → verify state has 5 claims with zero weight (unattested)
  - [ ] RI-15i — Unit test: replay log forward and backward → forward produces expected state
- [ ] RI-16 — Implement DAG acyclicity enforcement at write-time (R5)
  - [ ] RI-16a — Build dependency graph: claims → attestations → dependent claims
  - [ ] RI-16b — Implement cycle detection: DFS from new attestation's target, reject if cycle detected
  - [ ] RI-16c — Test: three claims forming a cycle, verify third attestation is rejected
  - [ ] RI-16d — Test: linear chain of claims (A → B → C → D) is acyclic, all attestations accepted
  - [ ] RI-16e — Test: large DAG (100 claims, 500 edges) cycle detection completes in <1s
- [ ] RI-17 — Implement reputation-gated attestation acceptance (R6)
  - [ ] RI-17a — Read SBT requirement from field schema: `min_sbt_score: u64` per field
  - [ ] RI-17b — Look up attester's SBT score for the claim's field
  - [ ] RI-17c — Reject attestation if attester.sbt_score < min_sbt_score
  - [ ] RI-17d — Test: attester with 0 SBT cannot attest → rejected
  - [ ] RI-17e — Test: attester with 100 SBT, field min=50 → attestation accepted
  - [ ] RI-17f — Test: cross-field: attester with high SBT in oncology, low in cardiology, cannot attest cardiology claims
- [ ] RI-18 — Implement SBT lifecycle without exposing any transfer operation (R7) — explicit unit test asserts no `transfer` symbol exists in the public API
  - [ ] RI-18a — Define `SBTRecord` struct: `{ holder_did, field_id, current_score, attestation_count, survival_count }`
  - [ ] RI-18b — Implement mint: on first accepted attestation, create SBTRecord with score=1
  - [ ] RI-18c — Implement increment: on claim survival event (time threshold or replication), increment score
  - [ ] RI-18d — Implement penalty: on retraction, apply δ(c) discount to all dependent claims, decrement SBT score proportionally
  - [ ] RI-18e — CRITICAL: Ensure NO `transfer` function in public API (compile-time check: `grep -E 'pub.*fn transfer' src/*.rs` returns empty)
  - [ ] RI-18f — Unit test: mint SBT, verify holder has score=1
  - [ ] RI-18g — Unit test: increment score 5 times, verify score=6
  - [ ] RI-18h — Unit test: apply penalty, verify score decrements but never reaches 0 (per wp-v0.2 §7.2 non-zero baseline)
  - [ ] RI-18i — API audit: review all public methods, confirm none enable SBT transfer
- [ ] RI-19 — Implement W(c, t) computation calling `apodokimos-core` per-claim with that claim's `spec_version_doi` (R13)
  - [ ] RI-19a — For each claim in state: read spec_version_doi field
  - [ ] RI-19b — Load the corresponding spec version's WeightFunction from apodokimos-core (compile-time: always wp-v0.2 for now)
  - [ ] RI-19c — Call WeightFunction::compute(claim, attestations, graph) → ClaimWeight
  - [ ] RI-19d — Store weight in ClaimState
  - [ ] RI-19e — Document: "claims registered under wp-v0.2 are scored under wp-v0.2 rules, even if wp-v0.5 ships" (spec-version coherence)
  - [ ] RI-19f — Test: weight increases as attestations accumulate
  - [ ] RI-19g — Test: weight for unattested claim > 0 (non-zero baseline)
  - [ ] RI-19h — Test: basic-science claim (O=0) has W > 0 (regression test against wp-v0.1 bug B1, per C-33)
- [ ] RI-20 — Implement δ retraction cascade per wp-v0.2 §5.2
  - [ ] RI-20a — On claim retraction: compute δ(c) from wp-v0.2 §5.2 formula (depends on pre-retraction weight and field)
  - [ ] RI-20b — Store δ(c) in ClaimState as retraction_discount
  - [ ] RI-20c — Apply δ(c) to all dependent claims: W_new = W_old × δ(c)
  - [ ] RI-20d — Recursively cascade to second-order dependents, etc. (until W change < threshold or max depth)
  - [ ] RI-20e — Update affected claims' SBT holders: penalty proportional to weight decrease
  - [ ] RI-20f — Test: retract claim → immediate dependent has W reduced by δ
  - [ ] RI-20g — Test: three-level dependency (A → B → C), retract A → C is cascaded
  - [ ] RI-20h — Test: cascade threshold (per field) stops propagation if weight reduction < Θ_field
- [ ] RI-21 — Implement multi-signature governance action handling per wp-v0.2 §7.5: validate k-of-n signatures, enforce timelock, apply parameter changes after timelock
  - [ ] RI-21a — Define `GovernanceAction` enum: `ParameterChange | FieldSchemaAdd | OracleWhitelistUpdate | GovernanceSetRotation`
  - [ ] RI-21b — Implement action parsing from log entry
  - [ ] RI-21c — Verify k-of-n signatures: load genesis governance set, check ≥k valid ed25519 signatures
  - [ ] RI-21d — Check timelock: action must be older than timelock_delay (default 7 days) before taking effect
  - [ ] RI-21e — Apply parameter changes: update governance parameters (e.g., outcome_bonus_coefficient γ)
  - [ ] RI-21f — Apply field schema changes: add new field to schema allowlist
  - [ ] RI-21g — Apply oracle whitelist changes: add/remove oracle sources
  - [ ] RI-21h — Test: 3-of-5 governance action with 3 valid signatures accepted, 2 rejected
  - [ ] RI-21i — Test: timelock enforcement: fresh action is queued but not applied
  - [ ] RI-21j — Test: rotation: governance set itself can be rotated via governance action (self-referential)
- [ ] RI-22 — Implement state snapshot Merkle root computation
  - [ ] RI-22a — Serialize claims in deterministic order (sorted by ClaimId)
  - [ ] RI-22b — Serialize attestations in deterministic order (sorted by (ClaimId, AttesterId))
  - [ ] RI-22c — Serialize SBT records in deterministic order (sorted by DID)
  - [ ] RI-22d — Compute Merkle tree: leaf nodes are blake3-hashes of serialized entries, combine pairwise
  - [ ] RI-22e — Root = merkle_root(all_claims || all_attestations || all_sbts)
  - [ ] RI-22f — Store root in ProtocolState
  - [ ] RI-22g — Unit test: same state produces same root across 100 invocations (determinism)
  - [ ] RI-22h — Unit test: single claim change causes root change
  - [ ] RI-22i — Unit test: reorder claims in input → root unchanged (ensures sort order, not input order, drives tree)
- [ ] RI-23 — Write determinism test: two independent runs over same log produce byte-identical state Merkle roots
  - [ ] RI-23a — Load test log with 20 claims and 30 attestations from fixture
  - [ ] RI-23b — Derive state twice in same process: state1, state2
  - [ ] RI-23c — Assert state1.merkle_root == state2.merkle_root (byte-identical)
  - [ ] RI-23d — Spawn two independent processes running state derivation, compare merkle roots
  - [ ] RI-23e — Regression: if roots differ, pretty-print diff of claims/attestations/sbt to debug
  - [ ] RI-23f — Benchmark: derive state for 1000-claim log in <5s
- [ ] RI-24 — Write multi-operator agreement test: state computed from independent log mirrors agrees
  - [ ] RI-24a — Set up two log mirror instances with identical entries (via sync)
  - [ ] RI-24b — Derive state from log mirror 1 → state1
  - [ ] RI-24c — Derive state from log mirror 2 → state2
  - [ ] RI-24d — Assert state1.merkle_root == state2.merkle_root
  - [ ] RI-24e — Test: if one mirror lags (missing latest entries), states differ (expected)
  - [ ] RI-24f — Document: "Multiple operators running this state-derivation code over the same log must agree on output; disagreement signals misbehavior"

### Governance

- [ ] GV-01 — Define genesis governance set (`governance/genesis.toml`): DIDs, public keys, threshold k, timelock period
  - [ ] GV-01a — Generate 5 ed25519 keypairs for genesis governors (placeholder DIDs: `did:apodokimos:gov-001` through `gov-005`)
  - [ ] GV-01b — Create governance/genesis.toml: `threshold_k = 3`, `timelock_period_secs = 604800` (7 days)
  - [ ] GV-01c — List all 5 public keys in TOML
  - [ ] GV-01d — Document decision: why 3-of-5 threshold, why 7-day timelock
  - [ ] GV-01e — Generate test keypair fixtures in `tests/governance_fixtures/` for integration tests
- [ ] GV-02 — Implement `apodokimos-cli governance sign` — produce a signed governance action record
  - [ ] GV-02a — Define CLI subcommand: `apodokimos governance sign --action <json-file> --key <private-key-hex>`
  - [ ] GV-02b — Parse action JSON: `{ action_type, payload, nonce, timestamp }`
  - [ ] GV-02c — Canonicalize action JSON (deterministic serialization)
  - [ ] GV-02d — Sign with ed25519: `signature = sign(private_key, canonical_json)`
  - [ ] GV-02e — Output signed action: `{ action, signature, signer_did }`
  - [ ] GV-02f — Store signed actions in `governance/actions/` directory
  - [ ] GV-02g — Test: sign same action with 3 different keys → 3 signatures in output
- [ ] GV-03 — Implement `apodokimos-cli governance verify` — verify a k-of-n signed action against `governance/genesis.toml`
  - [ ] GV-03a — Define CLI subcommand: `apodokimos governance verify --action <json-file>`
  - [ ] GV-03b — Load genesis.toml, extract public keys and threshold k
  - [ ] GV-03c — Parse signed action file: extract all signatures and signer DIDs
  - [ ] GV-03d — For each signature: verify ed25519(pub_key, canonical_action_json, sig)
  - [ ] GV-03e — Count valid signatures; output "VALID (3/5 required sigs)" or "INVALID (only 2/5)"
  - [ ] GV-03f — Check timelock: if action is fresh (<7 days old), warn "Action will take effect in X days"
  - [ ] GV-03g — Test: verify action with 3 valid sigs → VALID
  - [ ] GV-03h — Test: verify action with 2 valid sigs → INVALID
  - [ ] GV-03i — Test: tampered action fails verification
- [ ] GV-04 — Document the governance action submission flow in `docs/governance-flow.md`
  - [ ] GV-04a — Write workflow: "Create action JSON → distribute to 3+ governors → each runs `governance sign` → collect sigs → merge into one file → submit to log"
  - [ ] GV-04b — Document action types: `ParameterChange` (e.g., update γ), `FieldSchemaAdd` (e.g., add new field), `OracleWhitelistUpdate`, `GovernanceSetRotation`
  - [ ] GV-04c — Explain timelock: "7-day cooldown before action takes effect; community can exit if they disagree"
  - [ ] GV-04d — Show example: rotating governance set (5 governors → 7 governors)
  - [ ] GV-04e — Security model: "Requires k-of-n signatures; no single governor can unilaterally change protocol"
- [ ] GV-05 — Test: governance set rotation as a governance action (the protocol upgrades its own governance set through its existing governance, per wp-v0.2 §7.5)
  - [ ] GV-05a — Start with 3-of-5 genesis set (5 governors)
  - [ ] GV-05b — Create GovernanceSetRotation action: remove 1 governor, add 2 new ones (7 total)
  - [ ] GV-05c — Have 3 current governors sign the rotation action
  - [ ] GV-05d — Wait past timelock
  - [ ] GV-05e — Apply action via state derivation (RI-21)
  - [ ] GV-05f — Verify governance set is now 4-of-7 (or whatever new threshold)
  - [ ] GV-05g — Create new action with old governors → should fail (no longer in set)
  - [ ] GV-05h — Create new action with new governors → should work (now in set)

---

## Phase 4 — `apodokimos-arweave`

> AGPL-3.0 | Rust

- [ ] A-01 — Create `apodokimos-arweave` crate and add dependencies
  - [ ] A-01a — Init `crates/apodokimos-arweave/Cargo.toml` with workspace members
  - [ ] A-01b — Research: evaluate `arweave-rs` vs `bundlr-sdk` vs custom Arweave HTTP client
  - [ ] A-01c — Document decision in `docs/arweave-implementation-choice.md`
  - [ ] A-01d — Add chosen dependency (latest stable)
  - [ ] A-01e — Add serde, serde_json, blake3 for serialization
  - [ ] A-01f — Test build: `cargo build -p apodokimos-arweave`
- [ ] A-02 — Implement `ClaimUploader::upload(claim: &Claim, wallet: &ArweaveWallet) -> Result<TxId>`
  - [ ] A-02a — Define `ClaimUploader` struct: holds Arweave client and wallet
  - [ ] A-02b — Serialize claim to canonical JSON (from apodokimos-core)
  - [ ] A-02c — Compute blake3 hash of canonical JSON
  - [ ] A-02d — Build Arweave transaction with claim JSON as data
  - [ ] A-02e — Sign transaction with wallet (AR token cost estimation)
  - [ ] A-02f — Submit to Arweave network, wait for confirmation
  - [ ] A-02g — Return TxId on success
  - [ ] A-02h — Error handling: insufficient balance, network timeout, invalid wallet
  - [ ] A-02i — Integration test: upload claim, wait for confirmation, fetch tx from Arweave
- [ ] A-03 — Implement canonical Arweave tags per ARCHITECTURE.md spec
  - [ ] A-03a — Define tag schema: `App-Name: "apodokimos"`, `App-Version: "0.2.0"`, `Content-Type: "application/json"`
  - [ ] A-03b — Add tags: `Claim-Type` (enum: primary-claim, hypothesis, method, result, replication, null-result)
  - [ ] A-03c — Add tags: `Field-Id` (e.g., "clinical-medicine-v0.1"), `Schema-Version` (field schema version)
  - [ ] A-03d — Add tag: `Claim-Hash` (blake3 hex-encoded)
  - [ ] A-03e — Add tag: `Spec-Version-DOI` (e.g., "10.5281/zenodo.19763292", required and immutable per wp-v0.2)
  - [ ] A-03f — Document: each tag is set during upload, immutable after confirmation
  - [ ] A-03g — Test: verify all tags are present in Arweave response
- [ ] A-04 — Implement `ClaimFetcher::fetch(tx_id: &TxId) -> Result<Claim>`
  - [ ] A-04a — Query Arweave for transaction data
  - [ ] A-04b — Extract JSON data from transaction
  - [ ] A-04c — Deserialize to Claim struct
  - [ ] A-04d — Extract tags from transaction metadata
  - [ ] A-04e — Return (Claim, Tags)
  - [ ] A-04f — Error handling: tx not found, invalid JSON, deserialization error
  - [ ] A-04g — Integration test: fetch claim uploaded by A-02
- [ ] A-05 — Implement content hash verification on fetch: reject if hash mismatch
  - [ ] A-05a — After deserializing claim JSON, recompute blake3 hash
  - [ ] A-05b — Compare with `Claim-Hash` tag from Arweave
  - [ ] A-05c — Reject claim if hashes don't match; return error
  - [ ] A-05d — Document: this is "rehash-on-fetch" verification per ARCHITECTURE.md
  - [ ] A-05e — Test: fetch claim, modify cached JSON in mock, verify rejection
- [ ] A-06 — Implement `AttestationUploader::upload(attestation: &Attestation) -> Result<TxId>`
  - [ ] A-06a — Similar to A-02 but for attestations
  - [ ] A-06b — Serialize attestation to JSON
  - [ ] A-06c — Compute blake3, tag, sign, submit
  - [ ] A-06d — Add tags: `Related-Claim-Id` (the claim being attested), `Attestation-Verdict` (enum)
  - [ ] A-06e — Test: upload attestation, fetch back
- [ ] A-07 — Write integration tests against Arweave testnet (arlocal)
  - [ ] A-07a — Set up arlocal instance (in-memory Arweave for testing)
  - [ ] A-07b — Test: upload claim to arlocal, fetch back, verify hash and tags
  - [ ] A-07c — Test: upload attestation, link to claim
  - [ ] A-07d — Test: batch upload 10 claims, verify all 10 can be fetched
  - [ ] A-07e — Cleanup: stop arlocal at test end
- [ ] A-08 — Write `fields/clinical-medicine-v0.1.json` CC0 schema to Arweave at deploy time
  - [ ] A-08a — Read `fields/clinical-medicine-v0.1.json` from repo
  - [ ] A-08b — Upload to Arweave with tags: `Content-Type: "application/json"`, `Field-Schema: "clinical-medicine"`, `Version: "0.1.0"`
  - [ ] A-08c — Store resulting TxId in `docs/arweave-field-schemas.json` (registry)
  - [ ] A-08d — Document: schema is immutable and permanent on Arweave; future versions use new TxIds

### v0.2 additions

- [ ] A-09 — Add `Spec-Version-DOI` tag to Arweave upload schema per wp-v0.2 §9.2 — required and immutable
  - [ ] A-09a — Require `Spec-Version-DOI` parameter in `ClaimUploader::upload()` signature
  - [ ] A-09b — Add tag to transaction: `Spec-Version-DOI: "10.5281/zenodo.19763292"` (or relevant version)
  - [ ] A-09c — Reject upload if DOI is missing or invalid format
  - [ ] A-09d — Test: upload without Spec-Version-DOI → error; with valid DOI → success
- [ ] A-10 — Implement `ClaimFetcher` validation: reject claims whose `Spec-Version-DOI` tag does not match a known whitepaper Version DOI (allowlist maintained by governance)
  - [ ] A-10a — Load allowlist of known Spec-Version-DOI strings from `governance/known-versions.toml`
  - [ ] A-10b — During fetch, extract `Spec-Version-DOI` tag
  - [ ] A-10c — Reject claim if DOI not in allowlist
  - [ ] A-10d — Document: allowlist is governance-controlled and can be updated via GV-04
  - [ ] A-10e — Test: fetch claim with known DOI → success; with unknown DOI → error
- [ ] A-11 — Update `App-Version` tag from `0.1.0` to `0.2.0` to reflect wp-v0.2 schema
  - [ ] A-11a — Change tag constant in code to `0.2.0`
  - [ ] A-11b — Test: new uploads have `App-Version: 0.2.0`
- [ ] A-12 — Test that fetching a v0.1 claim (with no `Spec-Version-DOI` tag) is handled per a documented backward-compatibility policy
  - [ ] A-12a — Mock v0.1 claim (missing Spec-Version-DOI tag) in test
  - [ ] A-12b — Define policy: "Treat absent tag as wp-v0.1 for backward compatibility; emit warning on fetch"
  - [ ] A-12c — Implement policy in ClaimFetcher
  - [ ] A-12d — Document in `docs/spec-version-handling.md`
  - [ ] A-12e — Test: fetch v0.1 claim → success with warning

---

## Phase 5 — `apodokimos-indexer`

> AGPL-3.0 | Rust
>
> NOTE: Indexer scope narrowed in wp-v0.2. Graph reconstruction and W scoring moved to `apodokimos-state`. The indexer's residual role is oracle polling + snapshot publication.

- [x] I-01 — Implement Substrate event subscriber via `subxt` _(deferred with Phase 3-ALT; superseded by I-11 transparency-log subscriber)_
- [x] I-02 — Implement `GraphBuilder`: reconstruct ECG from `ClaimRegistered` + `AttestationRecorded` events _(superseded by `apodokimos-state` per RI-15)_
- [x] I-03 — Implement DAG integrity check: detect and reject cycles _(superseded by RI-16 in `apodokimos-state`)_
- [x] I-04 — Implement `Scorer::compute_all()` — batch W(claim) for all claims in graph _(superseded by RI-19 in `apodokimos-state`)_

### Oracle Integration (w-v0.2)

- [ ] I-05 — Implement `OracleConnector::clinicaltrials(nct_id) -> Result<OracleResult>`
  - [ ] I-05a — Init ClinicalTrials.gov API client (https://clinicaltrials.gov/api/query/)
  - [ ] I-05b — Implement NCT ID query: fetch trial status, enrollment, outcome data
  - [ ] I-05c — Compute O factor: determine if trial is completed, has results, if results match claim
  - [ ] I-05d — Return OracleResult: { nct_id, trial_status, O_score ∈ [0, 1], timestamp }
  - [ ] I-05e — Error handling: NCT not found (404), API timeout, malformed response
  - [ ] I-05f — Caching: cache results for 24h to avoid duplicate queries
  - [ ] I-05g — Unit test: fetch known NCT (NCT02718404), verify result structure
  - [ ] I-05h — Integration test: fetch 5 different NCTs, verify O scores are in [0, 1]
- [ ] I-06 — Implement `OracleConnector::prospero(prospero_id) -> Result<OracleResult>`
  - [ ] I-06a — PROSPERO API client (https://www.crd.york.ac.uk/prospero/api/)
  - [ ] I-06b — Query PROSPERO registry: fetch registration status, review status
  - [ ] I-06c — Compute O factor: if review published and aligns with claim, O > 0
  - [ ] I-06d — Return OracleResult: { prospero_id, review_status, O_score, timestamp }
  - [ ] I-06e — Error handling: PROSPERO ID not found, API timeout
  - [ ] I-06f — Caching: cache results for 7 days (reviews update less frequently)
  - [ ] I-06g — Unit test: fetch known PROSPERO ID, verify result
- [ ] I-07 — Implement additional oracle: OpenAlex DOI-to-policy linkage (wp-v0.2 §3.5 outcome linkage)
  - [ ] I-07a — OpenAlex API client (https://openalex.org/api/)
  - [ ] I-07b — Query DOI: fetch publication, find linked policy/legislation documents
  - [ ] I-07c — Compute O factor: if policy is recent and cites the paper, O > 0
  - [ ] I-07d — Return OracleResult: { doi, linked_policies: Vec<String>, O_score, timestamp }
  - [ ] I-07e — Error handling: DOI not found, no linked policies
  - [ ] I-07f — Unit test: fetch known DOI with policy linkage, verify result

### Snapshot Publication & Reconciliation (v0.2)

- [ ] I-08 — Implement transparency-log subscriber: subscribe to new log entries via the `apodokimos-log` client
  - [ ] I-08a — Create `LogSubscriber` struct holding a log client
  - [ ] I-08b — Implement polling loop: fetch latest STH every 10 seconds
  - [ ] I-08c — On new STH: trigger state derivation (delegate to apodokimos-state)
  - [ ] I-08d — Error handling: log unavailable, network timeout, invalid STH format
  - [ ] I-08e — Unit test: mock log subscriber, verify STH polling interval
- [ ] I-09 — Publish oracle results into the log so they form part of the auditable history (per ARCHITECTURE.md state-derivation flow)
  - [ ] I-09a — Wrap OracleResult in a log entry type: `OracleObservation { claim_id, oracle_source, result_json, timestamp }`
  - [ ] I-09b — Sign oracle observation with indexer's DID/key
  - [ ] I-09c — Submit to transparency log via apodokimos-log client
  - [ ] I-09d — Error handling: log submit failure, invalid signature
  - [ ] I-09e — Retry logic: if submit fails, queue and retry up to 3 times
  - [ ] I-09f — Integration test: publish oracle result, verify it appears in log
- [ ] I-10 — Publish state snapshot Merkle roots into the log for efficient verification by lightweight clients
  - [ ] I-10a — After state derivation completes (from apodokimos-state), get merkle root
  - [ ] I-10b — Create log entry: `StateSnapshot { block_height, merkle_root, timestamp }`
  - [ ] I-10c — Sign snapshot with indexer's DID/key
  - [ ] I-10d — Submit to log
  - [ ] I-10e — Store latest published snapshot in local state
  - [ ] I-10f — Test: snapshot published once per state derivation cycle
- [ ] I-11 — Implement cross-operator reconciliation: detect divergence between independent indexers and flag as misbehavior signal
  - [ ] I-11a — Maintain list of known indexer endpoints (configured in `governance/indexers.toml`)
  - [ ] I-11b — Periodically query other indexers' latest snapshot merkle root
  - [ ] I-11c — Compare: if my snapshot != peer snapshot at same block, flag divergence
  - [ ] I-11d — Emit alert/log message: "Indexer divergence detected with {peer}: my_root {my} vs their_root {their}"
  - [ ] I-11e — Publish divergence record to log as evidence (per transparency principle)
  - [ ] I-11f — Error handling: peer indexer down, timeout, invalid response
  - [ ] I-11g — Unit test: mock two indexers with different snapshots, verify divergence detection
  - [ ] I-11h — Integration test: run 2 indexers on same log, verify they agree (or divergence is detected and logged)

### Indexer Lifecycle & Configuration

- [ ] I-12 — Implement indexer startup and initialization
  - [ ] I-12a — Load configuration: log endpoint, oracle API keys (ClinicalTrials.gov, PROSPERO, OpenAlex), state-derivation operator endpoints
  - [ ] I-12b — Verify connectivity: test log client, test state-derivation operator
  - [ ] I-12c — Initialize: load last known STH, last known state snapshot
  - [ ] I-12d — Emit info log: "Indexer started, last STH: {sth_tree_size}"
- [ ] I-13 — Implement graceful shutdown
  - [ ] I-13a — On SIGTERM or stop signal: finish current log polling cycle
  - [ ] I-13b — Flush any pending oracle results to log
  - [ ] I-13c — Persist latest state to disk (for fast recovery)
  - [ ] I-13d — Close log client connection gracefully
  - [ ] I-13e — Emit info log: "Indexer shutdown complete"
- [ ] I-14 — Write integration tests against local transparency log instance + state-derivation operator
  - [ ] I-14a — Set up test log instance (in-memory or arlocal)
  - [ ] I-14b — Set up test state-derivation operator (in-process)
  - [ ] I-14c — Test: indexer starts, subscribes to log, triggers state derivation on new entry
  - [ ] I-14d — Test: oracle polling queries ClinicalTrials.gov mock, result published to log
  - [ ] I-14e — Test: snapshot merkle root published to log after state derivation
  - [ ] I-14f — Test: two indexers on same log produce same snapshot roots (or divergence is logged)
  - [ ] I-14g — Cleanup: stop both indexer instances and log server
- [ ] I-15 — Benchmark and performance tuning
  - [ ] I-15a — Measure: time to poll log (should be <1s for typical log size)
  - [ ] I-15b — Measure: time to query oracle (ClinicalTrials.gov ~5s, PROSPERO ~5s, OpenAlex ~3s)
  - [ ] I-15c — Measure: end-to-end latency from claim submission to snapshot published (target <60s)
  - [ ] I-15d — Document performance targets in `docs/indexer-performance.md`

---

## Phase 6 — `apodokimos-sdk` + `sdk-ts`

> Apache-2.0

### Rust SDK

- [ ] S-01 — Implement `ApodokimosClient::new(rpc_endpoint, arweave_gateway)` _(rephrase for v0.2: `new(log_endpoint, arweave_gateway, state_operator_endpoint)`)_
- [ ] S-02 — Implement `submit_claim(claim, wallet) -> ClaimId` — uploads to Arweave, submits to log
- [ ] S-03 — Implement `attest(claim_id, verdict, evidence_tx, signer) -> AttestationId`
- [ ] S-04 — Implement `get_score(claim_id) -> ClaimWeight` with Merkle proof verification (via state-derivation operator)
- [ ] S-05 — Implement `get_reputation(did) -> ReputationRecord`
- [ ] S-06 — Implement wasm-bindgen exports for all public SDK methods
- [ ] S-07 — Publish `apodokimos-sdk` v0.1.0 to crates.io

### v0.2 SDK additions

- [ ] S-12 — `submit_claim` takes a `spec_version_doi` parameter and writes it to both the Claim struct and the Arweave `Spec-Version-DOI` tag (R13)
- [ ] S-13 — `get_score` returns the spec_version_doi alongside the weight (so clients can verify which spec was applied)
- [ ] S-14 — Implement multi-operator state query: query several state-derivation operators and require agreement before accepting a result

### TypeScript SDK

- [ ] S-08 — Init `sdk-ts` with pnpm, TypeScript, vitest
- [ ] S-09 — Wrap WASM SDK with idiomatic TypeScript types
- [ ] S-10 — Write TypeScript integration tests against testnet
- [ ] S-11 — Publish `@apodokimos/sdk` v0.1.0 to npm

---

## Phase 7 — `apodokimos-cli`

> AGPL-3.0 | Rust | `clap` v4

- [ ] CL-01 — Implement `apodokimos claim submit --file <claim.json> --wallet <key>`
- [ ] CL-02 — Implement `apodokimos claim attest --id <claim_id> --verdict <supports|contradicts|...>`
- [ ] CL-03 — Implement `apodokimos claim score --id <claim_id>`
- [ ] CL-04 — Implement `apodokimos claim verify --id <claim_id>` — fetch from Arweave + verify hash
- [ ] CL-05 — Implement `apodokimos reputation get --did <did>`
- [ ] CL-06 — Implement `apodokimos field list` — list available field schemas
- [ ] CL-07 — Write CLI integration tests
- [ ] CL-08 — Publish `apodokimos-cli` v0.1.0 to crates.io

### v0.2 additions

- [ ] CL-09 — `apodokimos spec version` — print the protocol's currently-active Version DOI (read from runtime config, validates against allowlist)
- [ ] CL-10 — `apodokimos governance sign --proposal <id>` — produce a k-of-n signature contribution (per GV-02)
- [ ] CL-11 — `apodokimos governance verify --action <id>` — verify a signed governance action (per GV-03)
- [ ] CL-12 — `apodokimos log inclusion --entry <hash>` — verify an entry's inclusion proof
- [ ] CL-13 — `apodokimos log consistency --old-sth <hash> --new-sth <hash>` — verify the log was not rewritten
- [ ] CL-14 — `apodokimos anchor verify --proof <ots>` — verify an OpenTimestamps anchor proof against Bitcoin

---

## Phase 8 — Bootstrap: Clinical Medicine Pilot

- [ ] B-01 — Identify 3–5 clinical researchers willing to register claims outside journal system
- [ ] B-02 — Register first claim on testnet: a PICO claim from existing published trial
- [ ] B-03 — Register contradicting claim: known failed replication of same trial
- [ ] B-04 — Demonstrate penalty propagation: retract base claim, observe score cascade
- [ ] B-05 — Register O factor: link to ClinicalTrials.gov NCT for same trial
- [x] B-06 — Whitepaper anchored on Arweave + DOI on Zenodo (10.5281/zenodo.19583091) — completed at Phase 1 _(wp-v0.1)_
- [ ] B-07 — Register whitepaper as first ECG claim on testnet — _re-pointed to wp-v0.2; tracked under V-08_

### v0.2 additions

- [x] B-08 — Whitepaper wp-v0.2 anchored on Zenodo (Version DOI: 10.5281/zenodo.19763292) as new version of v0.1 record
- [ ] B-09 — Identify 5–10 credentialed reviewers for the v0.2 demonstration; meet wp-v0.2 §11.4 minimum participant set (8–15 total)
- [ ] B-10 — Identify ≥2 independent state-derivation operators; verify they produce identical state Merkle roots over the seed dataset
- [ ] B-11 — Bootstrap dataset: 50 Cochrane systematic-review-derived claims with known replication histories, registered with `Spec-Version-DOI = 10.5281/zenodo.19763292`
- [ ] B-12 — Demonstrate that a claim with O=0 (basic-science, no trial registry) has W > 0 — direct regression test against wp-v0.1's bug B1

---

## Phase 9 — Public Testnet & Audit

- [ ] T-01 — Deploy Apodokimos testnet — _under wp-v0.2: deploy transparency log instance + state-derivation operators + Arweave gateway + OTS anchoring; ARCHITECTURE.md per RI-01–RI-24_
- [ ] T-02 — Commission external security audit _(scope updated for v0.2: transparency log integrity, state-derivation determinism, multi-sig governance correctness, Arweave tag binding integrity)_
- [ ] T-03 — Commission formal verification of SBT non-transferability constraint — verify by absence of `transfer` operation in `apodokimos-state` per R7
- [ ] T-04 — Publish audit report as Apodokimos claim (self-referential)
- [ ] T-05 — Bug bounty program: defined scope, reward in governance SBTs not tokens
- [ ] T-06 — Resolve all critical and high findings before mainnet

---

## Phase 3-ALT — Alternative Implementation: Substrate Parachain (wp-v0.1)

> Preserved in full as historical record and as the primary alternative implementation track per wp-v0.2 §10.4.
> Status: deferred. May be revisited at v1.0 if scale or feature requirements justify migration. Until then, this track is a valid alternative path that is not currently being executed.

### Environment Setup

- [ ] CH-01 — Pin Substrate/Polkadot SDK version in `Cargo.toml`
- [ ] CH-02 — Configure `substrate-node-template` as runtime base
- [ ] CH-03 — Set up local dev chain with `--dev` flag for integration testing

### `pallet-claim-registry` — `v0.1.0`

- [ ] CH-04 — Define `ClaimRecord` storage struct: `{ claim_hash, arweave_tx_id, field_id, submitter, block, status }`
- [ ] CH-05 — Implement `register_claim` extrinsic with deposit mechanism
- [ ] CH-06 — Implement `Claims` storage map: `ClaimId → ClaimRecord`
- [ ] CH-07 — Implement `ClaimRegistered` event
- [ ] CH-08 — Implement deposit refund on first attestation received
- [ ] CH-09 — Write pallet unit tests: register, duplicate rejection, deposit logic
- [ ] CH-10 — Write integration test: full register → attest flow on dev chain

### `pallet-attestation` — `v0.1.0`

- [ ] CH-11 — Implement `attest` extrinsic: `(claim_id, verdict, evidence_arweave_tx)`
- [ ] CH-12 — Enforce reviewer SBT minimum threshold check before recording
- [ ] CH-13 — Implement `Attestations` storage: `(ClaimId, AttesterId) → Attestation`
- [ ] CH-14 — Prevent duplicate attestation from same attester on same claim
- [ ] CH-15 — Implement `AttestationRecorded` event
- [ ] CH-16 — Implement `ClaimRetracted` extrinsic — triggers penalty propagation event
- [ ] CH-17 — Write pallet unit tests: happy path, duplicate rejection, SBT gate

### `pallet-sbt-reputation` — `v0.1.0`

- [ ] CH-18 — Implement `ReputationRecord` storage: `AccountId → { field_scores: BTreeMap<FieldId, u64> }`
- [ ] CH-19 — Implement `mint_initial_sbt` — called on first accepted attestation
- [ ] CH-20 — Implement `increment_score` — called on claim survival event from indexer
- [ ] CH-21 — Implement `apply_penalty` — called on retraction propagation
- [ ] CH-22 — Disable transfer extrinsic at runtime level (SBT non-transferability enforcement)
- [ ] CH-23 — Implement `FieldSBT` query: `get_field_score(account, field_id) -> u64`
- [ ] CH-24 — Write pallet unit tests: mint, increment, penalty, transfer-disabled

### `pallet-governance` — `v0.2.0`

- [ ] CH-25 — Define `Proposal` types: `ParameterChange | FieldSchemaAdd | OracleWhitelistUpdate`
- [ ] CH-26 — Implement quadratic SBT voting: `vote_weight = sqrt(field_sbt_score)` _(field-specific); cross-field uses `sqrt(mean_nonzero(field_scores))` per wp-v0.2 §7.1 correction_
- [ ] CH-27 — Implement proposal lifecycle: `Proposed → Voting → Passed | Rejected → Enacted`
- [ ] CH-28 — Implement quorum threshold as governable parameter
- [ ] CH-29 — Write governance integration tests: proposal, vote, enactment
- [ ] CH-30 — Publish runtime v0.2.0 to testnet

---

## SemVer Milestones Summary

_(Restructured for wp-v0.2; v0.1.0 below targets the new reference implementation, not the Substrate parachain.)_

| Version  | Deliverable                                                                                                                                                                       |
| -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `v0.1.0` | `apodokimos-core` math correction (wp-v0.2 W formula) + reference-implementation skeleton (`apodokimos-log`, `apodokimos-anchor`, `apodokimos-state` stubs with happy-path tests) |
| `v0.2.0` | Reference implementation functional: transparency log integration, OpenTimestamps anchoring, state derivation with §5 cascade and R13 spec-version binding, multi-sig governance  |
| `v0.3.0` | `apodokimos-arweave` integrated with `Spec-Version-DOI` tags + CLI published                                                                                                      |
| `v0.4.0` | SDK (Rust + TypeScript) published with multi-operator state queries                                                                                                               |
| `v0.5.0` | Clinical medicine pilot: 50-claim Cochrane bootstrap registered with `spec_version_doi = 10.5281/zenodo.19763292`; 8–15 participants per wp-v0.2 §11.4                            |
| `v0.9.0` | Public testnet live + external audit complete (transparency-log integrity, state determinism, multi-sig governance correctness)                                                   |
| `v1.0.0` | Mainnet — implementation choice locked based on v0.2 demonstration; governance live; protocol owned by no one                                                                     |

---

## DevSecOps Checklist (Every Release)

- [ ] `cargo audit` — zero known vulnerabilities
- [ ] `cargo deny check` — license compliance (AGPL-3.0, Apache-2.0, CC0 boundary respected)
- [ ] `cargo clippy --deny warnings` — clean
- [ ] `cargo fmt --check` — clean
- [ ] All tests green on CI
- [ ] `CHANGELOG.md` updated with SemVer entry
- [ ] Crate versions bumped in `Cargo.toml`
- [ ] Git tag `vX.Y.Z` on `main`
- [ ] crates.io publish (apodokimos-sdk, apodokimos-cli only — internal crates not published)
- [ ] npm publish (for `sdk-ts`)

### v0.2 additions

- [ ] No placeholder strings (`<TO-BE-ASSIGNED-AT-ANCHOR>` etc.) in `WHITEPAPER/` artifacts (per F-25 lint, wp-v0.2 §1.6)
- [ ] State-derivation determinism check: `apodokimos-state` produces byte-identical Merkle roots across two independent runs (per RI-23)
- [ ] Multi-operator agreement check: state computed from independent log mirrors agrees (per RI-24)
- [ ] No `transfer` symbol in `apodokimos-state` public API (per R7 + RI-18)
- [ ] `WHITEPAPER_v0.2.md` Version DOI in header matches the README's "Current" entry (per F-28)

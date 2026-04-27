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
- [ ] F-23 — Add branch protection rules: require CI green before merge, no force push to `main` *(requires manual GitHub UI configuration)*
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
- [x] P-03 — Formally specify W(claim) = R(t) × D × S × O with typed definitions for each variable *(targets wp-v0.1; superseded by wp-v0.2 §3.1 and tracked under V-04 below)*
- [x] P-04 — Define field-calibrated time-decay function for R(t) per domain class *(targets wp-v0.1; reparameterized as half-life in wp-v0.2; see V-04)*
- [x] P-05 — Define dependency depth D: DAG traversal algorithm specification *(targets wp-v0.1; log-normalized as D̃ in wp-v0.2; see V-04)*
- [x] P-06 — Define survival rate S: ratio of supporting to total non-mentioning attestations *(targets wp-v0.1; Laplace-smoothed in wp-v0.2; see V-04)*
- [x] P-07 — Define O factor: enumerated oracle source types and linkage schema *(targets wp-v0.1; bonus form `(1 + γO)` in wp-v0.2; see V-04)*
- [x] P-08 — Define penalty propagation: retraction event cascades to dependent claims *(targets wp-v0.1; explicit δ persistence in wp-v0.2; see V-04)*
- [x] P-09 — Define SBT reputation score structure: `{ field_id, score, attestation_count, survival_rate }`
- [x] P-10 — Define quadratic SBT voting weight formula *(targets wp-v0.1; cross-field formula corrected in wp-v0.2; see V-05)*
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
  - [x] P-11j — Substrate pallet architecture: storage, extrinsics, events per pallet *(superseded by wp-v0.2 §10; preserved as Alternative A in ARCHITECTURE.md)*
  - [x] P-11k — Bootstrap strategy: clinical medicine pilot rationale and PICO schema
  - [x] P-11l — Security analysis: Sybil, governance capture, oracle manipulation, GDPR
  - [x] P-11m — Anchor `WHITEPAPER.md` on Arweave + Zenodo (DOI for timestamped priority)
  - [ ] P-11n — Register whitepaper as first Apodokimos claim on testnet (protocol validates itself) *(re-pointed to wp-v0.2; see V-08)*

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
- [x] V-03 — Update `WHITEPAPER/WHITEPAPER.md` (wp-v0.1) header to add a "Superseded by" pointer to wp-v0.2 *(metadata-only edit, document bytes preserved)*
- [x] V-04 — Document the wp-v0.2 W(c, t) revision in a `CHANGELOG-WHITEPAPER.md` at repo root: Laplace smoothing for R and S, log-normalized D̃, multiplicative O bonus, explicit δ retraction discount (per wp-v0.2 Appendix D)
- [x] V-05 — Document the wp-v0.2 cross-field voting correction (geomean → arithmetic mean over non-zero) in `CHANGELOG-WHITEPAPER.md`
- [x] V-06 — Document the §1.6 versioning convention in `CHANGELOG-WHITEPAPER.md`: Version DOI for archival, Concept DOI for navigation, placeholder discipline for drafts
- [ ] V-07 — Once Concept DOI is visible on Zenodo's Versions panel, update README to display it explicitly (currently a forward-pointer)
- [ ] V-08 — Register wp-v0.2 as the first ECG claim on testnet, with `spec_version_doi = 10.5281/zenodo.19763292` *(blocks on RI- testnet readiness; supersedes P-11n)*

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

- [x] C-14 — Implement `WeightFunction::compute(claim_id, graph_snapshot) -> ClaimWeight` *(implements wp-v0.1 W; superseded by C-25 implementing wp-v0.2 W)*
- [x] C-15 — Implement R(t) time-decay with field-calibrated half-life *(targets wp-v0.1; superseded by C-22)*
- [x] C-16 — Implement D dependency depth traversal on DAG *(targets wp-v0.1; superseded by C-23)*
- [x] C-17 — Implement S survival rate from attestation set *(targets wp-v0.1; superseded by C-24)*
- [x] C-18 — Implement O factor: typed `OFactorSource` enum + linkage validation *(targets wp-v0.1; superseded by C-26)*
- [x] C-19 — Implement penalty propagation: `propagate_retraction(claim_id, graph) -> Vec<AffectedClaim>` *(targets wp-v0.1; superseded by C-27)*
- [x] C-20 — Write property-based tests with `proptest` for weight function monotonicity *(targets wp-v0.1; superseded by C-30)*

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
- [ ] C-33 — Numerical sanity-check suite: verify §11.4 self-registered whitepaper claim has W > 0 under wp-v0.2 (regression against bug B1)
- [ ] C-34 — Write `CHANGELOG.md` entry for v0.3.0 — annotate all C-21 through C-33 as wp-v0.2 alignment

---

## Phase 3 — Reference Implementation (wp-v0.2 path)
> AGPL-3.0 | Rust | transparency log + Arweave + OpenTimestamps + state-derivation per wp-v0.2 §10.2.
>
> NOTE: The original Phase 3 (Substrate parachain pallets) is preserved in full at the end of this document under "Phase 3-ALT — Alternative Implementation: Substrate Parachain (wp-v0.1)". That track is deferred to v1.0+ as one valid alternative path per wp-v0.2 §10.4.

### `apodokimos-log` — Transparency Log Client

- [ ] RI-01 — Survey transparency log implementations (Trillian, Sigstore Rekor, custom RFC 6962 implementation); pin a choice with documented rationale in `docs/log-impl-decision.md`
- [ ] RI-02 — Pin `apodokimos-log` Cargo.toml dependencies for chosen log (latest stable)
- [ ] RI-03 — Implement `LogClient::submit(entry: SignedEntry) -> Result<InclusionProof>`
- [ ] RI-04 — Implement `LogClient::verify_inclusion(entry, proof, sth) -> bool`
- [ ] RI-05 — Implement `LogClient::verify_consistency(old_sth, new_sth) -> bool`
- [ ] RI-06 — Implement witness co-signature verification: `verify_witness_signatures(sth, witnesses) -> bool`
- [ ] RI-07 — Write integration tests against a local log instance
- [ ] RI-08 — Document the genesis witness set in `governance/witnesses.toml`

### `apodokimos-anchor` — OpenTimestamps Integration

- [ ] RI-09 — Add `opentimestamps` Rust crate dependency
- [ ] RI-10 — Implement `Anchor::batch(sths: &[SignedTreeHead]) -> Result<OtsProof>`
- [ ] RI-11 — Implement `Anchor::verify(proof: &OtsProof, bitcoin_node) -> Result<BlockHeight>`
- [ ] RI-12 — Implement scheduled anchoring driver (configurable cadence, default daily)
- [ ] RI-13 — Write integration tests using OTS calendar testnet

### `apodokimos-state` — Deterministic State Derivation

- [ ] RI-14 — Init `apodokimos-state` crate with workspace dependencies on `apodokimos-core` and `apodokimos-log`
- [ ] RI-15 — Implement `Derivation::state_at(log_state) -> ProtocolState` — pure function reading log entries up to STH
- [ ] RI-16 — Implement DAG acyclicity enforcement at write-time (R5)
- [ ] RI-17 — Implement reputation-gated attestation acceptance (R6)
- [ ] RI-18 — Implement SBT lifecycle without exposing any transfer operation (R7) — explicit unit test asserts no `transfer` symbol exists in the public API
- [ ] RI-19 — Implement W(c, t) computation calling `apodokimos-core` per-claim with that claim's `spec_version_doi` (R13)
- [ ] RI-20 — Implement δ retraction cascade per wp-v0.2 §5.2
- [ ] RI-21 — Implement multi-signature governance action handling per wp-v0.2 §7.5: validate k-of-n signatures, enforce timelock, apply parameter changes after timelock
- [ ] RI-22 — Implement state snapshot Merkle root computation
- [ ] RI-23 — Write determinism test: two independent runs over same log produce byte-identical state Merkle roots
- [ ] RI-24 — Write multi-operator agreement test: state computed from independent log mirrors agrees

### Governance

- [ ] GV-01 — Define genesis governance set (`governance/genesis.toml`): DIDs, public keys, threshold k, timelock period
- [ ] GV-02 — Implement `apodokimos-cli governance sign` — produce a signed governance action record
- [ ] GV-03 — Implement `apodokimos-cli governance verify` — verify a k-of-n signed action against `governance/genesis.toml`
- [ ] GV-04 — Document the governance action submission flow in `docs/governance-flow.md`
- [ ] GV-05 — Test: governance set rotation as a governance action (the protocol upgrades its own governance set through its existing governance, per wp-v0.2 §7.5)

---

## Phase 4 — `apodokimos-arweave`
> AGPL-3.0 | Rust

- [ ] A-01 — Add `arweave-rs` or `bundlr-sdk` dependency (evaluate: latest stable)
- [ ] A-02 — Implement `ClaimUploader::upload(claim: &Claim, wallet: &ArweaveWallet) -> TxId`
- [ ] A-03 — Implement canonical Arweave tags per ARCHITECTURE.md spec
- [ ] A-04 — Implement `ClaimFetcher::fetch(tx_id: &TxId) -> Result<Claim>`
- [ ] A-05 — Implement content hash verification on fetch: reject if hash mismatch
- [ ] A-06 — Implement `AttestationUploader::upload(attestation: &Attestation) -> TxId`
- [ ] A-07 — Write integration tests against Arweave testnet (arlocal)
- [ ] A-08 — Write `fields/clinical-medicine-v0.1.json` CC0 schema to Arweave at deploy time

### v0.2 additions

- [ ] A-09 — Add `Spec-Version-DOI` tag to Arweave upload schema per wp-v0.2 §9.2 — required and immutable
- [ ] A-10 — Implement `ClaimFetcher` validation: reject claims whose `Spec-Version-DOI` tag does not match a known whitepaper Version DOI (allowlist maintained by governance)
- [ ] A-11 — Update `App-Version` tag from `0.1.0` to `0.2.0` to reflect wp-v0.2 schema
- [ ] A-12 — Test that fetching a v0.1 claim (with no `Spec-Version-DOI` tag) is handled per a documented backward-compatibility policy (probably: treat absent tag as v0.1, document in `docs/spec-version-handling.md`)

---

## Phase 5 — `apodokimos-indexer`
> AGPL-3.0 | Rust
>
> NOTE: Indexer scope narrowed in wp-v0.2. Graph reconstruction and W scoring moved to `apodokimos-state`. The indexer's residual role is oracle polling + snapshot publication.

- [x] I-01 — Implement Substrate event subscriber via `subxt` *(deferred with Phase 3-ALT; superseded by I-11 transparency-log subscriber)*
- [x] I-02 — Implement `GraphBuilder`: reconstruct ECG from `ClaimRegistered` + `AttestationRecorded` events *(superseded by `apodokimos-state` per RI-15)*
- [x] I-03 — Implement DAG integrity check: detect and reject cycles *(superseded by RI-16 in `apodokimos-state`)*
- [x] I-04 — Implement `Scorer::compute_all()` — batch W(claim) for all claims in graph *(superseded by RI-19 in `apodokimos-state`)*
- [ ] I-05 — Implement `OracleConnector::clinicaltrials(nct_id) -> OFactorScore`
- [ ] I-06 — Implement `OracleConnector::prospero(prospero_id) -> OFactorScore`
- [ ] I-07 — Implement `MerkleAnchor::snapshot(scores) -> MerkleRoot` + on-chain submission *(reframed: snapshots are now published into the transparency log; see I-13)*
- [ ] I-08 — Implement `ScoreServer` — HTTP API for score queries with Merkle proof responses
- [ ] I-09 — Write indexer integration tests against local dev chain *(rescope: against local transparency log instance + state-derivation operator; see I-14)*
- [ ] I-10 — Benchmark: target <5s score recomputation for 10k claims *(this benchmark belongs to `apodokimos-state` now; tracked under RI-23)*

### v0.2 additions

- [ ] I-11 — Implement transparency-log subscriber: subscribe to new log entries via the `apodokimos-log` client
- [ ] I-12 — Publish oracle results into the log so they form part of the auditable history (per ARCHITECTURE.md state-derivation flow)
- [ ] I-13 — Publish state snapshot Merkle roots into the log for efficient verification by lightweight clients
- [ ] I-14 — Implement cross-operator reconciliation: detect divergence between independent indexers and flag as misbehavior signal

---

## Phase 6 — `apodokimos-sdk` + `sdk-ts`
> Apache-2.0

### Rust SDK

- [ ] S-01 — Implement `ApodokimosClient::new(rpc_endpoint, arweave_gateway)` *(rephrase for v0.2: `new(log_endpoint, arweave_gateway, state_operator_endpoint)`)*
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
- [x] B-06 — Whitepaper anchored on Arweave + DOI on Zenodo (10.5281/zenodo.19583091) — completed at Phase 1 *(wp-v0.1)*
- [ ] B-07 — Register whitepaper as first ECG claim on testnet — *re-pointed to wp-v0.2; tracked under V-08*

### v0.2 additions

- [x] B-08 — Whitepaper wp-v0.2 anchored on Zenodo (Version DOI: 10.5281/zenodo.19763292) as new version of v0.1 record
- [ ] B-09 — Identify 5–10 credentialed reviewers for the v0.2 demonstration; meet wp-v0.2 §11.4 minimum participant set (8–15 total)
- [ ] B-10 — Identify ≥2 independent state-derivation operators; verify they produce identical state Merkle roots over the seed dataset
- [ ] B-11 — Bootstrap dataset: 50 Cochrane systematic-review-derived claims with known replication histories, registered with `Spec-Version-DOI = 10.5281/zenodo.19763292`
- [ ] B-12 — Demonstrate that a claim with O=0 (basic-science, no trial registry) has W > 0 — direct regression test against wp-v0.1's bug B1

---

## Phase 9 — Public Testnet & Audit

- [ ] T-01 — Deploy Apodokimos testnet — *under wp-v0.2: deploy transparency log instance + state-derivation operators + Arweave gateway + OTS anchoring; ARCHITECTURE.md per RI-01–RI-24*
- [ ] T-02 — Commission external security audit *(scope updated for v0.2: transparency log integrity, state-derivation determinism, multi-sig governance correctness, Arweave tag binding integrity)*
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
- [ ] CH-26 — Implement quadratic SBT voting: `vote_weight = sqrt(field_sbt_score)` *(field-specific); cross-field uses `sqrt(mean_nonzero(field_scores))` per wp-v0.2 §7.1 correction*
- [ ] CH-27 — Implement proposal lifecycle: `Proposed → Voting → Passed | Rejected → Enacted`
- [ ] CH-28 — Implement quorum threshold as governable parameter
- [ ] CH-29 — Write governance integration tests: proposal, vote, enactment
- [ ] CH-30 — Publish runtime v0.2.0 to testnet

---

## SemVer Milestones Summary

*(Restructured for wp-v0.2; v0.1.0 below targets the new reference implementation, not the Substrate parachain.)*

| Version | Deliverable |
|---|---|
| `v0.1.0` | `apodokimos-core` math correction (wp-v0.2 W formula) + reference-implementation skeleton (`apodokimos-log`, `apodokimos-anchor`, `apodokimos-state` stubs with happy-path tests) |
| `v0.2.0` | Reference implementation functional: transparency log integration, OpenTimestamps anchoring, state derivation with §5 cascade and R13 spec-version binding, multi-sig governance |
| `v0.3.0` | `apodokimos-arweave` integrated with `Spec-Version-DOI` tags + CLI published |
| `v0.4.0` | SDK (Rust + TypeScript) published with multi-operator state queries |
| `v0.5.0` | Clinical medicine pilot: 50-claim Cochrane bootstrap registered with `spec_version_doi = 10.5281/zenodo.19763292`; 8–15 participants per wp-v0.2 §11.4 |
| `v0.9.0` | Public testnet live + external audit complete (transparency-log integrity, state determinism, multi-sig governance correctness) |
| `v1.0.0` | Mainnet — implementation choice locked based on v0.2 demonstration; governance live; protocol owned by no one |

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

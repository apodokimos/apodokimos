# TODO

Atomic task list following OSS SDLC/DevSecOps. Each task is independently completable.
Format: `[ ] TASK-ID — description`

SemVer milestones marked as `### vX.Y.Z`.

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

- [ ] F-19 — Create `.github/workflows/ci.yml` — `cargo check`, `cargo test`, `cargo clippy --deny warnings`
- [ ] F-20 — Create `.github/workflows/audit.yml` — `cargo audit` on push + weekly schedule
- [ ] F-21 — Create `.github/workflows/deny.yml` — `cargo deny check` (licenses, advisories, bans)
- [ ] F-22 — Create `.github/workflows/fmt.yml` — `cargo fmt --check`
- [ ] F-23 — Add branch protection rules: require CI green before merge, no force push to `main`
- [ ] F-24 — Configure Dependabot for Cargo and npm weekly updates

---

## Phase 1 — Protocol Specification

### Claim Model Formalization

- [x] P-01 — Define `ClaimType` taxonomy: `PrimaryClaim | Hypothesis | Method | Result | Replication | Null`
- [x] P-02 — Define `AttestationVerdict` enum: `Supports | Contradicts | Replicates | Refutes | Mentions`
- [x] P-03 — Formally specify W(claim) = R(t) × D × S × O with typed definitions for each variable
- [x] P-04 — Define field-calibrated time-decay function for R(t) per domain class
- [x] P-05 — Define dependency depth D: DAG traversal algorithm specification
- [x] P-06 — Define survival rate S: ratio of supporting to total non-mentioning attestations
- [x] P-07 — Define O factor: enumerated oracle source types and linkage schema
- [x] P-08 — Define penalty propagation: retraction event cascades to dependent claims
- [x] P-09 — Define SBT reputation score structure: `{ field_id, score, attestation_count, survival_rate }`
- [x] P-10 — Define quadratic SBT voting weight formula
- [ ] P-11 — Write `WHITEPAPER.md` — ECG technical specification (versioned, `wp-v0.1`), covering:
  - [x] P-11a — Abstract: problem statement (IF misalignment, DeSci ontology failure)
  - [x] P-11b — Claim model: formal definition, types, granularity constraints
  - [x] P-11c — W(claim) formal specification: R(t), D, S, O with typed math
  - [x] P-11d — Attestation graph: DAG structure, edge semantics, cycle prevention
  - [x] P-11e — Penalty propagation: retraction cascade formal model
  - [x] P-11f — SBT reputation: mint/increment/burn lifecycle, non-transferability proof
  - [x] P-11g — Governance: quadratic SBT voting, proposal lifecycle, attack surface analysis
  - [x] P-11h — Identity layer: DID integration, ZK credential proof scheme
  - [x] P-11i — Arweave content layer: tagging schema, hash binding, permanence guarantees
  - [x] P-11j — Substrate pallet architecture: storage, extrinsics, events per pallet
  - [x] P-11k — Bootstrap strategy: clinical medicine pilot rationale and PICO schema
  - [x] P-11l — Security analysis: Sybil, governance capture, oracle manipulation, GDPR
  - [x] P-11m — Anchor `WHITEPAPER.md` on Arweave + Zenodo (DOI for timestamped priority)
  - [ ] P-11n — Register whitepaper as first Apodokimos claim on testnet (protocol validates itself)

### Field Schema: Clinical Medicine Bootstrap

- [x] P-12 — Define PICO claim schema (population, intervention, comparator, outcome, effect)
- [x] P-13 — Define O factor oracle mapping: `trial_registry_id → ClinicalTrials.gov NCT`
- [x] P-14 — Define O factor oracle mapping: `prospero_id → PROSPERO registration`
- [x] P-15 — Write `fields/clinical-medicine-v0.1.json` — CC0 schema file
- [x] P-16 — Define field normalization coefficient for clinical medicine (baseline USI calibration)

---

## Phase 2 — `apodokimos-core`
> AGPL-3.0 | `no_std` compatible | MSRV: latest stable

### `v0.1.0`

- [ ] C-01 — Init crate with `Cargo.toml`: `no_std`, `thiserror`, `serde`, `blake3`
- [ ] C-02 — Implement `ClaimId` newtype (blake3 hash of canonical claim content)
- [ ] C-03 — Implement `Claim` struct with all fields per P-01 spec
- [ ] C-04 — Implement `ClaimType` enum per P-01
- [ ] C-05 — Implement `AttestationVerdict` enum per P-02
- [ ] C-06 — Implement `Attestation` struct: `{ claim_id, attester_did, verdict, evidence_tx_id, block }`
- [ ] C-07 — Implement `FieldSchema` trait: `field_id()`, `normalize_score()`, `decay_half_life()`
- [ ] C-08 — Implement `ApodokimosError` with `thiserror`
- [ ] C-09 — Implement canonical JSON serialization for `Claim` (deterministic, CC0 schema)
- [ ] C-10 — Implement `ClaimHash::compute(claim: &Claim) -> ClaimId`
- [ ] C-11 — Write unit tests: hash stability, serialization round-trip, enum exhaustiveness
- [ ] C-12 — Write `CHANGELOG.md` entry for v0.1.0
- [ ] C-13 — Publish `apodokimos-core` v0.1.0 to crates.io

### `v0.2.0`

- [ ] C-14 — Implement `WeightFunction::compute(claim_id, graph_snapshot) -> ClaimWeight`
- [ ] C-15 — Implement R(t) time-decay with field-calibrated half-life
- [ ] C-16 — Implement D dependency depth traversal on DAG
- [ ] C-17 — Implement S survival rate from attestation set
- [ ] C-18 — Implement O factor: typed `OFactorSource` enum + linkage validation
- [ ] C-19 — Implement penalty propagation: `propagate_retraction(claim_id, graph) -> Vec<AffectedClaim>`
- [ ] C-20 — Write property-based tests with `proptest` for weight function monotonicity
- [ ] C-21 — Publish `apodokimos-core` v0.2.0

---

## Phase 3 — `apodokimos-chain`
> AGPL-3.0 | Substrate FRAME | Rust

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
- [ ] CH-26 — Implement quadratic SBT voting: `vote_weight = sqrt(field_sbt_score)`
- [ ] CH-27 — Implement proposal lifecycle: `Proposed → Voting → Passed | Rejected → Enacted`
- [ ] CH-28 — Implement quorum threshold as governable parameter
- [ ] CH-29 — Write governance integration tests: proposal, vote, enactment
- [ ] CH-30 — Publish runtime v0.2.0 to testnet

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

---

## Phase 5 — `apodokimos-indexer`
> AGPL-3.0 | Rust

- [ ] I-01 — Implement Substrate event subscriber via `subxt`
- [ ] I-02 — Implement `GraphBuilder`: reconstruct ECG from `ClaimRegistered` + `AttestationRecorded` events
- [ ] I-03 — Implement DAG integrity check: detect and reject cycles
- [ ] I-04 — Implement `Scorer::compute_all()` — batch W(claim) for all claims in graph
- [ ] I-05 — Implement `OracleConnector::clinicaltrials(nct_id) -> OFactorScore`
- [ ] I-06 — Implement `OracleConnector::prospero(prospero_id) -> OFactorScore`
- [ ] I-07 — Implement `MerkleAnchor::snapshot(scores) -> MerkleRoot` + on-chain submission
- [ ] I-08 — Implement `ScoreServer` — HTTP API for score queries with Merkle proof responses
- [ ] I-09 — Write indexer integration tests against local dev chain
- [ ] I-10 — Benchmark: target <5s score recomputation for 10k claims

---

## Phase 6 — `apodokimos-sdk` + `sdk-ts`
> Apache-2.0

### Rust SDK

- [ ] S-01 — Implement `ApodokimosClient::new(rpc_endpoint, arweave_gateway)`
- [ ] S-02 — Implement `submit_claim(claim, wallet) -> ClaimId`
- [ ] S-03 — Implement `attest(claim_id, verdict, evidence_tx, signer) -> AttestationId`
- [ ] S-04 — Implement `get_score(claim_id) -> ClaimWeight` with Merkle proof verification
- [ ] S-05 — Implement `get_reputation(did) -> ReputationRecord`
- [ ] S-06 — Implement wasm-bindgen exports for all public SDK methods
- [ ] S-07 — Publish `apodokimos-sdk` v0.1.0 to crates.io

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

---

## Phase 8 — Bootstrap: Clinical Medicine Pilot

- [ ] B-01 — Identify 3–5 clinical researchers willing to register claims outside journal system
- [ ] B-02 — Register first claim on testnet: a PICO claim from existing published trial
- [ ] B-03 — Register contradicting claim: known failed replication of same trial
- [ ] B-04 — Demonstrate penalty propagation: retract base claim, observe score cascade
- [ ] B-05 — Register O factor: link to ClinicalTrials.gov NCT for same trial
- [ ] B-06 — Publish case study as first Apodokimos whitepaper — anchored on Arweave, registered as first ECG claim
- [ ] B-07 — Pilot whitepaper is itself an Apodokimos claim — protocol bootstraps its own legitimacy

---

## Phase 9 — Public Testnet & Audit

- [ ] T-01 — Deploy Apodokimos parachain to Rococo testnet
- [ ] T-02 — Commission external security audit of all four pallets
- [ ] T-03 — Commission formal verification of SBT non-transferability constraint
- [ ] T-04 — Publish audit report as Apodokimos claim (self-referential)
- [ ] T-05 — Bug bounty program: defined scope, reward in governance SBTs not tokens
- [ ] T-06 — Resolve all critical and high findings before mainnet

---

## SemVer Milestones Summary

| Version | Deliverable |
|---|---|
| `v0.1.0` | `apodokimos-core` claim model + `pallet-claim-registry` + `pallet-attestation` + `pallet-sbt-reputation` |
| `v0.2.0` | W(claim) scoring in core + `pallet-governance` + indexer with oracle connectors |
| `v0.3.0` | Arweave content layer integrated + CLI published |
| `v0.4.0` | SDK (Rust + TypeScript) published |
| `v0.5.0` | Clinical medicine pilot complete + bootstrap case study on Arweave |
| `v0.9.0` | Public testnet live + external audit complete |
| `v1.0.0` | Mainnet — governance live, protocol owned by no one |

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
- [ ] crates.io publish (for publishable crates)
- [ ] npm publish (for `sdk-ts`)

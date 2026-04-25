# Architecture

This document maps the Apodokimos whitepaper specification (wp-v0.2, [10.5281/zenodo.19763292](https://doi.org/10.5281/zenodo.19763292)) to a concrete implementation. The whitepaper defines the protocol's mechanism requirements (R1–R13); this document describes the reference implementation that satisfies them. Alternative implementations satisfying the same requirements are equally valid (see §10.4 of the whitepaper, and the "Alternative Implementations" section at the end of this document).

---

## Guiding Constraints

1. **No single owner** — protocol must be ungovernable by any institution or individual
2. **Claim-atomic** — paper is evidence, not the product; claim is the unit
3. **Immutable attestations, mutable scores** — what was attested cannot be altered; scores recompute as evidence accumulates
4. **Reputation as identity, not capital** — SBTs are non-transferable; governance cannot be purchased
5. **Verifiable derived state** — graph scoring is a deterministic pure function of the history; multiple operators must produce identical outputs
6. **Stack-agnostic** *(new in v0.2)* — the protocol is defined by its mechanism requirements, not by its implementation. The reference implementation is one valid path; alternatives are listed at the end of this document
7. **Specification coherence over time** *(new in v0.2)* — every claim is bound to the Version DOI of the specification under which it was registered (whitepaper §1.6); past claims retain their original semantics indefinitely

---

## System Layers

```
┌─────────────────────────────────────────────────────────────┐
│                       Client Layer                          │
│   apodokimos-cli  │  apodokimos-sdk  │  web (future)        │
├─────────────────────────────────────────────────────────────┤
│              Indexer / Oracle Layer                         │
│              apodokimos-indexer                             │
│   oracle polling (ClinicalTrials.gov, PROSPERO)             │
│   snapshot publication of Merkle roots                      │
├─────────────────────────────────────────────────────────────┤
│                State Derivation Layer                       │
│              apodokimos-state                               │
│   reads history → applies §3 W(c,t) → applies §5 cascade    │
│   reads spec_version_doi per claim → applies historical     │
│     schema and rules per R13                                │
│   pure function · deterministic · independently verifiable  │
├─────────────────────────────────────────────────────────────┤
│            History Layer  (transparency log)                │
│   apodokimos-log                                            │
│   append-only Merkle log · inclusion + consistency proofs   │
│   multi-operator mirroring · independent witness co-signing │
│   (Certificate Transparency / Rekor lineage)                │
├─────────────────────────────────────────────────────────────┤
│           Timestamp Anchor  (OpenTimestamps)                │
│   apodokimos-anchor                                         │
│   batches log STHs → anchors in Bitcoin                     │
│   external, non-Apodokimos durability guarantee             │
├─────────────────────────────────────────────────────────────┤
│              Content Layer  (Arweave)                       │
│   apodokimos-arweave                                        │
│   claim text · evidence · review payloads                   │
│   immutable · permanent · content-addressed                 │
│   Spec-Version-DOI tag binds claim to spec it was made under│
├─────────────────────────────────────────────────────────────┤
│                  Protocol Core                              │
│              apodokimos-core                                │
│   claim model · weight function · field schemas             │
│   error types · traits · canonical serialization            │
└─────────────────────────────────────────────────────────────┘
```

The state-derivation layer is the conceptual replacement for v0.1's Substrate-pallet attestation layer. It enforces R5 (DAG acyclicity at write), R6 (reputation-gated writes), R7 (SBT non-transferability), R8 (deterministic computation), R9 (verifiable derived state), R10 (governance binding), and R13 (spec-version coherence).

---

## Crate / Package Structure

```
apodokimos/
├── apodokimos-core/          # AGPL-3.0 | Rust | no_std compatible
│   ├── src/
│   │   ├── claim.rs          # Claim struct (incl. spec_version_doi), ClaimId, ClaimHash
│   │   ├── weight.rs         # W(c,t) = R × D̃ × S × (1 + γO) × δ  per wp-v0.2 §3
│   │   ├── attestation.rs    # Attestation types: Supports|Contradicts|Replicates|Refutes|Mentions
│   │   ├── field.rs          # Field schema, normalization coefficients
│   │   ├── version_doi.rs    # VersionDOI newtype + parsing/validation
│   │   └── error.rs
│   └── Cargo.toml
│
├── apodokimos-state/         # AGPL-3.0 | Rust  -- NEW IN v0.2 (replaces apodokimos-chain)
│   ├── src/
│   │   ├── derivation.rs     # Deterministic state-derivation program
│   │   ├── graph.rs          # ECG graph reconstruction from log events
│   │   ├── scorer.rs         # W(c,t) computation, per-claim spec-version application
│   │   ├── retraction.rs     # δ-based penalty propagation per wp-v0.2 §5
│   │   ├── sbt.rs            # SBT mint/increment/penalty (no transfer code path; R7)
│   │   ├── governance.rs     # k-of-n signed governance actions per wp-v0.2 §7.5
│   │   └── snapshot.rs       # Merkle-root snapshot of derived state
│   └── Cargo.toml
│
├── apodokimos-log/           # AGPL-3.0 | Rust  -- NEW IN v0.2
│   ├── src/
│   │   ├── client.rs         # Submit/fetch entries to/from transparency log
│   │   ├── inclusion.rs      # Inclusion proof verification
│   │   ├── consistency.rs    # Consistency proof verification (no rewrite)
│   │   └── witness.rs        # Witness co-signature verification (split-view defense)
│   └── Cargo.toml
│
├── apodokimos-anchor/        # AGPL-3.0 | Rust  -- NEW IN v0.2
│   ├── src/
│   │   ├── opentimestamps.rs # Batch log STHs → submit to OTS calendar
│   │   ├── verify.rs         # Verify OTS proofs against Bitcoin
│   │   └── schedule.rs       # Periodic anchoring driver
│   └── Cargo.toml
│
├── apodokimos-arweave/       # AGPL-3.0 | Rust
│   ├── src/
│   │   ├── upload.rs         # Claim content → Arweave tx (with Spec-Version-DOI tag)
│   │   ├── fetch.rs          # Arweave tx → Claim content; rehash-on-fetch verification
│   │   └── schema.rs         # Canonical claim JSON schema (CC0)
│   └── Cargo.toml
│
├── apodokimos-indexer/       # AGPL-3.0 | Rust  -- SCOPE NARROWED IN v0.2
│   ├── src/
│   │   ├── oracle.rs         # O factor: ClinicalTrials.gov, PROSPERO connectors
│   │   ├── publisher.rs      # Publishes oracle results + state snapshots into log
│   │   └── reconcile.rs      # Cross-checks oracle outputs across operators
│   └── Cargo.toml
│   # NOTE: graph reconstruction and W scoring moved to apodokimos-state.
│   # The indexer's role in v0.2 is specifically external-data ingress and
│   # snapshot publication. State derivation is verifiable independently
│   # of any single indexer operator.
│
├── apodokimos-sdk/           # Apache-2.0 | Rust + WASM bindings
│   ├── src/
│   │   ├── client.rs         # Submit claims, query scores, fetch attestations
│   │   └── wasm.rs           # wasm-bindgen exports for TypeScript consumers
│   └── Cargo.toml
│
├── apodokimos-cli/           # AGPL-3.0 | Rust
│   ├── src/
│   │   └── main.rs           # claim submit | attest | score | verify
│   └── Cargo.toml
│
├── sdk-ts/                   # Apache-2.0 | TypeScript | pnpm
│   ├── src/
│   │   └── index.ts          # TypeScript wrapper over WASM SDK
│   └── package.json
│
├── fields/                   # CC0-1.0 | versioned field schemas
│   └── clinical-medicine-v0.1.json
│
├── governance/               # AGPL-3.0 | governance set definition
│   ├── genesis.toml          # Genesis governance set: DIDs, public keys, threshold k
│   └── actions/              # Signed governance action records (post-genesis)
│
├── Cargo.toml                # Workspace root
├── README.md
├── ARCHITECTURE.md
├── TODO.md
├── WHITEPAPER/               # versioned whitepaper artifacts
│   ├── WHITEPAPER.md         # wp-v0.1 (preserved)
│   └── WHITEPAPER_v0.2.md    # current
└── LICENSE-AGPL  LICENSE-APACHE  LICENSE-CC0
```

---

## Reference Implementation: Layer Responsibilities

This section maps each layer to its R1–R13 obligations and key operations. See whitepaper §10.2 for the formal description.

### `apodokimos-core` — Protocol Core

The foundation crate. Defines types and pure functions used by all other crates. `no_std`-compatible so it can be used in constrained environments (WASM, embedded, future native chain implementations).

**Key types:**
- `Claim` — including the new `spec_version_doi: VersionDOI` field per wp-v0.2 §2.2
- `Attestation` — typed verdict (`Supports | Contradicts | Replicates | Refutes | Mentions`)
- `ClaimWeight` — output of W computation
- `VersionDOI` — newtype, validated DOI string per §1.6

**Key functions:**
- `ClaimHash::compute(&Claim) -> ClaimId` — blake3 of canonical JSON
- `WeightFunction::compute(...)` — implements W = R × D̃ × S × (1 + γO) × δ per wp-v0.2 §3, with Laplace smoothing, log-normalized D̃, multiplicative O bonus, and explicit δ
- `propagate_retraction(...)` — δ-based cascade per wp-v0.2 §5.2

### `apodokimos-state` — State Derivation

Deterministic Rust program that reads the transparency log, applies the protocol's rules, and produces the current protocol state. This is the layer that satisfies R5, R6, R7, R8, R9, R10, and R13. Multiple operators run it independently; their outputs must agree.

**Operations:**

```
state_at(log_state) -> ProtocolState
  reads:    every entry up to log_state's STH
  applies:  per claim, the schema and scoring rules from claim.spec_version_doi
  enforces: DAG acyclicity at write (R5)
  enforces: SBT threshold gating for attestations (R6)
  enforces: governance actions after timelock (R10)
  produces: SBT records, claim weights, Merkle root over derived state
```

**Properties:**
- Pure function of the log contents — given the same log, all honest operators produce identical state
- No `transfer` operation exists for SBTs anywhere in the code (R7 — non-transferability by absence, not by gating)
- Spec-version coherence: claim registered under wp-v0.2 is scored under wp-v0.2 rules even after wp-v0.5 ships (R13)

### `apodokimos-log` — History Layer

Client for an append-only transparency log in the Certificate Transparency / Rekor lineage (RFC 6962).

**Operations:**
- `submit(entry, signature) -> InclusionProof`
- `verify_inclusion(entry, proof, sth)` — Merkle proof against a Signed Tree Head
- `verify_consistency(old_sth, new_sth)` — verifies the log was not rewritten
- `verify_witness_signatures(sth, witness_set)` — multi-witness co-signing defends against split-view attacks

**Why a transparency log rather than a blockchain.** Apodokimos at typical write rates does not have conflicting-write conditions that require Byzantine consensus. A transparency log provides append-only verifiability, content addressing, and cryptographic proofs of inclusion/consistency without imposing block production, validator economics, or a native token (which would reintroduce capital into a layer beneath reputation-weighted governance — see whitepaper §10.3).

### `apodokimos-anchor` — Timestamp Layer

OpenTimestamps integration. Periodically batches log Signed Tree Heads and anchors them into Bitcoin via OTS calendar servers.

**Operations:**
- `batch_anchor(sths) -> OtsProof`
- `verify_against_bitcoin(ots_proof, bitcoin_node) -> Result<BlockHeight>`

The Bitcoin anchor is an external, non-Apodokimos durability guarantee. Any reader with a Bitcoin node can verify that a log entry existed before a given block — without trusting Apodokimos, the log operator, or any other party.

### `apodokimos-arweave` — Content Layer

Unchanged in role from wp-v0.1. Updated in wp-v0.2 to require the `Spec-Version-DOI` tag on every upload (whitepaper §9.2).

### `apodokimos-indexer` — Indexer / Oracle Layer

**Scope narrowed in v0.2.** In wp-v0.1, the indexer was responsible for graph reconstruction and W scoring. In wp-v0.2, those responsibilities live in `apodokimos-state`, where they are pure-function-of-log and independently verifiable. The indexer's residual role is specifically:

- **Oracle polling** — periodic queries to ClinicalTrials.gov, PROSPERO, OpenAlex
- **Oracle result publication** — publishes results into the transparency log so the state-derivation program can consume them
- **Snapshot publication** — periodically publishes Merkle roots of derived state into the log (for efficient verification by lightweight clients)
- **Cross-operator reconciliation** — flags divergence between independent indexers as a signal of misbehavior

### `apodokimos-sdk` and `apodokimos-cli` — Client Layer

Unchanged in role. Updated to consume the v0.2 architecture (transparency log + Arweave + state queries against any operator).

---

## Arweave Content Layer

Claim content is stored as a JSON transaction on Arweave with tags:

```json
{
  "App-Name":          "apodokimos",
  "App-Version":       "0.2.0",
  "Content-Type":      "application/json",
  "Claim-Type":        "primary-claim | hypothesis | method | result | replication | null-result",
  "Field-Id":          "<field_schema_id>",
  "Claim-Hash":        "<blake3_hash_of_canonical_json>",
  "Schema-Version":    "<field_schema_version>",
  "Spec-Version-DOI":  "<version_doi_of_whitepaper_in_force_at_registration>"
}
```

The `Claim-Hash` tag enables content verification on fetch: the client recomputes blake3 of the downloaded content and rejects on mismatch. The `Spec-Version-DOI` tag binds the claim to the specification version that defines its schema and scoring rules; the state-derivation program reads this tag to apply the correct historical rules per R13. Both tags are required and immutable.

---

## State Derivation

The state-derivation program subscribes to the transparency log, reconstructs the ECG, and computes W(c, t) for every claim. State snapshots are Merkle-anchored back into the log periodically.

```
log entries (claims + attestations + governance actions)
  → apodokimos-state graph engine
    → reads claim.spec_version_doi for each claim
    → applies that version's schema and W formula
    → applies §5 retraction cascade through δ
    → produces ProtocolState (SBT records + claim weights)
    → Merkle root of state → published into log via apodokimos-anchor
    → state proofs queryable by SDK
```

The O factor oracle initially supports:
- ClinicalTrials.gov (NCT linkage for clinical pilot)
- PROSPERO (systematic review protocol registration)
- DOI-to-policy linkage via OpenAlex API

Oracle whitelisting is governed via the multi-signature governance mechanism (whitepaper §7.5). Oracle results are themselves published into the transparency log so they are part of the auditable history.

---

## Identity and DID

Submitter and reviewer identity uses W3C Decentralized Identifiers (W3C, 2022). No real-name requirement. ZK-proof of credential allows anonymous-but-credentialed participation — e.g., prove you hold a medical license without revealing identity.

DID method: `did:apodokimos:<account_id>` *(generalized in v0.2 from v0.1's `did:substrate:apodokimos`)*.

The specific ZK scheme (Groth16 or PLONK) is selected at the time the DID service is implemented (whitepaper §8.2).

---

## Bootstrap Domain: Clinical Medicine

The first field schema deployed targets PICO-structured clinical claims:

```
Claim := {
  spec_version_doi:  VersionDOI,    // immutable; binds claim to wp version
  population:        String,
  intervention:      String,
  comparator:        String,
  outcome:           String,
  effect_direction:  Positive | Negative | Null | Mixed,
  effect_size:       Option<f64>,
  confidence_interval: Option<(f64, f64)>,
  trial_registry_id: Option<String>,    // O factor anchor
  prospero_id:       Option<String>     // O factor anchor
}
```

PICO structure is already standardized in evidence-based medicine (Richardson, Wilson, Nishikawa, & Hayward, 1995), making claim granularity well-defined. Trial registry IDs provide a direct O factor oracle without requiring subjective judgment.

The bootstrap pilot targets 50 claims from Cochrane systematic reviews. The minimum participant set for a credible v0.2 demonstration is 8–15 real participants (whitepaper §11.4): 1–3 submitters, 5–10 attesters, ≥2 independent state-derivation operators, 1 oracle operator per source.

---

## Versioning and Citation

This repository follows the citation discipline established in whitepaper §1.6:

- **Version DOI** — immutable per-version DOI; used in all archival contexts (claim metadata's `spec_version_doi`, source code comments, academic citations, anchored documents)
- **Concept DOI** — moving pointer to latest version; used in this README's "current spec" link, in onboarding documentation, and for navigation within Zenodo

Drafts of future whitepaper versions use placeholder strings until anchoring; CI rejects merges to `main` containing placeholder strings in published whitepaper artifacts (per whitepaper §1.6 documentation hygiene rule).

---

## Security Considerations

| Vector | Mitigation |
|---|---|
| Sybil reviewers | Field SBT minimum threshold; ZK credential proof; SBT accumulation cost per identity (whitepaper §12.1) |
| Governance plutocracy | Quadratic SBT voting; SBTs non-transferable; field-specific weights |
| Claim spam | Registration deposit (refunded on first attestation); field schema validation |
| Oracle manipulation | Multi-source O factor; governed oracle whitelist; oracle results in audit log |
| GDPR right to erasure | No personal data on-chain; claim content on Arweave, removable by uploader |
| **Protocol capture** *(generalized in v0.2)* | AGPL-3.0; cross-layer independence (Arweave, transparency log, Bitcoin anchor are independent systems); forkable canonical history; multi-sig governance prevents unilateral parameter changes |

The "Chain capture" row from v0.1 has been replaced by the more general "Protocol capture" row, because v0.2 does not commit to a single chain. The capture-resistance argument now rests on the cross-layer property: capturing the protocol would require simultaneous capture of Arweave, the transparency log network, and the Bitcoin timestamp anchor — three independent systems with separate governance and economics.

---

## Alternative Implementations

The reference implementation above is one valid path through the requirements R1–R13. wp-v0.2 §10.4 enumerates alternatives equally valid by specification. The most fully-developed alternative is the v0.1 Substrate parachain design, preserved here as historical record.

### Alternative A: Substrate Parachain *(wp-v0.1 design)*

A Substrate-based parachain implementing four custom FRAME pallets. This was the wp-v0.1 reference architecture.

**Pallets:**

- `pallet-claim-registry` — extrinsic `register_claim(claim_hash, arweave_tx_id, field_id, submitter_did)`; storage `Claims: map ClaimId => ClaimRecord`; events `ClaimRegistered`. Claim text never on-chain — only content-addressed hash + Arweave pointer.

- `pallet-attestation` — extrinsic `attest(claim_id, verdict, evidence_arweave_tx)`; verdict enum `Supports | Contradicts | Replicates | Refutes | Mentions`; requires reviewer holds field SBT above threshold; storage `Attestations: map (ClaimId, AttesterId) => Attestation`.

- `pallet-sbt-reputation` — non-transferable by runtime enforcement (transfer extrinsic absent from dispatchable function set, satisfying R7); mint on first accepted attestation; increment on claim-survival events; decrement on retraction cascade; storage `Reputation: map AccountId => ReputationRecord`.

- `pallet-governance` — proposals by any SBT holder above threshold; vote weight `sqrt(field_sbt_score)` for field-specific, `sqrt(mean_nonzero(field_scores))` for cross-field per wp-v0.2 §7.1; scope: protocol parameter changes, field schema additions, oracle whitelist updates.

**Trade-offs:** Forkless runtime upgrades; mature ecosystem; but heavier infrastructure footprint, relay-chain coupling (parachain slot depends on Polkadot governance), and the validator-set economics introduce a token-weighted layer beneath the SBT-weighted governance.

This alternative remains a valid future migration target if v1.0 scale demands it. Per whitepaper §10.5, the v0.2 reference implementation can be carried forward into a v1.0 parachain implementation by replaying the transparency log into the parachain's genesis state.

### Alternative B: Cosmos SDK + CometBFT

A sovereign chain implementing protocol rules as Cosmos modules. Properties comparable to Substrate with smaller surface area and no relay-chain dependency; upgrades are coordinated hard forks rather than forkless. Same token-economics tension as Alternative A.

### Alternative C: Hybrid

Transparency log for history and Arweave for content (as in the reference implementation) combined with a narrow-scope chain (Substrate or Cosmos) for SBT state and governance only. Preserves the minimal-infrastructure property of the reference implementation while providing chain-enforced governance if multi-signature becomes inadequate at v1.0 scale.

---

## Summary

The reference implementation composes existing, well-understood technology categories — transparency log, Arweave, OpenTimestamps, deterministic Rust state derivation — to satisfy R1–R13 with substantially less infrastructure than a blockchain. This is the "right tool for the right problem" answer for wp-v0.2's scope (the clinical-medicine bootstrap, 8–15 participants, ~50 seed claims).

For v1.0 mainnet scope, the implementation choice is explicitly deferred (whitepaper §10.5). The protocol's versioning independence (whitepaper Appendix B) ensures that a future v1.0 architectural pivot does not invalidate v0.2 history: claims registered under wp-v0.2 carry their `spec_version_doi` and continue to be scored under wp-v0.2 rules, regardless of how the implementation evolves.

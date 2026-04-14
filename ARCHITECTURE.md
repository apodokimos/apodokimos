# Architecture

## Guiding Constraints

1. **No single owner** — protocol must be ungovernable by any institution or individual
2. **Claim-atomic** — paper is evidence, not the product; claim is the unit
3. **Immutable attestations, mutable scores** — what was attested cannot be altered; scores recompute as evidence accumulates
4. **Reputation as identity, not capital** — SBTs are non-transferable; governance cannot be purchased
5. **Verifiable off-chain computation** — graph scoring stays off-chain for performance; results are Merkle-anchored on-chain

---

## System Layers

```
┌─────────────────────────────────────────────────────────┐
│                   Client Layer                          │
│   apodokimos-cli  │  apodokimos-sdk  │  web (future)    │
├─────────────────────────────────────────────────────────┤
│                 Indexer / Graph Engine                  │
│              apodokimos-indexer                         │
│   W(claim) scoring · field normalization · O oracles    │
├─────────────────────────────────────────────────────────┤
│              Content Layer  (Arweave)                   │
│   apodokimos-arweave                                    │
│   claim text · evidence · review payloads               │
│   immutable · permanent · content-addressed             │
├─────────────────────────────────────────────────────────┤
│            Attestation Layer  (Substrate)               │
│   apodokimos-chain                                      │
│   pallet-claim-registry · pallet-attestation            │
│   pallet-sbt-reputation · pallet-governance             │
│   claim hashes · attestation records · SBTs             │
├─────────────────────────────────────────────────────────┤
│                  Protocol Core                          │
│              apodokimos-core                            │
│   claim model · weight function · field schemas         │
│   error types · traits · serialization                  │
└─────────────────────────────────────────────────────────┘
```

---

## Crate / Package Structure

```
apodokimos/
├── apodokimos-core/          # AGPL-3.0 | Rust | no_std compatible
│   ├── src/
│   │   ├── claim.rs          # Claim struct, ClaimId, ClaimHash
│   │   ├── weight.rs         # W(claim) = R(t) × D × S × O
│   │   ├── attestation.rs    # Attestation types: supports|contradicts|replicates|refutes
│   │   ├── field.rs          # Field schema, normalization coefficients
│   │   └── error.rs
│   └── Cargo.toml
│
├── apodokimos-chain/         # AGPL-3.0 | Rust | Substrate FRAME pallets
│   ├── pallets/
│   │   ├── claim-registry/   # Register claim hash + submitter DID
│   │   ├── attestation/      # Record attestation + reviewer SBT check
│   │   ├── sbt-reputation/   # Mint/burn non-transferable reputation tokens
│   │   └── governance/       # Epistemic-weighted voting
│   ├── runtime/
│   └── Cargo.toml
│
├── apodokimos-arweave/       # AGPL-3.0 | Rust
│   ├── src/
│   │   ├── upload.rs         # Claim content → Arweave tx
│   │   ├── fetch.rs          # Arweave tx → Claim content
│   │   └── schema.rs         # Canonical claim JSON schema (CC0)
│   └── Cargo.toml
│
├── apodokimos-indexer/       # AGPL-3.0 | Rust
│   ├── src/
│   │   ├── graph.rs          # ECG graph construction from on-chain events
│   │   ├── scorer.rs         # W(claim) computation
│   │   ├── oracle.rs         # O factor: ClinicalTrials.gov, PROSPERO connectors
│   │   └── merkle.rs         # Anchor score snapshots on-chain
│   └── Cargo.toml
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
├── Cargo.toml                # Workspace root
├── README.md
├── ARCHITECTURE.md
└── TODO.md
```

---

## Substrate Parachain Design

### Pallets

**`pallet-claim-registry`**
- Extrinsics: `register_claim(claim_hash, arweave_tx_id, field_id, submitter_did)`
- Storage: `Claims: map ClaimId => ClaimRecord`
- Events: `ClaimRegistered { claim_id, submitter, block }`
- No claim text on-chain — only content-addressed hash + Arweave pointer

**`pallet-attestation`**
- Extrinsics: `attest(claim_id, verdict, evidence_arweave_tx)`
- Verdict enum: `Supports | Contradicts | Replicates | Refutes | Mentions`
- Requires: reviewer holds field SBT (minimum reputation threshold)
- Storage: `Attestations: map (ClaimId, AttesterId) => Attestation`
- Events: `AttestationRecorded { claim_id, attester, verdict }`

**`pallet-sbt-reputation`**
- SBTs are non-transferable by runtime enforcement (transfer extrinsic disabled)
- Mint on: first accepted attestation
- Increment on: claim survival events propagated from indexer
- Burn/decrement on: retraction penalty propagation
- Storage: `Reputation: map AccountId => ReputationRecord { field_scores: BTreeMap<FieldId, u64> }`

**`pallet-governance`**
- Proposal: any SBT holder with minimum field score
- Vote weight: `sqrt(field_sbt_score)` — quadratic to resist concentration
- Scope: protocol parameter changes, field schema additions, oracle whitelist

---

## Arweave Content Layer

Claim content is stored as a JSON transaction on Arweave with tags:

```json
{
  "App-Name": "apodokimos",
  "App-Version": "0.1.0",
  "Content-Type": "application/json",
  "Claim-Type": "primary-claim | hypothesis | method | result | replication",
  "Field-Id": "<field_schema_id>",
  "Claim-Hash": "<blake3_hash_of_content>"
}
```

The `Claim-Hash` ties the Arweave transaction to the on-chain registry entry. Content is immutable once uploaded. Arweave's permaweb guarantees permanent availability without Apodokimos running any infrastructure.

---

## Off-Chain Indexer

The indexer subscribes to Substrate events, reconstructs the ECG, and computes W(claim) scores. Score snapshots are Merkle-anchored on-chain periodically.

```
Substrate events → indexer graph engine → W(claim) for all claims
                                        → Merkle root anchored on-chain
                                        → Score proofs queryable by SDK
```

The O factor oracle initially supports:
- ClinicalTrials.gov (NCT linkage for clinical pilot)
- PROSPERO (systematic review protocol registration)
- DOI-to-policy linkage via OpenAlex API

---

## Identity and DID

Submitter and reviewer identity uses W3C DIDs. No real-name requirement. ZK-proof of credential allows anonymous-but-credentialed participation — e.g., prove you hold a medical license without revealing identity.

DID method: `did:substrate:apodokimos` (custom, defined at v0.2.0).

---

## Bootstrap Domain: Clinical Medicine

The first field schema deployed targets PICO-structured clinical claims:

```
Claim := {
  population: String,
  intervention: String,
  comparator: String,
  outcome: String,
  effect_direction: Positive | Negative | Null,
  effect_size: Option<f64>,
  confidence_interval: Option<(f64, f64)>,
  trial_registry_id: Option<String>   // O factor anchor
}
```

PICO structure is already standardized in EBM, making claim granularity well-defined in this domain. Trial registry IDs provide a direct O factor oracle without requiring subjective judgment.

---

## Security Considerations

| Vector | Mitigation |
|---|---|
| Sybil reviewers | Field SBT minimum threshold; ZK credential proof |
| Governance plutocracy | Quadratic SBT voting; SBTs non-transferable |
| Claim spam | Registration deposit (refunded on first attestation) |
| Oracle manipulation | Multi-source O factor; oracle whitelist governed on-chain |
| GDPR right to erasure | Personal data off-chain only; on-chain stores hashes not PII |
| Chain capture | AGPL-3.0; any fork must stay open; governance on-chain not in repo |

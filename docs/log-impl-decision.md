# Transparency Log Implementation Decision (`apodokimos-log`)

## Spec reference
- Implements TODO Phase 3 items `RI-01` and `RI-02` in `TODO.md`
- Aligned with `ARCHITECTURE.md` requirement that `apodokimos-log` is in RFC 6962 / CT / Rekor lineage

## Surveyed options

### 1) Google Trillian
- **Pros:** production-grade CT-style log backend, mature architecture, ecosystem adoption
- **Cons:** substantial operational complexity (MySQL/Spanner + signer + sequencer + map/log deployment), stronger fit for infrastructure teams than small bootstrap protocol teams

### 2) Sigstore Rekor
- **Pros:** widely used transparency log in software supply chain; proven operational patterns; rich API semantics
- **Cons:** domain model is optimized for software artifacts and supply-chain attestations rather than protocol-native ECG events; introduces adaptation overhead for custom claim/attestation event envelopes

### 3) Custom RFC 6962-style implementation (selected)
- **Pros:**
  - minimal dependency and operational footprint for v0.2 bootstrap
  - direct control over event envelope and canonical serialization for protocol-native entries
  - deterministic inclusion/consistency verification logic can be kept close to `apodokimos-state`
  - easiest path to local integration testing (in-memory local log instance)
- **Cons:**
  - less battle-tested than Trillian/Rekor
  - implementation and security hardening burden is on project maintainers

## Decision
For the **v0.2 reference implementation bootstrap**, `apodokimos-log` uses a **custom RFC 6962-style Merkle log client + local in-memory backend**.

This satisfies the current constraints (small operator set, low write throughput, deterministic replay, local integration testing) while keeping migration paths open to Trillian/Rekor-backed operators later.

## Dependency policy (`RI-02`)
Pinned to latest stable compatible with workspace Rust toolchain:
- `sha2 = 0.10` (Merkle hashing)
- `ed25519-dalek = 2.0` (STH/witness signature verification)
- `serde = 1.0`, `serde_json = 1.0` (canonical payload serialization)
- `hex = 0.4` (encoded signatures/public keys)
- `thiserror = 2.0` (typed errors)

## Migration note
The API in `apodokimos-log` is intentionally backend-oriented: a future Trillian/Rekor transport layer can implement the same client contract (`submit`, inclusion verification, consistency verification, witness verification) without changing protocol-layer call sites.

# Apodokimos

> *ἀποδόκιμος — rejected after testing, found counterfeit.*

Apodokimos is a decentralized Epistemic Contribution Graph (ECG) protocol that replaces journal-level citation counting with claim-level survival scoring. Claim content is stored permanently on Arweave; an append-only verifiable history records claims and attestations; protocol state is derived deterministically from that history.

The Journal Impact Factor measures citation velocity inside a prestige economy. Apodokimos measures whether claims are true, whether they survived falsification, and whether reality changed because of them.

**Repository:** https://github.com/apodokimos/apodokimos
**Site:** https://apodokimos.science

## Specification

The protocol is defined by its whitepaper, versioned independently of any implementation.

| Version | Status | Version DOI |
|---|---|---|
| wp-v0.2 | Current | [10.5281/zenodo.19763292](https://doi.org/10.5281/zenodo.19763292) |
| wp-v0.1 | Superseded | [10.5281/zenodo.19583091](https://doi.org/10.5281/zenodo.19583091) |

When citing this specification in academic work, claim metadata, or any archival context, **always cite the Version DOI** of the specific version you are referring to. See [whitepaper §1.6](https://doi.org/10.5281/zenodo.19763292) for the full versioning and citation convention. The Concept DOI (always pointing to the latest version) is visible on Zenodo's "Versions" panel and is appropriate for navigation, never for archival citation.

**v0.1 historical artifacts** (preserved for completeness, do not cite for current work):
- Arweave (md): https://arweave.net/Xy_wY5mer8OHEX8QhdJW7w0L983Fz86k6gQaM8XEweM
- Arweave (pdf): https://arweave.net/5HXQRTB37pxh9UzYr69BdiVlI-5yJL3tvYj_cWMytKM

---

## The Problem

The Journal Impact Factor (IF):
- Measures average citations per article in a 2-year window
- Conflates citation with quality (retracted papers count equally)
- Is systematically biased toward large fields and review articles
- Has zero correlation with reproducibility, clinical translation, or real-world outcome
- Is owned and controlled by commercial oligarchs (Elsevier, Clarivate, Springer Nature)

Every proposed alternative (h-index, CiteScore, altmetrics) fixes one parameter while keeping the same broken ontology: **the paper as the atomic unit, citations as the signal, journals as the prestige layer**.

## The Solution

Apodokimos changes the ontology:

| Current system | Apodokimos |
|---|---|
| Paper is the atom | Claim is the atom |
| Citation count | Survival score |
| Journal prestige | Epistemic weight |
| Owned by publishers | Owned by no one |
| Retraction has no consequence | Penalty propagates through dependency graph |
| Self-referential | Real-world outcome linkage |

### Claim Weight Function

```
W(c, t) = R(c, t) × D̃(c) × S(c) × (1 + γ · O(c)) × δ(c)
```

| Variable | Meaning |
|---|---|
| `R(c, t)` | Replication score, field-calibrated time decay, Laplace-smoothed |
| `D̃(c)` | Log-normalized dependency depth in the claim graph |
| `S(c)` | Survival rate under falsification attempts, Laplace-smoothed |
| `O(c)` | Real-world outcome linkage (trial registry, policy, measurable event); enters as a multiplicative bonus `(1 + γ · O)` so basic science is not penalized |
| `γ` | Governed outcome-bonus coefficient (default 1) |
| `δ(c)` | Retraction discount; carries the persistence of penalty propagation |

Each factor has a strictly positive baseline. A newly-registered claim, a terminal claim with no dependents, an unattested claim, or a basic-science claim without real-world linkage all carry non-zero weight; none of those conditions collapse W to zero. See whitepaper §3 for the full derivation and §3.7 for the retraction discount mechanism.

### Governance

Voting weight is epistemic, not plutocratic. Reputation is encoded as **Soulbound Tokens** (SBTs) — non-transferable, non-sellable, identity-bound. Institutions cannot bulk-buy influence.

```
vote_weight(account, F) = sqrt(field_score(account, F))         # field-specific
vote_weight(account, global) = sqrt(mean_nonzero(field_scores(account)))  # cross-field
```

Quadratic weighting provides concentration resistance. Sybil resistance comes from a separate mechanism — SBT scores are accumulated slowly through demonstrated survival and attestation work, and DID-to-SBT one-to-one binding prevents fragmentation. See whitepaper §7.1 and §12.1.

---

## Architecture

The whitepaper specifies the protocol's mechanism requirements (R1–R13) independent of any implementation stack. The reference implementation composes a transparency log (Certificate Transparency / Rekor lineage), Arweave for content, OpenTimestamps for Bitcoin-anchored timestamps, and a deterministic Rust state-derivation program. Alternative implementations satisfying R1–R13 are equally valid (whitepaper §10.4).

See [ARCHITECTURE.md](./ARCHITECTURE.md) for the implementation map.

## Roadmap

See [TODO.md](./TODO.md).

---

## Licenses

| Layer | License |
|---|---|
| Protocol core (`apodokimos-core`, `apodokimos-state`) | AGPL-3.0 |
| Reference implementation crates (`apodokimos-log`, `apodokimos-anchor`, `apodokimos-arweave`, `apodokimos-indexer`, `apodokimos-cli`) | AGPL-3.0 |
| Client SDK (`apodokimos-sdk`, `sdk-ts`) | Apache-2.0 |
| Claim content schema (e.g., `clinical-medicine-v0.1.json`) | CC0-1.0 |

The protocol core is AGPL-3.0 by design. Any hosted fork must remain open. This is not a conventional open source choice — it is a structural defense against institutional capture.

---

## Contributing

Apodokimos is not owned by any institution or individual. Governance is recorded in the protocol's append-only history under a multi-signature scheme (whitepaper §7.5); future versions may upgrade governance to a chain-based state machine if scale demands it. Protocol changes require valid k-of-n signed governance actions, not maintainer approval.

Contribution guide: TBD at v0.2.0.

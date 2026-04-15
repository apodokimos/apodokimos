# Apodokimos

> *ἀποδόκιμος — rejected after testing, found counterfeit.*

Apodokimos is a decentralized Epistemic Contribution Graph (ECG) protocol that replaces journal-level citation counting with claim-level survival scoring — anchored on a Substrate parachain, with claim content stored permanently on Arweave.

The Journal Impact Factor measures citation velocity inside a prestige economy. Apodokimos measures whether claims are true, whether they survived falsification, and whether reality changed because of them.

Repository: https://github.com/apodokimos/apodokimos

Whitepaper: https://doi.org/10.5281/zenodo.19583091

Arweave (md): https://arweave.net/Xy_wY5mer8OHEX8QhdJW7w0L983Fz86k6gQaM8XEweM

Arweave (pdf): https://arweave.net/5HXQRTB37pxh9UzYr69BdiVlI-5yJL3tvYj_cWMytKM

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
W(claim) = R(t) × D × S × O
```

| Variable | Meaning |
|---|---|
| `R(t)` | Replication score, field-calibrated time-decay |
| `D` | Downstream dependency depth in the claim graph |
| `S` | Survival rate under falsification attempts |
| `O` | Real-world outcome linkage (trial registry, policy, measurable event) |

### Governance

Voting weight is epistemic, not plutocratic:

```
vote_weight = f(claim_survival_rate, replication_depth, field_SBT)
```

Reputation is encoded as Soulbound Tokens (SBTs) — non-transferable, non-sellable, identity-bound. Institutions cannot bulk-buy influence.

---

## Architecture

See [ARCHITECTURE.md](./ARCHITECTURE.md).

## Roadmap

See [TODO.md](./TODO.md).

---

## Licenses

| Layer | License |
|---|---|
| Protocol core (`apodokimos-core`, `apodokimos-chain`) | AGPL-3.0 |
| Client SDK (`apodokimos-sdk`) | Apache-2.0 |
| Claim content schema | CC0-1.0 |

The protocol core is AGPL-3.0 by design. Any hosted fork must remain open. This is not a conventional open source choice — it is a structural defense against institutional capture.

---

## Contributing

Apodokimos is not owned by any institution or individual. Governance is on-chain. Protocol changes require epistemic-weighted SBT vote, not maintainer approval.

Contribution guide: TBD at v0.1.0.

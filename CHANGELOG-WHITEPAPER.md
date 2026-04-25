# Whitepaper Changelog

This document records revisions between whitepaper versions. Each claim registered on Apodokimos is bound to the Version DOI of the specification under which it was registered (wp-v0.2 §1.6); past claims retain their original scoring rules indefinitely.

---

## wp-v0.1 → wp-v0.2

**Version DOI:** `10.5281/zenodo.19763292`
**Supersedes:** `10.5281/zenodo.19583091`

### W(c, t) Formula Revision (Appendix D)

The claim weight function was revised from:

```
W(claim) = R(t) × D × S × O
```

to:

```
W(c, t) = R(c, t) × D̃(c) × S(c) × (1 + γ · O(c)) × δ(c)
```

| Change | Rationale |
|--------|-----------|
| Laplace smoothing for `R(c, t)` | Uniform Beta prior (α = β = 1) prevents zero weight for newly-registered claims with no replication data |
| Log-normalized `D̃(c) = [1 + log(1 + D)] / [1 + log(1 + D_ref_field)]` | Prevents dependency-depth dominance; terminal claims (D = 0) carry non-zero weight |
| Laplace smoothing for `S(c)` | Uniform prior prevents zero weight for unattested claims |
| Multiplicative bonus `(1 + γ · O)` instead of direct multiplier | Basic-science claims (O = 0) are not penalized; O = 1 yields weight × (1 + γ) |
| Explicit retraction discount `δ(c)` | Penalty from retraction cascades through the graph with `W_pre` snapshot preservation; avoids silent zeroing of dependent claims |

Each factor has a strictly positive baseline. A newly-registered claim, a terminal claim, an unattested claim, or a basic-science claim without real-world linkage all carry non-zero weight.

### Cross-Field Voting Correction (§7.1)

The cross-field voting weight formula was corrected from a geometric mean (which could collapse to zero) to an arithmetic mean over non-zero field scores:

```
# wp-v0.1 (incorrect)
vote_weight(account, global) = sqrt(geomean(field_scores(account)))

# wp-v0.2 (corrected)
vote_weight(account, global) = sqrt(mean_nonzero(field_scores(account)))
```

A specialist active in exactly one field retains their full field-specific weight in cross-field votes. The quadratic weighting (`sqrt`) provides concentration resistance; Sybil resistance is achieved via SBT accumulation cost per identity, not via the voting formula itself (§12.1).

### Versioning Convention Formalization (§1.6)

The distinction between **Version DOI** and **Concept DOI** is now explicit:

- **Version DOI** (`10.5281/zenodo.XXXXX`) — immutable per-version identifier. Used in all archival contexts: claim metadata, source code comments, academic citations, anchored documents.
- **Concept DOI** — moving pointer to the latest version. Used in README links, onboarding documentation, and for navigation within Zenodo. Never used for archival citation.

**Placeholder discipline:** Draft whitepaper versions use the literal string `<TO-BE-ASSIGNED-AT-ANCHOR>` as a placeholder for the Version DOI until anchoring. CI rejects merges to `main` containing this string inside `WHITEPAPER/` artifacts.

### Architecture Pivot (§10)

The reference implementation was restructured from a Substrate parachain design (wp-v0.1) to a transparency-log-plus-deterministic-state-derivation model (wp-v0.2 §10.2). The Substrate design is preserved as Alternative A (§10.4). Claims registered under wp-v0.1 continue to be scored under wp-v0.1 rules regardless of implementation changes.

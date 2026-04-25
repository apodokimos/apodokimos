# Apodokimos: A Decentralized Epistemic Contribution Graph for Scientific Claim Evaluation

**Version:** wp-v0.2  
**Status:** Draft  
**License:** CC0-1.0  
**Version DOI** *(this document, immutable)*: `10.5281/zenodo.19763292`  
**Concept DOI** *(always points to latest version)*: *see Zenodo "Versions" panel on the v0.1 record (10.5281/zenodo.19583091)*  
**Supersedes:** wp-v0.1 (Version DOI: [10.5281/zenodo.19583091](https://doi.org/10.5281/zenodo.19583091))  
**Site:** https://apodokimos.science  
**Repository:** https://github.com/apodokimos/apodokimos  

> **Citation discipline.** When citing this specification in academic work, claim metadata, source code, or any context that will be archived, **use the Version DOI**. The Concept DOI is a navigation pointer to the latest version and changes meaning over time; it is appropriate for README links and "current specification" references but not for archival citation. See §1.6.  

---

## Abstract

The Journal Impact Factor (IF) — a 2-year citation velocity metric designed in 1955 for library collection management (Garfield, 1955) — has become the dominant instrument for evaluating scientific output, allocating research funding, and determining academic careers (McKiernan et al., 2019). This repurposing is structurally unsound: the IF measures citation frequency within a journal-bounded prestige economy, not epistemic quality, reproducibility, or real-world consequence. Every proposed alternative (h-index, CiteScore, altmetrics, Scite Index) addresses surface parameters while preserving the broken ontology: the paper as the atomic unit of science, citations as the signal, and journals as the prestige layer.

We propose Apodokimos — a decentralized Epistemic Contribution Graph (ECG) protocol that replaces journal-level citation counting with claim-level survival scoring. In the ECG model, the atomic unit is a falsifiable claim, not a paper. Claims accumulate epistemic weight through a function of replication score, dependency depth, survival under falsification, and real-world outcome linkage. Attestations — typed as supports, contradicts, replicates, or refutes — are recorded in an append-only verifiable history with independent timestamp anchoring. Claim content is stored permanently on Arweave. Researcher reputation is encoded as non-transferable Soulbound Tokens (SBTs), making governance weight identity-bound rather than capital-bound. The protocol is owned by no institution and licensed under AGPL-3.0, structurally preventing commercial capture.

The first deployment domain is clinical medicine, where PICO-structured claim schemas and existing trial registry infrastructure provide well-defined granularity and objective real-world outcome oracles.

**Revision note (wp-v0.2):** This version supersedes wp-v0.1 in two respects: (1) it makes explicit the priority ordering between the ECG itself (primary contribution) and the governance mechanism that protects it (secondary, defensive); and (2) it rewrites the implementation architecture section as requirements-first and stack-agnostic. The ECG formal model (§2–§5) is unchanged from wp-v0.1. A full change log appears in Appendix D.

---

## 1. Problem Statement

### 1.1 What the Impact Factor Actually Measures

The Journal Impact Factor for year X is defined as (Garfield, 1955):

```
IF(X) =  citations in X to articles published in (X-1) and (X-2)
         ─────────────────────────────────────────────────────────
         total citable articles published in (X-1) and (X-2)
```

This is a measure of citation velocity of a journal's average article within a 2-year window. It captures reading habits of a specific community, modulated by field size, citation norms, and editorial strategy. It does not measure:

- Whether findings are reproducible
- Whether claims survived independent testing
- Whether the work influenced clinical practice, policy, or technology
- Whether citations are positive, negative, or retractions

The widespread failure of published findings to replicate is itself well-documented (Ioannidis, 2005). The Reproducibility Project (Open Science Collaboration, 2015) replicated 100 psychology studies and found that higher-IF journals had worse replication rates — the metric is negatively correlated with the quality it purports to measure in at least one major field.

Chalmers and Glasziou (2009) estimated that 85% of biomedical research investment is wasted, the majority published in indexed, respectable journals. The IF certified the packaging, not the truth.

### 1.2 Structural Origin of the Misalignment

The IF persists not because it is accurate but because it solves a coordination problem. Every hiring committee, grant reviewer, and tenure panel requires a shared scalar. The IF is wrong but universally legible. Any replacement must solve the coordination problem, not only the measurement problem.

The incentive loop is entirely self-referential:

```
Researcher needs tenure
  → needs publications in high-IF journals
  → journals reward novelty + p < 0.05
  → researchers optimize for publishability, not truth
  → loop closes on itself, never touching reality
```

### 1.3 Why Existing Alternatives Fail

Every alternative published to date — DORA (DORA, 2013), Leiden Manifesto (Hicks et al., 2015), Scite (Nicholson et al., 2021), Octopus (launched 2022), CRediT taxonomy (Brand et al., 2015) — fails to exit the paper ontology. They improve the lens on the existing system without replacing the system's atomic unit.

DORA gathered 26,000+ signatories and produced marginal behavioral change. An eLife study by McKiernan et al. (2019) analyzed review, promotion, and tenure documents at a large sample of research-intensive institutions and found that IF-based metrics continued to dominate hiring, promotion, and funding decisions in a substantial fraction of cases — well after DORA's adoption. Signing a declaration does not change an incentive structure.

Scite (Nicholson et al., 2021) contextualizes citations (supporting vs. contradicting) but remains paper-level, and the product has since been acquired commercially, reproducing the capture dynamic it was designed to challenge.

Octopus restructures the paper into typed micro-publications but retains the paper as the product. Adoption remains near zero because it offers no prestige signal.

The failure mode is consistent: downstream interventions operating within an upstream ontology they do not challenge.

### 1.4 The Required Ontological Break

The necessary change is not a better metric for papers. It is a replacement of the paper as the atomic unit of science with the falsifiable claim — the actual unit that reality cares about.

Science does not produce papers. Science produces claims about how the world works. Papers are the containers those claims travel in. The container has captured all the value.

### 1.5 Primary and Secondary Commitments

Apodokimos is structured around two commitments in explicit priority order.

**Primary commitment — the Epistemic Contribution Graph itself.** The claim-as-atomic-unit ontology (§2), the typed attestation taxonomy (§2.3), the weight function W(c, t) = R(t) × D × S × O (§3), the DAG dependency structure (§4), and the penalty propagation mechanism (§5) together constitute the substantive contribution that distinguishes Apodokimos from existing metrics. This is the content of the critique and the structure of the alternative.

**Secondary commitment — the governance and reputation mechanism that protects the ECG from capture.** Non-transferable Soulbound Tokens (§6), epistemic-weighted voting (§7), and the capture-resistance analysis (§12) exist to defend the ECG from the dynamics that ended every prior reform attempt. Governance is reinforcement around the ECG; the ECG does not exist to support governance.

This ordering has practical consequences for implementation scope. An implementation that delivers a correct ECG with a weak or provisional governance mechanism still carries the protocol's primary value. An implementation that delivers governance machinery without the ECG delivers nothing. Therefore:

- When trading off implementation complexity against time-to-demonstration, governance may be deferred or simplified before the ECG may be.
- When choosing implementation technology, the ECG's mechanism requirements dominate the selection; governance's requirements are secondary inputs.
- The whitepaper versions itself alongside the protocol; governance schema may evolve between versions without invalidating earlier ECG content.

This ordering was implicit in wp-v0.1 but is made explicit here to prevent its inversion during implementation work.

### 1.6 Versioning and Citation Convention

The Apodokimos specification evolves across versions (wp-v0.1, wp-v0.2, wp-v0.3, …). The protocol's own self-validation thesis (§11.4) and its commitment to long-term scientific archival (§9.1) require that every version of the specification remain permanently retrievable, individually citable, and unambiguously identifiable. This section defines the convention that satisfies these requirements.

**Two classes of identifier.** Each anchored version of the whitepaper is associated with two DOIs assigned by Zenodo:

- A **Version DOI** is permanent, immutable, and resolves to the exact bytes of one specific version. wp-v0.1's Version DOI (`10.5281/zenodo.19583091`) will continue to resolve to wp-v0.1's exact content indefinitely, regardless of how many later versions are published. Each new version receives a new Version DOI that never redirects.
- A **Concept DOI** is shared across all linked versions of the same work and resolves to whichever version is currently latest. The Concept DOI is a *moving pointer*: a reader resolving it today receives the current version; the same DOI in three years may resolve to a different version. The Concept DOI exists because Zenodo's "new version" feature creates linked versions under a single concept; this is documented at https://help.zenodo.org and is the standard mechanism for versioned scientific deposits on the platform.

**Convention for Apodokimos.**

The two identifiers serve different purposes and are used in different contexts. Conflating them is the most common versioning error in long-lived scientific artifacts and is the practice this convention is designed to prevent.

| Use the **Version DOI** for: | Use the **Concept DOI** for: |
|---|---|
| Academic citation in any external work | README links to "the current specification" |
| Claim metadata in the ECG (every claim records the Version DOI of the spec it was registered against) | Project landing pages and onboarding documentation |
| Source code comments and protocol implementations | Zenodo navigation between sibling versions |
| Archived references — anything intended to remain stable across decades | Newcomer-facing "what's the latest" references |
| The whitepaper's own self-reference in its header (`<this-version-doi>`) | The README's reference to "the spec" without version commitment |

**Rationale.** The Version DOI is the address-of-record. The Concept DOI is a convenience for navigation. Citing the Concept DOI in archival contexts produces a class of error in which a 2026 paper cites "the Apodokimos protocol" via the Concept DOI; a reader in 2030 resolves the same DOI and lands on a substantially evolved later version that may not reflect what the 2026 author was actually referring to. Citing the Version DOI in archival contexts is robust against this drift: the 2030 reader lands on exactly the bytes the 2026 author cited.

**Self-validation under versioning.** The §11.4 self-registration property requires that the version of the specification be unambiguous at the moment of registration. Each version of the whitepaper is registered as a distinct ECG claim. The claim's metadata includes the Version DOI of the specification it represents. Future readers verifying the registration can resolve the Version DOI to retrieve the exact bytes that were claimed.

**Schema versioning under versioned specifications.** Field schemas (e.g., `clinical-medicine-v0.1.json`) are versioned independently of the whitepaper. Claims registered against a specific schema version retain their original interpretation: the state-derivation program (§10.2) reads each claim's referenced schema version and applies that schema, not the latest. Specifications may add new schemas or revise existing ones in later versions; past claims continue to be scored under the schema in force at their registration. This makes the ECG history coherent across decades of specification evolution.

**Documentation hygiene.** The whitepaper's header carries its own Version DOI (the document is self-referential). Any draft of a future version uses placeholder strings (`10.5281/zenodo.19763292`) until the moment of anchoring, at which point the assigned DOI is filled in. CI in the protocol repository should reject any merge to `main` that contains placeholder strings in published whitepaper artifacts. Drafts may carry placeholders; published versions may not.

---

## 2. The Epistemic Contribution Graph

### 2.1 Formal Model

The Epistemic Contribution Graph G is a directed acyclic graph (DAG):

```
G = (C, A, E)
```

Where:

- `C` is the set of registered claims (nodes)
- `A` is the set of attestations (typed edges)
- `E ⊆ C × C` is the dependency relation (claim builds on claim)

**Acyclicity constraint.** No claim may depend on itself directly or transitively. Dependency cycles are rejected at the protocol level.

### 2.2 Claim Definition

A claim is the minimal unit of falsifiable scientific assertion. Formally:

```
Claim := {
    id:                ClaimId,           // blake3(canonical_json(content)) — see O'Connor et al. (2020)
    claim_type:        ClaimType,
    field_id:          FieldId,
    spec_version_doi:  VersionDOI,        // §1.6: immutable DOI of the spec
                                          // version in force at registration
    content:           ClaimContent,      // stored on Arweave, hash anchored in log
    submitter:         DID,
    depends_on:        Vec<ClaimId>,      // edges in E
    arweave_tx:        TxId,
    registered:        RecordTimestamp
}
```

*(Note: `RecordTimestamp` replaces `BlockNumber` from wp-v0.1 to reflect stack-agnostic framing. See §10.)*

#### 2.2.1 ClaimType Taxonomy

```
ClaimType :=
    | PrimaryClaim    // a novel falsifiable assertion about the world
    | Hypothesis      // a testable prediction not yet tested
    | Method          // a procedure claim (reproducibility target)
    | Result          // an empirical measurement or observation
    | Replication     // an independent repetition of a prior Result
    | NullResult      // a Result with no detectable effect
```

Null results are first-class citizens. The current system suppresses null results (publication bias). In the ECG, a null result that contradicts a prior PrimaryClaim carries epistemic weight equivalent to a positive result.

#### 2.2.2 Claim Granularity

Claim granularity is the hardest operational parameter. Too fine: spam and decomposition gaming. Too coarse: reproduces the paper ontology.

The protocol enforces granularity through field schemas. Each field defines a canonical claim template that specifies required and optional fields. A submission that cannot be represented in the field schema cannot be registered as a single claim — it must be decomposed.

For clinical medicine (the bootstrap domain), the PICO schema provides well-defined granularity (see §11).

### 2.3 Attestation Definition

An attestation is a typed statement by a credentialed reviewer about a claim's relationship to existing evidence:

```
Attestation := {
    id:            AttestationId,
    claim_id:      ClaimId,
    attester:      DID,
    verdict:       AttestationVerdict,
    evidence_tx:   Option<TxId>,      // Arweave pointer to supporting evidence
    attester_sbt:  FieldSBTScore,     // snapshot at time of attestation
    recorded:      RecordTimestamp
}
```

#### 2.3.1 AttestationVerdict Taxonomy

```
AttestationVerdict :=
    | Supports       // attester's evidence corroborates the claim
    | Contradicts    // attester's evidence is in tension with the claim
    | Replicates     // independent repetition confirms the claim
    | Refutes        // independent repetition disconfirms the claim
    | Mentions       // neutral reference; excluded from survival scoring
```

`Mentions` is tracked but does not contribute to survival score S. This eliminates the current system's fundamental flaw where a citation that reports failure is counted identically to a citation that confirms.

---

## 3. Claim Weight Function

*(Substantially revised in wp-v0.2 to fix zero-collapse bugs in wp-v0.1's multiplicative formula. See Appendix D for the change log and rationale.)*

### 3.1 Definition

The epistemic weight of a claim at time t is:

```
W(c, t) = R(c, t) × D̃(c) × S(c) × (1 + γ · O(c)) × δ(c)
```

Where:

- `R(c, t)` — replication score with time decay, `R ∈ (0, 1)`
- `D̃(c)` — normalized dependency depth, `D̃ > 0` and unbounded above
- `S(c)` — survival rate, `S ∈ (0, 1)`
- `O(c)` — real-world outcome linkage, `O ∈ [0, 1]`; enters multiplicatively as bonus `(1 + γ · O)`
- `γ` — governed coefficient controlling the magnitude of the outcome bonus; default `γ = 1`, so `(1 + γ · O) ∈ [1, 2]`
- `δ(c)` — retraction discount, `δ ∈ [0, 1]`; default `1.0` at registration

Each factor has a strictly positive baseline so that newly-registered claims, terminal claims with no dependents, unattested claims, and basic-science claims without real-world linkage all have non-zero weight. A claim acquires weight through evidence accumulation across any factor; no single zeroed factor collapses the entire weight.

W is normalized per-field for cross-field comparison (§3.6).

### 3.2 R(c, t) — Replication Score with Time Decay

R(c, t) is the posterior mean replication rate under a uniform Beta prior, with time-decayed weighting of replication attestations:

```
R(c, t) = [Σ_{a ∈ Replicates(c)} w_decay(t − a.recorded, t_½_field)] + α
          ──────────────────────────────────────────────────────────────
          |Replicates(c)| + |Refutes(c)| + α + β
```

Where:

```
w_decay(Δt, t_½) = 2^(−Δt / t_½)
```

- `t_½_field` is the field-specific half-life expressed in the same units as `RecordTimestamp` (see §2.2). Fields with fast knowledge cycles (clinical trials, molecular biology) use short half-lives; fields with slow cycles (mathematics, geology) use long half-lives. Half-life notation is adopted over decay-constant notation so the governable parameter is directly interpretable.
- `α, β` are Beta pseudocounts (Laplace smoothing). Default `α = β = 1`, producing baseline `R = 0.5` for claims with no replication evidence. `α, β` are governable per field.

**Properties.**

- No replication evidence: `R = α / (α + β) = 0.5` (neutral prior). This is the bootstrap case.
- Many recent `Replicates`, no `Refutes`: `R → 1`.
- Many `Refutes`, no `Replicates`: `R → 0`.
- `R ∈ (0, 1)` strictly; extreme values approached asymptotically.

**Rationale.** wp-v0.1's R had `ε` in the denominator only, leaving the numerator potentially zero — a claim with no `Replicates` had `R = 0`, zeroing W. Laplace smoothing (Laplace, 1774) places the baseline at the uniform-prior posterior mean, which is the standard Bayesian answer for "what do we believe before seeing evidence."

### 3.3 D̃(c) — Normalized Dependency Depth

The raw dependency depth D(c) counts transitive descendants in the dependency relation E:

```
D(c) = |{c' ∈ C : c ∈ ancestors(c')}|
```

D̃(c) is the log-normalized form with a non-zero baseline:

```
D̃(c) = [1 + log(1 + D(c))] / [1 + log(1 + D_ref_field)]
```

Where `D_ref_field` is a governed reference depth per field — for example, the running median or 75th percentile of D over non-terminal claims in the field.

**Properties.**

- Terminal claim (D = 0): `D̃ = 1 / [1 + log(1 + D_ref_field)] > 0`. Non-zero baseline; carries weight.
- Claim at the reference depth: `D̃ = 1`.
- Claim with `D >> D_ref_field`: `D̃ > 1`, unbounded but growing logarithmically — preventing runaway weight from "seminal" claims while still rewarding structural importance.

**Rationale.** wp-v0.1 used raw D, which zeroed W for every terminal claim — contradicting the spec's narrative that terminal claims should still carry their own survival weight. Log-normalization with `+1` offsets keeps D̃ strictly positive and bounds the growth rate.

### 3.4 S(c) — Survival Rate

S(c) is the posterior mean survival rate under a uniform Beta prior, over non-Mentions attestations:

```
S(c) = |Supports(c)| + |Replicates(c)| + α
       ─────────────────────────────────────────────────────────────
       |Supports(c)| + |Contradicts(c)| + |Replicates(c)| + |Refutes(c)| + α + β
```

With default `α = β = 1`:

- No attestations: `S = 0.5` (neutral prior). Bootstrap case.
- Overwhelming support: `S → 1`.
- Overwhelming refutation: `S → 0`.
- `S ∈ (0, 1)` strictly.

**Retraction note.** Formal retraction of a claim does *not* modify S. Retraction affects the retraction discount δ (§3.7). This separation keeps S strictly evidence-based — it reflects what the attestation record shows — while δ carries the policy/integrity signal.

### 3.5 O(c) — Real-World Outcome Linkage

O(c) ∈ [0, 1] measures the strength of traceable connections between claim c and real-world outcomes outside the academic system:

```
O(c) = max_{s ∈ registered_sources(c)} oracle_score(s)
```

If `registered_sources(c)` is empty, `O(c) = 0`.

O enters W as a multiplicative bonus `(1 + γ · O)`, not as a direct factor. This is the mechanism that preserves the spec's narrative intent from wp-v0.1 §3.5: "O = 0 does not penalize basic science." With the bonus formulation:

- Basic science (`O = 0`): bonus term is `(1 + γ · 0) = 1`; no effect on W.
- Strong real-world linkage (`O = 0.9`): bonus term is `(1 + γ · 0.9)`; multiplies W by up to `(1 + γ)` depending on the governed coefficient.

**Oracle source types** (unchanged from wp-v0.1):

```
OracleSource :=
    | TrialRegistry(nct_id, outcome_reported: bool)
    | SystematicReviewProtocol(prospero_id, review_completed: bool)
    | PolicyDocument(doi, adoption_confirmed: bool)
    | Patent(patent_id, granted: bool)
    | OpenAlexPolicyLinkage(doi)
```

Oracle scoring thresholds per source are defined at §11.3 for the clinical-medicine bootstrap. The mapping from `oracle_score` to the `[0, 1]` range is oracle-specific and governable.

**Rationale.** wp-v0.1 defined `O ∈ {0} ∪ [0.1, 1.0]` — a disjoint gap between "no linkage" and "minimal linkage" — and multiplied it directly into W. This produced two pathologies: (i) a discontinuous jump between `O = 0` and `O = 0.1`; (ii) `O = 0` zeroed W entirely for basic-science claims despite the narrative denying this. The bonus formulation eliminates both.

### 3.6 Normalized Field Score

W(c, t) values are not directly comparable across fields due to different `D̃` baselines, attestation rates, and oracle availability. For cross-field comparison:

```
W_norm(c, t, field) = W(c, t) / μ_field(t)
```

Where `μ_field(t)` is the running mean W across all claims in field at time t. `μ_field(t)` is computed by the state-derivation program (§10.2) and published with each snapshot.

### 3.7 Retraction Discount δ

Each claim carries a retraction discount `δ(c) ∈ [0, 1]`, initialized to `1.0` at registration. δ is the persistence mechanism for retraction penalties: it is the field that §5's propagation algorithm mutates.

```
δ(c) at registration:
    δ(c) ← 1.0

On retraction of c itself:
    δ(c) ← 0

On retraction of any c_retracted ∈ direct_dependencies(c):
    δ(c) ← δ(c) × (1 − penalty_fraction(c, c_retracted))

Where:
    penalty_fraction(c, c_retracted) = W_pre(c_retracted)
                                       ──────────────────────────────────────
                                       Σ_{d ∈ direct_dependencies(c)} W_pre(d)

    W_pre denotes W evaluated immediately before the current retraction event.
```

**Properties.**

- δ is monotonically non-increasing over a claim's lifetime.
- A retracted claim has `δ = 0`, forcing `W = 0` regardless of its other factors. Retraction is terminal for the retracted claim.
- A claim with all dependencies intact retains `δ = 1`.
- A claim with partial dependency retraction has `δ ∈ (0, 1)`, proportional to the retracted dependencies' pre-retraction weight share.
- Cascading effects (retraction of a dependency's dependency) compose multiplicatively through δ — each layer of upstream retraction multiplies δ by another `(1 − penalty_fraction)` factor.

**Rationale.** wp-v0.1's §5.2 specified `apply_penalty(c_dep, penalty)` but did not name the state field being mutated. W was computed on demand from the graph, so there was nowhere for penalty to persist. Introducing δ as an explicit field on each claim makes retraction effects concrete, auditable, and monotonically bounded.

---

## 4. Attestation Graph Structure

### 4.1 DAG Invariant

The dependency subgraph (edges in E, claim-to-claim dependencies) must be acyclic. The protocol enforces this at registration time:

```
register_claim(c) requires:
    ∀ dep ∈ c.depends_on: dep ∈ C ∧ c ∉ ancestors(dep)
```

This check is performed at registration and enforced by the reference implementation's state-derivation program. A claim that would create a cycle is rejected with `Error::DependencyCycle`.

Attestation edges (A) are not subject to the acyclicity constraint — an attester may attest to any registered claim regardless of dependency structure.

### 4.2 Edge Semantics

**Dependency edge** (c₁ → c₂ in E):

```
c₁ builds upon c₂
c₁'s validity is partially conditional on c₂'s survival
retraction of c₂ triggers penalty propagation to c₁
```

**Attestation edge** (a: c₁ ← c₂ in A, typed):

```
an attester asserts a verdict about c₁ based on c₂ as evidence
the direction is: attestation points to the claim being evaluated
the evidence (c₂ or external) is referenced via arweave_tx
```

### 4.3 Graph Consistency

The ECG maintains the following invariants:

1. Every node in C has a registered entry in the append-only history with a valid Arweave content pointer
2. Every node in A has a registered attestation record with attester SBT verified at attestation time
3. The dependency subgraph is acyclic (enforced at registration)
4. No attester may attest to the same claim twice (one attestation per attester per claim)
5. An attester may not attest to their own claim

---

## 5. Penalty Propagation

### 5.1 Retraction Event

When a claim c is retracted — either by the submitter or by governance action — a `ClaimRetracted` event is recorded in the append-only history. This triggers the penalty propagation algorithm in the state-derivation program.

### 5.2 Propagation Algorithm

Retraction propagates through δ, the retraction discount defined in §3.7.

```
propagate_retraction(c_retracted, G):
    # Step 1: mark the retracted claim itself
    δ(c_retracted) ← 0

    # Step 2: snapshot pre-retraction weights for use in denominators
    # (prevents the algorithm from depending on mid-cascade state)
    W_pre: Map<ClaimId, f64> = { c: W(c, t_event) for all c affected by cascade }

    affected = { c_retracted }
    queue = direct_dependents(c_retracted, G)

    while queue is not empty:
        c_dep = dequeue(queue)
        if c_dep ∈ affected:
            continue

        penalty_fraction = W_pre[c_retracted]
                           / Σ_{d ∈ direct_dependencies(c_dep)} W_pre[d]

        δ(c_dep) ← δ(c_dep) × (1 − penalty_fraction)
        affected.insert(c_dep)

        # Cascade only if the dependent's weight drops below the field threshold
        if W(c_dep, t_event) < Θ_field(c_dep.field_id):
            queue.extend(direct_dependents(c_dep, G))

    return affected
```

Where:

- `W_pre` is a snapshot of W evaluated immediately *before* the retraction event. All penalty calculations in a single propagation event use the same pre-retraction weights, ensuring the algorithm's output is independent of traversal order.
- `Θ_field(field_id)` is the per-field governed cascade threshold. A claim whose W remains above `Θ_field` after its δ is reduced does not propagate further — the cascade stops at that edge. This prevents minor retractions from cascading indefinitely through the graph.
- `affected` tracks visited claims to prevent re-visitation when the graph has multiple paths between nodes.

**Termination.** The cascade terminates because (i) δ is monotonically non-increasing and bounded below by 0, (ii) the DAG has finite ancestors for any node, and (iii) the `affected` set prevents re-visits.

**Atomicity.** Honest implementations execute the full cascade as a single transition: all affected δ values update atomically with respect to the retraction event. This property must be preserved by any alternative implementation per §10.

### 5.3 Rationale

In the current system, retraction has near-zero career consequence. A retracted paper continues to be cited positively for years after retraction — a phenomenon documented in bibliometric studies (Bar-Ilan & Halevi, 2017). Apodokimos makes retraction structurally consequential — not as punishment, but as accurate signal propagation. The graph corrects itself.

Researcher SBT scores are decremented proportionally to their contribution to retracted claims (§6.3). The submitter of a retracted claim bears the largest penalty.

---

## 6. Soulbound Token Reputation System

### 6.1 Design Principles

Reputation in Apodokimos is encoded as **Soulbound Tokens** — identity-bound, non-transferable on-record credentials in the sense developed by Weyl, Ohlhaver, and Buterin (2022). In Apodokimos specifically, reputation is:

- **Identity-bound** — attached to a DID, not an address or token balance
- **Non-transferable** — cannot be sold, delegated, or concentrated
- **Field-specific** — a clinician's SBT score in clinical medicine does not grant governance weight in mathematics
- **Earned through survival** — score increments when claims survive or attestations are validated; decrements when they do not

This design directly prevents the governance capture that destroys most DAOs: token concentration by institutions or wealthy actors is structurally impossible when the governance weight is a non-transferable function of demonstrated epistemic track record.

### 6.2 ReputationRecord Structure

```
ReputationRecord := {
    did:                DID,
    field_scores:       BTreeMap<FieldId, u64>,
    attestation_count:  u64,
    claim_count:        u64,
    survival_rate:      f64,            // W(claims_submitted) mean
    last_updated:       RecordTimestamp
}
```

### 6.3 SBT Lifecycle

**Mint.** On first accepted attestation. A researcher with no prior history receives a minimal SBT entry establishing their DID-to-record linkage.

**Increment — attestation validated.** When a claim the researcher attested to as `Supports` or `Replicates` later receives independent confirmation, the field score increments:

```
Δscore_attestation = base_increment × agreement_weight
```

**Increment — claim survived.** When a researcher's submitted claim reaches a survival milestone (e.g., 5 independent replications, O factor linkage confirmed), the field score increments:

```
Δscore_claim_survival = base_increment × W(claim) × survival_milestone_multiplier
```

**Decrement — retraction cascade.** When a submitted claim is retracted or a supported claim is refuted:

```
Δscore_penalty = -base_increment × penalty_weight × submitter_multiplier
```

The `submitter_multiplier > 1` for the original claim submitter and = 1 for attesters. False positive attestations are penalized less severely than fraudulent claim submission.

**Non-transferability enforcement (mechanism requirement).** The implementation must not expose any transfer operation for SBT scores. Non-transferability is enforced by *absence* of transfer code paths in the authoritative state-transition function, not by access control gates or revert-on-call patterns. An implementation in which a transfer function exists and reverts does not satisfy this requirement. An implementation in which no transfer function exists at all does.

*(Revision note: wp-v0.1 phrased this as "disabled at the runtime level" with specific reference to Substrate's dispatchable function set. The mechanism requirement is generalized here; the specific realization depends on the implementation stack — see §10.)*

---

## 7. Governance

### 7.1 Epistemic-Weighted Voting

All protocol governance uses epistemic-weighted SBT voting, adapting the quadratic voting mechanism (Lalley & Weyl, 2018) to reputation rather than capital. For a proposal in field F, a voter's weight is:

```
vote_weight(account, F) = sqrt(field_score(account, F))
```

Quadratic weighting produces diminishing returns on individual concentration: a researcher with 4× another's SBT score in a field has only 2× the vote weight in that field. This is the formula's contribution to **concentration resistance** — preventing a single high-reputation researcher from dominating field-specific governance.

**Important clarification** *(corrected in wp-v0.2)*. Quadratic weighting is *not* a Sybil-resistance mechanism. Under sqrt weighting, fragmenting a single account's score across multiple sub-accounts strictly *increases* total vote weight, because sqrt is subadditive: for non-negative x₁, ..., xₙ,

```
Σᵢ sqrt(xᵢ) ≥ sqrt(Σᵢ xᵢ)    (equality only when at most one xᵢ is non-zero)
```

Sybil resistance in Apodokimos comes from a separate mechanism: SBT scores are accumulated slowly through demonstrated survival and attestation work (§6.3), so manufacturing many identities with non-trivial SBT is costly. The identity layer's DID-to-SBT binding constrains fragmentation to one SBT record per (DID, field). Quadratic voting and Sybil resistance operate orthogonally; each addresses a different threat. See §12.1 for the Sybil-resistance argument.

**Cross-field voting.** For proposals whose scope is protocol-global rather than field-specific, a voter's weight uses the arithmetic mean of their non-zero field scores:

```
vote_weight(account, global) = sqrt(mean_nonzero(field_scores(account)))

Where:
    mean_nonzero(s) = (Σ_{f : s[f] > 0} s[f]) / |{f : s[f] > 0}|
```

An account with no non-zero field scores cannot vote on cross-field proposals (nor on any field-specific proposal, by the SBT minimum threshold). A specialist with high score in a single field retains their full mean as their cross-field weight; a generalist with scores in many fields votes with weight proportional to their average.

*(Revision note from wp-v0.1. The original cross-field formula used the geometric mean of all field scores, which zeros out whenever any field score is zero. This made specialists ineligible to vote on cross-field proposals, which was not the intended behavior. The arithmetic mean over non-zero fields preserves specialist participation while still diminishing returns on concentration through the outer sqrt.)*

### 7.2 Proposal Types

```
ProposalType :=
    | ParameterChange(parameter_id, old_value, new_value)
    | FieldSchemaAdd(field_id, schema_arweave_tx)
    | OracleWhitelistUpdate(oracle_id, action: Add | Remove)
    | PenaltyParameterUpdate(parameter_id, value)
    | EmergencyRetraction(claim_id, evidence_arweave_tx)
```

### 7.3 Proposal Lifecycle

```
Proposed
  → Voting (duration: governance_period)
  → Quorum met + majority Yes → Passed
  → Quorum not met or majority No → Rejected
  → (if Passed) Timelock (execution_delay)
  → Enacted
```

Quorum threshold and execution delay are themselves governable parameters, set conservatively at genesis:

- Initial quorum: 10% of total active SBT weight
- Initial execution delay: 7 days
- Voting period: 14 days

### 7.4 Capture Resistance

The governance mechanism is resistant to capture by design:

- **No token purchases** — SBTs cannot be bought; governance weight requires demonstrated epistemic contribution
- **Field specificity** — an institution that publishes heavily in one field cannot control governance in another
- **AGPL-3.0** — any fork of the protocol must remain open; a captured fork cannot be privatized
- **On-record governance** — protocol changes require recorded votes in the append-only history, not maintainer approval; no single repository owner can unilaterally change the protocol

### 7.5 v0.2 Governance Implementation: Signed Multi-Signature

*(New in wp-v0.2.)*

The governance mechanism at v0.2 is implemented as a signed multi-signature scheme over the append-only history, not as a state machine on a validator-set chain. The design satisfies the four governance requirements (R10a–R10d; see §10 and Appendix C) with substantially less infrastructure than a chain-based implementation.

**Mechanism.**

1. A governance set G of n named participants is established at genesis, each holding a public key bound to their DID.
2. A governance action is a signed record of form `(ProposalId, ProposalType, Outcome, {signature_i}_{i ∈ S})` where S ⊆ G with `|S| ≥ k` for a genesis-fixed threshold k.
3. Signed governance actions are submitted to the append-only history. Their inclusion timestamp establishes the timelock start.
4. The state-derivation program honors any signed governance action it observes — from the primary log or from any independent republication — once the timelock has elapsed. The log is the *primary distribution channel*; signatures are the *enforcement mechanism*.

**Why this satisfies R10.**

- **R10a (objective event)**: inclusion in the log with valid k-of-n signatures is objective; inclusion proofs are mathematical.
- **R10b (binding)**: the state-derivation program is deterministic; all honest operators compute identical state after the action.
- **R10c (detectable non-compliance)**: an operator whose output ignores a valid signed governance action diverges from honest operators' outputs; divergence is detectable.
- **R10d (no unilateral action)**: a single signer cannot enact governance; k signers must cooperate. Log-operator censorship of a signed governance action is defeated by republication: the state-derivation program honors the action from any source, not only from the primary log.

**Scope at v0.2.** The governance set G is composed of the pilot participants (§11). k is set at a genesis-fixed threshold proportional to the expected operational load of v0.2. The expected count of actual governance actions during the v0.2 demonstration is approximately zero (see §11.4); the mechanism is declared and testable, but is not expected to be exercised at scope.

**v1.0 upgrade path.** If participant scale at v1.0 makes multi-signature coordination inadequate (either because n becomes too large for practical signature collection or because the governance set itself needs to rotate based on evolving SBT scores), the governance mechanism may be upgraded to a chain-based state machine. The upgrade is a governance action under the v0.2 multi-sig mechanism itself — the protocol upgrades its own governance through its existing governance. This self-referential property is preserved by the ordering commitment in §1.5: governance is a reinforcement mechanism around the ECG, and a reinforcement mechanism may be upgraded in place without invalidating the ECG it protects.

---

## 8. Identity Layer

### 8.1 W3C Decentralized Identifiers

All participants in Apodokimos are identified by Decentralized Identifiers conforming to the W3C DID Core specification (W3C, 2022). No real-name requirement exists at the protocol level.

DID method: `did:apodokimos:<account_id>`

*(Revision note: wp-v0.1 specified `did:substrate:apodokimos:<account_id>`. The method is renamed to remove implementation-stack specificity.)*

A DID document registered at creation associates the DID with:

- A public key for signing claims and attestations
- Optional credential proofs (see §8.2)
- Optional service endpoints

### 8.2 ZK Credential Proofs

The protocol supports anonymous-but-credentialed participation via zero-knowledge proofs (Goldwasser, Micali, & Rackoff, 1985). A researcher can prove they satisfy a field SBT minimum threshold or hold an external credential (e.g., medical license, PhD in relevant field) without revealing identity.

The ZK proof scheme:

```
Prover holds: credential C issued by authority A
Verifier wants: proof that prover holds credential in category K
Prover reveals: ZKProof(C, K) — proves membership without revealing C or identity
```

This enables, for example:

- Proving board-certified physician status for clinical medicine field attestations
- Proving PhD in relevant field without revealing institution affiliation
- Meeting minimum SBT threshold without revealing absolute score

The specific ZK scheme (Groth16 per Groth, 2016; or PLONK per Gabizon, Williamson, & Ciobotaru, 2019) is defined at v0.2.0 of the reference implementation when the DID service is implemented.

### 8.3 Privacy Model

The append-only history contains:

- Claim hashes (not claim content)
- Attestation records with DID references (not personal data)
- SBT scores per field (not personal data by default)

Claim content and review payloads are stored on Arweave, referenced by hash. Personal data is never written to the history. This design is compatible with GDPR: the right to erasure applies to personal data, and personal data exists off-chain where it can be removed. The history hash of a deleted document becomes an orphaned pointer — the claim entry persists (attestations already made remain valid) but the content is no longer retrievable.

---

## 9. Arweave Content Layer

### 9.1 Rationale

Claim content must be:

- **Permanent** — a claim registered today must be retrievable in 100 years
- **Immutable** — content cannot be altered after registration
- **Decentralized** — not dependent on Apodokimos infrastructure remaining operational
- **Content-addressed** — the recorded hash uniquely identifies the content

Arweave's permaweb (Williams et al., 2019) satisfies all four constraints. Miners are economically incentivized to store data permanently via the endowment model. No Apodokimos server needs to run for claim content to remain accessible.

### 9.2 Canonical Claim JSON Schema

All claims are serialized to a canonical JSON format before upload. Canonical means deterministic: same logical content always produces the same byte sequence, and therefore the same blake3 hash. Field ordering is alphabetical. Floating point values use a fixed precision representation.

Arweave transaction tags for a claim upload:

```json
{
    "App-Name":          "apodokimos",
    "App-Version":       "0.2.0",
    "Content-Type":      "application/json",
    "Claim-Type":        "<ClaimType variant>",
    "Field-Id":          "<field_schema_id>",
    "Claim-Hash":        "<blake3_hex_of_canonical_json>",
    "Schema-Version":    "<field_schema_version>",
    "Spec-Version-DOI":  "<version_doi_of_whitepaper_in_force_at_registration>"
}
```

The `Claim-Hash` tag enables content verification on fetch: the client recomputes the blake3 hash of the downloaded content and compares it to the tag. A mismatch indicates corruption or tampering and the client rejects the content.

The `Spec-Version-DOI` tag binds the claim to the specification version under which it was registered (§1.6, §11.4). This tag is required and immutable; the state-derivation program uses it to apply the correct historical rules and schemas when scoring the claim, regardless of how the specification has evolved since registration.

### 9.3 Attestation Content

Attestation evidence is also stored on Arweave, structured as:

```json
{
    "claim_id":   "<ClaimId>",
    "verdict":    "<AttestationVerdict>",
    "narrative":  "<reviewer's reasoning in free text>",
    "evidence": [
        {
            "type":  "ArweaveRef | ExternalDOI | TrialRegistryId",
            "ref":   "<reference>"
        }
    ],
    "field_id":   "<FieldId>",
    "schema":     "<attestation_schema_version>"
}
```

The `narrative` field provides qualitative context that the attestation record cannot. It is permanently retrievable but does not affect W(claim) computation directly.

---

## 10. Implementation Architecture

*(This section is substantially rewritten in wp-v0.2. wp-v0.1 specified a four-pallet Substrate parachain architecture as the implementation. wp-v0.2 re-expresses the requirements independent of any particular stack and identifies a reference implementation that satisfies them with minimal infrastructure.)*

### 10.1 Requirements-First Framing

The protocol's mechanism requirements are defined by the ECG model (§2–§5), the SBT lifecycle (§6), the governance mechanism (§7), and the content layer (§9). These requirements are implementation-independent. An implementation is correct if it satisfies all requirements; the particular stack and its internal architecture are secondary.

The thirteen derived requirements are enumerated in Appendix C. They fall into five categories:

- **Append-only verifiable history** (R1, R2, R3, R4): claims and attestations are recorded with authenticated authorship, content addressing, and credible timestamps.
- **Stateful enforcement of protocol rules** (R5, R6, R7, R8): DAG acyclicity, reputation-gated writes, non-transferable SBT, and deterministic W(c, t) computation.
- **Verifiable derived state** (R9): W(c, t) scores and SBT state can be independently recomputed and verified.
- **Governance and survival** (R10, R11, R12): governance actions bind all operators; the protocol survives any single operator; the protocol is forkable.
- **Specification coherence over time** (R13): each claim binds to the spec version under which it was registered; the ECG history remains coherent across decades of specification evolution.

### 10.2 Reference Implementation

The wp-v0.2 reference implementation composes four existing technology categories, each chosen for fit to a specific requirement class:

**History layer — Append-only transparency log.** Claims and attestations are recorded as signed entries in a Merkle-tree transparency log in the Certificate Transparency / Rekor lineage (Laurie, Langley, & Kasper, 2013; Newman, Meyers, & Torres-Arias, 2022). The log supports inclusion proofs (any reader can verify an entry is in the log) and consistency proofs (the log cannot be rewritten). Multiple operators may run mirrors; independent witnesses co-sign Signed Tree Heads to prevent split-view attacks. This layer satisfies R1, R2, R3, R11, and R12.

**Timestamp anchoring — OpenTimestamps.** Log tree heads are Merkle-batched and anchored in Bitcoin (Nakamoto, 2008) via OpenTimestamps (Todd, 2016). Any reader with a Bitcoin node can independently verify that a log entry existed before a given block. This provides decades-durable timestamp credibility without depending on Apodokimos running any chain. This layer satisfies R4.

**Content layer — Arweave.** Claim content and attestation evidence are stored on Arweave as specified in §9. Arweave transaction IDs are recorded in the transparency log; content hashes are verified on fetch. This layer is unchanged from wp-v0.1.

**State-derivation program.** A deterministic Rust program reads the transparency log, applies the protocol's rules (DAG acyclicity from §4.1, SBT gating from §2.3 and §6, W(c, t) computation from §3, penalty propagation from §5, governance actions from §7), and produces the current protocol state. The program is pure: given the same log contents, it produces the same state. For each claim, the program reads the claim's `spec_version_doi` (§2.2, §11.4) and applies the schema and scoring rules from that specific specification version — past claims retain their original semantics regardless of how the specification has evolved since registration. Multiple operators run the program independently; their outputs must agree. Merkle roots of state snapshots are recorded in the log for efficient verification. This layer satisfies R5, R6, R7, R8, and R9.

**Governance — signed multi-signature.** Governance actions are k-of-n signed records submitted to the log (§7.5). The state-derivation program honors valid signed actions after their timelock expires. This layer satisfies R10.

### 10.3 What This Reference Implementation Is Not

This architecture is not a blockchain. It does not have a validator set, a consensus protocol, a native token, a gas market, or block production. These absences are deliberate:

- Byzantine consensus is a mechanism for resolving conflicting simultaneous writes. Apodokimos, per the ordering in §1.5 and the scope analysis in §11, does not have conflicting-write conditions at meaningful rates.
- A native token reintroduces capital-weighted dynamics into the layer beneath reputation-weighted governance, creating a structural tension with §6's non-capital commitment.
- Block production imposes latency and operational overhead that is unnecessary for the protocol's actual write rates.

The transparency log provides the properties a blockchain would provide for this use case — append-only verifiable history, content addressing, cryptographic proofs of inclusion and consistency — without the properties a blockchain would additionally impose.

### 10.4 Alternative Implementations

The requirements enumerated in Appendix C do not uniquely determine the reference implementation above. Alternative implementations that satisfy all thirteen requirements are equally valid:

- **Substrate parachain** (the wp-v0.1 architecture; see Wood, 2016). Substrate's FRAME pallet pattern naturally supports R5–R8 via explicit dispatchable function sets. R10 is satisfied by an on-runtime governance pallet rather than signed multi-signature. This architecture is more infrastructure-heavy and introduces a relay-chain coupling, but offers forkless runtime upgrades and a mature ecosystem.
- **Cosmos SDK + CometBFT**. A sovereign chain implementing protocol rules as Cosmos modules. Properties comparable to Substrate with a smaller surface area and no relay-chain dependency; upgrades are coordinated hard forks rather than forkless.
- **Hybrid.** A transparency log for history and Arweave for content (as in the reference implementation) combined with a narrow-scope chain for SBT state and governance only. Preserves the minimal-infrastructure property of the reference implementation while providing chain-enforced governance at v1.0 if multi-signature becomes inadequate.

Any implementation that satisfies R1–R13 is a valid Apodokimos implementation. The protocol is defined by its requirements, not by its reference implementation.

### 10.5 Scope: v0.2 vs v1.0

The reference implementation is targeted at wp-v0.2 scope, corresponding to the clinical medicine bootstrap demonstration (§11). At this scope, the expected counts of participants, writes, and governance actions make a minimal-infrastructure implementation correct and sufficient.

At v1.0 mainnet scope — permanent infrastructure expected to operate across decades with unknown participant counts — the reference implementation may or may not remain sufficient. The v1.0 implementation decision is explicitly deferred. It is constrained by the requirements in Appendix C but not by the specific architecture here.

The v0.2 implementation choice does not bind v1.0. The protocol's versioning independence (Appendix B) covers this: a v1.0 protocol running on a different architecture may carry forward the v0.2 history by replaying the log into the v1.0 state, provided the v1.0 implementation also satisfies R1–R13.

---

## 11. Bootstrap Strategy: Clinical Medicine

### 11.1 Rationale for Domain-Specificity

A general-purpose claim registry with no initial population has no signal. The coordination problem demands a domain where:

1. Claim granularity is already standardized (reduces the granularity problem)
2. Real-world outcome oracles already exist (enables O factor from day one)
3. The IF failure is most documented and felt most acutely
4. A motivated community exists outside the prestige economy

Clinical medicine satisfies all four. The PICO (Population, Intervention, Comparator, Outcome) framework (Richardson, Wilson, Nishikawa, & Hayward, 1995) provides a standardized claim template already used globally in evidence-based medicine. Trial registries (ClinicalTrials.gov, WHO ICTRP) provide structured real-world outcome data. The 85% waste estimate (Chalmers & Glasziou, 2009) and the replication crisis in clinical research are well-documented motivators.

### 11.2 PICO Claim Schema v0.1

```json
{
    "$schema":         "https://apodokimos.science/fields/clinical-medicine-v0.1.json",
    "field_id":        "clinical-medicine",
    "schema_version":  "0.1.0",
    "license":         "CC0-1.0",
    "claim_template": {
        "population":            { "type": "string", "required": true,
                                   "description": "Patient population or study participants" },
        "intervention":          { "type": "string", "required": true,
                                   "description": "Intervention, exposure, or treatment" },
        "comparator":            { "type": "string", "required": false,
                                   "description": "Control or comparison group" },
        "outcome":               { "type": "string", "required": true,
                                   "description": "Primary outcome measure" },
        "effect_direction":      { "type": "enum", "required": true,
                                   "values": ["positive", "negative", "null", "mixed"] },
        "effect_size":           { "type": "number", "required": false,
                                   "description": "Point estimate of effect size" },
        "confidence_interval":   { "type": "array", "items": "number",
                                   "minItems": 2, "maxItems": 2, "required": false,
                                   "description": "[lower_bound, upper_bound]" },
        "trial_registry_id":     { "type": "string", "required": false,
                                   "description": "NCT number or equivalent trial registry ID" },
        "prospero_id":           { "type": "string", "required": false,
                                   "description": "PROSPERO systematic review registration ID" }
    }
}
```

### 11.3 O Factor Oracles for Clinical Medicine

**ClinicalTrials.gov oracle:**

```
input:  NCT ID from claim.trial_registry_id
output: OFactorScore
    0.3 if trial registered but not yet completed
    0.6 if trial completed, results posted
    0.9 if trial completed + results published + consistent with claim
    0.1 if trial completed + results inconsistent with claim
```

**PROSPERO oracle:**

```
input:  PROSPERO ID from claim.prospero_id
output: OFactorScore
    0.2 if protocol registered, review not completed
    0.7 if systematic review completed and published
    0.9 if systematic review completed + meta-analysis supports claim
    0.1 if systematic review completed + meta-analysis contradicts claim
```

Both oracles are queried by the state-derivation program via public APIs. Oracle sources are whitelisted by governance action. Multiple oracle sources for the same claim produce `O = max(source_scores)` to avoid penalizing claims that only have one linkage type.

### 11.4 Bootstrap Sequence

The bootstrap pilot registers a set of well-characterized clinical claims where the evidence record is already established — both the original claims and their replication/refutation history. This seeds the ECG with a non-trivial graph immediately, demonstrating the scoring model on real data before any new claims are submitted.

**Target seed dataset.** 50 claims from Cochrane systematic reviews, covering well-replicated interventions and well-documented failures. All claims submitted by pilot researchers who co-design the process.

**Minimum participant set for a credible v0.2 demonstration** *(new in wp-v0.2):*

- Submitters: 1–3 pilot researchers. Sufficient to register the 50 seed claims and the whitepaper as the first ECG claim.
- Attesters: 5–10 credentialed reviewers. Each seed claim should receive at least 2 attestations; partial overlap among attesters is required for SBT increment logic to exercise.
- State-derivation operators: 2 minimum, independent. Demonstrating that W(c, t) recomputes identically across independent operators is the core verifiability claim of the architecture in §10.
- Oracle operators: 1 per source at v0.2 (ClinicalTrials.gov, PROSPERO), acceptable because oracle whitelisting is governance-managed and v0.2 is the mechanism demonstration, not the operator-set decentralization demonstration.

Approximate total: 8–15 real participants for a credible v0.2 demonstration.

**Self-registration.** The Apodokimos whitepaper itself (this document, wp-v0.2) is registered as an ECG claim on the demonstration history. Protocol validates its own founding document. (wp-v0.1 was similarly registered; wp-v0.2 supersedes it as the current specification while the v0.1 claim remains a permanent record in the history.)

**Version-DOI binding in claim metadata.** Each registered claim records the Version DOI of the specification it was registered under (per §1.6). Concretely, the claim's Arweave content carries a `spec_version_doi` field whose value is the immutable Version DOI of the whitepaper that defines the schema and rules in force at registration time. The state-derivation program (§10.2) reads this field and applies the schema and scoring rules from that specific specification version, not the latest. This makes the ECG history coherent under specification evolution: a claim registered under wp-v0.2 is scored under wp-v0.2's rules indefinitely, even after wp-v0.5 has revised the schema.

The whitepaper-as-first-claim self-registers with `spec_version_doi` equal to its own Version DOI, completing the self-validation loop: the document defines the protocol; the protocol's first claim is the document; the claim references the document's own immutable address.

---

## 12. Security Analysis

### 12.1 Sybil Attack on Reviewer Identity

**Vector.** An adversary creates many DIDs to flood attestations for or against a claim, or to fragment reputation across many accounts for governance gain.

**Mitigation.**

- **Minimum SBT threshold to attest.** A new DID with no track record cannot attest at all (§10.3 of wp-v0.1 equivalent; mechanism requirement R6 per Appendix C). The threshold is governed and can be raised in response to attacks.
- **SBT accumulation is expensive per identity.** Scores increment only on attestation validation and claim survival (§6.3). Manufacturing many accounts with non-trivial SBT requires performing the underlying epistemic work for each identity separately, at roughly the same marginal cost as a legitimate researcher doing the same work once. There is no shortcut.
- **ZK credential proofs tie attestation to external credentials** (§8.2). For fields where credentials are hard to obtain (medical license, relevant PhD), the credential itself is the scarce resource the adversary must counterfeit or acquire.
- **DID-to-SBT one-to-one binding.** The protocol enforces a single SBT record per `(DID, field)`. Creating additional DIDs does not multiply reputation for a single real identity.
- **No on-chain reward for account creation itself.** Sybil accounts gain nothing by existing; they must do attestation work to become operational.

**What quadratic voting does and does not contribute** *(corrected in wp-v0.2)*. Quadratic voting (§7.1) provides *concentration resistance* — diminishing the governance power of any single highly-concentrated reputation — but it does not provide Sybil resistance. Under sqrt weighting, fragmenting a score across many accounts strictly increases total vote weight (§7.1). Sybil resistance is carried entirely by the SBT accumulation cost and the identity layer; the voting formula addresses a different threat.

This separation matters for threat modeling: a Sybil attacker must defeat the identity layer, not the voting formula. A capital-rich attacker who wishes to concentrate governance weight must defeat the voting formula, but cannot bypass the identity layer. Different attacks require different mechanisms.

*(Revision note from wp-v0.1. The original §12.1 attributed Sybil resistance to quadratic voting's subadditivity, citing `sqrt(n) < n × sqrt(1)`. The cited inequality is mathematically correct but its interpretation was inverted: subadditivity means fragmentation increases total weight, not decreases it. The Sybil-resistance argument is restructured in wp-v0.2 to locate the mechanism correctly.)*

### 12.2 Governance Plutocracy

**Vector.** A wealthy actor accumulates governance control.

**Mitigation.**

- SBTs are non-transferable — cannot be purchased on a market
- Governance weight is `sqrt(field_score)` — diminishing returns on concentration
- Field specificity — no single actor can dominate cross-field governance without demonstrated expertise across all fields
- AGPL-3.0 — even if governance is captured, a community fork must remain open and can restore the original governance model

### 12.3 Claim Spam

**Vector.** Adversary floods the registry with low-quality claims to degrade signal-to-noise ratio.

**Mitigation.**

- Registration deposit — refunded on first attestation, burned if claim receives no attestations within a governance-defined window
- Field schema validation — claims that do not conform to the field schema are rejected
- SBT threshold for registration — very low threshold at launch, raiseable by governance

### 12.4 Oracle Manipulation

**Vector.** An adversary manipulates an oracle data source to inflate O factor scores.

**Mitigation.**

- Oracle sources are whitelisted by governance — adding a manipulated oracle requires SBT-weighted action
- Multiple oracle sources per claim — `O = max(source_scores)` means one manipulated source cannot pull down legitimate scores, but also means a single compromised source cannot inflate a score if other sources disagree
- Oracle results are recorded in the history — manipulation is auditable
- ClinicalTrials.gov and PROSPERO are public government/institutional databases — manipulation requires compromising external infrastructure, not just the Apodokimos oracle layer

### 12.5 GDPR Right to Erasure

**Vector.** A researcher requests deletion of personal data, but data is in an immutable history.

**Mitigation.**

- No personal data is written to the history — only claim hashes, DID references, and SBT scores
- Claim content and review payloads are on Arweave, not in the history
- DID-to-identity mapping is controlled by the researcher — they can abandon a DID
- Arweave content can be removed by the uploader (though permanence guarantees are lost) — the history hash becomes an orphaned pointer
- The history record of a deleted claim's attestations remains valid — the attestations happened; their content evidence becomes unavailable but their verdicts persist

This design is compliant with GDPR by construction: the history stores no personal data to erase.

### 12.6 Protocol Capture Resistance

*(Revised from wp-v0.1's "Chain Capture" section for stack-agnostic framing.)*

**Vector.** A large institution forks the protocol and operates a modified version that re-introduces IF-equivalent metrics, or a single operator captures the primary history and uses censorship to enforce captured behavior.

**Mitigation.**

- **AGPL-3.0** requires any hosted fork to publish source — the fork is visible and auditable. Any divergence from the published protocol is detectable.
- **Protocol governance is recorded** — protocol changes require valid k-of-n signed governance actions (v0.2) or on-chain votes (v1.0+ if upgraded); no single entity can unilaterally change protocol parameters.
- **The canonical history is forkable** — an operator who censors entries or publishes invalid state-derivation outputs is detectable by independent operators and auditors. The protocol's state is a pure function of the log; any community member with log access can fork the canonical state and continue operation.
- **Timestamp anchoring is external** — OpenTimestamps' Bitcoin anchor is not under Apodokimos' control. An attempt to rewrite history past the first Bitcoin confirmation is detectable by any party with a Bitcoin node.
- **No single operator controls all layers** — Arweave is a separate network with its own economics; the transparency log has multiple operators and external witnesses; the Bitcoin timestamp anchor is entirely external. Capturing the protocol would require simultaneous capture of all three independent systems.

*(Rationale for revision: wp-v0.1's mitigation relied specifically on "Polkadot governance, not Apodokimos" controlling the parachain slot. Because wp-v0.2 no longer commits to Substrate/Polkadot, the capture-resistance argument is generalized to the cross-layer independence property that holds regardless of implementation.)*

---

## 13. Limitations and Open Problems

The following are known open problems that are not resolved in wp-v0.2 and are acknowledged honestly.

**Granularity gaming.** A motivated actor could decompose a single result into many micro-claims to artificially inflate D. The field schema mitigates but does not eliminate this. Governance can introduce claim clustering rules at a later version.

**O factor automation reliability.** The oracle connectors query public APIs that can change structure, go offline, or return inconsistent data. Multi-source O factor and governed oracle whitelisting reduce but do not eliminate this risk.

**Cold start signal quality.** The bootstrap seed dataset provides initial signal but remains limited. Early W(claim) scores in underrepresented fields will have high variance until sufficient attestation density accumulates.

**ZK scheme selection.** The identity layer ZK proof scheme is deferred to a later reference-implementation version. The choice between Groth16 (efficient verification, trusted setup required) and PLONK (no trusted setup, larger proof size) has security and operational tradeoffs not yet resolved.

**Cross-field claim classification.** Some claims are genuinely interdisciplinary. The current model assigns a single `field_id` per claim. Multi-field claims will be addressed in a future schema version.

**Log-operator censorship in v0.2 governance** *(new in wp-v0.2).* The multi-signature governance mechanism in §7.5 defends against unilateral censorship by allowing signed governance actions to be honored from any source, not only from the primary log. In practice, however, participants may not know to look beyond the primary log. This is an operational limitation rather than a mechanism weakness: the mechanism permits bypass of a censorious log operator, but effective bypass requires the community to monitor for and act on such censorship. v1.0 implementations using multi-operator logs (e.g., Trillian federated deployments) or chain-based governance reduce this operational burden.

**Witness ecosystem bootstrap** *(new in wp-v0.2).* The reference implementation depends on independent witnesses co-signing transparency-log Signed Tree Heads to prevent split-view attacks. At v0.2 demonstration scope the witness set is small and partially overlaps with the pilot participant set. Full split-view resistance requires a witness ecosystem with operators that are independent of the pilot. This is a recognized bootstrapping gap.

---

## 14. Conclusion

Apodokimos does not propose a better metric for papers. It proposes the elimination of the paper as the atomic unit of scientific evaluation, replacing it with the falsifiable claim — the unit that reality actually cares about.

The weight function `W(c, t) = R(t) × D × S × O` measures four things that matter: whether a claim has been independently replicated, how load-bearing it is in the knowledge graph, whether it has survived falsification attempts, and whether it has propagated to real-world consequence. None of these are captured by IF.

The governance model — quadratic SBT voting, non-transferable, identity-bound — eliminates the capture dynamic that has ended every prior reform attempt. You cannot buy Apodokimos governance. You can only earn it by being right, repeatedly, over time. Governance is defensive infrastructure around the ECG, not the protocol's primary contribution.

The bootstrap strategy acknowledges the coordination problem honestly. Apodokimos does not launch as a universal replacement for IF. It launches in clinical medicine, where the tools for claim standardization and real-world outcome measurement already exist, where the evidence of IF failure is most quantified, and where the motivation to exit the prestige economy is highest.

The implementation architecture (§10) is requirements-driven rather than stack-committed. The reference implementation composes a transparency log, Arweave, OpenTimestamps, and a deterministic state-derivation program — substantially less infrastructure than a blockchain, sufficient for the protocol's actual requirements. Alternative implementations that satisfy the same requirements are equally valid. The protocol is defined by what it must do, not by how it must do it.

The protocol is owned by no one. It is licensed under AGPL-3.0. Any fork must remain open. The first claim it registers is itself.

---

## References

*(Expanded and alphabetized in wp-v0.2. All entries now correspond to inline citations in the body text. See Appendix D for the revision log.)*

Bar-Ilan, J., & Halevi, G. (2017). Post retraction citations in context: a case study. *Scientometrics*, 113(1), 547–565.

Brand, A., Allen, L., Altman, M., Hlava, M., & Scott, J. (2015). Beyond authorship: attribution, contribution, collaboration, and credit. *Learned Publishing*, 28(2), 151–155.

Chalmers, I., & Glasziou, P. (2009). Avoidable waste in the production and reporting of research evidence. *The Lancet*, 374(9683), 86–89.

DORA (San Francisco Declaration on Research Assessment). (2013). https://sfdora.org/read/

Gabizon, A., Williamson, Z. J., & Ciobotaru, O. (2019). PLONK: Permutations over Lagrange-bases for Oecumenical Noninteractive arguments of Knowledge. *IACR Cryptology ePrint Archive*, 2019/953.

Garfield, E. (1955). Citation indexes for science. *Science*, 122(3159), 108–111.

Goldwasser, S., Micali, S., & Rackoff, C. (1985). The knowledge complexity of interactive proof-systems. In *Proceedings of the 17th Annual ACM Symposium on Theory of Computing (STOC '85)*, 291–304.

Groth, J. (2016). On the size of pairing-based non-interactive arguments. In *Advances in Cryptology — EUROCRYPT 2016*, 305–326. Springer.

Hicks, D., Wouters, P., Waltman, L., de Rijcke, S., & Rafols, I. (2015). Bibliometrics: The Leiden Manifesto for research metrics. *Nature*, 520, 429–431.

Ioannidis, J. P. A. (2005). Why most published research findings are false. *PLOS Medicine*, 2(8), e124.

Laplace, P. S. (1774). Mémoire sur la probabilité des causes par les événements. *Mémoires de Mathématique et de Physique, présentés à l'Académie Royale des Sciences*, 6, 621–656. *(Foundational treatment of additive smoothing used in §3.2 and §3.4.)*

Laurie, B., Langley, A., & Kasper, E. (2013). *Certificate Transparency*. RFC 6962, Internet Engineering Task Force.

Lalley, S. P., & Weyl, E. G. (2018). Quadratic Voting: How mechanism design can radicalize democracy. *AEA Papers and Proceedings*, 108, 33–37.

McKiernan, E. C., Schimanski, L. A., Muñoz Nieves, C., Matthias, L., Niles, M. T., & Alperin, J. P. (2019). Meta-Research: Use of the Journal Impact Factor in academic review, promotion, and tenure evaluations. *eLife*, 8, e47338.

Nakamoto, S. (2008). *Bitcoin: A peer-to-peer electronic cash system*.

Newman, Z., Meyers, J. S., & Torres-Arias, S. (2022). Sigstore: Software signing for everybody. In *Proceedings of the 2022 ACM SIGSAC Conference on Computer and Communications Security (CCS '22)*, 2353–2367.

Nicholson, J. M., Mordaunt, M., Lopez, P., Uppala, A., Rosati, D., Rodrigues, N. P., Grabitz, P., & Rife, S. C. (2021). scite: A smart citation index that displays the context of citations and classifies their intent using deep learning. *Quantitative Science Studies*, 2(3), 882–898.

O'Connor, J., Aumasson, J.-P., Neves, S., & Wilcox-O'Hearn, Z. (2020). *BLAKE3: one function, fast everywhere*. Specification and reference implementation.

Open Science Collaboration. (2015). Estimating the reproducibility of psychological science. *Science*, 349(6251), aac4716.

Richardson, W. S., Wilson, M. C., Nishikawa, J., & Hayward, R. S. (1995). The well-built clinical question: a key to evidence-based decisions. *ACP Journal Club*, 123(3), A12–A13.

Todd, P. (2016). *OpenTimestamps: scalable, trust-minimized, distributed timestamping with Bitcoin*.

W3C. (2022). *Decentralized Identifiers (DIDs) v1.0: Core architecture, data model, and representations*. W3C Recommendation, 19 July 2022.

Weyl, E. G., Ohlhaver, P., & Buterin, V. (2022). Decentralized Society: Finding Web3's soul. *SSRN Working Paper 4105763*.

Williams, S., Diordiiev, V., Berman, L., Raybould, R., & Uemlianin, I. (2019). *Arweave: A Protocol for Economically Sustainable Information Permanence*. Arweave Yellow Paper.

Wood, G. (2016). *Polkadot: Vision for a heterogeneous multi-chain framework*.

---

## Appendix A: Notation Summary

| Symbol | Meaning |
|---|---|
| `G = (C, A, E)` | Epistemic Contribution Graph |
| `C` | Set of registered claims (nodes) |
| `A` | Set of attestations (typed edges) |
| `E ⊆ C × C` | Dependency relation |
| `W(c, t)` | Epistemic weight of claim c at time t |
| `R(c, t)` | Replication score with time decay, `R ∈ (0, 1)` |
| `D(c)` | Raw dependency depth (ancestor count) |
| `D̃(c)` | Log-normalized dependency depth, `D̃ > 0` |
| `D_ref_field` | Governed reference depth per field (median or 75th percentile) |
| `S(c)` | Survival rate, `S ∈ (0, 1)` |
| `O(c)` | Real-world outcome linkage, `O ∈ [0, 1]` |
| `γ` | Governed outcome-bonus coefficient; default `γ = 1` |
| `δ(c)` | Retraction discount, `δ ∈ [0, 1]`, initialized to 1.0 |
| `α, β` | Beta pseudocounts (Laplace smoothing); default `α = β = 1` |
| `t_½_field` | Field-specific half-life for replication time decay |
| `w_decay(Δt, t_½)` | Time decay function: `2^(−Δt / t_½)` |
| `Θ_field` | Retraction cascade threshold per field |
| `μ_field(t)` | Running mean W for field normalization |
| `W_pre` | Pre-retraction weight snapshot (used in penalty propagation) |
| `G` (governance) | Named governance set of multi-signature participants (§7.5) |
| `k`, `n` | Threshold and size of governance multi-signature (§7.5) |
| `VersionDOI` | Immutable DOI of a specific whitepaper version (§1.6); appears in every claim's metadata as `spec_version_doi` |
| `ConceptDOI` | Moving DOI pointer to the latest whitepaper version (§1.6); used for navigation, never for archival citation |

---

## Appendix B: Versioning

This whitepaper is versioned independently of the protocol reference implementation.

| WP Version | Status | Notes |
|---|---|---|
| wp-v0.1 | Superseded | Initial release, Substrate-parachain implementation commitment, ZK scheme TBD. DOI: 10.5281/zenodo.19583091 |
| wp-v0.2 | Draft | Stack-agnostic implementation section (§10), primary/secondary ordering (§1.5), versioning and citation convention (§1.6), multi-signature governance (§7.5), derived requirements R1–R13 (Appendix C), corrected weight-function math (Laplace smoothing, log-normalized depth, outcome as multiplicative bonus, explicit retraction discount δ), full inline citation of body-text claims to a 25-entry bibliography — see Appendix D for full change log |
| wp-v1.0 | Planned | Coincides with protocol v1.0 mainnet; audited, stable; implementation architecture locked based on v0.2 demonstration experience |

---

## Appendix C: Derived Requirements

*(New in wp-v0.2.)*

The protocol's mechanism requirements, derived from §2–§9. An implementation is correct if and only if it satisfies all thirteen.

**R1 — Append-only ordered record.** Once a claim or attestation is recorded, the record does not change. Records have a consistent ordering. New records append; existing records do not mutate. *(From §2, §4.3.)*

**R2 — Content-addressed binding.** `ClaimId = blake3(canonical_json(content))`. The record's identifier is a cryptographic hash of its canonical content. Anyone possessing the content can verify its identifier. *(From §2.2, §9.2.)*

**R3 — Authenticated authorship.** Each record is signed by the author's DID key. The signature is verifiable independently of the protocol's infrastructure. *(From §2.2, §8.1.)*

**R4 — Credible timestamps.** Each record has a timestamp that someone who distrusts the protocol's operators can verify. *(From §3.2 [R(t) requires credible ordering for time decay], §7.3 [timelock requires credible time].)*

**R5 — DAG acyclicity at write.** A new claim may not create a cycle in the dependency relation. Cycle-creating writes are rejected, not accepted and later cleaned up. *(From §4.1.)*

**R6 — Reputation-gated writes.** An attestation requires the attester's field SBT score to meet a governed threshold at the time of writing. The check is non-bypassable. *(From §2.3, §6, §10.3 [of wp-v0.1].)*

**R7 — SBT non-transferability as mechanism.** The implementation exposes no transfer operation for SBT scores. Non-transferability is a property of the code, not of access-control rules over a transfer function. *(From §6.3.)*

**R8 — Deterministic score computation.** Given the same input record set, any two correct implementations of the W(c, t) computation produce the same output. *(From §3; implicit in protocol correctness.)*

**R9 — Independently verifiable derived state.** W(c, t) scores and SBT state can be recomputed and verified by any party with access to the record history. *(From §9 [implicit verifiability model], §10.3 [of wp-v0.1; Merkle-anchor of scores].)*

**R10 — Governance binds future operators.** Valid governance actions, once enacted, modify the protocol's behavior for all compliant operators. Decomposed:

- **R10a** — Enactment of a governance action is objective: all honest operators agree whether and when it was enacted.
- **R10b** — After enactment, state derivation reflects the action.
- **R10c** — An operator ignoring a valid enacted action is detectable by auditors.
- **R10d** — No single operator (including the primary log operator) can unilaterally enact or block a governance action. *(From §7.)*

**R11 — Operator independence.** The protocol survives any single operator losing interest, being compromised, or being legally constrained. State and history remain accessible; operation can continue with a different operator set. *(From §12.6.)*

**R12 — Forkability.** The protocol's state and history are reproducible by any community member with log and content access. A community fork in response to capture can carry forward all prior history. *(From AGPL-3.0 commitment; §12.6.)*

**R13 — Spec-version binding.** Each claim records the immutable Version DOI of the specification under which it was registered. State derivation applies the schema and rules from that specific version, not the latest. The ECG history remains coherent under specification evolution: past claims retain their original semantics indefinitely. *(From §1.6, §2.2, §9.2, §11.4.)*

---

## Appendix D: Change Log from wp-v0.1 to wp-v0.2

*(New in wp-v0.2. Surgical enumeration of changes.)*

### Structural changes (first revision round)

**§1.5 added** — Primary and Secondary Commitments. Names the ordering between ECG (primary) and governance (secondary) explicitly.

**§6.3 revised** — non-transferability enforcement language generalized from Substrate-specific ("disabled at the runtime level") to mechanism-requirement form ("absence of transfer code paths").

**§7 extended** — new §7.5 specifying multi-signature governance for wp-v0.2 scope, with R10a–R10d satisfaction argument and v1.0 upgrade path.

**§8.1 revised** — DID method generalized from `did:substrate:apodokimos` to `did:apodokimos`.

**§9 preserved verbatim** except for notational changes (App-Version bumped to 0.2.0).

**§10 substantially rewritten** — was "Substrate Parachain Architecture" (implementation-specific); now "Implementation Architecture" (requirements-first, stack-agnostic). The four-pallet design of wp-v0.1 is relocated to §10.4 as one valid implementation among several.

**§11.4 extended** — minimum participant set for credible v0.2 demonstration specified (8–15 real participants).

**§12.6 revised** — "Chain Capture" renamed "Protocol Capture Resistance"; mitigation argument generalized from Polkadot-specific to cross-layer independence.

**§13 extended** — two new acknowledged limitations (log-operator censorship, witness ecosystem bootstrap).

**§14 revised** — conclusion updated to reflect primary/secondary ordering and stack-agnostic implementation framing.

**References extended** — added RFC 6962 (Certificate Transparency), OpenTimestamps, Sigstore.

**Appendix C added** — Derived Requirements R1–R13.

**Appendix D added** — this change log.

**Appendix B updated** — wp-v0.2 row added with supersession note.

### Mathematical corrections (second revision round)

The following fixes address bugs in wp-v0.1's formulas that caused the weight function to contradict its own narrative intent. Each fix is a design decision and is named explicitly.

**Bug B1 — Multiplicative zero-collapse.** wp-v0.1's `W = R × D × S × O` zeroed W whenever any single factor was zero. Under wp-v0.1's definitions, this meant:
- Every newly-registered claim (no attestations) had `R = 0, S = 0`, therefore `W = 0`.
- Every terminal claim (no dependents) had `D = 0`, therefore `W = 0`.
- Every basic-science claim (no real-world linkage) had `O = 0`, therefore `W = 0` — contradicting §3.5's narrative claim that basic science is not penalized.
- The 50 Cochrane seed claims (§11.4) and the self-registered whitepaper would all score zero at bootstrap.

**Fix B1.** Each factor redesigned with a strictly positive baseline:
- R uses Laplace smoothing (uniform Beta prior): baseline R = 0.5 for no replication evidence.
- D replaced with D̃, log-normalized with `+1` offsets: baseline `D̃ > 0` for terminal claims.
- S uses Laplace smoothing: baseline S = 0.5 for no attestations.
- O enters as a multiplicative bonus `(1 + γ · O)` rather than a direct factor: `O = 0` contributes no penalty.
- New retraction discount δ (§3.7) is the explicit zero-path for retracted claims.

**Design choices made in this fix (documented for review).**
- Uniform Beta prior `α = β = 1` for Laplace smoothing. Alternatives: field-calibrated priors, Jeffreys prior (α = β = 1/2), or governance-tunable priors. Uniform is the principled default; field-calibration can be added as a governance action.
- Log-normalization for D̃ rather than arctan-normalization or rank-based normalization. Log preserves ordinal ranking under any monotonic rescaling and is well-understood.
- Outcome bonus coefficient γ = 1 by default. Alternatives: field-specific γ, larger γ to amplify the clinical-translation signal. γ is governable.
- Retraction discount δ as multiplicative factor rather than additive penalty. Multiplicative composes cleanly under cascades; additive requires a floor and does not compose.

**Bug B2 — Inverted Sybil-resistance argument.** wp-v0.1 §12.1 claimed quadratic voting made Sybil fragmentation provide "no advantage" via sqrt subadditivity. The math works the other direction: fragmenting increases total weight. Sybil resistance was attributed to the wrong mechanism.

**Fix B2.** §12.1 rewritten to locate Sybil resistance in (a) SBT accumulation cost per identity, (b) ZK credential proofs tying attestation to external credentials, (c) DID-to-SBT one-to-one binding. §7.1 clarified to distinguish concentration resistance (quadratic voting's role) from Sybil resistance (identity layer's role).

**Bug B3 — Undefined terms in penalty propagation.** wp-v0.1 §5.2 used `weight_of_c_retracted_in_dep_set` and `total_dep_weight` without defining them.

**Fix B3.** §3.7 and §5.2 now define `penalty_fraction(c, c_retracted) = W_pre(c_retracted) / Σ W_pre(d)` explicitly, with `W_pre` being the pre-retraction weight snapshot.

**Bug B4 — Penalty persistence mechanism undefined.** wp-v0.1 specified `apply_penalty(c_dep, penalty)` but W was computed on demand from graph state; there was no field for penalty to persist in.

**Fix B4.** New claim field `δ(c)` (§3.7) serves as the explicit persistence mechanism. Retraction events mutate δ; W incorporates δ directly as `W = R × D̃ × S × (1 + γ O) × δ`.

**Bug B5 — Time decay units unspecified.** wp-v0.1 used `e^(-λ × Δt)` without specifying units for λ or Δt.

**Fix B5.** Time decay reparameterized as `2^(−Δt / t_½)` using half-life, which makes the governable parameter directly interpretable. Δt is in the same units as `RecordTimestamp` (implementation-specific but consistent within any given deployment).

**Bug B6 — O-factor discontinuity.** wp-v0.1 defined `O ∈ {0} ∪ [0.1, 1.0]` with a gap between "no linkage" and "minimal linkage."

**Fix B6.** O simplified to `O ∈ [0, 1]` and the discontinuity eliminated by the multiplicative-bonus form `(1 + γ · O)`.

**Bug B7 — Cross-field geomean zeros specialists.** wp-v0.1 §7.1 used geometric mean of field scores for cross-field voting, which is zero if any field score is zero.

**Fix B7.** Cross-field weight uses arithmetic mean over non-zero field scores: `vote_weight(account, global) = sqrt(mean_nonzero(field_scores(account)))`. Specialists retain full weight; generalists get averaged weight.

**Bug B8 — Type claim on S.** wp-v0.1 §3.4 stated `S ∈ (0, 1)` (open interval), but the formula permitted `S = 0` (with empty numerator).

**Fix B8.** With Laplace smoothing, `S ∈ (0, 1)` strictly — the type claim now matches the formula.

### Properties preserved across revisions

- All four W-factor semantics (replication, dependency depth, survival, real-world linkage) preserved.
- The claim taxonomy (§2.2.1) and attestation taxonomy (§2.3.1) unchanged.
- The DAG acyclicity invariant unchanged.
- The SBT non-transferability mechanism requirement unchanged.
- The Arweave content layer unchanged.
- The bootstrap-clinical-medicine strategy unchanged.

### Properties that require re-validation under the new formulas

- Attestation threshold values in §7.3 ("Initial quorum: 10% of total active SBT weight") must be recalibrated against the new cross-field voting formula.
- Field-specific `t_½_field`, `α`, `β`, `D_ref_field`, `γ`, `Θ_field` require calibration during the bootstrap (§11). Values should be set conservatively at genesis and adjusted by governance once real data accumulates.
- The PROSPERO and ClinicalTrials.gov oracle score mappings (§11.3) were designed for wp-v0.1's `O ∈ {0} ∪ [0.1, 1.0]`. They remain valid for wp-v0.2 but are now interpreted as input to the bonus term `(1 + γ · O)` rather than as a direct multiplier.

### Bibliographic anchoring (third revision round)

wp-v0.1 contained a References section whose entries were not inline-cited anywhere in the body text — references existed but were not connected to specific claims. wp-v0.2 (this round) anchors every factual and conceptual claim to a source.

**Inline citations added.**

- Abstract and §1.1 — Garfield (1955) for IF's original definition; McKiernan et al. (2019) for IF's persistence in tenure/promotion.
- §1.1 — Ioannidis (2005) for prior evidence of replication failure rates.
- §1.3 — DORA (2013), Hicks et al. (2015) for Leiden Manifesto, Nicholson et al. (2021) for Scite, Brand et al. (2015) for CRediT. The unverifiable "2021 EMBO Reports commentary" claim from wp-v0.1 was replaced with McKiernan et al. (2019)'s eLife study, which measures post-DORA persistence of IF use in hiring, promotion, and tenure with methodology that is publicly reproducible.
- §2.2 — O'Connor et al. (2020) for BLAKE3.
- §3.2 — Laplace (1774) for foundational smoothing.
- §5.3 — Bar-Ilan & Halevi (2017) for the post-retraction citation phenomenon (previously claimed as "a documented phenomenon" without citation).
- §6.1 — Weyl, Ohlhaver, & Buterin (2022) for the Soulbound Token concept.
- §7.1 — Lalley & Weyl (2018) for quadratic voting.
- §8.1 — W3C (2022) for DID Core specification.
- §8.2 — Goldwasser, Micali, & Rackoff (1985) for zero-knowledge proofs; Groth (2016) for Groth16; Gabizon, Williamson, & Ciobotaru (2019) for PLONK.
- §9.1 — Williams et al. (2019) for Arweave.
- §10.2 — Laurie, Langley, & Kasper (2013), Newman, Meyers, & Torres-Arias (2022), Nakamoto (2008), Todd (2016) for the transparency log and anchoring stack.
- §10.4 — Wood (2016) for Polkadot/Substrate.
- §11.1 — Richardson et al. (1995) for the PICO framework.

**References section.** Alphabetized and expanded from 13 to 25 entries. All newly-cited works added. wp-v0.1's Buterin et al. (2022) citation corrected to its full author list (Weyl, Ohlhaver, & Buterin, 2022) to match the Decentralized Society paper's actual authorship. wp-v0.1's Weidener & Spreckelsen (2024) entry was removed: the reference was never cited inline in wp-v0.1, and its exact bibliographic details could not be independently verified to the standard this whitepaper commits to elsewhere. Removing an unverified reference is preferable to retaining one that a reader cannot trust.

**Claim replaced due to source uncertainty.** The "2021 EMBO Reports commentary" claim in wp-v0.1 §1.3 was replaced with McKiernan et al. (2019) as a more load-bearing and verifiable citation for the same point. The original claim's source could not be confirmed; the replacement source is a peer-reviewed empirical study of IF use in academic evaluation.

**Appendix A and Appendix D updated** to reflect bibliographic changes.

### Versioning convention (fourth revision round)

wp-v0.1 anchored to Zenodo as a single deposit with one DOI. wp-v0.2 introduces an explicit convention for handling the multi-version lineage that the protocol's own self-validation thesis (§11.4) and its archival commitments (§9.1) require. The convention prevents a class of citation-drift errors that would otherwise accumulate over decades of specification evolution.

**§1.6 added — Versioning and Citation Convention.** Distinguishes Version DOIs (immutable, per-version, used for archival citation) from Concept DOIs (moving pointer to the latest version, used for navigation). Establishes which contexts use which identifier. Documents the placeholder discipline for drafts and the CI-enforcement requirement for published artifacts.

**Header revised.** wp-v0.2's header now carries placeholder strings `10.5281/zenodo.19763292` and *see Zenodo "Versions" panel on the v0.1 record (10.5281/zenodo.19583091)* for the Version DOI and Concept DOI respectively. These placeholders are filled in at the moment of Zenodo upload, not before. The header includes an inline citation-discipline note pointing readers to §1.6.

**§2.2 Claim struct extended.** New required field `spec_version_doi: VersionDOI` on every claim. The field records the immutable Version DOI of the specification under which the claim was registered. This is the binding mechanism that makes specification evolution coherent across the ECG's lifetime.

**§9.2 Arweave tags extended.** New tag `Spec-Version-DOI` accompanies every claim's Arweave upload. The tag is required and immutable. Combined with the existing `Claim-Hash` and `Schema-Version` tags, every claim carries within its content-addressed envelope the full set of references needed to reconstruct its original semantics.

**§10.2 state-derivation program extended.** The program reads each claim's `spec_version_doi` and applies the schema and scoring rules from that specific specification version, not the latest. Past claims retain their original semantics regardless of how the specification has evolved since registration.

**§11.4 self-registration paragraph extended.** Documents how the version-DOI binding closes the self-validation loop: the whitepaper-as-first-claim self-registers with `spec_version_doi` equal to its own Version DOI. The document defines the protocol; the protocol's first claim is the document; the claim references the document's own immutable address.

**Appendix A notation extended** with `VersionDOI` and `ConceptDOI`.

**Appendix C extended with R13 — Spec-version binding.** Promotes the versioning convention from a documentation guideline to a formal protocol requirement. Implementations that do not record and honor `spec_version_doi` per claim do not satisfy R13. The requirement count increases from twelve to thirteen across §10.1, §10.4, §10.5, and Appendix C.

**Operational note.** When wp-v0.2 is anchored to Zenodo, the recommended path is to use Zenodo's "New version" feature on the existing wp-v0.1 deposit rather than creating an independent deposit. This produces a Concept DOI shared between v0.1 and v0.2 (and any future versions), with each version retaining its own permanent Version DOI. wp-v0.1's existing DOI (`10.5281/zenodo.19583091`) becomes its Version DOI within the linked record; nothing about v0.1 is altered or invalidated.

**Why this is structural, not cosmetic.** The protocol is committed to long-term scientific archival across decades. Without an explicit citation discipline, both internal artifacts (claim metadata, source code) and external citations would drift toward whichever DOI was most prominent in the README at the moment of writing. Within a few specification versions, the corpus of citations to "the Apodokimos protocol" would be a mixture of Version DOIs and Concept DOIs with no way to distinguish "the author of this 2027 claim cited the spec-as-it-was-in-2027" from "the author cited the latest spec, whatever that is." The §1.6 convention prevents this drift by codifying the rule before any drift can begin.

---

*End of wp-v0.2 draft.*

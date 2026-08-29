# Phase A implementation classification

This overlay classifies the predecessor Rust semantic/public surface and schemas without allowing
implementation occurrence to define either v2.0 or successor meaning. It closes an authority and
source-incidence boundary; it does not prove that Rust correctly implements any TeX relation.

## Exact admitted field

`PREDECESSOR_IMPLEMENTATION_CLASSIFICATION_SCHEMA.json` admits exactly:

- 64 Rust semantic-module candidates;
- 1,989 public Rust surface identities;
- 6 canonical wire fixtures; and
- 3 storage migrations.

Every item remains predecessor implementation or schema evidence. `ImplementationOnly` classifies
that source authority; it does not prove that the artifact has no semantic correspondence. For an
item without an admitted edge, future correspondence therefore remains `Unknown`.

## Edge contraction

A naive exact-word join produces 10,300 edges dominated by generic vocabulary such as question,
occurrence, operator, continuation, coverage, and identity. Those edges are rejected. A claim edge
survives only when a PascalCase public Rust name occurs in the TeX under one of these exact forms:

```text
\mathsf{Name}
\texttt{Name}
\operatorname{Name}
```

Plain words, case folding, stems, synonyms, headings, comments, and fuzzy similarity create no
edge. At the pinned inputs this leaves 15 exact TeX occurrences for 7 public symbols across 6
modules. Public records carry direct edge provenance; modules carry only the target union of their
public edges and preserve every contributing public identity.

The 13 public/module records with an edge are `LegacyObligation`, not accepted correspondence.
Their candidate proposition is an unelaborated `ImplementsCandidate`, their status is `Unproved`,
and no breaker is fabricated. Phase F must independently establish, correct, or reject each edge.
The other 2,049 records are `ImplementationOnly` under their source authority.

## Regeneration and independent checks

```text
node tools/predecessor_implementation_classification.js check
node tools/predecessor_implementation_classification_check.js
```

The independent checker reads pinned Git blobs and exact TeX ranges itself. It rejects missing
modules/items/schemas, duplicate identities, foreign or fuzzy claim edges, blanket
`ImplementationOnly`, blanket correspondence, promoted edge authority, erased module provenance,
detached source digests, fixture/schema role collapse, and Formal Gate A self-promotion.

Complete implementation review closes only `FORMAL-A-RUST-SURFACE-INVENTORY`. The corrected 36
Rust conformance modules and 190 fixture-row occurrences are reviewed separately under
`FORMAL-A-FIXTURE-INVENTORY`. Formal Gate A remains pending until the final cross-surface coverage
relation closes.

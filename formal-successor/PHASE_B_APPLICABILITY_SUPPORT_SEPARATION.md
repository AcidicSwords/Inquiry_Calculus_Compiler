# Phase B applicability/support separation

## Status

`FORMAL-B-APPLICABILITY-SUPPORT-SEPARATION-001` is a checked predecessor reconstruction
at v2.0 lines 4615–4632. It does not pass Formal Gate B or promote a successor primitive.

The boundary retains four exact `LegacyObligation` records at v2.0 lines 4615–4632. Two
remain `Unproved` and two remain `Ambiguous`. The Lean model is a precise typed reading of
the named profile and noncollapse claims; it does not silently discharge the stored prose.

## Retained profile

Every record contains seven independent coordinates: relation, scope, applicability,
support family, negative boundary, warrant class, and certificate references. `MayUse` reads
only the supplied applicability predicate. `HasEvidentialSupport` reads only the supplied
support predicate. Neither relation is defined from the other.

`Deactivate` changes applicability alone. It definitionally retains relation, scope, support,
negative boundary, warrant class, and certificate references.

## Decisive contrast

The finite model supplies:

- one active, supported, warranted record;
- its inactive version, with every non-applicability coordinate identical;
- an active record with no evidential support.

The inactive record cannot be used but retains its warrant class and certificate references.
The active unsupported record can be used but has no support. Thus current applicability,
support, current usability, and historical warrant ancestry remain independently visible.

## Machine checks

`node tools/applicability_support_separation_check.js --compile` checks exact source
identity and classification, builds with warnings rejected, audits thirteen proof declarations
for axioms, and rejects fourteen source ablations.

## Coverage and reopening

This pass supplies no applicability decision engine, support search, warrant decision,
certificate validation, deletion semantics, execution, Rust change, successor promotion, or
Gate-B passage. Reopen if any profile coordinate collapses, use becomes support, deactivation
erases historical material, inactive becomes synonymous with unsupported/deleted, source
classification changes, proof dependencies expand, or an ablation escapes.

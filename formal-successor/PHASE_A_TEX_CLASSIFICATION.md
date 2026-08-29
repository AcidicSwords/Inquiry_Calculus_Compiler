# Phase A TeX classification

This overlay classifies every generated v2.0 TeX candidate under the six dispositions required by
the construction specification. It closes a source-review boundary; it does not prove a v2.0
claim, select a successor primitive, or pass Formal Gate A.

## Classification boundary

`PREDECESSOR_TEX_CLASSIFICATION_SCHEMA.json` admits exactly the 1,370 generated TeX identities:

- 203 explicit declaration environments;
- 516 narrative boundaries;
- 426 list components; and
- 225 mathematical display components.

The policy distinguishes source role before disposition. Explicit `definition` environments route
to `FormalDefinition`. Explicit theorem, proposition, and corollary environments route to
`FormalTheorem`, except that results under the exact admitted linear binding heading route to
`BindingTheorem`. Laws and other unproved declaration kinds remain `LegacyObligation`. The exact
rendering command `\maketitle` is `CanonicalProseOnly`.

Narrative, list, and display extraction does not by itself establish a standalone proposition
boundary. Those units therefore remain source-bound `LegacyObligation` records. An explicit
normative signal records `Unproved`; absence of such a signal records `Ambiguous`, never
expository or negative: keyword absence cannot discharge it. Each obligation carries its source coordinate, unelaborated candidate
proposition identity, scope, referenced-label dependencies, status, and explicit absence of an
established breaker.

This is a conservative classification, not a semantic shortcut. Phase B must elaborate, split,
prove, refute, bind, correct, or otherwise discharge each legacy obligation. It may not replace
the source-bound placeholder with an axiom.

## Regeneration and independent checks

```text
node tools/predecessor_tex_classification.js check
node tools/predecessor_tex_classification_check.js
```

The first command regenerates the overlay exactly from the pinned inventory, schema, and TeX. The
second independently rejoins every exact source range and rejects whole-section or single-item
omission, duplicate or foreign identities, keyword-only classification, blanket theorem
promotion, invalid legacy status, lost display boundaries, detached source digests, binding
collapse, and Formal Gate A self-promotion.

The generated overlay reports `FORMAL-A-TEX-INVENTORY` as ready for independent checking rather
than warranting itself. Durable conformance standing belongs only in `CONFORMANCE_STATUS.md` after
the independent checker passes. Formal Gate A remains pending while Rust/schema/fixture review and
claim-edge coverage remain open.

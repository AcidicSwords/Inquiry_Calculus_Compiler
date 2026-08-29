# Phase A joined coverage

Formal Gate A asks one narrow question: does every predecessor item have a formalization destination
or explicit classification? It does not ask whether any unresolved claim is true or whether the
successor has already selected the right primitives.

## Exact partition

`PHASE_A_COVERAGE_CERTIFICATE.json` is a rebuildable, non-self-warranting projection over the
corrected source inventory and three reviewed overlays. It partitions all 3,662 source identities:

- 1,370 TeX identities owned by the TeX overlay;
- 2,062 Rust semantic/public and schema identities owned by the implementation overlay;
- 226 conformance-module and fixture-row identities owned by the fixture overlay; and
- 4 crate manifests already explicitly classified as predecessor build evidence in the inventory.

The partition has no intersection, unowned identity, or foreign owner. The certificate also checks
29 implementation-to-TeX candidate edges, 40 fixture-overlay-to-TeX edges including module
aggregates, 1,928 direct fixture-to-implementation candidate incidences, 6 fixture-file witnesses,
200 execution routes, and 187 conformance-module fixture routes. Every target is inside its declared
reviewed universe.

## Independent acceptance

```text
node tools/phase_a_coverage.js check
node tools/phase_a_coverage_check.js
```

The first command only regenerates the candidate certificate. It cannot pass Gate A. The second
independently reconstructs ownership and target validity from the source inventory and overlays. It
rejects 19 whole-class, source-omission, cross-owner duplication, foreign-source, foreign-target,
standing-promotion, manifest, digest, retained-boundary, and Gate A self-promotion mutations.

The independent result establishes Gate A at exactly this coverage. All 1,236 TeX
`LegacyObligation` records, 13 implementation correspondence candidates, 10 fixture rows without an
exact test-function or fixture-file witness relation, and 226 `Unknown` successor standings survive
unchanged into Phase B/F work. Total ownership is not semantic proof.

## Next boundary

Phase B must formalize the predecessor surface with no `sorry` and leave every unresolved claim as
an explicit obligation. It begins from the classified ambient boundary and dependency order; it
must not repair gaps by axiom or import Rust behavior as meaning.

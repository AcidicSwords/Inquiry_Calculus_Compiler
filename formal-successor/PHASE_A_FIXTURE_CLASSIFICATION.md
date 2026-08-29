# Phase A fixture and conformance-test classification

This overlay preserves the predecessor conformance field as typed evidence without allowing a
status label, registry entry, test declaration, or green execution to warrant successor semantics.
It closes the local fixture/test review boundary only; the joined Phase A coverage relation remains
open.

## Corrected source field

The first fixture pass exposed a source-extraction error before classification began. The original
fixture regular expression omitted one-hyphen and slash-bearing labels, truncated variant letters,
and keyed rows only by displayed label. It therefore reported 166 records, invented three
registry-only gaps, and merged repeated provider rows.

The corrected extraction retains 190 exact status-row occurrences carrying 185 displayed labels.
`PROVIDER-001A`, `PROVIDER-001`, `PROVIDER-001B`, and `PROVIDER-001C` account for five additional
source occurrences. All 19 executable-registry labels have exact status rows; registry-only count
is zero. A registry relation is joined only through one unique displayed label. Repeated labels
remain distinct source occurrences and do not inherit an ambiguous registry relation.

## Typed graph

`PREDECESSOR_FIXTURE_CLASSIFICATION.json` reviews exactly 36 integration-test modules and 190
fixture-row occurrences. It separately retains:

- the exact predecessor status assertion (`PASS` or `PENDING`);
- exact `#[test]` and `#[tokio::test]` declarations at the pinned commit;
- status-row witness names and deliberately registered executable routes;
- exact fixture-file paths;
- exact complete fixture-label incidence in reviewed TeX ranges;
- exact PascalCase public-symbol incidence in masked executable test bodies; and
- `Unknown` successor standing for every record.

At the pinned boundary, 174 rows have exact test-function routes, six have exact canonical fixture
file witnesses, and ten preserve unresolved or non-function witness prose. The 200 exact row-to-test
routes include 19 registered routes and reach all 36 conformance modules. Nineteen exact fixture-ID
occurrences reach reviewed TeX records. Public-symbol edges are syntactic execution-body candidates,
not call-graph proofs or semantic correspondence.

The predecessor ledger contains 189 `PASS` rows and one `PENDING` row. Those are source assertions
with their original scope and reopen conditions. The overlay neither erases them nor copies their
authority into the successor.

## Checks and breakers

```text
node tools/predecessor_fixture_classification.js check
node tools/predecessor_fixture_classification_check.js
```

The independent checker re-reads pinned Git blobs, reconstructs test bodies without importing the
generator, and rejoins the reviewed TeX and implementation overlays. It rejects fixture or module
deletion, repeated-label collapse, registry redirection, route deletion, fabricated TeX or
implementation edges, promoted successor standing, altered predecessor status, detached test
identity, erased registry provenance, blanket disposition, detached input digests, and Gate A
self-promotion.

Complete fixture review closes only `FORMAL-A-FIXTURE-INVENTORY`. Formal Gate A remains `PENDING`
until `FORMAL-A-COVERAGE-CHECKER` independently checks the joined TeX, implementation, fixture, and
source-universe coverage relation.

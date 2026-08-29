# Formal successor proof project

This pinned Lean 4/Lake project is the candidate semantic authority for the Inquiry Calculus
successor. A compiled file establishes only the claims it contains; it does not by itself satisfy a
formal gate or warrant promotion.

## Run locally

```text
cd formal
lake update
lake build --wfail
```

The lock manifest is committed after dependency resolution. CI also runs `leanchecker` over both
declared libraries, enforces no-`sorry`, and audits axioms from the aggregate `Spec` import root.

## Layer direction

The construction specification fixes this dependency direction:

```text
Meta
-> Core
-> Questions / Programs / Evidence
-> Transform / Compression / Methods / Language
-> Models / Bindings / Legacy
-> Successor
-> generated Spec
```

Only `Meta/Ambient.lean` is admitted at setup. Remaining recommended directories are created when
their first typed obligation becomes live; empty modules are not evidence of progress.

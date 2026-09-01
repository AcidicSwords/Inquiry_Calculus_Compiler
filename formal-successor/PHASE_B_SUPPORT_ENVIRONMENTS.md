# Phase B support environments

Status: local checked predecessor-definition realization; Formal Gate B remains pending.

The exact source surface is the two `FormalDefinition` records at v2.0 lines 4516–4533. A
candidate support environment contains finite or finitely represented typed premises, actual
returns, checker results, assumptions, and applicable standing relations for a candidate. The
construction specification supplies the necessary sharpening: support is an explicit relation,
not a consequence of membership, adjacency, or environment construction.

The checked interface therefore separates candidate environments, content interpretation,
support, warrant, and standing. Its minimal support family uses proper environment inclusion and
is a predicate rather than a selected basis. It neither asserts a least environment nor uniqueness.

The decisive finite model has two independently supporting two-element environments: one uses a
premise and actual return, while the other uses a checker result and assumption. They are
incomparable and both subset-minimal. Their union remains supported but is nonminimal. Removing one
element from either route breaks that route, and a separately constructed candidate environment is
unsupported. Warrant and standing remain false even for a supported environment.

Twelve source-coordinate ablations are rejected and twenty proof contracts are axiom-free. An
initial `Finset` model was discarded because its quotient implementation expanded the proof
dependency surface; the retained duplicate-free `List` representation preserves the same boundary
without quotient, choice, or propositional-extensionality axioms.

This local model does not define support search, support certificates, open dependencies, closed
support, warrant sufficiency, standing construction, execution, Rust semantics, successor
primitives, or Gate B. The represented-content constructor is typed but the decisive model exercises
the finite case only.

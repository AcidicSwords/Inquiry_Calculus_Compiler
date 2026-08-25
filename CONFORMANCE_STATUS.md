# Conformance Status

**Last updated:** 2026-08-25
**Standing semantics:** Inquiry Calculus v1.1, Successor Reciprocal-Boundary / Positive-Negation Edition
**Implemented boundary:** Phase 1 typed-form and binding-identity kernel

Passing means only that the named executable fixture distinguishes its stated behavior.
It does not imply completion of a later phase or of the full v1.1 specification.
Adopting successor semantic authority does not make any successor fixture executable
or passing.

## Demonstrated Phase 0 fixtures

| Fixture | Status | Witness |
|---|---|---|
| P0-ART-001 canonical envelope round trip | PASS | `artifact_identity::matches_independent_known_vector` and property test |
| P0-ART-002 stable external byte/hash vector | PASS | `fixtures/artifacts/envelope-v1.json` |
| P0-ART-003 kind/schema domain separation | PASS | `kind_and_schema_version_domain_separate_identity` |
| P0-ART-004 malformed/truncated/trailing input rejection | PASS | `rejects_malformed_envelopes` |
| P0-ART-005 kind/reference validation | PASS | `validates_kinds_and_reference_text` |
| P0-STORE-001 embedded migration and repeat run | PASS | `migrations_apply_and_repeat_without_schema_changes` |
| P0-STORE-002 exact insert/fetch and idempotence | PASS | `insertion_fetch_and_duplicate_insertion_are_exact` |
| P0-STORE-003 reference mismatch/conflict rejection | PASS | two insertion rejection fixtures |
| P0-STORE-004 corruption detection on read | PASS | `fetch_detects_corrupt_envelope_and_reference` |
| P0-GATE-001 formatting/check/clippy/tests | PASS | local Phase 0 gate run on 2026-08-24 |

## Demonstrated Phase 1 fixtures

| Fixture | Status | Witness |
|---|---|---|
| P1-TY-001 canonical type grammar round trip | PASS | `type_identity::canonical_type_grammar_round_trips_and_domain_separates` |
| P1-TY-002 stable named-type byte/hash vector | PASS | `fixtures/types/type-v1-named.json` |
| P1-TY-003 `Prog(A)` and unary `Code(A)` domain separation | PASS | `canonical_type_grammar_round_trips_and_domain_separates` |
| P1-TY-004 malformed type and typed-form encoding rejection | PASS | `rejects_malformed_type_and_typed_form_encodings` |
| P1-TY-005 binding, child identity, and dependent-family checks | PASS | `checks_binding_scope_children_and_dependent_family_domain` |
| P1-FORM-001 typed-form declaration canonical identity and binding scope | PASS | `checks_binding_scope_children_and_dependent_family_domain` |
| P1-STORE-001 explicit reference dependency transaction | PASS | `referencing_insert_requires_declared_dependencies_before_commit` |

## Demonstrated Phase 2 formula fixtures

| Fixture | Status | Witness |
|---|---|---|
| P2-FORM-001 complete canonical formula surface round trip | PASS | `formula_identity::complete_canonical_formula_surface_round_trips_without_normalization` |
| P2-FORM-002 stable top-formula byte/hash vector | PASS | `fixtures/formulas/formula-v1-top.json` |
| P2-FORM-003 malformed formula encoding rejection | PASS | `rejects_malformed_formula_encodings` |
| P2-FORM-004 typed terms, quantifier capture safety, and formula contexts | PASS | `checks_typed_terms_capture_safe_quantification_and_contexts` |
| P2-FORM-005 logical negation remains a distinct formula constructor | PASS | `complete_canonical_formula_surface_round_trips_without_normalization` |
| P2-REL-001 native relation-schema byte/hash vector and canonical round trip | PASS | `fixtures/relations/relation-v1-native.json` |
| P2-REL-002 formula-defined relation exact port context | PASS | `formula_defined_relation_requires_the_exact_named_port_context` |
| P2-REL-003 atom signature arity and argument-type checks | PASS | `checks_atom_arity_and_types_against_a_resolved_named_signature` |
| P2-REL-004 duplicate named-port and malformed relation rejection | PASS | `rejects_duplicate_ports_and_malformed_relation_encodings` |
| P2-REL-005 relation-use occurrence identity, typed bindings, and scope context | PASS | `relation_use_is_a_distinct_typed_and_scoped_occurrence` |
| P2-QUERY-001 direct OpenQuery round trip and typed complete port partition | PASS | `open_query_is_a_complete_partition_with_a_nonempty_open_section` |
| P2-QUERY-002 empty or overlapping open-port partitions reject | PASS | `open_query_is_a_complete_partition_with_a_nonempty_open_section` |
| P2-QUERY-003 checked `Bind` and `Expose` preserve a nonempty typed open section | PASS | `open_query_is_a_complete_partition_with_a_nonempty_open_section` |
| P2-QUERY-004 stable OpenQuery byte/hash vector | PASS | `fixtures/queries/open-query-v1-single-open.json` |
| P2-QUERY-005 complete typed `Plug` produces a candidate assignment, not a relation result | PASS | `open_query_is_a_complete_partition_with_a_nonempty_open_section` |

## Pending specification and plan categories

| Category | Status | First planned phase |
|---|---|---|
| Typed forms, typed reification, and type verification | PARTIAL | Phase 1: canonical type artifacts, typed-form declarations, and structural checks pass; binding-native term/reification semantics remain pending |
| Relations, open ports, partial binding, and question kernels | PARTIAL | Phase 2: direct `OpenQuery` partial bindings, checked `Bind`/`Expose`, candidate-only `Plug`, formula artifacts, typed terms, relation schemas, atom-signature checks, and relation uses pass; normalization, composition, dependent binding, and completion-fiber views remain pending |
| First-order programs and proposal/actuality/authority separation | PENDING | Phase 3 |
| Successor determination and departure, fixtures 1-13 | PENDING | Phase 4 |
| Typed negation and coverage separation, fixtures 14-24 | PENDING | Phase 4 |
| Tagged multiple negation uses, fixtures 25-33 | PENDING | Phase 4 |
| Same-use return and protected recovery, fixtures 34-47 | PENDING | Phase 4 |
| Pure return versus warranted reconciliation, fixtures 48-52 | PENDING | Phase 4 |
| Dependent sixfold and downstream `Gamma`, fixtures 53-64 | PENDING | Phase 4 |
| Reciprocal representation and learning, fixtures 65-70 | PENDING | Phases 4, 12-16 |
| Cross-cutting determination, jointness, recovery-loss, regenerative-economy, method, growth, approximation, and consequence-subspace breakers | PENDING | Phases 4, 10, 12-16 |
| Return/Branch/Probe runtime and continuation descent | PENDING | Phase 5 |
| Actuality, ledger/domain ordering, resolution, and replay | PENDING | Phases 6-7 |
| Paired actuality and reciprocal residual reconstruction | PENDING | Phase 8 |
| Retained/access/active separation and recurrent memory crawl | PENDING | Phase 9 |
| Surface/backend/raw-return compilation boundaries | PENDING | Phase 10 |
| Positive standing, support environments, and rootless cycles | PENDING | Phase 11 |
| Separator generation, cue planning, and bounded unknown results | PENDING | Phases 12-13 |
| Materialization, expressibility, and representation gaps | PENDING | Phase 14 |
| Binding extension, bridges, rebinding, and history locality | PENDING | Phase 15 |
| Folding, recovery, compression licences, and reopening | PENDING | Phase 16 |
| Cross-binding standing lift | PENDING | Phase 17 |
| Predecessor-judged self-revision | PENDING | Phase 18 |
| Measured breadth and optimization only after semantic closure | PENDING | Phase 19 |

All 70 successor reciprocal-boundary fixtures are pending. Every unaffected predecessor
typing, compiler, actuality, history, standing, fold, binding, and self-revision fixture
also remains required. The former independent-round-trip reciprocal fixture survives
only as ancestry and negative-breaker evidence; it is not successor conformance.

The eleven cross-cutting derived breakers are also pending. They protect reusable
consequences without adding reciprocal roles: exact determination-through, informational
product versus joint actuality, conservative question-language growth, semantic method
residual versus operational failure, directional approximation soundness,
current-consequence versus inquiry-regenerative sufficiency, unknown versus witnessed
recovery loss, incomparable minimal cue bases, exact linear consequence subspaces,
the nonzero-mean covariance breaker, and query-distribution-sensitive reopening.

The pending set also includes canonical minimum witnesses more specific than the plan
summary: `top_B` refinement, recursive distinction composition, square
information-loss, question-kernel intersection, unified observation/action probes,
ledger-order versus domain-order separation, and fresh versus history-conditioned
probing. None is represented as a passing stub or fake verifier.

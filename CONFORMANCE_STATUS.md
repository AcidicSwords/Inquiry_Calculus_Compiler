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
| P2-QUERY-006 completion-fiber view revalidates its source query without selecting a completion | PASS | `open_query_is_a_complete_partition_with_a_nonempty_open_section` |
| P2-QUERY-007 direct normalization sorts to schema port order and is idempotent | PASS | `open_query_is_a_complete_partition_with_a_nonempty_open_section` |
| P2-REL-006 canonical data-only relation-expression grammar round trip | PASS | `canonical_relation_expression_grammar_round_trips_without_evaluation` |
| P3-IPROG-001 first-order Return/Ask canonical identities and explicit continuation reference | PASS | `iprog_identity::first_order_return_and_ask_round_trip_without_closures` |
| P3-IPROG-002 malformed first-order program rejection | PASS | `iprog_identity::rejects_malformed_inquiry_program_encodings` |
| P3-IPROG-003 explicit named environment identity, declared dependencies, and duplicate-name rejection | PASS | `iprog_identity::{first_order_return_and_ask_round_trip_without_closures,rejects_duplicate_explicit_environment_names}` |
| P3-IPROG-004 structural program checking revalidates result, typed forms, query, and continuation | PASS | `relation_schema::first_order_program_check_rejects_forged_or_result_mismatched_continuations` |
| P3-IPROG-005 stable Ask-with-environment byte/hash vector | PASS | `fixtures/iprogs/iprog-v2-ask-environment.json` |
| P4-DETERMINATION-IR-001 claim-local determination-presentation canonical identity and ancestry | PASS | `determination_identity::determination_presentations_round_trip_with_explicit_context_and_ancestry` |
| P4-DETERMINATION-IR-002 malformed determination-presentation rejection | PASS | `determination_identity::determination_presentations_reject_malformed_encodings` |
| P4-DETERMINATION-IR-003 typed source and context-preserving predecessor validation | PASS | `determination_identity::determination_presentation_check_rejects_forged_source_and_incompatible_ancestry` |
| P4-DEPARTURE-IR-001 positive departure-witness canonical identity | PASS | `departure_identity::departure_witnesses_round_trip_with_all_positive_evidence_roles` |
| P4-DEPARTURE-IR-002 malformed departure-witness rejection | PASS | `departure_identity::departure_witnesses_reject_malformed_encodings` |
| P4-DEPARTURE-IR-003 presentation, typed-form, and relation-use context validation | PASS | `relation_schema::departure_witness_check_requires_the_declared_presentation_and_context` |
| P4-CELL-IR-001 exact finite cell separator/exclusion coincidence across 65,536 binary pairs | PASS | `finite_cell::exact_finite_cell_exclusion_and_separator_existence_coincide_for_65536_pairs` |
| P4-CELL-IR-002 unknown finite observation remains neither separator nor same-cell conclusion | PASS | `finite_cell::{unknown_observation_never_becomes_a_positive_separator_or_same_cell_claim,comparison_rejects_mismatched_coordinate_schemas}` |
| P4-NEGATION-IR-001 oriented negation-use and semantic-coverage canonical identity | PASS | `negation_identity::negation_use_round_trips_and_keeps_semantic_coverage_distinct` |
| P4-NEGATION-IR-002 malformed negation-use rejection | PASS | `negation_identity::negation_use_rejects_malformed_encodings` |
| P4-NEGATION-IR-003 presentation/relation-use/program linkage validation | PASS | `relation_schema::negation_use_check_requires_one_oriented_presentation_context` |
| P5-RUNTIME-001 typed Return/Branch/Probe control flow and suspension/resume | PASS | `ic-runtime::program::verified_runtime_program_branches_suspends_and_preserves_raw_return_identity` |
| P5-RUNTIME-002 empty, dangling, and unguarded branch control-flow rejection | PASS | `ic-runtime::program::runtime_verifier_rejects_empty_dangling_and_unguarded_branch_control_flow` |
| P10-PROBE-IR-001 compiled probe-operator canonical identity and compiled-code separation | PASS | `probe_identity::probe_operator_identity_separates_compiled_code_from_request_and_return_data` |
| P10-PROBE-IR-002 malformed compiled probe-operator payload rejection | PASS | `probe_identity::probe_operator_rejects_noncanonical_payload_lengths` |
| P10-CONTRACT-IR-001 recurrent probe-contract canonical identity and field separation | PASS | `probe_contract_identity::probe_contract_identity_covers_each_contract_field` |
| P10-CONTRACT-IR-002 malformed recurrent probe-contract payload rejection | PASS | `probe_contract_identity::probe_contract_rejects_noncanonical_payload_lengths` |
| P6-RAW-001 raw-return opaque byte preservation and content identity | PASS | `raw_return_identity::raw_return_preserves_exact_opaque_bytes_and_domain_separates_identity` |
| P6-RAW-002 raw-return envelope-domain rejection | PASS | `raw_return_identity::raw_return_rejects_the_wrong_envelope_domain` |
| P6-RAW-003 raw-return opaque artifact-store round trip | PASS | `ic-store::tests::raw_returns_persist_as_opaque_immutable_artifacts_without_decoding` |
| P6-EVENT-IR-001 canonical event identity keeps required boundary and optional distinction distinct | PASS | `event_identity::actual_event_requires_boundary_and_keeps_optional_distinction_separate` |
| P6-EVENT-IR-002 event raw-return recheck without decoding | PASS | `event_identity::actual_event_rechecks_the_opaque_raw_return_without_decoding_or_interpreting_it` |
| P6-EVENT-IR-003 event boundary-chart and probe-operator recheck without inferring open contracts | PASS | `event_identity::actual_event_requires_a_rehashed_boundary_chart_without_checking_its_open_roles` |
| P6-EVENT-STORE-001 append-only ledger ordering, idempotence, raw/boundary/operator-kind rejection, and parent corruption detection | PASS | `ic-store::tests::{actual_events_append_in_order_and_recheck_stored_identity,actual_event_append_rejects_stale_parent_and_detects_ledger_corruption}` |
| P6-EVENT-STORE-002 file-backed journal restart and immutable ledger revalidation | PASS | `ic-store::tests::event_ledger_reopens_and_revalidates_immutable_history_after_restart` |
| P7-RESOLUTION-IR-001 typed resolution-route identity round trip | PASS | `resolution_identity::resolution_paths_round_trip_without_executing_their_route` |
| P7-RESOLUTION-IR-002 identity and exact composition-interface validation | PASS | `resolution_identity::resolution_paths_check_identity_and_exact_composition_interfaces` |
| P7-RESOLUTION-IR-003 malformed resolution-path rejection | PASS | `resolution_identity::resolution_paths_reject_malformed_envelopes` |
| P4-DETERMINE-THROUGH-001 exact finite kernel-inclusion factorization | PASS | `factorization::exact_factorization_constructs_the_target_map_when_kernels_are_included` |
| P4-DETERMINE-THROUGH-002 kernel separator and context/coverage mismatch rejection | PASS | `factorization::exact_factorization_returns_a_kernel_separator_and_rejects_incomplete_contexts` |
| P4-RECOVERY-IR-001 exact finite protected-signature constancy | PASS | `recovery::exact_fiber_recovery_requires_signature_constancy_and_emits_a_positive_separator` |
| P4-RECOVERY-IR-002 empty fiber rejection and unknown/loss separation | PASS | `recovery::empty_or_incomplete_evidence_is_not_conflated_with_non_recovery` |
| P4-FAMILY-IR-001 joint exact-family factorization without composite actuality | PASS | `family_factorization::exact_family_product_recovers_a_target_that_no_member_recovers` |
| P4-FAMILY-IR-002 empty/mismatched family rejection and joint separator | PASS | `family_factorization::exact_family_product_rejects_bad_coverage_and_exposes_joint_kernel_separators` |
| P4-BOUNDARY-IR-001 local boundary-chart canonical identity preserves tagged frontiers and open roles | PASS | `boundary_identity::boundary_chart_keeps_tagged_frontiers_and_absent_roles_explicit` |
| P4-BOUNDARY-IR-002 malformed chart optional-field rejection | PASS | `boundary_identity::boundary_chart_rejects_malformed_optional_and_count_fields` |

## Pending specification and plan categories

| Category | Status | First planned phase |
|---|---|---|
| Typed forms, typed reification, and type verification | PARTIAL | Phase 1: canonical type artifacts, typed-form declarations, and structural checks pass; binding-native term/reification semantics remain pending |
| Relations, open ports, partial binding, and question kernels | PARTIAL | Phase 2: direct `OpenQuery` partial bindings, data-only relation expressions, checked `Bind`/`Expose`, candidate-only `Plug`, direct schema-order normalization, a derived completion-fiber view, formula artifacts, typed terms, relation schemas, atom-signature checks, and relation uses pass; typed expression validation and dependent binding remain pending |
| First-order programs and proposal/actuality/authority separation | PARTIAL | Phase 3: canonical `Return`/`Ask` identity, explicit environment, malformed-input rejection, and recursive structural validation pass; supported-answer representation, substitution, normalization, pure operations, and runtime separation remain pending |
| Successor determination and departure, fixtures 1-13 | PENDING | Phase 4: claim-local determination-presentation identity and malformed-input rejection pass; a derived exact finite cell comparator demonstrates separator/exclusion behavior while preserving unknown; typed observation provenance, incompatibility admission, departure, and all successor fixtures remain pending |
| Typed negation and coverage separation, fixtures 14-24 | PARTIAL | Phase 4: oriented declaration identity, coverage-state separation, malformed-input rejection, and structural linkage pass; soundness, admission, execution coverage, and all successor fixtures remain pending |
| Tagged multiple negation uses, fixtures 25-33 | PARTIAL | Phase 4: exact finite tagged family product and joint-information factorization pass; admitted frontiers, coverage certificates, jointness evidence, and successor fixtures remain pending |
| Same-use return and protected recovery, fixtures 34-47 | PARTIAL | Phase 4: derived three-valued recovery status and exact finite signature-constancy checking pass; same-use fiber derivation/certification, occurrence context, and successor fixtures remain pending |
| Pure return versus warranted reconciliation, fixtures 48-52 | PENDING | Phase 4 |
| Dependent sixfold and downstream `Gamma`, fixtures 53-64 | PARTIAL | Phase 4: derived local boundary-chart identity preserves declared context and absent roles; sixfold reconstruction, field validation, and all successor fixtures remain pending |
| Reciprocal representation and learning, fixtures 65-70 | PENDING | Phases 4, 12-16 |
| Cross-cutting determination, jointness, recovery-loss, regenerative-economy, method, growth, approximation, and consequence-subspace breakers | PARTIAL | Phase 4: exact finite determination-through factorization/kernel separators, exact finite recovery constancy/separators, and informational tagged-family factorization pass; all other derived breakers and non-exact contracts remain pending |
| Return/Branch/Probe runtime and continuation descent | PARTIAL | Phase 5: typed Return/Branch/Probe control flow, structural verification, and non-actual suspension/resume pass; operator contracts, actual dispatch, raw-dependent continuation selection, and runtime persistence remain pending |
| Actuality, ledger/domain ordering, resolution, and replay | PARTIAL | Phase 6: opaque immutable raw-return, canonical event identity, append-only parent-linked ledger checks, and file-backed restart/revalidation pass. Phase 7: typed resolution-route identity and composition checking pass; request/attempt, dispatch, opaque-contract validation, state-transition replay, route execution, answer sets, and full resolution remain pending |
| Paired actuality and reciprocal residual reconstruction | PENDING | Phase 8 |
| Retained/access/active separation and recurrent memory crawl | PENDING | Phase 9 |
| Surface/backend/raw-return compilation boundaries | PARTIAL | Phase 10: canonical compiled probe-operator and recurrent probe-contract identities are distinct from runtime control flow and raw return; contract comparability/bridges, surface plans, backend requests, rendering, dispatch, and provider execution remain pending |
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

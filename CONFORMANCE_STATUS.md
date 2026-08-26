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
| P2-QUERY-008 canonical completion-candidate identity revalidates a complete typed source assignment without support or relation evaluation | PASS | `relation_schema::completion_candidate_has_canonical_answer_carrier_identity_without_supporting_it` |
| P2-REL-006 canonical data-only relation-expression grammar round trip | PASS | `canonical_relation_expression_grammar_round_trips_without_evaluation` |
| P3-IPROG-001 first-order Return/Ask canonical identities and explicit continuation reference | PASS | `iprog_identity::first_order_return_and_ask_round_trip_without_closures` |
| P3-IPROG-002 malformed first-order program rejection | PASS | `iprog_identity::rejects_malformed_inquiry_program_encodings` |
| P3-IPROG-003 explicit named environment identity, declared dependencies, and duplicate-name rejection | PASS | `iprog_identity::{first_order_return_and_ask_round_trip_without_closures,rejects_duplicate_explicit_environment_names}` |
| P3-IPROG-004 structural program checking revalidates result, typed forms, query, and continuation | PASS | `relation_schema::first_order_program_check_rejects_forged_or_result_mismatched_continuations` |
| P3-IPROG-005 stable Ask-with-environment byte/hash vector | PASS | `fixtures/iprogs/iprog-v2-ask-environment.json` |
| P3-IPROG-BIND-006 capture-safe finite answer binding retains the entire admitted partial set and rejects slot capture, question mismatch, and non-Ask sources | PASS | `decoder_identity::finite_supported_answers_require_exact_decoded_probe_and_standing_route_coverage` |
| P5-RUNTIME-ANSWER-003 exact admitted resumption requires matching event operator, bound source continuation, and fixed runtime resume target while retaining the lexical answer | PASS | `admitted_resume::admitted_answer_resumption_preserves_cold_replay_provenance_and_exact_lowering`; `admitted_lowering_tests::admitted_lowering_keeps_operator_continuation_and_resume_target_independent` |
| P6/7-REPLAY-VERTICAL-001 one preserved event replays through finite decode, standing support, source binding, verified suspension, and exact lowering to the same event/raw-return-bearing resumed state | PASS | `admitted_resume::admitted_answer_resumption_preserves_cold_replay_provenance_and_exact_lowering` |
| P6-EFFECT-JOURNAL-001 durable external-effect preparation precedes completion, authorizes dispatch only for the newly inserted intent, is idempotent only for exact data, allows one unresolved single-writer effect, and survives restart as existing unknown/pending without redispatch authority | PASS | `ic_store::tests::external_effect_preparation_survives_restart_and_completes_as_one_raw_event` |
| P6-EFFECT-JOURNAL-002 completion requires exact raw/operator/parent linkage and atomically commits immutable raw return, checked ordinary event, ledger edge, and completion link across restart | PASS | `ic_store::tests::external_effect_preparation_survives_restart_and_completes_as_one_raw_event` |
| P10-MOCK-DISPATCH-001 injected mock provider dispatch requires exact suspension/request identity and fresh durable authority, commits opaque raw/event before returning, never redispatches existing rows, and keeps provider failure operationally distinct | PASS | `mock_dispatch::mock_dispatch_requires_fresh_preparation_and_commits_raw_event_before_return` |
| P4-DETERMINATION-IR-001 claim-local determination-presentation canonical identity and ancestry | PASS | `determination_identity::determination_presentations_round_trip_with_explicit_context_and_ancestry` |
| P4-DETERMINATION-IR-002 malformed determination-presentation rejection | PASS | `determination_identity::determination_presentations_reject_malformed_encodings` |
| P4-DETERMINATION-IR-003 typed source and context-preserving predecessor validation | PASS | `determination_identity::determination_presentation_check_rejects_forged_source_and_incompatible_ancestry` |
| P4-DEPARTURE-IR-001 positive departure-witness canonical identity | PASS | `departure_identity::departure_witnesses_round_trip_with_all_positive_evidence_roles` |
| P4-DEPARTURE-IR-002 malformed departure-witness rejection | PASS | `departure_identity::departure_witnesses_reject_malformed_encodings` |
| P4-DEPARTURE-IR-003 presentation, typed-form, and relation-use context validation | PASS | `relation_schema::departure_witness_check_requires_the_declared_presentation_and_context` |
| P4-DEPARTURE-IR-004 source/candidate observation and incompatibility claimed-pair linkage | PASS | `relation_schema::departure_witness_check_requires_the_declared_presentation_and_context` |
| P4-DEPARTURE-IR-005 declared `Generate` evidence route rejected without over-rejecting a `Pure` derivation | PASS | `relation_schema::departure_witness_check_requires_the_declared_presentation_and_context` |
| P4-DEPARTURE-STAND-001 source-presentation claim target must occur in declared least-fixed-point standing, while evidence-use support remains separately represented | PASS | `decoder_identity::departure_witness_requires_its_source_presentation_support_to_stand`; `relation_schema::departure_witness_check_requires_the_declared_presentation_and_context` |
| P4-DEPARTURE-SUPPORT-002 each source/candidate/incompatibility use resolves only through its own matching relation-targeted support environment | PASS | `decoder_identity::departure_witness_requires_its_source_presentation_support_to_stand` |
| P4-DEPARTURE-SUPPORT-003 each evidence use's exact relation-targeted environment must close for its standing relation; a different unclosed route cannot borrow that standing | PASS | `decoder_identity::departure_witness_requires_its_source_presentation_support_to_stand` |
| P4-DEPARTURE-ADMIT-004 finite positive departure requires two event-linked decoded `Probe` observations, exact standing routes, raw-return coverage, and a positively listed oriented incompatibility pair | PASS | `decoder_identity::finite_departure_requires_positive_probed_supported_relevant_non_circular_evidence` |
| P4-DEPARTURE-ADMIT-005 missing observation-return coverage or source-presentation relevance rejects; rootless mixed support remains outside the least fixed point | PASS | `decoder_identity::finite_departure_requires_positive_probed_supported_relevant_non_circular_evidence`; `standing::mixed_claim_relation_standing_preserves_kinds_and_rejects_rootless_cycles` |
| P4-NEGATION-ADMIT-004 pointwise finite negation admission requires one admitted departure per exact use-tagged incidence and preserves semantic/execution coverage separation | PASS | `decoder_identity::finite_negation_admission_requires_one_departure_per_use_tagged_incidence` |
| P4-NEGATION-ADMIT-005 unsupported and duplicate incidence evidence rejects; the same exterior under another compatible use retains another return-fiber tag | PASS | `decoder_identity::finite_negation_admission_requires_one_departure_per_use_tagged_incidence` |
| P4-RECIPROCAL-VERTICAL-001 two independently admitted opposite finite sides compose through seed, same-use entire-fiber recovery, residuals, and downstream-only `Gamma`; same-orientation reciprocity rejects | PASS | `decoder_identity::independently_admitted_sides_form_one_reciprocal_occurrence_vertical_slice` |
| P7-SUPPANS-001 finite supported-answer admission replays one event-linked decode and requires exact `Probe`, standing-route, and raw-return provenance for every completion | PASS | `decoder_identity::finite_supported_answers_require_exact_decoded_probe_and_standing_route_coverage` |
| P7-SUPPANS-002 a two-completion decoded result remains one partial supported answer; missing or duplicate completion evidence rejects rather than selecting a singleton | PASS | `decoder_identity::finite_supported_answers_require_exact_decoded_probe_and_standing_route_coverage` |
| P4-CELL-IR-001 exact finite cell separator/exclusion coincidence across 65,536 binary pairs | PASS | `finite_cell::exact_finite_cell_exclusion_and_separator_existence_coincide_for_65536_pairs` |
| P4-CELL-IR-002 unknown finite observation remains neither separator nor same-cell conclusion | PASS | `finite_cell::{unknown_observation_never_becomes_a_positive_separator_or_same_cell_claim,comparison_rejects_mismatched_coordinate_schemas}` |
| P4-INCOMPAT-IR-001 finite declared-pair incompatibility preserves positive witness, no-witness, and unknown | PASS | `finite_cell::{finite_incompatibility_requires_a_positive_declared_pair,finite_incompatibility_rejects_duplicate_pair_declarations}` |
| P4-INCOMPAT-IR-002 typed finite declared-pair checker rehashes/type-checks cross-typed values | PASS | `finite_cell::{typed_finite_incompatibility_rehashes_checked_cross_typed_pairs,typed_finite_incompatibility_rejects_duplicate_and_forged_declarations}` |
| P4-INCOMPAT-IR-003 typed finite positive-pair/use membership rejects a missing bound form and a generated route | PASS | `finite_cell::typed_finite_incompatibility_requires_its_declared_use_to_bind_the_positive_pair` |
| P4-INCOMPAT-IR-004 explicit typed source/candidate role ports reject swapped roles and duplicate role declarations | PASS | `finite_cell::typed_finite_incompatibility_keeps_explicit_source_and_candidate_port_roles` |
| P11-CLAIM-IR-001 canonical claim identity revalidates source question/raw-return/path provenance without self-standing | PASS | `decoder_identity::claim_identity_preserves_candidate_provenance_without_claiming_standing` |
| P11-SUPPORT-IR-001 canonical support-environment identity revalidates a tagged claim/relation target, actual returns, and claim-target context while preserving generic premise references without closure | PASS | `decoder_identity::support_environment_identity_preserves_candidate_support_without_closure` |
| P11-SUPPORT-LINK-001 a relation use resolves only its exact relation-targeted support environment with matching scope/applicability, without admission | PASS | `decoder_identity::support_environment_identity_preserves_candidate_support_without_closure` |
| P11-STAND-LINK-001 checked claim-targeted support environments enter the existing least fixed point only through named checked premises and an explicit declared closure assessment | PASS | `decoder_identity::support_environment_identity_preserves_candidate_support_without_closure` |
| P11-DETERMINATION-LINK-001 a determination presentation resolves only its exact claim-targeted support environment, and its target claim must occur in the declared standing result | PASS | `decoder_identity::determination_support_requires_one_checked_claim_targeted_standing_environment` |
| P14-MATERIALIZATION-001 declared finite regime separates materialized routes, fresh routes, and routes outside the declared regime | PASS | `separator_identity::declared_finite_generator_regime_keeps_materialization_distinct_from_availability` |
| P14-NO-SEPARATOR-001 exact no-separator result requires a signature for every declared regime route and remains regime-relative | PASS | `separator_identity::exact_no_separator_remains_relative_to_one_declared_finite_regime` |
| P14-EXTENSION-001 fresh materialization and outside-regime extension candidates reject conflation with materialized or existing routes | PASS | `separator_identity::materialization_gap_and_regime_extension_remain_distinct_candidates` |
| P15-BRIDGE-IR-001 finite conservative question transport is injective with an external strict-growth witness; rebinding cannot claim growth | PASS | `binding_bridge::finite_bridge_keeps_conservative_growth_distinct_from_rebinding` |
| P15-BRIDGE-IR-002 every finite bridge endpoint rehashes, resolves, and matches its declared source or target binding | PASS | `binding_bridge::finite_bridge_rechecks_named_questions_against_their_declared_bindings` |
| P15-BRIDGE-IR-003 finite bridge endpoints outside the bridge's declared scope or horizon are rejected | PASS | `binding_bridge::finite_bridge_rechecks_named_questions_against_their_declared_bindings` |
| P4-EXTERIOR-IR-001 derived tagged exterior-claim preserves use identity and structural witness linkage | PASS | `relation_schema::tagged_exterior_claim_preserves_use_tag_without_admitting_an_incidence` |
| P4-EXTERIOR-IR-002 a tagged exterior claim checked against a declared extension must name an incidence that extension relates to its own source, under its own use | PASS | `relation_schema::an_exterior_claim_must_name_an_incidence_its_own_use_declares` |
| P4-SEED-IR-001 identity and bridged seed reorientation retain both roles, the use tag, and refuse a generated route (successor fixture 56) | PASS | `relation_schema::tagged_exterior_claim_preserves_use_tag_without_admitting_an_incidence` |
| P4-SIXFOLD-IR-001 dependent reciprocal occurrence: the Y side continues from the seeded source and each return fiber is its own use taken at its own exterior (successor fixtures 53, 55) | PASS | `relation_schema::sixfold_roles_are_generated_dependently_and_gamma_stays_downstream` |
| P4-SIXFOLD-IR-002 Gamma refuses while a role is missing and supplies none; a stable X return coexists with an unstable Y return; differing role fillings report Undecided, not different (successor fixtures 59, 60) | PASS | `relation_schema::sixfold_roles_are_generated_dependently_and_gamma_stays_downstream` |
| P4-NEGQUERY-IR-001 positive-negation question binds the presented source, opens the candidate, and retains use tag and semantic coverage | PASS | `relation_schema::positive_negation_query_binds_the_presented_source_and_opens_the_candidate` |
| P13-CUE-IR-001 exact finite discriminator-basis sufficiency and concrete residual separator | PASS | `cue::{exact_finite_cue_basis_returns_a_protected_separator_or_sufficiency,empty_basis_is_only_sufficient_for_a_constant_protected_signature,exact_finite_cue_basis_rejects_context_and_domain_mismatches}` |
| P13-CUE-IR-002 finite declared resource preorder retains incomparable sufficient candidates | PASS | `cue::{finite_resource_preorder_keeps_incomparable_sufficient_bases_and_residuals,finite_resource_preorder_rejects_invalid_orders_and_candidates}` |
| P4-FRONTIER-IR-001 tagged active-use frontier preserves distinct use and execution-coverage identities | PASS | `frontier::tagged_frontier_keeps_same_source_through_distinct_use_identities` |
| P4-FRONTIER-IR-002 duplicate tags, source mismatch, and empty-frontier closure inference reject | PASS | `frontier::{tagged_frontier_rejects_duplicate_use_and_mismatched_source,empty_frontier_does_not_infer_coverage_or_closure}` |
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
| P6-EVENT-IR-004 event/chart/operator question, boundary, grain, and horizon occurrence-context linkage | PASS | `event_identity::actual_event_requires_a_rehashed_boundary_chart_without_checking_its_open_roles` |
| P6-EVENT-STORE-001 append-only ledger ordering, idempotence, raw/boundary/operator-kind rejection, and parent corruption detection | PASS | `ic-store::tests::{actual_events_append_in_order_and_recheck_stored_identity,actual_event_append_rejects_stale_parent_and_detects_ledger_corruption}` |
| P6-EVENT-STORE-002 file-backed journal restart and immutable ledger revalidation | PASS | `ic-store::tests::event_ledger_reopens_and_revalidates_immutable_history_after_restart` |
| P6-EVENT-STORE-003 event append rejects a rehashed query from a different occurrence context | PASS | `ic-store::tests::actual_event_append_rejects_stale_parent_and_detects_ledger_corruption` |
| P7-RESOLUTION-IR-001 typed resolution-route identity round trip | PASS | `resolution_identity::resolution_paths_round_trip_without_executing_their_route` |
| P7-RESOLUTION-IR-002 identity and exact composition-interface validation | PASS | `resolution_identity::resolution_paths_check_identity_and_exact_composition_interfaces` |
| P7-RESOLUTION-IR-003 malformed resolution-path rejection | PASS | `resolution_identity::resolution_paths_reject_malformed_envelopes` |
| P7-DECODER-IR-001 finite decoder identity and checked decoded/undefined/unknown outcomes | PASS | `decoder_identity::finite_decoder_preserves_decoded_undefined_and_unknown_outcomes` |
| P7-DECODER-IR-002 direct single-port finite decode rechecks ordinary event record, route, raw/input, and output types | PASS | `decoder_identity::finite_decode_links_an_event_record_to_its_direct_decoder_route` |
| P7-DECODER-IR-003 decoded candidate/declared observation-use structural correspondence rejects unlisted candidates and binding mismatch | PASS | `decoder_identity::finite_decode_links_an_event_record_to_its_direct_decoder_route` |
| P10-SURFACE-IR-001 canonical surface-plan identity rechecks exact operator query, boundary, active view, executable code, probe contract, renderer version, and rendered body | PASS | `backend_identity::surface_and_backend_request_are_distinct_checked_operator_derivations` |
| P10-REQUEST-IR-001 canonical backend-request identity remains distinct from its plan/operator and rejects borrowed fields, bodies, versions, plans, and operators | PASS | `backend_identity::{surface_and_backend_request_are_distinct_checked_operator_derivations,backend_request_rejects_borrowed_plan_and_operator_fields,backend_boundary_rejects_noncanonical_payload_lengths}` |
| P4-DETERMINE-THROUGH-001 exact finite kernel-inclusion factorization | PASS | `factorization::exact_factorization_constructs_the_target_map_when_kernels_are_included` |
| P4-DETERMINE-THROUGH-002 kernel separator and context/coverage mismatch rejection | PASS | `factorization::exact_factorization_returns_a_kernel_separator_and_rejects_incomplete_contexts` |
| P4-FIBER-IR-001 exact finite same-use reverse return section, source membership, and non-unique return (successor fixtures 34-35) | PASS | `fiber::{every_admitted_incidence_returns_its_source_through_the_same_use,the_same_exterior_through_two_uses_keeps_two_distinct_returns}` |
| P4-FIBER-IR-002 undeclared exterior, duplicate incidence, and fiber/signature domain disagreement reject | PASS | `fiber::{an_undeclared_exterior_has_no_return_and_a_duplicate_incidence_is_refused,recovery_is_checked_against_the_derived_fiber_not_an_unconnected_table}` |
| P4-FIBER-IR-003 selection is drawn from the return fiber, and closure is decided from the whole fiber rather than the selection (successor fixture 37) | PASS | `fiber::a_stable_selected_return_does_not_close_a_fiber_that_still_splits` |
| P4-NEGFIELD-IR-001 typed finite negation extension type-checks each incidence against the port it fills, supplies the forward section `NegField_u(s)`, and agrees with the reverse section on every declared incidence | PASS | `relation_schema::a_typed_negation_extension_checks_each_incidence_against_the_port_it_fills` |
| P4-RECOVERY-IR-001 exact finite protected-signature constancy | PASS | `recovery::exact_fiber_recovery_requires_signature_constancy_and_emits_a_positive_separator` |
| P4-RECOVERY-IR-002 empty fiber rejection and unknown/loss separation | PASS | `recovery::empty_or_incomplete_evidence_is_not_conflated_with_non_recovery` |
| P4-FAMILY-IR-001 joint exact-family factorization without composite actuality | PASS | `family_factorization::exact_family_product_recovers_a_target_that_no_member_recovers` |
| P4-FAMILY-IR-002 empty/mismatched family rejection and joint separator | PASS | `family_factorization::exact_family_product_rejects_bad_coverage_and_exposes_joint_kernel_separators` |
| P4-BOUNDARY-IR-001 local boundary-chart canonical identity preserves tagged frontiers and open roles | PASS | `boundary_identity::boundary_chart_keeps_tagged_frontiers_and_absent_roles_explicit` |
| P4-BOUNDARY-IR-002 malformed chart optional-field rejection | PASS | `boundary_identity::boundary_chart_rejects_malformed_optional_and_count_fields` |
| P12-SEPARATOR-IR-001 generic separator-problem identity preserves residual/generator/effectivity context and rejects malformed domains | PASS | `separator_identity::{separator_problem_round_trips_and_keeps_its_generic_residual_context,separator_problem_rejects_malformed_and_wrong_domain_encodings}` |
| P12-GENERATED-IR-001 generated inquiry rechecks its residual problem and question binding/grain/horizon while remaining an unselected candidate | PASS | `decoder_identity::generated_inquiry_is_a_checked_problem_relative_candidate_not_a_policy_choice` |
| P16-LICENSE-IR-001 compression licence identity preserves exact versus directional-approximate contracts, canonical reference ordering, and malformed-domain rejection | PASS | `compression_identity::{compression_licence_keeps_exact_and_directional_approximate_contracts_distinct,compression_licence_rejects_duplicate_and_malformed_contracts}` |
| P10-METHOD-IR-001 method contract identity rechecks its implemented relation and preserves authority/coverage/backend/failure/provenance without admission or execution | PASS | `method_identity::{method_contract_is_a_typed_registry_record_without_admission_or_execution,method_contract_rejects_duplicate_and_malformed_registry_data}` |
| P16-OCCURRENCE-IR-001 operator occurrence is derived from one rechecked actual event and rejects a detached raw return | PASS | `decoder_identity::operator_occurrence_is_derived_from_one_exact_ordinary_event` |
| P11-STAND-IR-001 least-fixed-point standing: a rootless positive support cycle never enters, and grounding one member admits the cycle | PASS | `standing::a_rootless_support_cycle_never_enters_standing` |
| P11-STAND-IR-002 each of the five closed-support conditions independently blocks a route, and one closed route suffices among incomparable environments | PASS | `standing::{each_closed_support_condition_independently_blocks_a_route,one_closed_route_suffices_when_another_is_blocked,standing_grows_only_through_routes_that_already_reach}` |
| P11-STAND-IR-003 one typed fixed point admits claim and relation subjects, rejects mixed rootless cycles, and domain-separates equal underlying digests by subject kind | PASS | `standing::{mixed_claim_relation_standing_preserves_kinds_and_rejects_rootless_cycles,standing_subject_kind_separates_equal_underlying_digests}` |

## Pending specification and plan categories

| Category | Status | First planned phase |
|---|---|---|
| Typed forms, typed reification, and type verification | PARTIAL | Phase 1: canonical type artifacts, typed-form declarations, and structural checks pass; binding-native term/reification semantics remain pending |
| Relations, open ports, partial binding, and question kernels | PARTIAL | Phase 2: direct `OpenQuery` partial bindings, data-only relation expressions, checked `Bind`/`Expose`, canonical complete-candidate identity/checking, direct schema-order normalization, a derived completion-fiber view, formula artifacts, typed terms, relation schemas, atom-signature checks, and relation uses pass; typed expression validation and dependent binding remain pending |
| First-order programs and proposal/actuality/authority separation | PARTIAL | Phase 3: canonical `Return`/`Ask` identity, explicit environment, malformed-input rejection, and recursive structural validation pass; a candidate element of an answer carrier is canonical, while supported-answer-set representation, substitution, normalization, pure operations, and runtime separation remain pending |
| Successor determination and departure, fixtures 1-13 | PARTIAL | Phase 4: claim-local determination identity, structural witness validation, exact claim/relation standing-route provenance, and a derived finite positive-departure admission pass. The finite route requires event-linked decoded probes, explicit source-route relevance, raw-return coverage, and an oriented listed incompatibility pair; general observation execution, coverage admission, warranted relation truth, and all 13 canonical successor fixtures remain pending |
| Typed negation and coverage separation, fixtures 14-24 | PARTIAL | Phase 4: oriented identity, pointwise finite incidence admission, per-row admitted departure, and semantic/execution coverage separation pass. Soundness-program execution, global coverage certificates, warrant, and all canonical fixtures remain pending |
| Tagged multiple negation uses, fixtures 25-33 | PARTIAL | Phase 4: exact finite tagged family product, derived tagged active-use frontier structure, and joint-information factorization pass; use admission/re-hashing, coverage certificates, jointness evidence, and successor fixtures remain pending |
| Same-use return and protected recovery, fixtures 34-47 | PARTIAL | Phase 4: same-use fibers over a fully departure-matched finite extension and entire-fiber exact recovery pass, including the selected-return insufficiency breaker. Supported/actual return selection, non-singleton admitted vertical recovery, and canonical fixtures remain pending |
| Pure return versus warranted reconciliation, fixtures 48-52 | PENDING | Phase 4 |
| Dependent sixfold and downstream `Gamma`, fixtures 53-64 | PARTIAL | Phase 4: independently admitted finite X/Y sides, explicit seed, opposite-orientation enforcement, same-use returns, residuals, and downstream-only `Gamma` compose in one derived vertical fixture. Runtime actuality, persisted/replayed occurrence reconstruction, compatibility evaluation, and canonical fixtures remain pending |
| Reciprocal representation and learning, fixtures 65-70 | PENDING | Phases 4, 12-16 |
| Cross-cutting determination, jointness, recovery-loss, regenerative-economy, method, growth, approximation, and consequence-subspace breakers | PARTIAL | Phase 4: exact finite determination-through factorization/kernel separators, exact finite recovery constancy/separators, and informational tagged-family factorization pass; all other derived breakers and non-exact contracts remain pending |
| Return/Branch/Probe runtime and continuation descent | PARTIAL | Phase 5: typed Return/Branch/Probe control flow, structural verification, non-actual suspension/resume, and one injected mock dispatch pass; general operator contracts, raw-dependent continuation selection, and runtime persistence remain pending |
| Actuality, ledger/domain ordering, resolution, and replay | PARTIAL | Phase 6: opaque immutable raw-return, canonical event identity, append-only parent-linked ledger checks, typed request-before-dispatch, pending/unknown recovery, injected mock dispatch, atomic raw/event completion, and file-backed restart/revalidation pass. Phase 7: typed resolution-route identity/composition, canonical complete-candidate identity/checking, finite direct single-port decoded/undefined/unknown event results, exact supported-answer admission/binding, finite event-to-runtime resumption, and one complete live-to-cold-replay cycle pass; typed semantic attempts, opaque-contract validation, general state-transition replay, multi-port answer carriers, general route execution, relation evaluation, and full resolution remain pending |
| Paired actuality and reciprocal residual reconstruction | PENDING | Phase 8 |
| Retained/access/active separation and recurrent memory crawl | PENDING | Phase 9 |
| Surface/backend/raw-return compilation boundaries | PARTIAL | Phase 10: canonical compiled probe-operator, recurrent probe-contract, surface-plan, backend-request, and method-contract identities are distinct from runtime control flow and raw return; exact operator/plan/request checking, typed durable preparation, and one injected mock-provider execution pass. Method admission/runnable/usable classification, semantic resolution, contract comparability/bridges, actual rendering, and real-provider execution remain pending |
| Positive standing, support environments, and rootless cycles | PARTIAL | Phase 11: canonical claim and candidate support-environment identity/provenance checking, typed claim/relation least-fixed-point standing over derived declared support environments, exact closing-route provenance, the five closed-support conditions, claim/relation/mixed rootless-cycle rejection, relation-use standing-support resolution, and determination-to-claim standing linkage pass; opaque checker/assumption/dependency evaluation, ingress grounding, claim-payload/source-form interpretation, web admission/relevance, non-circularity, and warrant remain pending |
| Separator generation, cue planning, and bounded unknown results | PARTIAL | Phase 12: structural generic `SeparatorProblem` identity and problem-relative canonical `GeneratedInquiry` candidates pass; route-regime lawfulness/materialization, deterministic question policy, candidate-set exhaustiveness, and bounded unknown residuals remain pending. Phase 13: exact finite total/deterministic/caller-covered sufficient-basis checking, concrete separators, and nondominated selection over caller-supplied finite candidate/resource-preorder inputs pass; approximate/frontier results remain pending |
| Materialization, expressibility, and representation gaps | PARTIAL | Phase 14: declared finite generator regimes, materialization/extension candidates, and regime-relative exact no-separator checks pass; route lawfulness/completeness, global expressibility, and `RepresentationGap` remain pending |
| Binding extension, bridges, rebinding, and history locality | PARTIAL | Phase 15: finite injective question transport under declared scope/horizon with target-only conservative-growth witness passes; universal transport, interpretation preservation, typed transports, extension admission, targeted reopening, and history locality remain pending |
| Folding, recovery, compression licences, and reopening | PARTIAL | Phase 16: canonical compression-licence identity and event-derived operator-occurrence links pass; regeneration, recovery contracts, approximation soundness, economy selection, method folding/admission, and reopening remain pending |
| Cross-binding standing lift | PENDING | Phase 17 |
| Predecessor-judged self-revision | PENDING | Phase 18 |
| Measured breadth and optimization only after semantic closure | PENDING | Phase 19 |

## Post-research replay fixtures

The final research corpus is derived breaker ancestry. Its adoption changed no executable status
by itself; the following entries pass only through their named executable evidence:

| Fixture | Status | Required discriminator |
|---|---|---|
| RPL-001 completed effect reload | PASS | `ic-store::tests::external_effect_preparation_survives_restart_and_completes_as_one_raw_event` reopens a file-backed store, rechecks the exact request/event/raw bytes without provider access, and rejects pending, unknown, corrupt, version-mismatched, and provenance-mismatched states. |
| RPL-002 finite semantic answer replay | PASS | `ic-runtime::finite_probe_executes_once_and_cold_replays_with_distinct_residuals` reloads canonical types, forms, relation/query, candidates, uses, decoder/path, event, support, and programs; `Decoded`, `Undefined`, and `Unknown` remain distinct and both decoded completions survive admission. |
| RPL-003 source continuation replay | PASS | The same fresh-process fixture decodes the persisted source `Ask`, reconstructs standing and the complete answer set, and creates a new capture-safe `BoundFiniteAskContinuation`; mismatched and capturing sources reject. |
| RPL-004 runtime continuation replay | PASS | The fixture regenerates and verifies `ProgramIR`, `ProbeSuspension`, and `ContinuationLowering` from reloaded source/operator/version identities and reaches the same admitted resumption without a persisted lowering recipe. |
| RPL-005 replay failure matrix | PASS | `ic-store` restart tests, `ic-runtime::mock_dispatch_requires_fresh_preparation_and_commits_raw_event_before_return`, and the cold-replay fixture keep provider failure, pending/unknown actuality, `Undefined`, `Unknown`, unsupported/no-standing answers, source mismatch, capture, operator mismatch, continuation mismatch, and target mismatch distinct. |
| CYCLE-001 cold-replayable inquiry cycle | PASS | `ic-runtime::finite_probe_executes_once_and_cold_replays_with_distinct_residuals` runs `Ask -> durable dispatch -> one provider return -> raw/event -> complete supported binding -> Return`, closes the file-backed store, rebuilds the catalog and lowering from persisted roots, reaches the same event/raw/answer/provenance identities and protected return, and leaves the shared provider-call counter at exactly one. |

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

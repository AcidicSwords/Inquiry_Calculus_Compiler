# Inquiry Calculus

> **Branch mode — formal successor:** `codex/formal-successor` is an isolated construction branch.
> Its active objective is the Lean-checked successor under
> `formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md`, not continued semantic expansion of the
> v2.0 Rust implementation. The Rust workspace remains the predecessor reference and regression
> boundary until Formal Gate F.

Rust reference implementation of Inquiry Calculus v2.0: a typed relational language for answer-dependent inquiry, preserved actuality, regenerative compression, and cold replay.

Version 2.0 consolidates the accepted v1.1 substrate, positive-negation successor, paired actuality, and corrected interrogative succession into one forward authority. The version change does not restart the implementation or create executable conformance by itself.

## Authority

| Question | Active source |
|---|---|
| What the calculus means | `Inquiry_Calculus_v2_0.tex` |
| How the successor is constructed and accepted on this branch | `formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md` |
| Machine-checked candidate successor meaning | `formal/` (only to the coverage actually proved) |
| Final architecture, phase dependencies, and completion contract | `Inquiry_Calculus_v2_0_Comprehensive_Implementation_Plan.md` |
| How consequential engineering work proceeds | `AGENTS.md` and the `.claude` inquiry harness |
| The single strongest live executable residual | `IMPLEMENTATION_FRONTIER.md` |
| Successor evidence, decisions, and failures | `formal-successor/CONFORMANCE_STATUS.md`, `formal-successor/DECISIONS.jsonl`, `formal-successor/FAILURES.jsonl` |
| Demonstrated executable evidence | `CONFORMANCE_STATUS.md` |
| Accepted implementation choices and reopen conditions | `DECISIONS.jsonl` |
| Observed failures and environmental constraints | `FAILURES.jsonl` |
| Historical ancestry | Git |
| Derived breaker and regression material | `research/` |

Research is consulted only when a live residual needs one of its distinctions. It is not semantic authority or a second implementation plan.

## Workspace

- `ic-core` defines canonical identities, typed semantic forms, first-order inquiry programs, and derived finite checkers.
- `ic-runtime` verifies and coordinates runtime control, providers, dispatch, resolution, and cold replay without becoming semantic authority.
- `ic-store` provides immutable content-addressed artifacts, ordinary event history, and crash-recovery persistence through SQLite.
- `ic-cli` remains the narrow command-line boundary and contains no independent semantic machinery.
- `formal` is the pinned Lean/Lake proof project for the successor.
- `formal-successor` owns successor inputs, inventories, evidence, decisions, failures, and
  propagation reports. These do not overwrite predecessor ledgers. The construction specification
  and the generated `.claude/hooks/ic-spine.js` projection provide one cohesive inquiry path.

## Build and verification

```bash
cargo fmt --all --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
```

The canonical specification is compiled with the pinned Tectonic workflow:

```bash
tectonic -X compile --keep-logs --outdir target/tex Inquiry_Calculus_v2_0.tex
```

See `.github/workflows/ci.yml` for the complete gate set.

The formal branch also runs:

```bash
node tools/successor_control_check.js
node tools/inquiry_spine_check.js
node tools/inquiry_spine_lifecycle_check.js
node tools/predecessor_inventory.js check
node tools/predecessor_inventory_check.js
node tools/predecessor_tex_classification.js check
node tools/predecessor_tex_classification_check.js
node tools/predecessor_implementation_classification.js check
node tools/predecessor_implementation_classification_check.js
node tools/predecessor_fixture_classification.js check
node tools/predecessor_fixture_classification_check.js
node tools/phase_a_coverage.js check
node tools/phase_a_coverage_check.js
node tools/phase_b_predecessor_spine.js check
node tools/phase_b_predecessor_spine_check.js --compile
node tools/phase_b_binding_type.js check
node tools/phase_b_binding_type_check.js --compile
node tools/phase_b_forms.js check
node tools/phase_b_forms_check.js --compile
node tools/phase_b_relations.js check
node tools/phase_b_relations_check.js --compile
node tools/phase_b_refinement.js check
node tools/phase_b_refinement_check.js --compile
node tools/phase_b_formula_grammar.js check
node tools/phase_b_formula_grammar_check.js --compile
node tools/phase_b_minimal_logical_basis.js check
node tools/phase_b_minimal_logical_basis_check.js --compile
node tools/phase_b_relation_expression_ir.js check
node tools/phase_b_relation_expression_ir_check.js --compile
node tools/operator_descent_check.js --compile
node tools/regenerative_sufficiency_check.js --compile
node tools/regenerative_economy_correspondence_check.js --compile
node tools/differentiate_only_enough_check.js --compile
node tools/understanding_check.js --compile
node tools/ablative_regeneration_check.js --compile
node tools/learning_gain_check.js --compile
node tools/method_promotion_check.js --compile
node tools/traversal_learning_check.js --compile
node tools/question_pattern_learning_check.js --compile
node tools/memory_recovery_check.js --compile
node tools/historical_reconstruction_check.js --compile
node tools/exact_representation_quotient_check.js --compile
node tools/continuation_sufficiency_check.js --compile
node tools/regenerative_preservation_check.js --compile
node tools/recovery_reopening_contract_check.js --compile
node tools/compression_license_check.js --compile
node tools/approximate_compression_check.js --compile
node tools/unlock_field_check.js --compile
cd formal
lake build --wfail
```

New inquiry traces pin `Questions.txt` and the compact derived
`formal-successor/INQUIRY_SPINE_CONTRACT.json`. The sole model-facing recurrence is
`RELATE -> OPEN -> TURN -> RETURN -> DISTINGUISH -> FOLD -> CARRY -> RELATE`.
The underlying event lifecycle makes returns accountable without becoming another reasoning loop.
Unchosen live questions remain represented, generated products remain non-Standing, typed paths
remain distinct from actual history, transported discriminators can reopen inadequate folds, and a
checkpoint remains distinct from task closure. The broad residual index remains a deletable
projection rebuilt from append-only evidence and `formal-successor/RESIDUAL_OBLIGATIONS.json`.
The Phase A source universe and its non-promotion boundary are documented in
`formal-successor/PHASE_A_INVENTORY.md`; its generated inventory keeps unresolved classification
visible rather than converting source coverage into Gate A.
The source-bound six-way TeX review overlay and its non-promotion rules are documented in
`formal-successor/PHASE_A_TEX_CLASSIFICATION.md`.
The authority-separated Rust/schema overlay and exact-symbol candidate-edge rules are documented
in `formal-successor/PHASE_A_IMPLEMENTATION_CLASSIFICATION.md`.
The first Phase B ambient-metalanguage boundary and source-regenerative predecessor dependency
spine are documented in `formal-successor/PHASE_B_AMBIENT_BOUNDARY.md`; Gate B remains pending.
The following binding-indexed predecessor type syntax and its retained grammar obligations are
documented in `formal-successor/PHASE_B_BINDING_TYPE_SURFACE.md`.
The subsequent binding-typed represented-form carrier, reification boundary, and partial
operational interpretation are documented in `formal-successor/PHASE_B_FORMS.md`; relations,
questions, programs, and Gate B remain open.
The following typed relation surface is documented in `formal-successor/PHASE_B_RELATIONS.md`;
it retains typed endpoints, partial converse, and the function condition while leaving relation
expressions and later semantics open.
The coarsest relation/refinement boundary is documented in
`formal-successor/PHASE_B_REFINEMENT.md`; universal relatedness, binding-supplied coexistence,
reverse inclusion, and the retained no-vacuity obligation remain distinct.
The formula display is documented in `formal-successor/PHASE_B_FORMULA_GRAMMAR.md` as a typed
candidate syntax with all six source records still obligation-bound rather than accepted logic.
The classical reference-dialect basis and its conditional derivation shapes are documented in
`formal-successor/PHASE_B_MINIMAL_LOGICAL_BASIS.md`; their binding-native-complement limitation
remains explicit.
The data-only relation-expression display is documented in
`formal-successor/PHASE_B_RELATION_EXPRESSION_IR.md` as candidate syntax only; denotation and
semantic-question claims remain obligation-bound.
The typed schema/signature boundary is documented in
`formal-successor/PHASE_B_RELATION_SCHEMA_PORTS.md`; named ports remain non-string typed
coordinates and schemas remain distinct from relation instances and questions.
The partial-binding and completion-fiber boundary is documented in
`formal-successor/PHASE_B_PARTIAL_BINDING_FIBER.md`; it preserves typed candidate carriers while
leaving satisfaction, valid completion, and question semantics open.
The canonical-question syntax boundary is documented in
`formal-successor/PHASE_B_CANONICAL_QUESTION_SYNTAX.md`; answers, validity, probes, and programs
remain separate later layers.
The exact-quotient boundary is documented in
`formal-successor/PHASE_B_EXACT_REPRESENTATION_QUOTIENT.md`; forward consequence sufficiency,
finite tested nondistinction, and coarsest characterization remain distinct, while continuation
descent stays an explicitly open later obligation.
The continuation-sufficiency boundary is documented in
`formal-successor/PHASE_B_CONTINUATION_SUFFICIENCY.md`; a target continuation must be supplied
and commute with the quotient, while a source action that splits a quotient fiber is rejected.
The regenerative-preservation boundary is documented in
`formal-successor/PHASE_B_REGENERATIVE_PRESERVATION.md`; recovery requires an explicit witness
for the specified source, not merely a retained quotient image.
The recovery/reopening boundary is documented in
`formal-successor/PHASE_B_RECOVERY_REOPENING_CONTRACT.md`; provenance, residual, factorization,
recovery, and unlock coordinates remain independently required for every protected requirement.
The compression-license boundary is documented in
`formal-successor/PHASE_B_COMPRESSION_LICENSE.md`; a quotient map is only one coordinate of a
candidate licence and cannot establish exactness by itself.
The approximate-compression boundary is documented in
`formal-successor/PHASE_B_APPROXIMATE_COMPRESSION.md`; scalar error never substitutes for the
directional distortion contract it licenses.
The unlock-field boundary is documented in `formal-successor/PHASE_B_UNLOCK_FIELD.md`; its
observational, dynamic, and context-contract triggers stay distinct.
The six-root primitive-elimination boundary is documented in
`formal-successor/PHASE_B_DERIVED_INTERROGATIVE_ROOTS.md`; `Orient` has two presentation variants
without becoming two roots, and reification adds no seventh root.
The adjacent no-universal-polarization boundary is documented in
`formal-successor/PHASE_B_NO_UNIVERSAL_POLARIZATION.md`; five relation kinds and explicit
binding-supplied correspondence remain separate from semantic admission and standing.
The operational-name primitive-elimination boundary is documented in
`formal-successor/PHASE_B_OPERATIONAL_ROOT_ALIASES.md`; eleven names remain removable presentations
over supplied nonempty root expansions only when every preservation and non-promotion obligation
holds.
The reciprocal-regenerative reformulation is integrated through the non-authoritative
`formal-successor/REGENERATIVE_SPINE.json` dependency projection and the expanded planned theorem
registry. Neither artifact promotes a candidate relation or creates another live frontier.

## Current implementation state

Moving status is intentionally not duplicated here:

- `IMPLEMENTATION_FRONTIER.md` names what is live now.
- `CONFORMANCE_STATUS.md` records exactly what executable checks have demonstrated.
- `DECISIONS.jsonl`, `FAILURES.jsonl`, and Git explain why the repository has its present shape.

Consequential changes follow `AGENTS.md`. Generated proposals, actual returns, decoded results, checks, warrant, and standing remain distinct throughout the implementation.

Test results are recorded as scoped consequence boundaries, not generic proofs of correctness:
each claim states its protected consequence, proposed condition, scope/semantics, probe, checker,
coverage, and reopening condition. A missing counterexample remains `Unknown` unless the declared
breaker field is independently established empty.

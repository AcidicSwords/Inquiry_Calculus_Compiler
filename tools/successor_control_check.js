#!/usr/bin/env node
"use strict";

const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const root = path.resolve(__dirname, "..");
const errors = [];
const baseline = "4a18e2e308f359a64f19b7d056652f19fd9aaeae";

function rel(name) {
  return path.join(root, ...name.split("/"));
}

function read(name) {
  try {
    return fs.readFileSync(rel(name), "utf8");
  } catch (error) {
    errors.push(`${name}: ${error.message}`);
    return "";
  }
}

function digest(name) {
  try {
    return crypto.createHash("sha256").update(fs.readFileSync(rel(name))).digest("hex");
  } catch (error) {
    errors.push(`${name}: ${error.message}`);
    return "";
  }
}

function requireFile(name) {
  if (!fs.statSync(rel(name), { throwIfNoEntry: false })?.isFile()) {
    errors.push(`required successor file is missing: ${name}`);
  }
}

function requireContains(name, fragments) {
  const text = read(name);
  for (const fragment of fragments) {
    if (!text.includes(fragment)) errors.push(`${name}: missing ${JSON.stringify(fragment)}`);
  }
}

function requireExcludes(name, fragments) {
  const text = read(name);
  for (const fragment of fragments) {
    if (text.includes(fragment)) errors.push(`${name}: must not contain ${JSON.stringify(fragment)}`);
  }
}

const required = [
  "AGENTS.md",
  "IMPLEMENTATION_FRONTIER.md",
  "formal/lean-toolchain",
  "formal/lakefile.toml",
  "formal/lake-manifest.json",
  "formal/InquiryCalculus.lean",
  "formal/InquiryCalculus/Meta/Ambient.lean",
  "formal/Spec.lean",
  "formal/Spec/InquiryCalculus.lean",
  "formal-successor/ACTIVE_INPUTS.json",
  "formal-successor/AUTONOMOUS_ITERATION.md",
  "formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md",
  "formal-successor/PREFORMAL_SUCCESSOR_CODING_INQUIRY_HARNESS.md",
  "formal-successor/PREFORMAL_SEARCH_ASYMMETRY.md",
  "formal-successor/SUCCESSOR_CONSTRUCTION_HARNESS_SPEC.md",
  "formal-successor/QUESTION_BANK_DERIVED_EXPLORATION_ALGORITHM.md",
  "formal-successor/QUESTION_RHYTHM.md",
  "formal-successor/RESIDUAL_OBLIGATIONS.json",
  "formal-successor/PHASE_A_INVENTORY.md",
  "formal-successor/PREDECESSOR_INVENTORY_GRAMMAR.json",
  "formal-successor/PREDECESSOR_INVENTORY.json",
  "formal-successor/PHASE_A_TEX_CLASSIFICATION.md",
  "formal-successor/PREDECESSOR_TEX_CLASSIFICATION_SCHEMA.json",
  "formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json",
  "formal-successor/PHASE_A_IMPLEMENTATION_CLASSIFICATION.md",
  "formal-successor/PREDECESSOR_IMPLEMENTATION_CLASSIFICATION_SCHEMA.json",
  "formal-successor/PREDECESSOR_IMPLEMENTATION_CLASSIFICATION.json",
  "formal-successor/PHASE_A_FIXTURE_CLASSIFICATION.md",
  "formal-successor/PREDECESSOR_FIXTURE_CLASSIFICATION_SCHEMA.json",
  "formal-successor/PREDECESSOR_FIXTURE_CLASSIFICATION.json",
  "formal-successor/PHASE_A_COVERAGE.md",
  "formal-successor/PHASE_A_COVERAGE_SCHEMA.json",
  "formal-successor/PHASE_A_COVERAGE_CERTIFICATE.json",
  "formal-successor/PHASE_B_AMBIENT_BOUNDARY.md",
  "formal-successor/PHASE_B_PREDECESSOR_SPINE_SCHEMA.json",
  "formal-successor/PHASE_B_PREDECESSOR_SPINE.json",
  "formal-successor/PHASE_B_BINDING_TYPE_SURFACE.md",
  "formal-successor/PHASE_B_BINDING_TYPE_SCHEMA.json",
  "formal-successor/PHASE_B_BINDING_TYPE_SURFACE.json",
  "formal-successor/PHASE_B_FORMS.md",
  "formal-successor/PHASE_B_FORMS_SCHEMA.json",
  "formal-successor/PHASE_B_FORMS_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/Forms.lean",
  "formal-successor/PHASE_B_RELATIONS.md",
  "formal-successor/PHASE_B_RELATIONS_SCHEMA.json",
  "formal-successor/PHASE_B_RELATIONS_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/Relations.lean",
  "formal-successor/PHASE_B_REFINEMENT.md",
  "formal-successor/PHASE_B_REFINEMENT_SCHEMA.json",
  "formal-successor/PHASE_B_REFINEMENT_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/Refinement.lean",
  "formal-successor/PHASE_B_FORMULA_GRAMMAR.md",
  "formal-successor/PHASE_B_FORMULA_GRAMMAR_SCHEMA.json",
  "formal-successor/PHASE_B_FORMULA_GRAMMAR_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/FormulaGrammar.lean",
  "formal-successor/PHASE_B_MINIMAL_LOGICAL_BASIS.md",
  "formal-successor/PHASE_B_MINIMAL_LOGICAL_BASIS_SCHEMA.json",
  "formal-successor/PHASE_B_MINIMAL_LOGICAL_BASIS_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/MinimalLogicalBasis.lean",
  "formal-successor/PHASE_B_RELATION_EXPRESSION_IR.md",
  "formal-successor/PHASE_B_RELATION_EXPRESSION_IR_SCHEMA.json",
  "formal-successor/PHASE_B_RELATION_EXPRESSION_IR_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/RelationExpressionIR.lean",
  "formal-successor/PHASE_B_RELATION_SCHEMA_PORTS.md",
  "formal-successor/PHASE_B_RELATION_SCHEMA_PORTS_SCHEMA.json",
  "formal-successor/PHASE_B_RELATION_SCHEMA_PORTS_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/RelationSchemaPorts.lean",
  "formal-successor/PHASE_B_PARTIAL_BINDING_FIBER_SCHEMA.json",
  "formal-successor/PHASE_B_PARTIAL_BINDING_FIBER_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/PartialBindingFiber.lean",
  "formal-successor/PHASE_B_CANONICAL_QUESTION_SYNTAX_SCHEMA.json",
  "formal-successor/PHASE_B_CANONICAL_QUESTION_SYNTAX_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/CanonicalQuestionSyntax.lean",
  "formal-successor/PHASE_B_ANSWER_CARRIER_VALIDITY_SCHEMA.json",
  "formal-successor/PHASE_B_ANSWER_CARRIER_VALIDITY_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/AnswerCarrierValidity.lean",
  "formal-successor/PHASE_B_PROPOSITION_NOT_WARRANT_SCHEMA.json",
  "formal-successor/PHASE_B_PROPOSITION_NOT_WARRANT_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/PropositionNotWarrant.lean",
  "formal-successor/PHASE_B_MANY_QUESTIONS_GENERATION_SCHEMA.json",
  "formal-successor/PHASE_B_MANY_QUESTIONS_GENERATION_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/ManyQuestionsGeneration.lean",
  "formal-successor/PHASE_B_DISCHARGE_MODE_SYNTAX_SCHEMA.json",
  "formal-successor/PHASE_B_DISCHARGE_MODE_SYNTAX_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/DischargeModeSyntax.lean",
  "formal-successor/PHASE_B_QUESTION_COMPOSITION_SYNTAX_SCHEMA.json",
  "formal-successor/PHASE_B_QUESTION_COMPOSITION_SYNTAX_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/QuestionCompositionSyntax.lean",
  "formal-successor/PHASE_B_QUESTION_CONDITIONED_DISCRIMINATION_SCHEMA.json",
  "formal-successor/PHASE_B_QUESTION_CONDITIONED_DISCRIMINATION_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/QuestionConditionedDiscrimination.lean",
  "formal-successor/PHASE_B_QUESTION_REFINEMENT_PREORDER_SCHEMA.json",
  "formal-successor/PHASE_B_QUESTION_REFINEMENT_PREORDER_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/QuestionRefinementPreorder.lean",
  "formal-successor/PHASE_B_QUESTION_REFINEMENT_SEMANTICS_SCHEMA.json",
  "formal-successor/PHASE_B_QUESTION_REFINEMENT_SEMANTICS_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/QuestionRefinementSemantics.lean",
  "formal-successor/PHASE_B_QUESTION_JOINT_ACTIVE_REFINEMENT_SCHEMA.json",
  "formal-successor/PHASE_B_QUESTION_JOINT_ACTIVE_REFINEMENT_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/QuestionJointActiveRefinement.lean",
  "formal-successor/PHASE_B_QUESTION_REDUNDANCY_SCHEMA.json",
  "formal-successor/PHASE_B_QUESTION_REDUNDANCY_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/QuestionRedundancy.lean",
  "formal-successor/PHASE_B_PRECISION_NOT_IMPROVEMENT_SCHEMA.json",
  "formal-successor/PHASE_B_PRECISION_NOT_IMPROVEMENT_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/PrecisionNotImprovement.lean",
  "formal-successor/PHASE_B_RELATIONAL_SECTIONS_SCHEMA.json",
  "formal-successor/PHASE_B_RELATIONAL_SECTIONS_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/RelationalSections.lean",
  "formal-successor/PHASE_B_SOLUTION_FIBERS_SCHEMA.json",
  "formal-successor/PHASE_B_SOLUTION_FIBERS_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/SolutionFibers.lean",
  "formal-successor/PHASE_B_QUESTION_STRUCTURED_HOLE_SCHEMA.json",
  "formal-successor/PHASE_B_QUESTION_STRUCTURED_HOLE_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/QuestionStructuredHole.lean",
  "formal-successor/PHASE_B_RELATIONAL_ABSTRACTION_SCHEMA.json",
  "formal-successor/PHASE_B_RELATIONAL_ABSTRACTION_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/RelationalAbstraction.lean",
  "formal-successor/PHASE_B_ABSTRACTION_BY_REMOVAL_SCHEMA.json",
  "formal-successor/PHASE_B_ABSTRACTION_BY_REMOVAL_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/AbstractionByRemoval.lean",
  "formal-successor/PHASE_B_SOLUTION_FIELD_WEB_SCHEMA.json",
  "formal-successor/PHASE_B_SOLUTION_FIELD_WEB_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/SolutionFieldWeb.lean",
  "formal-successor/PHASE_B_INDEXED_MEET_REFINEMENT_SCHEMA.json",
  "formal-successor/PHASE_B_INDEXED_MEET_REFINEMENT_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/IndexedMeetRefinement.lean",
  "formal-successor/PHASE_B_PROPERTY_IMAGE_HOLE_SCHEMA.json",
  "formal-successor/PHASE_B_PROPERTY_IMAGE_HOLE_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/PropertyImageHole.lean",
  "formal-successor/PHASE_B_PROTECTED_DETERMINATION_SCHEMA.json",
  "formal-successor/PHASE_B_PROTECTED_DETERMINATION_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/ProtectedDetermination.lean",
  "formal-successor/PHASE_B_EXACT_DETERMINATION_SIGNATURE_SCHEMA.json",
  "formal-successor/PHASE_B_EXACT_DETERMINATION_SIGNATURE_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/ExactDeterminationSignature.lean",
  "formal-successor/PHASE_B_RESIDUAL_AMBIGUITY_SCHEMA.json",
  "formal-successor/PHASE_B_RESIDUAL_AMBIGUITY_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/ResidualAmbiguity.lean",
  "formal-successor/PHASE_B_REPRESENTATION_DEFECT_SCHEMA.json",
  "formal-successor/PHASE_B_REPRESENTATION_DEFECT_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/RepresentationDefect.lean",
  "formal-successor/PHASE_B_SEPARATING_CONTEXT_QUESTION_SCHEMA.json",
  "formal-successor/PHASE_B_SEPARATING_CONTEXT_QUESTION_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/SeparatingContextQuestion.lean",
  "formal-successor/PHASE_B_REPRESENTATION_QUESTION_SCHEMA.json",
  "formal-successor/PHASE_B_REPRESENTATION_QUESTION_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/RepresentationQuestion.lean",
  "formal-successor/PHASE_B_GRAIN_QUESTION_SCHEMA.json",
  "formal-successor/PHASE_B_GRAIN_QUESTION_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/GrainQuestion.lean",
  "formal-successor/PHASE_B_PROBE_TOOL_INVENTION_QUESTION_SCHEMA.json",
  "formal-successor/PHASE_B_PROBE_TOOL_INVENTION_QUESTION_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/ProbeToolInventionQuestion.lean",
  "formal-successor/PHASE_B_REPRESENTATION_GAP_LOCALIZATION_SCHEMA.json",
  "formal-successor/PHASE_B_REPRESENTATION_GAP_LOCALIZATION_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/RepresentationGapLocalization.lean",
  "formal-successor/PHASE_B_TYPED_DISTINCTION_SCHEMA_SCHEMA.json",
  "formal-successor/PHASE_B_TYPED_DISTINCTION_SCHEMA_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/TypedDistinctionSchema.lean",
  "formal-successor/PHASE_B_CANDIDATE_BOUNDARY_INCIDENCE_SCHEMA.json",
  "formal-successor/PHASE_B_CANDIDATE_BOUNDARY_INCIDENCE_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/CandidateBoundaryIncidence.lean",
  "formal-successor/PHASE_B_BOUNDARY_POINT_PROFILE_SCHEMA.json",
  "formal-successor/PHASE_B_BOUNDARY_POINT_PROFILE_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/BoundaryPointProfile.lean",
  "formal-successor/PHASE_B_BOUNDARY_POINT_REGENERATION_SCHEMA.json",
  "formal-successor/PHASE_B_BOUNDARY_POINT_REGENERATION_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/BoundaryPointRegeneration.lean",
  "formal-successor/PHASE_B_DETERMINATION_PRESENTATION_SCHEMA.json",
  "formal-successor/PHASE_B_DETERMINATION_PRESENTATION_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/DeterminationPresentation.lean",
  "formal-successor/PHASE_B_POSITIVE_DEPARTURE_WITNESS_SCHEMA.json",
  "formal-successor/PHASE_B_POSITIVE_DEPARTURE_WITNESS_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/PositiveDepartureWitness.lean",
  "formal-successor/PHASE_B_DEPARTURE_RELATIVE_POSITIVITY_SCHEMA.json",
  "formal-successor/PHASE_B_DEPARTURE_RELATIVE_POSITIVITY_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/DepartureRelativePositivity.lean",
  "formal-successor/PHASE_B_DERIVED_BOUNDARY_CROSSING_SCHEMA.json",
  "formal-successor/PHASE_B_DERIVED_BOUNDARY_CROSSING_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/DerivedBoundaryCrossing.lean",
  "formal-successor/PHASE_B_RELATION_AND_NEGATION_USE_SCHEMA.json",
  "formal-successor/PHASE_B_RELATION_AND_NEGATION_USE_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/RelationAndNegationUse.lean",
  "formal-successor/PHASE_B_POSITIVE_NEGATION_FILLING_SCHEMA.json",
  "formal-successor/PHASE_B_POSITIVE_NEGATION_FILLING_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/PositiveNegationFilling.lean",
  "formal-successor/PHASE_B_SEMANTIC_AND_EXECUTION_COVERAGE_SCHEMA.json",
  "formal-successor/PHASE_B_SEMANTIC_AND_EXECUTION_COVERAGE_SURFACE.json",
  "formal/InquiryCalculus/Legacy/V20/SemanticAndExecutionCoverage.lean",
  "formal-successor/Questions.txt",
  "formal-successor/PREDECESSOR_BASELINE.md",
  "formal-successor/CONFORMANCE_STATUS.md",
  "formal-successor/DECISIONS.jsonl",
  "formal-successor/ENGINEERING_QUESTION_PROGRAMS.json",
  "formal-successor/FAILURES.jsonl",
  "formal-successor/reports/latest.json",
  ".claude/hooks/ic-question-program.js",
  ".claude/hooks/ic-residual-topology.js",
  ".claude/hooks/ic-construction-policy.js",
  "tools/harness_acceptance_check.js",
  "tools/exploration_algorithm_check.js",
  "tools/predecessor_inventory.js",
  "tools/predecessor_inventory_check.js",
  "tools/predecessor_tex_classification.js",
  "tools/predecessor_tex_classification_check.js",
  "tools/predecessor_implementation_classification.js",
  "tools/predecessor_implementation_classification_check.js",
  "tools/predecessor_fixture_classification.js",
  "tools/predecessor_fixture_classification_check.js",
  "tools/phase_a_coverage.js",
  "tools/phase_a_coverage_check.js",
  "tools/phase_b_predecessor_spine.js",
  "tools/phase_b_predecessor_spine_check.js",
  "tools/phase_b_binding_type.js",
  "tools/phase_b_binding_type_check.js",
  "tools/phase_b_forms.js",
  "tools/phase_b_forms_check.js",
  "tools/phase_b_relations.js",
  "tools/phase_b_relations_check.js",
  "tools/phase_b_refinement.js",
  "tools/phase_b_refinement_check.js",
  "tools/phase_b_formula_grammar.js",
  "tools/phase_b_formula_grammar_check.js",
  "tools/phase_b_minimal_logical_basis.js",
  "tools/phase_b_minimal_logical_basis_check.js",
  "tools/phase_b_relation_expression_ir.js",
  "tools/phase_b_relation_expression_ir_check.js",
  "tools/phase_b_relation_schema_ports.js",
  "tools/phase_b_relation_schema_ports_check.js",
  "tools/phase_b_partial_binding_fiber.js",
  "tools/phase_b_partial_binding_fiber_check.js",
  "tools/phase_b_canonical_question_syntax.js",
  "tools/phase_b_canonical_question_syntax_check.js",
  "tools/phase_b_answer_carrier_validity.js",
  "tools/phase_b_answer_carrier_validity_check.js",
  "tools/phase_b_proposition_not_warrant.js",
  "tools/phase_b_proposition_not_warrant_check.js",
  "tools/phase_b_many_questions_generation.js",
  "tools/phase_b_many_questions_generation_check.js",
  "tools/phase_b_discharge_mode_syntax.js",
  "tools/phase_b_discharge_mode_syntax_check.js",
  "tools/phase_b_question_composition_syntax.js",
  "tools/phase_b_question_composition_syntax_check.js",
  "tools/phase_b_question_conditioned_discrimination.js",
  "tools/phase_b_question_conditioned_discrimination_check.js",
  ".gitattributes",
];
for (const name of required) requireFile(name);

requireContains("AGENTS.md", [
  "FORMAL-SUCCESSOR BRANCH CONTRACT",
  "formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md",
  "formal-successor/CONFORMANCE_STATUS.md",
  "Until Formal Gate F",
  "Operational question programs from `Questions.txt`",
  "outer repository work lifecycle",
  "admissibility; discrimination; path/direction/order; actuality/support; and",
  "RECIPROCAL WHY PAIR",
  "first policy record",
  "residual-selected compiled-question sequence",
  "represented with its declared opposed corpus pair or individually recorded as typed but blocked",
  "QUESTION_BANK_DERIVED_EXPLORATION_ALGORITHM.md",
]);

requireContains(".claude/hooks/ic-question-program.js", [
  "PREFORMAL_SUCCESSOR_CODING_INQUIRY_HARNESS.md",
  "required reciprocal closure is incomplete",
  "question-program record is detached from the active trace policy",
  "residual_class",
]);

requireContains(".claude/hooks/ic-inject", [
  "outer repository lifecycle",
  "residual-required reciprocal challenge",
  "decisive admissible contrast",
]);

requireContains(".claude/skills/inquire/references/software-engineering-binding.md", [
  "outer repository work lifecycle",
  "Coding + Reciprocal-Why programs",
  "MAXIMAL BREAK / SUBTRACT",
  "strong admissible contrast",
]);

requireContains("formal-successor/QUESTION_RHYTHM.md", [
  "## Two control levels",
  "## The actual default questions",
  "## The actual reciprocal challenges",
  "CONSTRAIN <-> RELEASE",
  "DISTINGUISH <-> COARSEN",
  "## Residual-selected rhythms",
  "## Rebuildable residual index",
  "## Demand-driven boundary incidence",
  "## Residual-shape method dispatch",
]);

requireContains(".gitattributes", ["formal-successor/Questions.txt -text -whitespace"]);

requireContains("formal-successor/AUTONOMOUS_ITERATION.md", [
  "## Persistent objective",
  "## Resume coordinate",
  "## One finite ratchet",
  "After each actual return",
  "The harness rejects a residual while any raw return lacks a subsequent",
  "## Phase progression",
  "## Autonomous safety boundary",
]);

requireContains("formal-successor/PHASE_A_INVENTORY.md", [
  "overinclusive, generated view",
  "Line numbers are source loci, never the sole identity",
  "Gate A stays",
]);

requireContains("formal-successor/PHASE_A_TEX_CLASSIFICATION.md", [
  "six dispositions required by",
  "keyword absence cannot discharge it",
  "Formal Gate A remains pending",
]);

requireContains("formal-successor/PHASE_A_IMPLEMENTATION_CLASSIFICATION.md", [
  "10,300 edges",
  "future correspondence therefore remains `Unknown`",
  "not accepted correspondence",
  "Formal Gate A remains pending",
]);

requireContains("formal-successor/PHASE_A_FIXTURE_CLASSIFICATION.md", [
  "190 exact status-row occurrences",
  "registry-only count",
  "`Unknown` successor standing",
  "Formal Gate A remains `PENDING`",
]);

requireContains("formal-successor/PHASE_A_COVERAGE.md", [
  "all 3,662 source identities",
  "It cannot pass Gate A",
  "rejects 19",
  "Total ownership is not semantic proof",
]);

requireContains("formal-successor/PHASE_B_AMBIENT_BOUNDARY.md", [
  "41 exact reviewed TeX identities",
  "13 elaboration layers",
  "FORMAL-B-BINDING-TYPE-SURFACE",
  "Formal Gate B remains `PENDING`",
]);

requireContains("formal-successor/PHASE_B_BINDING_TYPE_SURFACE.md", [
  "fifteen explicit reference-type alternatives",
  "ReferenceTypeGrammarObligation",
  "14 mutations",
  "Gate B",
]);

requireContains("formal-successor/PHASE_B_FORMS.md", [
  "typed represented-form carrier",
  "partial operational interpretation",
  "RepresentedFormObligation",
  "nine mutations",
  "Gate B",
]);

requireContains("formal-successor/PHASE_B_RELATIONS.md", [
  "seven consecutive explicit v2.0 relation definitions",
  "RelationSchema",
  "ConverseBoundary",
  "eleven mutations",
  "Gate B",
]);

requireContains("formal-successor/PHASE_B_REFINEMENT.md", [
  "coarsest represented relation",
  "ExistenceBoundary",
  "reverses inclusion",
  "twelve mutations",
  "Gate B",
]);

requireContains("formal-successor/PHASE_B_FORMULA_GRAMMAR.md", [
  "six selected source records",
  "CandidateFormulaSyntax",
  "does not create an oriented negation use",
  "thirteen mutations",
  "Gate B",
]);

requireContains("formal-successor/PHASE_B_MINIMAL_LOGICAL_BASIS.md", [
  "seven selected records",
  "NativeComplementBoundary",
  "not a semantic equivalence theorem",
  "fourteen mutations",
  "Gate B",
]);

requireContains("formal-successor/PHASE_B_RELATION_EXPRESSION_IR.md", [
  "three selected source records",
  "RelationExpressionIR",
  "does not by itself create a semantic question",
  "thirteen mutations",
  "Gate B",
]);

requireContains("formal-successor/CONFORMANCE_STATUS.md", [
  "FORMAL-A-COVERAGE-001 | PASS",
  "FORMAL-GATE-A | PASS",
  "FORMAL-B-AMBIENT-BOUNDARY-001 | PASS",
  "inventory closure, not a successor definition",
  "Gate B stays pending",
]);

let predecessorInventoryGrammar;
let predecessorInventory;
try {
  predecessorInventoryGrammar = JSON.parse(read("formal-successor/PREDECESSOR_INVENTORY_GRAMMAR.json"));
  predecessorInventory = JSON.parse(read("formal-successor/PREDECESSOR_INVENTORY.json"));
} catch (error) {
  errors.push(`predecessor inventory control JSON: ${error.message}`);
}
if (
  predecessorInventoryGrammar?.status !== "phase_a_extraction_grammar_not_successor_semantics" ||
  predecessorInventoryGrammar?.predecessor_commit !== baseline ||
  predecessorInventoryGrammar?.canonical_tex?.sha256 !== "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89"
) {
  errors.push("Phase A predecessor inventory grammar is detached from its authority or pinned inputs");
}
if (
  predecessorInventory?.status !== "generated_phase_a_predecessor_inventory_not_gate_a" ||
  predecessorInventory?.gate_a?.status !== "PENDING" ||
  !(predecessorInventory?.coverage?.pending_review_items > 0)
) {
  errors.push("generated predecessor inventory must preserve an explicit pending Gate A review residual");
}

let predecessorTexClassificationSchema;
let predecessorTexClassification;
try {
  predecessorTexClassificationSchema = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION_SCHEMA.json"));
  predecessorTexClassification = JSON.parse(read("formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json"));
} catch (error) {
  errors.push(`predecessor TeX classification control JSON: ${error.message}`);
}
if (
  predecessorTexClassificationSchema?.status !== "phase_a_tex_classification_contract_not_successor_semantics" ||
  predecessorTexClassificationSchema?.inventory?.canonical_tex_sha256 !==
    "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89" ||
  predecessorTexClassificationSchema?.construction_specification_dispositions?.length !== 6
) {
  errors.push("Phase A TeX classification schema is detached from its authority or six-way contract");
}
if (
  predecessorTexClassification?.status !== "reviewed_phase_a_tex_classification_not_successor_semantics" ||
  predecessorTexClassification?.coverage?.classified_source_items !== 1370 ||
  predecessorTexClassification?.coverage?.unclassified_source_items !== 0 ||
  predecessorTexClassification?.formal_gate_a?.status !== "PENDING"
) {
  errors.push("Phase A TeX classification must be total at TeX coverage while preserving pending Formal Gate A");
}

let predecessorImplementationClassificationSchema;
let predecessorImplementationClassification;
try {
  predecessorImplementationClassificationSchema = JSON.parse(read("formal-successor/PREDECESSOR_IMPLEMENTATION_CLASSIFICATION_SCHEMA.json"));
  predecessorImplementationClassification = JSON.parse(read("formal-successor/PREDECESSOR_IMPLEMENTATION_CLASSIFICATION.json"));
} catch (error) {
  errors.push(`predecessor implementation classification control JSON: ${error.message}`);
}
if (
  predecessorImplementationClassificationSchema?.status !==
    "phase_a_implementation_classification_contract_not_semantic_authority" ||
  predecessorImplementationClassificationSchema?.inputs?.predecessor_commit !== baseline ||
  predecessorImplementationClassificationSchema?.edge_grammar?.authority !==
    "source_incidence_candidate_not_semantic_correspondence"
) {
  errors.push("Phase A implementation classification schema is detached from its authority boundary");
}
if (
  predecessorImplementationClassification?.status !==
    "reviewed_phase_a_implementation_classification_not_semantic_authority" ||
  predecessorImplementationClassification?.coverage?.classified_source_items !== 2062 ||
  predecessorImplementationClassification?.coverage?.direct_exact_symbol_edges !== 15 ||
  predecessorImplementationClassification?.coverage?.unclassified_source_items !== 0 ||
  predecessorImplementationClassification?.formal_gate_a?.status !== "PENDING"
) {
  errors.push("Phase A implementation classification must be exact, total, candidate-only, and non-promoting");
}

let predecessorFixtureClassificationSchema;
let predecessorFixtureClassification;
try {
  predecessorFixtureClassificationSchema = JSON.parse(read("formal-successor/PREDECESSOR_FIXTURE_CLASSIFICATION_SCHEMA.json"));
  predecessorFixtureClassification = JSON.parse(read("formal-successor/PREDECESSOR_FIXTURE_CLASSIFICATION.json"));
} catch (error) {
  errors.push(`predecessor fixture classification control JSON: ${error.message}`);
}
if (
  predecessorFixtureClassificationSchema?.status !==
    "phase_a_fixture_classification_contract_not_semantic_authority" ||
  predecessorFixtureClassificationSchema?.inputs?.predecessor_commit !== baseline ||
  predecessorFixtureClassificationSchema?.standing_law?.successor_standing !== "Unknown"
) {
  errors.push("Phase A fixture classification schema is detached from its authority boundary");
}
if (
  predecessorFixtureClassification?.status !==
    "reviewed_phase_a_fixture_classification_not_semantic_authority" ||
  predecessorFixtureClassification?.coverage?.classified_source_items !== 226 ||
  predecessorFixtureClassification?.coverage?.exact_execution_routes !== 200 ||
  predecessorFixtureClassification?.coverage?.registry_without_status_rows !== 0 ||
  predecessorFixtureClassification?.coverage?.successor_standing_counts?.Unknown !== 226 ||
  predecessorFixtureClassification?.coverage?.unclassified_source_items !== 0 ||
  predecessorFixtureClassification?.formal_gate_a?.status !== "PENDING"
) {
  errors.push("Phase A fixture classification must be exact, total, authority-separated, and non-promoting");
}

let phaseACoverageSchema;
let phaseACoverageCertificate;
try {
  phaseACoverageSchema = JSON.parse(read("formal-successor/PHASE_A_COVERAGE_SCHEMA.json"));
  phaseACoverageCertificate = JSON.parse(read("formal-successor/PHASE_A_COVERAGE_CERTIFICATE.json"));
} catch (error) {
  errors.push(`Phase A coverage control JSON: ${error.message}`);
}
if (
  phaseACoverageSchema?.status !== "phase_a_joined_coverage_contract_not_successor_semantics" ||
  phaseACoverageSchema?.inputs?.predecessor_commit !== baseline ||
  phaseACoverageSchema?.expected_boundary_at_pinned_inputs?.inventory_items !== 3662
) {
  errors.push("Phase A joined coverage schema is detached from the corrected inventory boundary");
}
if (
  phaseACoverageCertificate?.status !== "generated_phase_a_joined_coverage_candidate_not_self_warrant" ||
  phaseACoverageCertificate?.observed_boundary?.inventory_items !== 3662 ||
  phaseACoverageCertificate?.observed_boundary?.owner_intersections !== 0 ||
  phaseACoverageCertificate?.observed_boundary?.unowned_items !== 0 ||
  phaseACoverageCertificate?.observed_boundary?.invalid_edge_targets !== 0 ||
  phaseACoverageCertificate?.gate_a_candidate?.status !== "READY_FOR_INDEPENDENT_CHECK"
) {
  errors.push("Phase A coverage certificate must be exact, closed, and non-self-warranting");
}

let phaseBSpineSchema;
let phaseBSpine;
try {
  phaseBSpineSchema = JSON.parse(read("formal-successor/PHASE_B_PREDECESSOR_SPINE_SCHEMA.json"));
  phaseBSpine = JSON.parse(read("formal-successor/PHASE_B_PREDECESSOR_SPINE.json"));
} catch (error) {
  errors.push(`Phase B predecessor spine control JSON: ${error.message}`);
}
if (
  phaseBSpineSchema?.status !== "phase_b_predecessor_elaboration_spine_not_successor_semantics" ||
  phaseBSpineSchema?.required_layer_order?.length !== 13 ||
  phaseBSpineSchema?.gate_b?.status !== "PENDING"
) {
  errors.push("Phase B predecessor spine schema is detached from its local non-promotion boundary");
}
if (
  phaseBSpine?.status !== "generated_phase_b_predecessor_spine_not_semantic_completion" ||
  phaseBSpine?.coverage?.layer_count !== 13 ||
  phaseBSpine?.coverage?.selected_source_count !== 41 ||
  phaseBSpine?.coverage?.checked_boundary_layers !== 1 ||
  phaseBSpine?.coverage?.open_layers !== 12 ||
  phaseBSpine?.ambient_boundary?.classicality_status !== "ExplicitPredecessorObligation" ||
  phaseBSpine?.next_residual?.id !== "FORMAL-B-BINDING-TYPE-SURFACE" ||
  phaseBSpine?.formal_gate_b?.status !== "PENDING"
) {
  errors.push("Phase B predecessor spine must retain its exact ambient-only pass and open Gate B residual");
}

let phaseBBindingTypeSchema;
let phaseBBindingTypeSurface;
try {
  phaseBBindingTypeSchema = JSON.parse(read("formal-successor/PHASE_B_BINDING_TYPE_SCHEMA.json"));
  phaseBBindingTypeSurface = JSON.parse(read("formal-successor/PHASE_B_BINDING_TYPE_SURFACE.json"));
} catch (error) {
  errors.push(`Phase B binding/type control JSON: ${error.message}`);
}
if (
  phaseBBindingTypeSchema?.status !== "phase_b_binding_type_contract_not_successor_semantics" ||
  phaseBBindingTypeSchema?.binding_slots?.length !== 10 ||
  phaseBBindingTypeSchema?.type_constructors?.length !== 15 ||
  phaseBBindingTypeSchema?.gate_b?.status !== "PENDING"
) errors.push("Phase B binding/type schema is detached from the explicit predecessor boundary");
if (
  phaseBBindingTypeSurface?.status !== "generated_phase_b_binding_type_surface_not_successor_semantics" ||
  phaseBBindingTypeSurface?.retained_surface?.explicit_definition_count !== 4 ||
  phaseBBindingTypeSurface?.retained_surface?.explicit_obligation_count !== 3 ||
  phaseBBindingTypeSurface?.next_residual !== "FORMAL-B-REPRESENTED-FORMS" ||
  phaseBBindingTypeSurface?.formal_gate_b?.status !== "PENDING"
) errors.push("Phase B binding/type surface must preserve source obligations and pending Gate B");

let phaseBFormsSchema;
let phaseBFormsSurface;
try {
  phaseBFormsSchema = JSON.parse(read("formal-successor/PHASE_B_FORMS_SCHEMA.json"));
  phaseBFormsSurface = JSON.parse(read("formal-successor/PHASE_B_FORMS_SURFACE.json"));
} catch (error) {
  errors.push(`Phase B forms control JSON: ${error.message}`);
}
if (
  phaseBFormsSchema?.status !== "phase_b_forms_contract_not_successor_semantics" ||
  phaseBFormsSchema?.sources?.length !== 7 ||
  phaseBFormsSchema?.required_declarations?.length !== 7 ||
  phaseBFormsSchema?.obligations?.length !== 3 ||
  phaseBFormsSchema?.gate_b?.status !== "PENDING"
) errors.push("Phase B forms schema is detached from the explicit predecessor boundary");
if (
  phaseBFormsSurface?.status !== "generated_phase_b_forms_surface_not_successor_semantics" ||
  phaseBFormsSurface?.coverage?.explicit_definitions !== 4 ||
  phaseBFormsSurface?.coverage?.obligations !== 3 ||
  phaseBFormsSurface?.next_residual !== "FORMAL-B-TYPED-RELATIONS" ||
  phaseBFormsSurface?.formal_gate_b?.status !== "PENDING"
) errors.push("Phase B forms surface must preserve source obligations and pending Gate B");

let phaseBRelationsSchema;
let phaseBRelationsSurface;
try {
  phaseBRelationsSchema = JSON.parse(read("formal-successor/PHASE_B_RELATIONS_SCHEMA.json"));
  phaseBRelationsSurface = JSON.parse(read("formal-successor/PHASE_B_RELATIONS_SURFACE.json"));
} catch (error) {
  errors.push(`Phase B relations control JSON: ${error.message}`);
}
if (
  phaseBRelationsSchema?.status !== "phase_b_relations_contract_not_successor_semantics" ||
  phaseBRelationsSchema?.sources?.length !== 7 ||
  phaseBRelationsSchema?.required_declarations?.length !== 11 ||
  phaseBRelationsSchema?.gate_b?.status !== "PENDING"
) errors.push("Phase B relations schema is detached from the explicit predecessor boundary");
if (
  phaseBRelationsSurface?.status !== "generated_phase_b_relations_surface_not_successor_semantics" ||
  phaseBRelationsSurface?.coverage?.explicit_definitions !== 7 ||
  phaseBRelationsSurface?.coverage?.obligations !== 0 ||
  phaseBRelationsSurface?.next_residual !== "FORMAL-B-COARSE-RELATION-REFINEMENT" ||
  phaseBRelationsSurface?.formal_gate_b?.status !== "PENDING"
) errors.push("Phase B relations surface must preserve its typed boundary and pending Gate B");

let phaseBRefinementSchema;
let phaseBRefinementSurface;
try {
  phaseBRefinementSchema = JSON.parse(read("formal-successor/PHASE_B_REFINEMENT_SCHEMA.json"));
  phaseBRefinementSurface = JSON.parse(read("formal-successor/PHASE_B_REFINEMENT_SURFACE.json"));
} catch (error) {
  errors.push(`Phase B refinement control JSON: ${error.message}`);
}
if (
  phaseBRefinementSchema?.status !== "phase_b_refinement_contract_not_successor_semantics" ||
  phaseBRefinementSchema?.sources?.length !== 5 ||
  phaseBRefinementSchema?.required_declarations?.length !== 6 ||
  phaseBRefinementSchema?.obligations?.length !== 4 ||
  phaseBRefinementSchema?.gate_b?.status !== "PENDING"
) errors.push("Phase B refinement schema is detached from the explicit predecessor boundary");
if (
  phaseBRefinementSurface?.status !== "generated_phase_b_refinement_surface_not_successor_semantics" ||
  phaseBRefinementSurface?.coverage?.explicit_definitions !== 4 ||
  phaseBRefinementSurface?.coverage?.obligations !== 1 ||
  phaseBRefinementSurface?.next_residual !== "FORMAL-B-FORMULA-GRAMMAR" ||
  phaseBRefinementSurface?.formal_gate_b?.status !== "PENDING"
) errors.push("Phase B refinement surface must preserve the explicit obligation and pending Gate B");

let phaseBFormulaGrammarSchema;
let phaseBFormulaGrammarSurface;
try {
  phaseBFormulaGrammarSchema = JSON.parse(read("formal-successor/PHASE_B_FORMULA_GRAMMAR_SCHEMA.json"));
  phaseBFormulaGrammarSurface = JSON.parse(read("formal-successor/PHASE_B_FORMULA_GRAMMAR_SURFACE.json"));
} catch (error) {
  errors.push(`Phase B formula grammar control JSON: ${error.message}`);
}
if (
  phaseBFormulaGrammarSchema?.status !== "phase_b_formula_grammar_contract_not_successor_semantics" ||
  phaseBFormulaGrammarSchema?.sources?.length !== 6 ||
  phaseBFormulaGrammarSchema?.required_declarations?.length !== 5 ||
  phaseBFormulaGrammarSchema?.obligations?.length !== 6 ||
  phaseBFormulaGrammarSchema?.gate_b?.status !== "PENDING"
) errors.push("Phase B formula grammar schema is detached from its ambiguous predecessor boundary");
if (
  phaseBFormulaGrammarSurface?.status !== "generated_phase_b_formula_grammar_surface_not_successor_semantics" ||
  phaseBFormulaGrammarSurface?.coverage?.explicit_definitions !== 0 ||
  phaseBFormulaGrammarSurface?.coverage?.obligations !== 6 ||
  phaseBFormulaGrammarSurface?.coverage?.obligation_statuses?.filter((status) => status === "Ambiguous")?.length !== 5 ||
  phaseBFormulaGrammarSurface?.coverage?.obligation_statuses?.filter((status) => status === "Unproved")?.length !== 1 ||
  phaseBFormulaGrammarSurface?.next_residual !== "FORMAL-B-MINIMAL-LOGICAL-BASIS" ||
  phaseBFormulaGrammarSurface?.formal_gate_b?.status !== "PENDING"
) errors.push("Phase B formula grammar surface must preserve all source ambiguity and pending Gate B");

let phaseBMinimalLogicalBasisSchema;
let phaseBMinimalLogicalBasisSurface;
try {
  phaseBMinimalLogicalBasisSchema = JSON.parse(read("formal-successor/PHASE_B_MINIMAL_LOGICAL_BASIS_SCHEMA.json"));
  phaseBMinimalLogicalBasisSurface = JSON.parse(read("formal-successor/PHASE_B_MINIMAL_LOGICAL_BASIS_SURFACE.json"));
} catch (error) {
  errors.push(`Phase B minimal logical basis control JSON: ${error.message}`);
}
if (
  phaseBMinimalLogicalBasisSchema?.status !== "phase_b_minimal_logical_basis_contract_not_successor_semantics" ||
  phaseBMinimalLogicalBasisSchema?.sources?.length !== 7 ||
  phaseBMinimalLogicalBasisSchema?.required_declarations?.length !== 7 ||
  phaseBMinimalLogicalBasisSchema?.obligations?.length !== 7 ||
  phaseBMinimalLogicalBasisSchema?.gate_b?.status !== "PENDING"
) errors.push("Phase B minimal logical basis schema is detached from its ambiguous predecessor boundary");
if (
  phaseBMinimalLogicalBasisSurface?.status !== "generated_phase_b_minimal_logical_basis_surface_not_successor_semantics" ||
  phaseBMinimalLogicalBasisSurface?.coverage?.explicit_definitions !== 0 ||
  phaseBMinimalLogicalBasisSurface?.coverage?.obligations !== 7 ||
  phaseBMinimalLogicalBasisSurface?.coverage?.obligation_statuses?.every((status) => status === "Ambiguous") !== true ||
  phaseBMinimalLogicalBasisSurface?.next_residual !== "FORMAL-B-RELATION-EXPRESSION-IR" ||
  phaseBMinimalLogicalBasisSurface?.formal_gate_b?.status !== "PENDING"
) errors.push("Phase B minimal logical basis surface must preserve all ambiguity and pending Gate B");

let phaseBRelationExpressionIRSchema;
let phaseBRelationExpressionIRSurface;
try {
  phaseBRelationExpressionIRSchema = JSON.parse(read("formal-successor/PHASE_B_RELATION_EXPRESSION_IR_SCHEMA.json"));
  phaseBRelationExpressionIRSurface = JSON.parse(read("formal-successor/PHASE_B_RELATION_EXPRESSION_IR_SURFACE.json"));
} catch (error) {
  errors.push(`Phase B relation-expression IR control JSON: ${error.message}`);
}
if (
  phaseBRelationExpressionIRSchema?.status !== "phase_b_relation_expression_ir_contract_not_successor_semantics" ||
  phaseBRelationExpressionIRSchema?.sources?.length !== 3 ||
  phaseBRelationExpressionIRSchema?.required_declarations?.length !== 6 ||
  phaseBRelationExpressionIRSchema?.obligations?.length !== 3 ||
  phaseBRelationExpressionIRSchema?.gate_b?.status !== "PENDING"
) errors.push("Phase B relation-expression IR schema is detached from its ambiguous predecessor boundary");
if (
  phaseBRelationExpressionIRSurface?.status !== "generated_phase_b_relation_expression_ir_surface_not_successor_semantics" ||
  phaseBRelationExpressionIRSurface?.coverage?.explicit_definitions !== 0 ||
  phaseBRelationExpressionIRSurface?.coverage?.obligations !== 3 ||
  phaseBRelationExpressionIRSurface?.coverage?.obligation_statuses?.every((status) => status === "Ambiguous") !== true ||
  phaseBRelationExpressionIRSurface?.next_residual !== "FORMAL-B-RELATION-SCHEMAS-PORTS" ||
  phaseBRelationExpressionIRSurface?.formal_gate_b?.status !== "PENDING"
) errors.push("Phase B relation-expression IR surface must preserve all ambiguity and pending Gate B");

const frontier = read("IMPLEMENTATION_FRONTIER.md");
if ((frontier.match(/<!-- LIVE_FRONTIER_BEGIN -->/gu) ?? []).length !== 1 ||
    (frontier.match(/<!-- LIVE_FRONTIER_END -->/gu) ?? []).length !== 1) {
  errors.push("IMPLEMENTATION_FRONTIER.md must contain exactly one live block");
}
if (!frontier.includes("id: FORMAL-") || frontier.includes("id: QASK-MIXED-RESOLUTION-007")) {
  errors.push("root frontier is not routed exclusively to the formal successor");
}
for (const key of [
  "id", "plan_phase", "goal", "protected_difference", "discriminator", "horizon",
  "relevant_decisions", "relevant_failures", "if_pass", "if_fail",
]) {
  const count = frontier.match(new RegExp(`^${key}:`, "gmu"))?.length ?? 0;
  if (count !== 1) errors.push(`live frontier key ${key} must occur exactly once (found ${count})`);
}

let inputs;
try {
  inputs = JSON.parse(read("formal-successor/ACTIVE_INPUTS.json"));
} catch (error) {
  errors.push(`formal-successor/ACTIVE_INPUTS.json: ${error.message}`);
}
const expectedInputs = new Map([
  ["Inquiry_Calculus_v2_0.tex", "1f548e0fa3e8374a01b6268e813cedcc757b26ed460adb5b82fe8ba60ca1dd89"],
  ["formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md", "c62ac86b3f551d03ce687e28f0870f53af19a22d70f41fe4a468e707d4da540e"],
  ["formal-successor/Questions.txt", "5a0dbb45bd1e9ff838a0396f6a1f17ba23cfa726889d28ab9b809b0c9b9cd019"],
  ["formal-successor/PREFORMAL_SUCCESSOR_CODING_INQUIRY_HARNESS.md", "fc80d0e84a3fa5a2a16a15354e4ae5d6b7f342f9cfe8bdaf3d2b83c2fe8e357e"],
  ["formal-successor/PREFORMAL_SEARCH_ASYMMETRY.md", "74f6abf1fb944e76d65071c83ff0fe319ed008d4d6e086de20afe22ca323c765"],
  ["formal-successor/SUCCESSOR_CONSTRUCTION_HARNESS_SPEC.md", "4695daa72e33d2e3d82300047124030de5d13fe82c28cee086ef27da111fefc2"],
  ["formal-successor/QUESTION_BANK_DERIVED_EXPLORATION_ALGORITHM.md", "82cfaaae3e25b4703f2d73554676047844afa0f7b9807bfadaf6b2716fefefec"],
]);
if (inputs) {
  if (inputs.branch !== "codex/formal-successor" || inputs.predecessor_commit !== baseline) {
    errors.push("ACTIVE_INPUTS branch or predecessor coordinate changed without a control migration");
  }
  const declared = new Map((inputs.inputs ?? []).map((item) => [item.path, item.sha256]));
  for (const [name, expected] of expectedInputs) {
    if (declared.get(name) !== expected) errors.push(`${name}: declared digest is not the accepted input digest`);
    const actual = digest(name);
    if (actual !== expected) errors.push(`${name}: expected SHA-256 ${expected}, got ${actual}`);
  }
  if (inputs.adopted_process_input?.source_sha256 !==
      "7f316218d7a9aa9ba17461445b575034d45365020c785e4f5de9f029b94c8f89" ||
      inputs.adopted_process_input?.role !== "engineering_control_proposal_not_semantic_authority") {
    errors.push("the proposed engineering clock is not classified as non-semantic process input");
  }
}

let questionPrograms;
try {
  questionPrograms = JSON.parse(read("formal-successor/ENGINEERING_QUESTION_PROGRAMS.json"));
} catch (error) {
  errors.push(`formal-successor/ENGINEERING_QUESTION_PROGRAMS.json: ${error.message}`);
}
if (questionPrograms) {
  const questions = read("formal-successor/Questions.txt").split(/\r?\n/u);
  const line = (number) => questions[number - 1] ?? "";
  if (questionPrograms.schema !== 3) {
    errors.push("active engineering question-program manifest must use schema 3");
  }
  if (questionPrograms.source_sha256 !== expectedInputs.get("formal-successor/Questions.txt")) {
    errors.push("engineering question programs are not bound to the accepted corpus digest");
  }
  const harness = questionPrograms.preformal_harness;
  if (
    harness?.source !== "formal-successor/PREFORMAL_SUCCESSOR_CODING_INQUIRY_HARNESS.md" ||
    harness?.source_sha256 !==
      expectedInputs.get("formal-successor/PREFORMAL_SUCCESSOR_CODING_INQUIRY_HARNESS.md") ||
    digest("formal-successor/PREFORMAL_SUCCESSOR_CODING_INQUIRY_HARNESS.md") !==
      harness?.source_sha256
  ) {
    errors.push("engineering question programs are detached from the pinned preformal harness");
  }
  if (
    harness?.search_asymmetry_source !== "formal-successor/PREFORMAL_SEARCH_ASYMMETRY.md" ||
    harness?.search_asymmetry_sha256 !==
      expectedInputs.get("formal-successor/PREFORMAL_SEARCH_ASYMMETRY.md") ||
    digest("formal-successor/PREFORMAL_SEARCH_ASYMMETRY.md") !==
      harness?.search_asymmetry_sha256
  ) {
    errors.push("engineering question programs are detached from the pinned search asymmetry");
  }
  if (
    harness?.consolidated_spec_source !== "formal-successor/SUCCESSOR_CONSTRUCTION_HARNESS_SPEC.md" ||
    harness?.consolidated_spec_sha256 !==
      expectedInputs.get("formal-successor/SUCCESSOR_CONSTRUCTION_HARNESS_SPEC.md") ||
    digest("formal-successor/SUCCESSOR_CONSTRUCTION_HARNESS_SPEC.md") !==
      harness?.consolidated_spec_sha256
  ) {
    errors.push("engineering question programs are detached from the consolidated harness specification");
  }
  if (
    harness?.exploration_algorithm_source !== "formal-successor/QUESTION_BANK_DERIVED_EXPLORATION_ALGORITHM.md" ||
    harness?.exploration_algorithm_sha256 !==
      expectedInputs.get("formal-successor/QUESTION_BANK_DERIVED_EXPLORATION_ALGORITHM.md") ||
    digest("formal-successor/QUESTION_BANK_DERIVED_EXPLORATION_ALGORITHM.md") !==
      harness?.exploration_algorithm_sha256
  ) {
    errors.push("engineering question programs are detached from the question-bank-derived exploration algorithm");
  }
  if (
    harness?.two_scale_law?.search !== "wide_contrasting_deliberately_expansive" ||
    harness?.two_scale_law?.commit !== "small_warranted_exactly_supported" ||
    !/preserves_the_unresolved_wide_field/u.test(
      harness?.two_scale_law?.anti_premature_closure ?? "",
    )
  ) {
    errors.push("search-wide/commit-narrow and anti-premature-closure law is incomplete");
  }
  if (!/not_successor_semantics/u.test(harness?.status ?? "")) {
    errors.push("preformal harness must remain classified outside successor semantics");
  }
  const algorithm = harness?.exploration_algorithm;
  const expectedDispositions = [
    "Answered", "Productive", "Required", "Redundant", "Inapplicable", "Blocked", "Unknown",
  ];
  const expectedResolutions = [
    "Supported", "Plural", "ExactEmpty", "Unsupported", "Unknown", "Blocked", "ResourceBounded",
  ];
  if (JSON.stringify(algorithm?.question_dispositions) !== JSON.stringify(expectedDispositions)) {
    errors.push("exploration algorithm does not preserve all seven question dispositions");
  }
  if (JSON.stringify(algorithm?.resolution_classes) !== JSON.stringify(expectedResolutions)) {
    errors.push("exploration algorithm does not preserve partial and non-success resolution classes");
  }
  if ((algorithm?.condition_identity ?? []).join(",") !==
      "schema,bound_roles,scope,applicability,grain,orientation") {
    errors.push("typed condition identity is incomplete");
  }
  if (!/demand_driven/u.test(algorithm?.materialization_law ?? "")) {
    errors.push("exploration algorithm lacks demand-driven materialization");
  }
  if (!/remains_predecessor_behavior/u.test(harness?.predecessor_recurrence ?? "")) {
    errors.push("the v2.0 BIND/OPEN/VARY/RETURN/DETERMINE/REFACTOR recurrence was repurposed");
  }
  for (const section of [questionPrograms.sections?.coding, questionPrograms.sections?.reciprocal_why]) {
    if (!section || line(section.heading_line) !== section.heading) {
      errors.push(`question-program section heading does not match Questions.txt: ${section?.heading ?? "missing"}`);
    }
    if (!line(section.first_question_line).endsWith("?") ||
        !line(section.last_question_line).endsWith("?")) {
      errors.push(`question-program section bounds do not address questions: ${section?.heading ?? "missing"}`);
    }
  }
  const reciprocalEnd = questionPrograms.sections?.reciprocal_why?.last_question_line ?? 0;
  if (questions.slice(reciprocalEnd).some((entry) => entry.trim() !== "")) {
    errors.push("reciprocal-why section bound does not reach the final corpus question");
  }
  const requiredPrograms = new Set([
    "QP-CODING-FRAME",
    "QP-CODING-ROLE-INCIDENCE",
    "QP-CODING-FOLD-REGENERATE",
    "QP-CODING-TRACE-COMPOSE",
    "QP-CODING-BOUNDARY",
    "QP-CODING-PROPAGATE-RATCHET",
    "QP-WHY-RETURN-CONTRAST",
    "QP-WHY-RECIPROCAL-NECESSITY",
    "QP-WHY-CONTRACT-REOPEN",
    "QP-WHY-EVIDENCE",
    "QP-WHY-FAILURE-REPAIR",
    "QP-WHY-QUESTION-RATCHET",
  ]);
  for (const program of questionPrograms.programs ?? []) {
    requiredPrograms.delete(program.id);
    const references = [
      ...(program.sequence ?? []),
      ...(program.paired_sequence ?? []).flat(),
    ];
    if (references.length === 0) errors.push(`${program.id}: question program has no source questions`);
    for (const sourceLine of references) {
      const question = line(sourceLine);
      if (!question.endsWith("?")) {
        errors.push(`${program.id}: source line ${sourceLine} is not a question in Questions.txt`);
      }
    }
    for (const pair of program.paired_sequence ?? []) {
      if (!Array.isArray(pair) || pair.length !== 2) {
        errors.push(`${program.id}: every reciprocal/boundary pair must have exactly two orientations`);
      }
    }
  }
  if (requiredPrograms.size > 0) {
    errors.push(`missing required engineering question programs: ${[...requiredPrograms].join(", ")}`);
  }
  const expectedPositions = [
    "FRAME", "OPEN", "EXPAND", "DISTINGUISH", "CONSTRAIN", "DISCHARGE", "RESOLVE",
    "RECIPROCATE", "RELEASE_SUBTRACT", "FOLD_REOPEN",
  ];
  const expectedDimensions = [
    "ADMISSIBILITY", "DISCRIMINATION", "PATH_DIRECTION_ORDER", "ACTUALITY_SUPPORT",
    "REPRESENTATION_REGENERATION",
    "FRONTIER_ANCESTRY_REUSE",
  ];
  const expectedRoots = ["Expose", "Orient", "Factor", "Polarize", "Vary", "Ground"];
  const expectedAxes = [
    ["ADMISSIBILITY", "CONSTRAIN", "RELEASE", "solution_field"],
    ["DISCRIMINATION", "DISTINGUISH", "COARSEN", "question_kernel"],
  ];
  if (JSON.stringify(harness?.provisional_positions) !== JSON.stringify(expectedPositions)) {
    errors.push("preformal harness positions do not match the accepted provisional rhythm");
  }
  if (JSON.stringify(harness?.coverage_dimensions) !== JSON.stringify(expectedDimensions)) {
    errors.push("preformal harness does not preserve the five relational coverage dimensions");
  }
  if (JSON.stringify(harness?.root_hypothesis) !== JSON.stringify(expectedRoots)) {
    errors.push("v2.0 root lowering is not explicitly retained as a six-root hypothesis");
  }
  if (JSON.stringify((harness?.central_reciprocal_axes ?? []).map(
    (axis) => [axis.id, axis.forward, axis.reverse, axis.acts_on],
  )) !== JSON.stringify(expectedAxes)) {
    errors.push("constrain/release and distinguish/coarsen are not preserved as distinct axes");
  }

  const compiledIds = new Set();
  for (const question of harness?.compiled_questions ?? []) {
    if (compiledIds.has(question.id)) errors.push(`duplicate compiled question ${question.id}`);
    compiledIds.add(question.id);
    if (typeof question.prompt !== "string" || !question.prompt.endsWith("?")) {
      errors.push(`${question.id}: compiled prompt is not an actual question`);
    }
    if (!expectedPositions.includes(question.position)) {
      errors.push(`${question.id}: unknown preformal position ${question.position}`);
    }
    for (const dimension of question.dimensions ?? []) {
      if (!expectedDimensions.includes(dimension)) {
        errors.push(`${question.id}: unknown relational dimension ${dimension}`);
      }
    }
    for (const rootName of question.roots ?? []) {
      if (!expectedRoots.includes(rootName)) errors.push(`${question.id}: unknown root ${rootName}`);
    }
    for (const sourceLine of question.source_lines ?? []) {
      if (
        sourceLine < questionPrograms.sections.coding.first_question_line ||
        sourceLine > questionPrograms.sections.coding.last_question_line ||
        !line(sourceLine).endsWith("?")
      ) {
        errors.push(`${question.id}: invalid Coding source line ${sourceLine}`);
      }
    }
  }

  const challengeIds = new Set();
  for (const challenge of harness?.reciprocal_challenges ?? []) {
    if (challengeIds.has(challenge.id)) errors.push(`duplicate reciprocal challenge ${challenge.id}`);
    challengeIds.add(challenge.id);
    if (!Array.isArray(challenge.pair) || challenge.pair.length !== 2) {
      errors.push(`${challenge.id}: reciprocal challenge must have exactly two orientations`);
      continue;
    }
    for (const sourceLine of challenge.pair) {
      if (
        sourceLine < questionPrograms.sections.reciprocal_why.first_question_line ||
        sourceLine > questionPrograms.sections.reciprocal_why.last_question_line ||
        !line(sourceLine).endsWith("?")
      ) {
        errors.push(`${challenge.id}: invalid Reciprocal why source line ${sourceLine}`);
      }
    }
    for (const axis of challenge.axes ?? []) {
      if (!expectedAxes.some(([id]) => id === axis)) errors.push(`${challenge.id}: unknown axis ${axis}`);
    }
  }

  const expectedFamilyCodes = Array.from({ length: 14 }, (_, index) => `Q${index + 1}`).sort();
  const familyCodes = new Set();
  const familyIds = new Set();
  for (const family of harness?.program_families ?? []) {
    familyCodes.add(family.code);
    familyIds.add(family.id);
    for (const id of family.compiled_questions ?? []) {
      if (!compiledIds.has(id)) errors.push(`${family.id}: unknown compiled question ${id}`);
    }
    for (const id of family.reciprocal_challenges ?? []) {
      if (!challengeIds.has(id)) errors.push(`${family.id}: unknown reciprocal challenge ${id}`);
    }
  }
  if (JSON.stringify([...familyCodes].sort()) !== JSON.stringify(expectedFamilyCodes)) {
    errors.push("compiled question-program families must cover Q1 through Q14 exactly");
  }
  for (const question of harness?.compiled_questions ?? []) {
    if (!familyIds.has(question.family)) errors.push(`${question.id}: unknown family ${question.family}`);
  }

  const rhythmIds = new Set();
  for (const rhythm of harness?.principal_rhythms ?? []) {
    if (rhythmIds.has(rhythm.id)) errors.push(`duplicate principal rhythm ${rhythm.id}`);
    rhythmIds.add(rhythm.id);
    for (const id of rhythm.required_questions ?? []) {
      if (!compiledIds.has(id)) errors.push(`${rhythm.id}: unknown required question ${id}`);
    }
    for (const id of rhythm.required_reciprocals ?? []) {
      if (!challengeIds.has(id)) errors.push(`${rhythm.id}: unknown required reciprocal ${id}`);
    }
    const covered = new Set(
      (rhythm.required_questions ?? []).flatMap((id) =>
        (harness.compiled_questions ?? []).find((question) => question.id === id)?.dimensions ?? [],
      ),
    );
    for (const dimension of rhythm.required_dimensions ?? []) {
      if (!covered.has(dimension)) {
        errors.push(`${rhythm.id}: required dimension ${dimension} has no compiled question`);
      }
    }
  }
  for (const [residual, rhythm] of Object.entries(harness?.residual_schedule ?? {})) {
    if (!rhythmIds.has(rhythm)) errors.push(`residual ${residual} selects unknown rhythm ${rhythm}`);
  }
  if (harness?.residual_schedule?.default !== "RHYTHM-DEFAULT-SUCCESSOR-CONSTRUCTION") {
    errors.push("default successor-construction rhythm is not the residual-schedule fallback");
  }
  if (questionPrograms.composition?.id !== "QP-PREFORMAL-RESIDUAL-RATCHET") {
    errors.push("active composition is not the preformal residual ratchet");
  }
  const order = questionPrograms.composition?.order ?? [];
  for (const stage of [
    "residual_selected_principal_rhythm", "compiled_coding_questions",
    "actual_discharge_and_return", "required_reciprocal_challenges",
    "release_subtract", "fold_reopen_regenerate", "answer_dependent_next_residual",
  ]) {
    if (!order.includes(stage)) errors.push(`composed ratchet is missing stage ${stage}`);
  }
  if (!/represented or individually blocked/u.test(
    questionPrograms.composition?.continuation_rule ?? "",
  )) {
    errors.push("composed ratchet must preserve per-challenge represented/blocked closure");
  }
}

const residualIndexCheck = spawnSync(
  process.execPath,
  [rel(".claude/hooks/ic-residual-topology.js"), "validate", root],
  { cwd: root, encoding: "utf8", windowsHide: true },
);
if (residualIndexCheck.status !== 0) {
  errors.push(`rebuildable residual index failed: ${residualIndexCheck.stderr.trim()}`);
}
const harnessAcceptanceCheck = spawnSync(
  process.execPath,
  [rel("tools/harness_acceptance_check.js")],
  { cwd: root, encoding: "utf8", windowsHide: true },
);
if (harnessAcceptanceCheck.status !== 0) {
  errors.push(`successor harness acceptance checks failed: ${harnessAcceptanceCheck.stderr.trim()}`);
}
const explorationAlgorithmCheck = spawnSync(
  process.execPath,
  [rel("tools/exploration_algorithm_check.js")],
  { cwd: root, encoding: "utf8", windowsHide: true },
);
if (explorationAlgorithmCheck.status !== 0) {
  errors.push(`question-bank exploration checks failed: ${explorationAlgorithmCheck.stderr.trim()}`);
}

const toolchain = read("formal/lean-toolchain").trim();
if (toolchain !== "leanprover/lean4:v4.33.1") errors.push(`unexpected Lean toolchain: ${toolchain}`);
requireContains("formal/lakefile.toml", [
  'rev = "0df444a360eaa60ab8c11dca51a86af692955474"',
  'rev = "3bdedf29bada13d8103e6c979001c51dcee210c8"',
  'warningAsError = true',
]);
const manifest = read("formal/lake-manifest.json");
for (const revision of [
  '"rev": "0df444a360eaa60ab8c11dca51a86af692955474"',
  '"rev": "3bdedf29bada13d8103e6c979001c51dcee210c8"',
]) {
  if (!manifest.includes(revision)) errors.push(`formal/lake-manifest.json: missing locked ${revision}`);
}
if (/rev\s*=\s*"(?:main|master|nightly)"/iu.test(read("formal/lakefile.toml"))) {
  errors.push("formal dependencies must not track a moving revision");
}

for (const ledger of ["formal-successor/DECISIONS.jsonl", "formal-successor/FAILURES.jsonl"]) {
  const ids = new Set();
  for (const [index, line] of read(ledger).split(/\r?\n/u).entries()) {
    if (!line.trim()) continue;
    try {
      const record = JSON.parse(line);
      assert.equal(typeof record.id, "string");
      if (ids.has(record.id)) errors.push(`${ledger}:${index + 1}: duplicate id ${record.id}`);
      ids.add(record.id);
    } catch (error) {
      errors.push(`${ledger}:${index + 1}: invalid JSONL (${error.message})`);
    }
  }
}

const conformance = read("formal-successor/CONFORMANCE_STATUS.md");
const gateFPassed = /^\| FORMAL-GATE-F \| PASS \|/mu.test(conformance);
if (!gateFPassed) {
  const protectedPaths = [
    "Inquiry_Calculus_v2_0.tex",
    "Inquiry_Calculus_v2_0_Comprehensive_Implementation_Plan.md",
    "Cargo.toml", "Cargo.lock", "rust-toolchain.toml",
    "crates", "fixtures", "migrations",
    "CONFORMANCE_STATUS.md", "DECISIONS.jsonl", "FAILURES.jsonl",
  ];
  const result = spawnSync(
    "git",
    ["diff", "--name-only", baseline, "--", ...protectedPaths],
    { cwd: root, encoding: "utf8", windowsHide: true },
  );
  if (result.status !== 0) {
    errors.push(`could not compare frozen predecessor to ${baseline}: ${result.stderr.trim()}`);
  } else if (result.stdout.trim()) {
    errors.push(`pre-Gate-F predecessor surfaces changed:\n${result.stdout.trim()}`);
  }
}

function leanSources(directory) {
  const found = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    if (entry.name === ".lake") continue;
    const candidate = path.join(directory, entry.name);
    if (entry.isDirectory()) found.push(...leanSources(candidate));
    else if (entry.isFile() && entry.name.endsWith(".lean")) found.push(candidate);
  }
  return found;
}
for (const absolute of leanSources(rel("formal"))) {
  const name = path.relative(root, absolute).split(path.sep).join("/");
  const source = read(name);
  if (/\bsorry\b/u.test(source)) errors.push(`${name}: contains sorry`);
  if (/^\s*axiom\s+/mu.test(source)) errors.push(`${name}: contains a custom axiom declaration`);
}

requireContains(".github/workflows/ci.yml", [
  "codex/formal-successor",
  "tools/successor_control_check.js",
  "node --check .claude/hooks/ic-question-program.js",
  "node tools/predecessor_inventory.js check",
  "node tools/predecessor_inventory_check.js",
  "node tools/predecessor_tex_classification.js check",
  "node tools/predecessor_tex_classification_check.js",
  "node tools/predecessor_implementation_classification.js check",
  "node tools/predecessor_implementation_classification_check.js",
  "node tools/predecessor_fixture_classification.js check",
  "node tools/predecessor_fixture_classification_check.js",
  "node tools/phase_a_coverage.js check",
  "node tools/phase_a_coverage_check.js",
  "node tools/phase_b_predecessor_spine.js check",
  "node tools/phase_b_predecessor_spine_check.js",
  "node tools/phase_b_predecessor_spine_check.js --compile",
  "node tools/phase_b_binding_type.js check",
  "node tools/phase_b_binding_type_check.js",
  "node tools/phase_b_forms.js check",
  "node tools/phase_b_forms_check.js",
  "node tools/phase_b_relations.js check",
  "node tools/phase_b_relations_check.js",
  "node tools/phase_b_refinement.js check",
  "node tools/phase_b_refinement_check.js",
  "node tools/phase_b_formula_grammar.js check",
  "node tools/phase_b_formula_grammar_check.js",
  "node tools/phase_b_minimal_logical_basis.js check",
  "node tools/phase_b_minimal_logical_basis_check.js",
  "node tools/phase_b_relation_expression_ir.js check",
  "node tools/phase_b_relation_expression_ir_check.js",
  "node tools/phase_b_relation_schema_ports.js check",
  "node tools/phase_b_relation_schema_ports_check.js",
  "node tools/phase_b_partial_binding_fiber.js check",
  "node tools/phase_b_partial_binding_fiber_check.js",
  "node tools/phase_b_canonical_question_syntax.js check",
  "node tools/phase_b_canonical_question_syntax_check.js",
  "node tools/phase_b_answer_carrier_validity.js check",
  "node tools/phase_b_answer_carrier_validity_check.js",
  "node tools/phase_b_proposition_not_warrant.js check",
  "node tools/phase_b_proposition_not_warrant_check.js",
  "node tools/phase_b_many_questions_generation.js check",
  "node tools/phase_b_many_questions_generation_check.js",
  "node tools/phase_b_discharge_mode_syntax.js check",
  "node tools/phase_b_discharge_mode_syntax_check.js",
  "node tools/phase_b_question_composition_syntax.js check",
  "node tools/phase_b_question_composition_syntax_check.js",
  "node tools/phase_b_question_conditioned_discrimination.js check",
  "node tools/phase_b_question_conditioned_discrimination_check.js",
  "node tools/phase_b_question_refinement_preorder.js check",
  "node tools/phase_b_question_refinement_preorder_check.js",
  "node tools/phase_b_question_refinement_semantics.js check",
  "node tools/phase_b_question_refinement_semantics_check.js",
  "node tools/phase_b_question_joint_active_refinement.js check",
  "node tools/phase_b_question_joint_active_refinement_check.js",
  "node tools/phase_b_question_redundancy.js check",
  "node tools/phase_b_question_redundancy_check.js",
  "node tools/phase_b_precision_not_improvement.js check",
  "node tools/phase_b_precision_not_improvement_check.js",
  "node tools/phase_b_relational_sections.js check",
  "node tools/phase_b_relational_sections_check.js",
  "node tools/phase_b_solution_fibers.js check",
  "node tools/phase_b_solution_fibers_check.js",
  "node tools/phase_b_question_structured_hole.js check",
  "node tools/phase_b_question_structured_hole_check.js",
  "node tools/phase_b_relational_abstraction.js check",
  "node tools/phase_b_relational_abstraction_check.js",
  "node tools/phase_b_abstraction_by_removal.js check",
  "node tools/phase_b_abstraction_by_removal_check.js",
  "node tools/phase_b_solution_field_web.js check",
  "node tools/phase_b_solution_field_web_check.js",
  "node tools/phase_b_indexed_meet_refinement.js check",
  "node tools/phase_b_indexed_meet_refinement_check.js",
  "node tools/phase_b_property_image_hole.js check",
  "node tools/phase_b_property_image_hole_check.js",
  "node tools/phase_b_protected_determination.js check",
  "node tools/phase_b_protected_determination_check.js",
  "node tools/phase_b_exact_determination_signature.js check",
  "node tools/phase_b_exact_determination_signature_check.js",
  "node tools/phase_b_residual_ambiguity.js check",
  "node tools/phase_b_residual_ambiguity_check.js",
  "node tools/phase_b_representation_defect.js check",
  "node tools/phase_b_representation_defect_check.js",
  "node tools/phase_b_separating_context_question.js check",
  "node tools/phase_b_separating_context_question_check.js",
  "node tools/phase_b_representation_question.js check",
  "node tools/phase_b_representation_question_check.js",
  "node tools/phase_b_grain_question.js check",
  "node tools/phase_b_grain_question_check.js",
  "node tools/phase_b_probe_tool_invention_question.js check",
  "node tools/phase_b_probe_tool_invention_question_check.js",
  "node tools/phase_b_representation_gap_localization.js check",
  "node tools/phase_b_representation_gap_localization_check.js",
  "node tools/phase_b_typed_distinction_schema.js check",
  "node tools/phase_b_typed_distinction_schema_check.js",
  "node tools/phase_b_candidate_boundary_incidence.js check",
  "node tools/phase_b_candidate_boundary_incidence_check.js",
  "node tools/phase_b_boundary_point_profile.js check",
  "node tools/phase_b_boundary_point_profile_check.js",
  "node tools/phase_b_boundary_point_regeneration.js check",
  "node tools/phase_b_boundary_point_regeneration_check.js",
  "node tools/phase_b_determination_presentation.js check",
  "node tools/phase_b_determination_presentation_check.js",
  "node tools/phase_b_positive_departure_witness.js check",
  "node tools/phase_b_positive_departure_witness_check.js",
  "node tools/phase_b_departure_relative_positivity.js check",
  "node tools/phase_b_departure_relative_positivity_check.js",
  "node tools/phase_b_derived_boundary_crossing.js check",
  "node tools/phase_b_derived_boundary_crossing_check.js",
  "node tools/phase_b_relation_and_negation_use.js check",
  "node tools/phase_b_relation_and_negation_use_check.js",
  "node tools/phase_b_positive_negation_filling.js check",
  "node tools/phase_b_positive_negation_filling_check.js",
  "node tools/phase_b_semantic_and_execution_coverage.js check",
  "node tools/phase_b_semantic_and_execution_coverage_check.js",
  "lake-package-directory: formal",
  'LEAN_NUM_THREADS: "1"',
  "leanchecker: true",
  "leanchecker-args: InquiryCalculus Spec",
  "axiom-audit: true",
  "axiom-audit root is a declaration namespace, not a Lake module root",
  "axiom-audit-root: InquiryCalculus",
]);
requireExcludes(".github/workflows/ci.yml", ["nanoda: true"]);

if (errors.length > 0) {
  process.stderr.write(`formal successor control check failed:\n- ${errors.join("\n- ")}\n`);
  process.exit(1);
}

process.stdout.write("formal successor control checks passed\n");

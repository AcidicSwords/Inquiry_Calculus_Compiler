//! Canonical identity and structural-checking infrastructure for Inquiry Calculus.
//!
//! Every persisted semantic artifact uses the versioned canonical envelope to assign a stable
//! content identity. Individual modules state their phase and semantic limits explicitly.

mod adjunction;
mod artifact;
mod backend;
mod boundary;
mod bridge;
mod claim;
mod compression;
mod context;
mod cue;
mod decoder;
mod departure;
mod determination;
mod event;
mod exterior;
mod factorization;
mod fiber;
mod finite_cell;
mod formula;
mod frontier;
mod iprog;
mod live_question_frontier;
mod local_interrogative_fixed_point;
mod method;
mod negation;
mod negation_query;
mod occurrence;
mod probe;
mod probe_contract;
mod query;
mod question_readiness;
mod question_succession;
mod raw_return;
mod recovery;
mod relation;
mod relation_expr;
mod relation_use;
mod resolution;
mod seed;
mod separator;
mod sixfold;
mod standing;
mod sufficient_present;
mod supported_answer;
mod ty;

pub use adjunction::{
    AdmittedFiniteAdjunction, ExactFinitePreorder, FiniteAdjunctionCandidate,
    FiniteAdjunctionCatalog, FiniteAdjunctionError, FiniteAdjunctionLawCounterexample,
    admit_finite_adjunction,
};
pub use artifact::{
    ARTIFACT_DOMAIN, ARTIFACT_WIRE_VERSION, ArtifactEnvelope, ArtifactError, ArtifactKind,
    ArtifactRef,
};
pub use backend::{
    BACKEND_REQUEST_ARTIFACT_KIND, BACKEND_REQUEST_SCHEMA_VERSION, BackendBoundaryCatalog,
    BackendBoundaryCheckError, BackendBoundaryError, BackendRequest, BackendRequestRef,
    SURFACE_PLAN_ARTIFACT_KIND, SURFACE_PLAN_SCHEMA_VERSION, SurfacePlan, SurfacePlanRef,
};
pub use boundary::{
    BOUNDARY_CHART_ARTIFACT_KIND, BOUNDARY_CHART_SCHEMA_VERSION, BoundaryChart, BoundaryChartError,
};
pub use bridge::{BindingBridgeCheckError, BindingBridgeError, BindingBridgeIR, BindingChangeKind};
pub use claim::{
    CLAIM_ARTIFACT_KIND, CLAIM_SCHEMA_VERSION, ClaimArtifact, ClaimCatalog, ClaimCheckError,
    ClaimError, ClaimStatus,
};
pub use compression::{
    COMPRESSION_LICENSE_ARTIFACT_KIND, COMPRESSION_LICENSE_SCHEMA_VERSION, CompressionKind,
    CompressionLicense, CompressionLicenseError, CompressionLicenseRef, DistortionContractRef,
    FoldOrQuotientRef, ProtectedContinuationRef, RecoveryContractRef, UnlockConditionRef,
};
pub use context::{
    ApplicabilityRef, DischargeMode, GrainRef, HorizonRef, ScopeRef, SupportRef, WarrantRef,
};
pub use cue::{
    AdmittedExactFiniteCue, EXACT_FINITE_CUE_ARTIFACT_KIND, EXACT_FINITE_CUE_SCHEMA_VERSION,
    ExactFiniteCue, ExactFiniteCueAdmission, ExactFiniteCueBasisCandidate,
    ExactFiniteCueBasisError, ExactFiniteCueBasisResult, ExactFiniteCueCatalog,
    ExactFiniteCueCheckError, ExactFiniteCueError, ExactFiniteCueFrontier,
    ExactFiniteCueFrontierError, ExactFiniteCueUnknown, FiniteCueSeparator, FiniteResourcePreorder,
    InsufficientExactFiniteCueBasis, admit_exact_finite_cue, check_admitted_exact_finite_cue_basis,
    check_exact_finite_cue_basis, select_nondominated_admitted_exact_finite_cue_bases,
    select_nondominated_exact_finite_cue_bases,
};
pub use decoder::{
    ActualDecodeCatalog, ActualDecodeError, ActualDecodeResult, DecodedCandidateSet,
    DecodedObservationError, DecodedObservationUse, FINITE_DECODER_ARTIFACT_KIND,
    FINITE_DECODER_SCHEMA_VERSION, FiniteDecoder, FiniteDecoderCatalog, FiniteDecoderCheckError,
    FiniteDecoderEntry, FiniteDecoderError, FiniteDecoderOutcome, FiniteDecoderRef,
    ObservationResultCatalog, decode_actual_event, match_decoded_observation_use,
};
pub use departure::{
    AdmittedFiniteDeparture, DEPARTURE_WITNESS_ARTIFACT_KIND, DEPARTURE_WITNESS_SCHEMA_VERSION,
    DepartureCatalog, DepartureEvidenceSupportCatalog, DepartureEvidenceSupportError,
    DepartureStandingCatalog, DepartureStandingCheckError, DepartureWitness,
    DepartureWitnessCheckError, DepartureWitnessError, DepartureWitnessRef,
    FiniteDepartureAdmissionCatalog, FiniteDepartureAdmissionError, FiniteDepartureEvidence,
    ResolvedDepartureEvidenceSupport, admit_probed_finite_departure,
    check_departure_witness_standing_support, resolve_departure_witness_evidence_support,
};
pub use determination::{
    DETERMINATION_PRESENTATION_ARTIFACT_KIND, DETERMINATION_PRESENTATION_SCHEMA_VERSION,
    DeterminationCatalog, DeterminationPresentation, DeterminationPresentationCheckError,
    DeterminationPresentationError, DeterminationPresentationRef, DistinctionRef, Orientation,
    RelationalWebRef,
};
pub use event::{
    ACTUAL_EVENT_ARTIFACT_KIND, ACTUAL_EVENT_SCHEMA_VERSION, ActualEvent, ActualEventCatalog,
    ActualEventCheckError, ActualEventError, BoundaryRef, EventRef, OperatorRef, ProvenanceRef,
    RawReturnCatalog, RouteRef, StateRef, check_actual_event, check_event_context,
    check_raw_return,
};
pub use exterior::{TaggedExteriorCatalog, TaggedExteriorClaim, TaggedExteriorClaimError};
pub use factorization::{
    ExactDeterminationError, ExactDeterminationResult, ExactFactorization,
    ExactFamilyDeterminationResult, ExactFamilyFactorization, ExactFamilySignature,
    ExactFiniteSignature, FamilyKernelSeparator, KernelSeparator, SignatureContext,
    determine_through_exact, determine_through_exact_family,
};
pub use fiber::{
    AdmittedFiniteNegationExtension, DeclaredIncidenceError, ExactNegationField, ExactReturnFiber,
    FiberRecoveryError, FiniteNegationAdmissionError, FiniteNegationExtension, ReturnClosure,
    ReturnFiberError, SelectedReturn, TypedFiniteNegationExtension, TypedNegationExtensionError,
    admit_finite_negation_extension, check_declared_incidence, check_fiber_recovery,
    check_return_closure, exact_return_fiber,
};
pub use finite_cell::{
    FiniteCellComparison, FiniteCellError, FiniteCellSeparator, FiniteIncompatibilityError,
    FiniteIncompatibilityResult, FiniteIncompatibilityTable, FiniteIncompatibilityWitness,
    FiniteObservation, FiniteTypedIncompatibilityUseCatalog, FiniteTypedObservationCatalog,
    TypedFiniteIncompatibilityError, TypedFiniteIncompatibilityResult,
    TypedFiniteIncompatibilityRoleError, TypedFiniteIncompatibilityRoles,
    TypedFiniteIncompatibilityTable, TypedFiniteIncompatibilityUseError,
    TypedFiniteIncompatibilityUseResult, TypedFiniteIncompatibilityUseWitness,
    TypedFiniteIncompatibilityWitness, TypedFiniteObservation,
    TypedFiniteOrientedIncompatibilityUseError, TypedFiniteOrientedIncompatibilityUseResult,
    TypedFiniteOrientedIncompatibilityUseWitness, check_finite_incompatibility,
    check_typed_finite_incompatibility, check_typed_finite_incompatibility_use,
    check_typed_finite_oriented_incompatibility_use, compare_finite_observation_cells,
};
pub use formula::{
    FORMULA_ARTIFACT_KIND, FORMULA_SCHEMA_VERSION, FormulaArtifact, FormulaCatalog,
    FormulaCheckError, FormulaError, FormulaIR, FormulaRef, RelationRef, RelationSignature, TermIR,
};
pub use frontier::{
    ActiveNegationUse, CollectiveCoverageRef, GeneratorCoverageRef, NegationFrontierError,
    NegationFrontierView,
};
pub use iprog::{
    IPROG_ARTIFACT_KIND, IPROG_SCHEMA_VERSION, IProgArtifact, IProgCatalog, IProgCheckError,
    IProgError, IProgIR, IProgRef, ProgramBinding,
};
pub use live_question_frontier::{
    FiniteLiveQuestionFrontier, FiniteLiveQuestionFrontierError, LiveQuestionCandidate,
    LiveQuestionCandidateError, LiveQuestionOrigin, ProtectedQuestionBranch, RequiredDischargeKind,
    RequiredQuestionDischarge, derive_finite_live_question_frontier,
};
pub use local_interrogative_fixed_point::{
    FiniteLocalEffectivityCoverage, FiniteLocalEffectivityCoverageError, LocalEffectivityEdge,
    LocalInterrogativeContext, LocalInterrogativeFixedPoint, LocalInterrogativeFixedPointError,
    LocalInterrogativeResidual, LocalQuestionAssessment, LocalQuestionClassification,
    LocalQuestionClosingReason, LocalQuestionExit, OpenRequiredQuestion,
    derive_finite_local_interrogative_fixed_point,
};
pub use method::{
    BackendRef, CheckerRef, CostModelRef, CoverageRef, ExtensionDomainRef,
    METHOD_BRIDGE_ARTIFACT_KIND, METHOD_BRIDGE_SCHEMA_VERSION, METHOD_CONTRACT_ARTIFACT_KIND,
    METHOD_CONTRACT_SCHEMA_VERSION, MethodBridge, MethodBridgeCatalog, MethodBridgeCheckError,
    MethodBridgeError, MethodBridgeRef, MethodContract, MethodContractCheckError,
    MethodContractError, MethodRef, ResidualSchemaRef,
};
pub use negation::{
    NEGATION_USE_ARTIFACT_KIND, NEGATION_USE_SCHEMA_VERSION, NegationCatalog, NegationCoverage,
    NegationUse, NegationUseCheckError, NegationUseError, NegationUseRef,
};
pub use negation_query::{
    PositiveNegationQuery, PositiveNegationQueryError, positive_negation_query,
};
pub use occurrence::{
    OPERATOR_OCCURRENCE_ARTIFACT_KIND, OPERATOR_OCCURRENCE_SCHEMA_VERSION, OperatorOccurrence,
    OperatorOccurrenceCatalog, OperatorOccurrenceCheckError, OperatorOccurrenceError,
    OperatorOccurrenceRef,
};
pub use probe::{
    PROBE_OPERATOR_ARTIFACT_KIND, PROBE_OPERATOR_SCHEMA_VERSION, ProbeOperator, ProbeOperatorError,
    ProbeOperatorRef,
};
pub use probe_contract::{
    PROBE_CONTRACT_ARTIFACT_KIND, PROBE_CONTRACT_SCHEMA_VERSION, ProbeContract, ProbeContractError,
    ProbeContractRef,
};
pub use query::{
    COMPLETION_CANDIDATE_ARTIFACT_KIND, COMPLETION_CANDIDATE_SCHEMA_VERSION, CompletionCandidate,
    CompletionCandidateCatalog, CompletionCandidateCheckError, CompletionCandidateError,
    CompletionCandidateRef, CompletionFiberView, CompletionFiberViewError,
    OPEN_QUERY_ARTIFACT_KIND, OPEN_QUERY_SCHEMA_VERSION, OpenPort, OpenQuery, OpenQueryCatalog,
    OpenQueryCheckError, OpenQueryError, OpenQueryFiberError, OpenQueryPlugError,
    OpenQueryTransformError, QueryRef,
};
pub use question_readiness::{
    QuestionReadiness, QuestionReadinessCatalog, QuestionReadinessError,
    QuestionReadinessRequirement, await_question_readiness, derive_question_readiness,
};
pub use question_succession::{
    ASK_OCCURRENCE_ARTIFACT_KIND, ASK_OCCURRENCE_SCHEMA_VERSION, AskOccurrence,
    AskOccurrenceCheckError, AskOccurrenceError, AskOccurrenceRef, PROGRAM_POSITION_ARTIFACT_KIND,
    PROGRAM_POSITION_SCHEMA_VERSION, ProgramPosition, ProgramPositionError, ProgramPositionRef,
    QuestionSuccessionCatalog, QuestionSuccessor, QuestionSuccessorError,
    SOURCE_CONFIG_ARTIFACT_KIND, SOURCE_CONFIG_SCHEMA_VERSION, SourceConfig,
    SourceConfigCheckError, SourceConfigError, SourceConfigRef, derive_question_successor,
};
pub use raw_return::{
    RAW_RETURN_ARTIFACT_KIND, RAW_RETURN_SCHEMA_VERSION, RawReturn, RawReturnError, RawReturnRef,
};
pub use recovery::{
    ExactFiberRecovery, ExactFiberRecoveryError, RecoverySeparator, RecoveryStatusIR,
    check_exact_fiber_recovery,
};
pub use relation::{
    RELATION_SCHEMA_ARTIFACT_KIND, RELATION_SCHEMA_VERSION, RelationBodyIR, RelationCheckError,
    RelationError, RelationPort, RelationSchema,
};
pub use relation_expr::{
    PortRename, RELATION_EXPR_ARTIFACT_KIND, RELATION_EXPR_SCHEMA_VERSION, RelationExprArtifact,
    RelationExprError, RelationExprIR, RelationExprRef,
};
pub use relation_use::{
    PortBinding, RELATION_USE_ARTIFACT_KIND, RELATION_USE_SCHEMA_VERSION, RelationCatalog,
    RelationUse, RelationUseCheckError, RelationUseContext, RelationUseError, RelationUseRef,
};
pub use resolution::{
    DecoderRef, RESOLUTION_PATH_ARTIFACT_KIND, RESOLUTION_PATH_SCHEMA_VERSION, ResolutionCatalog,
    ResolutionPath, ResolutionPathCheckError, ResolutionPathError, ResolutionPathIR,
    ResolutionPathRef,
};
pub use seed::{SeedReorientation, SeedReorientationError};
pub use separator::{
    BoundGeneratedInquiryContinuation, DeclaredFiniteGeneratorRegime,
    DeclaredFiniteGeneratorRegimeError, DeclaredRouteMaterialization, EffectivityRef,
    ExactFiniteRegimeRoute, ExactFiniteRegimeSeparatorError, ExactFiniteRegimeSeparatorResult,
    GENERATED_INQUIRY_ARTIFACT_KIND, GENERATED_INQUIRY_SCHEMA_VERSION, GeneratedInquiry,
    GeneratedInquiryBindingError, GeneratedInquiryCatalog, GeneratedInquiryCheckError,
    GeneratedInquiryError, GeneratorRegimeRef, MaterializationGap, MaterializationGapError,
    ProposedRegimeExtension, ProposedRegimeExtensionError, ProtectedClassRef,
    ProtectedCompletionFieldRef, SEPARATOR_PROBLEM_ARTIFACT_KIND, SEPARATOR_PROBLEM_SCHEMA_VERSION,
    SeparatorProblem, SeparatorProblemError, SeparatorProblemRef, StructureViewRef,
    bind_generated_inquiry_continuation, check_exact_no_separator_within_declared_regime,
};
pub use sixfold::{GammaError, ReciprocalOccurrence, ReciprocalOccurrenceError, RoleComparison};
pub use standing::{
    ClaimRef, DeclaredStandingError, DeclaredSupportClosure, DeterminationSupportCatalog,
    DeterminationSupportError, RelationUseSupportCatalog, RelationUseSupportError,
    ResolvedDeterminationSupport, ResolvedRelationUseSupport, SUPPORT_ENVIRONMENT_ARTIFACT_KIND,
    SUPPORT_ENVIRONMENT_SCHEMA_VERSION, Standing, StandingProblem, SupportEnvironment,
    SupportEnvironmentArtifact, SupportEnvironmentArtifactCheckError,
    SupportEnvironmentArtifactError, SupportEnvironmentCatalog, SupportEnvironmentRef,
    SupportSubjectRef, resolve_determination_presentation_support, resolve_relation_use_support,
    standing, standing_determination_presentation_support, standing_from_declared_subject_support,
    standing_from_declared_support, standing_relation_use_support,
};
pub use sufficient_present::{
    ExactFinitePresentChallenge, ExactFinitePresentReopenError, ExactFinitePresentReopenWitness,
    ExactFinitePresentUpdate, ExactFinitePresentUpdateError, ExactFiniteSufficientPresent,
    ExactFiniteSufficientPresentError, ExactFiniteSufficientPresentResult,
    ExactProtectedContinuation, FINITE_PRESENT_REOPEN_ARTIFACT_KIND,
    FINITE_PRESENT_REOPEN_SCHEMA_VERSION, challenge_exact_finite_sufficient_present,
    derive_exact_finite_sufficient_present, extend_exact_finite_sufficient_present,
};
pub use supported_answer::{
    AdmittedFiniteAnswerSet, BoundFiniteAskContinuation, FiniteAnswerBindingError,
    FiniteSupportedAnswerCatalog, FiniteSupportedAnswerError, admit_finite_supported_answers,
    bind_finite_ask_continuation,
};
pub use ty::{
    BindingVersionRef, TYPE_ARTIFACT_KIND, TYPE_SCHEMA_VERSION, TYPED_FORM_ARTIFACT_KIND,
    TYPED_FORM_SCHEMA_VERSION, TyIR, TypeArtifact, TypeCatalog, TypeCheckError, TypeError,
    TypeFamilyRef, TypeRef, TypeSymbol, TypedForm, TypedFormRef,
};

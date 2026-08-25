//! Canonical identity and structural-checking infrastructure for Inquiry Calculus.
//!
//! Every persisted semantic artifact uses the versioned canonical envelope to assign a stable
//! content identity. Individual modules state their phase and semantic limits explicitly.

mod artifact;
mod boundary;
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
mod negation;
mod negation_query;
mod probe;
mod probe_contract;
mod query;
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
mod ty;

pub use artifact::{
    ARTIFACT_DOMAIN, ARTIFACT_WIRE_VERSION, ArtifactEnvelope, ArtifactError, ArtifactKind,
    ArtifactRef,
};
pub use boundary::{
    BOUNDARY_CHART_ARTIFACT_KIND, BOUNDARY_CHART_SCHEMA_VERSION, BoundaryChart, BoundaryChartError,
};
pub use context::{
    ApplicabilityRef, DischargeMode, GrainRef, HorizonRef, ScopeRef, SupportRef, WarrantRef,
};
pub use cue::{
    ExactFiniteCueBasisCandidate, ExactFiniteCueBasisError, ExactFiniteCueBasisResult,
    ExactFiniteCueFrontier, ExactFiniteCueFrontierError, FiniteCueSeparator,
    FiniteResourcePreorder, InsufficientExactFiniteCueBasis, check_exact_finite_cue_basis,
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
    DEPARTURE_WITNESS_ARTIFACT_KIND, DEPARTURE_WITNESS_SCHEMA_VERSION, DepartureCatalog,
    DepartureWitness, DepartureWitnessCheckError, DepartureWitnessError, DepartureWitnessRef,
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
    DeclaredIncidenceError, ExactNegationField, ExactReturnFiber, FiberRecoveryError,
    FiniteNegationExtension, ReturnClosure, ReturnFiberError, SelectedReturn,
    TypedFiniteNegationExtension, TypedNegationExtensionError, check_declared_incidence,
    check_fiber_recovery, check_return_closure, exact_return_fiber,
};
pub use finite_cell::{
    FiniteCellComparison, FiniteCellError, FiniteCellSeparator, FiniteIncompatibilityError,
    FiniteIncompatibilityResult, FiniteIncompatibilityTable, FiniteIncompatibilityWitness,
    FiniteObservation, FiniteTypedObservationCatalog, TypedFiniteIncompatibilityError,
    TypedFiniteIncompatibilityResult, TypedFiniteIncompatibilityTable,
    TypedFiniteIncompatibilityWitness, TypedFiniteObservation, check_finite_incompatibility,
    check_typed_finite_incompatibility, compare_finite_observation_cells,
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
pub use negation::{
    NEGATION_USE_ARTIFACT_KIND, NEGATION_USE_SCHEMA_VERSION, NegationCatalog, NegationCoverage,
    NegationUse, NegationUseCheckError, NegationUseError, NegationUseRef,
};
pub use negation_query::{
    PositiveNegationQuery, PositiveNegationQueryError, positive_negation_query,
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
    EffectivityRef, GeneratorRegimeRef, ProtectedClassRef, ProtectedCompletionFieldRef,
    SEPARATOR_PROBLEM_ARTIFACT_KIND, SEPARATOR_PROBLEM_SCHEMA_VERSION, SeparatorProblem,
    SeparatorProblemError, SeparatorProblemRef, StructureViewRef,
};
pub use sixfold::{GammaError, ReciprocalOccurrence, ReciprocalOccurrenceError, RoleComparison};
pub use standing::{ClaimRef, Standing, StandingProblem, SupportEnvironment, standing};
pub use ty::{
    BindingVersionRef, TYPE_ARTIFACT_KIND, TYPE_SCHEMA_VERSION, TYPED_FORM_ARTIFACT_KIND,
    TYPED_FORM_SCHEMA_VERSION, TyIR, TypeArtifact, TypeCatalog, TypeCheckError, TypeError,
    TypeFamilyRef, TypeRef, TypeSymbol, TypedForm, TypedFormRef,
};

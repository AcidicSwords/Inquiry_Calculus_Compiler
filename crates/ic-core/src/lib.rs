//! Phase 0 identity infrastructure for Inquiry Calculus.
//!
//! This crate deliberately does not define Phase 1 semantic types. It only defines
//! the versioned canonical envelope used to assign stable content identities.

mod artifact;
mod context;
mod departure;
mod determination;
mod formula;
mod iprog;
mod query;
mod relation;
mod relation_expr;
mod relation_use;
mod ty;

pub use artifact::{
    ARTIFACT_DOMAIN, ARTIFACT_WIRE_VERSION, ArtifactEnvelope, ArtifactError, ArtifactKind,
    ArtifactRef,
};
pub use context::{
    ApplicabilityRef, DischargeMode, GrainRef, HorizonRef, ScopeRef, SupportRef, WarrantRef,
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
pub use formula::{
    FORMULA_ARTIFACT_KIND, FORMULA_SCHEMA_VERSION, FormulaArtifact, FormulaCatalog,
    FormulaCheckError, FormulaError, FormulaIR, FormulaRef, RelationRef, RelationSignature, TermIR,
};
pub use iprog::{
    IPROG_ARTIFACT_KIND, IPROG_SCHEMA_VERSION, IProgArtifact, IProgCatalog, IProgCheckError,
    IProgError, IProgIR, IProgRef, ProgramBinding,
};
pub use query::{
    CompletionCandidate, CompletionFiberView, CompletionFiberViewError, OPEN_QUERY_ARTIFACT_KIND,
    OPEN_QUERY_SCHEMA_VERSION, OpenPort, OpenQuery, OpenQueryCatalog, OpenQueryCheckError,
    OpenQueryError, OpenQueryFiberError, OpenQueryPlugError, OpenQueryTransformError, QueryRef,
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
pub use ty::{
    BindingVersionRef, TYPE_ARTIFACT_KIND, TYPE_SCHEMA_VERSION, TYPED_FORM_ARTIFACT_KIND,
    TYPED_FORM_SCHEMA_VERSION, TyIR, TypeArtifact, TypeCatalog, TypeCheckError, TypeError,
    TypeFamilyRef, TypeRef, TypeSymbol, TypedForm, TypedFormRef,
};

//! Phase 0 identity infrastructure for Inquiry Calculus.
//!
//! This crate deliberately does not define Phase 1 semantic types. It only defines
//! the versioned canonical envelope used to assign stable content identities.

mod artifact;
mod formula;
mod relation;
mod ty;

pub use artifact::{
    ARTIFACT_DOMAIN, ARTIFACT_WIRE_VERSION, ArtifactEnvelope, ArtifactError, ArtifactKind,
    ArtifactRef,
};
pub use formula::{
    FORMULA_ARTIFACT_KIND, FORMULA_SCHEMA_VERSION, FormulaArtifact, FormulaCatalog,
    FormulaCheckError, FormulaError, FormulaIR, FormulaRef, RelationRef, RelationSignature, TermIR,
};
pub use relation::{
    RELATION_SCHEMA_ARTIFACT_KIND, RELATION_SCHEMA_VERSION, RelationBodyIR, RelationCheckError,
    RelationError, RelationPort, RelationSchema,
};
pub use ty::{
    BindingVersionRef, TYPE_ARTIFACT_KIND, TYPE_SCHEMA_VERSION, TYPED_FORM_ARTIFACT_KIND,
    TYPED_FORM_SCHEMA_VERSION, TyIR, TypeArtifact, TypeCatalog, TypeCheckError, TypeError,
    TypeFamilyRef, TypeRef, TypeSymbol, TypedForm, TypedFormRef,
};

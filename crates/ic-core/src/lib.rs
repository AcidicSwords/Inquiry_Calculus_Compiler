//! Phase 0 identity infrastructure for Inquiry Calculus.
//!
//! This crate deliberately does not define Phase 1 semantic types. It only defines
//! the versioned canonical envelope used to assign stable content identities.

mod artifact;
mod ty;

pub use artifact::{
    ARTIFACT_DOMAIN, ARTIFACT_WIRE_VERSION, ArtifactEnvelope, ArtifactError, ArtifactKind,
    ArtifactRef,
};
pub use ty::{
    BindingVersionRef, TYPE_ARTIFACT_KIND, TYPE_SCHEMA_VERSION, TYPED_FORM_ARTIFACT_KIND,
    TYPED_FORM_SCHEMA_VERSION, TyIR, TypeArtifact, TypeCatalog, TypeCheckError, TypeError,
    TypeFamilyRef, TypeRef, TypeSymbol, TypedForm, TypedFormRef,
};

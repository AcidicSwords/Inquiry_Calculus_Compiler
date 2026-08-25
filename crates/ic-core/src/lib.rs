//! Phase 0 identity infrastructure for Inquiry Calculus.
//!
//! This crate deliberately does not define Phase 1 semantic types. It only defines
//! the versioned canonical envelope used to assign stable content identities.

mod artifact;

pub use artifact::{
    ARTIFACT_DOMAIN, ARTIFACT_WIRE_VERSION, ArtifactEnvelope, ArtifactError, ArtifactKind,
    ArtifactRef,
};

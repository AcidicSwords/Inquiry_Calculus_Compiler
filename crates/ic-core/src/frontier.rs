//! Derived tagged negation-frontier views.
//!
//! A frontier retains distinct oriented negation-use identities and their independently supplied
//! execution-coverage references. It is not a combined negation relation, an admitted incidence,
//! a candidate generator, or an authoritative stored object.

use std::{collections::BTreeSet, fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ArtifactError, ArtifactRef, DistinctionRef, NegationCoverage, NegationUseRef, Orientation,
    RelationRef, TypedFormRef,
};

macro_rules! artifact_reference {
    ($name:ident) => {
        /// Opaque identity supplied by the execution/materialization layer.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(ArtifactRef);

        impl $name {
            #[must_use]
            pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
                Self(reference)
            }

            #[must_use]
            pub const fn as_artifact_ref(self) -> ArtifactRef {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl FromStr for $name {
            type Err = ArtifactError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                ArtifactRef::from_str(value).map(Self)
            }
        }
    };
}

artifact_reference!(GeneratorCoverageRef);
artifact_reference!(CollectiveCoverageRef);

/// One tagged active negation-use view for a fixed source determination.
///
/// `semantic_coverage` comes from the declared use; `execution_coverage` is intentionally a
/// distinct type and remains unevaluated until occurrence/materialization contracts exist.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActiveNegationUse {
    use_ref: NegationUseRef,
    source: TypedFormRef,
    candidate_field: RelationRef,
    semantic_coverage: NegationCoverage,
    execution_coverage: GeneratorCoverageRef,
}

impl ActiveNegationUse {
    #[must_use]
    pub const fn new(
        use_ref: NegationUseRef,
        source: TypedFormRef,
        candidate_field: RelationRef,
        semantic_coverage: NegationCoverage,
        execution_coverage: GeneratorCoverageRef,
    ) -> Self {
        Self {
            use_ref,
            source,
            candidate_field,
            semantic_coverage,
            execution_coverage,
        }
    }

    #[must_use]
    pub const fn use_ref(self) -> NegationUseRef {
        self.use_ref
    }

    #[must_use]
    pub const fn source(self) -> TypedFormRef {
        self.source
    }

    #[must_use]
    pub const fn candidate_field(self) -> RelationRef {
        self.candidate_field
    }

    #[must_use]
    pub const fn semantic_coverage(self) -> NegationCoverage {
        self.semantic_coverage
    }

    #[must_use]
    pub const fn execution_coverage(self) -> GeneratorCoverageRef {
        self.execution_coverage
    }
}

/// A tagged dependent family of active negation-use views.
///
/// The member list retains every use identity. Empty and partial frontiers are valid views; they
/// do not establish any closure. A collective-coverage reference is only a separately declared
/// input and is never inferred from the member coverage values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NegationFrontierView {
    source: TypedFormRef,
    distinction: DistinctionRef,
    orientation: Orientation,
    members: Vec<ActiveNegationUse>,
    collective_coverage: Option<CollectiveCoverageRef>,
    regime: ArtifactRef,
}

impl NegationFrontierView {
    pub fn new(
        source: TypedFormRef,
        distinction: DistinctionRef,
        orientation: Orientation,
        members: Vec<ActiveNegationUse>,
        collective_coverage: Option<CollectiveCoverageRef>,
        regime: ArtifactRef,
    ) -> Result<Self, NegationFrontierError> {
        let mut use_refs = BTreeSet::new();
        for member in &members {
            if member.source != source {
                return Err(NegationFrontierError::MemberSourceMismatch {
                    expected: source,
                    actual: member.source,
                });
            }
            if !use_refs.insert(member.use_ref) {
                return Err(NegationFrontierError::DuplicateNegationUse(member.use_ref));
            }
        }
        Ok(Self {
            source,
            distinction,
            orientation,
            members,
            collective_coverage,
            regime,
        })
    }

    #[must_use]
    pub const fn source(&self) -> TypedFormRef {
        self.source
    }

    #[must_use]
    pub const fn distinction(&self) -> DistinctionRef {
        self.distinction
    }

    #[must_use]
    pub const fn orientation(&self) -> Orientation {
        self.orientation
    }

    #[must_use]
    pub fn members(&self) -> &[ActiveNegationUse] {
        &self.members
    }

    #[must_use]
    pub const fn collective_coverage(&self) -> Option<CollectiveCoverageRef> {
        self.collective_coverage
    }

    #[must_use]
    pub const fn regime(&self) -> ArtifactRef {
        self.regime
    }
}

/// Errors from derived tagged-frontier construction.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum NegationFrontierError {
    #[error("frontier member source {actual} differs from declared source {expected}")]
    MemberSourceMismatch {
        expected: TypedFormRef,
        actual: TypedFormRef,
    },
    #[error("frontier repeats negation use {0}")]
    DuplicateNegationUse(NegationUseRef),
}

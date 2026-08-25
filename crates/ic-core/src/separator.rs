//! Canonical generic separator-problem artifacts.
//!
//! A separator problem preserves one unresolved protected completion field and the declared
//! resources through which later phases may try to distinguish it.  It is deliberately not a
//! generator, policy, answer, or representation-gap verdict: those require their own admitted
//! evaluators and evidence routes.

use std::{collections::BTreeSet, fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, BindingVersionRef, GrainRef,
    HorizonRef,
};

/// Canonical artifact kind for generic protected residual/separator problems.
pub const SEPARATOR_PROBLEM_ARTIFACT_KIND: &str = "ic.separator-problem";
/// Payload schema version for generic protected residual/separator problems.
pub const SEPARATOR_PROBLEM_SCHEMA_VERSION: u32 = 1;

macro_rules! artifact_reference {
    ($name:ident) => {
        /// Opaque identity whose semantics belong to the later named phase.
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

artifact_reference!(SeparatorProblemRef);
artifact_reference!(ProtectedCompletionFieldRef);
artifact_reference!(ProtectedClassRef);
artifact_reference!(StructureViewRef);
artifact_reference!(GeneratorRegimeRef);
artifact_reference!(EffectivityRef);

/// A finite, caller-declared generator regime and its currently materialized route identities.
///
/// This is a narrow Phase-14 boundary: route membership is declared, not discovered, and the
/// regime does not choose or execute a route. Its purpose is to retain the distinction between a
/// route that is available in the declared regime and one that has actually been materialized.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclaredFiniteGeneratorRegime {
    regime: GeneratorRegimeRef,
    routes: Vec<ArtifactRef>,
    materialized: BTreeSet<ArtifactRef>,
}

impl DeclaredFiniteGeneratorRegime {
    pub fn new(
        regime: GeneratorRegimeRef,
        mut routes: Vec<ArtifactRef>,
        materialized: Vec<ArtifactRef>,
    ) -> Result<Self, DeclaredFiniteGeneratorRegimeError> {
        routes.sort_unstable();
        if let Some(duplicate) = routes
            .windows(2)
            .find_map(|pair| (pair[0] == pair[1]).then_some(pair[0]))
        {
            return Err(DeclaredFiniteGeneratorRegimeError::DuplicateRoute(
                duplicate,
            ));
        }
        let materialized: BTreeSet<_> = materialized.into_iter().collect();
        if let Some(route) = materialized.iter().find(|route| !routes.contains(route)) {
            return Err(DeclaredFiniteGeneratorRegimeError::MaterializedRouteOutsideRegime(*route));
        }
        Ok(Self {
            regime,
            routes,
            materialized,
        })
    }

    #[must_use]
    pub const fn regime(&self) -> GeneratorRegimeRef {
        self.regime
    }
    #[must_use]
    pub fn routes(&self) -> &[ArtifactRef] {
        &self.routes
    }
    #[must_use]
    pub const fn materialized(&self) -> &BTreeSet<ArtifactRef> {
        &self.materialized
    }

    /// Distinguishes materialized, fresh-within-regime, and unavailable route identities.
    #[must_use]
    pub fn route_status(&self, route: ArtifactRef) -> DeclaredRouteMaterialization {
        if !self.routes.contains(&route) {
            DeclaredRouteMaterialization::OutsideDeclaredRegime
        } else if self.materialized.contains(&route) {
            DeclaredRouteMaterialization::Materialized
        } else {
            DeclaredRouteMaterialization::FreshWithinRegime
        }
    }
}

/// Materialization state relative only to one caller-declared finite regime.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeclaredRouteMaterialization {
    /// The route is one of the declared candidates and has been materialized.
    Materialized,
    /// The route is declared available but not currently materialized.
    FreshWithinRegime,
    /// No statement about expressibility follows from absence from this declared finite set.
    OutsideDeclaredRegime,
}

#[derive(Debug, Error)]
pub enum DeclaredFiniteGeneratorRegimeError {
    #[error("declared generator regime repeats route {0}")]
    DuplicateRoute(ArtifactRef),
    #[error("materialized route {0} is not in the declared generator regime")]
    MaterializedRouteOutsideRegime(ArtifactRef),
}

/// One declared generic residual to be separated by a later admitted inquiry route.
///
/// The fields are exactly the phase-12 problem boundary: a protected completion field, optional
/// target class, indexed binding/grain/horizon, available structure, generator regime, and
/// effectivity horizon.  Construction records none of the semantics of those opaque references;
/// in particular, it neither proves a residual exists nor generates or selects a question.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SeparatorProblem {
    residual: ProtectedCompletionFieldRef,
    target: Option<ProtectedClassRef>,
    grain: GrainRef,
    horizon: HorizonRef,
    binding: BindingVersionRef,
    available_structure: StructureViewRef,
    generator_regime: GeneratorRegimeRef,
    effectivity: EffectivityRef,
}

impl SeparatorProblem {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        residual: ProtectedCompletionFieldRef,
        target: Option<ProtectedClassRef>,
        grain: GrainRef,
        horizon: HorizonRef,
        binding: BindingVersionRef,
        available_structure: StructureViewRef,
        generator_regime: GeneratorRegimeRef,
        effectivity: EffectivityRef,
    ) -> Self {
        Self {
            residual,
            target,
            grain,
            horizon,
            binding,
            available_structure,
            generator_regime,
            effectivity,
        }
    }

    #[must_use]
    pub const fn residual(&self) -> ProtectedCompletionFieldRef {
        self.residual
    }
    #[must_use]
    pub const fn target(&self) -> Option<ProtectedClassRef> {
        self.target
    }
    #[must_use]
    pub const fn grain(&self) -> GrainRef {
        self.grain
    }
    #[must_use]
    pub const fn horizon(&self) -> HorizonRef {
        self.horizon
    }
    #[must_use]
    pub const fn binding(&self) -> BindingVersionRef {
        self.binding
    }
    #[must_use]
    pub const fn available_structure(&self) -> StructureViewRef {
        self.available_structure
    }
    #[must_use]
    pub const fn generator_regime(&self) -> GeneratorRegimeRef {
        self.generator_regime
    }
    #[must_use]
    pub const fn effectivity(&self) -> EffectivityRef {
        self.effectivity
    }

    /// Encodes this problem directly; identity never depends on generic serialization.
    #[must_use]
    pub fn canonical_payload(&self) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(32 * 8 + 1);
        reference(&mut encoded, self.residual.as_artifact_ref());
        optional_reference(
            &mut encoded,
            self.target.map(ProtectedClassRef::as_artifact_ref),
        );
        reference(&mut encoded, self.grain.as_artifact_ref());
        reference(&mut encoded, self.horizon.as_artifact_ref());
        reference(&mut encoded, self.binding.as_artifact_ref());
        reference(&mut encoded, self.available_structure.as_artifact_ref());
        reference(&mut encoded, self.generator_regime.as_artifact_ref());
        reference(&mut encoded, self.effectivity.as_artifact_ref());
        encoded
    }

    /// Decodes a complete canonical separator-problem payload.
    pub fn decode_payload(payload: &[u8]) -> Result<Self, SeparatorProblemError> {
        let mut cursor = Cursor::new(payload);
        let residual = ProtectedCompletionFieldRef::from_artifact_ref(cursor.reference()?);
        let target = cursor
            .optional_reference()?
            .map(ProtectedClassRef::from_artifact_ref);
        let grain = GrainRef::from_artifact_ref(cursor.reference()?);
        let horizon = HorizonRef::from_artifact_ref(cursor.reference()?);
        let binding = BindingVersionRef::from_artifact_ref(cursor.reference()?);
        let available_structure = StructureViewRef::from_artifact_ref(cursor.reference()?);
        let generator_regime = GeneratorRegimeRef::from_artifact_ref(cursor.reference()?);
        let effectivity = EffectivityRef::from_artifact_ref(cursor.reference()?);
        if !cursor.finished() {
            return Err(SeparatorProblemError::TrailingPayloadBytes(
                cursor.remaining(),
            ));
        }
        Ok(Self::new(
            residual,
            target,
            grain,
            horizon,
            binding,
            available_structure,
            generator_regime,
            effectivity,
        ))
    }

    /// Returns the canonical artifact envelope for this structural residual record.
    pub fn envelope(&self) -> Result<ArtifactEnvelope, SeparatorProblemError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(SEPARATOR_PROBLEM_ARTIFACT_KIND)?,
            SEPARATOR_PROBLEM_SCHEMA_VERSION,
            self.canonical_payload(),
        ))
    }

    /// Returns the stable content identity of this separator problem.
    pub fn separator_problem_ref(&self) -> Result<SeparatorProblemRef, SeparatorProblemError> {
        Ok(SeparatorProblemRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    /// Decodes this artifact only when it names the canonical separator-problem domain/version.
    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, SeparatorProblemError> {
        if envelope.kind().as_str() != SEPARATOR_PROBLEM_ARTIFACT_KIND {
            return Err(SeparatorProblemError::UnexpectedArtifactKind {
                expected: SEPARATOR_PROBLEM_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != SEPARATOR_PROBLEM_SCHEMA_VERSION {
            return Err(SeparatorProblemError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    /// Lists prerequisite identities without interpreting their payloads.
    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = vec![self.residual.as_artifact_ref()];
        if let Some(target) = self.target {
            references.push(target.as_artifact_ref());
        }
        references.extend([
            self.grain.as_artifact_ref(),
            self.horizon.as_artifact_ref(),
            self.binding.as_artifact_ref(),
            self.available_structure.as_artifact_ref(),
            self.generator_regime.as_artifact_ref(),
            self.effectivity.as_artifact_ref(),
        ]);
        references
    }
}

fn reference(encoded: &mut Vec<u8>, value: ArtifactRef) {
    encoded.extend_from_slice(value.as_bytes());
}

fn optional_reference(encoded: &mut Vec<u8>, value: Option<ArtifactRef>) {
    match value {
        None => encoded.push(0),
        Some(value) => {
            encoded.push(1);
            reference(encoded, value);
        }
    }
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], SeparatorProblemError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(SeparatorProblemError::PayloadLengthOverflow)?;
        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(SeparatorProblemError::TruncatedPayload)?;
        self.position = end;
        Ok(bytes)
    }

    fn reference(&mut self) -> Result<ArtifactRef, SeparatorProblemError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| SeparatorProblemError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(bytes))
    }

    fn optional_reference(&mut self) -> Result<Option<ArtifactRef>, SeparatorProblemError> {
        match self.take(1)?[0] {
            0 => Ok(None),
            1 => self.reference().map(Some),
            tag => Err(SeparatorProblemError::UnknownOptionalTag(tag)),
        }
    }

    const fn finished(&self) -> bool {
        self.position == self.bytes.len()
    }

    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

/// Errors from canonical separator-problem encoding and decoding.
#[derive(Debug, Error)]
pub enum SeparatorProblemError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("separator-problem payload is truncated")]
    TruncatedPayload,
    #[error("separator-problem payload length overflows this platform")]
    PayloadLengthOverflow,
    #[error("separator-problem payload contains {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("separator-problem payload has unknown optional-reference tag {0}")]
    UnknownOptionalTag(u8),
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported separator-problem schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

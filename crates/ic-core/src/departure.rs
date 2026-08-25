use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef,
    DeterminationPresentationRef, DistinctionRef, GrainRef, RelationUseRef, ScopeRef, SupportRef,
    TypedFormRef,
};

/// Canonical artifact kind for positive determination-relative departure witnesses.
pub const DEPARTURE_WITNESS_ARTIFACT_KIND: &str = "ic.departure-witness";
/// Payload schema version for positive determination-relative departure witnesses.
pub const DEPARTURE_WITNESS_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DepartureWitnessRef(ArtifactRef);

impl DepartureWitnessRef {
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }

    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}

impl fmt::Display for DepartureWitnessRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for DepartureWitnessRef {
    type Err = ArtifactError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// Positive evidence that a candidate departs from one standing source determination.
///
/// The record does not itself evaluate observations, infer incompatibility, create an exterior,
/// or turn unknown/failed work into a negative result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepartureWitness {
    distinction: DistinctionRef,
    source: TypedFormRef,
    candidate: TypedFormRef,
    source_presentation: DeterminationPresentationRef,
    source_observation: RelationUseRef,
    candidate_observation: RelationUseRef,
    source_answer: TypedFormRef,
    candidate_answer: TypedFormRef,
    incompatibility: RelationUseRef,
    support: SupportRef,
    scope: ScopeRef,
    applicability: ApplicabilityRef,
    grain: GrainRef,
}

impl DepartureWitness {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        distinction: DistinctionRef,
        source: TypedFormRef,
        candidate: TypedFormRef,
        source_presentation: DeterminationPresentationRef,
        source_observation: RelationUseRef,
        candidate_observation: RelationUseRef,
        source_answer: TypedFormRef,
        candidate_answer: TypedFormRef,
        incompatibility: RelationUseRef,
        support: SupportRef,
        scope: ScopeRef,
        applicability: ApplicabilityRef,
        grain: GrainRef,
    ) -> Self {
        Self {
            distinction,
            source,
            candidate,
            source_presentation,
            source_observation,
            candidate_observation,
            source_answer,
            candidate_answer,
            incompatibility,
            support,
            scope,
            applicability,
            grain,
        }
    }

    #[must_use]
    pub const fn distinction(&self) -> DistinctionRef {
        self.distinction
    }
    #[must_use]
    pub const fn source(&self) -> TypedFormRef {
        self.source
    }
    #[must_use]
    pub const fn candidate(&self) -> TypedFormRef {
        self.candidate
    }
    #[must_use]
    pub const fn source_presentation(&self) -> DeterminationPresentationRef {
        self.source_presentation
    }
    #[must_use]
    pub const fn source_observation(&self) -> RelationUseRef {
        self.source_observation
    }
    #[must_use]
    pub const fn candidate_observation(&self) -> RelationUseRef {
        self.candidate_observation
    }
    #[must_use]
    pub const fn source_answer(&self) -> TypedFormRef {
        self.source_answer
    }
    #[must_use]
    pub const fn candidate_answer(&self) -> TypedFormRef {
        self.candidate_answer
    }
    #[must_use]
    pub const fn incompatibility(&self) -> RelationUseRef {
        self.incompatibility
    }
    #[must_use]
    pub const fn support(&self) -> SupportRef {
        self.support
    }
    #[must_use]
    pub const fn scope(&self) -> ScopeRef {
        self.scope
    }
    #[must_use]
    pub const fn applicability(&self) -> ApplicabilityRef {
        self.applicability
    }
    #[must_use]
    pub const fn grain(&self) -> GrainRef {
        self.grain
    }

    #[must_use]
    pub fn canonical_payload(&self) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(416);
        for reference in self.referenced_artifacts() {
            encoded.extend_from_slice(reference.as_bytes());
        }
        encoded
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, DepartureWitnessError> {
        let mut cursor = Cursor::new(payload);
        let witness = Self::new(
            DistinctionRef::from_artifact_ref(cursor.reference()?),
            TypedFormRef::from_artifact_ref(cursor.reference()?),
            TypedFormRef::from_artifact_ref(cursor.reference()?),
            DeterminationPresentationRef::from_artifact_ref(cursor.reference()?),
            RelationUseRef::from_artifact_ref(cursor.reference()?),
            RelationUseRef::from_artifact_ref(cursor.reference()?),
            TypedFormRef::from_artifact_ref(cursor.reference()?),
            TypedFormRef::from_artifact_ref(cursor.reference()?),
            RelationUseRef::from_artifact_ref(cursor.reference()?),
            SupportRef::from_artifact_ref(cursor.reference()?),
            ScopeRef::from_artifact_ref(cursor.reference()?),
            ApplicabilityRef::from_artifact_ref(cursor.reference()?),
            GrainRef::from_artifact_ref(cursor.reference()?),
        );
        if !cursor.finished() {
            return Err(DepartureWitnessError::TrailingPayloadBytes(
                cursor.remaining(),
            ));
        }
        Ok(witness)
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, DepartureWitnessError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(DEPARTURE_WITNESS_ARTIFACT_KIND)?,
            DEPARTURE_WITNESS_SCHEMA_VERSION,
            self.canonical_payload(),
        ))
    }

    pub fn departure_witness_ref(&self) -> Result<DepartureWitnessRef, DepartureWitnessError> {
        Ok(DepartureWitnessRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, DepartureWitnessError> {
        if envelope.kind().as_str() != DEPARTURE_WITNESS_ARTIFACT_KIND {
            return Err(DepartureWitnessError::UnexpectedArtifactKind {
                expected: DEPARTURE_WITNESS_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != DEPARTURE_WITNESS_SCHEMA_VERSION {
            return Err(DepartureWitnessError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> [ArtifactRef; 13] {
        [
            self.distinction.as_artifact_ref(),
            self.source.as_artifact_ref(),
            self.candidate.as_artifact_ref(),
            self.source_presentation.as_artifact_ref(),
            self.source_observation.as_artifact_ref(),
            self.candidate_observation.as_artifact_ref(),
            self.source_answer.as_artifact_ref(),
            self.candidate_answer.as_artifact_ref(),
            self.incompatibility.as_artifact_ref(),
            self.support.as_artifact_ref(),
            self.scope.as_artifact_ref(),
            self.applicability.as_artifact_ref(),
            self.grain.as_artifact_ref(),
        ]
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
    fn reference(&mut self) -> Result<ArtifactRef, DepartureWitnessError> {
        let end = self
            .position
            .checked_add(32)
            .ok_or(DepartureWitnessError::PayloadLengthOverflow)?;
        let bytes: [u8; 32] = self
            .bytes
            .get(self.position..end)
            .ok_or(DepartureWitnessError::TruncatedPayload)?
            .try_into()
            .map_err(|_| DepartureWitnessError::TruncatedPayload)?;
        self.position = end;
        Ok(ArtifactRef::from_bytes(bytes))
    }
    const fn finished(&self) -> bool {
        self.position == self.bytes.len()
    }
    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

#[derive(Debug, Error)]
pub enum DepartureWitnessError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("departure-witness payload is truncated")]
    TruncatedPayload,
    #[error("departure-witness payload length overflows this platform")]
    PayloadLengthOverflow,
    #[error("departure-witness payload contains {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported departure-witness schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

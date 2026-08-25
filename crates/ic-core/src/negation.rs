use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, DepartureCatalog,
    DeterminationPresentationCheckError, DeterminationPresentationError,
    DeterminationPresentationRef, DistinctionRef, GrainRef, HorizonRef, IProgCatalog,
    IProgCheckError, IProgError, IProgRef, Orientation, RelationCatalog, RelationCheckError,
    RelationError, RelationRef, RelationUseCheckError, RelationUseError, RelationUseRef, ScopeRef,
};

/// Canonical artifact kind for an oriented, not-yet-admitted negation-use declaration.
pub const NEGATION_USE_ARTIFACT_KIND: &str = "ic.negation-use";
/// Payload schema version for oriented negation-use declarations.
pub const NEGATION_USE_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NegationUseRef(ArtifactRef);

impl NegationUseRef {
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }

    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}

impl fmt::Display for NegationUseRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for NegationUseRef {
    type Err = ArtifactError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// The declared semantic coverage of an oriented negation use.
///
/// This describes a claimed semantic boundary only. It is deliberately distinct from any later
/// executed/materialized generator coverage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NegationCoverage {
    ExactExhaustive {
        regime: ArtifactRef,
        certificate: ArtifactRef,
    },
    ExactOnField {
        field: RelationRef,
        certificate: ArtifactRef,
    },
    CertifiedPartial,
    WorkingOpen,
}

/// A declaration that one immutable relation use is intended as an oriented negation role.
///
/// Structural checking validates the stated identities and shared determination context. It does
/// not evaluate the relation, execute the soundness program, establish coverage, or admit an
/// incidence as positive negation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NegationUse {
    relation_use: RelationUseRef,
    distinction: DistinctionRef,
    orientation: Orientation,
    source_determination: DeterminationPresentationRef,
    candidate_field: RelationRef,
    soundness_derivation: IProgRef,
    semantic_coverage: NegationCoverage,
    applicability: ApplicabilityRef,
    scope: ScopeRef,
    grain: GrainRef,
    horizon: HorizonRef,
    provenance: Vec<ArtifactRef>,
}

/// The checked source for negation-use declarations.
pub trait NegationCatalog: DepartureCatalog + IProgCatalog {}

impl<T: DepartureCatalog + IProgCatalog> NegationCatalog for T {}

impl NegationUse {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        relation_use: RelationUseRef,
        distinction: DistinctionRef,
        orientation: Orientation,
        source_determination: DeterminationPresentationRef,
        candidate_field: RelationRef,
        soundness_derivation: IProgRef,
        semantic_coverage: NegationCoverage,
        applicability: ApplicabilityRef,
        scope: ScopeRef,
        grain: GrainRef,
        horizon: HorizonRef,
        provenance: Vec<ArtifactRef>,
    ) -> Self {
        Self {
            relation_use,
            distinction,
            orientation,
            source_determination,
            candidate_field,
            soundness_derivation,
            semantic_coverage,
            applicability,
            scope,
            grain,
            horizon,
            provenance,
        }
    }

    #[must_use]
    pub const fn relation_use(&self) -> RelationUseRef {
        self.relation_use
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
    pub const fn source_determination(&self) -> DeterminationPresentationRef {
        self.source_determination
    }
    #[must_use]
    pub const fn candidate_field(&self) -> RelationRef {
        self.candidate_field
    }
    #[must_use]
    pub const fn soundness_derivation(&self) -> IProgRef {
        self.soundness_derivation
    }
    #[must_use]
    pub const fn semantic_coverage(&self) -> NegationCoverage {
        self.semantic_coverage
    }
    #[must_use]
    pub const fn applicability(&self) -> ApplicabilityRef {
        self.applicability
    }
    #[must_use]
    pub const fn scope(&self) -> ScopeRef {
        self.scope
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
    pub fn provenance(&self) -> &[ArtifactRef] {
        &self.provenance
    }

    pub fn canonical_payload(&self) -> Result<Vec<u8>, NegationUseError> {
        let mut encoded = Vec::new();
        reference(&mut encoded, self.relation_use.as_artifact_ref());
        reference(&mut encoded, self.distinction.as_artifact_ref());
        encoded.push(orientation_tag(self.orientation));
        reference(&mut encoded, self.source_determination.as_artifact_ref());
        reference(&mut encoded, self.candidate_field.as_artifact_ref());
        reference(&mut encoded, self.soundness_derivation.as_artifact_ref());
        match self.semantic_coverage {
            NegationCoverage::ExactExhaustive {
                regime,
                certificate,
            } => {
                encoded.push(0);
                reference(&mut encoded, regime);
                reference(&mut encoded, certificate);
            }
            NegationCoverage::ExactOnField { field, certificate } => {
                encoded.push(1);
                reference(&mut encoded, field.as_artifact_ref());
                reference(&mut encoded, certificate);
            }
            NegationCoverage::CertifiedPartial => encoded.push(2),
            NegationCoverage::WorkingOpen => encoded.push(3),
        }
        reference(&mut encoded, self.applicability.as_artifact_ref());
        reference(&mut encoded, self.scope.as_artifact_ref());
        reference(&mut encoded, self.grain.as_artifact_ref());
        reference(&mut encoded, self.horizon.as_artifact_ref());
        count(&mut encoded, self.provenance.len())?;
        for reference_value in &self.provenance {
            reference(&mut encoded, *reference_value);
        }
        Ok(encoded)
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, NegationUseError> {
        let mut cursor = Cursor::new(payload);
        let relation_use = RelationUseRef::from_artifact_ref(cursor.reference()?);
        let distinction = DistinctionRef::from_artifact_ref(cursor.reference()?);
        let orientation = parse_orientation(cursor.byte()?)?;
        let source_determination =
            DeterminationPresentationRef::from_artifact_ref(cursor.reference()?);
        let candidate_field = RelationRef::from_artifact_ref(cursor.reference()?);
        let soundness_derivation = IProgRef::from_artifact_ref(cursor.reference()?);
        let semantic_coverage = match cursor.byte()? {
            0 => NegationCoverage::ExactExhaustive {
                regime: cursor.reference()?,
                certificate: cursor.reference()?,
            },
            1 => NegationCoverage::ExactOnField {
                field: RelationRef::from_artifact_ref(cursor.reference()?),
                certificate: cursor.reference()?,
            },
            2 => NegationCoverage::CertifiedPartial,
            3 => NegationCoverage::WorkingOpen,
            tag => return Err(NegationUseError::UnknownCoverageTag(tag)),
        };
        let applicability = ApplicabilityRef::from_artifact_ref(cursor.reference()?);
        let scope = ScopeRef::from_artifact_ref(cursor.reference()?);
        let grain = GrainRef::from_artifact_ref(cursor.reference()?);
        let horizon = HorizonRef::from_artifact_ref(cursor.reference()?);
        let provenance_count = cursor.count()?;
        let mut provenance = Vec::with_capacity(provenance_count);
        for _ in 0..provenance_count {
            provenance.push(cursor.reference()?);
        }
        if !cursor.finished() {
            return Err(NegationUseError::TrailingPayloadBytes(cursor.remaining()));
        }
        Ok(Self::new(
            relation_use,
            distinction,
            orientation,
            source_determination,
            candidate_field,
            soundness_derivation,
            semantic_coverage,
            applicability,
            scope,
            grain,
            horizon,
            provenance,
        ))
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, NegationUseError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(NEGATION_USE_ARTIFACT_KIND)?,
            NEGATION_USE_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }

    pub fn negation_use_ref(&self) -> Result<NegationUseRef, NegationUseError> {
        Ok(NegationUseRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, NegationUseError> {
        if envelope.kind().as_str() != NEGATION_USE_ARTIFACT_KIND {
            return Err(NegationUseError::UnexpectedArtifactKind {
                expected: NEGATION_USE_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != NEGATION_USE_SCHEMA_VERSION {
            return Err(NegationUseError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    /// Checks only declaration linkage; it neither executes nor admits negation soundness.
    pub fn check<C: NegationCatalog>(&self, catalog: &C) -> Result<(), NegationUseCheckError> {
        let presentation = catalog
            .resolve_determination_presentation(self.source_determination)
            .ok_or(NegationUseCheckError::UnresolvedPresentation(
                self.source_determination,
            ))?;
        let calculated_presentation = presentation.determination_presentation_ref()?;
        if calculated_presentation != self.source_determination {
            return Err(NegationUseCheckError::PresentationIdentityMismatch {
                reference: self.source_determination,
                calculated: calculated_presentation,
            });
        }
        presentation.check(catalog)?;
        if self.distinction != presentation.distinction() {
            return Err(NegationUseCheckError::PresentationMismatch("distinction"));
        }
        if self.orientation != presentation.orientation() {
            return Err(NegationUseCheckError::PresentationMismatch("orientation"));
        }
        if self.applicability != presentation.applicability() {
            return Err(NegationUseCheckError::PresentationMismatch("applicability"));
        }
        if self.scope != presentation.scope() {
            return Err(NegationUseCheckError::PresentationMismatch("scope"));
        }
        if self.grain != presentation.grain() {
            return Err(NegationUseCheckError::PresentationMismatch("grain"));
        }
        if self.horizon != presentation.horizon() {
            return Err(NegationUseCheckError::PresentationMismatch("horizon"));
        }

        let relation_use = catalog.resolve_relation_use(self.relation_use).ok_or(
            NegationUseCheckError::UnresolvedRelationUse(self.relation_use),
        )?;
        let calculated_use = relation_use.relation_use_ref()?;
        if calculated_use != self.relation_use {
            return Err(NegationUseCheckError::RelationUseIdentityMismatch {
                reference: self.relation_use,
                calculated: calculated_use,
            });
        }
        relation_use.check(catalog)?;
        if relation_use.applicability() != self.applicability
            || relation_use.scope() != self.scope
            || relation_use.grain() != self.grain
            || relation_use.horizon() != self.horizon
        {
            return Err(NegationUseCheckError::RelationUseContextMismatch(
                self.relation_use,
            ));
        }

        check_relation(self.candidate_field, presentation.binding(), catalog)?;
        if let NegationCoverage::ExactOnField { field, .. } = self.semantic_coverage {
            check_relation(field, presentation.binding(), catalog)?;
        }

        let derivation = catalog.resolve_iprog(self.soundness_derivation).ok_or(
            NegationUseCheckError::UnresolvedSoundnessDerivation(self.soundness_derivation),
        )?;
        let calculated_derivation = derivation.iprog_ref()?;
        if calculated_derivation != self.soundness_derivation {
            return Err(NegationUseCheckError::SoundnessDerivationIdentityMismatch {
                reference: self.soundness_derivation,
                calculated: calculated_derivation,
            });
        }
        derivation.check(catalog)?;
        Ok(())
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = vec![
            self.relation_use.as_artifact_ref(),
            self.distinction.as_artifact_ref(),
            self.source_determination.as_artifact_ref(),
            self.candidate_field.as_artifact_ref(),
            self.soundness_derivation.as_artifact_ref(),
        ];
        match self.semantic_coverage {
            NegationCoverage::ExactExhaustive {
                regime,
                certificate,
            } => references.extend([regime, certificate]),
            NegationCoverage::ExactOnField { field, certificate } => {
                references.extend([field.as_artifact_ref(), certificate]);
            }
            NegationCoverage::CertifiedPartial | NegationCoverage::WorkingOpen => {}
        }
        references.extend([
            self.applicability.as_artifact_ref(),
            self.scope.as_artifact_ref(),
            self.grain.as_artifact_ref(),
            self.horizon.as_artifact_ref(),
        ]);
        references.extend(self.provenance.iter().copied());
        references
    }
}

fn check_relation<C: RelationCatalog>(
    reference: RelationRef,
    expected_binding: crate::BindingVersionRef,
    catalog: &C,
) -> Result<(), NegationUseCheckError> {
    let relation = catalog
        .resolve_relation_schema(reference)
        .ok_or(NegationUseCheckError::UnresolvedRelation(reference))?;
    let calculated = relation.relation_ref()?;
    if calculated != reference {
        return Err(NegationUseCheckError::RelationIdentityMismatch {
            reference,
            calculated,
        });
    }
    relation.check(catalog)?;
    if relation.binding() != expected_binding {
        return Err(NegationUseCheckError::RelationBindingMismatch {
            reference,
            expected: expected_binding,
            actual: relation.binding(),
        });
    }
    Ok(())
}

const fn orientation_tag(orientation: Orientation) -> u8 {
    match orientation {
        Orientation::X => 0,
        Orientation::Y => 1,
    }
}

fn parse_orientation(tag: u8) -> Result<Orientation, NegationUseError> {
    match tag {
        0 => Ok(Orientation::X),
        1 => Ok(Orientation::Y),
        _ => Err(NegationUseError::UnknownOrientation(tag)),
    }
}

fn reference(encoded: &mut Vec<u8>, value: ArtifactRef) {
    encoded.extend_from_slice(value.as_bytes());
}

fn count(encoded: &mut Vec<u8>, value: usize) -> Result<(), NegationUseError> {
    let value = u32::try_from(value).map_err(|_| NegationUseError::TooManyProvenance(value))?;
    encoded.extend_from_slice(&value.to_be_bytes());
    Ok(())
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], NegationUseError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(NegationUseError::PayloadLengthOverflow)?;
        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(NegationUseError::TruncatedPayload)?;
        self.position = end;
        Ok(bytes)
    }

    fn byte(&mut self) -> Result<u8, NegationUseError> {
        Ok(self.take(1)?[0])
    }

    fn reference(&mut self) -> Result<ArtifactRef, NegationUseError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| NegationUseError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(bytes))
    }

    fn count(&mut self) -> Result<usize, NegationUseError> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| NegationUseError::TruncatedPayload)?;
        usize::try_from(u32::from_be_bytes(bytes)).map_err(|_| NegationUseError::CountOverflow)
    }

    const fn finished(&self) -> bool {
        self.position == self.bytes.len()
    }

    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

#[derive(Debug, Error)]
pub enum NegationUseError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("negation-use payload is truncated")]
    TruncatedPayload,
    #[error("negation-use payload length overflows this platform")]
    PayloadLengthOverflow,
    #[error("negation-use payload contains {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("negation-use payload has an unknown orientation tag {0}")]
    UnknownOrientation(u8),
    #[error("negation-use payload has an unknown coverage tag {0}")]
    UnknownCoverageTag(u8),
    #[error("negation-use provenance has {0} entries, exceeding u32")]
    TooManyProvenance(usize),
    #[error("negation-use payload count does not fit this platform")]
    CountOverflow,
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported negation-use schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

#[derive(Debug, Error)]
pub enum NegationUseCheckError {
    #[error(transparent)]
    Encoding(#[from] NegationUseError),
    #[error(transparent)]
    PresentationEncoding(#[from] DeterminationPresentationError),
    #[error(transparent)]
    Presentation(#[from] DeterminationPresentationCheckError),
    #[error(transparent)]
    RelationEncoding(#[from] RelationError),
    #[error(transparent)]
    Relation(#[from] RelationCheckError),
    #[error(transparent)]
    RelationUseEncoding(#[from] RelationUseError),
    #[error(transparent)]
    RelationUse(#[from] RelationUseCheckError),
    #[error(transparent)]
    ProgramEncoding(#[from] IProgError),
    #[error(transparent)]
    Program(#[from] IProgCheckError),
    #[error("source determination {0} is unavailable")]
    UnresolvedPresentation(DeterminationPresentationRef),
    #[error("source determination {reference} hashes to {calculated}, not its claimed identity")]
    PresentationIdentityMismatch {
        reference: DeterminationPresentationRef,
        calculated: DeterminationPresentationRef,
    },
    #[error("negation-use {0} does not match its source determination")]
    PresentationMismatch(&'static str),
    #[error("relation use {0} is unavailable")]
    UnresolvedRelationUse(RelationUseRef),
    #[error("relation use {reference} hashes to {calculated}, not its claimed identity")]
    RelationUseIdentityMismatch {
        reference: RelationUseRef,
        calculated: RelationUseRef,
    },
    #[error("relation use {0} does not match negation-use context")]
    RelationUseContextMismatch(RelationUseRef),
    #[error("relation schema {0} is unavailable")]
    UnresolvedRelation(RelationRef),
    #[error("relation schema {reference} hashes to {calculated}, not its claimed identity")]
    RelationIdentityMismatch {
        reference: RelationRef,
        calculated: RelationRef,
    },
    #[error("relation schema {reference} has binding {actual}, expected {expected}")]
    RelationBindingMismatch {
        reference: RelationRef,
        expected: crate::BindingVersionRef,
        actual: crate::BindingVersionRef,
    },
    #[error("soundness derivation {0} is unavailable")]
    UnresolvedSoundnessDerivation(IProgRef),
    #[error("soundness derivation {reference} hashes to {calculated}, not its claimed identity")]
    SoundnessDerivationIdentityMismatch {
        reference: IProgRef,
        calculated: IProgRef,
    },
}

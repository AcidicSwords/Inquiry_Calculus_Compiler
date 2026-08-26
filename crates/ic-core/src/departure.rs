use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef,
    DeterminationCatalog, DeterminationPresentationCheckError, DeterminationPresentationError,
    DeterminationPresentationRef, DeterminationSupportCatalog, DeterminationSupportError,
    DischargeMode, DistinctionRef, GrainRef, RelationCatalog, RelationUse, RelationUseCheckError,
    RelationUseError, RelationUseRef, RelationUseSupportCatalog, RelationUseSupportError,
    ResolvedDeterminationSupport, ResolvedRelationUseSupport, ScopeRef, Standing, SupportRef,
    TypeCheckError, TypeError, TypedFormRef, resolve_relation_use_support,
    standing_determination_presentation_support,
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

/// The checked source for departure-witness references.
pub trait DepartureCatalog: DeterminationCatalog + RelationCatalog {
    /// Resolves a relation use by its claimed stable identity.
    fn resolve_relation_use(&self, reference: RelationUseRef) -> Option<RelationUse>;
}

/// The checked source needed to connect a departure witness to declared standing support.
pub trait DepartureStandingCatalog: DepartureCatalog + DeterminationSupportCatalog {}

impl<T> DepartureStandingCatalog for T where T: DepartureCatalog + DeterminationSupportCatalog {}

/// The checked source needed to validate both a departure's source standing link and its three
/// relation-targeted evidence-support routes.
pub trait DepartureEvidenceSupportCatalog:
    DepartureStandingCatalog + RelationUseSupportCatalog
{
}

impl<T> DepartureEvidenceSupportCatalog for T where
    T: DepartureStandingCatalog + RelationUseSupportCatalog
{
}

/// Resolved support routes for one structurally checked positive-departure witness.
///
/// The source presentation and relation uses intentionally retain different support roles: the
/// former is claim-targeted and the latter are relation-targeted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedDepartureEvidenceSupport {
    source_presentation: ResolvedDeterminationSupport,
    source_observation: ResolvedRelationUseSupport,
    candidate_observation: ResolvedRelationUseSupport,
    incompatibility: ResolvedRelationUseSupport,
}

impl ResolvedDepartureEvidenceSupport {
    #[must_use]
    pub const fn source_presentation(&self) -> ResolvedDeterminationSupport {
        self.source_presentation
    }
    #[must_use]
    pub const fn source_observation(&self) -> ResolvedRelationUseSupport {
        self.source_observation
    }
    #[must_use]
    pub const fn candidate_observation(&self) -> ResolvedRelationUseSupport {
        self.candidate_observation
    }
    #[must_use]
    pub const fn incompatibility(&self) -> ResolvedRelationUseSupport {
        self.incompatibility
    }
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

    /// Checks that this witness describes one exact standing presentation and context.
    ///
    /// It does not evaluate either observation, infer incompatibility, or certify that the
    /// declared relation uses are non-circular; those require later observation and derivation
    /// representations.
    pub fn check<C: DepartureCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), DepartureWitnessCheckError> {
        let presentation = catalog
            .resolve_determination_presentation(self.source_presentation)
            .ok_or(DepartureWitnessCheckError::UnresolvedPresentation(
                self.source_presentation,
            ))?;
        let calculated = presentation.determination_presentation_ref()?;
        if calculated != self.source_presentation {
            return Err(
                DepartureWitnessCheckError::PresentationReferenceIdentityMismatch {
                    reference: self.source_presentation,
                    calculated,
                },
            );
        }
        presentation.check(catalog)?;
        if self.distinction != presentation.distinction() {
            return Err(DepartureWitnessCheckError::PresentationMismatch(
                "distinction",
            ));
        }
        if self.source != presentation.source() {
            return Err(DepartureWitnessCheckError::PresentationMismatch("source"));
        }
        if self.scope != presentation.scope() {
            return Err(DepartureWitnessCheckError::PresentationMismatch("scope"));
        }
        if self.applicability != presentation.applicability() {
            return Err(DepartureWitnessCheckError::PresentationMismatch(
                "applicability",
            ));
        }
        if self.grain != presentation.grain() {
            return Err(DepartureWitnessCheckError::PresentationMismatch("grain"));
        }
        for form in [
            self.source,
            self.candidate,
            self.source_answer,
            self.candidate_answer,
        ] {
            let resolved = catalog
                .resolve_typed_form(form)
                .ok_or(DepartureWitnessCheckError::UnresolvedTypedForm(form))?;
            let calculated = resolved.typed_form_ref()?;
            if calculated != form {
                return Err(
                    DepartureWitnessCheckError::TypedFormReferenceIdentityMismatch {
                        reference: form,
                        calculated,
                    },
                );
            }
            resolved.check(catalog)?;
            if resolved.binding() != presentation.binding() {
                return Err(DepartureWitnessCheckError::TypedFormBindingMismatch {
                    expected: presentation.binding(),
                    actual: resolved.binding(),
                });
            }
        }
        for (claim, use_ref, left, right) in [
            (
                "source observation",
                self.source_observation,
                self.source,
                self.source_answer,
            ),
            (
                "candidate observation",
                self.candidate_observation,
                self.candidate,
                self.candidate_answer,
            ),
            (
                "incompatibility",
                self.incompatibility,
                self.source_answer,
                self.candidate_answer,
            ),
        ] {
            let relation_use = catalog
                .resolve_relation_use(use_ref)
                .ok_or(DepartureWitnessCheckError::UnresolvedRelationUse(use_ref))?;
            let calculated = relation_use.relation_use_ref()?;
            if calculated != use_ref {
                return Err(
                    DepartureWitnessCheckError::RelationUseReferenceIdentityMismatch {
                        reference: use_ref,
                        calculated,
                    },
                );
            }
            relation_use.check(catalog)?;
            if relation_use.scope() != self.scope
                || relation_use.applicability() != self.applicability
                || relation_use.grain() != self.grain
                || relation_use.horizon() != presentation.horizon()
            {
                return Err(DepartureWitnessCheckError::RelationUseContextMismatch(
                    use_ref,
                ));
            }
            // A generator proposes a provisional filling; it never supports one.
            // Admitting a `Generate` route here would let a merely generated
            // answer stand as positive departure evidence, which is the exact
            // self-promotion the departure contract exists to refuse. The other
            // modes are left alone: a `Pure` derivation from already-standing
            // data is a lawful route, so this rejects generation, not everything
            // that is not a probe. It does not establish that the retained
            // routes are actually supported; support remains unrepresented.
            if relation_use.mode() == DischargeMode::Generate {
                return Err(DepartureWitnessCheckError::GeneratedEvidenceRoute {
                    claim,
                    relation_use: use_ref,
                });
            }
            if !relation_use_binds_pair(&relation_use, left, right) {
                return Err(DepartureWitnessCheckError::ClaimedPairNotBound {
                    claim,
                    relation_use: use_ref,
                });
            }
        }
        Ok(())
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

/// Revalidates a positive-departure witness and requires its source presentation support to target
/// a claim in one declared least-fixed-point standing result.
///
/// This is intentionally a narrow association. It does not evaluate observations or
/// incompatibility, connect individual relation-use support routes to standing, interpret the
/// target claim as the source form, prove web relevance or non-circularity, or turn declared
/// closure inputs into actuality or warrant.
pub fn check_departure_witness_standing_support<C: DepartureStandingCatalog>(
    witness: &DepartureWitness,
    standing: &Standing,
    catalog: &C,
) -> Result<ResolvedDeterminationSupport, DepartureStandingCheckError> {
    witness.check(catalog)?;
    Ok(standing_determination_presentation_support(
        witness.source_presentation(),
        standing,
        catalog,
    )?)
}

/// Resolves the evidence routes of a standing-source departure witness without making them
/// standing evidence themselves.
///
/// Every relation use must name an exact relation-targeted support environment with matching
/// context. This does not evaluate any observation, close a support route, establish that an
/// incompatibility relation stands, prove relevance/non-circularity, or create actuality/warrant.
pub fn resolve_departure_witness_evidence_support<C: DepartureEvidenceSupportCatalog>(
    witness: &DepartureWitness,
    standing: &Standing,
    catalog: &C,
) -> Result<ResolvedDepartureEvidenceSupport, DepartureEvidenceSupportError> {
    let source_presentation = check_departure_witness_standing_support(witness, standing, catalog)?;
    let source_observation = resolve_relation_use_support(witness.source_observation(), catalog)?;
    let candidate_observation =
        resolve_relation_use_support(witness.candidate_observation(), catalog)?;
    let incompatibility = resolve_relation_use_support(witness.incompatibility(), catalog)?;
    Ok(ResolvedDepartureEvidenceSupport {
        source_presentation,
        source_observation,
        candidate_observation,
        incompatibility,
    })
}

pub(crate) fn relation_use_binds_pair(
    relation_use: &RelationUse,
    left: TypedFormRef,
    right: TypedFormRef,
) -> bool {
    let left_bindings = relation_use
        .bindings()
        .iter()
        .filter(|binding| binding.value() == left)
        .count();
    if left == right {
        left_bindings >= 2
    } else {
        left_bindings >= 1
            && relation_use
                .bindings()
                .iter()
                .any(|binding| binding.value() == right)
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

/// Errors from structural positive-departure witness checking.
#[derive(Debug, Error)]
pub enum DepartureWitnessCheckError {
    #[error(transparent)]
    Witness(#[from] DepartureWitnessError),
    #[error(transparent)]
    PresentationEncoding(#[from] DeterminationPresentationError),
    #[error(transparent)]
    Presentation(#[from] DeterminationPresentationCheckError),
    #[error(transparent)]
    Type(#[from] TypeError),
    #[error(transparent)]
    TypeCheck(#[from] TypeCheckError),
    #[error(transparent)]
    RelationUse(#[from] RelationUseCheckError),
    #[error(transparent)]
    RelationUseEncoding(#[from] RelationUseError),
    #[error("source presentation {0} is not available from the declared catalog")]
    UnresolvedPresentation(DeterminationPresentationRef),
    #[error("catalog presentation {reference} hashes to {calculated}, not its claimed identity")]
    PresentationReferenceIdentityMismatch {
        reference: DeterminationPresentationRef,
        calculated: DeterminationPresentationRef,
    },
    #[error("departure witness does not match its source presentation's {0}")]
    PresentationMismatch(&'static str),
    #[error("typed form {0} is not available from the declared catalog")]
    UnresolvedTypedForm(TypedFormRef),
    #[error("catalog typed form {reference} hashes to {calculated}, not its claimed identity")]
    TypedFormReferenceIdentityMismatch {
        reference: TypedFormRef,
        calculated: TypedFormRef,
    },
    #[error("typed form binding {actual} does not match presentation binding {expected}")]
    TypedFormBindingMismatch {
        expected: crate::BindingVersionRef,
        actual: crate::BindingVersionRef,
    },
    #[error("relation use {0} is not available from the declared catalog")]
    UnresolvedRelationUse(RelationUseRef),
    #[error("catalog relation use {reference} hashes to {calculated}, not its claimed identity")]
    RelationUseReferenceIdentityMismatch {
        reference: RelationUseRef,
        calculated: RelationUseRef,
    },
    #[error("relation use {0} does not match the departure witness context")]
    RelationUseContextMismatch(RelationUseRef),
    #[error(
        "{claim} relation use {relation_use} declares Generate, which proposes rather than supports"
    )]
    GeneratedEvidenceRoute {
        claim: &'static str,
        relation_use: RelationUseRef,
    },
    #[error("{claim} relation use {relation_use} does not bind its claimed pair")]
    ClaimedPairNotBound {
        claim: &'static str,
        relation_use: RelationUseRef,
    },
}

/// Errors from checking a departure witness against declared standing support.
#[derive(Debug, Error)]
pub enum DepartureStandingCheckError {
    #[error(transparent)]
    Witness(#[from] DepartureWitnessCheckError),
    #[error(transparent)]
    DeterminationSupport(#[from] DeterminationSupportError),
}

/// Errors from resolving a departure witness's source and evidence support routes.
#[derive(Debug, Error)]
pub enum DepartureEvidenceSupportError {
    #[error(transparent)]
    Standing(#[from] DepartureStandingCheckError),
    #[error(transparent)]
    RelationUse(#[from] RelationUseSupportError),
}

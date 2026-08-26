use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    str::FromStr,
};

use thiserror::Error;

use crate::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactKind, ClaimArtifact, ClaimCatalog, ClaimCheckError,
    ClaimError, DeterminationCatalog, DeterminationPresentationCheckError,
    DeterminationPresentationError, DeterminationPresentationRef, RawReturnError, RawReturnRef,
    RelationCatalog, RelationCheckError, RelationError, RelationRef, RelationSchema, RelationUse,
    RelationUseCheckError, RelationUseError, RelationUseRef, ScopeRef, SupportRef,
};
use crate::{ArtifactError, ArtifactRef};

/// Canonical artifact kind for candidate support environments.
pub const SUPPORT_ENVIRONMENT_ARTIFACT_KIND: &str = "ic.support-environment";
/// Payload schema version for candidate support environments.
pub const SUPPORT_ENVIRONMENT_SCHEMA_VERSION: u32 = 1;

/// The identity of one claim whose standing may be at issue.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ClaimRef(ArtifactRef);

impl ClaimRef {
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }

    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}

impl fmt::Display for ClaimRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for ClaimRef {
    type Err = ArtifactError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// Stable identity for one canonical support-environment record.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SupportEnvironmentRef(ArtifactRef);

impl SupportEnvironmentRef {
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }
    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
    /// The existing context-field spelling for this support environment identity.
    #[must_use]
    pub const fn as_support_ref(self) -> SupportRef {
        SupportRef::from_artifact_ref(self.0)
    }
}

impl fmt::Display for SupportEnvironmentRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for SupportEnvironmentRef {
    type Err = ArtifactError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// The canonical target of a support environment.
///
/// A relation target remains distinct from a claim target even when their artifact digests happen
/// to be otherwise comparable. The environment itself supplies scope and applicability for a
/// relation target, because a relation schema does not carry a claim-local context.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SupportSubjectRef {
    Claim(ClaimRef),
    Relation(RelationRef),
}

impl SupportSubjectRef {
    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        match self {
            Self::Claim(reference) => reference.as_artifact_ref(),
            Self::Relation(reference) => reference.as_artifact_ref(),
        }
    }

    const fn tag(self) -> u8 {
        match self {
            Self::Claim(_) => 0,
            Self::Relation(_) => 1,
        }
    }

    fn from_tag_and_reference(
        tag: u8,
        reference: ArtifactRef,
    ) -> Result<Self, SupportEnvironmentArtifactError> {
        match tag {
            0 => Ok(Self::Claim(ClaimRef::from_artifact_ref(reference))),
            1 => Ok(Self::Relation(RelationRef::from_artifact_ref(reference))),
            _ => Err(SupportEnvironmentArtifactError::UnknownTargetTag(tag)),
        }
    }
}

/// A content-addressed candidate support route for one claim.
///
/// It preserves each supplied component of a support environment but has no closure bit: whether
/// applicable conditions hold, checks succeeded, or inconsistency invalidates the route remains a
/// separate checking/admission question.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupportEnvironmentArtifact {
    target: SupportSubjectRef,
    premises: Vec<ArtifactRef>,
    actual_returns: Vec<RawReturnRef>,
    checkers: Vec<ArtifactRef>,
    assumptions: Vec<ArtifactRef>,
    open_dependencies: Vec<ArtifactRef>,
    applicability: ApplicabilityRef,
    scope: ScopeRef,
}

impl SupportEnvironmentArtifact {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        target: SupportSubjectRef,
        mut premises: Vec<ArtifactRef>,
        mut actual_returns: Vec<RawReturnRef>,
        mut checkers: Vec<ArtifactRef>,
        mut assumptions: Vec<ArtifactRef>,
        mut open_dependencies: Vec<ArtifactRef>,
        applicability: ApplicabilityRef,
        scope: ScopeRef,
    ) -> Result<Self, SupportEnvironmentArtifactError> {
        canonicalize(
            &mut premises,
            SupportEnvironmentArtifactError::DuplicatePremise,
        )?;
        canonicalize(
            &mut actual_returns,
            SupportEnvironmentArtifactError::DuplicateActualReturn,
        )?;
        canonicalize(
            &mut checkers,
            SupportEnvironmentArtifactError::DuplicateChecker,
        )?;
        canonicalize(
            &mut assumptions,
            SupportEnvironmentArtifactError::DuplicateAssumption,
        )?;
        canonicalize(
            &mut open_dependencies,
            SupportEnvironmentArtifactError::DuplicateOpenDependency,
        )?;
        Ok(Self {
            target,
            premises,
            actual_returns,
            checkers,
            assumptions,
            open_dependencies,
            applicability,
            scope,
        })
    }

    #[must_use]
    pub const fn target(&self) -> SupportSubjectRef {
        self.target
    }
    #[must_use]
    pub fn premises(&self) -> &[ArtifactRef] {
        &self.premises
    }
    #[must_use]
    pub fn actual_returns(&self) -> &[RawReturnRef] {
        &self.actual_returns
    }
    #[must_use]
    pub fn checkers(&self) -> &[ArtifactRef] {
        &self.checkers
    }
    #[must_use]
    pub fn assumptions(&self) -> &[ArtifactRef] {
        &self.assumptions
    }
    #[must_use]
    pub fn open_dependencies(&self) -> &[ArtifactRef] {
        &self.open_dependencies
    }
    #[must_use]
    pub const fn applicability(&self) -> ApplicabilityRef {
        self.applicability
    }
    #[must_use]
    pub const fn scope(&self) -> ScopeRef {
        self.scope
    }

    pub fn canonical_payload(&self) -> Result<Vec<u8>, SupportEnvironmentArtifactError> {
        let mut encoded = Vec::new();
        encoded.push(self.target.tag());
        reference(&mut encoded, self.target.as_artifact_ref());
        encode_artifact_refs(&mut encoded, &self.premises)?;
        encode_raw_returns(&mut encoded, &self.actual_returns)?;
        encode_artifact_refs(&mut encoded, &self.checkers)?;
        encode_artifact_refs(&mut encoded, &self.assumptions)?;
        encode_artifact_refs(&mut encoded, &self.open_dependencies)?;
        reference(&mut encoded, self.applicability.as_artifact_ref());
        reference(&mut encoded, self.scope.as_artifact_ref());
        Ok(encoded)
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, SupportEnvironmentArtifactError> {
        let mut cursor = SupportCursor::new(payload);
        let target =
            SupportSubjectRef::from_tag_and_reference(cursor.byte()?, cursor.reference()?)?;
        let premises = cursor.artifact_refs()?;
        let actual_returns = cursor.raw_returns()?;
        let checkers = cursor.artifact_refs()?;
        let assumptions = cursor.artifact_refs()?;
        let open_dependencies = cursor.artifact_refs()?;
        let applicability = ApplicabilityRef::from_artifact_ref(cursor.reference()?);
        let scope = ScopeRef::from_artifact_ref(cursor.reference()?);
        if !cursor.finished() {
            return Err(SupportEnvironmentArtifactError::TrailingPayloadBytes(
                cursor.remaining(),
            ));
        }
        let environment = Self::new(
            target,
            premises.clone(),
            actual_returns.clone(),
            checkers.clone(),
            assumptions.clone(),
            open_dependencies.clone(),
            applicability,
            scope,
        )?;
        if environment.premises != premises
            || environment.actual_returns != actual_returns
            || environment.checkers != checkers
            || environment.assumptions != assumptions
            || environment.open_dependencies != open_dependencies
        {
            return Err(SupportEnvironmentArtifactError::NonCanonicalReferenceOrder);
        }
        Ok(environment)
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, SupportEnvironmentArtifactError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(SUPPORT_ENVIRONMENT_ARTIFACT_KIND)?,
            SUPPORT_ENVIRONMENT_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }

    pub fn support_environment_ref(
        &self,
    ) -> Result<SupportEnvironmentRef, SupportEnvironmentArtifactError> {
        Ok(SupportEnvironmentRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(
        envelope: &ArtifactEnvelope,
    ) -> Result<Self, SupportEnvironmentArtifactError> {
        if envelope.kind().as_str() != SUPPORT_ENVIRONMENT_ARTIFACT_KIND {
            return Err(SupportEnvironmentArtifactError::UnexpectedArtifactKind {
                expected: SUPPORT_ENVIRONMENT_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != SUPPORT_ENVIRONMENT_SCHEMA_VERSION {
            return Err(SupportEnvironmentArtifactError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    /// Revalidates target, claim-premise, and raw-return identities without closing this route.
    pub fn check<C: SupportEnvironmentCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), SupportEnvironmentArtifactCheckError> {
        match self.target {
            SupportSubjectRef::Claim(reference) => {
                let claim = checked_claim(catalog, reference)?;
                if claim.scope() != self.scope {
                    return Err(SupportEnvironmentArtifactCheckError::ClaimContextMismatch(
                        "scope",
                    ));
                }
                if claim.applicability() != self.applicability {
                    return Err(SupportEnvironmentArtifactCheckError::ClaimContextMismatch(
                        "applicability",
                    ));
                }
            }
            SupportSubjectRef::Relation(reference) => {
                checked_relation(catalog, reference)?;
            }
        }
        for raw_return_ref in &self.actual_returns {
            let raw_return = catalog.resolve_raw_return(*raw_return_ref).ok_or(
                SupportEnvironmentArtifactCheckError::UnresolvedRawReturn(*raw_return_ref),
            )?;
            let calculated = raw_return.raw_return_ref()?;
            if calculated != *raw_return_ref {
                return Err(
                    SupportEnvironmentArtifactCheckError::RawReturnIdentityMismatch {
                        reference: *raw_return_ref,
                        calculated,
                    },
                );
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = vec![self.target.as_artifact_ref()];
        references.extend(self.premises.iter().copied());
        references.extend(
            self.actual_returns
                .iter()
                .map(|reference_value| reference_value.as_artifact_ref()),
        );
        references.extend(self.checkers.iter().copied());
        references.extend(self.assumptions.iter().copied());
        references.extend(self.open_dependencies.iter().copied());
        references.push(self.applicability.as_artifact_ref());
        references.push(self.scope.as_artifact_ref());
        references
    }
}

/// The catalog boundary for canonical support-environment validation.
pub trait SupportEnvironmentCatalog: ClaimCatalog + RelationCatalog {
    fn resolve_claim(&self, reference: ClaimRef) -> Option<ClaimArtifact>;
    fn resolve_support_environment(
        &self,
        reference: SupportEnvironmentRef,
    ) -> Option<SupportEnvironmentArtifact>;
}

/// Resolves the ordinary relation-use occurrence that names a support environment.
pub trait RelationUseSupportCatalog: SupportEnvironmentCatalog {
    fn resolve_relation_use(&self, reference: RelationUseRef) -> Option<RelationUse>;
}

/// The catalog boundary needed to connect a determination presentation's declared support to a
/// claim-targeted support environment.
pub trait DeterminationSupportCatalog: SupportEnvironmentCatalog + DeterminationCatalog {}

impl<T> DeterminationSupportCatalog for T where T: SupportEnvironmentCatalog + DeterminationCatalog {}

/// A checked structural link from one determination presentation to its claim-targeted support.
///
/// The target claim is deliberately retained separately from the presentation source form: the
/// current canonical records contain no claim-to-form interpretation law. This link therefore
/// does not assert that the claim proposition denotes the form, that the claim is relevant to the
/// web, or that its support route is independently grounded.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedDeterminationSupport {
    presentation: DeterminationPresentationRef,
    environment: SupportEnvironmentRef,
    claim: ClaimRef,
}

impl ResolvedDeterminationSupport {
    #[must_use]
    pub const fn presentation(&self) -> DeterminationPresentationRef {
        self.presentation
    }
    #[must_use]
    pub const fn environment(&self) -> SupportEnvironmentRef {
        self.environment
    }
    #[must_use]
    pub const fn claim(&self) -> ClaimRef {
        self.claim
    }
}

/// Resolves the exact claim-targeted environment named by a determination presentation.
pub fn resolve_determination_presentation_support<C: DeterminationSupportCatalog>(
    presentation_ref: DeterminationPresentationRef,
    catalog: &C,
) -> Result<ResolvedDeterminationSupport, DeterminationSupportError> {
    let presentation = catalog
        .resolve_determination_presentation(presentation_ref)
        .ok_or(DeterminationSupportError::UnresolvedPresentation(
            presentation_ref,
        ))?;
    let calculated_presentation = presentation.determination_presentation_ref()?;
    if calculated_presentation != presentation_ref {
        return Err(DeterminationSupportError::PresentationIdentityMismatch {
            reference: presentation_ref,
            calculated: calculated_presentation,
        });
    }
    presentation.check(catalog)?;
    let environment_ref =
        SupportEnvironmentRef::from_artifact_ref(presentation.support().as_artifact_ref());
    let environment = catalog.resolve_support_environment(environment_ref).ok_or(
        DeterminationSupportError::UnresolvedEnvironment(environment_ref),
    )?;
    let calculated_environment = environment.support_environment_ref()?;
    if calculated_environment != environment_ref {
        return Err(DeterminationSupportError::EnvironmentIdentityMismatch {
            reference: environment_ref,
            calculated: calculated_environment,
        });
    }
    environment.check(catalog)?;
    let SupportSubjectRef::Claim(claim) = environment.target() else {
        return Err(DeterminationSupportError::RelationTargetIsNotDeterminationSupport);
    };
    if environment.scope() != presentation.scope() {
        return Err(DeterminationSupportError::ContextMismatch("scope"));
    }
    if environment.applicability() != presentation.applicability() {
        return Err(DeterminationSupportError::ContextMismatch("applicability"));
    }
    Ok(ResolvedDeterminationSupport {
        presentation: presentation_ref,
        environment: environment_ref,
        claim,
    })
}

/// Requires the presentation's exact target claim to occur in a declared least-fixed-point
/// standing result.
///
/// This is still relative to the declared support problem: it does not turn caller-declared
/// applicability/check/ingress conditions into independently established actuality or warrant.
pub fn standing_determination_presentation_support<C: DeterminationSupportCatalog>(
    presentation_ref: DeterminationPresentationRef,
    standing: &Standing,
    catalog: &C,
) -> Result<ResolvedDeterminationSupport, DeterminationSupportError> {
    let resolved = resolve_determination_presentation_support(presentation_ref, catalog)?;
    if !standing.contains(resolved.claim()) {
        return Err(DeterminationSupportError::ClaimIsNotStanding(
            resolved.claim(),
        ));
    }
    if !standing.closes_through(
        SupportSubjectRef::Claim(resolved.claim()),
        resolved.environment(),
    ) {
        return Err(DeterminationSupportError::EnvironmentDidNotClose(
            resolved.environment(),
        ));
    }
    Ok(resolved)
}

/// A derived checked association between one relation-use occurrence and its relation-targeted
/// support environment.
///
/// This proves only reference, target, scope, and applicability agreement. It is not closure,
/// standing admission, relation evaluation, checker success, or warrant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedRelationUseSupport {
    relation_use: RelationUseRef,
    environment: SupportEnvironmentRef,
}

impl ResolvedRelationUseSupport {
    #[must_use]
    pub const fn relation_use(&self) -> RelationUseRef {
        self.relation_use
    }
    #[must_use]
    pub const fn environment(&self) -> SupportEnvironmentRef {
        self.environment
    }
}

/// Resolves and checks a relation use's declared support identity.
pub fn resolve_relation_use_support<C: RelationUseSupportCatalog>(
    relation_use_ref: RelationUseRef,
    catalog: &C,
) -> Result<ResolvedRelationUseSupport, RelationUseSupportError> {
    let relation_use = catalog.resolve_relation_use(relation_use_ref).ok_or(
        RelationUseSupportError::UnresolvedRelationUse(relation_use_ref),
    )?;
    let calculated_use = relation_use.relation_use_ref()?;
    if calculated_use != relation_use_ref {
        return Err(RelationUseSupportError::RelationUseIdentityMismatch {
            reference: relation_use_ref,
            calculated: calculated_use,
        });
    }
    relation_use.check(catalog)?;
    let environment_ref =
        SupportEnvironmentRef::from_artifact_ref(relation_use.support().as_artifact_ref());
    let environment = catalog.resolve_support_environment(environment_ref).ok_or(
        RelationUseSupportError::UnresolvedEnvironment(environment_ref),
    )?;
    let calculated_environment = environment.support_environment_ref()?;
    if calculated_environment != environment_ref {
        return Err(RelationUseSupportError::EnvironmentIdentityMismatch {
            reference: environment_ref,
            calculated: calculated_environment,
        });
    }
    environment.check(catalog)?;
    let SupportSubjectRef::Relation(target_relation) = environment.target() else {
        return Err(RelationUseSupportError::ClaimTargetIsNotRelationUseSupport);
    };
    if target_relation != relation_use.relation() {
        return Err(RelationUseSupportError::TargetRelationMismatch {
            target: target_relation,
            relation_use: relation_use.relation(),
        });
    }
    if environment.scope() != relation_use.scope() {
        return Err(RelationUseSupportError::ContextMismatch("scope"));
    }
    if environment.applicability() != relation_use.applicability() {
        return Err(RelationUseSupportError::ContextMismatch("applicability"));
    }
    Ok(ResolvedRelationUseSupport {
        relation_use: relation_use_ref,
        environment: environment_ref,
    })
}

/// Requires the exact relation-targeted environment named by a relation use to close in one
/// declared least-fixed-point standing result.
///
/// This establishes standing of the represented relation through that exact route. It does not
/// evaluate the relation use, prove that its bound tuple occurs, independently discharge declared
/// closure inputs, or confer actuality or warrant.
pub fn standing_relation_use_support<C: RelationUseSupportCatalog>(
    relation_use_ref: RelationUseRef,
    standing: &Standing,
    catalog: &C,
) -> Result<ResolvedRelationUseSupport, RelationUseSupportError> {
    let resolved = resolve_relation_use_support(relation_use_ref, catalog)?;
    let environment = catalog
        .resolve_support_environment(resolved.environment())
        .ok_or(RelationUseSupportError::UnresolvedEnvironment(
            resolved.environment(),
        ))?;
    let SupportSubjectRef::Relation(relation) = environment.target() else {
        return Err(RelationUseSupportError::ClaimTargetIsNotRelationUseSupport);
    };
    if !standing.contains_relation(relation) {
        return Err(RelationUseSupportError::RelationIsNotStanding(relation));
    }
    if !standing.closes_through(environment.target(), resolved.environment()) {
        return Err(RelationUseSupportError::EnvironmentDidNotClose(
            resolved.environment(),
        ));
    }
    Ok(resolved)
}

fn checked_relation<C: SupportEnvironmentCatalog>(
    catalog: &C,
    reference_value: RelationRef,
) -> Result<RelationSchema, SupportEnvironmentArtifactCheckError> {
    let relation = catalog.resolve_relation_schema(reference_value).ok_or(
        SupportEnvironmentArtifactCheckError::UnresolvedRelation(reference_value),
    )?;
    let calculated = relation.relation_ref()?;
    if calculated != reference_value {
        return Err(
            SupportEnvironmentArtifactCheckError::RelationIdentityMismatch {
                reference: reference_value,
                calculated,
            },
        );
    }
    relation.check(catalog)?;
    Ok(relation)
}

fn checked_claim<C: SupportEnvironmentCatalog>(
    catalog: &C,
    reference_value: ClaimRef,
) -> Result<ClaimArtifact, SupportEnvironmentArtifactCheckError> {
    let claim = catalog.resolve_claim(reference_value).ok_or(
        SupportEnvironmentArtifactCheckError::UnresolvedClaim(reference_value),
    )?;
    let calculated = claim.claim_ref()?;
    if calculated != reference_value {
        return Err(
            SupportEnvironmentArtifactCheckError::ClaimIdentityMismatch {
                reference: reference_value,
                calculated,
            },
        );
    }
    claim.check(catalog)?;
    Ok(claim)
}

fn canonicalize<T: Copy + Ord>(
    values: &mut [T],
    error: impl FnOnce(T) -> SupportEnvironmentArtifactError,
) -> Result<(), SupportEnvironmentArtifactError> {
    values.sort_unstable();
    for pair in values.windows(2) {
        if pair[0] == pair[1] {
            return Err(error(pair[0]));
        }
    }
    Ok(())
}

fn reference(encoded: &mut Vec<u8>, value: ArtifactRef) {
    encoded.extend_from_slice(value.as_bytes());
}
fn count(encoded: &mut Vec<u8>, value: usize) -> Result<(), SupportEnvironmentArtifactError> {
    let value = u32::try_from(value)
        .map_err(|_| SupportEnvironmentArtifactError::CollectionTooLong(value))?;
    encoded.extend_from_slice(&value.to_be_bytes());
    Ok(())
}
fn encode_raw_returns(
    encoded: &mut Vec<u8>,
    values: &[RawReturnRef],
) -> Result<(), SupportEnvironmentArtifactError> {
    count(encoded, values.len())?;
    for value in values {
        reference(encoded, value.as_artifact_ref());
    }
    Ok(())
}
fn encode_artifact_refs(
    encoded: &mut Vec<u8>,
    values: &[ArtifactRef],
) -> Result<(), SupportEnvironmentArtifactError> {
    count(encoded, values.len())?;
    for value in values {
        reference(encoded, *value);
    }
    Ok(())
}

struct SupportCursor<'a> {
    bytes: &'a [u8],
    position: usize,
}
impl<'a> SupportCursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }
    fn take(&mut self, length: usize) -> Result<&'a [u8], SupportEnvironmentArtifactError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(SupportEnvironmentArtifactError::PayloadLengthOverflow)?;
        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(SupportEnvironmentArtifactError::TruncatedPayload)?;
        self.position = end;
        Ok(bytes)
    }
    fn byte(&mut self) -> Result<u8, SupportEnvironmentArtifactError> {
        Ok(self.take(1)?[0])
    }
    fn reference(&mut self) -> Result<ArtifactRef, SupportEnvironmentArtifactError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| SupportEnvironmentArtifactError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(bytes))
    }
    fn count(&mut self) -> Result<usize, SupportEnvironmentArtifactError> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| SupportEnvironmentArtifactError::TruncatedPayload)?;
        usize::try_from(u32::from_be_bytes(bytes))
            .map_err(|_| SupportEnvironmentArtifactError::PayloadLengthOverflow)
    }
    fn raw_returns(&mut self) -> Result<Vec<RawReturnRef>, SupportEnvironmentArtifactError> {
        (0..self.count()?)
            .map(|_| self.reference().map(RawReturnRef::from_artifact_ref))
            .collect()
    }
    fn artifact_refs(&mut self) -> Result<Vec<ArtifactRef>, SupportEnvironmentArtifactError> {
        (0..self.count()?).map(|_| self.reference()).collect()
    }
    const fn finished(&self) -> bool {
        self.position == self.bytes.len()
    }
    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

#[derive(Debug, Error)]
pub enum SupportEnvironmentArtifactError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("support-environment collection is too long: {0} entries")]
    CollectionTooLong(usize),
    #[error("support environment repeats premise reference {0}")]
    DuplicatePremise(ArtifactRef),
    #[error("support environment repeats actual return {0}")]
    DuplicateActualReturn(RawReturnRef),
    #[error("support environment repeats checker reference {0}")]
    DuplicateChecker(ArtifactRef),
    #[error("support environment repeats assumption reference {0}")]
    DuplicateAssumption(ArtifactRef),
    #[error("support environment repeats open dependency {0}")]
    DuplicateOpenDependency(ArtifactRef),
    #[error("support-environment payload is truncated")]
    TruncatedPayload,
    #[error("support-environment payload has an unknown target tag {0}")]
    UnknownTargetTag(u8),
    #[error("support-environment payload length overflows this platform")]
    PayloadLengthOverflow,
    #[error("support-environment payload has {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("support-environment payload is not in canonical reference order")]
    NonCanonicalReferenceOrder,
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported support-environment schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

#[derive(Debug, Error)]
pub enum SupportEnvironmentArtifactCheckError {
    #[error(transparent)]
    Environment(#[from] SupportEnvironmentArtifactError),
    #[error(transparent)]
    Claim(#[from] ClaimError),
    #[error(transparent)]
    ClaimCheck(#[from] ClaimCheckError),
    #[error(transparent)]
    Relation(#[from] RelationError),
    #[error(transparent)]
    RelationCheck(#[from] RelationCheckError),
    #[error(transparent)]
    RawReturn(#[from] RawReturnError),
    #[error("claim {0} is unavailable")]
    UnresolvedClaim(ClaimRef),
    #[error("claim {reference} hashes to {calculated}, not its claimed identity")]
    ClaimIdentityMismatch {
        reference: ClaimRef,
        calculated: ClaimRef,
    },
    #[error("support environment context differs from its target claim at {0}")]
    ClaimContextMismatch(&'static str),
    #[error("relation {0} is unavailable")]
    UnresolvedRelation(RelationRef),
    #[error("relation {reference} hashes to {calculated}, not its claimed identity")]
    RelationIdentityMismatch {
        reference: RelationRef,
        calculated: RelationRef,
    },
    #[error("actual return {0} is unavailable")]
    UnresolvedRawReturn(RawReturnRef),
    #[error("actual return {reference} hashes to {calculated}, not its claimed identity")]
    RawReturnIdentityMismatch {
        reference: RawReturnRef,
        calculated: RawReturnRef,
    },
}

#[derive(Debug, Error)]
pub enum RelationUseSupportError {
    #[error(transparent)]
    Environment(#[from] SupportEnvironmentArtifactError),
    #[error(transparent)]
    EnvironmentCheck(#[from] SupportEnvironmentArtifactCheckError),
    #[error(transparent)]
    RelationUse(#[from] RelationUseError),
    #[error(transparent)]
    RelationUseCheck(#[from] RelationUseCheckError),
    #[error("relation use {0} is unavailable")]
    UnresolvedRelationUse(RelationUseRef),
    #[error("relation use {reference} hashes to {calculated}, not its claimed identity")]
    RelationUseIdentityMismatch {
        reference: RelationUseRef,
        calculated: RelationUseRef,
    },
    #[error("support environment {0} is unavailable")]
    UnresolvedEnvironment(SupportEnvironmentRef),
    #[error("support environment {reference} hashes to {calculated}, not its claimed identity")]
    EnvironmentIdentityMismatch {
        reference: SupportEnvironmentRef,
        calculated: SupportEnvironmentRef,
    },
    #[error("a claim-targeted support environment cannot yet support a relation-use association")]
    ClaimTargetIsNotRelationUseSupport,
    #[error("support target relation {target} differs from relation use relation {relation_use}")]
    TargetRelationMismatch {
        target: RelationRef,
        relation_use: RelationRef,
    },
    #[error("support environment context differs from relation use at {0}")]
    ContextMismatch(&'static str),
    #[error("relation {0} is absent from the declared standing result")]
    RelationIsNotStanding(RelationRef),
    #[error("support environment {0} did not close in the declared standing result")]
    EnvironmentDidNotClose(SupportEnvironmentRef),
}

#[derive(Debug, Error)]
pub enum DeterminationSupportError {
    #[error(transparent)]
    Determination(#[from] DeterminationPresentationError),
    #[error(transparent)]
    DeterminationCheck(#[from] DeterminationPresentationCheckError),
    #[error(transparent)]
    Environment(#[from] SupportEnvironmentArtifactError),
    #[error(transparent)]
    EnvironmentCheck(#[from] SupportEnvironmentArtifactCheckError),
    #[error("determination presentation {0} is unavailable")]
    UnresolvedPresentation(DeterminationPresentationRef),
    #[error(
        "determination presentation {reference} hashes to {calculated}, not its claimed identity"
    )]
    PresentationIdentityMismatch {
        reference: DeterminationPresentationRef,
        calculated: DeterminationPresentationRef,
    },
    #[error("support environment {0} is unavailable")]
    UnresolvedEnvironment(SupportEnvironmentRef),
    #[error("support environment {reference} hashes to {calculated}, not its claimed identity")]
    EnvironmentIdentityMismatch {
        reference: SupportEnvironmentRef,
        calculated: SupportEnvironmentRef,
    },
    #[error("a relation-targeted support environment cannot support a determination presentation")]
    RelationTargetIsNotDeterminationSupport,
    #[error("support environment context differs from determination presentation at {0}")]
    ContextMismatch(&'static str),
    #[error("claim {0} is absent from the declared standing result")]
    ClaimIsNotStanding(ClaimRef),
    #[error("support environment {0} did not close in the declared standing result")]
    EnvironmentDidNotClose(SupportEnvironmentRef),
}

/// One explicitly declared assessment of the closure conditions not evaluable by this phase.
///
/// The assessment is intentionally separate from `SupportEnvironmentArtifact`: it can drive a
/// derived standing computation only after the artifact identities are checked, but it neither
/// proves applicability/check success nor promotes itself to a warrant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclaredSupportClosure {
    environment: SupportEnvironmentRef,
    standing_premises: BTreeSet<ClaimRef>,
    standing_subjects: BTreeSet<SupportSubjectRef>,
    applicable: bool,
    checks_discharged: bool,
    invalidated: bool,
}

impl DeclaredSupportClosure {
    #[must_use]
    pub fn new(
        environment: SupportEnvironmentRef,
        standing_premises: Vec<ClaimRef>,
        applicable: bool,
        checks_discharged: bool,
        invalidated: bool,
    ) -> Self {
        let standing_premises: BTreeSet<_> = standing_premises.into_iter().collect();
        let standing_subjects = standing_premises
            .iter()
            .copied()
            .map(SupportSubjectRef::Claim)
            .collect();
        Self {
            environment,
            standing_premises,
            standing_subjects,
            applicable,
            checks_discharged,
            invalidated,
        }
    }

    /// Declares closure over typed claim and relation subjects without collapsing their kinds.
    #[must_use]
    pub fn for_subjects(
        environment: SupportEnvironmentRef,
        standing_subjects: Vec<SupportSubjectRef>,
        applicable: bool,
        checks_discharged: bool,
        invalidated: bool,
    ) -> Self {
        let standing_subjects: BTreeSet<_> = standing_subjects.into_iter().collect();
        let standing_premises = standing_subjects
            .iter()
            .filter_map(|subject| match subject {
                SupportSubjectRef::Claim(claim) => Some(*claim),
                SupportSubjectRef::Relation(_) => None,
            })
            .collect();
        Self {
            environment,
            standing_premises,
            standing_subjects,
            applicable,
            checks_discharged,
            invalidated,
        }
    }

    #[must_use]
    pub const fn environment(&self) -> SupportEnvironmentRef {
        self.environment
    }
    #[must_use]
    pub const fn standing_premises(&self) -> &BTreeSet<ClaimRef> {
        &self.standing_premises
    }

    /// Returns every typed subject that must stand before this route closes.
    #[must_use]
    pub const fn standing_subjects(&self) -> &BTreeSet<SupportSubjectRef> {
        &self.standing_subjects
    }
}

/// Builds the existing least-fixed-point problem from checked canonical support records and
/// explicitly declared unevaluable closure conditions.
pub fn standing_from_declared_support<C: SupportEnvironmentCatalog>(
    ingress: Vec<ClaimRef>,
    closures: &[DeclaredSupportClosure],
    catalog: &C,
) -> Result<Standing, DeclaredStandingError> {
    standing_from_declared_subject_support(
        ingress.into_iter().map(SupportSubjectRef::Claim).collect(),
        closures,
        catalog,
    )
}

/// Builds a least-fixed-point standing result over typed claim and relation subjects.
///
/// Closure flags remain caller declarations. This function checks subject identities, exact
/// premise membership, and canonical route provenance before applying the shared fixed point.
pub fn standing_from_declared_subject_support<C: SupportEnvironmentCatalog>(
    ingress: Vec<SupportSubjectRef>,
    closures: &[DeclaredSupportClosure],
    catalog: &C,
) -> Result<Standing, DeclaredStandingError> {
    for subject in &ingress {
        check_support_subject(catalog, *subject)?;
    }
    let mut environments = Vec::with_capacity(closures.len());
    for closure in closures {
        let environment = catalog
            .resolve_support_environment(closure.environment)
            .ok_or(DeclaredStandingError::UnresolvedEnvironment(
                closure.environment,
            ))?;
        let calculated = environment.support_environment_ref()?;
        if calculated != closure.environment {
            return Err(DeclaredStandingError::EnvironmentIdentityMismatch {
                reference: closure.environment,
                calculated,
            });
        }
        environment.check(catalog)?;
        for premise in closure.standing_subjects() {
            check_support_subject(catalog, *premise)?;
            if !environment.premises().contains(&premise.as_artifact_ref()) {
                return Err(DeclaredStandingError::PremiseNotNamedByEnvironment {
                    environment: closure.environment,
                    premise: *premise,
                });
            }
        }
        let declared = SupportEnvironment::for_subjects(
            environment.target(),
            closure.standing_subjects().iter().copied().collect(),
        )
        .with_canonical_environment(closure.environment)
        .with_applicability(closure.applicable)
        .with_checks_discharged(closure.checks_discharged)
        .invalidated(closure.invalidated)
        .with_open_dependencies(environment.open_dependencies().to_vec());
        environments.push(declared);
    }
    Ok(standing(&StandingProblem::for_subjects(
        ingress,
        environments,
    )))
}

fn check_support_subject<C: SupportEnvironmentCatalog>(
    catalog: &C,
    subject: SupportSubjectRef,
) -> Result<(), SupportEnvironmentArtifactCheckError> {
    match subject {
        SupportSubjectRef::Claim(claim) => {
            checked_claim(catalog, claim)?;
        }
        SupportSubjectRef::Relation(relation) => {
            checked_relation(catalog, relation)?;
        }
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum DeclaredStandingError {
    #[error(transparent)]
    Environment(#[from] SupportEnvironmentArtifactError),
    #[error(transparent)]
    EnvironmentCheck(#[from] SupportEnvironmentArtifactCheckError),
    #[error("support environment {0} is unavailable")]
    UnresolvedEnvironment(SupportEnvironmentRef),
    #[error("support environment {reference} hashes to {calculated}, not its claimed identity")]
    EnvironmentIdentityMismatch {
        reference: SupportEnvironmentRef,
        calculated: SupportEnvironmentRef,
    },
    #[error("standing premise {premise:?} is not named by support environment {environment}")]
    PremiseNotNamedByEnvironment {
        environment: SupportEnvironmentRef,
        premise: SupportSubjectRef,
    },
}

/// One candidate support route for a typed claim or relation subject.
///
/// The specification's `Closed_X(E, lambda)` has five conditions. Two of them are decided here
/// against the standing set as it grows -- the premises requiring standing, and the emptiness of
/// the open dependency boundary. The other three are properties of the route itself that this
/// phase cannot evaluate: whether applicability and scope hold, whether the independent checks the
/// route requires actually succeeded, and whether an inconsistency policy invalidates the
/// environment. Those arrive as caller declarations.
///
/// Declaring them is not discharging them. A caller who marks an unchecked route as checked has
/// asserted something this engine will believe, exactly as it will believe a declared ingress; the
/// engine decides what follows from the declarations, not whether they are true.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupportEnvironment {
    target: SupportSubjectRef,
    premises: BTreeSet<SupportSubjectRef>,
    claim_premises: BTreeSet<ClaimRef>,
    canonical_environment: Option<SupportEnvironmentRef>,
    open_dependencies: BTreeSet<ArtifactRef>,
    applicable: bool,
    checks_discharged: bool,
    invalidated: bool,
}

impl SupportEnvironment {
    /// Declares one support route whose premises must themselves stand.
    #[must_use]
    pub fn new(claim: ClaimRef, premises: Vec<ClaimRef>) -> Self {
        let claim_premises: BTreeSet<_> = premises.into_iter().collect();
        Self {
            target: SupportSubjectRef::Claim(claim),
            premises: claim_premises
                .iter()
                .copied()
                .map(SupportSubjectRef::Claim)
                .collect(),
            claim_premises,
            canonical_environment: None,
            open_dependencies: BTreeSet::new(),
            applicable: true,
            checks_discharged: true,
            invalidated: false,
        }
    }

    /// Declares one typed support route over claim and relation premises.
    #[must_use]
    pub fn for_subjects(target: SupportSubjectRef, premises: Vec<SupportSubjectRef>) -> Self {
        let premises: BTreeSet<_> = premises.into_iter().collect();
        let claim_premises = premises
            .iter()
            .filter_map(|subject| match subject {
                SupportSubjectRef::Claim(claim) => Some(*claim),
                SupportSubjectRef::Relation(_) => None,
            })
            .collect();
        Self {
            target,
            premises,
            claim_premises,
            canonical_environment: None,
            open_dependencies: BTreeSet::new(),
            applicable: true,
            checks_discharged: true,
            invalidated: false,
        }
    }

    #[must_use]
    const fn with_canonical_environment(mut self, environment: SupportEnvironmentRef) -> Self {
        self.canonical_environment = Some(environment);
        self
    }

    /// Records dependencies the route requires but neither supplies nor independently discharges.
    ///
    /// A nonempty boundary is an open question, not a failure: the claim simply cannot close
    /// through this route while it stands open.
    #[must_use]
    pub fn with_open_dependencies(mut self, open: Vec<ArtifactRef>) -> Self {
        self.open_dependencies = open.into_iter().collect();
        self
    }

    /// Declares whether the route's applicability and scope conditions hold.
    #[must_use]
    pub const fn with_applicability(mut self, applicable: bool) -> Self {
        self.applicable = applicable;
        self
    }

    /// Declares whether the independent checks this route requires have succeeded.
    #[must_use]
    pub const fn with_checks_discharged(mut self, discharged: bool) -> Self {
        self.checks_discharged = discharged;
        self
    }

    /// Declares that an explicit inconsistency policy invalidates this route.
    #[must_use]
    pub const fn invalidated(mut self, invalidated: bool) -> Self {
        self.invalidated = invalidated;
        self
    }

    /// Returns the claim this route supports.
    ///
    /// # Panics
    ///
    /// Panics when called on a relation-targeted route. New subject-generic code should use
    /// [`Self::target`] instead; this accessor is retained for claim-only callers.
    #[must_use]
    pub const fn claim(&self) -> ClaimRef {
        match self.target {
            SupportSubjectRef::Claim(claim) => claim,
            SupportSubjectRef::Relation(_) => {
                panic!("relation-targeted support route has no claim target")
            }
        }
    }

    /// Returns the typed subject this route supports.
    #[must_use]
    pub const fn target(&self) -> SupportSubjectRef {
        self.target
    }

    /// Returns the premises that must themselves stand.
    #[must_use]
    pub const fn premises(&self) -> &BTreeSet<ClaimRef> {
        &self.claim_premises
    }

    /// Returns every typed premise that must itself stand.
    #[must_use]
    pub const fn subject_premises(&self) -> &BTreeSet<SupportSubjectRef> {
        &self.premises
    }

    /// Returns the open dependency boundary.
    #[must_use]
    pub const fn open_dependencies(&self) -> &BTreeSet<ArtifactRef> {
        &self.open_dependencies
    }

    /// Decides `Closed_X(E, lambda)` against a standing set.
    #[must_use]
    pub fn is_closed(&self, standing: &BTreeSet<SupportSubjectRef>) -> bool {
        !self.invalidated
            && self.applicable
            && self.checks_discharged
            && self.open_dependencies.is_empty()
            && self
                .premises
                .iter()
                .all(|premise| standing.contains(premise))
    }
}

/// The declared inputs to one standing computation.
///
/// `ingress` holds the grounded facts available independently of inference: preserved actual
/// returns, trusted configuration, accepted predecessor relations, checker axioms. Everything else
/// must earn its place through a closed route.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StandingProblem {
    ingress: BTreeSet<ClaimRef>,
    subjects: BTreeSet<SupportSubjectRef>,
    environments: Vec<SupportEnvironment>,
}

impl StandingProblem {
    #[must_use]
    pub fn new(ingress: Vec<ClaimRef>, environments: Vec<SupportEnvironment>) -> Self {
        let ingress: BTreeSet<_> = ingress.into_iter().collect();
        Self {
            subjects: ingress
                .iter()
                .copied()
                .map(SupportSubjectRef::Claim)
                .collect(),
            ingress,
            environments,
        }
    }

    /// Declares a standing problem over typed claim and relation ingress subjects.
    #[must_use]
    pub fn for_subjects(
        subjects: Vec<SupportSubjectRef>,
        environments: Vec<SupportEnvironment>,
    ) -> Self {
        let subjects: BTreeSet<_> = subjects.into_iter().collect();
        let ingress = subjects
            .iter()
            .filter_map(|subject| match subject {
                SupportSubjectRef::Claim(claim) => Some(*claim),
                SupportSubjectRef::Relation(_) => None,
            })
            .collect();
        Self {
            ingress,
            subjects,
            environments,
        }
    }

    /// Returns the grounded ingress.
    #[must_use]
    pub const fn ingress(&self) -> &BTreeSet<ClaimRef> {
        &self.ingress
    }

    /// Returns every typed grounded ingress subject.
    #[must_use]
    pub const fn ingress_subjects(&self) -> &BTreeSet<SupportSubjectRef> {
        &self.subjects
    }

    /// Returns every declared support route.
    #[must_use]
    pub fn environments(&self) -> &[SupportEnvironment] {
        &self.environments
    }
}

/// The result of the fixed-point computation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Standing {
    subjects: BTreeSet<SupportSubjectRef>,
    claims: BTreeSet<ClaimRef>,
    relations: BTreeSet<RelationRef>,
    admitted_by: BTreeMap<ClaimRef, usize>,
    admitted_subject_by: BTreeMap<SupportSubjectRef, usize>,
    closed_environments: BTreeMap<SupportSubjectRef, BTreeSet<SupportEnvironmentRef>>,
    rounds: usize,
}

impl Standing {
    /// Returns every claim that stands.
    #[must_use]
    pub const fn claims(&self) -> &BTreeSet<ClaimRef> {
        &self.claims
    }

    /// Returns every relation that stands.
    #[must_use]
    pub const fn relations(&self) -> &BTreeSet<RelationRef> {
        &self.relations
    }

    /// Returns every typed subject in the fixed point.
    #[must_use]
    pub const fn subjects(&self) -> &BTreeSet<SupportSubjectRef> {
        &self.subjects
    }

    /// Reports whether one claim stands.
    #[must_use]
    pub fn contains(&self, claim: ClaimRef) -> bool {
        self.claims.contains(&claim)
    }

    /// Reports whether one typed subject stands.
    #[must_use]
    pub fn contains_subject(&self, subject: SupportSubjectRef) -> bool {
        self.subjects.contains(&subject)
    }

    /// Reports whether one relation stands.
    #[must_use]
    pub fn contains_relation(&self, relation: RelationRef) -> bool {
        self.relations.contains(&relation)
    }

    /// Returns the index of the route that first admitted a claim, when one did.
    ///
    /// Ingress has no admitting route, so a grounded claim answers `None`. This is provenance for
    /// reading the result, not a claim that the route is the only one that would have worked.
    #[must_use]
    pub fn admitted_by(&self, claim: ClaimRef) -> Option<usize> {
        self.admitted_by.get(&claim).copied()
    }

    /// Returns the index of the route that first admitted a typed subject.
    #[must_use]
    pub fn subject_admitted_by(&self, subject: SupportSubjectRef) -> Option<usize> {
        self.admitted_subject_by.get(&subject).copied()
    }

    /// Returns the index of the route that first admitted a relation, when one did.
    #[must_use]
    pub fn relation_admitted_by(&self, relation: RelationRef) -> Option<usize> {
        self.subject_admitted_by(SupportSubjectRef::Relation(relation))
    }

    /// Reports whether an exact canonical support environment closes for a standing subject.
    #[must_use]
    pub fn closes_through(
        &self,
        subject: SupportSubjectRef,
        environment: SupportEnvironmentRef,
    ) -> bool {
        self.closed_environments
            .get(&subject)
            .is_some_and(|environments| environments.contains(&environment))
    }

    /// Returns how many iterations the fixed point took to close.
    #[must_use]
    pub const fn rounds(&self) -> usize {
        self.rounds
    }
}

/// Computes `Stand = mu T`, the least fixed point of the support operator.
///
/// Iteration starts from the empty set and adds only what a closed route already reaches, which is
/// what makes the result least rather than merely consistent. The distinction is the whole content
/// of the no-rootless-cycle theorem: a group of claims that support one another and nothing else
/// is a perfectly consistent set, so the *greatest* fixed point contains it. Starting from nothing
/// and growing, no member is ever reachable, and none is admitted.
///
/// Standing here follows from the declarations supplied. It does not check that an ingress fact is
/// grounded, that a declared check ran, or that an applicability condition holds.
#[must_use]
pub fn standing(problem: &StandingProblem) -> Standing {
    let mut subjects: BTreeSet<SupportSubjectRef> =
        problem.ingress_subjects().iter().copied().collect();
    let mut admitted_by = BTreeMap::new();
    let mut admitted_subject_by = BTreeMap::new();
    let mut rounds = 0;

    loop {
        rounds += 1;
        let mut grew = false;
        for (index, environment) in problem.environments().iter().enumerate() {
            if subjects.contains(&environment.target()) {
                continue;
            }
            if environment.is_closed(&subjects) {
                subjects.insert(environment.target());
                admitted_subject_by.insert(environment.target(), index);
                if let SupportSubjectRef::Claim(claim) = environment.target() {
                    admitted_by.insert(claim, index);
                }
                grew = true;
            }
        }
        if !grew {
            break;
        }
    }

    let claims = subjects
        .iter()
        .filter_map(|subject| match subject {
            SupportSubjectRef::Claim(claim) => Some(*claim),
            SupportSubjectRef::Relation(_) => None,
        })
        .collect();
    let relations = subjects
        .iter()
        .filter_map(|subject| match subject {
            SupportSubjectRef::Claim(_) => None,
            SupportSubjectRef::Relation(relation) => Some(*relation),
        })
        .collect();
    let mut closed_environments: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
    for environment in problem.environments() {
        if subjects.contains(&environment.target())
            && environment.is_closed(&subjects)
            && let Some(reference) = environment.canonical_environment
        {
            closed_environments
                .entry(environment.target())
                .or_default()
                .insert(reference);
        }
    }

    Standing {
        subjects,
        claims,
        relations,
        admitted_by,
        admitted_subject_by,
        closed_environments,
        rounds,
    }
}

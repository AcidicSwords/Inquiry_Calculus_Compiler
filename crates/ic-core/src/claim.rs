//! Canonical claim identity before standing admission.
//!
//! A claim records its subject, source question, preserved raw-return references, resolution
//! paths, and declared context. Its lifecycle status is a claim about the record, never proof of
//! standing; the least-fixed-point standing engine remains the separate admission boundary.

use std::collections::BTreeSet;

use thiserror::Error;

use crate::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, ClaimRef,
    OpenQueryCatalog, OpenQueryCheckError, OpenQueryError, QueryRef, RawReturnCatalog,
    RawReturnError, RawReturnRef, ResolutionCatalog, ResolutionPathCheckError, ResolutionPathError,
    ResolutionPathRef, ScopeRef,
};

/// Canonical artifact kind for declared claim records.
pub const CLAIM_ARTIFACT_KIND: &str = "ic.claim";
/// Payload schema version for claim records.
pub const CLAIM_SCHEMA_VERSION: u32 = 1;

/// The declared lifecycle position of a claim record.
///
/// This field records a candidate status; it does not establish that a checker ran, that a claim
/// stands, or that any standing state has been revised.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaimStatus {
    Candidate,
    Checked,
    Standing,
    Rejected,
    Unknown,
}

impl ClaimStatus {
    const fn tag(self) -> u8 {
        match self {
            Self::Candidate => 0,
            Self::Checked => 1,
            Self::Standing => 2,
            Self::Rejected => 3,
            Self::Unknown => 4,
        }
    }

    const fn from_tag(tag: u8) -> Option<Self> {
        match tag {
            0 => Some(Self::Candidate),
            1 => Some(Self::Checked),
            2 => Some(Self::Standing),
            3 => Some(Self::Rejected),
            4 => Some(Self::Unknown),
            _ => None,
        }
    }
}

/// A content-addressed claim record with explicit candidate evidence provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaimArtifact {
    subject: ArtifactRef,
    source_question: QueryRef,
    supporting_returns: Vec<RawReturnRef>,
    resolution_paths: Vec<ResolutionPathRef>,
    scope: ScopeRef,
    applicability: ApplicabilityRef,
    status: ClaimStatus,
}

impl ClaimArtifact {
    /// Constructs a claim and canonically orders its independently named return/path references.
    pub fn new(
        subject: ArtifactRef,
        source_question: QueryRef,
        mut supporting_returns: Vec<RawReturnRef>,
        mut resolution_paths: Vec<ResolutionPathRef>,
        scope: ScopeRef,
        applicability: ApplicabilityRef,
        status: ClaimStatus,
    ) -> Result<Self, ClaimError> {
        supporting_returns.sort_unstable();
        if let Some(duplicate) = duplicate(&supporting_returns) {
            return Err(ClaimError::DuplicateSupportingReturn(duplicate));
        }
        resolution_paths.sort_unstable();
        if let Some(duplicate) = duplicate(&resolution_paths) {
            return Err(ClaimError::DuplicateResolutionPath(duplicate));
        }
        Ok(Self {
            subject,
            source_question,
            supporting_returns,
            resolution_paths,
            scope,
            applicability,
            status,
        })
    }

    #[must_use]
    pub const fn subject(&self) -> ArtifactRef {
        self.subject
    }
    #[must_use]
    pub const fn source_question(&self) -> QueryRef {
        self.source_question
    }
    #[must_use]
    pub fn supporting_returns(&self) -> &[RawReturnRef] {
        &self.supporting_returns
    }
    #[must_use]
    pub fn resolution_paths(&self) -> &[ResolutionPathRef] {
        &self.resolution_paths
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
    pub const fn status(&self) -> ClaimStatus {
        self.status
    }

    pub fn canonical_payload(&self) -> Result<Vec<u8>, ClaimError> {
        let mut encoded = Vec::new();
        reference(&mut encoded, self.subject);
        reference(&mut encoded, self.source_question.as_artifact_ref());
        count(&mut encoded, self.supporting_returns.len())?;
        for reference_value in &self.supporting_returns {
            reference(&mut encoded, reference_value.as_artifact_ref());
        }
        count(&mut encoded, self.resolution_paths.len())?;
        for reference_value in &self.resolution_paths {
            reference(&mut encoded, reference_value.as_artifact_ref());
        }
        reference(&mut encoded, self.scope.as_artifact_ref());
        reference(&mut encoded, self.applicability.as_artifact_ref());
        encoded.push(self.status.tag());
        Ok(encoded)
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, ClaimError> {
        let mut cursor = Cursor::new(payload);
        let subject = cursor.reference()?;
        let source_question = QueryRef::from_artifact_ref(cursor.reference()?);
        let return_count = cursor.count()?;
        let mut supporting_returns = Vec::with_capacity(return_count);
        for _ in 0..return_count {
            supporting_returns.push(RawReturnRef::from_artifact_ref(cursor.reference()?));
        }
        let path_count = cursor.count()?;
        let mut resolution_paths = Vec::with_capacity(path_count);
        for _ in 0..path_count {
            resolution_paths.push(ResolutionPathRef::from_artifact_ref(cursor.reference()?));
        }
        let scope = ScopeRef::from_artifact_ref(cursor.reference()?);
        let applicability = ApplicabilityRef::from_artifact_ref(cursor.reference()?);
        let status = ClaimStatus::from_tag(cursor.byte()?).ok_or(ClaimError::UnknownStatus)?;
        if !cursor.finished() {
            return Err(ClaimError::TrailingPayloadBytes(cursor.remaining()));
        }
        let claim = Self::new(
            subject,
            source_question,
            supporting_returns.clone(),
            resolution_paths.clone(),
            scope,
            applicability,
            status,
        )?;
        if claim.supporting_returns != supporting_returns
            || claim.resolution_paths != resolution_paths
        {
            return Err(ClaimError::NonCanonicalReferenceOrder);
        }
        Ok(claim)
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, ClaimError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(CLAIM_ARTIFACT_KIND)?,
            CLAIM_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }

    pub fn claim_ref(&self) -> Result<ClaimRef, ClaimError> {
        Ok(ClaimRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, ClaimError> {
        if envelope.kind().as_str() != CLAIM_ARTIFACT_KIND {
            return Err(ClaimError::UnexpectedArtifactKind {
                expected: CLAIM_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != CLAIM_SCHEMA_VERSION {
            return Err(ClaimError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    /// Revalidates every typed/provenance reference without treating the declared status as fact.
    pub fn check<C: ClaimCatalog>(&self, catalog: &C) -> Result<(), ClaimCheckError> {
        let query = catalog
            .resolve_open_query(self.source_question)
            .ok_or(ClaimCheckError::UnresolvedQuery(self.source_question))?;
        let calculated_query = query.query_ref()?;
        if calculated_query != self.source_question {
            return Err(ClaimCheckError::QueryIdentityMismatch {
                reference: self.source_question,
                calculated: calculated_query,
            });
        }
        query.check(catalog)?;
        for reference_value in &self.supporting_returns {
            let raw_return = catalog
                .resolve_raw_return(*reference_value)
                .ok_or(ClaimCheckError::UnresolvedRawReturn(*reference_value))?;
            let calculated = raw_return.raw_return_ref()?;
            if calculated != *reference_value {
                return Err(ClaimCheckError::RawReturnIdentityMismatch {
                    reference: *reference_value,
                    calculated,
                });
            }
        }
        for reference_value in &self.resolution_paths {
            let path = catalog
                .resolve_resolution_path(*reference_value)
                .ok_or(ClaimCheckError::UnresolvedResolutionPath(*reference_value))?;
            let calculated = path.resolution_path_ref()?;
            if calculated != *reference_value {
                return Err(ClaimCheckError::ResolutionPathIdentityMismatch {
                    reference: *reference_value,
                    calculated,
                });
            }
            path.check(catalog)?;
        }
        Ok(())
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = vec![self.subject, self.source_question.as_artifact_ref()];
        references.extend(
            self.supporting_returns
                .iter()
                .map(|reference_value| reference_value.as_artifact_ref()),
        );
        references.extend(
            self.resolution_paths
                .iter()
                .map(|reference_value| reference_value.as_artifact_ref()),
        );
        references.push(self.scope.as_artifact_ref());
        references.push(self.applicability.as_artifact_ref());
        references
    }
}

/// The catalog boundary used to structurally revalidate a claim record.
pub trait ClaimCatalog: OpenQueryCatalog + RawReturnCatalog + ResolutionCatalog {}

impl<T> ClaimCatalog for T where T: OpenQueryCatalog + RawReturnCatalog + ResolutionCatalog {}

fn duplicate<T: Copy + Ord>(references: &[T]) -> Option<T> {
    let mut seen = BTreeSet::new();
    references
        .iter()
        .find_map(|reference_value| (!seen.insert(*reference_value)).then_some(*reference_value))
}

fn reference(encoded: &mut Vec<u8>, value: ArtifactRef) {
    encoded.extend_from_slice(value.as_bytes());
}

fn count(encoded: &mut Vec<u8>, value: usize) -> Result<(), ClaimError> {
    let value = u32::try_from(value).map_err(|_| ClaimError::CollectionTooLong(value))?;
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
    fn take(&mut self, length: usize) -> Result<&'a [u8], ClaimError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(ClaimError::PayloadLengthOverflow)?;
        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(ClaimError::TruncatedPayload)?;
        self.position = end;
        Ok(bytes)
    }
    fn byte(&mut self) -> Result<u8, ClaimError> {
        Ok(self.take(1)?[0])
    }
    fn count(&mut self) -> Result<usize, ClaimError> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| ClaimError::TruncatedPayload)?;
        usize::try_from(u32::from_be_bytes(bytes)).map_err(|_| ClaimError::PayloadLengthOverflow)
    }
    fn reference(&mut self) -> Result<ArtifactRef, ClaimError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| ClaimError::TruncatedPayload)?;
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
pub enum ClaimError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("claim collection is too long: {0} entries")]
    CollectionTooLong(usize),
    #[error("claim repeats supporting raw return {0}")]
    DuplicateSupportingReturn(RawReturnRef),
    #[error("claim repeats resolution path {0}")]
    DuplicateResolutionPath(ResolutionPathRef),
    #[error("claim payload is truncated")]
    TruncatedPayload,
    #[error("claim payload length overflows this platform")]
    PayloadLengthOverflow,
    #[error("claim payload has {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("claim payload has an unknown lifecycle status")]
    UnknownStatus,
    #[error("claim payload is not in canonical reference order")]
    NonCanonicalReferenceOrder,
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported claim schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

#[derive(Debug, Error)]
pub enum ClaimCheckError {
    #[error(transparent)]
    Claim(#[from] ClaimError),
    #[error(transparent)]
    Query(#[from] OpenQueryError),
    #[error(transparent)]
    QueryCheck(#[from] OpenQueryCheckError),
    #[error(transparent)]
    RawReturn(#[from] RawReturnError),
    #[error(transparent)]
    ResolutionPath(#[from] ResolutionPathError),
    #[error(transparent)]
    ResolutionPathCheck(#[from] ResolutionPathCheckError),
    #[error("source question {0} is unavailable")]
    UnresolvedQuery(QueryRef),
    #[error("source question {reference} hashes to {calculated}, not its claimed identity")]
    QueryIdentityMismatch {
        reference: QueryRef,
        calculated: QueryRef,
    },
    #[error("supporting raw return {0} is unavailable")]
    UnresolvedRawReturn(RawReturnRef),
    #[error("raw return {reference} hashes to {calculated}, not its claimed identity")]
    RawReturnIdentityMismatch {
        reference: RawReturnRef,
        calculated: RawReturnRef,
    },
    #[error("resolution path {0} is unavailable")]
    UnresolvedResolutionPath(ResolutionPathRef),
    #[error("resolution path {reference} hashes to {calculated}, not its claimed identity")]
    ResolutionPathIdentityMismatch {
        reference: ResolutionPathRef,
        calculated: ResolutionPathRef,
    },
}

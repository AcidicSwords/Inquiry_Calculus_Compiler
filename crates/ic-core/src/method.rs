//! Canonical method-contract identities.
//!
//! A method contract records one typed traversal capability. It is not standing acceptance,
//! backend availability, execution, a raw return, or a semantic warrant.

use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, DischargeMode,
    RelationCatalog, RelationCheckError, RelationError, RelationRef,
};

/// Canonical artifact kind for method contracts.
pub const METHOD_CONTRACT_ARTIFACT_KIND: &str = "ic.method-contract";
/// Payload schema version for method contracts.
pub const METHOD_CONTRACT_SCHEMA_VERSION: u32 = 1;

macro_rules! artifact_reference {
    ($name:ident) => {
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

        impl From<$name> for ArtifactRef {
            fn from(value: $name) -> Self {
                value.0
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

artifact_reference!(MethodRef);
artifact_reference!(CoverageRef);
artifact_reference!(ExtensionDomainRef);
artifact_reference!(BackendRef);
artifact_reference!(CheckerRef);
artifact_reference!(CostModelRef);
artifact_reference!(ResidualSchemaRef);

/// A canonical registry contract for a native or learned method.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MethodContract {
    relation: RelationRef,
    applicability: ApplicabilityRef,
    law: ArtifactRef,
    coverage: CoverageRef,
    authority: DischargeMode,
    extension_domain: ExtensionDomainRef,
    backend: BackendRef,
    checker: Option<CheckerRef>,
    cost: Option<CostModelRef>,
    failure_schemas: Vec<ResidualSchemaRef>,
    provenance: Vec<ArtifactRef>,
}

impl MethodContract {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        relation: RelationRef,
        applicability: ApplicabilityRef,
        law: ArtifactRef,
        coverage: CoverageRef,
        authority: DischargeMode,
        extension_domain: ExtensionDomainRef,
        backend: BackendRef,
        checker: Option<CheckerRef>,
        cost: Option<CostModelRef>,
        mut failure_schemas: Vec<ResidualSchemaRef>,
        mut provenance: Vec<ArtifactRef>,
    ) -> Result<Self, MethodContractError> {
        canonicalize(
            &mut failure_schemas,
            MethodContractError::DuplicateFailureSchema,
        )?;
        canonicalize(&mut provenance, MethodContractError::DuplicateProvenance)?;
        Ok(Self {
            relation,
            applicability,
            law,
            coverage,
            authority,
            extension_domain,
            backend,
            checker,
            cost,
            failure_schemas,
            provenance,
        })
    }

    #[must_use]
    pub const fn relation(&self) -> RelationRef {
        self.relation
    }
    #[must_use]
    pub const fn applicability(&self) -> ApplicabilityRef {
        self.applicability
    }
    #[must_use]
    pub const fn law(&self) -> ArtifactRef {
        self.law
    }
    #[must_use]
    pub const fn coverage(&self) -> CoverageRef {
        self.coverage
    }
    #[must_use]
    pub const fn authority(&self) -> DischargeMode {
        self.authority
    }
    #[must_use]
    pub const fn extension_domain(&self) -> ExtensionDomainRef {
        self.extension_domain
    }
    #[must_use]
    pub const fn backend(&self) -> BackendRef {
        self.backend
    }
    #[must_use]
    pub const fn checker(&self) -> Option<CheckerRef> {
        self.checker
    }
    #[must_use]
    pub const fn cost(&self) -> Option<CostModelRef> {
        self.cost
    }
    #[must_use]
    pub fn failure_schemas(&self) -> &[ResidualSchemaRef] {
        &self.failure_schemas
    }
    #[must_use]
    pub fn provenance(&self) -> &[ArtifactRef] {
        &self.provenance
    }

    pub fn canonical_payload(&self) -> Result<Vec<u8>, MethodContractError> {
        let mut encoded = Vec::new();
        reference(&mut encoded, self.relation.as_artifact_ref());
        reference(&mut encoded, self.applicability.as_artifact_ref());
        reference(&mut encoded, self.law);
        reference(&mut encoded, self.coverage.as_artifact_ref());
        encoded.push(self.authority.tag());
        reference(&mut encoded, self.extension_domain.as_artifact_ref());
        reference(&mut encoded, self.backend.as_artifact_ref());
        optional_reference(&mut encoded, self.checker.map(CheckerRef::as_artifact_ref));
        optional_reference(&mut encoded, self.cost.map(CostModelRef::as_artifact_ref));
        references(&mut encoded, &self.failure_schemas)?;
        references(&mut encoded, &self.provenance)?;
        Ok(encoded)
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, MethodContractError> {
        let mut cursor = Cursor::new(payload);
        let relation = RelationRef::from_artifact_ref(cursor.reference()?);
        let applicability = ApplicabilityRef::from_artifact_ref(cursor.reference()?);
        let law = cursor.reference()?;
        let coverage = CoverageRef::from_artifact_ref(cursor.reference()?);
        let authority =
            DischargeMode::from_tag(cursor.byte()?).ok_or(MethodContractError::UnknownAuthority)?;
        let extension_domain = ExtensionDomainRef::from_artifact_ref(cursor.reference()?);
        let backend = BackendRef::from_artifact_ref(cursor.reference()?);
        let checker = cursor
            .optional_reference()?
            .map(CheckerRef::from_artifact_ref);
        let cost = cursor
            .optional_reference()?
            .map(CostModelRef::from_artifact_ref);
        let failure_schemas: Vec<ResidualSchemaRef> = cursor
            .references()?
            .into_iter()
            .map(ResidualSchemaRef::from_artifact_ref)
            .collect();
        let provenance = cursor.references()?;
        if !cursor.finished() {
            return Err(MethodContractError::TrailingPayloadBytes(
                cursor.remaining(),
            ));
        }
        let contract = Self::new(
            relation,
            applicability,
            law,
            coverage,
            authority,
            extension_domain,
            backend,
            checker,
            cost,
            failure_schemas.clone(),
            provenance.clone(),
        )?;
        if contract.failure_schemas != failure_schemas || contract.provenance != provenance {
            return Err(MethodContractError::NonCanonicalReferenceOrder);
        }
        Ok(contract)
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, MethodContractError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(METHOD_CONTRACT_ARTIFACT_KIND)?,
            METHOD_CONTRACT_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }

    pub fn method_ref(&self) -> Result<MethodRef, MethodContractError> {
        Ok(MethodRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, MethodContractError> {
        if envelope.kind().as_str() != METHOD_CONTRACT_ARTIFACT_KIND {
            return Err(MethodContractError::UnexpectedArtifactKind {
                expected: METHOD_CONTRACT_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != METHOD_CONTRACT_SCHEMA_VERSION {
            return Err(MethodContractError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = vec![
            self.relation.as_artifact_ref(),
            self.applicability.as_artifact_ref(),
            self.law,
            self.coverage.as_artifact_ref(),
            self.extension_domain.as_artifact_ref(),
            self.backend.as_artifact_ref(),
        ];
        if let Some(checker) = self.checker {
            references.push(checker.as_artifact_ref());
        }
        if let Some(cost) = self.cost {
            references.push(cost.as_artifact_ref());
        }
        references.extend(
            self.failure_schemas
                .iter()
                .map(|value| value.as_artifact_ref()),
        );
        references.extend(self.provenance.iter().copied());
        references
    }

    /// Rechecks only the implemented relation's canonical typed schema.
    pub fn check<C: RelationCatalog>(&self, catalog: &C) -> Result<(), MethodContractCheckError> {
        let relation = catalog
            .resolve_relation_schema(self.relation)
            .ok_or(MethodContractCheckError::UnresolvedRelation(self.relation))?;
        let calculated = relation.relation_ref()?;
        if calculated != self.relation {
            return Err(MethodContractCheckError::RelationIdentityMismatch {
                reference: self.relation,
                calculated,
            });
        }
        relation.check(catalog)?;
        Ok(())
    }
}

fn canonicalize<T: Copy + Ord>(
    values: &mut [T],
    error: impl FnOnce(T) -> MethodContractError,
) -> Result<(), MethodContractError> {
    values.sort_unstable();
    if let Some(duplicate) = values
        .windows(2)
        .find_map(|pair| (pair[0] == pair[1]).then_some(pair[0]))
    {
        return Err(error(duplicate));
    }
    Ok(())
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

fn references<T: Copy + Into<ArtifactRef>>(
    encoded: &mut Vec<u8>,
    values: &[T],
) -> Result<(), MethodContractError> {
    let count = u32::try_from(values.len())
        .map_err(|_| MethodContractError::CollectionTooLong(values.len()))?;
    encoded.extend_from_slice(&count.to_be_bytes());
    for value in values {
        reference(encoded, (*value).into());
    }
    Ok(())
}

struct Cursor<'a> {
    payload: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    const fn new(payload: &'a [u8]) -> Self {
        Self {
            payload,
            position: 0,
        }
    }
    fn take(&mut self, length: usize) -> Result<&'a [u8], MethodContractError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(MethodContractError::PayloadLengthOverflow)?;
        let bytes = self
            .payload
            .get(self.position..end)
            .ok_or(MethodContractError::TruncatedPayload)?;
        self.position = end;
        Ok(bytes)
    }
    fn byte(&mut self) -> Result<u8, MethodContractError> {
        Ok(self.take(1)?[0])
    }
    fn reference(&mut self) -> Result<ArtifactRef, MethodContractError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| MethodContractError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(bytes))
    }
    fn optional_reference(&mut self) -> Result<Option<ArtifactRef>, MethodContractError> {
        match self.byte()? {
            0 => Ok(None),
            1 => self.reference().map(Some),
            tag => Err(MethodContractError::UnknownOptionalTag(tag)),
        }
    }
    fn references(&mut self) -> Result<Vec<ArtifactRef>, MethodContractError> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| MethodContractError::TruncatedPayload)?;
        let count = usize::try_from(u32::from_be_bytes(bytes))
            .map_err(|_| MethodContractError::PayloadLengthOverflow)?;
        (0..count).map(|_| self.reference()).collect()
    }
    const fn finished(&self) -> bool {
        self.position == self.payload.len()
    }
    const fn remaining(&self) -> usize {
        self.payload.len() - self.position
    }
}

#[derive(Debug, Error)]
pub enum MethodContractError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("method-contract collection is too long: {0} entries")]
    CollectionTooLong(usize),
    #[error("method contract repeats failure schema {0}")]
    DuplicateFailureSchema(ResidualSchemaRef),
    #[error("method contract repeats provenance reference {0}")]
    DuplicateProvenance(ArtifactRef),
    #[error("method-contract payload is truncated")]
    TruncatedPayload,
    #[error("method-contract payload length overflows this platform")]
    PayloadLengthOverflow,
    #[error("method-contract payload has {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("method-contract payload has unknown authority tag")]
    UnknownAuthority,
    #[error("method-contract payload has unknown optional tag {0}")]
    UnknownOptionalTag(u8),
    #[error("method-contract payload is not in canonical reference order")]
    NonCanonicalReferenceOrder,
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported method-contract schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

#[derive(Debug, Error)]
pub enum MethodContractCheckError {
    #[error(transparent)]
    Contract(#[from] MethodContractError),
    #[error(transparent)]
    Relation(#[from] RelationError),
    #[error(transparent)]
    RelationCheck(#[from] RelationCheckError),
    #[error("implemented relation {0} is unavailable")]
    UnresolvedRelation(RelationRef),
    #[error("implemented relation {reference} hashes to {calculated}, not its claimed identity")]
    RelationIdentityMismatch {
        reference: RelationRef,
        calculated: RelationRef,
    },
}

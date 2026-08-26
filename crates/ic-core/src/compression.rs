//! Canonical compression-licence identities.
//!
//! A licence retains the declared boundary for a future exact or directional approximate fold. It
//! does not itself prove regeneration, evaluate recovery, activate an unlock, or authorize a
//! compression.

use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::{ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, HorizonRef, ScopeRef};

/// Canonical artifact kind for compression licences.
pub const COMPRESSION_LICENSE_ARTIFACT_KIND: &str = "ic.compression-license";
/// Payload schema version for compression licences.
pub const COMPRESSION_LICENSE_SCHEMA_VERSION: u32 = 1;

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

artifact_reference!(CompressionLicenseRef);
artifact_reference!(FoldOrQuotientRef);
artifact_reference!(ProtectedContinuationRef);
artifact_reference!(RecoveryContractRef);
artifact_reference!(UnlockConditionRef);
artifact_reference!(DistortionContractRef);

/// Whether a licence claims exact preservation or a directionally specified approximation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompressionKind {
    Exact,
    Approximate {
        distortion_contract: DistortionContractRef,
    },
}

impl CompressionKind {
    const fn tag(self) -> u8 {
        match self {
            Self::Exact => 0,
            Self::Approximate { .. } => 1,
        }
    }
}

/// A canonical licence boundary for one quotient or fold.
///
/// It distinguishes an exact claim from an approximate claim with an explicit directional
/// distortion contract. The references name evidence, recovery, and unlock obligations but no
/// evaluator is supplied at this phase.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompressionLicense {
    folded: FoldOrQuotientRef,
    kind: CompressionKind,
    horizon: HorizonRef,
    continuations: Vec<ProtectedContinuationRef>,
    scope: ScopeRef,
    evidence: Vec<ArtifactRef>,
    residual: ArtifactRef,
    recovery: RecoveryContractRef,
    unlock_conditions: Vec<UnlockConditionRef>,
}

impl CompressionLicense {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        folded: FoldOrQuotientRef,
        kind: CompressionKind,
        horizon: HorizonRef,
        mut continuations: Vec<ProtectedContinuationRef>,
        scope: ScopeRef,
        mut evidence: Vec<ArtifactRef>,
        residual: ArtifactRef,
        recovery: RecoveryContractRef,
        mut unlock_conditions: Vec<UnlockConditionRef>,
    ) -> Result<Self, CompressionLicenseError> {
        canonicalize(
            &mut continuations,
            CompressionLicenseError::DuplicateContinuation,
        )?;
        canonicalize(&mut evidence, CompressionLicenseError::DuplicateEvidence)?;
        canonicalize(
            &mut unlock_conditions,
            CompressionLicenseError::DuplicateUnlockCondition,
        )?;
        Ok(Self {
            folded,
            kind,
            horizon,
            continuations,
            scope,
            evidence,
            residual,
            recovery,
            unlock_conditions,
        })
    }

    #[must_use]
    pub const fn folded(&self) -> FoldOrQuotientRef {
        self.folded
    }
    #[must_use]
    pub const fn kind(&self) -> CompressionKind {
        self.kind
    }
    #[must_use]
    pub const fn horizon(&self) -> HorizonRef {
        self.horizon
    }
    #[must_use]
    pub fn continuations(&self) -> &[ProtectedContinuationRef] {
        &self.continuations
    }
    #[must_use]
    pub const fn scope(&self) -> ScopeRef {
        self.scope
    }
    #[must_use]
    pub fn evidence(&self) -> &[ArtifactRef] {
        &self.evidence
    }
    #[must_use]
    pub const fn residual(&self) -> ArtifactRef {
        self.residual
    }
    #[must_use]
    pub const fn recovery(&self) -> RecoveryContractRef {
        self.recovery
    }
    #[must_use]
    pub fn unlock_conditions(&self) -> &[UnlockConditionRef] {
        &self.unlock_conditions
    }

    pub fn canonical_payload(&self) -> Result<Vec<u8>, CompressionLicenseError> {
        let mut encoded = Vec::new();
        reference(&mut encoded, self.folded.as_artifact_ref());
        encoded.push(self.kind.tag());
        if let CompressionKind::Approximate {
            distortion_contract,
        } = self.kind
        {
            reference(&mut encoded, distortion_contract.as_artifact_ref());
        }
        reference(&mut encoded, self.horizon.as_artifact_ref());
        references(&mut encoded, &self.continuations)?;
        reference(&mut encoded, self.scope.as_artifact_ref());
        references(&mut encoded, &self.evidence)?;
        reference(&mut encoded, self.residual);
        reference(&mut encoded, self.recovery.as_artifact_ref());
        references(&mut encoded, &self.unlock_conditions)?;
        Ok(encoded)
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, CompressionLicenseError> {
        let mut cursor = Cursor::new(payload);
        let folded = FoldOrQuotientRef::from_artifact_ref(cursor.reference()?);
        let kind = match cursor.byte()? {
            0 => CompressionKind::Exact,
            1 => CompressionKind::Approximate {
                distortion_contract: DistortionContractRef::from_artifact_ref(cursor.reference()?),
            },
            tag => return Err(CompressionLicenseError::UnknownKind(tag)),
        };
        let horizon = HorizonRef::from_artifact_ref(cursor.reference()?);
        let continuations: Vec<ProtectedContinuationRef> = cursor
            .references()?
            .into_iter()
            .map(ProtectedContinuationRef::from_artifact_ref)
            .collect();
        let scope = ScopeRef::from_artifact_ref(cursor.reference()?);
        let evidence = cursor.references()?;
        let residual = cursor.reference()?;
        let recovery = RecoveryContractRef::from_artifact_ref(cursor.reference()?);
        let unlock_conditions: Vec<UnlockConditionRef> = cursor
            .references()?
            .into_iter()
            .map(UnlockConditionRef::from_artifact_ref)
            .collect();
        if !cursor.finished() {
            return Err(CompressionLicenseError::TrailingPayloadBytes(
                cursor.remaining(),
            ));
        }
        let license = Self::new(
            folded,
            kind,
            horizon,
            continuations.clone(),
            scope,
            evidence.clone(),
            residual,
            recovery,
            unlock_conditions.clone(),
        )?;
        if license.continuations != continuations
            || license.evidence != evidence
            || license.unlock_conditions != unlock_conditions
        {
            return Err(CompressionLicenseError::NonCanonicalReferenceOrder);
        }
        Ok(license)
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, CompressionLicenseError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(COMPRESSION_LICENSE_ARTIFACT_KIND)?,
            COMPRESSION_LICENSE_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }

    pub fn compression_license_ref(
        &self,
    ) -> Result<CompressionLicenseRef, CompressionLicenseError> {
        Ok(CompressionLicenseRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, CompressionLicenseError> {
        if envelope.kind().as_str() != COMPRESSION_LICENSE_ARTIFACT_KIND {
            return Err(CompressionLicenseError::UnexpectedArtifactKind {
                expected: COMPRESSION_LICENSE_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != COMPRESSION_LICENSE_SCHEMA_VERSION {
            return Err(CompressionLicenseError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = vec![self.folded.as_artifact_ref()];
        if let CompressionKind::Approximate {
            distortion_contract,
        } = self.kind
        {
            references.push(distortion_contract.as_artifact_ref());
        }
        references.push(self.horizon.as_artifact_ref());
        references.extend(
            self.continuations
                .iter()
                .map(|value| value.as_artifact_ref()),
        );
        references.push(self.scope.as_artifact_ref());
        references.extend(self.evidence.iter().copied());
        references.push(self.residual);
        references.push(self.recovery.as_artifact_ref());
        references.extend(
            self.unlock_conditions
                .iter()
                .map(|value| value.as_artifact_ref()),
        );
        references
    }
}

fn canonicalize<T: Copy + Ord>(
    values: &mut [T],
    error: impl FnOnce(T) -> CompressionLicenseError,
) -> Result<(), CompressionLicenseError> {
    values.sort_unstable();
    if let Some(duplicate) = values
        .windows(2)
        .find_map(|pair| (pair[0] == pair[1]).then_some(pair[0]))
    {
        return Err(error(duplicate));
    }
    Ok(())
}

fn reference(encoded: &mut Vec<u8>, reference: ArtifactRef) {
    encoded.extend_from_slice(reference.as_bytes());
}

fn references<T: Copy + Into<ArtifactRef>>(
    encoded: &mut Vec<u8>,
    values: &[T],
) -> Result<(), CompressionLicenseError> {
    let count = u32::try_from(values.len())
        .map_err(|_| CompressionLicenseError::CollectionTooLong(values.len()))?;
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
    fn take(&mut self, length: usize) -> Result<&'a [u8], CompressionLicenseError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(CompressionLicenseError::PayloadLengthOverflow)?;
        let bytes = self
            .payload
            .get(self.position..end)
            .ok_or(CompressionLicenseError::TruncatedPayload)?;
        self.position = end;
        Ok(bytes)
    }
    fn byte(&mut self) -> Result<u8, CompressionLicenseError> {
        Ok(self.take(1)?[0])
    }
    fn reference(&mut self) -> Result<ArtifactRef, CompressionLicenseError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| CompressionLicenseError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(bytes))
    }
    fn references(&mut self) -> Result<Vec<ArtifactRef>, CompressionLicenseError> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| CompressionLicenseError::TruncatedPayload)?;
        let count = usize::try_from(u32::from_be_bytes(bytes))
            .map_err(|_| CompressionLicenseError::PayloadLengthOverflow)?;
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
pub enum CompressionLicenseError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("compression-licence collection is too long: {0} entries")]
    CollectionTooLong(usize),
    #[error("compression licence repeats protected continuation {0}")]
    DuplicateContinuation(ProtectedContinuationRef),
    #[error("compression licence repeats evidence reference {0}")]
    DuplicateEvidence(ArtifactRef),
    #[error("compression licence repeats unlock condition {0}")]
    DuplicateUnlockCondition(UnlockConditionRef),
    #[error("compression-licence payload is truncated")]
    TruncatedPayload,
    #[error("compression-licence payload length overflows this platform")]
    PayloadLengthOverflow,
    #[error("compression-licence payload has {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("compression-licence payload has unknown kind tag {0}")]
    UnknownKind(u8),
    #[error("compression-licence payload is not in canonical reference order")]
    NonCanonicalReferenceOrder,
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported compression-licence schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

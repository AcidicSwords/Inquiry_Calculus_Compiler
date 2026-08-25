use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::{ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef};

/// Canonical artifact kind for an immutable, undecoded raw external return.
pub const RAW_RETURN_ARTIFACT_KIND: &str = "ic.raw-return";
/// Payload schema version for immutable raw external returns.
pub const RAW_RETURN_SCHEMA_VERSION: u32 = 1;

/// Stable content identity for raw return bytes.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RawReturnRef(ArtifactRef);

impl RawReturnRef {
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }

    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}

impl fmt::Display for RawReturnRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for RawReturnRef {
    type Err = ArtifactError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// Exact, immutable bytes returned by an external probe before decoding or interpretation.
///
/// Possessing this artifact does not establish that an interaction was actualized. Phase 6 event
/// history supplies that occurrence/ledger assertion separately.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawReturn {
    bytes: Vec<u8>,
}

impl RawReturn {
    #[must_use]
    pub const fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The raw bytes are already the canonical payload; they are never decoded as JSON or another
    /// generic semantic format for identity purposes.
    #[must_use]
    pub fn canonical_payload(&self) -> &[u8] {
        self.bytes()
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, RawReturnError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(RAW_RETURN_ARTIFACT_KIND)?,
            RAW_RETURN_SCHEMA_VERSION,
            self.bytes.clone(),
        ))
    }

    pub fn raw_return_ref(&self) -> Result<RawReturnRef, RawReturnError> {
        Ok(RawReturnRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, RawReturnError> {
        if envelope.kind().as_str() != RAW_RETURN_ARTIFACT_KIND {
            return Err(RawReturnError::UnexpectedArtifactKind {
                expected: RAW_RETURN_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != RAW_RETURN_SCHEMA_VERSION {
            return Err(RawReturnError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Ok(Self::new(envelope.canonical_payload().to_vec()))
    }
}

#[derive(Debug, Error)]
pub enum RawReturnError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported raw-return schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

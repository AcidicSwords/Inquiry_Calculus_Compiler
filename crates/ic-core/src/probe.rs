//! Structural probe-operator identity.
//!
//! A [`ProbeOperator`] is compiled operator data. It is distinct from a backend request and a raw
//! return, and its construction neither dispatches code nor establishes actuality.

use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, BoundaryRef, QueryRef, TypeRef,
};

/// Canonical artifact kind for one compiled probe operator.
pub const PROBE_OPERATOR_ARTIFACT_KIND: &str = "ic.probe-operator";
/// Payload schema version for compiled probe operators.
pub const PROBE_OPERATOR_SCHEMA_VERSION: u32 = 1;

/// Stable identity for a compiled probe operator.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProbeOperatorRef(ArtifactRef);

impl ProbeOperatorRef {
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }
    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}

impl fmt::Display for ProbeOperatorRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for ProbeOperatorRef {
    type Err = ArtifactError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// A compiled operator reference bundle; every opaque compiler/backend field stays explicit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProbeOperator {
    query: QueryRef,
    boundary: BoundaryRef,
    active_view: ArtifactRef,
    backend: ArtifactRef,
    executable_code: ArtifactRef,
    return_type: TypeRef,
    decoder_contract: ArtifactRef,
    probe_contract: ArtifactRef,
    compiler_version: ArtifactRef,
}

impl ProbeOperator {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        query: QueryRef,
        boundary: BoundaryRef,
        active_view: ArtifactRef,
        backend: ArtifactRef,
        executable_code: ArtifactRef,
        return_type: TypeRef,
        decoder_contract: ArtifactRef,
        probe_contract: ArtifactRef,
        compiler_version: ArtifactRef,
    ) -> Self {
        Self {
            query,
            boundary,
            active_view,
            backend,
            executable_code,
            return_type,
            decoder_contract,
            probe_contract,
            compiler_version,
        }
    }

    #[must_use]
    pub const fn query(&self) -> QueryRef {
        self.query
    }
    #[must_use]
    pub const fn boundary(&self) -> BoundaryRef {
        self.boundary
    }
    #[must_use]
    pub const fn active_view(&self) -> ArtifactRef {
        self.active_view
    }
    #[must_use]
    pub const fn backend(&self) -> ArtifactRef {
        self.backend
    }
    #[must_use]
    pub const fn executable_code(&self) -> ArtifactRef {
        self.executable_code
    }
    #[must_use]
    pub const fn return_type(&self) -> TypeRef {
        self.return_type
    }
    #[must_use]
    pub const fn decoder_contract(&self) -> ArtifactRef {
        self.decoder_contract
    }
    #[must_use]
    pub const fn probe_contract(&self) -> ArtifactRef {
        self.probe_contract
    }
    #[must_use]
    pub const fn compiler_version(&self) -> ArtifactRef {
        self.compiler_version
    }

    #[must_use]
    pub fn canonical_payload(&self) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(32 * 9);
        for value in self.referenced_artifacts() {
            encoded.extend_from_slice(value.as_bytes());
        }
        encoded
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, ProbeOperatorError> {
        if payload.len() != 32 * 9 {
            return Err(ProbeOperatorError::WrongPayloadLength(payload.len()));
        }
        let reference = |index: usize| {
            let bytes: [u8; 32] = payload[index * 32..(index + 1) * 32]
                .try_into()
                .expect("payload length is checked before fixed-width parsing");
            ArtifactRef::from_bytes(bytes)
        };
        Ok(Self::new(
            QueryRef::from_artifact_ref(reference(0)),
            BoundaryRef::from_artifact_ref(reference(1)),
            reference(2),
            reference(3),
            reference(4),
            TypeRef::from_artifact_ref(reference(5)),
            reference(6),
            reference(7),
            reference(8),
        ))
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, ProbeOperatorError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(PROBE_OPERATOR_ARTIFACT_KIND)?,
            PROBE_OPERATOR_SCHEMA_VERSION,
            self.canonical_payload(),
        ))
    }

    pub fn probe_operator_ref(&self) -> Result<ProbeOperatorRef, ProbeOperatorError> {
        Ok(ProbeOperatorRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, ProbeOperatorError> {
        if envelope.kind().as_str() != PROBE_OPERATOR_ARTIFACT_KIND {
            return Err(ProbeOperatorError::UnexpectedArtifactKind {
                expected: PROBE_OPERATOR_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != PROBE_OPERATOR_SCHEMA_VERSION {
            return Err(ProbeOperatorError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> [ArtifactRef; 9] {
        [
            self.query.as_artifact_ref(),
            self.boundary.as_artifact_ref(),
            self.active_view,
            self.backend,
            self.executable_code,
            self.return_type.as_artifact_ref(),
            self.decoder_contract,
            self.probe_contract,
            self.compiler_version,
        ]
    }
}

#[derive(Debug, Error)]
pub enum ProbeOperatorError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("probe-operator payload has {0} bytes instead of 288")]
    WrongPayloadLength(usize),
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported probe-operator schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

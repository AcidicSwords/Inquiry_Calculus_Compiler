//! Canonical identity for a recurrent probe contract.
//!
//! This records the fields that determine a probe's protected identity. It does not decide
//! occurrence comparability, evaluate a bridge policy, render a request, or dispatch a probe.

use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef,
    BindingVersionRef, GrainRef, HorizonRef,
};

/// Canonical artifact kind for one recurrent probe contract.
pub const PROBE_CONTRACT_ARTIFACT_KIND: &str = "ic.probe-contract";
/// Payload schema version for recurrent probe contracts.
pub const PROBE_CONTRACT_SCHEMA_VERSION: u32 = 1;

/// Stable identity for a recurrent probe contract.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProbeContractRef(ArtifactRef);

impl ProbeContractRef {
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }

    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}

impl fmt::Display for ProbeContractRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for ProbeContractRef {
    type Err = ArtifactError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// The canonical identity fields of one recurrent probe contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProbeContract {
    relational_role: ArtifactRef,
    binding_version: BindingVersionRef,
    grain: GrainRef,
    applicability: ApplicabilityRef,
    comparator: ArtifactRef,
    protected_horizon: HorizonRef,
    decoder_version: ArtifactRef,
    bridge_policy: ArtifactRef,
}

impl ProbeContract {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        relational_role: ArtifactRef,
        binding_version: BindingVersionRef,
        grain: GrainRef,
        applicability: ApplicabilityRef,
        comparator: ArtifactRef,
        protected_horizon: HorizonRef,
        decoder_version: ArtifactRef,
        bridge_policy: ArtifactRef,
    ) -> Self {
        Self {
            relational_role,
            binding_version,
            grain,
            applicability,
            comparator,
            protected_horizon,
            decoder_version,
            bridge_policy,
        }
    }

    #[must_use]
    pub const fn relational_role(&self) -> ArtifactRef {
        self.relational_role
    }

    #[must_use]
    pub const fn binding_version(&self) -> BindingVersionRef {
        self.binding_version
    }

    #[must_use]
    pub const fn grain(&self) -> GrainRef {
        self.grain
    }

    #[must_use]
    pub const fn applicability(&self) -> ApplicabilityRef {
        self.applicability
    }

    #[must_use]
    pub const fn comparator(&self) -> ArtifactRef {
        self.comparator
    }

    #[must_use]
    pub const fn protected_horizon(&self) -> HorizonRef {
        self.protected_horizon
    }

    #[must_use]
    pub const fn decoder_version(&self) -> ArtifactRef {
        self.decoder_version
    }

    #[must_use]
    pub const fn bridge_policy(&self) -> ArtifactRef {
        self.bridge_policy
    }

    #[must_use]
    pub fn canonical_payload(&self) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(32 * 8);
        for reference in self.referenced_artifacts() {
            encoded.extend_from_slice(reference.as_bytes());
        }
        encoded
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, ProbeContractError> {
        if payload.len() != 32 * 8 {
            return Err(ProbeContractError::WrongPayloadLength(payload.len()));
        }
        let reference = |index: usize| {
            let bytes: [u8; 32] = payload[index * 32..(index + 1) * 32]
                .try_into()
                .expect("payload length is checked before fixed-width parsing");
            ArtifactRef::from_bytes(bytes)
        };
        Ok(Self::new(
            reference(0),
            BindingVersionRef::from_artifact_ref(reference(1)),
            GrainRef::from_artifact_ref(reference(2)),
            ApplicabilityRef::from_artifact_ref(reference(3)),
            reference(4),
            HorizonRef::from_artifact_ref(reference(5)),
            reference(6),
            reference(7),
        ))
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, ProbeContractError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(PROBE_CONTRACT_ARTIFACT_KIND)?,
            PROBE_CONTRACT_SCHEMA_VERSION,
            self.canonical_payload(),
        ))
    }

    pub fn probe_contract_ref(&self) -> Result<ProbeContractRef, ProbeContractError> {
        Ok(ProbeContractRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, ProbeContractError> {
        if envelope.kind().as_str() != PROBE_CONTRACT_ARTIFACT_KIND {
            return Err(ProbeContractError::UnexpectedArtifactKind {
                expected: PROBE_CONTRACT_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != PROBE_CONTRACT_SCHEMA_VERSION {
            return Err(ProbeContractError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> [ArtifactRef; 8] {
        [
            self.relational_role,
            self.binding_version.as_artifact_ref(),
            self.grain.as_artifact_ref(),
            self.applicability.as_artifact_ref(),
            self.comparator,
            self.protected_horizon.as_artifact_ref(),
            self.decoder_version,
            self.bridge_policy,
        ]
    }
}

#[derive(Debug, Error)]
pub enum ProbeContractError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("probe-contract payload has {0} bytes instead of 256")]
    WrongPayloadLength(usize),
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported probe-contract schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

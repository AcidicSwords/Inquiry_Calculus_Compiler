//! Canonical compiler/backend boundary identities.
//!
//! A rendered surface plan and a backend request are distinct from the compiled probe operator,
//! from operational dispatch preparation, and from a raw return. Construction and structural
//! checking perform no rendering, dispatch, actuality, decoding, or warrant.

use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, BoundaryRef, ProbeContractRef,
    ProbeOperator, ProbeOperatorError, ProbeOperatorRef, QueryRef,
};

pub const SURFACE_PLAN_ARTIFACT_KIND: &str = "ic.surface-plan";
pub const SURFACE_PLAN_SCHEMA_VERSION: u32 = 1;
pub const BACKEND_REQUEST_ARTIFACT_KIND: &str = "ic.backend-request";
pub const BACKEND_REQUEST_SCHEMA_VERSION: u32 = 1;

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

artifact_reference!(SurfacePlanRef);
artifact_reference!(BackendRequestRef);

/// Exact rendered surface data for one compiled operator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfacePlan {
    operator: ProbeOperatorRef,
    query: QueryRef,
    boundary: BoundaryRef,
    active_view: ArtifactRef,
    executable_code: ArtifactRef,
    probe_contract: ProbeContractRef,
    renderer_version: ArtifactRef,
    rendered_body: ArtifactRef,
}

impl SurfacePlan {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        operator: ProbeOperatorRef,
        query: QueryRef,
        boundary: BoundaryRef,
        active_view: ArtifactRef,
        executable_code: ArtifactRef,
        probe_contract: ProbeContractRef,
        renderer_version: ArtifactRef,
        rendered_body: ArtifactRef,
    ) -> Self {
        Self {
            operator,
            query,
            boundary,
            active_view,
            executable_code,
            probe_contract,
            renderer_version,
            rendered_body,
        }
    }

    #[must_use]
    pub const fn operator(&self) -> ProbeOperatorRef {
        self.operator
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
    pub const fn executable_code(&self) -> ArtifactRef {
        self.executable_code
    }
    #[must_use]
    pub const fn probe_contract(&self) -> ProbeContractRef {
        self.probe_contract
    }
    #[must_use]
    pub const fn renderer_version(&self) -> ArtifactRef {
        self.renderer_version
    }
    #[must_use]
    pub const fn rendered_body(&self) -> ArtifactRef {
        self.rendered_body
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> [ArtifactRef; 8] {
        [
            self.operator.as_artifact_ref(),
            self.query.as_artifact_ref(),
            self.boundary.as_artifact_ref(),
            self.active_view,
            self.executable_code,
            self.probe_contract.as_artifact_ref(),
            self.renderer_version,
            self.rendered_body,
        ]
    }

    #[must_use]
    pub fn canonical_payload(&self) -> Vec<u8> {
        encode_references(&self.referenced_artifacts())
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, BackendBoundaryError> {
        let references = decode_references::<8>(payload, "surface plan")?;
        Ok(Self::new(
            ProbeOperatorRef::from_artifact_ref(references[0]),
            QueryRef::from_artifact_ref(references[1]),
            BoundaryRef::from_artifact_ref(references[2]),
            references[3],
            references[4],
            ProbeContractRef::from_artifact_ref(references[5]),
            references[6],
            references[7],
        ))
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, BackendBoundaryError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(SURFACE_PLAN_ARTIFACT_KIND)?,
            SURFACE_PLAN_SCHEMA_VERSION,
            self.canonical_payload(),
        ))
    }

    pub fn surface_plan_ref(&self) -> Result<SurfacePlanRef, BackendBoundaryError> {
        Ok(SurfacePlanRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, BackendBoundaryError> {
        check_envelope(
            envelope,
            SURFACE_PLAN_ARTIFACT_KIND,
            SURFACE_PLAN_SCHEMA_VERSION,
        )?;
        Self::decode_payload(envelope.canonical_payload())
    }

    pub fn check<C: BackendBoundaryCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), BackendBoundaryCheckError> {
        let operator = resolve_operator(self.operator, catalog)?;
        require_equal(
            "query",
            self.query.as_artifact_ref(),
            operator.query().as_artifact_ref(),
        )?;
        require_equal(
            "boundary",
            self.boundary.as_artifact_ref(),
            operator.boundary().as_artifact_ref(),
        )?;
        require_equal("active view", self.active_view, operator.active_view())?;
        require_equal(
            "executable code",
            self.executable_code,
            operator.executable_code(),
        )?;
        require_equal(
            "probe contract",
            self.probe_contract.as_artifact_ref(),
            operator.probe_contract().as_artifact_ref(),
        )
    }
}

/// Exact provider-facing request data derived from one checked surface plan and operator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendRequest {
    operator: ProbeOperatorRef,
    surface_plan: SurfacePlanRef,
    query: QueryRef,
    boundary: BoundaryRef,
    backend: ArtifactRef,
    executable_code: ArtifactRef,
    compiler_version: ArtifactRef,
    backend_version: ArtifactRef,
    request_body: ArtifactRef,
}

impl BackendRequest {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        operator: ProbeOperatorRef,
        surface_plan: SurfacePlanRef,
        query: QueryRef,
        boundary: BoundaryRef,
        backend: ArtifactRef,
        executable_code: ArtifactRef,
        compiler_version: ArtifactRef,
        backend_version: ArtifactRef,
        request_body: ArtifactRef,
    ) -> Self {
        Self {
            operator,
            surface_plan,
            query,
            boundary,
            backend,
            executable_code,
            compiler_version,
            backend_version,
            request_body,
        }
    }

    #[must_use]
    pub const fn operator(&self) -> ProbeOperatorRef {
        self.operator
    }
    #[must_use]
    pub const fn surface_plan(&self) -> SurfacePlanRef {
        self.surface_plan
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
    pub const fn backend(&self) -> ArtifactRef {
        self.backend
    }
    #[must_use]
    pub const fn executable_code(&self) -> ArtifactRef {
        self.executable_code
    }
    #[must_use]
    pub const fn compiler_version(&self) -> ArtifactRef {
        self.compiler_version
    }
    #[must_use]
    pub const fn backend_version(&self) -> ArtifactRef {
        self.backend_version
    }
    #[must_use]
    pub const fn request_body(&self) -> ArtifactRef {
        self.request_body
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> [ArtifactRef; 9] {
        [
            self.operator.as_artifact_ref(),
            self.surface_plan.as_artifact_ref(),
            self.query.as_artifact_ref(),
            self.boundary.as_artifact_ref(),
            self.backend,
            self.executable_code,
            self.compiler_version,
            self.backend_version,
            self.request_body,
        ]
    }

    #[must_use]
    pub fn canonical_payload(&self) -> Vec<u8> {
        encode_references(&self.referenced_artifacts())
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, BackendBoundaryError> {
        let references = decode_references::<9>(payload, "backend request")?;
        Ok(Self::new(
            ProbeOperatorRef::from_artifact_ref(references[0]),
            SurfacePlanRef::from_artifact_ref(references[1]),
            QueryRef::from_artifact_ref(references[2]),
            BoundaryRef::from_artifact_ref(references[3]),
            references[4],
            references[5],
            references[6],
            references[7],
            references[8],
        ))
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, BackendBoundaryError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(BACKEND_REQUEST_ARTIFACT_KIND)?,
            BACKEND_REQUEST_SCHEMA_VERSION,
            self.canonical_payload(),
        ))
    }

    pub fn backend_request_ref(&self) -> Result<BackendRequestRef, BackendBoundaryError> {
        Ok(BackendRequestRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, BackendBoundaryError> {
        check_envelope(
            envelope,
            BACKEND_REQUEST_ARTIFACT_KIND,
            BACKEND_REQUEST_SCHEMA_VERSION,
        )?;
        Self::decode_payload(envelope.canonical_payload())
    }

    pub fn check<C: BackendBoundaryCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), BackendBoundaryCheckError> {
        let operator = resolve_operator(self.operator, catalog)?;
        let plan = catalog.resolve_surface_plan(self.surface_plan).ok_or(
            BackendBoundaryCheckError::UnresolvedSurfacePlan(self.surface_plan),
        )?;
        let calculated = plan.surface_plan_ref()?;
        if calculated != self.surface_plan {
            return Err(BackendBoundaryCheckError::SurfacePlanIdentityMismatch {
                reference: self.surface_plan,
                calculated,
            });
        }
        plan.check(catalog)?;
        require_equal(
            "surface-plan operator",
            plan.operator().as_artifact_ref(),
            self.operator.as_artifact_ref(),
        )?;
        require_equal(
            "query",
            self.query.as_artifact_ref(),
            operator.query().as_artifact_ref(),
        )?;
        require_equal(
            "boundary",
            self.boundary.as_artifact_ref(),
            operator.boundary().as_artifact_ref(),
        )?;
        require_equal("backend", self.backend, operator.backend())?;
        require_equal(
            "executable code",
            self.executable_code,
            operator.executable_code(),
        )?;
        require_equal(
            "compiler version",
            self.compiler_version,
            operator.compiler_version(),
        )
    }
}

pub trait BackendBoundaryCatalog {
    fn resolve_probe_operator(&self, reference: ProbeOperatorRef) -> Option<ProbeOperator>;
    fn resolve_surface_plan(&self, reference: SurfacePlanRef) -> Option<SurfacePlan>;
}

fn resolve_operator<C: BackendBoundaryCatalog>(
    reference: ProbeOperatorRef,
    catalog: &C,
) -> Result<ProbeOperator, BackendBoundaryCheckError> {
    let operator = catalog.resolve_probe_operator(reference).ok_or(
        BackendBoundaryCheckError::UnresolvedProbeOperator(reference),
    )?;
    let calculated = operator.probe_operator_ref()?;
    if calculated != reference {
        return Err(BackendBoundaryCheckError::ProbeOperatorIdentityMismatch {
            reference,
            calculated,
        });
    }
    Ok(operator)
}

fn require_equal(
    field: &'static str,
    actual: ArtifactRef,
    expected: ArtifactRef,
) -> Result<(), BackendBoundaryCheckError> {
    if actual == expected {
        Ok(())
    } else {
        Err(BackendBoundaryCheckError::OperatorFieldMismatch {
            field,
            expected,
            actual,
        })
    }
}

fn encode_references(references: &[ArtifactRef]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(references.len() * 32);
    for reference in references {
        encoded.extend_from_slice(reference.as_bytes());
    }
    encoded
}

fn decode_references<const N: usize>(
    payload: &[u8],
    record: &'static str,
) -> Result<[ArtifactRef; N], BackendBoundaryError> {
    if payload.len() != N * 32 {
        return Err(BackendBoundaryError::WrongPayloadLength {
            record,
            expected: N * 32,
            actual: payload.len(),
        });
    }
    let mut references = [ArtifactRef::from_bytes([0; 32]); N];
    for (index, reference) in references.iter_mut().enumerate() {
        let bytes: [u8; 32] = payload[index * 32..(index + 1) * 32]
            .try_into()
            .expect("payload length is checked before fixed-width parsing");
        *reference = ArtifactRef::from_bytes(bytes);
    }
    Ok(references)
}

fn check_envelope(
    envelope: &ArtifactEnvelope,
    expected_kind: &'static str,
    expected_schema: u32,
) -> Result<(), BackendBoundaryError> {
    if envelope.kind().as_str() != expected_kind {
        return Err(BackendBoundaryError::UnexpectedArtifactKind {
            expected: expected_kind,
            actual: envelope.kind().as_str().to_owned(),
        });
    }
    if envelope.schema_version() != expected_schema {
        return Err(BackendBoundaryError::UnsupportedSchemaVersion {
            record: expected_kind,
            actual: envelope.schema_version(),
        });
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum BackendBoundaryError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("{record} payload has {actual} bytes instead of {expected}")]
    WrongPayloadLength {
        record: &'static str,
        expected: usize,
        actual: usize,
    },
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported {record} schema version {actual}")]
    UnsupportedSchemaVersion { record: &'static str, actual: u32 },
}

#[derive(Debug, Error)]
pub enum BackendBoundaryCheckError {
    #[error(transparent)]
    Encoding(#[from] BackendBoundaryError),
    #[error(transparent)]
    ProbeOperator(#[from] ProbeOperatorError),
    #[error("probe operator {0} is unavailable")]
    UnresolvedProbeOperator(ProbeOperatorRef),
    #[error("probe operator {reference} hashes to {calculated}, not its claimed identity")]
    ProbeOperatorIdentityMismatch {
        reference: ProbeOperatorRef,
        calculated: ProbeOperatorRef,
    },
    #[error("surface plan {0} is unavailable")]
    UnresolvedSurfacePlan(SurfacePlanRef),
    #[error("surface plan {reference} hashes to {calculated}, not its claimed identity")]
    SurfacePlanIdentityMismatch {
        reference: SurfacePlanRef,
        calculated: SurfacePlanRef,
    },
    #[error("{field} is {actual}, but the compiled operator requires {expected}")]
    OperatorFieldMismatch {
        field: &'static str,
        expected: ArtifactRef,
        actual: ArtifactRef,
    },
}

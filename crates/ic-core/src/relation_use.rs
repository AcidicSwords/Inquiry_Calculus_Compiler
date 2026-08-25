use std::{collections::BTreeSet, fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, DischargeMode,
    FormulaCatalog, GrainRef, HorizonRef, RelationError, RelationRef, RelationSchema, ScopeRef,
    SupportRef, TypeError, TypeSymbol, TypedFormRef, WarrantRef,
};

/// Canonical artifact kind for a particular occurrence of a relation schema.
pub const RELATION_USE_ARTIFACT_KIND: &str = "ic.relation-use";
/// Payload schema version for relation uses.
pub const RELATION_USE_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RelationUseRef(ArtifactRef);

impl RelationUseRef {
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }

    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}

impl fmt::Display for RelationUseRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for RelationUseRef {
    type Err = ArtifactError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// A typed form supplied to one named relation port in a particular use.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PortBinding {
    port: TypeSymbol,
    value: TypedFormRef,
}

impl PortBinding {
    #[must_use]
    pub const fn new(port: TypeSymbol, value: TypedFormRef) -> Self {
        Self { port, value }
    }

    #[must_use]
    pub const fn port(&self) -> &TypeSymbol {
        &self.port
    }

    #[must_use]
    pub const fn value(&self) -> TypedFormRef {
        self.value
    }
}

/// An immutable, scoped occurrence of a relation schema.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationUse {
    relation: RelationRef,
    bindings: Vec<PortBinding>,
    scope: ScopeRef,
    applicability: ApplicabilityRef,
    grain: GrainRef,
    horizon: HorizonRef,
    mode: DischargeMode,
    support: SupportRef,
    warrant: Option<WarrantRef>,
}

/// The contextual support record carried by one relation occurrence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RelationUseContext {
    scope: ScopeRef,
    applicability: ApplicabilityRef,
    grain: GrainRef,
    horizon: HorizonRef,
    mode: DischargeMode,
    support: SupportRef,
    warrant: Option<WarrantRef>,
}

impl RelationUseContext {
    #[must_use]
    pub const fn new(
        scope: ScopeRef,
        applicability: ApplicabilityRef,
        grain: GrainRef,
        horizon: HorizonRef,
        mode: DischargeMode,
        support: SupportRef,
        warrant: Option<WarrantRef>,
    ) -> Self {
        Self {
            scope,
            applicability,
            grain,
            horizon,
            mode,
            support,
            warrant,
        }
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
    pub const fn horizon(&self) -> HorizonRef {
        self.horizon
    }
    #[must_use]
    pub const fn mode(&self) -> DischargeMode {
        self.mode
    }
    #[must_use]
    pub const fn support(&self) -> SupportRef {
        self.support
    }
    #[must_use]
    pub const fn warrant(&self) -> Option<WarrantRef> {
        self.warrant
    }
}

impl RelationUse {
    #[must_use]
    pub const fn new(
        relation: RelationRef,
        bindings: Vec<PortBinding>,
        context: RelationUseContext,
    ) -> Self {
        Self {
            relation,
            bindings,
            scope: context.scope,
            applicability: context.applicability,
            grain: context.grain,
            horizon: context.horizon,
            mode: context.mode,
            support: context.support,
            warrant: context.warrant,
        }
    }

    #[must_use]
    pub const fn relation(&self) -> RelationRef {
        self.relation
    }
    #[must_use]
    pub fn bindings(&self) -> &[PortBinding] {
        &self.bindings
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
    pub const fn horizon(&self) -> HorizonRef {
        self.horizon
    }
    #[must_use]
    pub const fn mode(&self) -> DischargeMode {
        self.mode
    }
    #[must_use]
    pub const fn support(&self) -> SupportRef {
        self.support
    }
    #[must_use]
    pub const fn warrant(&self) -> Option<WarrantRef> {
        self.warrant
    }

    pub fn canonical_payload(&self) -> Result<Vec<u8>, RelationUseError> {
        let mut encoded = Vec::new();
        reference(&mut encoded, self.relation.as_artifact_ref());
        count(&mut encoded, self.bindings.len())?;
        for binding in &self.bindings {
            text(&mut encoded, binding.port().as_str())?;
            reference(&mut encoded, binding.value().as_artifact_ref());
        }
        reference(&mut encoded, self.scope.as_artifact_ref());
        reference(&mut encoded, self.applicability.as_artifact_ref());
        reference(&mut encoded, self.grain.as_artifact_ref());
        reference(&mut encoded, self.horizon.as_artifact_ref());
        encoded.push(self.mode.tag());
        reference(&mut encoded, self.support.as_artifact_ref());
        match self.warrant {
            None => encoded.push(0),
            Some(warrant) => {
                encoded.push(1);
                reference(&mut encoded, warrant.as_artifact_ref());
            }
        }
        Ok(encoded)
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, RelationUseError> {
        let mut cursor = Cursor::new(payload);
        let relation = RelationRef::from_artifact_ref(cursor.reference()?);
        let binding_count = cursor.count()?;
        let mut bindings = Vec::with_capacity(binding_count);
        for _ in 0..binding_count {
            let port_text = cursor.text()?;
            let port = TypeSymbol::new(port_text.clone())
                .map_err(|_| RelationUseError::InvalidPortName(port_text))?;
            bindings.push(PortBinding::new(
                port,
                TypedFormRef::from_artifact_ref(cursor.reference()?),
            ));
        }
        let scope = ScopeRef::from_artifact_ref(cursor.reference()?);
        let applicability = ApplicabilityRef::from_artifact_ref(cursor.reference()?);
        let grain = GrainRef::from_artifact_ref(cursor.reference()?);
        let horizon = HorizonRef::from_artifact_ref(cursor.reference()?);
        let mode = DischargeMode::from_tag(cursor.byte()?).ok_or(RelationUseError::UnknownMode)?;
        let support = SupportRef::from_artifact_ref(cursor.reference()?);
        let warrant = match cursor.byte()? {
            0 => None,
            1 => Some(WarrantRef::from_artifact_ref(cursor.reference()?)),
            value => return Err(RelationUseError::UnknownOptionalTag(value)),
        };
        if !cursor.finished() {
            return Err(RelationUseError::TrailingPayloadBytes(cursor.remaining()));
        }
        Ok(Self::new(
            relation,
            bindings,
            RelationUseContext::new(scope, applicability, grain, horizon, mode, support, warrant),
        ))
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, RelationUseError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(RELATION_USE_ARTIFACT_KIND)?,
            RELATION_USE_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }
    pub fn relation_use_ref(&self) -> Result<RelationUseRef, RelationUseError> {
        Ok(RelationUseRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }
    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, RelationUseError> {
        if envelope.kind().as_str() != RELATION_USE_ARTIFACT_KIND {
            return Err(RelationUseError::UnexpectedArtifactKind {
                expected: RELATION_USE_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != RELATION_USE_SCHEMA_VERSION {
            return Err(RelationUseError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }
    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = vec![self.relation.as_artifact_ref()];
        references.extend(
            self.bindings
                .iter()
                .map(|binding| binding.value().as_artifact_ref()),
        );
        references.extend([
            self.scope.as_artifact_ref(),
            self.applicability.as_artifact_ref(),
            self.grain.as_artifact_ref(),
            self.horizon.as_artifact_ref(),
            self.support.as_artifact_ref(),
        ]);
        if let Some(warrant) = self.warrant {
            references.push(warrant.as_artifact_ref());
        }
        references
    }
    pub fn check<C: RelationCatalog>(&self, catalog: &C) -> Result<(), RelationUseCheckError> {
        let schema = catalog
            .resolve_relation_schema(self.relation)
            .ok_or(RelationUseCheckError::UnresolvedRelation(self.relation))?;
        if schema.relation_ref()? != self.relation {
            return Err(RelationUseCheckError::RelationReferenceIdentityMismatch {
                reference: self.relation,
                calculated: schema.relation_ref()?,
            });
        }
        schema.check(catalog)?;
        let mut names = BTreeSet::new();
        for binding in &self.bindings {
            if !names.insert(binding.port().clone()) {
                return Err(RelationUseCheckError::DuplicateBoundPort(
                    binding.port().clone(),
                ));
            }
            let expected = schema
                .ports()
                .iter()
                .find(|port| port.name() == binding.port())
                .ok_or_else(|| RelationUseCheckError::UnknownPort(binding.port().clone()))?;
            let form = catalog
                .resolve_typed_form(binding.value())
                .ok_or(RelationUseCheckError::UnresolvedTypedForm(binding.value()))?;
            if form.typed_form_ref()? != binding.value() {
                return Err(RelationUseCheckError::TypedFormReferenceIdentityMismatch {
                    reference: binding.value(),
                    calculated: form.typed_form_ref()?,
                });
            }
            if form.binding() != schema.binding() {
                return Err(RelationUseCheckError::TypedFormBindingMismatch {
                    expected: schema.binding(),
                    actual: form.binding(),
                });
            }
            if form.ty() != expected.ty() {
                return Err(RelationUseCheckError::PortTypeMismatch {
                    port: binding.port().clone(),
                    expected: expected.ty(),
                    actual: form.ty(),
                });
            }
        }
        Ok(())
    }
}

/// A catalog that can resolve relation-schema authority for a scoped use.
pub trait RelationCatalog: FormulaCatalog {
    fn resolve_relation_schema(&self, reference: RelationRef) -> Option<RelationSchema>;
}

fn reference(encoded: &mut Vec<u8>, value: ArtifactRef) {
    encoded.extend_from_slice(value.as_bytes());
}
fn count(encoded: &mut Vec<u8>, value: usize) -> Result<(), RelationUseError> {
    let value = u32::try_from(value).map_err(|_| RelationUseError::CollectionTooLong(value))?;
    encoded.extend_from_slice(&value.to_be_bytes());
    Ok(())
}
fn text(encoded: &mut Vec<u8>, value: &str) -> Result<(), RelationUseError> {
    count(encoded, value.len())?;
    encoded.extend_from_slice(value.as_bytes());
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
    fn take(&mut self, length: usize) -> Result<&'a [u8], RelationUseError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(RelationUseError::PayloadLengthOverflow)?;
        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(RelationUseError::TruncatedPayload)?;
        self.position = end;
        Ok(bytes)
    }
    fn byte(&mut self) -> Result<u8, RelationUseError> {
        Ok(self.take(1)?[0])
    }
    fn count(&mut self) -> Result<usize, RelationUseError> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| RelationUseError::TruncatedPayload)?;
        usize::try_from(u32::from_be_bytes(bytes))
            .map_err(|_| RelationUseError::PayloadLengthOverflow)
    }
    fn reference(&mut self) -> Result<ArtifactRef, RelationUseError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| RelationUseError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(bytes))
    }
    fn text(&mut self) -> Result<String, RelationUseError> {
        let length = self.count()?;
        String::from_utf8(self.take(length)?.to_vec())
            .map_err(RelationUseError::InvalidPortNameUtf8)
    }
    const fn finished(&self) -> bool {
        self.position == self.bytes.len()
    }
    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

#[derive(Debug, Error)]
pub enum RelationUseError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("relation-use collection is too long: {0} entries")]
    CollectionTooLong(usize),
    #[error("invalid relation-use port name {0:?}")]
    InvalidPortName(String),
    #[error("relation-use port name bytes are not valid UTF-8")]
    InvalidPortNameUtf8(#[source] std::string::FromUtf8Error),
    #[error("relation-use payload is truncated")]
    TruncatedPayload,
    #[error("relation-use payload length overflows this platform")]
    PayloadLengthOverflow,
    #[error("relation-use payload contains {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("relation-use payload contains unknown discharge mode")]
    UnknownMode,
    #[error("relation-use payload contains unknown optional tag {0}")]
    UnknownOptionalTag(u8),
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported relation-use schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

#[derive(Debug, Error)]
pub enum RelationUseCheckError {
    #[error(transparent)]
    Use(#[from] RelationUseError),
    #[error(transparent)]
    Schema(#[from] crate::RelationCheckError),

    #[error(transparent)]
    RelationArtifact(#[from] RelationError),
    #[error(transparent)]
    FormulaArtifact(#[from] crate::FormulaError),

    #[error(transparent)]
    TypeArtifact(#[from] TypeError),
    #[error("relation {0} is not available from the declared catalog")]
    UnresolvedRelation(RelationRef),
    #[error("catalog relation {reference} hashes to {calculated}, not its claimed identity")]
    RelationReferenceIdentityMismatch {
        reference: RelationRef,
        calculated: RelationRef,
    },
    #[error("relation use binds port {0} more than once")]
    DuplicateBoundPort(TypeSymbol),
    #[error("relation use names unknown port {0}")]
    UnknownPort(TypeSymbol),
    #[error("typed form {0} is not available from the declared catalog")]
    UnresolvedTypedForm(TypedFormRef),
    #[error("catalog typed form {reference} hashes to {calculated}, not its claimed identity")]
    TypedFormReferenceIdentityMismatch {
        reference: TypedFormRef,
        calculated: TypedFormRef,
    },
    #[error("typed form binding {actual} does not match relation binding {expected}")]
    TypedFormBindingMismatch {
        expected: crate::BindingVersionRef,
        actual: crate::BindingVersionRef,
    },
    #[error("bound port {port} has type {actual}, expected {expected}")]
    PortTypeMismatch {
        port: TypeSymbol,
        expected: crate::TypeRef,
        actual: crate::TypeRef,
    },
}

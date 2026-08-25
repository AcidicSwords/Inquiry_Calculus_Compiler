use std::{collections::BTreeSet, fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, DischargeMode, PortBinding,
    RelationCatalog, RelationRef, RelationUseContext, TypeError, TypeSymbol, TypedFormRef,
};

/// Canonical artifact kind for data-only open relation questions.
pub const OPEN_QUERY_ARTIFACT_KIND: &str = "ic.open-query";
/// Payload schema version for data-only open relation questions.
pub const OPEN_QUERY_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct QueryRef(ArtifactRef);

impl QueryRef {
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }
    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}

impl fmt::Display for QueryRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}
impl FromStr for QueryRef {
    type Err = ArtifactError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// A port retained as a question coordinate, with the evidence route required to discharge it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenPort {
    port: TypeSymbol,
    mode: DischargeMode,
}

impl OpenPort {
    #[must_use]
    pub const fn new(port: TypeSymbol, mode: DischargeMode) -> Self {
        Self { port, mode }
    }
    #[must_use]
    pub const fn port(&self) -> &TypeSymbol {
        &self.port
    }
    #[must_use]
    pub const fn mode(&self) -> DischargeMode {
        self.mode
    }
}

/// A canonical partially bound typed relation with a nonempty exposed port set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenQuery {
    relation: RelationRef,
    bound_ports: Vec<PortBinding>,
    open_ports: Vec<OpenPort>,
    context: RelationUseContext,
}

/// A complete typed filling for an OpenQuery.
///
/// This is a candidate completion only. It records no relation evaluation, support, actuality,
/// warrant, or membership in a completion fiber.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompletionCandidate {
    source: QueryRef,
    bindings: Vec<PortBinding>,
}

impl CompletionCandidate {
    #[must_use]
    pub const fn source(&self) -> QueryRef {
        self.source
    }

    #[must_use]
    pub fn bindings(&self) -> &[PortBinding] {
        &self.bindings
    }
}

impl OpenQuery {
    #[must_use]
    pub const fn new(
        relation: RelationRef,
        bound_ports: Vec<PortBinding>,
        open_ports: Vec<OpenPort>,
        context: RelationUseContext,
    ) -> Self {
        Self {
            relation,
            bound_ports,
            open_ports,
            context,
        }
    }

    #[must_use]
    pub const fn relation(&self) -> RelationRef {
        self.relation
    }
    #[must_use]
    pub fn bound_ports(&self) -> &[PortBinding] {
        &self.bound_ports
    }
    #[must_use]
    pub fn open_ports(&self) -> &[OpenPort] {
        &self.open_ports
    }
    #[must_use]
    pub const fn context(&self) -> &RelationUseContext {
        &self.context
    }

    pub fn canonical_payload(&self) -> Result<Vec<u8>, OpenQueryError> {
        let mut encoded = Vec::new();
        reference(&mut encoded, self.relation.as_artifact_ref());
        count(&mut encoded, self.bound_ports.len())?;
        for binding in &self.bound_ports {
            text(&mut encoded, binding.port().as_str())?;
            reference(&mut encoded, binding.value().as_artifact_ref());
        }
        count(&mut encoded, self.open_ports.len())?;
        for open in &self.open_ports {
            text(&mut encoded, open.port().as_str())?;
            encoded.push(open.mode().tag());
        }
        write_context(&mut encoded, &self.context);
        Ok(encoded)
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, OpenQueryError> {
        let mut cursor = Cursor::new(payload);
        let relation = RelationRef::from_artifact_ref(cursor.reference()?);
        let bound_count = cursor.count()?;
        let mut bound_ports = Vec::with_capacity(bound_count);
        for _ in 0..bound_count {
            bound_ports.push(PortBinding::new(
                cursor.port_name()?,
                TypedFormRef::from_artifact_ref(cursor.reference()?),
            ));
        }
        let open_count = cursor.count()?;
        let mut open_ports = Vec::with_capacity(open_count);
        for _ in 0..open_count {
            let port = cursor.port_name()?;
            let mode =
                DischargeMode::from_tag(cursor.byte()?).ok_or(OpenQueryError::UnknownMode)?;
            open_ports.push(OpenPort::new(port, mode));
        }
        let context = cursor.context()?;
        if !cursor.finished() {
            return Err(OpenQueryError::TrailingPayloadBytes(cursor.remaining()));
        }
        Ok(Self::new(relation, bound_ports, open_ports, context))
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, OpenQueryError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(OPEN_QUERY_ARTIFACT_KIND)?,
            OPEN_QUERY_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }
    pub fn query_ref(&self) -> Result<QueryRef, OpenQueryError> {
        Ok(QueryRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }
    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, OpenQueryError> {
        if envelope.kind().as_str() != OPEN_QUERY_ARTIFACT_KIND {
            return Err(OpenQueryError::UnexpectedArtifactKind {
                expected: OPEN_QUERY_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != OPEN_QUERY_SCHEMA_VERSION {
            return Err(OpenQueryError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }
    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = vec![self.relation.as_artifact_ref()];
        references.extend(
            self.bound_ports
                .iter()
                .map(|binding| binding.value().as_artifact_ref()),
        );
        references.extend([
            self.context.scope().as_artifact_ref(),
            self.context.applicability().as_artifact_ref(),
            self.context.grain().as_artifact_ref(),
            self.context.horizon().as_artifact_ref(),
            self.context.support().as_artifact_ref(),
        ]);
        if let Some(warrant) = self.context.warrant() {
            references.push(warrant.as_artifact_ref());
        }
        references
    }
    pub fn check<C: RelationCatalog>(&self, catalog: &C) -> Result<(), OpenQueryCheckError> {
        self.check_partition(catalog, true)
    }

    fn check_partition<C: RelationCatalog>(
        &self,
        catalog: &C,
        require_open_port: bool,
    ) -> Result<(), OpenQueryCheckError> {
        if require_open_port && self.open_ports.is_empty() {
            return Err(OpenQueryCheckError::EmptyOpenPorts);
        }
        let schema = catalog
            .resolve_relation_schema(self.relation)
            .ok_or(OpenQueryCheckError::UnresolvedRelation(self.relation))?;
        if schema.relation_ref()? != self.relation {
            return Err(OpenQueryCheckError::RelationReferenceIdentityMismatch {
                reference: self.relation,
                calculated: schema.relation_ref()?,
            });
        }
        schema.check(catalog)?;
        let mut covered = BTreeSet::new();
        for binding in &self.bound_ports {
            if !covered.insert(binding.port().clone()) {
                return Err(OpenQueryCheckError::DuplicatePort(binding.port().clone()));
            }
            let expected = schema
                .ports()
                .iter()
                .find(|port| port.name() == binding.port())
                .ok_or_else(|| OpenQueryCheckError::UnknownPort(binding.port().clone()))?;
            let form = catalog
                .resolve_typed_form(binding.value())
                .ok_or(OpenQueryCheckError::UnresolvedTypedForm(binding.value()))?;
            if form.typed_form_ref()? != binding.value() {
                return Err(OpenQueryCheckError::TypedFormReferenceIdentityMismatch {
                    reference: binding.value(),
                    calculated: form.typed_form_ref()?,
                });
            }
            if form.binding() != schema.binding() {
                return Err(OpenQueryCheckError::TypedFormBindingMismatch {
                    expected: schema.binding(),
                    actual: form.binding(),
                });
            }
            if form.ty() != expected.ty() {
                return Err(OpenQueryCheckError::PortTypeMismatch {
                    port: binding.port().clone(),
                    expected: expected.ty(),
                    actual: form.ty(),
                });
            }
        }
        for open in &self.open_ports {
            if !covered.insert(open.port().clone()) {
                return Err(OpenQueryCheckError::DuplicatePort(open.port().clone()));
            }
            if !schema.ports().iter().any(|port| port.name() == open.port()) {
                return Err(OpenQueryCheckError::UnknownPort(open.port().clone()));
            }
        }
        if covered.len() != schema.ports().len() {
            return Err(OpenQueryCheckError::IncompletePortPartition);
        }
        Ok(())
    }

    /// Binds one currently open port while preserving the question's remaining open section.
    ///
    /// The result remains a query only if another port survives. Supplying every answer
    /// coordinate is `Plug`, which remains a distinct later operation.
    pub fn bind<C: RelationCatalog>(
        &self,
        binding: PortBinding,
        catalog: &C,
    ) -> Result<Self, OpenQueryTransformError> {
        self.check(catalog)?;
        let Some(index) = self
            .open_ports
            .iter()
            .position(|open| open.port() == binding.port())
        else {
            return Err(OpenQueryTransformError::PortIsNotOpen(
                binding.port().clone(),
            ));
        };
        let mut bound_ports = self.bound_ports.clone();
        bound_ports.push(binding);
        let mut open_ports = self.open_ports.clone();
        open_ports.remove(index);
        let transformed = Self::new(self.relation, bound_ports, open_ports, self.context);
        transformed.check(catalog)?;
        Ok(transformed)
    }

    /// Reopens one currently bound port under a declared evidence mode.
    pub fn expose<C: RelationCatalog>(
        &self,
        port: TypeSymbol,
        mode: DischargeMode,
        catalog: &C,
    ) -> Result<Self, OpenQueryTransformError> {
        self.check(catalog)?;
        let Some(index) = self
            .bound_ports
            .iter()
            .position(|binding| binding.port() == &port)
        else {
            return Err(OpenQueryTransformError::PortIsNotBound(port));
        };
        let mut bound_ports = self.bound_ports.clone();
        bound_ports.remove(index);
        let mut open_ports = self.open_ports.clone();
        open_ports.push(OpenPort::new(port, mode));
        let transformed = Self::new(self.relation, bound_ports, open_ports, self.context);
        transformed.check(catalog)?;
        Ok(transformed)
    }

    /// Supplies all currently open ports as a typed full assignment without evaluating the relation.
    pub fn plug<C: RelationCatalog>(
        &self,
        answers: Vec<PortBinding>,
        catalog: &C,
    ) -> Result<CompletionCandidate, OpenQueryPlugError> {
        self.check(catalog)?;
        let mut supplied = BTreeSet::new();
        for answer in &answers {
            if !supplied.insert(answer.port().clone()) {
                return Err(OpenQueryPlugError::DuplicateAnswerPort(
                    answer.port().clone(),
                ));
            }
            if !self
                .open_ports
                .iter()
                .any(|open| open.port() == answer.port())
            {
                return Err(OpenQueryPlugError::PortIsNotOpen(answer.port().clone()));
            }
        }
        for open in &self.open_ports {
            if !supplied.contains(open.port()) {
                return Err(OpenQueryPlugError::MissingAnswerPort(open.port().clone()));
            }
        }

        let mut bindings = self.bound_ports.clone();
        bindings.extend(answers);
        let completed = Self::new(self.relation, bindings.clone(), Vec::new(), self.context);
        completed.check_partition(catalog, false)?;
        Ok(CompletionCandidate {
            source: self.query_ref()?,
            bindings,
        })
    }
}

fn reference(encoded: &mut Vec<u8>, value: ArtifactRef) {
    encoded.extend_from_slice(value.as_bytes());
}
fn count(encoded: &mut Vec<u8>, value: usize) -> Result<(), OpenQueryError> {
    let value = u32::try_from(value).map_err(|_| OpenQueryError::CollectionTooLong(value))?;
    encoded.extend_from_slice(&value.to_be_bytes());
    Ok(())
}
fn text(encoded: &mut Vec<u8>, value: &str) -> Result<(), OpenQueryError> {
    count(encoded, value.len())?;
    encoded.extend_from_slice(value.as_bytes());
    Ok(())
}
fn write_context(encoded: &mut Vec<u8>, context: &RelationUseContext) {
    reference(encoded, context.scope().as_artifact_ref());
    reference(encoded, context.applicability().as_artifact_ref());
    reference(encoded, context.grain().as_artifact_ref());
    reference(encoded, context.horizon().as_artifact_ref());
    encoded.push(context.mode().tag());
    reference(encoded, context.support().as_artifact_ref());
    match context.warrant() {
        None => encoded.push(0),
        Some(warrant) => {
            encoded.push(1);
            reference(encoded, warrant.as_artifact_ref());
        }
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
    fn take(&mut self, length: usize) -> Result<&'a [u8], OpenQueryError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(OpenQueryError::PayloadLengthOverflow)?;
        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(OpenQueryError::TruncatedPayload)?;
        self.position = end;
        Ok(bytes)
    }
    fn byte(&mut self) -> Result<u8, OpenQueryError> {
        Ok(self.take(1)?[0])
    }
    fn count(&mut self) -> Result<usize, OpenQueryError> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| OpenQueryError::TruncatedPayload)?;
        usize::try_from(u32::from_be_bytes(bytes))
            .map_err(|_| OpenQueryError::PayloadLengthOverflow)
    }
    fn reference(&mut self) -> Result<ArtifactRef, OpenQueryError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| OpenQueryError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(bytes))
    }
    fn port_name(&mut self) -> Result<TypeSymbol, OpenQueryError> {
        let length = self.count()?;
        let text = String::from_utf8(self.take(length)?.to_vec())
            .map_err(OpenQueryError::InvalidPortNameUtf8)?;
        TypeSymbol::new(text.clone()).map_err(|_| OpenQueryError::InvalidPortName(text))
    }
    fn context(&mut self) -> Result<RelationUseContext, OpenQueryError> {
        let scope = crate::ScopeRef::from_artifact_ref(self.reference()?);
        let applicability = crate::ApplicabilityRef::from_artifact_ref(self.reference()?);
        let grain = crate::GrainRef::from_artifact_ref(self.reference()?);
        let horizon = crate::HorizonRef::from_artifact_ref(self.reference()?);
        let mode = DischargeMode::from_tag(self.byte()?).ok_or(OpenQueryError::UnknownMode)?;
        let support = crate::SupportRef::from_artifact_ref(self.reference()?);
        let warrant = match self.byte()? {
            0 => None,
            1 => Some(crate::WarrantRef::from_artifact_ref(self.reference()?)),
            value => return Err(OpenQueryError::UnknownOptionalTag(value)),
        };
        Ok(RelationUseContext::new(
            scope,
            applicability,
            grain,
            horizon,
            mode,
            support,
            warrant,
        ))
    }
    const fn finished(&self) -> bool {
        self.position == self.bytes.len()
    }
    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

#[derive(Debug, Error)]
pub enum OpenQueryError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("open-query collection is too long: {0} entries")]
    CollectionTooLong(usize),
    #[error("invalid open-query port name {0:?}")]
    InvalidPortName(String),
    #[error("open-query port name bytes are not valid UTF-8")]
    InvalidPortNameUtf8(#[source] std::string::FromUtf8Error),
    #[error("open-query payload is truncated")]
    TruncatedPayload,
    #[error("open-query payload length overflows this platform")]
    PayloadLengthOverflow,
    #[error("open-query payload contains {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("open-query payload contains unknown discharge mode")]
    UnknownMode,
    #[error("open-query payload contains unknown optional tag {0}")]
    UnknownOptionalTag(u8),
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported open-query schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

#[derive(Debug, Error)]
pub enum OpenQueryCheckError {
    #[error(transparent)]
    Query(#[from] OpenQueryError),
    #[error(transparent)]
    Schema(#[from] crate::RelationCheckError),
    #[error(transparent)]
    RelationArtifact(#[from] crate::RelationError),
    #[error(transparent)]
    TypeArtifact(#[from] TypeError),
    #[error("an open query must expose at least one port")]
    EmptyOpenPorts,
    #[error("relation {0} is not available from the declared catalog")]
    UnresolvedRelation(RelationRef),
    #[error("catalog relation {reference} hashes to {calculated}, not its claimed identity")]
    RelationReferenceIdentityMismatch {
        reference: RelationRef,
        calculated: RelationRef,
    },
    #[error("open query names port {0} more than once")]
    DuplicatePort(TypeSymbol),
    #[error("open query names unknown port {0}")]
    UnknownPort(TypeSymbol),
    #[error("open query does not classify every schema port as bound or open")]
    IncompletePortPartition,
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

/// Errors from typed data-only binding and exposure transformations.
#[derive(Debug, Error)]
pub enum OpenQueryTransformError {
    #[error(transparent)]
    Check(#[from] OpenQueryCheckError),

    #[error("cannot bind {0}: that port is not currently open")]
    PortIsNotOpen(TypeSymbol),

    #[error("cannot expose {0}: that port is not currently bound")]
    PortIsNotBound(TypeSymbol),
}

/// Errors from complete typed filling of an OpenQuery.
#[derive(Debug, Error)]
pub enum OpenQueryPlugError {
    #[error(transparent)]
    Check(#[from] OpenQueryCheckError),

    #[error(transparent)]
    Query(#[from] OpenQueryError),

    #[error("plugging supplies port {0} more than once")]
    DuplicateAnswerPort(TypeSymbol),

    #[error("plugging supplies {0}, which is not currently open")]
    PortIsNotOpen(TypeSymbol),

    #[error("plugging omits open port {0}")]
    MissingAnswerPort(TypeSymbol),
}

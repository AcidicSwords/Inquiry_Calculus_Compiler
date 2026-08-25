use std::collections::BTreeSet;

use thiserror::Error;

use crate::{
    ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, BindingVersionRef, FormulaCatalog,
    FormulaCheckError, FormulaError, FormulaRef, RelationRef, RelationSignature, TypeArtifact,
    TypeCatalog, TypeCheckError, TypeError, TypeRef, TypeSymbol,
};

/// Canonical artifact kind for Phase 2 relation schemas.
pub const RELATION_SCHEMA_ARTIFACT_KIND: &str = "ic.relation-schema";

/// Payload schema version for Phase 2 relation schemas.
pub const RELATION_SCHEMA_VERSION: u32 = 1;

/// A named, binding-scoped port in a relation schema.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationPort {
    name: TypeSymbol,
    ty: TypeRef,
}

impl RelationPort {
    #[must_use]
    pub const fn new(name: TypeSymbol, ty: TypeRef) -> Self {
        Self { name, ty }
    }

    #[must_use]
    pub const fn name(&self) -> &TypeSymbol {
        &self.name
    }

    #[must_use]
    pub const fn ty(&self) -> TypeRef {
        self.ty
    }
}

/// The explicit semantic route for a relation's behavior.
///
/// Binding-native behavior is represented by an immutable contract artifact reference;
/// it is never supplied as an opaque host-language callback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationBodyIR {
    Formula(FormulaRef),
    BindingNative { contract: ArtifactRef },
}

/// A canonical relation schema with named, ordered typed ports.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationSchema {
    binding: BindingVersionRef,
    ports: Vec<RelationPort>,
    body: RelationBodyIR,
    laws: Vec<ArtifactRef>,
    provenance: Vec<ArtifactRef>,
}

impl RelationSchema {
    #[must_use]
    pub fn new(
        binding: BindingVersionRef,
        ports: Vec<RelationPort>,
        body: RelationBodyIR,
        laws: Vec<ArtifactRef>,
        provenance: Vec<ArtifactRef>,
    ) -> Self {
        Self {
            binding,
            ports,
            body,
            laws,
            provenance,
        }
    }

    #[must_use]
    pub const fn binding(&self) -> BindingVersionRef {
        self.binding
    }

    #[must_use]
    pub fn ports(&self) -> &[RelationPort] {
        &self.ports
    }

    #[must_use]
    pub const fn body(&self) -> RelationBodyIR {
        self.body
    }

    #[must_use]
    pub fn laws(&self) -> &[ArtifactRef] {
        &self.laws
    }

    #[must_use]
    pub fn provenance(&self) -> &[ArtifactRef] {
        &self.provenance
    }

    /// Calculates the checked signature supplied to formula atom validation.
    pub fn signature(&self) -> Result<RelationSignature, RelationError> {
        Ok(RelationSignature::new(
            self.relation_ref()?,
            self.binding,
            self.ports.iter().map(RelationPort::ty).collect(),
        ))
    }

    /// Encodes this relation schema's canonical payload.
    pub fn canonical_payload(&self) -> Result<Vec<u8>, RelationError> {
        let mut encoded = Vec::new();
        write_reference(&mut encoded, self.binding.as_artifact_ref());
        write_count(&mut encoded, self.ports.len())?;
        for port in &self.ports {
            write_text(&mut encoded, port.name().as_str())?;
            write_reference(&mut encoded, port.ty().as_artifact_ref());
        }
        match self.body {
            RelationBodyIR::Formula(reference) => {
                encoded.push(0);
                write_reference(&mut encoded, reference.as_artifact_ref());
            }
            RelationBodyIR::BindingNative { contract } => {
                encoded.push(1);
                write_reference(&mut encoded, contract);
            }
        }
        write_references(&mut encoded, &self.laws)?;
        write_references(&mut encoded, &self.provenance)?;
        Ok(encoded)
    }

    /// Decodes one complete canonical relation-schema payload.
    pub fn decode_payload(payload: &[u8]) -> Result<Self, RelationError> {
        let mut cursor = RelationCursor::new(payload);
        let binding = BindingVersionRef::from_artifact_ref(cursor.read_reference()?);
        let port_count = cursor.read_count()?;
        let mut ports = Vec::with_capacity(port_count);
        for _ in 0..port_count {
            let name = cursor.read_text()?;
            let name =
                TypeSymbol::new(name.clone()).map_err(|_| RelationError::InvalidPortName(name))?;
            let ty = TypeRef::from_artifact_ref(cursor.read_reference()?);
            ports.push(RelationPort::new(name, ty));
        }
        let body = match cursor.read_byte()? {
            0 => RelationBodyIR::Formula(FormulaRef::from_artifact_ref(cursor.read_reference()?)),
            1 => RelationBodyIR::BindingNative {
                contract: cursor.read_reference()?,
            },
            other => return Err(RelationError::UnknownBodyTag(other)),
        };
        let laws = cursor.read_references()?;
        let provenance = cursor.read_references()?;
        if !cursor.is_finished() {
            return Err(RelationError::TrailingPayloadBytes(cursor.remaining()));
        }
        Ok(Self::new(binding, ports, body, laws, provenance))
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, RelationError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(RELATION_SCHEMA_ARTIFACT_KIND)?,
            RELATION_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }

    pub fn relation_ref(&self) -> Result<RelationRef, RelationError> {
        Ok(RelationRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, RelationError> {
        if envelope.kind().as_str() != RELATION_SCHEMA_ARTIFACT_KIND {
            return Err(RelationError::UnexpectedArtifactKind {
                expected: RELATION_SCHEMA_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != RELATION_SCHEMA_VERSION {
            return Err(RelationError::UnsupportedRelationSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    /// Returns explicit dependencies without reinterpreting encoded bytes.
    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = vec![self.binding.as_artifact_ref()];
        references.extend(self.ports.iter().map(|port| port.ty().as_artifact_ref()));
        match self.body {
            RelationBodyIR::Formula(reference) => references.push(reference.as_artifact_ref()),
            RelationBodyIR::BindingNative { contract } => references.push(contract),
        }
        references.extend(self.laws.iter().copied());
        references.extend(self.provenance.iter().copied());
        references
    }

    /// Checks named-port uniqueness, binding-scoped type references, and formula bodies.
    pub fn check<C: FormulaCatalog>(&self, catalog: &C) -> Result<(), RelationCheckError> {
        let mut names = BTreeSet::new();
        for port in &self.ports {
            if !names.insert(port.name().clone()) {
                return Err(RelationCheckError::DuplicatePortName(port.name().clone()));
            }
            let ty = resolve_type(port.ty(), catalog)?;
            if ty.binding() != self.binding {
                return Err(RelationCheckError::PortBindingMismatch {
                    name: port.name().clone(),
                    expected: self.binding,
                    actual: ty.binding(),
                });
            }
            ty.check(catalog)?;
        }

        if let RelationBodyIR::Formula(reference) = self.body {
            let Some(formula) = catalog.resolve_formula(reference) else {
                return Err(RelationCheckError::UnresolvedFormula(reference));
            };
            let calculated = formula.formula_ref()?;
            if calculated != reference {
                return Err(RelationCheckError::FormulaReferenceIdentityMismatch {
                    reference,
                    calculated,
                });
            }
            if formula.binding() != self.binding {
                return Err(RelationCheckError::FormulaBindingMismatch {
                    expected: self.binding,
                    actual: formula.binding(),
                    reference,
                });
            }
            let expected_context: Vec<_> = self.ports.iter().map(RelationPort::ty).collect();
            if formula.context() != expected_context {
                return Err(RelationCheckError::FormulaContextMismatch {
                    reference,
                    expected: expected_context,
                    actual: formula.context().to_vec(),
                });
            }
            formula.check(catalog)?;
        }
        Ok(())
    }
}

fn resolve_type<C: TypeCatalog>(
    reference: TypeRef,
    catalog: &C,
) -> Result<TypeArtifact, RelationCheckError> {
    let Some(ty) = catalog.resolve_type(reference) else {
        return Err(RelationCheckError::UnresolvedType(reference));
    };
    let calculated = ty.type_ref()?;
    if calculated != reference {
        return Err(RelationCheckError::TypeReferenceIdentityMismatch {
            reference,
            calculated,
        });
    }
    Ok(ty)
}

fn write_reference(encoded: &mut Vec<u8>, reference: ArtifactRef) {
    encoded.extend_from_slice(reference.as_bytes());
}

fn write_text(encoded: &mut Vec<u8>, value: &str) -> Result<(), RelationError> {
    let length = u32::try_from(value.len()).map_err(|_| RelationError::TextTooLong(value.len()))?;
    encoded.extend_from_slice(&length.to_be_bytes());
    encoded.extend_from_slice(value.as_bytes());
    Ok(())
}

fn write_count(encoded: &mut Vec<u8>, count: usize) -> Result<(), RelationError> {
    let count = u32::try_from(count).map_err(|_| RelationError::CollectionTooLong(count))?;
    encoded.extend_from_slice(&count.to_be_bytes());
    Ok(())
}

fn write_references(
    encoded: &mut Vec<u8>,
    references: &[ArtifactRef],
) -> Result<(), RelationError> {
    write_count(encoded, references.len())?;
    for reference in references {
        write_reference(encoded, *reference);
    }
    Ok(())
}

struct RelationCursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> RelationCursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], RelationError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(RelationError::PayloadLengthOverflow)?;
        let value = self
            .bytes
            .get(self.position..end)
            .ok_or(RelationError::TruncatedPayload)?;
        self.position = end;
        Ok(value)
    }

    fn read_byte(&mut self) -> Result<u8, RelationError> {
        Ok(self.take(1)?[0])
    }

    fn read_u32(&mut self) -> Result<u32, RelationError> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| RelationError::TruncatedPayload)?;
        Ok(u32::from_be_bytes(bytes))
    }

    fn read_count(&mut self) -> Result<usize, RelationError> {
        usize::try_from(self.read_u32()?).map_err(|_| RelationError::PayloadLengthOverflow)
    }

    fn read_reference(&mut self) -> Result<ArtifactRef, RelationError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| RelationError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(bytes))
    }

    fn read_text(&mut self) -> Result<String, RelationError> {
        let length = self.read_count()?;
        String::from_utf8(self.take(length)?.to_vec()).map_err(RelationError::InvalidPortNameUtf8)
    }

    fn read_references(&mut self) -> Result<Vec<ArtifactRef>, RelationError> {
        let count = self.read_count()?;
        let mut references = Vec::with_capacity(count);
        for _ in 0..count {
            references.push(self.read_reference()?);
        }
        Ok(references)
    }

    const fn is_finished(&self) -> bool {
        self.position == self.bytes.len()
    }

    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

/// Relation-schema encoding and decoding errors.
#[derive(Debug, Error)]
pub enum RelationError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),

    #[error("relation collection is too long: {0} entries")]
    CollectionTooLong(usize),

    #[error("relation port name is too long: {0} bytes")]
    TextTooLong(usize),

    #[error("invalid relation port name {0:?}; expected [A-Za-z_][A-Za-z0-9_.-]*")]
    InvalidPortName(String),

    #[error("relation port name bytes are not valid UTF-8")]
    InvalidPortNameUtf8(#[source] std::string::FromUtf8Error),

    #[error("relation payload is truncated")]
    TruncatedPayload,

    #[error("relation payload length overflows this platform")]
    PayloadLengthOverflow,

    #[error("relation payload contains {0} trailing bytes")]
    TrailingPayloadBytes(usize),

    #[error("relation payload contains unknown body tag {0}")]
    UnknownBodyTag(u8),

    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },

    #[error("unsupported relation schema version {0}")]
    UnsupportedRelationSchemaVersion(u32),
}

/// Relation-schema validation errors.
#[derive(Debug, Error)]
pub enum RelationCheckError {
    #[error(transparent)]
    Relation(#[from] RelationError),

    #[error(transparent)]
    Formula(#[from] FormulaCheckError),

    #[error(transparent)]
    FormulaArtifact(#[from] FormulaError),

    #[error(transparent)]
    Type(#[from] TypeCheckError),

    #[error(transparent)]
    TypeArtifact(#[from] TypeError),

    #[error("relation has duplicate port name {0}")]
    DuplicatePortName(TypeSymbol),

    #[error("type {0} is not available from the declared catalog")]
    UnresolvedType(TypeRef),

    #[error("catalog entry for type {reference} hashes to {calculated}, not its claimed identity")]
    TypeReferenceIdentityMismatch {
        reference: TypeRef,
        calculated: TypeRef,
    },

    #[error("port {name} belongs to binding {actual}, expected {expected}")]
    PortBindingMismatch {
        name: TypeSymbol,
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },

    #[error("formula {0} is not available from the declared catalog")]
    UnresolvedFormula(FormulaRef),

    #[error(
        "catalog entry for formula {reference} hashes to {calculated}, not its claimed identity"
    )]
    FormulaReferenceIdentityMismatch {
        reference: FormulaRef,
        calculated: FormulaRef,
    },

    #[error("formula {reference} belongs to binding {actual}, expected {expected}")]
    FormulaBindingMismatch {
        expected: BindingVersionRef,
        actual: BindingVersionRef,
        reference: FormulaRef,
    },

    #[error("formula {reference} has a different relation-port context")]
    FormulaContextMismatch {
        reference: FormulaRef,
        expected: Vec<TypeRef>,
        actual: Vec<TypeRef>,
    },
}

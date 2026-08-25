use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, FormulaRef, PortBinding,
    RelationRef, TypeSymbol,
};

/// Canonical artifact kind for the data-only relational expression grammar.
pub const RELATION_EXPR_ARTIFACT_KIND: &str = "ic.relation-expr";
/// Payload schema version for the data-only relational expression grammar.
pub const RELATION_EXPR_SCHEMA_VERSION: u32 = 1;

/// A reference known to identify a canonical relation-expression artifact.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RelationExprRef(ArtifactRef);

impl RelationExprRef {
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }
    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}

impl fmt::Display for RelationExprRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for RelationExprRef {
    type Err = ArtifactError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// A source and destination port name in an alpha-renaming expression.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PortRename {
    from: TypeSymbol,
    to: TypeSymbol,
}

impl PortRename {
    #[must_use]
    pub const fn new(from: TypeSymbol, to: TypeSymbol) -> Self {
        Self { from, to }
    }
    #[must_use]
    pub const fn from(&self) -> &TypeSymbol {
        &self.from
    }
    #[must_use]
    pub const fn to(&self) -> &TypeSymbol {
        &self.to
    }
}

/// The canonical v1.1 data-only relational query grammar.
///
/// Each variant is syntax. Its later denotation is relational substitution, join, exposure,
/// hiding, renaming, or guarded restriction; construction never evaluates a relation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelationExprIR {
    Relation(RelationRef),
    Bind {
        source: Box<Self>,
        bindings: Vec<PortBinding>,
    },
    Join {
        left: Box<Self>,
        right: Box<Self>,
    },
    Expose {
        source: Box<Self>,
        ports: Vec<TypeSymbol>,
    },
    Hide {
        source: Box<Self>,
        ports: Vec<TypeSymbol>,
    },
    Rename {
        source: Box<Self>,
        renames: Vec<PortRename>,
    },
    Guard {
        source: Box<Self>,
        guard: FormulaRef,
    },
}

/// A content-addressed data-only relation expression.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationExprArtifact {
    expression: RelationExprIR,
}

impl RelationExprArtifact {
    #[must_use]
    pub const fn new(expression: RelationExprIR) -> Self {
        Self { expression }
    }
    #[must_use]
    pub const fn expression(&self) -> &RelationExprIR {
        &self.expression
    }

    pub fn canonical_payload(&self) -> Result<Vec<u8>, RelationExprError> {
        let mut encoded = Vec::new();
        write_expression(&mut encoded, &self.expression)?;
        Ok(encoded)
    }
    pub fn decode_payload(payload: &[u8]) -> Result<Self, RelationExprError> {
        let mut cursor = Cursor::new(payload);
        let expression = cursor.expression()?;
        if !cursor.finished() {
            return Err(RelationExprError::TrailingPayloadBytes(cursor.remaining()));
        }
        Ok(Self::new(expression))
    }
    pub fn envelope(&self) -> Result<ArtifactEnvelope, RelationExprError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(RELATION_EXPR_ARTIFACT_KIND)?,
            RELATION_EXPR_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }
    pub fn relation_expr_ref(&self) -> Result<RelationExprRef, RelationExprError> {
        Ok(RelationExprRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }
    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, RelationExprError> {
        if envelope.kind().as_str() != RELATION_EXPR_ARTIFACT_KIND {
            return Err(RelationExprError::UnexpectedArtifactKind {
                expected: RELATION_EXPR_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != RELATION_EXPR_SCHEMA_VERSION {
            return Err(RelationExprError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }
    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = Vec::new();
        collect_references(&self.expression, &mut references);
        references
    }
}

fn collect_references(expression: &RelationExprIR, references: &mut Vec<ArtifactRef>) {
    match expression {
        RelationExprIR::Relation(relation) => references.push(relation.as_artifact_ref()),
        RelationExprIR::Bind { source, bindings } => {
            collect_references(source, references);
            references.extend(
                bindings
                    .iter()
                    .map(|binding| binding.value().as_artifact_ref()),
            );
        }
        RelationExprIR::Join { left, right } => {
            collect_references(left, references);
            collect_references(right, references);
        }
        RelationExprIR::Expose { source, .. }
        | RelationExprIR::Hide { source, .. }
        | RelationExprIR::Rename { source, .. } => collect_references(source, references),
        RelationExprIR::Guard { source, guard } => {
            collect_references(source, references);
            references.push(guard.as_artifact_ref());
        }
    }
}

fn write_expression(
    encoded: &mut Vec<u8>,
    expression: &RelationExprIR,
) -> Result<(), RelationExprError> {
    match expression {
        RelationExprIR::Relation(relation) => {
            encoded.push(0);
            reference(encoded, relation.as_artifact_ref());
        }
        RelationExprIR::Bind { source, bindings } => {
            encoded.push(1);
            write_expression(encoded, source)?;
            count(encoded, bindings.len())?;
            for binding in bindings {
                text(encoded, binding.port().as_str())?;
                reference(encoded, binding.value().as_artifact_ref());
            }
        }
        RelationExprIR::Join { left, right } => {
            encoded.push(2);
            write_expression(encoded, left)?;
            write_expression(encoded, right)?;
        }
        RelationExprIR::Expose { source, ports } => {
            encoded.push(3);
            write_expression(encoded, source)?;
            names(encoded, ports)?;
        }
        RelationExprIR::Hide { source, ports } => {
            encoded.push(4);
            write_expression(encoded, source)?;
            names(encoded, ports)?;
        }
        RelationExprIR::Rename { source, renames } => {
            encoded.push(5);
            write_expression(encoded, source)?;
            count(encoded, renames.len())?;
            for rename in renames {
                text(encoded, rename.from().as_str())?;
                text(encoded, rename.to().as_str())?;
            }
        }
        RelationExprIR::Guard { source, guard } => {
            encoded.push(6);
            write_expression(encoded, source)?;
            reference(encoded, guard.as_artifact_ref());
        }
    }
    Ok(())
}
fn reference(encoded: &mut Vec<u8>, reference: ArtifactRef) {
    encoded.extend_from_slice(reference.as_bytes());
}
fn count(encoded: &mut Vec<u8>, count: usize) -> Result<(), RelationExprError> {
    let count = u32::try_from(count).map_err(|_| RelationExprError::CollectionTooLong(count))?;
    encoded.extend_from_slice(&count.to_be_bytes());
    Ok(())
}
fn text(encoded: &mut Vec<u8>, text: &str) -> Result<(), RelationExprError> {
    count(encoded, text.len())?;
    encoded.extend_from_slice(text.as_bytes());
    Ok(())
}
fn names(encoded: &mut Vec<u8>, names: &[TypeSymbol]) -> Result<(), RelationExprError> {
    count(encoded, names.len())?;
    for name in names {
        text(encoded, name.as_str())?;
    }
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
    fn take(&mut self, length: usize) -> Result<&'a [u8], RelationExprError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(RelationExprError::PayloadLengthOverflow)?;
        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(RelationExprError::TruncatedPayload)?;
        self.position = end;
        Ok(bytes)
    }
    fn byte(&mut self) -> Result<u8, RelationExprError> {
        Ok(self.take(1)?[0])
    }
    fn count(&mut self) -> Result<usize, RelationExprError> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| RelationExprError::TruncatedPayload)?;
        usize::try_from(u32::from_be_bytes(bytes))
            .map_err(|_| RelationExprError::PayloadLengthOverflow)
    }
    fn reference(&mut self) -> Result<ArtifactRef, RelationExprError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| RelationExprError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(bytes))
    }
    fn symbol(&mut self) -> Result<TypeSymbol, RelationExprError> {
        let length = self.count()?;
        let text = String::from_utf8(self.take(length)?.to_vec())
            .map_err(RelationExprError::InvalidPortNameUtf8)?;
        TypeSymbol::new(text.clone()).map_err(|_| RelationExprError::InvalidPortName(text))
    }
    fn symbols(&mut self) -> Result<Vec<TypeSymbol>, RelationExprError> {
        let count = self.count()?;
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(self.symbol()?);
        }
        Ok(values)
    }
    fn expression(&mut self) -> Result<RelationExprIR, RelationExprError> {
        match self.byte()? {
            0 => Ok(RelationExprIR::Relation(RelationRef::from_artifact_ref(
                self.reference()?,
            ))),
            1 => {
                let source = Box::new(self.expression()?);
                let count = self.count()?;
                let mut bindings = Vec::with_capacity(count);
                for _ in 0..count {
                    bindings.push(PortBinding::new(
                        self.symbol()?,
                        crate::TypedFormRef::from_artifact_ref(self.reference()?),
                    ));
                }
                Ok(RelationExprIR::Bind { source, bindings })
            }
            2 => Ok(RelationExprIR::Join {
                left: Box::new(self.expression()?),
                right: Box::new(self.expression()?),
            }),
            3 => Ok(RelationExprIR::Expose {
                source: Box::new(self.expression()?),
                ports: self.symbols()?,
            }),
            4 => Ok(RelationExprIR::Hide {
                source: Box::new(self.expression()?),
                ports: self.symbols()?,
            }),
            5 => {
                let source = Box::new(self.expression()?);
                let count = self.count()?;
                let mut renames = Vec::with_capacity(count);
                for _ in 0..count {
                    renames.push(PortRename::new(self.symbol()?, self.symbol()?));
                }
                Ok(RelationExprIR::Rename { source, renames })
            }
            6 => Ok(RelationExprIR::Guard {
                source: Box::new(self.expression()?),
                guard: FormulaRef::from_artifact_ref(self.reference()?),
            }),
            tag => Err(RelationExprError::UnknownTag(tag)),
        }
    }
    const fn finished(&self) -> bool {
        self.position == self.bytes.len()
    }
    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

#[derive(Debug, Error)]
pub enum RelationExprError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("relation-expression collection is too long: {0} entries")]
    CollectionTooLong(usize),
    #[error("invalid relation-expression port name {0:?}")]
    InvalidPortName(String),
    #[error("relation-expression port name bytes are not valid UTF-8")]
    InvalidPortNameUtf8(#[source] std::string::FromUtf8Error),
    #[error("relation-expression payload is truncated")]
    TruncatedPayload,
    #[error("relation-expression payload length overflows this platform")]
    PayloadLengthOverflow,
    #[error("relation-expression payload contains {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("relation-expression payload contains unknown tag {0}")]
    UnknownTag(u8),
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported relation-expression schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, QueryRef, TypeRef, TypeSymbol,
    TypedFormRef,
};

/// Canonical artifact kind for first-order inquiry programs.
pub const IPROG_ARTIFACT_KIND: &str = "ic.iprog";
/// Payload schema version for first-order inquiry programs.
pub const IPROG_SCHEMA_VERSION: u32 = 1;

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
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
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
artifact_reference!(IProgRef);

/// Capture-safe, first-order inquiry-program syntax.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IProgIR {
    Return {
        value: TypedFormRef,
    },
    Ask {
        question: QueryRef,
        answer_slot: TypeSymbol,
        continuation: IProgRef,
    },
}

/// A typed first-order inquiry program artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IProgArtifact {
    result: TypeRef,
    program: IProgIR,
}

impl IProgArtifact {
    #[must_use]
    pub const fn new(result: TypeRef, program: IProgIR) -> Self {
        Self { result, program }
    }
    #[must_use]
    pub const fn result(&self) -> TypeRef {
        self.result
    }
    #[must_use]
    pub const fn program(&self) -> &IProgIR {
        &self.program
    }
    pub fn canonical_payload(&self) -> Result<Vec<u8>, IProgError> {
        let mut encoded = Vec::new();
        reference(&mut encoded, self.result.as_artifact_ref());
        match &self.program {
            IProgIR::Return { value } => {
                encoded.push(0);
                reference(&mut encoded, value.as_artifact_ref());
            }
            IProgIR::Ask {
                question,
                answer_slot,
                continuation,
            } => {
                encoded.push(1);
                reference(&mut encoded, question.as_artifact_ref());
                text(&mut encoded, answer_slot.as_str())?;
                reference(&mut encoded, continuation.as_artifact_ref());
            }
        }
        Ok(encoded)
    }
    pub fn decode_payload(payload: &[u8]) -> Result<Self, IProgError> {
        let mut c = Cursor::new(payload);
        let result = TypeRef::from_artifact_ref(c.reference()?);
        let program = match c.byte()? {
            0 => IProgIR::Return {
                value: TypedFormRef::from_artifact_ref(c.reference()?),
            },
            1 => {
                let question = QueryRef::from_artifact_ref(c.reference()?);
                let answer_slot = c.symbol()?;
                let continuation = IProgRef::from_artifact_ref(c.reference()?);
                IProgIR::Ask {
                    question,
                    answer_slot,
                    continuation,
                }
            }
            tag => return Err(IProgError::UnknownTag(tag)),
        };
        if !c.finished() {
            return Err(IProgError::TrailingPayloadBytes(c.remaining()));
        }
        Ok(Self::new(result, program))
    }
    pub fn envelope(&self) -> Result<ArtifactEnvelope, IProgError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(IPROG_ARTIFACT_KIND)?,
            IPROG_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }
    pub fn iprog_ref(&self) -> Result<IProgRef, IProgError> {
        Ok(IProgRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }
    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, IProgError> {
        if envelope.kind().as_str() != IPROG_ARTIFACT_KIND {
            return Err(IProgError::UnexpectedArtifactKind {
                expected: IPROG_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != IPROG_SCHEMA_VERSION {
            return Err(IProgError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }
    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut refs = vec![self.result.as_artifact_ref()];
        match &self.program {
            IProgIR::Return { value } => refs.push(value.as_artifact_ref()),
            IProgIR::Ask {
                question,
                continuation,
                ..
            } => {
                refs.push(question.as_artifact_ref());
                refs.push(continuation.as_artifact_ref());
            }
        }
        refs
    }
}
fn reference(encoded: &mut Vec<u8>, reference: ArtifactRef) {
    encoded.extend_from_slice(reference.as_bytes());
}
fn text(encoded: &mut Vec<u8>, value: &str) -> Result<(), IProgError> {
    let length = u32::try_from(value.len()).map_err(|_| IProgError::SlotTooLong(value.len()))?;
    encoded.extend_from_slice(&length.to_be_bytes());
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
    fn take(&mut self, length: usize) -> Result<&'a [u8], IProgError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(IProgError::PayloadLengthOverflow)?;
        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(IProgError::TruncatedPayload)?;
        self.position = end;
        Ok(bytes)
    }
    fn byte(&mut self) -> Result<u8, IProgError> {
        Ok(self.take(1)?[0])
    }
    fn reference(&mut self) -> Result<ArtifactRef, IProgError> {
        let b: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| IProgError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(b))
    }
    fn symbol(&mut self) -> Result<TypeSymbol, IProgError> {
        let b: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| IProgError::TruncatedPayload)?;
        let l = usize::try_from(u32::from_be_bytes(b))
            .map_err(|_| IProgError::PayloadLengthOverflow)?;
        let s = String::from_utf8(self.take(l)?.to_vec()).map_err(IProgError::InvalidSlotUtf8)?;
        TypeSymbol::new(s.clone()).map_err(|_| IProgError::InvalidSlot(s))
    }
    const fn finished(&self) -> bool {
        self.position == self.bytes.len()
    }
    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}
#[derive(Debug, Error)]
pub enum IProgError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("invalid answer slot {0:?}")]
    InvalidSlot(String),
    #[error("answer slot is too long: {0} bytes")]
    SlotTooLong(usize),
    #[error("answer slot bytes are not valid UTF-8")]
    InvalidSlotUtf8(#[source] std::string::FromUtf8Error),
    #[error("inquiry-program payload is truncated")]
    TruncatedPayload,
    #[error("inquiry-program payload length overflows this platform")]
    PayloadLengthOverflow,
    #[error("inquiry-program payload contains {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("inquiry-program payload contains unknown tag {0}")]
    UnknownTag(u8),
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported inquiry-program schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

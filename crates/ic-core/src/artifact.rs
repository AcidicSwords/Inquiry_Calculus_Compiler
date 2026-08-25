use std::{fmt, str::FromStr};

use sha2::{Digest, Sha256};
use thiserror::Error;

/// Domain separator for canonical artifact envelopes.
pub const ARTIFACT_DOMAIN: &[u8] = b"inquiry-calculus:artifact\0";

/// Version of the canonical artifact envelope wire format.
pub const ARTIFACT_WIRE_VERSION: u16 = 1;

/// The SHA-256 identity of a canonical artifact envelope.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ArtifactRef([u8; 32]);

impl ArtifactRef {
    /// Constructs a reference from its exact digest bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for ArtifactRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&hex::encode(self.0))
    }
}

impl FromStr for ArtifactRef {
    type Err = ArtifactError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(value).map_err(ArtifactError::InvalidReferenceHex)?;
        let bytes: [u8; 32] = bytes
            .try_into()
            .map_err(|bytes: Vec<u8>| ArtifactError::InvalidReferenceLength(bytes.len()))?;
        Ok(Self(bytes))
    }
}

/// An implementation-level artifact kind identifier.
///
/// Kinds use a stable lowercase ASCII grammar so their byte representation needs no
/// Unicode or case normalization: `[a-z][a-z0-9._-]*`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ArtifactKind(String);

impl ArtifactKind {
    /// Validates and constructs an artifact kind.
    pub fn new(value: impl Into<String>) -> Result<Self, ArtifactError> {
        let value = value.into();
        let mut bytes = value.bytes();
        let Some(first) = bytes.next() else {
            return Err(ArtifactError::EmptyKind);
        };

        if !first.is_ascii_lowercase()
            || !bytes.all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'.' | b'_' | b'-')
            })
        {
            return Err(ArtifactError::InvalidKind(value));
        }

        u32::try_from(value.len()).map_err(|_| ArtifactError::KindTooLong(value.len()))?;
        Ok(Self(value))
    }

    /// Returns the exact kind identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ArtifactKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for ArtifactKind {
    type Err = ArtifactError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A versioned envelope around payload bytes already made canonical by their type.
///
/// This type does not claim arbitrary bytes or arbitrary serde output are canonical.
/// Future typed artifact implementations must define their own canonical payload
/// encoders before constructing an envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactEnvelope {
    kind: ArtifactKind,
    schema_version: u32,
    canonical_payload: Vec<u8>,
}

impl ArtifactEnvelope {
    /// Constructs an envelope from payload bytes whose canonical form is already defined.
    #[must_use]
    pub fn from_canonical_payload(
        kind: ArtifactKind,
        schema_version: u32,
        canonical_payload: Vec<u8>,
    ) -> Self {
        Self {
            kind,
            schema_version,
            canonical_payload,
        }
    }

    /// Returns the artifact kind.
    #[must_use]
    pub const fn kind(&self) -> &ArtifactKind {
        &self.kind
    }

    /// Returns the payload schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns the exact canonical payload bytes.
    #[must_use]
    pub fn canonical_payload(&self) -> &[u8] {
        &self.canonical_payload
    }

    /// Encodes the canonical artifact envelope.
    pub fn encode(&self) -> Result<Vec<u8>, ArtifactError> {
        let kind = self.kind.as_str().as_bytes();
        let kind_len =
            u32::try_from(kind.len()).map_err(|_| ArtifactError::KindTooLong(kind.len()))?;
        let payload_len = u64::try_from(self.canonical_payload.len())
            .map_err(|_| ArtifactError::PayloadTooLong(self.canonical_payload.len()))?;

        let capacity = ARTIFACT_DOMAIN
            .len()
            .checked_add(2)
            .and_then(|length| length.checked_add(4))
            .and_then(|length| length.checked_add(kind.len()))
            .and_then(|length| length.checked_add(4))
            .and_then(|length| length.checked_add(8))
            .and_then(|length| length.checked_add(self.canonical_payload.len()))
            .ok_or(ArtifactError::EnvelopeTooLong)?;

        let mut encoded = Vec::with_capacity(capacity);
        encoded.extend_from_slice(ARTIFACT_DOMAIN);
        encoded.extend_from_slice(&ARTIFACT_WIRE_VERSION.to_be_bytes());
        encoded.extend_from_slice(&kind_len.to_be_bytes());
        encoded.extend_from_slice(kind);
        encoded.extend_from_slice(&self.schema_version.to_be_bytes());
        encoded.extend_from_slice(&payload_len.to_be_bytes());
        encoded.extend_from_slice(&self.canonical_payload);
        Ok(encoded)
    }

    /// Decodes one complete canonical artifact envelope.
    pub fn decode(encoded: &[u8]) -> Result<Self, ArtifactError> {
        let mut cursor = Cursor::new(encoded);

        if cursor.take(ARTIFACT_DOMAIN.len())? != ARTIFACT_DOMAIN {
            return Err(ArtifactError::InvalidDomain);
        }

        let wire_version = cursor.read_u16()?;
        if wire_version != ARTIFACT_WIRE_VERSION {
            return Err(ArtifactError::UnsupportedWireVersion(wire_version));
        }

        let kind_len =
            usize::try_from(cursor.read_u32()?).map_err(|_| ArtifactError::LengthOverflow)?;
        let kind_bytes = cursor.take(kind_len)?;
        let kind_text = std::str::from_utf8(kind_bytes).map_err(ArtifactError::InvalidKindUtf8)?;
        let kind = ArtifactKind::new(kind_text)?;

        let schema_version = cursor.read_u32()?;
        let payload_len =
            usize::try_from(cursor.read_u64()?).map_err(|_| ArtifactError::LengthOverflow)?;
        let canonical_payload = cursor.take(payload_len)?.to_vec();

        if !cursor.is_finished() {
            return Err(ArtifactError::TrailingBytes(cursor.remaining()));
        }

        Ok(Self {
            kind,
            schema_version,
            canonical_payload,
        })
    }

    /// Calculates the SHA-256 reference of the complete canonical envelope.
    pub fn artifact_ref(&self) -> Result<ArtifactRef, ArtifactError> {
        let digest = Sha256::digest(self.encode()?);
        Ok(ArtifactRef::from_bytes(digest.into()))
    }
}

#[derive(Debug)]
struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], ArtifactError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(ArtifactError::LengthOverflow)?;
        let value = self
            .bytes
            .get(self.position..end)
            .ok_or(ArtifactError::TruncatedEnvelope)?;
        self.position = end;
        Ok(value)
    }

    fn read_u16(&mut self) -> Result<u16, ArtifactError> {
        let bytes: [u8; 2] = self
            .take(2)?
            .try_into()
            .map_err(|_| ArtifactError::TruncatedEnvelope)?;
        Ok(u16::from_be_bytes(bytes))
    }

    fn read_u32(&mut self) -> Result<u32, ArtifactError> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| ArtifactError::TruncatedEnvelope)?;
        Ok(u32::from_be_bytes(bytes))
    }

    fn read_u64(&mut self) -> Result<u64, ArtifactError> {
        let bytes: [u8; 8] = self
            .take(8)?
            .try_into()
            .map_err(|_| ArtifactError::TruncatedEnvelope)?;
        Ok(u64::from_be_bytes(bytes))
    }

    const fn is_finished(&self) -> bool {
        self.position == self.bytes.len()
    }

    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

/// Errors from artifact kind, reference, or envelope validation.
#[derive(Debug, Error)]
pub enum ArtifactError {
    #[error("artifact kind cannot be empty")]
    EmptyKind,

    #[error("invalid artifact kind {0:?}; expected [a-z][a-z0-9._-]*")]
    InvalidKind(String),

    #[error("artifact kind is too long: {0} bytes")]
    KindTooLong(usize),

    #[error("canonical payload is too long: {0} bytes")]
    PayloadTooLong(usize),

    #[error("canonical envelope length overflow")]
    EnvelopeTooLong,

    #[error("decoded length does not fit this platform")]
    LengthOverflow,

    #[error("invalid artifact domain separator")]
    InvalidDomain,

    #[error("unsupported artifact wire version {0}")]
    UnsupportedWireVersion(u16),

    #[error("artifact envelope is truncated")]
    TruncatedEnvelope,

    #[error("artifact kind is not valid UTF-8")]
    InvalidKindUtf8(#[source] std::str::Utf8Error),

    #[error("artifact envelope contains {0} trailing bytes")]
    TrailingBytes(usize),

    #[error("artifact reference is not valid hexadecimal")]
    InvalidReferenceHex(#[source] hex::FromHexError),

    #[error("artifact reference must be 32 bytes, got {0}")]
    InvalidReferenceLength(usize),
}

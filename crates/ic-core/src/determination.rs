use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef,
    BindingVersionRef, GrainRef, HorizonRef, ScopeRef, SupportRef, TypedFormRef,
};

/// Canonical artifact kind for claim-local determination presentations.
pub const DETERMINATION_PRESENTATION_ARTIFACT_KIND: &str = "ic.determination-presentation";
/// Payload schema version for claim-local determination presentations.
pub const DETERMINATION_PRESENTATION_SCHEMA_VERSION: u32 = 1;

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

artifact_reference!(DeterminationPresentationRef);
artifact_reference!(DistinctionRef);
artifact_reference!(RelationalWebRef);

/// The side of a distinction currently occupied by the source determination.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Orientation {
    X,
    Y,
}

impl Orientation {
    const fn tag(self) -> u8 {
        match self {
            Self::X => 0,
            Self::Y => 1,
        }
    }

    const fn from_tag(tag: u8) -> Option<Self> {
        match tag {
            0 => Some(Self::X),
            1 => Some(Self::Y),
            _ => None,
        }
    }
}

/// An explicit versioned support/dependency web for one standing source determination.
///
/// This is neither a complete fact store nor a negation, exterior, return, or revision record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeterminationPresentation {
    distinction: DistinctionRef,
    orientation: Orientation,
    source: TypedFormRef,
    web: RelationalWebRef,
    binding: BindingVersionRef,
    scope: ScopeRef,
    applicability: ApplicabilityRef,
    grain: GrainRef,
    horizon: HorizonRef,
    support: SupportRef,
    predecessor: Option<DeterminationPresentationRef>,
}

impl DeterminationPresentation {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        distinction: DistinctionRef,
        orientation: Orientation,
        source: TypedFormRef,
        web: RelationalWebRef,
        binding: BindingVersionRef,
        scope: ScopeRef,
        applicability: ApplicabilityRef,
        grain: GrainRef,
        horizon: HorizonRef,
        support: SupportRef,
        predecessor: Option<DeterminationPresentationRef>,
    ) -> Self {
        Self {
            distinction,
            orientation,
            source,
            web,
            binding,
            scope,
            applicability,
            grain,
            horizon,
            support,
            predecessor,
        }
    }

    #[must_use]
    pub const fn distinction(&self) -> DistinctionRef {
        self.distinction
    }

    #[must_use]
    pub const fn orientation(&self) -> Orientation {
        self.orientation
    }

    #[must_use]
    pub const fn source(&self) -> TypedFormRef {
        self.source
    }

    #[must_use]
    pub const fn web(&self) -> RelationalWebRef {
        self.web
    }

    #[must_use]
    pub const fn binding(&self) -> BindingVersionRef {
        self.binding
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
    pub const fn support(&self) -> SupportRef {
        self.support
    }

    #[must_use]
    pub const fn predecessor(&self) -> Option<DeterminationPresentationRef> {
        self.predecessor
    }

    pub fn canonical_payload(&self) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(353);
        reference(&mut encoded, self.distinction.as_artifact_ref());
        encoded.push(self.orientation.tag());
        reference(&mut encoded, self.source.as_artifact_ref());
        reference(&mut encoded, self.web.as_artifact_ref());
        reference(&mut encoded, self.binding.as_artifact_ref());
        reference(&mut encoded, self.scope.as_artifact_ref());
        reference(&mut encoded, self.applicability.as_artifact_ref());
        reference(&mut encoded, self.grain.as_artifact_ref());
        reference(&mut encoded, self.horizon.as_artifact_ref());
        reference(&mut encoded, self.support.as_artifact_ref());
        match self.predecessor {
            None => encoded.push(0),
            Some(predecessor) => {
                encoded.push(1);
                reference(&mut encoded, predecessor.as_artifact_ref());
            }
        }
        encoded
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, DeterminationPresentationError> {
        let mut cursor = Cursor::new(payload);
        let distinction = DistinctionRef::from_artifact_ref(cursor.reference()?);
        let orientation = Orientation::from_tag(cursor.byte()?)
            .ok_or(DeterminationPresentationError::UnknownOrientation)?;
        let source = TypedFormRef::from_artifact_ref(cursor.reference()?);
        let web = RelationalWebRef::from_artifact_ref(cursor.reference()?);
        let binding = BindingVersionRef::from_artifact_ref(cursor.reference()?);
        let scope = ScopeRef::from_artifact_ref(cursor.reference()?);
        let applicability = ApplicabilityRef::from_artifact_ref(cursor.reference()?);
        let grain = GrainRef::from_artifact_ref(cursor.reference()?);
        let horizon = HorizonRef::from_artifact_ref(cursor.reference()?);
        let support = SupportRef::from_artifact_ref(cursor.reference()?);
        let predecessor = match cursor.byte()? {
            0 => None,
            1 => Some(DeterminationPresentationRef::from_artifact_ref(
                cursor.reference()?,
            )),
            tag => return Err(DeterminationPresentationError::UnknownOptionalTag(tag)),
        };
        if !cursor.finished() {
            return Err(DeterminationPresentationError::TrailingPayloadBytes(
                cursor.remaining(),
            ));
        }
        Ok(Self::new(
            distinction,
            orientation,
            source,
            web,
            binding,
            scope,
            applicability,
            grain,
            horizon,
            support,
            predecessor,
        ))
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, DeterminationPresentationError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(DETERMINATION_PRESENTATION_ARTIFACT_KIND)?,
            DETERMINATION_PRESENTATION_SCHEMA_VERSION,
            self.canonical_payload(),
        ))
    }

    pub fn determination_presentation_ref(
        &self,
    ) -> Result<DeterminationPresentationRef, DeterminationPresentationError> {
        Ok(DeterminationPresentationRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(
        envelope: &ArtifactEnvelope,
    ) -> Result<Self, DeterminationPresentationError> {
        if envelope.kind().as_str() != DETERMINATION_PRESENTATION_ARTIFACT_KIND {
            return Err(DeterminationPresentationError::UnexpectedArtifactKind {
                expected: DETERMINATION_PRESENTATION_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != DETERMINATION_PRESENTATION_SCHEMA_VERSION {
            return Err(DeterminationPresentationError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = vec![
            self.distinction.as_artifact_ref(),
            self.source.as_artifact_ref(),
            self.web.as_artifact_ref(),
            self.binding.as_artifact_ref(),
            self.scope.as_artifact_ref(),
            self.applicability.as_artifact_ref(),
            self.grain.as_artifact_ref(),
            self.horizon.as_artifact_ref(),
            self.support.as_artifact_ref(),
        ];
        if let Some(predecessor) = self.predecessor {
            references.push(predecessor.as_artifact_ref());
        }
        references
    }
}

fn reference(encoded: &mut Vec<u8>, reference: ArtifactRef) {
    encoded.extend_from_slice(reference.as_bytes());
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], DeterminationPresentationError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(DeterminationPresentationError::PayloadLengthOverflow)?;
        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(DeterminationPresentationError::TruncatedPayload)?;
        self.position = end;
        Ok(bytes)
    }

    fn byte(&mut self) -> Result<u8, DeterminationPresentationError> {
        Ok(self.take(1)?[0])
    }

    fn reference(&mut self) -> Result<ArtifactRef, DeterminationPresentationError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| DeterminationPresentationError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(bytes))
    }

    const fn finished(&self) -> bool {
        self.position == self.bytes.len()
    }

    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

#[derive(Debug, Error)]
pub enum DeterminationPresentationError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("determination-presentation payload is truncated")]
    TruncatedPayload,
    #[error("determination-presentation payload length overflows this platform")]
    PayloadLengthOverflow,
    #[error("determination-presentation payload contains {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("determination-presentation payload has an unknown orientation")]
    UnknownOrientation,
    #[error("determination-presentation payload has an unknown optional tag {0}")]
    UnknownOptionalTag(u8),
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported determination-presentation schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

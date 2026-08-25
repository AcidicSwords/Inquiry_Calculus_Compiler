use std::{collections::BTreeSet, fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef,
    BindingVersionRef, FormulaCatalog, GrainRef, HorizonRef, ScopeRef, SupportRef, TypeCheckError,
    TypeError, TypedFormRef,
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

/// The checked source for determination presentations and their ancestry.
pub trait DeterminationCatalog: FormulaCatalog {
    /// Resolves a determination presentation by its claimed stable identity.
    fn resolve_determination_presentation(
        &self,
        reference: DeterminationPresentationRef,
    ) -> Option<DeterminationPresentation>;
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

    /// Revalidates the typed source and context-preserving predecessor ancestry.
    ///
    /// Relational-web admission and minimization remain separate: this check establishes only
    /// that the presentation names the source and ancestry it claims to name.
    pub fn check<C: DeterminationCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), DeterminationPresentationCheckError> {
        let reference = self.determination_presentation_ref()?;
        let mut visiting = BTreeSet::new();
        let mut completed = BTreeSet::new();
        self.check_inner(reference, catalog, &mut visiting, &mut completed)
    }

    fn check_inner<C: DeterminationCatalog>(
        &self,
        reference: DeterminationPresentationRef,
        catalog: &C,
        visiting: &mut BTreeSet<DeterminationPresentationRef>,
        completed: &mut BTreeSet<DeterminationPresentationRef>,
    ) -> Result<(), DeterminationPresentationCheckError> {
        if completed.contains(&reference) {
            return Ok(());
        }
        if !visiting.insert(reference) {
            return Err(DeterminationPresentationCheckError::CyclicPredecessor(
                reference,
            ));
        }
        let source = catalog.resolve_typed_form(self.source).ok_or(
            DeterminationPresentationCheckError::UnresolvedSource(self.source),
        )?;
        let calculated = source.typed_form_ref()?;
        if calculated != self.source {
            return Err(
                DeterminationPresentationCheckError::SourceReferenceIdentityMismatch {
                    reference: self.source,
                    calculated,
                },
            );
        }
        source.check(catalog)?;
        if source.binding() != self.binding {
            return Err(DeterminationPresentationCheckError::SourceBindingMismatch {
                expected: self.binding,
                actual: source.binding(),
            });
        }
        let checked = if let Some(predecessor_ref) = self.predecessor {
            let predecessor = catalog
                .resolve_determination_presentation(predecessor_ref)
                .ok_or(DeterminationPresentationCheckError::UnresolvedPredecessor(
                    predecessor_ref,
                ))?;
            let calculated = predecessor.determination_presentation_ref()?;
            if calculated != predecessor_ref {
                return Err(
                    DeterminationPresentationCheckError::PredecessorReferenceIdentityMismatch {
                        reference: predecessor_ref,
                        calculated,
                    },
                );
            }
            if let Some(field) = self.predecessor_context_difference(&predecessor) {
                return Err(
                    DeterminationPresentationCheckError::PredecessorContextMismatch { field },
                );
            }
            predecessor.check_inner(predecessor_ref, catalog, visiting, completed)
        } else {
            Ok(())
        };
        visiting.remove(&reference);
        if checked.is_ok() {
            completed.insert(reference);
        }
        checked
    }

    fn predecessor_context_difference(&self, predecessor: &Self) -> Option<&'static str> {
        if self.distinction != predecessor.distinction {
            Some("distinction")
        } else if self.orientation != predecessor.orientation {
            Some("orientation")
        } else if self.source != predecessor.source {
            Some("source")
        } else if self.binding != predecessor.binding {
            Some("binding")
        } else if self.scope != predecessor.scope {
            Some("scope")
        } else if self.applicability != predecessor.applicability {
            Some("applicability")
        } else if self.grain != predecessor.grain {
            Some("grain")
        } else if self.horizon != predecessor.horizon {
            Some("horizon")
        } else {
            None
        }
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

/// Errors from structural determination-presentation checking.
#[derive(Debug, Error)]
pub enum DeterminationPresentationCheckError {
    #[error(transparent)]
    Presentation(#[from] DeterminationPresentationError),
    #[error(transparent)]
    Type(#[from] TypeError),
    #[error(transparent)]
    TypeCheck(#[from] TypeCheckError),
    #[error("source typed form {0} is not available from the declared catalog")]
    UnresolvedSource(TypedFormRef),
    #[error("catalog source form {reference} hashes to {calculated}, not its claimed identity")]
    SourceReferenceIdentityMismatch {
        reference: TypedFormRef,
        calculated: TypedFormRef,
    },
    #[error("source form binding {actual} does not match presentation binding {expected}")]
    SourceBindingMismatch {
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },
    #[error("predecessor presentation {0} is not available from the declared catalog")]
    UnresolvedPredecessor(DeterminationPresentationRef),
    #[error("catalog predecessor {reference} hashes to {calculated}, not its claimed identity")]
    PredecessorReferenceIdentityMismatch {
        reference: DeterminationPresentationRef,
        calculated: DeterminationPresentationRef,
    },
    #[error("predecessor presentation changes indexed context field {field}")]
    PredecessorContextMismatch { field: &'static str },
    #[error("determination-presentation predecessor graph contains cycle at {0}")]
    CyclicPredecessor(DeterminationPresentationRef),
}

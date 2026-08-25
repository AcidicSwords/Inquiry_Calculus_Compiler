use std::{collections::BTreeSet, fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, IProgRef, RelationRef, TypeCatalog,
    TypeCheckError, TypeError, TypeRef,
};

/// Canonical artifact kind for first-order typed resolution paths.
pub const RESOLUTION_PATH_ARTIFACT_KIND: &str = "ic.resolution-path";
/// Payload schema version for first-order typed resolution paths.
pub const RESOLUTION_PATH_SCHEMA_VERSION: u32 = 1;

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

artifact_reference!(DecoderRef);
artifact_reference!(ResolutionPathRef);

/// First-order route syntax from an input type to an output type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResolutionPathIR {
    Identity,
    Decode {
        decoder: DecoderRef,
    },
    Relation {
        relation: RelationRef,
    },
    Compose {
        first: ResolutionPathRef,
        second: ResolutionPathRef,
    },
    Program {
        program: IProgRef,
    },
}

/// An inspectable typed route by which raw or represented input may later be resolved.
///
/// This artifact never runs a decoder, relation, or program, and cannot by itself make an answer
/// supported, complete, actual, checked, or warranted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolutionPath {
    input: TypeRef,
    output: TypeRef,
    path: ResolutionPathIR,
}

/// The minimal checked source for recursively composed resolution paths.
pub trait ResolutionCatalog: TypeCatalog {
    /// Resolves a resolution path by its claimed content identity.
    fn resolve_resolution_path(&self, reference: ResolutionPathRef) -> Option<ResolutionPath>;
}

impl ResolutionPath {
    #[must_use]
    pub const fn new(input: TypeRef, output: TypeRef, path: ResolutionPathIR) -> Self {
        Self {
            input,
            output,
            path,
        }
    }

    #[must_use]
    pub const fn input(&self) -> TypeRef {
        self.input
    }

    #[must_use]
    pub const fn output(&self) -> TypeRef {
        self.output
    }

    #[must_use]
    pub const fn path(&self) -> ResolutionPathIR {
        self.path
    }

    pub fn canonical_payload(&self) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(97);
        reference(&mut encoded, self.input.as_artifact_ref());
        reference(&mut encoded, self.output.as_artifact_ref());
        match self.path {
            ResolutionPathIR::Identity => encoded.push(0),
            ResolutionPathIR::Decode { decoder } => {
                encoded.push(1);
                reference(&mut encoded, decoder.as_artifact_ref());
            }
            ResolutionPathIR::Relation { relation } => {
                encoded.push(2);
                reference(&mut encoded, relation.as_artifact_ref());
            }
            ResolutionPathIR::Compose { first, second } => {
                encoded.push(3);
                reference(&mut encoded, first.as_artifact_ref());
                reference(&mut encoded, second.as_artifact_ref());
            }
            ResolutionPathIR::Program { program } => {
                encoded.push(4);
                reference(&mut encoded, program.as_artifact_ref());
            }
        }
        encoded
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, ResolutionPathError> {
        let mut cursor = Cursor::new(payload);
        let input = TypeRef::from_artifact_ref(cursor.reference()?);
        let output = TypeRef::from_artifact_ref(cursor.reference()?);
        let path = match cursor.byte()? {
            0 => ResolutionPathIR::Identity,
            1 => ResolutionPathIR::Decode {
                decoder: DecoderRef::from_artifact_ref(cursor.reference()?),
            },
            2 => ResolutionPathIR::Relation {
                relation: RelationRef::from_artifact_ref(cursor.reference()?),
            },
            3 => ResolutionPathIR::Compose {
                first: ResolutionPathRef::from_artifact_ref(cursor.reference()?),
                second: ResolutionPathRef::from_artifact_ref(cursor.reference()?),
            },
            4 => ResolutionPathIR::Program {
                program: IProgRef::from_artifact_ref(cursor.reference()?),
            },
            tag => return Err(ResolutionPathError::UnknownPathTag(tag)),
        };
        if !cursor.finished() {
            return Err(ResolutionPathError::TrailingPayloadBytes(
                cursor.remaining(),
            ));
        }
        Ok(Self::new(input, output, path))
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, ResolutionPathError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(RESOLUTION_PATH_ARTIFACT_KIND)?,
            RESOLUTION_PATH_SCHEMA_VERSION,
            self.canonical_payload(),
        ))
    }

    pub fn resolution_path_ref(&self) -> Result<ResolutionPathRef, ResolutionPathError> {
        Ok(ResolutionPathRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, ResolutionPathError> {
        if envelope.kind().as_str() != RESOLUTION_PATH_ARTIFACT_KIND {
            return Err(ResolutionPathError::UnexpectedArtifactKind {
                expected: RESOLUTION_PATH_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != RESOLUTION_PATH_SCHEMA_VERSION {
            return Err(ResolutionPathError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    /// Checks type identities and exact composition interfaces without executing any route.
    pub fn check<C: ResolutionCatalog>(&self, catalog: &C) -> Result<(), ResolutionPathCheckError> {
        let mut visiting = BTreeSet::new();
        let mut completed = BTreeSet::new();
        self.check_inner(catalog, &mut visiting, &mut completed)
    }

    fn check_inner<C: ResolutionCatalog>(
        &self,
        catalog: &C,
        visiting: &mut BTreeSet<ResolutionPathRef>,
        completed: &mut BTreeSet<ResolutionPathRef>,
    ) -> Result<(), ResolutionPathCheckError> {
        check_type(self.input, catalog)?;
        check_type(self.output, catalog)?;
        match self.path {
            ResolutionPathIR::Identity => {
                if self.input != self.output {
                    return Err(ResolutionPathCheckError::IdentityTypeMismatch {
                        input: self.input,
                        output: self.output,
                    });
                }
            }
            ResolutionPathIR::Compose { first, second } => {
                let first_path = resolve_path(first, catalog, visiting, completed)?;
                let second_path = resolve_path(second, catalog, visiting, completed)?;
                if self.input != first_path.input() {
                    return Err(ResolutionPathCheckError::ComposeInputMismatch {
                        declared: self.input,
                        first_input: first_path.input(),
                    });
                }
                if first_path.output() != second_path.input() {
                    return Err(ResolutionPathCheckError::ComposeMiddleMismatch {
                        first_output: first_path.output(),
                        second_input: second_path.input(),
                    });
                }
                if self.output != second_path.output() {
                    return Err(ResolutionPathCheckError::ComposeOutputMismatch {
                        declared: self.output,
                        second_output: second_path.output(),
                    });
                }
            }
            ResolutionPathIR::Decode { .. }
            | ResolutionPathIR::Relation { .. }
            | ResolutionPathIR::Program { .. } => {}
        }
        Ok(())
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = vec![self.input.as_artifact_ref(), self.output.as_artifact_ref()];
        match self.path {
            ResolutionPathIR::Identity => {}
            ResolutionPathIR::Decode { decoder } => references.push(decoder.as_artifact_ref()),
            ResolutionPathIR::Relation { relation } => references.push(relation.as_artifact_ref()),
            ResolutionPathIR::Compose { first, second } => {
                references.extend([first.as_artifact_ref(), second.as_artifact_ref()]);
            }
            ResolutionPathIR::Program { program } => references.push(program.as_artifact_ref()),
        }
        references
    }
}

fn resolve_path<C: ResolutionCatalog>(
    reference_value: ResolutionPathRef,
    catalog: &C,
    visiting: &mut BTreeSet<ResolutionPathRef>,
    completed: &mut BTreeSet<ResolutionPathRef>,
) -> Result<ResolutionPath, ResolutionPathCheckError> {
    if completed.contains(&reference_value) {
        return catalog
            .resolve_resolution_path(reference_value)
            .ok_or(ResolutionPathCheckError::UnresolvedPath(reference_value));
    }
    if !visiting.insert(reference_value) {
        return Err(ResolutionPathCheckError::CyclicComposition(reference_value));
    }
    let path = catalog
        .resolve_resolution_path(reference_value)
        .ok_or(ResolutionPathCheckError::UnresolvedPath(reference_value))?;
    let calculated = path.resolution_path_ref()?;
    if calculated != reference_value {
        return Err(ResolutionPathCheckError::PathIdentityMismatch {
            reference: reference_value,
            calculated,
        });
    }
    path.check_inner(catalog, visiting, completed)?;
    visiting.remove(&reference_value);
    completed.insert(reference_value);
    Ok(path)
}

fn check_type<C: TypeCatalog>(
    reference_value: TypeRef,
    catalog: &C,
) -> Result<(), ResolutionPathCheckError> {
    let ty = catalog
        .resolve_type(reference_value)
        .ok_or(ResolutionPathCheckError::UnresolvedType(reference_value))?;
    let calculated = ty.type_ref()?;
    if calculated != reference_value {
        return Err(ResolutionPathCheckError::TypeIdentityMismatch {
            reference: reference_value,
            calculated,
        });
    }
    ty.check(catalog)?;
    Ok(())
}

fn reference(encoded: &mut Vec<u8>, value: ArtifactRef) {
    encoded.extend_from_slice(value.as_bytes());
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn reference(&mut self) -> Result<ArtifactRef, ResolutionPathError> {
        let end = self
            .position
            .checked_add(32)
            .ok_or(ResolutionPathError::PayloadLengthOverflow)?;
        let bytes: [u8; 32] = self
            .bytes
            .get(self.position..end)
            .ok_or(ResolutionPathError::TruncatedPayload)?
            .try_into()
            .map_err(|_| ResolutionPathError::TruncatedPayload)?;
        self.position = end;
        Ok(ArtifactRef::from_bytes(bytes))
    }

    fn byte(&mut self) -> Result<u8, ResolutionPathError> {
        let byte = *self
            .bytes
            .get(self.position)
            .ok_or(ResolutionPathError::TruncatedPayload)?;
        self.position = self
            .position
            .checked_add(1)
            .ok_or(ResolutionPathError::PayloadLengthOverflow)?;
        Ok(byte)
    }

    const fn finished(&self) -> bool {
        self.position == self.bytes.len()
    }

    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

#[derive(Debug, Error)]
pub enum ResolutionPathError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("resolution-path payload is truncated")]
    TruncatedPayload,
    #[error("resolution-path payload length overflows this platform")]
    PayloadLengthOverflow,
    #[error("resolution-path payload contains {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("resolution-path payload has an unknown path tag {0}")]
    UnknownPathTag(u8),
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported resolution-path schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

#[derive(Debug, Error)]
pub enum ResolutionPathCheckError {
    #[error(transparent)]
    Encoding(#[from] ResolutionPathError),
    #[error(transparent)]
    Type(#[from] TypeError),
    #[error(transparent)]
    TypeCheck(#[from] TypeCheckError),
    #[error("resolution type {0} is unavailable")]
    UnresolvedType(TypeRef),
    #[error("resolution type {reference} hashes to {calculated}, not its claimed identity")]
    TypeIdentityMismatch {
        reference: TypeRef,
        calculated: TypeRef,
    },
    #[error("identity resolution path has input {input} and output {output}")]
    IdentityTypeMismatch { input: TypeRef, output: TypeRef },
    #[error("resolution path {0} is unavailable")]
    UnresolvedPath(ResolutionPathRef),
    #[error("resolution path {reference} hashes to {calculated}, not its claimed identity")]
    PathIdentityMismatch {
        reference: ResolutionPathRef,
        calculated: ResolutionPathRef,
    },
    #[error("resolution path composition contains a cycle at {0}")]
    CyclicComposition(ResolutionPathRef),
    #[error("composed path declares input {declared}, but first path starts at {first_input}")]
    ComposeInputMismatch {
        declared: TypeRef,
        first_input: TypeRef,
    },
    #[error("composed path has first output {first_output}, but second input {second_input}")]
    ComposeMiddleMismatch {
        first_output: TypeRef,
        second_input: TypeRef,
    },
    #[error("composed path declares output {declared}, but second path ends at {second_output}")]
    ComposeOutputMismatch {
        declared: TypeRef,
        second_output: TypeRef,
    },
}

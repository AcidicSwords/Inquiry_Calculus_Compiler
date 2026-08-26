use std::{collections::BTreeSet, fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, OpenQueryCatalog,
    OpenQueryCheckError, OpenQueryError, QueryRef, TypeCheckError, TypeError, TypeRef, TypeSymbol,
    TypedFormRef,
};

/// Canonical artifact kind for first-order inquiry programs.
pub const IPROG_ARTIFACT_KIND: &str = "ic.iprog";
/// Payload schema version for first-order inquiry programs.
pub const IPROG_SCHEMA_VERSION: u32 = 2;

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

/// One named, explicitly supplied lexical value for an `IProgIR::Ask` continuation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgramBinding {
    name: TypeSymbol,
    value: TypedFormRef,
}

impl ProgramBinding {
    #[must_use]
    pub const fn new(name: TypeSymbol, value: TypedFormRef) -> Self {
        Self { name, value }
    }

    #[must_use]
    pub const fn name(&self) -> &TypeSymbol {
        &self.name
    }

    #[must_use]
    pub const fn value(&self) -> TypedFormRef {
        self.value
    }
}

/// Capture-safe, first-order inquiry-program syntax.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IProgIR {
    Return {
        value: TypedFormRef,
    },
    Ask {
        question: QueryRef,
        environment: Vec<ProgramBinding>,
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

/// The checked source for first-order inquiry-program references.
pub trait IProgCatalog: OpenQueryCatalog {
    /// Resolves a first-order program by its claimed stable identity.
    fn resolve_iprog(&self, reference: IProgRef) -> Option<IProgArtifact>;
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
                environment,
                answer_slot,
                continuation,
            } => {
                encoded.push(1);
                reference(&mut encoded, question.as_artifact_ref());
                bindings(&mut encoded, environment)?;
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
                let environment = c.bindings()?;
                let answer_slot = c.symbol()?;
                let continuation = IProgRef::from_artifact_ref(c.reference()?);
                IProgIR::Ask {
                    question,
                    environment,
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

    /// Checks the structurally represented references of this first-order program.
    ///
    /// This does not interpret a supported answer or execute a program.  In particular,
    /// the answer-set type of an open query is not yet a Phase 3 artifact.  It verifies
    /// every type, typed form, query, explicit environment value, and continuation that
    /// the current representation can name.
    pub fn check<C: IProgCatalog>(&self, catalog: &C) -> Result<(), IProgCheckError> {
        let reference = self.iprog_ref()?;
        let mut visiting = BTreeSet::new();
        let mut completed = BTreeSet::new();
        self.check_inner(reference, catalog, &mut visiting, &mut completed)
    }

    fn check_inner<C: IProgCatalog>(
        &self,
        reference: IProgRef,
        catalog: &C,
        visiting: &mut BTreeSet<IProgRef>,
        completed: &mut BTreeSet<IProgRef>,
    ) -> Result<(), IProgCheckError> {
        if completed.contains(&reference) {
            return Ok(());
        }
        if !visiting.insert(reference) {
            return Err(IProgCheckError::CyclicContinuation(reference));
        }
        check_type_reference(self.result, catalog)?;
        let checked = match &self.program {
            IProgIR::Return { value } => {
                let actual = check_typed_form_reference(*value, catalog)?;
                if actual != self.result {
                    Err(IProgCheckError::ReturnTypeMismatch {
                        expected: self.result,
                        actual,
                    })
                } else {
                    Ok(())
                }
            }
            IProgIR::Ask {
                question,
                environment,
                answer_slot,
                continuation,
            } => {
                if environment
                    .iter()
                    .any(|binding| binding.name() == answer_slot)
                {
                    return Err(IProgCheckError::AnswerSlotShadowsEnvironment(
                        answer_slot.as_str().to_owned(),
                    ));
                }
                let query = catalog
                    .resolve_open_query(*question)
                    .ok_or(IProgCheckError::UnresolvedQuery(*question))?;
                let calculated = query.query_ref()?;
                if calculated != *question {
                    return Err(IProgCheckError::QueryReferenceIdentityMismatch {
                        reference: *question,
                        calculated,
                    });
                }
                query.check(catalog)?;
                for binding in environment {
                    check_typed_form_reference(binding.value, catalog)?;
                }
                let next = catalog
                    .resolve_iprog(*continuation)
                    .ok_or(IProgCheckError::UnresolvedContinuation(*continuation))?;
                let calculated = next.iprog_ref()?;
                if calculated != *continuation {
                    return Err(IProgCheckError::ContinuationReferenceIdentityMismatch {
                        reference: *continuation,
                        calculated,
                    });
                }
                if next.result != self.result {
                    return Err(IProgCheckError::ContinuationResultTypeMismatch {
                        expected: self.result,
                        actual: next.result,
                    });
                }
                next.check_inner(*continuation, catalog, visiting, completed)
            }
        };
        visiting.remove(&reference);
        if checked.is_ok() {
            completed.insert(reference);
        }
        checked
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut refs = vec![self.result.as_artifact_ref()];
        match &self.program {
            IProgIR::Return { value } => refs.push(value.as_artifact_ref()),
            IProgIR::Ask {
                question,
                environment,
                continuation,
                ..
            } => {
                refs.push(question.as_artifact_ref());
                refs.extend(
                    environment
                        .iter()
                        .map(|binding| binding.value.as_artifact_ref()),
                );
                refs.push(continuation.as_artifact_ref());
            }
        }
        refs
    }
}

fn check_type_reference<C: IProgCatalog>(
    reference: TypeRef,
    catalog: &C,
) -> Result<(), IProgCheckError> {
    let ty = catalog
        .resolve_type(reference)
        .ok_or(IProgCheckError::UnresolvedResultType(reference))?;
    let calculated = ty.type_ref()?;
    if calculated != reference {
        return Err(IProgCheckError::ResultTypeReferenceIdentityMismatch {
            reference,
            calculated,
        });
    }
    ty.check(catalog)?;
    Ok(())
}

fn check_typed_form_reference<C: IProgCatalog>(
    reference: TypedFormRef,
    catalog: &C,
) -> Result<TypeRef, IProgCheckError> {
    let form = catalog
        .resolve_typed_form(reference)
        .ok_or(IProgCheckError::UnresolvedTypedForm(reference))?;
    let calculated = form.typed_form_ref()?;
    if calculated != reference {
        return Err(IProgCheckError::TypedFormReferenceIdentityMismatch {
            reference,
            calculated,
        });
    }
    form.check(catalog)?;
    Ok(form.ty())
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

fn bindings(encoded: &mut Vec<u8>, bindings: &[ProgramBinding]) -> Result<(), IProgError> {
    let length = u32::try_from(bindings.len())
        .map_err(|_| IProgError::EnvironmentTooLarge(bindings.len()))?;
    let mut names = std::collections::BTreeSet::new();
    for binding in bindings {
        if !names.insert(binding.name.as_str()) {
            return Err(IProgError::DuplicateEnvironmentBinding(
                binding.name.as_str().to_owned(),
            ));
        }
    }
    encoded.extend_from_slice(&length.to_be_bytes());
    for binding in bindings {
        text(encoded, binding.name.as_str())?;
        reference(encoded, binding.value.as_artifact_ref());
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
    fn bindings(&mut self) -> Result<Vec<ProgramBinding>, IProgError> {
        let b: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| IProgError::TruncatedPayload)?;
        let length = usize::try_from(u32::from_be_bytes(b))
            .map_err(|_| IProgError::PayloadLengthOverflow)?;
        let minimum_bytes = length
            .checked_mul(36)
            .ok_or(IProgError::PayloadLengthOverflow)?;
        if minimum_bytes > self.remaining() {
            return Err(IProgError::TruncatedPayload);
        }
        let mut bindings = Vec::with_capacity(length);
        let mut names = std::collections::BTreeSet::new();
        for _ in 0..length {
            let name = self.symbol()?;
            if !names.insert(name.as_str().to_owned()) {
                return Err(IProgError::DuplicateEnvironmentBinding(
                    name.as_str().to_owned(),
                ));
            }
            let value = TypedFormRef::from_artifact_ref(self.reference()?);
            bindings.push(ProgramBinding::new(name, value));
        }
        Ok(bindings)
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
    #[error("explicit environment is too large: {0} bindings")]
    EnvironmentTooLarge(usize),
    #[error("duplicate explicit environment binding {0:?}")]
    DuplicateEnvironmentBinding(String),
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

/// Errors from structural first-order inquiry-program checking.
#[derive(Debug, Error)]
pub enum IProgCheckError {
    #[error(transparent)]
    IProg(#[from] IProgError),
    #[error(transparent)]
    Type(#[from] TypeError),
    #[error(transparent)]
    TypeCheck(#[from] TypeCheckError),
    #[error(transparent)]
    OpenQuery(#[from] OpenQueryCheckError),
    #[error(transparent)]
    OpenQueryEncoding(#[from] OpenQueryError),
    #[error("result type {0} is not available from the declared catalog")]
    UnresolvedResultType(TypeRef),
    #[error("catalog result type {reference} hashes to {calculated}, not its claimed identity")]
    ResultTypeReferenceIdentityMismatch {
        reference: TypeRef,
        calculated: TypeRef,
    },
    #[error("typed form {0} is not available from the declared catalog")]
    UnresolvedTypedForm(TypedFormRef),
    #[error("catalog typed form {reference} hashes to {calculated}, not its claimed identity")]
    TypedFormReferenceIdentityMismatch {
        reference: TypedFormRef,
        calculated: TypedFormRef,
    },
    #[error("return value has type {actual}, expected program result {expected}")]
    ReturnTypeMismatch { expected: TypeRef, actual: TypeRef },
    #[error("open query {0} is not available from the declared catalog")]
    UnresolvedQuery(QueryRef),
    #[error("catalog query {reference} hashes to {calculated}, not its claimed identity")]
    QueryReferenceIdentityMismatch {
        reference: QueryRef,
        calculated: QueryRef,
    },
    #[error("continuation program {0} is not available from the declared catalog")]
    UnresolvedContinuation(IProgRef),
    #[error("catalog continuation {reference} hashes to {calculated}, not its claimed identity")]
    ContinuationReferenceIdentityMismatch {
        reference: IProgRef,
        calculated: IProgRef,
    },
    #[error("continuation result type {actual} does not match enclosing result {expected}")]
    ContinuationResultTypeMismatch { expected: TypeRef, actual: TypeRef },
    #[error("answer slot {0:?} shadows an explicit environment binding")]
    AnswerSlotShadowsEnvironment(String),
    #[error("first-order program continuation graph contains cycle at {0}")]
    CyclicContinuation(IProgRef),
}

//! Nonexecuting, typed quotations of source and runtime programs.
//!
//! `Code` retains only a checked program identity, its result type, binding, and compiler
//! coordinate.  It neither evaluates a program nor creates an actuality event.

use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, BindingVersionRef, IProgCatalog,
    IProgCheckError, IProgError, IProgRef, TypeRef,
};

/// Canonical artifact kind for nonexecuting program quotations.
pub const CODE_ARTIFACT_KIND: &str = "ic.code";
/// Payload schema version for nonexecuting program quotations.
pub const CODE_SCHEMA_VERSION: u32 = 1;

/// A reference to an immutable runtime-program artifact owned by `ic-runtime`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RuntimeProgramRef(ArtifactRef);

impl RuntimeProgramRef {
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }
    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}
impl fmt::Display for RuntimeProgramRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}
impl FromStr for RuntimeProgramRef {
    type Err = ArtifactError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// A reference known to identify a canonical code quotation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CodeRef(ArtifactRef);
impl CodeRef {
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }
    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}
impl fmt::Display for CodeRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}
impl FromStr for CodeRef {
    type Err = ArtifactError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// The quoted, nonexecuting program family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodeIR {
    Source { program: IProgRef },
    Runtime { program: RuntimeProgramRef },
}

/// A typed quotation scoped by an immutable binding and compiler version.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CodeArtifact {
    result: TypeRef,
    binding: BindingVersionRef,
    compiler_version: ArtifactRef,
    code: CodeIR,
}

/// A catalog that supplies checked quotation referents and explicit interpretation admission.
pub trait CodeCatalog: IProgCatalog {
    /// Resolves the immutable metadata of a runtime program quotation.
    fn resolve_runtime_program(
        &self,
        reference: RuntimeProgramRef,
    ) -> Option<(TypeRef, BindingVersionRef, ArtifactRef)>;

    /// Whether this exact binding/compiler coordinate admits this quoted family.
    fn admits_code_interpretation(
        &self,
        binding: BindingVersionRef,
        compiler_version: ArtifactRef,
        kind: CodeInterpretationKind,
    ) -> bool;
}

/// The family selected by a successful interpretation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CodeInterpretationKind {
    Source,
    Runtime,
}

/// A successful interpretation returns a program reference only; it never runs it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodeInterpretation {
    Source(IProgRef),
    Runtime(RuntimeProgramRef),
}

impl CodeArtifact {
    #[must_use]
    pub const fn new(
        result: TypeRef,
        binding: BindingVersionRef,
        compiler_version: ArtifactRef,
        code: CodeIR,
    ) -> Self {
        Self {
            result,
            binding,
            compiler_version,
            code,
        }
    }
    #[must_use]
    pub const fn result(self) -> TypeRef {
        self.result
    }
    #[must_use]
    pub const fn binding(self) -> BindingVersionRef {
        self.binding
    }
    #[must_use]
    pub const fn compiler_version(self) -> ArtifactRef {
        self.compiler_version
    }
    #[must_use]
    pub const fn code(self) -> CodeIR {
        self.code
    }

    pub fn canonical_payload(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(129);
        bytes.extend_from_slice(self.result.as_artifact_ref().as_bytes());
        bytes.extend_from_slice(self.binding.as_artifact_ref().as_bytes());
        bytes.extend_from_slice(self.compiler_version.as_bytes());
        match self.code {
            CodeIR::Source { program } => {
                bytes.push(0);
                bytes.extend_from_slice(program.as_artifact_ref().as_bytes());
            }
            CodeIR::Runtime { program } => {
                bytes.push(1);
                bytes.extend_from_slice(program.as_artifact_ref().as_bytes());
            }
        }
        bytes
    }
    pub fn decode_payload(payload: &[u8]) -> Result<Self, CodeError> {
        if payload.len() != 129 {
            return Err(CodeError::InvalidPayloadLength(payload.len()));
        }
        let reference = |offset| {
            ArtifactRef::from_bytes(
                payload[offset..offset + 32]
                    .try_into()
                    .expect("fixed length"),
            )
        };
        let result = TypeRef::from_artifact_ref(reference(0));
        let binding = BindingVersionRef::from_artifact_ref(reference(32));
        let compiler_version = reference(64);
        let program = reference(97);
        let code = match payload[96] {
            0 => CodeIR::Source {
                program: IProgRef::from_artifact_ref(program),
            },
            1 => CodeIR::Runtime {
                program: RuntimeProgramRef::from_artifact_ref(program),
            },
            tag => return Err(CodeError::UnknownTag(tag)),
        };
        Ok(Self::new(result, binding, compiler_version, code))
    }
    pub fn envelope(self) -> Result<ArtifactEnvelope, CodeError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(CODE_ARTIFACT_KIND)?,
            CODE_SCHEMA_VERSION,
            self.canonical_payload(),
        ))
    }
    pub fn code_ref(self) -> Result<CodeRef, CodeError> {
        Ok(CodeRef::from_artifact_ref(self.envelope()?.artifact_ref()?))
    }
    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, CodeError> {
        if envelope.kind().as_str() != CODE_ARTIFACT_KIND {
            return Err(CodeError::UnexpectedArtifactKind {
                expected: CODE_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != CODE_SCHEMA_VERSION {
            return Err(CodeError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }
    pub fn check<C: CodeCatalog>(self, catalog: &C) -> Result<(), CodeCheckError> {
        match self.code {
            CodeIR::Source { program } => {
                let source = catalog
                    .resolve_iprog(program)
                    .ok_or(CodeCheckError::UnresolvedSource(program))?;
                if source.iprog_ref()? != program {
                    return Err(CodeCheckError::SourceIdentityMismatch {
                        reference: program,
                        calculated: source.iprog_ref()?,
                    });
                }
                source.check(catalog)?;
                if source.result() != self.result {
                    return Err(CodeCheckError::SourceResultMismatch {
                        expected: self.result,
                        actual: source.result(),
                    });
                }
                let result_type = catalog
                    .resolve_type(self.result)
                    .ok_or(CodeCheckError::UnresolvedResultType(self.result))?;
                if result_type.binding() != self.binding {
                    return Err(CodeCheckError::SourceBindingMismatch {
                        expected: self.binding,
                        actual: result_type.binding(),
                    });
                }
            }
            CodeIR::Runtime { program } => {
                let (result, binding, compiler_version) = catalog
                    .resolve_runtime_program(program)
                    .ok_or(CodeCheckError::UnresolvedRuntime(program))?;
                if result != self.result {
                    return Err(CodeCheckError::RuntimeResultMismatch {
                        expected: self.result,
                        actual: result,
                    });
                }
                if binding != self.binding {
                    return Err(CodeCheckError::RuntimeBindingMismatch {
                        expected: self.binding,
                        actual: binding,
                    });
                }
                if compiler_version != self.compiler_version {
                    return Err(CodeCheckError::RuntimeCompilerMismatch {
                        expected: self.compiler_version,
                        actual: compiler_version,
                    });
                }
            }
        }
        Ok(())
    }
}

/// Quotes an already identified source program without interpreting it.
#[must_use]
pub const fn quote_iprog(
    result: TypeRef,
    binding: BindingVersionRef,
    compiler_version: ArtifactRef,
    program: IProgRef,
) -> CodeArtifact {
    CodeArtifact::new(
        result,
        binding,
        compiler_version,
        CodeIR::Source { program },
    )
}
/// Quotes an already identified runtime program without stepping or executing it.
#[must_use]
pub const fn quote_program(
    result: TypeRef,
    binding: BindingVersionRef,
    compiler_version: ArtifactRef,
    program: RuntimeProgramRef,
) -> CodeArtifact {
    CodeArtifact::new(
        result,
        binding,
        compiler_version,
        CodeIR::Runtime { program },
    )
}

/// Interprets only an exact, catalog-admitted quotation coordinate.
pub fn interpret_code<C: CodeCatalog>(
    code: CodeArtifact,
    binding: BindingVersionRef,
    compiler_version: ArtifactRef,
    catalog: &C,
) -> Result<Option<CodeInterpretation>, CodeInterpretationError> {
    code.check(catalog)?;
    if code.binding != binding || code.compiler_version != compiler_version {
        return Ok(None);
    }
    let (kind, interpretation) = match code.code {
        CodeIR::Source { program } => (
            CodeInterpretationKind::Source,
            CodeInterpretation::Source(program),
        ),
        CodeIR::Runtime { program } => (
            CodeInterpretationKind::Runtime,
            CodeInterpretation::Runtime(program),
        ),
    };
    if catalog.admits_code_interpretation(binding, compiler_version, kind) {
        Ok(Some(interpretation))
    } else {
        Ok(None)
    }
}

#[derive(Debug, Error)]
pub enum CodeError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("code payload is {0} bytes, expected 129")]
    InvalidPayloadLength(usize),
    #[error("unknown code payload tag {0}")]
    UnknownTag(u8),
    #[error("code artifact kind is {actual}, expected {expected}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported code schema version {0}")]
    UnsupportedSchemaVersion(u32),
}
#[derive(Debug, Error)]
pub enum CodeCheckError {
    #[error(transparent)]
    Code(#[from] CodeError),
    #[error(transparent)]
    IProg(#[from] IProgError),
    #[error(transparent)]
    IProgCheck(#[from] IProgCheckError),
    #[error("source program {0} is unavailable")]
    UnresolvedSource(IProgRef),
    #[error("source program {reference} hashes to {calculated}, not its claimed identity")]
    SourceIdentityMismatch {
        reference: IProgRef,
        calculated: IProgRef,
    },
    #[error("source result type {actual} differs from quoted result {expected}")]
    SourceResultMismatch { expected: TypeRef, actual: TypeRef },
    #[error("source result type {0} is unavailable")]
    UnresolvedResultType(TypeRef),
    #[error("source result binding {actual} differs from quoted binding {expected}")]
    SourceBindingMismatch {
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },
    #[error("runtime program {0} is unavailable")]
    UnresolvedRuntime(RuntimeProgramRef),
    #[error("runtime result type {actual} differs from quoted result {expected}")]
    RuntimeResultMismatch { expected: TypeRef, actual: TypeRef },
    #[error("runtime binding {actual} differs from quoted binding {expected}")]
    RuntimeBindingMismatch {
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },
    #[error("runtime compiler {actual} differs from quoted compiler {expected}")]
    RuntimeCompilerMismatch {
        expected: ArtifactRef,
        actual: ArtifactRef,
    },
}
#[derive(Debug, Error)]
pub enum CodeInterpretationError {
    #[error(transparent)]
    Check(#[from] CodeCheckError),
}

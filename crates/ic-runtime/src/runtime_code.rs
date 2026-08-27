//! Immutable runtime-program identities used solely by nonexecuting quotations.

use thiserror::Error;

use crate::{BasicBlock, BlockTarget, ProgramCheckError, ProgramIR, RuntimeCatalog, Terminator};
use ic_core::{
    ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, BindingVersionRef,
    RuntimeProgramRef, TypeRef,
};

/// Canonical artifact kind for a reifiable runtime program.
pub const RUNTIME_PROGRAM_ARTIFACT_KIND: &str = "ic.runtime-program";
/// Payload schema version for reifiable runtime programs.
pub const RUNTIME_PROGRAM_SCHEMA_VERSION: u32 = 1;

/// A runtime program paired with the exact binding and compiler that produced it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeProgramArtifact {
    binding: BindingVersionRef,
    compiler_version: ArtifactRef,
    program: ProgramIR,
}

impl RuntimeProgramArtifact {
    #[must_use]
    pub const fn new(
        binding: BindingVersionRef,
        compiler_version: ArtifactRef,
        program: ProgramIR,
    ) -> Self {
        Self {
            binding,
            compiler_version,
            program,
        }
    }
    #[must_use]
    pub const fn binding(&self) -> BindingVersionRef {
        self.binding
    }
    #[must_use]
    pub const fn compiler_version(&self) -> ArtifactRef {
        self.compiler_version
    }
    #[must_use]
    pub const fn program(&self) -> &ProgramIR {
        &self.program
    }
    #[must_use]
    pub const fn result(&self) -> TypeRef {
        self.program.result()
    }

    pub fn canonical_payload(&self) -> Result<Vec<u8>, RuntimeProgramError> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.binding.as_artifact_ref().as_bytes());
        bytes.extend_from_slice(self.compiler_version.as_bytes());
        bytes.extend_from_slice(self.program.result().as_artifact_ref().as_bytes());
        bytes.extend_from_slice(&self.program.entry().value().to_be_bytes());
        let count = u32::try_from(self.program.blocks().len())
            .map_err(|_| RuntimeProgramError::TooManyBlocks(self.program.blocks().len()))?;
        bytes.extend_from_slice(&count.to_be_bytes());
        for block in self.program.blocks() {
            bytes.extend_from_slice(&block.target().value().to_be_bytes());
            match block.terminator() {
                Terminator::Return { value } => {
                    bytes.push(0);
                    bytes.extend_from_slice(value.as_artifact_ref().as_bytes());
                }
                Terminator::Branch { targets } => {
                    bytes.push(1);
                    let target_count = u32::try_from(targets.len())
                        .map_err(|_| RuntimeProgramError::TooManyBranchTargets(targets.len()))?;
                    bytes.extend_from_slice(&target_count.to_be_bytes());
                    for target in targets {
                        bytes.extend_from_slice(&target.value().to_be_bytes());
                    }
                }
                Terminator::Probe { operator, resume } => {
                    bytes.push(2);
                    bytes.extend_from_slice(operator.as_artifact_ref().as_bytes());
                    bytes.extend_from_slice(&resume.value().to_be_bytes());
                }
            }
        }
        Ok(bytes)
    }
    pub fn decode_payload(payload: &[u8]) -> Result<Self, RuntimeProgramError> {
        let mut cursor = Cursor::new(payload);
        let binding = BindingVersionRef::from_artifact_ref(cursor.reference()?);
        let compiler_version = cursor.reference()?;
        let result = TypeRef::from_artifact_ref(cursor.reference()?);
        let entry = BlockTarget::new(cursor.u32()?);
        let count = cursor.u32()?;
        let count = usize::try_from(count).expect("u32 fits usize on supported platforms");
        let mut blocks = Vec::with_capacity(count);
        for _ in 0..count {
            let target = BlockTarget::new(cursor.u32()?);
            let terminator = match cursor.byte()? {
                0 => Terminator::Return {
                    value: ic_core::TypedFormRef::from_artifact_ref(cursor.reference()?),
                },
                1 => {
                    let targets = cursor.u32()?;
                    let targets =
                        usize::try_from(targets).expect("u32 fits usize on supported platforms");
                    let mut values = Vec::with_capacity(targets);
                    for _ in 0..targets {
                        values.push(BlockTarget::new(cursor.u32()?));
                    }
                    Terminator::Branch { targets: values }
                }
                2 => Terminator::Probe {
                    operator: ic_core::ProbeOperatorRef::from_artifact_ref(cursor.reference()?),
                    resume: BlockTarget::new(cursor.u32()?),
                },
                tag => return Err(RuntimeProgramError::UnknownTerminatorTag(tag)),
            };
            blocks.push(BasicBlock::new(target, terminator));
        }
        if !cursor.finished() {
            return Err(RuntimeProgramError::TrailingPayloadBytes(
                cursor.remaining(),
            ));
        }
        Ok(Self::new(
            binding,
            compiler_version,
            ProgramIR::new(result, entry, blocks),
        ))
    }
    pub fn envelope(&self) -> Result<ArtifactEnvelope, RuntimeProgramError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(RUNTIME_PROGRAM_ARTIFACT_KIND)?,
            RUNTIME_PROGRAM_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }
    pub fn runtime_program_ref(&self) -> Result<RuntimeProgramRef, RuntimeProgramError> {
        Ok(RuntimeProgramRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }
    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, RuntimeProgramError> {
        if envelope.kind().as_str() != RUNTIME_PROGRAM_ARTIFACT_KIND {
            return Err(RuntimeProgramError::UnexpectedArtifactKind {
                expected: RUNTIME_PROGRAM_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != RUNTIME_PROGRAM_SCHEMA_VERSION {
            return Err(RuntimeProgramError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }
    pub fn check<C: RuntimeCatalog>(&self, catalog: &C) -> Result<(), RuntimeProgramCheckError> {
        self.program.verify(catalog)?;
        let result = catalog.resolve_type(self.program.result()).ok_or(
            RuntimeProgramCheckError::UnresolvedResultType(self.program.result()),
        )?;
        if result.binding() != self.binding {
            return Err(RuntimeProgramCheckError::BindingMismatch {
                program: self.binding,
                result: result.binding(),
            });
        }
        Ok(())
    }
}

struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}
impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }
    fn byte(&mut self) -> Result<u8, RuntimeProgramError> {
        let value = *self
            .bytes
            .get(self.offset)
            .ok_or(RuntimeProgramError::TruncatedPayload)?;
        self.offset += 1;
        Ok(value)
    }
    fn u32(&mut self) -> Result<u32, RuntimeProgramError> {
        Ok(u32::from_be_bytes(self.take::<4>()?))
    }
    fn reference(&mut self) -> Result<ArtifactRef, RuntimeProgramError> {
        Ok(ArtifactRef::from_bytes(self.take::<32>()?))
    }
    fn take<const N: usize>(&mut self) -> Result<[u8; N], RuntimeProgramError> {
        let end = self
            .offset
            .checked_add(N)
            .ok_or(RuntimeProgramError::TruncatedPayload)?;
        let bytes: [u8; N] = self
            .bytes
            .get(self.offset..end)
            .ok_or(RuntimeProgramError::TruncatedPayload)?
            .try_into()
            .expect("exact slice length");
        self.offset = end;
        Ok(bytes)
    }
    fn finished(&self) -> bool {
        self.offset == self.bytes.len()
    }
    fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.offset)
    }
}

#[derive(Debug, Error)]
pub enum RuntimeProgramError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("runtime program payload is truncated")]
    TruncatedPayload,
    #[error("runtime program payload has {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("unknown runtime terminator tag {0}")]
    UnknownTerminatorTag(u8),
    #[error("too many runtime blocks: {0}")]
    TooManyBlocks(usize),
    #[error("too many branch targets: {0}")]
    TooManyBranchTargets(usize),
    #[error("runtime program artifact kind is {actual}, expected {expected}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported runtime program schema version {0}")]
    UnsupportedSchemaVersion(u32),
}
#[derive(Debug, Error)]
pub enum RuntimeProgramCheckError {
    #[error(transparent)]
    Program(#[from] ProgramCheckError),
    #[error("runtime program result type {0} is unavailable")]
    UnresolvedResultType(TypeRef),
    #[error("runtime program binding {program} differs from result type binding {result}")]
    BindingMismatch {
        program: BindingVersionRef,
        result: BindingVersionRef,
    },
}

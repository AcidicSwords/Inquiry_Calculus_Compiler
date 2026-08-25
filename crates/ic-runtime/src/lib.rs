//! First-order structural control flow for the Inquiry Calculus runtime.
//!
//! This phase contains only `Return`, `Branch`, and probe suspension/resumption. It never
//! dispatches an operator, records an actuality event, decodes a raw return, or selects a
//! raw-return-dependent continuation; those contracts remain Phase 6/7 work.

use std::collections::BTreeMap;

pub use ic_core::ProbeOperatorRef;
use ic_core::{
    RawReturnRef, TypeCatalog, TypeCheckError, TypeError, TypeRef, TypedForm, TypedFormRef,
};
use thiserror::Error;

/// A stable target within one inspected runtime program.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BlockTarget(u32);

impl BlockTarget {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }
}

/// The only runtime control-flow terminators.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Terminator {
    /// A pure typed return; this creates no external actuality.
    Return { value: TypedFormRef },
    /// A nonempty internal or generated alternative set; this creates no external actuality.
    Branch { targets: Vec<BlockTarget> },
    /// A suspended request for later actualization. No operator call occurs at this phase.
    Probe {
        operator: ProbeOperatorRef,
        resume: BlockTarget,
    },
}

/// One explicitly addressed control-flow block.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasicBlock {
    target: BlockTarget,
    terminator: Terminator,
}

impl BasicBlock {
    #[must_use]
    pub const fn new(target: BlockTarget, terminator: Terminator) -> Self {
        Self { target, terminator }
    }

    #[must_use]
    pub const fn target(&self) -> BlockTarget {
        self.target
    }

    #[must_use]
    pub const fn terminator(&self) -> &Terminator {
        &self.terminator
    }
}

/// First-order runtime program data with one declared return type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgramIR {
    result: TypeRef,
    entry: BlockTarget,
    blocks: Vec<BasicBlock>,
}

impl ProgramIR {
    #[must_use]
    pub const fn new(result: TypeRef, entry: BlockTarget, blocks: Vec<BasicBlock>) -> Self {
        Self {
            result,
            entry,
            blocks,
        }
    }

    #[must_use]
    pub const fn result(&self) -> TypeRef {
        self.result
    }

    #[must_use]
    pub const fn entry(&self) -> BlockTarget {
        self.entry
    }

    #[must_use]
    pub fn blocks(&self) -> &[BasicBlock] {
        &self.blocks
    }

    /// Verifies typed return agreement, target closure, nonempty branches, and the currently
    /// representable guarded-recurrence condition.
    pub fn verify<C: RuntimeCatalog>(&self, catalog: &C) -> Result<(), ProgramCheckError> {
        let result_type = catalog
            .resolve_type(self.result)
            .ok_or(ProgramCheckError::UnresolvedResultType(self.result))?;
        let calculated = result_type.type_ref()?;
        if calculated != self.result {
            return Err(ProgramCheckError::ResultTypeIdentityMismatch {
                reference: self.result,
                calculated,
            });
        }
        result_type.check(catalog)?;

        let mut blocks = BTreeMap::new();
        for block in &self.blocks {
            if blocks.insert(block.target(), block).is_some() {
                return Err(ProgramCheckError::DuplicateBlockTarget(block.target()));
            }
        }
        if !blocks.contains_key(&self.entry) {
            return Err(ProgramCheckError::MissingEntry(self.entry));
        }
        for block in &self.blocks {
            match block.terminator() {
                Terminator::Return { value } => {
                    let form = catalog
                        .resolve_typed_form(*value)
                        .ok_or(ProgramCheckError::UnresolvedReturnValue(*value))?;
                    let calculated = form.typed_form_ref()?;
                    if calculated != *value {
                        return Err(ProgramCheckError::ReturnValueIdentityMismatch {
                            reference: *value,
                            calculated,
                        });
                    }
                    form.check(catalog)?;
                    if form.ty() != self.result {
                        return Err(ProgramCheckError::ReturnTypeMismatch {
                            block: block.target(),
                            expected: self.result,
                            actual: form.ty(),
                        });
                    }
                }
                Terminator::Branch { targets } => {
                    if targets.is_empty() {
                        return Err(ProgramCheckError::EmptyBranch(block.target()));
                    }
                    for (index, target) in targets.iter().enumerate() {
                        if targets[..index].contains(target) {
                            return Err(ProgramCheckError::DuplicateBranchTarget {
                                block: block.target(),
                                target: *target,
                            });
                        }
                        ensure_target(*target, &blocks)?;
                    }
                }
                Terminator::Probe { resume, .. } => ensure_target(*resume, &blocks)?,
            }
        }
        reject_unguarded_branch_cycles(&blocks)?;
        Ok(())
    }

    /// Begins a verified program at its entry block.
    #[must_use]
    pub const fn start(&self) -> ReadyState {
        ReadyState { target: self.entry }
    }

    /// Takes one internal control-flow step without dispatching an external probe.
    pub fn step(&self, state: ReadyState) -> Result<MachineStep, RuntimeStepError> {
        let block = self
            .blocks
            .iter()
            .find(|block| block.target() == state.target)
            .ok_or(RuntimeStepError::MissingBlock(state.target))?;
        match block.terminator() {
            Terminator::Return { value } => Ok(MachineStep::Returned(*value)),
            Terminator::Branch { targets } => Ok(MachineStep::Branched(
                targets
                    .iter()
                    .copied()
                    .map(|target| ReadyState { target })
                    .collect(),
            )),
            Terminator::Probe { operator, resume } => Ok(MachineStep::Suspended(ProbeSuspension {
                operator: *operator,
                resume: *resume,
            })),
        }
    }
}

/// The catalog required to check runtime returns before probe operators acquire a Phase 10 schema.
pub trait RuntimeCatalog: TypeCatalog {
    /// Resolves a typed form by its claimed content identity.
    fn resolve_typed_form(&self, reference: TypedFormRef) -> Option<TypedForm>;
}

/// One ready-to-step program position.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReadyState {
    target: BlockTarget,
}

impl ReadyState {
    #[must_use]
    pub const fn target(self) -> BlockTarget {
        self.target
    }
}

/// A probe request that has suspended control flow but has not been actualized.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProbeSuspension {
    operator: ProbeOperatorRef,
    resume: BlockTarget,
}

impl ProbeSuspension {
    #[must_use]
    pub const fn operator(self) -> ProbeOperatorRef {
        self.operator
    }

    #[must_use]
    pub const fn resume_target(self) -> BlockTarget {
        self.resume
    }

    /// Associates a separately preserved raw-return identity with the resumed position.
    ///
    /// This is not an actualization record and does not decode the return or branch on it.
    #[must_use]
    pub const fn resume(self, raw_return: RawReturnRef) -> Resumption {
        Resumption {
            state: ReadyState {
                target: self.resume,
            },
            raw_return,
        }
    }
}

/// A control-flow resumption carrying the raw return for later Phase 6/7 event and resolution work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Resumption {
    state: ReadyState,
    raw_return: RawReturnRef,
}

impl Resumption {
    #[must_use]
    pub const fn state(self) -> ReadyState {
        self.state
    }

    #[must_use]
    pub const fn raw_return(self) -> RawReturnRef {
        self.raw_return
    }
}

/// The observable outcome of one internal runtime control-flow step.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MachineStep {
    Returned(TypedFormRef),
    Branched(Vec<ReadyState>),
    Suspended(ProbeSuspension),
}

fn ensure_target(
    target: BlockTarget,
    blocks: &BTreeMap<BlockTarget, &BasicBlock>,
) -> Result<(), ProgramCheckError> {
    if blocks.contains_key(&target) {
        Ok(())
    } else {
        Err(ProgramCheckError::UnknownBlockTarget(target))
    }
}

fn reject_unguarded_branch_cycles(
    blocks: &BTreeMap<BlockTarget, &BasicBlock>,
) -> Result<(), ProgramCheckError> {
    let mut visiting = Vec::new();
    let mut completed = Vec::new();
    for target in blocks.keys().copied() {
        visit_unguarded_branch_cycle(target, blocks, &mut visiting, &mut completed)?;
    }
    Ok(())
}

fn visit_unguarded_branch_cycle(
    target: BlockTarget,
    blocks: &BTreeMap<BlockTarget, &BasicBlock>,
    visiting: &mut Vec<BlockTarget>,
    completed: &mut Vec<BlockTarget>,
) -> Result<(), ProgramCheckError> {
    if completed.contains(&target) {
        return Ok(());
    }
    if visiting.contains(&target) {
        return Err(ProgramCheckError::UnguardedBranchCycle(target));
    }
    let block = blocks
        .get(&target)
        .expect("target closure is checked before recurrence validation");
    if let Terminator::Branch { targets } = block.terminator() {
        visiting.push(target);
        for successor in targets {
            visit_unguarded_branch_cycle(*successor, blocks, visiting, completed)?;
        }
        let popped = visiting.pop();
        debug_assert_eq!(popped, Some(target));
    }
    completed.push(target);
    Ok(())
}

#[derive(Debug, Error)]
pub enum ProgramCheckError {
    #[error(transparent)]
    Type(#[from] TypeError),
    #[error(transparent)]
    TypeCheck(#[from] TypeCheckError),
    #[error("program result type {0} is unavailable")]
    UnresolvedResultType(TypeRef),
    #[error("result type {reference} hashes to {calculated}, not its claimed identity")]
    ResultTypeIdentityMismatch {
        reference: TypeRef,
        calculated: TypeRef,
    },
    #[error("runtime program has duplicate block target {0:?}")]
    DuplicateBlockTarget(BlockTarget),
    #[error("runtime program entry target {0:?} is missing")]
    MissingEntry(BlockTarget),
    #[error("runtime program refers to missing block target {0:?}")]
    UnknownBlockTarget(BlockTarget),
    #[error("branch block {0:?} has no alternatives")]
    EmptyBranch(BlockTarget),
    #[error("branch block {block:?} repeats target {target:?}")]
    DuplicateBranchTarget {
        block: BlockTarget,
        target: BlockTarget,
    },
    #[error("return value {0} is unavailable")]
    UnresolvedReturnValue(TypedFormRef),
    #[error("return value {reference} hashes to {calculated}, not its claimed identity")]
    ReturnValueIdentityMismatch {
        reference: TypedFormRef,
        calculated: TypedFormRef,
    },
    #[error("return block {block:?} has type {actual}, expected program result {expected}")]
    ReturnTypeMismatch {
        block: BlockTarget,
        expected: TypeRef,
        actual: TypeRef,
    },
    #[error("branch-only runtime recurrence is unguarded at block {0:?}")]
    UnguardedBranchCycle(BlockTarget),
}

#[derive(Debug, Error)]
pub enum RuntimeStepError {
    #[error("runtime state refers to missing block {0:?}")]
    MissingBlock(BlockTarget),
}

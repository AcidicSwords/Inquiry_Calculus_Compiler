//! Program-wide admission of a resolved finite all-Probe source occurrence.
//!
//! The record composes the already checked port bundle, five-way resolution gate, exact source
//! `Ask`, whole answer binding, and derived next source control. Non-supported outcomes retain
//! their payload and cannot inhabit the record.

use ic_core::{
    AskOccurrenceRef, FiniteAnswerBindingError, FiniteResolutionOutcome,
    FiniteSupportedAnswerCatalog, FiniteSupportedResolution, IProgArtifact, IProgError, IProgRef,
    QuestionSuccessionCatalog, QuestionSuccessor, QuestionSuccessorError, SourceConfigRef,
    bind_finite_ask_continuation, derive_question_successor,
};
use thiserror::Error;

use crate::FiniteProbeDischargeBundle;

/// One resolved finite all-Probe occurrence. Every field is derived from existing checked data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedFiniteProbeOccurrence {
    source_config: SourceConfigRef,
    ask_occurrence: AskOccurrenceRef,
    discharge_bundle: FiniteProbeDischargeBundle,
    resolution: FiniteSupportedResolution,
    binding: ic_core::BoundFiniteAskContinuation,
    next: QuestionSuccessor,
}

impl ResolvedFiniteProbeOccurrence {
    #[must_use]
    pub const fn source_config(&self) -> SourceConfigRef {
        self.source_config
    }

    #[must_use]
    pub const fn ask_occurrence(&self) -> AskOccurrenceRef {
        self.ask_occurrence
    }

    #[must_use]
    pub const fn discharge_bundle(&self) -> &FiniteProbeDischargeBundle {
        &self.discharge_bundle
    }

    #[must_use]
    pub const fn resolution(&self) -> &FiniteSupportedResolution {
        &self.resolution
    }

    #[must_use]
    pub const fn binding(&self) -> &ic_core::BoundFiniteAskContinuation {
        &self.binding
    }

    #[must_use]
    pub const fn next(&self) -> &QuestionSuccessor {
        &self.next
    }
}

/// Constructs the resolved occurrence only from the `Supported` resolution summand.
pub fn resolve_finite_probe_occurrence<C>(
    discharge_bundle: FiniteProbeDischargeBundle,
    outcome: FiniteResolutionOutcome,
    source_ask: &IProgArtifact,
    catalog: &C,
) -> Result<ResolvedFiniteProbeOccurrence, ResolvedFiniteProbeOccurrenceError>
where
    C: FiniteSupportedAnswerCatalog + QuestionSuccessionCatalog,
{
    let resolution = outcome
        .into_supported()
        .map_err(ResolvedFiniteProbeOccurrenceError::NonSupported)?;
    let occurrence = discharge_bundle.occurrence();
    occurrence
        .check(catalog)
        .map_err(|error| ResolvedFiniteProbeOccurrenceError::OccurrenceCheck(Box::new(error)))?;
    let occurrence_ref = occurrence.ask_occurrence_ref()?;
    let source_ref = source_ask.iprog_ref()?;
    if source_ref != occurrence.position().target() {
        return Err(ResolvedFiniteProbeOccurrenceError::SourceProgramMismatch {
            occurrence: occurrence.position().target(),
            supplied: source_ref,
        });
    }
    if resolution.answer().event()
        != discharge_bundle
            .components()
            .iter()
            .find(|component| component.event().event_ref() == resolution.answer().event())
            .map(|component| component.event().event_ref())
            .ok_or(
                ResolvedFiniteProbeOccurrenceError::AnswerEventMissingFromBundle(
                    resolution.answer().event(),
                ),
            )?
    {
        unreachable!("the selected component was compared by exact event identity")
    }
    let binding = bind_finite_ask_continuation(source_ask, resolution.answer().clone(), catalog)?;
    if binding.question() != occurrence.question()
        || binding.answer_slot() != occurrence.answer_slot()
        || binding.continuation() != occurrence.continuation()
    {
        return Err(ResolvedFiniteProbeOccurrenceError::ContinuationMismatch);
    }
    let next = derive_question_successor(occurrence.clone(), resolution.answer().clone(), catalog)?;
    Ok(ResolvedFiniteProbeOccurrence {
        source_config: occurrence.source_config(),
        ask_occurrence: occurrence_ref,
        discharge_bundle,
        resolution,
        binding,
        next,
    })
}

#[derive(Debug, Error)]
pub enum ResolvedFiniteProbeOccurrenceError {
    #[error("resolution outcome {0} cannot enter a source continuation")]
    NonSupported(Box<FiniteResolutionOutcome>),
    #[error(transparent)]
    IProgEncoding(#[from] IProgError),
    #[error(transparent)]
    OccurrenceEncoding(#[from] ic_core::AskOccurrenceError),
    #[error("source occurrence failed recheck: {0}")]
    OccurrenceCheck(Box<ic_core::AskOccurrenceCheckError>),
    #[error(transparent)]
    Binding(#[from] FiniteAnswerBindingError),
    #[error(transparent)]
    Successor(#[from] QuestionSuccessorError),
    #[error("source occurrence targets {occurrence}, but supplied Ask program is {supplied}")]
    SourceProgramMismatch {
        occurrence: IProgRef,
        supplied: IProgRef,
    },
    #[error("supported answer event {0} is absent from the exact discharge bundle")]
    AnswerEventMissingFromBundle(ic_core::EventRef),
    #[error("bound answer slot or continuation differs from the exact source occurrence")]
    ContinuationMismatch,
}

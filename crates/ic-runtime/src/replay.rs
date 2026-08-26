//! Cold replay composition for one completed finite probe.
//!
//! This module adds no replay opcode or second history. It composes the existing immutable store,
//! finite decoder, standing admission, source binding, and runtime resumption boundaries. Callers
//! must reconstruct the supplied catalog, source program, runtime program, suspension, and
//! lowering after restart; none is accepted as historical actuality.

use ic_core::{
    ActualDecodeError, ActualDecodeResult, CompletionCandidateRef, DecodedObservationError,
    FiniteAnswerBindingError, FiniteDecoder, FiniteSupportedAnswerCatalog,
    FiniteSupportedAnswerError, IProgArtifact, IProgCatalog, RelationUseRef, ResolutionPathRef,
    Standing, admit_finite_supported_answers, bind_finite_ask_continuation, decode_actual_event,
    match_decoded_observation_use,
};
use ic_store::{ArtifactStore, DispatchToken, ReplayedExternalEffect, StoreError};
use thiserror::Error;

use crate::{
    AdmittedResumeError, AdmittedResumption, ContinuationLowering, ProbeSuspension, ProgramIR,
    RuntimeCatalog,
};

/// One explicit candidate-to-observation-use correspondence supplied to finite replay.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplayObservation {
    candidate: CompletionCandidateRef,
    observation: RelationUseRef,
}

impl ReplayObservation {
    #[must_use]
    pub const fn new(candidate: CompletionCandidateRef, observation: RelationUseRef) -> Self {
        Self {
            candidate,
            observation,
        }
    }
}

/// A completed actuality and its freshly reconstructed admitted continuation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColdReplayedProbe {
    actuality: ReplayedExternalEffect,
    resumption: AdmittedResumption,
}

impl ColdReplayedProbe {
    #[must_use]
    pub const fn actuality(&self) -> &ReplayedExternalEffect {
        &self.actuality
    }

    #[must_use]
    pub const fn resumption(&self) -> &AdmittedResumption {
        &self.resumption
    }
}

/// Reconstructs one completed finite probe through semantic admission and runtime resumption.
///
/// The store is consulted first and no provider is available on this path. `Undefined` and
/// `Unknown` remain distinct terminal replay results. A decoded result is admitted only through
/// the existing exact observation/support route and is then bound to the checked source `Ask`.
#[allow(clippy::too_many_arguments)]
pub async fn replay_completed_finite_probe<C>(
    store: &ArtifactStore,
    token: DispatchToken,
    decoder: &FiniteDecoder,
    path: ResolutionPathRef,
    observations: &[ReplayObservation],
    standing: &Standing,
    source: &IProgArtifact,
    suspension: ProbeSuspension,
    lowering: ContinuationLowering,
    program: &ProgramIR,
    catalog: &C,
) -> Result<ColdReplayedProbe, FiniteProbeReplayError>
where
    C: FiniteSupportedAnswerCatalog + IProgCatalog + RuntimeCatalog,
{
    let actuality = store.replay_completed_external_effect(token).await?;
    let decoded = match decode_actual_event(actuality.event(), decoder, path, catalog)? {
        ActualDecodeResult::Decoded(decoded) => decoded,
        ActualDecodeResult::Undefined {
            event,
            decoder,
            path,
            ..
        } => {
            return Err(FiniteProbeReplayError::Undefined {
                event,
                decoder,
                path,
            });
        }
        ActualDecodeResult::Unknown {
            event,
            decoder,
            path,
            ..
        } => {
            return Err(FiniteProbeReplayError::Unknown {
                event,
                decoder,
                path,
            });
        }
    };
    let decoded_observations = observations
        .iter()
        .map(|binding| {
            match_decoded_observation_use(&decoded, binding.candidate, binding.observation, catalog)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let admitted =
        admit_finite_supported_answers(decoded, decoded_observations, standing, catalog)?;
    let bound = bind_finite_ask_continuation(source, admitted, catalog)?;
    let resumption = suspension.resume_admitted(bound, lowering, program)?;
    Ok(ColdReplayedProbe {
        actuality,
        resumption,
    })
}

/// Distinct replay exits; operational, resolution, support, source, and lowering failures never
/// collapse to a Boolean or one generic "no answer" result.
#[derive(Debug, Error)]
pub enum FiniteProbeReplayError {
    #[error(transparent)]
    Store(#[from] StoreError),
    #[error(transparent)]
    Decode(#[from] ActualDecodeError),
    #[error(
        "event {event} has an explicitly undefined result through decoder {decoder} and path {path}"
    )]
    Undefined {
        event: ic_core::EventRef,
        decoder: ic_core::FiniteDecoderRef,
        path: ResolutionPathRef,
    },
    #[error("event {event} remains unknown to decoder {decoder} through path {path}")]
    Unknown {
        event: ic_core::EventRef,
        decoder: ic_core::FiniteDecoderRef,
        path: ResolutionPathRef,
    },
    #[error(transparent)]
    Observation(#[from] DecodedObservationError),
    #[error(transparent)]
    SupportedAnswer(#[from] FiniteSupportedAnswerError),
    #[error(transparent)]
    SourceBinding(#[from] FiniteAnswerBindingError),
    #[error(transparent)]
    Resumption(#[from] AdmittedResumeError),
}

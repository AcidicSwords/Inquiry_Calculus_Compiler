//! Cold replay composition for one completed finite probe.
//!
//! This module adds no replay opcode or second history. It composes the existing immutable store,
//! finite decoder, standing admission, source binding, and runtime resumption boundaries. Callers
//! must reconstruct the supplied catalog, source program, runtime program, suspension, and
//! lowering after restart; none is accepted as historical actuality.

use ic_core::{
    ActualDecodeError, ActualDecodeResult, CompletionCandidateRef, DecodedObservationError,
    FiniteAnswerBindingError, FiniteDecoder, FiniteSupportedAnswerCatalog,
    FiniteSupportedAnswerError, GeneratedInquiry, GeneratedInquiryBindingError,
    GeneratedInquiryCatalog, IProgArtifact, IProgCatalog, IProgRef, MethodBridge,
    MethodBridgeCatalog, MethodBridgeCheckError, RelationUseRef, ResolutionPathRef, Standing,
    admit_finite_supported_answers, bind_finite_ask_continuation,
    bind_generated_inquiry_continuation, decode_actual_event, match_decoded_observation_use,
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

/// A cold-replayed supported continuation retaining the generic separator problem and route that
/// generated its source question.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColdReplayedSeparatorInquiry {
    replay: ColdReplayedProbe,
    separator: ic_core::BoundGeneratedInquiryContinuation,
}

/// A checked transparent method reentry retaining the separator return that selected it.
///
/// This derived view does not execute a method or evaluate its guard. It proves that reentry is
/// represented by a checked first-order bridge whose transport is the exact continuation already
/// selected by the admitted answer, rather than by a method-name switch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MethodBridgeReentry {
    separator: ColdReplayedSeparatorInquiry,
    bridge: MethodBridge,
}

impl MethodBridgeReentry {
    #[must_use]
    pub const fn separator(&self) -> &ColdReplayedSeparatorInquiry {
        &self.separator
    }

    #[must_use]
    pub const fn bridge(&self) -> MethodBridge {
        self.bridge
    }
}

impl ColdReplayedSeparatorInquiry {
    #[must_use]
    pub const fn replay(&self) -> &ColdReplayedProbe {
        &self.replay
    }

    #[must_use]
    pub const fn separator(&self) -> &ic_core::BoundGeneratedInquiryContinuation {
        &self.separator
    }
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

/// Replays one finite probe and retains the checked generic separator inquiry through its
/// supported, answer-dependent source continuation.
///
/// The provider remains absent. The ordinary replay path reconstructs actuality, decoding,
/// standing, binding, and lowering first; the generated-inquiry bridge then independently
/// rechecks the separator problem and exact question before retaining its problem and route.
#[allow(clippy::too_many_arguments)]
pub async fn replay_completed_finite_separator_inquiry<C>(
    store: &ArtifactStore,
    token: DispatchToken,
    inquiry: GeneratedInquiry,
    decoder: &FiniteDecoder,
    path: ResolutionPathRef,
    observations: &[ReplayObservation],
    standing: &Standing,
    source: &IProgArtifact,
    suspension: ProbeSuspension,
    lowering: ContinuationLowering,
    program: &ProgramIR,
    catalog: &C,
) -> Result<ColdReplayedSeparatorInquiry, FiniteSeparatorReplayError>
where
    C: FiniteSupportedAnswerCatalog + GeneratedInquiryCatalog + IProgCatalog + RuntimeCatalog,
{
    let replay = replay_completed_finite_probe(
        store,
        token,
        decoder,
        path,
        observations,
        standing,
        source,
        suspension,
        lowering,
        program,
        catalog,
    )
    .await?;
    let separator = bind_generated_inquiry_continuation(
        inquiry,
        source,
        replay.resumption().binding().answer().clone(),
        catalog,
    )?;
    debug_assert_eq!(separator.binding(), replay.resumption().binding());
    Ok(ColdReplayedSeparatorInquiry { replay, separator })
}

/// Routes an admitted separator return through one checked, inspectable method bridge.
///
/// Both method contracts and all bridge dependencies are revalidated by `MethodBridge::check`.
/// The transport must be the answer-bound source continuation itself. Consequently a catalog
/// entry, method name, or generated proposal cannot silently choose a different continuation.
pub fn route_separator_through_method_bridge<C>(
    separator: ColdReplayedSeparatorInquiry,
    bridge: MethodBridge,
    catalog: &C,
) -> Result<MethodBridgeReentry, MethodBridgeReentryError>
where
    C: MethodBridgeCatalog,
{
    bridge.check(catalog)?;
    let selected = separator.separator().binding().continuation();
    if bridge.transport() != selected {
        return Err(MethodBridgeReentryError::TransportMismatch {
            selected,
            bridge: bridge.transport(),
        });
    }
    Ok(MethodBridgeReentry { separator, bridge })
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

/// Distinct failures while cold replaying a generated separator inquiry.
#[derive(Debug, Error)]
pub enum FiniteSeparatorReplayError {
    #[error(transparent)]
    Replay(#[from] FiniteProbeReplayError),
    #[error(transparent)]
    Binding(#[from] GeneratedInquiryBindingError),
}

/// Distinct failures while routing a replayed separator through a transparent method bridge.
#[derive(Debug, Error)]
pub enum MethodBridgeReentryError {
    #[error(transparent)]
    Bridge(#[from] MethodBridgeCheckError),
    #[error(
        "method bridge transport {bridge} does not match answer-selected continuation {selected}"
    )]
    TransportMismatch {
        selected: IProgRef,
        bridge: IProgRef,
    },
}

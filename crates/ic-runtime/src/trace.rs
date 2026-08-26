//! Derived paired-actuality traces over one admitted ordinary event.
//!
//! A question projection and a return projection are two views of the same authoritative event
//! occurrence. They retain source, event, resolution-path, and continuation provenance so an
//! equal runtime endpoint cannot erase how the endpoint was reached. These values are derived
//! runtime evidence: they create no event, causal edge, replay occurrence, or second history.

use ic_core::{
    ActualEvent, ActualEventError, BindingVersionRef, BoundaryRef, CompletionCandidateRef,
    EventRef, FiniteDecoderRef, IProgRef, ProbeOperatorRef, ProvenanceRef, QueryRef, RawReturnRef,
    RelationUseRef, ResolutionPathRef, RouteRef, StateRef,
};
use thiserror::Error;

use crate::{AdmittedResumption, BlockTarget};

/// The source/question side of one admitted ordinary actuality occurrence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuestionTrace {
    event: EventRef,
    source: IProgRef,
    question: QueryRef,
    operator: ProbeOperatorRef,
    boundary: BoundaryRef,
    state_before: StateRef,
    route: RouteRef,
    binding: BindingVersionRef,
    provenance: ProvenanceRef,
}

impl QuestionTrace {
    #[must_use]
    pub const fn event(&self) -> EventRef {
        self.event
    }

    #[must_use]
    pub const fn source(&self) -> IProgRef {
        self.source
    }

    #[must_use]
    pub const fn question(&self) -> QueryRef {
        self.question
    }

    #[must_use]
    pub const fn operator(&self) -> ProbeOperatorRef {
        self.operator
    }

    #[must_use]
    pub const fn boundary(&self) -> BoundaryRef {
        self.boundary
    }

    #[must_use]
    pub const fn state_before(&self) -> StateRef {
        self.state_before
    }

    #[must_use]
    pub const fn route(&self) -> RouteRef {
        self.route
    }

    #[must_use]
    pub const fn binding(&self) -> BindingVersionRef {
        self.binding
    }

    #[must_use]
    pub const fn provenance(&self) -> ProvenanceRef {
        self.provenance
    }
}

/// The raw-return/resolution/continuation side of the same admitted occurrence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReturnTrace {
    event: EventRef,
    raw_return: RawReturnRef,
    decoder: FiniteDecoderRef,
    path: ResolutionPathRef,
    candidates: Vec<CompletionCandidateRef>,
    observations: Vec<RelationUseRef>,
    continuation: IProgRef,
    resume_target: BlockTarget,
    state_after: StateRef,
    route: RouteRef,
    binding: BindingVersionRef,
    provenance: ProvenanceRef,
}

impl ReturnTrace {
    #[must_use]
    pub const fn event(&self) -> EventRef {
        self.event
    }

    #[must_use]
    pub const fn raw_return(&self) -> RawReturnRef {
        self.raw_return
    }

    #[must_use]
    pub const fn decoder(&self) -> FiniteDecoderRef {
        self.decoder
    }

    #[must_use]
    pub const fn path(&self) -> ResolutionPathRef {
        self.path
    }

    #[must_use]
    pub fn candidates(&self) -> &[CompletionCandidateRef] {
        &self.candidates
    }

    #[must_use]
    pub fn observations(&self) -> &[RelationUseRef] {
        &self.observations
    }

    #[must_use]
    pub const fn continuation(&self) -> IProgRef {
        self.continuation
    }

    #[must_use]
    pub const fn resume_target(&self) -> BlockTarget {
        self.resume_target
    }

    #[must_use]
    pub const fn state_after(&self) -> StateRef {
        self.state_after
    }

    #[must_use]
    pub const fn route(&self) -> RouteRef {
        self.route
    }

    #[must_use]
    pub const fn binding(&self) -> BindingVersionRef {
        self.binding
    }

    #[must_use]
    pub const fn provenance(&self) -> ProvenanceRef {
        self.provenance
    }
}

/// Use-sensitive paired projections of one admitted event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PairedActualityTrace {
    question: QuestionTrace,
    returned: ReturnTrace,
}

impl PairedActualityTrace {
    /// Derives paired projections only when the supplied canonical event is the exact event whose
    /// admitted answer resumed control.
    pub fn derive(
        event: &ActualEvent,
        resumption: &AdmittedResumption,
    ) -> Result<Self, PairedActualityTraceError> {
        let event_ref = event.event_ref()?;
        if event_ref != resumption.event() {
            return Err(PairedActualityTraceError::EventMismatch {
                supplied: event_ref,
                resumed: resumption.event(),
            });
        }
        let binding = resumption.binding();
        let answer = binding.answer();
        if event.question() != binding.question() {
            return Err(PairedActualityTraceError::QuestionMismatch {
                event: event.question(),
                source_question: binding.question(),
            });
        }
        if event.operator() != answer.operator() {
            return Err(PairedActualityTraceError::OperatorMismatch {
                event: event.operator(),
                answer: answer.operator(),
            });
        }
        if event.raw_return() != answer.raw_return() {
            return Err(PairedActualityTraceError::RawReturnMismatch {
                event: event.raw_return(),
                answer: answer.raw_return(),
            });
        }
        let decoded = answer.decoded();
        if decoded.event() != event_ref {
            return Err(PairedActualityTraceError::DecodedEventMismatch {
                decoded: decoded.event(),
                event: event_ref,
            });
        }

        Ok(Self {
            question: QuestionTrace {
                event: event_ref,
                source: binding.source(),
                question: binding.question(),
                operator: event.operator(),
                boundary: event.boundary(),
                state_before: event.state_before(),
                route: event.route(),
                binding: event.binding(),
                provenance: event.provenance(),
            },
            returned: ReturnTrace {
                event: event_ref,
                raw_return: answer.raw_return(),
                decoder: decoded.decoder(),
                path: decoded.path(),
                candidates: answer.candidates().to_vec(),
                observations: answer
                    .observations()
                    .iter()
                    .map(|observation| observation.observation())
                    .collect(),
                continuation: binding.continuation(),
                resume_target: resumption.state().target(),
                state_after: event.state_after(),
                route: event.route(),
                binding: event.binding(),
                provenance: event.provenance(),
            },
        })
    }

    #[must_use]
    pub const fn question(&self) -> &QuestionTrace {
        &self.question
    }

    #[must_use]
    pub const fn returned(&self) -> &ReturnTrace {
        &self.returned
    }
}

/// Failures while pairing one event with one admitted resumption.
#[derive(Debug, Error)]
pub enum PairedActualityTraceError {
    #[error(transparent)]
    EventEncoding(#[from] ActualEventError),
    #[error("supplied event {supplied} differs from resumed event {resumed}")]
    EventMismatch {
        supplied: EventRef,
        resumed: EventRef,
    },
    #[error("event question {event} differs from source question {source_question}")]
    QuestionMismatch {
        event: QueryRef,
        source_question: QueryRef,
    },
    #[error("event operator {event} differs from admitted answer operator {answer}")]
    OperatorMismatch {
        event: ProbeOperatorRef,
        answer: ProbeOperatorRef,
    },
    #[error("event raw return {event} differs from admitted answer return {answer}")]
    RawReturnMismatch {
        event: RawReturnRef,
        answer: RawReturnRef,
    },
    #[error("decoded event {decoded} differs from supplied event {event}")]
    DecodedEventMismatch { decoded: EventRef, event: EventRef },
}

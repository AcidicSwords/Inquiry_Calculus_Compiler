//! Derived paired-actuality traces over one admitted ordinary event.
//!
//! A question projection and a return projection are two views of the same authoritative event
//! occurrence. They retain source, event, resolution-path, and continuation provenance so an
//! equal runtime endpoint cannot erase how the endpoint was reached. These values are derived
//! runtime evidence: they create no event, causal edge, replay occurrence, or second history.

use std::collections::{BTreeMap, BTreeSet};

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

/// A derived causal reading of an explicitly supplied finite traversal.
///
/// Ledger order is durable event membership and append order.  It is not evidence of a causal
/// direction.  A caller may provide a separately supported, acyclic candidate edge set; this
/// type preserves that distinction without creating authoritative causal history.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TraversalCausalOrder {
    /// No causal-order evidence was admitted for this traversal.
    Unknown,
    /// A separately declared candidate edge set, oriented `cause -> consequence`.
    Declared(Vec<(EventRef, EventRef)>),
}

/// A derived finite view joining paired actuality projections with ledger membership.
///
/// This view neither writes history nor promotes a declared edge set to causal actuality.  Its
/// only invariant is exact event coverage and internally coherent candidate-edge syntax.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PairedActualityTraversal {
    traces: Vec<PairedActualityTrace>,
    ledger_order: Vec<EventRef>,
    causal_order: TraversalCausalOrder,
}

impl PairedActualityTraversal {
    /// Builds a finite derived traversal.  The ledger order must enumerate each paired event
    /// exactly once.  Declared causal edges may differ from ledger order, but must stay inside
    /// the traversal and be acyclic.
    pub fn new(
        traces: Vec<PairedActualityTrace>,
        ledger_order: Vec<EventRef>,
        causal_order: TraversalCausalOrder,
    ) -> Result<Self, PairedActualityTraversalError> {
        if traces.is_empty() {
            return Err(PairedActualityTraversalError::EmptyTraversal);
        }

        let trace_events: BTreeSet<_> = traces
            .iter()
            .map(|trace| trace.question().event())
            .collect();
        if trace_events.len() != traces.len() {
            return Err(PairedActualityTraversalError::DuplicateTraceEvent);
        }
        if traces
            .iter()
            .any(|trace| trace.question().event() != trace.returned().event())
        {
            return Err(PairedActualityTraversalError::MismatchedTraceEvent);
        }

        let ledger_events: BTreeSet<_> = ledger_order.iter().copied().collect();
        if ledger_events.len() != ledger_order.len() {
            return Err(PairedActualityTraversalError::DuplicateLedgerEvent);
        }
        if ledger_events != trace_events {
            return Err(PairedActualityTraversalError::LedgerCoverageMismatch);
        }
        if let TraversalCausalOrder::Declared(edges) = &causal_order {
            validate_causal_edges(&trace_events, edges)?;
        }

        Ok(Self {
            traces,
            ledger_order,
            causal_order,
        })
    }

    #[must_use]
    pub fn traces(&self) -> &[PairedActualityTrace] {
        &self.traces
    }

    #[must_use]
    pub fn ledger_order(&self) -> &[EventRef] {
        &self.ledger_order
    }

    #[must_use]
    pub const fn causal_order(&self) -> &TraversalCausalOrder {
        &self.causal_order
    }
}

fn validate_causal_edges(
    events: &BTreeSet<EventRef>,
    edges: &[(EventRef, EventRef)],
) -> Result<(), PairedActualityTraversalError> {
    let mut seen = BTreeSet::new();
    let mut successors: BTreeMap<EventRef, Vec<EventRef>> = BTreeMap::new();
    for &(from, to) in edges {
        if !events.contains(&from) || !events.contains(&to) {
            return Err(PairedActualityTraversalError::CausalEndpointOutsideTraversal { from, to });
        }
        if from == to {
            return Err(PairedActualityTraversalError::CausalSelfEdge { event: from });
        }
        if !seen.insert((from, to)) {
            return Err(PairedActualityTraversalError::DuplicateCausalEdge { from, to });
        }
        successors.entry(from).or_default().push(to);
    }

    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    if events
        .iter()
        .copied()
        .any(|event| causal_cycle(event, &successors, &mut visiting, &mut visited))
    {
        return Err(PairedActualityTraversalError::CausalCycle);
    }
    Ok(())
}

fn causal_cycle(
    event: EventRef,
    successors: &BTreeMap<EventRef, Vec<EventRef>>,
    visiting: &mut BTreeSet<EventRef>,
    visited: &mut BTreeSet<EventRef>,
) -> bool {
    if visited.contains(&event) {
        return false;
    }
    if !visiting.insert(event) {
        return true;
    }
    let has_cycle = successors.get(&event).is_some_and(|next| {
        next.iter()
            .copied()
            .any(|child| causal_cycle(child, successors, visiting, visited))
    });
    visiting.remove(&event);
    visited.insert(event);
    has_cycle
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

/// Failures while deriving a finite paired-actuality traversal.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum PairedActualityTraversalError {
    #[error("a paired-actuality traversal must contain at least one trace")]
    EmptyTraversal,
    #[error("a paired-actuality traversal contains the same trace event more than once")]
    DuplicateTraceEvent,
    #[error("a paired trace does not pair one question event with the same return event")]
    MismatchedTraceEvent,
    #[error("the ledger order contains the same event more than once")]
    DuplicateLedgerEvent,
    #[error("ledger membership does not exactly cover the paired trace events")]
    LedgerCoverageMismatch,
    #[error("declared causal edge {from} -> {to} leaves the traversal")]
    CausalEndpointOutsideTraversal { from: EventRef, to: EventRef },
    #[error("declared causal edge {event} -> {event} is reflexive")]
    CausalSelfEdge { event: EventRef },
    #[error("declared causal edge {from} -> {to} appears more than once")]
    DuplicateCausalEdge { from: EventRef, to: EventRef },
    #[error("declared causal edges contain a directed cycle")]
    CausalCycle,
}

#[cfg(test)]
mod traversal_tests {
    use super::*;

    fn artifact(byte: u8) -> ic_core::ArtifactRef {
        ic_core::ArtifactRef::from_bytes([byte; 32])
    }

    fn trace(byte: u8) -> PairedActualityTrace {
        let event = EventRef::from_artifact_ref(artifact(byte));
        PairedActualityTrace {
            question: QuestionTrace {
                event,
                source: IProgRef::from_artifact_ref(artifact(byte.wrapping_add(1))),
                question: QueryRef::from_artifact_ref(artifact(byte.wrapping_add(2))),
                operator: ProbeOperatorRef::from_artifact_ref(artifact(byte.wrapping_add(3))),
                boundary: BoundaryRef::from_artifact_ref(artifact(byte.wrapping_add(4))),
                state_before: StateRef::from_artifact_ref(artifact(byte.wrapping_add(5))),
                route: RouteRef::from_artifact_ref(artifact(byte.wrapping_add(6))),
                binding: BindingVersionRef::from_artifact_ref(artifact(byte.wrapping_add(7))),
                provenance: ProvenanceRef::from_artifact_ref(artifact(byte.wrapping_add(8))),
            },
            returned: ReturnTrace {
                event,
                raw_return: RawReturnRef::from_artifact_ref(artifact(byte.wrapping_add(9))),
                decoder: FiniteDecoderRef::from_artifact_ref(artifact(byte.wrapping_add(10))),
                path: ResolutionPathRef::from_artifact_ref(artifact(byte.wrapping_add(11))),
                candidates: Vec::new(),
                observations: Vec::new(),
                continuation: IProgRef::from_artifact_ref(artifact(byte.wrapping_add(12))),
                resume_target: BlockTarget::new(u32::from(byte)),
                state_after: StateRef::from_artifact_ref(artifact(byte.wrapping_add(13))),
                route: RouteRef::from_artifact_ref(artifact(byte.wrapping_add(6))),
                binding: BindingVersionRef::from_artifact_ref(artifact(byte.wrapping_add(7))),
                provenance: ProvenanceRef::from_artifact_ref(artifact(byte.wrapping_add(8))),
            },
        }
    }

    #[test]
    fn traversal_preserves_ledger_and_causal_order_as_distinct_readings() {
        let first = trace(1);
        let second = trace(32);
        let first_event = first.question().event();
        let second_event = second.question().event();

        let unknown = PairedActualityTraversal::new(
            vec![first.clone(), second.clone()],
            vec![first_event, second_event],
            TraversalCausalOrder::Unknown,
        )
        .expect("complete ledger membership with unknown causality must remain representable");
        assert!(matches!(
            unknown.causal_order(),
            TraversalCausalOrder::Unknown
        ));

        let declared = PairedActualityTraversal::new(
            vec![first, second],
            vec![first_event, second_event],
            TraversalCausalOrder::Declared(vec![(second_event, first_event)]),
        )
        .expect("a separately declared reverse edge is not contradicted by ledger order");
        assert_eq!(declared.ledger_order(), [first_event, second_event]);
        assert_eq!(
            declared.causal_order(),
            &TraversalCausalOrder::Declared(vec![(second_event, first_event)])
        );
    }

    #[test]
    fn traversal_rejects_incomplete_ledger_coverage_and_causal_cycles() {
        let first = trace(1);
        let second = trace(32);
        let first_event = first.question().event();
        let second_event = second.question().event();
        assert!(matches!(
            PairedActualityTraversal::new(
                vec![first.clone(), second.clone()],
                vec![first_event],
                TraversalCausalOrder::Unknown,
            ),
            Err(PairedActualityTraversalError::LedgerCoverageMismatch)
        ));
        assert!(matches!(
            PairedActualityTraversal::new(
                vec![first, second],
                vec![first_event, second_event],
                TraversalCausalOrder::Declared(vec![
                    (first_event, second_event),
                    (second_event, first_event),
                ]),
            ),
            Err(PairedActualityTraversalError::CausalCycle)
        ));
    }
}

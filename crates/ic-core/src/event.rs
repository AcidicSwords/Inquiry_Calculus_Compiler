//! Canonical ordinary-actuality event records.
//!
//! An [`ActualEvent`] records an already-realized external return in the append-only event
//! ledger.  It does not dispatch an operator, decode the return, select an answer, establish a
//! claim, or warrant an interpretation.  The required boundary reference follows the canonical
//! v1.1 event spine; the optional distinction preserves the implementation-plan's additional
//! reciprocal context without allowing either field to stand in for the other.

use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, BindingVersionRef, BoundaryChart,
    BoundaryChartError, DistinctionRef, GrainRef, OpenQuery, OpenQueryError, ProbeOperator,
    ProbeOperatorError, ProbeOperatorRef, QueryRef, RawReturn, RawReturnError, RawReturnRef,
};

/// Canonical artifact kind for one ordinary, realized actuality occurrence.
pub const ACTUAL_EVENT_ARTIFACT_KIND: &str = "ic.actual-event";
/// Payload schema version for ordinary actuality occurrences.
pub const ACTUAL_EVENT_SCHEMA_VERSION: u32 = 1;

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

artifact_reference!(EventRef);
artifact_reference!(StateRef);
artifact_reference!(BoundaryRef);
artifact_reference!(RouteRef);
artifact_reference!(ProvenanceRef);

/// The event-spine name for the shared compiled probe-operator identity.
pub type OperatorRef = ProbeOperatorRef;

/// One append-only actual occurrence.
///
/// All semantic interpretation remains downstream.  `boundary` is required because an actual
/// event is positioned in the active canonical boundary chart.  `distinction` is optional because
/// not every ordinary actualization is a reciprocal-boundary occurrence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActualEvent {
    ledger_parent: Option<EventRef>,
    state_before: StateRef,
    question: QueryRef,
    boundary: BoundaryRef,
    distinction: Option<DistinctionRef>,
    operator: ProbeOperatorRef,
    raw_return: RawReturnRef,
    state_after: StateRef,
    grain: GrainRef,
    route: RouteRef,
    binding: BindingVersionRef,
    backend_version: ArtifactRef,
    provenance: ProvenanceRef,
}

impl ActualEvent {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        ledger_parent: Option<EventRef>,
        state_before: StateRef,
        question: QueryRef,
        boundary: BoundaryRef,
        distinction: Option<DistinctionRef>,
        operator: ProbeOperatorRef,
        raw_return: RawReturnRef,
        state_after: StateRef,
        grain: GrainRef,
        route: RouteRef,
        binding: BindingVersionRef,
        backend_version: ArtifactRef,
        provenance: ProvenanceRef,
    ) -> Self {
        Self {
            ledger_parent,
            state_before,
            question,
            boundary,
            distinction,
            operator,
            raw_return,
            state_after,
            grain,
            route,
            binding,
            backend_version,
            provenance,
        }
    }

    #[must_use]
    pub const fn ledger_parent(&self) -> Option<EventRef> {
        self.ledger_parent
    }
    #[must_use]
    pub const fn state_before(&self) -> StateRef {
        self.state_before
    }
    #[must_use]
    pub const fn question(&self) -> QueryRef {
        self.question
    }
    #[must_use]
    pub const fn boundary(&self) -> BoundaryRef {
        self.boundary
    }
    #[must_use]
    pub const fn distinction(&self) -> Option<DistinctionRef> {
        self.distinction
    }
    #[must_use]
    pub const fn operator(&self) -> ProbeOperatorRef {
        self.operator
    }
    #[must_use]
    pub const fn raw_return(&self) -> RawReturnRef {
        self.raw_return
    }
    #[must_use]
    pub const fn state_after(&self) -> StateRef {
        self.state_after
    }
    #[must_use]
    pub const fn grain(&self) -> GrainRef {
        self.grain
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
    pub const fn backend_version(&self) -> ArtifactRef {
        self.backend_version
    }
    #[must_use]
    pub const fn provenance(&self) -> ProvenanceRef {
        self.provenance
    }

    pub fn canonical_payload(&self) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(32 * 13 + 2);
        optional_reference(
            &mut encoded,
            self.ledger_parent.map(EventRef::as_artifact_ref),
        );
        reference(&mut encoded, self.state_before.as_artifact_ref());
        reference(&mut encoded, self.question.as_artifact_ref());
        reference(&mut encoded, self.boundary.as_artifact_ref());
        optional_reference(
            &mut encoded,
            self.distinction.map(DistinctionRef::as_artifact_ref),
        );
        reference(&mut encoded, self.operator.as_artifact_ref());
        reference(&mut encoded, self.raw_return.as_artifact_ref());
        reference(&mut encoded, self.state_after.as_artifact_ref());
        reference(&mut encoded, self.grain.as_artifact_ref());
        reference(&mut encoded, self.route.as_artifact_ref());
        reference(&mut encoded, self.binding.as_artifact_ref());
        reference(&mut encoded, self.backend_version);
        reference(&mut encoded, self.provenance.as_artifact_ref());
        encoded
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, ActualEventError> {
        let mut cursor = Cursor::new(payload);
        let ledger_parent = cursor
            .optional_reference()?
            .map(EventRef::from_artifact_ref);
        let state_before = StateRef::from_artifact_ref(cursor.reference()?);
        let question = QueryRef::from_artifact_ref(cursor.reference()?);
        let boundary = BoundaryRef::from_artifact_ref(cursor.reference()?);
        let distinction = cursor
            .optional_reference()?
            .map(DistinctionRef::from_artifact_ref);
        let operator = OperatorRef::from_artifact_ref(cursor.reference()?);
        let raw_return = RawReturnRef::from_artifact_ref(cursor.reference()?);
        let state_after = StateRef::from_artifact_ref(cursor.reference()?);
        let grain = GrainRef::from_artifact_ref(cursor.reference()?);
        let route = RouteRef::from_artifact_ref(cursor.reference()?);
        let binding = BindingVersionRef::from_artifact_ref(cursor.reference()?);
        let backend_version = cursor.reference()?;
        let provenance = ProvenanceRef::from_artifact_ref(cursor.reference()?);
        if !cursor.finished() {
            return Err(ActualEventError::TrailingPayloadBytes(cursor.remaining()));
        }
        Ok(Self::new(
            ledger_parent,
            state_before,
            question,
            boundary,
            distinction,
            operator,
            raw_return,
            state_after,
            grain,
            route,
            binding,
            backend_version,
            provenance,
        ))
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, ActualEventError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(ACTUAL_EVENT_ARTIFACT_KIND)?,
            ACTUAL_EVENT_SCHEMA_VERSION,
            self.canonical_payload(),
        ))
    }

    pub fn event_ref(&self) -> Result<EventRef, ActualEventError> {
        Ok(EventRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, ActualEventError> {
        if envelope.kind().as_str() != ACTUAL_EVENT_ARTIFACT_KIND {
            return Err(ActualEventError::UnexpectedArtifactKind {
                expected: ACTUAL_EVENT_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != ACTUAL_EVENT_SCHEMA_VERSION {
            return Err(ActualEventError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    /// References that must exist before the event can enter the immutable artifact store.
    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = Vec::with_capacity(13);
        if let Some(parent) = self.ledger_parent {
            references.push(parent.as_artifact_ref());
        }
        references.extend([
            self.state_before.as_artifact_ref(),
            self.question.as_artifact_ref(),
            self.boundary.as_artifact_ref(),
        ]);
        if let Some(distinction) = self.distinction {
            references.push(distinction.as_artifact_ref());
        }
        references.extend([
            self.operator.as_artifact_ref(),
            self.raw_return.as_artifact_ref(),
            self.state_after.as_artifact_ref(),
            self.grain.as_artifact_ref(),
            self.route.as_artifact_ref(),
            self.binding.as_artifact_ref(),
            self.backend_version,
            self.provenance.as_artifact_ref(),
        ]);
        references
    }
}

fn reference(encoded: &mut Vec<u8>, value: ArtifactRef) {
    encoded.extend_from_slice(value.as_bytes());
}

fn optional_reference(encoded: &mut Vec<u8>, value: Option<ArtifactRef>) {
    match value {
        Some(value) => {
            encoded.push(1);
            reference(encoded, value);
        }
        None => encoded.push(0),
    }
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], ActualEventError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(ActualEventError::PayloadLengthOverflow)?;
        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(ActualEventError::TruncatedPayload)?;
        self.position = end;
        Ok(bytes)
    }

    fn reference(&mut self) -> Result<ArtifactRef, ActualEventError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| ActualEventError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(bytes))
    }

    fn optional_reference(&mut self) -> Result<Option<ArtifactRef>, ActualEventError> {
        match self.take(1)?[0] {
            0 => Ok(None),
            1 => self.reference().map(Some),
            tag => Err(ActualEventError::UnknownOptionalTag(tag)),
        }
    }

    const fn finished(&self) -> bool {
        self.position == self.bytes.len()
    }

    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

/// Errors from canonical actual-event encoding and decoding.
#[derive(Debug, Error)]
pub enum ActualEventError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("actual-event payload is truncated")]
    TruncatedPayload,
    #[error("actual-event payload length overflows this platform")]
    PayloadLengthOverflow,
    #[error("actual-event payload contains {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("actual-event payload has an unknown optional-reference tag {0}")]
    UnknownOptionalTag(u8),
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported actual-event schema version {0}")]
    UnsupportedSchemaVersion(u32),
    #[error(transparent)]
    RawReturn(#[from] RawReturnError),
}

/// Verifies only that an event's named raw-return artifact is available and hashes correctly.
///
/// Broader question, boundary, operator, route, state, and provenance validation remains pending
/// their corresponding typed artifact contracts.  This check cannot establish that a tool call
/// occurred; ledger insertion is the ordinary-history assertion after an authorized actualization.
pub fn check_raw_return<C: RawReturnCatalog>(
    event: &ActualEvent,
    catalog: &C,
) -> Result<(), ActualEventCheckError> {
    let raw_return = catalog
        .resolve_raw_return(event.raw_return)
        .ok_or(ActualEventCheckError::UnresolvedRawReturn(event.raw_return))?;
    let calculated = raw_return.raw_return_ref()?;
    if calculated != event.raw_return {
        return Err(ActualEventCheckError::RawReturnIdentityMismatch {
            reference: event.raw_return,
            calculated,
        });
    }
    Ok(())
}

/// Verifies the typed raw-return, query, boundary-chart, and probe-operator identities retained
/// by an actual event.
///
/// This does not validate an actual dispatch or the opaque state, query semantics, operator
/// contracts, route, backend, provenance, or chart-field semantics. Those contracts remain
/// distinct later work.
pub fn check_actual_event<C: ActualEventCatalog>(
    event: &ActualEvent,
    catalog: &C,
) -> Result<(), ActualEventCheckError> {
    check_raw_return(event, catalog)?;
    let chart = catalog
        .resolve_boundary_chart(event.boundary)
        .ok_or(ActualEventCheckError::UnresolvedBoundary(event.boundary))?;
    let calculated = chart.boundary_ref()?;
    if calculated != event.boundary {
        return Err(ActualEventCheckError::BoundaryIdentityMismatch {
            reference: event.boundary,
            calculated,
        });
    }
    let operator = catalog.resolve_probe_operator(event.operator).ok_or(
        ActualEventCheckError::UnresolvedProbeOperator(event.operator),
    )?;
    let question = catalog
        .resolve_open_query(event.question)
        .ok_or(ActualEventCheckError::UnresolvedQuestion(event.question))?;
    check_event_context(event, &question, &chart, &operator)
}

/// Checks that an event's named query, chart, and compiled operator are the same occurrence
/// context, after callers have resolved those artifacts. This establishes identity linkage only.
pub fn check_event_context(
    event: &ActualEvent,
    question: &OpenQuery,
    chart: &BoundaryChart,
    operator: &ProbeOperator,
) -> Result<(), ActualEventCheckError> {
    let calculated_question = question.query_ref()?;
    if calculated_question != event.question {
        return Err(ActualEventCheckError::QuestionIdentityMismatch {
            reference: event.question,
            calculated: calculated_question,
        });
    }
    let calculated_boundary = chart.boundary_ref()?;
    if calculated_boundary != event.boundary {
        return Err(ActualEventCheckError::BoundaryIdentityMismatch {
            reference: event.boundary,
            calculated: calculated_boundary,
        });
    }
    let calculated_operator = operator.probe_operator_ref()?;
    if calculated_operator != event.operator {
        return Err(ActualEventCheckError::ProbeOperatorIdentityMismatch {
            reference: event.operator,
            calculated: calculated_operator,
        });
    }
    if chart.query() != event.question {
        return Err(ActualEventCheckError::BoundaryQuestionMismatch {
            event: event.question,
            boundary: chart.query(),
        });
    }
    if operator.query() != event.question {
        return Err(ActualEventCheckError::OperatorQuestionMismatch {
            event: event.question,
            operator: operator.query(),
        });
    }
    if operator.boundary() != event.boundary {
        return Err(ActualEventCheckError::OperatorBoundaryMismatch {
            event: event.boundary,
            operator: operator.boundary(),
        });
    }
    if chart.grain() != event.grain {
        return Err(ActualEventCheckError::BoundaryGrainMismatch {
            event: event.grain,
            boundary: chart.grain(),
        });
    }
    Ok(())
}

/// Minimal catalog required to verify the preserved opaque raw return.
pub trait RawReturnCatalog {
    fn resolve_raw_return(&self, reference: RawReturnRef) -> Option<RawReturn>;
}

/// The currently available catalog boundary for actual-event identity checking.
pub trait ActualEventCatalog: RawReturnCatalog {
    fn resolve_boundary_chart(&self, reference: BoundaryRef) -> Option<BoundaryChart>;
    fn resolve_probe_operator(&self, reference: ProbeOperatorRef) -> Option<ProbeOperator>;
    fn resolve_open_query(&self, reference: QueryRef) -> Option<OpenQuery>;
}

/// Errors from the currently available actual-event validation boundary.
#[derive(Debug, Error)]
pub enum ActualEventCheckError {
    #[error(transparent)]
    Encoding(#[from] ActualEventError),
    #[error(transparent)]
    RawReturn(#[from] RawReturnError),
    #[error(transparent)]
    BoundaryChart(#[from] BoundaryChartError),
    #[error(transparent)]
    ProbeOperator(#[from] ProbeOperatorError),
    #[error(transparent)]
    OpenQuery(#[from] OpenQueryError),
    #[error("raw return {0} is unavailable from the declared catalog")]
    UnresolvedRawReturn(RawReturnRef),
    #[error("catalog raw return {reference} hashes to {calculated}, not its claimed identity")]
    RawReturnIdentityMismatch {
        reference: RawReturnRef,
        calculated: RawReturnRef,
    },
    #[error("boundary chart {0} is unavailable from the declared catalog")]
    UnresolvedBoundary(BoundaryRef),
    #[error("catalog boundary chart {reference} hashes to {calculated}, not its claimed identity")]
    BoundaryIdentityMismatch {
        reference: BoundaryRef,
        calculated: BoundaryRef,
    },
    #[error("probe operator {0} is unavailable from the declared catalog")]
    UnresolvedProbeOperator(ProbeOperatorRef),
    #[error("catalog probe operator {reference} hashes to {calculated}, not its claimed identity")]
    ProbeOperatorIdentityMismatch {
        reference: ProbeOperatorRef,
        calculated: ProbeOperatorRef,
    },
    #[error("question {0} is unavailable from the declared catalog")]
    UnresolvedQuestion(QueryRef),
    #[error("catalog question {reference} hashes to {calculated}, not its claimed identity")]
    QuestionIdentityMismatch {
        reference: QueryRef,
        calculated: QueryRef,
    },
    #[error("event question {event} does not match boundary-chart question {boundary}")]
    BoundaryQuestionMismatch { event: QueryRef, boundary: QueryRef },
    #[error("event question {event} does not match probe-operator question {operator}")]
    OperatorQuestionMismatch { event: QueryRef, operator: QueryRef },
    #[error("event boundary {event} does not match probe-operator boundary {operator}")]
    OperatorBoundaryMismatch {
        event: BoundaryRef,
        operator: BoundaryRef,
    },
    #[error("event grain {event} does not match boundary-chart grain {boundary}")]
    BoundaryGrainMismatch { event: GrainRef, boundary: GrainRef },
}

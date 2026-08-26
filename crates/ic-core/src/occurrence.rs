//! Derived operator-occurrence views over ordinary actual events.
//!
//! An occurrence preserves exactly the operator/state/raw-return links already carried by one
//! authoritative event. It does not create a new event, prove dispatch, or interpret the return.

use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ActualEvent, ActualEventCatalog, ActualEventCheckError, ActualEventError, ArtifactEnvelope,
    ArtifactError, ArtifactKind, ArtifactRef, BoundaryRef, EventRef, ProbeOperatorRef,
    RawReturnRef, StateRef, check_actual_event,
};

/// Canonical artifact kind for derived operator occurrences.
pub const OPERATOR_OCCURRENCE_ARTIFACT_KIND: &str = "ic.operator-occurrence";
/// Payload schema version for operator occurrences.
pub const OPERATOR_OCCURRENCE_SCHEMA_VERSION: u32 = 1;

/// Stable identity of one derived operator-occurrence view.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OperatorOccurrenceRef(ArtifactRef);

impl OperatorOccurrenceRef {
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }
    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}

impl fmt::Display for OperatorOccurrenceRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for OperatorOccurrenceRef {
    type Err = ArtifactError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// A derived operator occurrence tied to one exact ordinary event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OperatorOccurrence {
    event: EventRef,
    operator: ProbeOperatorRef,
    state_before: StateRef,
    raw_return: RawReturnRef,
    state_after: StateRef,
    boundary: BoundaryRef,
}

impl OperatorOccurrence {
    #[must_use]
    pub const fn new(
        event: EventRef,
        operator: ProbeOperatorRef,
        state_before: StateRef,
        raw_return: RawReturnRef,
        state_after: StateRef,
        boundary: BoundaryRef,
    ) -> Self {
        Self {
            event,
            operator,
            state_before,
            raw_return,
            state_after,
            boundary,
        }
    }

    #[must_use]
    pub const fn event(&self) -> EventRef {
        self.event
    }
    #[must_use]
    pub const fn operator(&self) -> ProbeOperatorRef {
        self.operator
    }
    #[must_use]
    pub const fn state_before(&self) -> StateRef {
        self.state_before
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
    pub const fn boundary(&self) -> BoundaryRef {
        self.boundary
    }

    #[must_use]
    pub fn canonical_payload(&self) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(32 * 6);
        for reference in self.referenced_artifacts() {
            encoded.extend_from_slice(reference.as_bytes());
        }
        encoded
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, OperatorOccurrenceError> {
        if payload.len() != 32 * 6 {
            return Err(OperatorOccurrenceError::WrongPayloadLength(payload.len()));
        }
        let reference = |index: usize| {
            let bytes: [u8; 32] = payload[index * 32..(index + 1) * 32]
                .try_into()
                .expect("payload length is checked before fixed-width parsing");
            ArtifactRef::from_bytes(bytes)
        };
        Ok(Self::new(
            EventRef::from_artifact_ref(reference(0)),
            ProbeOperatorRef::from_artifact_ref(reference(1)),
            StateRef::from_artifact_ref(reference(2)),
            RawReturnRef::from_artifact_ref(reference(3)),
            StateRef::from_artifact_ref(reference(4)),
            BoundaryRef::from_artifact_ref(reference(5)),
        ))
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, OperatorOccurrenceError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(OPERATOR_OCCURRENCE_ARTIFACT_KIND)?,
            OPERATOR_OCCURRENCE_SCHEMA_VERSION,
            self.canonical_payload(),
        ))
    }

    pub fn operator_occurrence_ref(
        &self,
    ) -> Result<OperatorOccurrenceRef, OperatorOccurrenceError> {
        Ok(OperatorOccurrenceRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, OperatorOccurrenceError> {
        if envelope.kind().as_str() != OPERATOR_OCCURRENCE_ARTIFACT_KIND {
            return Err(OperatorOccurrenceError::UnexpectedArtifactKind {
                expected: OPERATOR_OCCURRENCE_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != OPERATOR_OCCURRENCE_SCHEMA_VERSION {
            return Err(OperatorOccurrenceError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> [ArtifactRef; 6] {
        [
            self.event.as_artifact_ref(),
            self.operator.as_artifact_ref(),
            self.state_before.as_artifact_ref(),
            self.raw_return.as_artifact_ref(),
            self.state_after.as_artifact_ref(),
            self.boundary.as_artifact_ref(),
        ]
    }

    /// Rechecks the ordinary event and requires every occurrence field to agree exactly.
    pub fn check<C: OperatorOccurrenceCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), OperatorOccurrenceCheckError> {
        let event = catalog
            .resolve_actual_event(self.event)
            .ok_or(OperatorOccurrenceCheckError::UnresolvedEvent(self.event))?;
        let calculated = event.event_ref()?;
        if calculated != self.event {
            return Err(OperatorOccurrenceCheckError::EventIdentityMismatch {
                reference: self.event,
                calculated,
            });
        }
        check_actual_event(&event, catalog)?;
        check_equal(
            "operator",
            self.operator.as_artifact_ref(),
            event.operator().as_artifact_ref(),
        )?;
        check_equal(
            "state_before",
            self.state_before.as_artifact_ref(),
            event.state_before().as_artifact_ref(),
        )?;
        check_equal(
            "raw_return",
            self.raw_return.as_artifact_ref(),
            event.raw_return().as_artifact_ref(),
        )?;
        check_equal(
            "state_after",
            self.state_after.as_artifact_ref(),
            event.state_after().as_artifact_ref(),
        )?;
        check_equal(
            "boundary",
            self.boundary.as_artifact_ref(),
            event.boundary().as_artifact_ref(),
        )?;
        Ok(())
    }

    /// Constructs the sole occurrence view induced by a checked actual event.
    pub fn from_actual_event<C: OperatorOccurrenceCatalog>(
        event_ref: EventRef,
        catalog: &C,
    ) -> Result<Self, OperatorOccurrenceCheckError> {
        let event = catalog
            .resolve_actual_event(event_ref)
            .ok_or(OperatorOccurrenceCheckError::UnresolvedEvent(event_ref))?;
        let occurrence = Self::new(
            event_ref,
            event.operator(),
            event.state_before(),
            event.raw_return(),
            event.state_after(),
            event.boundary(),
        );
        occurrence.check(catalog)?;
        Ok(occurrence)
    }
}

fn check_equal(
    field: &'static str,
    occurrence: ArtifactRef,
    event: ArtifactRef,
) -> Result<(), OperatorOccurrenceCheckError> {
    if occurrence != event {
        return Err(OperatorOccurrenceCheckError::EventFieldMismatch {
            field,
            occurrence,
            event,
        });
    }
    Ok(())
}

/// Catalog boundary for rechecking an occurrence against ordinary actual history.
pub trait OperatorOccurrenceCatalog: ActualEventCatalog {
    fn resolve_actual_event(&self, reference: EventRef) -> Option<ActualEvent>;
}

#[derive(Debug, Error)]
pub enum OperatorOccurrenceError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("operator-occurrence payload has wrong length {0}")]
    WrongPayloadLength(usize),
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported operator-occurrence schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

#[derive(Debug, Error)]
pub enum OperatorOccurrenceCheckError {
    #[error(transparent)]
    Occurrence(#[from] OperatorOccurrenceError),
    #[error(transparent)]
    Event(#[from] ActualEventError),
    #[error(transparent)]
    EventCheck(#[from] ActualEventCheckError),
    #[error("actual event {0} is unavailable")]
    UnresolvedEvent(EventRef),
    #[error("actual event {reference} hashes to {calculated}, not its claimed identity")]
    EventIdentityMismatch {
        reference: EventRef,
        calculated: EventRef,
    },
    #[error(
        "operator occurrence {field} reference {occurrence} differs from event reference {event}"
    )]
    EventFieldMismatch {
        field: &'static str,
        occurrence: ArtifactRef,
        event: ArtifactRef,
    },
}

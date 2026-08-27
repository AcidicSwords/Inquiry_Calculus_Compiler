//! Crash-safe execution boundary for one injected probe provider.
//!
//! The coordinator is deliberately narrower than a provider framework. It admits dispatch only
//! after a fresh durable preparation, preserves the exact returned bytes as a `RawReturn`, and
//! completes the ordinary event before returning data to any decoder.

use std::error::Error;

use ic_core::{
    ActualEvent, AskOccurrenceRef, BackendBoundaryError, BackendRequest, BackendRequestRef,
    BindingVersionRef, DistinctionRef, EventRef, GrainRef, ProvenanceRef, RawReturn,
    RawReturnError, RawReturnRef, RouteRef, StateRef,
};
use ic_store::{
    ArtifactStore, DispatchToken, ExternalEffectPreparation, ExternalEffectState, StoreError,
};
use thiserror::Error;

use crate::ProbeSuspension;

/// Event fields fixed before provider dispatch.
///
/// Question, boundary, operator, and backend version come from the checked backend request. The
/// provider supplies only opaque returned bytes; it cannot rewrite event identity or state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProbeDispatchContext {
    ledger_parent: Option<EventRef>,
    source_ask_occurrence: Option<AskOccurrenceRef>,
    state_before: StateRef,
    distinction: Option<DistinctionRef>,
    state_after: StateRef,
    grain: GrainRef,
    route: RouteRef,
    binding: BindingVersionRef,
    provenance: ProvenanceRef,
}

impl ProbeDispatchContext {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        ledger_parent: Option<EventRef>,
        state_before: StateRef,
        distinction: Option<DistinctionRef>,
        state_after: StateRef,
        grain: GrainRef,
        route: RouteRef,
        binding: BindingVersionRef,
        provenance: ProvenanceRef,
    ) -> Self {
        Self {
            ledger_parent,
            source_ask_occurrence: None,
            state_before,
            distinction,
            state_after,
            grain,
            route,
            binding,
            provenance,
        }
    }

    #[must_use]
    pub const fn ledger_parent(self) -> Option<EventRef> {
        self.ledger_parent
    }

    /// Links a source-compiled dispatch to the exact already-persisted `Ask` occurrence.
    #[must_use]
    pub const fn with_source_ask_occurrence(mut self, occurrence: AskOccurrenceRef) -> Self {
        self.source_ask_occurrence = Some(occurrence);
        self
    }

    #[must_use]
    pub const fn source_ask_occurrence(self) -> Option<AskOccurrenceRef> {
        self.source_ask_occurrence
    }
}

/// Exact opaque bytes returned by a provider invocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderReturn {
    bytes: Vec<u8>,
}

impl ProviderReturn {
    #[must_use]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

/// Injected operational provider boundary.
///
/// Implementations may be mocks or real adapters. A returned error is operational failure, not a
/// decoder result, semantic non-discharge, `Unknown`, or warrant.
pub trait ProbeProvider {
    type Error: Error + Send + Sync + 'static;

    fn dispatch(&mut self, request: &BackendRequest) -> Result<ProviderReturn, Self::Error>;
}

/// One provider return after its raw bytes and ordinary event have committed atomically.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActualizedProbe {
    request: BackendRequestRef,
    event: ActualEvent,
    event_ref: EventRef,
    raw_return: RawReturn,
    raw_return_ref: RawReturnRef,
}

impl ActualizedProbe {
    #[must_use]
    pub const fn request(&self) -> BackendRequestRef {
        self.request
    }

    #[must_use]
    pub const fn event(&self) -> &ActualEvent {
        &self.event
    }

    #[must_use]
    pub const fn event_ref(&self) -> EventRef {
        self.event_ref
    }

    #[must_use]
    pub const fn raw_return(&self) -> &RawReturn {
        &self.raw_return
    }

    #[must_use]
    pub const fn raw_return_ref(&self) -> RawReturnRef {
        self.raw_return_ref
    }
}

/// Executes one freshly prepared probe and commits its ordinary actuality before interpretation.
pub async fn dispatch_probe<P: ProbeProvider>(
    store: &ArtifactStore,
    suspension: ProbeSuspension,
    token: DispatchToken,
    request: BackendRequestRef,
    context: ProbeDispatchContext,
    provider: &mut P,
) -> Result<ActualizedProbe, ProbeDispatchError<P::Error>> {
    let request_value = store.checked_backend_request(request).await?;
    if request_value.operator() != suspension.operator() {
        return Err(ProbeDispatchError::SuspendedOperatorMismatch {
            suspended: suspension.operator(),
            request: request_value.operator(),
        });
    }

    let preparation = store
        .prepare_backend_request(token, request, suspension.operator(), context.ledger_parent)
        .await?;
    let ExternalEffectPreparation::DispatchAuthorized(_) = preparation else {
        return Err(ProbeDispatchError::DispatchNotAuthorized(
            preparation.state(),
        ));
    };

    let provider_return = provider
        .dispatch(&request_value)
        .map_err(ProbeDispatchError::Provider)?;
    let raw_return = RawReturn::new(provider_return.into_bytes());
    let raw_return_ref = raw_return.raw_return_ref()?;
    let event = if let Some(source_ask_occurrence) = context.source_ask_occurrence {
        ActualEvent::new_source_linked(
            context.ledger_parent,
            context.state_before,
            source_ask_occurrence,
            request_value.query(),
            request_value.boundary(),
            context.distinction,
            suspension.operator(),
            raw_return_ref,
            context.state_after,
            context.grain,
            context.route,
            context.binding,
            request_value.compiler_version(),
            request_value.backend_version(),
            context.provenance,
        )
    } else {
        ActualEvent::new(
            context.ledger_parent,
            context.state_before,
            request_value.query(),
            request_value.boundary(),
            context.distinction,
            suspension.operator(),
            raw_return_ref,
            context.state_after,
            context.grain,
            context.route,
            context.binding,
            request_value.backend_version(),
            context.provenance,
        )
    };
    let event_ref = store
        .complete_external_effect(token, &raw_return, &event)
        .await?;
    Ok(ActualizedProbe {
        request,
        event,
        event_ref,
        raw_return,
        raw_return_ref,
    })
}

#[derive(Debug, Error)]
pub enum ProbeDispatchError<E: Error + Send + Sync + 'static> {
    #[error(transparent)]
    Store(#[from] StoreError),
    #[error(transparent)]
    BackendBoundary(#[from] BackendBoundaryError),
    #[error(transparent)]
    RawReturn(#[from] RawReturnError),
    #[error("suspended operator {suspended} differs from backend request operator {request}")]
    SuspendedOperatorMismatch {
        suspended: ic_core::ProbeOperatorRef,
        request: ic_core::ProbeOperatorRef,
    },
    #[error("an existing effect row carries recovery state but no dispatch authority")]
    DispatchNotAuthorized(ExternalEffectState),
    #[error("provider dispatch failed")]
    Provider(#[source] E),
}

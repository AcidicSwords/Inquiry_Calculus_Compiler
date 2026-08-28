//! Occurrence-linked actuality and finite Probe-port discharge views.
//!
//! The immutable `ActualEvent` and event ledger remain the only actuality authority. These
//! structures recheck source-occurrence linkage and port-indexed evidence after replay; they do
//! not create events, histories, opcodes, schedulers, or persistence tables.

use std::collections::{BTreeMap, BTreeSet};

use ic_core::{
    AskOccurrence, AskOccurrenceCheckError, AskOccurrenceError, AskOccurrenceRef,
    BindingVersionRef, DischargeMode, EventRef, OpenQueryCheckError, OpenQueryError,
    ProbeOperatorRef, ProvenanceRef, QueryRef, QuestionSuccessionCatalog, ResolutionCatalog,
    ResolutionPathCheckError, ResolutionPathError, ResolutionPathRef, RouteRef, SupportRef,
    TypeCheckError, TypeRef, TypeSymbol, TypedFormRef, WarrantRef,
};
use ic_store::ReplayedExternalEffect;
use thiserror::Error;

/// Catalog boundary for source occurrence, query, operator, and resolution-route rechecking.
pub trait ActualitySeparationCatalog:
    QuestionSuccessionCatalog + ResolutionCatalog + ic_core::ActualEventCatalog
{
}

impl<T> ActualitySeparationCatalog for T where
    T: QuestionSuccessionCatalog + ResolutionCatalog + ic_core::ActualEventCatalog
{
}

/// A cold-replayed ordinary event proven to be for one exact source `Ask` occurrence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceEventLink {
    actuality: ReplayedExternalEffect,
    occurrence: AskOccurrence,
}

impl SourceEventLink {
    #[must_use]
    pub const fn actuality(&self) -> &ReplayedExternalEffect {
        &self.actuality
    }

    #[must_use]
    pub const fn occurrence(&self) -> &AskOccurrence {
        &self.occurrence
    }

    #[must_use]
    pub const fn occurrence_ref(&self) -> AskOccurrenceRef {
        self.actuality
            .event()
            .source_ask_occurrence()
            .expect("a checked source-event link always has an occurrence")
    }

    #[must_use]
    pub const fn event_ref(&self) -> EventRef {
        self.actuality.event_ref()
    }
}

/// Rechecks the canonical v2 `EventFor(event, AskOccurrence)` proposition from a completed store
/// replay and a freshly reconstructed source occurrence.
pub fn check_source_event_link<C: ActualitySeparationCatalog>(
    actuality: ReplayedExternalEffect,
    occurrence: AskOccurrence,
    catalog: &C,
) -> Result<SourceEventLink, SourceEventLinkError> {
    occurrence
        .check(catalog)
        .map_err(|error| SourceEventLinkError::OccurrenceCheck(Box::new(error)))?;
    let occurrence_ref = occurrence.ask_occurrence_ref()?;
    let event = actuality.event();
    let linked = event
        .source_ask_occurrence()
        .ok_or(SourceEventLinkError::LegacyOrDirectEvent(
            actuality.event_ref(),
        ))?;
    if linked != occurrence_ref {
        return Err(SourceEventLinkError::OccurrenceMismatch {
            event: linked,
            supplied: occurrence_ref,
        });
    }
    if event.question() != occurrence.question() {
        return Err(SourceEventLinkError::QuestionMismatch {
            event: event.question(),
            occurrence: occurrence.question(),
        });
    }
    if event.binding() != occurrence.binding_version() {
        return Err(SourceEventLinkError::BindingMismatch {
            event: event.binding(),
            occurrence: occurrence.binding_version(),
        });
    }
    let event_compiler =
        event
            .compiler_version()
            .ok_or(SourceEventLinkError::MissingCompilerVersion(
                actuality.event_ref(),
            ))?;
    if event_compiler != occurrence.compiler_version() {
        return Err(SourceEventLinkError::CompilerVersionMismatch {
            event: event_compiler,
            occurrence: occurrence.compiler_version(),
        });
    }
    if event.provenance() != occurrence.provenance() {
        return Err(SourceEventLinkError::ProvenanceMismatch {
            event: event.provenance(),
            occurrence: occurrence.provenance(),
        });
    }
    let request = actuality.request();
    if request.query() != occurrence.question() {
        return Err(SourceEventLinkError::RequestQuestionMismatch {
            request: request.query(),
            occurrence: occurrence.question(),
        });
    }
    if request.operator() != event.operator() {
        return Err(SourceEventLinkError::RequestOperatorMismatch {
            request: request.operator(),
            event: event.operator(),
        });
    }
    if request.compiler_version() != occurrence.compiler_version() {
        return Err(SourceEventLinkError::RequestCompilerVersionMismatch {
            request: request.compiler_version(),
            occurrence: occurrence.compiler_version(),
        });
    }
    if request.backend_version() != event.backend_version() {
        return Err(SourceEventLinkError::RequestBackendVersionMismatch);
    }
    Ok(SourceEventLink {
        actuality,
        occurrence,
    })
}

/// One Probe-mode open-port evidence chain within a source occurrence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProbePortDischargeEvidence {
    port: TypeSymbol,
    route: RouteRef,
    resolution_path: ResolutionPathRef,
    binding: BindingVersionRef,
    compiler_version: ic_core::ArtifactRef,
    provenance: ProvenanceRef,
    event: SourceEventLink,
}

impl ProbePortDischargeEvidence {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        port: TypeSymbol,
        route: RouteRef,
        resolution_path: ResolutionPathRef,
        binding: BindingVersionRef,
        compiler_version: ic_core::ArtifactRef,
        provenance: ProvenanceRef,
        event: SourceEventLink,
    ) -> Self {
        Self {
            port,
            route,
            resolution_path,
            binding,
            compiler_version,
            provenance,
            event,
        }
    }

    #[must_use]
    pub const fn port(&self) -> &TypeSymbol {
        &self.port
    }

    #[must_use]
    pub const fn route(&self) -> RouteRef {
        self.route
    }

    #[must_use]
    pub const fn resolution_path(&self) -> ResolutionPathRef {
        self.resolution_path
    }

    #[must_use]
    pub const fn event(&self) -> &SourceEventLink {
        &self.event
    }
}

/// Explicit permission for multiple open Probe ports to reuse one already checked event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SharedProbeEventAdmission {
    event: EventRef,
    ports: Vec<TypeSymbol>,
}

impl SharedProbeEventAdmission {
    pub fn new(
        event: EventRef,
        mut ports: Vec<TypeSymbol>,
    ) -> Result<Self, ProbeDischargeBundleError> {
        ports.sort_unstable();
        if ports.len() < 2 {
            return Err(ProbeDischargeBundleError::SharedEventNeedsMultiplePorts(
                event,
            ));
        }
        if ports.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(ProbeDischargeBundleError::DuplicateSharedEventPort(event));
        }
        Ok(Self { event, ports })
    }

    #[must_use]
    pub const fn event(&self) -> EventRef {
        self.event
    }

    #[must_use]
    pub fn ports(&self) -> &[TypeSymbol] {
        &self.ports
    }
}

/// Exact finite all-Probe specialization of a port-indexed discharge bundle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteProbeDischargeBundle {
    occurrence: AskOccurrence,
    components: Vec<ProbePortDischargeEvidence>,
    shared_events: Vec<SharedProbeEventAdmission>,
}

/// Typed evidence for one non-`Probe` source port.  It intentionally has no event or raw return.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonProbePortDischargeEvidence {
    port: TypeSymbol,
    mode: DischargeMode,
    value: TypedFormRef,
    route: RouteRef,
    binding: BindingVersionRef,
    compiler_version: ic_core::ArtifactRef,
    provenance: ProvenanceRef,
    support: SupportRef,
    warrant: Option<WarrantRef>,
}

impl NonProbePortDischargeEvidence {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        port: TypeSymbol,
        mode: DischargeMode,
        value: TypedFormRef,
        route: RouteRef,
        binding: BindingVersionRef,
        compiler_version: ic_core::ArtifactRef,
        provenance: ProvenanceRef,
        support: SupportRef,
        warrant: Option<WarrantRef>,
    ) -> Self {
        Self {
            port,
            mode,
            value,
            route,
            binding,
            compiler_version,
            provenance,
            support,
            warrant,
        }
    }
    #[must_use]
    pub const fn port(&self) -> &TypeSymbol {
        &self.port
    }
    #[must_use]
    pub const fn mode(&self) -> DischargeMode {
        self.mode
    }
    #[must_use]
    pub const fn value(&self) -> TypedFormRef {
        self.value
    }
    #[must_use]
    pub const fn route(&self) -> RouteRef {
        self.route
    }
}

/// One explicitly tagged member of a finite mixed-mode source discharge field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MixedPortDischargeEvidence {
    Probe(Box<ProbePortDischargeEvidence>),
    NonProbe(Box<NonProbePortDischargeEvidence>),
}

impl MixedPortDischargeEvidence {
    #[must_use]
    pub const fn port(&self) -> &TypeSymbol {
        match self {
            Self::Probe(evidence) => evidence.port(),
            Self::NonProbe(evidence) => evidence.port(),
        }
    }
    #[must_use]
    pub const fn mode(&self) -> DischargeMode {
        match self {
            Self::Probe(_) => DischargeMode::Probe,
            Self::NonProbe(evidence) => evidence.mode(),
        }
    }
}

/// A finite occurrence-indexed mixed field with exactly one actualized Probe port.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteMixedDischargeBundle {
    occurrence: AskOccurrence,
    components: Vec<MixedPortDischargeEvidence>,
}

impl FiniteMixedDischargeBundle {
    #[must_use]
    pub const fn occurrence(&self) -> &AskOccurrence {
        &self.occurrence
    }
    #[must_use]
    pub fn components(&self) -> &[MixedPortDischargeEvidence] {
        &self.components
    }
}

impl FiniteProbeDischargeBundle {
    #[must_use]
    pub const fn occurrence(&self) -> &AskOccurrence {
        &self.occurrence
    }

    #[must_use]
    pub fn components(&self) -> &[ProbePortDischargeEvidence] {
        &self.components
    }

    #[must_use]
    pub fn shared_events(&self) -> &[SharedProbeEventAdmission] {
        &self.shared_events
    }
}

/// Admits one finite bundle only when every open port is Probe-mode and covered exactly once,
/// every event is linked to the same occurrence, and every shared event has an exact explicit
/// multi-port admission.
pub fn admit_finite_probe_discharge_bundle<C: ActualitySeparationCatalog>(
    occurrence: AskOccurrence,
    mut components: Vec<ProbePortDischargeEvidence>,
    mut shared_events: Vec<SharedProbeEventAdmission>,
    catalog: &C,
) -> Result<FiniteProbeDischargeBundle, ProbeDischargeBundleError> {
    occurrence
        .check(catalog)
        .map_err(|error| ProbeDischargeBundleError::OccurrenceCheck(Box::new(error)))?;
    let occurrence_ref = occurrence.ask_occurrence_ref()?;
    let query = ic_core::OpenQueryCatalog::resolve_open_query(catalog, occurrence.question())
        .ok_or(ProbeDischargeBundleError::UnresolvedQuery(
            occurrence.question(),
        ))?;
    let calculated = query.query_ref()?;
    if calculated != occurrence.question() {
        return Err(ProbeDischargeBundleError::QueryIdentityMismatch {
            expected: occurrence.question(),
            actual: calculated,
        });
    }
    query.check(catalog)?;
    let schema = ic_core::RelationCatalog::resolve_relation_schema(catalog, query.relation())
        .ok_or(ProbeDischargeBundleError::UnresolvedRelation(
            query.relation(),
        ))?;

    let mut expected_ports = BTreeMap::new();
    for open in query.open_ports() {
        if open.mode() != DischargeMode::Probe {
            return Err(
                ProbeDischargeBundleError::NonProbePortOutsideFiniteSpecialization(
                    open.port().clone(),
                ),
            );
        }
        let ty = schema
            .ports()
            .iter()
            .find(|port| port.name() == open.port())
            .map(ic_core::RelationPort::ty)
            .ok_or_else(|| ProbeDischargeBundleError::PortMissingFromSchema(open.port().clone()))?;
        expected_ports.insert(open.port().clone(), ty);
    }
    if components.is_empty() {
        return Err(ProbeDischargeBundleError::EmptyBundle);
    }
    components.sort_by(|left, right| left.port.cmp(&right.port));
    if components
        .windows(2)
        .any(|pair| pair[0].port == pair[1].port)
    {
        return Err(ProbeDischargeBundleError::DuplicateComponentPort);
    }
    let actual_ports: BTreeSet<_> = components.iter().map(|item| item.port.clone()).collect();
    let declared_ports: BTreeSet<_> = expected_ports.keys().cloned().collect();
    if actual_ports != declared_ports {
        return Err(ProbeDischargeBundleError::PortCoverageMismatch);
    }

    let mut event_ports: BTreeMap<EventRef, Vec<TypeSymbol>> = BTreeMap::new();
    for component in &components {
        if component.event.occurrence_ref() != occurrence_ref
            || component.event.occurrence() != &occurrence
        {
            return Err(ProbeDischargeBundleError::ComponentOccurrenceMismatch(
                component.port.clone(),
            ));
        }
        let event = component.event.actuality().event();
        if component.route != event.route() {
            return Err(ProbeDischargeBundleError::RouteMismatch(
                component.port.clone(),
            ));
        }
        if component.binding != event.binding() || component.binding != occurrence.binding_version()
        {
            return Err(ProbeDischargeBundleError::BindingMismatch(
                component.port.clone(),
            ));
        }
        if event.compiler_version() != Some(component.compiler_version)
            || component.compiler_version != occurrence.compiler_version()
        {
            return Err(ProbeDischargeBundleError::CompilerVersionMismatch(
                component.port.clone(),
            ));
        }
        if component.provenance != event.provenance()
            || component.provenance != occurrence.provenance()
        {
            return Err(ProbeDischargeBundleError::ProvenanceMismatch(
                component.port.clone(),
            ));
        }
        let path = catalog
            .resolve_resolution_path(component.resolution_path)
            .ok_or(ProbeDischargeBundleError::UnresolvedResolutionPath(
                component.resolution_path,
            ))?;
        let calculated = path.resolution_path_ref()?;
        if calculated != component.resolution_path {
            return Err(ProbeDischargeBundleError::ResolutionPathIdentityMismatch {
                expected: component.resolution_path,
                actual: calculated,
            });
        }
        path.check(catalog)
            .map_err(|error| ProbeDischargeBundleError::ResolutionPathCheck(Box::new(error)))?;
        let operator =
            ic_core::ActualEventCatalog::resolve_probe_operator(catalog, event.operator()).ok_or(
                ProbeDischargeBundleError::UnresolvedOperator(event.operator()),
            )?;
        if path.input() != operator.return_type()
            || path.output() != expected_ports[&component.port]
        {
            return Err(ProbeDischargeBundleError::ResolutionTypeMismatch(Box::new(
                ResolutionTypeMismatch {
                    port: component.port.clone(),
                    expected_input: operator.return_type(),
                    actual_input: path.input(),
                    expected_output: expected_ports[&component.port],
                    actual_output: path.output(),
                },
            )));
        }
        event_ports
            .entry(component.event.event_ref())
            .or_default()
            .push(component.port.clone());
    }
    for ports in event_ports.values_mut() {
        ports.sort_unstable();
    }

    shared_events.sort_by_key(SharedProbeEventAdmission::event);
    if shared_events
        .windows(2)
        .any(|pair| pair[0].event == pair[1].event)
    {
        return Err(ProbeDischargeBundleError::DuplicateSharedEventAdmission);
    }
    let required_shared: BTreeMap<_, _> = event_ports
        .iter()
        .filter(|(_, ports)| ports.len() > 1)
        .map(|(event, ports)| (*event, ports.clone()))
        .collect();
    let supplied_shared: BTreeMap<_, _> = shared_events
        .iter()
        .map(|admission| (admission.event, admission.ports.clone()))
        .collect();
    if supplied_shared != required_shared {
        return Err(ProbeDischargeBundleError::SharedEventCoverageMismatch);
    }

    Ok(FiniteProbeDischargeBundle {
        occurrence,
        components,
        shared_events,
    })
}

/// Admits one finite mixed-mode field with one actualized Probe port and typed non-Probe ports.
/// Non-Probe evidence never contains or creates an event.
pub fn admit_finite_mixed_discharge_bundle<C: ActualitySeparationCatalog>(
    occurrence: AskOccurrence,
    mut components: Vec<MixedPortDischargeEvidence>,
    catalog: &C,
) -> Result<FiniteMixedDischargeBundle, MixedDischargeBundleError> {
    occurrence.check(catalog)?;
    let query = ic_core::OpenQueryCatalog::resolve_open_query(catalog, occurrence.question())
        .ok_or(MixedDischargeBundleError::UnresolvedQuery(
            occurrence.question(),
        ))?;
    if query.query_ref()? != occurrence.question() {
        return Err(MixedDischargeBundleError::QueryIdentityMismatch);
    }
    query.check(catalog)?;
    let schema = ic_core::RelationCatalog::resolve_relation_schema(catalog, query.relation())
        .ok_or(MixedDischargeBundleError::UnresolvedRelation(
            query.relation(),
        ))?;
    let expected = query
        .open_ports()
        .iter()
        .map(|open| {
            let ty = schema
                .ports()
                .iter()
                .find(|schema_port| schema_port.name() == open.port())
                .map(ic_core::RelationPort::ty)
                .ok_or_else(|| {
                    MixedDischargeBundleError::PortMissingFromSchema(open.port().clone())
                })?;
            Ok((open.port().clone(), (open.mode(), ty)))
        })
        .collect::<Result<BTreeMap<_, _>, MixedDischargeBundleError>>()?;
    if components.is_empty() {
        return Err(MixedDischargeBundleError::EmptyBundle);
    }
    components.sort_by(|left, right| left.port().cmp(right.port()));
    if components
        .windows(2)
        .any(|pair| pair[0].port() == pair[1].port())
    {
        return Err(MixedDischargeBundleError::DuplicateComponentPort);
    }
    let actual_ports = components
        .iter()
        .map(MixedPortDischargeEvidence::port)
        .collect::<BTreeSet<_>>();
    let expected_ports = expected.keys().collect::<BTreeSet<_>>();
    if actual_ports != expected_ports {
        return Err(MixedDischargeBundleError::PortCoverageMismatch);
    }
    let mut probe_count = 0usize;
    for component in &components {
        let (expected_mode, expected_type) = expected
            .get(component.port())
            .expect("component coverage was checked above");
        if component.mode() != *expected_mode {
            return Err(MixedDischargeBundleError::ModeMismatch(
                component.port().clone(),
            ));
        }
        match component {
            MixedPortDischargeEvidence::Probe(evidence) => {
                probe_count += 1;
                let event = evidence.event().actuality().event();
                if evidence.event().occurrence() != &occurrence
                    || event.route() != evidence.route
                    || event.binding() != occurrence.binding_version()
                    || event.compiler_version() != Some(occurrence.compiler_version())
                    || event.provenance() != occurrence.provenance()
                {
                    return Err(MixedDischargeBundleError::ProbeEvidenceMismatch(
                        evidence.port().clone(),
                    ));
                }
                let path = catalog
                    .resolve_resolution_path(evidence.resolution_path)
                    .ok_or(MixedDischargeBundleError::UnresolvedResolutionPath(
                        evidence.resolution_path,
                    ))?;
                if path.resolution_path_ref()? != evidence.resolution_path {
                    return Err(MixedDischargeBundleError::ResolutionPathIdentityMismatch(
                        evidence.port().clone(),
                    ));
                }
                path.check(catalog)?;
                let operator =
                    ic_core::ActualEventCatalog::resolve_probe_operator(catalog, event.operator())
                        .ok_or(MixedDischargeBundleError::UnresolvedOperator(
                            event.operator(),
                        ))?;
                if path.input() != operator.return_type() || path.output() != *expected_type {
                    return Err(MixedDischargeBundleError::ProbeResolutionTypeMismatch(
                        evidence.port().clone(),
                    ));
                }
            }
            MixedPortDischargeEvidence::NonProbe(evidence) => {
                if evidence.mode() == DischargeMode::Probe {
                    return Err(MixedDischargeBundleError::NonProbeCarriesProbeMode(
                        evidence.port().clone(),
                    ));
                }
                let value = ic_core::FormulaCatalog::resolve_typed_form(catalog, evidence.value())
                    .ok_or(MixedDischargeBundleError::UnresolvedTypedValue(
                        evidence.value(),
                    ))?;
                if value.typed_form_ref()? != evidence.value() {
                    return Err(MixedDischargeBundleError::TypedValueIdentityMismatch);
                }
                value.check(catalog)?;
                if value.ty() != *expected_type || value.binding() != occurrence.binding_version() {
                    return Err(MixedDischargeBundleError::NonProbeValueTypeMismatch(
                        evidence.port().clone(),
                    ));
                }
                if evidence.binding != occurrence.binding_version()
                    || evidence.compiler_version != occurrence.compiler_version()
                    || evidence.provenance != occurrence.provenance()
                    || evidence.support != query.context().support()
                    || evidence.warrant != query.context().warrant()
                {
                    return Err(MixedDischargeBundleError::NonProbeAuthorityMismatch(
                        evidence.port().clone(),
                    ));
                }
            }
        }
    }
    if probe_count != 1 {
        return Err(MixedDischargeBundleError::ExpectedExactlyOneProbe(
            probe_count,
        ));
    }
    Ok(FiniteMixedDischargeBundle {
        occurrence,
        components,
    })
}

#[derive(Debug, Error)]
pub enum SourceEventLinkError {
    #[error(transparent)]
    OccurrenceEncoding(#[from] AskOccurrenceError),
    #[error("source Ask occurrence failed recheck: {0}")]
    OccurrenceCheck(Box<AskOccurrenceCheckError>),
    #[error("event {0} is legacy/direct and has no source Ask occurrence link")]
    LegacyOrDirectEvent(EventRef),
    #[error("event links occurrence {event}, but supplied occurrence is {supplied}")]
    OccurrenceMismatch {
        event: AskOccurrenceRef,
        supplied: AskOccurrenceRef,
    },
    #[error("event question {event} differs from source occurrence question {occurrence}")]
    QuestionMismatch {
        event: QueryRef,
        occurrence: QueryRef,
    },
    #[error("event binding {event} differs from source occurrence binding {occurrence}")]
    BindingMismatch {
        event: BindingVersionRef,
        occurrence: BindingVersionRef,
    },
    #[error("event {0} has no explicit compiler version")]
    MissingCompilerVersion(EventRef),
    #[error("event compiler version {event} differs from source occurrence version {occurrence}")]
    CompilerVersionMismatch {
        event: ic_core::ArtifactRef,
        occurrence: ic_core::ArtifactRef,
    },
    #[error("event provenance {event} differs from source occurrence provenance {occurrence}")]
    ProvenanceMismatch {
        event: ProvenanceRef,
        occurrence: ProvenanceRef,
    },
    #[error("request question {request} differs from source occurrence question {occurrence}")]
    RequestQuestionMismatch {
        request: QueryRef,
        occurrence: QueryRef,
    },
    #[error("request operator {request} differs from event operator {event}")]
    RequestOperatorMismatch {
        request: ProbeOperatorRef,
        event: ProbeOperatorRef,
    },
    #[error(
        "request compiler version {request} differs from source occurrence version {occurrence}"
    )]
    RequestCompilerVersionMismatch {
        request: ic_core::ArtifactRef,
        occurrence: ic_core::ArtifactRef,
    },
    #[error("request backend version differs from event backend version")]
    RequestBackendVersionMismatch,
}

#[derive(Debug, Error)]
pub enum ProbeDischargeBundleError {
    #[error(transparent)]
    OccurrenceEncoding(#[from] AskOccurrenceError),
    #[error("source Ask occurrence failed recheck: {0}")]
    OccurrenceCheck(Box<AskOccurrenceCheckError>),
    #[error(transparent)]
    QueryEncoding(#[from] OpenQueryError),
    #[error(transparent)]
    QueryCheck(#[from] OpenQueryCheckError),
    #[error(transparent)]
    ResolutionPathEncoding(#[from] ResolutionPathError),
    #[error("resolution path failed recheck: {0}")]
    ResolutionPathCheck(Box<ResolutionPathCheckError>),
    #[error("question {0} is unavailable")]
    UnresolvedQuery(QueryRef),
    #[error("question identity is {actual}, expected {expected}")]
    QueryIdentityMismatch {
        expected: QueryRef,
        actual: QueryRef,
    },
    #[error("relation {0} is unavailable")]
    UnresolvedRelation(ic_core::RelationRef),
    #[error("open port {0} is absent from its relation schema")]
    PortMissingFromSchema(TypeSymbol),
    #[error("non-Probe port {0} is outside this finite all-Probe specialization")]
    NonProbePortOutsideFiniteSpecialization(TypeSymbol),
    #[error("a finite Probe discharge bundle must be nonempty")]
    EmptyBundle,
    #[error("a Probe discharge bundle contains the same port more than once")]
    DuplicateComponentPort,
    #[error("Probe discharge components do not cover exactly the open port set")]
    PortCoverageMismatch,
    #[error("Probe component {0} belongs to a different source occurrence")]
    ComponentOccurrenceMismatch(TypeSymbol),
    #[error("Probe component {0} route differs from its event route")]
    RouteMismatch(TypeSymbol),
    #[error("Probe component {0} binding differs from its event or occurrence")]
    BindingMismatch(TypeSymbol),
    #[error("Probe component {0} compiler version differs from its event or occurrence")]
    CompilerVersionMismatch(TypeSymbol),
    #[error("Probe component {0} provenance differs from its event or occurrence")]
    ProvenanceMismatch(TypeSymbol),
    #[error("resolution path {0} is unavailable")]
    UnresolvedResolutionPath(ResolutionPathRef),
    #[error("resolution path identity is {actual}, expected {expected}")]
    ResolutionPathIdentityMismatch {
        expected: ResolutionPathRef,
        actual: ResolutionPathRef,
    },
    #[error("operator {0} is unavailable")]
    UnresolvedOperator(ProbeOperatorRef),
    #[error(transparent)]
    ResolutionTypeMismatch(Box<ResolutionTypeMismatch>),
    #[error("shared event {0} admission must name at least two distinct ports")]
    SharedEventNeedsMultiplePorts(EventRef),
    #[error("shared event {0} admission repeats a port")]
    DuplicateSharedEventPort(EventRef),
    #[error("the same event has more than one shared-event admission")]
    DuplicateSharedEventAdmission,
    #[error("shared-event admissions do not exactly match the multiply used event/port groups")]
    SharedEventCoverageMismatch,
}

#[derive(Debug, Error)]
pub enum MixedDischargeBundleError {
    #[error(transparent)]
    Occurrence(#[from] AskOccurrenceCheckError),
    #[error(transparent)]
    OccurrenceEncoding(#[from] AskOccurrenceError),
    #[error(transparent)]
    Query(#[from] OpenQueryCheckError),
    #[error(transparent)]
    QueryEncoding(#[from] OpenQueryError),
    #[error(transparent)]
    ResolutionPath(#[from] ResolutionPathCheckError),
    #[error(transparent)]
    ResolutionPathEncoding(#[from] ResolutionPathError),
    #[error(transparent)]
    TypedValue(#[from] TypeCheckError),
    #[error(transparent)]
    TypedValueEncoding(#[from] ic_core::TypeError),
    #[error("mixed discharge source query {0} is unavailable")]
    UnresolvedQuery(QueryRef),
    #[error("mixed discharge source query differs from its claimed identity")]
    QueryIdentityMismatch,
    #[error("mixed discharge relation {0} is unavailable")]
    UnresolvedRelation(ic_core::RelationRef),
    #[error("mixed discharge open port {0} is absent from its relation schema")]
    PortMissingFromSchema(TypeSymbol),
    #[error("a finite mixed discharge bundle must be nonempty")]
    EmptyBundle,
    #[error("a finite mixed discharge bundle repeats a source port")]
    DuplicateComponentPort,
    #[error("mixed discharge components do not cover exactly the source open-port field")]
    PortCoverageMismatch,
    #[error("mixed discharge component mode differs from source mode at port {0}")]
    ModeMismatch(TypeSymbol),
    #[error("Probe evidence at port {0} disagrees with the source-linked event")]
    ProbeEvidenceMismatch(TypeSymbol),
    #[error("resolution path {0} is unavailable")]
    UnresolvedResolutionPath(ResolutionPathRef),
    #[error("resolution path identity differs at port {0}")]
    ResolutionPathIdentityMismatch(TypeSymbol),
    #[error("probe operator {0} is unavailable")]
    UnresolvedOperator(ProbeOperatorRef),
    #[error("Probe resolution type differs from the source port at {0}")]
    ProbeResolutionTypeMismatch(TypeSymbol),
    #[error("non-Probe evidence carries Probe mode at port {0}")]
    NonProbeCarriesProbeMode(TypeSymbol),
    #[error("typed non-Probe value {0} is unavailable")]
    UnresolvedTypedValue(TypedFormRef),
    #[error("typed non-Probe value differs from its claimed identity")]
    TypedValueIdentityMismatch,
    #[error("typed non-Probe value disagrees with source port type or binding at {0}")]
    NonProbeValueTypeMismatch(TypeSymbol),
    #[error("non-Probe authority, version, or provenance differs from source context at {0}")]
    NonProbeAuthorityMismatch(TypeSymbol),
    #[error("finite mixed discharge requires exactly one Probe port, found {0}")]
    ExpectedExactlyOneProbe(usize),
}

#[derive(Debug, Error)]
#[error(
    "Probe component {port} resolution path has {actual_input}->{actual_output}, expected {expected_input}->{expected_output}"
)]
pub struct ResolutionTypeMismatch {
    port: TypeSymbol,
    expected_input: TypeRef,
    actual_input: TypeRef,
    expected_output: TypeRef,
    actual_output: TypeRef,
}

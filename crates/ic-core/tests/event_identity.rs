use std::collections::BTreeMap;

use ic_core::{
    ActualEvent, ActualEventCatalog, ActualEventCheckError, ActualEventError, ApplicabilityRef,
    ArtifactRef, BindingVersionRef, BoundaryChart, BoundaryRef, DeterminationPresentationRef,
    DischargeMode, DistinctionRef, EventRef, FormulaRef, GrainRef, HorizonRef, OpenQuery,
    OperatorRef, ProbeContractRef, ProbeOperator, ProbeOperatorRef, ProvenanceRef, QueryRef,
    RawReturn, RawReturnCatalog, RawReturnRef, RelationRef, RelationUseContext, RelationUseRef,
    RouteRef, ScopeRef, StateRef, SupportRef, TypeRef, check_actual_event, check_raw_return,
};

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn event(raw_return: RawReturnRef, distinction: Option<DistinctionRef>) -> ActualEvent {
    event_at_boundary(
        raw_return,
        distinction,
        BoundaryRef::from_artifact_ref(artifact(4)),
    )
}

fn event_at_boundary(
    raw_return: RawReturnRef,
    distinction: Option<DistinctionRef>,
    boundary: BoundaryRef,
) -> ActualEvent {
    ActualEvent::new(
        Some(EventRef::from_artifact_ref(artifact(1))),
        StateRef::from_artifact_ref(artifact(2)),
        QueryRef::from_artifact_ref(artifact(3)),
        boundary,
        distinction,
        OperatorRef::from_artifact_ref(artifact(5)),
        raw_return,
        StateRef::from_artifact_ref(artifact(6)),
        GrainRef::from_artifact_ref(artifact(7)),
        RouteRef::from_artifact_ref(artifact(8)),
        BindingVersionRef::from_artifact_ref(artifact(9)),
        artifact(10),
        ProvenanceRef::from_artifact_ref(artifact(11)),
    )
}

#[test]
fn actual_event_requires_boundary_and_keeps_optional_distinction_separate() {
    let raw_return = RawReturnRef::from_artifact_ref(artifact(12));
    let without_distinction = event(raw_return, None);
    let with_distinction = event(
        raw_return,
        Some(DistinctionRef::from_artifact_ref(artifact(13))),
    );

    assert_ne!(
        without_distinction.event_ref().expect("event must encode"),
        with_distinction.event_ref().expect("event must encode"),
        "the plan's optional distinction cannot replace or disappear into the canonical boundary"
    );
    let envelope = with_distinction.envelope().expect("event must encode");
    assert_eq!(
        ActualEvent::from_envelope(&envelope).expect("event must decode"),
        with_distinction
    );
    assert!(matches!(
        ActualEvent::decode_payload(&[2]),
        Err(ActualEventError::UnknownOptionalTag(2))
    ));
}

struct RawReturns(BTreeMap<RawReturnRef, RawReturn>);

impl RawReturnCatalog for RawReturns {
    fn resolve_raw_return(&self, reference: RawReturnRef) -> Option<RawReturn> {
        self.0.get(&reference).cloned()
    }
}

#[derive(Clone)]
struct EventCatalog {
    raw_returns: BTreeMap<RawReturnRef, RawReturn>,
    boundary_charts: BTreeMap<BoundaryRef, BoundaryChart>,
    probe_operators: BTreeMap<ProbeOperatorRef, ProbeOperator>,
    open_queries: BTreeMap<QueryRef, OpenQuery>,
}

impl RawReturnCatalog for EventCatalog {
    fn resolve_raw_return(&self, reference: RawReturnRef) -> Option<RawReturn> {
        self.raw_returns.get(&reference).cloned()
    }
}

impl ActualEventCatalog for EventCatalog {
    fn resolve_boundary_chart(&self, reference: BoundaryRef) -> Option<BoundaryChart> {
        self.boundary_charts.get(&reference).cloned()
    }

    fn resolve_probe_operator(&self, reference: ProbeOperatorRef) -> Option<ProbeOperator> {
        self.probe_operators.get(&reference).cloned()
    }

    fn resolve_open_query(&self, reference: QueryRef) -> Option<OpenQuery> {
        self.open_queries.get(&reference).cloned()
    }
}

fn open_query(byte: u8) -> OpenQuery {
    OpenQuery::new(
        RelationRef::from_artifact_ref(artifact(byte)),
        Vec::new(),
        Vec::new(),
        RelationUseContext::new(
            ScopeRef::from_artifact_ref(artifact(byte.wrapping_add(1))),
            ApplicabilityRef::from_artifact_ref(artifact(byte.wrapping_add(2))),
            GrainRef::from_artifact_ref(artifact(byte.wrapping_add(3))),
            HorizonRef::from_artifact_ref(artifact(byte.wrapping_add(4))),
            DischargeMode::Probe,
            SupportRef::from_artifact_ref(artifact(byte.wrapping_add(5))),
            None,
        ),
    )
}

fn boundary_chart(query: QueryRef, grain: GrainRef) -> BoundaryChart {
    BoundaryChart::new(
        query,
        TypeRef::from_artifact_ref(artifact(31)),
        TypeRef::from_artifact_ref(artifact(32)),
        TypeRef::from_artifact_ref(artifact(33)),
        RelationRef::from_artifact_ref(artifact(34)),
        RelationRef::from_artifact_ref(artifact(35)),
        DeterminationPresentationRef::from_artifact_ref(artifact(36)),
        None,
        vec![],
        vec![],
        RelationUseRef::from_artifact_ref(artifact(37)),
        FormulaRef::from_artifact_ref(artifact(38)),
        None,
        grain,
        HorizonRef::from_artifact_ref(artifact(40)),
    )
}

fn probe_operator(query: QueryRef, boundary: BoundaryRef) -> ProbeOperator {
    ProbeOperator::new(
        query,
        boundary,
        artifact(43),
        artifact(44),
        artifact(45),
        TypeRef::from_artifact_ref(artifact(46)),
        artifact(47),
        ProbeContractRef::from_artifact_ref(artifact(48)),
        artifact(49),
    )
}

#[test]
fn actual_event_rechecks_the_opaque_raw_return_without_decoding_or_interpreting_it() {
    let raw = RawReturn::new(vec![0, 0xff, b'{', 0]);
    let raw_ref = raw.raw_return_ref().expect("raw return must encode");
    let mut returns = BTreeMap::new();
    returns.insert(raw_ref, raw);
    let catalog = RawReturns(returns);
    assert!(check_raw_return(&event(raw_ref, None), &catalog).is_ok());

    let missing = RawReturnRef::from_artifact_ref(artifact(20));
    assert!(matches!(
        check_raw_return(&event(missing, None), &catalog),
        Err(ActualEventCheckError::UnresolvedRawReturn(reference)) if reference == missing
    ));
}

#[test]
fn actual_event_requires_a_rehashed_boundary_chart_without_checking_its_open_roles() {
    let raw = RawReturn::new(vec![0, 0xff]);
    let raw_ref = raw.raw_return_ref().expect("raw return must encode");
    let question = open_query(30);
    let question_ref = question.query_ref().expect("question must encode");
    let grain = GrainRef::from_artifact_ref(artifact(39));
    let chart = boundary_chart(question_ref, grain);
    let boundary_ref = chart.boundary_ref().expect("chart must encode");
    let operator = probe_operator(question_ref, boundary_ref);
    let operator_ref = operator.probe_operator_ref().expect("operator must encode");
    let catalog = EventCatalog {
        raw_returns: BTreeMap::from([(raw_ref, raw)]),
        boundary_charts: BTreeMap::from([(boundary_ref, chart)]),
        probe_operators: BTreeMap::from([(operator_ref, operator)]),
        open_queries: BTreeMap::from([(question_ref, question)]),
    };
    let checked_event = ActualEvent::new(
        Some(EventRef::from_artifact_ref(artifact(1))),
        StateRef::from_artifact_ref(artifact(2)),
        question_ref,
        boundary_ref,
        None,
        operator_ref,
        raw_ref,
        StateRef::from_artifact_ref(artifact(6)),
        grain,
        RouteRef::from_artifact_ref(artifact(8)),
        BindingVersionRef::from_artifact_ref(artifact(9)),
        artifact(10),
        ProvenanceRef::from_artifact_ref(artifact(11)),
    );
    assert!(check_actual_event(&checked_event, &catalog).is_ok());

    let other_question = open_query(50);
    let other_question_ref = other_question.query_ref().expect("question must encode");
    let mut mismatched_catalog = catalog.clone();
    mismatched_catalog
        .open_queries
        .insert(other_question_ref, other_question);
    let mismatched_question = ActualEvent::new(
        checked_event.ledger_parent(),
        checked_event.state_before(),
        other_question_ref,
        checked_event.boundary(),
        checked_event.distinction(),
        checked_event.operator(),
        checked_event.raw_return(),
        checked_event.state_after(),
        checked_event.grain(),
        checked_event.route(),
        checked_event.binding(),
        checked_event.backend_version(),
        checked_event.provenance(),
    );
    assert!(matches!(
        check_actual_event(&mismatched_question, &mismatched_catalog),
        Err(ActualEventCheckError::BoundaryQuestionMismatch { event, boundary })
            if event == other_question_ref && boundary == question_ref
    ));

    let operator_with_other_question = probe_operator(other_question_ref, boundary_ref);
    let operator_with_other_question_ref = operator_with_other_question
        .probe_operator_ref()
        .expect("operator must encode");
    let mut operator_question_catalog = catalog.clone();
    operator_question_catalog.probe_operators.insert(
        operator_with_other_question_ref,
        operator_with_other_question,
    );
    let mismatched_operator_question = ActualEvent::new(
        checked_event.ledger_parent(),
        checked_event.state_before(),
        checked_event.question(),
        checked_event.boundary(),
        checked_event.distinction(),
        operator_with_other_question_ref,
        checked_event.raw_return(),
        checked_event.state_after(),
        checked_event.grain(),
        checked_event.route(),
        checked_event.binding(),
        checked_event.backend_version(),
        checked_event.provenance(),
    );
    assert!(matches!(
        check_actual_event(&mismatched_operator_question, &operator_question_catalog),
        Err(ActualEventCheckError::OperatorQuestionMismatch { event, operator })
            if event == question_ref && operator == other_question_ref
    ));

    let operator_with_other_boundary =
        probe_operator(question_ref, BoundaryRef::from_artifact_ref(artifact(51)));
    let operator_with_other_boundary_ref = operator_with_other_boundary
        .probe_operator_ref()
        .expect("operator must encode");
    let mut operator_boundary_catalog = catalog.clone();
    operator_boundary_catalog.probe_operators.insert(
        operator_with_other_boundary_ref,
        operator_with_other_boundary,
    );
    let mismatched_operator_boundary = ActualEvent::new(
        checked_event.ledger_parent(),
        checked_event.state_before(),
        checked_event.question(),
        checked_event.boundary(),
        checked_event.distinction(),
        operator_with_other_boundary_ref,
        checked_event.raw_return(),
        checked_event.state_after(),
        checked_event.grain(),
        checked_event.route(),
        checked_event.binding(),
        checked_event.backend_version(),
        checked_event.provenance(),
    );
    assert!(matches!(
        check_actual_event(&mismatched_operator_boundary, &operator_boundary_catalog),
        Err(ActualEventCheckError::OperatorBoundaryMismatch { event, operator })
            if event == boundary_ref
                && operator == BoundaryRef::from_artifact_ref(artifact(51))
    ));

    let mismatched_grain = ActualEvent::new(
        checked_event.ledger_parent(),
        checked_event.state_before(),
        checked_event.question(),
        checked_event.boundary(),
        checked_event.distinction(),
        checked_event.operator(),
        checked_event.raw_return(),
        checked_event.state_after(),
        GrainRef::from_artifact_ref(artifact(52)),
        checked_event.route(),
        checked_event.binding(),
        checked_event.backend_version(),
        checked_event.provenance(),
    );
    assert!(matches!(
        check_actual_event(&mismatched_grain, &catalog),
        Err(ActualEventCheckError::BoundaryGrainMismatch { event, boundary })
            if event == GrainRef::from_artifact_ref(artifact(52)) && boundary == grain
    ));

    let missing = BoundaryRef::from_artifact_ref(artifact(41));
    assert!(matches!(
        check_actual_event(&event_at_boundary(raw_ref, None, missing), &catalog),
        Err(ActualEventCheckError::UnresolvedBoundary(reference)) if reference == missing
    ));
}

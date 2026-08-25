use std::collections::BTreeMap;

use ic_core::{
    ActualEvent, ActualEventCheckError, ActualEventError, ArtifactRef, BindingVersionRef,
    BoundaryRef, DistinctionRef, EventRef, GrainRef, OperatorRef, ProvenanceRef, QueryRef,
    RawReturn, RawReturnCatalog, RawReturnRef, RouteRef, StateRef, check_raw_return,
};

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn event(raw_return: RawReturnRef, distinction: Option<DistinctionRef>) -> ActualEvent {
    ActualEvent::new(
        Some(EventRef::from_artifact_ref(artifact(1))),
        StateRef::from_artifact_ref(artifact(2)),
        QueryRef::from_artifact_ref(artifact(3)),
        BoundaryRef::from_artifact_ref(artifact(4)),
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

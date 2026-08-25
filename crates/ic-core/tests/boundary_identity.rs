use ic_core::{
    ArtifactRef, BoundaryChart, BoundaryChartError, DeterminationPresentationRef, FormulaRef,
    GrainRef, HorizonRef, NegationUseRef, QueryRef, RelationRef, RelationUseRef, TypeRef,
};

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn chart(frontier_x: Vec<NegationUseRef>) -> BoundaryChart {
    BoundaryChart::new(
        QueryRef::from_artifact_ref(artifact(1)),
        TypeRef::from_artifact_ref(artifact(2)),
        TypeRef::from_artifact_ref(artifact(3)),
        TypeRef::from_artifact_ref(artifact(4)),
        RelationRef::from_artifact_ref(artifact(5)),
        RelationRef::from_artifact_ref(artifact(6)),
        DeterminationPresentationRef::from_artifact_ref(artifact(7)),
        None,
        frontier_x,
        vec![NegationUseRef::from_artifact_ref(artifact(8))],
        RelationUseRef::from_artifact_ref(artifact(9)),
        FormulaRef::from_artifact_ref(artifact(10)),
        None,
        GrainRef::from_artifact_ref(artifact(11)),
        HorizonRef::from_artifact_ref(artifact(12)),
    )
}

#[test]
fn boundary_chart_keeps_tagged_frontiers_and_absent_roles_explicit() {
    let first = chart(vec![NegationUseRef::from_artifact_ref(artifact(13))]);
    let second = chart(vec![
        NegationUseRef::from_artifact_ref(artifact(13)),
        NegationUseRef::from_artifact_ref(artifact(14)),
    ]);
    assert_ne!(
        first.boundary_ref().expect("chart must encode"),
        second.boundary_ref().expect("chart must encode"),
        "adding a use may not collapse an existing tagged frontier"
    );
    let envelope = first.envelope().expect("chart must encode");
    assert_eq!(
        BoundaryChart::from_envelope(&envelope).expect("chart must decode"),
        first
    );
    assert!(first.y_determination().is_none());
    assert!(first.traversal().is_none());
}

#[test]
fn boundary_chart_rejects_malformed_optional_and_count_fields() {
    assert!(matches!(
        BoundaryChart::decode_payload(&[2]),
        Err(BoundaryChartError::TruncatedPayload)
    ));

    let mut payload = chart(vec![])
        .canonical_payload()
        .expect("chart must encode");
    payload[32 * 7] = 2;
    assert!(matches!(
        BoundaryChart::decode_payload(&payload),
        Err(BoundaryChartError::UnknownOptionalTag(2))
    ));
}

use ic_core::{
    ActiveNegationUse, ArtifactRef, CollectiveCoverageRef, DistinctionRef, GeneratorCoverageRef,
    NegationCoverage, NegationFrontierError, NegationFrontierView, NegationUseRef, Orientation,
    RelationRef, TypedFormRef,
};

fn artifact(value: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([value; 32])
}

fn member(use_value: u8, source: u8, execution: u8) -> ActiveNegationUse {
    ActiveNegationUse::new(
        NegationUseRef::from_artifact_ref(artifact(use_value)),
        TypedFormRef::from_artifact_ref(artifact(source)),
        RelationRef::from_artifact_ref(artifact(use_value + 20)),
        NegationCoverage::CertifiedPartial,
        GeneratorCoverageRef::from_artifact_ref(artifact(execution)),
    )
}

#[test]
fn tagged_frontier_keeps_same_source_through_distinct_use_identities() {
    let first = member(1, 10, 30);
    let second = member(2, 10, 31);
    let frontier = NegationFrontierView::new(
        first.source(),
        DistinctionRef::from_artifact_ref(artifact(40)),
        Orientation::X,
        vec![first, second],
        Some(CollectiveCoverageRef::from_artifact_ref(artifact(50))),
        artifact(60),
    )
    .expect("distinct tagged use views share one source");

    assert_eq!(frontier.members(), &[first, second]);
    assert_ne!(
        frontier.members()[0].use_ref(),
        frontier.members()[1].use_ref(),
        "the view must not collapse distinct uses into one combined negation"
    );
    assert_ne!(
        frontier.members()[0].execution_coverage(),
        frontier.members()[1].execution_coverage(),
        "execution coverage remains occurrence-side data distinct from semantic coverage"
    );
}

#[test]
fn tagged_frontier_rejects_duplicate_use_and_mismatched_source() {
    let first = member(1, 10, 30);
    assert!(matches!(
        NegationFrontierView::new(
            first.source(),
            DistinctionRef::from_artifact_ref(artifact(40)),
            Orientation::X,
            vec![first, first],
            None,
            artifact(60),
        ),
        Err(NegationFrontierError::DuplicateNegationUse(reference)) if reference == first.use_ref()
    ));

    let second = member(2, 11, 31);
    assert!(matches!(
        NegationFrontierView::new(
            first.source(),
            DistinctionRef::from_artifact_ref(artifact(40)),
            Orientation::X,
            vec![first, second],
            None,
            artifact(60),
        ),
        Err(NegationFrontierError::MemberSourceMismatch { expected, actual })
            if expected == first.source() && actual == second.source()
    ));
}

#[test]
fn empty_frontier_does_not_infer_coverage_or_closure() {
    let frontier = NegationFrontierView::new(
        TypedFormRef::from_artifact_ref(artifact(10)),
        DistinctionRef::from_artifact_ref(artifact(40)),
        Orientation::Y,
        vec![],
        None,
        artifact(60),
    )
    .expect("an empty derived frontier remains a valid non-closure view");

    assert!(frontier.members().is_empty());
    assert_eq!(frontier.collective_coverage(), None);
}

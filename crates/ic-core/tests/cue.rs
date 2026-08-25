use ic_core::{
    ApplicabilityRef, ArtifactRef, BindingVersionRef, ExactFiniteCueBasisCandidate,
    ExactFiniteCueBasisError, ExactFiniteCueBasisResult, ExactFiniteCueFrontierError,
    ExactFiniteSignature, FiniteResourcePreorder, GrainRef, HorizonRef, ScopeRef, SignatureContext,
    TypeRef, check_exact_finite_cue_basis, select_nondominated_exact_finite_cue_bases,
};

fn artifact(value: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([value; 32])
}

fn context(scope: u8) -> SignatureContext {
    SignatureContext::new(
        BindingVersionRef::from_artifact_ref(artifact(1)),
        ScopeRef::from_artifact_ref(artifact(scope)),
        ApplicabilityRef::from_artifact_ref(artifact(3)),
        GrainRef::from_artifact_ref(artifact(4)),
        HorizonRef::from_artifact_ref(artifact(5)),
        TypeRef::from_artifact_ref(artifact(6)),
    )
}

fn signature(context: SignatureContext, values: &[(u8, u8)]) -> ExactFiniteSignature {
    ExactFiniteSignature::new(
        context,
        values
            .iter()
            .map(|(domain, answer)| (artifact(*domain), artifact(*answer)))
            .collect(),
    )
    .expect("finite fixture has unique domain entries")
}

#[test]
fn exact_finite_cue_basis_returns_a_protected_separator_or_sufficiency() {
    let context = context(2);
    let protected = signature(context, &[(10, 1), (11, 2), (12, 1)]);
    let insufficient = signature(context, &[(10, 7), (11, 7), (12, 8)]);
    let sufficient = signature(context, &[(10, 3), (11, 4), (12, 3)]);

    assert!(matches!(
        check_exact_finite_cue_basis(std::slice::from_ref(&insufficient), &protected),
        Ok(ExactFiniteCueBasisResult::Insufficient { separator })
            if separator.first_domain() == artifact(10)
                && separator.second_domain() == artifact(11)
                && separator.first_protected_value() == artifact(1)
                && separator.second_protected_value() == artifact(2)
                && separator.cue_answers() == [artifact(7)]
    ));
    assert_eq!(
        check_exact_finite_cue_basis(&[insufficient, sufficient], &protected),
        Ok(ExactFiniteCueBasisResult::Sufficient)
    );
}

#[test]
fn empty_basis_is_only_sufficient_for_a_constant_protected_signature() {
    let context = context(2);
    assert!(matches!(
        check_exact_finite_cue_basis(&[], &signature(context, &[(10, 1), (11, 2)])),
        Ok(ExactFiniteCueBasisResult::Insufficient { separator })
            if separator.cue_answers().is_empty()
    ));
    assert_eq!(
        check_exact_finite_cue_basis(&[], &signature(context, &[(10, 1), (11, 1)])),
        Ok(ExactFiniteCueBasisResult::Sufficient)
    );
}

#[test]
fn exact_finite_cue_basis_rejects_context_and_domain_mismatches() {
    let protected = signature(context(2), &[(10, 1), (11, 2)]);
    assert!(matches!(
        check_exact_finite_cue_basis(&[signature(context(9), &[(10, 3), (11, 4)])], &protected),
        Err(ExactFiniteCueBasisError::ContextMismatch { cue_index: 0, .. })
    ));
    assert_eq!(
        check_exact_finite_cue_basis(&[signature(context(2), &[(10, 3)])], &protected),
        Err(ExactFiniteCueBasisError::DomainMismatch { cue_index: 0 })
    );
}

#[test]
fn finite_resource_preorder_keeps_incomparable_sufficient_bases_and_residuals() {
    let context = context(2);
    let protected = signature(context, &[(10, 1), (11, 2), (12, 1)]);
    let cues = vec![
        signature(context, &[(10, 7), (11, 7), (12, 8)]),
        signature(context, &[(10, 3), (11, 4), (12, 3)]),
        signature(context, &[(10, 5), (11, 6), (12, 5)]),
    ];
    let insufficient = ExactFiniteCueBasisCandidate::new(vec![0], artifact(30))
        .expect("one sorted cue index is valid");
    let first = ExactFiniteCueBasisCandidate::new(vec![1], artifact(31))
        .expect("one sorted cue index is valid");
    let second = ExactFiniteCueBasisCandidate::new(vec![2], artifact(32))
        .expect("one sorted cue index is valid");
    let dominated = ExactFiniteCueBasisCandidate::new(vec![1, 2], artifact(33))
        .expect("sorted cue indices are valid");
    let order = FiniteResourcePreorder::new(vec![
        (artifact(30), artifact(30)),
        (artifact(31), artifact(31)),
        (artifact(32), artifact(32)),
        (artifact(33), artifact(33)),
        (artifact(31), artifact(33)),
        (artifact(32), artifact(33)),
    ])
    .expect("distinct preorder edges are valid");

    let frontier = select_nondominated_exact_finite_cue_bases(
        &cues,
        &protected,
        &[
            insufficient.clone(),
            first.clone(),
            second.clone(),
            dominated,
        ],
        &order,
    )
    .expect("the declared order is reflexive and transitive over candidates");

    assert_eq!(frontier.members(), &[first, second]);
    assert_eq!(frontier.insufficient().len(), 1);
    assert_eq!(frontier.insufficient()[0].candidate(), &insufficient);
    assert_eq!(
        frontier.insufficient()[0].separator().cue_answers(),
        [artifact(7)]
    );
}

#[test]
fn finite_resource_preorder_rejects_invalid_orders_and_candidates() {
    assert_eq!(
        ExactFiniteCueBasisCandidate::new(vec![1, 1], artifact(30)),
        Err(ExactFiniteCueFrontierError::NonCanonicalCueIndices)
    );

    let context = context(2);
    let protected = signature(context, &[(10, 1), (11, 2)]);
    let cues = vec![signature(context, &[(10, 3), (11, 4)])];
    let candidate =
        ExactFiniteCueBasisCandidate::new(vec![0], artifact(31)).expect("one cue index is valid");
    let non_reflexive = FiniteResourcePreorder::new(vec![]).expect("empty relation constructs");
    assert_eq!(
        select_nondominated_exact_finite_cue_bases(
            &cues,
            &protected,
            std::slice::from_ref(&candidate),
            &non_reflexive,
        ),
        Err(ExactFiniteCueFrontierError::NonReflexiveResource(artifact(
            31
        )))
    );
    let reflexive = FiniteResourcePreorder::new(vec![(artifact(31), artifact(31))])
        .expect("one reflexive edge constructs");
    let out_of_range = ExactFiniteCueBasisCandidate::new(vec![1], artifact(31))
        .expect("canonical but out-of-range index constructs");
    assert_eq!(
        select_nondominated_exact_finite_cue_bases(&cues, &protected, &[out_of_range], &reflexive),
        Err(ExactFiniteCueFrontierError::CueIndexOutOfRange {
            index: 1,
            cue_count: 1,
        })
    );
}

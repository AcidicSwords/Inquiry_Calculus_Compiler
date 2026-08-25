use ic_core::{
    ApplicabilityRef, ArtifactRef, BindingVersionRef, ExactFiniteCueBasisError,
    ExactFiniteCueBasisResult, ExactFiniteSignature, GrainRef, HorizonRef, ScopeRef,
    SignatureContext, TypeRef, check_exact_finite_cue_basis,
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

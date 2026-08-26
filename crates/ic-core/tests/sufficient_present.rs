use ic_core::{
    ApplicabilityRef, ArtifactRef, BindingVersionRef, ExactDeterminationError,
    ExactFinitePresentChallenge, ExactFiniteSignature, ExactFiniteSufficientPresentError,
    ExactFiniteSufficientPresentResult, ExactProtectedContinuation, GrainRef, HorizonRef,
    ProtectedContinuationRef, ScopeRef, SignatureContext, TypeRef,
    challenge_exact_finite_sufficient_present, derive_exact_finite_sufficient_present,
};

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn context(byte: u8) -> SignatureContext {
    SignatureContext::new(
        BindingVersionRef::from_artifact_ref(artifact(byte)),
        ScopeRef::from_artifact_ref(artifact(byte + 1)),
        ApplicabilityRef::from_artifact_ref(artifact(byte + 2)),
        GrainRef::from_artifact_ref(artifact(byte + 3)),
        HorizonRef::from_artifact_ref(artifact(byte + 4)),
        TypeRef::from_artifact_ref(artifact(byte + 5)),
    )
}

fn signature(
    context: SignatureContext,
    first: ArtifactRef,
    second: ArtifactRef,
) -> ExactFiniteSignature {
    ExactFiniteSignature::new(context, vec![(artifact(1), first), (artifact(2), second)])
        .expect("fixture domain is exact")
}

#[test]
fn one_class_present_is_exactly_sufficient_then_reopens_on_a_new_continuation() {
    let context = context(0x20);
    let presentation = signature(context, artifact(0x40), artifact(0x40));
    let current = ExactProtectedContinuation::new(
        ProtectedContinuationRef::from_artifact_ref(artifact(0x50)),
        signature(context, artifact(0x60), artifact(0x60)),
    );
    let ExactFiniteSufficientPresentResult::Sufficient(present) =
        derive_exact_finite_sufficient_present(presentation.clone(), vec![current.clone()])
            .expect("exact contexts must agree")
    else {
        panic!("the current continuation is constant on the sole present class")
    };
    assert_eq!(
        present.class_count(),
        1,
        "one class is the coarsest quotient"
    );
    assert_eq!(present.factorizations().len(), 1);

    let regenerated = derive_exact_finite_sufficient_present(presentation, vec![current])
        .expect("the same exact basis must regenerate");
    assert_eq!(
        regenerated,
        ExactFiniteSufficientPresentResult::Sufficient(present.clone())
    );

    let future = ExactProtectedContinuation::new(
        ProtectedContinuationRef::from_artifact_ref(artifact(0x51)),
        signature(context, artifact(0x70), artifact(0x71)),
    );
    let ExactFinitePresentChallenge::Reopened(witness) =
        challenge_exact_finite_sufficient_present(&present, future)
            .expect("future context must agree")
    else {
        panic!("a future continuation that splits the sole class must reopen the fold")
    };
    let separator = witness.separator();
    assert_eq!(separator.first_domain(), artifact(1));
    assert_eq!(separator.second_domain(), artifact(2));
    assert_eq!(separator.available_value(), artifact(0x40));
    assert_ne!(
        separator.first_target_value(),
        separator.second_target_value()
    );
}

#[test]
fn present_extension_rejects_duplicates_and_context_drift() {
    let signature_context = context(0x80);
    let continuation_ref = ProtectedContinuationRef::from_artifact_ref(artifact(0x90));
    let protected = ExactProtectedContinuation::new(
        continuation_ref,
        signature(signature_context, artifact(0xa0), artifact(0xa0)),
    );
    let presentation = signature(signature_context, artifact(0xb0), artifact(0xb0));
    assert!(matches!(
        derive_exact_finite_sufficient_present(
            presentation.clone(),
            vec![protected.clone(), protected.clone()],
        ),
        Err(ExactFiniteSufficientPresentError::DuplicateProtectedContinuation(value))
            if value == continuation_ref
    ));

    let changed_context = ExactProtectedContinuation::new(
        ProtectedContinuationRef::from_artifact_ref(artifact(0x91)),
        signature(context(0xa0), artifact(0xc0), artifact(0xc0)),
    );
    assert!(matches!(
        derive_exact_finite_sufficient_present(presentation, vec![changed_context]),
        Err(ExactFiniteSufficientPresentError::Determination(
            ExactDeterminationError::ContextMismatch { .. }
        ))
    ));
}

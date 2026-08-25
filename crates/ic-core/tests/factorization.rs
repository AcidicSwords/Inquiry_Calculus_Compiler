use ic_core::{
    ApplicabilityRef, ArtifactRef, BindingVersionRef, ExactDeterminationError,
    ExactDeterminationResult, ExactFiniteSignature, GrainRef, HorizonRef, ScopeRef,
    SignatureContext, TypeRef, determine_through_exact,
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

#[test]
fn exact_factorization_constructs_the_target_map_when_kernels_are_included() {
    let signature_context = context(0x10);
    let available = ExactFiniteSignature::new(
        signature_context,
        vec![
            (artifact(1), artifact(10)),
            (artifact(2), artifact(10)),
            (artifact(3), artifact(11)),
        ],
    )
    .expect("unique domain values");
    let target = ExactFiniteSignature::new(
        signature_context,
        vec![
            (artifact(1), artifact(20)),
            (artifact(2), artifact(20)),
            (artifact(3), artifact(21)),
        ],
    )
    .expect("unique domain values");
    let ExactDeterminationResult::Exact { factorization } =
        determine_through_exact(&available, &target).expect("contexts match")
    else {
        panic!("target must factor through available signature")
    };
    assert_eq!(
        factorization.factor().get(&artifact(10)),
        Some(&artifact(20))
    );
    assert_eq!(
        factorization.factor().get(&artifact(11)),
        Some(&artifact(21))
    );
}

#[test]
fn exact_factorization_returns_a_kernel_separator_and_rejects_incomplete_contexts() {
    let signature_context = context(0x30);
    let available = ExactFiniteSignature::new(
        signature_context,
        vec![(artifact(1), artifact(10)), (artifact(2), artifact(10))],
    )
    .expect("unique domain values");
    let target = ExactFiniteSignature::new(
        signature_context,
        vec![(artifact(1), artifact(20)), (artifact(2), artifact(21))],
    )
    .expect("unique domain values");
    let ExactDeterminationResult::NotDetermined { separator } =
        determine_through_exact(&available, &target).expect("contexts match")
    else {
        panic!("different target values inside one available kernel must separate")
    };
    assert_eq!(separator.available_value(), artifact(10));
    assert_ne!(
        separator.first_target_value(),
        separator.second_target_value()
    );

    let incomplete =
        ExactFiniteSignature::new(signature_context, vec![(artifact(1), artifact(20))])
            .expect("unique domain values");
    assert!(matches!(
        determine_through_exact(&available, &incomplete),
        Err(ExactDeterminationError::DomainCoverageMismatch { .. })
    ));
    let changed_scope = ExactFiniteSignature::new(
        context(0x40),
        available.values().iter().map(|(x, y)| (*x, *y)).collect(),
    )
    .expect("unique domain values");
    assert!(matches!(
        determine_through_exact(&available, &changed_scope),
        Err(ExactDeterminationError::ContextMismatch { .. })
    ));
}

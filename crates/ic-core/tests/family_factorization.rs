use ic_core::{
    ApplicabilityRef, ArtifactRef, BindingVersionRef, ExactDeterminationError,
    ExactDeterminationResult, ExactFamilyDeterminationResult, ExactFamilySignature,
    ExactFiniteSignature, GrainRef, HorizonRef, ScopeRef, SignatureContext, TypeRef,
    determine_through_exact, determine_through_exact_family,
};

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn context() -> SignatureContext {
    SignatureContext::new(
        BindingVersionRef::from_artifact_ref(artifact(0x10)),
        ScopeRef::from_artifact_ref(artifact(0x11)),
        ApplicabilityRef::from_artifact_ref(artifact(0x12)),
        GrainRef::from_artifact_ref(artifact(0x13)),
        HorizonRef::from_artifact_ref(artifact(0x14)),
        TypeRef::from_artifact_ref(artifact(0x15)),
    )
}

fn signature(values: [u8; 4]) -> ExactFiniteSignature {
    ExactFiniteSignature::new(
        context(),
        vec![
            (artifact(1), artifact(values[0])),
            (artifact(2), artifact(values[1])),
            (artifact(3), artifact(values[2])),
            (artifact(4), artifact(values[3])),
        ],
    )
    .expect("fixture domain is unique")
}

#[test]
fn exact_family_product_recovers_a_target_that_no_member_recovers() {
    let first = signature([20, 20, 21, 21]);
    let second = signature([30, 31, 30, 31]);
    let target = signature([40, 41, 42, 43]);
    assert!(matches!(
        determine_through_exact(&first, &target),
        Ok(ExactDeterminationResult::NotDetermined { .. })
    ));
    assert!(matches!(
        determine_through_exact(&second, &target),
        Ok(ExactDeterminationResult::NotDetermined { .. })
    ));

    let family = ExactFamilySignature::new(vec![first, second]).expect("shared exact domain");
    let ExactFamilyDeterminationResult::Exact { factorization } =
        determine_through_exact_family(&family, &target).expect("shared exact context")
    else {
        panic!("the tagged product must retain the four target distinctions")
    };
    assert_eq!(factorization.factor().len(), 4);
}

#[test]
fn exact_family_product_rejects_bad_coverage_and_exposes_joint_kernel_separators() {
    assert!(matches!(
        ExactFamilySignature::new(vec![]),
        Err(ExactDeterminationError::EmptyFamily)
    ));

    let first = signature([20, 20, 21, 21]);
    let incomplete = ExactFiniteSignature::new(
        context(),
        vec![(artifact(1), artifact(30)), (artifact(2), artifact(31))],
    )
    .expect("fixture domain is unique");
    assert!(matches!(
        ExactFamilySignature::new(vec![first.clone(), incomplete]),
        Err(ExactDeterminationError::DomainCoverageMismatch { .. })
    ));

    let family = ExactFamilySignature::new(vec![first, signature([30, 30, 31, 31])])
        .expect("shared exact domain");
    let target = signature([40, 41, 42, 43]);
    let ExactFamilyDeterminationResult::NotDetermined { separator } =
        determine_through_exact_family(&family, &target).expect("shared exact context")
    else {
        panic!("equal product signatures with different targets must separate")
    };
    assert_eq!(separator.available_values(), &[artifact(20), artifact(30)]);
    assert_ne!(
        separator.first_target_value(),
        separator.second_target_value()
    );
}

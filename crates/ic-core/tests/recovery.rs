use ic_core::{
    ApplicabilityRef, ArtifactRef, BindingVersionRef, ExactFiberRecovery, ExactFiberRecoveryError,
    ExactFiniteSignature, GrainRef, HorizonRef, RecoveryStatusIR, ScopeRef, SignatureContext,
    TypeRef, check_exact_fiber_recovery,
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

#[test]
fn exact_fiber_recovery_requires_signature_constancy_and_emits_a_positive_separator() {
    let recovered = ExactFiniteSignature::new(
        context(),
        vec![(artifact(1), artifact(9)), (artifact(2), artifact(9))],
    )
    .expect("unique candidates");
    assert_eq!(
        check_exact_fiber_recovery(&recovered),
        Ok(ExactFiberRecovery::Recovered {
            protected_signature: artifact(9),
        })
    );

    let split = ExactFiniteSignature::new(
        context(),
        vec![(artifact(1), artifact(9)), (artifact(2), artifact(10))],
    )
    .expect("unique candidates");
    let Ok(ExactFiberRecovery::NotRecovered { separator }) = check_exact_fiber_recovery(&split)
    else {
        panic!("different protected signatures must produce a positive separator")
    };
    assert_eq!(separator.first_candidate(), artifact(1));
    assert_eq!(separator.second_candidate(), artifact(2));
    assert_ne!(separator.first_signature(), separator.second_signature());
}

#[test]
fn empty_or_incomplete_evidence_is_not_conflated_with_non_recovery() {
    let empty = ExactFiniteSignature::new(context(), vec![]).expect("unique candidates");
    assert_eq!(
        check_exact_fiber_recovery(&empty),
        Err(ExactFiberRecoveryError::EmptyFiber)
    );

    let unknown = RecoveryStatusIR::Unknown {
        residual: ic_core::QueryRef::from_artifact_ref(artifact(20)),
    };
    let not_recovered = RecoveryStatusIR::NotRecovered {
        separator: artifact(21),
    };
    assert_ne!(unknown, not_recovered);
}

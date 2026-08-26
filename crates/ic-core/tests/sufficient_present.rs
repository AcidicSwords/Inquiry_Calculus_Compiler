use ic_core::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactKind, ArtifactRef, BindingVersionRef,
    ExactDeterminationError, ExactFinitePresentChallenge, ExactFinitePresentReopenError,
    ExactFinitePresentReopenWitness, ExactFinitePresentUpdate, ExactFinitePresentUpdateError,
    ExactFiniteSignature, ExactFiniteSufficientPresentError, ExactFiniteSufficientPresentResult,
    ExactProtectedContinuation, FINITE_PRESENT_REOPEN_ARTIFACT_KIND,
    FINITE_PRESENT_REOPEN_SCHEMA_VERSION, GrainRef, HorizonRef, ProtectedContinuationRef, ScopeRef,
    SignatureContext, TypeRef, challenge_exact_finite_sufficient_present,
    derive_exact_finite_sufficient_present, extend_exact_finite_sufficient_present,
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

    let envelope = witness.envelope().expect("reopen witness must encode");
    assert_eq!(
        ExactFinitePresentReopenWitness::from_envelope(&envelope)
            .expect("reopen witness must decode"),
        witness
    );
    assert_eq!(
        witness.reopen_ref().expect("reopen witness must hash"),
        envelope.artifact_ref().expect("envelope must hash")
    );
    assert_eq!(
        witness.referenced_artifacts(),
        vec![
            artifact(0x51),
            artifact(1),
            artifact(2),
            artifact(0x40),
            separator.first_target_value(),
            separator.second_target_value(),
        ]
    );

    let payload = witness.canonical_payload();
    assert!(matches!(
        ExactFinitePresentReopenWitness::decode_payload(&payload[..payload.len() - 1]),
        Err(ExactFinitePresentReopenError::TruncatedPayload)
    ));
    let mut trailing = payload;
    trailing.push(0);
    assert!(matches!(
        ExactFinitePresentReopenWitness::decode_payload(&trailing),
        Err(ExactFinitePresentReopenError::TrailingPayloadBytes(1))
    ));
    let mut repeated_domain = witness.canonical_payload();
    repeated_domain.copy_within(32..64, 64);
    assert!(matches!(
        ExactFinitePresentReopenWitness::decode_payload(&repeated_domain),
        Err(ExactFinitePresentReopenError::IdenticalDomains(_))
    ));
    let mut equal_targets = witness.canonical_payload();
    equal_targets.copy_within(128..160, 160);
    assert!(matches!(
        ExactFinitePresentReopenWitness::decode_payload(&equal_targets),
        Err(ExactFinitePresentReopenError::UndifferentiatedTargets(_))
    ));
    let wrong_kind = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("fixture").expect("kind must be valid"),
        FINITE_PRESENT_REOPEN_SCHEMA_VERSION,
        witness.canonical_payload(),
    );
    assert!(matches!(
        ExactFinitePresentReopenWitness::from_envelope(&wrong_kind),
        Err(ExactFinitePresentReopenError::UnexpectedArtifactKind { .. })
    ));
    let wrong_schema = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new(FINITE_PRESENT_REOPEN_ARTIFACT_KIND).expect("kind must be valid"),
        FINITE_PRESENT_REOPEN_SCHEMA_VERSION + 1,
        witness.canonical_payload(),
    );
    assert!(matches!(
        ExactFinitePresentReopenWitness::from_envelope(&wrong_schema),
        Err(ExactFinitePresentReopenError::UnsupportedSchemaVersion(_))
    ));
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

#[test]
fn present_history_update_preserves_prior_rows_or_returns_a_positive_reopen_witness() {
    let signature_context = context(0xb0);
    let continuation = ProtectedContinuationRef::from_artifact_ref(artifact(0xc0));
    let prior_presentation = ExactFiniteSignature::new(
        signature_context,
        vec![(artifact(1), artifact(0xd0)), (artifact(2), artifact(0xd0))],
    )
    .expect("prior history must be exact");
    let prior_observation = ExactFiniteSignature::new(
        signature_context,
        vec![(artifact(1), artifact(0xe0)), (artifact(2), artifact(0xe0))],
    )
    .expect("prior protected observation must be exact");
    let ExactFiniteSufficientPresentResult::Sufficient(prior) =
        derive_exact_finite_sufficient_present(
            prior_presentation,
            vec![ExactProtectedContinuation::new(
                continuation,
                prior_observation,
            )],
        )
        .expect("prior fold must be sufficient")
    else {
        panic!("constant prior observation must fold")
    };

    let updated_presentation = ExactFiniteSignature::new(
        signature_context,
        vec![
            (artifact(1), artifact(0xd0)),
            (artifact(2), artifact(0xd0)),
            (artifact(3), artifact(0xd0)),
        ],
    )
    .expect("extended history must be exact");
    let updated_observation = ExactFiniteSignature::new(
        signature_context,
        vec![
            (artifact(1), artifact(0xe0)),
            (artifact(2), artifact(0xe0)),
            (artifact(3), artifact(0xe0)),
        ],
    )
    .expect("extended protected observation must be exact");
    let ExactFinitePresentUpdate::Updated(updated) = extend_exact_finite_sufficient_present(
        &prior,
        updated_presentation.clone(),
        vec![ExactProtectedContinuation::new(
            continuation,
            updated_observation,
        )],
    )
    .expect("a constant appended history remains sufficient") else {
        panic!("constant appended history must remain folded")
    };
    assert_eq!(updated.class_count(), 1);

    let reopening_observation = ExactFiniteSignature::new(
        signature_context,
        vec![
            (artifact(1), artifact(0xe0)),
            (artifact(2), artifact(0xe0)),
            (artifact(3), artifact(0xe1)),
        ],
    )
    .expect("reopening observation must be exact");
    let ExactFinitePresentUpdate::Reopened(witness) = extend_exact_finite_sufficient_present(
        &prior,
        updated_presentation.clone(),
        vec![ExactProtectedContinuation::new(
            continuation,
            reopening_observation,
        )],
    )
    .expect("a new protected difference must produce a positive separator") else {
        panic!("new history difference must reopen the prior fold")
    };
    assert_eq!(witness.continuation(), continuation);
    assert_eq!(witness.separator().available_value(), artifact(0xd0));

    let rewritten = ExactFiniteSignature::new(
        signature_context,
        vec![
            (artifact(1), artifact(0xd1)),
            (artifact(2), artifact(0xd0)),
            (artifact(3), artifact(0xd0)),
        ],
    )
    .expect("rewritten proposed history is syntactically exact");
    assert!(matches!(
        extend_exact_finite_sufficient_present(
            &prior,
            rewritten,
            vec![ExactProtectedContinuation::new(
                continuation,
                ExactFiniteSignature::new(
                    signature_context,
                    vec![
                        (artifact(1), artifact(0xe0)),
                        (artifact(2), artifact(0xe0)),
                        (artifact(3), artifact(0xe0)),
                    ],
                )
                .expect("rewritten check observation is exact"),
            )],
        ),
        Err(ExactFinitePresentUpdateError::PresentationHistoryChanged(history))
            if history == artifact(1)
    ));
}

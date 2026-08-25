use ic_core::{
    ArtifactEnvelope, ArtifactKind, ArtifactRef, BindingVersionRef, EffectivityRef,
    GeneratorRegimeRef, GrainRef, HorizonRef, ProtectedClassRef, ProtectedCompletionFieldRef,
    SEPARATOR_PROBLEM_ARTIFACT_KIND, SEPARATOR_PROBLEM_SCHEMA_VERSION, SeparatorProblem,
    SeparatorProblemError, StructureViewRef,
};

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn problem(target: Option<ProtectedClassRef>) -> SeparatorProblem {
    SeparatorProblem::new(
        ProtectedCompletionFieldRef::from_artifact_ref(artifact(0x11)),
        target,
        GrainRef::from_artifact_ref(artifact(0x22)),
        HorizonRef::from_artifact_ref(artifact(0x33)),
        BindingVersionRef::from_artifact_ref(artifact(0x44)),
        StructureViewRef::from_artifact_ref(artifact(0x55)),
        GeneratorRegimeRef::from_artifact_ref(artifact(0x66)),
        EffectivityRef::from_artifact_ref(artifact(0x77)),
    )
}

#[test]
fn separator_problem_round_trips_and_keeps_its_generic_residual_context() {
    let target = ProtectedClassRef::from_artifact_ref(artifact(0x88));
    let with_target = problem(Some(target));
    let without_target = problem(None);
    let envelope = with_target.envelope().expect("problem must encode");

    assert_eq!(
        SeparatorProblem::from_envelope(&envelope).expect("problem must decode"),
        with_target
    );
    assert_eq!(
        with_target
            .separator_problem_ref()
            .expect("problem must hash")
            .as_artifact_ref(),
        envelope.artifact_ref().expect("envelope must hash")
    );
    assert_ne!(
        with_target
            .separator_problem_ref()
            .expect("targeted problem must hash"),
        without_target
            .separator_problem_ref()
            .expect("untargeted problem must hash"),
        "an absent target class is not the same residual context as a declared target"
    );
    assert_eq!(
        with_target.referenced_artifacts(),
        vec![
            artifact(0x11),
            artifact(0x88),
            artifact(0x22),
            artifact(0x33),
            artifact(0x44),
            artifact(0x55),
            artifact(0x66),
            artifact(0x77),
        ]
    );
}

#[test]
fn separator_problem_rejects_malformed_and_wrong_domain_encodings() {
    let problem = problem(None);
    let payload = problem.canonical_payload();

    assert!(matches!(
        SeparatorProblem::decode_payload(&payload[..payload.len() - 1]),
        Err(SeparatorProblemError::TruncatedPayload)
    ));
    let mut trailing = payload.clone();
    trailing.push(0);
    assert!(matches!(
        SeparatorProblem::decode_payload(&trailing),
        Err(SeparatorProblemError::TrailingPayloadBytes(1))
    ));
    let mut malformed_option = payload;
    malformed_option[32] = 0xFF;
    assert!(matches!(
        SeparatorProblem::decode_payload(&malformed_option),
        Err(SeparatorProblemError::UnknownOptionalTag(0xFF))
    ));

    let wrong_kind = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("ic.open-query").expect("kind valid"),
        SEPARATOR_PROBLEM_SCHEMA_VERSION,
        problem.canonical_payload(),
    );
    assert!(matches!(
        SeparatorProblem::from_envelope(&wrong_kind),
        Err(SeparatorProblemError::UnexpectedArtifactKind { .. })
    ));
    let wrong_schema = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new(SEPARATOR_PROBLEM_ARTIFACT_KIND).expect("kind valid"),
        SEPARATOR_PROBLEM_SCHEMA_VERSION + 1,
        problem.canonical_payload(),
    );
    assert!(matches!(
        SeparatorProblem::from_envelope(&wrong_schema),
        Err(SeparatorProblemError::UnsupportedSchemaVersion(_))
    ));
}

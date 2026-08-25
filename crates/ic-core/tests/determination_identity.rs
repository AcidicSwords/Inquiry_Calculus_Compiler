use ic_core::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactKind, ArtifactRef, BindingVersionRef,
    DETERMINATION_PRESENTATION_ARTIFACT_KIND, DETERMINATION_PRESENTATION_SCHEMA_VERSION,
    DeterminationPresentation, DeterminationPresentationError, DeterminationPresentationRef,
    DistinctionRef, GrainRef, HorizonRef, Orientation, RelationalWebRef, ScopeRef, SupportRef,
    TypedFormRef,
};

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn presentation(
    orientation: Orientation,
    predecessor: Option<DeterminationPresentationRef>,
) -> DeterminationPresentation {
    DeterminationPresentation::new(
        DistinctionRef::from_artifact_ref(artifact(0x11)),
        orientation,
        TypedFormRef::from_artifact_ref(artifact(0x22)),
        RelationalWebRef::from_artifact_ref(artifact(0x33)),
        BindingVersionRef::from_artifact_ref(artifact(0x44)),
        ScopeRef::from_artifact_ref(artifact(0x55)),
        ApplicabilityRef::from_artifact_ref(artifact(0x66)),
        GrainRef::from_artifact_ref(artifact(0x77)),
        HorizonRef::from_artifact_ref(artifact(0x88)),
        SupportRef::from_artifact_ref(artifact(0x99)),
        predecessor,
    )
}

#[test]
fn determination_presentations_round_trip_with_explicit_context_and_ancestry() {
    let predecessor = DeterminationPresentationRef::from_artifact_ref(artifact(0xaa));
    let current = presentation(Orientation::X, Some(predecessor));
    let envelope = current.envelope().expect("presentation must encode");
    assert_eq!(
        DeterminationPresentation::from_envelope(&envelope).expect("presentation must decode"),
        current
    );
    assert_eq!(
        current
            .determination_presentation_ref()
            .expect("presentation must hash")
            .as_artifact_ref(),
        envelope.artifact_ref().expect("presentation must hash")
    );
    assert_eq!(
        current.referenced_artifacts(),
        vec![
            artifact(0x11),
            artifact(0x22),
            artifact(0x33),
            artifact(0x44),
            artifact(0x55),
            artifact(0x66),
            artifact(0x77),
            artifact(0x88),
            artifact(0x99),
            artifact(0xaa),
        ]
    );
    assert_ne!(
        current
            .determination_presentation_ref()
            .expect("presentation must hash"),
        presentation(Orientation::Y, Some(predecessor))
            .determination_presentation_ref()
            .expect("presentation must hash")
    );
    assert_ne!(
        current
            .determination_presentation_ref()
            .expect("presentation must hash"),
        presentation(Orientation::X, None)
            .determination_presentation_ref()
            .expect("presentation must hash")
    );
}

#[test]
fn determination_presentations_reject_malformed_encodings() {
    let current = presentation(Orientation::X, None);
    let payload = current.canonical_payload();
    assert!(matches!(
        DeterminationPresentation::decode_payload(&payload[..payload.len() - 1]),
        Err(DeterminationPresentationError::TruncatedPayload)
    ));
    let mut trailing = payload.clone();
    trailing.push(0);
    assert!(matches!(
        DeterminationPresentation::decode_payload(&trailing),
        Err(DeterminationPresentationError::TrailingPayloadBytes(1))
    ));
    let mut orientation = payload;
    orientation[32] = 0xff;
    assert!(matches!(
        DeterminationPresentation::decode_payload(&orientation),
        Err(DeterminationPresentationError::UnknownOrientation)
    ));
    let wrong_kind = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("ic.iprog").expect("kind must be valid"),
        DETERMINATION_PRESENTATION_SCHEMA_VERSION,
        current.canonical_payload(),
    );
    assert!(matches!(
        DeterminationPresentation::from_envelope(&wrong_kind),
        Err(DeterminationPresentationError::UnexpectedArtifactKind { .. })
    ));
    let wrong_schema = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new(DETERMINATION_PRESENTATION_ARTIFACT_KIND).expect("kind must be valid"),
        DETERMINATION_PRESENTATION_SCHEMA_VERSION + 1,
        current.canonical_payload(),
    );
    assert!(matches!(
        DeterminationPresentation::from_envelope(&wrong_schema),
        Err(DeterminationPresentationError::UnsupportedSchemaVersion(_))
    ));
}

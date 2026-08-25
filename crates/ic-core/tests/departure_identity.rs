use ic_core::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactKind, ArtifactRef, DEPARTURE_WITNESS_ARTIFACT_KIND,
    DEPARTURE_WITNESS_SCHEMA_VERSION, DepartureWitness, DepartureWitnessError,
    DeterminationPresentationRef, DistinctionRef, GrainRef, RelationUseRef, ScopeRef, SupportRef,
    TypedFormRef,
};

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn witness() -> DepartureWitness {
    DepartureWitness::new(
        DistinctionRef::from_artifact_ref(artifact(0x11)),
        TypedFormRef::from_artifact_ref(artifact(0x22)),
        TypedFormRef::from_artifact_ref(artifact(0x33)),
        DeterminationPresentationRef::from_artifact_ref(artifact(0x44)),
        RelationUseRef::from_artifact_ref(artifact(0x55)),
        RelationUseRef::from_artifact_ref(artifact(0x66)),
        TypedFormRef::from_artifact_ref(artifact(0x77)),
        TypedFormRef::from_artifact_ref(artifact(0x88)),
        RelationUseRef::from_artifact_ref(artifact(0x99)),
        SupportRef::from_artifact_ref(artifact(0xaa)),
        ScopeRef::from_artifact_ref(artifact(0xbb)),
        ApplicabilityRef::from_artifact_ref(artifact(0xcc)),
        GrainRef::from_artifact_ref(artifact(0xdd)),
    )
}

#[test]
fn departure_witnesses_round_trip_with_all_positive_evidence_roles() {
    let witness = witness();
    let envelope = witness.envelope().expect("witness must encode");
    assert_eq!(
        DepartureWitness::from_envelope(&envelope).expect("witness must decode"),
        witness
    );
    assert_eq!(
        witness
            .departure_witness_ref()
            .expect("witness must hash")
            .as_artifact_ref(),
        envelope.artifact_ref().expect("witness must hash")
    );
    assert_eq!(
        witness.referenced_artifacts(),
        [
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
            artifact(0xbb),
            artifact(0xcc),
            artifact(0xdd),
        ]
    );
}

#[test]
fn departure_witnesses_reject_malformed_encodings() {
    let witness = witness();
    let payload = witness.canonical_payload();
    assert!(matches!(
        DepartureWitness::decode_payload(&payload[..payload.len() - 1]),
        Err(DepartureWitnessError::TruncatedPayload)
    ));
    let mut trailing = payload.clone();
    trailing.push(0);
    assert!(matches!(
        DepartureWitness::decode_payload(&trailing),
        Err(DepartureWitnessError::TrailingPayloadBytes(1))
    ));
    let wrong_kind = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("ic.determination-presentation").expect("kind valid"),
        DEPARTURE_WITNESS_SCHEMA_VERSION,
        payload,
    );
    assert!(matches!(
        DepartureWitness::from_envelope(&wrong_kind),
        Err(DepartureWitnessError::UnexpectedArtifactKind { .. })
    ));
    let wrong_schema = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new(DEPARTURE_WITNESS_ARTIFACT_KIND).expect("kind valid"),
        DEPARTURE_WITNESS_SCHEMA_VERSION + 1,
        witness.canonical_payload(),
    );
    assert!(matches!(
        DepartureWitness::from_envelope(&wrong_schema),
        Err(DepartureWitnessError::UnsupportedSchemaVersion(_))
    ));
}

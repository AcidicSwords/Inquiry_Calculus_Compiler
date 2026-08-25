use ic_core::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactKind, ArtifactRef, DeterminationPresentationRef,
    DistinctionRef, GrainRef, HorizonRef, IProgRef, NEGATION_USE_ARTIFACT_KIND,
    NEGATION_USE_SCHEMA_VERSION, NegationCoverage, NegationUse, NegationUseError, Orientation,
    RelationRef, RelationUseRef, ScopeRef,
};

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn negation_use(coverage: NegationCoverage) -> NegationUse {
    NegationUse::new(
        RelationUseRef::from_artifact_ref(artifact(0x11)),
        DistinctionRef::from_artifact_ref(artifact(0x22)),
        Orientation::Y,
        DeterminationPresentationRef::from_artifact_ref(artifact(0x33)),
        RelationRef::from_artifact_ref(artifact(0x44)),
        IProgRef::from_artifact_ref(artifact(0x55)),
        coverage,
        ApplicabilityRef::from_artifact_ref(artifact(0x66)),
        ScopeRef::from_artifact_ref(artifact(0x77)),
        GrainRef::from_artifact_ref(artifact(0x88)),
        HorizonRef::from_artifact_ref(artifact(0x99)),
        vec![artifact(0xaa), artifact(0xbb)],
    )
}

#[test]
fn negation_use_round_trips_and_keeps_semantic_coverage_distinct() {
    for coverage in [
        NegationCoverage::ExactExhaustive {
            regime: artifact(0xcc),
            certificate: artifact(0xdd),
        },
        NegationCoverage::ExactOnField {
            field: RelationRef::from_artifact_ref(artifact(0xee)),
            certificate: artifact(0xff),
        },
        NegationCoverage::CertifiedPartial,
        NegationCoverage::WorkingOpen,
    ] {
        let use_declaration = negation_use(coverage);
        let envelope = use_declaration.envelope().expect("use must encode");
        assert_eq!(
            NegationUse::from_envelope(&envelope).expect("use must decode"),
            use_declaration
        );
        assert_eq!(
            use_declaration
                .negation_use_ref()
                .expect("use must hash")
                .as_artifact_ref(),
            envelope.artifact_ref().expect("use must hash")
        );
    }
    assert_ne!(
        negation_use(NegationCoverage::CertifiedPartial)
            .negation_use_ref()
            .expect("partial use must hash"),
        negation_use(NegationCoverage::WorkingOpen)
            .negation_use_ref()
            .expect("open use must hash")
    );
}

#[test]
fn negation_use_rejects_malformed_encodings() {
    let use_declaration = negation_use(NegationCoverage::CertifiedPartial);
    let payload = use_declaration
        .canonical_payload()
        .expect("use must encode");
    assert!(matches!(
        NegationUse::decode_payload(&payload[..payload.len() - 1]),
        Err(NegationUseError::TruncatedPayload)
    ));
    let mut trailing = payload.clone();
    trailing.push(0);
    assert!(matches!(
        NegationUse::decode_payload(&trailing),
        Err(NegationUseError::TrailingPayloadBytes(1))
    ));
    let wrong_kind = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("ic.departure-witness").expect("kind valid"),
        NEGATION_USE_SCHEMA_VERSION,
        payload,
    );
    assert!(matches!(
        NegationUse::from_envelope(&wrong_kind),
        Err(NegationUseError::UnexpectedArtifactKind { .. })
    ));
    let wrong_schema = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new(NEGATION_USE_ARTIFACT_KIND).expect("kind valid"),
        NEGATION_USE_SCHEMA_VERSION + 1,
        use_declaration
            .canonical_payload()
            .expect("use must encode"),
    );
    assert!(matches!(
        NegationUse::from_envelope(&wrong_schema),
        Err(NegationUseError::UnsupportedSchemaVersion(_))
    ));
}

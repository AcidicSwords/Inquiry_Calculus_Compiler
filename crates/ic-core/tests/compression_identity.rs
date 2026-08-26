use ic_core::{
    ArtifactEnvelope, ArtifactKind, ArtifactRef, COMPRESSION_LICENSE_ARTIFACT_KIND,
    COMPRESSION_LICENSE_SCHEMA_VERSION, CompressionKind, CompressionLicense,
    CompressionLicenseError, DistortionContractRef, FoldOrQuotientRef, HorizonRef,
    ProtectedContinuationRef, RecoveryContractRef, ScopeRef, UnlockConditionRef,
};

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn exact_license() -> CompressionLicense {
    CompressionLicense::new(
        FoldOrQuotientRef::from_artifact_ref(artifact(0x10)),
        CompressionKind::Exact,
        HorizonRef::from_artifact_ref(artifact(0x11)),
        vec![
            ProtectedContinuationRef::from_artifact_ref(artifact(0x13)),
            ProtectedContinuationRef::from_artifact_ref(artifact(0x12)),
        ],
        ScopeRef::from_artifact_ref(artifact(0x14)),
        vec![artifact(0x16), artifact(0x15)],
        artifact(0x17),
        RecoveryContractRef::from_artifact_ref(artifact(0x18)),
        vec![
            UnlockConditionRef::from_artifact_ref(artifact(0x1a)),
            UnlockConditionRef::from_artifact_ref(artifact(0x19)),
        ],
    )
    .expect("fixture references must canonicalize")
}

#[test]
fn compression_licence_keeps_exact_and_directional_approximate_contracts_distinct() {
    let exact = exact_license();
    let approximate = CompressionLicense::new(
        exact.folded(),
        CompressionKind::Approximate {
            distortion_contract: DistortionContractRef::from_artifact_ref(artifact(0x20)),
        },
        exact.horizon(),
        exact.continuations().to_vec(),
        exact.scope(),
        exact.evidence().to_vec(),
        exact.residual(),
        exact.recovery(),
        exact.unlock_conditions().to_vec(),
    )
    .expect("explicit directional approximation remains representable");

    assert_eq!(
        exact.continuations(),
        [
            ProtectedContinuationRef::from_artifact_ref(artifact(0x12)),
            ProtectedContinuationRef::from_artifact_ref(artifact(0x13)),
        ]
    );
    assert_eq!(exact.evidence(), [artifact(0x15), artifact(0x16)]);
    assert_eq!(
        exact.unlock_conditions(),
        [
            UnlockConditionRef::from_artifact_ref(artifact(0x19)),
            UnlockConditionRef::from_artifact_ref(artifact(0x1a)),
        ]
    );
    assert_ne!(
        exact
            .compression_license_ref()
            .expect("exact licence must hash"),
        approximate
            .compression_license_ref()
            .expect("approximate licence must hash"),
        "a directional approximation cannot silently become an exact licence"
    );
    assert_eq!(
        CompressionLicense::from_envelope(&exact.envelope().expect("licence must encode"))
            .expect("licence must decode"),
        exact
    );
    assert_eq!(
        exact.referenced_artifacts(),
        vec![
            artifact(0x10),
            artifact(0x11),
            artifact(0x12),
            artifact(0x13),
            artifact(0x14),
            artifact(0x15),
            artifact(0x16),
            artifact(0x17),
            artifact(0x18),
            artifact(0x19),
            artifact(0x1a),
        ]
    );
}

#[test]
fn compression_licence_rejects_duplicate_and_malformed_contracts() {
    assert!(matches!(
        CompressionLicense::new(
            FoldOrQuotientRef::from_artifact_ref(artifact(0x10)),
            CompressionKind::Exact,
            HorizonRef::from_artifact_ref(artifact(0x11)),
            vec![
                ProtectedContinuationRef::from_artifact_ref(artifact(0x12)),
                ProtectedContinuationRef::from_artifact_ref(artifact(0x12)),
            ],
            ScopeRef::from_artifact_ref(artifact(0x14)),
            Vec::new(),
            artifact(0x17),
            RecoveryContractRef::from_artifact_ref(artifact(0x18)),
            Vec::new(),
        ),
        Err(CompressionLicenseError::DuplicateContinuation(reference))
            if reference == ProtectedContinuationRef::from_artifact_ref(artifact(0x12))
    ));
    let exact = exact_license();
    let payload = exact.canonical_payload().expect("licence must encode");
    assert!(matches!(
        CompressionLicense::decode_payload(&payload[..payload.len() - 1]),
        Err(CompressionLicenseError::TruncatedPayload)
    ));
    let mut malformed = payload;
    malformed[32] = 0xff;
    assert!(matches!(
        CompressionLicense::decode_payload(&malformed),
        Err(CompressionLicenseError::UnknownKind(0xff))
    ));
    let wrong_kind = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("ic.separator-problem").expect("kind must be valid"),
        COMPRESSION_LICENSE_SCHEMA_VERSION,
        exact.canonical_payload().expect("licence must encode"),
    );
    assert!(matches!(
        CompressionLicense::from_envelope(&wrong_kind),
        Err(CompressionLicenseError::UnexpectedArtifactKind { .. })
    ));
    let wrong_schema = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new(COMPRESSION_LICENSE_ARTIFACT_KIND).expect("kind must be valid"),
        COMPRESSION_LICENSE_SCHEMA_VERSION + 1,
        exact.canonical_payload().expect("licence must encode"),
    );
    assert!(matches!(
        CompressionLicense::from_envelope(&wrong_schema),
        Err(CompressionLicenseError::UnsupportedSchemaVersion(_))
    ));
}

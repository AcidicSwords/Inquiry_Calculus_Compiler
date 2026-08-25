use ic_core::{
    ArtifactEnvelope, ArtifactKind, RAW_RETURN_ARTIFACT_KIND, RAW_RETURN_SCHEMA_VERSION, RawReturn,
    RawReturnError,
};

#[test]
fn raw_return_preserves_exact_opaque_bytes_and_domain_separates_identity() {
    let raw = RawReturn::new(vec![0, 0xff, b'{', b'"', 0, b'}']);
    let envelope = raw.envelope().expect("raw return must encode");
    assert_eq!(envelope.canonical_payload(), raw.bytes());
    assert_eq!(
        RawReturn::from_envelope(&envelope).expect("raw return must decode"),
        raw
    );
    assert_eq!(
        raw.raw_return_ref()
            .expect("raw return must hash")
            .as_artifact_ref(),
        envelope.artifact_ref().expect("raw return must hash")
    );
    assert_ne!(
        raw.raw_return_ref().expect("raw return must hash"),
        RawReturn::new(vec![0, 0xff, b'{', b'"', 0, b'!'])
            .raw_return_ref()
            .expect("changed raw return must hash")
    );
}

#[test]
fn raw_return_rejects_the_wrong_envelope_domain() {
    let raw = RawReturn::new(vec![1, 2, 3]);
    let wrong_kind = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("ic.formula").expect("kind valid"),
        RAW_RETURN_SCHEMA_VERSION,
        raw.bytes().to_vec(),
    );
    assert!(matches!(
        RawReturn::from_envelope(&wrong_kind),
        Err(RawReturnError::UnexpectedArtifactKind { .. })
    ));
    let wrong_schema = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new(RAW_RETURN_ARTIFACT_KIND).expect("kind valid"),
        RAW_RETURN_SCHEMA_VERSION + 1,
        raw.bytes().to_vec(),
    );
    assert!(matches!(
        RawReturn::from_envelope(&wrong_schema),
        Err(RawReturnError::UnsupportedSchemaVersion(_))
    ));
}

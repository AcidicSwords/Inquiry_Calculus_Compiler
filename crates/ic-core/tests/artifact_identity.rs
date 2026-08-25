use std::str::FromStr;

use ic_core::{ARTIFACT_DOMAIN, ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef};
use proptest::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct KnownVector {
    kind: String,
    schema_version: u32,
    payload_hex: String,
    encoded_hex: String,
    sha256: String,
}

fn known_vector() -> KnownVector {
    serde_json::from_str(include_str!("../../../fixtures/artifacts/envelope-v1.json"))
        .expect("known vector fixture must be valid JSON")
}

#[test]
fn matches_independent_known_vector() {
    let vector = known_vector();
    let envelope = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new(vector.kind).expect("fixture kind must be valid"),
        vector.schema_version,
        hex::decode(vector.payload_hex).expect("fixture payload must be valid hex"),
    );

    assert_eq!(
        hex::encode(envelope.encode().expect("fixture must encode")),
        vector.encoded_hex
    );
    assert_eq!(
        envelope
            .artifact_ref()
            .expect("fixture must hash")
            .to_string(),
        vector.sha256
    );
    assert_eq!(
        ArtifactEnvelope::decode(&envelope.encode().expect("fixture must encode"))
            .expect("fixture must decode"),
        envelope
    );
}

#[test]
fn kind_and_schema_version_domain_separate_identity() {
    let payload = vec![1, 2, 3];
    let first = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("form").expect("valid kind"),
        1,
        payload.clone(),
    );
    let other_kind = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("type").expect("valid kind"),
        1,
        payload.clone(),
    );
    let other_schema = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("form").expect("valid kind"),
        2,
        payload,
    );

    assert_ne!(
        first.artifact_ref().expect("first artifact must hash"),
        other_kind.artifact_ref().expect("other kind must hash")
    );
    assert_ne!(
        first.artifact_ref().expect("first artifact must hash"),
        other_schema.artifact_ref().expect("other schema must hash")
    );
}

#[test]
fn rejects_malformed_envelopes() {
    let envelope = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("example").expect("valid kind"),
        1,
        vec![7, 8, 9],
    );
    let encoded = envelope.encode().expect("fixture must encode");

    assert!(matches!(
        ArtifactEnvelope::decode(&encoded[..encoded.len() - 1]),
        Err(ArtifactError::TruncatedEnvelope)
    ));

    let mut trailing = encoded.clone();
    trailing.push(0);
    assert!(matches!(
        ArtifactEnvelope::decode(&trailing),
        Err(ArtifactError::TrailingBytes(1))
    ));

    let mut wrong_domain = encoded.clone();
    wrong_domain[0] ^= 1;
    assert!(matches!(
        ArtifactEnvelope::decode(&wrong_domain),
        Err(ArtifactError::InvalidDomain)
    ));

    let mut wrong_version = encoded.clone();
    wrong_version[ARTIFACT_DOMAIN.len() + 1] = 2;
    assert!(matches!(
        ArtifactEnvelope::decode(&wrong_version),
        Err(ArtifactError::UnsupportedWireVersion(2))
    ));

    let mut invalid_utf8 = encoded;
    let kind_start = ARTIFACT_DOMAIN.len() + 2 + 4;
    invalid_utf8[kind_start] = 0xff;
    assert!(matches!(
        ArtifactEnvelope::decode(&invalid_utf8),
        Err(ArtifactError::InvalidKindUtf8(_))
    ));
}

#[test]
fn validates_kinds_and_reference_text() {
    for invalid in ["", "Form", "two words", "9kind", "kind/child", "é"] {
        assert!(ArtifactKind::new(invalid).is_err(), "accepted {invalid:?}");
    }

    let reference = ArtifactRef::from_bytes([0xab; 32]);
    assert_eq!(
        ArtifactRef::from_str(&reference.to_string()).expect("reference must parse"),
        reference
    );
    assert!(ArtifactRef::from_str("ab").is_err());
}

proptest! {
    #[test]
    fn arbitrary_payload_round_trips(
        suffix in "[a-z0-9._-]{0,32}",
        schema_version in any::<u32>(),
        payload in prop::collection::vec(any::<u8>(), 0..4096),
    ) {
        let kind = ArtifactKind::new(format!("k{suffix}"))?;
        let envelope = ArtifactEnvelope::from_canonical_payload(
            kind,
            schema_version,
            payload,
        );
        let encoded = envelope.encode()?;
        prop_assert_eq!(ArtifactEnvelope::decode(&encoded)?, envelope);
    }
}

use ic_core::{ArtifactRef, BoundaryRef, ProbeOperator, ProbeOperatorError, QueryRef, TypeRef};

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn operator(code: u8) -> ProbeOperator {
    ProbeOperator::new(
        QueryRef::from_artifact_ref(artifact(1)),
        BoundaryRef::from_artifact_ref(artifact(2)),
        artifact(3),
        artifact(4),
        artifact(code),
        TypeRef::from_artifact_ref(artifact(5)),
        artifact(6),
        artifact(7),
        artifact(8),
    )
}

#[test]
fn probe_operator_identity_separates_compiled_code_from_request_and_return_data() {
    let first = operator(9);
    let second = operator(10);
    assert_ne!(
        first.probe_operator_ref().expect("operator must encode"),
        second.probe_operator_ref().expect("operator must encode")
    );
    let envelope = first.envelope().expect("operator must encode");
    assert_eq!(
        ProbeOperator::from_envelope(&envelope).expect("operator must decode"),
        first
    );
}

#[test]
fn probe_operator_rejects_noncanonical_payload_lengths() {
    assert!(matches!(
        ProbeOperator::decode_payload(&[0; 287]),
        Err(ProbeOperatorError::WrongPayloadLength(287))
    ));
}

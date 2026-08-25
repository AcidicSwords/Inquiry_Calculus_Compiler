use ic_core::{
    ApplicabilityRef, ArtifactRef, BindingVersionRef, GrainRef, HorizonRef, ProbeContract,
    ProbeContractError,
};

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn contract(fields: [u8; 8]) -> ProbeContract {
    ProbeContract::new(
        artifact(fields[0]),
        BindingVersionRef::from_artifact_ref(artifact(fields[1])),
        GrainRef::from_artifact_ref(artifact(fields[2])),
        ApplicabilityRef::from_artifact_ref(artifact(fields[3])),
        artifact(fields[4]),
        HorizonRef::from_artifact_ref(artifact(fields[5])),
        artifact(fields[6]),
        artifact(fields[7]),
    )
}

#[test]
fn probe_contract_identity_covers_each_contract_field() {
    let fields = [1, 2, 3, 4, 5, 6, 7, 8];
    let first = contract(fields);
    let first_ref = first.probe_contract_ref().expect("contract must encode");
    for index in 0..fields.len() {
        let mut changed = fields;
        changed[index] += 10;
        assert_ne!(
            first_ref,
            contract(changed)
                .probe_contract_ref()
                .expect("contract must encode"),
            "field {index} must contribute to recurrent probe-contract identity"
        );
    }
    let envelope = first.envelope().expect("contract must encode");
    assert_eq!(
        ProbeContract::from_envelope(&envelope).expect("contract must decode"),
        first
    );
}

#[test]
fn probe_contract_rejects_noncanonical_payload_lengths() {
    assert!(matches!(
        ProbeContract::decode_payload(&[0; 255]),
        Err(ProbeContractError::WrongPayloadLength(255))
    ));
}

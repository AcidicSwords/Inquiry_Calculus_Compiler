use ic_core::{
    ArtifactEnvelope, ArtifactKind, ArtifactRef, IPROG_ARTIFACT_KIND, IPROG_SCHEMA_VERSION,
    IProgArtifact, IProgError, IProgIR, IProgRef, ProgramBinding, QueryRef, TypeRef, TypeSymbol,
    TypedFormRef,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct KnownVector {
    kind: String,
    schema_version: u32,
    payload_hex: String,
    encoded_hex: String,
    sha256: String,
}

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn ask_environment_vector() -> KnownVector {
    serde_json::from_str(include_str!(
        "../../../fixtures/iprogs/iprog-v2-ask-environment.json"
    ))
    .expect("known inquiry-program vector must be valid JSON")
}

#[test]
fn ask_with_environment_matches_independent_canonical_vector() {
    let program = IProgArtifact::new(
        TypeRef::from_artifact_ref(artifact(0x11)),
        IProgIR::Ask {
            question: QueryRef::from_artifact_ref(artifact(0x33)),
            environment: vec![ProgramBinding::new(
                TypeSymbol::new("standing").expect("environment name must be valid"),
                TypedFormRef::from_artifact_ref(artifact(0x55)),
            )],
            answer_slot: TypeSymbol::new("answer").expect("slot must be valid"),
            continuation: IProgRef::from_artifact_ref(artifact(0x44)),
        },
    );
    let envelope = program.envelope().expect("program must encode");
    let vector = ask_environment_vector();
    assert_eq!(envelope.kind().as_str(), vector.kind);
    assert_eq!(envelope.schema_version(), vector.schema_version);
    assert_eq!(
        hex::encode(envelope.canonical_payload()),
        vector.payload_hex
    );
    assert_eq!(
        hex::encode(envelope.encode().expect("program must encode")),
        vector.encoded_hex
    );
    assert_eq!(
        envelope
            .artifact_ref()
            .expect("program must hash")
            .to_string(),
        vector.sha256
    );
}

#[test]
fn first_order_return_and_ask_round_trip_without_closures() {
    let result = TypeRef::from_artifact_ref(artifact(0x11));
    let value = TypedFormRef::from_artifact_ref(artifact(0x22));
    let question = QueryRef::from_artifact_ref(artifact(0x33));
    let continuation = IProgRef::from_artifact_ref(artifact(0x44));
    let returned = IProgArtifact::new(result, IProgIR::Return { value });
    let asked = IProgArtifact::new(
        result,
        IProgIR::Ask {
            question,
            environment: vec![ProgramBinding::new(
                TypeSymbol::new("standing").expect("environment name must be valid"),
                TypedFormRef::from_artifact_ref(artifact(0x55)),
            )],
            answer_slot: TypeSymbol::new("answer").expect("slot must be valid"),
            continuation,
        },
    );
    for program in [&returned, &asked] {
        let envelope = program.envelope().expect("program must encode");
        assert_eq!(
            IProgArtifact::from_envelope(&envelope).expect("program must decode"),
            *program
        );
        assert_eq!(
            program
                .iprog_ref()
                .expect("program must hash")
                .as_artifact_ref(),
            envelope.artifact_ref().expect("program must hash")
        );
    }
    assert_ne!(
        returned.iprog_ref().expect("return must hash"),
        asked.iprog_ref().expect("ask must hash")
    );
    assert_eq!(
        asked.referenced_artifacts(),
        vec![
            artifact(0x11),
            artifact(0x33),
            artifact(0x55),
            artifact(0x44)
        ]
    );
}

#[test]
fn current_source_grammar_has_only_direct_nonexecuting_heads() {
    enum Head {
        Return(TypedFormRef),
        Ask {
            question: QueryRef,
            continuation: IProgRef,
        },
    }

    fn head(program: &IProgArtifact) -> Head {
        match program.program() {
            IProgIR::Return { value } => Head::Return(*value),
            IProgIR::Ask {
                question,
                continuation,
                ..
            } => Head::Ask {
                question: *question,
                continuation: *continuation,
            },
        }
    }

    let result = TypeRef::from_artifact_ref(artifact(0x71));
    let value = TypedFormRef::from_artifact_ref(artifact(0x72));
    let question = QueryRef::from_artifact_ref(artifact(0x73));
    let continuation = IProgRef::from_artifact_ref(artifact(0x74));
    assert!(matches!(
        head(&IProgArtifact::new(result, IProgIR::Return { value })),
        Head::Return(returned) if returned == value
    ));
    assert!(matches!(
        head(&IProgArtifact::new(
            result,
            IProgIR::Ask {
                question,
                environment: Vec::new(),
                answer_slot: TypeSymbol::new("answer").expect("slot must be valid"),
                continuation,
            },
        )),
        Head::Ask { question: actual_question, continuation: actual_continuation }
            if actual_question == question && actual_continuation == continuation
    ));

    // Schema v2 admits only the two direct source constructors. Any future pure operation
    // must introduce a checked, versioned source representation rather than hiding in a normalizer.
    let mut unknown_operation = IProgArtifact::new(result, IProgIR::Return { value })
        .canonical_payload()
        .expect("Return encodes");
    unknown_operation[32] = 2;
    assert!(matches!(
        IProgArtifact::decode_payload(&unknown_operation),
        Err(IProgError::UnknownTag(2))
    ));
}

#[test]
fn rejects_duplicate_explicit_environment_names() {
    let program = IProgArtifact::new(
        TypeRef::from_artifact_ref(artifact(0x11)),
        IProgIR::Ask {
            question: QueryRef::from_artifact_ref(artifact(0x33)),
            environment: vec![
                ProgramBinding::new(
                    TypeSymbol::new("standing").expect("name must be valid"),
                    TypedFormRef::from_artifact_ref(artifact(0x55)),
                ),
                ProgramBinding::new(
                    TypeSymbol::new("standing").expect("name must be valid"),
                    TypedFormRef::from_artifact_ref(artifact(0x66)),
                ),
            ],
            answer_slot: TypeSymbol::new("answer").expect("slot must be valid"),
            continuation: IProgRef::from_artifact_ref(artifact(0x44)),
        },
    );
    assert!(matches!(
        program.canonical_payload(),
        Err(IProgError::DuplicateEnvironmentBinding(name)) if name == "standing"
    ));
}

#[test]
fn rejects_malformed_inquiry_program_encodings() {
    let program = IProgArtifact::new(
        TypeRef::from_artifact_ref(artifact(0x11)),
        IProgIR::Return {
            value: TypedFormRef::from_artifact_ref(artifact(0x22)),
        },
    );
    let payload = program.canonical_payload().expect("program must encode");
    assert!(matches!(
        IProgArtifact::decode_payload(&payload[..payload.len() - 1]),
        Err(IProgError::TruncatedPayload)
    ));
    let mut trailing = payload.clone();
    trailing.push(0);
    assert!(matches!(
        IProgArtifact::decode_payload(&trailing),
        Err(IProgError::TrailingPayloadBytes(1))
    ));
    let mut unknown = payload;
    unknown[32] = 0xff;
    assert!(matches!(
        IProgArtifact::decode_payload(&unknown),
        Err(IProgError::UnknownTag(0xff))
    ));
    let wrong_kind = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("ic.query").expect("kind valid"),
        IPROG_SCHEMA_VERSION,
        program.canonical_payload().expect("program must encode"),
    );
    assert!(matches!(
        IProgArtifact::from_envelope(&wrong_kind),
        Err(IProgError::UnexpectedArtifactKind { .. })
    ));
    let wrong_schema = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new(IPROG_ARTIFACT_KIND).expect("kind valid"),
        IPROG_SCHEMA_VERSION + 1,
        program.canonical_payload().expect("program must encode"),
    );
    assert!(matches!(
        IProgArtifact::from_envelope(&wrong_schema),
        Err(IProgError::UnsupportedSchemaVersion(_))
    ));
}

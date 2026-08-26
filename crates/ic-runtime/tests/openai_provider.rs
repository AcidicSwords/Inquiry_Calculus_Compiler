use std::{
    io::{Read, Write},
    net::TcpListener,
    path::PathBuf,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use ic_core::{
    ArtifactEnvelope, ArtifactRef, BackendRequest, BoundaryRef, ProbeOperatorRef, QueryRef,
    RawReturn, SurfacePlanRef,
};
use ic_runtime::{
    OPENAI_DECODED_TEXT_ARTIFACT_KIND, OpenAiDecodedText, OpenAiDecodedTextCheckError,
    OpenAiHttpResponse, OpenAiHttpResponseError, OpenAiProviderError, OpenAiResponseDecodeError,
    OpenAiResponsesProvider, ProbeProvider, decode_openai_json_array_response,
    materialize_openai_decoded_texts,
};
use ic_store::ArtifactStore;

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn request(body: ArtifactRef) -> BackendRequest {
    BackendRequest::new(
        ProbeOperatorRef::from_artifact_ref(artifact(1)),
        SurfacePlanRef::from_artifact_ref(artifact(2)),
        QueryRef::from_artifact_ref(artifact(3)),
        BoundaryRef::from_artifact_ref(artifact(4)),
        artifact(5),
        artifact(6),
        artifact(7),
        artifact(8),
        body,
    )
}

fn read_http_request(stream: &mut std::net::TcpStream) -> Vec<u8> {
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("read timeout must configure");
    let mut bytes = Vec::new();
    let mut buffer = [0_u8; 4096];
    loop {
        let count = stream.read(&mut buffer).expect("request bytes must read");
        assert!(count > 0, "client closed before request completed");
        bytes.extend_from_slice(&buffer[..count]);
        let Some(header_end) = bytes.windows(4).position(|window| window == b"\r\n\r\n") else {
            continue;
        };
        let header_end = header_end + 4;
        let headers = std::str::from_utf8(&bytes[..header_end]).expect("headers must be UTF-8");
        let content_length = headers
            .lines()
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse::<usize>().expect("length must parse"))
            })
            .expect("request must carry content length");
        if bytes.len() >= header_end + content_length {
            return bytes;
        }
    }
}

#[test]
fn openai_adapter_sends_exact_addressed_json_and_returns_opaque_response_bytes() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("local listener must bind");
    let address = listener
        .local_addr()
        .expect("listener address must resolve");
    let expected_request = br#"{"model":"fixture-model","input":"two candidates"}"#.to_vec();
    let expected_response = br#"{"id":"resp_fixture","status":"completed","output":[]}"#.to_vec();
    let server_request = expected_request.clone();
    let server_response = expected_response.clone();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("provider must connect");
        let bytes = read_http_request(&mut stream);
        let header_end = bytes
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .expect("request must end headers")
            + 4;
        let headers = std::str::from_utf8(&bytes[..header_end]).expect("headers must decode");
        assert!(
            headers
                .lines()
                .any(|line| { line.eq_ignore_ascii_case("authorization: Bearer fixture-secret") })
        );
        assert_eq!(&bytes[header_end..], server_request);
        write!(
            stream,
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            server_response.len()
        )
        .expect("response headers must write");
        stream
            .write_all(&server_response)
            .expect("response body must write");
    });

    let request_body_ref = artifact(0x20);
    let mut provider = OpenAiResponsesProvider::with_endpoint(
        "fixture-secret".to_owned(),
        format!("http://{address}/v1/responses"),
        request_body_ref,
        expected_request,
        Duration::from_secs(5),
    )
    .expect("local provider must configure");
    let returned = provider
        .dispatch(&request(request_body_ref))
        .expect("exact addressed request must dispatch");
    let framed = OpenAiHttpResponse::decode(returned.as_bytes())
        .expect("provider response framing must decode");
    assert_eq!(framed.status(), 200);
    assert_eq!(framed.body(), expected_response);
    server.join().expect("local provider server must finish");

    let error = provider
        .dispatch(&request(artifact(0x21)))
        .expect_err("another request body must reject before transport");
    assert!(matches!(
        error,
        OpenAiProviderError::RequestBodyMismatch { .. }
    ));
    assert!(!format!("{error:?}").contains("fixture-secret"));

    assert!(matches!(
        OpenAiHttpResponse::decode(b"not-an-openai-return"),
        Err(OpenAiHttpResponseError::WrongDomain)
    ));
    let mut truncated = OpenAiHttpResponse::new(200, vec![1, 2, 3])
        .encode()
        .expect("fixture response must encode");
    truncated.pop();
    assert!(matches!(
        OpenAiHttpResponse::decode(&truncated),
        Err(OpenAiHttpResponseError::BodyLengthMismatch { .. })
    ));
}

#[test]
fn openai_adapter_preserves_non_success_status_and_body_as_actual_return() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("local listener must bind");
    let address = listener
        .local_addr()
        .expect("listener address must resolve");
    let request_body = br#"{"model":"fixture-model","input":"fixture"}"#.to_vec();
    let response_body = br#"{"error":{"message":"fixture unauthorized"}}"#.to_vec();
    let server_response = response_body.clone();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("provider must connect");
        let _request = read_http_request(&mut stream);
        write!(
            stream,
            "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            server_response.len()
        )
        .expect("response headers must write");
        stream
            .write_all(&server_response)
            .expect("response body must write");
    });

    let request_body_ref = artifact(0x30);
    let mut provider = OpenAiResponsesProvider::with_endpoint(
        "fixture-secret".to_owned(),
        format!("http://{address}/v1/responses"),
        request_body_ref,
        request_body,
        Duration::from_secs(5),
    )
    .expect("local provider must configure");
    let returned = provider
        .dispatch(&request(request_body_ref))
        .expect("an HTTP error response is still an actual provider return");
    let framed = OpenAiHttpResponse::decode(returned.as_bytes())
        .expect("provider response framing must decode");
    assert_eq!(framed.status(), 401);
    assert_eq!(framed.body(), response_body);
    server.join().expect("local provider server must finish");
}

fn framed(status: u16, body: serde_json::Value) -> Vec<u8> {
    OpenAiHttpResponse::new(
        status,
        serde_json::to_vec(&body).expect("fixture JSON must encode"),
    )
    .encode()
    .expect("fixture transport return must encode")
}

#[test]
fn responses_decoder_scans_heterogeneous_output_and_preserves_every_candidate() {
    let bytes = framed(
        200,
        serde_json::json!({
            "id": "resp_fixture",
            "model": "fixture-model-2026-08-01",
            "status": "completed",
            "output": [
                {"type": "reasoning", "id": "reasoning_fixture", "summary": []},
                {
                    "type": "message",
                    "id": "message_one",
                    "content": [
                        {"type": "output_text", "text": "[\"alpha\",\"beta\"]"}
                    ]
                },
                {
                    "type": "message",
                    "id": "message_two",
                    "content": [
                        {"type": "output_text", "text": "[\"gamma\"]"}
                    ]
                }
            ]
        }),
    );
    let decoded = decode_openai_json_array_response(&bytes)
        .expect("every message output_text array must be decoded");
    assert_eq!(decoded.response_id(), "resp_fixture");
    assert_eq!(decoded.model(), "fixture-model-2026-08-01");
    assert_eq!(decoded.candidates(), ["alpha", "beta", "gamma"]);
}

#[test]
fn responses_decoder_keeps_transport_parse_completion_and_candidate_failures_distinct() {
    assert!(matches!(
        decode_openai_json_array_response(b"not-a-frame"),
        Err(OpenAiResponseDecodeError::TransportFrame(
            OpenAiHttpResponseError::WrongDomain
        ))
    ));
    assert!(matches!(
        decode_openai_json_array_response(&framed(401, serde_json::json!({"error": {}}))),
        Err(OpenAiResponseDecodeError::HttpStatus(401))
    ));

    let invalid_json = OpenAiHttpResponse::new(200, b"not-json".to_vec())
        .encode()
        .expect("fixture transport return must encode");
    assert!(matches!(
        decode_openai_json_array_response(&invalid_json),
        Err(OpenAiResponseDecodeError::Json(_))
    ));
    assert!(matches!(
        decode_openai_json_array_response(&framed(
            200,
            serde_json::json!({
                "id": "resp_pending",
                "model": "fixture-model",
                "status": "in_progress",
                "output": []
            })
        )),
        Err(OpenAiResponseDecodeError::ResponseNotCompleted(status)) if status == "in_progress"
    ));
    assert!(matches!(
        decode_openai_json_array_response(&framed(
            200,
            serde_json::json!({
                "id": "resp_no_text",
                "model": "fixture-model",
                "status": "completed",
                "output": [{"type": "reasoning", "summary": []}]
            })
        )),
        Err(OpenAiResponseDecodeError::NoCandidateOutputText)
    ));
    assert!(matches!(
        decode_openai_json_array_response(&framed(
            200,
            serde_json::json!({
                "id": "resp_bad_array",
                "model": "fixture-model",
                "status": "completed",
                "output": [{
                    "type": "message",
                    "content": [{"type": "output_text", "text": "not-json"}]
                }]
            })
        )),
        Err(OpenAiResponseDecodeError::InvalidCandidateArray(_))
    ));
    assert!(matches!(
        decode_openai_json_array_response(&framed(
            200,
            serde_json::json!({
                "id": "resp_duplicate",
                "model": "fixture-model",
                "status": "completed",
                "output": [{
                    "type": "message",
                    "content": [{"type": "output_text", "text": "[\"same\",\"same\"]"}]
                }]
            })
        )),
        Err(OpenAiResponseDecodeError::DuplicateCandidate(candidate)) if candidate == "same"
    ));
}

#[test]
fn decoded_text_values_regenerate_exactly_from_raw_return_and_decoder_version() {
    let raw = RawReturn::new(framed(
        200,
        serde_json::json!({
            "id": "resp_materialize",
            "model": "fixture-model",
            "status": "completed",
            "output": [{
                "type": "message",
                "content": [{"type": "output_text", "text": "[\"alpha\",\"beta\"]"}]
            }]
        }),
    ));
    let raw_ref = raw.raw_return_ref().expect("raw return must encode");
    let decoder_version = artifact(0x41);
    let values = materialize_openai_decoded_texts(raw_ref, &raw, decoder_version)
        .expect("valid response must materialize");
    assert_eq!(values.len(), 2);
    assert_eq!(values[0].candidate_ordinal(), 0);
    assert_eq!(values[0].text(), "alpha");
    assert_eq!(values[1].candidate_ordinal(), 1);
    assert_eq!(values[1].text(), "beta");
    assert_eq!(values[0].raw_return(), raw_ref);
    assert_eq!(values[0].decoder_version(), decoder_version);
    assert_eq!(
        values[0].referenced_artifacts(),
        [raw_ref.as_artifact_ref(), decoder_version]
    );

    let regenerated = materialize_openai_decoded_texts(raw_ref, &raw, decoder_version)
        .expect("same roots must regenerate");
    assert_eq!(regenerated, values);
    assert_eq!(
        regenerated[0]
            .artifact_ref()
            .expect("regenerated value must address"),
        values[0].artifact_ref().expect("value must address")
    );
    for value in &values {
        value.check(&raw).expect("materialized value must replay");
        let envelope = value.envelope().expect("value must encode");
        assert_eq!(envelope.kind().as_str(), OPENAI_DECODED_TEXT_ARTIFACT_KIND);
        assert_eq!(
            OpenAiDecodedText::from_envelope(&envelope).expect("value must decode"),
            *value
        );
    }

    let different_version = materialize_openai_decoded_texts(raw_ref, &raw, artifact(0x42))
        .expect("another decoder version must materialize");
    assert_ne!(
        different_version[0]
            .artifact_ref()
            .expect("other version must address"),
        values[0].artifact_ref().expect("value must address")
    );
    let forged = OpenAiDecodedText::new(raw_ref, decoder_version, 0, "beta".to_owned());
    assert!(matches!(
        forged.check(&raw),
        Err(OpenAiDecodedTextCheckError::CandidateTextMismatch { ordinal: 0 })
    ));
    let wrong_kind = ArtifactEnvelope::from_canonical_payload(
        ic_core::ArtifactKind::new("fixture.wrong-kind").expect("fixture kind must be valid"),
        1,
        Vec::new(),
    );
    assert!(OpenAiDecodedText::from_envelope(&wrong_kind).is_err());
}

fn test_database_path() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time must follow epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ic-openai-decoded-text-{}-{nonce}.sqlite",
        std::process::id()
    ))
}

#[tokio::test]
async fn decoded_text_values_cold_regenerate_from_stored_roots() {
    let database_path = test_database_path();
    let database_url = format!("sqlite://{}?mode=rwc", database_path.display());
    let store = ArtifactStore::open(&database_url)
        .await
        .expect("file-backed store must open");
    store.migrate().await.expect("migrations must apply");
    let decoder_version_value = ArtifactEnvelope::from_canonical_payload(
        ic_core::ArtifactKind::new("ic.openai-json-array-decoder-version")
            .expect("decoder version kind must be valid"),
        1,
        b"fixture-v1".to_vec(),
    );
    let decoder_version = store
        .insert(&decoder_version_value)
        .await
        .expect("decoder version must persist");
    let raw = RawReturn::new(framed(
        200,
        serde_json::json!({
            "id": "resp_cold_replay",
            "model": "fixture-model",
            "status": "completed",
            "output": [{
                "type": "message",
                "content": [{"type": "output_text", "text": "[\"north\",\"south\"]"}]
            }]
        }),
    ));
    let raw_ref = ic_core::RawReturnRef::from_artifact_ref(
        store
            .insert(&raw.envelope().expect("raw return must encode"))
            .await
            .expect("raw return must persist"),
    );
    let values = materialize_openai_decoded_texts(raw_ref, &raw, decoder_version)
        .expect("values must materialize");
    let mut value_refs = Vec::new();
    for value in &values {
        value_refs.push(
            store
                .insert_referencing(
                    &value.envelope().expect("decoded value must encode"),
                    &value.referenced_artifacts(),
                )
                .await
                .expect("decoded value dependencies must persist"),
        );
    }
    store.close().await;

    let reopened = ArtifactStore::open(&database_url)
        .await
        .expect("file-backed store must reopen");
    reopened
        .migrate()
        .await
        .expect("embedded migrations must remain repeatable");
    let stored_raw = reopened
        .get(raw_ref.as_artifact_ref())
        .await
        .expect("raw return lookup must work")
        .expect("raw return must remain stored");
    let stored_raw = RawReturn::from_envelope(&stored_raw).expect("raw return must decode");
    let regenerated = materialize_openai_decoded_texts(raw_ref, &stored_raw, decoder_version)
        .expect("stored roots must regenerate every decoded value");
    let regenerated_refs = regenerated
        .iter()
        .map(OpenAiDecodedText::artifact_ref)
        .collect::<Result<Vec<_>, _>>()
        .expect("regenerated values must address");
    assert_eq!(regenerated_refs, value_refs);
    for (reference, regenerated_value) in value_refs.iter().zip(&regenerated) {
        let stored = reopened
            .get(*reference)
            .await
            .expect("decoded value lookup must work")
            .expect("decoded value must remain stored");
        assert_eq!(
            OpenAiDecodedText::from_envelope(&stored).expect("stored value must decode"),
            *regenerated_value
        );
        regenerated_value
            .check(&stored_raw)
            .expect("stored value must recheck against stored raw return");
    }
    reopened.close().await;
    std::fs::remove_file(&database_path).expect("test database must be removable after close");
}

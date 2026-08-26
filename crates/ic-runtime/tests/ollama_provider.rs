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
    OLLAMA_DECODED_TEXT_ARTIFACT_KIND, OllamaDecodedText, OllamaDecodedTextCheckError,
    OllamaGenerateProvider, OllamaHttpResponse, OllamaHttpResponseError, OllamaProviderError,
    OllamaResponseDecodeError, ProbeProvider, decode_ollama_candidate_response,
    materialize_ollama_decoded_texts,
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

fn framed(status: u16, body: serde_json::Value) -> Vec<u8> {
    OllamaHttpResponse::new(
        status,
        serde_json::to_vec(&body).expect("fixture JSON must encode"),
    )
    .encode()
    .expect("fixture transport return must encode")
}

#[test]
fn ollama_adapter_sends_exact_addressed_json_and_preserves_actual_return() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("local listener must bind");
    let address = listener
        .local_addr()
        .expect("listener address must resolve");
    let expected_request = br#"{"model":"fixture-model","stream":false}"#.to_vec();
    let expected_response = br#"{"model":"fixture-model","response":"{\"candidates\":[\"alpha\",\"beta\"]}","done":true,"done_reason":"stop"}"#.to_vec();
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
            !headers
                .lines()
                .any(|line| line.to_ascii_lowercase().starts_with("authorization:")),
            "the local provider must not manufacture an authorization header"
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
    let mut provider = OllamaGenerateProvider::with_endpoint(
        format!("http://{address}/api/generate"),
        request_body_ref,
        expected_request,
        Duration::from_secs(5),
    )
    .expect("local provider must configure");
    let returned = provider
        .dispatch(&request(request_body_ref))
        .expect("exact addressed request must dispatch");
    let response = OllamaHttpResponse::decode(returned.as_bytes())
        .expect("provider response framing must decode");
    assert_eq!(response.status(), 200);
    assert_eq!(response.body(), expected_response);
    let decoded = decode_ollama_candidate_response(returned.as_bytes())
        .expect("interpretation after the return must preserve all candidates");
    assert_eq!(decoded.model(), "fixture-model");
    assert_eq!(decoded.done_reason(), "stop");
    assert_eq!(decoded.candidates(), ["alpha", "beta"]);
    server.join().expect("local provider server must finish");

    assert!(matches!(
        provider.dispatch(&request(artifact(0x21))),
        Err(OllamaProviderError::RequestBodyMismatch { .. })
    ));
}

#[test]
fn ollama_adapter_preserves_http_failure_as_actual_provider_return() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("local listener must bind");
    let address = listener
        .local_addr()
        .expect("listener address must resolve");
    let request_body = br#"{"model":"missing-model","stream":false}"#.to_vec();
    let response_body = br#"{"error":"model not found"}"#.to_vec();
    let server_response = response_body.clone();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("provider must connect");
        let _request = read_http_request(&mut stream);
        write!(
            stream,
            "HTTP/1.1 404 Not Found\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            server_response.len()
        )
        .expect("response headers must write");
        stream
            .write_all(&server_response)
            .expect("response body must write");
    });

    let request_body_ref = artifact(0x30);
    let mut provider = OllamaGenerateProvider::with_endpoint(
        format!("http://{address}/api/generate"),
        request_body_ref,
        request_body,
        Duration::from_secs(5),
    )
    .expect("local provider must configure");
    let returned = provider
        .dispatch(&request(request_body_ref))
        .expect("an HTTP failure remains an actual provider return");
    let response = OllamaHttpResponse::decode(returned.as_bytes())
        .expect("provider response framing must decode");
    assert_eq!(response.status(), 404);
    assert_eq!(response.body(), response_body);
    assert!(matches!(
        decode_ollama_candidate_response(returned.as_bytes()),
        Err(OllamaResponseDecodeError::HttpStatus(404))
    ));
    server.join().expect("local provider server must finish");
}

#[test]
fn ollama_decoder_keeps_transport_completion_and_candidate_failures_distinct() {
    assert!(matches!(
        decode_ollama_candidate_response(b"not-an-ollama-return"),
        Err(OllamaResponseDecodeError::TransportFrame(
            OllamaHttpResponseError::WrongDomain
        ))
    ));
    let invalid_json = OllamaHttpResponse::new(200, b"not-json".to_vec())
        .encode()
        .expect("fixture transport return must encode");
    assert!(matches!(
        decode_ollama_candidate_response(&invalid_json),
        Err(OllamaResponseDecodeError::Json(_))
    ));
    assert!(matches!(
        decode_ollama_candidate_response(&framed(
            200,
            serde_json::json!({
                "model": "fixture-model",
                "response": "{\"candidates\":[\"alpha\"]}",
                "done": false,
                "done_reason": "load"
            })
        )),
        Err(OllamaResponseDecodeError::ResponseNotDone)
    ));
    assert!(matches!(
        decode_ollama_candidate_response(&framed(
            200,
            serde_json::json!({
                "model": "fixture-model",
                "response": "not-json",
                "done": true,
                "done_reason": "stop"
            })
        )),
        Err(OllamaResponseDecodeError::InvalidGeneratedJson(_))
    ));
    assert!(matches!(
        decode_ollama_candidate_response(&framed(
            200,
            serde_json::json!({
                "model": "fixture-model",
                "response": "{\"candidates\":[\"same\",\"same\"]}",
                "done": true,
                "done_reason": "stop"
            })
        )),
        Err(OllamaResponseDecodeError::DuplicateCandidate(candidate)) if candidate == "same"
    ));

    let mut truncated = OllamaHttpResponse::new(200, vec![1, 2, 3])
        .encode()
        .expect("fixture response must encode");
    truncated.pop();
    assert!(matches!(
        OllamaHttpResponse::decode(&truncated),
        Err(OllamaHttpResponseError::BodyLengthMismatch { .. })
    ));
}

#[test]
fn ollama_decoded_values_replay_exact_candidate_identity() {
    let raw = RawReturn::new(framed(
        200,
        serde_json::json!({
            "model": "qwen3.5:9b",
            "response": "{\"candidates\":[\"north\",\"south\"]}",
            "done": true,
            "done_reason": "stop"
        }),
    ));
    let raw_ref = raw.raw_return_ref().expect("raw return must address");
    let decoder_version = artifact(0x41);
    let values = materialize_ollama_decoded_texts(raw_ref, &raw, decoder_version)
        .expect("valid local return must materialize");
    assert_eq!(values.len(), 2);
    assert_eq!(values[0].text(), "north");
    assert_eq!(values[1].text(), "south");
    assert_eq!(values[0].candidate_ordinal(), 0);
    assert_eq!(values[0].raw_return(), raw_ref);
    assert_eq!(values[0].decoder_version(), decoder_version);
    assert_eq!(
        values[0].referenced_artifacts(),
        [raw_ref.as_artifact_ref(), decoder_version]
    );
    for value in &values {
        value.check(&raw).expect("decoded value must replay");
        let envelope = value.envelope().expect("decoded value must encode");
        assert_eq!(envelope.kind().as_str(), OLLAMA_DECODED_TEXT_ARTIFACT_KIND);
        assert_eq!(
            OllamaDecodedText::from_envelope(&envelope).expect("decoded value must decode"),
            *value
        );
    }
    let regenerated = materialize_ollama_decoded_texts(raw_ref, &raw, decoder_version)
        .expect("same roots must regenerate");
    assert_eq!(regenerated, values);
    let other_version = materialize_ollama_decoded_texts(raw_ref, &raw, artifact(0x42))
        .expect("another decoder version must materialize");
    assert_ne!(
        other_version[0]
            .artifact_ref()
            .expect("other version must address"),
        values[0].artifact_ref().expect("value must address")
    );
    let forged = OllamaDecodedText::new(raw_ref, decoder_version, 0, "south".to_owned());
    assert!(matches!(
        forged.check(&raw),
        Err(OllamaDecodedTextCheckError::CandidateTextMismatch { ordinal: 0 })
    ));
}

fn test_database_path() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time must follow epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ic-ollama-decoded-text-{}-{nonce}.sqlite",
        std::process::id()
    ))
}

#[tokio::test]
async fn ollama_decoded_values_cold_regenerate_from_stored_roots() {
    let database_path = test_database_path();
    let database_url = format!("sqlite://{}?mode=rwc", database_path.display());
    let store = ArtifactStore::open(&database_url)
        .await
        .expect("file-backed store must open");
    store.migrate().await.expect("migrations must apply");
    let decoder_version_value = ArtifactEnvelope::from_canonical_payload(
        ic_core::ArtifactKind::new("ic.ollama-schema-decoder-version")
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
            "model": "qwen3.5:9b",
            "response": "{\"candidates\":[\"alpha\",\"beta\"]}",
            "done": true,
            "done_reason": "stop"
        }),
    ));
    let raw_ref = ic_core::RawReturnRef::from_artifact_ref(
        store
            .insert(&raw.envelope().expect("raw return must encode"))
            .await
            .expect("raw return must persist"),
    );
    let values = materialize_ollama_decoded_texts(raw_ref, &raw, decoder_version)
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
    let regenerated = materialize_ollama_decoded_texts(raw_ref, &stored_raw, decoder_version)
        .expect("stored roots must regenerate every decoded value");
    let regenerated_refs = regenerated
        .iter()
        .map(OllamaDecodedText::artifact_ref)
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
            OllamaDecodedText::from_envelope(&stored).expect("stored value must decode"),
            *regenerated_value
        );
        regenerated_value
            .check(&stored_raw)
            .expect("stored value must recheck against stored raw return");
    }
    reopened.close().await;
    std::fs::remove_file(&database_path).expect("test database must be removable after close");
}

use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
    time::Duration,
};

use ic_core::{
    ArtifactRef, BackendRequest, BoundaryRef, ProbeOperatorRef, QueryRef, SurfacePlanRef,
};
use ic_runtime::{
    OpenAiHttpResponse, OpenAiHttpResponseError, OpenAiProviderError, OpenAiResponseDecodeError,
    OpenAiResponsesProvider, ProbeProvider, decode_openai_json_array_response,
};

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

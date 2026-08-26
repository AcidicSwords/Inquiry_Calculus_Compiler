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
    OllamaGenerateProvider, OllamaHttpResponse, OllamaHttpResponseError, OllamaProviderError,
    OllamaResponseDecodeError, ProbeProvider, decode_ollama_candidate_response,
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

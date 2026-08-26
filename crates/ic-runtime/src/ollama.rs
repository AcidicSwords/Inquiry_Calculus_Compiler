//! Ollama `/api/generate` transport behind the existing probe-provider boundary.
//!
//! The adapter sends one exact, content-addressed JSON body and returns a versioned HTTP frame as
//! opaque provider bytes. Decoding is a separate post-actuality operation and creates no support,
//! standing, continuation binding, or warrant.

use std::{collections::BTreeSet, time::Duration};

use ic_core::{ArtifactRef, BackendRequest};
use thiserror::Error;

use crate::{ProbeProvider, ProviderReturn};

/// Default local Ollama non-streaming generation endpoint.
pub const OLLAMA_GENERATE_ENDPOINT: &str = "http://127.0.0.1:11434/api/generate";
const OLLAMA_HTTP_RESPONSE_DOMAIN: &[u8] = b"inquiry-calculus:ollama-http-response\0";
const OLLAMA_HTTP_RESPONSE_VERSION: u16 = 1;

/// Versioned exact HTTP return from the local Ollama provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OllamaHttpResponse {
    status: u16,
    body: Vec<u8>,
}

impl OllamaHttpResponse {
    #[must_use]
    pub const fn new(status: u16, body: Vec<u8>) -> Self {
        Self { status, body }
    }

    #[must_use]
    pub const fn status(&self) -> u16 {
        self.status
    }

    #[must_use]
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    pub fn encode(&self) -> Result<Vec<u8>, OllamaHttpResponseError> {
        let body_len = u64::try_from(self.body.len())
            .map_err(|_| OllamaHttpResponseError::BodyLengthOverflow)?;
        let mut encoded =
            Vec::with_capacity(OLLAMA_HTTP_RESPONSE_DOMAIN.len() + 12 + self.body.len());
        encoded.extend_from_slice(OLLAMA_HTTP_RESPONSE_DOMAIN);
        encoded.extend_from_slice(&OLLAMA_HTTP_RESPONSE_VERSION.to_be_bytes());
        encoded.extend_from_slice(&self.status.to_be_bytes());
        encoded.extend_from_slice(&body_len.to_be_bytes());
        encoded.extend_from_slice(&self.body);
        Ok(encoded)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, OllamaHttpResponseError> {
        let Some(remainder) = bytes.strip_prefix(OLLAMA_HTTP_RESPONSE_DOMAIN) else {
            return Err(OllamaHttpResponseError::WrongDomain);
        };
        let version = u16::from_be_bytes(
            remainder
                .get(..2)
                .ok_or(OllamaHttpResponseError::Truncated)?
                .try_into()
                .map_err(|_| OllamaHttpResponseError::Truncated)?,
        );
        if version != OLLAMA_HTTP_RESPONSE_VERSION {
            return Err(OllamaHttpResponseError::UnsupportedVersion(version));
        }
        let status = u16::from_be_bytes(
            remainder
                .get(2..4)
                .ok_or(OllamaHttpResponseError::Truncated)?
                .try_into()
                .map_err(|_| OllamaHttpResponseError::Truncated)?,
        );
        let body_len = u64::from_be_bytes(
            remainder
                .get(4..12)
                .ok_or(OllamaHttpResponseError::Truncated)?
                .try_into()
                .map_err(|_| OllamaHttpResponseError::Truncated)?,
        );
        let body_len =
            usize::try_from(body_len).map_err(|_| OllamaHttpResponseError::BodyLengthOverflow)?;
        let body = remainder
            .get(12..)
            .ok_or(OllamaHttpResponseError::Truncated)?;
        if body.len() != body_len {
            return Err(OllamaHttpResponseError::BodyLengthMismatch {
                declared: body_len,
                actual: body.len(),
            });
        }
        Ok(Self::new(status, body.to_vec()))
    }
}

/// A parsed local-model return that is not yet a semantically supported answer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodedOllamaCandidates {
    model: String,
    done_reason: String,
    candidates: Vec<String>,
}

impl DecodedOllamaCandidates {
    #[must_use]
    pub fn model(&self) -> &str {
        &self.model
    }

    #[must_use]
    pub fn done_reason(&self) -> &str {
        &self.done_reason
    }

    #[must_use]
    pub fn candidates(&self) -> &[String] {
        &self.candidates
    }
}

/// Decodes the schema-constrained `{ "candidates": [..] }` string inside an Ollama response.
pub fn decode_ollama_candidate_response(
    bytes: &[u8],
) -> Result<DecodedOllamaCandidates, OllamaResponseDecodeError> {
    let response = OllamaHttpResponse::decode(bytes)?;
    if !(200..300).contains(&response.status()) {
        return Err(OllamaResponseDecodeError::HttpStatus(response.status()));
    }
    let value: serde_json::Value = serde_json::from_slice(response.body())?;
    let object = value
        .as_object()
        .ok_or(OllamaResponseDecodeError::ResponseIsNotObject)?;
    if object.get("done").and_then(serde_json::Value::as_bool) != Some(true) {
        return Err(OllamaResponseDecodeError::ResponseNotDone);
    }
    let model = required_string(object, "model")?;
    let done_reason = required_string(object, "done_reason")?;
    let generated = required_string(object, "response")?;
    let generated: serde_json::Value = serde_json::from_str(&generated)
        .map_err(OllamaResponseDecodeError::InvalidGeneratedJson)?;
    let candidates = generated
        .get("candidates")
        .and_then(serde_json::Value::as_array)
        .ok_or(OllamaResponseDecodeError::MissingCandidates)?;
    if candidates.is_empty() {
        return Err(OllamaResponseDecodeError::EmptyCandidates);
    }
    let mut decoded = Vec::with_capacity(candidates.len());
    let mut seen = BTreeSet::new();
    for candidate in candidates {
        let candidate = candidate
            .as_str()
            .ok_or(OllamaResponseDecodeError::CandidateIsNotString)?
            .to_owned();
        if candidate.is_empty() {
            return Err(OllamaResponseDecodeError::EmptyCandidate);
        }
        if !seen.insert(candidate.clone()) {
            return Err(OllamaResponseDecodeError::DuplicateCandidate(candidate));
        }
        decoded.push(candidate);
    }
    Ok(DecodedOllamaCandidates {
        model,
        done_reason,
        candidates: decoded,
    })
}

fn required_string(
    object: &serde_json::Map<String, serde_json::Value>,
    field: &'static str,
) -> Result<String, OllamaResponseDecodeError> {
    object
        .get(field)
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .ok_or(OllamaResponseDecodeError::MissingString(field))
}

/// One narrow synchronous local Ollama provider.
pub struct OllamaGenerateProvider {
    agent: ureq::Agent,
    endpoint: String,
    request_body_ref: ArtifactRef,
    request_body: Vec<u8>,
}

impl OllamaGenerateProvider {
    pub fn new(
        request_body_ref: ArtifactRef,
        request_body: Vec<u8>,
    ) -> Result<Self, OllamaProviderError> {
        Self::with_endpoint(
            OLLAMA_GENERATE_ENDPOINT.to_owned(),
            request_body_ref,
            request_body,
            Duration::from_secs(120),
        )
    }

    pub fn with_endpoint(
        endpoint: String,
        request_body_ref: ArtifactRef,
        request_body: Vec<u8>,
        timeout: Duration,
    ) -> Result<Self, OllamaProviderError> {
        if endpoint.is_empty() {
            return Err(OllamaProviderError::EmptyEndpoint);
        }
        if request_body.is_empty() {
            return Err(OllamaProviderError::EmptyRequestBody);
        }
        let config = ureq::Agent::config_builder()
            .timeout_global(Some(timeout))
            .http_status_as_error(false)
            .build();
        Ok(Self {
            agent: ureq::Agent::new_with_config(config),
            endpoint,
            request_body_ref,
            request_body,
        })
    }
}

impl ProbeProvider for OllamaGenerateProvider {
    type Error = OllamaProviderError;

    fn dispatch(&mut self, request: &BackendRequest) -> Result<ProviderReturn, Self::Error> {
        if request.request_body() != self.request_body_ref {
            return Err(OllamaProviderError::RequestBodyMismatch {
                expected: self.request_body_ref,
                actual: request.request_body(),
            });
        }
        let mut response = self
            .agent
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .send(&self.request_body)?;
        let status = response.status().as_u16();
        let body = response.body_mut().read_to_vec()?;
        Ok(ProviderReturn::new(
            OllamaHttpResponse::new(status, body).encode()?,
        ))
    }
}

#[derive(Debug, Error)]
pub enum OllamaProviderError {
    #[error("Ollama generate endpoint is empty")]
    EmptyEndpoint,
    #[error("Ollama request body is empty")]
    EmptyRequestBody,
    #[error("backend request body {actual} differs from configured provider body {expected}")]
    RequestBodyMismatch {
        expected: ArtifactRef,
        actual: ArtifactRef,
    },
    #[error("Ollama transport failed")]
    Transport(#[from] ureq::Error),
    #[error(transparent)]
    ResponseEncoding(#[from] OllamaHttpResponseError),
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum OllamaHttpResponseError {
    #[error("Ollama HTTP response framing has the wrong domain")]
    WrongDomain,
    #[error("Ollama HTTP response framing is truncated")]
    Truncated,
    #[error("Ollama HTTP response framing uses unsupported version {0}")]
    UnsupportedVersion(u16),
    #[error("Ollama HTTP response body length overflows this platform")]
    BodyLengthOverflow,
    #[error("Ollama HTTP response declares body length {declared}, but carries {actual} bytes")]
    BodyLengthMismatch { declared: usize, actual: usize },
}

#[derive(Debug, Error)]
pub enum OllamaResponseDecodeError {
    #[error(transparent)]
    TransportFrame(#[from] OllamaHttpResponseError),
    #[error("Ollama returned HTTP status {0}")]
    HttpStatus(u16),
    #[error("Ollama response body is not valid JSON")]
    Json(#[from] serde_json::Error),
    #[error("Ollama response body is not a JSON object")]
    ResponseIsNotObject,
    #[error("Ollama response is not complete")]
    ResponseNotDone,
    #[error("Ollama response has no string field {0:?}")]
    MissingString(&'static str),
    #[error("Ollama generated response is not valid JSON")]
    InvalidGeneratedJson(serde_json::Error),
    #[error("Ollama generated response has no candidates array")]
    MissingCandidates,
    #[error("Ollama generated candidate array is empty")]
    EmptyCandidates,
    #[error("Ollama generated candidate is not a string")]
    CandidateIsNotString,
    #[error("Ollama generated candidate is empty")]
    EmptyCandidate,
    #[error("Ollama generated response repeats candidate {0:?}")]
    DuplicateCandidate(String),
}

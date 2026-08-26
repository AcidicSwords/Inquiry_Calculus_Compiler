//! OpenAI Responses API transport behind the existing probe-provider boundary.
//!
//! The adapter sends one already content-addressed JSON request body and returns the complete
//! response body as opaque bytes. It performs no decoding, candidate selection, support check, or
//! warrant. `dispatch_probe` remains responsible for durable preparation and for committing those
//! bytes as ordinary actuality before any caller can interpret them.

use std::{env, time::Duration};

use ic_core::{ArtifactRef, BackendRequest};
use thiserror::Error;

use crate::{ProbeProvider, ProviderReturn};

/// Official OpenAI Responses endpoint.
pub const OPENAI_RESPONSES_ENDPOINT: &str = "https://api.openai.com/v1/responses";
const OPENAI_HTTP_RESPONSE_DOMAIN: &[u8] = b"inquiry-calculus:openai-http-response\0";
const OPENAI_HTTP_RESPONSE_VERSION: u16 = 1;

/// Versioned provider-transport return preserved inside the opaque `RawReturn` bytes.
///
/// HTTP status is consequential transport actuality: a 4xx/5xx response is preserved rather than
/// collapsed into a transport exception or semantic `Unknown`. The response body remains exact
/// bytes and is not parsed here.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiHttpResponse {
    status: u16,
    body: Vec<u8>,
}

impl OpenAiHttpResponse {
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

    pub fn encode(&self) -> Result<Vec<u8>, OpenAiHttpResponseError> {
        let body_len = u64::try_from(self.body.len())
            .map_err(|_| OpenAiHttpResponseError::BodyLengthOverflow)?;
        let mut encoded =
            Vec::with_capacity(OPENAI_HTTP_RESPONSE_DOMAIN.len() + 12 + self.body.len());
        encoded.extend_from_slice(OPENAI_HTTP_RESPONSE_DOMAIN);
        encoded.extend_from_slice(&OPENAI_HTTP_RESPONSE_VERSION.to_be_bytes());
        encoded.extend_from_slice(&self.status.to_be_bytes());
        encoded.extend_from_slice(&body_len.to_be_bytes());
        encoded.extend_from_slice(&self.body);
        Ok(encoded)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, OpenAiHttpResponseError> {
        let Some(remainder) = bytes.strip_prefix(OPENAI_HTTP_RESPONSE_DOMAIN) else {
            return Err(OpenAiHttpResponseError::WrongDomain);
        };
        let version = u16::from_be_bytes(
            remainder
                .get(..2)
                .ok_or(OpenAiHttpResponseError::Truncated)?
                .try_into()
                .map_err(|_| OpenAiHttpResponseError::Truncated)?,
        );
        if version != OPENAI_HTTP_RESPONSE_VERSION {
            return Err(OpenAiHttpResponseError::UnsupportedVersion(version));
        }
        let status = u16::from_be_bytes(
            remainder
                .get(2..4)
                .ok_or(OpenAiHttpResponseError::Truncated)?
                .try_into()
                .map_err(|_| OpenAiHttpResponseError::Truncated)?,
        );
        let body_len = u64::from_be_bytes(
            remainder
                .get(4..12)
                .ok_or(OpenAiHttpResponseError::Truncated)?
                .try_into()
                .map_err(|_| OpenAiHttpResponseError::Truncated)?,
        );
        let body_len =
            usize::try_from(body_len).map_err(|_| OpenAiHttpResponseError::BodyLengthOverflow)?;
        let body = remainder
            .get(12..)
            .ok_or(OpenAiHttpResponseError::Truncated)?;
        if body.len() != body_len {
            return Err(OpenAiHttpResponseError::BodyLengthMismatch {
                declared: body_len,
                actual: body.len(),
            });
        }
        Ok(Self::new(status, body.to_vec()))
    }
}

/// One narrow synchronous Responses API provider.
///
/// The API key is private and this type intentionally has no `Debug` implementation.
pub struct OpenAiResponsesProvider {
    agent: ureq::Agent,
    endpoint: String,
    api_key: String,
    request_body_ref: ArtifactRef,
    request_body: Vec<u8>,
}

impl OpenAiResponsesProvider {
    /// Uses the official endpoint and a bounded whole-call timeout.
    pub fn new(
        api_key: String,
        request_body_ref: ArtifactRef,
        request_body: Vec<u8>,
    ) -> Result<Self, OpenAiProviderError> {
        Self::with_endpoint(
            api_key,
            OPENAI_RESPONSES_ENDPOINT.to_owned(),
            request_body_ref,
            request_body,
            Duration::from_secs(120),
        )
    }

    /// Loads `OPENAI_API_KEY` without placing its value in an error or return record.
    pub fn from_env(
        request_body_ref: ArtifactRef,
        request_body: Vec<u8>,
    ) -> Result<Self, OpenAiProviderError> {
        let api_key = env::var("OPENAI_API_KEY").map_err(|_| OpenAiProviderError::MissingApiKey)?;
        Self::new(api_key, request_body_ref, request_body)
    }

    /// Supplies an explicit endpoint for deterministic local transport tests.
    pub fn with_endpoint(
        api_key: String,
        endpoint: String,
        request_body_ref: ArtifactRef,
        request_body: Vec<u8>,
        timeout: Duration,
    ) -> Result<Self, OpenAiProviderError> {
        if api_key.is_empty() || api_key.chars().any(char::is_control) {
            return Err(OpenAiProviderError::InvalidApiKey);
        }
        if endpoint.is_empty() {
            return Err(OpenAiProviderError::EmptyEndpoint);
        }
        if request_body.is_empty() {
            return Err(OpenAiProviderError::EmptyRequestBody);
        }
        let config = ureq::Agent::config_builder()
            .timeout_global(Some(timeout))
            .http_status_as_error(false)
            .build();
        Ok(Self {
            agent: ureq::Agent::new_with_config(config),
            endpoint,
            api_key,
            request_body_ref,
            request_body,
        })
    }
}

impl ProbeProvider for OpenAiResponsesProvider {
    type Error = OpenAiProviderError;

    fn dispatch(&mut self, request: &BackendRequest) -> Result<ProviderReturn, Self::Error> {
        if request.request_body() != self.request_body_ref {
            return Err(OpenAiProviderError::RequestBodyMismatch {
                expected: self.request_body_ref,
                actual: request.request_body(),
            });
        }
        let authorization = format!("Bearer {}", self.api_key);
        let mut response = self
            .agent
            .post(&self.endpoint)
            .header("Authorization", &authorization)
            .header("Content-Type", "application/json")
            .send(&self.request_body)?;
        let status = response.status().as_u16();
        let bytes = response.body_mut().read_to_vec()?;
        let framed = OpenAiHttpResponse::new(status, bytes).encode()?;
        Ok(ProviderReturn::new(framed))
    }
}

#[derive(Debug, Error)]
pub enum OpenAiProviderError {
    #[error("OPENAI_API_KEY is unavailable")]
    MissingApiKey,
    #[error("OpenAI API key is empty or contains invalid control characters")]
    InvalidApiKey,
    #[error("OpenAI Responses endpoint is empty")]
    EmptyEndpoint,
    #[error("OpenAI Responses request body is empty")]
    EmptyRequestBody,
    #[error("backend request body {actual} differs from configured provider body {expected}")]
    RequestBodyMismatch {
        expected: ArtifactRef,
        actual: ArtifactRef,
    },
    #[error("OpenAI Responses transport failed")]
    Transport(#[from] ureq::Error),
    #[error(transparent)]
    ResponseEncoding(#[from] OpenAiHttpResponseError),
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum OpenAiHttpResponseError {
    #[error("OpenAI HTTP response framing has the wrong domain")]
    WrongDomain,
    #[error("OpenAI HTTP response framing is truncated")]
    Truncated,
    #[error("OpenAI HTTP response framing uses unsupported version {0}")]
    UnsupportedVersion(u16),
    #[error("OpenAI HTTP response body length overflows this platform")]
    BodyLengthOverflow,
    #[error("OpenAI HTTP response declares body length {declared}, but carries {actual} bytes")]
    BodyLengthMismatch { declared: usize, actual: usize },
}

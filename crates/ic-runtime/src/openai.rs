//! OpenAI Responses API transport behind the existing probe-provider boundary.
//!
//! The adapter sends one already content-addressed JSON request body and returns the complete
//! response body as opaque bytes. It performs no decoding, candidate selection, support check, or
//! warrant. `dispatch_probe` remains responsible for durable preparation and for committing those
//! bytes as ordinary actuality before any caller can interpret them.

use std::{collections::BTreeSet, env, time::Duration};

use ic_core::{
    ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, BackendRequest, RawReturn,
    RawReturnError, RawReturnRef,
};
use thiserror::Error;

use crate::{ProbeProvider, ProviderReturn};

/// Official OpenAI Responses endpoint.
pub const OPENAI_RESPONSES_ENDPOINT: &str = "https://api.openai.com/v1/responses";
/// Canonical artifact kind for one deterministically decoded provider string value.
pub const OPENAI_DECODED_TEXT_ARTIFACT_KIND: &str = "ic.openai-decoded-text";
/// Schema version for one deterministically decoded provider string value.
pub const OPENAI_DECODED_TEXT_SCHEMA_VERSION: u32 = 1;
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

/// A decoded, but not yet semantically supported, Responses API JSON-array return.
///
/// Candidate order and exact UTF-8 text are retained. Construction proves only that the committed
/// HTTP response matches this narrow decoder contract; it does not construct typed forms, relation
/// uses, support, standing, a continuation binding, or warrant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodedOpenAiJsonArray {
    response_id: String,
    model: String,
    candidates: Vec<String>,
}

impl DecodedOpenAiJsonArray {
    #[must_use]
    pub fn response_id(&self) -> &str {
        &self.response_id
    }

    #[must_use]
    pub fn model(&self) -> &str {
        &self.model
    }

    #[must_use]
    pub fn candidates(&self) -> &[String] {
        &self.candidates
    }
}

/// One content-addressed text value regenerated from an exact raw return and decoder version.
///
/// This is the opaque represented-form artifact used by a later `TypedForm`; it is not itself a
/// typed form, completion candidate, support assertion, or warrant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiDecodedText {
    raw_return: RawReturnRef,
    decoder_version: ArtifactRef,
    candidate_ordinal: u32,
    text: String,
}

impl OpenAiDecodedText {
    #[must_use]
    pub const fn new(
        raw_return: RawReturnRef,
        decoder_version: ArtifactRef,
        candidate_ordinal: u32,
        text: String,
    ) -> Self {
        Self {
            raw_return,
            decoder_version,
            candidate_ordinal,
            text,
        }
    }

    #[must_use]
    pub const fn raw_return(&self) -> RawReturnRef {
        self.raw_return
    }

    #[must_use]
    pub const fn decoder_version(&self) -> ArtifactRef {
        self.decoder_version
    }

    #[must_use]
    pub const fn candidate_ordinal(&self) -> u32 {
        self.candidate_ordinal
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn canonical_payload(&self) -> Result<Vec<u8>, OpenAiDecodedTextError> {
        let text_len = u32::try_from(self.text.len())
            .map_err(|_| OpenAiDecodedTextError::TextTooLong(self.text.len()))?;
        let mut payload = Vec::with_capacity(72 + self.text.len());
        payload.extend_from_slice(self.raw_return.as_artifact_ref().as_bytes());
        payload.extend_from_slice(self.decoder_version.as_bytes());
        payload.extend_from_slice(&self.candidate_ordinal.to_be_bytes());
        payload.extend_from_slice(&text_len.to_be_bytes());
        payload.extend_from_slice(self.text.as_bytes());
        Ok(payload)
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, OpenAiDecodedTextError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(OPENAI_DECODED_TEXT_ARTIFACT_KIND)?,
            OPENAI_DECODED_TEXT_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }

    pub fn artifact_ref(&self) -> Result<ArtifactRef, OpenAiDecodedTextError> {
        Ok(self.envelope()?.artifact_ref()?)
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, OpenAiDecodedTextError> {
        if envelope.kind().as_str() != OPENAI_DECODED_TEXT_ARTIFACT_KIND {
            return Err(OpenAiDecodedTextError::UnexpectedArtifactKind {
                expected: OPENAI_DECODED_TEXT_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != OPENAI_DECODED_TEXT_SCHEMA_VERSION {
            return Err(OpenAiDecodedTextError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        let payload = envelope.canonical_payload();
        if payload.len() < 72 {
            return Err(OpenAiDecodedTextError::TruncatedPayload);
        }
        let raw_return = RawReturnRef::from_artifact_ref(ArtifactRef::from_bytes(
            payload[0..32]
                .try_into()
                .map_err(|_| OpenAiDecodedTextError::TruncatedPayload)?,
        ));
        let decoder_version = ArtifactRef::from_bytes(
            payload[32..64]
                .try_into()
                .map_err(|_| OpenAiDecodedTextError::TruncatedPayload)?,
        );
        let candidate_ordinal = u32::from_be_bytes(
            payload[64..68]
                .try_into()
                .map_err(|_| OpenAiDecodedTextError::TruncatedPayload)?,
        );
        let text_len = u32::from_be_bytes(
            payload[68..72]
                .try_into()
                .map_err(|_| OpenAiDecodedTextError::TruncatedPayload)?,
        );
        let text_len =
            usize::try_from(text_len).map_err(|_| OpenAiDecodedTextError::TextLengthOverflow)?;
        let text_bytes = payload
            .get(72..)
            .ok_or(OpenAiDecodedTextError::TruncatedPayload)?;
        if text_bytes.len() != text_len {
            return Err(OpenAiDecodedTextError::TextLengthMismatch {
                declared: text_len,
                actual: text_bytes.len(),
            });
        }
        let text = String::from_utf8(text_bytes.to_vec())
            .map_err(|_| OpenAiDecodedTextError::InvalidTextUtf8)?;
        Ok(Self::new(
            raw_return,
            decoder_version,
            candidate_ordinal,
            text,
        ))
    }

    #[must_use]
    pub const fn referenced_artifacts(&self) -> [ArtifactRef; 2] {
        [self.raw_return.as_artifact_ref(), self.decoder_version]
    }

    /// Replays this value from the exact raw return and decoder contract.
    pub fn check(&self, raw_return: &RawReturn) -> Result<(), OpenAiDecodedTextCheckError> {
        let calculated = raw_return.raw_return_ref()?;
        if calculated != self.raw_return {
            return Err(OpenAiDecodedTextCheckError::RawReturnIdentityMismatch {
                expected: self.raw_return,
                actual: calculated,
            });
        }
        let decoded = decode_openai_json_array_response(raw_return.bytes())?;
        let ordinal = usize::try_from(self.candidate_ordinal)
            .map_err(|_| OpenAiDecodedTextCheckError::CandidateOrdinalOverflow)?;
        let actual = decoded.candidates().get(ordinal).ok_or(
            OpenAiDecodedTextCheckError::CandidateOrdinalMissing {
                ordinal: self.candidate_ordinal,
                count: decoded.candidates().len(),
            },
        )?;
        if actual != &self.text {
            return Err(OpenAiDecodedTextCheckError::CandidateTextMismatch {
                ordinal: self.candidate_ordinal,
            });
        }
        Ok(())
    }
}

/// Decodes every `output_text` content item whose exact text is a JSON array of strings.
///
/// Responses output is heterogeneous, so this scans the complete output array rather than
/// assuming the first item is a message. Every decoded string survives in provider order.
pub fn decode_openai_json_array_response(
    bytes: &[u8],
) -> Result<DecodedOpenAiJsonArray, OpenAiResponseDecodeError> {
    let response = OpenAiHttpResponse::decode(bytes)?;
    if !(200..300).contains(&response.status()) {
        return Err(OpenAiResponseDecodeError::HttpStatus(response.status()));
    }
    let value: serde_json::Value = serde_json::from_slice(response.body())?;
    let object = value
        .as_object()
        .ok_or(OpenAiResponseDecodeError::ResponseIsNotObject)?;
    let status = object
        .get("status")
        .and_then(serde_json::Value::as_str)
        .ok_or(OpenAiResponseDecodeError::MissingStatus)?;
    if status != "completed" {
        return Err(OpenAiResponseDecodeError::ResponseNotCompleted(
            status.to_owned(),
        ));
    }
    let response_id = required_string(object, "id")?;
    let model = required_string(object, "model")?;
    let output = object
        .get("output")
        .and_then(serde_json::Value::as_array)
        .ok_or(OpenAiResponseDecodeError::MissingOutput)?;

    let mut candidates = Vec::new();
    let mut seen = BTreeSet::new();
    for item in output {
        if item.get("type").and_then(serde_json::Value::as_str) != Some("message") {
            continue;
        }
        let content = item
            .get("content")
            .and_then(serde_json::Value::as_array)
            .ok_or(OpenAiResponseDecodeError::MessageMissingContent)?;
        for part in content {
            if part.get("type").and_then(serde_json::Value::as_str) != Some("output_text") {
                continue;
            }
            let text = part
                .get("text")
                .and_then(serde_json::Value::as_str)
                .ok_or(OpenAiResponseDecodeError::OutputTextMissingText)?;
            let decoded: Vec<String> = serde_json::from_str(text)
                .map_err(OpenAiResponseDecodeError::InvalidCandidateArray)?;
            if decoded.is_empty() {
                return Err(OpenAiResponseDecodeError::EmptyCandidateArray);
            }
            for candidate in decoded {
                if candidate.is_empty() {
                    return Err(OpenAiResponseDecodeError::EmptyCandidate);
                }
                if !seen.insert(candidate.clone()) {
                    return Err(OpenAiResponseDecodeError::DuplicateCandidate(candidate));
                }
                candidates.push(candidate);
            }
        }
    }
    if candidates.is_empty() {
        return Err(OpenAiResponseDecodeError::NoCandidateOutputText);
    }
    Ok(DecodedOpenAiJsonArray {
        response_id,
        model,
        candidates,
    })
}

/// Deterministically materializes all text values from one preserved raw provider return.
pub fn materialize_openai_decoded_texts(
    raw_return: RawReturnRef,
    raw: &RawReturn,
    decoder_version: ArtifactRef,
) -> Result<Vec<OpenAiDecodedText>, OpenAiDecodedTextCheckError> {
    let calculated = raw.raw_return_ref()?;
    if calculated != raw_return {
        return Err(OpenAiDecodedTextCheckError::RawReturnIdentityMismatch {
            expected: raw_return,
            actual: calculated,
        });
    }
    let decoded = decode_openai_json_array_response(raw.bytes())?;
    decoded
        .candidates
        .into_iter()
        .enumerate()
        .map(|(ordinal, text)| {
            let ordinal = u32::try_from(ordinal)
                .map_err(|_| OpenAiDecodedTextCheckError::CandidateOrdinalOverflow)?;
            Ok(OpenAiDecodedText::new(
                raw_return,
                decoder_version,
                ordinal,
                text,
            ))
        })
        .collect()
}

fn required_string(
    object: &serde_json::Map<String, serde_json::Value>,
    field: &'static str,
) -> Result<String, OpenAiResponseDecodeError> {
    object
        .get(field)
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .ok_or(OpenAiResponseDecodeError::MissingString(field))
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

#[derive(Debug, Error)]
pub enum OpenAiResponseDecodeError {
    #[error(transparent)]
    TransportFrame(#[from] OpenAiHttpResponseError),
    #[error("OpenAI returned HTTP status {0}")]
    HttpStatus(u16),
    #[error("OpenAI response body is not valid JSON")]
    Json(#[from] serde_json::Error),
    #[error("OpenAI response body is not a JSON object")]
    ResponseIsNotObject,
    #[error("OpenAI response has no string status")]
    MissingStatus,
    #[error("OpenAI response status is {0:?}, not completed")]
    ResponseNotCompleted(String),
    #[error("OpenAI response has no output array")]
    MissingOutput,
    #[error("OpenAI response message has no content array")]
    MessageMissingContent,
    #[error("OpenAI output_text content has no string text")]
    OutputTextMissingText,
    #[error("OpenAI response has no JSON-array output_text completion")]
    NoCandidateOutputText,
    #[error("OpenAI output_text is not a JSON array of strings")]
    InvalidCandidateArray(serde_json::Error),
    #[error("OpenAI output_text candidate array is empty")]
    EmptyCandidateArray,
    #[error("OpenAI output_text contains an empty candidate")]
    EmptyCandidate,
    #[error("OpenAI output_text repeats candidate {0:?}")]
    DuplicateCandidate(String),
    #[error("OpenAI response has no string field {0:?}")]
    MissingString(&'static str),
}

#[derive(Debug, Error)]
pub enum OpenAiDecodedTextError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("OpenAI decoded text is too long: {0} UTF-8 bytes")]
    TextTooLong(usize),
    #[error("OpenAI decoded text payload is truncated")]
    TruncatedPayload,
    #[error("OpenAI decoded text length overflows this platform")]
    TextLengthOverflow,
    #[error("OpenAI decoded text declares {declared} bytes but carries {actual}")]
    TextLengthMismatch { declared: usize, actual: usize },
    #[error("OpenAI decoded text bytes are not valid UTF-8")]
    InvalidTextUtf8,
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported OpenAI decoded text schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

#[derive(Debug, Error)]
pub enum OpenAiDecodedTextCheckError {
    #[error(transparent)]
    RawReturn(#[from] RawReturnError),
    #[error(transparent)]
    Decode(#[from] OpenAiResponseDecodeError),
    #[error("raw return identity is {actual}, not expected {expected}")]
    RawReturnIdentityMismatch {
        expected: RawReturnRef,
        actual: RawReturnRef,
    },
    #[error("candidate ordinal cannot be represented on this platform")]
    CandidateOrdinalOverflow,
    #[error("candidate ordinal {ordinal} is absent from decoded set of size {count}")]
    CandidateOrdinalMissing { ordinal: u32, count: usize },
    #[error("candidate text at ordinal {ordinal} differs from the decoded raw return")]
    CandidateTextMismatch { ordinal: u32 },
}

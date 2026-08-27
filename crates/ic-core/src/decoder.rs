//! Finite exact decoder contracts and their actual-event results.
//!
//! A [`FiniteDecoder`] is the first executable-free realization of the canonical partial decoder
//! relation `delta_q : Y_q -> P_+(A(q))`. It is a caller-declared finite table: a listed nonempty
//! candidate set is a route-supported decoded result, a listed undefined return is an explicit
//! decoder failure, and an unlisted raw return remains `Unknown`. It neither dispatches a probe
//! nor admits the table as complete, standing, or warranted.

use std::{collections::BTreeSet, fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ActualEvent, ActualEventCatalog, ActualEventCheckError, ActualEventError, ArtifactEnvelope,
    ArtifactError, ArtifactKind, ArtifactRef, CompletionCandidateCatalog,
    CompletionCandidateCheckError, CompletionCandidateError, CompletionCandidateRef, DecoderRef,
    EventRef, OpenQueryCheckError, OpenQueryError, ProbeOperatorError, QueryRef, RawReturnCatalog,
    RawReturnError, RawReturnRef, RelationUse, RelationUseCheckError, RelationUseError,
    RelationUseRef, ResolutionCatalog, ResolutionPathCheckError, ResolutionPathError,
    ResolutionPathIR, ResolutionPathRef, TypeCatalog, TypeCheckError, TypeError, TypeRef,
    TypeSymbol, check_actual_event,
};

/// Canonical artifact kind for a declared finite exact decoder table.
pub const FINITE_DECODER_ARTIFACT_KIND: &str = "ic.finite-decoder";
/// Payload schema version for finite exact decoder tables.
pub const FINITE_DECODER_SCHEMA_VERSION: u32 = 1;

/// Stable identity for a declared finite exact decoder table.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FiniteDecoderRef(ArtifactRef);

impl FiniteDecoderRef {
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }

    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }

    /// The resolution-path spelling for this decoder contract.
    #[must_use]
    pub const fn as_decoder_ref(self) -> DecoderRef {
        DecoderRef::from_artifact_ref(self.0)
    }
}

impl fmt::Display for FiniteDecoderRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for FiniteDecoderRef {
    type Err = ArtifactError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// One exact finite decoder row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FiniteDecoderEntry {
    /// This exact raw return decodes to a nonempty, possibly ambiguous candidate set.
    Decoded {
        raw_return: RawReturnRef,
        candidates: Vec<CompletionCandidateRef>,
    },
    /// This exact raw return lies outside the decoder's partial domain.
    Undefined { raw_return: RawReturnRef },
}

impl FiniteDecoderEntry {
    #[must_use]
    pub const fn raw_return(&self) -> RawReturnRef {
        match self {
            Self::Decoded { raw_return, .. } | Self::Undefined { raw_return } => *raw_return,
        }
    }
}

/// A typed finite partial decoder for one semantic question.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteDecoder {
    query: QueryRef,
    input: TypeRef,
    entries: Vec<FiniteDecoderEntry>,
}

/// One finite decoder lookup, preserving all three semantically distinct outcomes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FiniteDecoderOutcome {
    /// A listed nonempty candidate set; this is route support, not a standing or warranted claim.
    Decoded(Vec<CompletionCandidateRef>),
    /// A listed result at which this partial decoder is explicitly undefined.
    Undefined,
    /// No table entry. A finite table is not assumed exhaustive.
    Unknown,
}

/// A decoded candidate set linked to its named ordinary event record and direct decoder route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodedCandidateSet {
    event: EventRef,
    query: QueryRef,
    decoder: FiniteDecoderRef,
    path: ResolutionPathRef,
    candidates: Vec<CompletionCandidateRef>,
}

/// A derived structural link from one decoded completion candidate to one declared observation use.
///
/// This view establishes only that the complete candidate named by a checked finite decoder result
/// spells the same relation occurrence and context as the declared use. It does not execute or
/// evaluate that relation, establish actual dispatch, admit its support, or establish standing,
/// incompatibility, or departure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodedObservationUse {
    decoded: DecodedCandidateSet,
    candidate: CompletionCandidateRef,
    observation: RelationUseRef,
}

impl DecodedCandidateSet {
    #[must_use]
    pub const fn event(&self) -> EventRef {
        self.event
    }
    #[must_use]
    pub const fn query(&self) -> QueryRef {
        self.query
    }
    #[must_use]
    pub const fn decoder(&self) -> FiniteDecoderRef {
        self.decoder
    }
    #[must_use]
    pub const fn path(&self) -> ResolutionPathRef {
        self.path
    }
    #[must_use]
    pub fn candidates(&self) -> &[CompletionCandidateRef] {
        &self.candidates
    }
}

impl DecodedObservationUse {
    #[must_use]
    pub const fn decoded(&self) -> &DecodedCandidateSet {
        &self.decoded
    }

    #[must_use]
    pub const fn candidate(&self) -> CompletionCandidateRef {
        self.candidate
    }

    #[must_use]
    pub const fn observation(&self) -> RelationUseRef {
        self.observation
    }
}

/// The result of applying a checked finite decoder to a checked ordinary event record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActualDecodeResult {
    Decoded(DecodedCandidateSet),
    Undefined {
        event: EventRef,
        query: QueryRef,
        decoder: FiniteDecoderRef,
        path: ResolutionPathRef,
    },
    Unknown {
        event: EventRef,
        query: QueryRef,
        decoder: FiniteDecoderRef,
        path: ResolutionPathRef,
    },
}

/// The catalog needed to check finite decoder entries.
pub trait FiniteDecoderCatalog: CompletionCandidateCatalog + RawReturnCatalog {
    fn resolve_finite_decoder(&self, reference: FiniteDecoderRef) -> Option<FiniteDecoder>;
}

/// The combined catalog needed to resolve a finite decoder over an ordinary actual event.
pub trait ActualDecodeCatalog:
    ActualEventCatalog + FiniteDecoderCatalog + ResolutionCatalog
{
}

impl<T> ActualDecodeCatalog for T where
    T: ActualEventCatalog + FiniteDecoderCatalog + ResolutionCatalog
{
}

/// The catalog needed to connect a decoder result with one declared relation occurrence.
pub trait ObservationResultCatalog: ActualDecodeCatalog {
    fn resolve_relation_use(&self, reference: RelationUseRef) -> Option<RelationUse>;
}

/// Checks the exact structural correspondence between one decoded candidate and one relation use.
///
/// The candidate must be among `decoded`'s preserved alternatives. Its complete binding assignment
/// and the use's binding assignment must agree exactly, as must the source query's relation,
/// scope, applicability, grain, horizon, discharge mode, and warrant. The observation's support
/// route may be constructed after the raw return and is checked independently by answer/departure
/// admission; requiring it to equal the pre-dispatch query support would create a content-address
/// cycle. The result is a checked derived view, never a new artifact or a claim that the relation
/// was observed, true, accepted, or a departure witness.
pub fn match_decoded_observation_use<C: ObservationResultCatalog>(
    decoded: &DecodedCandidateSet,
    candidate_ref: CompletionCandidateRef,
    observation_ref: RelationUseRef,
    catalog: &C,
) -> Result<DecodedObservationUse, DecodedObservationError> {
    if !decoded.candidates.contains(&candidate_ref) {
        return Err(DecodedObservationError::CandidateNotDecoded {
            candidate: candidate_ref,
            event: decoded.event,
        });
    }
    let candidate = catalog
        .resolve_completion_candidate(candidate_ref)
        .ok_or(DecodedObservationError::UnresolvedCandidate(candidate_ref))?;
    let calculated_candidate = candidate.completion_candidate_ref()?;
    if calculated_candidate != candidate_ref {
        return Err(DecodedObservationError::CandidateIdentityMismatch {
            reference: candidate_ref,
            calculated: calculated_candidate,
        });
    }
    candidate.check(catalog)?;
    if candidate.source() != decoded.query {
        return Err(DecodedObservationError::CandidateQueryMismatch {
            decoded: decoded.query,
            candidate: candidate.source(),
        });
    }

    let query = crate::OpenQueryCatalog::resolve_open_query(catalog, decoded.query)
        .ok_or(DecodedObservationError::UnresolvedQuery(decoded.query))?;
    let calculated_query = query.query_ref()?;
    if calculated_query != decoded.query {
        return Err(DecodedObservationError::QueryIdentityMismatch {
            reference: decoded.query,
            calculated: calculated_query,
        });
    }
    query.check(catalog)?;

    let observation = catalog.resolve_relation_use(observation_ref).ok_or(
        DecodedObservationError::UnresolvedObservation(observation_ref),
    )?;
    let calculated_observation = observation.relation_use_ref()?;
    if calculated_observation != observation_ref {
        return Err(DecodedObservationError::ObservationIdentityMismatch {
            reference: observation_ref,
            calculated: calculated_observation,
        });
    }
    observation.check(catalog)?;
    if observation.relation() != query.relation() {
        return Err(DecodedObservationError::RelationMismatch {
            query: query.relation(),
            observation: observation.relation(),
        });
    }
    if !same_bindings(candidate.bindings(), observation.bindings()) {
        return Err(DecodedObservationError::BindingMismatch);
    }
    if observation.scope() != query.context().scope()
        || observation.applicability() != query.context().applicability()
        || observation.grain() != query.context().grain()
        || observation.horizon() != query.context().horizon()
        || observation.mode() != query.context().mode()
        || observation.warrant() != query.context().warrant()
    {
        return Err(DecodedObservationError::ContextMismatch);
    }
    Ok(DecodedObservationUse {
        decoded: decoded.clone(),
        candidate: candidate_ref,
        observation: observation_ref,
    })
}

fn same_bindings(candidate: &[crate::PortBinding], observation: &[crate::PortBinding]) -> bool {
    candidate.len() == observation.len()
        && candidate.iter().all(|expected| {
            observation.iter().any(|actual| {
                actual.port() == expected.port() && actual.value() == expected.value()
            })
        })
}

impl FiniteDecoder {
    /// Constructs a finite partial decoder and canonicalizes its raw-return and candidate order.
    pub fn new(
        query: QueryRef,
        input: TypeRef,
        mut entries: Vec<FiniteDecoderEntry>,
    ) -> Result<Self, FiniteDecoderError> {
        for entry in &mut entries {
            if let FiniteDecoderEntry::Decoded { candidates, .. } = entry {
                if candidates.is_empty() {
                    return Err(FiniteDecoderError::EmptyDecodedSet);
                }
                candidates.sort_unstable();
                for pair in candidates.windows(2) {
                    if pair[0] == pair[1] {
                        return Err(FiniteDecoderError::DuplicateCandidate(pair[0]));
                    }
                }
            }
        }
        entries.sort_unstable_by_key(FiniteDecoderEntry::raw_return);
        for pair in entries.windows(2) {
            if pair[0].raw_return() == pair[1].raw_return() {
                return Err(FiniteDecoderError::DuplicateRawReturn(pair[0].raw_return()));
            }
        }
        Ok(Self {
            query,
            input,
            entries,
        })
    }

    #[must_use]
    pub const fn query(&self) -> QueryRef {
        self.query
    }
    #[must_use]
    pub const fn input(&self) -> TypeRef {
        self.input
    }
    #[must_use]
    pub fn entries(&self) -> &[FiniteDecoderEntry] {
        &self.entries
    }

    pub fn canonical_payload(&self) -> Result<Vec<u8>, FiniteDecoderError> {
        let mut encoded = Vec::new();
        reference(&mut encoded, self.query.as_artifact_ref());
        reference(&mut encoded, self.input.as_artifact_ref());
        count(&mut encoded, self.entries.len())?;
        for entry in &self.entries {
            match entry {
                FiniteDecoderEntry::Decoded {
                    raw_return,
                    candidates,
                } => {
                    encoded.push(0);
                    reference(&mut encoded, raw_return.as_artifact_ref());
                    count(&mut encoded, candidates.len())?;
                    for candidate in candidates {
                        reference(&mut encoded, candidate.as_artifact_ref());
                    }
                }
                FiniteDecoderEntry::Undefined { raw_return } => {
                    encoded.push(1);
                    reference(&mut encoded, raw_return.as_artifact_ref());
                }
            }
        }
        Ok(encoded)
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, FiniteDecoderError> {
        let mut cursor = Cursor::new(payload);
        let query = QueryRef::from_artifact_ref(cursor.reference()?);
        let input = TypeRef::from_artifact_ref(cursor.reference()?);
        let entry_count = cursor.count()?;
        let mut entries = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            let tag = cursor.byte()?;
            let raw_return = RawReturnRef::from_artifact_ref(cursor.reference()?);
            let entry = match tag {
                0 => {
                    let candidate_count = cursor.count()?;
                    let mut candidates = Vec::with_capacity(candidate_count);
                    for _ in 0..candidate_count {
                        candidates.push(CompletionCandidateRef::from_artifact_ref(
                            cursor.reference()?,
                        ));
                    }
                    FiniteDecoderEntry::Decoded {
                        raw_return,
                        candidates,
                    }
                }
                1 => FiniteDecoderEntry::Undefined { raw_return },
                other => return Err(FiniteDecoderError::UnknownEntryTag(other)),
            };
            entries.push(entry);
        }
        if !cursor.finished() {
            return Err(FiniteDecoderError::TrailingPayloadBytes(cursor.remaining()));
        }
        let decoded = Self::new(query, input, entries.clone())?;
        if decoded.entries != entries {
            return Err(FiniteDecoderError::NonCanonicalEntryOrder);
        }
        Ok(decoded)
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, FiniteDecoderError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(FINITE_DECODER_ARTIFACT_KIND)?,
            FINITE_DECODER_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }

    pub fn finite_decoder_ref(&self) -> Result<FiniteDecoderRef, FiniteDecoderError> {
        Ok(FiniteDecoderRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, FiniteDecoderError> {
        if envelope.kind().as_str() != FINITE_DECODER_ARTIFACT_KIND {
            return Err(FiniteDecoderError::UnexpectedArtifactKind {
                expected: FINITE_DECODER_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != FINITE_DECODER_SCHEMA_VERSION {
            return Err(FiniteDecoderError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    /// Revalidates the query, raw-return, and candidate identities named by this table.
    ///
    /// This proves neither that the table is exhaustive nor that its decoder was admitted or
    /// actually executed. Those are supplied only by a checked event result below.
    pub fn check<C: FiniteDecoderCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), FiniteDecoderCheckError> {
        let query = catalog
            .resolve_open_query(self.query)
            .ok_or(FiniteDecoderCheckError::UnresolvedQuery(self.query))?;
        let calculated = query.query_ref()?;
        if calculated != self.query {
            return Err(FiniteDecoderCheckError::QueryIdentityMismatch {
                reference: self.query,
                calculated,
            });
        }
        query.check(catalog)?;
        check_type(self.input, catalog)?;

        let mut seen = BTreeSet::new();
        for entry in &self.entries {
            if !seen.insert(entry.raw_return()) {
                return Err(FiniteDecoderCheckError::DuplicateRawReturn(
                    entry.raw_return(),
                ));
            }
            let raw_return = catalog.resolve_raw_return(entry.raw_return()).ok_or(
                FiniteDecoderCheckError::UnresolvedRawReturn(entry.raw_return()),
            )?;
            let calculated = raw_return.raw_return_ref()?;
            if calculated != entry.raw_return() {
                return Err(FiniteDecoderCheckError::RawReturnIdentityMismatch {
                    reference: entry.raw_return(),
                    calculated,
                });
            }
            if let FiniteDecoderEntry::Decoded { candidates, .. } = entry {
                if candidates.is_empty() {
                    return Err(FiniteDecoderCheckError::EmptyDecodedSet);
                }
                let mut candidate_seen = BTreeSet::new();
                for candidate_ref in candidates {
                    if !candidate_seen.insert(*candidate_ref) {
                        return Err(FiniteDecoderCheckError::DuplicateCandidate(*candidate_ref));
                    }
                    let candidate = catalog
                        .resolve_completion_candidate(*candidate_ref)
                        .ok_or(FiniteDecoderCheckError::UnresolvedCandidate(*candidate_ref))?;
                    let calculated = candidate.completion_candidate_ref()?;
                    if calculated != *candidate_ref {
                        return Err(FiniteDecoderCheckError::CandidateIdentityMismatch {
                            reference: *candidate_ref,
                            calculated,
                        });
                    }
                    candidate.check(catalog)?;
                    if candidate.source() != self.query {
                        return Err(FiniteDecoderCheckError::CandidateQueryMismatch {
                            decoder: self.query,
                            candidate: candidate.source(),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    /// Looks up a raw return without inferring that an omitted row is a decoder failure.
    #[must_use]
    pub fn outcome(&self, raw_return: RawReturnRef) -> FiniteDecoderOutcome {
        match self
            .entries
            .binary_search_by_key(&raw_return, FiniteDecoderEntry::raw_return)
        {
            Ok(index) => match &self.entries[index] {
                FiniteDecoderEntry::Decoded { candidates, .. } => {
                    FiniteDecoderOutcome::Decoded(candidates.clone())
                }
                FiniteDecoderEntry::Undefined { .. } => FiniteDecoderOutcome::Undefined,
            },
            Err(_) => FiniteDecoderOutcome::Unknown,
        }
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = vec![self.query.as_artifact_ref(), self.input.as_artifact_ref()];
        for entry in &self.entries {
            references.push(entry.raw_return().as_artifact_ref());
            if let FiniteDecoderEntry::Decoded { candidates, .. } = entry {
                references.extend(
                    candidates
                        .iter()
                        .map(|candidate| candidate.as_artifact_ref()),
                );
            }
        }
        references
    }
}

/// Applies one direct checked finite decoder route to an ordinary event record.
///
/// A `Decoded` outcome is linked to the explicitly retained event record, direct resolution path,
/// finite decoder, and checked candidate identities. It remains distinct from relation truth,
/// standing, checking, warrant, actual dispatch, or a positive departure certificate.
pub fn decode_actual_event<C: ActualDecodeCatalog>(
    event: &ActualEvent,
    decoder: &FiniteDecoder,
    path_ref: ResolutionPathRef,
    catalog: &C,
) -> Result<ActualDecodeResult, ActualDecodeError> {
    decode_actual_event_scoped(
        AnswerPortScope::SoleOpenPort,
        event,
        decoder,
        path_ref,
        catalog,
    )
}

/// Applies the same direct decoder route while typing its output against one named open port.
///
/// The decoded completions still range over the whole port field; only the route's declared output
/// type is checked against the named port's carrier, so a question with several open ports can be
/// decoded one port at a time without any port borrowing a sibling's carrier.
pub fn decode_actual_event_for_port<C: ActualDecodeCatalog>(
    port: &TypeSymbol,
    event: &ActualEvent,
    decoder: &FiniteDecoder,
    path_ref: ResolutionPathRef,
    catalog: &C,
) -> Result<ActualDecodeResult, ActualDecodeError> {
    decode_actual_event_scoped(
        AnswerPortScope::NamedPort(port),
        event,
        decoder,
        path_ref,
        catalog,
    )
}

/// Which open port's carrier a decoded route must land in.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AnswerPortScope<'a> {
    /// The question has exactly one open port: the existing single-answer specialization.
    SoleOpenPort,
    /// One named open port of a question that may have several.
    NamedPort(&'a TypeSymbol),
}

pub(crate) fn decode_actual_event_scoped<C: ActualDecodeCatalog>(
    scope: AnswerPortScope<'_>,
    event: &ActualEvent,
    decoder: &FiniteDecoder,
    path_ref: ResolutionPathRef,
    catalog: &C,
) -> Result<ActualDecodeResult, ActualDecodeError> {
    check_actual_event(event, catalog)?;
    decoder.check(catalog)?;
    let decoder_ref = decoder.finite_decoder_ref()?;
    let path = catalog
        .resolve_resolution_path(path_ref)
        .ok_or(ActualDecodeError::UnresolvedPath(path_ref))?;
    let calculated_path = path.resolution_path_ref()?;
    if calculated_path != path_ref {
        return Err(ActualDecodeError::PathIdentityMismatch {
            reference: path_ref,
            calculated: calculated_path,
        });
    }
    path.check(catalog)?;
    let ResolutionPathIR::Decode {
        decoder: route_decoder,
    } = path.path()
    else {
        return Err(ActualDecodeError::PathIsNotDirectDecoder(path_ref));
    };
    if route_decoder != decoder_ref.as_decoder_ref() {
        return Err(ActualDecodeError::PathDecoderMismatch {
            path: route_decoder,
            decoder: decoder_ref.as_decoder_ref(),
        });
    }
    if path.input() != decoder.input() {
        return Err(ActualDecodeError::PathInputMismatch {
            path: path.input(),
            decoder: decoder.input(),
        });
    }
    let operator = catalog
        .resolve_probe_operator(event.operator())
        .ok_or(ActualDecodeError::UnresolvedOperator(event.operator()))?;
    let calculated_operator = operator.probe_operator_ref()?;
    if calculated_operator != event.operator() {
        return Err(ActualDecodeError::OperatorIdentityMismatch {
            reference: event.operator(),
            calculated: calculated_operator,
        });
    }
    if operator.return_type() != decoder.input() {
        return Err(ActualDecodeError::EventReturnTypeMismatch {
            operator: operator.return_type(),
            decoder: decoder.input(),
        });
    }
    if event.question() != decoder.query() {
        return Err(ActualDecodeError::EventQueryMismatch {
            event: event.question(),
            decoder: decoder.query(),
        });
    }
    let query = ActualEventCatalog::resolve_open_query(catalog, decoder.query())
        .ok_or(ActualDecodeError::UnresolvedQuery(decoder.query()))?;
    let answer_port = match scope {
        AnswerPortScope::SoleOpenPort => {
            if query.open_ports().len() != 1 {
                return Err(ActualDecodeError::UnsupportedAnswerArity {
                    actual: query.open_ports().len(),
                });
            }
            query.open_ports()[0].port()
        }
        AnswerPortScope::NamedPort(port) => query
            .open_ports()
            .iter()
            .find(|open| open.port() == port)
            .ok_or_else(|| ActualDecodeError::ForeignAnswerPort(port.clone()))?
            .port(),
    };
    let schema = catalog
        .resolve_relation_schema(query.relation())
        .ok_or(ActualDecodeError::UnresolvedRelation(query.relation()))?;
    let expected_output = schema
        .ports()
        .iter()
        .find(|port| port.name() == answer_port)
        .expect("the checked open query has only schema ports")
        .ty();
    if path.output() != expected_output {
        return Err(ActualDecodeError::PathOutputMismatch {
            path: path.output(),
            answer: expected_output,
        });
    }
    let event_ref = event.event_ref()?;
    Ok(match decoder.outcome(event.raw_return()) {
        FiniteDecoderOutcome::Decoded(candidates) => {
            ActualDecodeResult::Decoded(DecodedCandidateSet {
                event: event_ref,
                query: decoder.query(),
                decoder: decoder_ref,
                path: path_ref,
                candidates,
            })
        }
        FiniteDecoderOutcome::Undefined => ActualDecodeResult::Undefined {
            event: event_ref,
            query: decoder.query(),
            decoder: decoder_ref,
            path: path_ref,
        },
        FiniteDecoderOutcome::Unknown => ActualDecodeResult::Unknown {
            event: event_ref,
            query: decoder.query(),
            decoder: decoder_ref,
            path: path_ref,
        },
    })
}

fn check_type<C: TypeCatalog>(
    reference_value: TypeRef,
    catalog: &C,
) -> Result<(), FiniteDecoderCheckError> {
    let ty = catalog.resolve_type(reference_value).ok_or(
        FiniteDecoderCheckError::UnresolvedInputType(reference_value),
    )?;
    let calculated = ty.type_ref()?;
    if calculated != reference_value {
        return Err(FiniteDecoderCheckError::InputTypeIdentityMismatch {
            reference: reference_value,
            calculated,
        });
    }
    ty.check(catalog)?;
    Ok(())
}

fn reference(encoded: &mut Vec<u8>, value: ArtifactRef) {
    encoded.extend_from_slice(value.as_bytes());
}

fn count(encoded: &mut Vec<u8>, value: usize) -> Result<(), FiniteDecoderError> {
    let value = u32::try_from(value).map_err(|_| FiniteDecoderError::CollectionTooLong(value))?;
    encoded.extend_from_slice(&value.to_be_bytes());
    Ok(())
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }
    fn take(&mut self, length: usize) -> Result<&'a [u8], FiniteDecoderError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(FiniteDecoderError::PayloadLengthOverflow)?;
        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(FiniteDecoderError::TruncatedPayload)?;
        self.position = end;
        Ok(bytes)
    }
    fn byte(&mut self) -> Result<u8, FiniteDecoderError> {
        Ok(self.take(1)?[0])
    }
    fn count(&mut self) -> Result<usize, FiniteDecoderError> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| FiniteDecoderError::TruncatedPayload)?;
        usize::try_from(u32::from_be_bytes(bytes))
            .map_err(|_| FiniteDecoderError::PayloadLengthOverflow)
    }
    fn reference(&mut self) -> Result<ArtifactRef, FiniteDecoderError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| FiniteDecoderError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(bytes))
    }
    const fn finished(&self) -> bool {
        self.position == self.bytes.len()
    }
    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

#[derive(Debug, Error)]
pub enum FiniteDecoderError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("finite decoder has a decoded row with no candidates")]
    EmptyDecodedSet,
    #[error("finite decoder names raw return {0} more than once")]
    DuplicateRawReturn(RawReturnRef),
    #[error("finite decoder names candidate {0} more than once in one row")]
    DuplicateCandidate(CompletionCandidateRef),
    #[error("finite decoder collection is too long: {0} entries")]
    CollectionTooLong(usize),
    #[error("finite-decoder payload is truncated")]
    TruncatedPayload,
    #[error("finite-decoder payload length overflows this platform")]
    PayloadLengthOverflow,
    #[error("finite-decoder payload contains {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("finite-decoder payload has an unknown row tag {0}")]
    UnknownEntryTag(u8),
    #[error("finite-decoder payload is not in canonical raw-return/candidate order")]
    NonCanonicalEntryOrder,
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported finite-decoder schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

#[derive(Debug, Error)]
pub enum FiniteDecoderCheckError {
    #[error(transparent)]
    Encoding(#[from] FiniteDecoderError),
    #[error(transparent)]
    Query(#[from] OpenQueryError),
    #[error(transparent)]
    QueryCheck(#[from] OpenQueryCheckError),
    #[error(transparent)]
    Candidate(#[from] CompletionCandidateError),
    #[error(transparent)]
    CandidateCheck(#[from] CompletionCandidateCheckError),
    #[error(transparent)]
    RawReturn(#[from] RawReturnError),
    #[error(transparent)]
    TypeArtifact(#[from] TypeError),
    #[error(transparent)]
    TypeCheck(#[from] TypeCheckError),
    #[error("open query {0} is unavailable")]
    UnresolvedQuery(QueryRef),
    #[error("query {reference} hashes to {calculated}, not its claimed identity")]
    QueryIdentityMismatch {
        reference: QueryRef,
        calculated: QueryRef,
    },
    #[error("input type {0} is unavailable")]
    UnresolvedInputType(TypeRef),
    #[error("input type {reference} hashes to {calculated}, not its claimed identity")]
    InputTypeIdentityMismatch {
        reference: TypeRef,
        calculated: TypeRef,
    },
    #[error("raw return {0} is unavailable")]
    UnresolvedRawReturn(RawReturnRef),
    #[error("raw return {reference} hashes to {calculated}, not its claimed identity")]
    RawReturnIdentityMismatch {
        reference: RawReturnRef,
        calculated: RawReturnRef,
    },
    #[error("finite decoder names raw return {0} more than once")]
    DuplicateRawReturn(RawReturnRef),
    #[error("finite decoder has a decoded row with no candidates")]
    EmptyDecodedSet,
    #[error("completion candidate {0} is unavailable")]
    UnresolvedCandidate(CompletionCandidateRef),
    #[error("completion candidate {reference} hashes to {calculated}, not its claimed identity")]
    CandidateIdentityMismatch {
        reference: CompletionCandidateRef,
        calculated: CompletionCandidateRef,
    },
    #[error("finite decoder names candidate {0} more than once in one row")]
    DuplicateCandidate(CompletionCandidateRef),
    #[error("decoder is for query {decoder}, but candidate is for {candidate}")]
    CandidateQueryMismatch {
        decoder: QueryRef,
        candidate: QueryRef,
    },
}

/// Failures while connecting an event-record-linked decoded candidate to an observation use.
#[derive(Debug, Error)]
pub enum DecodedObservationError {
    #[error(transparent)]
    Candidate(#[from] CompletionCandidateError),
    #[error(transparent)]
    CandidateCheck(#[from] CompletionCandidateCheckError),
    #[error(transparent)]
    Query(#[from] OpenQueryError),
    #[error(transparent)]
    QueryCheck(#[from] OpenQueryCheckError),
    #[error(transparent)]
    Observation(#[from] RelationUseError),
    #[error(transparent)]
    ObservationCheck(#[from] RelationUseCheckError),
    #[error("decoded event {event} does not preserve candidate {candidate}")]
    CandidateNotDecoded {
        candidate: CompletionCandidateRef,
        event: EventRef,
    },
    #[error("completion candidate {0} is unavailable")]
    UnresolvedCandidate(CompletionCandidateRef),
    #[error("completion candidate {reference} hashes to {calculated}, not its claimed identity")]
    CandidateIdentityMismatch {
        reference: CompletionCandidateRef,
        calculated: CompletionCandidateRef,
    },
    #[error("decoded query {decoded} does not match candidate source {candidate}")]
    CandidateQueryMismatch {
        decoded: QueryRef,
        candidate: QueryRef,
    },
    #[error("decoded query {0} is unavailable")]
    UnresolvedQuery(QueryRef),
    #[error("query {reference} hashes to {calculated}, not its claimed identity")]
    QueryIdentityMismatch {
        reference: QueryRef,
        calculated: QueryRef,
    },
    #[error("declared observation use {0} is unavailable")]
    UnresolvedObservation(RelationUseRef),
    #[error("observation use {reference} hashes to {calculated}, not its claimed identity")]
    ObservationIdentityMismatch {
        reference: RelationUseRef,
        calculated: RelationUseRef,
    },
    #[error("observation relation {observation} does not match decoded query relation {query}")]
    RelationMismatch {
        query: crate::RelationRef,
        observation: crate::RelationRef,
    },
    #[error("observation bindings do not exactly match the decoded candidate")]
    BindingMismatch,
    #[error("observation context does not exactly match the decoded query")]
    ContextMismatch,
}

#[derive(Debug, Error)]
pub enum ActualDecodeError {
    #[error(transparent)]
    Event(#[from] ActualEventError),
    #[error(transparent)]
    EventCheck(#[from] ActualEventCheckError),
    #[error(transparent)]
    Decoder(#[from] FiniteDecoderError),
    #[error(transparent)]
    DecoderCheck(#[from] FiniteDecoderCheckError),
    #[error(transparent)]
    Path(#[from] ResolutionPathError),
    #[error(transparent)]
    PathCheck(#[from] ResolutionPathCheckError),
    #[error(transparent)]
    Operator(#[from] ProbeOperatorError),
    #[error("resolution path {0} is unavailable")]
    UnresolvedPath(ResolutionPathRef),
    #[error("resolution path {reference} hashes to {calculated}, not its claimed identity")]
    PathIdentityMismatch {
        reference: ResolutionPathRef,
        calculated: ResolutionPathRef,
    },
    #[error("resolution path {0} is not one direct decoder route")]
    PathIsNotDirectDecoder(ResolutionPathRef),
    #[error("resolution path decoder {path} does not name finite decoder {decoder}")]
    PathDecoderMismatch {
        path: DecoderRef,
        decoder: DecoderRef,
    },
    #[error("resolution path input {path} does not match finite decoder input {decoder}")]
    PathInputMismatch { path: TypeRef, decoder: TypeRef },
    #[error("probe operator {0} is unavailable")]
    UnresolvedOperator(crate::ProbeOperatorRef),
    #[error("probe operator {reference} hashes to {calculated}, not its claimed identity")]
    OperatorIdentityMismatch {
        reference: crate::ProbeOperatorRef,
        calculated: crate::ProbeOperatorRef,
    },
    #[error("event operator return type {operator} does not match finite decoder input {decoder}")]
    EventReturnTypeMismatch { operator: TypeRef, decoder: TypeRef },
    #[error("actual event query {event} does not match finite decoder query {decoder}")]
    EventQueryMismatch { event: QueryRef, decoder: QueryRef },
    #[error("finite decoder query {0} is unavailable")]
    UnresolvedQuery(QueryRef),
    #[error("finite decoder relation {0} is unavailable")]
    UnresolvedRelation(crate::RelationRef),
    #[error("finite direct decode requires exactly one open answer port, got {actual}")]
    UnsupportedAnswerArity { actual: usize },
    #[error("answer port {0} is not an open port of this question")]
    ForeignAnswerPort(TypeSymbol),
    #[error("resolution path output {path} does not match query answer type {answer}")]
    PathOutputMismatch { path: TypeRef, answer: TypeRef },
}

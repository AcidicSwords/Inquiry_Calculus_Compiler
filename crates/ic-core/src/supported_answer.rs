//! Derived finite supported-answer admission over preserved probe events.
//!
//! A decoded candidate set is not yet `SuppAns(q)`: decoding can preserve alternatives without
//! establishing that their relation occurrences are supported. This module supplies the first
//! finite bridge while retaining event, decoder, path, use, and exact standing-route provenance.

use std::collections::BTreeSet;

use thiserror::Error;

use crate::{
    ActualDecodeError, ActualDecodeResult, ActualEventCheckError, ActualEventError,
    CompletionCandidateRef, DecodedCandidateSet, DecodedObservationError, DecodedObservationUse,
    EventRef, FiniteDecoderCatalog, FiniteDecoderRef, IProgArtifact, IProgCatalog, IProgCheckError,
    IProgError, IProgIR, IProgRef, ObservationResultCatalog, OperatorOccurrenceCatalog,
    ProbeOperatorRef, ProgramBinding, QueryRef, RawReturnRef, RelationUseRef,
    RelationUseSupportCatalog, RelationUseSupportError, ResolvedRelationUseSupport, Standing,
    SupportEnvironmentArtifactCheckError, SupportEnvironmentArtifactError,
    SupportEnvironmentCatalog, SupportEnvironmentRef, TypeSymbol, check_actual_event,
    match_decoded_observation_use, standing_relation_use_support,
};

/// Catalog boundary for finite event-linked supported-answer admission.
pub trait FiniteSupportedAnswerCatalog:
    ObservationResultCatalog + RelationUseSupportCatalog + OperatorOccurrenceCatalog
{
}

impl<T> FiniteSupportedAnswerCatalog for T where
    T: ObservationResultCatalog + RelationUseSupportCatalog + OperatorOccurrenceCatalog
{
}

/// One nonempty finite supported answer set for a single semantic question.
///
/// This is derived evidence, not a canonical artifact, warrant, standing mutation, or arbitrary
/// singleton selection. Every decoded completion remains present and retains its own exact
/// observation-use support route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedFiniteAnswerSet {
    decoded: DecodedCandidateSet,
    observations: Vec<DecodedObservationUse>,
    support: Vec<ResolvedRelationUseSupport>,
    event: EventRef,
    operator: ProbeOperatorRef,
    raw_return: RawReturnRef,
}

impl AdmittedFiniteAnswerSet {
    #[must_use]
    pub const fn decoded(&self) -> &DecodedCandidateSet {
        &self.decoded
    }

    #[must_use]
    pub fn candidates(&self) -> &[CompletionCandidateRef] {
        self.decoded.candidates()
    }

    #[must_use]
    pub fn observations(&self) -> &[DecodedObservationUse] {
        &self.observations
    }

    #[must_use]
    pub fn support(&self) -> &[ResolvedRelationUseSupport] {
        &self.support
    }

    #[must_use]
    pub const fn event(&self) -> EventRef {
        self.event
    }

    #[must_use]
    pub const fn operator(&self) -> ProbeOperatorRef {
        self.operator
    }

    #[must_use]
    pub const fn raw_return(&self) -> RawReturnRef {
        self.raw_return
    }
}

/// Admits all and only the completions in one rechecked finite decoded result.
///
/// Every observation must reproduce the decoded candidate/use correspondence, declare `Probe`,
/// close through its exact relation-targeted standing environment, and name the event's preserved
/// raw return. Missing, duplicate, foreign, generated/check-only, or extra observations reject.
pub fn admit_finite_supported_answers<C: FiniteSupportedAnswerCatalog>(
    decoded: DecodedCandidateSet,
    observations: Vec<DecodedObservationUse>,
    standing: &Standing,
    catalog: &C,
) -> Result<AdmittedFiniteAnswerSet, FiniteSupportedAnswerError> {
    admit_finite_supported_answers_scoped(
        crate::decoder::AnswerPortScope::SoleOpenPort,
        decoded,
        observations,
        standing,
        catalog,
    )
}

pub(crate) fn admit_finite_supported_answers_scoped<C: FiniteSupportedAnswerCatalog>(
    scope: crate::decoder::AnswerPortScope<'_>,
    decoded: DecodedCandidateSet,
    observations: Vec<DecodedObservationUse>,
    standing: &Standing,
    catalog: &C,
) -> Result<AdmittedFiniteAnswerSet, FiniteSupportedAnswerError> {
    let event_ref = decoded.event();
    let event = OperatorOccurrenceCatalog::resolve_actual_event(catalog, event_ref)
        .ok_or(FiniteSupportedAnswerError::UnresolvedEvent(event_ref))?;
    let calculated = event.event_ref()?;
    if calculated != event_ref {
        return Err(FiniteSupportedAnswerError::EventIdentityMismatch {
            reference: event_ref,
            calculated,
        });
    }
    check_actual_event(&event, catalog)?;
    let decoder_ref = decoded.decoder();
    let decoder = FiniteDecoderCatalog::resolve_finite_decoder(catalog, decoder_ref)
        .ok_or(FiniteSupportedAnswerError::UnresolvedDecoder(decoder_ref))?;
    let rerun = crate::decoder::decode_actual_event_scoped(
        scope,
        &event,
        &decoder,
        decoded.path(),
        catalog,
    )?;
    let ActualDecodeResult::Decoded(rerun) = rerun else {
        return Err(FiniteSupportedAnswerError::ResultNoLongerDecoded(event_ref));
    };
    if rerun != decoded {
        return Err(FiniteSupportedAnswerError::DecodedResultMismatch);
    }

    let expected: BTreeSet<_> = decoded.candidates().iter().copied().collect();
    let mut seen = BTreeSet::new();
    let mut admitted = Vec::with_capacity(observations.len());
    for observation in &observations {
        let candidate = observation.candidate();
        if !seen.insert(candidate) {
            return Err(FiniteSupportedAnswerError::DuplicateCandidate(candidate));
        }
        if observation.decoded() != &decoded {
            return Err(FiniteSupportedAnswerError::ForeignDecodedResult(candidate));
        }
        let use_ref = observation.observation();
        match_decoded_observation_use(&decoded, candidate, use_ref, catalog)?;
        let relation_use = ObservationResultCatalog::resolve_relation_use(catalog, use_ref)
            .ok_or(FiniteSupportedAnswerError::UnresolvedRelationUse(use_ref))?;
        if relation_use.mode() != crate::DischargeMode::Probe {
            return Err(FiniteSupportedAnswerError::ObservationIsNotProbe(use_ref));
        }
        let support = standing_relation_use_support(use_ref, standing, catalog)?;
        ensure_return_named(support.environment(), event.raw_return(), catalog)?;
        admitted.push(support);
    }
    if seen != expected {
        return Err(FiniteSupportedAnswerError::CandidateCoverageMismatch);
    }

    let mut paired: Vec<_> = observations.into_iter().zip(admitted).collect();
    paired.sort_by_key(|(observation, _)| observation.candidate());
    let (observations, support): (Vec<_>, Vec<_>) = paired.into_iter().unzip();
    Ok(AdmittedFiniteAnswerSet {
        decoded,
        observations,
        support,
        event: event_ref,
        operator: event.operator(),
        raw_return: event.raw_return(),
    })
}

/// One checked lexical binding of an admitted finite answer to an `IProg::Ask` continuation.
///
/// The explicit environment and answer binding remain separate so the answer slot cannot capture
/// a pre-existing value. This is inspectable derived data, not substitution, execution, or a new
/// canonical program artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundFiniteAskContinuation {
    source: IProgRef,
    question: QueryRef,
    environment: Vec<ProgramBinding>,
    answer_slot: TypeSymbol,
    answer: AdmittedFiniteAnswerSet,
    continuation: IProgRef,
}

impl BoundFiniteAskContinuation {
    #[must_use]
    pub const fn source(&self) -> IProgRef {
        self.source
    }

    #[must_use]
    pub const fn question(&self) -> QueryRef {
        self.question
    }

    #[must_use]
    pub fn environment(&self) -> &[ProgramBinding] {
        &self.environment
    }

    #[must_use]
    pub const fn answer_slot(&self) -> &TypeSymbol {
        &self.answer_slot
    }

    #[must_use]
    pub const fn answer(&self) -> &AdmittedFiniteAnswerSet {
        &self.answer
    }

    #[must_use]
    pub const fn continuation(&self) -> IProgRef {
        self.continuation
    }
}

/// Binds the whole admitted finite answer to the named slot of one checked source ask.
///
/// The source program is rechecked, including capture rejection and continuation identity. The
/// admitted answer must name exactly the same semantic question. No candidate is selected from a
/// multi-completion answer and the continuation is returned as data rather than invoked.
pub fn bind_finite_ask_continuation<C: IProgCatalog>(
    source: &IProgArtifact,
    answer: AdmittedFiniteAnswerSet,
    catalog: &C,
) -> Result<BoundFiniteAskContinuation, FiniteAnswerBindingError> {
    source.check(catalog)?;
    let source_ref = source.iprog_ref()?;
    let IProgIR::Ask {
        question,
        environment,
        answer_slot,
        continuation,
    } = source.program()
    else {
        return Err(FiniteAnswerBindingError::SourceIsNotAsk(source_ref));
    };
    if *question != answer.decoded().query() {
        return Err(FiniteAnswerBindingError::QuestionMismatch {
            program_question: *question,
            answer_question: answer.decoded().query(),
        });
    }
    Ok(BoundFiniteAskContinuation {
        source: source_ref,
        question: *question,
        environment: environment.clone(),
        answer_slot: answer_slot.clone(),
        answer,
        continuation: *continuation,
    })
}

fn ensure_return_named<C: SupportEnvironmentCatalog>(
    environment_ref: SupportEnvironmentRef,
    raw_return: RawReturnRef,
    catalog: &C,
) -> Result<(), FiniteSupportedAnswerError> {
    let environment = catalog.resolve_support_environment(environment_ref).ok_or(
        FiniteSupportedAnswerError::UnresolvedEnvironment(environment_ref),
    )?;
    let calculated = environment.support_environment_ref()?;
    if calculated != environment_ref {
        return Err(FiniteSupportedAnswerError::EnvironmentIdentityMismatch {
            reference: environment_ref,
            calculated,
        });
    }
    environment.check(catalog)?;
    if !environment.actual_returns().contains(&raw_return) {
        return Err(FiniteSupportedAnswerError::SupportMissingReturn {
            environment: environment_ref,
            raw_return,
        });
    }
    Ok(())
}

/// Errors from finite supported-answer admission.
#[derive(Debug, Error)]
pub enum FiniteSupportedAnswerError {
    #[error(transparent)]
    EventEncoding(#[from] ActualEventError),
    #[error(transparent)]
    EventCheck(Box<ActualEventCheckError>),
    #[error(transparent)]
    Decode(Box<ActualDecodeError>),
    #[error(transparent)]
    Observation(Box<DecodedObservationError>),
    #[error(transparent)]
    RelationSupport(Box<RelationUseSupportError>),
    #[error(transparent)]
    EnvironmentEncoding(#[from] SupportEnvironmentArtifactError),
    #[error(transparent)]
    EnvironmentCheck(Box<SupportEnvironmentArtifactCheckError>),
    #[error("actual event {0} is unavailable")]
    UnresolvedEvent(EventRef),
    #[error("actual event {reference} hashes to {calculated}, not its claimed identity")]
    EventIdentityMismatch {
        reference: EventRef,
        calculated: EventRef,
    },
    #[error("finite decoder {0} is unavailable")]
    UnresolvedDecoder(FiniteDecoderRef),
    #[error("event {0} no longer has a decoded finite result")]
    ResultNoLongerDecoded(EventRef),
    #[error("rechecked decoded result differs from the supplied result")]
    DecodedResultMismatch,
    #[error("completion candidate {0} occurs more than once")]
    DuplicateCandidate(CompletionCandidateRef),
    #[error("completion candidate {0} belongs to another decoded result")]
    ForeignDecodedResult(CompletionCandidateRef),
    #[error("observation relation use {0} is unavailable")]
    UnresolvedRelationUse(RelationUseRef),
    #[error("observation relation use {0} is not discharged through Probe")]
    ObservationIsNotProbe(RelationUseRef),
    #[error("the supported observations do not cover exactly the decoded candidate set")]
    CandidateCoverageMismatch,
    #[error("support environment {0} is unavailable")]
    UnresolvedEnvironment(SupportEnvironmentRef),
    #[error("support environment {reference} hashes to {calculated}, not its claimed identity")]
    EnvironmentIdentityMismatch {
        reference: SupportEnvironmentRef,
        calculated: SupportEnvironmentRef,
    },
    #[error("support environment {environment} does not name actual return {raw_return}")]
    SupportMissingReturn {
        environment: SupportEnvironmentRef,
        raw_return: RawReturnRef,
    },
}

impl From<ActualEventCheckError> for FiniteSupportedAnswerError {
    fn from(error: ActualEventCheckError) -> Self {
        Self::EventCheck(Box::new(error))
    }
}

impl From<ActualDecodeError> for FiniteSupportedAnswerError {
    fn from(error: ActualDecodeError) -> Self {
        Self::Decode(Box::new(error))
    }
}

impl From<DecodedObservationError> for FiniteSupportedAnswerError {
    fn from(error: DecodedObservationError) -> Self {
        Self::Observation(Box::new(error))
    }
}

impl From<RelationUseSupportError> for FiniteSupportedAnswerError {
    fn from(error: RelationUseSupportError) -> Self {
        Self::RelationSupport(Box::new(error))
    }
}

impl From<SupportEnvironmentArtifactCheckError> for FiniteSupportedAnswerError {
    fn from(error: SupportEnvironmentArtifactCheckError) -> Self {
        Self::EnvironmentCheck(Box::new(error))
    }
}

/// Errors from capture-safe finite answer binding.
#[derive(Debug, Error)]
pub enum FiniteAnswerBindingError {
    #[error(transparent)]
    IProgEncoding(#[from] IProgError),
    #[error(transparent)]
    IProgCheck(Box<IProgCheckError>),
    #[error("source inquiry program {0} is not an Ask")]
    SourceIsNotAsk(IProgRef),
    #[error(
        "source Ask names question {program_question}, but the admitted answer names {answer_question}"
    )]
    QuestionMismatch {
        program_question: QueryRef,
        answer_question: QueryRef,
    },
}

impl From<IProgCheckError> for FiniteAnswerBindingError {
    fn from(error: IProgCheckError) -> Self {
        Self::IProgCheck(Box::new(error))
    }
}

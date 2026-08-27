//! The whole-question resolution gate over one checked mixed-mode source `Ask`.
//!
//! Canonical `Resolver(AskRef)` maps the complete port-indexed discharge bundle, not one port, to
//! exactly one `ResolutionOutcome`. This module derives that map for a mixed-mode occurrence: each
//! Probe port supplies an independently classified outcome, each non-Probe port supplies its
//! already checked typed result, and only a joint `Supported` may reach the source continuation.
//!
//! It runs no operator, appends no event, decodes nothing again, and creates no resolver artifact.

use std::collections::{BTreeMap, BTreeSet};

use ic_core::{
    AskOccurrenceRef, CompletionCandidateCatalog, DischargeMode, FiniteResolutionOutcome,
    FiniteResolutionOutcomeKind, FiniteSupportedAnswerCatalog, FiniteSupportedResolution,
    IProgArtifact, IProgError, IProgIR, IProgRef, QueryRef, TypeSymbol, TypedFormRef,
};
use thiserror::Error;

use crate::{
    MixedModeSourceAskDischarge, MixedModeSourceAskDischargeError, NonProbePortDischargeEvidence,
    SourceAskProbeDischargeCatalog,
};

/// The catalog needed to rewalk a mixed view and recheck its per-port answers without dispatch.
pub trait MixedQuestionResolutionCatalog:
    SourceAskProbeDischargeCatalog + FiniteSupportedAnswerCatalog + CompletionCandidateCatalog
{
}
impl<T> MixedQuestionResolutionCatalog for T where
    T: SourceAskProbeDischargeCatalog + FiniteSupportedAnswerCatalog + CompletionCandidateCatalog
{
}

/// One open port's contribution to a whole-question answer.
///
/// A Probe port contributes its event-linked supported resolution; a non-Probe port contributes
/// its checked typed result and declared authority route. Neither can stand in for the other.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MixedPortContribution {
    Probe {
        port: TypeSymbol,
        resolution: Box<FiniteSupportedResolution>,
    },
    NonProbe {
        port: TypeSymbol,
        evidence: Box<NonProbePortDischargeEvidence>,
    },
}

impl MixedPortContribution {
    #[must_use]
    pub const fn port(&self) -> &TypeSymbol {
        match self {
            Self::Probe { port, .. } | Self::NonProbe { port, .. } => port,
        }
    }

    #[must_use]
    pub fn mode(&self) -> DischargeMode {
        match self {
            Self::Probe { .. } => DischargeMode::Probe,
            Self::NonProbe { evidence, .. } => evidence.mode(),
        }
    }
}

/// One whole-question supported answer over the complete open-port field of one occurrence.
///
/// This record exists only as the output of [`resolve_mixed_mode_question`]: it has no public
/// constructor, so a partial port field cannot be presented as a whole-question answer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WholeQuestionSupportedAnswer {
    occurrence: AskOccurrenceRef,
    question: QueryRef,
    contributions: Vec<MixedPortContribution>,
}

impl WholeQuestionSupportedAnswer {
    #[must_use]
    pub const fn occurrence(&self) -> AskOccurrenceRef {
        self.occurrence
    }

    #[must_use]
    pub const fn question(&self) -> QueryRef {
        self.question
    }

    /// Every open port's contribution, in canonical port order.
    #[must_use]
    pub fn contributions(&self) -> &[MixedPortContribution] {
        &self.contributions
    }
}

/// One port whose evidence did not resolve as `Supported`, with its retained residual.
#[derive(Debug)]
pub struct NonSupportedPort {
    port: TypeSymbol,
    outcome: FiniteResolutionOutcome,
}

impl NonSupportedPort {
    #[must_use]
    pub const fn port(&self) -> &TypeSymbol {
        &self.port
    }

    #[must_use]
    pub const fn outcome(&self) -> &FiniteResolutionOutcome {
        &self.outcome
    }
}

/// Exactly one whole-question outcome. Its kind is one of the same five as a single port's.
#[derive(Debug)]
pub enum WholeQuestionOutcome {
    Supported(WholeQuestionSupportedAnswer),
    NotSupported(Box<NonSupportedPort>),
}

impl WholeQuestionOutcome {
    #[must_use]
    pub const fn kind(&self) -> FiniteResolutionOutcomeKind {
        match self {
            Self::Supported(_) => FiniteResolutionOutcomeKind::Supported,
            Self::NotSupported(port) => port.outcome.kind(),
        }
    }
}

/// Maps one checked mixed view and its per-Probe-port outcomes to exactly one whole-question
/// outcome.
///
/// Every open port must be accounted for exactly once before any outcome is produced, so no Probe
/// return can resolve the question while a declared non-Probe port is still undischarged. When
/// every Probe port is `Supported`, each decoded completion must additionally agree with every
/// non-Probe port's checked typed result: the completion field is the relation's, not a product of
/// independently chosen port values.
pub fn resolve_mixed_mode_question<C: MixedQuestionResolutionCatalog>(
    view: &MixedModeSourceAskDischarge,
    port_outcomes: Vec<(TypeSymbol, FiniteResolutionOutcome)>,
    catalog: &C,
) -> Result<WholeQuestionOutcome, MixedQuestionResolutionError> {
    view.check(catalog)?;
    let occurrence = view.lowering().occurrence();
    let occurrence_ref = occurrence.ask_occurrence_ref()?;
    let question = occurrence.question();

    let mut probe_ports = BTreeSet::new();
    let mut non_probe = BTreeMap::new();
    for lowering in view.lowering().port_lowerings() {
        if lowering.mode() == DischargeMode::Probe {
            probe_ports.insert(lowering.port().clone());
        }
    }
    for evidence in view.non_probe() {
        non_probe.insert(evidence.port().clone(), evidence.clone());
    }

    let mut supplied = BTreeMap::new();
    for (port, outcome) in port_outcomes {
        if !probe_ports.contains(&port) {
            return Err(MixedQuestionResolutionError::ForeignPortOutcome(
                port.as_str().to_owned(),
            ));
        }
        if supplied.insert(port.clone(), outcome).is_some() {
            return Err(MixedQuestionResolutionError::DuplicatePortOutcome(
                port.as_str().to_owned(),
            ));
        }
    }
    for port in &probe_ports {
        if !supplied.contains_key(port) {
            return Err(MixedQuestionResolutionError::MissingPortOutcome(
                port.as_str().to_owned(),
            ));
        }
    }

    // The whole question takes the first non-Supported port's outcome, residual intact.
    let mut supported = BTreeMap::new();
    for (port, outcome) in supplied {
        match outcome.into_supported() {
            Ok(resolution) => {
                supported.insert(port, resolution);
            }
            Err(outcome) => {
                return Ok(WholeQuestionOutcome::NotSupported(Box::new(
                    NonSupportedPort {
                        port,
                        outcome: *outcome,
                    },
                )));
            }
        }
    }

    for (port, resolution) in &supported {
        if resolution.query() != question {
            return Err(MixedQuestionResolutionError::PortAnswerQuestionMismatch {
                port: port.as_str().to_owned(),
                answer: resolution.query(),
                occurrence: question,
            });
        }
        let component = view
            .probe_bundle()
            .components()
            .iter()
            .find(|component| component.port() == port)
            .ok_or_else(|| {
                MixedQuestionResolutionError::PortMissingFromBundle(port.as_str().to_owned())
            })?;
        if resolution.event() != component.event().event_ref() {
            return Err(MixedQuestionResolutionError::PortAnswerEventMismatch(
                port.as_str().to_owned(),
            ));
        }
        for candidate_ref in resolution.answer().candidates() {
            let candidate = catalog.resolve_completion_candidate(*candidate_ref).ok_or(
                MixedQuestionResolutionError::UnresolvedCandidate(*candidate_ref),
            )?;
            for (non_probe_port, evidence) in &non_probe {
                let bound = candidate
                    .bindings()
                    .iter()
                    .find(|binding| binding.port() == non_probe_port)
                    .ok_or_else(|| {
                        MixedQuestionResolutionError::CompletionOmitsNonProbePort(
                            non_probe_port.as_str().to_owned(),
                        )
                    })?;
                if bound.value() != evidence.result() {
                    return Err(
                        MixedQuestionResolutionError::CompletionContradictsNonProbeResult {
                            probe_port: port.as_str().to_owned(),
                            non_probe_port: non_probe_port.as_str().to_owned(),
                            completion: bound.value(),
                            evidence: evidence.result(),
                        },
                    );
                }
            }
        }
    }

    let mut contributions = Vec::with_capacity(supported.len() + non_probe.len());
    for (port, resolution) in supported {
        contributions.push(MixedPortContribution::Probe {
            port,
            resolution: Box::new(resolution),
        });
    }
    for (port, evidence) in non_probe {
        contributions.push(MixedPortContribution::NonProbe {
            port,
            evidence: Box::new(evidence),
        });
    }
    contributions.sort_by(|left, right| left.port().cmp(right.port()));
    Ok(WholeQuestionOutcome::Supported(
        WholeQuestionSupportedAnswer {
            occurrence: occurrence_ref,
            question,
            contributions,
        },
    ))
}

/// One mixed-mode occurrence whose whole-question answer reached its exact checked continuation.
///
/// The continuation is retained as data. Nothing here invokes it or derives a successor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedMixedModeContinuation {
    source: IProgRef,
    occurrence: AskOccurrenceRef,
    question: QueryRef,
    answer_slot: TypeSymbol,
    answer: WholeQuestionSupportedAnswer,
    continuation: IProgRef,
}

impl AdmittedMixedModeContinuation {
    #[must_use]
    pub const fn source(&self) -> IProgRef {
        self.source
    }

    #[must_use]
    pub const fn occurrence(&self) -> AskOccurrenceRef {
        self.occurrence
    }

    #[must_use]
    pub const fn question(&self) -> QueryRef {
        self.question
    }

    #[must_use]
    pub const fn answer_slot(&self) -> &TypeSymbol {
        &self.answer_slot
    }

    #[must_use]
    pub const fn answer(&self) -> &WholeQuestionSupportedAnswer {
        &self.answer
    }

    #[must_use]
    pub const fn continuation(&self) -> IProgRef {
        self.continuation
    }
}

/// Binds a whole-question supported answer to the exact source `Ask` slot of its own occurrence.
///
/// Only the `Supported` summand may enter; the other four retain their residual and are refused
/// here rather than reaching a continuation.
pub fn admit_mixed_mode_continuation<C: MixedQuestionResolutionCatalog>(
    outcome: WholeQuestionOutcome,
    view: &MixedModeSourceAskDischarge,
    source_ask: &IProgArtifact,
    catalog: &C,
) -> Result<AdmittedMixedModeContinuation, MixedQuestionResolutionError> {
    let WholeQuestionOutcome::Supported(answer) = outcome else {
        return Err(MixedQuestionResolutionError::NonSupported(Box::new(
            outcome,
        )));
    };
    let occurrence = view.lowering().occurrence();
    if answer.occurrence != occurrence.ask_occurrence_ref()? {
        return Err(MixedQuestionResolutionError::AnswerOccurrenceMismatch);
    }
    source_ask.check(catalog)?;
    let source_ref = source_ask.iprog_ref()?;
    if source_ref != occurrence.position().target() {
        return Err(MixedQuestionResolutionError::SourceProgramMismatch {
            occurrence: occurrence.position().target(),
            supplied: source_ref,
        });
    }
    let IProgIR::Ask {
        question,
        answer_slot,
        continuation,
        ..
    } = source_ask.program()
    else {
        return Err(MixedQuestionResolutionError::SourceIsNotAsk(source_ref));
    };
    if *question != answer.question
        || answer_slot != occurrence.answer_slot()
        || *continuation != occurrence.continuation()
    {
        return Err(MixedQuestionResolutionError::ContinuationMismatch);
    }
    Ok(AdmittedMixedModeContinuation {
        source: source_ref,
        occurrence: answer.occurrence,
        question: *question,
        answer_slot: answer_slot.clone(),
        answer,
        continuation: *continuation,
    })
}

#[derive(Debug, Error)]
pub enum MixedQuestionResolutionError {
    #[error(transparent)]
    View(#[from] MixedModeSourceAskDischargeError),
    #[error(transparent)]
    OccurrenceEncoding(#[from] ic_core::AskOccurrenceError),
    #[error(transparent)]
    IProgEncoding(#[from] IProgError),
    #[error(transparent)]
    IProgCheck(#[from] ic_core::IProgCheckError),
    #[error("outcome names port {0:?}, which this source Ask does not declare Probe")]
    ForeignPortOutcome(String),
    #[error("more than one outcome was supplied for Probe port {0:?}")]
    DuplicatePortOutcome(String),
    #[error("Probe port {0:?} has no resolution outcome, so the question is not whole")]
    MissingPortOutcome(String),
    #[error("port {port:?} answered question {answer}, but the occurrence asks {occurrence}")]
    PortAnswerQuestionMismatch {
        port: String,
        answer: QueryRef,
        occurrence: QueryRef,
    },
    #[error("port {0:?} is absent from the checked Probe discharge bundle")]
    PortMissingFromBundle(String),
    #[error("port {0:?} answered from an event outside its own bundle component")]
    PortAnswerEventMismatch(String),
    #[error("completion candidate {0} is unavailable")]
    UnresolvedCandidate(ic_core::CompletionCandidateRef),
    #[error("a decoded completion omits non-Probe port {0:?}")]
    CompletionOmitsNonProbePort(String),
    #[error(
        "a completion from Probe port {probe_port:?} binds non-Probe port {non_probe_port:?} to {completion}, but that port's checked evidence is {evidence}"
    )]
    CompletionContradictsNonProbeResult {
        probe_port: String,
        non_probe_port: String,
        completion: TypedFormRef,
        evidence: TypedFormRef,
    },
    #[error("whole-question outcome {} cannot enter a source continuation", .0.kind())]
    NonSupported(Box<WholeQuestionOutcome>),
    #[error("whole-question answer belongs to a different source Ask occurrence")]
    AnswerOccurrenceMismatch,
    #[error("source occurrence targets {occurrence}, but supplied Ask program is {supplied}")]
    SourceProgramMismatch {
        occurrence: IProgRef,
        supplied: IProgRef,
    },
    #[error("supplied source program {0} is not an Ask")]
    SourceIsNotAsk(IProgRef),
    #[error("bound question, answer slot, or continuation differs from the exact occurrence")]
    ContinuationMismatch,
}

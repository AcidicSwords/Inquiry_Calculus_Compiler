//! Required-safe finite live-question frontiers.
//!
//! This module derives a local view over checked source `Ask` occurrences. It is not a
//! scheduler, controller, event history, or source of discharge authority. The caller supplies
//! the finite candidate set, protected answer/continuation observations, exact obligation
//! provenance, and a declared resource preorder.

use std::collections::{BTreeMap, BTreeSet};

use thiserror::Error;

use crate::{
    ArtifactRef, AskOccurrence, AskOccurrenceCheckError, AskOccurrenceError, AskOccurrenceRef,
    FiniteResourcePreorder, ProtectedContinuationRef, QuestionSuccessionCatalog,
};

/// A discharge that can keep one exact question occurrence live without discretionary
/// productivity.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RequiredDischargeKind {
    Probe,
    Check,
    Warrant,
    Support,
    Reconstruction,
}

/// Exact provenance for one undischarged occurrence-local obligation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RequiredQuestionDischarge {
    kind: RequiredDischargeKind,
    source: ArtifactRef,
    authority: ArtifactRef,
}

impl RequiredQuestionDischarge {
    #[must_use]
    pub const fn new(
        kind: RequiredDischargeKind,
        source: ArtifactRef,
        authority: ArtifactRef,
    ) -> Self {
        Self {
            kind,
            source,
            authority,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> RequiredDischargeKind {
        self.kind
    }

    #[must_use]
    pub const fn source(&self) -> ArtifactRef {
        self.source
    }

    #[must_use]
    pub const fn authority(&self) -> ArtifactRef {
        self.authority
    }
}

/// How one candidate entered the supplied finite field.
///
/// Generation is proposal provenance only. It never creates a required discharge.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveQuestionOrigin {
    Existing { provenance: ArtifactRef },
    Generated { proposal: ArtifactRef },
}

/// One represented answer class and its protected continuation signature.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ProtectedQuestionBranch {
    answer_class: ArtifactRef,
    continuation: ProtectedContinuationRef,
}

impl ProtectedQuestionBranch {
    #[must_use]
    pub const fn new(answer_class: ArtifactRef, continuation: ProtectedContinuationRef) -> Self {
        Self {
            answer_class,
            continuation,
        }
    }

    #[must_use]
    pub const fn answer_class(&self) -> ArtifactRef {
        self.answer_class
    }

    #[must_use]
    pub const fn continuation(&self) -> ProtectedContinuationRef {
        self.continuation
    }
}

/// One checked-occurrence candidate for a finite live-question frontier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveQuestionCandidate {
    occurrence: AskOccurrence,
    branches: Vec<ProtectedQuestionBranch>,
    resource: ArtifactRef,
    origin: LiveQuestionOrigin,
    required_discharges: Vec<RequiredQuestionDischarge>,
}

impl LiveQuestionCandidate {
    pub fn new(
        occurrence: AskOccurrence,
        branches: Vec<ProtectedQuestionBranch>,
        resource: ArtifactRef,
        origin: LiveQuestionOrigin,
        required_discharges: Vec<RequiredQuestionDischarge>,
    ) -> Result<Self, LiveQuestionCandidateError> {
        let mut by_answer = BTreeMap::new();
        for branch in branches {
            if by_answer.insert(branch.answer_class(), branch).is_some() {
                return Err(LiveQuestionCandidateError::DuplicateAnswerClass(
                    branch.answer_class(),
                ));
            }
        }
        let mut obligations = BTreeSet::new();
        for discharge in required_discharges {
            if !obligations.insert(discharge) {
                return Err(LiveQuestionCandidateError::DuplicateRequiredDischarge(
                    discharge,
                ));
            }
        }
        Ok(Self {
            occurrence,
            branches: by_answer.into_values().collect(),
            resource,
            origin,
            required_discharges: obligations.into_iter().collect(),
        })
    }

    #[must_use]
    pub const fn occurrence(&self) -> &AskOccurrence {
        &self.occurrence
    }

    #[must_use]
    pub fn branches(&self) -> &[ProtectedQuestionBranch] {
        &self.branches
    }

    #[must_use]
    pub const fn resource(&self) -> ArtifactRef {
        self.resource
    }

    #[must_use]
    pub const fn origin(&self) -> LiveQuestionOrigin {
        self.origin
    }

    #[must_use]
    pub fn required_discharges(&self) -> &[RequiredQuestionDischarge] {
        &self.required_discharges
    }

    /// Whether represented supported answers reach at least two protected-different
    /// continuation signatures.
    #[must_use]
    pub fn is_productive(&self) -> bool {
        self.branches
            .iter()
            .map(ProtectedQuestionBranch::continuation)
            .collect::<BTreeSet<_>>()
            .len()
            > 1
    }

    #[must_use]
    pub fn is_required(&self) -> bool {
        !self.required_discharges.is_empty()
    }
}

/// Required-safe nondominated candidates from one finite supplied field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteLiveQuestionFrontier {
    members: Vec<LiveQuestionCandidate>,
    dominated_optional: Vec<LiveQuestionCandidate>,
    inactive_optional: Vec<LiveQuestionCandidate>,
}

impl FiniteLiveQuestionFrontier {
    #[must_use]
    pub fn members(&self) -> &[LiveQuestionCandidate] {
        &self.members
    }

    #[must_use]
    pub fn dominated_optional(&self) -> &[LiveQuestionCandidate] {
        &self.dominated_optional
    }

    #[must_use]
    pub fn inactive_optional(&self) -> &[LiveQuestionCandidate] {
        &self.inactive_optional
    }
}

/// Derives `required candidates union ordinary nondominated live candidates`.
///
/// The function rechecks every supplied `Ask` occurrence. An optional candidate is live only
/// when its represented answer classes lead to protected-different continuations. Required
/// candidates remain live regardless of productivity or strict resource domination. Ordinary
/// nondominance is still evaluated over the whole live field, so a required candidate can
/// remove an optional candidate that it strictly dominates.
pub fn derive_finite_live_question_frontier<C: QuestionSuccessionCatalog>(
    candidates: &[LiveQuestionCandidate],
    resources: &FiniteResourcePreorder,
    catalog: &C,
) -> Result<FiniteLiveQuestionFrontier, FiniteLiveQuestionFrontierError> {
    let mut occurrences = BTreeSet::new();
    let mut resource_set = BTreeSet::new();
    for candidate in candidates {
        candidate.occurrence().check(catalog)?;
        let occurrence = candidate.occurrence().ask_occurrence_ref()?;
        if !occurrences.insert(occurrence) {
            return Err(FiniteLiveQuestionFrontierError::DuplicateOccurrence(
                occurrence,
            ));
        }
        resource_set.insert(candidate.resource());
    }
    check_resource_preorder(resources, &resource_set)?;

    let live: Vec<_> = candidates
        .iter()
        .filter(|candidate| candidate.is_required() || candidate.is_productive())
        .collect();
    let mut members = Vec::new();
    let mut dominated_optional = Vec::new();
    let mut inactive_optional = Vec::new();
    for candidate in candidates {
        if !candidate.is_required() && !candidate.is_productive() {
            inactive_optional.push(candidate.clone());
            continue;
        }
        let dominated = !candidate.is_required()
            && live.iter().any(|other| {
                *other != candidate
                    && less_or_equal(resources, other.resource(), candidate.resource())
                    && !less_or_equal(resources, candidate.resource(), other.resource())
            });
        if dominated {
            dominated_optional.push(candidate.clone());
        } else {
            members.push(candidate.clone());
        }
    }
    Ok(FiniteLiveQuestionFrontier {
        members,
        dominated_optional,
        inactive_optional,
    })
}

fn less_or_equal(
    resources: &FiniteResourcePreorder,
    lower: ArtifactRef,
    upper: ArtifactRef,
) -> bool {
    resources.less_or_equal().contains(&(lower, upper))
}

fn check_resource_preorder(
    resources: &FiniteResourcePreorder,
    resource_set: &BTreeSet<ArtifactRef>,
) -> Result<(), FiniteLiveQuestionFrontierError> {
    for resource in resource_set {
        if !less_or_equal(resources, *resource, *resource) {
            return Err(FiniteLiveQuestionFrontierError::NonReflexiveResource(
                *resource,
            ));
        }
    }
    for lower in resource_set {
        for middle in resource_set {
            for upper in resource_set {
                if less_or_equal(resources, *lower, *middle)
                    && less_or_equal(resources, *middle, *upper)
                    && !less_or_equal(resources, *lower, *upper)
                {
                    return Err(
                        FiniteLiveQuestionFrontierError::NonTransitiveResourceOrder {
                            lower: *lower,
                            middle: *middle,
                            upper: *upper,
                        },
                    );
                }
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum LiveQuestionCandidateError {
    #[error("candidate repeats protected answer class {0}")]
    DuplicateAnswerClass(ArtifactRef),
    #[error("candidate repeats required discharge {0:?}")]
    DuplicateRequiredDischarge(RequiredQuestionDischarge),
}

#[derive(Debug, Error)]
pub enum FiniteLiveQuestionFrontierError {
    #[error(transparent)]
    OccurrenceCheck(#[from] AskOccurrenceCheckError),
    #[error(transparent)]
    OccurrenceIdentity(#[from] AskOccurrenceError),
    #[error("candidate field repeats checked Ask occurrence {0}")]
    DuplicateOccurrence(AskOccurrenceRef),
    #[error("resource preorder is missing reflexive edge {0} <= {0}")]
    NonReflexiveResource(ArtifactRef),
    #[error("resource preorder is not transitive: {lower} <= {middle} <= {upper}")]
    NonTransitiveResourceOrder {
        lower: ArtifactRef,
        middle: ArtifactRef,
        upper: ArtifactRef,
    },
}

//! Derived local readiness for answer-conditioned inquiry questions.
//!
//! A readiness result is not an event, a scheduler decision, a program transition, or a
//! canonical artifact. It only exposes whether one already admitted whole answer contains the
//! one exact dependency a declared target question requires at this local boundary.

use thiserror::Error;

use crate::{
    AdmittedFiniteAnswerSet, ArtifactRef, OpenQueryCatalog, OpenQueryCheckError, OpenQueryError,
    QueryRef,
};

/// One exact dependency declared as locally required by one target question.
///
/// The dependency stays an ordinary artifact reference because its more specific type is owned
/// by the target question's later phase. This derived boundary neither invents nor interprets
/// that later dependency domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QuestionReadinessRequirement {
    target: QueryRef,
    dependency: ArtifactRef,
}

impl QuestionReadinessRequirement {
    #[must_use]
    pub const fn new(target: QueryRef, dependency: ArtifactRef) -> Self {
        Self { target, dependency }
    }

    #[must_use]
    pub const fn target(self) -> QueryRef {
        self.target
    }

    #[must_use]
    pub const fn dependency(self) -> ArtifactRef {
        self.dependency
    }
}

/// The local readiness boundary for one target/dependency pair.
///
/// `AwaitingAnswer` does not claim a negative condition: no event-linked supported answer has
/// been supplied to this derivation yet. `DependencyNotSupplied` retains the whole answer that
/// was checked and therefore distinguishes a mismatched actual answer from mere absence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuestionReadiness {
    AwaitingAnswer {
        requirement: QuestionReadinessRequirement,
    },
    DependencyNotSupplied {
        requirement: QuestionReadinessRequirement,
        answer: AdmittedFiniteAnswerSet,
    },
    Ready {
        requirement: QuestionReadinessRequirement,
        answer: AdmittedFiniteAnswerSet,
    },
}

impl QuestionReadiness {
    #[must_use]
    pub const fn requirement(&self) -> QuestionReadinessRequirement {
        match self {
            Self::AwaitingAnswer { requirement }
            | Self::DependencyNotSupplied { requirement, .. }
            | Self::Ready { requirement, .. } => *requirement,
        }
    }

    #[must_use]
    pub const fn answer(&self) -> Option<&AdmittedFiniteAnswerSet> {
        match self {
            Self::AwaitingAnswer { .. } => None,
            Self::DependencyNotSupplied { answer, .. } | Self::Ready { answer, .. } => Some(answer),
        }
    }

    #[must_use]
    pub const fn is_ready(&self) -> bool {
        matches!(self, Self::Ready { .. })
    }
}

/// The catalog required to recheck a declared target question before deriving readiness.
pub trait QuestionReadinessCatalog: OpenQueryCatalog {}

impl<T> QuestionReadinessCatalog for T where T: OpenQueryCatalog {}

/// Rechecks a target question and records that no supported answer has yet reached this local
/// readiness derivation.
pub fn await_question_readiness<C: QuestionReadinessCatalog>(
    requirement: QuestionReadinessRequirement,
    catalog: &C,
) -> Result<QuestionReadiness, QuestionReadinessError> {
    check_target(requirement, catalog)?;
    Ok(QuestionReadiness::AwaitingAnswer { requirement })
}

/// Rechecks a target question and tests one complete event-linked supported answer against its
/// exact declared dependency. The answer is never reduced to a selected completion.
pub fn derive_question_readiness<C: QuestionReadinessCatalog>(
    requirement: QuestionReadinessRequirement,
    answer: AdmittedFiniteAnswerSet,
    catalog: &C,
) -> Result<QuestionReadiness, QuestionReadinessError> {
    check_target(requirement, catalog)?;
    if answer
        .candidates()
        .iter()
        .any(|candidate| candidate.as_artifact_ref() == requirement.dependency())
    {
        Ok(QuestionReadiness::Ready {
            requirement,
            answer,
        })
    } else {
        Ok(QuestionReadiness::DependencyNotSupplied {
            requirement,
            answer,
        })
    }
}

fn check_target<C: QuestionReadinessCatalog>(
    requirement: QuestionReadinessRequirement,
    catalog: &C,
) -> Result<(), QuestionReadinessError> {
    let target = catalog.resolve_open_query(requirement.target()).ok_or(
        QuestionReadinessError::UnresolvedTarget(requirement.target()),
    )?;
    let calculated = target.query_ref()?;
    if calculated != requirement.target() {
        return Err(QuestionReadinessError::TargetIdentityMismatch {
            reference: requirement.target(),
            calculated,
        });
    }
    target.check(catalog)?;
    Ok(())
}

/// Readiness derivation failures.
#[derive(Debug, Error)]
pub enum QuestionReadinessError {
    #[error(transparent)]
    TargetEncoding(#[from] OpenQueryError),
    #[error(transparent)]
    TargetCheck(Box<OpenQueryCheckError>),
    #[error("target question {0} is unavailable")]
    UnresolvedTarget(QueryRef),
    #[error("target question {reference} hashes to {calculated}, not its claimed identity")]
    TargetIdentityMismatch {
        reference: QueryRef,
        calculated: QueryRef,
    },
}

impl From<OpenQueryCheckError> for QuestionReadinessError {
    fn from(error: OpenQueryCheckError) -> Self {
        Self::TargetCheck(Box::new(error))
    }
}

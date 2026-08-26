//! Exact finite sufficient-present checking under protected continuations.
//!
//! A present is sufficient only when every declared protected continuation factors through its
//! finite signature. A later continuation that splits one existing present class is returned as
//! a positive reopen witness. This is derived checker data, not active mutable memory, standing,
//! a compression licence, or a claim of bounded size beyond the supplied exact finite domain.

use std::collections::BTreeSet;

use thiserror::Error;

use crate::{
    ArtifactRef, ExactDeterminationError, ExactDeterminationResult, ExactFactorization,
    ExactFiniteSignature, KernelSeparator, ProtectedContinuationRef, determine_through_exact,
};

/// One protected continuation and its exact observation over the declared finite history domain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactProtectedContinuation {
    continuation: ProtectedContinuationRef,
    observation: ExactFiniteSignature,
}

impl ExactProtectedContinuation {
    #[must_use]
    pub const fn new(
        continuation: ProtectedContinuationRef,
        observation: ExactFiniteSignature,
    ) -> Self {
        Self {
            continuation,
            observation,
        }
    }

    #[must_use]
    pub const fn continuation(&self) -> ProtectedContinuationRef {
        self.continuation
    }

    #[must_use]
    pub const fn observation(&self) -> &ExactFiniteSignature {
        &self.observation
    }
}

/// One exact finite present together with every protected continuation it currently determines.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFiniteSufficientPresent {
    presentation: ExactFiniteSignature,
    protected: Vec<ExactProtectedContinuation>,
    factorizations: Vec<ExactFactorization>,
}

impl ExactFiniteSufficientPresent {
    #[must_use]
    pub const fn presentation(&self) -> &ExactFiniteSignature {
        &self.presentation
    }

    #[must_use]
    pub fn protected(&self) -> &[ExactProtectedContinuation] {
        &self.protected
    }

    #[must_use]
    pub fn factorizations(&self) -> &[ExactFactorization] {
        &self.factorizations
    }

    /// Number of equivalence classes retained by the finite presentation signature.
    #[must_use]
    pub fn class_count(&self) -> usize {
        self.presentation
            .values()
            .values()
            .copied()
            .collect::<BTreeSet<_>>()
            .len()
    }
}

/// Initial result of checking one finite presentation against protected continuations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExactFiniteSufficientPresentResult {
    Sufficient(ExactFiniteSufficientPresent),
    Insufficient {
        continuation: ProtectedContinuationRef,
        separator: KernelSeparator,
    },
}

/// A new protected continuation that distinguishes histories folded by the current present.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFinitePresentReopenWitness {
    continuation: ProtectedContinuationRef,
    separator: KernelSeparator,
}

impl ExactFinitePresentReopenWitness {
    #[must_use]
    pub const fn continuation(&self) -> ProtectedContinuationRef {
        self.continuation
    }

    #[must_use]
    pub const fn separator(&self) -> KernelSeparator {
        self.separator
    }
}

/// Result of extending the protected horizon of an already sufficient present.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExactFinitePresentChallenge {
    StillSufficient(ExactFiniteSufficientPresent),
    Reopened(ExactFinitePresentReopenWitness),
}

/// Result of extending an exact finite present with newly declared history under the same
/// currently protected continuations.
///
/// The update is derived data only. It retains no mutable memory and refuses a proposed
/// extension that rewrites the already checked history or one of its protected observations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExactFinitePresentUpdate {
    Updated(ExactFiniteSufficientPresent),
    Reopened(ExactFinitePresentReopenWitness),
}

/// Checks `ker(presentation) subseteq ker(continuation)` for every protected continuation.
pub fn derive_exact_finite_sufficient_present(
    presentation: ExactFiniteSignature,
    protected: Vec<ExactProtectedContinuation>,
) -> Result<ExactFiniteSufficientPresentResult, ExactFiniteSufficientPresentError> {
    reject_duplicate_continuations(&protected)?;
    let mut factorizations = Vec::with_capacity(protected.len());
    for contract in &protected {
        match determine_through_exact(&presentation, contract.observation())? {
            ExactDeterminationResult::Exact { factorization } => {
                factorizations.push(factorization);
            }
            ExactDeterminationResult::NotDetermined { separator } => {
                return Ok(ExactFiniteSufficientPresentResult::Insufficient {
                    continuation: contract.continuation(),
                    separator,
                });
            }
        }
    }
    Ok(ExactFiniteSufficientPresentResult::Sufficient(
        ExactFiniteSufficientPresent {
            presentation,
            protected,
            factorizations,
        },
    ))
}

/// Challenges an existing sufficient present with one newly protected continuation.
///
/// Failure returns the exact histories merged by the present and separated by the new
/// continuation. No accepted history or presentation is mutated by this check.
pub fn challenge_exact_finite_sufficient_present(
    present: &ExactFiniteSufficientPresent,
    added: ExactProtectedContinuation,
) -> Result<ExactFinitePresentChallenge, ExactFiniteSufficientPresentError> {
    if present
        .protected
        .iter()
        .any(|contract| contract.continuation() == added.continuation())
    {
        return Err(
            ExactFiniteSufficientPresentError::DuplicateProtectedContinuation(added.continuation()),
        );
    }
    match determine_through_exact(&present.presentation, added.observation())? {
        ExactDeterminationResult::Exact { factorization } => {
            let mut extended = present.clone();
            extended.protected.push(added);
            extended.factorizations.push(factorization);
            Ok(ExactFinitePresentChallenge::StillSufficient(extended))
        }
        ExactDeterminationResult::NotDetermined { separator } => Ok(
            ExactFinitePresentChallenge::Reopened(ExactFinitePresentReopenWitness {
                continuation: added.continuation(),
                separator,
            }),
        ),
    }
}

/// Rechecks an exact finite present after appending declared history.
///
/// Every old history/value pair and every old protected observation must remain an exact
/// restriction of the supplied extension. A new event may therefore expose a positive separator,
/// but cannot silently rewrite the prior fold. New protected continuations are intentionally not
/// accepted here: challenge the present separately so horizon growth remains distinct from history
/// growth.
pub fn extend_exact_finite_sufficient_present(
    prior: &ExactFiniteSufficientPresent,
    presentation: ExactFiniteSignature,
    protected: Vec<ExactProtectedContinuation>,
) -> Result<ExactFinitePresentUpdate, ExactFinitePresentUpdateError> {
    if prior.presentation().context() != presentation.context() {
        return Err(ExactFinitePresentUpdateError::PresentationContextChanged);
    }
    if presentation.values().len() <= prior.presentation().values().len() {
        return Err(ExactFinitePresentUpdateError::NoNewHistory);
    }
    preserve_signature(
        prior.presentation(),
        &presentation,
        ExactFinitePresentUpdateError::PresentationHistoryChanged,
    )?;
    reject_duplicate_continuations(&protected)?;
    if protected.len() != prior.protected().len() {
        return Err(ExactFinitePresentUpdateError::ProtectedSetChanged);
    }

    for prior_contract in prior.protected() {
        let Some(updated_contract) = protected
            .iter()
            .find(|contract| contract.continuation() == prior_contract.continuation())
        else {
            return Err(ExactFinitePresentUpdateError::ProtectedSetChanged);
        };
        if updated_contract.observation().context() != prior_contract.observation().context() {
            return Err(
                ExactFinitePresentUpdateError::ProtectedObservationContextChanged {
                    continuation: prior_contract.continuation(),
                },
            );
        }
        preserve_signature(
            prior_contract.observation(),
            updated_contract.observation(),
            |history| ExactFinitePresentUpdateError::ProtectedHistoryChanged {
                continuation: prior_contract.continuation(),
                history,
            },
        )?;
    }

    match derive_exact_finite_sufficient_present(presentation, protected)? {
        ExactFiniteSufficientPresentResult::Sufficient(present) => {
            Ok(ExactFinitePresentUpdate::Updated(present))
        }
        ExactFiniteSufficientPresentResult::Insufficient {
            continuation,
            separator,
        } => Ok(ExactFinitePresentUpdate::Reopened(
            ExactFinitePresentReopenWitness {
                continuation,
                separator,
            },
        )),
    }
}

fn preserve_signature<E>(
    prior: &ExactFiniteSignature,
    extended: &ExactFiniteSignature,
    changed: impl Fn(ArtifactRef) -> E,
) -> Result<(), E> {
    for (history, value) in prior.values() {
        if extended.values().get(history) != Some(value) {
            return Err(changed(*history));
        }
    }
    Ok(())
}

fn reject_duplicate_continuations(
    protected: &[ExactProtectedContinuation],
) -> Result<(), ExactFiniteSufficientPresentError> {
    let mut seen = BTreeSet::new();
    for contract in protected {
        if !seen.insert(contract.continuation()) {
            return Err(
                ExactFiniteSufficientPresentError::DuplicateProtectedContinuation(
                    contract.continuation(),
                ),
            );
        }
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum ExactFiniteSufficientPresentError {
    #[error(transparent)]
    Determination(#[from] ExactDeterminationError),
    #[error("protected continuation {0} is declared more than once")]
    DuplicateProtectedContinuation(ProtectedContinuationRef),
}

/// Failures while extending a derived exact finite sufficient present.
#[derive(Debug, Error)]
pub enum ExactFinitePresentUpdateError {
    #[error("the proposed presentation changes indexed signature context")]
    PresentationContextChanged,
    #[error("the proposed extension contains no newly declared history")]
    NoNewHistory,
    #[error("the proposed presentation rewrites or removes prior history {0}")]
    PresentationHistoryChanged(ArtifactRef),
    #[error("the proposed update changes the protected continuation set")]
    ProtectedSetChanged,
    #[error("protected continuation {continuation} changes indexed signature context")]
    ProtectedObservationContextChanged {
        continuation: ProtectedContinuationRef,
    },
    #[error("protected continuation {continuation} rewrites or removes prior history {history}")]
    ProtectedHistoryChanged {
        continuation: ProtectedContinuationRef,
        history: ArtifactRef,
    },
    #[error(transparent)]
    SufficientPresent(#[from] ExactFiniteSufficientPresentError),
}

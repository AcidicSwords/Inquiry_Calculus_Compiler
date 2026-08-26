//! Exact finite sufficient-present checking under protected continuations.
//!
//! A present is sufficient only when every declared protected continuation factors through its
//! finite signature. A later continuation that splits one existing present class is returned as
//! a positive reopen witness. This is derived checker data, not active mutable memory, standing,
//! a compression licence, or a claim of bounded size beyond the supplied exact finite domain.

use std::collections::BTreeSet;

use thiserror::Error;

use crate::{
    ExactDeterminationError, ExactDeterminationResult, ExactFactorization, ExactFiniteSignature,
    KernelSeparator, ProtectedContinuationRef, determine_through_exact,
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

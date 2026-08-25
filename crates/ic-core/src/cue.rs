//! Exact finite sufficient-discriminator basis checking.
//!
//! This module checks the finite, total, deterministic instance of a sufficient discriminator
//! basis. It returns a concrete protected pair whenever the supplied basis fails to separate one.
//! The signature tables are caller-certified exact data; this module neither establishes that
//! certification nor selects a resource-minimal basis.

use std::collections::BTreeSet;

use thiserror::Error;

use crate::{ArtifactRef, ExactFiniteSignature, SignatureContext};

/// A concrete protectedly distinct pair that every supplied cue answers identically.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteCueSeparator {
    first_domain: ArtifactRef,
    second_domain: ArtifactRef,
    first_protected_value: ArtifactRef,
    second_protected_value: ArtifactRef,
    cue_answers: Vec<ArtifactRef>,
}

impl FiniteCueSeparator {
    /// Returns the first live residual value.
    #[must_use]
    pub const fn first_domain(&self) -> ArtifactRef {
        self.first_domain
    }

    /// Returns the second live residual value.
    #[must_use]
    pub const fn second_domain(&self) -> ArtifactRef {
        self.second_domain
    }

    /// Returns the protected answer of the first value.
    #[must_use]
    pub const fn first_protected_value(&self) -> ArtifactRef {
        self.first_protected_value
    }

    /// Returns the protected answer of the second value.
    #[must_use]
    pub const fn second_protected_value(&self) -> ArtifactRef {
        self.second_protected_value
    }

    /// Returns the common answer from every cue, in declared cue order.
    #[must_use]
    pub fn cue_answers(&self) -> &[ArtifactRef] {
        &self.cue_answers
    }
}

/// Result of checking one declared exact finite discriminator basis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExactFiniteCueBasisResult {
    /// Every protectedly distinct pair is separated by at least one supplied cue.
    Sufficient,
    /// A protectedly distinct pair remains indistinguishable to every supplied cue.
    Insufficient { separator: FiniteCueSeparator },
}

/// Checks finite discriminator-basis sufficiency under caller-certified exact signatures.
///
/// `protected` partitions the residual domain at the declared horizon. Every cue must share its
/// binding, scope, applicability, grain, horizon, domain type, and exact finite domain with it.
/// An empty basis is valid precisely when the protected signature is constant. The result is only
/// about the supplied exact tables: it does not establish support, applicability, coverage,
/// resource minimality, an optimal query policy, or a broader impossibility claim.
pub fn check_exact_finite_cue_basis(
    cues: &[ExactFiniteSignature],
    protected: &ExactFiniteSignature,
) -> Result<ExactFiniteCueBasisResult, ExactFiniteCueBasisError> {
    let protected_domain: BTreeSet<_> = protected.values().keys().copied().collect();
    for (index, cue) in cues.iter().enumerate() {
        if cue.context() != protected.context() {
            return Err(ExactFiniteCueBasisError::ContextMismatch {
                cue_index: index,
                expected: Box::new(protected.context()),
                actual: Box::new(cue.context()),
            });
        }
        let cue_domain: BTreeSet<_> = cue.values().keys().copied().collect();
        if cue_domain != protected_domain {
            return Err(ExactFiniteCueBasisError::DomainMismatch { cue_index: index });
        }
    }

    let entries: Vec<_> = protected.values().iter().collect();
    for (first_index, (first_domain, first_protected_value)) in entries.iter().enumerate() {
        for (second_domain, second_protected_value) in entries.iter().skip(first_index + 1) {
            if first_protected_value == second_protected_value {
                continue;
            }
            let mut answers = Vec::with_capacity(cues.len());
            let mut separated = false;
            for cue in cues {
                let first_answer = cue.values()[first_domain];
                let second_answer = cue.values()[second_domain];
                answers.push(first_answer);
                if first_answer != second_answer {
                    separated = true;
                    break;
                }
            }
            if !separated {
                return Ok(ExactFiniteCueBasisResult::Insufficient {
                    separator: FiniteCueSeparator {
                        first_domain: **first_domain,
                        second_domain: **second_domain,
                        first_protected_value: **first_protected_value,
                        second_protected_value: **second_protected_value,
                        cue_answers: answers,
                    },
                });
            }
        }
    }
    Ok(ExactFiniteCueBasisResult::Sufficient)
}

/// Errors from comparing the declared exact signature contexts of a cue basis.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ExactFiniteCueBasisError {
    #[error("cue {cue_index} has a different exact signature context")]
    ContextMismatch {
        /// Position of the mismatched cue in the declared basis.
        cue_index: usize,
        /// Context required by the protected signature.
        expected: Box<SignatureContext>,
        /// Context carried by the mismatched cue.
        actual: Box<SignatureContext>,
    },

    #[error("cue {cue_index} has a different exact finite domain")]
    DomainMismatch {
        /// Position of the mismatched cue in the declared basis.
        cue_index: usize,
    },
}

//! Derived protected-recovery status and exact finite constancy checking.
//!
//! These values neither construct a return fiber nor decide that a declaration denotes a
//! complete one.  A caller may use [`check_exact_fiber_recovery`] only after independently
//! establishing that the supplied finite signature covers one same-use return fiber under its
//! declared context.  Incomplete coverage remains [`RecoveryStatusIR::Unknown`], never an
//! inferred recovery failure.

use crate::{ArtifactRef, ExactFiniteSignature, QueryRef};

/// The executable three-valued recovery record required for an occurrence.
///
/// This is derived result data, not an artifact kind, authoritative history record, or semantic
/// admission.  The evidence references remain explicit so that later occurrence/replay work can
/// validate them in its own context.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryStatusIR {
    /// Exact recovery is supported by a declared recovery certificate.
    Recovered { certificate: ArtifactRef },
    /// Protected non-recovery is supported by a positive separator witness.
    NotRecovered { separator: ArtifactRef },
    /// Evidence, coverage, or decision capability is missing.
    Unknown { residual: QueryRef },
}

/// A positive finite witness that two candidates in an exactly covered return fiber differ in
/// their protected signatures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecoverySeparator {
    first_candidate: ArtifactRef,
    second_candidate: ArtifactRef,
    first_signature: ArtifactRef,
    second_signature: ArtifactRef,
}

impl RecoverySeparator {
    #[must_use]
    pub const fn first_candidate(self) -> ArtifactRef {
        self.first_candidate
    }

    #[must_use]
    pub const fn second_candidate(self) -> ArtifactRef {
        self.second_candidate
    }

    #[must_use]
    pub const fn first_signature(self) -> ArtifactRef {
        self.first_signature
    }

    #[must_use]
    pub const fn second_signature(self) -> ArtifactRef {
        self.second_signature
    }
}

/// The result of checking a complete finite protected-signature table for constancy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExactFiberRecovery {
    /// Every candidate in the declared fiber has this protected signature.
    Recovered { protected_signature: ArtifactRef },
    /// Two candidates in the declared fiber have different protected signatures.
    NotRecovered { separator: RecoverySeparator },
}

/// Checks protected-signature constancy on an independently certified exact finite return fiber.
///
/// The signature's domain is the complete fiber and its codomain values are protected
/// signatures.  An empty input is rejected: a same-use return fiber for an admitted incidence
/// contains its source, so vacuous constancy is not evidence of recovery.
pub fn check_exact_fiber_recovery(
    protected_signatures: &ExactFiniteSignature,
) -> Result<ExactFiberRecovery, ExactFiberRecoveryError> {
    let mut entries = protected_signatures.values().iter();
    let Some((first_candidate, first_signature)) = entries.next() else {
        return Err(ExactFiberRecoveryError::EmptyFiber);
    };

    for (candidate, signature) in entries {
        if signature != first_signature {
            return Ok(ExactFiberRecovery::NotRecovered {
                separator: RecoverySeparator {
                    first_candidate: *first_candidate,
                    second_candidate: *candidate,
                    first_signature: *first_signature,
                    second_signature: *signature,
                },
            });
        }
    }

    Ok(ExactFiberRecovery::Recovered {
        protected_signature: *first_signature,
    })
}

/// Errors from exact finite protected-recovery checking.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExactFiberRecoveryError {
    /// The alleged fiber has no candidates, so it cannot witness recovery of an incidence.
    EmptyFiber,
}

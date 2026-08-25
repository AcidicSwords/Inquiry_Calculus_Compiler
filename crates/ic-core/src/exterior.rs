//! Derived, use-tagged exterior-claim views.
//!
//! A positive-negation filling is a role of an admitted relation incidence, not a new carrier.
//! This view therefore preserves the source, candidate, departure-witness, use, and
//! occurrence-side execution-coverage identities without storing, evaluating, or admitting an
//! incidence.

use thiserror::Error;

use crate::{
    DepartureCatalog, DepartureWitness, DepartureWitnessCheckError, DepartureWitnessError,
    DepartureWitnessRef, GeneratorCoverageRef, NegationCatalog, NegationUse, NegationUseCheckError,
    NegationUseError, NegationUseRef, TypedFormRef,
};

/// Catalog required to validate a derived tagged exterior claim.
pub trait TaggedExteriorCatalog: NegationCatalog + DepartureCatalog {
    /// Resolves one oriented negation-use declaration.
    fn resolve_negation_use(&self, reference: NegationUseRef) -> Option<NegationUse>;

    /// Resolves one positive-departure witness declaration.
    fn resolve_departure_witness(&self, reference: DepartureWitnessRef)
    -> Option<DepartureWitness>;
}

/// A derived candidate for the tagged positive-exterior role.
///
/// It retains the use tag even when two uses name the same candidate. `execution_coverage` is
/// occurrence-side data and remains distinct from the use's declared semantic coverage.
/// Construction and checking do not assert that the candidate belongs to the negation relation,
/// that the use is admitted, or that the candidate is actualized or warranted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TaggedExteriorClaim {
    negation_use: NegationUseRef,
    source: TypedFormRef,
    candidate: TypedFormRef,
    departure_witness: DepartureWitnessRef,
    execution_coverage: GeneratorCoverageRef,
}

impl TaggedExteriorClaim {
    /// Creates one derived tagged exterior claim.
    #[must_use]
    pub const fn new(
        negation_use: NegationUseRef,
        source: TypedFormRef,
        candidate: TypedFormRef,
        departure_witness: DepartureWitnessRef,
        execution_coverage: GeneratorCoverageRef,
    ) -> Self {
        Self {
            negation_use,
            source,
            candidate,
            departure_witness,
            execution_coverage,
        }
    }

    /// Returns the immutable negation-use identity retained by this claim.
    #[must_use]
    pub const fn negation_use(self) -> NegationUseRef {
        self.negation_use
    }

    /// Returns the source form under the selected orientation.
    #[must_use]
    pub const fn source(self) -> TypedFormRef {
        self.source
    }

    /// Returns the candidate proposed for the exterior role.
    #[must_use]
    pub const fn candidate(self) -> TypedFormRef {
        self.candidate
    }

    /// Returns the named positive-departure witness declaration.
    #[must_use]
    pub const fn departure_witness(self) -> DepartureWitnessRef {
        self.departure_witness
    }

    /// Returns the separately supplied occurrence-side execution-coverage identity.
    #[must_use]
    pub const fn execution_coverage(self) -> GeneratorCoverageRef {
        self.execution_coverage
    }

    /// Checks dependency identity and shared determination context.
    ///
    /// This is deliberately not relation evaluation. In particular, it does not establish
    /// `N_u(source, candidate)`, positive negation, an exterior, coverage, actuality, or warrant.
    pub fn check<C: TaggedExteriorCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), TaggedExteriorClaimError> {
        let negation_use = catalog.resolve_negation_use(self.negation_use).ok_or(
            TaggedExteriorClaimError::UnresolvedNegationUse(self.negation_use),
        )?;
        let calculated_use = negation_use.negation_use_ref()?;
        if calculated_use != self.negation_use {
            return Err(TaggedExteriorClaimError::NegationUseIdentityMismatch {
                reference: self.negation_use,
                calculated: calculated_use,
            });
        }
        negation_use.check(catalog)?;

        let departure_witness = catalog
            .resolve_departure_witness(self.departure_witness)
            .ok_or(TaggedExteriorClaimError::UnresolvedDepartureWitness(
                self.departure_witness,
            ))?;
        let calculated_witness = departure_witness.departure_witness_ref()?;
        if calculated_witness != self.departure_witness {
            return Err(TaggedExteriorClaimError::DepartureWitnessIdentityMismatch {
                reference: self.departure_witness,
                calculated: calculated_witness,
            });
        }
        departure_witness.check(catalog)?;

        if departure_witness.source() != self.source {
            return Err(TaggedExteriorClaimError::DepartureWitnessMismatch("source"));
        }
        if departure_witness.candidate() != self.candidate {
            return Err(TaggedExteriorClaimError::DepartureWitnessMismatch(
                "candidate",
            ));
        }
        if departure_witness.distinction() != negation_use.distinction() {
            return Err(TaggedExteriorClaimError::ContextMismatch("distinction"));
        }
        if departure_witness.source_presentation() != negation_use.source_determination() {
            return Err(TaggedExteriorClaimError::ContextMismatch(
                "source presentation",
            ));
        }
        if departure_witness.scope() != negation_use.scope() {
            return Err(TaggedExteriorClaimError::ContextMismatch("scope"));
        }
        if departure_witness.applicability() != negation_use.applicability() {
            return Err(TaggedExteriorClaimError::ContextMismatch("applicability"));
        }
        if departure_witness.grain() != negation_use.grain() {
            return Err(TaggedExteriorClaimError::ContextMismatch("grain"));
        }
        Ok(())
    }
}

/// Errors from validating a derived tagged exterior claim.
#[derive(Debug, Error)]
pub enum TaggedExteriorClaimError {
    #[error(transparent)]
    NegationUseEncoding(#[from] NegationUseError),

    #[error(transparent)]
    NegationUse(#[from] NegationUseCheckError),

    #[error(transparent)]
    DepartureWitnessEncoding(#[from] DepartureWitnessError),

    #[error(transparent)]
    DepartureWitness(#[from] DepartureWitnessCheckError),

    #[error("negation use {0} is not available from the declared catalog")]
    UnresolvedNegationUse(NegationUseRef),

    #[error("catalog negation use {reference} hashes to {calculated}, not its claimed identity")]
    NegationUseIdentityMismatch {
        /// Claimed negation-use identity.
        reference: NegationUseRef,
        /// Identity calculated from the resolved declaration.
        calculated: NegationUseRef,
    },

    #[error("departure witness {0} is not available from the declared catalog")]
    UnresolvedDepartureWitness(DepartureWitnessRef),

    #[error(
        "catalog departure witness {reference} hashes to {calculated}, not its claimed identity"
    )]
    DepartureWitnessIdentityMismatch {
        /// Claimed departure-witness identity.
        reference: DepartureWitnessRef,
        /// Identity calculated from the resolved declaration.
        calculated: DepartureWitnessRef,
    },

    #[error("departure witness does not match this exterior claim's {0}")]
    DepartureWitnessMismatch(&'static str),

    #[error("departure witness and negation use disagree on {0}")]
    ContextMismatch(&'static str),
}

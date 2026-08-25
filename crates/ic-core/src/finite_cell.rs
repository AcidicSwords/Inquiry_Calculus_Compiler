//! Exact finite observation-cell comparison.
//!
//! This is the finite deterministic instance of the constitutive-separator observation used by
//! positive departure. It is derived checker data only: a separator is not itself a
//! [`DepartureWitness`](crate::DepartureWitness), does not establish relation membership or
//! support, and cannot make an unknown observation exterior.

use std::collections::BTreeSet;

use thiserror::Error;

use crate::ArtifactRef;

/// One coordinate of a finite represented observation.
///
/// A missing observation is kept distinct from every observed value. In particular, it is not a
/// negative value, an equality result, or evidence of interiority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FiniteObservation {
    /// A value established through the declared finite observation route.
    Observed(ArtifactRef),
    /// The coordinate was not established under the declared observation route.
    Unknown,
}

/// A positive coordinate at which two finite cells differ by observed values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FiniteCellSeparator {
    coordinate: usize,
    source_value: ArtifactRef,
    candidate_value: ArtifactRef,
}

impl FiniteCellSeparator {
    #[must_use]
    pub const fn coordinate(self) -> usize {
        self.coordinate
    }

    #[must_use]
    pub const fn source_value(self) -> ArtifactRef {
        self.source_value
    }

    #[must_use]
    pub const fn candidate_value(self) -> ArtifactRef {
        self.candidate_value
    }
}

/// The finite comparison result for one pair of candidate cells.
///
/// `SameObservedCell` says only that every coordinate in this declared table was observed and
/// equal. It is not an interiority, equivalence, coverage, or no-departure result outside that
/// table.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FiniteCellComparison {
    /// One observed coordinate positively separates the two cells.
    Separated(FiniteCellSeparator),
    /// All declared coordinates were observed and equal.
    SameObservedCell,
    /// No positive separator was observed and at least one coordinate remains unknown.
    Unknown,
}

/// Compares two finite observation cells without turning missing evidence into a conclusion.
///
/// A positive difference wins over unrelated unknown coordinates because it is already an
/// explicit separator. If no such difference exists, any unknown coordinate keeps the result
/// `Unknown`. The caller must still establish that the observation route is relevant to a live
/// determination and that the differing answers are related by a standing incompatibility use
/// before treating the separator as departure evidence.
pub fn compare_finite_observation_cells(
    source: &[FiniteObservation],
    candidate: &[FiniteObservation],
) -> Result<FiniteCellComparison, FiniteCellError> {
    if source.len() != candidate.len() {
        return Err(FiniteCellError::CoordinateCountMismatch {
            source_coordinates: source.len(),
            candidate_coordinates: candidate.len(),
        });
    }

    let mut has_unknown = false;
    for (coordinate, (source, candidate)) in source.iter().zip(candidate).enumerate() {
        match (source, candidate) {
            (
                FiniteObservation::Observed(source_value),
                FiniteObservation::Observed(candidate_value),
            ) if source_value != candidate_value => {
                return Ok(FiniteCellComparison::Separated(FiniteCellSeparator {
                    coordinate,
                    source_value: *source_value,
                    candidate_value: *candidate_value,
                }));
            }
            (FiniteObservation::Unknown, _) | (_, FiniteObservation::Unknown) => {
                has_unknown = true;
            }
            (FiniteObservation::Observed(_), FiniteObservation::Observed(_)) => {}
        }
    }

    Ok(if has_unknown {
        FiniteCellComparison::Unknown
    } else {
        FiniteCellComparison::SameObservedCell
    })
}

/// Errors from finite observation-cell comparison.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum FiniteCellError {
    #[error(
        "source has {source_coordinates} observation coordinates but candidate has {candidate_coordinates}"
    )]
    CoordinateCountMismatch {
        source_coordinates: usize,
        candidate_coordinates: usize,
    },
}

/// One directly listed ordered pair in a finite incompatibility relation.
///
/// The pair is only a member of its caller-declared finite table. It does not by itself establish
/// that the relation is typed, standing, relevant to a determination, or supported by a warrant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FiniteIncompatibilityWitness {
    source_value: ArtifactRef,
    candidate_value: ArtifactRef,
}

impl FiniteIncompatibilityWitness {
    #[must_use]
    pub const fn source_value(self) -> ArtifactRef {
        self.source_value
    }

    #[must_use]
    pub const fn candidate_value(self) -> ArtifactRef {
        self.candidate_value
    }
}

/// A finite, ordered table of positively declared incompatible observation-value pairs.
///
/// Absence from this table remains absence of a witness, not compatibility, equality,
/// interiority, or no-departure evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteIncompatibilityTable {
    pairs: BTreeSet<(ArtifactRef, ArtifactRef)>,
}

impl FiniteIncompatibilityTable {
    /// Creates one finite table, rejecting duplicated declarations that would add no distinction.
    pub fn new(pairs: Vec<(ArtifactRef, ArtifactRef)>) -> Result<Self, FiniteIncompatibilityError> {
        let mut declared = BTreeSet::new();
        for pair in pairs {
            if !declared.insert(pair) {
                return Err(FiniteIncompatibilityError::DuplicatePair {
                    source_value: pair.0,
                    candidate_value: pair.1,
                });
            }
        }
        Ok(Self { pairs: declared })
    }

    #[must_use]
    pub const fn pairs(&self) -> &BTreeSet<(ArtifactRef, ArtifactRef)> {
        &self.pairs
    }
}

/// Result of checking two finite observations against a finite incompatibility table.
///
/// Only [`FiniteIncompatibilityResult::Incompatible`] carries a positive pair witness. A missing
/// observation remains `Unknown`, and an observed pair absent from the table remains `NoWitness`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FiniteIncompatibilityResult {
    /// The observed ordered pair appears in the declared finite table.
    Incompatible(FiniteIncompatibilityWitness),
    /// Both values are observed but this table supplies no incompatibility witness for the pair.
    NoWitness,
    /// At least one value is not established through the declared observation route.
    Unknown,
}

/// Finds a positive finite incompatibility witness without inferring a negative conclusion.
///
/// This is a derived finite checker. It does not turn a table membership into a standing relation
/// use, type-check either value, establish observation provenance, certify table coverage, or
/// produce a [`DepartureWitness`](crate::DepartureWitness).
#[must_use]
pub fn check_finite_incompatibility(
    table: &FiniteIncompatibilityTable,
    source: FiniteObservation,
    candidate: FiniteObservation,
) -> FiniteIncompatibilityResult {
    let (FiniteObservation::Observed(source_value), FiniteObservation::Observed(candidate_value)) =
        (source, candidate)
    else {
        return FiniteIncompatibilityResult::Unknown;
    };
    if table.pairs.contains(&(source_value, candidate_value)) {
        FiniteIncompatibilityResult::Incompatible(FiniteIncompatibilityWitness {
            source_value,
            candidate_value,
        })
    } else {
        FiniteIncompatibilityResult::NoWitness
    }
}

/// Errors from constructing a finite incompatibility table.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum FiniteIncompatibilityError {
    #[error("finite incompatibility table repeats ({source_value}, {candidate_value})")]
    DuplicatePair {
        source_value: ArtifactRef,
        candidate_value: ArtifactRef,
    },
}

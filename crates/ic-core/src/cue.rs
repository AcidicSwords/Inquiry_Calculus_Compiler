//! Exact finite sufficient-discriminator basis checking.
//!
//! This module checks the finite, total, deterministic instance of a sufficient discriminator
//! basis. It returns a concrete protected pair whenever the supplied basis fails to separate one.
//! The signature tables are caller-certified exact data; this module neither establishes that
//! certification, generate candidate bases, or claim the supplied set is exhaustive.

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

/// One caller-supplied candidate basis, addressed by indices in the declared cue sequence.
///
/// The resource identity is opaque and receives its order only from a separately declared
/// [`FiniteResourcePreorder`]. The candidate order is intentionally not an enumeration claim.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ExactFiniteCueBasisCandidate {
    cue_indices: Vec<usize>,
    resource: ArtifactRef,
}

impl ExactFiniteCueBasisCandidate {
    /// Creates a candidate basis with strictly increasing declared cue indices.
    pub fn new(
        cue_indices: Vec<usize>,
        resource: ArtifactRef,
    ) -> Result<Self, ExactFiniteCueFrontierError> {
        if cue_indices.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(ExactFiniteCueFrontierError::NonCanonicalCueIndices);
        }
        Ok(Self {
            cue_indices,
            resource,
        })
    }

    /// Returns indices into the declared cue sequence.
    #[must_use]
    pub fn cue_indices(&self) -> &[usize] {
        &self.cue_indices
    }

    /// Returns the binding-supplied resource identity for this candidate.
    #[must_use]
    pub const fn resource(&self) -> ArtifactRef {
        self.resource
    }
}

/// A finite declared preorder over opaque resource identities.
///
/// Every relation retained here is an assertion by the caller's binding. Construction rejects
/// duplicate edges; checking the order against a finite resource set requires reflexivity and
/// transitivity over exactly that supplied set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteResourcePreorder {
    less_or_equal: BTreeSet<(ArtifactRef, ArtifactRef)>,
}

impl FiniteResourcePreorder {
    /// Constructs one finite declared preorder relation.
    pub fn new(
        less_or_equal: Vec<(ArtifactRef, ArtifactRef)>,
    ) -> Result<Self, ExactFiniteCueFrontierError> {
        let mut edges = BTreeSet::new();
        for edge in less_or_equal {
            if !edges.insert(edge) {
                return Err(ExactFiniteCueFrontierError::DuplicateResourceOrderEdge {
                    lower: edge.0,
                    upper: edge.1,
                });
            }
        }
        Ok(Self {
            less_or_equal: edges,
        })
    }

    /// Returns the explicitly declared resource-order edges.
    #[must_use]
    pub const fn less_or_equal(&self) -> &BTreeSet<(ArtifactRef, ArtifactRef)> {
        &self.less_or_equal
    }

    fn has_edge(&self, lower: ArtifactRef, upper: ArtifactRef) -> bool {
        self.less_or_equal.contains(&(lower, upper))
    }

    fn check_over(
        &self,
        resources: &BTreeSet<ArtifactRef>,
    ) -> Result<(), ExactFiniteCueFrontierError> {
        for resource in resources {
            if !self.has_edge(*resource, *resource) {
                return Err(ExactFiniteCueFrontierError::NonReflexiveResource(*resource));
            }
        }
        for lower in resources {
            for middle in resources {
                for upper in resources {
                    if self.has_edge(*lower, *middle)
                        && self.has_edge(*middle, *upper)
                        && !self.has_edge(*lower, *upper)
                    {
                        return Err(ExactFiniteCueFrontierError::NonTransitiveResourceOrder {
                            lower: *lower,
                            middle: *middle,
                            upper: *upper,
                        });
                    }
                }
            }
        }
        Ok(())
    }
}

/// An exact candidate rejected because it fails the declared sufficient-basis condition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InsufficientExactFiniteCueBasis {
    candidate: ExactFiniteCueBasisCandidate,
    separator: FiniteCueSeparator,
}

impl InsufficientExactFiniteCueBasis {
    /// Returns the candidate basis that failed.
    #[must_use]
    pub const fn candidate(&self) -> &ExactFiniteCueBasisCandidate {
        &self.candidate
    }

    /// Returns the concrete protected separator retained from that failure.
    #[must_use]
    pub const fn separator(&self) -> &FiniteCueSeparator {
        &self.separator
    }
}

/// Nondominated sufficient candidates from one finite, caller-supplied candidate set.
///
/// This is not a proof that every possible cue basis was supplied. `members` are minimal only
/// relative to the supplied candidates and their declared preorder; `insufficient` retains
/// positive residual separators rather than treating them as an impossibility result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFiniteCueFrontier {
    members: Vec<ExactFiniteCueBasisCandidate>,
    insufficient: Vec<InsufficientExactFiniteCueBasis>,
}

impl ExactFiniteCueFrontier {
    /// Returns all nondominated sufficient candidates in the caller's original order.
    #[must_use]
    pub fn members(&self) -> &[ExactFiniteCueBasisCandidate] {
        &self.members
    }

    /// Returns failed candidates with their concrete protected separators.
    #[must_use]
    pub fn insufficient(&self) -> &[InsufficientExactFiniteCueBasis] {
        &self.insufficient
    }
}

/// Selects nondominated sufficient bases from one finite, caller-supplied candidate set.
///
/// Every candidate is first checked by [`check_exact_finite_cue_basis`]. The order is validated
/// over all candidate resource identities, then a candidate is removed only when another
/// sufficient candidate is strictly lower (`other <= candidate` but not conversely). This does
/// not generate candidates, establish that the input set is exhaustive, certify resource facts,
/// or convert a frontier with no sufficient member into impossibility.
pub fn select_nondominated_exact_finite_cue_bases(
    cues: &[ExactFiniteSignature],
    protected: &ExactFiniteSignature,
    candidates: &[ExactFiniteCueBasisCandidate],
    resources: &FiniteResourcePreorder,
) -> Result<ExactFiniteCueFrontier, ExactFiniteCueFrontierError> {
    let mut seen = BTreeSet::new();
    let mut resource_set = BTreeSet::new();
    for candidate in candidates {
        if !seen.insert(candidate.clone()) {
            return Err(ExactFiniteCueFrontierError::DuplicateCandidate(
                candidate.clone(),
            ));
        }
        for index in candidate.cue_indices() {
            if *index >= cues.len() {
                return Err(ExactFiniteCueFrontierError::CueIndexOutOfRange {
                    index: *index,
                    cue_count: cues.len(),
                });
            }
        }
        resource_set.insert(candidate.resource());
    }
    resources.check_over(&resource_set)?;

    let mut sufficient = Vec::new();
    let mut insufficient = Vec::new();
    for candidate in candidates {
        let selected: Vec<_> = candidate
            .cue_indices()
            .iter()
            .map(|index| cues[*index].clone())
            .collect();
        match check_exact_finite_cue_basis(&selected, protected)? {
            ExactFiniteCueBasisResult::Sufficient => sufficient.push(candidate.clone()),
            ExactFiniteCueBasisResult::Insufficient { separator } => {
                insufficient.push(InsufficientExactFiniteCueBasis {
                    candidate: candidate.clone(),
                    separator,
                });
            }
        }
    }

    let members = sufficient
        .iter()
        .filter(|candidate| {
            !sufficient.iter().any(|other| {
                other != *candidate
                    && resources.has_edge(other.resource(), candidate.resource())
                    && !resources.has_edge(candidate.resource(), other.resource())
            })
        })
        .cloned()
        .collect();
    Ok(ExactFiniteCueFrontier {
        members,
        insufficient,
    })
}

/// Errors from finite candidate-basis frontier selection.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ExactFiniteCueFrontierError {
    #[error("candidate cue indices must be strictly increasing without duplicates")]
    NonCanonicalCueIndices,

    #[error("resource preorder repeats edge {lower} <= {upper}")]
    DuplicateResourceOrderEdge {
        /// Lower resource identity in the repeated edge.
        lower: ArtifactRef,
        /// Upper resource identity in the repeated edge.
        upper: ArtifactRef,
    },

    #[error("candidate repeats an identical cue basis and resource")]
    DuplicateCandidate(ExactFiniteCueBasisCandidate),

    #[error("candidate names cue index {index}, but only {cue_count} cues are declared")]
    CueIndexOutOfRange {
        /// Out-of-range cue index.
        index: usize,
        /// Declared cue count.
        cue_count: usize,
    },

    #[error("resource preorder is missing reflexive edge {0} <= {0}")]
    NonReflexiveResource(ArtifactRef),

    #[error("resource preorder is not transitive: {lower} <= {middle} <= {upper}")]
    NonTransitiveResourceOrder {
        /// Lower resource identity.
        lower: ArtifactRef,
        /// Middle resource identity.
        middle: ArtifactRef,
        /// Upper resource identity.
        upper: ArtifactRef,
    },

    #[error(transparent)]
    CueBasis(#[from] ExactFiniteCueBasisError),
}

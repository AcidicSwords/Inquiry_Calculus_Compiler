//! Exact finite observation-cell comparison.
//!
//! This is the finite deterministic instance of the constitutive-separator observation used by
//! positive departure. It is derived checker data only: a separator is not itself a
//! [`DepartureWitness`](crate::DepartureWitness), does not establish relation membership or
//! support, and cannot make an unknown observation exterior.

use std::collections::BTreeSet;

use thiserror::Error;

use crate::{
    ArtifactRef, FormulaCatalog, RelationCatalog, RelationUse, RelationUseCheckError,
    RelationUseError, RelationUseRef, TypeCatalog, TypeCheckError, TypeError, TypeSymbol,
    TypedForm, TypedFormRef,
};

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

/// The minimal catalog needed to resolve a finite typed observation without making that
/// observation a formula, a relation result, or an actual event.
pub trait FiniteTypedObservationCatalog: TypeCatalog {
    /// Resolves a typed-form declaration by its claimed stable identity.
    fn resolve_typed_form(&self, reference: TypedFormRef) -> Option<TypedForm>;
}

impl<C: FormulaCatalog + ?Sized> FiniteTypedObservationCatalog for C {
    fn resolve_typed_form(&self, reference: TypedFormRef) -> Option<TypedForm> {
        FormulaCatalog::resolve_typed_form(self, reference)
    }
}

/// One finite observation whose established value is a checked typed-form declaration.
///
/// `Unknown` remains distinct from every typed value. It is not an observation result, a
/// negative answer, or evidence that a source or candidate lies inside a determination.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypedFiniteObservation {
    /// A value represented by a typed-form declaration.
    Observed(TypedFormRef),
    /// The value has not been established through the declared route.
    Unknown,
}

/// One positively declared ordered incompatible pair of checked typed-form declarations.
///
/// This witness establishes only membership in its caller-declared finite table after both
/// declarations have been resolved, rehashed, and type-checked. It is not a standing
/// incompatibility relation, support, relevance, coverage, or departure certificate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypedFiniteIncompatibilityWitness {
    source_value: TypedFormRef,
    candidate_value: TypedFormRef,
}

impl TypedFiniteIncompatibilityWitness {
    /// Returns the source typed-form declaration.
    #[must_use]
    pub const fn source_value(self) -> TypedFormRef {
        self.source_value
    }

    /// Returns the candidate typed-form declaration.
    #[must_use]
    pub const fn candidate_value(self) -> TypedFormRef {
        self.candidate_value
    }
}

/// A finite ordered table of incompatibility pairs whose values must be checked typed forms.
///
/// The two sides may have different declared types: the canonical departure contract permits
/// observations into distinct codomains. This table does not itself admit a cross-type relation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedFiniteIncompatibilityTable {
    pairs: BTreeSet<(TypedFormRef, TypedFormRef)>,
}

impl TypedFiniteIncompatibilityTable {
    /// Creates one finite typed table, rejecting duplicate pair declarations.
    pub fn new(
        pairs: Vec<(TypedFormRef, TypedFormRef)>,
    ) -> Result<Self, TypedFiniteIncompatibilityError> {
        let mut declared = BTreeSet::new();
        for pair in pairs {
            if !declared.insert(pair) {
                return Err(TypedFiniteIncompatibilityError::DuplicatePair {
                    source_value: pair.0,
                    candidate_value: pair.1,
                });
            }
        }
        Ok(Self { pairs: declared })
    }

    /// Returns the declared ordered typed-form pairs.
    #[must_use]
    pub const fn pairs(&self) -> &BTreeSet<(TypedFormRef, TypedFormRef)> {
        &self.pairs
    }

    /// Resolves, rehashes, and type-checks every declaration retained by this finite table.
    ///
    /// This validates representation identity, not observation provenance or semantic admission.
    pub fn check<C: FiniteTypedObservationCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), TypedFiniteIncompatibilityError> {
        for (source_value, candidate_value) in &self.pairs {
            check_typed_finite_form(catalog, "table source", *source_value)?;
            check_typed_finite_form(catalog, "table candidate", *candidate_value)?;
        }
        Ok(())
    }
}

/// Result of checking a finite typed observation pair against a finite typed table.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypedFiniteIncompatibilityResult {
    /// The observed typed-form pair appears in the declared, checked finite table.
    Incompatible(TypedFiniteIncompatibilityWitness),
    /// Both typed values are established but the table supplies no pair witness.
    NoWitness,
    /// At least one typed value is not established through the declared route.
    Unknown,
}

/// A positive typed finite pair tied to the declared incompatibility relation use that binds it.
///
/// This is still only caller-declared finite evidence. The relation use is structurally checked
/// and cannot be a generated route, but this witness does not admit the relation as standing,
/// establish its support, relevance, coverage, non-circularity, or departure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypedFiniteIncompatibilityUseWitness {
    pair: TypedFiniteIncompatibilityWitness,
    incompatibility_use: RelationUseRef,
}

/// The explicit port roles used to read one ordinary relation use as an oriented incompatibility.
///
/// The roles are an input to this derived check, not a new semantic relation or persisted
/// certificate. They make the source/candidate orientation inspectable without relying on a
/// port-name convention.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedFiniteIncompatibilityRoles {
    source_port: TypeSymbol,
    candidate_port: TypeSymbol,
}

impl TypedFiniteIncompatibilityRoles {
    /// Creates distinct named source and candidate roles.
    pub fn new(
        source_port: TypeSymbol,
        candidate_port: TypeSymbol,
    ) -> Result<Self, TypedFiniteIncompatibilityRoleError> {
        if source_port == candidate_port {
            return Err(TypedFiniteIncompatibilityRoleError::DuplicateRolePort(
                source_port,
            ));
        }
        Ok(Self {
            source_port,
            candidate_port,
        })
    }

    #[must_use]
    pub const fn source_port(&self) -> &TypeSymbol {
        &self.source_port
    }

    #[must_use]
    pub const fn candidate_port(&self) -> &TypeSymbol {
        &self.candidate_port
    }
}

/// A finite incompatibility/use witness that retains the declared typed port roles.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedFiniteOrientedIncompatibilityUseWitness {
    witness: TypedFiniteIncompatibilityUseWitness,
    roles: TypedFiniteIncompatibilityRoles,
}

impl TypedFiniteOrientedIncompatibilityUseWitness {
    #[must_use]
    pub const fn witness(&self) -> &TypedFiniteIncompatibilityUseWitness {
        &self.witness
    }

    #[must_use]
    pub const fn roles(&self) -> &TypedFiniteIncompatibilityRoles {
        &self.roles
    }
}

impl TypedFiniteIncompatibilityUseWitness {
    #[must_use]
    pub const fn pair(self) -> TypedFiniteIncompatibilityWitness {
        self.pair
    }

    #[must_use]
    pub const fn incompatibility_use(self) -> RelationUseRef {
        self.incompatibility_use
    }
}

/// The outcome of checking a typed finite pair against a declared incompatibility use.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypedFiniteIncompatibilityUseResult {
    /// A listed typed pair is bound by the named non-generated incompatibility use.
    Incompatible(TypedFiniteIncompatibilityUseWitness),
    /// Both values are established, but the finite table gives no positive pair witness.
    NoWitness,
    /// At least one value remains unknown through the caller's declared route.
    Unknown,
}

/// The outcome of checking a finite incompatibility use with explicit source/candidate ports.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypedFiniteOrientedIncompatibilityUseResult {
    /// The positive pair occurs at the declared source and candidate ports, respectively.
    Incompatible(TypedFiniteOrientedIncompatibilityUseWitness),
    /// Both values are established, but the finite table gives no positive pair witness.
    NoWitness,
    /// At least one value remains unknown through the caller's declared route.
    Unknown,
}

/// The catalog needed to tie a finite incompatibility pair to one declared relation occurrence.
pub trait FiniteTypedIncompatibilityUseCatalog: RelationCatalog {
    fn resolve_relation_use(&self, reference: RelationUseRef) -> Option<RelationUse>;
}

/// Checks a typed finite pair without inferring standing incompatibility or departure.
///
/// The table and both observed declarations are resolved, rehashed, and type-checked. A listed
/// pair is still only derived finite evidence: this function does not evaluate an observation
/// route, make the table standing, establish relevance/non-circularity/coverage/support, link a
/// relation use, or construct a [`DepartureWitness`](crate::DepartureWitness).
pub fn check_typed_finite_incompatibility<C: FiniteTypedObservationCatalog>(
    table: &TypedFiniteIncompatibilityTable,
    catalog: &C,
    source: TypedFiniteObservation,
    candidate: TypedFiniteObservation,
) -> Result<TypedFiniteIncompatibilityResult, TypedFiniteIncompatibilityError> {
    table.check(catalog)?;
    let (
        TypedFiniteObservation::Observed(source_value),
        TypedFiniteObservation::Observed(candidate_value),
    ) = (source, candidate)
    else {
        return Ok(TypedFiniteIncompatibilityResult::Unknown);
    };
    check_typed_finite_form(catalog, "source observation", source_value)?;
    check_typed_finite_form(catalog, "candidate observation", candidate_value)?;
    Ok(if table.pairs.contains(&(source_value, candidate_value)) {
        TypedFiniteIncompatibilityResult::Incompatible(TypedFiniteIncompatibilityWitness {
            source_value,
            candidate_value,
        })
    } else {
        TypedFiniteIncompatibilityResult::NoWitness
    })
}

/// Checks a positive typed finite pair against its named incompatibility use.
///
/// This adds exact use identity and pair membership to [`check_typed_finite_incompatibility`].
/// Generic relation uses do not yet expose typed left/right incompatibility roles, so this cannot
/// check port orientation. It never turns finite-table membership into standing/admitted
/// incompatibility, a completed observation route, or a departure witness.
pub fn check_typed_finite_incompatibility_use<C: FiniteTypedIncompatibilityUseCatalog>(
    table: &TypedFiniteIncompatibilityTable,
    catalog: &C,
    incompatibility_use: RelationUseRef,
    source: TypedFiniteObservation,
    candidate: TypedFiniteObservation,
) -> Result<TypedFiniteIncompatibilityUseResult, TypedFiniteIncompatibilityUseError> {
    let relation_use = catalog.resolve_relation_use(incompatibility_use).ok_or(
        TypedFiniteIncompatibilityUseError::UnresolvedRelationUse(incompatibility_use),
    )?;
    let calculated = relation_use.relation_use_ref()?;
    if calculated != incompatibility_use {
        return Err(
            TypedFiniteIncompatibilityUseError::RelationUseReferenceIdentityMismatch {
                reference: incompatibility_use,
                calculated,
            },
        );
    }
    relation_use.check(catalog)?;
    if relation_use.mode() == crate::DischargeMode::Generate {
        return Err(
            TypedFiniteIncompatibilityUseError::GeneratedIncompatibilityUse(incompatibility_use),
        );
    }
    let result = check_typed_finite_incompatibility(table, catalog, source, candidate)?;
    Ok(match result {
        TypedFiniteIncompatibilityResult::Incompatible(pair) => {
            if !crate::departure::relation_use_binds_pair(
                &relation_use,
                pair.source_value(),
                pair.candidate_value(),
            ) {
                return Err(TypedFiniteIncompatibilityUseError::ClaimedPairNotBound(
                    incompatibility_use,
                ));
            }
            TypedFiniteIncompatibilityUseResult::Incompatible(
                TypedFiniteIncompatibilityUseWitness {
                    pair,
                    incompatibility_use,
                },
            )
        }
        TypedFiniteIncompatibilityResult::NoWitness => {
            TypedFiniteIncompatibilityUseResult::NoWitness
        }
        TypedFiniteIncompatibilityResult::Unknown => TypedFiniteIncompatibilityUseResult::Unknown,
    })
}

/// Checks a finite incompatibility pair at explicit named source/candidate ports.
///
/// This is the smallest structural orientation check available over an ordinary `RelationUse`:
/// callers must name both distinct ports, and the check verifies that the positive pair occurs at
/// those exact ports. It does not infer port roles from spelling and does not establish standing,
/// admission, observation actuality, relevance, non-circularity, coverage, or departure.
pub fn check_typed_finite_oriented_incompatibility_use<C: FiniteTypedIncompatibilityUseCatalog>(
    table: &TypedFiniteIncompatibilityTable,
    catalog: &C,
    incompatibility_use: RelationUseRef,
    roles: TypedFiniteIncompatibilityRoles,
    source: TypedFiniteObservation,
    candidate: TypedFiniteObservation,
) -> Result<TypedFiniteOrientedIncompatibilityUseResult, TypedFiniteOrientedIncompatibilityUseError>
{
    let result = check_typed_finite_incompatibility_use(
        table,
        catalog,
        incompatibility_use,
        source,
        candidate,
    )?;
    match result {
        TypedFiniteIncompatibilityUseResult::Incompatible(witness) => {
            let relation_use = catalog.resolve_relation_use(incompatibility_use).ok_or(
                TypedFiniteOrientedIncompatibilityUseError::UnresolvedRelationUse(
                    incompatibility_use,
                ),
            )?;
            let source_matches = relation_use.bindings().iter().any(|binding| {
                binding.port() == roles.source_port()
                    && binding.value() == witness.pair().source_value()
            });
            let candidate_matches = relation_use.bindings().iter().any(|binding| {
                binding.port() == roles.candidate_port()
                    && binding.value() == witness.pair().candidate_value()
            });
            if !source_matches || !candidate_matches {
                return Err(
                    TypedFiniteOrientedIncompatibilityUseError::ClaimedPairAtWrongRoles {
                        incompatibility_use,
                        source_port: roles.source_port().clone(),
                        candidate_port: roles.candidate_port().clone(),
                    },
                );
            }
            Ok(TypedFiniteOrientedIncompatibilityUseResult::Incompatible(
                TypedFiniteOrientedIncompatibilityUseWitness { witness, roles },
            ))
        }
        TypedFiniteIncompatibilityUseResult::NoWitness => {
            Ok(TypedFiniteOrientedIncompatibilityUseResult::NoWitness)
        }
        TypedFiniteIncompatibilityUseResult::Unknown => {
            Ok(TypedFiniteOrientedIncompatibilityUseResult::Unknown)
        }
    }
}

fn check_typed_finite_form<C: FiniteTypedObservationCatalog>(
    catalog: &C,
    role: &'static str,
    reference: TypedFormRef,
) -> Result<(), TypedFiniteIncompatibilityError> {
    let typed_form = catalog
        .resolve_typed_form(reference)
        .ok_or(TypedFiniteIncompatibilityError::UnresolvedTypedForm { role, reference })?;
    let calculated = typed_form.typed_form_ref()?;
    if calculated != reference {
        return Err(
            TypedFiniteIncompatibilityError::TypedFormReferenceIdentityMismatch {
                role,
                reference,
                calculated,
            },
        );
    }
    typed_form
        .check(catalog)
        .map_err(|source| TypedFiniteIncompatibilityError::InvalidTypedForm { role, source })
}

/// Errors from constructing or checking a finite typed incompatibility table.
#[derive(Debug, Error)]
pub enum TypedFiniteIncompatibilityError {
    #[error(transparent)]
    TypedFormIdentity(#[from] TypeError),

    #[error("finite typed incompatibility table repeats ({source_value}, {candidate_value})")]
    DuplicatePair {
        /// The duplicated source typed-form declaration.
        source_value: TypedFormRef,
        /// The duplicated candidate typed-form declaration.
        candidate_value: TypedFormRef,
    },

    #[error("{role} typed form {reference} is not available from the declared catalog")]
    UnresolvedTypedForm {
        /// The table or observation position being checked.
        role: &'static str,
        /// The unresolved typed-form reference.
        reference: TypedFormRef,
    },

    #[error("{role} typed form {reference} hashes to {calculated}, not its claimed identity")]
    TypedFormReferenceIdentityMismatch {
        /// The table or observation position being checked.
        role: &'static str,
        /// The claimed typed-form identity.
        reference: TypedFormRef,
        /// The identity calculated from the resolved declaration.
        calculated: TypedFormRef,
    },

    #[error("{role} typed form is not well-typed: {source}")]
    InvalidTypedForm {
        /// The table or observation position being checked.
        role: &'static str,
        /// The type-checking failure.
        #[source]
        source: TypeCheckError,
    },
}

/// Errors while associating typed finite incompatibility evidence with a declared use.
#[derive(Debug, Error)]
pub enum TypedFiniteIncompatibilityUseError {
    #[error(transparent)]
    Finite(#[from] TypedFiniteIncompatibilityError),
    #[error(transparent)]
    RelationUse(#[from] RelationUseError),
    #[error(transparent)]
    RelationUseCheck(#[from] RelationUseCheckError),
    #[error("incompatibility use {0} is unavailable")]
    UnresolvedRelationUse(RelationUseRef),
    #[error("relation use {reference} hashes to {calculated}, not its claimed identity")]
    RelationUseReferenceIdentityMismatch {
        reference: RelationUseRef,
        calculated: RelationUseRef,
    },
    #[error("generated relation use {0} cannot supply positive incompatibility evidence")]
    GeneratedIncompatibilityUse(RelationUseRef),
    #[error("incompatibility use {0} does not bind the positively witnessed ordered pair")]
    ClaimedPairNotBound(RelationUseRef),
}

/// Errors from declaring or checking explicit finite incompatibility port roles.
#[derive(Debug, Error)]
pub enum TypedFiniteIncompatibilityRoleError {
    #[error("source and candidate incompatibility roles name the same port {0}")]
    DuplicateRolePort(TypeSymbol),
}

/// Errors while checking the source/candidate orientation of a finite incompatibility use.
#[derive(Debug, Error)]
pub enum TypedFiniteOrientedIncompatibilityUseError {
    #[error(transparent)]
    Use(#[from] TypedFiniteIncompatibilityUseError),
    #[error("incompatibility use {0} is unavailable")]
    UnresolvedRelationUse(RelationUseRef),
    #[error(
        "incompatibility use {incompatibility_use} does not bind the positive pair at source port {source_port} and candidate port {candidate_port}"
    )]
    ClaimedPairAtWrongRoles {
        incompatibility_use: RelationUseRef,
        source_port: TypeSymbol,
        candidate_port: TypeSymbol,
    },
}

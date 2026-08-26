//! Exhaustive finite checking for binding-supplied adjunction laws.
//!
//! Converse incidence supplies existential preimages.  It does not supply the universal law
//! `forward(x) <= y` iff `x <= backward(y)`.  This module admits that stronger reading only after
//! checking an explicit finite preorder and both total maps at every declared pair.  The result is
//! derived checker data, not a semantic primitive, opcode, canonical artifact, or warrant.

use std::collections::{BTreeMap, BTreeSet};

use thiserror::Error;

use crate::{
    BindingVersionRef, TypeArtifact, TypeCatalog, TypeCheckError, TypeError, TypeRef, TypedForm,
    TypedFormRef,
};

/// One exact finite typed preorder supplied by a binding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFinitePreorder {
    binding: BindingVersionRef,
    ty: TypeRef,
    elements: BTreeSet<TypedFormRef>,
    less_or_equal: BTreeSet<(TypedFormRef, TypedFormRef)>,
}

impl ExactFinitePreorder {
    pub fn new(
        binding: BindingVersionRef,
        ty: TypeRef,
        elements: Vec<TypedFormRef>,
        less_or_equal: Vec<(TypedFormRef, TypedFormRef)>,
    ) -> Result<Self, FiniteAdjunctionError> {
        let mut element_set = BTreeSet::new();
        for element in elements {
            if !element_set.insert(element) {
                return Err(FiniteAdjunctionError::DuplicateElement(element));
            }
        }
        let mut comparison_set = BTreeSet::new();
        for comparison in less_or_equal {
            if !comparison_set.insert(comparison) {
                return Err(FiniteAdjunctionError::DuplicateComparison(comparison));
            }
        }
        Ok(Self {
            binding,
            ty,
            elements: element_set,
            less_or_equal: comparison_set,
        })
    }

    #[must_use]
    pub const fn binding(&self) -> BindingVersionRef {
        self.binding
    }

    #[must_use]
    pub const fn ty(&self) -> TypeRef {
        self.ty
    }

    #[must_use]
    pub const fn elements(&self) -> &BTreeSet<TypedFormRef> {
        &self.elements
    }

    #[must_use]
    pub const fn less_or_equal(&self) -> &BTreeSet<(TypedFormRef, TypedFormRef)> {
        &self.less_or_equal
    }

    #[must_use]
    pub fn relates(&self, lower: TypedFormRef, upper: TypedFormRef) -> bool {
        self.less_or_equal.contains(&(lower, upper))
    }

    fn check<C: FiniteAdjunctionCatalog>(&self, catalog: &C) -> Result<(), FiniteAdjunctionError> {
        check_type(self.ty, self.binding, catalog)?;
        for element in &self.elements {
            check_element(*element, self.ty, self.binding, catalog)?;
        }
        for (lower, upper) in &self.less_or_equal {
            if !self.elements.contains(lower) || !self.elements.contains(upper) {
                return Err(FiniteAdjunctionError::ComparisonOutsideDomain {
                    lower: *lower,
                    upper: *upper,
                });
            }
        }
        for element in &self.elements {
            if !self.relates(*element, *element) {
                return Err(FiniteAdjunctionError::MissingReflexiveComparison(*element));
            }
        }
        for lower in &self.elements {
            for middle in &self.elements {
                if !self.relates(*lower, *middle) {
                    continue;
                }
                for upper in &self.elements {
                    if self.relates(*middle, *upper) && !self.relates(*lower, *upper) {
                        return Err(FiniteAdjunctionError::NonTransitiveComparison {
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

/// One declared pair of total finite maps proposed as an adjunction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteAdjunctionCandidate {
    binding: BindingVersionRef,
    left: ExactFinitePreorder,
    right: ExactFinitePreorder,
    forward: BTreeMap<TypedFormRef, TypedFormRef>,
    backward: BTreeMap<TypedFormRef, TypedFormRef>,
}

impl FiniteAdjunctionCandidate {
    pub fn new(
        binding: BindingVersionRef,
        left: ExactFinitePreorder,
        right: ExactFinitePreorder,
        forward: Vec<(TypedFormRef, TypedFormRef)>,
        backward: Vec<(TypedFormRef, TypedFormRef)>,
    ) -> Result<Self, FiniteAdjunctionError> {
        Ok(Self {
            binding,
            left,
            right,
            forward: collect_map(forward, MapDirection::Forward)?,
            backward: collect_map(backward, MapDirection::Backward)?,
        })
    }

    #[must_use]
    pub const fn binding(&self) -> BindingVersionRef {
        self.binding
    }

    #[must_use]
    pub const fn left(&self) -> &ExactFinitePreorder {
        &self.left
    }

    #[must_use]
    pub const fn right(&self) -> &ExactFinitePreorder {
        &self.right
    }

    #[must_use]
    pub const fn forward(&self) -> &BTreeMap<TypedFormRef, TypedFormRef> {
        &self.forward
    }

    #[must_use]
    pub const fn backward(&self) -> &BTreeMap<TypedFormRef, TypedFormRef> {
        &self.backward
    }
}

/// A candidate whose exact finite domains, maps, and pointwise adjunction law were rechecked.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedFiniteAdjunction(FiniteAdjunctionCandidate);

impl AdmittedFiniteAdjunction {
    #[must_use]
    pub const fn candidate(&self) -> &FiniteAdjunctionCandidate {
        &self.0
    }
}

/// Catalog boundary for rechecking the typed inhabitants of a finite adjunction candidate.
pub trait FiniteAdjunctionCatalog: TypeCatalog {
    fn resolve_typed_form(&self, reference: TypedFormRef) -> Option<TypedForm>;
}

/// Exhaustively checks `forward(x) <= y` iff `x <= backward(y)` on the declared finite domains.
pub fn admit_finite_adjunction<C: FiniteAdjunctionCatalog>(
    candidate: FiniteAdjunctionCandidate,
    catalog: &C,
) -> Result<AdmittedFiniteAdjunction, FiniteAdjunctionError> {
    if candidate.left.binding() != candidate.binding {
        return Err(FiniteAdjunctionError::PreorderBindingMismatch {
            side: "left",
            expected: candidate.binding,
            actual: candidate.left.binding(),
        });
    }
    if candidate.right.binding() != candidate.binding {
        return Err(FiniteAdjunctionError::PreorderBindingMismatch {
            side: "right",
            expected: candidate.binding,
            actual: candidate.right.binding(),
        });
    }
    candidate.left.check(catalog)?;
    candidate.right.check(catalog)?;
    check_total_map(
        &candidate.forward,
        candidate.left.elements(),
        candidate.right.elements(),
        MapDirection::Forward,
    )?;
    check_total_map(
        &candidate.backward,
        candidate.right.elements(),
        candidate.left.elements(),
        MapDirection::Backward,
    )?;

    for left in candidate.left.elements() {
        let forward = candidate.forward[left];
        for right in candidate.right.elements() {
            let backward = candidate.backward[right];
            let forward_below = candidate.right.relates(forward, *right);
            let below_backward = candidate.left.relates(*left, backward);
            if forward_below != below_backward {
                return Err(FiniteAdjunctionError::LawViolation(Box::new(
                    FiniteAdjunctionLawCounterexample {
                        left: *left,
                        right: *right,
                        forward,
                        backward,
                        forward_below,
                        below_backward,
                    },
                )));
            }
        }
    }

    Ok(AdmittedFiniteAdjunction(candidate))
}

#[derive(Clone, Copy)]
enum MapDirection {
    Forward,
    Backward,
}

fn collect_map(
    entries: Vec<(TypedFormRef, TypedFormRef)>,
    direction: MapDirection,
) -> Result<BTreeMap<TypedFormRef, TypedFormRef>, FiniteAdjunctionError> {
    let mut map = BTreeMap::new();
    for (source, target) in entries {
        if map.insert(source, target).is_some() {
            return Err(match direction {
                MapDirection::Forward => FiniteAdjunctionError::DuplicateForwardSource(source),
                MapDirection::Backward => FiniteAdjunctionError::DuplicateBackwardSource(source),
            });
        }
    }
    Ok(map)
}

fn check_total_map(
    map: &BTreeMap<TypedFormRef, TypedFormRef>,
    source: &BTreeSet<TypedFormRef>,
    target: &BTreeSet<TypedFormRef>,
    direction: MapDirection,
) -> Result<(), FiniteAdjunctionError> {
    for element in source {
        if !map.contains_key(element) {
            return Err(match direction {
                MapDirection::Forward => FiniteAdjunctionError::MissingForwardValue(*element),
                MapDirection::Backward => FiniteAdjunctionError::MissingBackwardValue(*element),
            });
        }
    }
    for (input, output) in map {
        if !source.contains(input) {
            return Err(match direction {
                MapDirection::Forward => FiniteAdjunctionError::ForwardInputOutsideDomain(*input),
                MapDirection::Backward => FiniteAdjunctionError::BackwardInputOutsideDomain(*input),
            });
        }
        if !target.contains(output) {
            return Err(match direction {
                MapDirection::Forward => {
                    FiniteAdjunctionError::ForwardOutputOutsideCodomain(*output)
                }
                MapDirection::Backward => {
                    FiniteAdjunctionError::BackwardOutputOutsideCodomain(*output)
                }
            });
        }
    }
    Ok(())
}

fn check_type<C: FiniteAdjunctionCatalog>(
    reference: TypeRef,
    binding: BindingVersionRef,
    catalog: &C,
) -> Result<TypeArtifact, FiniteAdjunctionError> {
    let ty = catalog
        .resolve_type(reference)
        .ok_or(FiniteAdjunctionError::UnresolvedType(reference))?;
    let calculated = ty.type_ref()?;
    if calculated != reference {
        return Err(FiniteAdjunctionError::TypeIdentityMismatch {
            reference,
            calculated,
        });
    }
    if ty.binding() != binding {
        return Err(FiniteAdjunctionError::TypeBindingMismatch {
            reference,
            expected: binding,
            actual: ty.binding(),
        });
    }
    ty.check(catalog)?;
    Ok(ty)
}

fn check_element<C: FiniteAdjunctionCatalog>(
    reference: TypedFormRef,
    ty: TypeRef,
    binding: BindingVersionRef,
    catalog: &C,
) -> Result<(), FiniteAdjunctionError> {
    let form = catalog
        .resolve_typed_form(reference)
        .ok_or(FiniteAdjunctionError::UnresolvedElement(reference))?;
    let calculated = form.typed_form_ref()?;
    if calculated != reference {
        return Err(FiniteAdjunctionError::ElementIdentityMismatch {
            reference,
            calculated,
        });
    }
    form.check(catalog)?;
    if form.binding() != binding {
        return Err(FiniteAdjunctionError::ElementBindingMismatch {
            reference,
            expected: binding,
            actual: form.binding(),
        });
    }
    if form.ty() != ty {
        return Err(FiniteAdjunctionError::ElementTypeMismatch {
            reference,
            expected: ty,
            actual: form.ty(),
        });
    }
    Ok(())
}

/// A named finite pair at which a proposed adjunction law fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FiniteAdjunctionLawCounterexample {
    left: TypedFormRef,
    right: TypedFormRef,
    forward: TypedFormRef,
    backward: TypedFormRef,
    forward_below: bool,
    below_backward: bool,
}

impl FiniteAdjunctionLawCounterexample {
    #[must_use]
    pub const fn left(self) -> TypedFormRef {
        self.left
    }

    #[must_use]
    pub const fn right(self) -> TypedFormRef {
        self.right
    }

    #[must_use]
    pub const fn forward(self) -> TypedFormRef {
        self.forward
    }

    #[must_use]
    pub const fn backward(self) -> TypedFormRef {
        self.backward
    }

    #[must_use]
    pub const fn forward_below(self) -> bool {
        self.forward_below
    }

    #[must_use]
    pub const fn below_backward(self) -> bool {
        self.below_backward
    }
}

/// Structural or law failures from exact finite adjunction admission.
#[derive(Debug, Error)]
pub enum FiniteAdjunctionError {
    #[error(transparent)]
    TypeEncoding(#[from] TypeError),
    #[error(transparent)]
    TypeCheck(Box<TypeCheckError>),
    #[error("finite preorder repeats element {0}")]
    DuplicateElement(TypedFormRef),
    #[error("finite preorder repeats comparison {0:?}")]
    DuplicateComparison((TypedFormRef, TypedFormRef)),
    #[error("finite preorder comparison ({lower}, {upper}) leaves its declared domain")]
    ComparisonOutsideDomain {
        lower: TypedFormRef,
        upper: TypedFormRef,
    },
    #[error("finite preorder omits reflexive comparison for {0}")]
    MissingReflexiveComparison(TypedFormRef),
    #[error("finite preorder is non-transitive at {lower} <= {middle} <= {upper}")]
    NonTransitiveComparison {
        lower: TypedFormRef,
        middle: TypedFormRef,
        upper: TypedFormRef,
    },
    #[error("{side} preorder binding {actual} differs from candidate binding {expected}")]
    PreorderBindingMismatch {
        side: &'static str,
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },
    #[error("type {0} is unavailable")]
    UnresolvedType(TypeRef),
    #[error("type {reference} hashes to {calculated}")]
    TypeIdentityMismatch {
        reference: TypeRef,
        calculated: TypeRef,
    },
    #[error("type {reference} has binding {actual}, expected {expected}")]
    TypeBindingMismatch {
        reference: TypeRef,
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },
    #[error("typed finite element {0} is unavailable")]
    UnresolvedElement(TypedFormRef),
    #[error("typed finite element {reference} hashes to {calculated}")]
    ElementIdentityMismatch {
        reference: TypedFormRef,
        calculated: TypedFormRef,
    },
    #[error("typed finite element {reference} has binding {actual}, expected {expected}")]
    ElementBindingMismatch {
        reference: TypedFormRef,
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },
    #[error("typed finite element {reference} has type {actual}, expected {expected}")]
    ElementTypeMismatch {
        reference: TypedFormRef,
        expected: TypeRef,
        actual: TypeRef,
    },
    #[error("forward map repeats source {0}")]
    DuplicateForwardSource(TypedFormRef),
    #[error("backward map repeats source {0}")]
    DuplicateBackwardSource(TypedFormRef),
    #[error("forward map omits source {0}")]
    MissingForwardValue(TypedFormRef),
    #[error("backward map omits source {0}")]
    MissingBackwardValue(TypedFormRef),
    #[error("forward map input {0} is outside its domain")]
    ForwardInputOutsideDomain(TypedFormRef),
    #[error("forward map output {0} is outside its codomain")]
    ForwardOutputOutsideCodomain(TypedFormRef),
    #[error("backward map input {0} is outside its domain")]
    BackwardInputOutsideDomain(TypedFormRef),
    #[error("backward map output {0} is outside its codomain")]
    BackwardOutputOutsideCodomain(TypedFormRef),
    #[error("adjunction law fails at {0:?}")]
    LawViolation(Box<FiniteAdjunctionLawCounterexample>),
}

impl From<TypeCheckError> for FiniteAdjunctionError {
    fn from(error: TypeCheckError) -> Self {
        Self::TypeCheck(Box::new(error))
    }
}

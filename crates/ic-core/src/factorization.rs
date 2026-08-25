//! Exact finite determination-through checking.
//!
//! This module implements the finite deterministic instance of the canonical kernel-inclusion
//! theorem. It is derived checker data, not an actuality, coverage, standing, or recovery record.

use std::collections::{BTreeMap, BTreeSet};

use thiserror::Error;

use crate::{
    ApplicabilityRef, ArtifactRef, BindingVersionRef, GrainRef, HorizonRef, ScopeRef, TypeRef,
};

/// The indexed type/context contract shared by exact signatures before factorization is lawful.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SignatureContext {
    binding: BindingVersionRef,
    scope: ScopeRef,
    applicability: ApplicabilityRef,
    grain: GrainRef,
    horizon: HorizonRef,
    domain: TypeRef,
}

impl SignatureContext {
    #[must_use]
    pub const fn new(
        binding: BindingVersionRef,
        scope: ScopeRef,
        applicability: ApplicabilityRef,
        grain: GrainRef,
        horizon: HorizonRef,
        domain: TypeRef,
    ) -> Self {
        Self {
            binding,
            scope,
            applicability,
            grain,
            horizon,
            domain,
        }
    }

    #[must_use]
    pub const fn binding(self) -> BindingVersionRef {
        self.binding
    }
    #[must_use]
    pub const fn scope(self) -> ScopeRef {
        self.scope
    }
    #[must_use]
    pub const fn applicability(self) -> ApplicabilityRef {
        self.applicability
    }
    #[must_use]
    pub const fn grain(self) -> GrainRef {
        self.grain
    }
    #[must_use]
    pub const fn horizon(self) -> HorizonRef {
        self.horizon
    }
    #[must_use]
    pub const fn domain(self) -> TypeRef {
        self.domain
    }
}

/// A complete, deterministic finite signature over a declared domain.
///
/// The caller must establish that its entry set is exactly covered; working, partial, or
/// nondeterministic signatures intentionally cannot be passed to [`determine_through_exact`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFiniteSignature {
    context: SignatureContext,
    values: BTreeMap<ArtifactRef, ArtifactRef>,
}

impl ExactFiniteSignature {
    pub fn new(
        context: SignatureContext,
        entries: Vec<(ArtifactRef, ArtifactRef)>,
    ) -> Result<Self, ExactDeterminationError> {
        let mut values = BTreeMap::new();
        for (domain_value, codomain_value) in entries {
            if values.insert(domain_value, codomain_value).is_some() {
                return Err(ExactDeterminationError::DuplicateDomainValue(domain_value));
            }
        }
        Ok(Self { context, values })
    }

    #[must_use]
    pub const fn context(&self) -> SignatureContext {
        self.context
    }

    #[must_use]
    pub const fn values(&self) -> &BTreeMap<ArtifactRef, ArtifactRef> {
        &self.values
    }
}

/// The explicitly constructed map (h) for an exact factorization target = h ∘ available.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFactorization {
    factor: BTreeMap<ArtifactRef, ArtifactRef>,
}

impl ExactFactorization {
    #[must_use]
    pub const fn factor(&self) -> &BTreeMap<ArtifactRef, ArtifactRef> {
        &self.factor
    }
}

/// A direct witness that kernel inclusion fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KernelSeparator {
    first_domain: ArtifactRef,
    second_domain: ArtifactRef,
    available_value: ArtifactRef,
    first_target_value: ArtifactRef,
    second_target_value: ArtifactRef,
}

impl KernelSeparator {
    #[must_use]
    pub const fn first_domain(self) -> ArtifactRef {
        self.first_domain
    }
    #[must_use]
    pub const fn second_domain(self) -> ArtifactRef {
        self.second_domain
    }
    #[must_use]
    pub const fn available_value(self) -> ArtifactRef {
        self.available_value
    }
    #[must_use]
    pub const fn first_target_value(self) -> ArtifactRef {
        self.first_target_value
    }
    #[must_use]
    pub const fn second_target_value(self) -> ArtifactRef {
        self.second_target_value
    }
}

/// The result of the exact finite kernel-inclusion test.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExactDeterminationResult {
    Exact { factorization: ExactFactorization },
    NotDetermined { separator: KernelSeparator },
}

/// A tagged exact finite family signature.
///
/// Its product is retained information only: constructing or factorizing one does not create a
/// jointly actual observation. Every component must cover exactly the same declared finite domain
/// and indexed context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFamilySignature {
    context: SignatureContext,
    components: Vec<ExactFiniteSignature>,
}

impl ExactFamilySignature {
    pub fn new(components: Vec<ExactFiniteSignature>) -> Result<Self, ExactDeterminationError> {
        let Some(first) = components.first() else {
            return Err(ExactDeterminationError::EmptyFamily);
        };
        let context = first.context;
        let domain: BTreeSet<_> = first.values.keys().copied().collect();
        for component in &components[1..] {
            ensure_shared_context(context, component.context)?;
            ensure_shared_domain(&domain, &component.values)?;
        }
        Ok(Self {
            context,
            components,
        })
    }

    #[must_use]
    pub const fn context(&self) -> SignatureContext {
        self.context
    }

    #[must_use]
    pub fn components(&self) -> &[ExactFiniteSignature] {
        &self.components
    }
}

/// The explicit target map factored through a tagged exact family product.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFamilyFactorization {
    factor: BTreeMap<Vec<ArtifactRef>, ArtifactRef>,
}

impl ExactFamilyFactorization {
    #[must_use]
    pub const fn factor(&self) -> &BTreeMap<Vec<ArtifactRef>, ArtifactRef> {
        &self.factor
    }
}

/// A witness that two sources agree on every tagged family coordinate but differ on the target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FamilyKernelSeparator {
    first_domain: ArtifactRef,
    second_domain: ArtifactRef,
    available_values: Vec<ArtifactRef>,
    first_target_value: ArtifactRef,
    second_target_value: ArtifactRef,
}

impl FamilyKernelSeparator {
    #[must_use]
    pub const fn first_domain(&self) -> ArtifactRef {
        self.first_domain
    }

    #[must_use]
    pub const fn second_domain(&self) -> ArtifactRef {
        self.second_domain
    }

    #[must_use]
    pub fn available_values(&self) -> &[ArtifactRef] {
        &self.available_values
    }

    #[must_use]
    pub const fn first_target_value(&self) -> ArtifactRef {
        self.first_target_value
    }

    #[must_use]
    pub const fn second_target_value(&self) -> ArtifactRef {
        self.second_target_value
    }
}

/// The result of exact target determination through a tagged family product.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExactFamilyDeterminationResult {
    Exact {
        factorization: ExactFamilyFactorization,
    },
    NotDetermined {
        separator: FamilyKernelSeparator,
    },
}

/// Decides the exact finite instance of `ker(available) ⊆ ker(target)`.
///
/// A context or domain-coverage mismatch is an error, not an `Exact` result. The function is
/// deliberately unavailable for incomplete, working, or nondeterministic signature inputs.
pub fn determine_through_exact(
    available: &ExactFiniteSignature,
    target: &ExactFiniteSignature,
) -> Result<ExactDeterminationResult, ExactDeterminationError> {
    ensure_shared_context(available.context, target.context)?;
    let available_domain: BTreeSet<_> = available.values.keys().copied().collect();
    let target_domain: BTreeSet<_> = target.values.keys().copied().collect();
    if available_domain != target_domain {
        return Err(ExactDeterminationError::DomainCoverageMismatch {
            available_only: available_domain.difference(&target_domain).next().copied(),
            target_only: target_domain.difference(&available_domain).next().copied(),
        });
    }

    let mut factor = BTreeMap::new();
    let mut first_domain = BTreeMap::new();
    for (domain, available_value) in &available.values {
        let target_value = target
            .values
            .get(domain)
            .expect("equal finite domains are checked before factorization");
        if let Some(previous_target) = factor.insert(*available_value, *target_value) {
            if previous_target != *target_value {
                return Ok(ExactDeterminationResult::NotDetermined {
                    separator: KernelSeparator {
                        first_domain: *first_domain
                            .get(available_value)
                            .expect("factor entry has its first domain"),
                        second_domain: *domain,
                        available_value: *available_value,
                        first_target_value: previous_target,
                        second_target_value: *target_value,
                    },
                });
            }
        } else {
            first_domain.insert(*available_value, *domain);
        }
    }
    Ok(ExactDeterminationResult::Exact {
        factorization: ExactFactorization { factor },
    })
}

/// Decides `ker(product(available_family)) ⊆ ker(target)` for exact finite data.
///
/// This is an informational product calculation. It deliberately does not establish supported
/// co-applicability, joint realizability, a composite raw return, or a composite actual event.
pub fn determine_through_exact_family(
    available: &ExactFamilySignature,
    target: &ExactFiniteSignature,
) -> Result<ExactFamilyDeterminationResult, ExactDeterminationError> {
    ensure_shared_context(available.context, target.context)?;
    let domain: BTreeSet<_> = available.components[0].values.keys().copied().collect();
    ensure_shared_domain(&domain, &target.values)?;

    let mut factor = BTreeMap::new();
    let mut first_domain = BTreeMap::new();
    for domain_value in domain {
        let available_values: Vec<_> = available
            .components
            .iter()
            .map(|component| {
                *component
                    .values
                    .get(&domain_value)
                    .expect("family domains are checked before factorization")
            })
            .collect();
        let target_value = *target
            .values
            .get(&domain_value)
            .expect("target domain is checked before factorization");
        if let Some(previous_target) = factor.insert(available_values.clone(), target_value) {
            if previous_target != target_value {
                return Ok(ExactFamilyDeterminationResult::NotDetermined {
                    separator: FamilyKernelSeparator {
                        first_domain: *first_domain
                            .get(&available_values)
                            .expect("factor entry has its first domain"),
                        second_domain: domain_value,
                        available_values,
                        first_target_value: previous_target,
                        second_target_value: target_value,
                    },
                });
            }
        } else {
            first_domain.insert(available_values, domain_value);
        }
    }
    Ok(ExactFamilyDeterminationResult::Exact {
        factorization: ExactFamilyFactorization { factor },
    })
}

fn ensure_shared_context(
    available: SignatureContext,
    target: SignatureContext,
) -> Result<(), ExactDeterminationError> {
    for (field, left, right) in [
        (
            "binding",
            available.binding.as_artifact_ref(),
            target.binding.as_artifact_ref(),
        ),
        (
            "scope",
            available.scope.as_artifact_ref(),
            target.scope.as_artifact_ref(),
        ),
        (
            "applicability",
            available.applicability.as_artifact_ref(),
            target.applicability.as_artifact_ref(),
        ),
        (
            "grain",
            available.grain.as_artifact_ref(),
            target.grain.as_artifact_ref(),
        ),
        (
            "horizon",
            available.horizon.as_artifact_ref(),
            target.horizon.as_artifact_ref(),
        ),
        (
            "domain",
            available.domain.as_artifact_ref(),
            target.domain.as_artifact_ref(),
        ),
    ] {
        if left != right {
            return Err(ExactDeterminationError::ContextMismatch { field, left, right });
        }
    }
    Ok(())
}

fn ensure_shared_domain(
    expected: &BTreeSet<ArtifactRef>,
    values: &BTreeMap<ArtifactRef, ArtifactRef>,
) -> Result<(), ExactDeterminationError> {
    let actual: BTreeSet<_> = values.keys().copied().collect();
    if expected != &actual {
        return Err(ExactDeterminationError::DomainCoverageMismatch {
            available_only: expected.difference(&actual).next().copied(),
            target_only: actual.difference(expected).next().copied(),
        });
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum ExactDeterminationError {
    #[error("exact family must contain at least one signature")]
    EmptyFamily,
    #[error("exact signature repeats domain value {0}")]
    DuplicateDomainValue(ArtifactRef),
    #[error("exact signatures differ in {field}: {left} versus {right}")]
    ContextMismatch {
        field: &'static str,
        left: ArtifactRef,
        right: ArtifactRef,
    },
    #[error("exact signatures do not cover the same domain")]
    DomainCoverageMismatch {
        available_only: Option<ArtifactRef>,
        target_only: Option<ArtifactRef>,
    },
}

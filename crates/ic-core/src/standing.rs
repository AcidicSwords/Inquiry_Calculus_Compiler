use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    str::FromStr,
};

use crate::{ArtifactError, ArtifactRef};

/// The identity of one claim whose standing may be at issue.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ClaimRef(ArtifactRef);

impl ClaimRef {
    #[must_use]
    pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
        Self(reference)
    }

    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}

impl fmt::Display for ClaimRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for ClaimRef {
    type Err = ArtifactError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ArtifactRef::from_str(value).map(Self)
    }
}

/// One candidate support route for a claim.
///
/// The specification's `Closed_X(E, lambda)` has five conditions. Two of them are decided here
/// against the standing set as it grows -- the premises requiring standing, and the emptiness of
/// the open dependency boundary. The other three are properties of the route itself that this
/// phase cannot evaluate: whether applicability and scope hold, whether the independent checks the
/// route requires actually succeeded, and whether an inconsistency policy invalidates the
/// environment. Those arrive as caller declarations.
///
/// Declaring them is not discharging them. A caller who marks an unchecked route as checked has
/// asserted something this engine will believe, exactly as it will believe a declared ingress; the
/// engine decides what follows from the declarations, not whether they are true.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupportEnvironment {
    claim: ClaimRef,
    premises: BTreeSet<ClaimRef>,
    open_dependencies: BTreeSet<ArtifactRef>,
    applicable: bool,
    checks_discharged: bool,
    invalidated: bool,
}

impl SupportEnvironment {
    /// Declares one support route whose premises must themselves stand.
    #[must_use]
    pub fn new(claim: ClaimRef, premises: Vec<ClaimRef>) -> Self {
        Self {
            claim,
            premises: premises.into_iter().collect(),
            open_dependencies: BTreeSet::new(),
            applicable: true,
            checks_discharged: true,
            invalidated: false,
        }
    }

    /// Records dependencies the route requires but neither supplies nor independently discharges.
    ///
    /// A nonempty boundary is an open question, not a failure: the claim simply cannot close
    /// through this route while it stands open.
    #[must_use]
    pub fn with_open_dependencies(mut self, open: Vec<ArtifactRef>) -> Self {
        self.open_dependencies = open.into_iter().collect();
        self
    }

    /// Declares whether the route's applicability and scope conditions hold.
    #[must_use]
    pub const fn with_applicability(mut self, applicable: bool) -> Self {
        self.applicable = applicable;
        self
    }

    /// Declares whether the independent checks this route requires have succeeded.
    #[must_use]
    pub const fn with_checks_discharged(mut self, discharged: bool) -> Self {
        self.checks_discharged = discharged;
        self
    }

    /// Declares that an explicit inconsistency policy invalidates this route.
    #[must_use]
    pub const fn invalidated(mut self, invalidated: bool) -> Self {
        self.invalidated = invalidated;
        self
    }

    /// Returns the claim this route supports.
    #[must_use]
    pub const fn claim(&self) -> ClaimRef {
        self.claim
    }

    /// Returns the premises that must themselves stand.
    #[must_use]
    pub const fn premises(&self) -> &BTreeSet<ClaimRef> {
        &self.premises
    }

    /// Returns the open dependency boundary.
    #[must_use]
    pub const fn open_dependencies(&self) -> &BTreeSet<ArtifactRef> {
        &self.open_dependencies
    }

    /// Decides `Closed_X(E, lambda)` against a standing set.
    #[must_use]
    pub fn is_closed(&self, standing: &BTreeSet<ClaimRef>) -> bool {
        !self.invalidated
            && self.applicable
            && self.checks_discharged
            && self.open_dependencies.is_empty()
            && self
                .premises
                .iter()
                .all(|premise| standing.contains(premise))
    }
}

/// The declared inputs to one standing computation.
///
/// `ingress` holds the grounded facts available independently of inference: preserved actual
/// returns, trusted configuration, accepted predecessor relations, checker axioms. Everything else
/// must earn its place through a closed route.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StandingProblem {
    ingress: BTreeSet<ClaimRef>,
    environments: Vec<SupportEnvironment>,
}

impl StandingProblem {
    #[must_use]
    pub fn new(ingress: Vec<ClaimRef>, environments: Vec<SupportEnvironment>) -> Self {
        Self {
            ingress: ingress.into_iter().collect(),
            environments,
        }
    }

    /// Returns the grounded ingress.
    #[must_use]
    pub const fn ingress(&self) -> &BTreeSet<ClaimRef> {
        &self.ingress
    }

    /// Returns every declared support route.
    #[must_use]
    pub fn environments(&self) -> &[SupportEnvironment] {
        &self.environments
    }
}

/// The result of the fixed-point computation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Standing {
    claims: BTreeSet<ClaimRef>,
    admitted_by: BTreeMap<ClaimRef, usize>,
    rounds: usize,
}

impl Standing {
    /// Returns every claim that stands.
    #[must_use]
    pub const fn claims(&self) -> &BTreeSet<ClaimRef> {
        &self.claims
    }

    /// Reports whether one claim stands.
    #[must_use]
    pub fn contains(&self, claim: ClaimRef) -> bool {
        self.claims.contains(&claim)
    }

    /// Returns the index of the route that first admitted a claim, when one did.
    ///
    /// Ingress has no admitting route, so a grounded claim answers `None`. This is provenance for
    /// reading the result, not a claim that the route is the only one that would have worked.
    #[must_use]
    pub fn admitted_by(&self, claim: ClaimRef) -> Option<usize> {
        self.admitted_by.get(&claim).copied()
    }

    /// Returns how many iterations the fixed point took to close.
    #[must_use]
    pub const fn rounds(&self) -> usize {
        self.rounds
    }
}

/// Computes `Stand = mu T`, the least fixed point of the support operator.
///
/// Iteration starts from the empty set and adds only what a closed route already reaches, which is
/// what makes the result least rather than merely consistent. The distinction is the whole content
/// of the no-rootless-cycle theorem: a group of claims that support one another and nothing else
/// is a perfectly consistent set, so the *greatest* fixed point contains it. Starting from nothing
/// and growing, no member is ever reachable, and none is admitted.
///
/// Standing here follows from the declarations supplied. It does not check that an ingress fact is
/// grounded, that a declared check ran, or that an applicability condition holds.
#[must_use]
pub fn standing(problem: &StandingProblem) -> Standing {
    let mut claims: BTreeSet<ClaimRef> = problem.ingress().iter().copied().collect();
    let mut admitted_by = BTreeMap::new();
    let mut rounds = 0;

    loop {
        rounds += 1;
        let mut grew = false;
        for (index, environment) in problem.environments().iter().enumerate() {
            if claims.contains(&environment.claim()) {
                continue;
            }
            if environment.is_closed(&claims) {
                claims.insert(environment.claim());
                admitted_by.insert(environment.claim(), index);
                grew = true;
            }
        }
        if !grew {
            break;
        }
    }

    Standing {
        claims,
        admitted_by,
        rounds,
    }
}

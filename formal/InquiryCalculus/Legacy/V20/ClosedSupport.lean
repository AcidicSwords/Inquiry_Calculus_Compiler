import InquiryCalculus.Legacy.V20.OpenDependencyBoundary

/-! # Closed support

Source-bound realization of v2.0 lines 4564–4575. Closed support is relative to an
exact standing set, environment, and candidate. Its five source clauses remain distinct;
the applicability/scope clause retains both subrelations. Closure does not place the
candidate in standing.
-/
namespace InquiryCalculus.Legacy.V20.ClosedSupport

universe u v w x y z q

structure ClosureContext (Candidate : Type u) (Environment : Type v)
    (StandingSet : Type w) (Premise : Type x) (Dependency : Type y)
    (IndependentCheck : Type z) (InconsistencyPolicy : Type q) where
  requiresStandingPremise : Environment → Premise → Prop
  belongsToStanding : StandingSet → Premise → Prop
  applicable : Environment → Candidate → Prop
  scopeHolds : Environment → Candidate → Prop
  openDependency : Environment → Candidate → Dependency → Prop
  requiredIndependentCheck : Environment → Candidate → IndependentCheck → Prop
  independentCheckSucceeded : IndependentCheck → Prop
  explicitPolicy : Environment → InconsistencyPolicy → Prop
  policyInvalidates : InconsistencyPolicy → Environment → Prop
  targetStanding : StandingSet → Candidate → Prop

def StandingPremisesSatisfied {Candidate : Type u} {Environment : Type v}
    {StandingSet : Type w} {Premise : Type x} {Dependency : Type y}
    {IndependentCheck : Type z} {InconsistencyPolicy : Type q}
    (context : ClosureContext Candidate Environment StandingSet Premise Dependency
      IndependentCheck InconsistencyPolicy)
    (standing : StandingSet) (environment : Environment) : Prop :=
  ∀ premise, context.requiresStandingPremise environment premise →
    context.belongsToStanding standing premise

def ApplicabilityAndScopeHold {Candidate : Type u} {Environment : Type v}
    {StandingSet : Type w} {Premise : Type x} {Dependency : Type y}
    {IndependentCheck : Type z} {InconsistencyPolicy : Type q}
    (context : ClosureContext Candidate Environment StandingSet Premise Dependency
      IndependentCheck InconsistencyPolicy)
    (environment : Environment) (candidate : Candidate) : Prop :=
  context.applicable environment candidate ∧ context.scopeHolds environment candidate

def OpenBoundaryEmpty {Candidate : Type u} {Environment : Type v}
    {StandingSet : Type w} {Premise : Type x} {Dependency : Type y}
    {IndependentCheck : Type z} {InconsistencyPolicy : Type q}
    (context : ClosureContext Candidate Environment StandingSet Premise Dependency
      IndependentCheck InconsistencyPolicy)
    (environment : Environment) (candidate : Candidate) : Prop :=
  ∀ dependency, ¬ context.openDependency environment candidate dependency

def RequiredChecksSucceeded {Candidate : Type u} {Environment : Type v}
    {StandingSet : Type w} {Premise : Type x} {Dependency : Type y}
    {IndependentCheck : Type z} {InconsistencyPolicy : Type q}
    (context : ClosureContext Candidate Environment StandingSet Premise Dependency
      IndependentCheck InconsistencyPolicy)
    (environment : Environment) (candidate : Candidate) : Prop :=
  ∀ check, context.requiredIndependentCheck environment candidate check →
    context.independentCheckSucceeded check

def NoExplicitPolicyInvalidates {Candidate : Type u} {Environment : Type v}
    {StandingSet : Type w} {Premise : Type x} {Dependency : Type y}
    {IndependentCheck : Type z} {InconsistencyPolicy : Type q}
    (context : ClosureContext Candidate Environment StandingSet Premise Dependency
      IndependentCheck InconsistencyPolicy)
    (environment : Environment) : Prop :=
  ∀ policy, context.explicitPolicy environment policy →
    ¬ context.policyInvalidates policy environment

/-- The five source clauses, with applicability and scope retained as separate subrelations. -/
def IsClosedSupport {Candidate : Type u} {Environment : Type v}
    {StandingSet : Type w} {Premise : Type x} {Dependency : Type y}
    {IndependentCheck : Type z} {InconsistencyPolicy : Type q}
    (context : ClosureContext Candidate Environment StandingSet Premise Dependency
      IndependentCheck InconsistencyPolicy)
    (standing : StandingSet) (environment : Environment) (candidate : Candidate) : Prop :=
  StandingPremisesSatisfied context standing environment ∧
    ApplicabilityAndScopeHold context environment candidate ∧
    OpenBoundaryEmpty context environment candidate ∧
    RequiredChecksSucceeded context environment candidate ∧
    NoExplicitPolicyInvalidates context environment

theorem closedSupportRetainsStandingPremises {Candidate : Type u} {Environment : Type v}
    {StandingSet : Type w} {Premise : Type x} {Dependency : Type y}
    {IndependentCheck : Type z} {InconsistencyPolicy : Type q}
    {context : ClosureContext Candidate Environment StandingSet Premise Dependency
      IndependentCheck InconsistencyPolicy}
    {standing : StandingSet} {environment : Environment} {candidate : Candidate}
    (closed : IsClosedSupport context standing environment candidate) :
    StandingPremisesSatisfied context standing environment :=
  closed.1

theorem closedSupportRetainsApplicabilityAndScope {Candidate : Type u} {Environment : Type v}
    {StandingSet : Type w} {Premise : Type x} {Dependency : Type y}
    {IndependentCheck : Type z} {InconsistencyPolicy : Type q}
    {context : ClosureContext Candidate Environment StandingSet Premise Dependency
      IndependentCheck InconsistencyPolicy}
    {standing : StandingSet} {environment : Environment} {candidate : Candidate}
    (closed : IsClosedSupport context standing environment candidate) :
    context.applicable environment candidate ∧ context.scopeHolds environment candidate :=
  closed.2.1

theorem closedSupportRetainsEmptyBoundary {Candidate : Type u} {Environment : Type v}
    {StandingSet : Type w} {Premise : Type x} {Dependency : Type y}
    {IndependentCheck : Type z} {InconsistencyPolicy : Type q}
    {context : ClosureContext Candidate Environment StandingSet Premise Dependency
      IndependentCheck InconsistencyPolicy}
    {standing : StandingSet} {environment : Environment} {candidate : Candidate}
    (closed : IsClosedSupport context standing environment candidate) :
    OpenBoundaryEmpty context environment candidate :=
  closed.2.2.1

theorem closedSupportRetainsChecks {Candidate : Type u} {Environment : Type v}
    {StandingSet : Type w} {Premise : Type x} {Dependency : Type y}
    {IndependentCheck : Type z} {InconsistencyPolicy : Type q}
    {context : ClosureContext Candidate Environment StandingSet Premise Dependency
      IndependentCheck InconsistencyPolicy}
    {standing : StandingSet} {environment : Environment} {candidate : Candidate}
    (closed : IsClosedSupport context standing environment candidate) :
    RequiredChecksSucceeded context environment candidate :=
  closed.2.2.2.1

theorem closedSupportRetainsPolicyBoundary {Candidate : Type u} {Environment : Type v}
    {StandingSet : Type w} {Premise : Type x} {Dependency : Type y}
    {IndependentCheck : Type z} {InconsistencyPolicy : Type q}
    {context : ClosureContext Candidate Environment StandingSet Premise Dependency
      IndependentCheck InconsistencyPolicy}
    {standing : StandingSet} {environment : Environment} {candidate : Candidate}
    (closed : IsClosedSupport context standing environment candidate) :
    NoExplicitPolicyInvalidates context environment :=
  closed.2.2.2.2

namespace Countermodel

inductive Policy where
  | accepted
  | invalidating
  deriving DecidableEq

structure Environment where
  requiresStanding : Bool
  applicable : Bool
  inScope : Bool
  hasOpenDependency : Bool
  checkSucceeded : Bool
  policy : Option Policy
  deriving DecidableEq

def completeEnvironment : Environment := ⟨true, true, true, false, true, some .accepted⟩
def inapplicableEnvironment : Environment := { completeEnvironment with applicable := false }
def outOfScopeEnvironment : Environment := { completeEnvironment with inScope := false }
def openBoundaryEnvironment : Environment := { completeEnvironment with hasOpenDependency := true }
def failedCheckEnvironment : Environment := { completeEnvironment with checkSucceeded := false }
def invalidatedEnvironment : Environment := { completeEnvironment with policy := some .invalidating }
def noPolicyEnvironment : Environment := { completeEnvironment with policy := none }

def context : ClosureContext Unit Environment Bool Unit Unit Bool Policy where
  requiresStandingPremise := fun environment _ => environment.requiresStanding = true
  belongsToStanding := fun standing _ => standing = true
  applicable := fun environment _ => environment.applicable = true
  scopeHolds := fun environment _ => environment.inScope = true
  openDependency := fun environment _ _ => environment.hasOpenDependency = true
  requiredIndependentCheck := fun environment _ check => check = environment.checkSucceeded
  independentCheckSucceeded := fun check => check = true
  explicitPolicy := fun environment policy => environment.policy = some policy
  policyInvalidates := fun policy _ => policy = .invalidating
  targetStanding := fun _ _ => False

theorem completeRouteIsClosed : IsClosedSupport context true completeEnvironment () := by
  refine ⟨?_, ⟨rfl, rfl⟩, ?_, ?_, ?_⟩
  · intro premise required
    cases premise
    exact rfl
  · intro dependency openDependency
    cases dependency
    exact Bool.noConfusion openDependency
  · intro check required
    exact required
  · intro policy explicit invalidates
    cases policy with
    | accepted => cases invalidates
    | invalidating => cases explicit

theorem emptyBoundaryAloneIsInsufficient :
    OpenBoundaryEmpty context inapplicableEnvironment () ∧
      ¬ IsClosedSupport context true inapplicableEnvironment () := by
  constructor
  · intro dependency openDependency
    cases dependency
    exact Bool.noConfusion openDependency
  · intro closed
    exact Bool.noConfusion closed.2.1.1

theorem sixAtomicAblationsBreakClosure :
    ¬ IsClosedSupport context false completeEnvironment () ∧
      ¬ IsClosedSupport context true inapplicableEnvironment () ∧
      ¬ IsClosedSupport context true outOfScopeEnvironment () ∧
      ¬ IsClosedSupport context true openBoundaryEnvironment () ∧
      ¬ IsClosedSupport context true failedCheckEnvironment () ∧
      ¬ IsClosedSupport context true invalidatedEnvironment () := by
  constructor
  · intro closed
    have standing := closed.1 () rfl
    exact Bool.noConfusion standing
  constructor
  · intro closed
    exact Bool.noConfusion closed.2.1.1
  constructor
  · intro closed
    exact Bool.noConfusion closed.2.1.2
  constructor
  · intro closed
    exact closed.2.2.1 () rfl
  constructor
  · intro closed
    have succeeded := closed.2.2.2.1 false rfl
    exact Bool.noConfusion succeeded
  · intro closed
    exact closed.2.2.2.2 .invalidating rfl rfl

theorem closedRouteDoesNotEstablishTargetStanding :
    IsClosedSupport context true completeEnvironment () ∧
      ¬ context.targetStanding true () :=
  ⟨completeRouteIsClosed, fun standing => standing⟩

theorem policyAbsenceDiffersFromExplicitNonInvalidation :
    IsClosedSupport context true noPolicyEnvironment () ∧
      (∀ policy, ¬ context.explicitPolicy noPolicyEnvironment policy) ∧
      noPolicyEnvironment ≠ completeEnvironment := by
  constructor
  · refine ⟨?_, ⟨rfl, rfl⟩, ?_, ?_, ?_⟩
    · intro premise required
      cases premise
      exact rfl
    · intro dependency openDependency
      cases dependency
      exact Bool.noConfusion openDependency
    · intro check required
      exact required
    · intro policy explicit
      cases explicit
  constructor
  · intro policy explicit
    cases explicit
  · intro equal
    have policyEqual := congrArg Environment.policy equal
    cases policyEqual

end Countermodel
end InquiryCalculus.Legacy.V20.ClosedSupport

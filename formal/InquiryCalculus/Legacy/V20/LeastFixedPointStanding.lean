import InquiryCalculus.Legacy.V20.ClosedSupport

/-! # Least-fixed-point standing

Source-bound realization of v2.0 lines 4579–4611. Grounded ingress is supplied
explicitly, and the standing operator is built from ingress plus supplied closed-support
routes. Standing is the intersection of all pre-fixed points, not an arbitrary fixed point.
The finite countermodel separates a rootless positive cycle, an ingress-rooted chain,
and a non-ingress candidate with no route dependencies.
-/
namespace InquiryCalculus.Legacy.V20.LeastFixedPointStanding

universe u v

abbrev CandidateSet (Candidate : Type u) := Candidate → Prop

def Included {Candidate : Type u} (left right : CandidateSet Candidate) : Prop :=
  ∀ candidate, left candidate → right candidate

def MonotoneOperator {Candidate : Type u}
    (operator : CandidateSet Candidate → CandidateSet Candidate) : Prop :=
  ∀ {left right}, Included left right → Included (operator left) (operator right)

def IsPreFixedPoint {Candidate : Type u}
    (operator : CandidateSet Candidate → CandidateSet Candidate)
    (candidateSet : CandidateSet Candidate) : Prop :=
  Included (operator candidateSet) candidateSet

def IsFixedPoint {Candidate : Type u}
    (operator : CandidateSet Candidate → CandidateSet Candidate)
    (candidateSet : CandidateSet Candidate) : Prop :=
  ∀ candidate, operator candidateSet candidate ↔ candidateSet candidate

/-- Intersection of all pre-fixed points; this definition makes leastness explicit. -/
def LeastFixedPoint {Candidate : Type u}
    (operator : CandidateSet Candidate → CandidateSet Candidate) : CandidateSet Candidate :=
  fun candidate => ∀ candidateSet, IsPreFixedPoint operator candidateSet → candidateSet candidate

theorem leastIncludedInPreFixedPoint {Candidate : Type u}
    {operator : CandidateSet Candidate → CandidateSet Candidate}
    {candidateSet : CandidateSet Candidate}
    (preFixed : IsPreFixedPoint operator candidateSet) :
    Included (LeastFixedPoint operator) candidateSet := by
  intro candidate inLeast
  exact inLeast candidateSet preFixed

theorem operatorLeastIncludedInLeast {Candidate : Type u}
    {operator : CandidateSet Candidate → CandidateSet Candidate}
    (monotone : MonotoneOperator operator) :
    Included (operator (LeastFixedPoint operator)) (LeastFixedPoint operator) := by
  intro candidate inOperator candidateSet preFixed
  exact preFixed candidate
    (monotone (leastIncludedInPreFixedPoint preFixed) candidate inOperator)

theorem leastIncludedInOperatorLeast {Candidate : Type u}
    {operator : CandidateSet Candidate → CandidateSet Candidate}
    (monotone : MonotoneOperator operator) :
    Included (LeastFixedPoint operator) (operator (LeastFixedPoint operator)) := by
  intro candidate inLeast
  apply inLeast (operator (LeastFixedPoint operator))
  intro next inTwice
  exact monotone (operatorLeastIncludedInLeast monotone) next inTwice

theorem leastIsFixedPoint {Candidate : Type u}
    {operator : CandidateSet Candidate → CandidateSet Candidate}
    (monotone : MonotoneOperator operator) :
    IsFixedPoint operator (LeastFixedPoint operator) := by
  intro candidate
  exact ⟨operatorLeastIncludedInLeast monotone candidate,
    leastIncludedInOperatorLeast monotone candidate⟩

theorem leastAmongFixedPoints {Candidate : Type u}
    {operator : CandidateSet Candidate → CandidateSet Candidate}
    {candidateSet : CandidateSet Candidate}
    (fixed : IsFixedPoint operator candidateSet) :
    Included (LeastFixedPoint operator) candidateSet := by
  apply leastIncludedInPreFixedPoint
  intro candidate inOperator
  exact (fixed candidate).mp inOperator

/-- `closedSupport standing route candidate` is supplied by the preceding typed boundary. -/
structure StandingContext (Candidate : Type u) (Route : Type v) where
  ingress : Candidate → Prop
  routeTarget : Route → Candidate → Prop
  requires : Route → Candidate → Prop
  closedSupport : CandidateSet Candidate → Route → Candidate → Prop
  closedSupportMonotone : ∀ {left right}, Included left right →
    ∀ {route candidate}, closedSupport left route candidate → closedSupport right route candidate

def StandingOperator {Candidate : Type u} {Route : Type v}
    (context : StandingContext Candidate Route) (standing : CandidateSet Candidate) :
    CandidateSet Candidate :=
  fun candidate => context.ingress candidate ∨
    ∃ route, context.closedSupport standing route candidate

theorem standingOperatorMonotone {Candidate : Type u} {Route : Type v}
    (context : StandingContext Candidate Route) :
    MonotoneOperator (StandingOperator context) := by
  intro left right included candidate generated
  cases generated with
  | inl ingress => exact Or.inl ingress
  | inr supported =>
      obtain ⟨route, closed⟩ := supported
      exact Or.inr ⟨route, context.closedSupportMonotone included closed⟩

def Standing {Candidate : Type u} {Route : Type v}
    (context : StandingContext Candidate Route) : CandidateSet Candidate :=
  LeastFixedPoint (StandingOperator context)

theorem standingIsLeastFixedPoint {Candidate : Type u} {Route : Type v}
    (context : StandingContext Candidate Route) :
    IsFixedPoint (StandingOperator context) (Standing context) :=
  leastIsFixedPoint (standingOperatorMonotone context)

theorem ingressStands {Candidate : Type u} {Route : Type v}
    {context : StandingContext Candidate Route} {candidate : Candidate}
    (ingress : context.ingress candidate) : Standing context candidate := by
  exact operatorLeastIncludedInLeast (standingOperatorMonotone context) candidate (Or.inl ingress)

theorem closedSupportIntoStandingStands {Candidate : Type u} {Route : Type v}
    {context : StandingContext Candidate Route} {route : Route} {candidate : Candidate}
    (closed : context.closedSupport (Standing context) route candidate) :
    Standing context candidate := by
  exact operatorLeastIncludedInLeast (standingOperatorMonotone context) candidate
    (Or.inr ⟨route, closed⟩)

/-- A finite rootless region whose every closed route positively requires a member of itself. -/
structure PositiveRootlessRegion {Candidate : Type u} {Route : Type v}
    (context : StandingContext Candidate Route) where
  member : Candidate → Prop
  finiteEnumeration : List Candidate
  covers : ∀ candidate, member candidate → candidate ∈ finiteEnumeration
  noIngress : ∀ candidate, member candidate → ¬ context.ingress candidate
  closedRouteRequiresRegionMember : ∀ {standing route candidate},
    member candidate → context.closedSupport standing route candidate →
      ∃ prerequisite, member prerequisite ∧ context.requires route prerequisite ∧
        standing prerequisite

/-- Exact finite predecessor theorem: positive rootless support cannot enter least standing. -/
theorem noRootlessPositiveSupportCycle {Candidate : Type u} {Route : Type v}
    {context : StandingContext Candidate Route}
    (region : PositiveRootlessRegion context) :
    ∀ candidate, region.member candidate → ¬ Standing context candidate := by
  intro candidate inRegion inStanding
  let outside : CandidateSet Candidate := fun item => ¬ region.member item
  have outsidePreFixed : IsPreFixedPoint (StandingOperator context) outside := by
    intro item generated itemInRegion
    cases generated with
    | inl ingress => exact region.noIngress item itemInRegion ingress
    | inr supported =>
        obtain ⟨route, closed⟩ := supported
        obtain ⟨prerequisite, prerequisiteInRegion, _, prerequisiteOutside⟩ :=
          region.closedRouteRequiresRegionMember itemInRegion closed
        exact prerequisiteOutside prerequisiteInRegion
  exact inStanding outside outsidePreFixed inRegion

/-- Iteration is defined separately; least-fixed-point existence does not assert its termination. -/
def Iterate {Candidate : Type u}
    (operator : CandidateSet Candidate → CandidateSet Candidate) : Nat → CandidateSet Candidate
  | 0 => fun _ => False
  | step + 1 => operator (Iterate operator step)

theorem iterateIncludedInLeast {Candidate : Type u}
    {operator : CandidateSet Candidate → CandidateSet Candidate}
    (monotone : MonotoneOperator operator) :
    ∀ step, Included (Iterate operator step) (LeastFixedPoint operator)
  | 0 => by
      intro candidate impossible
      exact False.elim impossible
  | step + 1 => by
      intro candidate generated
      exact operatorLeastIncludedInLeast monotone candidate
        (monotone (iterateIncludedInLeast monotone step) candidate generated)

namespace Countermodel

inductive Candidate where
  | root
  | rootedChild
  | cycleLeft
  | cycleRight
  | orphan
  deriving DecidableEq

inductive Route where
  | rootToChild
  | rightToLeft
  | leftToRight
  deriving DecidableEq

def ingress (candidate : Candidate) : Prop :=
  candidate = .root

def routeTarget (route : Route) (candidate : Candidate) : Prop :=
  (route = .rootToChild ∧ candidate = .rootedChild) ∨
    (route = .rightToLeft ∧ candidate = .cycleLeft) ∨
    (route = .leftToRight ∧ candidate = .cycleRight)

def requires (route : Route) (candidate : Candidate) : Prop :=
  (route = .rootToChild ∧ candidate = .root) ∨
    (route = .rightToLeft ∧ candidate = .cycleRight) ∨
    (route = .leftToRight ∧ candidate = .cycleLeft)

def closedSupport (standing : CandidateSet Candidate) (route : Route)
    (candidate : Candidate) : Prop :=
  (route = .rootToChild ∧ candidate = .rootedChild ∧ standing .root) ∨
    (route = .rightToLeft ∧ candidate = .cycleLeft ∧ standing .cycleRight) ∨
    (route = .leftToRight ∧ candidate = .cycleRight ∧ standing .cycleLeft)

def context : StandingContext Candidate Route where
  ingress := ingress
  routeTarget := routeTarget
  requires := requires
  closedSupport := closedSupport
  closedSupportMonotone := by
    intro left right included route candidate closed
    rcases closed with rootRoute | rightRoute | leftRoute
    · exact Or.inl ⟨rootRoute.1, rootRoute.2.1, included .root rootRoute.2.2⟩
    · exact Or.inr (Or.inl
        ⟨rightRoute.1, rightRoute.2.1, included .cycleRight rightRoute.2.2⟩)
    · exact Or.inr (Or.inr
        ⟨leftRoute.1, leftRoute.2.1, included .cycleLeft leftRoute.2.2⟩)

def rooted (candidate : Candidate) : Prop :=
  candidate = .root ∨ candidate = .rootedChild

def cycle (candidate : Candidate) : Prop :=
  candidate = .cycleLeft ∨ candidate = .cycleRight

def overlarge (candidate : Candidate) : Prop :=
  candidate ≠ .orphan

theorem rootedIsPreFixed : IsPreFixedPoint (StandingOperator context) rooted := by
  intro candidate generated
  rcases generated with ingressEvidence | ⟨route, closed⟩
  · exact Or.inl ingressEvidence
  · rcases closed with rootRoute | rightRoute | leftRoute
    · exact Or.inr rootRoute.2.1
    · rcases rightRoute.2.2 with impossible | impossible <;> cases impossible
    · rcases leftRoute.2.2 with impossible | impossible <;> cases impossible

theorem standingIncludedInRooted : Included (Standing context) rooted :=
  leastIncludedInPreFixedPoint rootedIsPreFixed

theorem rootStands : Standing context .root :=
  ingressStands rfl

theorem rootedChildStands : Standing context .rootedChild := by
  apply closedSupportIntoStandingStands (route := .rootToChild)
  exact Or.inl ⟨rfl, rfl, rootStands⟩

theorem leastStandingExactlyRooted (candidate : Candidate) :
    Standing context candidate ↔ rooted candidate := by
  cases candidate with
  | root => exact ⟨standingIncludedInRooted .root, fun _ => rootStands⟩
  | rootedChild => exact ⟨standingIncludedInRooted .rootedChild, fun _ => rootedChildStands⟩
  | cycleLeft => exact ⟨standingIncludedInRooted .cycleLeft, fun impossible => by
      rcases impossible with impossible | impossible <;> cases impossible⟩
  | cycleRight => exact ⟨standingIncludedInRooted .cycleRight, fun impossible => by
      rcases impossible with impossible | impossible <;> cases impossible⟩
  | orphan => exact ⟨standingIncludedInRooted .orphan, fun impossible => by
      rcases impossible with impossible | impossible <;> cases impossible⟩

def rootlessCycle : PositiveRootlessRegion context where
  member := cycle
  finiteEnumeration := [.cycleLeft, .cycleRight]
  covers := by
    intro candidate member
    rcases member with left | right
    · cases left
      exact @List.Mem.head Candidate .cycleLeft [.cycleRight]
    · cases right
      exact List.Mem.tail .cycleLeft (@List.Mem.head Candidate .cycleRight [])
  noIngress := by
    intro candidate member ingressEvidence
    rcases member with left | right
    · cases left
      cases ingressEvidence
    · cases right
      cases ingressEvidence
  closedRouteRequiresRegionMember := by
    intro standing route candidate member closed
    rcases closed with rootRoute | rightRoute | leftRoute
    · cases rootRoute.2.1
      rcases member with impossible | impossible <;> cases impossible
    · exact ⟨.cycleRight, Or.inr rfl, Or.inr (Or.inl ⟨rightRoute.1, rfl⟩),
        rightRoute.2.2⟩
    · exact ⟨.cycleLeft, Or.inl rfl, Or.inr (Or.inr ⟨leftRoute.1, rfl⟩),
        leftRoute.2.2⟩

theorem rootlessCycleExcluded :
    (¬ Standing context .cycleLeft) ∧ (¬ Standing context .cycleRight) := by
  constructor
  · exact noRootlessPositiveSupportCycle rootlessCycle .cycleLeft (Or.inl rfl)
  · exact noRootlessPositiveSupportCycle rootlessCycle .cycleRight (Or.inr rfl)

theorem overlargeIsFixedPoint : IsFixedPoint (StandingOperator context) overlarge := by
  intro candidate
  constructor
  · intro generated equalsOrphan
    rcases generated with ingressEvidence | ⟨route, closed⟩
    · cases ingressEvidence
      cases equalsOrphan
    · rcases closed with rootRoute | rightRoute | leftRoute
      · cases rootRoute.2.1
        cases equalsOrphan
      · cases rightRoute.2.1
        cases equalsOrphan
      · cases leftRoute.2.1
        cases equalsOrphan
  · intro notOrphan
    cases candidate with
    | root => exact Or.inl rfl
    | rootedChild => exact Or.inr ⟨.rootToChild, Or.inl ⟨rfl, rfl, by
        intro impossible
        cases impossible⟩⟩
    | cycleLeft => exact Or.inr ⟨.rightToLeft, Or.inr (Or.inl ⟨rfl, rfl, by
        intro impossible
        cases impossible⟩)⟩
    | cycleRight => exact Or.inr ⟨.leftToRight, Or.inr (Or.inr ⟨rfl, rfl, by
        intro impossible
        cases impossible⟩)⟩
    | orphan => exact False.elim (notOrphan rfl)

theorem arbitraryFixedPointIsNotStanding :
    IsFixedPoint (StandingOperator context) overlarge ∧
      overlarge .cycleLeft ∧ ¬ Standing context .cycleLeft :=
  ⟨overlargeIsFixedPoint, (by intro impossible; cases impossible), rootlessCycleExcluded.1⟩

def hasNoRouteDependencies (candidate : Candidate) : Prop :=
  ∀ route prerequisite, routeTarget route candidate → ¬ requires route prerequisite

theorem emptyDependenciesAreNotIngress :
    hasNoRouteDependencies .orphan ∧ ¬ context.ingress .orphan := by
  constructor
  · intro route prerequisite target
    rcases target with rootRoute | rightRoute | leftRoute
    · cases rootRoute.2
    · cases rightRoute.2
    · cases leftRoute.2
  · intro ingressEvidence
    cases ingressEvidence

/-- A finite-model calculation only; it makes no claim about general effective convergence. -/
theorem finiteModelStabilizesAtTwo :
    ∀ candidate,
      Iterate (StandingOperator context) 2 candidate ↔
        Iterate (StandingOperator context) 3 candidate := by
  intro candidate
  cases candidate with
  | root =>
      constructor <;> intro _ <;> exact Or.inl rfl
  | rootedChild =>
      constructor
      · intro _
        exact Or.inr ⟨.rootToChild, Or.inl ⟨rfl, rfl, Or.inl rfl⟩⟩
      · intro _
        exact Or.inr ⟨.rootToChild, Or.inl ⟨rfl, rfl, Or.inl rfl⟩⟩
  | cycleLeft =>
      constructor
      · intro generated
        exact False.elim (rootlessCycleExcluded.1
          (iterateIncludedInLeast (standingOperatorMonotone context) 2 .cycleLeft generated))
      · intro generated
        exact False.elim (rootlessCycleExcluded.1
          (iterateIncludedInLeast (standingOperatorMonotone context) 3 .cycleLeft generated))
  | cycleRight =>
      constructor
      · intro generated
        exact False.elim (rootlessCycleExcluded.2
          (iterateIncludedInLeast (standingOperatorMonotone context) 2 .cycleRight generated))
      · intro generated
        exact False.elim (rootlessCycleExcluded.2
          (iterateIncludedInLeast (standingOperatorMonotone context) 3 .cycleRight generated))
  | orphan =>
      constructor <;> intro generated
      · rcases generated with ingressEvidence | ⟨route, closed⟩
        · cases ingressEvidence
        · rcases closed with rootRoute | rightRoute | leftRoute
          · cases rootRoute.2.1
          · cases rightRoute.2.1
          · cases leftRoute.2.1
      · rcases generated with ingressEvidence | ⟨route, closed⟩
        · cases ingressEvidence
        · rcases closed with rootRoute | rightRoute | leftRoute
          · cases rootRoute.2.1
          · cases rightRoute.2.1
          · cases leftRoute.2.1

end Countermodel
end InquiryCalculus.Legacy.V20.LeastFixedPointStanding

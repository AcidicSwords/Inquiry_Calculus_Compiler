import InquiryCalculus.Legacy.V20.ClaimLifecycle

/-! # Support environments and minimal support families

Source-bound realization of v2.0 lines 4516–4533, sharpened only by the
construction-specification requirement that support be an explicit supplied relation.
A candidate environment is not thereby supporting, minimality is subset-minimality rather
than leastness or uniqueness, and support supplies neither warrant nor standing.
-/
namespace InquiryCalculus.Legacy.V20.SupportEnvironments

universe u v w x y z r

/-- The five typed kinds of material named by the predecessor support-environment definition. -/
inductive TypedSupportAtom (Premise : Type u) (ActualReturn : Type v)
    (CheckerResult : Type w) (Assumption : Type x) (StandingRelation : Type y) where
  | premise : Premise → TypedSupportAtom Premise ActualReturn CheckerResult Assumption StandingRelation
  | actualReturn : ActualReturn → TypedSupportAtom Premise ActualReturn CheckerResult Assumption StandingRelation
  | checkerResult : CheckerResult → TypedSupportAtom Premise ActualReturn CheckerResult Assumption StandingRelation
  | assumption : Assumption → TypedSupportAtom Premise ActualReturn CheckerResult Assumption StandingRelation
  | standingRelation : StandingRelation → TypedSupportAtom Premise ActualReturn CheckerResult Assumption StandingRelation
  deriving DecidableEq

/-- A duplicate-free finite set represented without quotient or choice axioms. -/
structure FiniteSupportSet (Atom : Type u) where
  items : List Atom
  nodup : items.Nodup
  deriving DecidableEq

/-- The predecessor admits either an explicitly finite set or a supplied finite representation. -/
inductive SupportContent (Atom : Type u) (FiniteRepresentation : Type v) where
  | finite : FiniteSupportSet Atom → SupportContent Atom FiniteRepresentation
  | represented : FiniteRepresentation → SupportContent Atom FiniteRepresentation
  deriving DecidableEq

/-- Merely constructing this value proposes an environment for a candidate; it does not support it. -/
structure CandidateSupportEnvironment (Candidate : Type u) (Atom : Type v)
    (FiniteRepresentation : Type w) where
  target : Candidate
  content : SupportContent Atom FiniteRepresentation
  deriving DecidableEq

/-- All semantic relations are supplied. In particular, `supports` is not membership or candidacy. -/
structure SupportContext (Candidate : Type u) (Atom : Type v)
    (FiniteRepresentation : Type w) where
  contains : SupportContent Atom FiniteRepresentation → Atom → Prop
  finiteContains : ∀ items atom, contains (.finite items) atom ↔ atom ∈ items.items
  supports : CandidateSupportEnvironment Candidate Atom FiniteRepresentation → Prop
  warranted : Candidate → Prop
  standing : Candidate → Prop

def EnvironmentSubset {Candidate : Type u} {Atom : Type v} {FiniteRepresentation : Type w}
    (context : SupportContext Candidate Atom FiniteRepresentation)
    (left right : CandidateSupportEnvironment Candidate Atom FiniteRepresentation) : Prop :=
  left.target = right.target ∧
    ∀ atom, context.contains left.content atom → context.contains right.content atom

def ProperEnvironmentSubset {Candidate : Type u} {Atom : Type v} {FiniteRepresentation : Type w}
    (context : SupportContext Candidate Atom FiniteRepresentation)
    (left right : CandidateSupportEnvironment Candidate Atom FiniteRepresentation) : Prop :=
  EnvironmentSubset context left right ∧ ¬ EnvironmentSubset context right left

/-- Minimal means that no proper subenvironment is supported; it does not assert a least element. -/
def IsMinimalSupport {Candidate : Type u} {Atom : Type v} {FiniteRepresentation : Type w}
    (context : SupportContext Candidate Atom FiniteRepresentation)
    (environment : CandidateSupportEnvironment Candidate Atom FiniteRepresentation) : Prop :=
  context.supports environment ∧
    ∀ smaller, ProperEnvironmentSubset context smaller environment → ¬ context.supports smaller

/-- The minimal support family is a predicate and may contain several incomparable environments. -/
def MinimalSupportFamily {Candidate : Type u} {Atom : Type v} {FiniteRepresentation : Type w}
    (context : SupportContext Candidate Atom FiniteRepresentation) (candidate : Candidate)
    (environment : CandidateSupportEnvironment Candidate Atom FiniteRepresentation) : Prop :=
  environment.target = candidate ∧ IsMinimalSupport context environment

theorem finiteMembership {Candidate : Type u} {Atom : Type v} {FiniteRepresentation : Type w}
    (context : SupportContext Candidate Atom FiniteRepresentation) (items : FiniteSupportSet Atom)
    (atom : Atom) : context.contains (.finite items) atom ↔ atom ∈ items.items :=
  context.finiteContains items atom

theorem environmentSubsetRetainsTarget {Candidate : Type u} {Atom : Type v}
    {FiniteRepresentation : Type w} {context : SupportContext Candidate Atom FiniteRepresentation}
    {left right : CandidateSupportEnvironment Candidate Atom FiniteRepresentation}
    (subset : EnvironmentSubset context left right) : left.target = right.target :=
  subset.1

theorem minimalFamilyTargetsCandidate {Candidate : Type u} {Atom : Type v}
    {FiniteRepresentation : Type w} {context : SupportContext Candidate Atom FiniteRepresentation}
    {candidate : Candidate} {environment : CandidateSupportEnvironment Candidate Atom FiniteRepresentation}
    (member : MinimalSupportFamily context candidate environment) : environment.target = candidate :=
  member.1

theorem environmentSubsetRefl {Candidate : Type u} {Atom : Type v}
    {FiniteRepresentation : Type w} (context : SupportContext Candidate Atom FiniteRepresentation)
    (environment : CandidateSupportEnvironment Candidate Atom FiniteRepresentation) :
    EnvironmentSubset context environment environment :=
  ⟨rfl, fun _ membership => membership⟩

namespace Countermodel

abbrev Atom := TypedSupportAtom Unit Unit Unit Unit Unit
abbrev Environment := CandidateSupportEnvironment Unit Atom Unit

def premiseAtom : Atom := .premise ()
def returnAtom : Atom := .actualReturn ()
def checkerAtom : Atom := .checkerResult ()
def assumptionAtom : Atom := .assumption ()

def leftItems : FiniteSupportSet Atom := ⟨[premiseAtom, returnAtom], by decide⟩
def rightItems : FiniteSupportSet Atom := ⟨[checkerAtom, assumptionAtom], by decide⟩
def unionItems : FiniteSupportSet Atom :=
  ⟨[premiseAtom, returnAtom, checkerAtom, assumptionAtom], by decide⟩

def leftEnvironment : Environment := ⟨(), .finite leftItems⟩
def rightEnvironment : Environment := ⟨(), .finite rightItems⟩
def unionEnvironment : Environment := ⟨(), .finite unionItems⟩
def leftWithoutPremise : Environment := ⟨(), .finite ⟨[returnAtom], by decide⟩⟩
def rightWithoutChecker : Environment := ⟨(), .finite ⟨[assumptionAtom], by decide⟩⟩
def unsupportedCandidate : Environment := ⟨(), .finite ⟨[premiseAtom], by decide⟩⟩

def context : SupportContext Unit Atom Unit where
  contains := fun content atom =>
    match content with
    | .finite items => atom ∈ items.items
    | .represented _ => False
  finiteContains := by intros; rfl
  supports := fun environment =>
    environment = leftEnvironment ∨
      environment = rightEnvironment ∨
      environment = unionEnvironment
  warranted := fun _ => False
  standing := fun _ => False

theorem leftSupports : context.supports leftEnvironment :=
  Or.inl rfl

theorem rightSupports : context.supports rightEnvironment :=
  Or.inr (Or.inl rfl)

theorem unionSupports : context.supports unionEnvironment :=
  Or.inr (Or.inr rfl)

theorem premiseInLeft : context.contains leftEnvironment.content premiseAtom :=
  (context.finiteContains leftItems premiseAtom).mpr (List.Mem.head [returnAtom])

theorem checkerInRight : context.contains rightEnvironment.content checkerAtom :=
  (context.finiteContains rightItems checkerAtom).mpr (List.Mem.head [assumptionAtom])

theorem premiseInUnion : context.contains unionEnvironment.content premiseAtom :=
  (context.finiteContains unionItems premiseAtom).mpr
    (List.Mem.head [returnAtom, checkerAtom, assumptionAtom])

theorem returnInUnion : context.contains unionEnvironment.content returnAtom :=
  (context.finiteContains unionItems returnAtom).mpr
    (List.Mem.tail premiseAtom (List.Mem.head [checkerAtom, assumptionAtom]))

theorem checkerInUnion : context.contains unionEnvironment.content checkerAtom :=
  (context.finiteContains unionItems checkerAtom).mpr
    (List.Mem.tail premiseAtom (List.Mem.tail returnAtom (List.Mem.head [assumptionAtom])))

theorem assumptionInUnion : context.contains unionEnvironment.content assumptionAtom :=
  (context.finiteContains unionItems assumptionAtom).mpr
    (List.Mem.tail premiseAtom (List.Mem.tail returnAtom
      (List.Mem.tail checkerAtom (List.Mem.head []))))

theorem checkerNotInLeft : ¬ context.contains leftEnvironment.content checkerAtom := by
  intro membership
  have finite := (context.finiteContains leftItems checkerAtom).mp membership
  unfold leftItems at finite
  cases finite with
  | tail _ tail =>
    cases tail with
    | tail _ impossible => exact nomatch impossible

theorem premiseNotInRight : ¬ context.contains rightEnvironment.content premiseAtom := by
  intro membership
  have finite := (context.finiteContains rightItems premiseAtom).mp membership
  unfold rightItems at finite
  cases finite with
  | tail _ tail =>
    cases tail with
    | tail _ impossible => exact nomatch impossible

theorem leftSubsetUnion : EnvironmentSubset context leftEnvironment unionEnvironment := by
  constructor
  · rfl
  · intro atom membership
    have finite := (context.finiteContains leftItems atom).mp membership
    unfold leftItems at finite
    cases finite with
    | head => exact premiseInUnion
    | tail _ tail =>
      cases tail with
      | head => exact returnInUnion
      | tail _ impossible => exact nomatch impossible

theorem rightSubsetUnion : EnvironmentSubset context rightEnvironment unionEnvironment := by
  constructor
  · rfl
  · intro atom membership
    have finite := (context.finiteContains rightItems atom).mp membership
    unfold rightItems at finite
    cases finite with
    | head => exact checkerInUnion
    | tail _ tail =>
      cases tail with
      | head => exact assumptionInUnion
      | tail _ impossible => exact nomatch impossible

theorem notRightSubsetLeft : ¬ EnvironmentSubset context rightEnvironment leftEnvironment := by
  intro subset
  exact checkerNotInLeft (subset.2 checkerAtom checkerInRight)

theorem notLeftSubsetRight : ¬ EnvironmentSubset context leftEnvironment rightEnvironment := by
  intro subset
  exact premiseNotInRight (subset.2 premiseAtom premiseInLeft)

theorem notUnionSubsetLeft : ¬ EnvironmentSubset context unionEnvironment leftEnvironment := by
  intro subset
  exact checkerNotInLeft (subset.2 checkerAtom checkerInUnion)

theorem notUnionSubsetRight : ¬ EnvironmentSubset context unionEnvironment rightEnvironment := by
  intro subset
  exact premiseNotInRight (subset.2 premiseAtom premiseInUnion)

theorem leftMinimal : IsMinimalSupport context leftEnvironment := by
  constructor
  · exact leftSupports
  · intro smaller proper supported
    rcases supported with equal | equal | equal
    · subst smaller
      exact proper.2 (environmentSubsetRefl context leftEnvironment)
    · subst smaller
      exact notRightSubsetLeft proper.1
    · subst smaller
      exact notUnionSubsetLeft proper.1

theorem rightMinimal : IsMinimalSupport context rightEnvironment := by
  constructor
  · exact rightSupports
  · intro smaller proper supported
    rcases supported with equal | equal | equal
    · subst smaller
      exact notLeftSubsetRight proper.1
    · subst smaller
      exact proper.2 (environmentSubsetRefl context rightEnvironment)
    · subst smaller
      exact notUnionSubsetRight proper.1

theorem twoIncomparableMinimalEnvironments :
    MinimalSupportFamily context () leftEnvironment ∧
      MinimalSupportFamily context () rightEnvironment ∧
      ¬ EnvironmentSubset context leftEnvironment rightEnvironment ∧
      ¬ EnvironmentSubset context rightEnvironment leftEnvironment :=
  ⟨⟨rfl, leftMinimal⟩, ⟨rfl, rightMinimal⟩, notLeftSubsetRight, notRightSubsetLeft⟩

theorem unionIsSupportedButNotMinimal :
    context.supports unionEnvironment ∧ ¬ IsMinimalSupport context unionEnvironment := by
  constructor
  · exact unionSupports
  · intro minimal
    exact minimal.2 leftEnvironment ⟨leftSubsetUnion, notUnionSubsetLeft⟩ leftSupports

theorem oneElementAblationsBreakSupport :
    ¬ context.supports leftWithoutPremise ∧
      ¬ context.supports rightWithoutChecker := by
  change
    ¬ (leftWithoutPremise = leftEnvironment ∨ leftWithoutPremise = rightEnvironment ∨
      leftWithoutPremise = unionEnvironment) ∧
    ¬ (rightWithoutChecker = leftEnvironment ∨ rightWithoutChecker = rightEnvironment ∨
      rightWithoutChecker = unionEnvironment)
  exact ⟨by decide, by decide⟩

theorem candidacyDoesNotEstablishSupport : ¬ context.supports unsupportedCandidate := by
  change ¬ (unsupportedCandidate = leftEnvironment ∨ unsupportedCandidate = rightEnvironment ∨
    unsupportedCandidate = unionEnvironment)
  decide

theorem supportDoesNotEstablishWarrantOrStanding :
    context.supports leftEnvironment ∧
      ¬ context.warranted leftEnvironment.target ∧
      ¬ context.standing leftEnvironment.target := by
  exact ⟨leftSupports, fun warranted => warranted, fun standing => standing⟩

end Countermodel
end InquiryCalculus.Legacy.V20.SupportEnvironments

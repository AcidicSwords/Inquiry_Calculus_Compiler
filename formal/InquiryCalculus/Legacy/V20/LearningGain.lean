import InquiryCalculus.Legacy.V20.AblativeRegeneration

/-! # Learning gain

Source-bound realization of the v2.0 definition at lines 4255–4266, while the
adjacent law at lines 4248–4253 remains an Unproved predecessor obligation.
Gain is evaluated over a declared applicability region and supplied resource
preorder. Promotion additionally requires independent standing.
-/
namespace InquiryCalculus.Legacy.V20.LearningGain

universe u v w

inductive Capability where
  | discriminate | regenerate | traverse | reacquire | cost | robustness
  deriving DecidableEq, Repr

structure ResourcePreorder (Resource : Type w) where
  le : Resource → Resource → Prop
  reflexive : ∀ resource, le resource resource
  transitive : ∀ {left middle right}, le left middle → le middle right → le left right

structure LearningContext (Candidate : Type u) (Region : Type v) (Resource : Type w) where
  applicable : Candidate → Region → Prop
  baselineResource : Region → Resource
  candidateResource : Candidate → Region → Resource
  resourceOrder : ResourcePreorder Resource
  strictlyImproves : Capability → Candidate → Region → Prop
  preservesProtectedBehavior : Candidate → Region → Prop
  preservesWarrantBoundary : Candidate → Region → Prop
  standing : Candidate → Prop

def HasLearningGain {Candidate : Type u} {Region : Type v} {Resource : Type w}
    (context : LearningContext Candidate Region Resource) (candidate : Candidate) : Prop :=
  ∀ region, context.applicable candidate region →
    context.resourceOrder.le (context.candidateResource candidate region)
      (context.baselineResource region) ∧
    (∃ capability, context.strictlyImproves capability candidate region) ∧
    context.preservesProtectedBehavior candidate region ∧
    context.preservesWarrantBoundary candidate region

/-- Gain cannot warrant its own promotion. -/
def PromotionAdmissible {Candidate : Type u} {Region : Type v} {Resource : Type w}
    (context : LearningContext Candidate Region Resource) (candidate : Candidate) : Prop :=
  HasLearningGain context candidate ∧ context.standing candidate

/-- Historical returns may support standing but are not the reusable capacity. -/
structure HistoricalEvidence (Event Candidate : Type u) where
  supports : Event → Candidate → Prop

namespace Countermodel

structure Candidate where
  admitted : Bool
  gains : Capability → Bool
  protection : Bool
  warrant : Bool
  resource : Nat

def allCoordinates : Capability → Bool := fun _ => true
def discriminateOnly : Capability → Bool
  | .discriminate => true
  | _ => false
def noCoordinates : Capability → Bool := fun _ => false

def context : LearningContext Candidate Unit Nat where
  applicable := fun _ _ => True
  baselineResource := fun _ => 1
  candidateResource := fun candidate _ => candidate.resource
  resourceOrder := { le := Nat.le, reflexive := Nat.le_refl, transitive := Nat.le_trans }
  strictlyImproves := fun capability candidate _ => candidate.gains capability = true
  preservesProtectedBehavior := fun candidate _ => candidate.protection = true
  preservesWarrantBoundary := fun candidate _ => candidate.warrant = true
  standing := fun candidate => candidate.admitted = true

def overcomplete : Candidate := ⟨true, allCoordinates, true, true, 0⟩
def minimal : Candidate := ⟨true, discriminateOnly, true, true, 0⟩
def historyOnly : Candidate := ⟨false, noCoordinates, true, true, 1⟩
def equalCapacity : Candidate := ⟨true, noCoordinates, true, true, 1⟩
def breaksProtected : Candidate := ⟨true, discriminateOnly, false, true, 0⟩
def breaksWarrant : Candidate := ⟨true, discriminateOnly, true, false, 0⟩
def lacksStanding : Candidate := ⟨false, discriminateOnly, true, true, 0⟩

theorem overcompleteCrosses : PromotionAdmissible context overcomplete := by
  constructor
  · intro region _
    cases region
    exact ⟨by change (0 : Nat) ≤ 1; decide, ⟨.discriminate, rfl⟩, rfl, rfl⟩
  · rfl

theorem contractedToOneGain : PromotionAdmissible context minimal := by
  constructor
  · intro region _
    cases region
    exact ⟨by change (0 : Nat) ≤ 1; decide, ⟨.discriminate, rfl⟩, rfl, rfl⟩
  · rfl

theorem historyAloneRejected : ¬ PromotionAdmissible context historyOnly := by
  intro admitted
  cases admitted.2

theorem equalCapacityRejected : ¬ PromotionAdmissible context equalCapacity := by
  intro admitted
  obtain ⟨_, ⟨capability, improved⟩, _, _⟩ := admitted.1 () trivial
  cases capability <;> cases improved

theorem breaksProtectedRejected : ¬ PromotionAdmissible context breaksProtected := by
  intro admitted
  cases (admitted.1 () trivial).2.2.1

theorem breaksWarrantRejected : ¬ PromotionAdmissible context breaksWarrant := by
  intro admitted
  cases (admitted.1 () trivial).2.2.2

theorem lacksStandingRejected : ¬ PromotionAdmissible context lacksStanding := by
  intro admitted
  cases admitted.2

end Countermodel
end InquiryCalculus.Legacy.V20.LearningGain

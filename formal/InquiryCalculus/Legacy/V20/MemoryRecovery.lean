import InquiryCalculus.Legacy.V20.QuestionPatternLearning
import InquiryCalculus.Legacy.V20.LearningGain

/-! # Memory and recovery

Source-bound realization of v2.0 lines 4399–4458. Memory is not a new semantic
state kind. Use, reconstruction, evaluation, and reacquisition are four supplied
relations with no universal implication. Reacquisition advantage is a separate
comparison between supplied nondominated frontiers.
-/
namespace InquiryCalculus.Legacy.V20.MemoryRecovery

open LearningGain

universe u v w x

structure RecoveryContext (Material : Type u) (Context : Type v) (Competence : Type w) where
  use : Material → Context → Competence → Prop
  reconstruct : Material → Context → Competence → Prop
  evaluate : Material → Context → Competence → Prop
  reacquire : Material → Context → Competence → Prop

inductive RecoveryCoordinate where
  | use | reconstruct | evaluate | reacquire
  deriving DecidableEq, Repr

structure NondominatedFrontier (Resource : Type u) (order : ResourcePreorder Resource) where
  member : Resource → Prop
  inhabited : ∃ resource, member resource
  nondominated : ∀ resource, member resource →
    ¬ ∃ other, member other ∧ order.le other resource ∧ ¬ order.le resource other

structure ReacquisitionComparison (Frontier : Type u) where
  strictlyBetter : Frontier → Frontier → Prop

def HasReacquisitionAdvantage {Frontier : Type u}
    (comparison : ReacquisitionComparison Frontier) (withMaterial baseline : Frontier) : Prop :=
  comparison.strictlyBetter withMaterial baseline

/-- The adjacent architecture remains a predecessor obligation, not a consequence
of the four relations or of one finite frontier comparison. -/
inductive MemoryRecoveryObligation where
  | keepExperiencesCompileRoutes
  | stateCompressionRequiresIndependentLicense
  deriving DecidableEq, Repr

namespace Countermodel

structure Candidate where
  canUse : Bool
  canReconstruct : Bool
  canEvaluate : Bool
  canReacquire : Bool

def holds (candidate : Candidate) : RecoveryCoordinate → Prop
  | .use => candidate.canUse = true
  | .reconstruct => candidate.canReconstruct = true
  | .evaluate => candidate.canEvaluate = true
  | .reacquire => candidate.canReacquire = true

def only : RecoveryCoordinate → Candidate
  | .use => ⟨true, false, false, false⟩
  | .reconstruct => ⟨false, true, false, false⟩
  | .evaluate => ⟨false, false, true, false⟩
  | .reacquire => ⟨false, false, false, true⟩

theorem everyCoordinateHasAnIndependentWitness (coordinate : RecoveryCoordinate) :
    holds (only coordinate) coordinate := by
  cases coordinate <;> rfl

theorem noCoordinateUniversallyImpliesAnother {source target : RecoveryCoordinate}
    (different : source ≠ target) :
    ∃ candidate, holds candidate source ∧ ¬ holds candidate target := by
  refine ⟨only source, everyCoordinateHasAnIndependentWitness source, ?_⟩
  cases source <;> cases target <;> first | contradiction | (intro impossible; cases impossible)

def recoveryContext : RecoveryContext Candidate Unit Unit where
  use := fun candidate _ _ => holds candidate .use
  reconstruct := fun candidate _ _ => holds candidate .reconstruct
  evaluate := fun candidate _ _ => holds candidate .evaluate
  reacquire := fun candidate _ _ => holds candidate .reacquire

def natOrder : ResourcePreorder Nat where
  le := Nat.le
  reflexive := Nat.le_refl
  transitive := Nat.le_trans

def withMaterial : NondominatedFrontier Nat natOrder where
  member := fun resource => resource = 0
  inhabited := ⟨0, rfl⟩
  nondominated := by
    rintro resource rfl ⟨other, rfl, _, strict⟩
    exact strict (Nat.le_refl 0)

def baseline : NondominatedFrontier Nat natOrder where
  member := fun resource => resource = 1
  inhabited := ⟨1, rfl⟩
  nondominated := by
    rintro resource rfl ⟨other, rfl, _, strict⟩
    exact strict (Nat.le_refl 1)

def frontierComparison : ReacquisitionComparison (NondominatedFrontier Nat natOrder) where
  strictlyBetter := fun left right =>
    ∃ leftResource rightResource, left.member leftResource ∧ right.member rightResource ∧
      leftResource < rightResource

def noAdvantageComparison : ReacquisitionComparison (NondominatedFrontier Nat natOrder) where
  strictlyBetter := fun _ _ => False

theorem retainedMaterialHasAdvantage :
    HasReacquisitionAdvantage frontierComparison withMaterial baseline := by
  exact ⟨0, 1, rfl, rfl, by decide⟩

theorem advantageRequiresSuppliedComparison :
    ¬ HasReacquisitionAdvantage noAdvantageComparison withMaterial baseline := by
  intro advantage
  exact advantage

def reacquireWithoutUseOrReconstruction : Candidate := only .reacquire

theorem advantageDoesNotCreateUseOrReconstruction :
    HasReacquisitionAdvantage frontierComparison withMaterial baseline ∧
    recoveryContext.reacquire reacquireWithoutUseOrReconstruction () () ∧
    ¬ recoveryContext.use reacquireWithoutUseOrReconstruction () () ∧
    ¬ recoveryContext.reconstruct reacquireWithoutUseOrReconstruction () () := by
  refine ⟨retainedMaterialHasAdvantage, rfl, ?_, ?_⟩
  · intro use
    cases use
  · intro reconstruction
    cases reconstruction

end Countermodel
end InquiryCalculus.Legacy.V20.MemoryRecovery

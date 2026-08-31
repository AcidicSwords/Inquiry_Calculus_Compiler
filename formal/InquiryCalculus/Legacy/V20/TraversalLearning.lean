import InquiryCalculus.Legacy.V20.MethodPromotion

/-! # Two forms of traversal learning

Source-bound realization of v2.0 lines 4306–4344. Recurrent path folding and
probe-basis extension are kept as different typed interfaces. Neither executes a
path or a probe, and neither supplies semantic warrant.
-/
namespace InquiryCalculus.Legacy.V20.TraversalLearning

open MethodPromotion

universe u v w x y z

/-- A recurrent path may be folded only by carrying an admitted promotion witness. -/
structure PathFoldAdmission {Method : Type u} {Path : Type v} {Region : Type w}
    {Continuation : Type x} {Evidence : Type y} {Unlock : Type z}
    (context : PromotionContext Method Path Region Continuation Evidence Unlock)
    (method : Method) (path : Path) : Prop where
  promotion : MethodPromotable context method path

def PathFoldAdmissible {Method : Type u} {Path : Type v} {Region : Type w}
    {Continuation : Type x} {Evidence : Type y} {Unlock : Type z}
    (context : PromotionContext Method Path Region Continuation Evidence Unlock)
    (method : Method) (path : Path) : Prop :=
  Nonempty (PathFoldAdmission context method path)

/-- The declared comparison surface for one candidate extension. -/
structure ProbeExtensionContext (Candidate : Type u) (State : Type v) (OldProbe : Type w)
    (Reading : Type x) (Consequence : Type y) (LaterUse : Type z) where
  oldProbe : OldProbe → Prop
  oldObserve : Candidate → OldProbe → State → Reading
  candidateObserve : Candidate → State → Reading
  protectedConsequence : Candidate → State → Consequence
  independentlyDemonstratesBenefit : LaterUse → Candidate → Prop

/-- Positive protected nonredundancy: an explicit pair is hidden by every old probe,
but separated by both the candidate probe and the protected consequence. -/
structure ProtectedNonredundancy {Candidate : Type u} {State : Type v} {OldProbe : Type w}
    {Reading : Type x} {Consequence : Type y} {LaterUse : Type z}
    (context : ProbeExtensionContext Candidate State OldProbe Reading Consequence LaterUse)
    (candidate : Candidate) where
  left : State
  right : State
  oldBasisAgrees : ∀ probe, context.oldProbe probe →
    context.oldObserve candidate probe left = context.oldObserve candidate probe right
  candidateSeparates :
    context.candidateObserve candidate left ≠ context.candidateObserve candidate right
  protectedConsequenceSeparates :
    context.protectedConsequence candidate left ≠ context.protectedConsequence candidate right

/-- Generation establishes only a candidate and its protected nonredundancy witness. -/
def ProbeGenerated {Candidate : Type u} {State : Type v} {OldProbe : Type w}
    {Reading : Type x} {Consequence : Type y} {LaterUse : Type z}
    (context : ProbeExtensionContext Candidate State OldProbe Reading Consequence LaterUse)
    (candidate : Candidate) : Prop :=
  Nonempty (ProtectedNonredundancy context candidate)

/-- Admission additionally requires an independently supplied later-use witness. -/
structure ProbeAdmission {Candidate : Type u} {State : Type v} {OldProbe : Type w}
    {Reading : Type x} {Consequence : Type y} {LaterUse : Type z}
    (context : ProbeExtensionContext Candidate State OldProbe Reading Consequence LaterUse)
    (candidate : Candidate) where
  nonredundant : ProtectedNonredundancy context candidate
  laterUse : LaterUse
  independentBenefit : context.independentlyDemonstratesBenefit laterUse candidate

def ProbeAdmissible {Candidate : Type u} {State : Type v} {OldProbe : Type w}
    {Reading : Type x} {Consequence : Type y} {LaterUse : Type z}
    (context : ProbeExtensionContext Candidate State OldProbe Reading Consequence LaterUse)
    (candidate : Candidate) : Prop :=
  Nonempty (ProbeAdmission context candidate)

namespace Countermodel

structure Candidate where
  oldLeft : Bool
  oldRight : Bool
  freshLeft : Bool
  freshRight : Bool
  protectedLeft : Bool
  protectedRight : Bool
  laterBenefit : Bool

def context : ProbeExtensionContext Candidate Bool Unit Bool Bool Unit where
  oldProbe := fun _ => True
  oldObserve := fun candidate _ state => if state then candidate.oldRight else candidate.oldLeft
  candidateObserve := fun candidate state => if state then candidate.freshRight else candidate.freshLeft
  protectedConsequence := fun candidate state =>
    if state then candidate.protectedRight else candidate.protectedLeft
  independentlyDemonstratesBenefit := fun _ candidate => candidate.laterBenefit = true

def overcomplete : Candidate := ⟨false, false, false, true, false, true, true⟩
def contracted : Candidate := overcomplete
def lacksOldAgreement : Candidate := { contracted with oldRight := true }
def lacksFreshSeparation : Candidate := { contracted with freshRight := false }
def lacksProtectedSeparation : Candidate := { contracted with protectedRight := false }
def lacksLaterUse : Candidate := { contracted with laterBenefit := false }

theorem pathPromotionAdmitsFold :
    PathFoldAdmissible MethodPromotion.Countermodel.context
      MethodPromotion.Countermodel.contracted () := by
  exact ⟨⟨MethodPromotion.Countermodel.contractedRetainsBoundary⟩⟩

theorem pathRecurrenceAloneRejected :
    ¬ PathFoldAdmissible MethodPromotion.Countermodel.context
      MethodPromotion.Countermodel.lacksAlignment () := by
  rintro ⟨admission⟩
  exact MethodPromotion.Countermodel.alignmentRequired admission.promotion

theorem overcompleteGenerated : ProbeGenerated context overcomplete := by
  exact ⟨⟨false, true, by intro probe _; cases probe; rfl, by decide, by decide⟩⟩

theorem contractedAdmitted : ProbeAdmissible context contracted := by
  exact ⟨⟨⟨false, true, by intro probe _; cases probe; rfl, by decide, by decide⟩,
    (), rfl⟩⟩

theorem oldAgreementRequired : ¬ ProbeGenerated context lacksOldAgreement := by
  rintro ⟨⟨left, right, old, fresh, consequence⟩⟩
  cases left
  · cases right
    · exact fresh rfl
    · exact Bool.noConfusion (old () trivial)
  · cases right
    · exact Bool.noConfusion (old () trivial)
    · exact fresh rfl

theorem freshSeparationRequired : ¬ ProbeGenerated context lacksFreshSeparation := by
  rintro ⟨⟨left, right, old, fresh, consequence⟩⟩
  cases left <;> cases right <;> exact fresh rfl

theorem protectedSeparationRequired : ¬ ProbeGenerated context lacksProtectedSeparation := by
  rintro ⟨⟨left, right, old, fresh, consequence⟩⟩
  cases left <;> cases right <;> exact consequence rfl

theorem generationRemainsInert : ProbeGenerated context lacksLaterUse := by
  exact ⟨⟨false, true, by intro probe _; cases probe; rfl, by decide, by decide⟩⟩

theorem independentLaterUseRequired : ¬ ProbeAdmissible context lacksLaterUse := by
  rintro ⟨admission⟩
  cases admission.independentBenefit

end Countermodel
end InquiryCalculus.Legacy.V20.TraversalLearning

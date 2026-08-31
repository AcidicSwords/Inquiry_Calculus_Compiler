import InquiryCalculus.Legacy.V20.MemoryRecovery

/-! # Historical fact versus generated reconstruction

Source-bound realization of v2.0 lines 4462–4477. Historical references and
generated reconstruction candidates have different types and roles. Working
continuity cannot rewrite ancestry, and protected-distinct candidates remain plural.
-/
namespace InquiryCalculus.Legacy.V20.HistoricalReconstruction

universe u v w x

inductive ReconstructionMaterial (HistoricalRef : Type u) (Candidate : Type v) where
  | historical : HistoricalRef → ReconstructionMaterial HistoricalRef Candidate
  | generated : Candidate → ReconstructionMaterial HistoricalRef Candidate

structure ReconstructionContext (HistoricalRef : Type u) (Candidate : Type v)
    (WorkingContext : Type w) (Obligation : Type x) where
  generatedCandidate : Candidate → Prop
  fillsWorkingContext : WorkingContext → Candidate → Prop
  protectedEquivalent : Candidate → Candidate → Prop
  raisesInquiry : List Candidate → Obligation → Prop

/-- A plural reconstruction preserves its historical-reference vector, retains every
generated working candidate, and carries a positive protected distinction into an
inquiry obligation. -/
structure PluralReconstruction {HistoricalRef : Type u} {Candidate : Type v}
    {WorkingContext : Type w} {Obligation : Type x}
    (context : ReconstructionContext HistoricalRef Candidate WorkingContext Obligation)
    (historicalRefs : List HistoricalRef) (working : WorkingContext) where
  historicalAfter : List HistoricalRef
  ancestryPreserved : historicalAfter = historicalRefs
  candidates : List Candidate
  everyCandidateGenerated : ∀ candidate, candidate ∈ candidates → context.generatedCandidate candidate
  everyCandidateFills : ∀ candidate, candidate ∈ candidates → context.fillsWorkingContext working candidate
  left : Candidate
  right : Candidate
  leftPresent : left ∈ candidates
  rightPresent : right ∈ candidates
  protectedDistinct : ¬ context.protectedEquivalent left right
  obligation : Obligation
  inquiryRaised : context.raisesInquiry candidates obligation

namespace Countermodel

def context : ReconstructionContext Bool Bool Unit Unit where
  generatedCandidate := fun _ => True
  fillsWorkingContext := fun _ _ => True
  protectedEquivalent := Eq
  raisesInquiry := fun candidates _ => candidates = [false, true]

def plural : PluralReconstruction context [false] () where
  historicalAfter := [false]
  ancestryPreserved := rfl
  candidates := [false, true]
  everyCandidateGenerated := by intro candidate membership; exact trivial
  everyCandidateFills := by intro candidate membership; exact trivial
  left := false
  right := true
  leftPresent := List.Mem.head [true]
  rightPresent := List.Mem.tail false (List.Mem.head [])
  protectedDistinct := by intro equal; cases equal
  obligation := ()
  inquiryRaised := rfl

theorem generatedTagIsNotHistorical (candidate : Bool) (reference : Bool) :
    ReconstructionMaterial.generated candidate ≠ ReconstructionMaterial.historical reference := by
  intro equal
  cases equal

theorem ancestryCannotBeRewritten (reconstruction : PluralReconstruction context [false] ()) :
    reconstruction.historicalAfter = [false] :=
  reconstruction.ancestryPreserved

theorem protectedCandidatesRemainPlural :
    plural.left ∈ plural.candidates ∧ plural.right ∈ plural.candidates ∧
      ¬ context.protectedEquivalent plural.left plural.right :=
  ⟨plural.leftPresent, plural.rightPresent, plural.protectedDistinct⟩

theorem inquiryCarriesThePluralField : context.raisesInquiry plural.candidates plural.obligation :=
  plural.inquiryRaised

end Countermodel
end InquiryCalculus.Legacy.V20.HistoricalReconstruction

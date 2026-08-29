/-! # v2.0 exact determination through a signature -/
namespace InquiryCalculus.Legacy.V20

universe u v w

/-- The actual image of an available signature, avoiding a default outside that image. -/
def SignatureImage {X : Type u} {S : Type v} (signature : X → S) := { value : S // ∃ point, signature point = value }

/-- Kernel inclusion for two exact signatures over the same carrier. -/
def SignatureKernelIncluded {X : Type u} {S : Type v} {C : Type w}
    (signature : X → S) (target : X → C) : Prop :=
  ∀ ⦃x y⦄, signature x = signature y → target x = target y

/-- Exact target factorization through the actual image of the available signature. -/
structure ExactSignatureFactorization {X : Type u} {S : Type v} {C : Type w}
    (signature : X → S) (target : X → C) where
  factor : SignatureImage signature → C
  commutes : ∀ point, target point = factor ⟨signature point, ⟨point, rfl⟩⟩

/-- Exact image-subtype factorization implies the source kernel inclusion. -/
theorem exactFactorization_implies_kernelIncluded {X : Type u} {S : Type v} {C : Type w}
    {signature : X → S} {target : X → C} (factorization : ExactSignatureFactorization signature target) :
    SignatureKernelIncluded signature target := by
  intro x y same
  rw [factorization.commutes x, factorization.commutes y]
  congr

/-- Kernel inclusion constructs an exact factorization on the signature's actual image. -/
noncomputable def kernelIncluded_exactFactorization {X : Type u} {S : Type v} {C : Type w}
    {signature : X → S} {target : X → C} (included : SignatureKernelIncluded signature target) :
    ExactSignatureFactorization signature target := by
  let representative : SignatureImage signature → X := fun value => Classical.choose value.property
  refine ⟨fun value => target (representative value), ?_⟩
  intro point
  apply included
  dsimp [representative]
  let value : SignatureImage signature := ⟨signature point, ⟨point, rfl⟩⟩
  have chosen : signature (Classical.choose value.property) = value.val := Classical.choose_spec value.property
  simpa [value] using chosen.symm

/-- Exact determination through a signature is precisely kernel inclusion. -/
theorem exactDetermination_iff_kernelIncluded {X : Type u} {S : Type v} {C : Type w}
    {signature : X → S} {target : X → C} :
    Nonempty (ExactSignatureFactorization signature target) ↔ SignatureKernelIncluded signature target := by
  constructor
  · intro factorization
    rcases factorization with ⟨witness⟩
    exact exactFactorization_implies_kernelIncluded witness
  · intro included
    exact ⟨kernelIncluded_exactFactorization included⟩

/-- Available exact signatures only; nonexact modes remain explicit obligations. -/
inductive ExactDeterminationSignatureObligation where
  | availableSignature
  | deterministicSignature
  | partialSignature
  | nondeterministicSignature
  | incompleteCoverage
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

import InquiryCalculus.Legacy.V20.QuestionRefinementPreorder

/-! # v2.0 question-refinement semantic boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w

/-- A functional question profile on one locally fixed carrier type. -/
structure FunctionalQuestionProfileSyntax (X : Type u) (A : Type v) where
  observe : X → A

/-- The local profile-kernel inclusion expressed by the v2.0 precision display. -/
def ProfileKernelIncluded {X : Type u} {A : Type v} {R : Type w}
    (coarser : X → A) (finer : X → R) : Prop :=
  ∀ ⦃x y : X⦄, finer x = finer y → coarser x = coarser y

/-- A candidate factorization of a coarser profile through a finer profile. -/
structure ProfileFactorizationSyntax {X : Type u} {A : Type v} {R : Type w}
    (coarser : X → A) (finer : X → R) where
  factor : R → A
  commutes : ∀ x, coarser x = factor (finer x)

/-- Functional factorization is sufficient for the local profile-kernel inclusion. -/
theorem factorization_implies_profileKernelIncluded {X : Type u} {A : Type v} {R : Type w}
    {coarser : X → A} {finer : X → R}
    (factorization : ProfileFactorizationSyntax coarser finer) :
    ProfileKernelIncluded coarser finer := by
  intro x y sameFiner
  rw [factorization.commutes x, factorization.commutes y, sameFiner]

/-- The reverse construction requires coverage of the finer profile's advertised answer type. -/
def ProfileCoverage {X : Type u} {R : Type w} (finer : X → R) : Prop :=
  Function.Surjective finer

/-- Under explicit profile coverage, local kernel inclusion yields a factorization witness. -/
noncomputable def profileKernelIncluded_factorization_of_coverage
    {X : Type u} {A : Type v} {R : Type w} {coarser : X → A} {finer : X → R}
    (included : ProfileKernelIncluded coarser finer) (coverage : ProfileCoverage finer) :
    ProfileFactorizationSyntax coarser finer := by
  let representative : R → X := fun answer => Classical.choose (coverage answer)
  have representative_ok : ∀ answer, finer (representative answer) = answer :=
    fun answer => Classical.choose_spec (coverage answer)
  refine ⟨fun answer => coarser (representative answer), ?_⟩
  intro point
  exact included (representative_ok (finer point)).symm

/-- Unconditioned reverse equivalence, joint-kernel laws, and representation semantics stay open. -/
inductive QuestionRefinementSemanticObligation where
  | reverseNeedsProfileCoverage
  | jointKernelIntersection
  | nonredundantCoordinate
  | activeRepresentationKernel
  | noGlobalSemanticPreorder
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

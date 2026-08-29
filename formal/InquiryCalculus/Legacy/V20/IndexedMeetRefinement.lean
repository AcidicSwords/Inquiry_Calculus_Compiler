import InquiryCalculus.Legacy.V20.SolutionFieldWeb

/-! # v2.0 indexed-meet-refinement boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w

/-- An extension retains every residual fiber of the base web at a typed index in the larger web. -/
structure WebExtension {I : Type u} {J : Type v} {X : Type w}
    (base : SolutionFieldWebSyntax I X) (extended : SolutionFieldWebSyntax J X) where
  embed : I → J
  preservesFiber : ∀ i, extended.residualFibers (embed i) = base.residualFibers i

/-- Predicate inclusion, used only for the two solution fields in this theorem. -/
def FieldSubset {X : Type u} (left right : X → Prop) : Prop :=
  ∀ x, left x → right x

/-- Adding residual fibers refines the solution field in the source direction. -/
theorem indexedMeet_refinement {I : Type u} {J : Type v} {X : Type w}
    (base : SolutionFieldWebSyntax I X) (extended : SolutionFieldWebSyntax J X)
    (extension : WebExtension base extended) :
    FieldSubset (solutionFieldOfWeb extended) (solutionFieldOfWeb base) := by
  intro x inExtended i
  have inIncludedFiber := inExtended (extension.embed i)
  rw [extension.preservesFiber i] at inIncludedFiber
  exact inIncludedFiber

/-- Converse, equality, answer, solver, execution, and successor interpretations remain open. -/
inductive IndexedMeetRefinementObligation where
  | reverseInclusion
  | equalityWithoutReverseExtension
  | residualRelationExtensionality
  | answerSemantics
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

import InquiryCalculus.Legacy.V20.SolutionFibers

/-! # v2.0 solution-field-of-a-web boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v

/-- A residual relation is represented only by its solution fiber at one already-bound context. -/
structure ResidualFiberSyntax (X : Type u) where
  members : X → Prop

/-- An indexed web of residual solution fibers constraining one open port. -/
structure SolutionFieldWebSyntax (I : Type u) (X : Type v) where
  residualFibers : I → ResidualFiberSyntax X

/-- The solution field is the indexed intersection of all residual solution fibers. -/
def solutionFieldOfWeb {I : Type u} {X : Type v}
    (web : SolutionFieldWebSyntax I X) : X → Prop :=
  fun x => ∀ i, (web.residualFibers i).members x

/-- Membership is exactly incidence in every indexed residual fiber. -/
theorem solutionFieldOfWeb_iff {I : Type u} {X : Type v}
    (web : SolutionFieldWebSyntax I X) (x : X) :
    solutionFieldOfWeb web x ↔ ∀ i, (web.residualFibers i).members x := Iff.rfl

/-- Refinement, answer, solver, execution, and successor interpretations remain open. -/
inductive SolutionFieldWebObligation where
  | boundOtherPorts
  | residualRelationMeaning
  | indexedMeetRefinement
  | answerSemantics
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

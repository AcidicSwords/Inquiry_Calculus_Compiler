/-! # v2.0 grain-question boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w

/-- A declared protected-exposure predicate indexed by candidate grain. -/
def GrainExposesProtectedDifference {G : Type u} := G → Prop

/-- A grain question asks whether its indexed representation distinguishes the pair and exposes a protected difference. -/
def separatingGrainQuestion {G : Type u} {X : Type v} {S : Type w}
    (representation : G → X → S) (exposes : GrainExposesProtectedDifference (G := G))
    (left right : X) (grain : G) : Prop :=
  representation grain left ≠ representation grain right ∧ exposes grain

/-- Successful separation, ordering, coarsestness, and execution remain open. -/
inductive GrainQuestionObligation where
  | protectedExposureMeaning
  | separatingGrainSuccess
  | grainOrdering
  | coarsestRepresentation
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

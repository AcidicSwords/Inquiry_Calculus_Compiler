import InquiryCalculus.Legacy.V20.ProtectedDetermination

/-! # v2.0 residual-ambiguity boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- Residual ambiguity has two surviving fillers in distinct declared protected classes. -/
structure ResidualAmbiguityWitness {X : Type u} (horizon : HorizonEquivalenceSyntax X)
    (field : X → Prop) where
  left : X
  right : X
  leftInField : field left
  rightInField : field right
  separated : ¬ horizon.equivalent left right

/-- Separator validity, answer selection, and execution remain open. -/
inductive ResidualAmbiguityObligation where
  | separatorQuestionValidity
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

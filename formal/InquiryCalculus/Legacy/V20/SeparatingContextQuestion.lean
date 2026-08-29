/-! # v2.0 separating-context-question boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w

/-- A family of candidate contexts over a fixed input and observed-context type. -/
def ContextFamily {X : Type u} {C : Type v} := (X → C) → Prop

/-- The question predicate asks whether one admitted context distinguishes a fixed pair. -/
def separatesByContext {X : Type u} {C : Type v} {K : Type w}
    (family : ContextFamily (X := X) (C := C)) (consequence : C → K)
    (left right : X) (context : X → C) : Prop :=
  family context ∧ consequence (context left) ≠ consequence (context right)

/-- Extension, availability, success, and execution remain open. -/
inductive SeparatingContextQuestionObligation where
  | higherOrderContextExtension
  | contextAvailability
  | contextSuccess
  | exhaustiveFailure
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

import InquiryCalculus.Legacy.V20.ProtectedDetermination

/-! # v2.0 probe/tool-invention-question boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v

/-- A current probe basis over one common observed output type. -/
def ProbeBasis {X : Type u} {O : Type v} := (X → O) → Prop

/-- A candidate probe distinguishes the pair that every current basis probe leaves equal. -/
def probeToolInventionQuestion {X : Type u} {O : Type v}
    (basis : ProbeBasis (X := X) (O := O)) (horizon : HorizonEquivalenceSyntax X)
    (left right : X) (candidate : X → O) : Prop :=
  (∀ probe, basis probe → probe left = probe right) ∧
    candidate left ≠ candidate right ∧ ¬ horizon.equivalent left right

/-- Implementation, actual return, checking, and warrant remain open. -/
inductive ProbeToolInventionQuestionObligation where
  | toolImplementation
  | actualUseReturn
  | typedDiscrimination
  | independentChecking
  | semanticAuthority
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

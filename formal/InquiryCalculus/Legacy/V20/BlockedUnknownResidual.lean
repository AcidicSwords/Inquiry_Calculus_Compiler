import InquiryCalculus.Legacy.V20.SemanticExecutableFrontier

/-! # v2.0 blocked, Unknown, and resource residual boundary -/
namespace InquiryCalculus.Legacy.V20.BlockedUnknownResidual

universe u

inductive ResidualClass where | blocked | unknown | resource deriving DecidableEq

inductive FailedSearchKind where | separator | path | counterexample | proof | usefulQuestion deriving DecidableEq

inductive NegativeConclusion where | equivalence | impossibility | necessity | irrelevance deriving DecidableEq

structure ResidualContext (Occurrence : Type u) where
  required : Occurrence → Prop
  executable : Occurrence → Prop
  undischarged : Occurrence → Prop
  residualClass : Occurrence → ResidualClass
  resolvedClaim : Occurrence → Prop

def RequiredNonexecutableResidual {Occurrence : Type u}
    (context : ResidualContext Occurrence) (occurrence : Occurrence) : Prop :=
  context.required occurrence ∧ ¬ context.executable occurrence ∧ context.undischarged occurrence ∧
    (context.residualClass occurrence = .blocked ∨ context.residualClass occurrence = .unknown ∨
      context.residualClass occurrence = .resource)

def UnknownLegal (_failure : FailedSearchKind) (established : NegativeConclusion → Prop) : Prop :=
  ∀ conclusion, ¬ established conclusion

theorem resolvedDoesNotSuppressResidual {Occurrence : Type u}
    (context : ResidualContext Occurrence) (occurrence : Occurrence) :
    RequiredNonexecutableResidual context occurrence → context.resolvedClaim occurrence →
      RequiredNonexecutableResidual context occurrence := fun residual _ => residual

namespace Countermodel

inductive Occurrence where | requiredBlocked | idleExecutable deriving DecidableEq

def context : ResidualContext Occurrence where
  required := fun occurrence => occurrence = .requiredBlocked
  executable := fun occurrence => occurrence = .idleExecutable
  undischarged := fun occurrence => occurrence = .requiredBlocked
  residualClass := fun occurrence => if occurrence = .requiredBlocked then .blocked else .unknown
  resolvedClaim := fun _ => True

def noNegativeConclusion : NegativeConclusion → Prop := fun _ => False

theorem requiredBlockedIsResidual : RequiredNonexecutableResidual context .requiredBlocked := by
  refine ⟨rfl, ?_, rfl, Or.inl ?_⟩
  · intro executable
    cases executable
  · simp [context]

theorem requiredBlockedIsNotIdleExecutable : ¬ context.executable .requiredBlocked := by
  intro executable
  cases executable

theorem resolvedClaimDoesNotEraseRequiredBlocked : RequiredNonexecutableResidual context .requiredBlocked :=
  resolvedDoesNotSuppressResidual context .requiredBlocked requiredBlockedIsResidual True.intro

theorem separatorFailureIsUnknownLegal : UnknownLegal .separator noNegativeConclusion := by
  intro conclusion established
  exact established.elim

theorem pathFailureIsUnknownLegal : UnknownLegal .path noNegativeConclusion := by
  intro conclusion established
  exact established.elim

theorem counterexampleFailureIsUnknownLegal : UnknownLegal .counterexample noNegativeConclusion := by
  intro conclusion established
  exact established.elim

theorem proofFailureIsUnknownLegal : UnknownLegal .proof noNegativeConclusion := by
  intro conclusion established
  exact established.elim

theorem usefulQuestionFailureIsUnknownLegal : UnknownLegal .usefulQuestion noNegativeConclusion := by
  intro conclusion established
  exact established.elim

end Countermodel
end InquiryCalculus.Legacy.V20.BlockedUnknownResidual

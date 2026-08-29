import InquiryCalculus.Legacy.V20.RepresentationSearch

/-! # v2.0 program-core-grammar boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w

/-- First-order core syntax: a nonempty branch is represented by its head and finite tail. -/
inductive CoreGrammarSyntax (Value : Type u) (ProbeOperator : Type v) (RawContinuation : Type w) where
  | pureReturn : Value → CoreGrammarSyntax Value ProbeOperator RawContinuation
  | branch : CoreGrammarSyntax Value ProbeOperator RawContinuation →
      List (CoreGrammarSyntax Value ProbeOperator RawContinuation) →
      CoreGrammarSyntax Value ProbeOperator RawContinuation
  | rawProbe : ProbeOperator → RawContinuation → CoreGrammarSyntax Value ProbeOperator RawContinuation

/-- Source obligations retained until core typing, event handling, and interpretation are separately formalized. -/
inductive CoreGrammarObligation where
  | returnHasNoExternalActualityEvent
  | branchAlternativesAreNotActual
  | rawProbeActualizableInteraction
  | rawReturnOnlyAfterOrdinaryEvent
  | rawHandlerNotSourceAnswerContinuation
  | rawContinuationIsDataNotClosure
  | branchIsNonemptyAndFinite
  | returnBranchAndProbeTypingRules
  | deterministicContinuationSingletonSpecialization
  | noExecutionOrSemanticEvaluation
  | noAnswerSelectionOrTotalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

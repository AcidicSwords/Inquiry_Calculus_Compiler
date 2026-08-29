import InquiryCalculus.Legacy.V20.SourceAskCompilationSafetyClaim

/-! # v2.0 sequencing-and-composition boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The three displayed bind equation shapes remain distinct predecessor obligations. -/
inductive BindEquationShape where
  | returnSubstitution
  | branchPointwise
  | probeRawReturnPointwise
  deriving DecidableEq, Repr

/-- The displayed protected-equivalence identity shapes remain distinct from equality. -/
inductive ProtectedEquivalenceIdentityShape where
  | leftReturn
  | rightReturn
  | associative
  deriving DecidableEq, Repr

/-- Sequencing syntax retains bind operands and equation shapes without an evaluator. -/
structure SequencingCompositionSyntax (Term : Type u) (Continuation : Type u)
    (RawContinuation : Type u) (ProtectedEquivalence : Type u) where
  sourceTerm : Term
  continuation : Continuation
  rawContinuation : RawContinuation
  protectedEquivalence : ProtectedEquivalence
  bindSyntaxDisplayed : Prop
  returnEquation : BindEquationShape
  branchEquation : BindEquationShape
  probeEquation : BindEquationShape
  protectedReturnIdentity : ProtectedEquivalenceIdentityShape
  protectedRightIdentity : ProtectedEquivalenceIdentityShape
  protectedAssociativeIdentity : ProtectedEquivalenceIdentityShape
  operationDefinedFragment : Prop
  equationsRemainUnproved : Prop
  protectedEquivalenceNotEquality : Prop
  noTotalEvaluator : Prop

/-- Source obligations retained until bind interpretation and equivalence laws are separately formalized. -/
inductive SequencingCompositionObligation where
  | bindSyntaxAndContinuationIntent
  | returnEquationShape
  | branchEquationShape
  | probeEquationShape
  | rawContinuationPointwiseBinding
  | operationDefinedFragment
  | protectedEquivalenceRatherThanEquality
  | leftReturnIdentityUnproved
  | rightReturnIdentityUnproved
  | associativeIdentityUnproved
  | noTotalEvaluator
  | noAutomaticMonadLawProof
  | noExecutionOrSemanticEvaluation
  | noAnswerSelectionOrTotalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

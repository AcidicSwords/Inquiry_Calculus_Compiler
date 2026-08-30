import InquiryCalculus.Legacy.V20.LLMPossibility

/-! # v2.0 prompt-operator boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The predecessor records prompt inputs, output, and occurrence membership without rendering or invoking a backend. -/
structure PromptOperatorSyntax (Question : Type u) (BoundaryChart : Type u) (ActiveView : Type u)
    (ModelParameters : Type u) (Policy : Type u) (Prompt : Type u) (Budget : Type u)
    (RawCompletion : Type u) (Occurrence : Type u) (Contract : Type u) where
  semanticQuestion : Question
  boundaryChart : BoundaryChart
  activeView : ActiveView
  modelParameters : ModelParameters
  policy : Policy
  prompt : Prompt
  budget : Budget
  rawCompletion : RawCompletion
  runtimeOccurrence : Occurrence
  typedOperatorContract : Contract
  renderingInputOutputShape : Prop
  promptIsProbeOperatorRepresentationUnproved : Prop
  runtimeOccurrenceMembershipShape : Prop
  semanticQuestionIsNotPromptUnproved : Prop
  wordingMayChangeWithSameTypedContractUnproved : Prop
  noPromptRenderingExecution : Prop
  noBackendInvocation : Prop
  noRawCompletionInterpretation : Prop
  noSemanticQuestionPromptIdentity : Prop
  noWordingContractChange : Prop

/-- Source obligations retained until rendering, backend behavior, and contract equivalence are separately checked. -/
inductive PromptOperatorObligation where
  | renderingInputOutputShape
  | promptProbeOperatorRepresentationUnproved
  | runtimeOccurrenceMembershipShape
  | semanticQuestionNotPromptUnproved
  | wordingVariationSameTypedContractUnproved
  | noPromptRenderingExecution
  | noBackendInvocation
  | noRawCompletionInterpretation
  | noSemanticQuestionPromptIdentity
  | noWordingContractChange
  | noSemanticAuthorityPromotion
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

import InquiryCalculus.Legacy.V20.CanonicalRenderingElaboration

/-! # v2.0 behavioral compiler-correctness boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The predecessor retains protected behavioral correctness coordinates without evaluating or proving them. -/
structure BehavioralCompilerCorrectnessSyntax (ProtectedContinuation : Type u) (Prompt : Type u)
    (Operator : Type u) (Compiler : Type u) (Behavior : Type u) (Denotation : Type u)
    (Approximation : Type u) (DiscriminatorSet : Type u) (ErrorRelation : Type u)
    (ReopeningCondition : Type u) where
  protectedContinuationFamily : ProtectedContinuation
  firstPrompt : Prompt
  secondPrompt : Prompt
  operator : Operator
  compiler : Compiler
  compiledBehavior : Behavior
  denotation : Denotation
  approximation : Approximation
  discriminatorSet : DiscriminatorSet
  errorRelation : ErrorRelation
  reopeningCondition : ReopeningCondition
  literalTextEqualityIsNotCriterionUnproved : Prop
  protectedLLMFacingOperationalIndistinguishabilityShape : Prop
  exactCompiledBehaviorDenotationEquivalenceUnproved : Prop
  licensedApproximationShapeUnproved : Prop
  approximationNamesDiscriminatorErrorAndReopeningUnproved : Prop
  noBehaviorEvaluator : Prop
  noCompilerCorrectnessProof : Prop
  noLiteralTextEqualityCriterion : Prop
  noApproximationLicenseDecision : Prop
  noDiscriminatorImplementation : Prop
  noErrorDecision : Prop
  noReopeningEvaluator : Prop
  noSemanticAuthorityPromotion : Prop

/-- Source obligations retained until behavioral equivalence and approximation are independently checked. -/
inductive BehavioralCompilerCorrectnessObligation where
  | literalTextEqualityIsNotCriterionUnproved
  | protectedLLMFacingOperationalIndistinguishabilityShape
  | exactCompiledBehaviorDenotationEquivalenceUnproved
  | licensedApproximationShapeUnproved
  | approximationNamesDiscriminatorErrorAndReopeningUnproved
  | noBehaviorEvaluator
  | noCompilerCorrectnessProof
  | noLiteralTextEqualityCriterion
  | noApproximationLicenseDecision
  | noDiscriminatorImplementation
  | noErrorDecision
  | noReopeningEvaluator
  | noSemanticAuthorityPromotion
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

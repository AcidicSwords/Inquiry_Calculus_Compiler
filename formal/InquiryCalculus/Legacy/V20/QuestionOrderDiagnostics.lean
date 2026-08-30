import InquiryCalculus.Legacy.V20.FreshProbeComparison

/-! # v2.0 question-order diagnostic boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

structure QuestionOrderDiagnosticsSyntax (Question : Type u) (State : Type u) (UpdateRelation : Type u)
    (ProtectedHorizon : Type u) (Composition : Type u) (NonEquivalence : Type u)
    (RelationalInteraction : Type u) (PathDependence : Type u) (RuntimeOrderEffect : Type u)
    (LearningPattern : Type u) where
  firstQuestion : Question
  secondQuestion : Question
  state : State
  inducedUpdateRelation : UpdateRelation
  protectedHorizon : ProtectedHorizon
  firstThenSecond : Composition
  secondThenFirst : Composition
  protectedNonEquivalence : NonEquivalence
  relationalInteraction : RelationalInteraction
  pathDependence : PathDependence
  irrelevantRuntimeOrderEffect : RuntimeOrderEffect
  questionPatternLearning : LearningPattern
  inducedUpdateRelationShape : Prop
  opposedQuestionCompositionShape : Prop
  protectedHorizonNonEquivalenceShape : Prop
  relationalInteractionOrPathDependenceQuestionShape : Prop
  irrelevantRuntimeOrderEffectQuestionShape : Prop
  protectedOrderDependenceNotSilentlyQuotientedUnproved : Prop
  noQuestionExecution : Prop
  noQuestionResolution : Prop
  noUpdateConstruction : Prop
  noIndependenceDecision : Prop
  noEquivalenceTest : Prop
  noRuntimeAttribution : Prop
  noOrderQuotient : Prop
  noSemanticAuthorityPromotion : Prop

inductive QuestionOrderDiagnosticsObligation where
  | inducedUpdateRelationShape
  | opposedQuestionCompositionShape
  | protectedHorizonNonEquivalenceShape
  | relationalInteractionOrPathDependenceQuestionShape
  | irrelevantRuntimeOrderEffectQuestionShape
  | protectedOrderDependenceNotSilentlyQuotientedUnproved
  | noQuestionExecution
  | noQuestionResolution
  | noUpdateConstruction
  | noIndependenceDecision
  | noEquivalenceTest
  | noRuntimeAttribution
  | noOrderQuotient
  | noSemanticAuthorityPromotion
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

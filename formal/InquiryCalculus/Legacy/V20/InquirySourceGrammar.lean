import InquiryCalculus.Legacy.V20.ResolutionOutcome

/-! # v2.0 inquiry-source-grammar boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x y z

/-- An inspectable continuation package binds a typed answer slot in an explicit environment. -/
structure CheckedContinuationPackage (Environment : Type u) (Question : Type v)
    (SupportedAnswer : Question → Type w) (AnswerSlot : Type x) (Expression : Type y)
    (SubprogramReference : Type z) (question : Question) where
  environment : Environment
  answerSlot : AnswerSlot
  answerSlotSort : Type w
  answerSlotHasSupportedAnswerType : answerSlotSort = SupportedAnswer question
  body : Expression
  namedSubprogramReferences : SubprogramReference → Prop
  captureSafeSubstitution : Prop
  mayConstructNewQuestionFromReturnedDistinction : Prop

/-- The first-order source grammar has returns and asks, never host-language callbacks. -/
inductive InquirySourceGrammar (Environment : Type u) (Question : Type v)
    (SupportedAnswer : Question → Type w) (AnswerSlot : Type x) (Expression : Type y)
    (SubprogramReference : Type z) where
  | returnI : Expression → InquirySourceGrammar Environment Question SupportedAnswer AnswerSlot Expression SubprogramReference
  | ask : (question : Question) →
      CheckedContinuationPackage Environment Question SupportedAnswer AnswerSlot Expression SubprogramReference question →
      InquirySourceGrammar Environment Question SupportedAnswer AnswerSlot Expression SubprogramReference

/-- Source obligations retained until expression typing, substitution, and interpretation are separately formalized. -/
inductive InquirySourceGrammarObligation where
  | returnAndAskFirstOrderSyntax
  | answerSlotRatherThanHostLanguageCallback
  | explicitEnvironment
  | typedSupportedAnswerBinding
  | inspectableSerializableProgramData
  | namedSubprogramReferences
  | captureSafeSubstitution
  | noOpaqueHostLanguageClosure
  | noHiddenModelPolicy
  | mayConstructNewRelationOrQuestion
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

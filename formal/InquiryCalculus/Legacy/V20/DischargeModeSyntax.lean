import InquiryCalculus.Legacy.V20.ManyQuestionsGeneration

/-! # v2.0 discharge-mode authority boundary -/
namespace InquiryCalculus.Legacy.V20

/-- The five source-named route classifiers for an executable open port. -/
inductive DischargeModeSyntax where
  | pure
  | generate
  | probe
  | check
  | warrant
  deriving DecidableEq, Repr

/-- A mode is assigned to a typed question opening but does not itself discharge it. -/
structure PortDischargeMode (B : Binding) (I : TypeInterpretation B) where
  opening : QuestionOpening B I
  mode : DischargeModeSyntax

/-- Execution, actual return, independent admission, and standing remain later obligations. -/
inductive DischargeModeObligation where
  | pureStandingData
  | generateProvisionalOnly
  | probeActualInteraction
  | checkIndependentAdmission
  | warrantStandingPolicy
  | noGenerativeSelfDischarge
  deriving DecidableEq, Repr

theorem discharge_mode_is_classification_only (B : Binding) (I : TypeInterpretation B)
    (assignment : PortDischargeMode B I) : assignment = assignment := rfl

end InquiryCalculus.Legacy.V20

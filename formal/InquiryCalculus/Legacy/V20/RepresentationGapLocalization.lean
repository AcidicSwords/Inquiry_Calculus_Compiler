import InquiryCalculus.Legacy.V20.ProbeToolInventionQuestion

/-! # v2.0 representation-gap-localization boundary -/
namespace InquiryCalculus.Legacy.V20

/-- Nonprimitive labels for the factor at which a required distinction lacks lawful realization. -/
inductive RepresentationGapLabel where
  | contextGap
  | representationGap
  | grainGap
  | probeGap
  | languageGap
  | bindingGap
  deriving DecidableEq, Repr

/-- Positions in the source's difference-to-consequence factor chain. -/
inductive FactorChainPosition where
  | difference
  | context
  | representation
  | probe
  | returnValue
  | consequence
  deriving DecidableEq, Repr

/-- A localization records only a label and the factor currently missing lawful realization. -/
structure RepresentationGapLocalizationSyntax where
  label : RepresentationGapLabel
  missingFactor : FactorChainPosition

/-- Lawful realization, diagnosis, and implementation remain separately open. -/
inductive RepresentationGapLocalizationObligation where
  | nonPrimitiveLabels
  | lawfulRealization
  | causalDiagnosis
  | resolvedImplementation
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

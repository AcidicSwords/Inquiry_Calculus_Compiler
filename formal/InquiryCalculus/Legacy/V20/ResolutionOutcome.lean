import InquiryCalculus.Legacy.V20.ResolutionCarriers

/-! # v2.0 resolution-outcome boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x y z

/-- The source's five question-indexed resolution alternatives. -/
inductive ResolutionOutcome (Question : Type u) (SupportedAnswer : Type v) (Coverage : Type w)
    (Candidate : Type x) (SupportObligation : Type y) (Completion : Type z) (Component : Type (max u v w x y z)) where
  | supported : SupportedAnswer → ResolutionOutcome Question SupportedAnswer Coverage Candidate SupportObligation Completion Component
  | exactEmpty : EmptyCertificate Question Coverage → ResolutionOutcome Question SupportedAnswer Coverage Candidate SupportObligation Completion Component
  | undefined : Question → ResolutionPathIssue → ResolutionOutcome Question SupportedAnswer Coverage Candidate SupportObligation Completion Component
  | unsupported : SupportResidual Question Candidate SupportObligation → ResolutionOutcome Question SupportedAnswer Coverage Candidate SupportObligation Completion Component
  | unknown : CoverageResidual Question Completion Component → ResolutionOutcome Question SupportedAnswer Coverage Candidate SupportObligation Completion Component

/-- Only a supported outcome can fill a source answer slot. -/
def mayFillSourceAnswerSlot {Question : Type u} {SupportedAnswer : Type v} {Coverage : Type w}
    {Candidate : Type x} {SupportObligation : Type y} {Completion : Type z} {Component : Type (max u v w x y z)} :
    ResolutionOutcome Question SupportedAnswer Coverage Candidate SupportObligation Completion Component → Prop
  | .supported _ => True
  | _ => False

/-- Provider and resource conditions are operational stop states, not resolution outcomes. -/
inductive ResolutionOperationalStop where
  | blocked
  | resourceBounded
  deriving DecidableEq, Repr

/-- Source obligations retained until supported-answer and operational semantics are separately formalized. -/
inductive ResolutionOutcomeObligation where
  | noFabricatedNonemptySupportedAnswer
  | exactEmptyRequiresExhaustiveAdmittedCoverage
  | undefinedMalformedOrInapplicablePath
  | unsupportedFailsDeclaredSupportRoute
  | unknownIncompleteCoverage
  | blockedDistinctOperationalStop
  | resourceBoundedDistinctOperationalStop
  | onlySupportedMayFillSourceAnswerSlot
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

import InquiryCalculus.Legacy.V20.RequiredDischarge

/-! # v2.0 resolution-certificate and residual-carrier boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x y z

/-- Provenance retained by every resolution certificate or residual. -/
structure ResolutionContext (Route : Type u) (Evidence : Type v) (Version : Type w) (Scope : Type x)
    (Grain : Type y) (Horizon : Type z) (Provenance : Type (max u v w x y z)) where
  exactRoute : Route
  evidence : Evidence
  versions : Version
  scope : Scope
  grain : Grain
  horizon : Horizon
  provenance : Provenance

/-- An exact-empty result pairs admitted exhaustive coverage with a checked absence derivation. -/
structure EmptyCertificate (Question : Type u) (Coverage : Type v) where
  question : Question
  admittedExhaustiveCoverage : Coverage
  checkedNoSupportedCompletion : Prop

/-- A resolution-path residual records why decoding or resolution cannot yield a defined result. -/
inductive ResolutionPathIssue where
  | malformed
  | inapplicable
  | undefined
  deriving DecidableEq, Repr

/-- A support residual preserves decoded candidates and the support obligations they fail. -/
structure SupportResidual (Question : Type u) (Candidate : Type v) (SupportObligation : Type w) where
  question : Question
  decodedCandidateMembers : Candidate → Prop
  failedDeclaredSupportObligations : SupportObligation → Prop

/-- A coverage residual preserves the region or components that remain unresolved. -/
structure CoverageResidual (Question : Type u) (Completion : Type v) (Component : Type w) where
  question : Question
  uncoveredCompletionRegion : Completion → Prop
  unresolvedComponents : Component → Prop

/-- The four source resolution payloads are disjoint by their constructors. -/
inductive ResolutionCarrier (Question : Type u) (Coverage : Type v) (Candidate : Type w)
    (SupportObligation : Type x) (Completion : Type y) (Component : Type z) where
  | emptyCert : EmptyCertificate Question Coverage → ResolutionCarrier Question Coverage Candidate SupportObligation Completion Component
  | resolutionResidual : Question → ResolutionPathIssue → ResolutionCarrier Question Coverage Candidate SupportObligation Completion Component
  | supportResidual : SupportResidual Question Candidate SupportObligation → ResolutionCarrier Question Coverage Candidate SupportObligation Completion Component
  | coverageResidual : CoverageResidual Question Completion Component → ResolutionCarrier Question Coverage Candidate SupportObligation Completion Component

/-- Source obligations retained until evidence, decoding, and resolution outcomes are separately elaborated. -/
inductive ResolutionCarriersObligation where
  | firstOrderQuestionIndexedCarriers
  | admittedExhaustiveCoverage
  | checkedNoSupportedCompletion
  | eventRawReturnAndVersionProvenance
  | decodedCandidateAndFailedDeclaredSupport
  | noFailureAsSemanticNegation
  | uncoveredCompletionOrUnresolvedComponents
  | disjointByConstructor
  | exactRouteEvidenceVersionsScopeGrainHorizonAndProvenance
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

import InquiryCalculus.Legacy.V20.DeterminationPresentation

/-! # v2.0 positive-departure-witness boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x y z

/--
A non-circular positive certificate made of two represented observations, their supported fillings,
and a standing incompatibility use relevant to the current determination presentation.
-/
structure PositiveDepartureWitnessSyntax
    (Source : Type u) (Candidate : Type v) (SourceObservation : Type w)
    (CandidateObservation : Type x) (SourceFill : Type y) (CandidateFill : Type z)
    (Presentation Authority Scope Applicability Grain : Type) where
  liveSource : Source
  candidate : Candidate
  sourceRepresentation : Source → SourceFill → SourceObservation
  candidateRepresentation : Candidate → CandidateFill → CandidateObservation
  sourceFill : SourceFill
  candidateFill : CandidateFill
  standingIncompatibility : SourceObservation → CandidateObservation → Prop
  incompatibilityUse : standingIncompatibility
      (sourceRepresentation liveSource sourceFill) (candidateRepresentation candidate candidateFill)
  relevantToPresentation : Presentation → Prop
  presentation : Presentation
  presentationRelevance : relevantToPresentation presentation
  declaredAuthority : Authority
  declaredScope : Scope
  declaredApplicability : Applicability
  declaredGrain : Grain
  nonCircularCertificate : Prop

/-- Equality absence and operational failure modes do not substitute for a positive witness. -/
inductive PositiveDepartureWitnessObligation where
  | representedObservations
  | supportedFillings
  | standingIncompatibilityUse
  | presentationRelevance
  | nonCircularCertificate
  | bindingNativeDirectIncompatibility
  | authorityScopeApplicabilityGrain
  | equalityFailure
  | searchFailure
  | retrievalFailure
  | generationFailure
  | proofFailure
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

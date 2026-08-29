import InquiryCalculus.Legacy.V20.CandidateBoundaryIncidence

/-! # v2.0 boundary-point-profile boundary -/
namespace InquiryCalculus.Legacy.V20

/-- Boundary points that project to one left-side candidate. -/
def boundaryPointProfile (B : Binding) (schema : TypedDistinctionSchema B)
    (left : schema.leftCandidate) : schema.boundaryPoint → Prop :=
  fun point => schema.projectLeft point = left

/-- Profile equality is only equality of projection-indexed candidate-incidence observations. -/
def boundaryPointProfileEquivalent (B : Binding) (schema : TypedDistinctionSchema B)
    (first second : schema.leftCandidate) : Prop :=
  boundaryPointProfile B schema first = boundaryPointProfile B schema second

/-- Contextual exteriority and inquiry-occurrence meanings remain separately open. -/
inductive BoundaryPointProfileObligation where
  | projectionIndexedProfile
  | profileEquality
  | candidateIncidenceObservation
  | contextualExteriority
  | departureWitness
  | negationRelation
  | crossing
  | completedOccurrence
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

import InquiryCalculus.Legacy.V20.TypedDistinctionSchema

/-! # v2.0 candidate-boundary-incidence boundary -/
namespace InquiryCalculus.Legacy.V20

/-- Two candidates are incident precisely when one boundary point projects to both. -/
def candidateBoundaryIncidence (B : Binding) (schema : TypedDistinctionSchema B)
    (left : schema.leftCandidate) (right : schema.rightCandidate) : Prop :=
  ∃ point : schema.boundaryPoint,
    schema.projectLeft point = left ∧ schema.projectRight point = right

/-- Departure, negation, crossing, and completed inquiry remain separately open. -/
inductive CandidateBoundaryIncidenceObligation where
  | jointCandidateChart
  | boundaryWitness
  | departureWitness
  | negationRelation
  | crossing
  | completedSixfoldOccurrence
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

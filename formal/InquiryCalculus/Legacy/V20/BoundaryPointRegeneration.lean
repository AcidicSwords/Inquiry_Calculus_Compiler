import InquiryCalculus.Legacy.V20.BoundaryPointProfile

/-! # v2.0 boundary-point-regeneration obligation -/
namespace InquiryCalculus.Legacy.V20

universe u

/--
A reified boundary point at a declared grain regenerates only its candidate schema and projected
sides.  It does not itself reconstruct an inquiry occurrence.
-/
structure BoundaryPointRegenerationSyntax (B : Binding) (Grain : Type u) where
  schema : TypedDistinctionSchema B
  point : schema.boundaryPoint
  declaredGrain : Grain
  regeneratedLeft : schema.leftCandidate
  regeneratedRight : schema.rightCandidate
  leftProjection : regeneratedLeft = schema.projectLeft point
  rightProjection : regeneratedRight = schema.projectRight point

/-- Occurrence-owned determination, recovery, and residual structure remain open. -/
inductive BoundaryPointRegenerationObligation where
  | reifiedPoint
  | declaredGrain
  | candidateChartAncestry
  | determinationPresentations
  | useIdentities
  | returnFibers
  | recovery
  | seed
  | eventsAndResiduals
  | completedReciprocalOccurrence
  | contextualExteriority
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

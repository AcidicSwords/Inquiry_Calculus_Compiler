import InquiryCalculus.Legacy.V20.RepresentationGapLocalization

/-! # v2.0 typed-distinction-schema boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- Binding-supplied compatibility at a boundary point, before an inquiry occurrence is formed. -/
structure DownstreamCompatibility (B : Binding) (BoundaryPoint : Type u) where
  holds : BoundaryPoint → B.generatorToken → Prop

/--
A candidate-boundary schema records two side carriers, a boundary-point carrier, projections, and
binding-supplied downstream compatibility.  Inquiry-occurrence structure remains outside it.
-/
structure TypedDistinctionSchema (B : Binding) where
  leftCandidate : Type
  rightCandidate : Type
  boundaryPoint : Type
  projectLeft : boundaryPoint → leftCandidate
  projectRight : boundaryPoint → rightCandidate
  downstreamCompatibility : DownstreamCompatibility B boundaryPoint

/-- Determinations, negations, seeds, return fibers, and occurrence completion remain open. -/
inductive TypedDistinctionSchemaObligation where
  | candidateSideCarriers
  | boundaryPointCarrier
  | projections
  | bindingSuppliedCompatibility
  | determinationPresentations
  | negationUses
  | seedsAndReturnFibers
  | truthOrPrivilegedOrientation
  | completedReciprocalOccurrence
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

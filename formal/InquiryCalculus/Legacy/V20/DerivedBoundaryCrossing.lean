import InquiryCalculus.Legacy.V20.DepartureRelativePositivity

/-! # v2.0 derived-boundary-crossing boundary -/
namespace InquiryCalculus.Legacy.V20

/-- An observed crossing pairs a departure witness with traversal or succession provenance. -/
structure DerivedBoundaryCrossingSyntax (DepartureWitness TraversalSuccessionProvenance : Type) where
  departureWitness : DepartureWitness
  traversalSuccessionProvenance : TraversalSuccessionProvenance

/-- Exteriority and candidate projection do not independently provide either crossing component. -/
inductive DerivedBoundaryCrossingObligation where
  | departureWitness
  | traversalSuccessionProvenance
  | positiveExteriority
  | observedCrossingPath
  | candidateBoundaryProjection
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

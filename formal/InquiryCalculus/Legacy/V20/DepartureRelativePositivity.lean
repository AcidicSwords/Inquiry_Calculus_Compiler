import InquiryCalculus.Legacy.V20.PositiveDepartureWitness

/-! # v2.0 departure-relative-positivity obligation -/
namespace InquiryCalculus.Legacy.V20

universe u v w

/-- Incomplete observation is typed as Unknown before any interior/exterior classification. -/
inductive IncompleteObservationStatus where
  | unknown
  deriving DecidableEq, Repr

/--
The source law's typed boundary: departure, horizon difference, and incomplete observation are
separate carriers.  The claimed non-implications remain obligations, not theorems here.
-/
structure DepartureRelativePositivitySyntax
    (Source : Type u) (Candidate : Type v) (Observation : Type w) where
  departure : Source → Candidate → Prop
  horizonDifferent : Source → Candidate → Prop
  incompleteObservation : Observation → IncompleteObservationStatus
  protectedNearDeparture : Source → Candidate → Prop

/-- The source's positivity and determination-relative claims still require independent proof. -/
inductive DepartureRelativePositivityObligation where
  | positiveWitnessRequirement
  | determinationRelativity
  | equalityFailureDoesNotEstablishDeparture
  | searchFailureDoesNotEstablishDeparture
  | retrievalFailureDoesNotEstablishDeparture
  | generationFailureDoesNotEstablishDeparture
  | proofFailureDoesNotEstablishDeparture
  | incompleteObservationUnknown
  | unknownNeitherInteriorNorExterior
  | departureDoesNotImplyHorizonDifference
  | horizonDifferenceDoesNotImplyDeparture
  | protectedNearDeparture
  | absentPresentationRelation
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

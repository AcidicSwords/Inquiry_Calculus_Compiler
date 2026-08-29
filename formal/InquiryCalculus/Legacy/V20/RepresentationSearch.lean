import InquiryCalculus.Legacy.V20.HoleSolving

/-! # v2.0 representation-search boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x y

/-- Candidate forms searched after a witnessed protected distinction. -/
inductive RepresentationSearchCandidate where
  | separatingContext
  | representation
  | grain
  | probe
  | languageExtension
  | rebind
  deriving DecidableEq, Repr

/-- A representation-search request retains the witnessed equal observation and protected difference. -/
structure RepresentationSearchSyntax (Object : Type u) (Observation : Type v) (Horizon : Type w)
    (Candidate : Type x) (BindingOrder : Type y) where
  first : Object
  second : Object
  equalObservation : Observation
  protectedHorizon : Horizon
  protectedDifferenceWitness : Prop
  bindingAppropriateOrder : BindingOrder
  candidate : Candidate
  protectedConsequenceRelevant : Prop
  unnecessaryExcessDetailTested : Prop

/-- Source obligations retained until candidate execution and protected-consequence semantics are separately formalized. -/
inductive RepresentationSearchObligation where
  | witnessedEqualObservationAndProtectedDifference
  | derivedInquiryProgram
  | bindingAppropriateSearchOrder
  | separatingContextRepresentationGrainProbeLanguageExtensionOrRebind
  | protectedConsequenceRelevance
  | unnecessaryExcessDetailTest
  | noUngroundedInvention
  | noNoveltyOnlySuccess
  | noAnswerSelectionOrTotalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

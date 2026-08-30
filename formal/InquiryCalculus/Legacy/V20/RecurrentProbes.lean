import InquiryCalculus.Legacy.V20.OperatorOccurrenceIndex

/-! # v2.0 recurrent-probe boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The predecessor retains probe-contract and bridge coordinates without deciding probe identity or comparability. -/
structure RecurrentProbesSyntax (RelationRole : Type u) (Binding : Type u) (Grain : Type u)
    (Applicability : Type u) (Comparator : Type u) (ProtectedContinuation : Type u)
    (DecoderVersion : Type u) (ProbeContract : Type u) (Occurrence : Type u)
    (Bridge : Type u) (StandingRelation : Type u) (OperatorIdentifier : Type u) (Wording : Type u) where
  relationRole : RelationRole
  binding : Binding
  grain : Grain
  applicability : Applicability
  comparator : Comparator
  protectedContinuationFamily : ProtectedContinuation
  decoderVersion : DecoderVersion
  probeContract : ProbeContract
  firstOccurrence : Occurrence
  secondOccurrence : Occurrence
  bridge : Bridge
  standingTypedRelation : StandingRelation
  operatorIdentifier : OperatorIdentifier
  wording : Wording
  recurrentProbeContractShape : Prop
  occurrenceComparabilitySameContractOrBridgeShape : Prop
  bridgeEstablishesProtectedCorrespondenceUnproved : Prop
  wordingNeitherNecessaryNorSufficientForProbeIdentityUnproved : Prop
  sameWordingIsNotSameProbeUnproved : Prop
  sameOperatorIdentifierDoesNotImplyComparableOccurrenceUnproved : Prop
  noProbeContractConstruction : Prop
  noComparabilityDecision : Prop
  noBridgeAdmission : Prop
  noWordingIdentityTest : Prop
  noOperatorIdentityTest : Prop
  noSemanticAuthorityPromotion : Prop

/-- Source obligations retained until contract, bridge, and comparability behavior are separately checked. -/
inductive RecurrentProbesObligation where
  | recurrentProbeContractShape
  | occurrenceComparabilitySameContractOrBridgeShape
  | bridgeEstablishesProtectedCorrespondenceUnproved
  | wordingNeitherNecessaryNorSufficientForProbeIdentityUnproved
  | sameWordingIsNotSameProbeUnproved
  | sameOperatorIdentifierDoesNotImplyComparableOccurrenceUnproved
  | noProbeContractConstruction
  | noComparabilityDecision
  | noBridgeAdmission
  | noWordingIdentityTest
  | noOperatorIdentityTest
  | noSemanticAuthorityPromotion
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

import InquiryCalculus.Legacy.V20.OperationalRootAliases

/-! # v2.0 static-pair discipline

Source-bound reconstruction of the ambiguous and unproved boundary at v2.0 lines 5118–5136. The
five names remain typed presentation identities. Finite countermodels separate converse from
inverse, supplied bridge law from co-availability, and generic backward presentation from an exact
same-use reciprocal return. No universal pairwise theorem is promoted.
-/
namespace InquiryCalculus.Legacy.V20.StaticPairDiscipline

inductive StaticPairKind where
  | inverse
  | converse
  | logicalBreakerDual
  | bindingSuppliedAdjoint
  | sameUseReciprocalReturn
  deriving DecidableEq

def staticPairKinds : List StaticPairKind :=
  [.inverse, .converse, .logicalBreakerDual, .bindingSuppliedAdjoint,
    .sameUseReciprocalReturn]

theorem staticPairKindsArePairwiseDistinct : staticPairKinds.Pairwise (· ≠ ·) := by
  decide

structure PairPresentation where
  kind : StaticPairKind
  relationIdentity : Nat
  deriving DecidableEq

structure BindingSuppliedEquivalence (source target : PairPresentation) where
  lawSupplied : Bool
  protectedContinuationPreserved : Bool

def LawfulSubstitution {source target : PairPresentation}
    (bridge : BindingSuppliedEquivalence source target) : Prop :=
  bridge.lawSupplied = true ∧ bridge.protectedContinuationPreserved = true

structure ReciprocalExpectation where
  orientedUse : Nat

structure ReciprocalReturnWitness where
  orientedUse : Nat
  useAdmitted : Bool
  departurePresent : Bool
  coveragePresent : Bool
  wholeFiberPresent : Bool
  provenancePresent : Bool

def LawfulReciprocalReturn (expectation : ReciprocalExpectation)
    (witness : ReciprocalReturnWitness) : Prop :=
  witness.orientedUse = expectation.orientedUse ∧ witness.useAdmitted = true ∧
    witness.departurePresent = true ∧ witness.coveragePresent = true ∧
    witness.wholeFiberPresent = true ∧ witness.provenancePresent = true

/-- These source obligations remain open beyond the finite separators below. -/
inductive StaticPairObligation where
  | universalPairwiseRelationIdentity
  | bindingSuppliedContinuationEquivalence
  | converseFiberNotInverse
  | adjointLawNotConverseIncidence
  | exactAdmittedUse
  | departureCoverageFiberProvenance
  | noSemanticBackwardToActualSuccession
  deriving DecidableEq

namespace Countermodel

inductive Source where | left | right deriving DecidableEq
inductive Target where | sole deriving DecidableEq

def manyToOne : Source → Target → Prop := fun _ _ => True
def relationalConverse : Target → Source → Prop := fun target source => manyToOne source target

theorem converseFiberContainsBoth :
    relationalConverse .sole .left ∧ relationalConverse .sole .right :=
  ⟨True.intro, True.intro⟩

theorem converseIsNotStrictInverse :
    ¬ ∃ inverse : Target → Source, ∀ source, inverse .sole = source := by
  rintro ⟨inverse, recovers⟩
  have collision : Source.left = Source.right :=
    (recovers .left).symm.trans (recovers .right)
  exact Source.noConfusion collision

def conversePresentation : PairPresentation := ⟨.converse, 10⟩
def adjointPresentation : PairPresentation := ⟨.bindingSuppliedAdjoint, 20⟩

def completeBridge : BindingSuppliedEquivalence conversePresentation adjointPresentation :=
  ⟨true, true⟩
def unsuppliedBridge : BindingSuppliedEquivalence conversePresentation adjointPresentation :=
  ⟨false, true⟩
def unpreservedBridge : BindingSuppliedEquivalence conversePresentation adjointPresentation :=
  ⟨true, false⟩

theorem completeBridgeIsLawful : LawfulSubstitution completeBridge := ⟨rfl, rfl⟩
theorem unsuppliedBridgeIsNotLawful : ¬ LawfulSubstitution unsuppliedBridge := by
  intro lawful
  exact Bool.noConfusion lawful.1
theorem unpreservedBridgeIsNotLawful : ¬ LawfulSubstitution unpreservedBridge := by
  intro lawful
  exact Bool.noConfusion lawful.2
theorem suppliedBridgeDoesNotCollapseKinds :
    conversePresentation.kind ≠ adjointPresentation.kind := by decide

def expectedReciprocal : ReciprocalExpectation := ⟨42⟩
def completeReciprocal : ReciprocalReturnWitness := ⟨42, true, true, true, true, true⟩
def wrongUseReciprocal : ReciprocalReturnWitness := ⟨7, true, true, true, true, true⟩
def missingAdmission : ReciprocalReturnWitness := ⟨42, false, true, true, true, true⟩
def missingDeparture : ReciprocalReturnWitness := ⟨42, true, false, true, true, true⟩
def missingCoverage : ReciprocalReturnWitness := ⟨42, true, true, false, true, true⟩
def missingFiber : ReciprocalReturnWitness := ⟨42, true, true, true, false, true⟩
def missingProvenance : ReciprocalReturnWitness := ⟨42, true, true, true, true, false⟩

theorem sevenIsNotFortyTwo : (7 : Nat) ≠ 42 := by decide

theorem completeReciprocalIsLawful :
    LawfulReciprocalReturn expectedReciprocal completeReciprocal :=
  ⟨rfl, rfl, rfl, rfl, rfl, rfl⟩
theorem wrongUseIsNotReciprocal :
    ¬ LawfulReciprocalReturn expectedReciprocal wrongUseReciprocal := by
  intro lawful
  have useIdentity := lawful.1
  change (7 : Nat) = 42 at useIdentity
  exact sevenIsNotFortyTwo useIdentity
theorem missingAdmissionIsNotReciprocal :
    ¬ LawfulReciprocalReturn expectedReciprocal missingAdmission := by
  intro lawful
  exact Bool.noConfusion lawful.2.1
theorem missingDepartureIsNotReciprocal :
    ¬ LawfulReciprocalReturn expectedReciprocal missingDeparture := by
  intro lawful
  exact Bool.noConfusion lawful.2.2.1
theorem missingCoverageIsNotReciprocal :
    ¬ LawfulReciprocalReturn expectedReciprocal missingCoverage := by
  intro lawful
  exact Bool.noConfusion lawful.2.2.2.1
theorem missingFiberIsNotReciprocal :
    ¬ LawfulReciprocalReturn expectedReciprocal missingFiber := by
  intro lawful
  exact Bool.noConfusion lawful.2.2.2.2.1
theorem missingProvenanceIsNotReciprocal :
    ¬ LawfulReciprocalReturn expectedReciprocal missingProvenance := by
  intro lawful
  exact Bool.noConfusion lawful.2.2.2.2.2

structure GenericBackwardPresentation where
  relationIdentity : Nat

def bareBackward : GenericBackwardPresentation := ⟨99⟩

theorem genericBackwardDoesNotSupplyReciprocal :
    ¬ LawfulReciprocalReturn expectedReciprocal missingProvenance :=
  missingProvenanceIsNotReciprocal

end Countermodel
end InquiryCalculus.Legacy.V20.StaticPairDiscipline

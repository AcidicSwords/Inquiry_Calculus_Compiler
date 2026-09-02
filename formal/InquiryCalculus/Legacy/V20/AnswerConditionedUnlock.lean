import InquiryCalculus.Legacy.V20.StaticPairDiscipline

/-! # v2.0 answer-conditioned unlock and route explanations

Source-bound reconstruction of the definition and unproved prose boundary at v2.0 lines
5140–5159. Unlock is exactly a readiness crossing under one typed Ask/whole-supported-Answer step.
It neither resolves the continuation nor grants route annotations execution or authority.
-/
namespace InquiryCalculus.Legacy.V20.AnswerConditionedUnlock

universe u v w x y c

def Unlock {State : Type u} {AskReference : Type v} {Question : Type w}
    {SupportedAnswer : Question → Type x} {Continuation : Type y}
    {Contract : Type c} (askQuestion : AskReference → Question)
    (ready : State → Contract → Continuation → Prop)
    (qstep : (state : State) → (askReference : AskReference) →
      SupportedAnswer (askQuestion askReference) → State)
    (contract : Contract) (state : State) (askReference : AskReference)
    (answer : SupportedAnswer (askQuestion askReference)) (continuation : Continuation) : Prop :=
  ¬ ready state contract continuation ∧
    ready (qstep state askReference answer) contract continuation

structure RouteAnnotations where
  narrowing : Bool
  discharge : Bool
  defeat : Bool
  reframe : Bool
  reorientation : Bool
  extension : Bool
  deriving DecidableEq

structure RouteAuthority where
  defeatAuthorized : Bool
  representationOrBindingRevisionAccepted : Bool
  canonicalSeedRetained : Bool
  successorAccepted : Bool

def LawfulRouteExplanation (annotations : RouteAnnotations) (authority : RouteAuthority) : Prop :=
  (annotations.defeat = true → authority.defeatAuthorized = true) ∧
    (annotations.reframe = true → authority.representationOrBindingRevisionAccepted = true) ∧
    (annotations.reorientation = true → authority.canonicalSeedRetained = true) ∧
    (annotations.extension = true → authority.successorAccepted = true)

/-- Runtime projection deliberately erases overlapping explanatory labels. -/
def eraseRouteAnnotations (transitionIdentity : Nat) (_ : RouteAnnotations) : Nat :=
  transitionIdentity

inductive AnswerConditionedUnlockObligation where
  | exactPreStateNonreadiness
  | exactPostStepReadiness
  | exactAskOccurrenceAndWholeSupportedAnswer
  | targetUseContract
  | readinessNotResolution
  | labelsMayOverlap
  | labelsNotRuntimeDispatch
  | defeatRequiresAuthority
  | reframeRequiresAcceptedRevision
  | reorientationRetainsCanonicalSeed
  | extensionRequiresAcceptedSuccessor
  | adjacencyNotUnlock
  | noAnswerConstructionOrExecution
  | noProgramOrRustAuthority
  deriving DecidableEq

namespace Countermodel

inductive State where | before | after deriving DecidableEq
inductive AskReference where | crossing | stalled deriving DecidableEq
inductive Question where | source deriving DecidableEq
inductive Answer : Question → Type where | supported : Answer .source
inductive Continuation where | target | other deriving DecidableEq
inductive Contract where | target | other deriving DecidableEq

def askQuestion : AskReference → Question := fun _ => .source

def readyFlag : State → Contract → Continuation → Bool
  | state, contract, continuation =>
      state == .after && contract == .target && continuation == .target

def ready (state : State) (contract : Contract) (continuation : Continuation) : Prop :=
  readyFlag state contract continuation = true

def qstep : (state : State) → (askReference : AskReference) →
    Answer (askQuestion askReference) → State
  | state, askReference, _ => if askReference = .crossing then .after else state

def resolved : State → Continuation → Prop := fun _ _ => False

theorem beforeTargetNotReady :
    ¬ ready .before .target .target := fun readiness => Bool.noConfusion readiness

theorem afterTargetReady :
    ready .after .target .target := rfl

theorem afterOtherContractNotReady :
    ¬ ready .after .other .target := fun readiness => Bool.noConfusion readiness

theorem afterOtherContinuationNotReady :
    ¬ ready .after .target .other := fun readiness => Bool.noConfusion readiness

theorem crossingUnlocks :
    Unlock askQuestion ready qstep .target .before .crossing
      .supported .target :=
  ⟨beforeTargetNotReady, afterTargetReady⟩

theorem alreadyReadyDoesNotUnlock :
    ¬ Unlock askQuestion ready qstep .target .after .crossing
      .supported .target := by
  intro unlock
  exact unlock.1 afterTargetReady

theorem stalledStepDoesNotUnlock :
    ¬ Unlock askQuestion ready qstep .target .before .stalled
      .supported .target := by
  intro unlock
  exact Bool.noConfusion unlock.2

theorem wrongContractDoesNotUnlock :
    ¬ Unlock askQuestion ready qstep .other .before .crossing
      .supported .target := by
  intro unlock
  exact afterOtherContractNotReady unlock.2

theorem wrongContinuationDoesNotUnlock :
    ¬ Unlock askQuestion ready qstep .target .before .crossing
      .supported .other := by
  intro unlock
  exact afterOtherContinuationNotReady unlock.2

theorem unlockDoesNotResolve :
    Unlock askQuestion ready qstep .target .before .crossing
      .supported .target ∧ ¬ resolved .after .target :=
  ⟨crossingUnlocks, fun resolution => resolution⟩

def overlappingRoutes : RouteAnnotations := ⟨true, true, true, true, true, true⟩
def noRoutes : RouteAnnotations := ⟨false, false, false, false, false, false⟩

theorem overlappingAnnotationsAreRepresentable :
    overlappingRoutes.defeat = true ∧ overlappingRoutes.reframe = true := ⟨rfl, rfl⟩

theorem deletingRouteLabelsCannotChangeExecution :
    eraseRouteAnnotations 17 overlappingRoutes = eraseRouteAnnotations 17 noRoutes := rfl

theorem routeLabelsDoNotCreateUnlock :
    ¬ Unlock askQuestion ready qstep .target .before .stalled
      .supported .target :=
  stalledStepDoesNotUnlock

def completeAuthority : RouteAuthority := ⟨true, true, true, true⟩
def missingDefeatAuthority : RouteAuthority := ⟨false, true, true, true⟩
def missingRevision : RouteAuthority := ⟨true, false, true, true⟩
def missingSeed : RouteAuthority := ⟨true, true, false, true⟩
def missingSuccessor : RouteAuthority := ⟨true, true, true, false⟩

theorem completeExplanationIsLawful :
    LawfulRouteExplanation overlappingRoutes completeAuthority :=
  ⟨fun _ => rfl, fun _ => rfl, fun _ => rfl, fun _ => rfl⟩

theorem missingDefeatAuthorityIsNotLawful :
    ¬ LawfulRouteExplanation overlappingRoutes missingDefeatAuthority := by
  intro lawful
  exact Bool.noConfusion (lawful.1 rfl)

theorem missingRevisionIsNotLawful :
    ¬ LawfulRouteExplanation overlappingRoutes missingRevision := by
  intro lawful
  exact Bool.noConfusion (lawful.2.1 rfl)

theorem missingSeedIsNotLawful :
    ¬ LawfulRouteExplanation overlappingRoutes missingSeed := by
  intro lawful
  exact Bool.noConfusion (lawful.2.2.1 rfl)

theorem missingSuccessorIsNotLawful :
    ¬ LawfulRouteExplanation overlappingRoutes missingSuccessor := by
  intro lawful
  exact Bool.noConfusion (lawful.2.2.2 rfl)

end Countermodel
end InquiryCalculus.Legacy.V20.AnswerConditionedUnlock

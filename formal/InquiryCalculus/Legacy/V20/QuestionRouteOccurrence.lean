import InquiryCalculus.Legacy.V20.LocalInterrogativeFixedPoint
import InquiryCalculus.Legacy.V20.OccurrenceIndexedQuestionSuccession

/-! # v2.0 question-route occurrence and regeneration

Source-bound reconstruction of v2.0 lines 5201–5239.  `QuestionRouteOccurrence` is a compact
derived projection whose coordinates retain exact occurrence, continuation, path, support, horizon,
coverage, and provenance identities.  It is neither semantic question identity nor event history.
Absent actual coordinates remain absent.  The final source law remains an obligation: this module
proves the finite non-collapse and regeneration boundary, not a universal conservative expansion.
-/
namespace InquiryCalculus.Legacy.V20.QuestionRouteOccurrence

universe u

structure ActualCoordinates (Event RawReturn SelectedAnswer : Type u) where
  event : Option Event
  rawReturn : Option RawReturn
  selectedAnswer : Option SelectedAnswer

def PureCoordinates (Event RawReturn SelectedAnswer : Type u) :
    ActualCoordinates Event RawReturn SelectedAnswer := ⟨none, none, none⟩

structure QuestionRouteOccurrence
    (AskReference Relation Route ResolutionPath SupportedAnswer Continuation Delta Horizon Mode
      Coverage Provenance Event RawReturn SelectedAnswer : Type u) where
  askReference : AskReference
  relation : Relation
  route : Route
  resolutionPath : ResolutionPath
  supportedAnswer : SupportedAnswer
  successorAskReference : AskReference
  continuation : Continuation
  inputDelta : Delta
  outputDelta : Delta
  horizon : Horizon
  modes : List Mode
  coverage : Coverage
  provenance : Provenance
  actual : ActualCoordinates Event RawReturn SelectedAnswer

def IsPureRoute {Event RawReturn SelectedAnswer : Type u}
    (actual : ActualCoordinates Event RawReturn SelectedAnswer) : Prop :=
  actual.event = none ∧ actual.rawReturn = none ∧ actual.selectedAnswer = none

theorem pureCoordinatesAreNonactual (Event RawReturn SelectedAnswer : Type u) :
    IsPureRoute (PureCoordinates Event RawReturn SelectedAnswer) := by
  exact ⟨rfl, rfl, rfl⟩

def SupportedAnswerCueFiber {AskReference Answer Question : Type u}
    (qSucc : AskReference → Answer → Question → Prop) (askReference : AskReference)
    (nextQuestion : Question) : Type u :=
  { answer : Answer // qSucc askReference answer nextQuestion }

def CheckedOccurrenceFiber {Occurrence Question : Type u}
    (questionOf : Occurrence → Question) (checked : Occurrence → Prop)
    (question : Question) : Type u :=
  { occurrence : Occurrence // questionOf occurrence = question ∧ checked occurrence }

structure ExactRegeneration (Carrier : Type u) (protectedEquivalent : Carrier → Carrier → Prop)
    (representative : Carrier) where
  oneProtectedClass : ∀ candidate, protectedEquivalent candidate representative
  typingSurvives : Prop
  authoritySurvives : Prop
  eventPathProvenanceSurvives : Prop
  reopeningObligationsSurvive : Prop

def ExactRegeneration.ready {Carrier : Type u} {protectedEquivalent : Carrier → Carrier → Prop}
    {representative : Carrier} (contract : ExactRegeneration Carrier protectedEquivalent representative) : Prop :=
  contract.typingSurvives ∧ contract.authoritySurvives ∧
    contract.eventPathProvenanceSurvives ∧ contract.reopeningObligationsSurvive

inductive QuestionRouteOccurrenceObligation where
  | compactDerivedProjection
  | exactOccurrenceIdentity
  | equalSemanticProjectionDoesNotCollapseOccurrence
  | continuationIdentity
  | resolutionPathIdentity
  | supportAncestryIdentity
  | absentActualFieldsRemainNonactual
  | supportedAnswerRemovalIsOccurrenceSpecific
  | questionRemovalYieldsCheckedOccurrences
  | exactRegenerationRequiresOneProtectedClass
  | typingAuthorityEventPathAndReopeningSurvive
  | ambiguityRemainsSeparatorResidual
  | recurrentRouteMethodIsDerived
  | occurrenceAblationBeforeMethodFold
  | frequencyDoesNotWarrant
  | hostPolicyDoesNotReplaceContinuation
  | noSecondQuestionLanguageOrHistory
  | noSelectionExecutionSupportOrWarrant
  | noRustOrSuccessorAuthority
  deriving DecidableEq, Repr

namespace Countermodel

structure CompactRoute where
  semanticQuestion : Bool
  answerCardinality : Nat
  occurrence : Bool
  continuation : Bool
  resolutionPath : Bool
  supportAncestry : Bool
  checked : Bool
  deriving DecidableEq, Repr

def semanticProjection (route : CompactRoute) : Bool × Nat :=
  (route.semanticQuestion, route.answerCardinality)

def leftRoute : CompactRoute := ⟨false, 1, false, false, false, false, true⟩
def rightRoute : CompactRoute := ⟨false, 1, true, true, true, true, true⟩

theorem equalSemanticProjection : semanticProjection leftRoute = semanticProjection rightRoute := by
  rfl

theorem distinctQuestionRouteOccurrences : leftRoute ≠ rightRoute := by
  decide

def cueFiber (route : CompactRoute) (answer : Bool) : Prop := route.occurrence = answer

theorem supportedAnswerRemovalIsOccurrenceSpecific :
    cueFiber leftRoute false ∧ ¬ cueFiber rightRoute false := by
  change false = false ∧ ¬ true = false
  exact ⟨rfl, by intro impossible; cases impossible⟩

def checkedFiber (question : Bool) :=
  { route : CompactRoute // route.semanticQuestion = question ∧ route.checked = true }

def leftChecked : checkedFiber false := ⟨leftRoute, by decide⟩
def rightChecked : checkedFiber false := ⟨rightRoute, by decide⟩

theorem questionRemovalRetainsDistinctCheckedOccurrences :
    leftChecked.val ≠ rightChecked.val := by
  decide

def dropOccurrence (route : CompactRoute) : CompactRoute := { route with occurrence := false }
def dropContinuation (route : CompactRoute) : CompactRoute := { route with continuation := false }
def dropResolutionPath (route : CompactRoute) : CompactRoute := { route with resolutionPath := false }
def dropSupportAncestry (route : CompactRoute) : CompactRoute := { route with supportAncestry := false }

theorem eachProtectedCoordinateSeparatesRightRoute :
    dropOccurrence rightRoute ≠ rightRoute ∧ dropContinuation rightRoute ≠ rightRoute ∧
      dropResolutionPath rightRoute ≠ rightRoute ∧ dropSupportAncestry rightRoute ≠ rightRoute := by
  decide

end Countermodel
end InquiryCalculus.Legacy.V20.QuestionRouteOccurrence

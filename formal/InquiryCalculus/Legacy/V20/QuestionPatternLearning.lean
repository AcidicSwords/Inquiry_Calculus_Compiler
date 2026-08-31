import InquiryCalculus.Legacy.V20.TraversalLearning

/-! # Question-pattern learning

Source-bound realization of v2.0 lines 4348–4395. The complete realized
occurrence remains primary. Compact operator and state/boundary views are lookup
projections and cannot authorize an occurrence-specific continuation by themselves.
-/
namespace InquiryCalculus.Legacy.V20.QuestionPatternLearning

universe u v w x y z u₁ v₁

structure OccurrenceChain (AskRef : Type u) (EventRef : Type v) (StateRef : Type w)
    (OpenIR : Type x) (BoundaryRef : Type y) (ProbeView : Type z)
    (Continuation : Type u₁) (Control : Type v₁) where
  askRef : AskRef
  dischargeBundle : List EventRef
  checkedState : StateRef
  openBefore : OpenIR
  boundary : BoundaryRef
  probeView : ProbeView
  nextState : StateRef
  openAfter : OpenIR
  checkedContinuation : Continuation
  nextControl : Control

def operatorProjection {AskRef : Type u} {EventRef : Type v} {StateRef : Type w}
    {OpenIR : Type x} {BoundaryRef : Type y} {ProbeView : Type z}
    {Continuation : Type u₁} {Control : Type v₁}
    (occurrence : OccurrenceChain AskRef EventRef StateRef OpenIR BoundaryRef ProbeView
      Continuation Control) : OpenIR × ProbeView × OpenIR :=
  (occurrence.openBefore, occurrence.probeView, occurrence.openAfter)

def stateProjection {AskRef : Type u} {EventRef : Type v} {StateRef : Type w}
    {OpenIR : Type x} {BoundaryRef : Type y} {ProbeView : Type z}
    {Continuation : Type u₁} {Control : Type v₁}
    (occurrence : OccurrenceChain AskRef EventRef StateRef OpenIR BoundaryRef ProbeView
      Continuation Control) : StateRef × OpenIR × BoundaryRef × ProbeView × StateRef × OpenIR :=
  (occurrence.checkedState, occurrence.openBefore, occurrence.boundary,
    occurrence.probeView, occurrence.nextState, occurrence.openAfter)

def ProjectionLookup {AskRef : Type u} {EventRef : Type v} {StateRef : Type w}
    {OpenIR : Type x} {BoundaryRef : Type y} {ProbeView : Type z}
    {Continuation : Type u₁} {Control : Type v₁}
    (target candidate : OccurrenceChain AskRef EventRef StateRef OpenIR BoundaryRef ProbeView
      Continuation Control) : Prop :=
  operatorProjection target = operatorProjection candidate

structure AggregationContext (Occurrence : Type u) where
  protectedEquivalent : Occurrence → Occurrence → Prop
  applicable : Occurrence → Prop

def AggregationLicensed {Occurrence : Type u} (context : AggregationContext Occurrence)
    (left right : Occurrence) : Prop :=
  context.applicable left ∧ context.applicable right ∧ context.protectedEquivalent left right

structure LearnedPolicy (Occurrence : Type u) (State : Type v) (Continuation : Type w) where
  applicable : Occurrence → State → Prop
  lookup : Occurrence → Occurrence → Prop
  recover : Occurrence → Occurrence → Prop
  selects : Occurrence → Continuation → Prop

/-- Lookup may nominate candidates, but selection requires an applicability proof and
recovery of the exact requested occurrence chain. -/
structure OccurrenceSpecificSelection {Occurrence : Type u} {State : Type v}
    {Continuation : Type w} (policy : LearnedPolicy Occurrence State Continuation)
    (target : Occurrence) (state : State) (continuation : Continuation) where
  applicability : policy.applicable target state
  lookupCandidate : policy.lookup target target
  recovered : Occurrence
  recovery : policy.recover target recovered
  exactChain : recovered = target
  selected : policy.selects recovered continuation

namespace Countermodel

abbrev Chain := OccurrenceChain Bool Bool Unit Unit Unit Unit Bool Bool

def left : Chain := ⟨false, [false], (), (), (), (), (), (), false, false⟩
def right : Chain := ⟨true, [true], (), (), (), (), (), (), true, true⟩

theorem operatorViewsCollide : operatorProjection left = operatorProjection right := rfl
theorem stateViewsCollide : stateProjection left = stateProjection right := rfl
theorem occurrencesRemainDistinct : left ≠ right := by
  intro equal
  have askEqual := congrArg OccurrenceChain.askRef equal
  cases askEqual

theorem projectionLookupRetainsLeft : ProjectionLookup left left := rfl
theorem projectionLookupRetainsRight : ProjectionLookup left right := rfl

def licensedAggregation : AggregationContext Chain where
  protectedEquivalent := fun first second => first = second ∨
    (first = left ∧ second = right) ∨ (first = right ∧ second = left)
  applicable := fun _ => True

def unlicensedAggregation : AggregationContext Chain where
  protectedEquivalent := fun _ _ => False
  applicable := fun _ => True

def inapplicableAggregation : AggregationContext Chain where
  protectedEquivalent := fun _ _ => True
  applicable := fun _ => False

theorem explicitLicenseAdmits : AggregationLicensed licensedAggregation left right := by
  exact ⟨trivial, trivial, Or.inr (Or.inl ⟨rfl, rfl⟩)⟩

theorem projectionEqualityDoesNotLicense :
    ¬ AggregationLicensed unlicensedAggregation left right := by
  intro admission
  exact admission.2.2

theorem aggregationApplicabilityRequired :
    ¬ AggregationLicensed inapplicableAggregation left right := by
  intro admission
  exact admission.1

def recoveredPolicy : LearnedPolicy Chain Unit Bool where
  applicable := fun occurrence _ => occurrence = left
  lookup := ProjectionLookup
  recover := fun requested recovered => requested = recovered
  selects := fun occurrence continuation => occurrence = left ∧ continuation = false

def projectionOnlyPolicy : LearnedPolicy Chain Unit Bool where
  applicable := fun _ _ => True
  lookup := ProjectionLookup
  recover := fun _ _ => False
  selects := fun _ _ => True

def inapplicablePolicy : LearnedPolicy Chain Unit Bool where
  applicable := fun _ _ => False
  lookup := ProjectionLookup
  recover := fun requested recovered => requested = recovered
  selects := fun _ _ => True

def wrongChainPolicy : LearnedPolicy Chain Unit Bool where
  applicable := fun _ _ => True
  lookup := ProjectionLookup
  recover := fun requested recovered => requested = left ∧ recovered = right
  selects := fun occurrence _ => occurrence = right

def unselectingPolicy : LearnedPolicy Chain Unit Bool where
  applicable := fun _ _ => True
  lookup := ProjectionLookup
  recover := fun requested recovered => requested = recovered
  selects := fun _ _ => False

theorem recoveredApplicableSelection :
    Nonempty (OccurrenceSpecificSelection recoveredPolicy left () false) := by
  exact ⟨⟨rfl, rfl, left, rfl, rfl, ⟨rfl, rfl⟩⟩⟩

theorem projectionOnlyCannotSelect :
    ¬ Nonempty (OccurrenceSpecificSelection projectionOnlyPolicy left () false) := by
  rintro ⟨selection⟩
  exact selection.recovery

theorem applicabilityRequired :
    ¬ Nonempty (OccurrenceSpecificSelection inapplicablePolicy left () false) := by
  rintro ⟨selection⟩
  exact selection.applicability

theorem exactOccurrenceRecoveryRequired :
    ¬ Nonempty (OccurrenceSpecificSelection wrongChainPolicy left () false) := by
  rintro ⟨selection⟩
  have rightEqualsLeft := selection.recovery.2.symm.trans selection.exactChain
  exact occurrencesRemainDistinct rightEqualsLeft.symm

theorem selectedContinuationRequired :
    ¬ Nonempty (OccurrenceSpecificSelection unselectingPolicy left () false) := by
  rintro ⟨selection⟩
  exact selection.selected

end Countermodel
end InquiryCalculus.Legacy.V20.QuestionPatternLearning

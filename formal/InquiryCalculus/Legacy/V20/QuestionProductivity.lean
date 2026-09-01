import InquiryCalculus.Legacy.V20.DerivedQuestionAvailability

/-! # v2.0 question-productivity boundary

Source-bound reconstruction of v2.0 lines 4945–4977.  Productivity is an existential
protected-difference between lawful supported answers at one exact checked occurrence.  The
protected-successor lifting is supplied by the binding; the finite reference lifting records the
set-of-projections form without selecting a universal lifting or an execution policy.
-/
namespace InquiryCalculus.Legacy.V20.QuestionProductivity

universe u v w x y

structure ProductivityContext (Question : Type u) (Occurrence : Type v) (Answer : Type w)
    (Successor : Type x) where
  normalizedQuestion : Occurrence → Question
  returnClass : Occurrence → Nat
  lawful : Answer → Prop
  supported : Occurrence → Answer → Prop
  successor : Occurrence → Answer → Successor
  sufficientCoverage : Occurrence → Prop

def Productive {Question : Type u} {Occurrence : Type v} {Answer : Type w} {Successor : Type x}
    (context : ProductivityContext Question Occurrence Answer Successor)
    (protectedEquivalent : Successor → Successor → Prop) (occurrence : Occurrence) : Prop :=
  ∃ left right, context.lawful left ∧ context.lawful right ∧ context.supported occurrence left ∧
    context.supported occurrence right ∧
      ¬ protectedEquivalent (context.successor occurrence left) (context.successor occurrence right)

def Resolved {Question : Type u} {Occurrence : Type v} {Answer : Type w} {Successor : Type x}
    (context : ProductivityContext Question Occurrence Answer Successor)
    (protectedEquivalent : Successor → Successor → Prop) (occurrence : Occurrence) : Prop :=
  context.sufficientCoverage occurrence ∧
    ∀ left right, context.lawful left → context.lawful right → context.supported occurrence left →
      context.supported occurrence right →
        protectedEquivalent (context.successor occurrence left) (context.successor occurrence right)

def consequenceFiber {Context : Type u} {Value : Type v} {Projected : Type w}
    (projection : Context → Value → Projected) (context : Context) (values : Value → Prop) : Projected → Prop :=
  fun projected => ∃ value, values value ∧ projection context value = projected

def referenceFiniteSymmetricLifting {Context : Type u} {Value : Type v} {Projected : Type w}
    (contexts : List Context) (projection : Context → Value → Projected)
    (left right : Value → Prop) : Prop :=
  ∀ context, context ∈ contexts →
    consequenceFiber projection context left = consequenceFiber projection context right

theorem resolvedNotProductive {Question : Type u} {Occurrence : Type v} {Answer : Type w}
    {Successor : Type x} (context : ProductivityContext Question Occurrence Answer Successor)
    (protectedEquivalent : Successor → Successor → Prop) (occurrence : Occurrence) :
    Resolved context protectedEquivalent occurrence → ¬ Productive context protectedEquivalent occurrence := by
  intro resolved productive
  rcases productive with ⟨left, right, lawfulLeft, lawfulRight, supportedLeft, supportedRight, different⟩
  exact different (resolved.2 left right lawfulLeft lawfulRight supportedLeft supportedRight)

namespace Countermodel

inductive Question where | same deriving DecidableEq

inductive Occurrence where | sensitive | neutral deriving DecidableEq

inductive Answer where | left | right deriving DecidableEq

def context : ProductivityContext Question Occurrence Answer Nat where
  normalizedQuestion := fun _ => .same
  returnClass := fun _ => 0
  lawful := fun _ => True
  supported := fun _ _ => True
  successor := fun occurrence answer =>
    match occurrence, answer with
    | .sensitive, .left => 0
    | .sensitive, .right => 1
    | .neutral, _ => 0
  sufficientCoverage := fun _ => True

def equalityLifting : Nat → Nat → Prop := Eq

def collapseLifting : Nat → Nat → Prop := fun _ _ => True

theorem sameNormalizedQuestionAndReturnClass :
    context.normalizedQuestion .sensitive = context.normalizedQuestion .neutral ∧
      context.returnClass .sensitive = context.returnClass .neutral :=
  ⟨rfl, rfl⟩

theorem sensitiveProductive : Productive context equalityLifting .sensitive := by
  refine ⟨.left, .right, True.intro, True.intro, True.intro, True.intro, ?_⟩
  exact Nat.zero_ne_one

theorem neutralNotProductive : ¬ Productive context equalityLifting .neutral := by
  intro productive
  rcases productive with ⟨firstAnswer, secondAnswer, _, _, _, _, different⟩
  cases firstAnswer <;> cases secondAnswer <;> exact different rfl

theorem occurrenceIndexMatters :
    Productive context equalityLifting .sensitive ∧ ¬ Productive context equalityLifting .neutral :=
  ⟨sensitiveProductive, neutralNotProductive⟩

theorem suppliedLiftingChangesProductivity : ¬ Productive context collapseLifting .sensitive := by
  intro productive
  rcases productive with ⟨firstAnswer, secondAnswer, _, _, _, _, different⟩
  exact different True.intro

theorem neutralResolved : Resolved context equalityLifting .neutral := by
  refine ⟨True.intro, ?_⟩
  intro firstAnswer secondAnswer _ _ _ _
  cases firstAnswer <;> cases secondAnswer <;> rfl

theorem resolutionExcludesProductivity : ¬ Productive context equalityLifting .neutral :=
  resolvedNotProductive context equalityLifting .neutral neutralResolved

def referenceContexts : List Unit := [()]

def referenceProjection : Unit → Nat → Nat := fun _ value => value

def leftFiber : Nat → Prop := fun value => value = 0

def rightFiber : Nat → Prop := fun value => value = 1

theorem referenceLiftingSeparatesFibers :
    ¬ referenceFiniteSymmetricLifting referenceContexts referenceProjection leftFiber rightFiber := by
  intro equivalent
  have equalFibers := equivalent () (by simp [referenceContexts])
  have leftMember : consequenceFiber referenceProjection () leftFiber 0 := ⟨0, rfl, rfl⟩
  have rightMember : consequenceFiber referenceProjection () rightFiber 0 := by
    rw [← equalFibers]
    exact leftMember
  rcases rightMember with ⟨value, valueIsOne, valueMapsToZero⟩
  exact Nat.zero_ne_one (valueMapsToZero ▸ valueIsOne)

end Countermodel
end InquiryCalculus.Legacy.V20.QuestionProductivity

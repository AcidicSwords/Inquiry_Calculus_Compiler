import InquiryCalculus.Legacy.V20.Reopening

/-! # Pattern separation and completion

Source-bound realization of v2.0 lines 4812–4820. Working similarity is not itself protected
equivalence: similar events with protected-different consequences need a separating
representation. Completion is separately licensed only by all-live protected equivalence or by
an explicitly supplied approximate licence.
-/
namespace InquiryCalculus.Legacy.V20.PatternSeparation

universe u v w

structure PatternBoundary (Event : Type u) (Representation : Type v) (Consequence : Type w) where
  represent : Event → Representation
  similar : Event → Event → Prop
  consequence : Event → Consequence
  protectedDifferent : Consequence → Consequence → Prop

def RequiresSeparation {Event : Type u} {Representation : Type v} {Consequence : Type w}
    (boundary : PatternBoundary Event Representation Consequence) (left right : Event) : Prop :=
  boundary.similar left right →
    boundary.protectedDifferent (boundary.consequence left) (boundary.consequence right) →
      boundary.represent left ≠ boundary.represent right

structure CompletionBoundary (Completion : Type u) where
  live : Completion → Prop
  protectedEquivalent : Completion → Completion → Prop
  approximatePermitsAmbiguity : Prop

def CompletionLicensed {Completion : Type u} (boundary : CompletionBoundary Completion) : Prop :=
  (∀ left right, boundary.live left → boundary.live right →
    boundary.protectedEquivalent left right) ∨ boundary.approximatePermitsAmbiguity

namespace Countermodel

inductive Event where | left | right deriving DecidableEq
inductive Merged where | only deriving DecidableEq
inductive Split where | left | right deriving DecidableEq
inductive Consequence where | retained | changed deriving DecidableEq

def commonSimilarity : Event → Event → Prop := fun _ _ => True
def consequence : Event → Consequence
  | .left => .retained
  | .right => .changed

def merged : PatternBoundary Event Merged Consequence where
  represent := fun _ => .only
  similar := commonSimilarity
  consequence := consequence
  protectedDifferent := fun left right => left ≠ right

def split : PatternBoundary Event Split Consequence where
  represent
    | .left => .left
    | .right => .right
  similar := commonSimilarity
  consequence := consequence
  protectedDifferent := fun left right => left ≠ right

theorem mergedDoesNotSeparate : ¬ RequiresSeparation merged .left .right := by
  intro separates
  exact separates True.intro (show Consequence.retained ≠ Consequence.changed by decide) rfl

theorem splitSeparates : RequiresSeparation split .left .right := by
  intro _ _
  decide

inductive Completion where | historic | novel deriving DecidableEq

def exactCompletion : CompletionBoundary Completion where
  live := fun completion => completion = .historic
  protectedEquivalent := fun left right => left = right
  approximatePermitsAmbiguity := False

def unlicensedCompletion : CompletionBoundary Completion where
  live := fun _ => True
  protectedEquivalent := fun left right => left = right
  approximatePermitsAmbiguity := False

def approximateCompletion : CompletionBoundary Completion :=
  { unlicensedCompletion with approximatePermitsAmbiguity := True }

theorem exactCompletionIsLicensed : CompletionLicensed exactCompletion := by
  left
  intro left right leftLive rightLive
  cases leftLive
  cases rightLive
  rfl

theorem inequivalentCompletionWithoutLicenceFails : ¬ CompletionLicensed unlicensedCompletion := by
  intro licensed
  rcases licensed with allEquivalent | approximate
  · exact Completion.noConfusion (allEquivalent .historic .novel True.intro True.intro)
  · exact approximate

theorem approximateLicencePermitsAmbiguity : CompletionLicensed approximateCompletion := by
  right
  trivial

end Countermodel
end InquiryCalculus.Legacy.V20.PatternSeparation

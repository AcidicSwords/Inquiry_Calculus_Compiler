import InquiryCalculus.Legacy.V20.ExactRepresentationQuotient

/-! # Continuation sufficiency

Source-bound realization of v2.0 line 4664.  A continuation descent is an explicitly supplied
target action commuting with a supplied quotient; quotient existence and current consequence
sufficiency do not synthesize that action.
-/
namespace InquiryCalculus.Legacy.V20.ContinuationSufficiency

open ExactRepresentationQuotient

universe u v

structure Continuation (Carrier : Type u) where
  step : Carrier → Carrier

structure ContinuationScope (Carrier : Type u) where
  protectedContinuation : Continuation Carrier → Prop

def DescendsThrough {Source : Type u} {Target : Type v}
    (quotient : ProposedQuotient Source Target) (source : Continuation Source)
    (target : Continuation Target) : Prop :=
  ∀ value, quotient.map (source.step value) = target.step (quotient.map value)

def HasDescendedContinuation {Source : Type u} {Target : Type v}
    (quotient : ProposedQuotient Source Target) (source : Continuation Source) : Prop :=
  ∃ target, DescendsThrough quotient source target

def AllProtectedContinuationsDescend {Source : Type u} {Target : Type v}
    (scope : ContinuationScope Source) (quotient : ProposedQuotient Source Target) : Prop :=
  ∀ source, scope.protectedContinuation source → HasDescendedContinuation quotient source

namespace Countermodel

open ExactRepresentationQuotient.Countermodel

def compatibleSource : Continuation Source where
  step := fun value => value

def compatibleTarget : Continuation ExactTarget where
  step := fun value => value

def compatibleScope : ContinuationScope Source where
  protectedContinuation := fun continuation => continuation = compatibleSource

def splittingSource : Continuation Source where
  step := fun value => match value with | .a => .a | .b => .c | .c => .c

def splittingScope : ContinuationScope Source where
  protectedContinuation := fun continuation => continuation = splittingSource

theorem compatibleDescends : DescendsThrough exactMap compatibleSource compatibleTarget := by
  intro value
  cases value <;> rfl

theorem compatibleScopeDescends : AllProtectedContinuationsDescend compatibleScope exactMap := by
  intro source sourceProtected
  subst source
  exact ⟨compatibleTarget, compatibleDescends⟩

theorem splittingHasNoDescendedContinuation :
    ¬ HasDescendedContinuation exactMap splittingSource := by
  rintro ⟨target, descended⟩
  have atA := descended .a
  have atB := descended .b
  change ExactTarget.ab = target.step ExactTarget.ab at atA
  change ExactTarget.c = target.step ExactTarget.ab at atB
  exact ExactTarget.noConfusion (atA.trans atB.symm)

theorem splittingScopeFails : ¬ AllProtectedContinuationsDescend splittingScope exactMap := by
  intro allDescend
  exact splittingHasNoDescendedContinuation (allDescend splittingSource rfl)

end Countermodel
end InquiryCalculus.Legacy.V20.ContinuationSufficiency

import InquiryCalculus.Legacy.V20.ApproximateCompression

/-! # Unlock field

Source-bound realization of v2.0 lines 4745–4779.  An unlock is a tagged observational, dynamic,
or context-contract failure.  Its tags remain distinct; a compression's continued existence does
not discharge an active trigger.
-/
namespace InquiryCalculus.Legacy.V20.UnlockField

universe u

inductive UnlockKind where | observational | dynamic | context deriving DecidableEq

structure UnlockTrigger (Compression : Type u) where
  compression : Compression
  kind : UnlockKind
  active : Prop

def IsUnlocked {Compression : Type u} (trigger : UnlockTrigger Compression) : Prop := trigger.active

def SeparatorTriggered {Compression : Type u} (trigger : UnlockTrigger Compression) : Prop :=
  trigger.kind = .observational ∧ trigger.active

theorem separatorIsUnlock {Compression : Type u} (trigger : UnlockTrigger Compression) :
    SeparatorTriggered trigger → IsUnlocked trigger := by
  intro separator
  exact separator.2

namespace Countermodel

inductive Compression where | retained deriving DecidableEq

def observationBreak : UnlockTrigger Compression := ⟨.retained, .observational, True⟩
def dynamicBreak : UnlockTrigger Compression := ⟨.retained, .dynamic, True⟩
def contextBreak : UnlockTrigger Compression := ⟨.retained, .context, True⟩
def inactiveObservation : UnlockTrigger Compression := ⟨.retained, .observational, False⟩

theorem observationUnlocks : IsUnlocked observationBreak := True.intro
theorem dynamicUnlocks : IsUnlocked dynamicBreak := True.intro
theorem contextUnlocks : IsUnlocked contextBreak := True.intro
theorem inactiveObservationDoesNotUnlock : ¬ IsUnlocked inactiveObservation := by intro active; exact active
theorem observationalIsNotDynamic : observationBreak.kind ≠ dynamicBreak.kind := by decide

end Countermodel
end InquiryCalculus.Legacy.V20.UnlockField

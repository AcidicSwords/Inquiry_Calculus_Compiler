import InquiryCalculus.Legacy.V20.UnlockField

/-! # Reopening

Source-bound realization of v2.0 lines 4792–4808.  Reopening requires an active unlock and keeps
historical ancestry while separately locating the implicated identification, restoring material,
refining the active representation, and recording the new breaker.
-/
namespace InquiryCalculus.Legacy.V20.Reopening

open UnlockField

universe u

structure ReopeningRecord (Compression : Type u) where
  trigger : UnlockTrigger Compression
  preservesHistory : Prop
  locatesSmallestIdentification : Prop
  restoresResidueProvenanceRecovery : Prop
  refinesActiveRepresentation : Prop
  recordsNewBreaker : Prop

def LawfulReopening {Compression : Type u} (record : ReopeningRecord Compression) : Prop :=
  IsUnlocked record.trigger ∧ record.preservesHistory ∧ record.locatesSmallestIdentification ∧
  record.restoresResidueProvenanceRecovery ∧ record.refinesActiveRepresentation ∧ record.recordsNewBreaker

namespace Countermodel

open UnlockField.Countermodel

def complete : ReopeningRecord Compression :=
  ⟨observationBreak, True, True, True, True, True⟩

def deletesHistory : ReopeningRecord Compression := { complete with preservesHistory := False }
def inactiveTrigger : ReopeningRecord Compression := { complete with trigger := inactiveObservation }

theorem completeIsLawful : LawfulReopening complete := by
  exact ⟨True.intro, True.intro, True.intro, True.intro, True.intro, True.intro⟩

theorem deletionIsNotLawful : ¬ LawfulReopening deletesHistory := by
  intro lawful
  exact lawful.2.1

theorem inactiveTriggerIsNotLawful : ¬ LawfulReopening inactiveTrigger := by
  intro lawful
  exact lawful.1

end Countermodel
end InquiryCalculus.Legacy.V20.Reopening

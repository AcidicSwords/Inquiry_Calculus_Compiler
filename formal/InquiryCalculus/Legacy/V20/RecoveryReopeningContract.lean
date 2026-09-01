import InquiryCalculus.Legacy.V20.RegenerativePreservation

/-! # Recovery and reopening contract

Source-bound realization of v2.0 lines 4679–4687.  A regeneration witness is not by itself a
recovery/reopening contract: provenance, residual distinction, factorization route, recovery
route, and unlock trigger are distinct requirement-indexed coordinates.
-/
namespace InquiryCalculus.Legacy.V20.RecoveryReopeningContract

universe u

structure RecoveryReopeningContract (Requirement : Type u) where
  provenance : Requirement → Prop
  residualDistinction : Requirement → Prop
  factorizationRoute : Requirement → Prop
  recoveryRoute : Requirement → Prop
  unlockTrigger : Requirement → Prop

def RetainsEnough {Requirement : Type u} (contract : RecoveryReopeningContract Requirement)
    (requirement : Requirement) : Prop :=
  contract.provenance requirement ∧
  contract.residualDistinction requirement ∧
  contract.factorizationRoute requirement ∧
  contract.recoveryRoute requirement ∧
  contract.unlockTrigger requirement

def ProtectsEvery {Requirement : Type u} (protectedRequirement : Requirement → Prop)
    (contract : RecoveryReopeningContract Requirement) : Prop :=
  ∀ requirement, protectedRequirement requirement → RetainsEnough contract requirement

namespace Countermodel

inductive Requirement where | reopen deriving DecidableEq

def protectedRequirement : Requirement → Prop := fun requirement => requirement = .reopen

def complete : RecoveryReopeningContract Requirement where
  provenance := fun _ => True
  residualDistinction := fun _ => True
  factorizationRoute := fun _ => True
  recoveryRoute := fun _ => True
  unlockTrigger := fun _ => True

def missingProvenance : RecoveryReopeningContract Requirement := { complete with provenance := fun _ => False }
def missingResidual : RecoveryReopeningContract Requirement := { complete with residualDistinction := fun _ => False }
def missingFactorization : RecoveryReopeningContract Requirement := { complete with factorizationRoute := fun _ => False }
def missingRecovery : RecoveryReopeningContract Requirement := { complete with recoveryRoute := fun _ => False }
def missingUnlock : RecoveryReopeningContract Requirement := { complete with unlockTrigger := fun _ => False }

theorem completeProtectsEvery : ProtectsEvery protectedRequirement complete := by
  intro requirement _
  cases requirement
  exact ⟨True.intro, True.intro, True.intro, True.intro, True.intro⟩

theorem missingProvenanceFails : ¬ ProtectsEvery protectedRequirement missingProvenance := by
  intro preserved
  exact (preserved .reopen rfl).1

theorem missingResidualFails : ¬ ProtectsEvery protectedRequirement missingResidual := by
  intro preserved
  exact (preserved .reopen rfl).2.1

theorem missingFactorizationFails : ¬ ProtectsEvery protectedRequirement missingFactorization := by
  intro preserved
  exact (preserved .reopen rfl).2.2.1

theorem missingRecoveryFails : ¬ ProtectsEvery protectedRequirement missingRecovery := by
  intro preserved
  exact (preserved .reopen rfl).2.2.2.1

theorem missingUnlockFails : ¬ ProtectsEvery protectedRequirement missingUnlock := by
  intro preserved
  exact (preserved .reopen rfl).2.2.2.2

end Countermodel
end InquiryCalculus.Legacy.V20.RecoveryReopeningContract

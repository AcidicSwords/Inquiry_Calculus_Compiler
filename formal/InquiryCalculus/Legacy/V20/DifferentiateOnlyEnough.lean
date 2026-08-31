import InquiryCalculus.Legacy.V20.RegenerativeEconomyCorrespondence

/-! # Differentiate only enough to regenerate

Conditional elaboration of the Unproved v2.0 law at lines 4209–4216.  The law
acts on a supplied active presentation.  Its authoritative ancestry is carried
unchanged, and every claim that a further removal is blocked requires positive
typed loss evidence.
-/
namespace InquiryCalculus.Legacy.V20.DifferentiateOnlyEnough

universe u v

/-- Active distinctions are a presentation over immutable authoritative
ancestry.  The ancestry carrier is deliberately opaque to ablation. -/
structure ActivePresentation (Distinction : Type u) (Ancestry : Type v) where
  active : Distinction → Prop
  ancestry : Ancestry

/-- Remove a supplied set of active distinctions and retain all governing
ancestry exactly. -/
def ablate {Distinction : Type u} {Ancestry : Type v}
    (removed : Distinction → Prop)
    (presentation : ActivePresentation Distinction Ancestry) :
    ActivePresentation Distinction Ancestry where
  active := fun distinction => presentation.active distinction ∧ ¬ removed distinction
  ancestry := presentation.ancestry

def removeOne {Distinction : Type u} {Ancestry : Type v}
    (presentation : ActivePresentation Distinction Ancestry)
    (removed : Distinction) : ActivePresentation Distinction Ancestry :=
  ablate (fun distinction => distinction = removed) presentation

theorem ablationPreservesAuthoritativeAncestry
    {Distinction : Type u} {Ancestry : Type v}
    (removed : Distinction → Prop)
    (presentation : ActivePresentation Distinction Ancestry) :
    (ablate removed presentation).ancestry = presentation.ancestry := rfl

/-- The three conditions that a binding must preserve while subtracting active
distinctions.  Their meanings are supplied by the protected horizon. -/
structure ProtectedActiveContract (Distinction : Type u) where
  separatesLiveCompletions : (Distinction → Prop) → Prop
  inquiryRegenerative : (Distinction → Prop) → Prop
  licensedContinuation : (Distinction → Prop) → Prop

def ContractSatisfied {Distinction : Type u}
    (contract : ProtectedActiveContract Distinction)
    (active : Distinction → Prop) : Prop :=
  contract.separatesLiveCompletions active ∧
  contract.inquiryRegenerative active ∧
  contract.licensedContinuation active

def PreservingAblation {Distinction : Type u} {Ancestry : Type v}
    (contract : ProtectedActiveContract Distinction)
    (presentation : ActivePresentation Distinction Ancestry)
    (removed : Distinction → Prop) : Prop :=
  ContractSatisfied contract presentation.active ∧
    ContractSatisfied contract (ablate removed presentation).active

/-- The five protected loss dimensions named by the source law. -/
inductive LossKind where
  | regeneration
  | discrimination
  | continuation
  | warrantProvenance
  | reopening
  deriving DecidableEq, Repr

/-- A supplied positive loss relation must be sound for the declared contract.
Not finding a witness cannot inhabit this structure. -/
structure ProtectedLossRelation {Distinction : Type u}
    (contract : ProtectedActiveContract Distinction) where
  loses : LossKind → (Distinction → Prop) → (Distinction → Prop) → Prop
  sound : ∀ {kind before after}, loses kind before after →
    ContractSatisfied contract before → ¬ ContractSatisfied contract after

structure WitnessedProtectedLoss {Distinction : Type u}
    (contract : ProtectedActiveContract Distinction)
    (loss : ProtectedLossRelation contract)
    (before after : Distinction → Prop) where
  kind : LossKind
  beforeProtected : ContractSatisfied contract before
  evidence : loss.loses kind before after

theorem witnessedLossBlocksSubtraction {Distinction : Type u}
    {contract : ProtectedActiveContract Distinction}
    {loss : ProtectedLossRelation contract}
    {before after : Distinction → Prop}
    (witness : WitnessedProtectedLoss contract loss before after) :
    ¬ ContractSatisfied contract after :=
  loss.sound witness.evidence witness.beforeProtected

/-- A retained active presentation is locally irreducible only when every
further single removal has a positive protected-loss witness. -/
def DifferentiatedOnlyEnough {Distinction : Type u} {Ancestry : Type v}
    (contract : ProtectedActiveContract Distinction)
    (loss : ProtectedLossRelation contract)
    (presentation : ActivePresentation Distinction Ancestry) : Prop :=
  ContractSatisfied contract presentation.active ∧
    ∀ distinction, presentation.active distinction →
      Nonempty (WitnessedProtectedLoss contract loss presentation.active
        (removeOne presentation distinction).active)

namespace Countermodel

inductive Distinction3 where
  | separates
  | regenerates
  | redundant
  deriving DecidableEq

open Distinction3

def allActive (_ : Distinction3) : Prop := True

def minimalActive : Distinction3 → Prop
  | separates => True
  | regenerates => True
  | redundant => False

def contract : ProtectedActiveContract Distinction3 where
  separatesLiveCompletions := fun active => active separates
  inquiryRegenerative := fun active => active regenerates
  licensedContinuation := fun _ => True

def fullPresentation : ActivePresentation Distinction3 (List Nat) where
  active := allActive
  ancestry := [2, 3, 5, 7]

def minimalPresentation : ActivePresentation Distinction3 (List Nat) where
  active := minimalActive
  ancestry := fullPresentation.ancestry

def protectedLosses : ProtectedLossRelation contract where
  loses
    | .discrimination, before, after => before separates ∧ ¬ after separates
    | .regeneration, before, after => before regenerates ∧ ¬ after regenerates
    | _, _, _ => False
  sound := by
    intro kind before after evidence beforeOk
    rcases beforeOk with ⟨separated, regenerated, licensed⟩
    cases kind with
    | discrimination => exact fun afterProtected => evidence.2 afterProtected.1
    | regeneration => exact fun afterProtected => evidence.2 afterProtected.2.1
    | continuation => exact False.elim evidence
    | warrantProvenance => exact False.elim evidence
    | reopening => exact False.elim evidence

theorem fullContract : ContractSatisfied contract fullPresentation.active := by
  exact ⟨trivial, trivial, trivial⟩

theorem removeRedundantPreserves :
    PreservingAblation contract fullPresentation (fun distinction => distinction = redundant) := by
  constructor
  · exact fullContract
  · exact ⟨⟨trivial, by decide⟩, ⟨trivial, by decide⟩, trivial⟩

theorem removeRedundantKeepsAncestry :
    (ablate (fun distinction => distinction = redundant) fullPresentation).ancestry =
      fullPresentation.ancestry := rfl

theorem minimalContract : ContractSatisfied contract minimalPresentation.active := by
  exact ⟨trivial, trivial, trivial⟩

theorem minimalIsDifferentiatedOnlyEnough :
    DifferentiatedOnlyEnough contract protectedLosses minimalPresentation := by
  constructor
  · exact minimalContract
  · intro distinction active
    cases distinction with
    | separates =>
        exact ⟨⟨.discrimination, minimalContract,
          ⟨trivial, fun after => after.2 rfl⟩⟩⟩
    | regenerates =>
        exact ⟨⟨.regeneration, minimalContract,
          ⟨trivial, fun after => after.2 rfl⟩⟩⟩
    | redundant => exact False.elim active

theorem economyMembershipDoesNotProveEveryDistinctionNecessary :
    ContractSatisfied contract fullPresentation.active ∧
      PreservingAblation contract fullPresentation
        (fun distinction => distinction = redundant) :=
  ⟨fullContract, removeRedundantPreserves⟩

theorem fullIsNotYetDifferentiatedOnlyEnough :
    ¬ DifferentiatedOnlyEnough contract protectedLosses fullPresentation := by
  intro enough
  obtain ⟨witness⟩ := enough.2 redundant trivial
  exact witnessedLossBlocksSubtraction witness removeRedundantPreserves.2

end Countermodel
end InquiryCalculus.Legacy.V20.DifferentiateOnlyEnough

import InquiryCalculus.Legacy.V20.RecoveryReopeningContract

/-! # Compression license

Source-bound realization of v2.0 lines 4691–4709.  A proposed exact compression license bundles
distinct quotient, protected-family, continuation-family, scope, recovery, residual, unlock, and
evidence coordinates.  The bundle does not decide exactness merely because tests do not separate
current outputs.
-/
namespace InquiryCalculus.Legacy.V20.CompressionLicense

open ExactRepresentationQuotient
open ContinuationSufficiency
open RegenerativePreservation

universe u v

structure CompressionLicense (Source : Type u) (Target : Type v) where
  quotient : ProposedQuotient Source Target
  protectedEquivalent : Source → Source → Prop
  protectedContinuations : ContinuationScope Source
  scope : Source → Prop
  recovery : RegenerationContext Source Target
  residual : Source → Prop
  unlock : Target → Prop
  evidence : Target → Prop

def LicensedFor {Source : Type u} {Target : Type v} (license : CompressionLicense Source Target)
    (source : Source) (continuation : Continuation Source) : Prop :=
  license.protectedEquivalent source source ∧
  license.protectedContinuations.protectedContinuation continuation ∧
  license.scope source ∧
  license.recovery.protectedFutureUse source ∧
  license.recovery.regenerates (license.quotient.map source) source ∧
  license.residual source ∧
  license.unlock (license.quotient.map source) ∧
  license.evidence (license.quotient.map source)

namespace Countermodel

open ExactRepresentationQuotient.Countermodel

def identityContinuation : Continuation Source where
  step := fun source => source

def complete : CompressionLicense Source ExactTarget where
  quotient := exactMap
  protectedEquivalent := equivalent
  protectedContinuations := { protectedContinuation := fun continuation => continuation = identityContinuation }
  scope := fun source => source = .a
  recovery := ⟨fun source => source = .a, fun target source => target = .ab ∧ source = .a⟩
  residual := fun source => source = .a
  unlock := fun target => target = .ab
  evidence := fun target => target = .ab

def missingEvidence : CompressionLicense Source ExactTarget := { complete with evidence := fun _ => False }
def missingUnlock : CompressionLicense Source ExactTarget := { complete with unlock := fun _ => False }

theorem completeLicensesProtectedSource : LicensedFor complete .a identityContinuation := by
  change (equivalent Source.a Source.a) ∧ identityContinuation = identityContinuation ∧
    Source.a = Source.a ∧ Source.a = Source.a ∧ (ExactTarget.ab = ExactTarget.ab ∧ Source.a = Source.a) ∧
    Source.a = Source.a ∧ ExactTarget.ab = ExactTarget.ab ∧ ExactTarget.ab = ExactTarget.ab
  exact ⟨Or.inl ⟨rfl, rfl⟩, rfl, rfl, rfl, ⟨rfl, rfl⟩, rfl, rfl, rfl⟩

theorem missingEvidenceFails : ¬ LicensedFor missingEvidence .a identityContinuation := by
  intro licensed
  exact licensed.2.2.2.2.2.2.2

theorem missingUnlockFails : ¬ LicensedFor missingUnlock .a identityContinuation := by
  intro licensed
  exact licensed.2.2.2.2.2.2.1

theorem unprotectedSourceIsNotLicensed : ¬ LicensedFor complete .c identityContinuation := by
  intro licensed
  exact Source.noConfusion licensed.2.2.1

end Countermodel
end InquiryCalculus.Legacy.V20.CompressionLicense

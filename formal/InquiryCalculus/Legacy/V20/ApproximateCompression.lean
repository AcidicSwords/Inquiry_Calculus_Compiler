import InquiryCalculus.Legacy.V20.CompressionLicense

/-! # Approximate compression

Source-bound realization of v2.0 lines 4713–4741.  Approximation carries a directional
distortion contract and a distinct coordinate bundle.  A common scalar error cannot identify
over- and under-approximation or make an approximate licence exact.
-/
namespace InquiryCalculus.Legacy.V20.ApproximateCompression

open ExactRepresentationQuotient

universe u v

inductive DistortionDirection where | over | under | bindingSpecific deriving DecidableEq

structure ApproximateLicense (Source : Type u) (Target : Type v) where
  map : Source → Target
  protectedFamily : Source → Prop
  direction : DistortionDirection
  scalarError : Nat
  admissibleError : Nat → Prop
  resourceGain : Nat → Prop
  residualRetained : Source → Prop
  reopenTrigger : Source → Prop
  continuationApproximated : Source → Prop

def DirectionallyLicensed {Source : Type u} {Target : Type v}
    (requiredDirection : DistortionDirection) (license : ApproximateLicense Source Target)
    (source : Source) : Prop :=
  license.protectedFamily source ∧
  license.direction = requiredDirection ∧
  license.admissibleError license.scalarError ∧
  license.resourceGain license.scalarError ∧
  license.residualRetained source ∧
  license.reopenTrigger source ∧
  license.continuationApproximated source

namespace Countermodel

inductive Source where | observed deriving DecidableEq
inductive Target where | compressed deriving DecidableEq

def base (direction : DistortionDirection) : ApproximateLicense Source Target where
  map := fun _ => .compressed
  protectedFamily := fun source => source = .observed
  direction := direction
  scalarError := 1
  admissibleError := fun error => error = 1
  resourceGain := fun error => error = 1
  residualRetained := fun source => source = .observed
  reopenTrigger := fun source => source = .observed
  continuationApproximated := fun source => source = .observed

def over : ApproximateLicense Source Target := base .over
def under : ApproximateLicense Source Target := base .under
def missingResidual : ApproximateLicense Source Target := { over with residualRetained := fun _ => False }

theorem sameScalarError : over.scalarError = under.scalarError := rfl

theorem overIsDirectionallyLicensed : DirectionallyLicensed .over over .observed := by
  exact ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl⟩

theorem underFailsOverDirection : ¬ DirectionallyLicensed .over under .observed := by
  intro licensed
  exact DistortionDirection.noConfusion licensed.2.1

theorem missingResidualFails : ¬ DirectionallyLicensed .over missingResidual .observed := by
  intro licensed
  exact licensed.2.2.2.2.1

end Countermodel
end InquiryCalculus.Legacy.V20.ApproximateCompression

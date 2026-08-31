import InquiryCalculus.Legacy.V20.RegenerativeEconomyFrontier
import InquiryCalculus.Legacy.V20.RegenerativeSufficiency

/-! # Dependent regenerative-economy correspondence -/
namespace InquiryCalculus.Legacy.V20.RegenerativeEconomyCorrespondence

open RegenerativeSufficiency

universe u v w x y

/-- The resource comparison is supplied with the two laws needed for a preorder;
the construction does not totalize it or attach a scalar score. -/
structure SuppliedResourcePreorder (Representation : Type u) where
  le : Representation → Representation → Prop
  reflexive : ∀ representation, le representation representation
  transitive : ∀ {left middle right}, le left middle → le middle right → le left right

/-- The seven source-named license coordinates remain independent supplied
relations on a representation and its protected source. -/
structure RegenerativeLicense (Representation : Type u) (Source : Type v) where
  requiredScope : Representation → Source → Prop
  applicability : Representation → Source → Prop
  authority : Representation → Source → Prop
  provenance : Representation → Source → Prop
  continuation : Representation → Source → Prop
  recovery : Representation → Source → Prop
  unlock : Representation → Source → Prop

def LicenseSatisfied {Representation : Type u} {Source : Type v}
    (license : RegenerativeLicense Representation Source)
    (representation : Representation) (source : Source) : Prop :=
  license.requiredScope representation source ∧
  license.applicability representation source ∧
  license.authority representation source ∧
  license.provenance representation source ∧
  license.continuation representation source ∧
  license.recovery representation source ∧
  license.unlock representation source

/-- The exact source candidate condition, now connected to the checked dependent
inquiry-regenerative predicate instead of an opaque Boolean parameter. -/
def DependentLicensedCandidate {Component : Type u} {Value : Component → Type v}
    {Source : Type w} {Representation : Type x} {RevisionRole : Type y}
    (components : ProtectedComponentFamily Component Value Source)
    (reconstruction : TypedReconstructionFamily Component Value Representation)
    (revision : InquiryRevisionFamily RevisionRole Representation Source)
    (license : RegenerativeLicense Representation Source) (source : Source) :
    Representation → Prop :=
  fun representation =>
    InquiryRegenerativeSufficient components reconstruction revision representation source ∧
      LicenseSatisfied license representation source

/-- The historical generic carrier is reused after its candidate is filled by
the dependent regenerative and exact license relations. -/
def DependentRegenerativeEconomyFrontier
    {Component : Type u} {Value : Component → Type v}
    {Source : Type w} {Representation : Type x} {RevisionRole : Type y}
    (components : ProtectedComponentFamily Component Value Source)
    (reconstruction : TypedReconstructionFamily Component Value Representation)
    (revision : InquiryRevisionFamily RevisionRole Representation Source)
    (license : RegenerativeLicense Representation Source)
    (resources : SuppliedResourcePreorder Representation) (source : Source) :
    Representation → Prop :=
  regenerativeEconomyFrontier resources.le
    (DependentLicensedCandidate components reconstruction revision license source)

theorem frontierMemberInquiryRegenerative
    {Component : Type u} {Value : Component → Type v}
    {Source : Type w} {Representation : Type x} {RevisionRole : Type y}
    (components : ProtectedComponentFamily Component Value Source)
    (reconstruction : TypedReconstructionFamily Component Value Representation)
    (revision : InquiryRevisionFamily RevisionRole Representation Source)
    (license : RegenerativeLicense Representation Source)
    (resources : SuppliedResourcePreorder Representation)
    (source : Source) (representation : Representation) :
    DependentRegenerativeEconomyFrontier components reconstruction revision license
        resources source representation →
      InquiryRegenerativeSufficient components reconstruction revision representation source := by
  intro frontier
  exact frontier.1.1

theorem frontierMemberLicensed
    {Component : Type u} {Value : Component → Type v}
    {Source : Type w} {Representation : Type x} {RevisionRole : Type y}
    (components : ProtectedComponentFamily Component Value Source)
    (reconstruction : TypedReconstructionFamily Component Value Representation)
    (revision : InquiryRevisionFamily RevisionRole Representation Source)
    (license : RegenerativeLicense Representation Source)
    (resources : SuppliedResourcePreorder Representation)
    (source : Source) (representation : Representation) :
    DependentRegenerativeEconomyFrontier components reconstruction revision license
        resources source representation →
      LicenseSatisfied license representation source := by
  intro frontier
  exact frontier.1.2

theorem noCandidateNoFrontier {Representation : Type u}
    (resources : SuppliedResourcePreorder Representation)
    (candidate : Representation → Prop)
    (none : ∀ representation, ¬ candidate representation) :
    ∀ representation, ¬ regenerativeEconomyFrontier resources.le candidate representation := by
  intro representation frontier
  exact none representation frontier.1

namespace Countermodel

open RegenerativeSufficiency.Countermodel

def completeLicense : RegenerativeLicense Source2 Source2 where
  requiredScope := fun _ _ => True
  applicability := fun _ _ => True
  authority := fun _ _ => True
  provenance := fun _ _ => True
  continuation := fun _ _ => True
  recovery := fun _ _ => True
  unlock := fun _ _ => True

/-- A named consequence can factor while the required reopening route is absent. -/
def namedConsequenceFactors (_representation _source : Source2) : Prop := True

theorem factorOnlyNotLicensedCandidate (source : Source2) :
    namedConsequenceFactors source source ∧
      ¬ DependentLicensedCandidate protectedComponents completeReconstruction
        missingRevision completeLicense source source := by
  constructor
  · trivial
  · intro candidate
    exact completeButNotInquiryRegenerative source candidate.1

def equalityPreorder : SuppliedResourcePreorder Bool where
  le := Eq
  reflexive := fun _ => rfl
  transitive := Eq.trans

def everyBoolCandidate (_ : Bool) : Prop := True

theorem falseIsEconomical :
    regenerativeEconomyFrontier equalityPreorder.le everyBoolCandidate false := by
  constructor
  · trivial
  · intro contender _ before
    exact before.symm

theorem trueIsEconomical :
    regenerativeEconomyFrontier equalityPreorder.le everyBoolCandidate true := by
  constructor
  · trivial
  · intro contender _ before
    exact before.symm

theorem economicalFrontierNotUnique : false ≠ true := by decide

theorem equalityPreorderNotTotal :
    ¬ ∀ left right, equalityPreorder.le left right ∨ equalityPreorder.le right left := by
  intro total
  rcases total false true with forward | backward
  · exact Bool.noConfusion forward
  · exact Bool.noConfusion backward

def emptyCandidate (_ : Bool) : Prop := False

theorem emptyAdmittedRegion :
    ∀ representation, ¬ regenerativeEconomyFrontier equalityPreorder.le
      emptyCandidate representation :=
  noCandidateNoFrontier equalityPreorder emptyCandidate (fun _ => id)

end Countermodel
end InquiryCalculus.Legacy.V20.RegenerativeEconomyCorrespondence

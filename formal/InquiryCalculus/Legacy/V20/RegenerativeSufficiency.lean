import InquiryCalculus.Legacy.V20.OperatorDescent

/-! # Regenerative sufficiency

Source-bound realization of the two v2.0 definitions at lines 4154–4182.
The horizon, component family, reconstruction relations, protected-equivalence
judgments, and revision roles are all supplied.  Nothing here searches for a
reconstruction, executes one, or installs a universal component ontology.
-/
namespace InquiryCalculus.Legacy.V20.RegenerativeSufficiency

universe u v w x y

/-- A binding supplies the components protected by its current horizon.  A
dependent value family keeps differently typed components distinct. -/
structure ProtectedComponentFamily (Component : Type u)
    (Value : Component → Type v) (Source : Type w) where
  requiredAtHorizon : Component → Prop
  sourceComponent : (component : Component) → Source → Value component
  protectedEquivalent : (component : Component) → Value component → Value component → Prop

/-- Candidate reconstruction is a family of typed relations, not a selected
reconstruction function or an effective reconstruction procedure. -/
structure TypedReconstructionFamily (Component : Type u)
    (Value : Component → Type v) (Representation : Type x) where
  reconstructs : (component : Component) → Representation → Value component → Prop

/-- Recovery of one named component.  This is deliberately weaker than
regenerative sufficiency. -/
def RecoversComponent {Component : Type u} {Value : Component → Type v}
    {Source : Type w} {Representation : Type x}
    (components : ProtectedComponentFamily Component Value Source)
    (reconstruction : TypedReconstructionFamily Component Value Representation)
    (representation : Representation) (source : Source) (component : Component) : Prop :=
  ∃ reconstructed,
    reconstruction.reconstructs component representation reconstructed ∧
      components.protectedEquivalent component reconstructed
        (components.sourceComponent component source)

/-- `Regen_H(m,z)`: every component required by the supplied horizon has some
typed relational reconstruction protected-equivalent to its source component.
The existential remains inside each component obligation, so no global choice
function or execution is introduced. -/
def RegenerativeSufficient {Component : Type u} {Value : Component → Type v}
    {Source : Type w} {Representation : Type x}
    (components : ProtectedComponentFamily Component Value Source)
    (reconstruction : TypedReconstructionFamily Component Value Representation)
    (representation : Representation) (source : Source) : Prop :=
  ∀ component, components.requiredAtHorizon component →
    RecoversComponent components reconstruction representation source component

/-- A binding also supplies the revision roles needed to test and revise the
source: discriminator, residual, support, or reopening roles can be represented
by the caller without being installed here as a fixed enumeration. -/
structure InquiryRevisionFamily (RevisionRole : Type y)
    (Representation : Type x) (Source : Type w) where
  requiredForRevision : RevisionRole → Prop
  retainsOrRegenerates : RevisionRole → Representation → Source → Prop

/-- `Regen^inq_H(m,z)`: regenerative sufficiency plus every supplied required
revision role.  Availability is relational evidence, not actual execution or
warrant. -/
def InquiryRegenerativeSufficient {Component : Type u} {Value : Component → Type v}
    {Source : Type w} {Representation : Type x} {RevisionRole : Type y}
    (components : ProtectedComponentFamily Component Value Source)
    (reconstruction : TypedReconstructionFamily Component Value Representation)
    (revision : InquiryRevisionFamily RevisionRole Representation Source)
    (representation : Representation) (source : Source) : Prop :=
  RegenerativeSufficient components reconstruction representation source ∧
    ∀ role, revision.requiredForRevision role →
      revision.retainsOrRegenerates role representation source

theorem inquiryRegenerativeImpliesRegenerative
    {Component : Type u} {Value : Component → Type v}
    {Source : Type w} {Representation : Type x} {RevisionRole : Type y}
    (components : ProtectedComponentFamily Component Value Source)
    (reconstruction : TypedReconstructionFamily Component Value Representation)
    (revision : InquiryRevisionFamily RevisionRole Representation Source)
    (representation : Representation) (source : Source) :
    InquiryRegenerativeSufficient components reconstruction revision representation source →
      RegenerativeSufficient components reconstruction representation source :=
  And.left

namespace Countermodel

/-- Two components suffice to localize the difference between one successful
reconstruction and recovery of the whole required horizon. -/
inductive Component2 where
  | observed
  | future
  deriving DecidableEq

open Component2

def ComponentValue : Component2 → Type
  | observed => Bool
  | future => Nat

structure Source2 where
  observed : Bool
  future : Nat

def sourceComponent : (component : Component2) → Source2 → ComponentValue component
  | observed, source => source.observed
  | future, source => source.future

def allComponentsRequired (_ : Component2) : Prop := True

def protectedComponents : ProtectedComponentFamily Component2 ComponentValue Source2 where
  requiredAtHorizon := allComponentsRequired
  sourceComponent := sourceComponent
  protectedEquivalent := fun _ left right => left = right

/-- The overlarge foil retains only the already observed Boolean component. -/
def observedOnlyReconstruction :
    TypedReconstructionFamily Component2 ComponentValue Bool where
  reconstructs
    | observed => fun representation value => representation = value
    | future => fun _ _ => False

theorem observedComponentRecovers (observedValue : Bool) (futureValue : Nat) :
    RecoversComponent protectedComponents observedOnlyReconstruction observedValue
      ⟨observedValue, futureValue⟩ observed := by
  exact ⟨observedValue, rfl, rfl⟩

theorem observedOnlyNotRegenerative (observedValue : Bool) (futureValue : Nat) :
    ¬ RegenerativeSufficient protectedComponents observedOnlyReconstruction observedValue
      ⟨observedValue, futureValue⟩ := by
  intro regenerative
  obtain ⟨_, impossible, _⟩ := regenerative future trivial
  exact impossible

/-- A complete relational family recovers both supplied component types. -/
def completeReconstruction :
    TypedReconstructionFamily Component2 ComponentValue Source2 where
  reconstructs := fun component representation value =>
    value = sourceComponent component representation

theorem completeIsRegenerative (source : Source2) :
    RegenerativeSufficient protectedComponents completeReconstruction source source := by
  intro component _
  exact ⟨sourceComponent component source, rfl, rfl⟩

inductive RevisionRole1 where
  | reopeningRoute
  deriving DecidableEq

def reopeningRequired (_ : RevisionRole1) : Prop := True

/-- This foil reconstructs every component but supplies no revision route. -/
def missingRevision : InquiryRevisionFamily RevisionRole1 Source2 Source2 where
  requiredForRevision := reopeningRequired
  retainsOrRegenerates := fun _ _ _ => False

theorem completeButNotInquiryRegenerative (source : Source2) :
    ¬ InquiryRegenerativeSufficient protectedComponents completeReconstruction
      missingRevision source source := by
  intro inquiryRegenerative
  exact inquiryRegenerative.2 RevisionRole1.reopeningRoute trivial

end Countermodel
end InquiryCalculus.Legacy.V20.RegenerativeSufficiency

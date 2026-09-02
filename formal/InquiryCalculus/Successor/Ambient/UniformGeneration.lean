import InquiryCalculus.Successor.Ambient.CapabilityBasis

/-! # Gate C: coverage-indexed uniform generation

This module types the relation needed before ambient sufficiency or ablation minimality can be
claimed.  It is deliberately presentation-parametric.  A presentation may conservatively grow;
old targets and their witnessed constructions transport, while every newly protected target must
also receive a construction, source correspondence, and coverage witness.  Merely naming all
currently desired targets as atoms satisfies `PointwiseGenerates` but not `UniformlyGenerates`.
-/
namespace InquiryCalculus.Successor.Ambient

universe u v w x

set_option linter.checkUnivs false

structure RegenerationWitness
    {Presentation : Type u}
    (Target : Presentation → Type v)
    (Construction : (presentation : Presentation) → Target presentation → Type w)
    (Source : (presentation : Presentation) → Target presentation → Type x)
    (Corresponds : {presentation : Presentation} → {target : Target presentation} →
      Construction presentation target → Source presentation target → Prop)
    (Covered : {presentation : Presentation} → Target presentation → Prop)
    (presentation : Presentation) (target : Target presentation) where
  construction : Construction presentation target
  source : Source presentation target
  corresponds : Corresponds construction source
  covered : Covered target

/--
`ConservativeExtension` is supplied structure: it must preserve old targets and their complete
regeneration witnesses.  It does not assert that every rebinding is conservative.
-/
structure UniformGenerationBoundary where
  Presentation : Type u
  Extends : Presentation → Presentation → Prop
  extendsRefl : ∀ presentation, Extends presentation presentation
  extendsTrans : ∀ {first second third}, Extends first second → Extends second third → Extends first third
  Target : Presentation → Type v
  Protected : {presentation : Presentation} → Target presentation → Prop
  Construction : (presentation : Presentation) → Target presentation → Type w
  Source : (presentation : Presentation) → Target presentation → Type x
  Corresponds : {presentation : Presentation} → {target : Target presentation} →
    Construction presentation target → Source presentation target → Prop
  Covered : {presentation : Presentation} → Target presentation → Prop
  transportTarget : ∀ {first second}, Extends first second → Target first → Target second
  transportProtected : ∀ {first second} (extension : Extends first second) {target},
    Protected target → Protected (transportTarget extension target)
  transportWitness : ∀ {first second} (extension : Extends first second) {target},
    RegenerationWitness Target Construction Source Corresponds Covered first target →
      RegenerationWitness Target Construction Source Corresponds Covered second
        (transportTarget extension target)
  transportTarget_id : ∀ {presentation} (target : Target presentation),
    transportTarget (extendsRefl presentation) target = target
  transportTarget_comp : ∀ {first second third}
      (firstExtension : Extends first second) (secondExtension : Extends second third)
      (target : Target first),
    transportTarget (extendsTrans firstExtension secondExtension) target =
      transportTarget secondExtension (transportTarget firstExtension target)

def PointwiseGenerates (boundary : UniformGenerationBoundary) (presentation : boundary.Presentation) : Prop :=
  ∀ target, boundary.Protected target →
    Nonempty (RegenerationWitness boundary.Target boundary.Construction boundary.Source
      boundary.Corresponds boundary.Covered presentation target)

def UniformlyGenerates (boundary : UniformGenerationBoundary) (presentation : boundary.Presentation) : Prop :=
  ∀ extended, boundary.Extends presentation extended → ∀ target, boundary.Protected target →
    Nonempty (RegenerationWitness boundary.Target boundary.Construction boundary.Source
      boundary.Corresponds boundary.Covered extended target)

theorem uniformGenerationIncludesPointwise (boundary : UniformGenerationBoundary)
    (presentation : boundary.Presentation) :
    UniformlyGenerates boundary presentation → PointwiseGenerates boundary presentation := by
  intro uniform target protectedTarget
  exact uniform presentation (boundary.extendsRefl presentation) target protectedTarget

namespace AtomizationFoil

def Target : Bool → Type
  | false => PUnit
  | true => Bool

def Extends (first second : Bool) : Prop := first = false ∨ first = second

def transportTarget {first second : Bool} (extension : Extends first second) : Target first → Target second := by
  cases first <;> cases second
  · exact fun _ => PUnit.unit
  · exact fun _ => false
  · exact fun _ => PUnit.unit
  · exact fun target => target

def Construction : (presentation : Bool) → Target presentation → Type
  | false, _ => PUnit
  | true, false => PUnit
  | true, true => PEmpty

def boundary : UniformGenerationBoundary where
  Presentation := Bool
  Extends := Extends
  extendsRefl := fun presentation => Or.inr rfl
  extendsTrans := by
    intro first second third firstExtension secondExtension
    rcases firstExtension with firstFalse | firstSecond
    · exact Or.inl firstFalse
    · subst second
      exact secondExtension
  Target := Target
  Protected := fun _ => True
  Construction := Construction
  Source := fun _ _ => PUnit
  Corresponds := fun _ _ => True
  Covered := fun _ => True
  transportTarget := transportTarget
  transportProtected := by intros; trivial
  transportWitness := by
    intro first second extension target witness
    cases first <;> cases second
    · exact ⟨PUnit.unit, PUnit.unit, trivial, trivial⟩
    · exact ⟨PUnit.unit, PUnit.unit, trivial, trivial⟩
    · exfalso
      rcases extension with impossible | impossible <;> contradiction
    · exact witness
  transportTarget_id := by
    intro presentation target
    cases presentation <;> cases target <;> rfl
  transportTarget_comp := by
    intro first second third firstExtension secondExtension target
    cases first <;> cases second <;> cases third <;> cases target <;>
      simp [Extends, transportTarget] at firstExtension secondExtension ⊢
    all_goals rfl

theorem atomizedCurrentTargetsArePointwise : PointwiseGenerates boundary false := by
  intro target _
  exact ⟨⟨PUnit.unit, PUnit.unit, trivial, trivial⟩⟩

theorem freshProtectedTargetBreaksUniformGeneration : ¬ UniformlyGenerates boundary false := by
  intro uniform
  have generated := uniform true (Or.inl rfl) true trivial
  rcases generated with ⟨witness⟩
  exact witness.construction.elim

theorem pointwiseGenerationDoesNotImplyUniformGeneration :
    PointwiseGenerates boundary false ∧ ¬ UniformlyGenerates boundary false :=
  ⟨atomizedCurrentTargetsArePointwise, freshProtectedTargetBreaksUniformGeneration⟩

end AtomizationFoil

inductive UniformGenerationObligation where
  | instantiateAmbientPresentations
  | mapEveryProtectedPredecessorTarget
  | proveSourceCorrespondence
  | proveCoverage
  | constructFourSemanticAblations
  | noBasisPromotionFromBoundaryAlone
  deriving DecidableEq, Repr

end InquiryCalculus.Successor.Ambient

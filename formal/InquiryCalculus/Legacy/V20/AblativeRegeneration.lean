import InquiryCalculus.Legacy.V20.Understanding
import InquiryCalculus.Legacy.V20.SolutionFieldWeb

/-! # Ablative regeneration test

Source-bound realization of v2.0 lines 4230–4246.  Ablation retains the whole
presentation and exposes typed open occurrences.  Regeneration is the complete
solution predicate; direct protection is singletonhood only after quotienting by
a supplied protected equivalence.  No filler is selected or executed.
-/
namespace InquiryCalculus.Legacy.V20.AblativeRegeneration

universe u

/-- The retained relational presentation around prospective holes. -/
structure RetainedPresentation (Occurrence Relation Form Port : Type u) where
  occurrenceForm : Occurrence → Form
  protectedOccurrence : Occurrence → Prop
  portAt : Occurrence → Port
  accepts : Port → Form → Prop
  governing : Relation → Occurrence → Form → Prop

/-- A shaped hole retains the whole source presentation and names the form whose
protected occurrences become open. -/
structure ShapedAblation (Occurrence Relation Form Port : Type u) where
  source : RetainedPresentation Occurrence Relation Form Port
  targetForm : Form

def ShapedAblation.opened {Occurrence Relation Form Port : Type u}
    (ablation : ShapedAblation Occurrence Relation Form Port)
    (occurrence : Occurrence) : Prop :=
  ablation.source.protectedOccurrence occurrence ∧
    ablation.source.occurrenceForm occurrence = ablation.targetForm

/-- Every filler must satisfy the typed port and every retained governing
relation at every opened occurrence.  This is a field, not a chosen result. -/
def solutionField {Occurrence Relation Form Port : Type u}
    (ablation : ShapedAblation Occurrence Relation Form Port) : Form → Prop :=
  fun filler => ∀ occurrence, ablation.opened occurrence →
    ablation.source.accepts (ablation.source.portAt occurrence) filler ∧
      ∀ relation, ablation.source.governing relation occurrence filler

/-- Exact source equation `Regen_x(M) = Sol_(Ablate_x(M))`. -/
def regenerationField {Occurrence Relation Form Port : Type u}
    (ablation : ShapedAblation Occurrence Relation Form Port) : Form → Prop :=
  solutionField ablation

theorem regenerationFieldIffSolutionField
    {Occurrence Relation Form Port : Type u}
    (ablation : ShapedAblation Occurrence Relation Form Port) (filler : Form) :
    regenerationField ablation filler ↔ solutionField ablation filler := Iff.rfl

theorem ablationRetainsGoverningRelations
    {Occurrence Relation Form Port : Type u}
    (ablation : ShapedAblation Occurrence Relation Form Port) :
    ablation.source.governing = ablation.source.governing := rfl

/-- The quotient relation and all equivalence laws are supplied by the protected
horizon rather than inferred from the solution field. -/
structure ProtectedEquivalence (Form : Type u) where
  relation : Form → Form → Prop
  reflexive : ∀ form, relation form form
  symmetric : ∀ {left right}, relation left right → relation right left
  transitive : ∀ {left middle right}, relation left middle → relation middle right →
    relation left right

def ProtectedEquivalence.toSetoid {Form : Type u}
    (protection : ProtectedEquivalence Form) : Setoid Form where
  r := protection.relation
  iseqv := ⟨protection.reflexive, protection.symmetric, protection.transitive⟩

def survivingProtectedClass {Form : Type u} (field : Form → Prop)
    (protection : ProtectedEquivalence Form)
    (quotientClass : Quotient protection.toSetoid) : Prop :=
  ∃ filler, field filler ∧ Quotient.mk protection.toSetoid filler = quotientClass

/-- The surviving quotient field is exactly the singleton class of the target.
Raw solution cardinality is deliberately absent from this definition. -/
def ProtectedQuotientSingleton {Form : Type u} (field : Form → Prop)
    (protection : ProtectedEquivalence Form) (target : Form) : Prop :=
  ∀ quotientClass, survivingProtectedClass field protection quotientClass ↔
    quotientClass = Quotient.mk protection.toSetoid target

namespace Countermodel

def presentation : RetainedPresentation Unit Unit Bool Unit where
  occurrenceForm := fun _ => false
  protectedOccurrence := fun _ => True
  portAt := fun _ => ()
  accepts := fun _ _ => True
  governing := fun _ _ _ => True

def hole : ShapedAblation Unit Unit Bool Unit where
  source := presentation
  targetForm := false

theorem occurrenceIsOpened : hole.opened () := ⟨trivial, rfl⟩

theorem falseIsSolution : regenerationField hole false := by
  intro _ _
  exact ⟨trivial, fun _ => trivial⟩

theorem trueIsSolution : regenerationField hole true := by
  intro _ _
  exact ⟨trivial, fun _ => trivial⟩

theorem twoRawSolutions :
    regenerationField hole false ∧ regenerationField hole true ∧ false ≠ true :=
  ⟨falseIsSolution, trueIsSolution, by decide⟩

def allEquivalent : ProtectedEquivalence Bool where
  relation := fun _ _ => True
  reflexive := fun _ => trivial
  symmetric := fun _ => trivial
  transitive := fun _ _ => trivial

theorem twoRawOneProtectedClass :
    ProtectedQuotientSingleton (regenerationField hole) allEquivalent false := by
  intro quotientClass
  constructor
  · rintro ⟨filler, _, rfl⟩
    exact Quotient.sound trivial
  · intro same
    exact ⟨false, falseIsSolution, same.symm⟩

def equalityProtected : ProtectedEquivalence Bool where
  relation := Eq
  reflexive := fun _ => rfl
  symmetric := Eq.symm
  transitive := Eq.trans

theorem twoRawTwoProtectedClasses :
    ¬ ProtectedQuotientSingleton (regenerationField hole) equalityProtected false := by
  intro singleton
  have survivingTrue : survivingProtectedClass (regenerationField hole)
      equalityProtected (Quotient.mk equalityProtected.toSetoid true) :=
    ⟨true, trueIsSolution, rfl⟩
  have classesEqual := (singleton _).mp survivingTrue
  have valuesEqual : true = false := Quotient.exact classesEqual
  exact Bool.noConfusion valuesEqual

end Countermodel
end InquiryCalculus.Legacy.V20.AblativeRegeneration

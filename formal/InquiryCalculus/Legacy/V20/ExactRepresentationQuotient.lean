import InquiryCalculus.Legacy.V20.ApplicabilitySupportSeparation

/-! # Exact representation quotient

Source-bound realization of v2.0 lines 4638–4664. A proposed quotient is a supplied map
whose equal images must preserve a supplied protected equivalence. The reverse direction
(coarseness) and continuation descent remain separate obligations.
-/
namespace InquiryCalculus.Legacy.V20.ExactRepresentationQuotient

universe u v

structure ProposedQuotient (Source : Type u) (Target : Type v) where
  map : Source → Target

structure QuotientContext (Source : Type u) (Target : Type v) where
  protectedEquivalent : Source → Source → Prop

def ConsequenceSufficient {Source : Type u} {Target : Type v}
    (context : QuotientContext Source Target) (quotient : ProposedQuotient Source Target) : Prop :=
  ∀ left right, quotient.map left = quotient.map right → context.protectedEquivalent left right

def CoarsestCharacterization {Source : Type u} {Target : Type v}
    (context : QuotientContext Source Target) (quotient : ProposedQuotient Source Target) : Prop :=
  ∀ left right, quotient.map left = quotient.map right ↔ context.protectedEquivalent left right

def TestedNondistinction {Source : Type u} (tested : Source → Prop)
    (context : QuotientContext Source Unit) : Prop :=
  ∀ left right, tested left → tested right → context.protectedEquivalent left right

theorem coarsestImpliesConsequenceSufficient {Source : Type u} {Target : Type v}
    {context : QuotientContext Source Target} {quotient : ProposedQuotient Source Target}
    (coarsest : CoarsestCharacterization context quotient) : ConsequenceSufficient context quotient := by
  intro left right equalImages
  exact (coarsest left right).mp equalImages

namespace Countermodel

inductive Source where | a | b | c deriving DecidableEq
inductive ExactTarget where | ab | c deriving DecidableEq
inductive FineTarget where | a | b | c deriving DecidableEq
inductive UnitTarget where | only deriving DecidableEq

def equivalent (left right : Source) : Prop :=
  (left = .a ∧ right = .a) ∨ (left = .a ∧ right = .b) ∨
    (left = .b ∧ right = .a) ∨ (left = .b ∧ right = .b) ∨
    (left = .c ∧ right = .c)

def exactMap : ProposedQuotient Source ExactTarget where
  map := fun source => if source = .a ∨ source = .b then .ab else .c

def fineMap : ProposedQuotient Source FineTarget where
  map := fun source => match source with | .a => .a | .b => .b | .c => .c

def overcoarseMap : ProposedQuotient Source UnitTarget where
  map := fun _ => .only

def exactContext : QuotientContext Source ExactTarget where
  protectedEquivalent := equivalent

def fineContext : QuotientContext Source FineTarget where
  protectedEquivalent := equivalent

def overcoarseContext : QuotientContext Source UnitTarget where
  protectedEquivalent := equivalent

def testedContext : QuotientContext Source Unit where
  protectedEquivalent := equivalent

def testedAB (source : Source) : Prop := source = .a ∨ source = .b

theorem exactMapIsConsequenceSufficient : ConsequenceSufficient exactContext exactMap := by
  intro left right equalImages
  cases left <;> cases right
  · exact Or.inl ⟨rfl, rfl⟩
  · exact Or.inr (Or.inl ⟨rfl, rfl⟩)
  · change ExactTarget.ab = ExactTarget.c at equalImages
    exact ExactTarget.noConfusion equalImages
  · exact Or.inr (Or.inr (Or.inl ⟨rfl, rfl⟩))
  · exact Or.inr (Or.inr (Or.inr (Or.inl ⟨rfl, rfl⟩)))
  · change ExactTarget.ab = ExactTarget.c at equalImages
    exact ExactTarget.noConfusion equalImages
  · change ExactTarget.c = ExactTarget.ab at equalImages
    exact ExactTarget.noConfusion equalImages
  · change ExactTarget.c = ExactTarget.ab at equalImages
    exact ExactTarget.noConfusion equalImages
  · exact Or.inr (Or.inr (Or.inr (Or.inr ⟨rfl, rfl⟩)))

theorem overcoarseMapFailsConsequenceSufficiency :
    ¬ ConsequenceSufficient overcoarseContext overcoarseMap := by
  intro sufficient
  have invalid := sufficient .a .c rfl
  rcases invalid with invalid | invalid | invalid | invalid | invalid <;>
    first | exact Source.noConfusion invalid.1 | exact Source.noConfusion invalid.2

theorem fineMapIsConsequenceSufficient : ConsequenceSufficient fineContext fineMap := by
  intro left right equalImages
  cases left <;> cases right
  · exact Or.inl ⟨rfl, rfl⟩
  · change FineTarget.a = FineTarget.b at equalImages
    exact FineTarget.noConfusion equalImages
  · change FineTarget.a = FineTarget.c at equalImages
    exact FineTarget.noConfusion equalImages
  · change FineTarget.b = FineTarget.a at equalImages
    exact FineTarget.noConfusion equalImages
  · exact Or.inr (Or.inr (Or.inr (Or.inl ⟨rfl, rfl⟩)))
  · change FineTarget.b = FineTarget.c at equalImages
    exact FineTarget.noConfusion equalImages
  · change FineTarget.c = FineTarget.a at equalImages
    exact FineTarget.noConfusion equalImages
  · change FineTarget.c = FineTarget.b at equalImages
    exact FineTarget.noConfusion equalImages
  · exact Or.inr (Or.inr (Or.inr (Or.inr ⟨rfl, rfl⟩)))

theorem fineMapIsNotCoarsest : ¬ CoarsestCharacterization fineContext fineMap := by
  intro coarsest
  have equivalentAB := (coarsest .a .b).mpr (Or.inr (Or.inl ⟨rfl, rfl⟩))
  change FineTarget.a = FineTarget.b at equivalentAB
  exact FineTarget.noConfusion equivalentAB

/-- This finite test relation is evidence at its declared coverage, not universal equivalence. -/
theorem testedABIsNondistinct : TestedNondistinction testedAB testedContext := by
  intro left right leftTested rightTested
  rcases leftTested with leftA | leftB <;> rcases rightTested with rightA | rightB
  · exact Or.inl ⟨leftA, rightA⟩
  · exact Or.inr (Or.inl ⟨leftA, rightB⟩)
  · exact Or.inr (Or.inr (Or.inl ⟨leftB, rightA⟩))
  · exact Or.inr (Or.inr (Or.inr (Or.inl ⟨leftB, rightB⟩)))

end Countermodel
end InquiryCalculus.Legacy.V20.ExactRepresentationQuotient

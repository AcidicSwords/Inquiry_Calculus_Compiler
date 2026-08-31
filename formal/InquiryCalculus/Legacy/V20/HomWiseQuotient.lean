import InquiryCalculus.Legacy.V20.ProtectedBehavioralEquivalenceLaws

/-! # Hom-wise quotients and conditional composition

Downstream realization of v2.0 lines 4061–4106 under a supplied observation
signature. Source ambiguities retain their classifications. A total operation
on one typed composable triple is supplied, not obtained from the quotient.
Partial composition regimes, recurrent operator descent, executors, and
successor promotion are outside this boundary.
-/
namespace InquiryCalculus.Legacy.V20.HomWiseQuotient

open ProtectedBehavioralEquivalenceLaws

universe u
variable {Object : Type u} {Term : Object → Object → Type u}
variable (S : ProtectedBehavioralEquivalenceContext Object Term)
variable (H : (A B : Object) → S.Context A B → Prop)

def protectedSetoid (A B : Object) : Setoid (Term A B) where
  r := protectedEquivalenceDefinitionShape S (H A B)
  iseqv := ⟨equivalenceReflexive S (H A B),
    fun {f g} => equivalenceSymmetric S (H A B) f g,
    fun {f g h} => equivalenceTransitive S (H A B) f g h⟩

def HomQuotient (A B : Object) : Type u := Quotient (protectedSetoid S H A B)

def quotientMap {A B : Object} (f : Term A B) : HomQuotient S H A B :=
  Quotient.mk (protectedSetoid S H A B) f

theorem quotientMapSound {A B : Object} (f g : Term A B)
    (equal : protectedEquivalenceDefinitionShape S (H A B) f g) :
    quotientMap S H f = quotientMap S H g := Quotient.sound equal

theorem quotientMapExact {A B : Object} (f g : Term A B)
    (equal : quotientMap S H f = quotientMap S H g) :
    protectedEquivalenceDefinitionShape S (H A B) f g := Quotient.exact equal

theorem quotientMapEqIff {A B : Object} (f g : Term A B) :
    quotientMap S H f = quotientMap S H g ↔
      protectedEquivalenceDefinitionShape S (H A B) f g :=
  ⟨quotientMapExact S H f g, quotientMapSound S H f g⟩

/-- This is a property of the supplied operation at three supplied hom horizons,
not an automatic consequence of equivalence being an equivalence relation. -/
def Congruent {A B C : Object} (compose : Term B C → Term A B → Term A C) : Prop :=
  ∀ g g' f f', protectedEquivalenceDefinitionShape S (H B C) g g' →
    protectedEquivalenceDefinitionShape S (H A B) f f' →
    protectedEquivalenceDefinitionShape S (H A C) (compose g f) (compose g' f')

def descendedComposition {A B C : Object}
    (compose : Term B C → Term A B → Term A C) (respects : Congruent S H compose)
    (g : HomQuotient S H B C) (f : HomQuotient S H A B) : HomQuotient S H A C :=
  Quotient.liftOn₂ g f (fun g f => quotientMap S H (compose g f))
    (fun g f g' f' eg ef => quotientMapSound S H _ _ (respects g g' f f' eg ef))

theorem descendedCompositionOnRepresentatives {A B C : Object}
    (compose : Term B C → Term A B → Term A C) (respects : Congruent S H compose)
    (g : Term B C) (f : Term A B) :
    descendedComposition S H compose respects (quotientMap S H g) (quotientMap S H f) =
      quotientMap S H (compose g f) := rfl

/-- The reverse condition is checked independently of the lifting construction. -/
theorem descentRequiresCongruence {A B C : Object}
    (compose : Term B C → Term A B → Term A C)
    (descended : HomQuotient S H B C → HomQuotient S H A B → HomQuotient S H A C)
    (commutes : ∀ g f, descended (quotientMap S H g) (quotientMap S H f) =
      quotientMap S H (compose g f)) : Congruent S H compose := by
  intro g g' f f' eg ef
  apply quotientMapExact S H
  have eqg := quotientMapSound S H g g' eg
  have eqf := quotientMapSound S H f f' ef
  have together : descended (quotientMap S H g) (quotientMap S H f) =
      descended (quotientMap S H g') (quotientMap S H f') := by
    rw [eqg, eqf]
  exact (commutes g f).symm.trans (together.trans (commutes g' f'))

theorem descentIffCongruence {A B C : Object}
    (compose : Term B C → Term A B → Term A C) :
    (∃ descended : HomQuotient S H B C → HomQuotient S H A B → HomQuotient S H A C,
      ∀ g f, descended (quotientMap S H g) (quotientMap S H f) =
        quotientMap S H (compose g f)) ↔ Congruent S H compose := by
  constructor
  · intro witness
    obtain ⟨descended, commutes⟩ := witness
    exact descentRequiresCongruence S H compose descended commutes
  · intro respects
    exact ⟨descendedComposition S H compose respects,
      descendedCompositionOnRepresentatives S H compose respects⟩

namespace Countermodel

inductive Term3 where
  | ordinary | hidden | visible

open Term3

def observe : Term3 → Bool
  | ordinary => false
  | hidden => false
  | visible => true

def observation : ProtectedBehavioralEquivalenceContext Unit (fun _ _ => Term3) where
  Context := fun _ _ => Unit
  Consequence := fun _ => Bool
  consequence := fun _ term => observe term

def horizon (_ _ : Unit) (_ : Unit) : Prop := True

def compose (_ : Term3) : Term3 → Term3
  | ordinary => ordinary
  | hidden => visible
  | visible => visible

theorem equivalentRepresentatives :
    protectedEquivalenceDefinitionShape observation (A := ()) (B := ())
      (horizon () ()) ordinary hidden := by
  intro _ _
  rfl

theorem separatedComposites :
    ¬ protectedEquivalenceDefinitionShape observation (A := ()) (B := ()) (horizon () ())
      (compose ordinary ordinary) (compose ordinary hidden) := by
  intro equal
  exact Bool.noConfusion (equal () True.intro)

theorem notCongruent : ¬ Congruent observation horizon (A := ()) (B := ()) (C := ()) compose := by
  intro respects
  exact separatedComposites (respects ordinary ordinary ordinary hidden
    (equivalenceReflexive observation (horizon () ()) ordinary) equivalentRepresentatives)

theorem noDescent : ¬ ∃ descended : HomQuotient observation horizon () () →
    HomQuotient observation horizon () () → HomQuotient observation horizon () (),
    ∀ g f, descended (quotientMap observation horizon g) (quotientMap observation horizon f) =
      quotientMap observation horizon (compose g f) := by
  intro witness
  exact notCongruent ((descentIffCongruence observation horizon compose).mp witness)

end Countermodel
end InquiryCalculus.Legacy.V20.HomWiseQuotient

/-!
# Ambient metalanguage boundary

Lean's dependent type theory supplies `Type`, `Prop`, equality, functions, dependent functions,
dependent pairs, inductive families, quantification, and logic. This module introduces no competing
Inquiry Calculus versions of those ambient forms.

The identity results are intentionally structural. They demonstrate that the host already supplies
the forms named below, without introducing Inquiry Calculus copies of them or claiming that the
successor's semantic primitive basis has been discovered.

The v2.0 prose at source identity `PRED-TEX-PROSE-983F2B30F7C1C1D2` calls its metalanguage
classical set-theoretic. This module does not silently strengthen Lean with classical axioms. That
predecessor claim remains an explicit Phase B obligation until its exact use sites are elaborated.
-/

namespace InquiryCalculus.Meta

universe u v

theorem ambient_type_identity (α : Type u) : α = α := rfl

theorem ambient_proposition_identity (p : Prop) : p ↔ p := Iff.rfl

theorem ambient_equality_identity {α : Type u} (x : α) : x = x := rfl

theorem ambient_function_identity {α : Type u} {β : Type v} (f : α → β) :
    (fun x => f x) = f := rfl

theorem ambient_dependent_function_identity {α : Type u} {β : α → Type v}
    (f : (x : α) → β x) : (fun x => f x) = f := rfl

theorem ambient_dependent_pair_identity {α : Type u} {β : α → Type v}
    (p : Sigma β) : p = p := rfl

theorem ambient_universal_quantifier_identity {α : Type u} (p : α → Prop) :
    (∀ x, p x) ↔ ∀ x, p x := Iff.rfl

theorem ambient_existential_quantifier_identity {α : Type u} (p : α → Prop) :
    (∃ x, p x) ↔ ∃ x, p x := Iff.rfl

end InquiryCalculus.Meta

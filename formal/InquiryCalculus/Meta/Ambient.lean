/-!
# Ambient metalanguage boundary

Lean's dependent type theory supplies `Type`, `Prop`, equality, functions, dependent functions,
dependent pairs, inductive families, quantification, and logic. This module introduces no competing
Inquiry Calculus versions of those ambient forms.

The identity result is intentionally small: it makes the initial module executable without claiming
that the successor's semantic primitive basis has already been discovered.
-/

namespace InquiryCalculus.Meta

theorem ambient_proposition_identity (p : Prop) : p ↔ p := Iff.rfl

end InquiryCalculus.Meta

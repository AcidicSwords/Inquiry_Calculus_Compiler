import InquiryCalculus.Legacy.V20.SolutionFieldWeb

/-! # v2.0 protected-determination boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The declared horizon-relative observational equivalence used by protected determination. -/
structure HorizonEquivalenceSyntax (X : Type u) where
  equivalent : X → X → Prop

/-- Every surviving filler lies in the representative's declared horizon-equivalence class. -/
def protectedDetermines {X : Type u} (horizon : HorizonEquivalenceSyntax X)
    (field : X → Prop) (representative : X) : Prop :=
  ∀ survivor, field survivor → horizon.equivalent survivor representative

/-- Literal uniqueness, global equivalence, and answer or solver meanings remain open. -/
inductive ProtectedDeterminationObligation where
  | horizonEquivalenceLaws
  | literalUniqueness
  | globalEquivalence
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

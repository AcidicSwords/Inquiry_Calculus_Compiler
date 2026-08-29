import InquiryCalculus.Legacy.V20.RelationalAbstraction

/-! # v2.0 abstraction-by-removal boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- Candidate regenerative removal retains the abstraction carrier rather than deleting its web. -/
structure AbstractionByRemovalSyntax (X : Type u) where
  abstraction : RelationalAbstractionSyntax X

/-- The removal's constraint web is retained by construction. -/
theorem abstractionByRemoval_retains_constraints {X : Type u}
    (removal : AbstractionByRemovalSyntax X) :
    removal.abstraction.web = removal.abstraction.web := rfl

/-- Regeneration, lawful refill, and destructive-deletion comparison remain open obligations. -/
inductive AbstractionByRemovalObligation where
  | removesFilling
  | preservesConstrainingRelations
  | regenerativeAbstraction
  | lawfulRefill
  | destructiveDeletionContrast
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

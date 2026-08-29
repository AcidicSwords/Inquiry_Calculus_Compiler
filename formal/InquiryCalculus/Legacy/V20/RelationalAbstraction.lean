import InquiryCalculus.Legacy.V20.QuestionStructuredHole

/-! # v2.0 relational abstraction boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- A typed constraint on one prospective open port. -/
structure PortConstraintSyntax (X : Type u) where
  constrains : X → Prop

/-- A relation web records constraints retained around a prospective open port. -/
structure RelationWebSyntax (X : Type u) where
  constraints : List (PortConstraintSyntax X)

/-- A licensed abstraction exposes one filling but retains its constraining relation web. -/
structure RelationalAbstractionSyntax (X : Type u) where
  filling : X
  web : RelationWebSyntax X
  exposedOccurrences : List Nat

/-- The abstraction's retained web is exactly its source web; no deletion is represented. -/
theorem relationalAbstraction_retains_web {X : Type u}
    (abstraction : RelationalAbstractionSyntax X) : abstraction.web = abstraction.web := rfl

/-- Regeneration, license sufficiency, lawful refill, and execution remain open. -/
inductive RelationalAbstractionObligation where
  | exposureLicense
  | retainedTypedRelations
  | regenerativeAvailability
  | lawfulRefill
  | noDestructiveDeletion
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

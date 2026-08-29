import InquiryCalculus.Legacy.V20.MinimalLogicalBasis

/-!
# v2.0 data-only relation-expression IR boundary

The three source records at `PRED-TEX-PROSE-B9B6C5BB3A711682`,
`PRED-TEX-DISPLAY-D6B8D037AB8E7E7F`, and `PRED-TEX-PROSE-638BA7EFC5FE6077` remain Ambiguous
LegacyObligations. This file preserves candidate IR grammar only. It does not define denotation,
join, substitution, hiding, renaming, guarded restriction, or a concrete semantic question.
-/

namespace InquiryCalculus.Legacy.V20

/-- A syntax-only binding token for candidate relation-expression IR. -/
structure IRBindingToken (B : Binding) (I : TypeInterpretation B) where
  index : Nat

/-- A syntax-only finite selection of port positions. -/
structure PortSelection where
  indices : List Nat

/-- A syntax-only finite port-renaming map. -/
structure PortRenaming where
  pairs : List (Nat × Nat)

/-- A syntax-only guard coordinate; it is not a restriction semantics. -/
structure IRGuard (B : Binding) (I : TypeInterpretation B) where
  formula : CandidateFormulaSyntax B I

/-- The displayed data-only relation-expression grammar as a candidate syntax only. -/
inductive RelationExpressionIR (B : Binding) (I : TypeInterpretation B) where
  | relation : B.relationToken → RelationExpressionIR B I
  | bind : RelationExpressionIR B I → IRBindingToken B I → RelationExpressionIR B I
  | join : RelationExpressionIR B I → RelationExpressionIR B I → RelationExpressionIR B I
  | expose : PortSelection → RelationExpressionIR B I → RelationExpressionIR B I
  | hide : PortSelection → RelationExpressionIR B I → RelationExpressionIR B I
  | rename : PortRenaming → RelationExpressionIR B I → RelationExpressionIR B I
  | guard : IRGuard B I → RelationExpressionIR B I → RelationExpressionIR B I

/-- The three source claims remain obligations rather than IR semantics or question formation. -/
inductive RelationExpressionIRObligation where
  | dataOnlyIntroduction
  | displayedExpressionGrammar
  | inheritedDenotationAndSemanticQuestion
  deriving DecidableEq, Repr

theorem relation_expression_ir_has_no_denotation_claim (B : Binding) (I : TypeInterpretation B)
    (expression : RelationExpressionIR B I) : expression = expression := rfl

end InquiryCalculus.Legacy.V20

import InquiryCalculus.Legacy.V20.Refinement

/-!
# v2.0 formula grammar boundary

The six source records at `PRED-TEX-PROSE-A08A2468E65A2DAB`,
`PRED-TEX-DISPLAY-75C44CA222A4214E`, `PRED-TEX-PROSE-983F2B30F7C1C1D2`,
`PRED-TEX-PROSE-DA438851DA28D75D`, `PRED-TEX-DISPLAY-212A4128A7A58270`, and
`PRED-TEX-PROSE-35B2FF04F304464B` are LegacyObligations. This file preserves a typed candidate
syntax only. It neither accepts a successor logic nor introduces oriented negation, coverage,
departure, question, probe, or program semantics.
-/

namespace InquiryCalculus.Legacy.V20

/-- A binding-indexed variable token for candidate formula syntax. -/
structure FormulaVariable (B : Binding) (I : TypeInterpretation B) where
  type : AdmittedType B I
  index : Nat

/-- Candidate terms preserve either a typed represented form or a typed bound variable. -/
inductive CandidateTerm (B : Binding) (I : TypeInterpretation B) where
  | represented : Form B I → CandidateTerm B I
  | variable : FormulaVariable B I → CandidateTerm B I

/-- Candidate atoms retain relation-token and represented-form equality shapes without denotation. -/
inductive CandidateAtom (B : Binding) (I : TypeInterpretation B) where
  | relation : B.relationToken → List (CandidateTerm B I) → CandidateAtom B I
  | equality : CandidateTerm B I → CandidateTerm B I → CandidateAtom B I

/-- The displayed predecessor grammar as a typed candidate syntax, not accepted successor meaning. -/
inductive CandidateFormulaSyntax (B : Binding) (I : TypeInterpretation B) where
  | truth : CandidateFormulaSyntax B I
  | falsity : CandidateFormulaSyntax B I
  | atom : CandidateAtom B I → CandidateFormulaSyntax B I
  | and : CandidateFormulaSyntax B I → CandidateFormulaSyntax B I → CandidateFormulaSyntax B I
  | or : CandidateFormulaSyntax B I → CandidateFormulaSyntax B I → CandidateFormulaSyntax B I
  | implies : CandidateFormulaSyntax B I → CandidateFormulaSyntax B I → CandidateFormulaSyntax B I
  | logicalNot : CandidateFormulaSyntax B I → CandidateFormulaSyntax B I
  | exists : FormulaVariable B I → CandidateFormulaSyntax B I → CandidateFormulaSyntax B I
  | forall : FormulaVariable B I → CandidateFormulaSyntax B I → CandidateFormulaSyntax B I

/-- Every source claim at this boundary remains an obligation rather than a logic selection. -/
inductive FormulaGrammarObligation where
  | referenceLanguageIntroduction
  | displayedGrammar
  | classicalityBindingSelection
  | logicalNegationSeparation
  | orientedRelationSectionAndFilling
  | coverageAndDepartureEvidence
  deriving DecidableEq, Repr

theorem candidate_formula_has_no_denotation_claim (B : Binding) (I : TypeInterpretation B)
    (phi : CandidateFormulaSyntax B I) : phi = phi := rfl

end InquiryCalculus.Legacy.V20

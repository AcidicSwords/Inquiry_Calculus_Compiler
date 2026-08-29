import InquiryCalculus.Legacy.V20.Types

/-!
# v2.0 represented forms

`PRED-TEX-DECL-4D609DF7B1B2239E` supplies the dependent typed form carrier;
`PRED-TEX-DECL-419C4E046A257830` supplies reification; and
`PRED-TEX-DECL-711F0AAA2722B10B` supplies partial operational interpretation.  No relation,
question, grain, ancestry, or execution semantics is introduced here.
-/

namespace InquiryCalculus.Legacy.V20

/-- An admitted type code keeps its binding-relative availability proof. -/
def AdmittedType (B : Binding) (I : TypeInterpretation B) :=
  { A : TypeCode B // I.admissible A }

/-- The dependent binding-local carrier of typed represented forms. -/
structure Form (B : Binding) (I : TypeInterpretation B) where
  type : AdmittedType B I
  value : I.realize type.1 type.2

/-- A typed operator is only a binding-supplied token until relations are elaborated. -/
structure OperatorTerm (B : Binding) (I : TypeInterpretation B)
    (domain codomain : AdmittedType B I) where
  token : B.generatorToken

/-- Reification is an explicitly supplied typed map, not an assumed inverse. -/
structure ReificationBoundary (B : Binding) (I : TypeInterpretation B) where
  reify : {domain codomain : AdmittedType B I} →
    OperatorTerm B I domain codomain → Form B I

/-- A binding-supplied operator role for one represented form. -/
structure OperationalRole (B : Binding) (I : TypeInterpretation B) where
  domain : AdmittedType B I
  codomain : AdmittedType B I
  operator : OperatorTerm B I domain codomain

/-- Operational interpretation remains partial and is not required to invert reification. -/
structure OperationalInterpretationBoundary (B : Binding) (I : TypeInterpretation B) where
  defined : Form B I → Prop
  interpret : (z : Form B I) → defined z → OperationalRole B I

theorem form_retains_its_admitted_type (B : Binding) (I : TypeInterpretation B)
    (z : Form B I) : z.type = z.type := rfl

theorem reification_has_a_form_target (B : Binding) (I : TypeInterpretation B)
    (R : ReificationBoundary B I) {domain codomain : AdmittedType B I}
    (f : OperatorTerm B I domain codomain) : ∃ z : Form B I, z = R.reify f :=
  ⟨R.reify f, rfl⟩

/-- Nearby form claims remain explicit rather than becoming relation semantics in this layer. -/
inductive RepresentedFormObligation where
  | roleClosureWithoutTypeCollapse
  | constructionAncestryRecomposition
  | grainRelativeOpening
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

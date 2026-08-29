import InquiryCalculus.Legacy.V20.Forms

/-!
# v2.0 typed relations

This source-bound predecessor surface elaborates the explicit relation definitions at
`PRED-TEX-DECL-3CA4350181076F20`, `PRED-TEX-DECL-D7E40B5FD5FF51AC`,
`PRED-TEX-DECL-79831E897328F3CD`, `PRED-TEX-DECL-D2CA560D7E745992`,
`PRED-TEX-DECL-9CA6ACF74CF9A5C1`, `PRED-TEX-DECL-502D7BD7B16BC721`, and
`PRED-TEX-DECL-1CC3B1DE1920C0E3`. It does not elaborate formulas, questions, fibers,
programs, contextual equivalence, or a successor relation basis.
-/

namespace InquiryCalculus.Legacy.V20

/-- A finite typed port context for an n-ary relation schema. -/
inductive RelationPorts (B : Binding) (I : TypeInterpretation B) where
  | empty : RelationPorts B I
  | extend : RelationPorts B I → AdmittedType B I → RelationPorts B I

/-- The dependent tuple realized by a finite typed port context. -/
def RelationPorts.realize {B : Binding} {I : TypeInterpretation B} :
    RelationPorts B I → Type
  | .empty => PUnit
  | .extend ports A => ports.realize × I.realize A.1 A.2

/-- A binding-typed binary relation, not an untyped host predicate. -/
structure Relation (B : Binding) (I : TypeInterpretation B)
    (domain codomain : AdmittedType B I) where
  holds : I.realize domain.1 domain.2 → I.realize codomain.1 codomain.2 → Prop

/-- A binding-typed relation schema over a finite list of typed ports. -/
structure RelationSchema (B : Binding) (I : TypeInterpretation B)
    (ports : RelationPorts B I) where
  holds : ports.realize → Prop

/-- The total-single-valued condition retained separately from a binary relation. -/
def Relation.isFunctional {B : Binding} {I : TypeInterpretation B}
    {domain codomain : AdmittedType B I} (R : Relation B I domain codomain) : Prop :=
  ∀ a, ∃ b, R.holds a b ∧ ∀ b', R.holds a b' → b' = b

/-- A function is a relation together with, rather than a replacement by, its source condition. -/
structure FunctionRelation (B : Binding) (I : TypeInterpretation B)
    (domain codomain : AdmittedType B I) where
  relation : Relation B I domain codomain
  totalSingleValued : relation.isFunctional

/-- The typed identity relation. -/
def identityRelation (B : Binding) (I : TypeInterpretation B) (A : AdmittedType B I) :
    Relation B I A A where
  holds a a' := a = a'

/-- Serial composition keeps the typed mediator explicit in its existential witness. -/
def serialCompose {B : Binding} {I : TypeInterpretation B}
    {A D C : AdmittedType B I} (R : Relation B I A D) (S : Relation B I D C) :
    Relation B I A C where
  holds a c := ∃ d, R.holds a d ∧ S.holds d c

/-- Converse is available only through a representation-supplied admissibility boundary. -/
structure ConverseBoundary (B : Binding) (I : TypeInterpretation B) where
  defined : {domain codomain : AdmittedType B I} → Relation B I domain codomain → Prop
  converse : {domain codomain : AdmittedType B I} →
    (R : Relation B I domain codomain) → defined R → Relation B I codomain domain

/-- A typed subset of an admitted type realization. -/
structure TypedSubset (B : Binding) (I : TypeInterpretation B) (A : AdmittedType B I) where
  contains : I.realize A.1 A.2 → Prop

/-- The relational image of a typed subset. -/
def relationImage {B : Binding} {I : TypeInterpretation B}
    {A C : AdmittedType B I} (R : Relation B I A C) (X : TypedSubset B I A) :
    TypedSubset B I C where
  contains c := ∃ a, X.contains a ∧ R.holds a c

/-- The relational inverse image of a typed subset. -/
def relationInverseImage {B : Binding} {I : TypeInterpretation B}
    {A C : AdmittedType B I} (R : Relation B I A C) (Y : TypedSubset B I C) :
    TypedSubset B I A where
  contains a := ∃ c, Y.contains c ∧ R.holds a c

/-- Parallel typed operators share interfaces; no contextual-equivalence relation is introduced. -/
structure ParallelOperators (B : Binding) (I : TypeInterpretation B)
    (domain codomain : AdmittedType B I) where
  left : OperatorTerm B I domain codomain
  right : OperatorTerm B I domain codomain

theorem identity_relation_holds_exactly {B : Binding} {I : TypeInterpretation B}
    (A : AdmittedType B I) (a a' : I.realize A.1 A.2) :
    (identityRelation B I A).holds a a' ↔ a = a' := Iff.rfl

theorem serial_composition_has_typed_mediator {B : Binding} {I : TypeInterpretation B}
    {A D C : AdmittedType B I} (R : Relation B I A D) (S : Relation B I D C)
    (a : I.realize A.1 A.2) (c : I.realize C.1 C.2) :
    (serialCompose R S).holds a c ↔ ∃ d, R.holds a d ∧ S.holds d c := Iff.rfl

end InquiryCalculus.Legacy.V20

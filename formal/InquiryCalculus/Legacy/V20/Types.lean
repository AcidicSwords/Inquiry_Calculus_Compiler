import InquiryCalculus.Legacy.V20.Binding

/-!
# v2.0 reference type grammar

This is a binding-indexed *syntax* for the fifteen alternatives explicitly displayed at
`PRED-TEX-DISPLAY-3B6A6E6DDEC8FA50`.  It is not Lean's `Type`, nor does it assert that every native
binding realizes every constructor.  The three surrounding ambiguous source boundaries remain
values of `ReferenceTypeGrammarObligation` below.
-/

namespace InquiryCalculus.Legacy.V20

/-- A de Bruijn-free binder token.  It is a syntax coordinate, not a stringly named type. -/
structure TypeVariable (B : Binding) where
  index : Nat
  deriving DecidableEq, Repr

/-- The fifteen explicit alternatives in the predecessor reference type grammar. -/
inductive TypeCode (B : Binding) where
  | variable : TypeVariable B → TypeCode B
  | unit : TypeCode B
  | bool : TypeCode B
  | nat : TypeCode B
  | product : TypeCode B → TypeCode B → TypeCode B
  | sum : TypeCode B → TypeCode B → TypeCode B
  | sigma : TypeCode B → (TypeVariable B → TypeCode B) → TypeCode B
  | pi : TypeCode B → (TypeVariable B → TypeCode B) → TypeCode B
  | fin : TypeCode B → TypeCode B
  | list : TypeCode B → TypeCode B
  | raw : TypeCode B → TypeCode B
  | result : TypeCode B → TypeCode B
  | inquiryProgram : TypeCode B → TypeCode B
  | runtimeProgram : TypeCode B → TypeCode B
  | code : TypeCode B → TypeCode B

/-- The three source boundaries deliberately not promoted to grammar constructors. -/
inductive ReferenceTypeGrammarObligation where
  | admissionProse
  | displayedGrammarInterpretation
  | nativeBindingQualification
  deriving DecidableEq, Repr

/--
An interpretation is partial by declared admissibility.  Its result is a host `Type`, while the
argument remains a distinct predecessor code.  This makes unavailable native structures explicit.
-/
structure TypeInterpretation (B : Binding) where
  admissible : TypeCode B → Prop
  realize : (A : TypeCode B) → admissible A → Type

theorem interpretation_target_identity (B : Binding) (I : TypeInterpretation B)
    (A : TypeCode B) (h : I.admissible A) : I.realize A h = I.realize A h := rfl

/-- Program-family grammar exists before its constructors and operational semantics are elaborated. -/
inductive ProgramFamily where
  | inquirySource
  | runtime
  | reifiedCode
  deriving DecidableEq, Repr

/-- A source-bound promise for the later program/code-family elaboration. -/
structure ProgramCodeFamilyBoundary (B : Binding) where
  family : ProgramFamily
  result : TypeCode B

theorem code_is_a_type_code (B : Binding) (A : TypeCode B) :
    ∃ C : TypeCode B, C = TypeCode.code A :=
  ⟨TypeCode.code A, rfl⟩

end InquiryCalculus.Legacy.V20

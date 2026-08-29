import InquiryCalculus.Legacy.V20.RelationExpressionIR

/-!
# v2.0 relation schemas and named ports boundary

The three exact source records at `PRED-TEX-DECL-C25B97BA06852456`,
`PRED-TEX-DECL-6F044DC1EE2FFE76`, and `PRED-TEX-DECL-10515A6565C257C2` are preserved as
source-bound candidate schema data. This module does not turn a schema signature into a relation
instance, define a completion fiber, or introduce question formation or refinement semantics.
-/

namespace InquiryCalculus.Legacy.V20

/-- A typed, non-string name coordinate for a port. -/
structure PortName where
  index : Nat
  deriving DecidableEq, Repr

/-- A port has both a typed name coordinate and a binding-indexed type coordinate. -/
structure NamedPort (B : Binding) (I : TypeInterpretation B) where
  name : PortName
  type : AdmittedType B I

/-- A relation schema is a token and named typed signature, not an instantiated relation. -/
structure RelationSchemaSignature (B : Binding) (I : TypeInterpretation B) where
  relation : B.relationToken
  ports : List (NamedPort B I)

/-- Source-bound obligations that remain outside this data-only schema layer. -/
inductive RelationSchemaPortObligation where
  | typedRelationSchema
  | partialBinding
  | completionFiber
  deriving DecidableEq, Repr

/-- The schema carrier does not imply a relation instance. -/
theorem relation_schema_signature_is_data_only (B : Binding) (I : TypeInterpretation B)
    (schema : RelationSchemaSignature B I) : schema = schema := rfl

end InquiryCalculus.Legacy.V20

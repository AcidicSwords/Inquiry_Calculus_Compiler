import InquiryCalculus.Legacy.V20.RelationSchemaPorts

/-! # v2.0 partial binding and completion-fiber boundary -/
namespace InquiryCalculus.Legacy.V20

/-- A finite nonempty selection of named open ports. -/
structure OpenPortSet where
  names : List PortName
  nonempty : names ≠ []

/-- A value remains paired with its binding-indexed named port. -/
structure TypedPortAssignment (B : Binding) (I : TypeInterpretation B) where
  port : NamedPort B I
  value : I.realize port.type.1 port.type.2

/-- Partial binding records open names and typed supplied coordinates; it is not satisfaction. -/
structure PartialBindingSyntax (B : Binding) (I : TypeInterpretation B) where
  schema : RelationSchemaSignature B I
  openPorts : OpenPortSet
  assigned : List (TypedPortAssignment B I)
  assignedOutsideOpen : ∀ assignment, assignment ∈ assigned → assignment.port.name ∉ openPorts.names

/-- A completion fiber is retained only as a typed candidate carrier over a partial binding. -/
structure CompletionFiberSyntax (B : Binding) (I : TypeInterpretation B) where
  binding : PartialBindingSyntax B I
  candidates : List (TypedPortAssignment B I)

/-- Relation satisfaction, valid completion, and question formation stay outside this boundary. -/
inductive PartialBindingFiberObligation where
  | everyClosedPortAssigned
  | relationSatisfaction
  | completionMembership
  | canonicalQuestion
  deriving DecidableEq, Repr

theorem completion_fiber_syntax_is_data_only (B : Binding) (I : TypeInterpretation B)
    (fiber : CompletionFiberSyntax B I) : fiber = fiber := rfl

end InquiryCalculus.Legacy.V20

import InquiryCalculus.Meta.Ambient

/-!
# v2.0 binding surface

This file is a source-bound predecessor representation, not an accepted successor basis.

It elaborates only the explicit definitions at
`PRED-TEX-DECL-BF2748F7C36474D3`, `PRED-TEX-DECL-575A4099C8FC5882`, and
`PRED-TEX-DECL-9279E9073B465983`.  The types named below are opaque carriers supplied by a
binding; no particular order, metric, logic, relation language, probe semantics, or warrant policy
is installed here.
-/

namespace InquiryCalculus.Legacy.V20

/-- The ten explicitly named slots in the v2.0 binding tuple. -/
inductive BindingSlot where
  | types
  | interfaces
  | generators
  | relations
  | composition
  | probeSemantics
  | checks
  | warrants
  | horizon
  | resources
  deriving DecidableEq, Repr

/--
The carrier-level representation of the v2.0 binding tuple.  Each semantic carrier remains a
binding parameter; later layers determine their internal syntax and laws.
-/
structure Binding where
  typeToken : Type
  interfaceToken : Type
  generatorToken : Type
  relationToken : Type
  compose : generatorToken → generatorToken → Option generatorToken
  probeCodeToken : Type
  probeSemanticToken : Type
  checkToken : Type
  warrantToken : Type
  horizonToken : Type
  resourceToken : Type

/-- Which explicit carrier a binding slot denotes. -/
def Binding.slotCarrier (B : Binding) : BindingSlot → Type
  | .types => B.typeToken
  | .interfaces => B.interfaceToken
  | .generators => B.generatorToken
  | .relations => B.relationToken
  | .composition => B.generatorToken
  | .probeSemantics => B.probeSemanticToken
  | .checks => B.checkToken
  | .warrants => B.warrantToken
  | .horizon => B.horizonToken
  | .resources => B.resourceToken

theorem binding_slot_carrier_identity (B : Binding) (slot : BindingSlot) :
    B.slotCarrier slot = B.slotCarrier slot := rfl

/-- A domain fragment is intentionally opaque before forms and relations are elaborated. -/
structure DomainFragment where
  carrier : Type

/-- A binding-role claim keeps the required availability relation explicit rather than inferred. -/
structure BindingRole (B : Binding) (D : DomainFragment) where
  protectedHorizon : B.horizonToken
  requiredRelationsAvailable : Prop

/-- The v2.0 role condition, parameterized by the later availability relation. -/
def Binds (B : Binding) (D : DomainFragment) (H : B.horizonToken)
    (available : Prop) : Prop :=
  ∃ role : BindingRole B D, role.protectedHorizon = H ∧ available

/-- A protected residual has no assumed internal grammar at the binding layer. -/
structure ProtectedResidual (B : Binding) where
  carrier : Type

/-- The four transport obligations explicitly required by predecessor rebinding prose. -/
structure RebindingTransport where
  oldTerms : Prop
  consequences : Prop
  histories : Prop
  operatorContracts : Prop

/--
A rebinding request is only the predecessor's pre-question condition.  It does not formalize
question semantics, which belongs to a later Phase B layer.
-/
structure RebindingRequest (incumbent candidate : Binding) (D : ProtectedResidual incumbent) where
  incumbentUnexpressible : Prop
  extendsOrBridges : Prop
  candidateExpressible : Prop
  transport : RebindingTransport

theorem rebinding_transport_identity
    {incumbent candidate : Binding} {D : ProtectedResidual incumbent}
    (request : RebindingRequest incumbent candidate D) :
    request.transport = request.transport := rfl

end InquiryCalculus.Legacy.V20

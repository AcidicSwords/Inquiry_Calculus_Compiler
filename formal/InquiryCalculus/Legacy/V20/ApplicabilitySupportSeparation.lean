import InquiryCalculus.Legacy.V20.LeastFixedPointStanding

/-! # Applicability is not evidential support

Source-bound realization of v2.0 lines 4615–4632. A retained relation preserves its
relation, scope, applicability, support family, negative boundary, warrant class, and
certificate references as distinct coordinates. Applicability determines current use only;
it supplies neither support nor deletion of historical warrant material.
-/
namespace InquiryCalculus.Legacy.V20.ApplicabilitySupportSeparation

universe u v w x y z q

structure RetainedRelation (Relation : Type u) (Scope : Type v) (Applicability : Type w)
    (Support : Type x) (NegativeBoundary : Type y) (WarrantClass : Type z)
    (CertificateRef : Type q) where
  relation : Relation
  scope : Scope
  applicability : Applicability
  supportFamily : Support
  negativeBoundary : NegativeBoundary
  warrantClass : WarrantClass
  certificateRefs : List CertificateRef

structure RetainedRelationContext (Applicability : Type u) (Support : Type v) where
  applicable : Applicability → Prop
  evidentiallySupported : Support → Prop

def MayUse {Relation : Type u} {Scope : Type v} {Applicability : Type w}
    {Support : Type x} {NegativeBoundary : Type y} {WarrantClass : Type z}
    {CertificateRef : Type q} (context : RetainedRelationContext Applicability Support)
    (record : RetainedRelation Relation Scope Applicability Support NegativeBoundary WarrantClass
      CertificateRef) : Prop :=
  context.applicable record.applicability

def HasEvidentialSupport {Relation : Type u} {Scope : Type v} {Applicability : Type w}
    {Support : Type x} {NegativeBoundary : Type y} {WarrantClass : Type z}
    {CertificateRef : Type q} (context : RetainedRelationContext Applicability Support)
    (record : RetainedRelation Relation Scope Applicability Support NegativeBoundary WarrantClass
      CertificateRef) : Prop :=
  context.evidentiallySupported record.supportFamily

def Deactivate {Relation : Type u} {Scope : Type v} {Applicability : Type w}
    {Support : Type x} {NegativeBoundary : Type y} {WarrantClass : Type z}
    {CertificateRef : Type q}
    (record : RetainedRelation Relation Scope Applicability Support NegativeBoundary WarrantClass
      CertificateRef) (inactive : Applicability) :
    RetainedRelation Relation Scope Applicability Support NegativeBoundary WarrantClass
      CertificateRef :=
  { record with applicability := inactive }

theorem deactivateRetainsRelation {Relation : Type u} {Scope : Type v} {Applicability : Type w}
    {Support : Type x} {NegativeBoundary : Type y} {WarrantClass : Type z}
    {CertificateRef : Type q}
    (record : RetainedRelation Relation Scope Applicability Support NegativeBoundary WarrantClass
      CertificateRef) (inactive : Applicability) :
    (Deactivate record inactive).relation = record.relation := rfl

theorem deactivateRetainsScope {Relation : Type u} {Scope : Type v} {Applicability : Type w}
    {Support : Type x} {NegativeBoundary : Type y} {WarrantClass : Type z}
    {CertificateRef : Type q}
    (record : RetainedRelation Relation Scope Applicability Support NegativeBoundary WarrantClass
      CertificateRef) (inactive : Applicability) :
    (Deactivate record inactive).scope = record.scope := rfl

theorem deactivateRetainsSupport {Relation : Type u} {Scope : Type v} {Applicability : Type w}
    {Support : Type x} {NegativeBoundary : Type y} {WarrantClass : Type z}
    {CertificateRef : Type q}
    (record : RetainedRelation Relation Scope Applicability Support NegativeBoundary WarrantClass
      CertificateRef) (inactive : Applicability) :
    (Deactivate record inactive).supportFamily = record.supportFamily := rfl

theorem deactivateRetainsNegativeBoundary {Relation : Type u} {Scope : Type v}
    {Applicability : Type w} {Support : Type x} {NegativeBoundary : Type y}
    {WarrantClass : Type z} {CertificateRef : Type q}
    (record : RetainedRelation Relation Scope Applicability Support NegativeBoundary WarrantClass
      CertificateRef) (inactive : Applicability) :
    (Deactivate record inactive).negativeBoundary = record.negativeBoundary := rfl

theorem deactivateRetainsWarrantClass {Relation : Type u} {Scope : Type v}
    {Applicability : Type w} {Support : Type x} {NegativeBoundary : Type y}
    {WarrantClass : Type z} {CertificateRef : Type q}
    (record : RetainedRelation Relation Scope Applicability Support NegativeBoundary WarrantClass
      CertificateRef) (inactive : Applicability) :
    (Deactivate record inactive).warrantClass = record.warrantClass := rfl

theorem deactivateRetainsCertificateRefs {Relation : Type u} {Scope : Type v}
    {Applicability : Type w} {Support : Type x} {NegativeBoundary : Type y}
    {WarrantClass : Type z} {CertificateRef : Type q}
    (record : RetainedRelation Relation Scope Applicability Support NegativeBoundary WarrantClass
      CertificateRef) (inactive : Applicability) :
    (Deactivate record inactive).certificateRefs = record.certificateRefs := rfl

namespace Countermodel

abbrev Record := RetainedRelation Unit Unit Bool Bool Unit Bool Unit

def context : RetainedRelationContext Bool Bool where
  applicable := fun flag => flag = true
  evidentiallySupported := fun flag => flag = true

def warrantedActive : Record := ⟨(), (), true, true, (), true, [()]⟩
def warrantedInactive : Record := Deactivate warrantedActive false
def applicableUnsupported : Record := ⟨(), (), true, false, (), true, [()]⟩

theorem activeRecordMayBeUsed : MayUse context warrantedActive := rfl

theorem inactiveRecordMayNotBeUsed : ¬ MayUse context warrantedInactive := by
  intro usable
  exact Bool.noConfusion usable

theorem activeAndInactiveDifferOnlyInApplicability :
    warrantedActive.relation = warrantedInactive.relation ∧
      warrantedActive.scope = warrantedInactive.scope ∧
      warrantedActive.supportFamily = warrantedInactive.supportFamily ∧
      warrantedActive.negativeBoundary = warrantedInactive.negativeBoundary ∧
      warrantedActive.warrantClass = warrantedInactive.warrantClass ∧
      warrantedActive.certificateRefs = warrantedInactive.certificateRefs ∧
      warrantedActive.applicability ≠ warrantedInactive.applicability := by
  exact ⟨rfl, rfl, rfl, rfl, rfl, rfl, Bool.noConfusion⟩

theorem deactivationPreservesHistoricalWarrant :
    warrantedInactive.warrantClass = true ∧ warrantedInactive.certificateRefs = [()] :=
  ⟨rfl, rfl⟩

theorem applicabilityDoesNotEstablishSupport :
    MayUse context applicableUnsupported ∧ ¬ HasEvidentialSupport context applicableUnsupported :=
  ⟨rfl, fun supported => Bool.noConfusion supported⟩

theorem deactivationDoesNotEraseSupport :
    HasEvidentialSupport context warrantedActive ↔
      HasEvidentialSupport context warrantedInactive :=
  ⟨fun _ => rfl, fun _ => rfl⟩

theorem supportDoesNotImplyCurrentUse :
    HasEvidentialSupport context warrantedInactive ∧ ¬ MayUse context warrantedInactive :=
  ⟨rfl, inactiveRecordMayNotBeUsed⟩

end Countermodel
end InquiryCalculus.Legacy.V20.ApplicabilitySupportSeparation

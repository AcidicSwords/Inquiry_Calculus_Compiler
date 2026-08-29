import InquiryCalculus.Legacy.V20.Relations

/-!
# v2.0 coarsest relation and relational refinement

This source-bound predecessor surface elaborates the definitions at
`PRED-TEX-DECL-010C4595F5CC17EE`, `PRED-TEX-DECL-4DAB46E45E8C0CF5`,
`PRED-TEX-DECL-14ACF76C220FCB11`, and `PRED-TEX-DECL-B8B8BF56A44CE2CC`.
`PRED-TEX-DECL-9D71EB923A8BD574` remains an explicit unproved obligation. Formula,
question, probe, protected-consequence, and program semantics remain later layers.
-/

namespace InquiryCalculus.Legacy.V20

/-- A relation on the binding's represented field, separate from typed value relations. -/
structure RepresentedRelation (B : Binding) (I : TypeInterpretation B) where
  holds : Form B I → Form B I → Prop

/-- Universal relatedness records only joint admission to the represented field. -/
def coarsestRepresentedRelation (B : Binding) (I : TypeInterpretation B) :
    RepresentedRelation B I where
  holds _ _ := True

/-- An existence/actuality predicate is supplied by a binding, not made constitutional. -/
structure ExistenceBoundary (B : Binding) (I : TypeInterpretation B) where
  actual : Form B I → Prop

/-- Binding-level coexistence is stronger than universal joint admission. -/
def bindingCoexists {B : Binding} {I : TypeInterpretation B}
    (E : ExistenceBoundary B I) : RepresentedRelation B I where
  holds x y := E.actual x ∧ E.actual y

/-- Refinement reverses extension inclusion: the more discriminating relation is smaller. -/
def RepresentedRelation.refines {B : Binding} {I : TypeInterpretation B}
    (R S : RepresentedRelation B I) : Prop :=
  ∀ x y, S.holds x y → R.holds x y

/-- The typed relational shape of a proper refinement, without question semantics. -/
structure RefinementDiscoveryBoundary (B : Binding) (I : TypeInterpretation B)
    (base : RepresentedRelation B I) where
  candidate : RepresentedRelation B I
  refinesBase : base.refines candidate
  proper : ∃ x y, base.holds x y ∧ ¬ candidate.holds x y

theorem coarsest_relation_is_least {B : Binding} {I : TypeInterpretation B}
    (R : RepresentedRelation B I) : (coarsestRepresentedRelation B I).refines R :=
  fun _ _ _ => True.intro

theorem refinement_is_reflexive {B : Binding} {I : TypeInterpretation B}
    (R : RepresentedRelation B I) : R.refines R :=
  fun _ _ related => related

theorem refinement_is_transitive {B : Binding} {I : TypeInterpretation B}
    {R S T : RepresentedRelation B I} (RS : R.refines S) (ST : S.refines T) : R.refines T :=
  fun x y related => RS x y (ST x y related)

/-- Source claims that require later protected-consequence or inquiry semantics remain obligations. -/
inductive CoarseRelationRefinementObligation where
  | discoveryProtectedConsequence
  | discoveryProtectedDiscriminator
  | discoveryRegenerativeFactor
  | noVacuityFromUniversalRelatedness
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

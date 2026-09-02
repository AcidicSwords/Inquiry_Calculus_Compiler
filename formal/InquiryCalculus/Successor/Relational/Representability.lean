import InquiryCalculus.Successor.Relational.TypedSurface

/-! # Gate C: regular representability of predecessor relations

This module types the contextual domain/codomain translation for a predecessor binary
`Relation` and characterizes representability by a successor `RegularFormula`.

Four boundaries are kept apart and none of them is collapsed here.

* A regular doctrine has no ambient truth-value carrier, so "the formula holds at an
  assignment" is entailment from `top`, not a `Prop` read off the predicate.
* Semantic existence is not representability. A predecessor relation is an arbitrary host
  `Prop`; a representing formula is a finite token in the regular syntax.
* A formula token is not an extensional soundness proof, so `Represents` demands pointwise
  equivalence in both directions rather than a one-way translation.
* Rejecting universal representability is not losing a protected predecessor capability: the
  binding-extension path below shows exactly how a nonregular relation is retained, and
  `UniformGeneration` already proves that atomizing current targets is not a minimality
  argument.
-/
namespace InquiryCalculus.Successor.Relational

open InquiryCalculus.Successor.Ambient
open InquiryCalculus.Legacy.V20

universe u v w x p q r s

set_option linter.checkUnivs false

namespace Representability

/-- Satisfaction in a regular doctrine is entailment from `top`. There is no ambient
two-valued carrier to project onto, so this is the only available reading of "holds". -/
def Satisfies {context : CtxFam.{u, v, w, x}}
    (predicates : RegPred.{u, v, w, x, p} context) {Γ : context.Ctx}
    (P : predicates.Pred Γ) : Prop :=
  predicates.Entails predicates.top P

/-- The supplied contextual translation of predecessor admitted types and their values.
Nothing here is inferred: a bridge must exhibit a base context, a type translation, and a
closed term for each predecessor value. -/
structure ContextualBridge (context : CtxFam.{u, v, w, x})
    (legacyBinding : Binding) (legacyInterpretation : TypeInterpretation legacyBinding) where
  base : context.Ctx
  ty : AdmittedType legacyBinding legacyInterpretation → context.Ty base
  encode : (A : AdmittedType legacyBinding legacyInterpretation) →
    legacyInterpretation.realize A.1 A.2 → context.Tm base (ty A)

variable {context : CtxFam.{u, v, w, x}}
variable {legacyBinding : Binding} {legacyInterpretation : TypeInterpretation legacyBinding}

/-- The one-variable telescope carrying the domain port. -/
def domainCtx (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    (A : AdmittedType legacyBinding legacyInterpretation) : context.Ctx :=
  context.extend bridge.base (bridge.ty A)

/-- The codomain type weakened over the domain port. -/
def codomainTy (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    (A C : AdmittedType legacyBinding legacyInterpretation) : context.Ty (domainCtx bridge A) :=
  context.reindexTy (bridge.ty C) (context.projection (bridge.ty A))

/-- The two-variable telescope for a binary relation: domain port then codomain port. -/
def relationCtx (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    (A C : AdmittedType legacyBinding legacyInterpretation) : context.Ctx :=
  context.extend (domainCtx bridge A) (codomainTy bridge A C)

/-- The closed assignment supplying one domain value. -/
def domainSub (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    (A : AdmittedType legacyBinding legacyInterpretation)
    (a : legacyInterpretation.realize A.1 A.2) : context.Sub bridge.base (domainCtx bridge A) :=
  context.pair (context.id bridge.base)
    (cast (congrArg (context.Tm bridge.base) (context.reindexTy_id (bridge.ty A)).symm)
      (bridge.encode A a))

theorem domainSub_projection (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    (A : AdmittedType legacyBinding legacyInterpretation)
    (a : legacyInterpretation.realize A.1 A.2) :
    context.comp (context.projection (bridge.ty A)) (domainSub bridge A a) =
      context.id bridge.base :=
  context.pair_projection _ _

/-- Weakening the codomain type and then substituting the domain value returns the codomain
type itself. This is what makes the second port well typed at the base context. -/
theorem codomainTy_reindex (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    (A C : AdmittedType legacyBinding legacyInterpretation)
    (a : legacyInterpretation.realize A.1 A.2) :
    context.reindexTy (codomainTy bridge A C) (domainSub bridge A a) = bridge.ty C :=
  calc context.reindexTy (codomainTy bridge A C) (domainSub bridge A a)
      = context.reindexTy (bridge.ty C)
          (context.comp (context.projection (bridge.ty A)) (domainSub bridge A a)) :=
        (context.reindexTy_comp (bridge.ty C) (context.projection (bridge.ty A))
          (domainSub bridge A a)).symm
    _ = context.reindexTy (bridge.ty C) (context.id bridge.base) := by
        rw [domainSub_projection]
    _ = bridge.ty C := context.reindexTy_id (bridge.ty C)

/-- The closed assignment supplying a domain/codomain value pair. -/
def relationSub (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    (A C : AdmittedType legacyBinding legacyInterpretation)
    (a : legacyInterpretation.realize A.1 A.2)
    (c : legacyInterpretation.realize C.1 C.2) :
    context.Sub bridge.base (relationCtx bridge A C) :=
  context.pair (domainSub bridge A a)
    (cast (congrArg (context.Tm bridge.base) (codomainTy_reindex bridge A C a).symm)
      (bridge.encode C c))

variable {predicates : RegPred.{u, v, w, x, p} context}

/-- Pointwise denotational soundness. Both directions are required: a formula token that only
implies the predecessor relation is not an extensional representation of it. -/
def Represents (binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates)
    (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    {A C : AdmittedType legacyBinding legacyInterpretation}
    (R : Relation legacyBinding legacyInterpretation A C)
    (formula : RegularFormula binding (relationCtx bridge A C)) : Prop :=
  ∀ a c, R.holds a c ↔
    Satisfies predicates (predicates.reindex formula.denote (relationSub bridge A C a c))

/-- A representation is a formula together with its soundness witness, never a formula alone. -/
structure RegularRepresentation
    (binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates)
    (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    {A C : AdmittedType legacyBinding legacyInterpretation}
    (R : Relation legacyBinding legacyInterpretation A C) where
  formula : RegularFormula binding (relationCtx bridge A C)
  sound : Represents binding bridge R formula

/-! ## The two telescope variables and their substitution behaviour -/

/-- The domain port, weakened over the codomain port. Its type is the domain type weakened
twice, which coincides with the codomain port's type exactly when the two ports agree. -/
abbrev domainVar (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    (A C : AdmittedType legacyBinding legacyInterpretation) :=
  context.reindexTm (context.projection (codomainTy bridge A C)) (context.genericTerm (bridge.ty A))

/-- The codomain port. -/
abbrev codomainVar (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    (A C : AdmittedType legacyBinding legacyInterpretation) :=
  context.genericTerm (codomainTy bridge A C)

theorem relationSub_projection
    (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    (A C : AdmittedType legacyBinding legacyInterpretation)
    (a : legacyInterpretation.realize A.1 A.2) (c : legacyInterpretation.realize C.1 C.2) :
    context.comp (context.projection (codomainTy bridge A C)) (relationSub bridge A C a c) =
      domainSub bridge A a :=
  context.pair_projection _ _

/-- Substituting the assignment into the codomain port returns the encoded codomain value. -/
theorem codomainVar_subst (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    (A C : AdmittedType legacyBinding legacyInterpretation)
    (a : legacyInterpretation.realize A.1 A.2) (c : legacyInterpretation.realize C.1 C.2) :
    HEq (context.reindexTm (relationSub bridge A C a c) (codomainVar bridge A C))
      (bridge.encode C c) := by
  refine HEq.trans (context.pair_genericTerm (domainSub bridge A a) _) ?_
  exact cast_heq _ _

/-- Substituting the assignment into the domain port returns the encoded domain value. -/
theorem domainVar_subst (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    (A C : AdmittedType legacyBinding legacyInterpretation)
    (a : legacyInterpretation.realize A.1 A.2) (c : legacyInterpretation.realize C.1 C.2) :
    HEq (context.reindexTm (relationSub bridge A C a c) (domainVar bridge A C))
      (bridge.encode A a) := by
  have composed :
      HEq (context.reindexTm
            (context.comp (context.projection (codomainTy bridge A C)) (relationSub bridge A C a c))
            (context.genericTerm (bridge.ty A)))
        (context.reindexTm (relationSub bridge A C a c) (domainVar bridge A C)) :=
    context.reindexTm_comp (relationSub bridge A C a c)
      (context.projection (codomainTy bridge A C)) (context.genericTerm (bridge.ty A))
  rw [relationSub_projection] at composed
  refine HEq.trans composed.symm ?_
  refine HEq.trans (context.pair_genericTerm (context.id bridge.base) _) ?_
  exact cast_heq _ _

/-! ## Identity representability under a faithful encoding

The assertion direction is derived from `equal_refl` and the substitution laws. Only the
converse is supplied, because a doctrine identifying every encoded value would satisfy the
assertion while the predecessor relation still separated the values. -/

/-- The candidate formula representing the predecessor identity relation. -/
def identityFormula
    {binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates}
    (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    (A : AdmittedType legacyBinding legacyInterpretation) :
    RegularFormula binding (relationCtx bridge A A) :=
  RegularFormula.equal (domainVar bridge A A) (codomainVar bridge A A)

/-- Under the diagonal assignment both ports substitute to the same encoded term. -/
theorem identity_ports_agree
    (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    (A : AdmittedType legacyBinding legacyInterpretation)
    (a : legacyInterpretation.realize A.1 A.2) :
    context.reindexTm (relationSub bridge A A a a) (domainVar bridge A A) =
      context.reindexTm (relationSub bridge A A a a) (codomainVar bridge A A) :=
  eq_of_heq (HEq.trans (domainVar_subst bridge A A a a)
    (codomainVar_subst bridge A A a a).symm)

/-- The identity formula is assertable at every diagonal assignment. This uses only equality
introduction and the substitution laws; nothing is supplied. -/
theorem identity_holds_on_diagonal
    {binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates}
    (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    (A : AdmittedType legacyBinding legacyInterpretation)
    (a : legacyInterpretation.realize A.1 A.2) :
    Satisfies predicates (predicates.reindex
      (identityFormula (binding := binding) bridge A).denote (relationSub bridge A A a a)) := by
  have key : predicates.reindex
        (identityFormula (binding := binding) bridge A).denote (relationSub bridge A A a a)
      = predicates.equal
          (context.reindexTm (relationSub bridge A A a a) (domainVar bridge A A))
          (context.reindexTm (relationSub bridge A A a a) (codomainVar bridge A A)) :=
    predicates.equal_reindex (relationSub bridge A A a a)
      (domainVar bridge A A) (codomainVar bridge A A)
  show predicates.Entails predicates.top _
  rw [key, identity_ports_agree]
  exact predicates.equal_refl _

/-- The one supplied condition: an asserted equality of encoded values reflects an actual
equality. A doctrine collapsing all values fails exactly here. -/
structure FaithfulEncoding
    (binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates)
    (bridge : ContextualBridge context legacyBinding legacyInterpretation) where
  reflect : ∀ (A : AdmittedType legacyBinding legacyInterpretation)
    (a c : legacyInterpretation.realize A.1 A.2),
    Satisfies predicates (predicates.reindex
      (identityFormula (binding := binding) bridge A).denote (relationSub bridge A A a c)) → a = c

/-- The displayed identity formula represents predecessor identity under supplied faithfulness.
This does not characterize every other possible representing formula. -/
def identityRepresentation
    {binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates}
    {bridge : ContextualBridge context legacyBinding legacyInterpretation}
    (faithful : FaithfulEncoding binding bridge)
    (A : AdmittedType legacyBinding legacyInterpretation) :
    RegularRepresentation binding bridge
      (identityRelation legacyBinding legacyInterpretation A) where
  formula := identityFormula bridge A
  sound := by
    intro a c
    constructor
    · intro equalValues
      cases equalValues
      exact identity_holds_on_diagonal bridge A a
    · exact faithful.reflect A a c

/-! ## Existential introduction and the reflection boundary

Serial composition of predecessor relations binds a mediator existentially. The carrier of the
protected difference is therefore the existential port: asserting that a mediator exists is not
the same as having an actual predecessor mediator value.

The forward direction below is derived from the declared adjunction alone. The converse is not
derivable, because a doctrine in which `existsAlong` is satisfied with no term witnessing it is
a perfectly good regular doctrine. That converse is exactly `ExistentialReflection`. -/

/-- The unit of the existential adjunction, obtained from the declared adjunction alone. -/
theorem exists_unit (predicates : RegPred.{u, v, w, x, p} context) {Γ : context.Ctx}
    (A : context.Ty Γ) (P : predicates.Pred (context.extend Γ A)) :
    predicates.Entails P
      (predicates.reindex (predicates.existsAlong A P) (context.projection A)) :=
  (predicates.exists_adjunction A P (predicates.existsAlong A P)).mp
    (predicates.entails_refl _)

/-- Substituting the assignment back through the projection collapses a weakened predicate. -/
theorem reindex_projection_pair (predicates : RegPred.{u, v, w, x, p} context)
    {Γ : context.Ctx} {A : context.Ty Γ} (P : predicates.Pred Γ)
    (witness : context.Tm Γ (context.reindexTy A (context.id Γ))) :
    predicates.reindex (predicates.reindex P (context.projection A))
        (context.pair (context.id Γ) witness) = P :=
  calc predicates.reindex (predicates.reindex P (context.projection A))
          (context.pair (context.id Γ) witness)
      = predicates.reindex P
          (context.comp (context.projection A) (context.pair (context.id Γ) witness)) :=
        (predicates.reindex_comp P (context.projection A) _).symm
    _ = predicates.reindex P (context.id Γ) := by rw [context.pair_projection]
    _ = P := predicates.reindex_id P

/-- Existential introduction. An actual witnessing term makes the existential assertable, and
nothing is supplied: this uses only the declared adjunction and reindexing laws. -/
theorem exists_intro (predicates : RegPred.{u, v, w, x, p} context) {Γ : context.Ctx}
    (A : context.Ty Γ) (P : predicates.Pred (context.extend Γ A))
    (witness : context.Tm Γ (context.reindexTy A (context.id Γ)))
    (satisfied : Satisfies predicates
      (predicates.reindex P (context.pair (context.id Γ) witness))) :
    Satisfies predicates (predicates.existsAlong A P) := by
  have transported := predicates.reindex_monotone (context.pair (context.id Γ) witness)
    (exists_unit predicates A P)
  rw [reindex_projection_pair predicates (predicates.existsAlong A P) witness] at transported
  exact predicates.entails_trans satisfied transported

/-- Supplied witness extraction for the contextual existential carrier. This returns a term,
not a predecessor value. Composition correspondence additionally needs coverage of the relevant
terms by encoded predecessor mediators; `ContextualBridge.encode` supplies no surjectivity law.
An independence countermodel for witness extraction is a separate obligation. -/
structure ExistentialReflection (predicates : RegPred.{u, v, w, x, p} context) where
  witness : ∀ {Γ : context.Ctx} (A : context.Ty Γ) (P : predicates.Pred (context.extend Γ A)),
    Satisfies predicates (predicates.existsAlong A P) →
    ∃ term : context.Tm Γ (context.reindexTy A (context.id Γ)),
      Satisfies predicates (predicates.reindex P (context.pair (context.id Γ) term))

/-- Existential assertion and actual witnessing coincide exactly under reflection. The forward
implication is the derived introduction rule; only the backward one consumes the condition. -/
theorem exists_iff_witness (predicates : RegPred.{u, v, w, x, p} context)
    (reflection : ExistentialReflection predicates) {Γ : context.Ctx}
    (A : context.Ty Γ) (P : predicates.Pred (context.extend Γ A)) :
    Satisfies predicates (predicates.existsAlong A P) ↔
      ∃ term : context.Tm Γ (context.reindexTy A (context.id Γ)),
        Satisfies predicates (predicates.reindex P (context.pair (context.id Γ) term)) := by
  constructor
  · exact reflection.witness A P
  · rintro ⟨term, satisfied⟩
    exact exists_intro predicates A P term satisfied

/-! ## The nonrepresentability breaker

Failure to find a representing formula is not a proof that none exists. A decisive breaker
must supply a property closed under every regular formation rule and then show no predicate
with that property is pointwise equivalent to the relation. -/

/-- A property of predicates closed under every regular formation rule. -/
structure DenotationInvariant
    (binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates) where
  holds : {Γ : context.Ctx} → predicates.Pred Γ → Prop
  atom : ∀ (symbol : binding.RelAtom) {Γ : context.Ctx}
    (substitution : context.Sub Γ (binding.atomContext symbol)),
    holds (predicates.reindex (binding.atomInterpretation symbol) substitution)
  equal : ∀ {Γ : context.Ctx} {A : context.Ty Γ} (left right : context.Tm Γ A),
    holds (predicates.equal left right)
  top : ∀ {Γ : context.Ctx}, holds (predicates.top (Γ := Γ))
  meet : ∀ {Γ : context.Ctx} (P Q : predicates.Pred Γ), holds P → holds Q →
    holds (predicates.meet P Q)
  existsAlong : ∀ {Γ : context.Ctx} (A : context.Ty Γ)
    (P : predicates.Pred (context.extend Γ A)), holds P → holds (predicates.existsAlong A P)
  extension : ∀ (operator : binding.LogicOperator) (Γ : context.Ctx)
    (arguments : Fin (binding.logicArity operator) → predicates.Pred Γ),
    (∀ index, holds (arguments index)) →
    holds (binding.logicInterpretation operator Γ arguments)

/-- Every regular formula denotation satisfies every such invariant. -/
theorem denote_invariant
    {binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates}
    (invariant : DenotationInvariant binding) :
    ∀ {Γ : context.Ctx} (formula : RegularFormula binding Γ), invariant.holds formula.denote := by
  intro Γ formula
  induction formula with
  | atom symbol substitution => exact invariant.atom symbol substitution
  | equal left right => exact invariant.equal left right
  | top => exact invariant.top
  | meet _ _ leftIH rightIH => exact invariant.meet _ _ leftIH rightIH
  | «exists» A _ bodyIH => exact invariant.existsAlong A _ bodyIH
  | extension operator _ argumentsIH => exact invariant.extension operator _ _ argumentsIH

/-- Exactly what a documented nonrepresentable relation must supply. -/
structure NonRepresentabilityWitness
    (binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates)
    (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    {A C : AdmittedType legacyBinding legacyInterpretation}
    (R : Relation legacyBinding legacyInterpretation A C) where
  invariant : DenotationInvariant binding
  separates : ∀ (P : predicates.Pred (relationCtx bridge A C)), invariant.holds P →
    ¬ (∀ a c, R.holds a c ↔
        Satisfies predicates (predicates.reindex P (relationSub bridge A C a c)))

/-- A relation admitting such a witness has no regular representation at all. This is a
negative theorem, not a failed search. -/
theorem not_representable
    {binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates}
    {bridge : ContextualBridge context legacyBinding legacyInterpretation}
    {A C : AdmittedType legacyBinding legacyInterpretation}
    {R : Relation legacyBinding legacyInterpretation A C}
    (witness : NonRepresentabilityWitness binding bridge R) :
    ¬ Nonempty (RegularRepresentation binding bridge R) := by
  rintro ⟨representation⟩
  exact witness.separates representation.formula.denote
    (denote_invariant witness.invariant representation.formula) representation.sound

/-! ## The binding-extension path

A protected nonregular relation is retained by admitting it as a binding atom whose
interpretation is the intended predicate. This preserves the capability; it does not show the
regular core was sufficient. `UniformGeneration.freshProtectedTargetBreaksUniformGeneration`
already proves that atomizing current targets fails under conservative atom extension, so the
route below must never be read as a minimality or sufficiency argument. -/

/-- Data admitting one predecessor relation as a binding atom reachable from its telescope.
The substitution is the same datum `RegularFormula.atom` already requires, so no transport
along a context equality is needed. -/
structure BindingExtensionRoute
    (binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates)
    (bridge : ContextualBridge context legacyBinding legacyInterpretation)
    {A C : AdmittedType legacyBinding legacyInterpretation}
    (R : Relation legacyBinding legacyInterpretation A C) where
  symbol : binding.RelAtom
  substitution : context.Sub (relationCtx bridge A C) (binding.atomContext symbol)
  interprets : ∀ a c, R.holds a c ↔
    Satisfies predicates (predicates.reindex
      (predicates.reindex (binding.atomInterpretation symbol) substitution)
      (relationSub bridge A C a c))

/-- An admitted atom does represent its relation. The content is the typed retention route,
not an expressivity claim: manufacturing an atom for each desired relation is exactly the move
`UniformGeneration` rejects as a sufficiency argument. -/
def BindingExtensionRoute.representation
    {binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates}
    {bridge : ContextualBridge context legacyBinding legacyInterpretation}
    {A C : AdmittedType legacyBinding legacyInterpretation}
    {R : Relation legacyBinding legacyInterpretation A C}
    (route : BindingExtensionRoute binding bridge R) :
    RegularRepresentation binding bridge R where
  formula := RegularFormula.atom route.symbol route.substitution
  sound := route.interprets

/-! ## Retained obligations

Identity representability is now checked under `FaithfulEncoding`, and the existential carrier
is checked under `ExistentialReflection`. Neither proves composition closure. The extracted
contextual witness must additionally be an encoded predecessor mediator (or satisfy a weaker
relevant-witness coverage condition). Also remaining is the three-port telescope construction
that actually builds a composite formula: placing the two component
formulas over a shared mediator port requires explicit substitutions into
`relationCtx bridge A D` and `relationCtx bridge D C`, which is context plumbing this module
has not yet discharged. It is retained here rather than asserted. -/
inductive RelationRepresentabilityObligation where
  /-- Discharged: `equal_refl` plus `FaithfulEncoding.reflect` give `identityRepresentation`. -/
  | identityRepresentableUnderFaithfulEncoding
  /-- Discharged as a semantic carrier: `exists_intro` is derived and `exists_iff_witness`
  isolates the supplied converse. The composite formula construction is separate. -/
  | existentialCarrierUnderReflection
  /-- Open: relevant contextual witnesses must come from encoded predecessor mediator values. -/
  | predecessorMediatorCoverage
  /-- Open: build the three-port composite formula and prove `Represents` for `serialCompose`. -/
  | compositeTelescopeConstruction
  /-- A concrete finite model instantiating `NonRepresentabilityWitness`. -/
  | concreteComplementCountermodel
  /-- Converse is not inverse; a converse boundary is supplied, never derived. -/
  | converseRequiresSuppliedBoundary
  deriving DecidableEq, Repr

end Representability

end InquiryCalculus.Successor.Relational

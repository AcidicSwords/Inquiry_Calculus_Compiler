/-! # Successor Gate C: ambient capability basis

This module types the four candidate capabilities selected by B→C normalization without claiming
that their names, one concrete constructor grammar, or their bundling is primitive.  `CtxFam`,
`RegPred`, and `IndPlus` are interfaces.  `BindingPresentation` carries domain content that the
universal interfaces do not manufacture.  The finite ablation model proves only interface-level
independence; protected predecessor sufficiency remains a separate witnessed-regeneration
obligation.
-/
namespace InquiryCalculus.Successor.Ambient

universe u v w x p q r s

set_option linter.checkUnivs false

/-- Context-indexed types and terms, functorial substitution, and comprehension presentation. -/
structure CtxFam where
  Ctx : Type u
  Sub : Ctx → Ctx → Type v
  id : (Γ : Ctx) → Sub Γ Γ
  comp : {Γ Δ Θ : Ctx} → Sub Δ Θ → Sub Γ Δ → Sub Γ Θ
  comp_assoc : ∀ {Γ Δ Θ Ξ} (υ : Sub Θ Ξ) (τ : Sub Δ Θ) (σ : Sub Γ Δ),
    comp (comp υ τ) σ = comp υ (comp τ σ)
  id_comp : ∀ {Γ Δ} (σ : Sub Γ Δ), comp (id Δ) σ = σ
  comp_id : ∀ {Γ Δ} (σ : Sub Γ Δ), comp σ (id Γ) = σ
  Ty : Ctx → Type w
  reindexTy : {Γ Δ : Ctx} → Ty Δ → Sub Γ Δ → Ty Γ
  reindexTy_id : ∀ {Γ} (A : Ty Γ), reindexTy A (id Γ) = A
  reindexTy_comp : ∀ {Γ Δ Θ} (A : Ty Θ) (τ : Sub Δ Θ) (σ : Sub Γ Δ),
    reindexTy A (comp τ σ) = reindexTy (reindexTy A τ) σ
  Tm : (Γ : Ctx) → Ty Γ → Type x
  reindexTm : {Γ Δ : Ctx} → (σ : Sub Γ Δ) → {A : Ty Δ} → Tm Δ A → Tm Γ (reindexTy A σ)
  reindexTm_id : ∀ {Γ} {A : Ty Γ} (term : Tm Γ A), HEq (reindexTm (id Γ) term) term
  reindexTm_comp : ∀ {Γ Δ Θ} (σ : Sub Γ Δ) (τ : Sub Δ Θ) {A : Ty Θ} (term : Tm Θ A),
    HEq (reindexTm (comp τ σ) term) (reindexTm σ (reindexTm τ term))
  extend : (Γ : Ctx) → Ty Γ → Ctx
  projection : ∀ {Γ} (A : Ty Γ), Sub (extend Γ A) Γ
  genericTerm : ∀ {Γ} (A : Ty Γ), Tm (extend Γ A) (reindexTy A (projection A))
  pair : ∀ {Γ Δ} {A : Ty Γ} (σ : Sub Δ Γ), Tm Δ (reindexTy A σ) → Sub Δ (extend Γ A)
  pair_projection : ∀ {Γ Δ} {A : Ty Γ} (σ : Sub Δ Γ) (term : Tm Δ (reindexTy A σ)),
    comp (projection A) (pair σ term) = σ
  pair_genericTerm : ∀ {Γ Δ} {A : Ty Γ} (σ : Sub Δ Γ) (term : Tm Δ (reindexTy A σ)),
    HEq (reindexTm (pair σ term) (genericTerm A)) term
  pair_unique : ∀ {Γ Δ} {A : Ty Γ} (σ : Sub Δ Γ) (term : Tm Δ (reindexTy A σ))
      (candidate : Sub Δ (extend Γ A)),
    comp (projection A) candidate = σ → HEq (reindexTm candidate (genericTerm A)) term →
      candidate = pair σ term
  lift : ∀ {Γ Δ} (A : Ty Γ) (σ : Sub Δ Γ),
    Sub (extend Δ (reindexTy A σ)) (extend Γ A)
  lift_projection : ∀ {Γ Δ} (A : Ty Γ) (σ : Sub Δ Γ),
    comp (projection A) (lift A σ) = comp σ (projection (reindexTy A σ))

/-- A proof-insensitive regular predicate doctrine over one context family. -/
structure RegPred (context : CtxFam.{u, v, w, x}) where
  Pred : context.Ctx → Type p
  Entails : {Γ : context.Ctx} → Pred Γ → Pred Γ → Prop
  entails_refl : ∀ {Γ} (P : Pred Γ), Entails P P
  entails_trans : ∀ {Γ} {P Q R : Pred Γ}, Entails P Q → Entails Q R → Entails P R
  entails_antisymm : ∀ {Γ} {P Q : Pred Γ}, Entails P Q → Entails Q P → P = Q
  top : ∀ {Γ}, Pred Γ
  top_intro : ∀ {Γ} (P : Pred Γ), Entails P top
  meet : ∀ {Γ}, Pred Γ → Pred Γ → Pred Γ
  meet_left : ∀ {Γ} (P Q : Pred Γ), Entails (meet P Q) P
  meet_right : ∀ {Γ} (P Q : Pred Γ), Entails (meet P Q) Q
  meet_intro : ∀ {Γ} {P Q R : Pred Γ}, Entails R P → Entails R Q → Entails R (meet P Q)
  reindex : ∀ {Γ Δ}, Pred Δ → context.Sub Γ Δ → Pred Γ
  reindex_id : ∀ {Γ} (P : Pred Γ), reindex P (context.id Γ) = P
  reindex_comp : ∀ {Γ Δ Θ} (P : Pred Θ) (τ : context.Sub Δ Θ) (σ : context.Sub Γ Δ),
    reindex P (context.comp τ σ) = reindex (reindex P τ) σ
  reindex_top : ∀ {Γ Δ} (σ : context.Sub Γ Δ), reindex top σ = top
  reindex_meet : ∀ {Γ Δ} (P Q : Pred Δ) (σ : context.Sub Γ Δ),
    reindex (meet P Q) σ = meet (reindex P σ) (reindex Q σ)
  reindex_monotone : ∀ {Γ Δ} {P Q : Pred Δ} (σ : context.Sub Γ Δ),
    Entails P Q → Entails (reindex P σ) (reindex Q σ)
  equal : ∀ {Γ} {A : context.Ty Γ}, context.Tm Γ A → context.Tm Γ A → Pred Γ
  equal_reindex : ∀ {Γ Δ} (σ : context.Sub Γ Δ) {A : context.Ty Δ}
      (left right : context.Tm Δ A),
    reindex (equal left right) σ = equal (context.reindexTm σ left) (context.reindexTm σ right)
  /-- Equality introduction. Without this rule `equal` is an opaque family and even the
  predecessor identity relation has no representing regular formula, so the doctrine could
  not regenerate a protected predecessor capability. It is the smallest addition that
  removes that specific failure; it grants no complement, implication, or universal. -/
  equal_refl : ∀ {Γ} {A : context.Ty Γ} (term : context.Tm Γ A), Entails top (equal term term)
  existsAlong : ∀ {Γ} (A : context.Ty Γ), Pred (context.extend Γ A) → Pred Γ
  exists_adjunction : ∀ {Γ} (A : context.Ty Γ) (P : Pred (context.extend Γ A)) (Q : Pred Γ),
    Entails (existsAlong A P) Q ↔ Entails P (reindex Q (context.projection A))
  beckChevalley : ∀ {Γ Δ} (A : context.Ty Γ) (P : Pred (context.extend Γ A))
      (σ : context.Sub Δ Γ),
    reindex (existsAlong A P) σ =
      existsAlong (context.reindexTy A σ) (reindex P (context.lift A σ))
  frobenius : ∀ {Γ} (A : context.Ty Γ) (P : Pred (context.extend Γ A)) (Q : Pred Γ),
    existsAlong A (meet P (reindex Q (context.projection A))) = meet (existsAlong A P) Q

/-- A context-indexed family with substitution action. -/
structure IndexedFamily (context : CtxFam.{u, v, w, x}) where
  Obj : context.Ctx → Type q
  reindex : ∀ {Γ Δ}, context.Sub Γ Δ → Obj Δ → Obj Γ
  reindex_id : ∀ {Γ} (value : Obj Γ), reindex (context.id Γ) value = value
  reindex_comp : ∀ {Γ Δ Θ} (σ : context.Sub Γ Δ) (τ : context.Sub Δ Θ) (value : Obj Θ),
    reindex (context.comp τ σ) value = reindex σ (reindex τ value)

/-- A substitution-natural map between indexed families. -/
structure FamilyHom {context : CtxFam.{u, v, w, x}}
    (source target : IndexedFamily.{u, v, w, x, q} context) where
  app : ∀ Γ, source.Obj Γ → target.Obj Γ
  natural : ∀ {Γ Δ} (σ : context.Sub Γ Δ) (value : source.Obj Δ),
    app Γ (source.reindex σ value) = target.reindex σ (app Δ value)

def FamilyHom.identity {context : CtxFam.{u, v, w, x}}
    (family : IndexedFamily.{u, v, w, x, q} context) : FamilyHom family family where
  app := fun _ value => value
  natural := by intros; rfl

def FamilyHom.comp {context : CtxFam.{u, v, w, x}}
    {first second third : IndexedFamily.{u, v, w, x, q} context}
    (later : FamilyHom second third) (earlier : FamilyHom first second) : FamilyHom first third where
  app := fun Γ value => later.app Γ (earlier.app Γ value)
  natural := by
    intro Γ Δ σ value
    rw [earlier.natural, later.natural]

/-- An admitted strictly-positive indexed operator, represented extensionally as an endofunctor. -/
structure PositiveIndexedOperator (context : CtxFam.{u, v, w, x}) where
  obj : IndexedFamily.{u, v, w, x, q} context → IndexedFamily.{u, v, w, x, q} context
  map : ∀ {source target}, FamilyHom source target → FamilyHom (obj source) (obj target)
  map_identity : ∀ (family) Γ (value : (obj family).Obj Γ),
    (map (FamilyHom.identity family)).app Γ value = value
  map_comp : ∀ {first second third} (later : FamilyHom second third)
      (earlier : FamilyHom first second) Γ (value : (obj first).Obj Γ),
    (map (FamilyHom.comp later earlier)).app Γ value =
      (FamilyHom.comp (map later) (map earlier)).app Γ value

structure PositiveAlgebra {context : CtxFam.{u, v, w, x}}
    (operator : PositiveIndexedOperator.{u, v, w, x, q} context) where
  carrier : IndexedFamily.{u, v, w, x, q} context
  act : FamilyHom (operator.obj carrier) carrier

structure PositiveAlgebraHom {context : CtxFam.{u, v, w, x}}
    {operator : PositiveIndexedOperator.{u, v, w, x, q} context}
    (source target : PositiveAlgebra operator) where
  map : FamilyHom source.carrier target.carrier
  commutes : ∀ Γ (value : (operator.obj source.carrier).Obj Γ),
    map.app Γ (source.act.app Γ value) = target.act.app Γ ((operator.map map).app Γ value)

/-- Initiality is the retained content of positive indexed generation. -/
structure InitialPositiveAlgebra {context : CtxFam.{u, v, w, x}}
    (operator : PositiveIndexedOperator.{u, v, w, x, q} context) where
  initial : PositiveAlgebra operator
  fold : (target : PositiveAlgebra operator) → PositiveAlgebraHom initial target
  unique : ∀ (target : PositiveAlgebra operator) (candidate : PositiveAlgebraHom initial target)
      Γ (value : initial.carrier.Obj Γ),
    candidate.map.app Γ value = (fold target).map.app Γ value

/-- Substitution-stable initial families for exactly the admitted positive signatures. -/
structure IndPlus (context : CtxFam.{u, v, w, x}) where
  Signature : Type s
  operator : Signature → PositiveIndexedOperator.{u, v, w, x, q} context
  strictlyPositive : Signature → Prop
  admittedPositive : ∀ signature, strictlyPositive signature
  initial : ∀ signature, InitialPositiveAlgebra (operator signature)

/-- Domain content interpreted in, rather than generated by, the universal regular ambient. -/
structure BindingPresentation (context : CtxFam.{u, v, w, x})
    (predicates : RegPred.{u, v, w, x, p} context) where
  TyCode : Type q
  El : TyCode → Type r
  RelAtom : Type s
  atomContext : RelAtom → context.Ctx
  atomInterpretation : ∀ atom, predicates.Pred (atomContext atom)
  LogicOperator : Type s
  logicArity : LogicOperator → Nat
  logicInterpretation : ∀ operator Γ,
    (Fin (logicArity operator) → predicates.Pred Γ) → predicates.Pred Γ
  logicNatural : ∀ operator {Γ Δ} (σ : context.Sub Γ Δ)
      (arguments : Fin (logicArity operator) → predicates.Pred Δ),
    predicates.reindex (logicInterpretation operator Δ arguments) σ =
      logicInterpretation operator Γ (fun index => predicates.reindex (arguments index) σ)
  LogicLaw : Type s
  logicLawContext : LogicLaw → context.Ctx
  logicLawPremise : ∀ law, predicates.Pred (logicLawContext law)
  logicLawConclusion : ∀ law, predicates.Pred (logicLawContext law)
  logicLawValid : ∀ law, predicates.Entails (logicLawPremise law) (logicLawConclusion law)
  TheoryStatement : Type s
  theoryContext : TheoryStatement → context.Ctx
  theoryPremise : ∀ statement, predicates.Pred (theoryContext statement)
  theoryConclusion : ∀ statement, predicates.Pred (theoryContext statement)
  theoryValid : ∀ statement, predicates.Entails (theoryPremise statement) (theoryConclusion statement)

/-- The candidate bundle is convenient dependency data, not a claim of primitive constructors. -/
structure CapabilityBasis where
  context : CtxFam.{u, v, w, x}
  predicates : RegPred.{u, v, w, x, p} context
  positiveGeneration : IndPlus.{u, v, w, x, q, s} context
  binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates

inductive Capability where
  | ctxFam | regPred | indPlus | bindingPresentation
  deriving DecidableEq, Repr

inductive ProtectedTarget where
  | contextualQuestionAndContinuationReindexing
  | regularRelationAndCompositionLanguage
  | positiveRecursiveSourceAndEvidenceFamilies
  | domainTypeRelationLogicAndTheoryInterpretation
  deriving DecidableEq, Repr

def protectedTarget : Capability → ProtectedTarget
  | .ctxFam => .contextualQuestionAndContinuationReindexing
  | .regPred => .regularRelationAndCompositionLanguage
  | .indPlus => .positiveRecursiveSourceAndEvidenceFamilies
  | .bindingPresentation => .domainTypeRelationLogicAndTheoryInterpretation

/-- Finite information model used only to test independent capability recoverability. -/
abbrev CapabilityProfile := Capability → Bool

def RemainingView (removed : Capability) := { capability : Capability // capability ≠ removed } → Bool

def remainingView (profile : CapabilityProfile) (removed : Capability) : RemainingView removed :=
  fun capability => profile capability.1

def noCapabilities : CapabilityProfile := fun _ => false

def onlyCapability (selected : Capability) : CapabilityProfile :=
  fun capability => decide (capability = selected)

theorem remainingViewCannotRecoverRemovedCapability (removed : Capability) :
    ¬ ∃ recover : RemainingView removed → Bool,
      ∀ profile, recover (remainingView profile removed) = profile removed := by
  intro alleged
  rcases alleged with ⟨recover, recovers⟩
  have sameView : remainingView noCapabilities removed = remainingView (onlyCapability removed) removed := by
    funext capability
    simp [remainingView, noCapabilities, onlyCapability, capability.property]
  have off := recovers noCapabilities
  have on := recovers (onlyCapability removed)
  rw [sameView] at off
  simp [noCapabilities, onlyCapability] at off on
  exact Bool.false_ne_true (off.symm.trans on)

theorem eachCandidateCapabilityHasIndependentFiniteAblation :
    ∀ removed, ¬ ∃ recover : RemainingView removed → Bool,
      ∀ profile, recover (remainingView profile removed) = profile removed :=
  remainingViewCannotRecoverRemovedCapability

inductive AmbientBasisObligation where
  | exactCapabilityTypes
  | capabilityNotConstructorList
  | semanticExistenceNotRepresentability
  | regularCoreNotBindingExtension
  | evidenceRepresentationNotActivation
  | protectedPredecessorRegenerationMap
  | protectedSufficiency
  | ablationMinimalityAtDeclaredHorizon
  | bindingElaboration
  | noGatePassFromInterfaceOrFiniteAblation
  | noRuntimeOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Successor.Ambient

import InquiryCalculus.Successor.Ambient.UniformGeneration
import InquiryCalculus.Legacy.V20.Relations

/-! # Gate C: typed relational surface

The successor surface below separates binding-local forms, contextual forms, free regular syntax,
and predicate denotation.  Reindexing is derived structurally, not admitted as a raw constructor.
The predecessor type/form bridge is explicit and conditional.  Arbitrary predecessor `Prop`-valued
relations are not silently imported as binding atoms or regular formulas.
-/
namespace InquiryCalculus.Successor.Relational

open InquiryCalculus.Successor.Ambient

universe u v w x p q r s

set_option linter.checkUnivs false

structure BindingForm {context : CtxFam.{u, v, w, x}}
    {predicates : RegPred.{u, v, w, x, p} context}
    (binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates) where
  typeCode : binding.TyCode
  value : binding.El typeCode

structure ContextualForm (context : CtxFam.{u, v, w, x}) (Γ : context.Ctx) where
  type : context.Ty Γ
  term : context.Tm Γ type

inductive RegularFormula {context : CtxFam.{u, v, w, x}}
    {predicates : RegPred.{u, v, w, x, p} context}
    (binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates) :
    context.Ctx → Type (max u v w x p q r s) where
  | atom {Γ} (symbol : binding.RelAtom) (substitution : context.Sub Γ (binding.atomContext symbol)) :
      RegularFormula binding Γ
  | equal {Γ} {A : context.Ty Γ} (left right : context.Tm Γ A) : RegularFormula binding Γ
  | top {Γ} : RegularFormula binding Γ
  | meet {Γ} : RegularFormula binding Γ → RegularFormula binding Γ → RegularFormula binding Γ
  | exists {Γ} (A : context.Ty Γ) : RegularFormula binding (context.extend Γ A) → RegularFormula binding Γ
  | extension {Γ} (operator : binding.LogicOperator)
      (arguments : Fin (binding.logicArity operator) → RegularFormula binding Γ) :
      RegularFormula binding Γ

def RegularFormula.reindex {context : CtxFam.{u, v, w, x}}
    {predicates : RegPred.{u, v, w, x, p} context}
    {binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates}
    {Γ Δ : context.Ctx} (σ : context.Sub Γ Δ) : RegularFormula binding Δ → RegularFormula binding Γ
  | .atom symbol τ => .atom symbol (context.comp τ σ)
  | .equal left right => .equal (context.reindexTm σ left) (context.reindexTm σ right)
  | .top => .top
  | .meet left right => .meet (left.reindex σ) (right.reindex σ)
  | .exists A body => .exists (context.reindexTy A σ) (body.reindex (context.lift A σ))
  | .extension operator arguments => .extension operator (fun index => (arguments index).reindex σ)

def RegularFormula.denote {context : CtxFam.{u, v, w, x}}
    {predicates : RegPred.{u, v, w, x, p} context}
    {binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates} :
    {Γ : context.Ctx} → RegularFormula binding Γ → predicates.Pred Γ
  | _, .atom symbol substitution => predicates.reindex (binding.atomInterpretation symbol) substitution
  | _, .equal left right => predicates.equal left right
  | _, .top => predicates.top
  | _, .meet left right => predicates.meet left.denote right.denote
  | _, .exists A body => predicates.existsAlong A body.denote
  | Γ, .extension operator arguments =>
      binding.logicInterpretation operator Γ (fun index => (arguments index).denote)

theorem RegularFormula.denote_reindex {context : CtxFam.{u, v, w, x}}
    {predicates : RegPred.{u, v, w, x, p} context}
    {binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates}
    {Γ Δ : context.Ctx} (σ : context.Sub Γ Δ) (formula : RegularFormula binding Δ) :
    (formula.reindex σ).denote = predicates.reindex formula.denote σ := by
  induction formula generalizing Γ with
  | atom symbol τ => exact predicates.reindex_comp (binding.atomInterpretation symbol) τ σ
  | equal left right => exact (predicates.equal_reindex σ left right).symm
  | top => exact (predicates.reindex_top σ).symm
  | meet left right leftIH rightIH =>
      simp only [RegularFormula.reindex, RegularFormula.denote]
      rw [leftIH σ, rightIH σ]
      exact (predicates.reindex_meet left.denote right.denote σ).symm
  | «exists» A body bodyIH =>
      simp only [RegularFormula.reindex, RegularFormula.denote]
      rw [bodyIH (context.lift A σ)]
      exact (predicates.beckChevalley A body.denote σ).symm
  | extension operator arguments argumentsIH =>
      simp only [RegularFormula.reindex, RegularFormula.denote]
      calc
        binding.logicInterpretation operator Γ (fun index => (arguments index).reindex σ |>.denote) =
            binding.logicInterpretation operator Γ
              (fun index => predicates.reindex (arguments index).denote σ) := by
                congr
                funext index
                exact argumentsIH index σ
        _ = predicates.reindex
              (binding.logicInterpretation operator _ (fun index => (arguments index).denote)) σ :=
                (binding.logicNatural operator σ _).symm

namespace TypeTagFoil

inductive Code where | left | right deriving DecidableEq
def El (_ : Code) := PUnit
def Form := (code : Code) × El code
def eraseValue : Form → PUnit := fun form => form.2
def leftForm : Form := ⟨.left, PUnit.unit⟩
def rightForm : Form := ⟨.right, PUnit.unit⟩

theorem distinctTaggedForms : leftForm ≠ rightForm := by
  intro equalForms
  have equalCodes := congrArg Sigma.fst equalForms
  cases equalCodes

theorem erasureCollides : eraseValue leftForm = eraseValue rightForm := rfl

theorem typeTagErasureIsNotInjective : ¬ Function.Injective eraseValue := by
  intro injective
  exact distinctTaggedForms (injective erasureCollides)

end TypeTagFoil

namespace LegacyBridge

open InquiryCalculus.Legacy.V20

/-- Exact data required to translate predecessor type codes and values into one successor binding. -/
structure TypeInterpretationBridge
    {context : CtxFam.{u, v, w, x}}
    {predicates : RegPred.{u, v, w, x, p} context}
    {binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates}
    (legacyBinding : Binding) (legacyInterpretation : TypeInterpretation legacyBinding) where
  mapType : TypeCode legacyBinding → binding.TyCode
  mapType_injective : Function.Injective mapType
  mapValue : ∀ (code : TypeCode legacyBinding) (admitted : legacyInterpretation.admissible code),
    legacyInterpretation.realize code admitted → binding.El (mapType code)

def mapForm
    {context : CtxFam.{u, v, w, x}}
    {predicates : RegPred.{u, v, w, x, p} context}
    {binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates}
    {legacyBinding : Binding} {legacyInterpretation : TypeInterpretation legacyBinding}
    (bridge : TypeInterpretationBridge (binding := binding) legacyBinding legacyInterpretation)
    (form : Form legacyBinding legacyInterpretation) : BindingForm binding where
  typeCode := bridge.mapType form.type.1
  value := bridge.mapValue form.type.1 form.type.2 form.value

theorem mapForm_preserves_type_code
    {context : CtxFam.{u, v, w, x}}
    {predicates : RegPred.{u, v, w, x, p} context}
    {binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates}
    {legacyBinding : Binding} {legacyInterpretation : TypeInterpretation legacyBinding}
    (bridge : TypeInterpretationBridge (binding := binding) legacyBinding legacyInterpretation)
    (form : Form legacyBinding legacyInterpretation) :
    (mapForm (binding := binding) bridge form).typeCode = bridge.mapType form.type.1 := rfl

/-- Arbitrary legacy relations need a supplied regular representation; it is not inferred from `Prop`. -/
structure RelationRepresentabilityBoundary
    {context : CtxFam.{u, v, w, x}}
    {predicates : RegPred.{u, v, w, x, p} context}
    {binding : BindingPresentation.{u, v, w, x, p, q, r, s} context predicates}
    (legacyBinding : Binding) (legacyInterpretation : TypeInterpretation legacyBinding) where
  represents : {domain codomain : AdmittedType legacyBinding legacyInterpretation} →
    Relation legacyBinding legacyInterpretation domain codomain → Prop
  formula : {domain codomain : AdmittedType legacyBinding legacyInterpretation} →
    (relation : Relation legacyBinding legacyInterpretation domain codomain) →
    represents relation → (Γ : context.Ctx) → RegularFormula binding Γ

end LegacyBridge

inductive TypedRelationalSurfaceObligation where
  | predecessorTypeGrammarCoverage
  | predecessorFormValueCorrespondence
  | arbitraryRelationRegularRepresentability
  | relationRepresentationSoundness
  | contextualTypeCodeBridge
  | noQuestionOrRuntimePromotion
  deriving DecidableEq, Repr

end InquiryCalculus.Successor.Relational

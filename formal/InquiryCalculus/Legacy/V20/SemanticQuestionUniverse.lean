import InquiryCalculus.Legacy.V20.PatternSeparation

/-! # Semantic question universe and conservative growth

Source-bound realization of v2.0 lines 4826–4856. A semantic question universe is represented
extensionally by its well-typed members; this finite model deliberately supplies no enumerator.
Conservative growth preserves each old well-typed question through an injective embedding. Strict
growth additionally needs a new well-typed question outside that image.
-/
namespace InquiryCalculus.Legacy.V20.SemanticQuestionUniverse

universe u v

structure QuestionPresentation (Question : Type u) where
  wellTyped : Question → Prop

structure ConservativeExtension (Old : Type u) (New : Type v)
    (source : QuestionPresentation Old) (target : QuestionPresentation New) where
  embed : Old → New
  injective : Function.Injective embed
  preservesWellTyped : ∀ question, source.wellTyped question → target.wellTyped (embed question)

def StrictGrowth {Old : Type u} {New : Type v} {source : QuestionPresentation Old}
    {target : QuestionPresentation New} (extension : ConservativeExtension Old New source target) : Prop :=
  ∃ newQuestion, target.wellTyped newQuestion ∧ ∀ oldQuestion, extension.embed oldQuestion ≠ newQuestion

namespace Countermodel

inductive OldQuestion where | base deriving DecidableEq
inductive ExtendedQuestion where | base | new deriving DecidableEq

def old : QuestionPresentation OldQuestion where
  wellTyped := fun _ => True

def unchanged : QuestionPresentation OldQuestion := old

def extended : QuestionPresentation ExtendedQuestion where
  wellTyped := fun _ => True

def replacement : QuestionPresentation OldQuestion where
  wellTyped := fun _ => False

def identityExtension : ConservativeExtension OldQuestion OldQuestion old unchanged where
  embed := id
  injective := fun _ _ equal => equal
  preservesWellTyped := fun _ _ => True.intro

def growingExtension : ConservativeExtension OldQuestion ExtendedQuestion old extended where
  embed := fun _ => .base
  injective := by intro left right _; cases left; cases right; rfl
  preservesWellTyped := fun _ _ => True.intro

def replacementEmbedding : OldQuestion → OldQuestion := id

theorem conservativeEmbeddingExists : Function.Injective growingExtension.embed := growingExtension.injective

theorem growthIsStrict : StrictGrowth growingExtension := by
  refine ⟨.new, True.intro, ?_⟩
  intro oldQuestion
  cases oldQuestion
  decide

theorem identityGrowthIsNotStrict : ¬ StrictGrowth identityExtension := by
  rintro ⟨newQuestion, _, outside⟩
  cases newQuestion
  exact outside .base rfl

theorem replacementDoesNotPreserve : ¬ replacement.wellTyped (replacementEmbedding .base) := by
  intro preserved
  exact preserved

end Countermodel
end InquiryCalculus.Legacy.V20.SemanticQuestionUniverse

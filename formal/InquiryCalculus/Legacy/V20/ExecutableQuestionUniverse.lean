import InquiryCalculus.Legacy.V20.SemanticQuestionUniverse

/-! # Executable question universe

Source-bound realization of v2.0 lines 4860–4904. An effectivity profile bounds generation,
decision, and policy resources. Its executable universe is a subset of the represented semantic
universe. A missed generation is not semantic nonexistence, and semantic/executable superscripts
classify the same checked occurrences rather than creating new occurrence species.
-/
namespace InquiryCalculus.Legacy.V20.ExecutableQuestionUniverse

universe u

structure EffectivityProfile (Question : Type u) where
  generationBudget : Nat
  dischargeBudget : Nat
  toolBudget : Nat
  deciderAvailable : Question → Prop
  semiDeciderAvailable : Question → Prop
  policyAllows : Question → Prop

structure QuestionUniverse (Question : Type u) where
  semantic : Question → Prop

def GeneratedUnder {Question : Type u} (profile : EffectivityProfile Question) (question : Question) : Prop :=
  0 < profile.generationBudget ∧
    (profile.deciderAvailable question ∨ profile.semiDeciderAvailable question) ∧ profile.policyAllows question

def Executable {Question : Type u} (semanticUniverse : QuestionUniverse Question)
    (profile : EffectivityProfile Question) (question : Question) : Prop :=
  semanticUniverse.semantic question ∧ GeneratedUnder profile question

theorem executableIsSemantic {Question : Type u} (semanticUniverse : QuestionUniverse Question)
    (profile : EffectivityProfile Question) (question : Question) :
    Executable semanticUniverse profile question → semanticUniverse.semantic question := fun executable => executable.1

structure AskOccurrence (Question : Type u) where
  askQuestion : Question

def SemanticOccurrence {Question : Type u} (semanticUniverse : QuestionUniverse Question)
    (occurrence : AskOccurrence Question) : Prop := semanticUniverse.semantic occurrence.askQuestion

def ExecutableOccurrence {Question : Type u} (semanticUniverse : QuestionUniverse Question)
    (profile : EffectivityProfile Question) (occurrence : AskOccurrence Question) : Prop :=
  SemanticOccurrence semanticUniverse occurrence ∧ Executable semanticUniverse profile occurrence.askQuestion

theorem executableOccurrenceIsSemantic {Question : Type u} (semanticUniverse : QuestionUniverse Question)
    (profile : EffectivityProfile Question) (occurrence : AskOccurrence Question) :
    ExecutableOccurrence semanticUniverse profile occurrence → SemanticOccurrence semanticUniverse occurrence := fun executable => executable.1

namespace Countermodel

inductive Question where | exposed | hidden deriving DecidableEq

def semanticUniverse : QuestionUniverse Question where
  semantic := fun _ => True

def low : EffectivityProfile Question where
  generationBudget := 0
  dischargeBudget := 0
  toolBudget := 0
  deciderAvailable := fun _ => False
  semiDeciderAvailable := fun _ => False
  policyAllows := fun _ => True

def high : EffectivityProfile Question where
  generationBudget := 1
  dischargeBudget := 1
  toolBudget := 1
  deciderAvailable := fun question => question = .exposed ∨ question = .hidden
  semiDeciderAvailable := fun _ => False
  policyAllows := fun _ => True

def hiddenOccurrence : AskOccurrence Question := ⟨.hidden⟩

theorem hiddenIsSemantic : semanticUniverse.semantic .hidden := True.intro

theorem hiddenIsNotExecutableLow : ¬ Executable semanticUniverse low .hidden := by
  intro executable
  exact Nat.not_lt_zero _ executable.2.1

theorem hiddenIsExecutableHigh : Executable semanticUniverse high .hidden := by
  refine ⟨True.intro, Nat.zero_lt_succ _, ?_, True.intro⟩
  left
  exact Or.inr rfl

theorem generationFailureDoesNotNegateSemantic :
    semanticUniverse.semantic .hidden ∧ ¬ Executable semanticUniverse low .hidden :=
  ⟨hiddenIsSemantic, hiddenIsNotExecutableLow⟩

theorem hiddenOccurrenceIsSemantic : SemanticOccurrence semanticUniverse hiddenOccurrence := True.intro

theorem hiddenOccurrenceIsNotExecutableLow :
    ¬ ExecutableOccurrence semanticUniverse low hiddenOccurrence := by
  intro executable
  exact hiddenIsNotExecutableLow executable.2

theorem hiddenOccurrenceIsExecutableHigh :
    ExecutableOccurrence semanticUniverse high hiddenOccurrence :=
  ⟨hiddenOccurrenceIsSemantic, hiddenIsExecutableHigh⟩

end Countermodel
end InquiryCalculus.Legacy.V20.ExecutableQuestionUniverse

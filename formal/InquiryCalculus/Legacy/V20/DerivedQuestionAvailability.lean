import InquiryCalculus.Legacy.V20.ExecutableQuestionUniverse
import InquiryCalculus.Legacy.V20.RequiredDischarge

/-! # Derived question availability and required discharge

Source-bound reconstruction of v2.0 lines 4906–4941.  The availability predicates classify a
represented question or its exact occurrence; they are not an execution policy, controller, or
successor primitive.  Required discharge reuses the declared route carrier and remains distinct
from execution, successful discharge, productivity, and authorization.
-/
namespace InquiryCalculus.Legacy.V20.DerivedQuestionAvailability

universe u v w

structure AvailabilityProfile (Question : Type u) (Occurrence : Type v) where
  formable : Question → Prop
  applicable : Question → Prop
  executable : Question → Prop
  answerable : Question → Prop
  productiveAlternatives : Occurrence → Prop
  allLiveAnswersProtectedEquivalent : Occurrence → Prop
  sufficientCoverage : Occurrence → Prop

def Productive {Question : Type u} {Occurrence : Type v}
    (profile : AvailabilityProfile Question Occurrence) (occurrence : Occurrence) : Prop :=
  profile.productiveAlternatives occurrence

def ResolvedQ {Question : Type u} {Occurrence : Type v}
    (profile : AvailabilityProfile Question Occurrence) (occurrence : Occurrence) : Prop :=
  profile.allLiveAnswersProtectedEquivalent occurrence ∧ profile.sufficientCoverage occurrence

def Ready {Question : Type u} {Occurrence : Type v}
    (profile : AvailabilityProfile Question Occurrence) (useContract : Question → Prop)
    (question : Question) : Prop :=
  profile.formable question ∧ profile.applicable question ∧ useContract question

def requiredDischargeAt {Occurrence : Type u} {Dependency : Type v}
    (explicitStandingOrSourceProgramDependency : Dependency → Prop) (openRelation : Occurrence → Prop)
    (mayLawfullyContinue : Occurrence → Prop)
    (discharged : Occurrence → RequiredDischargeRoute → Prop) (occurrence : Occurrence)
    (dependency : Dependency) (declaredRoute : RequiredDischargeRoute) : Prop :=
  explicitStandingOrSourceProgramDependency dependency ∧ openRelation occurrence ∧
    (mayLawfullyContinue occurrence → discharged occurrence declaredRoute)

theorem resolvedRequiresCoverage {Question : Type u} {Occurrence : Type v}
    (profile : AvailabilityProfile Question Occurrence) (occurrence : Occurrence) :
    ResolvedQ profile occurrence → profile.sufficientCoverage occurrence := fun resolved => resolved.2

namespace Countermodel

inductive Question where
  | outOfScope | noRoute | empty | discretionary | required | unresolved | unready
  deriving DecidableEq

inductive Occurrence where
  | noRoute | empty | discretionary | required | unresolved | unready
  deriving DecidableEq

inductive Dependency where | standing deriving DecidableEq

def semanticUniverse : ExecutableQuestionUniverse.QuestionUniverse Question where
  semantic := fun _ => True

def effectivity : ExecutableQuestionUniverse.EffectivityProfile Question where
  generationBudget := 1
  dischargeBudget := 1
  toolBudget := 1
  deciderAvailable := fun question => question ≠ .noRoute ∧ question ≠ .outOfScope
  semiDeciderAvailable := fun _ => False
  policyAllows := fun _ => True

def availability : AvailabilityProfile Question Occurrence where
  formable := fun _ => True
  applicable := fun question => question ≠ .outOfScope
  executable := ExecutableQuestionUniverse.Executable semanticUniverse effectivity
  answerable := fun question => question ≠ .empty
  productiveAlternatives := fun occurrence => occurrence = .discretionary
  allLiveAnswersProtectedEquivalent := fun occurrence => occurrence ≠ .discretionary
  sufficientCoverage := fun occurrence => occurrence ≠ .unresolved

def declaredUseContract : Question → Prop := fun question => question ≠ .unready

def standingDependency : Dependency → Prop := fun dependency => dependency = .standing

def openRelation : Occurrence → Prop := fun occurrence => occurrence = .required

def mayLawfullyContinue : Occurrence → Prop := fun _ => True

def discharged : Occurrence → RequiredDischargeRoute → Prop :=
  fun occurrence route => occurrence = .required ∧ route = .check

theorem formableButInapplicable :
    availability.formable .outOfScope ∧ ¬ availability.applicable .outOfScope :=
  ⟨True.intro, by simp [availability]⟩

theorem formableButNonexecutable :
    availability.formable .noRoute ∧ ¬ availability.executable .noRoute := by
  refine ⟨True.intro, ?_⟩
  intro executable
  rcases executable.2.2.1 with decider | semiDecider
  · exact decider.1 rfl
  · exact semiDecider.elim

theorem executableEmptyButNotAnswerable :
    availability.executable .empty ∧ ¬ availability.answerable .empty := by
  refine ⟨?_, by simp [availability]⟩
  refine ⟨True.intro, Nat.zero_lt_succ _, ?_, True.intro⟩
  exact Or.inl ⟨by simp, by simp⟩

theorem incompleteCoverageDoesNotResolve :
    availability.allLiveAnswersProtectedEquivalent .unresolved ∧
      ¬ ResolvedQ availability .unresolved := by
  refine ⟨by simp [availability], ?_⟩
  intro resolved
  exact resolved.2 rfl

theorem formableApplicableButNotReady :
    availability.formable .unready ∧ availability.applicable .unready ∧
      ¬ Ready availability declaredUseContract .unready := by
  refine ⟨True.intro, by simp [availability], ?_⟩
  intro ready
  exact ready.2.2 rfl

theorem nonproductiveRequired :
    ¬ Productive availability .required ∧
      requiredDischargeAt standingDependency openRelation mayLawfullyContinue discharged
        .required .standing .check := by
  refine ⟨by simp [Productive, availability], ?_⟩
  refine ⟨rfl, rfl, ?_⟩
  intro _
  exact ⟨rfl, rfl⟩

theorem productiveNotRequired :
    Productive availability .discretionary ∧
      ¬ requiredDischargeAt standingDependency openRelation mayLawfullyContinue discharged
        .discretionary .standing .check := by
  refine ⟨rfl, ?_⟩
  intro required
  simpa [openRelation] using required.2.1

theorem requiredDoesNotAssertExecution :
    requiredDischargeAt standingDependency openRelation mayLawfullyContinue discharged
      .required .standing .check ∧ ¬ availability.executable .noRoute :=
  ⟨nonproductiveRequired.2, formableButNonexecutable.2⟩

end Countermodel
end InquiryCalculus.Legacy.V20.DerivedQuestionAvailability

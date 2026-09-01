import InquiryCalculus.Legacy.V20.SupportEnvironments

/-! # Open dependency boundary

Source-bound realization of v2.0 lines 4537–4560. Openness is positive and relative
to an exact candidate, support environment, requirement relation, supply relation, and
independent-discharge relation. A positively open dependency constructs an indexed ordinary
question target, not an Ask occurrence, Answer, event, support result, or standing result.
-/
namespace InquiryCalculus.Legacy.V20.OpenDependencyBoundary

universe u v w x

structure DependencyContext (Candidate : Type u) (Environment : Type v)
    (Dependency : Type w) (Question : Type x) where
  required : Candidate → Dependency → Prop
  suppliedBy : Environment → Dependency → Prop
  independentlyDischarged : Candidate → Dependency → Prop
  questionFor : Environment → Candidate → Dependency → Question

def IsOpenDependency {Candidate : Type u} {Environment : Type v}
    {Dependency : Type w} {Question : Type x}
    (context : DependencyContext Candidate Environment Dependency Question)
    (environment : Environment) (candidate : Candidate) (dependency : Dependency) : Prop :=
  context.required candidate dependency ∧
    ¬ context.suppliedBy environment dependency ∧
    ¬ context.independentlyDischarged candidate dependency

/-- This value is only a lawful target for ordinary inquiry. It contains no answer or execution. -/
structure OpenDependencyQuestionTarget {Candidate : Type u} {Environment : Type v}
    {Dependency : Type w} {Question : Type x}
    (context : DependencyContext Candidate Environment Dependency Question)
    (environment : Environment) (candidate : Candidate) where
  dependency : Dependency
  openAtEnvironment : IsOpenDependency context environment candidate dependency
  question : Question
  exactQuestion : question = context.questionFor environment candidate dependency

def toQuestionTarget {Candidate : Type u} {Environment : Type v}
    {Dependency : Type w} {Question : Type x}
    (context : DependencyContext Candidate Environment Dependency Question)
    (environment : Environment) (candidate : Candidate) (dependency : Dependency)
    (openDependency : IsOpenDependency context environment candidate dependency) :
    OpenDependencyQuestionTarget context environment candidate where
  dependency := dependency
  openAtEnvironment := openDependency
  question := context.questionFor environment candidate dependency
  exactQuestion := rfl

theorem questionTargetRetainsPositiveBoundary {Candidate : Type u} {Environment : Type v}
    {Dependency : Type w} {Question : Type x}
    {context : DependencyContext Candidate Environment Dependency Question}
    {environment : Environment} {candidate : Candidate}
    (target : OpenDependencyQuestionTarget context environment candidate) :
    IsOpenDependency context environment candidate target.dependency :=
  target.openAtEnvironment

theorem questionTargetRetainsExactRendering {Candidate : Type u} {Environment : Type v}
    {Dependency : Type w} {Question : Type x}
    {context : DependencyContext Candidate Environment Dependency Question}
    {environment : Environment} {candidate : Candidate}
    (target : OpenDependencyQuestionTarget context environment candidate) :
    target.question = context.questionFor environment candidate target.dependency :=
  target.exactQuestion

namespace Countermodel

open SupportEnvironments.Countermodel

inductive Dependency where
  | supplied
  | independentlyClosed
  | unresolved
  | irrelevant
  deriving DecidableEq

structure Question where
  environment : Environment
  candidate : Unit
  dependency : Dependency
  deriving DecidableEq

def baseEnvironment : Environment := leftEnvironment
def expandedEnvironment : Environment := unionEnvironment

def context : DependencyContext Unit Environment Dependency Question where
  required := fun _ dependency => dependency ≠ .irrelevant
  suppliedBy := fun environment dependency =>
    (environment = baseEnvironment ∧ dependency = .supplied) ∨
      (environment = expandedEnvironment ∧
        (dependency = .supplied ∨ dependency = .unresolved))
  independentlyDischarged := fun _ dependency => dependency = .independentlyClosed
  questionFor := fun environment candidate dependency => ⟨environment, candidate, dependency⟩

def independentlyExpandedContext : DependencyContext Unit Environment Dependency Question :=
  { context with
    independentlyDischarged := fun _ dependency =>
      dependency = .independentlyClosed ∨ dependency = .unresolved }

theorem onlyUnresolvedIsOpenAtBase (dependency : Dependency) :
    IsOpenDependency context baseEnvironment () dependency ↔ dependency = .unresolved := by
  cases dependency <;> unfold IsOpenDependency context baseEnvironment <;> decide

theorem suppliedDependencyIsRequiredAndSupplied :
    context.required () .supplied ∧
      context.suppliedBy baseEnvironment .supplied ∧
      ¬ IsOpenDependency context baseEnvironment () .supplied := by
  unfold IsOpenDependency context baseEnvironment
  decide

theorem independentDischargeIsNotEnvironmentSupply :
    context.required () .independentlyClosed ∧
      ¬ context.suppliedBy baseEnvironment .independentlyClosed ∧
      context.independentlyDischarged () .independentlyClosed ∧
      ¬ IsOpenDependency context baseEnvironment () .independentlyClosed := by
  unfold IsOpenDependency context baseEnvironment
  decide

theorem notSuppliedDoesNotEstablishRequirementOrOpenness :
    ¬ context.suppliedBy baseEnvironment .irrelevant ∧
      ¬ context.required () .irrelevant ∧
      ¬ IsOpenDependency context baseEnvironment () .irrelevant := by
  unfold IsOpenDependency context baseEnvironment
  decide

theorem unresolvedDependencyIsPositivelyOpen :
    context.required () .unresolved ∧
      ¬ context.suppliedBy baseEnvironment .unresolved ∧
      ¬ context.independentlyDischarged () .unresolved ∧
      IsOpenDependency context baseEnvironment () .unresolved := by
  unfold IsOpenDependency context baseEnvironment
  decide

def unresolvedQuestionTarget : OpenDependencyQuestionTarget context baseEnvironment () :=
  toQuestionTarget context baseEnvironment () .unresolved unresolvedDependencyIsPositivelyOpen.2.2.2

theorem targetPreservesExactEnvironmentCandidateAndDependency :
    unresolvedQuestionTarget.question = ⟨baseEnvironment, (), .unresolved⟩ ∧
      unresolvedQuestionTarget.dependency = .unresolved := by
  exact ⟨rfl, rfl⟩

theorem environmentSupplyClosesOnlyTheLocalBoundary :
    IsOpenDependency context baseEnvironment () .unresolved ∧
      ¬ IsOpenDependency context expandedEnvironment () .unresolved ∧
      context.required () .unresolved := by
  unfold IsOpenDependency context baseEnvironment expandedEnvironment
  decide

theorem independentDischargeClosesWithoutSupplying :
    IsOpenDependency context baseEnvironment () .unresolved ∧
      ¬ IsOpenDependency independentlyExpandedContext baseEnvironment () .unresolved ∧
      ¬ independentlyExpandedContext.suppliedBy baseEnvironment .unresolved ∧
      independentlyExpandedContext.independentlyDischarged () .unresolved := by
  unfold IsOpenDependency independentlyExpandedContext context baseEnvironment
  decide

end Countermodel
end InquiryCalculus.Legacy.V20.OpenDependencyBoundary

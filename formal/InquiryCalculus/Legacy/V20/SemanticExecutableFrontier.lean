import InquiryCalculus.Legacy.V20.RequiredNondominantFrontier

/-! # v2.0 semantic and executable frontier boundary

Source-bound reconstruction of v2.0 lines 5003–5054.  Semantic and executable candidate
membership retain their distinct prerequisites before each is projected through the already checked
required-nondominant carrier.  This module records membership only; it does not select work.
-/
namespace InquiryCalculus.Legacy.V20.SemanticExecutableFrontier

open RequiredNondominantFrontier

universe u v

inductive ExecutableClass where | productive | idle | unknown deriving DecidableEq

structure MembershipContext (Occurrence : Type u) (Dependency : Type v) where
  formable : Occurrence → Prop
  applicable : Occurrence → Prop
  executable : Occurrence → Prop
  productive : Occurrence → Prop
  resolved : Occurrence → Prop
  executableClass : Occurrence → ExecutableClass
  order : FrontierContext Occurrence Dependency
  semanticField : List Occurrence
  executableField : List Occurrence

def SemanticCandidate {Occurrence : Type u} {Dependency : Type v}
    (context : MembershipContext Occurrence Dependency) (occurrence : Occurrence) : Prop :=
  context.formable occurrence ∧ context.applicable occurrence ∧
    ((context.productive occurrence ∧ ¬ context.resolved occurrence) ∨
      ∃ dependency, context.order.required occurrence dependency)

def ExecutableCandidate {Occurrence : Type u} {Dependency : Type v}
    (context : MembershipContext Occurrence Dependency) (occurrence : Occurrence) : Prop :=
  context.formable occurrence ∧ context.applicable occurrence ∧ context.executable occurrence ∧
    (context.executableClass occurrence = .productive ∨
      ∃ dependency, context.order.required occurrence dependency)

def SemanticFrontier {Occurrence : Type u} {Dependency : Type v}
    (context : MembershipContext Occurrence Dependency) (occurrence : Occurrence) : Prop :=
  SemanticCandidate context occurrence ∧ RequiredNondominant context.order context.semanticField occurrence

def ExecutableFrontier {Occurrence : Type u} {Dependency : Type v}
    (context : MembershipContext Occurrence Dependency) (occurrence : Occurrence) : Prop :=
  ExecutableCandidate context occurrence ∧ RequiredNondominant context.order context.executableField occurrence

theorem executableFrontierIsExecutable {Occurrence : Type u} {Dependency : Type v}
    (context : MembershipContext Occurrence Dependency) (occurrence : Occurrence) :
    ExecutableFrontier context occurrence → context.executable occurrence := fun membership => membership.1.2.2.1

namespace Countermodel

inductive Occurrence where
  | semanticProductive | executableProductive | executableIdle | executableUnknown | requiredIdle
  | inapplicable | nonformable
  deriving DecidableEq

inductive Dependency where | standing deriving DecidableEq

def noDominanceOrder : FrontierContext Occurrence Dependency where
  required := fun occurrence dependency => occurrence = .requiredIdle ∧ dependency = .standing
  strictlyDominatedBy := fun _ _ => False
  discharges := fun _ _ => False

def context : MembershipContext Occurrence Dependency where
  formable := fun occurrence => occurrence ≠ .nonformable
  applicable := fun occurrence => occurrence ≠ .inapplicable
  executable := fun occurrence => occurrence ≠ .semanticProductive ∧ occurrence ≠ .inapplicable ∧ occurrence ≠ .nonformable
  productive := fun occurrence => occurrence = .semanticProductive ∨ occurrence = .executableProductive
  resolved := fun _ => False
  executableClass := fun occurrence =>
    if occurrence = .executableProductive then .productive
    else if occurrence = .executableUnknown then .unknown
    else .idle
  order := noDominanceOrder
  semanticField := [.semanticProductive, .executableProductive, .requiredIdle]
  executableField := [.executableProductive, .requiredIdle]

theorem semanticProductiveIsSemanticCandidate : SemanticCandidate context .semanticProductive := by
  refine ⟨by simp [context], by simp [context], Or.inl ⟨?_, ?_⟩⟩
  · simp [context]
  · simp [context]

theorem semanticProductiveIsNotExecutableCandidate : ¬ ExecutableCandidate context .semanticProductive := by
  intro candidate
  exact candidate.2.2.1.1 rfl

theorem executableProductiveIsExecutableCandidate : ExecutableCandidate context .executableProductive := by
  refine ⟨by simp [context], by simp [context], by simp [context], Or.inl ?_⟩
  simp [context]

theorem executableIdleIsNotExecutableCandidate : ¬ ExecutableCandidate context .executableIdle := by
  intro candidate
  rcases candidate.2.2.2 with productive | required
  · change ExecutableClass.idle = ExecutableClass.productive at productive
    cases productive
  · rcases required with ⟨_, impossible, _⟩
    cases impossible

theorem executableUnknownIsNotExecutableCandidate : ¬ ExecutableCandidate context .executableUnknown := by
  intro candidate
  rcases candidate.2.2.2 with productive | required
  · change ExecutableClass.unknown = ExecutableClass.productive at productive
    cases productive
  · rcases required with ⟨_, impossible, _⟩
    cases impossible

theorem requiredIdleIsExecutableCandidate : ExecutableCandidate context .requiredIdle := by
  refine ⟨by simp [context], by simp [context], by simp [context], Or.inr ?_⟩
  exact ⟨.standing, rfl, rfl⟩

theorem inapplicableIsNotSemanticCandidate : ¬ SemanticCandidate context .inapplicable := by
  intro candidate
  exact candidate.2.1 rfl

theorem semanticProductiveIsSemanticFrontier : SemanticFrontier context .semanticProductive := by
  refine ⟨semanticProductiveIsSemanticCandidate, Or.inr ?_⟩
  refine ⟨by simp [context], ?_⟩
  intro _ _ dominated
  exact dominated.elim

theorem executableProductiveIsExecutableFrontier : ExecutableFrontier context .executableProductive := by
  refine ⟨executableProductiveIsExecutableCandidate, Or.inr ?_⟩
  refine ⟨by simp [context], ?_⟩
  intro _ _ dominated
  exact dominated.elim

end Countermodel
end InquiryCalculus.Legacy.V20.SemanticExecutableFrontier

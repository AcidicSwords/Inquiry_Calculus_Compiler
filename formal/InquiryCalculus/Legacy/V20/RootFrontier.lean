import InquiryCalculus.Legacy.V20.AnswerConditionedUnlock
import InquiryCalculus.Legacy.V20.SemanticExecutableFrontier
import InquiryCalculus.Legacy.V20.DerivedInterrogativeRoots

/-! # v2.0 root frontier

Source-bound reconstruction of the ambiguous and unproved root-frontier boundary at v2.0 lines
5163–5183. The definition is a transparent composition of admitted root production, the existing
executable-candidate relation, and the existing required-safe nondominant projection. It adds no
selection, scheduler, root-wheel, or closure authority.
-/
namespace InquiryCalculus.Legacy.V20.RootFrontier

open RequiredNondominantFrontier SemanticExecutableFrontier

universe u v

structure RootFrontierContext (Occurrence : Type u) (Dependency : Type v) where
  rootProduced : Occurrence → Prop
  admitted : Occurrence → Prop
  membership : MembershipContext Occurrence Dependency
  rootField : List Occurrence

def Eligible {Occurrence : Type u} {Dependency : Type v}
    (context : RootFrontierContext Occurrence Dependency) (occurrence : Occurrence) : Prop :=
  context.rootProduced occurrence ∧ context.admitted occurrence ∧
    ExecutableCandidate context.membership occurrence

def RootFrontier {Occurrence : Type u} {Dependency : Type v}
    (context : RootFrontierContext Occurrence Dependency) (occurrence : Occurrence) : Prop :=
  Eligible context occurrence ∧
    RequiredNondominant context.membership.order context.rootField occurrence

def ExactEligibleField {Occurrence : Type u} {Dependency : Type v}
    (context : RootFrontierContext Occurrence Dependency) : Prop :=
  ∀ occurrence, occurrence ∈ context.rootField ↔ Eligible context occurrence

theorem rootFrontierIsEligible {Occurrence : Type u} {Dependency : Type v}
    (context : RootFrontierContext Occurrence Dependency) (occurrence : Occurrence) :
    RootFrontier context occurrence → Eligible context occurrence := fun frontier => frontier.1

theorem missingPreorderPreservesRootField {Occurrence : Type u} {Dependency : Type v}
    (context : RootFrontierContext Occurrence Dependency) :
    NoPreorderFrontier context.rootField = context.rootField := rfl

inductive RootFrontierObligation where
  | admittedRootProduction
  | formable
  | applicable
  | executable
  | productiveOrRequired
  | requiredSafeNondominance
  | bindingSuppliedPreorder
  | noPreorderIdentity
  | requiredDependencyProtection
  | reconstructAfterReturn
  | noRootWheelOrScheduler
  | noGlobalClosure
  | noAnswerSelectionOrExecution
  | noProgramOrRustAuthority
  deriving DecidableEq

namespace Countermodel

inductive Occurrence where
  | productive | required | dominatedOptional | nonRoot | unadmitted | nonformable
  | inapplicable | nonexecutable | idle
  deriving DecidableEq

structure FiniteProfile where
  rootProduced : Bool
  admitted : Bool
  formable : Bool
  applicable : Bool
  executable : Bool
  productive : Bool
  required : Bool

def profile (occurrence : Occurrence) : FiniteProfile where
  rootProduced := !(occurrence == .nonRoot)
  admitted := !(occurrence == .unadmitted)
  formable := !(occurrence == .nonformable)
  applicable := !(occurrence == .inapplicable)
  executable := !(occurrence == .nonexecutable) && !(occurrence == .nonformable)
  productive := occurrence == .productive || occurrence == .dominatedOptional
  required := occurrence == .required

def eligibleFlag (occurrence : Occurrence) : Bool :=
  let candidate := profile occurrence
  candidate.rootProduced && candidate.admitted && candidate.formable && candidate.applicable &&
    candidate.executable && (candidate.productive || candidate.required)

def rootField : List Occurrence := [.productive, .required, .dominatedOptional]

def dominatedFlag (occurrence other : Occurrence) : Bool :=
  (occurrence == .required || occurrence == .dominatedOptional) && other == .productive

def nondominatedFlag (occurrence : Occurrence) : Bool :=
  rootField.all fun other => !(dominatedFlag occurrence other)

def rootFrontierFlag (occurrence : Occurrence) : Bool :=
  eligibleFlag occurrence && (profile occurrence).required ||
    eligibleFlag occurrence && nondominatedFlag occurrence

theorem eligibleFieldIsExact :
    rootField.all eligibleFlag = true ∧
      eligibleFlag .nonRoot = false ∧ eligibleFlag .unadmitted = false ∧
      eligibleFlag .nonformable = false ∧ eligibleFlag .inapplicable = false ∧
      eligibleFlag .nonexecutable = false ∧ eligibleFlag .idle = false := by
  decide

theorem productiveIsRootFrontier : rootFrontierFlag .productive = true := by decide

theorem requiredIsOrdinarilyDominated : dominatedFlag .required .productive = true := by decide

theorem requiredIsRootFrontier : rootFrontierFlag .required = true := by decide

theorem dominatedOptionalIsNotRootFrontier :
    rootFrontierFlag .dominatedOptional = false := by decide

theorem nonRootIsNotEligible : eligibleFlag .nonRoot = false := by decide

theorem rootProducedDoesNotImplyAdmission :
    (profile .unadmitted).rootProduced = true ∧ (profile .unadmitted).admitted = false := by
  decide

theorem nonformableIsNotEligible : eligibleFlag .nonformable = false := by decide

theorem inapplicableIsNotEligible : eligibleFlag .inapplicable = false := by decide

theorem nonexecutableIsNotEligible : eligibleFlag .nonexecutable = false := by decide

theorem idleIsNotEligible : eligibleFlag .idle = false := by decide

theorem missingPreorderRetainsEveryEligibleCandidate :
    NoPreorderFrontier rootField = rootField := rfl

end Countermodel
end InquiryCalculus.Legacy.V20.RootFrontier

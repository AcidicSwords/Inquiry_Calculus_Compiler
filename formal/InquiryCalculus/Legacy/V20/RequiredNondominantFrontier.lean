import InquiryCalculus.Legacy.V20.QuestionProductivity

/-! # v2.0 required-protected nondominant frontier boundary

Source-bound reconstruction of v2.0 lines 4981–5001.  Required occurrences are unioned with
ordinary nondominated occurrences.  A stronger removal needs a retained witness discharging every
same dependency; this local contract neither selects work nor supplies a resource preorder.
-/
namespace InquiryCalculus.Legacy.V20.RequiredNondominantFrontier

universe u v

structure FrontierContext (Occurrence : Type u) (Dependency : Type v) where
  required : Occurrence → Dependency → Prop
  strictlyDominatedBy : Occurrence → Occurrence → Prop
  discharges : Occurrence → Dependency → Prop

def RequiredSet {Occurrence : Type u} {Dependency : Type v}
    (context : FrontierContext Occurrence Dependency) (candidates : List Occurrence)
    (occurrence : Occurrence) : Prop :=
  occurrence ∈ candidates ∧ ∃ dependency, context.required occurrence dependency

def Nondominated {Occurrence : Type u} {Dependency : Type v}
    (context : FrontierContext Occurrence Dependency) (candidates : List Occurrence)
    (occurrence : Occurrence) : Prop :=
  occurrence ∈ candidates ∧ ∀ other, other ∈ candidates → ¬ context.strictlyDominatedBy occurrence other

def RequiredNondominant {Occurrence : Type u} {Dependency : Type v}
    (context : FrontierContext Occurrence Dependency) (candidates : List Occurrence)
    (occurrence : Occurrence) : Prop :=
  RequiredSet context candidates occurrence ∨ Nondominated context candidates occurrence

def NoPreorderFrontier {Occurrence : Type u} (candidates : List Occurrence) : List Occurrence := candidates

def RequiredRemovalLicensed {Occurrence : Type u} {Dependency : Type v}
    (context : FrontierContext Occurrence Dependency) (retained : List Occurrence)
    (occurrence : Occurrence) : Prop :=
  ∀ dependency, context.required occurrence dependency →
    ∃ replacement, replacement ∈ retained ∧ replacement ≠ occurrence ∧ context.discharges replacement dependency

theorem requiredIsRetained {Occurrence : Type u} {Dependency : Type v}
    (context : FrontierContext Occurrence Dependency) (candidates : List Occurrence)
    (occurrence : Occurrence) :
    RequiredSet context candidates occurrence → RequiredNondominant context candidates occurrence := Or.inl

namespace Countermodel

inductive Occurrence where | required | alphaSubstitute | betaSubstitute | optional deriving DecidableEq

inductive Dependency where | alpha | beta deriving DecidableEq

def candidates : List Occurrence := [.required, .alphaSubstitute, .betaSubstitute, .optional]

def context : FrontierContext Occurrence Dependency where
  required := fun occurrence dependency => occurrence = .required ∧ (dependency = .alpha ∨ dependency = .beta)
  strictlyDominatedBy := fun occurrence other =>
    (occurrence = .required ∧ other = .alphaSubstitute) ∨
      (occurrence = .optional ∧ other = .required)
  discharges := fun occurrence dependency =>
    (occurrence = .alphaSubstitute ∧ dependency = .alpha) ∨
      (occurrence = .betaSubstitute ∧ dependency = .beta)

theorem requiredIsOrdinarilyDominated :
    ∃ other, other ∈ candidates ∧ context.strictlyDominatedBy .required other :=
  ⟨.alphaSubstitute, by simp [candidates], Or.inl ⟨rfl, rfl⟩⟩

theorem requiredSurvivesOrdinaryDominance : RequiredNondominant context candidates .required := by
  left
  refine ⟨by simp [candidates], .alpha, ?_⟩
  exact ⟨rfl, Or.inl rfl⟩

theorem optionalIsDominatedByRequired : context.strictlyDominatedBy .optional .required :=
  Or.inr ⟨rfl, rfl⟩

theorem optionalIsNotRequiredNondominant : ¬ RequiredNondominant context candidates .optional := by
  intro frontier
  cases frontier with
  | inl required => simp [RequiredSet, context] at required
  | inr nondominated =>
      exact nondominated.2 .required (by simp [candidates]) optionalIsDominatedByRequired

theorem typedSubstitutesLicenseRequiredRemoval :
    RequiredRemovalLicensed context [.alphaSubstitute, .betaSubstitute] .required := by
  intro dependency required
  rcases required with ⟨_, alphaOrBeta⟩
  cases alphaOrBeta with
  | inl alpha =>
      refine ⟨.alphaSubstitute, by simp, by simp, ?_⟩
      exact Or.inl ⟨rfl, alpha⟩
  | inr beta =>
      refine ⟨.betaSubstitute, by simp, by simp, ?_⟩
      exact Or.inr ⟨rfl, beta⟩

theorem alphaSubstituteDoesNotDischargeBeta : ¬ context.discharges .alphaSubstitute .beta := by
  simp [context]

theorem missingPreorderRetainsAll : NoPreorderFrontier candidates = candidates := rfl

end Countermodel
end InquiryCalculus.Legacy.V20.RequiredNondominantFrontier

import InquiryCalculus.Legacy.V20.QuestionOrderDiagnostics

/-! # v2.0 protected behavioral equivalence

Source: Inquiry_Calculus_v2_0.tex, lines 4002–4059. These definitions preserve
the predecessor's relation shapes. Contexts and consequences are supplied by a
binding; this module implements no context evaluator or equivalence decision.
Law statements below are propositions, not proofs or successor authority.
-/
namespace InquiryCalculus.Legacy.V20

universe u

/-- Endpoint indexing keeps the two compared terms parallel. Consequences are
context-indexed: equality compares the terms in the same applicable context. -/
structure ProtectedBehavioralEquivalenceContext (Object : Type u)
    (Term : Object → Object → Type u) where
  Context : Object → Object → Type u
  Consequence : {A B : Object} → Context A B → Type u
  consequence : {A B : Object} → (K : Context A B) → Term A B → Consequence K

variable {Object : Type u} {Term : Object → Object → Type u}

/-- PRED-TEX-DECL-CA927522E6D4370C: equality in every protected context. -/
def protectedEquivalenceDefinitionShape
    (S : ProtectedBehavioralEquivalenceContext Object Term) {A B : Object}
    (H : S.Context A B → Prop) (f g : Term A B) : Prop :=
  ∀ K, H K → S.consequence K f = S.consequence K g

/-- PRED-TEX-DECL-34FF1494752E4183: the whole separating subfamily, not a chosen return. -/
def separatorFamilyDefinitionShape
    (S : ProtectedBehavioralEquivalenceContext Object Term) {A B : Object}
    (H : S.Context A B → Prop) (f g : Term A B) : S.Context A B → Prop :=
  fun K => H K ∧ S.consequence K f ≠ S.consequence K g

/-- The source's separator characterization is retained as an unproved claim. -/
def separatorCharacterizationUnproved
    (S : ProtectedBehavioralEquivalenceContext Object Term) {A B : Object}
    (H : S.Context A B → Prop) (f g : Term A B) : Prop :=
  (¬ protectedEquivalenceDefinitionShape S H f g) ↔
    ∃ K, separatorFamilyDefinitionShape S H f g K

/-- PRED-TEX-DECL-2FF1BAC4CB0C0B75: a list represents finite tested membership;
duplicates do not increase coverage. No completeness license is supplied here. -/
def workingNondistinctionDefinitionShape
    (S : ProtectedBehavioralEquivalenceContext Object Term) {A B : Object}
    (D : List (S.Context A B)) (f g : Term A B) : Prop :=
  ∀ K, K ∈ D → S.consequence K f = S.consequence K g

/-- PRED-TEX-DECL-E862C58481AED1AA: horizon inclusion reverses the inclusion
of equivalence relations. This definition names a claim; it does not prove it. -/
def horizonMonotonicityUnproved
    (S : ProtectedBehavioralEquivalenceContext Object Term) {A B : Object}
    (H H' : S.Context A B → Prop) : Prop :=
  (∀ K, H K → H' K) → ∀ f g,
    protectedEquivalenceDefinitionShape S H' f g →
      protectedEquivalenceDefinitionShape S H f g

/-- Binding-scoped coordinates. Tested coverage must lie within the protected
horizon; this carrier does not assert that either equivalence actually holds. -/
structure ProtectedBehavioralEquivalenceSyntax (Object : Type u)
    (Term : Object → Object → Type u) where
  signature : ProtectedBehavioralEquivalenceContext Object Term
  sourceObject : Object
  targetObject : Object
  leftTerm : Term sourceObject targetObject
  rightTerm : Term sourceObject targetObject
  protectedHorizon : signature.Context sourceObject targetObject → Prop
  testedDiscriminatorSet : List (signature.Context sourceObject targetObject)
  testedWithinHorizon : ∀ K, K ∈ testedDiscriminatorSet → protectedHorizon K

inductive ProtectedBehavioralEquivalenceObligation where
  | protectedEquivalenceDefinitionShape
  | separatorFamilyDefinitionShape
  | workingNondistinctionDefinitionShape
  | horizonMonotonicityUnproved
  | separatorCharacterizationUnproved
  | completenessLicenseAbsent
  | noContextExecution
  | noConsequenceEvaluation
  | noEquivalenceDecision
  | noSeparatorFinding
  | noHorizonInclusionProof
  | noSemanticAuthorityPromotion
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

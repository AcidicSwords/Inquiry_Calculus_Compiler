import InquiryCalculus.Legacy.V20.ProtectedDetermination

/-! # v2.0 representation-question boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w

/-- Relevance is a declared predicate on a candidate representation. -/
def RepresentationRelevant {X : Type u} {S : Type v} := (X → S) → Prop

/-- A candidate representation question distinguishes the fixed pair and is declared relevant. -/
def separatingRepresentationQuestion {X : Type u} {S : Type v}
    (relevant : RepresentationRelevant (X := X) (S := S)) (left right : X)
    (candidate : X → S) : Prop :=
  candidate left ≠ candidate right ∧ relevant candidate

/-- Exact functional sufficiency is scoped factorization through a candidate representation. -/
def representationFactorSufficient {X : Type u} {S : Type v} {K : Type w}
    (scope : X → Prop) (consequence : X → K) (candidate : X → S) : Prop :=
  ∃ factor : S → K, ∀ point, scope point → consequence point = factor (candidate point)

/-- Protected sufficiency is distinct: equal candidate values imply declared horizon equivalence. -/
def representationProtectedSufficient {X : Type u} {S : Type v}
    (scope : X → Prop) (horizon : HorizonEquivalenceSyntax X) (candidate : X → S) : Prop :=
  ∀ ⦃left right⦄, scope left → scope right → candidate left = candidate right → horizon.equivalent left right

/-- Candidate selection, sufficiency proof, and execution remain open. -/
inductive RepresentationQuestionObligation where
  | candidateSelection
  | relevanceMeaning
  | factorizationProof
  | protectedSufficiencyProof
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

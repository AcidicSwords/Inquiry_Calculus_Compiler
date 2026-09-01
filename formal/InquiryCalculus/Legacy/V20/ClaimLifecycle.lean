import InquiryCalculus.Legacy.V20.HistoricalReconstruction

/-! # Claim lifecycle

Source-bound realization of v2.0 lines 4483–4512. Generation, semantic truth,
partial reification, candidacy, warrant, and standing remain distinct. A failed
reification returns typed inquiry material rather than a negative judgment.
-/
namespace InquiryCalculus.Legacy.V20.ClaimLifecycle

universe u v

structure Claim (Statement : Type u) where
  statement : Statement

structure Candidate (Statement : Type u) where
  source : Claim Statement
  statement : Statement
  preservesStatement : statement = source.statement

/-- A reified candidate remains tied to the exact claim occurrence supplied to reification. -/
structure CandidateFor {Statement : Type u} (claim : Claim Statement) where
  candidate : Candidate Statement
  preservesSource : candidate.source = claim

inductive ReificationFailureKind where
  | decomposition
  | clarification
  | reExpression
  | binding
  | unknownStatus
  deriving DecidableEq

structure ReificationResidual (Inquiry : Type v) where
  kind : ReificationFailureKind
  inquiry : Inquiry

inductive ReificationResult (Candidate : Type u) (Inquiry : Type v) where
  | success : Candidate → ReificationResult Candidate Inquiry
  | failure : ReificationResidual Inquiry → ReificationResult Candidate Inquiry

inductive UnsupportedPromotionBasis where
  | questionSyntax
  | answerType
  | modelConfidence
  | generation
  | candidacy
  deriving DecidableEq

/-- The predecessor's expressly rejected promotion bases have no standing authority. -/
def UnsupportedPromotionBasis.authorizesStanding
    {Statement : Type u} (_ : UnsupportedPromotionBasis) (_ : Candidate Statement) : Prop :=
  False

structure Lifecycle (Statement : Type u) (Inquiry : Type v) where
  truth : Statement → Prop
  generated : Claim Statement → Prop
  reify : (claim : Claim Statement) → ReificationResult (CandidateFor claim) Inquiry
  warranted : Candidate Statement → Prop
  standing : Statement → Prop
  standingRequiresWarrant : ∀ statement, standing statement →
    ∃ candidate, candidate.statement = statement ∧ warranted candidate

theorem Candidate.retainsStatement {Statement : Type u} (candidate : Candidate Statement) :
    candidate.statement = candidate.source.statement :=
  candidate.preservesStatement

theorem standingRequiresWarrant {Statement : Type u} {Inquiry : Type v}
    (context : Lifecycle Statement Inquiry) {statement : Statement}
    (standing : context.standing statement) :
    ∃ candidate, candidate.statement = statement ∧ context.warranted candidate :=
  context.standingRequiresWarrant statement standing

theorem noStandingWithoutWarrantedCandidate {Statement : Type u} {Inquiry : Type v}
    (context : Lifecycle Statement Inquiry) (statement : Statement)
    (noneWarranted : ∀ candidate, candidate.statement = statement → ¬ context.warranted candidate) :
    ¬ context.standing statement := by
  intro standing
  obtain ⟨candidate, sameStatement, warranted⟩ := context.standingRequiresWarrant statement standing
  exact noneWarranted candidate sameStatement warranted

theorem unsupportedBasisNeverAuthorizesStanding {Statement : Type u}
    (basis : UnsupportedPromotionBasis) (candidate : Candidate Statement) :
    ¬ basis.authorizesStanding candidate := by
  intro authorization
  exact authorization

namespace Countermodel

def falseClaim : Claim Bool := ⟨false⟩
def trueClaim : Claim Bool := ⟨true⟩

def falseCandidate : Candidate Bool where
  source := falseClaim
  statement := false
  preservesStatement := rfl

def falseCandidateFor : CandidateFor falseClaim where
  candidate := falseCandidate
  preservesSource := rfl

def failedInquiry : ReificationResidual Unit where
  kind := .binding
  inquiry := ()

def context : Lifecycle Bool Unit where
  truth := fun statement => statement = true
  generated := fun _ => True
  reify := fun claim =>
    match claim with
    | ⟨false⟩ => .success falseCandidateFor
    | ⟨true⟩ => .failure failedInquiry
  warranted := fun _ => False
  standing := fun _ => False
  standingRequiresWarrant := by
    intro statement standing
    contradiction

theorem generatedClaimDoesNotEstablishTruthOrStanding :
    context.generated falseClaim ∧
      ¬ context.truth falseClaim.statement ∧
      ¬ context.standing falseClaim.statement := by
  change True ∧ ¬ false = true ∧ ¬ False
  exact ⟨True.intro, Bool.noConfusion, fun standing => standing⟩

theorem successfulReificationProducesCandidate :
    context.reify falseClaim = .success falseCandidateFor := by
  rfl

theorem successfulCandidateIsNotWarranted : ¬ context.warranted falseCandidate := by
  change ¬ False
  exact fun warranted => warranted

theorem successfulCandidateDoesNotStand : ¬ context.standing falseCandidate.statement := by
  change ¬ False
  exact fun standing => standing

theorem failedReificationReturnsTypedInquiry :
    context.reify trueClaim = .failure failedInquiry := by
  rfl

theorem failedReificationIsNotSemanticNegation : context.truth trueClaim.statement := by
  rfl

theorem everyUnsupportedBasisIsRejected (basis : UnsupportedPromotionBasis) :
    ¬ basis.authorizesStanding falseCandidate :=
  unsupportedBasisNeverAuthorizesStanding basis falseCandidate

end Countermodel
end InquiryCalculus.Legacy.V20.ClaimLifecycle

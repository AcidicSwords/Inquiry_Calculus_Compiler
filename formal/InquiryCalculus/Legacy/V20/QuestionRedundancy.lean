import InquiryCalculus.Legacy.V20.QuestionJointActiveRefinement

/-! # v2.0 question redundancy boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w

/-- A redundant question profile factors through the current local representation. -/
def QuestionRedundancySyntax {X : Type u} {V : Type v} {A : Type w}
    (current : X → V) (question : X → A) :=
  ProfileFactorizationSyntax question current

/-- Redundancy factorization implies that the current representation's kernel is included in the question's. -/
theorem redundancy_implies_profileKernelIncluded {X : Type u} {V : Type v} {A : Type w}
    {current : X → V} {question : X → A}
    (redundancy : QuestionRedundancySyntax current question) :
    ProfileKernelIncluded question current :=
  factorization_implies_profileKernelIncluded redundancy

/-- A local kernel inclusion constructs redundancy only if the current representation covers its advertised type. -/
noncomputable def profileKernelIncluded_redundancy_of_coverage
    {X : Type u} {V : Type v} {A : Type w} {current : X → V} {question : X → A}
    (included : ProfileKernelIncluded question current) (coverage : ProfileCoverage current) :
    QuestionRedundancySyntax current question :=
  profileKernelIncluded_factorization_of_coverage included coverage

/-- The predecessor's unconditioned equivalence and all improvement claims remain open. -/
inductive QuestionRedundancyObligation where
  | reverseNeedsCurrentRepresentationCoverage
  | exactNoActiveDiscriminatoryCoordinate
  | representationImprovementOrder
  | protectedBehaviorAndCost
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

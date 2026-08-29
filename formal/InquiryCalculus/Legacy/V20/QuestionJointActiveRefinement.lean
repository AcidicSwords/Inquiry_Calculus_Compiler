import InquiryCalculus.Legacy.V20.QuestionRefinementSemantics

/-! # v2.0 local joint and active refinement boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w z

/-- Equality of two local carrier points through one functional profile. -/
def SameProfile {X : Type u} {A : Type v} (profile : X → A) (x y : X) : Prop :=
  profile x = profile y

/-- The functional joint profile of two question coordinates on a shared local carrier. -/
def JointQuestionProfile {X : Type u} {A : Type v} {R : Type w}
    (left : X → A) (right : X → R) : X → A × R :=
  fun point => (left point, right point)

/-- A local active representation extended by one functional question coordinate. -/
def ActiveRepresentationExtension {X : Type u} {V : Type v} {A : Type w}
    (current : X → V) (added : X → A) : X → V × A :=
  fun point => (current point, added point)

/-- Joint profile equality is exactly the intersection of its two local profile equalities. -/
theorem sameProfile_jointQuestionProfile_iff {X : Type u} {A : Type v} {R : Type w}
    (left : X → A) (right : X → R) (x y : X) :
    SameProfile (JointQuestionProfile left right) x y ↔
      SameProfile left x y ∧ SameProfile right x y := by
  constructor
  · intro same
    exact ⟨congrArg Prod.fst same, congrArg Prod.snd same⟩
  · rintro ⟨sameLeft, sameRight⟩
    exact Prod.ext sameLeft sameRight

/-- An active extension has the same local kernel-intersection form as a joint profile. -/
theorem sameProfile_activeRepresentationExtension_iff {X : Type u} {V : Type v} {A : Type w}
    (current : X → V) (added : X → A) (x y : X) :
    SameProfile (ActiveRepresentationExtension current added) x y ↔
      SameProfile current x y ∧ SameProfile added x y := by
  exact sameProfile_jointQuestionProfile_iff current added x y

/-- Nonredundancy, improvement, and global active-representation claims remain separate. -/
inductive JointActiveRefinementObligation where
  | nonredundantCoordinate
  | activeRepresentationImprovement
  | globalKernelEquality
  | profileCoverageForReverseLaws
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

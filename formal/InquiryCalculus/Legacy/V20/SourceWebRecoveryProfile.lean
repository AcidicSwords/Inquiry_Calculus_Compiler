import InquiryCalculus.Legacy.V20.ProtectedRecovery

/-! # v2.0 source-web-recovery-profile boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x y

/-- The set-valued subset of a source web that recovers for one admitted use and exterior. -/
def sourceWebRecoveryProfile {Relation : Type u}
    (sourceWeb : Relation → Prop) (recovers : Relation → Prop) : Relation → Prop :=
  fun relation => sourceWeb relation ∧ recovers relation

/-- A semantic recovery profile records its source presentation, use, exterior, and recovered subset. -/
structure SourceWebRecoveryProfileSyntax (Presentation : Type u) (Use : Type v) (Exterior : Type w)
    (Relation : Type x) (Horizon : Type y) where
  sourcePresentation : Presentation
  admittedUse : Use
  exterior : Exterior
  protectedHorizon : Horizon
  recoveredRelations : Relation → Prop

/-- Partial source-web recovery remains set-valued and does not regenerate the entire protected class. -/
inductive SourceWebRecoveryProfileObligation where
  | sourcePresentation
  | admittedUse
  | exterior
  | protectedHorizon
  | setValuedRecoveredRelations
  | constitutiveRelationSubset
  | someRelationsWithoutWholeSourceClass
  | noUniversalScalarPercentage
  | semanticProfile
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

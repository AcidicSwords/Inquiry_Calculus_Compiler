import InquiryCalculus.Legacy.V20.BoundaryPointRegeneration

/-! # v2.0 determination-presentation boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x y z

/--
An explicit versioned support/dependency web for one standing source claim, indexed by the
conditions declared in the predecessor definition.  It is not a global essence of its source.
-/
structure DeterminationPresentationSyntax
    (Distinction : Type u) (Orientation : Type v) (Source : Type w) (Claim : Type x)
    (Relation : Type y) (Scope : Type z) (Applicability Grain Horizon Provenance : Type) where
  distinction : Distinction
  orientation : Orientation
  liveSource : Source
  standingClaim : Claim
  version : Nat
  supportDependencyWeb : Relation → Prop
  declaredScope : Scope
  declaredApplicability : Applicability
  declaredGrain : Grain
  protectedHorizon : Horizon
  bindingProvenance : Provenance
  constitutiveSupport : Prop
  retainsPredecessorAncestry : Prop

/-- Completeness, minimization, uniqueness, and executable meanings remain separately open. -/
inductive DeterminationPresentationObligation where
  | supportDependencyWeb
  | standingClaimSpecificity
  | versionedPresentation
  | declaredIndexes
  | constitutiveSupport
  | predecessorAncestry
  | admissionAndMinimizationLaw
  | everyRetainedFact
  | everySourceMentioningRelation
  | everyProtectedContinuation
  | globallyUniqueEssence
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

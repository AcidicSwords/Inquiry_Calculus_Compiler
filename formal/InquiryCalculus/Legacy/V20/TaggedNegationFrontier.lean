import InquiryCalculus.Legacy.V20.SemanticAndExecutionCoverage

/-! # v2.0 tagged-negation-frontier boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w

/-- A live exterior occurrence keeps the admitted use tag and its use-indexed target. -/
structure TaggedNegationFrontierOccurrence (Use : Type u) (Target : Use → Type v) where
  useIdentity : Use
  target : Target useIdentity

/-- The source's tagged dependent sum over admitted use fields. -/
def taggedNegationFrontier (Use : Type u) (Source : Type w) (Target : Use → Type v)
    (admitted : Use → Prop) (field : (use : Use) → Source → Target use → Prop)
    (source : Source) : Type max u v :=
  Σ use : { value : Use // admitted value }, { target : Target use.val // field use.val source target }

/-- Provenance, coverage, and authority obligations remain separate from ordinary relation generation. -/
inductive TaggedNegationFrontierObligation where
  | admittedUseFamily
  | useTagProvenance
  | dependentTargetCarrier
  | coincidentExteriorOccurrences
  | explicitCollectiveCoverCertificate
  | partialMembersNotExhaustive
  | untaggedUnionUnlawful
  | ordinaryRelationGeneration
  | independentlySoundNegationWarrant
  | resourceBoundedFairness
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

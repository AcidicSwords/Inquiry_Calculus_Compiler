import InquiryCalculus.Legacy.V20.TaggedNegationFrontier

/-! # v2.0 use-specific-return-fiber boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x

/-- The pure return field is the reverse section of one admitted use relation. -/
def useSpecificReturnFiber {Use : Type u} {Source : Type v} {Exterior : Type w}
    (incidence : Use → Source → Exterior → Prop) (use : Use) (exterior : Exterior) : Source → Prop :=
  fun source => incidence use source exterior

/-- A return occurrence carries its whole reverse section, source membership, and an optional selected role. -/
structure UseSpecificReturnFiberSyntax (Use : Type u) (Source : Type v) (Exterior : Type w)
    (Coverage : Type x) (incidence : Use → Source → Exterior → Prop) where
  useIdentity : Use
  source : Source
  exterior : Exterior
  admittedIncidence : incidence useIdentity source exterior
  returnFiber : Source → Prop := useSpecificReturnFiber incidence useIdentity exterior
  sourceInFiber : returnFiber source
  selectedReturnRole : Option Source
  declaredCoverage : Coverage

/-- Exact source closure and unique recovery remain separately constrained. -/
inductive UseSpecificReturnFiberObligation where
  | admittedIncidence
  | reverseSection
  | sourceMembership
  | selectedReturnRole
  | selectedRoleNotFiber
  | uniqueRecovery
  | horizonQuotientClosure
  | semanticAndExecutionCoverage
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

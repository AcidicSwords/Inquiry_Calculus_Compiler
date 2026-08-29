import InquiryCalculus.Legacy.V20.RelationAndNegationUse

/-! # v2.0 positive-negation-filling boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x

/-- The field selected by an admitted use at one source. -/
def negationField {Use : Type u} {Source : Type v} {Candidate : Type w}
    (incidence : Use → Source → Candidate → Prop)
    (use : Use) (source : Source) : Candidate → Prop :=
  fun candidate => incidence use source candidate

/-- A positive-negation filling is a role tying use identity, field membership, and witness together. -/
structure PositiveNegationFillingSyntax (Use : Type u) (Source : Type v) (Candidate : Type w)
    (Witness : Type x) (incidence : Use → Source → Candidate → Prop) where
  useIdentity : Use
  source : Source
  candidate : Candidate
  fieldMembership : negationField (Use := Use) (Source := Source) (Candidate := Candidate)
    incidence useIdentity source candidate
  departureWitness : Witness

/-- The named role does not collapse source negation, relation, field, candidate, or use identity. -/
inductive PositiveNegationFillingObligation where
  | admittedUse
  | useIndexedField
  | fieldMembership
  | departureWitness
  | useIdentity
  | roleNotAdditionalCarrier
  | sourceNegationDistinct
  | useRelationDistinct
  | sourceFieldDistinct
  | candidateDistinct
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

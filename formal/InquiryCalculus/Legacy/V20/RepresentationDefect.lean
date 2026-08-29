import InquiryCalculus.Legacy.V20.ProtectedDetermination

/-! # v2.0 representation-defect boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v

/-- A witnessed collision in one current representation cell across protected classes. -/
structure RepresentationDefectWitness {X : Type u} {S : Type v}
    (representation : X → S) (horizon : HorizonEquivalenceSyntax X) where
  left : X
  right : X
  sameRepresentation : representation left = representation right
  separated : ¬ horizon.equivalent left right

/-- Separator success, functional consequence specialization, and execution remain open. -/
inductive RepresentationDefectObligation where
  | protectedConsequenceRelation
  | functionalConsequenceSpecialization
  | separatorSearchObligation
  | separatorSuccess
  | unknownNotEquivalence
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

import InquiryCalculus.Legacy.V20.SourceWebRecoveryProfile

/-! # v2.0 three-valued recovery/loss profile boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x y z

/-- Positive non-recovery is a witnessed difference between two members of one return field. -/
structure PositiveNonRecoveryWitness (Return : Type u) (Signature : Type v) where
  first : Return
  second : Return
  firstInReturnField : Prop
  secondInReturnField : Prop
  firstSignature : Signature
  secondSignature : Signature
  signatureDifference : firstSignature ≠ secondSignature

/-- Recovery status retains certified recovery, witnessed non-recovery, and incomplete evidence separately. -/
inductive RecoveryLossStatus (Certificate : Type u) (Witness : Type v) (MissingEvidence : Type w) where
  | recovered : Certificate → RecoveryLossStatus Certificate Witness MissingEvidence
  | notRecovered : Witness → RecoveryLossStatus Certificate Witness MissingEvidence
  | unknown : MissingEvidence → RecoveryLossStatus Certificate Witness MissingEvidence

/-- The occurrence-indexed recovery/loss profile carries the exact three-way source distinction. -/
structure ThreeValuedRecoveryLossProfileSyntax (Occurrence : Type u) (Relation : Type v) (Exterior : Type w)
    (Use : Type x) (Certificate : Type y) (Witness : Type z) (MissingEvidence : Type (max u v w x y z)) where
  executableOccurrence : Occurrence
  relation : Relation
  exterior : Exterior
  admittedUse : Use
  recoveryStatus : RecoveryLossStatus Certificate Witness MissingEvidence

/-- Source obligations retained until exact decision and coverage interfaces are formalized. -/
inductive ThreeValuedRecoveryLossProfileObligation where
  | returnField
  | positiveNonRecoveryWitness
  | protectedSignatureDifference
  | executableOccurrence
  | recoveryCertificate
  | witnessedNonRecovery
  | unknownMissingEvidenceCoverageOrDecisionCapability
  | exactDecisionCoverageCertificate
  | irrecoverableResidue
  | noUnknownLossCollapse
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

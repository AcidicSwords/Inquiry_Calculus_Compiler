import InquiryCalculus.Legacy.V20.PositiveNegationFilling

/-! # v2.0 semantic-and-execution-coverage boundary -/
namespace InquiryCalculus.Legacy.V20

/-- Declared semantic coverage kinds for one negation use. -/
inductive SemanticCoverageKind where
  | exactExhaustive
  | certifiedPartial
  | open
  deriving DecidableEq, Repr

/-- Semantic coverage is declared with its applicability domain and certificate. -/
structure SemanticCoverageSyntax (ApplicabilityDomain Certificate : Type) where
  kind : SemanticCoverageKind
  applicabilityDomain : ApplicabilityDomain
  certificate : Certificate

/-- Execution coverage is occurrence-specific materialization, intentionally a different type. -/
structure ExecutionCoverageSyntax (Use Occurrence MaterializedPortion : Type) where
  use : Use
  occurrence : Occurrence
  materializedPortion : MaterializedPortion

/-- Partial coverage returns Unknown rather than absolute closure. -/
inductive SemanticAndExecutionCoverageObligation where
  | declaredSemanticCoverage
  | applicabilityDomain
  | semanticCertificate
  | occurrenceSpecificExecution
  | materializedPortion
  | semanticExecutionDistinct
  | emptyExactExhaustiveField
  | emptyUnsearchedField
  | partialCoverageUnknown
  | partialAbsenceNotAbsoluteClosure
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

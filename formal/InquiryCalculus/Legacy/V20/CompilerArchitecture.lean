import InquiryCalculus.Legacy.V20.GuardedRecurrence

/-! # v2.0 compiler-architecture boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The declared compilation stages remain source pipeline data. -/
inductive CompilerStage where
  | typedFormsAndRelationSchema
  | openRelationAndQuestion
  | inquirySyntax
  | openIntermediateForm
  | positiveNegationAndReciprocalChart
  | activeView
  | probeIntermediateForm
  | backendCode
  | runtimeSyntax
  deriving DecidableEq, Repr

/-- Compiler architecture retains only stage order and the non-strengthening condition. -/
structure CompilerArchitectureSyntax (StageWitness : Type u) where
  stageWitness : StageWitness
  stageOrder : CompilerStage → Nat
  typedFormsStage : Prop
  openRelationStage : Prop
  inquirySyntaxStage : Prop
  openIntermediateFormStage : Prop
  reciprocalChartStage : Prop
  activeViewStage : Prop
  probeIntermediateFormStage : Prop
  backendCodeStage : Prop
  runtimeSyntaxStage : Prop
  semanticAuthorityNonStrengtheningUnproved : Prop
  noExecutableCompilation : Prop
  noStageCorrectnessProof : Prop

/-- Source obligations retained until stages and authority preservation are separately checked. -/
inductive CompilerArchitectureObligation where
  | declaredStagedPipeline
  | typedFormsAndRelationSchemaStage
  | openRelationAndQuestionStage
  | inquirySyntaxStage
  | openIntermediateFormStage
  | positiveNegationAndReciprocalChartStage
  | activeViewStage
  | probeIntermediateFormStage
  | backendCodeStage
  | runtimeSyntaxStage
  | noSilentSemanticAuthorityStrengtheningUnproved
  | noStageSkipping
  | noExecutableCompilation
  | noStageCorrectnessProof
  | noAnswerSelectionOrTotalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

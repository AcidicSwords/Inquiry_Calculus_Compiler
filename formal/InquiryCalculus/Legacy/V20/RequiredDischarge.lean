import InquiryCalculus.Legacy.V20.RegenerativeEconomyFrontier

/-! # v2.0 required-discharge boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x

/-- A declared route is one of the source-authorized ways to discharge an open relation. -/
inductive RequiredDischargeRoute where
  | probe
  | check
  | warrant
  | exactSupport
  | reconstruction
  deriving DecidableEq, Repr

/-- A dependency constrains lawful continuation without asserting that its route has run or succeeded. -/
def requiredDischarge {Dependency : Type u}
    (standingOrSourceProgramDependency : Dependency → Prop) (mayLawfullyContinue : Prop)
    (discharged : RequiredDischargeRoute → Prop) (dependency : Dependency)
    (declaredRoute : RequiredDischargeRoute) : Prop :=
  standingOrSourceProgramDependency dependency ∧ (mayLawfullyContinue → discharged declaredRoute)

/-- The source carrier keeps dependency, exact occurrence, and declared route distinct. -/
structure RequiredDischargeSyntax (State : Type u) (AskReference : Type v) (Dependency : Type w) where
  state : State
  askReference : AskReference
  dependency : Dependency
  declaredRoute : RequiredDischargeRoute
  openRelation : Prop
  lawfulContinuation : Prop

/-- Source obligations retained until execution and successful-discharge semantics are separately formalized. -/
inductive RequiredDischargeObligation where
  | explicitStandingOrSourceProgramDependency
  | openRelationOfAskReference
  | declaredProbeCheckWarrantSupportOrReconstructionRoute
  | blocksLawfulContinuation
  | noNewQuestionSpecies
  | noExecutabilityAssertion
  | noSuccessfulDischargeAssertion
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

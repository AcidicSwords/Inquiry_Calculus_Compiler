import InquiryCalculus.Legacy.V20.NoUniversalPolarization

/-! # v2.0 operational root aliases

Source-bound reconstruction of the unproved transparency boundary at v2.0 lines 5110–5114.  The
eleven names are represented as aliases with supplied nonempty root-expansion data and an explicit
preservation contract.  No expansion is executed or promoted as the successor's canonical
definition.
-/
namespace InquiryCalculus.Legacy.V20.OperationalRootAliases

open DerivedInterrogativeRoots

inductive OperationalAlias where
  | contrast
  | backchain
  | preimage
  | project
  | ablate
  | substitute
  | localize
  | construct
  | scrutinize
  | whyNot
  | howCan
  deriving DecidableEq

structure RootInvocation where
  root : Root
  orientation : Option Orientation
  canonical : CanonicalForm
  deriving DecidableEq

def WellShaped (invocation : RootInvocation) : Bool :=
  canonicalFor invocation.root invocation.orientation == some invocation.canonical

def exposeInvocation : RootInvocation := ⟨.expose, none, .questionConstructor⟩
def converseInvocation : RootInvocation := ⟨.orient, some .converse, .converseExposure⟩
def factorInvocation : RootInvocation := ⟨.factor, none, .factorExposure⟩
def polarizeInvocation : RootInvocation := ⟨.polarize, none, .positiveAlternative⟩
def varyInvocation : RootInvocation := ⟨.vary, none, .admissibleVariation⟩
def groundInvocation : RootInvocation := ⟨.ground, none, .supportCheckWarrant⟩

structure RootExpansion where
  head : RootInvocation
  tail : List RootInvocation

def ExpansionWellShaped (expansion : RootExpansion) : Bool :=
  WellShaped expansion.head && expansion.tail.all WellShaped

/-- A finite nonempty source-pressure model of root expansion.  These choices are not asserted as
the successor's canonical alias definitions. -/
def sampleExpansion : OperationalAlias → RootExpansion
  | .contrast => ⟨polarizeInvocation, []⟩
  | .backchain => ⟨converseInvocation, []⟩
  | .preimage => ⟨converseInvocation, []⟩
  | .project => ⟨factorInvocation, []⟩
  | .ablate => ⟨varyInvocation, []⟩
  | .substitute => ⟨varyInvocation, []⟩
  | .localize => ⟨factorInvocation, []⟩
  | .construct => ⟨exposeInvocation, []⟩
  | .scrutinize => ⟨groundInvocation, []⟩
  | .whyNot => ⟨polarizeInvocation, []⟩
  | .howCan => ⟨exposeInvocation, [varyInvocation]⟩

theorem sampleExpansionIsWellShaped (alias : OperationalAlias) :
    ExpansionWellShaped (sampleExpansion alias) = true := by
  cases alias <;> decide

structure AliasPresentation where
  alias : OperationalAlias
  expansion : RootExpansion

structure ExpansionContract (presentation : AliasPresentation) where
  typingPreserved : Bool
  applicabilityPreserved : Bool
  wholeSupportedAnswerBehaviorPreserved : Bool
  authorityPreserved : Bool
  failureExitsPreserved : Bool
  provenancePreserved : Bool
  reopeningPreserved : Bool
  addsPrimitive : Bool
  addsRuntimeOpcode : Bool
  schedules : Bool

def Transparent {presentation : AliasPresentation}
    (contract : ExpansionContract presentation) : Prop :=
  contract.typingPreserved = true ∧ contract.applicabilityPreserved = true ∧
    contract.wholeSupportedAnswerBehaviorPreserved = true ∧ contract.authorityPreserved = true ∧
    contract.failureExitsPreserved = true ∧ contract.provenancePreserved = true ∧
    contract.reopeningPreserved = true ∧ contract.addsPrimitive = false ∧
    contract.addsRuntimeOpcode = false ∧ contract.schedules = false

/-- The source boundary remains a list of proof obligations rather than a proof of transparency. -/
inductive OperationalAliasObligation where
  | exactExpansion
  | typingPreservation
  | applicabilityPreservation
  | wholeSupportedAnswerBehaviorPreservation
  | authorityPreservation
  | failureExitPreservation
  | provenancePreservation
  | reopeningPreservation
  | noPrimitiveOpcodeOrScheduler
  | noProgramOrRustAuthority
  deriving DecidableEq

namespace Countermodel

def presentation (alias : OperationalAlias) : AliasPresentation where
  alias := alias
  expansion := sampleExpansion alias

def complete (alias : OperationalAlias) : ExpansionContract (presentation alias) where
  typingPreserved := true
  applicabilityPreserved := true
  wholeSupportedAnswerBehaviorPreserved := true
  authorityPreserved := true
  failureExitsPreserved := true
  provenancePreserved := true
  reopeningPreserved := true
  addsPrimitive := false
  addsRuntimeOpcode := false
  schedules := false

def missingTyping (alias : OperationalAlias) : ExpansionContract (presentation alias) :=
  { complete alias with typingPreserved := false }
def missingApplicability (alias : OperationalAlias) : ExpansionContract (presentation alias) :=
  { complete alias with applicabilityPreserved := false }
def missingWholeAnswerBehavior (alias : OperationalAlias) : ExpansionContract (presentation alias) :=
  { complete alias with wholeSupportedAnswerBehaviorPreserved := false }
def missingAuthority (alias : OperationalAlias) : ExpansionContract (presentation alias) :=
  { complete alias with authorityPreserved := false }
def missingFailureExit (alias : OperationalAlias) : ExpansionContract (presentation alias) :=
  { complete alias with failureExitsPreserved := false }
def missingProvenance (alias : OperationalAlias) : ExpansionContract (presentation alias) :=
  { complete alias with provenancePreserved := false }
def missingReopening (alias : OperationalAlias) : ExpansionContract (presentation alias) :=
  { complete alias with reopeningPreserved := false }
def primitiveAlias (alias : OperationalAlias) : ExpansionContract (presentation alias) :=
  { complete alias with addsPrimitive := true }
def runtimeOpcodeAlias (alias : OperationalAlias) : ExpansionContract (presentation alias) :=
  { complete alias with addsRuntimeOpcode := true }
def schedulingAlias (alias : OperationalAlias) : ExpansionContract (presentation alias) :=
  { complete alias with schedules := true }

theorem completeIsTransparent (alias : OperationalAlias) : Transparent (complete alias) := by
  exact ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl⟩

theorem missingTypingIsNotTransparent (alias : OperationalAlias) :
    ¬ Transparent (missingTyping alias) := by
  intro transparent
  exact Bool.noConfusion transparent.1
theorem missingApplicabilityIsNotTransparent (alias : OperationalAlias) :
    ¬ Transparent (missingApplicability alias) := by
  intro transparent
  exact Bool.noConfusion transparent.2.1
theorem missingWholeAnswerBehaviorIsNotTransparent (alias : OperationalAlias) :
    ¬ Transparent (missingWholeAnswerBehavior alias) := by
  intro transparent
  exact Bool.noConfusion transparent.2.2.1
theorem missingAuthorityIsNotTransparent (alias : OperationalAlias) :
    ¬ Transparent (missingAuthority alias) := by
  intro transparent
  exact Bool.noConfusion transparent.2.2.2.1
theorem missingFailureExitIsNotTransparent (alias : OperationalAlias) :
    ¬ Transparent (missingFailureExit alias) := by
  intro transparent
  exact Bool.noConfusion transparent.2.2.2.2.1
theorem missingProvenanceIsNotTransparent (alias : OperationalAlias) :
    ¬ Transparent (missingProvenance alias) := by
  intro transparent
  exact Bool.noConfusion transparent.2.2.2.2.2.1
theorem missingReopeningIsNotTransparent (alias : OperationalAlias) :
    ¬ Transparent (missingReopening alias) := by
  intro transparent
  exact Bool.noConfusion transparent.2.2.2.2.2.2.1
theorem primitiveAliasIsNotTransparent (alias : OperationalAlias) :
    ¬ Transparent (primitiveAlias alias) := by
  intro transparent
  exact Bool.noConfusion transparent.2.2.2.2.2.2.2.1
theorem runtimeOpcodeAliasIsNotTransparent (alias : OperationalAlias) :
    ¬ Transparent (runtimeOpcodeAlias alias) := by
  intro transparent
  exact Bool.noConfusion transparent.2.2.2.2.2.2.2.2.1
theorem schedulingAliasIsNotTransparent (alias : OperationalAlias) :
    ¬ Transparent (schedulingAlias alias) := by
  intro transparent
  exact Bool.noConfusion transparent.2.2.2.2.2.2.2.2.2

theorem ablateAndSubstituteShareSampleRootShape :
    sampleExpansion .ablate = sampleExpansion .substitute := rfl

theorem contrastAndWhyNotShareSampleRootShape :
    sampleExpansion .contrast = sampleExpansion .whyNot := rfl

end Countermodel
end InquiryCalculus.Legacy.V20.OperationalRootAliases

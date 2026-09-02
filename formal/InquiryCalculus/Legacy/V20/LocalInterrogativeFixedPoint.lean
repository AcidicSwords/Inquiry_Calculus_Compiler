import InquiryCalculus.Legacy.V20.BlockedUnknownResidual
import InquiryCalculus.Legacy.V20.QuestionRedundancy
import InquiryCalculus.Legacy.V20.RootFrontier

/-! # v2.0 local interrogative fixed point

Source-bound reconstruction of v2.0 lines 5185–5197.  Local closure is indexed by explicit
coverage.  Its seven exit predicates classify relevant root occurrences, while the already checked
root frontier supplies the executable productive-or-required boundary.  Reopening requires a
positive occurrence whose open-residual status changes; a changed context label alone is not
enough.  Nothing here establishes global completion, selects work, or executes an occurrence.
-/
namespace InquiryCalculus.Legacy.V20.LocalInterrogativeFixedPoint

open RequiredNondominantFrontier RootFrontier

universe u v

structure ExitEvidence (Occurrence : Type u) where
  determinedByRetainedProfile : Occurrence → Prop
  factorablyRedundant : Occurrence → Prop
  inapplicable : Occurrence → Prop
  protectedContinuationEquivalentWithoutRequired : Occurrence → Prop
  explicitlyBlocked : Occurrence → Prop
  resourceBounded : Occurrence → Prop
  representedExtensionDependent : Occurrence → Prop

def AllowedExit {Occurrence : Type u} (evidence : ExitEvidence Occurrence)
    (occurrence : Occurrence) : Prop :=
  evidence.determinedByRetainedProfile occurrence ∨
    evidence.factorablyRedundant occurrence ∨
    evidence.inapplicable occurrence ∨
    evidence.protectedContinuationEquivalentWithoutRequired occurrence ∨
    evidence.explicitlyBlocked occurrence ∨
    evidence.resourceBounded occurrence ∨
    evidence.representedExtensionDependent occurrence

structure LocalContext (Occurrence : Type u) (Dependency : Type v) where
  explicitCoverage : Prop
  relevantRootOccurrence : Occurrence → Prop
  exits : ExitEvidence Occurrence
  root : RootFrontierContext Occurrence Dependency

def CarriesRequiredDischarge {Occurrence : Type u} {Dependency : Type v}
    (context : LocalContext Occurrence Dependency) (occurrence : Occurrence) : Prop :=
  ∃ dependency, context.root.membership.order.required occurrence dependency

def OpenResidual {Occurrence : Type u} {Dependency : Type v}
    (context : LocalContext Occurrence Dependency) (occurrence : Occurrence) : Prop :=
  context.relevantRootOccurrence occurrence ∧ RootFrontier context.root occurrence ∧
    (¬ context.exits.factorablyRedundant occurrence ∨ CarriesRequiredDischarge context occurrence)

def LocalIFP {Occurrence : Type u} {Dependency : Type v}
    (context : LocalContext Occurrence Dependency) : Prop :=
  context.explicitCoverage ∧
    (∀ occurrence, context.relevantRootOccurrence occurrence → AllowedExit context.exits occurrence) ∧
    ¬ ∃ occurrence, OpenResidual context occurrence

theorem localIFPHasExplicitCoverage {Occurrence : Type u} {Dependency : Type v}
    (context : LocalContext Occurrence Dependency) :
    LocalIFP context → context.explicitCoverage := fun closed => closed.1

theorem localIFPClassifiesEveryRelevantOccurrence {Occurrence : Type u} {Dependency : Type v}
    (context : LocalContext Occurrence Dependency) :
    LocalIFP context → ∀ occurrence, context.relevantRootOccurrence occurrence →
      AllowedExit context.exits occurrence := fun closed => closed.2.1

theorem localIFPHasNoOpenResidual {Occurrence : Type u} {Dependency : Type v}
    (context : LocalContext Occurrence Dependency) :
    LocalIFP context → ¬ ∃ occurrence, OpenResidual context occurrence := fun closed => closed.2.2

inductive EnlargementKind where
  | separator | relation | probe | foil | binding | representation | protectedHorizon | resourceRegime
  deriving DecidableEq

structure ReopeningWitness {Occurrence : Type u} {Dependency : Type v}
    (oldContext newContext : LocalContext Occurrence Dependency) where
  trigger : EnlargementKind
  occurrence : Occurrence
  previouslyNotOpen : ¬ OpenResidual oldContext occurrence
  nowOpen : OpenResidual newContext occurrence

def Reopens {Occurrence : Type u} {Dependency : Type v}
    (oldContext newContext : LocalContext Occurrence Dependency) : Prop :=
  Nonempty (ReopeningWitness oldContext newContext)

theorem sameContextDoesNotReopen {Occurrence : Type u} {Dependency : Type v}
    (context : LocalContext Occurrence Dependency) : ¬ Reopens context context := by
  intro reopened
  rcases reopened with ⟨witness⟩
  exact witness.previouslyNotOpen witness.nowOpen

theorem positiveOpenResidualReopens {Occurrence : Type u} {Dependency : Type v}
    (oldContext newContext : LocalContext Occurrence Dependency) (trigger : EnlargementKind)
    (occurrence : Occurrence) (oldClosed : LocalIFP oldContext)
    (nowOpen : OpenResidual newContext occurrence) : Reopens oldContext newContext := by
  refine ⟨⟨trigger, occurrence, ?_, nowOpen⟩⟩
  intro previouslyOpen
  exact oldClosed.2.2 ⟨occurrence, previouslyOpen⟩

inductive LocalIFPObligation where
  | explicitCoverage
  | everyRelevantOccurrenceClassified
  | determinedExit
  | factorableRedundancyExit
  | inapplicableExit
  | protectedEquivalentWithoutRequiredExit
  | blockedExit
  | resourceBoundedExit
  | representedExtensionDependentExit
  | noExecutableNonredundantProductiveResidual
  | noExecutableRequiredResidual
  | positiveReopeningOccurrence
  | localNotGlobal
  | noSelectionSupportWarrantOrExecution
  | noRustOrSuccessorAuthority
  deriving DecidableEq

namespace Countermodel

inductive ExitKind where
  | determined | redundant | inapplicable | protectedEquivalent | blocked | resource | extension
  deriving DecidableEq

inductive Occurrence where
  | exit (kind : ExitKind) | productive | required | newlyLive
  deriving DecidableEq

def relevant (includeProductive includeRequired includeNew : Bool) (occurrence : Occurrence) : Bool :=
  match occurrence with
  | .exit _ => true
  | .productive => includeProductive
  | .required => includeRequired
  | .newlyLive => includeNew

def allowed (occurrence : Occurrence) : Bool :=
  match occurrence with
  | .exit _ => true
  | _ => false

def allowedKind (_kind : ExitKind) : Bool := true

def openResidual (includeProductive includeRequired includeNew : Bool)
    (occurrence : Occurrence) : Bool :=
  relevant includeProductive includeRequired includeNew occurrence &&
    match occurrence with
    | .productive | .required | .newlyLive => true
    | _ => false

def finiteField : List Occurrence :=
  [.exit .determined, .exit .redundant, .exit .inapplicable, .exit .protectedEquivalent,
    .exit .blocked, .exit .resource, .exit .extension]

def allSevenExitsFlag : Bool :=
  allowedKind .determined && allowedKind .redundant && allowedKind .inapplicable &&
    allowedKind .protectedEquivalent && allowedKind .blocked && allowedKind .resource &&
    allowedKind .extension

def localIFPFlag (coverage includeProductive includeRequired includeNew : Bool) : Bool :=
  coverage && allSevenExitsFlag && !(includeProductive || includeRequired || includeNew)

def reopeningFlag (oldOpen newOpen : Bool) : Bool := !oldOpen && newOpen

theorem everyAllowedExitIsRepresented : allSevenExitsFlag = true := by
  rfl

theorem coveredCompleteFieldIsLocallyClosed : localIFPFlag true false false false = true := by
  rfl

theorem missingCoverageIsNotClosed : localIFPFlag false false false false = false := by
  rfl

theorem productiveResidualPreventsClosure : localIFPFlag true true false false = false := by
  rfl

theorem requiredResidualPreventsClosure : localIFPFlag true false true false = false := by
  rfl

theorem extensionDependentExitRemainsPresent :
    allowedKind .extension = true := by
  rfl

theorem blockedExitRemainsPresent :
    allowedKind .blocked = true := by
  rfl

theorem resourceExitRemainsPresent :
    allowedKind .resource = true := by
  rfl

theorem newlyLiveOccurrenceReopens :
    reopeningFlag false true = true := by
  rfl

theorem contextLabelWithoutNewOpenOccurrenceDoesNotReopen : reopeningFlag false false = false := by
  rfl

end Countermodel
end InquiryCalculus.Legacy.V20.LocalInterrogativeFixedPoint

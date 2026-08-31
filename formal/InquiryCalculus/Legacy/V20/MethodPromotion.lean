import InquiryCalculus.Legacy.V20.LearningGain

/-! # Method promotion

Source-bound realization of the v2.0 definition at lines 4276–4291.  A
promotion is a typed witness record carrying every source condition.  It is a
pure admissibility relation: it neither executes the path nor folds anything.
-/
namespace InquiryCalculus.Legacy.V20.MethodPromotion

universe u v w x y z

structure PromotionContext (Method : Type u) (Path : Type v) (Region : Type w)
    (Continuation : Type x) (Evidence : Type y) (Unlock : Type z) where
  parallel : Method → Path → Prop
  explicitlyBridged : Method → Path → Prop
  applicable : Method → Path → Region → Prop
  protectedEquivalent : Method → Path → Region → Prop
  requiredContinuation : Continuation → Prop
  continuationDescends : Method → Path → Continuation → Prop
  typedOperationalGain : Method → Path → Prop
  necessaryImplementationPurpose : Method → Path → Prop
  definingEvidence : Method → Path → Evidence → Prop
  recoversDefiningPath : Evidence → Method → Path → Prop
  unlockStored : Method → Path → Unlock → Prop
  utilityConfersFutureOutputWarrant : Method → Path → Prop

structure PromotionWitness {Method : Type u} {Path : Type v} {Region : Type w}
    {Continuation : Type x} {Evidence : Type y} {Unlock : Type z}
    (context : PromotionContext Method Path Region Continuation Evidence Unlock)
    (method : Method) (path : Path) : Prop where
  parallelOrBridged : context.parallel method path ∨ context.explicitlyBridged method path
  equivalentOnApplicability :
    ∀ region, context.applicable method path region → context.protectedEquivalent method path region
  requiredContinuationsDescend :
    ∀ continuation, context.requiredContinuation continuation →
      context.continuationDescends method path continuation
  gainOrNecessaryPurpose :
    context.typedOperationalGain method path ∨ context.necessaryImplementationPurpose method path
  recoverableDefiningPath :
    ∃ evidence, context.definingEvidence method path evidence ∧
      context.recoversDefiningPath evidence method path
  reopeningStored : ∃ unlock, context.unlockStored method path unlock
  utilityDoesNotWarrantFutureOutputs : ¬ context.utilityConfersFutureOutputWarrant method path

def MethodPromotable {Method : Type u} {Path : Type v} {Region : Type w}
    {Continuation : Type x} {Evidence : Type y} {Unlock : Type z}
    (context : PromotionContext Method Path Region Continuation Evidence Unlock)
    (method : Method) (path : Path) : Prop :=
  Nonempty (PromotionWitness context method path)

namespace Countermodel

structure Candidate where
  parallel : Bool
  bridge : Bool
  equivalent : Bool
  descent : Bool
  gain : Bool
  purpose : Bool
  recoveryA : Bool
  recoveryB : Bool
  unlockA : Bool
  unlockB : Bool
  futureWarrant : Bool

def context : PromotionContext Candidate Unit Unit Unit Bool Bool where
  parallel := fun candidate _ => candidate.parallel = true
  explicitlyBridged := fun candidate _ => candidate.bridge = true
  applicable := fun _ _ _ => True
  protectedEquivalent := fun candidate _ _ => candidate.equivalent = true
  requiredContinuation := fun _ => True
  continuationDescends := fun candidate _ _ => candidate.descent = true
  typedOperationalGain := fun candidate _ => candidate.gain = true
  necessaryImplementationPurpose := fun candidate _ => candidate.purpose = true
  definingEvidence := fun _ _ _ => True
  recoversDefiningPath := fun evidence candidate _ =>
    if evidence then candidate.recoveryB = true else candidate.recoveryA = true
  unlockStored := fun candidate _ unlock =>
    if unlock then candidate.unlockB = true else candidate.unlockA = true
  utilityConfersFutureOutputWarrant := fun candidate _ => candidate.futureWarrant = true

def overcomplete : Candidate := ⟨true, true, true, true, true, true,
  true, true, true, true, false⟩
def contracted : Candidate := ⟨true, false, true, true, true, false,
  true, false, true, false, false⟩
def lacksAlignment : Candidate := { contracted with parallel := false, bridge := false }
def lacksEquivalence : Candidate := { contracted with equivalent := false }
def lacksDescent : Candidate := { contracted with descent := false }
def lacksGainOrPurpose : Candidate := { contracted with gain := false, purpose := false }
def lacksRecovery : Candidate := { contracted with recoveryA := false, recoveryB := false }
def lacksUnlock : Candidate := { contracted with unlockA := false, unlockB := false }
def selfWarrants : Candidate := { contracted with futureWarrant := true }

theorem overcompleteCrosses : MethodPromotable context overcomplete () := by
  refine ⟨{
    parallelOrBridged := Or.inl rfl
    equivalentOnApplicability := ?_
    requiredContinuationsDescend := ?_
    gainOrNecessaryPurpose := Or.inl rfl
    recoverableDefiningPath := ⟨false, trivial, rfl⟩
    reopeningStored := ⟨false, rfl⟩
    utilityDoesNotWarrantFutureOutputs := by intro warrant; cases warrant }⟩
  · intro region _; cases region; rfl
  · intro continuation _; cases continuation; rfl

theorem contractedRetainsBoundary : MethodPromotable context contracted () := by
  refine ⟨{
    parallelOrBridged := Or.inl rfl
    equivalentOnApplicability := ?_
    requiredContinuationsDescend := ?_
    gainOrNecessaryPurpose := Or.inl rfl
    recoverableDefiningPath := ⟨false, trivial, rfl⟩
    reopeningStored := ⟨false, rfl⟩
    utilityDoesNotWarrantFutureOutputs := by intro warrant; cases warrant }⟩
  · intro region _; cases region; rfl
  · intro continuation _; cases continuation; rfl

theorem alignmentRequired : ¬ MethodPromotable context lacksAlignment () := by
  rintro ⟨witness⟩
  rcases witness.parallelOrBridged with aligned | bridged
  · cases aligned
  · cases bridged

theorem equivalenceRequired : ¬ MethodPromotable context lacksEquivalence () := by
  rintro ⟨witness⟩
  cases witness.equivalentOnApplicability () trivial

theorem descentRequired : ¬ MethodPromotable context lacksDescent () := by
  rintro ⟨witness⟩
  cases witness.requiredContinuationsDescend () trivial

theorem gainOrPurposeRequired : ¬ MethodPromotable context lacksGainOrPurpose () := by
  rintro ⟨witness⟩
  rcases witness.gainOrNecessaryPurpose with gain | purpose
  · cases gain
  · cases purpose

theorem recoveryRequired : ¬ MethodPromotable context lacksRecovery () := by
  rintro ⟨witness⟩
  obtain ⟨evidence, _, recovered⟩ := witness.recoverableDefiningPath
  cases evidence <;> cases recovered

theorem unlockRequired : ¬ MethodPromotable context lacksUnlock () := by
  rintro ⟨witness⟩
  obtain ⟨unlock, stored⟩ := witness.reopeningStored
  cases unlock <;> cases stored

theorem futureOutputWarrantRejected : ¬ MethodPromotable context selfWarrants () := by
  rintro ⟨witness⟩
  exact witness.utilityDoesNotWarrantFutureOutputs rfl

end Countermodel
end InquiryCalculus.Legacy.V20.MethodPromotion

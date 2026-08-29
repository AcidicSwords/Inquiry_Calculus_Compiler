import InquiryCalculus.Legacy.V20.UseSpecificReturnFiber

/-! # v2.0 protected-recovery boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x

/-- The horizon-quotiented observation signature at one source point. -/
def protectedSignature {Source : Type u} {Observation : Type v} {HorizonClass : Type w}
    (represented : Source → Observation → Prop) (horizonClass : Observation → HorizonClass)
    (source : Source) : HorizonClass → Prop :=
  fun horizonCls => ∃ observation, represented source observation ∧ horizonClass observation = horizonCls

/-- A return fiber locally recovers an observation exactly when all of its points have equal signatures. -/
def protectedRecovers {Source : Type u} {Observation : Type v} {HorizonClass : Type w}
    (represented : Source → Observation → Prop) (horizonClass : Observation → HorizonClass)
    (fiber : Source → Prop) : Prop :=
  ∀ first second, fiber first → fiber second →
    protectedSignature represented horizonClass first = protectedSignature represented horizonClass second

/-- Recovery is protected fiber-wide determination, not scalar similarity or a new primitive. -/
inductive ProtectedRecoveryObligation where
  | protectedRepresentedObservation
  | horizonQuotientedSignature
  | returnFiber
  | fiberWideSignatureAgreement
  | protectedDetermination
  | scalarSimilarity
  | newPrimitive
  | rawRelationDifference
  | positiveSignatureDifference
  | selectedReturnRecovery
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

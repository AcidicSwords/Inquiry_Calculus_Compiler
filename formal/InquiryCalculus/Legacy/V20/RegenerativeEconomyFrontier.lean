import InquiryCalculus.Legacy.V20.ThreeValuedRecoveryLossProfile

/-! # v2.0 regenerative-economy-frontier boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x y z

/-- Candidates retain both inquiry-regenerative sufficiency and their binding-supplied license. -/
def licensedRegenerativeCandidate {Representation : Type u}
    (inquiryRegenerates licensed : Representation → Prop) : Representation → Prop :=
  fun representation => inquiryRegenerates representation ∧ licensed representation

/-- The minimal/nondominated frontier of licensed inquiry-regenerative candidates under a resource preorder. -/
def regenerativeEconomyFrontier {Representation : Type u}
    (resourcePreorder : Representation → Representation → Prop) (candidate : Representation → Prop) : Representation → Prop :=
  fun representation => candidate representation ∧
    ∀ contender, candidate contender → resourcePreorder contender representation → resourcePreorder representation contender

/-- A license records the contracts a representation must retain before economy is considered. -/
structure LicensedRepresentationSyntax (Representation : Type u) (Target : Type v) where
  representation : Representation
  target : Target
  requiredScope : Prop
  applicability : Prop
  authority : Prop
  provenance : Prop
  continuation : Prop
  recovery : Prop
  unlock : Prop

/-- Source obligations retained until the licensed contract and its consequence interfaces are elaborated. -/
inductive RegenerativeEconomyFrontierObligation where
  | bindingSuppliedResourcePreorder
  | inquiryRegenerativeSufficiency
  | licensedScopeApplicabilityAuthority
  | licensedProvenanceContinuationRecoveryUnlock
  | minimalOrNondominatedFrontier
  | noPromisedUniqueGlobalArgmin
  | emptyFrontierWhenNoAdmittedRepresentation
  | namedConsequenceFactorizationOnly
  | discriminatorsContinuationsProvenanceResidualsAndReopening
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

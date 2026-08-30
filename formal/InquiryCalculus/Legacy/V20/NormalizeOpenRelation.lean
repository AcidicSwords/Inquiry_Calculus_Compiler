import InquiryCalculus.Legacy.V20.CompilerArchitecture

/-! # v2.0 Stage 1 normalization boundary -/
namespace InquiryCalculus.Legacy.V20

universe u

/-- The displayed normalization signature remains a source type-shape. -/
inductive NormalizationSignature where
  | openIntermediateFormEndomorphism
  deriving DecidableEq, Repr

/-- The named normalization transformations remain separate source obligations. -/
inductive NormalizationTransformation where
  | alphaRenamePorts
  | normalizeJoinsAndHiding
  | expandDerivedLogicalSyntax
  | canonicalizeTypeReferences
  deriving DecidableEq, Repr

/-- Stage 1 retains only its signature, named transformations, and unproved completion claim. -/
structure NormalizeOpenRelationSyntax (OpenIntermediateForm : Type u) (CompletionRelation : Type u)
    (NormalizerWitness : Type u) where
  sourceForm : OpenIntermediateForm
  completionRelation : CompletionRelation
  normalizerWitness : NormalizerWitness
  signature : NormalizationSignature
  transformation : NormalizationTransformation
  alphaRenamingMayOccur : Prop
  joinsAndHidingMayNormalize : Prop
  derivedLogicalSyntaxMayExpand : Prop
  typeReferencesMayCanonicalize : Prop
  completionRelationIsomorphismClaimUnproved : Prop
  noExecutableNormalizer : Prop
  noSemanticAuthorityStrengthening : Prop

/-- Source obligations retained until normalization and completion preservation are separately checked. -/
inductive NormalizeOpenRelationObligation where
  | openIntermediateFormEndomorphismShape
  | alphaRenamePortsMayOccur
  | normalizeJoinsAndHidingMayOccur
  | expandDerivedLogicalSyntaxMayOccur
  | canonicalizeTypeReferencesMayOccur
  | completionRelationIsomorphismClaimUnproved
  | noExecutableNormalizer
  | noAutomaticIsomorphismProof
  | noSemanticAuthorityStrengthening
  | noAnswerSelectionOrTotalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

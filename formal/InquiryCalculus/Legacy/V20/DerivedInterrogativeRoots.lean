import InquiryCalculus.Legacy.V20.BlockedUnknownResidual

/-! # v2.0 derived interrogative root normal forms

Source-bound first reconstruction of v2.0 lines 5072–5101.  The six roots are transparent
presentations over canonical relation/question/source-program material; they have no independent
primitive, actuality, authority, or scheduling role.
-/
namespace InquiryCalculus.Legacy.V20.DerivedInterrogativeRoots

universe u v w

inductive Root where | expose | orient | factor | polarize | vary | ground
  deriving DecidableEq

inductive Orientation where | forward | converse
  deriving DecidableEq

inductive CanonicalForm where
  | questionConstructor | forwardExposure | converseExposure | factorExposure | positiveAlternative
  | admissibleVariation | supportCheckWarrant
  deriving DecidableEq

structure RootPresentation (Relation : Type u) (Question : Type v) where
  root : Root
  relation : Relation
  question : Question
  canonical : CanonicalForm
  orientation : Option Orientation
  addsPrimitive : Prop
  addsActuality : Prop
  addsAuthority : Prop
  schedules : Prop

def Transparent {Relation : Type u} {Question : Type v}
    (presentation : RootPresentation Relation Question) : Prop :=
  ¬ presentation.addsPrimitive ∧ ¬ presentation.addsActuality ∧
    ¬ presentation.addsAuthority ∧ ¬ presentation.schedules

def canonicalFor : Root → Option Orientation → Option CanonicalForm
  | .expose, none => some .questionConstructor
  | .orient, some .forward => some .forwardExposure
  | .orient, some .converse => some .converseExposure
  | .factor, none => some .factorExposure
  | .polarize, none => some .positiveAlternative
  | .vary, none => some .admissibleVariation
  | .ground, none => some .supportCheckWarrant
  | _, _ => none

/-- Reification supplies the ordinary represented relation on which the same six roots operate;
it does not extend the root family. -/
structure ReifiedQuestionSurface (Question : Type v) (Relation : Type w) where
  checkedOccurrence : Question
  reifiedRelation : Relation

theorem transparentDoesNotAuthorize {Relation : Type u} {Question : Type v}
    (presentation : RootPresentation Relation Question) :
    Transparent presentation → ¬ presentation.addsAuthority := fun transparent => transparent.2.2.1

namespace Countermodel

inductive Relation where | relation deriving DecidableEq
inductive Question where | question deriving DecidableEq

def transparentPresentation (root : Root) (orientation : Option Orientation)
    (canonical : CanonicalForm) : RootPresentation Relation Question where
  root := root
  relation := .relation
  question := .question
  canonical := canonical
  orientation := orientation
  addsPrimitive := False
  addsActuality := False
  addsAuthority := False
  schedules := False

theorem everyRootHasCanonicalForm :
    ∀ root, ∃ orientation canonical, canonicalFor root orientation = some canonical := by
  intro root
  cases root
  · exact ⟨none, .questionConstructor, rfl⟩
  · exact ⟨some .forward, .forwardExposure, rfl⟩
  · exact ⟨none, .factorExposure, rfl⟩
  · exact ⟨none, .positiveAlternative, rfl⟩
  · exact ⟨none, .admissibleVariation, rfl⟩
  · exact ⟨none, .supportCheckWarrant, rfl⟩

theorem transparentPresentationIsTransparent (root : Root) (orientation : Option Orientation)
    (canonical : CanonicalForm) : Transparent (transparentPresentation root orientation canonical) :=
  ⟨False.elim, False.elim, False.elim, False.elim⟩

theorem orientationIsNotInverseSynthesis :
    canonicalFor .orient (some .forward) ≠ canonicalFor .orient (some .converse) := by decide

theorem bareOrientIsIllShaped : canonicalFor .orient none = none := rfl
theorem nonOrientCannotClaimOrientation : canonicalFor .expose (some .forward) = none := rfl
theorem exposeIsQuestionConstructor : canonicalFor .expose none = some .questionConstructor := rfl
theorem factorIsFactorExposure : canonicalFor .factor none = some .factorExposure := rfl
theorem polarizeIsPositiveAlternative : canonicalFor .polarize none = some .positiveAlternative := rfl
theorem varyIsAdmissibleVariation : canonicalFor .vary none = some .admissibleVariation := rfl
theorem groundIsSupportCheckWarrant : canonicalFor .ground none = some .supportCheckWarrant := rfl

def reifiedQuestion : ReifiedQuestionSurface Question Relation := ⟨.question, .relation⟩

theorem reifiedQuestionReusesRootFamily :
    ∀ root, ∃ orientation canonical,
      canonicalFor root orientation = some canonical ∧ reifiedQuestion.reifiedRelation = .relation := by
  intro root
  rcases everyRootHasCanonicalForm root with ⟨orientation, canonical, shaped⟩
  exact ⟨orientation, canonical, shaped, rfl⟩

end Countermodel
end InquiryCalculus.Legacy.V20.DerivedInterrogativeRoots

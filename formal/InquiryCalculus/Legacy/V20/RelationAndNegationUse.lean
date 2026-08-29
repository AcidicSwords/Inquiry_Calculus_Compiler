import InquiryCalculus.Legacy.V20.DerivedBoundaryCrossing

/-! # v2.0 relation-and-negation-use boundary -/
namespace InquiryCalculus.Legacy.V20

universe u v w x y z

/-- An immutable, typed, represented relation occurrence with all declared use indexes. -/
structure RelationUseSyntax
    (RelationIdentity : Type u) (Bindings : Type v) (Orientation : Type w) (Scope : Type x)
    (Applicability Grain Support Warrant : Type) where
  relationIdentity : RelationIdentity
  bindings : Bindings
  orientation : Orientation
  scope : Scope
  applicability : Applicability
  grain : Grain
  support : Support
  warrant : Warrant

/--
An oriented ordinary relation in a negation role.  Its admitted incidences require a supplied route
to a non-circular positive-departure witness; the reverse use remains independent.
-/
structure NegationUseSyntax
    (RelationIdentity : Type u) (Bindings : Type v) (Orientation : Type w) (Scope : Type x)
    (Applicability Grain Support Warrant : Type) (Source Candidate Presentation Coverage Witness : Type)
    extends RelationUseSyntax RelationIdentity Bindings Orientation Scope Applicability Grain Support Warrant where
  incidence : Source → Candidate → Prop
  determinationPresentation : Presentation
  semanticCoverage : Coverage
  soundnessDerivation : ∀ source candidate, incidence source candidate → Witness

/-- Boolean complement, ontology, and reverse-use synthesis remain excluded. -/
inductive RelationAndNegationUseObligation where
  | immutableOccurrence
  | typedRepresentedRelation
  | explicitRelationIdentity
  | explicitBindingsOrientationScopeApplicabilityGrain
  | supportAndWarrant
  | orientedIncidence
  | determinationPresentation
  | semanticCoverage
  | nonCircularWitnessSoundness
  | ordinaryRelationRole
  | booleanComplement
  | newOntologicalCarrier
  | reverseUseSynthesis
  | answerSelection
  | totalSolver
  | noProgramOrRustAuthority
  deriving DecidableEq, Repr

end InquiryCalculus.Legacy.V20

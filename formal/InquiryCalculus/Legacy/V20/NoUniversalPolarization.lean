import InquiryCalculus.Legacy.V20.DerivedInterrogativeRoots

/-! # v2.0 no-universal-polarization boundary

Source-bound reconstruction of the unproved law at v2.0 lines 5103–5108.  The five named
polarization relations retain distinct identities.  A binding may supply a correspondence, but a
generated polarization candidate alone grants no negation use, exteriority, departure witness, or
standing breaker.  The source law remains an obligation rather than a promoted theorem.
-/
namespace InquiryCalculus.Legacy.V20.NoUniversalPolarization

universe u

inductive PolarizationKind where
  | logicalComplement
  | counterexample
  | alternativeDesign
  | separator
  | contextualPositiveNegation
  deriving DecidableEq

inductive AdmittedSemantic where
  | negationUse
  | exterior
  | departureWitness
  | standingBreaker
  deriving DecidableEq

/-- Availability and cross-kind correspondence are supplied by a binding.  Availability of two
kinds does not itself assert their correspondence. -/
structure BindingPolarization (Binding : Type u) where
  available : Binding → PolarizationKind → Prop
  corresponds : Binding → PolarizationKind → PolarizationKind → Prop

/-- A generated result retains its binding and exact relation kind. -/
structure PolarizedCandidate (Binding : Type u) where
  binding : Binding
  kind : PolarizationKind
  generated : Prop

/-- Semantic admission is an independent relation over a candidate, not a field filled by
`Polarize`. -/
structure AdmissionBoundary (Binding : Type u) where
  admitted : PolarizedCandidate Binding → AdmittedSemantic → Prop

def MayTransport {Binding : Type u} (surface : BindingPolarization Binding) (binding : Binding)
    (source target : PolarizationKind) : Prop :=
  source = target ∨ surface.corresponds binding source target

/-- The exact proof obligations retained from the source law. -/
inductive NoUniversalPolarizationObligation where
  | fiveRelationsRemainDistinct
  | correspondenceIsBindingSupplied
  | noManufacturedNegationUse
  | noManufacturedExterior
  | noManufacturedDepartureWitness
  | noManufacturedStandingBreaker
  | noProgramOrRustAuthority
  deriving DecidableEq

namespace Countermodel

inductive Binding where | separated | bridged
  deriving DecidableEq

def surface : BindingPolarization Binding where
  available := fun _ _ => True
  corresponds := fun binding source target =>
    match binding with
    | .separated => source = target
    | .bridged => source = target ∨
        (source = .counterexample ∧ target = .separator)

def allKinds : List PolarizationKind :=
  [.logicalComplement, .counterexample, .alternativeDesign, .separator,
    .contextualPositiveNegation]

theorem fiveKindsArePairwiseDistinct : allKinds.Nodup := by decide

theorem separatedCannotTransportCounterexampleToComplement :
    ¬ MayTransport surface .separated .counterexample .logicalComplement := by
  simp [MayTransport, surface]

theorem separatedCannotTransportAlternativeToSeparator :
    ¬ MayTransport surface .separated .alternativeDesign .separator := by
  simp [MayTransport, surface]

theorem bridgedCanTransportCounterexampleToSeparator :
    MayTransport surface .bridged .counterexample .separator := by
  simp [MayTransport, surface]

def generatedCandidate : PolarizedCandidate Binding where
  binding := .separated
  kind := .alternativeDesign
  generated := True

def noAdmission : AdmissionBoundary Binding where
  admitted := fun _ _ => False

theorem generatedCandidateHasNoAutomaticAdmission :
    generatedCandidate.generated ∧
      ∀ semantic, ¬ noAdmission.admitted generatedCandidate semantic := by
  exact ⟨True.intro, fun _ => False.elim⟩

theorem generatedCandidateDoesNotMakeNegationUse :
    ¬ noAdmission.admitted generatedCandidate .negationUse := False.elim

theorem generatedCandidateDoesNotMakeExterior :
    ¬ noAdmission.admitted generatedCandidate .exterior := False.elim

theorem generatedCandidateDoesNotMakeDepartureWitness :
    ¬ noAdmission.admitted generatedCandidate .departureWitness := False.elim

theorem generatedCandidateDoesNotMakeStandingBreaker :
    ¬ noAdmission.admitted generatedCandidate .standingBreaker := False.elim

def explicitAdmission : AdmissionBoundary Binding where
  admitted := fun candidate semantic =>
    candidate = generatedCandidate ∧ semantic = .negationUse

theorem explicitAdmissionIsIndependent :
    explicitAdmission.admitted generatedCandidate .negationUse := ⟨rfl, rfl⟩

end Countermodel
end InquiryCalculus.Legacy.V20.NoUniversalPolarization

import InquiryCalculus.Legacy.V20.ContinuationSufficiency

/-! # Regenerative preservation

Source-bound realization of v2.0 lines 4669–4675.  Regeneration is a supplied relation from the
retained target to the specified replaced source.  Equal current quotient images and source labels
do not select a representative or establish regeneration.
-/
namespace InquiryCalculus.Legacy.V20.RegenerativePreservation

open ExactRepresentationQuotient

universe u v

structure RegenerationContext (Source : Type u) (Target : Type v) where
  protectedFutureUse : Source → Prop
  regenerates : Target → Source → Prop

def RegenerativelySufficient {Source : Type u} {Target : Type v}
    (quotient : ProposedQuotient Source Target) (context : RegenerationContext Source Target) : Prop :=
  ∀ source target, quotient.map source = target → context.protectedFutureUse source →
    context.regenerates target source

namespace Countermodel

open ExactRepresentationQuotient.Countermodel

def witnessContext : RegenerationContext Source ExactTarget where
  protectedFutureUse := fun source => source = .a
  regenerates := fun target source => target = .ab ∧ source = .a

def wrongSourceContext : RegenerationContext Source ExactTarget where
  protectedFutureUse := fun source => source = .a
  regenerates := fun target source => target = .ab ∧ source = .b

theorem witnessContextIsRegenerativelySufficient :
    RegenerativelySufficient exactMap witnessContext := by
  intro source target image futureUse
  cases source <;> cases target
  · exact ⟨rfl, rfl⟩
  · change ExactTarget.ab = ExactTarget.c at image
    exact ExactTarget.noConfusion image
  · exact Source.noConfusion futureUse
  · exact Source.noConfusion futureUse
  · exact Source.noConfusion futureUse
  · exact Source.noConfusion futureUse

theorem wrongSourceContextFails : ¬ RegenerativelySufficient exactMap wrongSourceContext := by
  intro sufficient
  have recovered := sufficient .a .ab rfl rfl
  exact Source.noConfusion recovered.2

theorem currentConsequenceDoesNotImplyRegeneration :
    ConsequenceSufficient exactContext exactMap ∧ ¬ RegenerativelySufficient exactMap wrongSourceContext :=
  ⟨exactMapIsConsequenceSufficient, wrongSourceContextFails⟩

end Countermodel
end InquiryCalculus.Legacy.V20.RegenerativePreservation

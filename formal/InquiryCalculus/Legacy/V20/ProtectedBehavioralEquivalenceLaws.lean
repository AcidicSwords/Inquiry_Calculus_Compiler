import InquiryCalculus.Legacy.V20.ProtectedBehavioralEquivalence

/-! # Checks downstream of the recovered v2.0 definitions

This module is a separate proof/model boundary. It does not change the source
carrier, supply a general context executor, decide arbitrary equivalence, or
promote a successor definition. The Boolean model supplies its own observation
function solely to discriminate finite tested coverage from a whole horizon.
-/
namespace InquiryCalculus.Legacy.V20.ProtectedBehavioralEquivalenceLaws

universe u

variable {Object : Type u} {Term : Object → Object → Type u}
variable (S : ProtectedBehavioralEquivalenceContext Object Term) {A B : Object}

theorem equivalenceReflexive (H : S.Context A B → Prop) (f : Term A B) :
    protectedEquivalenceDefinitionShape S H f f := by
  intro _ _
  rfl

theorem equivalenceSymmetric (H : S.Context A B → Prop) (f g : Term A B)
    (equal : protectedEquivalenceDefinitionShape S H f g) :
    protectedEquivalenceDefinitionShape S H g f := by
  intro context member
  exact (equal context member).symm

theorem equivalenceTransitive (H : S.Context A B → Prop) (f g h : Term A B)
    (first : protectedEquivalenceDefinitionShape S H f g)
    (second : protectedEquivalenceDefinitionShape S H g h) :
    protectedEquivalenceDefinitionShape S H f h := by
  intro context member
  exact (first context member).trans (second context member)

/-- A proof of the recovered statement, separate from its source classification. -/
theorem horizonRestriction (H H' : S.Context A B → Prop) :
    horizonMonotonicityUnproved S H H' := by
  intro inclusion _ _ equal context member
  exact equal context (inclusion context member)

theorem separatorRefutesEquivalence (H : S.Context A B → Prop) (f g : Term A B)
    (context : S.Context A B) (separator : separatorFamilyDefinitionShape S H f g context) :
    ¬ protectedEquivalenceDefinitionShape S H f g := by
  intro equal
  exact separator.2 (equal context separator.1)

/-- The existential direction uses classical double-negation elimination.
It is not an algorithm for finding a separating context. -/
theorem separatorCharacterizationClassical (H : S.Context A B → Prop) (f g : Term A B) :
    separatorCharacterizationUnproved S H f g := by
  constructor
  · intro notEquivalent
    apply Classical.byContradiction
    intro noSeparator
    apply notEquivalent
    intro context member
    apply Classical.byContradiction
    intro different
    exact noSeparator ⟨context, member, different⟩
  · intro hasSeparator
    cases hasSeparator with
    | intro context separator => exact separatorRefutesEquivalence S H f g context separator

theorem exactImpliesTested (H : S.Context A B → Prop) (D : List (S.Context A B))
    (f g : Term A B) (within : ∀ context, context ∈ D → H context)
    (equal : protectedEquivalenceDefinitionShape S H f g) :
    workingNondistinctionDefinitionShape S D f g := by
  intro context member
  exact equal context (within context member)

/-- Enumerating every protected context is one sufficient completeness license,
not a claim that it is the only possible binding-relative license. -/
theorem testedImpliesExactUnderCoverage (H : S.Context A B → Prop) (D : List (S.Context A B))
    (f g : Term A B) (covers : ∀ context, H context → context ∈ D)
    (testedEqual : workingNondistinctionDefinitionShape S D f g) :
    protectedEquivalenceDefinitionShape S H f g := by
  intro context member
  exact testedEqual context (covers context member)

theorem separatorOutsideTests (H : S.Context A B → Prop) (D : List (S.Context A B))
    (f g : Term A B) (testedEqual : workingNondistinctionDefinitionShape S D f g)
    (context : S.Context A B) (separator : separatorFamilyDefinitionShape S H f g context) :
    context ∉ D := by
  intro member
  exact separator.2 (testedEqual context member)

/-- With a tested sample, one possible context cannot exhibit the coverage gap.
The nonempty-sample premise matters: an empty test list can agree vacuously. -/
theorem oneContextSampleComplete [Subsingleton (S.Context A B)]
    (H : S.Context A B → Prop) (D : List (S.Context A B)) (f g : Term A B)
    (sample : S.Context A B) (member : sample ∈ D)
    (testedEqual : workingNondistinctionDefinitionShape S D f g) :
    protectedEquivalenceDefinitionShape S H f g := by
  intro context _
  have same : context = sample := Subsingleton.elim context sample
  cases same
  exact testedEqual sample member

namespace FiniteCoverage

def observation : ProtectedBehavioralEquivalenceContext Unit (fun _ _ => Bool) where
  Context := fun _ _ => Bool
  Consequence := fun _ => Bool
  consequence := fun context term => context && term

def narrow : Bool → Prop := fun context => context = false
def whole : Bool → Prop := fun _ => True
def tested : List Bool := [false]

theorem testedAgreement :
    workingNondistinctionDefinitionShape observation (A := ()) (B := ()) tested false true := by
  intro context member
  cases member with
  | head => rfl
  | tail _ absent => cases absent

theorem testedWithinWhole : ∀ context, context ∈ tested → whole context := by
  intro _ _
  trivial

theorem protectedSeparator :
    separatorFamilyDefinitionShape observation (A := ()) (B := ()) whole false true true := by
  constructor
  · trivial
  · exact Bool.noConfusion

theorem separatorUntested : true ∉ tested := by
  intro member
  cases member with
  | tail _ absent => cases absent

theorem notWholeEquivalent :
    ¬ protectedEquivalenceDefinitionShape observation (A := ()) (B := ()) whole false true := by
  intro equivalent
  exact protectedSeparator.2 (equivalent true True.intro)

theorem narrowEquivalent :
    protectedEquivalenceDefinitionShape observation (A := ()) (B := ()) narrow false true := by
  intro context member
  cases member
  rfl

theorem narrowWithinWhole : ∀ context, narrow context → whole context := by
  intro _ _
  trivial

theorem completeTestsDistinguish :
    ¬ workingNondistinctionDefinitionShape observation (A := ()) (B := ()) [false, true] false true := by
  intro agreement
  have member : (true : Bool) ∈ [false, true] := .tail false (.head [])
  exact protectedSeparator.2 (agreement true member)

theorem finiteCoverageGap :
    workingNondistinctionDefinitionShape observation (A := ()) (B := ()) tested false true ∧
      ¬ protectedEquivalenceDefinitionShape observation (A := ()) (B := ()) whole false true :=
  ⟨testedAgreement, notWholeEquivalent⟩

def oneContextObservation : ProtectedBehavioralEquivalenceContext Unit (fun _ _ => Bool) where
  Context := fun _ _ => Unit
  Consequence := fun _ => Bool
  consequence := fun _ term => term

theorem emptyTestsAgree :
    workingNondistinctionDefinitionShape oneContextObservation (A := ()) (B := ()) [] false true := by
  intro _ member
  cases member

theorem oneContextEmptyTestGap :
    ¬ protectedEquivalenceDefinitionShape oneContextObservation (A := ()) (B := ()) (fun _ => True) false true := by
  intro equivalent
  exact Bool.noConfusion (equivalent () True.intro)

end FiniteCoverage

end InquiryCalculus.Legacy.V20.ProtectedBehavioralEquivalenceLaws

import InquiryCalculus.Legacy.V20.HomWiseQuotient

/-! # Relational continuation descent

Downstream realization of the v2.0 definition at lines 4108–4151.  The
source's existential next representation is retained separately from descent
relative to a supplied protected next representation.  Relations are never
executed or totalized here, and equality compares their whole images.
-/
namespace InquiryCalculus.Legacy.V20.OperatorDescent

universe u

/-- A universe-polymorphic typed relation used only for this source boundary. -/
def Relation (A B : Type u) := A → B → Prop

/-- A function embedded as its exact graph, not identified with every relation. -/
def functionGraph {A B : Type u} (f : A → B) : Relation A B :=
  fun a b => f a = b

/-- Left-to-right relational composition with the whole mediator fiber retained. -/
def serialCompose {A B C : Type u} (R : Relation A B) (S : Relation B C) : Relation A C :=
  fun a c => ∃ b, R a b ∧ S b c

def RelationallyEqual {A B : Type u} (R S : Relation A B) : Prop :=
  ∀ a b, R a b ↔ S a b

/-- The complete represented image after the continuation, not a selected return. -/
def wholeNextImage {X X' S' : Type u} (a : Relation X X') (q' : X' → S')
    (x : X) (s' : S') : Prop :=
  ∃ x', a x x' ∧ q' x' = s'

/-- `q' ∘ a = ā ∘ q`, with source orientation made explicit. -/
def DescentSquare {X S X' S' : Type u} (q : X → S) (a : Relation X X')
    (q' : X' → S') (descended : Relation S S') : Prop :=
  RelationallyEqual (serialCompose a (functionGraph q'))
    (serialCompose (functionGraph q) descended)

/-- Exact source-definition shape: the next reduced carrier, map, and relation
are existential witnesses.  This alone does not say that the next map protects
any independently supplied future distinction. -/
structure ContinuationDescentWitness {X S X' : Type u} (q : X → S)
    (a : Relation X X') where
  reducedNext : Type u
  quotientNext : X' → reducedNext
  descended : Relation S reducedNext
  commutes : DescentSquare q a quotientNext descended

def continuationDescentDefinitionShape {X S X' : Type u} (q : X → S)
    (a : Relation X X') : Prop :=
  Nonempty (ContinuationDescentWitness q a)

/-- Descent relative to a supplied next representation. -/
def DescendsTo {X S X' S' : Type u} (q : X → S) (a : Relation X X')
    (q' : X' → S') : Prop :=
  ∃ descended, DescentSquare q a q' descended

/-- Whole-image congruence on each current quotient fiber. -/
def FiberStable {X S X' S' : Type u} (q : X → S) (a : Relation X X')
    (q' : X' → S') : Prop :=
  ∀ x y, q x = q y → ∀ s', wholeNextImage a q' x s' ↔ wholeNextImage a q' y s'

/-- The largest relation induced by the represented source fiber. -/
def canonicalDescended {X S X' S' : Type u} (q : X → S) (a : Relation X X')
    (q' : X' → S') : Relation S S' :=
  fun s s' => ∃ x, q x = s ∧ wholeNextImage a q' x s'

theorem descentSquareIffPointwise {X S X' S' : Type u} (q : X → S)
    (a : Relation X X') (q' : X' → S') (descended : Relation S S') :
    DescentSquare q a q' descended ↔
      ∀ x s', wholeNextImage a q' x s' ↔ descended (q x) s' := by
  constructor
  · intro square x s'
    have atPoint := square x s'
    simpa [serialCompose, functionGraph, wholeNextImage] using atPoint
  · intro pointwise x s'
    simpa [DescentSquare, RelationallyEqual, serialCompose, functionGraph,
      wholeNextImage] using pointwise x s'

theorem descentImpliesFiberStable {X S X' S' : Type u} (q : X → S)
    (a : Relation X X') (q' : X' → S') : DescendsTo q a q' → FiberStable q a q' := by
  rintro ⟨descended, square⟩ x y same s'
  have pointwise := (descentSquareIffPointwise q a q' descended).mp square
  rw [pointwise x s', pointwise y s', same]

theorem fiberStableCanonicalSquare {X S X' S' : Type u} (q : X → S)
    (a : Relation X X') (q' : X' → S') (stable : FiberStable q a q') :
    DescentSquare q a q' (canonicalDescended q a q') := by
  apply (descentSquareIffPointwise q a q' _).mpr
  intro x s'
  constructor
  · intro image
    exact ⟨x, rfl, image⟩
  · rintro ⟨y, same, image⟩
    exact (stable y x same s').mp image

theorem descentIffFiberStable {X S X' S' : Type u} (q : X → S)
    (a : Relation X X') (q' : X' → S') :
    DescendsTo q a q' ↔ FiberStable q a q' := by
  constructor
  · exact descentImpliesFiberStable q a q'
  · intro stable
    exact ⟨canonicalDescended q a q', fiberStableCanonicalSquare q a q' stable⟩

namespace Countermodel

inductive State2 where
  | left
  | right
  deriving DecidableEq

open State2

def currentQuotient (_ : State2) : Unit := ()

def nextValue : State2 → Bool
  | left => false
  | right => true

def continuation : Relation State2 Bool := functionGraph nextValue

def protectedNext (value : Bool) : Bool := value

theorem presentEquivalent : currentQuotient left = currentQuotient right := rfl

theorem protectedFutureSeparated :
    wholeNextImage continuation protectedNext left false ∧
    ¬ wholeNextImage continuation protectedNext right false := by
  constructor
  · exact ⟨false, rfl, rfl⟩
  · rintro ⟨value, step, represented⟩
    change nextValue right = value at step
    change protectedNext value = false at represented
    have impossible : true = false := by
      calc
        true = value := by simpa [nextValue] using step
        _ = false := by simpa [protectedNext] using represented
    exact Bool.noConfusion impossible

theorem notFiberStable :
    ¬ FiberStable currentQuotient continuation protectedNext := by
  intro stable
  exact protectedFutureSeparated.2
    ((stable left right presentEquivalent false).mp protectedFutureSeparated.1)

theorem noProtectedFixedDescent :
    ¬ DescendsTo currentQuotient continuation protectedNext := by
  intro descent
  exact notFiberStable (descentImpliesFiberStable currentQuotient continuation protectedNext descent)

/-- An unconstrained next representation can erase the future distinction.
This witnesses the source-definition shape but not protected recurrent state. -/
def collapsedNext (_ : Bool) : Unit := ()

def collapsedDescended : Relation Unit Unit := functionGraph id

theorem collapsedSquare :
    DescentSquare currentQuotient continuation collapsedNext collapsedDescended := by
  apply (descentSquareIffPointwise currentQuotient continuation collapsedNext _).mpr
  intro state output
  constructor
  · intro _
    cases output
    rfl
  · intro _
    exact ⟨nextValue state, rfl, rfl⟩

theorem existentialDefinitionStillHolds :
    continuationDescentDefinitionShape currentQuotient continuation :=
  ⟨{ reducedNext := Unit, quotientNext := collapsedNext,
      descended := collapsedDescended, commutes := collapsedSquare }⟩

end Countermodel
end InquiryCalculus.Legacy.V20.OperatorDescent

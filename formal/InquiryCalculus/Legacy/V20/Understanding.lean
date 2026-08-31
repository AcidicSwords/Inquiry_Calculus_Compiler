import InquiryCalculus.Legacy.V20.DifferentiateOnlyEnough

/-! # Understanding as a derived role

Exact source-bound alias at v2.0 lines 4218–4228.  This declaration adds no
carrier and says nothing about whether stored historical content or memory is
necessary; it only names the already recovered inquiry-regenerative predicate.
-/
namespace InquiryCalculus.Legacy.V20.Understanding

open RegenerativeSufficiency

universe u v w x y

def Understands {Component : Type u} {Value : Component → Type v}
    {Source : Type w} {Representation : Type x} {RevisionRole : Type y}
    (components : ProtectedComponentFamily Component Value Source)
    (reconstruction : TypedReconstructionFamily Component Value Representation)
    (revision : InquiryRevisionFamily RevisionRole Representation Source)
    (representation : Representation) (source : Source) : Prop :=
  InquiryRegenerativeSufficient components reconstruction revision representation source

theorem understandsIffInquiryRegenerative
    {Component : Type u} {Value : Component → Type v}
    {Source : Type w} {Representation : Type x} {RevisionRole : Type y}
    (components : ProtectedComponentFamily Component Value Source)
    (reconstruction : TypedReconstructionFamily Component Value Representation)
    (revision : InquiryRevisionFamily RevisionRole Representation Source)
    (representation : Representation) (source : Source) :
    Understands components reconstruction revision representation source ↔
      InquiryRegenerativeSufficient components reconstruction revision representation source :=
  Iff.rfl

namespace Countermodel

open RegenerativeSufficiency.Countermodel

def availableRevision : InquiryRevisionFamily RevisionRole1 Source2 Source2 where
  requiredForRevision := reopeningRequired
  retainsOrRegenerates := fun _ _ _ => True

theorem completeWithRevisionUnderstands (source : Source2) :
    Understands protectedComponents completeReconstruction availableRevision source source := by
  constructor
  · exact completeIsRegenerative source
  · intro _ _
    trivial

theorem completeWithoutRevisionDoesNotUnderstand (source : Source2) :
    ¬ Understands protectedComponents completeReconstruction missingRevision source source :=
  completeButNotInquiryRegenerative source

end Countermodel
end InquiryCalculus.Legacy.V20.Understanding

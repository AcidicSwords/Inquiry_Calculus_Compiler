import InquiryCalculus.Meta.Ambient
import InquiryCalculus.Legacy.V20.Binding
import InquiryCalculus.Legacy.V20.Types
import InquiryCalculus.Legacy.V20.Forms
import InquiryCalculus.Legacy.V20.Relations
import InquiryCalculus.Legacy.V20.Refinement
import InquiryCalculus.Legacy.V20.FormulaGrammar
import InquiryCalculus.Legacy.V20.MinimalLogicalBasis
import InquiryCalculus.Legacy.V20.RelationExpressionIR
import InquiryCalculus.Legacy.V20.RelationSchemaPorts
import InquiryCalculus.Legacy.V20.PartialBindingFiber
import InquiryCalculus.Legacy.V20.CanonicalQuestionSyntax
import InquiryCalculus.Legacy.V20.AnswerCarrierValidity
import InquiryCalculus.Legacy.V20.PropositionNotWarrant
import InquiryCalculus.Legacy.V20.ManyQuestionsGeneration
import InquiryCalculus.Legacy.V20.DischargeModeSyntax
import InquiryCalculus.Legacy.V20.QuestionCompositionSyntax
import InquiryCalculus.Legacy.V20.QuestionConditionedDiscrimination
import InquiryCalculus.Legacy.V20.QuestionRefinementPreorder
import InquiryCalculus.Legacy.V20.QuestionRefinementSemantics
import InquiryCalculus.Legacy.V20.QuestionJointActiveRefinement
import InquiryCalculus.Legacy.V20.QuestionRedundancy
import InquiryCalculus.Legacy.V20.PrecisionNotImprovement
import InquiryCalculus.Legacy.V20.RelationalSections
import InquiryCalculus.Legacy.V20.SolutionFibers
import InquiryCalculus.Legacy.V20.QuestionStructuredHole
import InquiryCalculus.Legacy.V20.RelationalAbstraction
import InquiryCalculus.Legacy.V20.AbstractionByRemoval
import InquiryCalculus.Legacy.V20.SolutionFieldWeb
import InquiryCalculus.Legacy.V20.IndexedMeetRefinement
import InquiryCalculus.Legacy.V20.PropertyImageHole
import InquiryCalculus.Legacy.V20.ProtectedDetermination
import InquiryCalculus.Legacy.V20.ExactDeterminationSignature
import InquiryCalculus.Legacy.V20.ResidualAmbiguity
import InquiryCalculus.Legacy.V20.RepresentationDefect
import InquiryCalculus.Legacy.V20.SeparatingContextQuestion

/-!
# Inquiry Calculus formal successor

This library is the machine-checked candidate successor. Its current import surface is deliberately
limited to the checked ambient boundary and the source-bound v2.0 binding/type syntax; later
modules enter only through a live frontier and checked dependency propagation.
-/

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

/-!
# Inquiry Calculus formal successor

This library is the machine-checked candidate successor. Its current import surface is deliberately
limited to the checked ambient boundary and the source-bound v2.0 binding/type syntax; later
modules enter only through a live frontier and checked dependency propagation.
-/

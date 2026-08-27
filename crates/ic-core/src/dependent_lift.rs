//! Finite supported-family lifting over ordinary checked questions.
//!
//! This is the finite derived `LiftQ_F` view. It preserves the complete proof-carrying parent
//! answer, tags every child position by its parent member, and inherits each ordinary child
//! question's exact port type, discharge mode, and route context. It creates no source
//! constructor, runtime opcode, scheduler, or canonical dependent-family artifact.

use std::collections::BTreeSet;

use thiserror::Error;

use crate::{
    AdmittedFiniteAnswerSet, ApplicabilityRef, CompletionCandidateRef, CoverageRef, DischargeMode,
    GrainRef, HorizonRef, OpenQuery, OpenQueryCatalog, OpenQueryCheckError, OpenQueryError,
    QueryRef, RelationRef, ScopeRef, SupportRef, TypeRef, TypeSymbol, WarrantRef,
};

/// One materialized child question indexed by the exact parent answer member that selected it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MaterializedChildQuestion {
    parent: CompletionCandidateRef,
    question: QueryRef,
}

impl MaterializedChildQuestion {
    #[must_use]
    pub const fn new(parent: CompletionCandidateRef, question: QueryRef) -> Self {
        Self { parent, question }
    }

    #[must_use]
    pub const fn parent(self) -> CompletionCandidateRef {
        self.parent
    }

    #[must_use]
    pub const fn question(self) -> QueryRef {
        self.question
    }
}

/// One checked member of the materialized dependent question family.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaggedChildQuestion {
    parent: CompletionCandidateRef,
    question: QueryRef,
    value: OpenQuery,
}

impl TaggedChildQuestion {
    #[must_use]
    pub const fn parent(&self) -> CompletionCandidateRef {
        self.parent
    }

    #[must_use]
    pub const fn question(&self) -> QueryRef {
        self.question
    }

    #[must_use]
    pub const fn value(&self) -> &OpenQuery {
        &self.value
    }
}

/// One position `(a, i)` in the finite dependent sum of child ports.
///
/// The parent tag is part of identity. Equal child query/port names, types, or modes under two
/// parent members therefore remain distinct positions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaggedChildPort {
    parent: CompletionCandidateRef,
    question: QueryRef,
    relation: RelationRef,
    port: TypeSymbol,
    ty: TypeRef,
    mode: DischargeMode,
    scope: ScopeRef,
    applicability: ApplicabilityRef,
    grain: GrainRef,
    horizon: HorizonRef,
    support: SupportRef,
    warrant: Option<WarrantRef>,
}

impl TaggedChildPort {
    #[must_use]
    pub const fn parent(&self) -> CompletionCandidateRef {
        self.parent
    }

    #[must_use]
    pub const fn question(&self) -> QueryRef {
        self.question
    }

    #[must_use]
    pub const fn relation(&self) -> RelationRef {
        self.relation
    }

    #[must_use]
    pub const fn port(&self) -> &TypeSymbol {
        &self.port
    }

    #[must_use]
    pub const fn ty(&self) -> TypeRef {
        self.ty
    }

    #[must_use]
    pub const fn mode(&self) -> DischargeMode {
        self.mode
    }

    #[must_use]
    pub const fn scope(&self) -> ScopeRef {
        self.scope
    }

    #[must_use]
    pub const fn applicability(&self) -> ApplicabilityRef {
        self.applicability
    }

    #[must_use]
    pub const fn grain(&self) -> GrainRef {
        self.grain
    }

    #[must_use]
    pub const fn horizon(&self) -> HorizonRef {
        self.horizon
    }

    #[must_use]
    pub const fn support(&self) -> SupportRef {
        self.support
    }

    #[must_use]
    pub const fn warrant(&self) -> Option<WarrantRef> {
        self.warrant
    }
}

/// A whole-parent finite dependent question. Every parent member is materialized exactly once.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompleteFiniteSupportedFamilyLift {
    parent: AdmittedFiniteAnswerSet,
    coverage: CoverageRef,
    children: Vec<TaggedChildQuestion>,
    positions: Vec<TaggedChildPort>,
}

impl CompleteFiniteSupportedFamilyLift {
    #[must_use]
    pub const fn parent(&self) -> &AdmittedFiniteAnswerSet {
        &self.parent
    }

    #[must_use]
    pub const fn coverage(&self) -> CoverageRef {
        self.coverage
    }

    #[must_use]
    pub fn children(&self) -> &[TaggedChildQuestion] {
        &self.children
    }

    #[must_use]
    pub fn positions(&self) -> &[TaggedChildPort] {
        &self.positions
    }
}

/// A proper finite materialization. Its missing parent members remain explicitly `Unknown`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnknownFiniteSupportedFamilyLift {
    parent: AdmittedFiniteAnswerSet,
    coverage: CoverageRef,
    children: Vec<TaggedChildQuestion>,
    positions: Vec<TaggedChildPort>,
    uncovered_parents: Vec<CompletionCandidateRef>,
}

impl UnknownFiniteSupportedFamilyLift {
    #[must_use]
    pub const fn parent(&self) -> &AdmittedFiniteAnswerSet {
        &self.parent
    }

    #[must_use]
    pub const fn coverage(&self) -> CoverageRef {
        self.coverage
    }

    #[must_use]
    pub fn children(&self) -> &[TaggedChildQuestion] {
        &self.children
    }

    #[must_use]
    pub fn positions(&self) -> &[TaggedChildPort] {
        &self.positions
    }

    #[must_use]
    pub fn uncovered_parents(&self) -> &[CompletionCandidateRef] {
        &self.uncovered_parents
    }
}

/// The finite lifting result keeps incomplete materialization separate from a complete family.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FiniteSupportedFamilyLift {
    Complete(CompleteFiniteSupportedFamilyLift),
    Unknown(UnknownFiniteSupportedFamilyLift),
}

/// Constructs the finite dependent sum of child positions and its pointwise inherited authority.
///
/// The materialized family must be nonempty and contain at most one checked child question for
/// each member of the parent answer. Exact parent coverage returns `Complete`; a proper subset
/// returns `Unknown` together with every uncovered member. Extra or duplicate parent tags reject.
pub fn lift_finite_supported_family<C: OpenQueryCatalog>(
    parent: AdmittedFiniteAnswerSet,
    mut materialized: Vec<MaterializedChildQuestion>,
    coverage: CoverageRef,
    catalog: &C,
) -> Result<FiniteSupportedFamilyLift, FiniteSupportedFamilyLiftError> {
    if materialized.is_empty() {
        return Err(FiniteSupportedFamilyLiftError::EmptyMaterialization);
    }
    materialized.sort_unstable_by_key(|child| child.parent);
    if let Some(pair) = materialized
        .windows(2)
        .find(|pair| pair[0].parent == pair[1].parent)
    {
        return Err(FiniteSupportedFamilyLiftError::DuplicateParent(
            pair[0].parent,
        ));
    }

    let parent_members: BTreeSet<_> = parent.candidates().iter().copied().collect();
    let mut materialized_members = BTreeSet::new();
    let mut children = Vec::with_capacity(materialized.len());
    let mut positions = Vec::new();
    for child in materialized {
        if !parent_members.contains(&child.parent) {
            return Err(FiniteSupportedFamilyLiftError::ForeignParent(child.parent));
        }
        materialized_members.insert(child.parent);
        let question = catalog.resolve_open_query(child.question).ok_or(
            FiniteSupportedFamilyLiftError::UnresolvedQuestion(child.question),
        )?;
        let calculated = question.query_ref()?;
        if calculated != child.question {
            return Err(FiniteSupportedFamilyLiftError::QuestionIdentityMismatch {
                reference: child.question,
                calculated,
            });
        }
        question.check(catalog)?;
        let schema = catalog.resolve_relation_schema(question.relation()).ok_or(
            FiniteSupportedFamilyLiftError::UnresolvedRelation(question.relation()),
        )?;
        let context = question.context();
        for open in question.open_ports() {
            let ty = schema
                .ports()
                .iter()
                .find(|port| port.name() == open.port())
                .map(crate::RelationPort::ty)
                .expect("a checked query contains only schema ports");
            positions.push(TaggedChildPort {
                parent: child.parent,
                question: child.question,
                relation: question.relation(),
                port: open.port().clone(),
                ty,
                mode: open.mode(),
                scope: context.scope(),
                applicability: context.applicability(),
                grain: context.grain(),
                horizon: context.horizon(),
                support: context.support(),
                warrant: context.warrant(),
            });
        }
        children.push(TaggedChildQuestion {
            parent: child.parent,
            question: child.question,
            value: question,
        });
    }
    positions.sort_unstable_by(|left, right| {
        (left.parent, left.port.as_str()).cmp(&(right.parent, right.port.as_str()))
    });

    let uncovered_parents = parent_members
        .difference(&materialized_members)
        .copied()
        .collect::<Vec<_>>();
    if uncovered_parents.is_empty() {
        Ok(FiniteSupportedFamilyLift::Complete(
            CompleteFiniteSupportedFamilyLift {
                parent,
                coverage,
                children,
                positions,
            },
        ))
    } else {
        Ok(FiniteSupportedFamilyLift::Unknown(
            UnknownFiniteSupportedFamilyLift {
                parent,
                coverage,
                children,
                positions,
                uncovered_parents,
            },
        ))
    }
}

#[derive(Debug, Error)]
pub enum FiniteSupportedFamilyLiftError {
    #[error("a finite supported-family materialization must be nonempty")]
    EmptyMaterialization,
    #[error("the materialized family repeats parent member {0}")]
    DuplicateParent(CompletionCandidateRef),
    #[error("the materialized family names {0}, which is absent from the parent answer")]
    ForeignParent(CompletionCandidateRef),
    #[error("child question {0} is unavailable")]
    UnresolvedQuestion(QueryRef),
    #[error("child question identity is {calculated}, expected {reference}")]
    QuestionIdentityMismatch {
        reference: QueryRef,
        calculated: QueryRef,
    },
    #[error("child relation {0} is unavailable")]
    UnresolvedRelation(RelationRef),
    #[error(transparent)]
    QuestionEncoding(#[from] OpenQueryError),
    #[error(transparent)]
    QuestionCheck(#[from] OpenQueryCheckError),
}

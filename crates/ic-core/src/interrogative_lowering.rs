//! Conservative erasure of implementation-only interrogative annotations.
//!
//! Root and route labels help a renderer or compiler explain how it reached an ordinary
//! question. They are not semantic constructors. This module therefore accepts only labels,
//! a controlled prompt, an ordinary source configuration, and a finite protected branch set;
//! lowering succeeds only when a fresh check reproduces an independently sealed ordinary
//! signature. The returned value contains no annotation data.

use std::collections::BTreeSet;

use thiserror::Error;

use crate::{
    AskOccurrence, AskOccurrenceCheckError, BindingVersionRef, ControlledBackwardKind,
    ControlledCoverage, ControlledElaborationExpectation, ControlledInterrogativePrompt,
    ControlledRenderingCatalog, ControlledRenderingError, DischargeMode, IProgArtifact,
    IProgCheckError, IProgError, IProgIR, IProgRef, NegationUseRef, OpenQuery,
    ProtectedQuestionBranch, ProvenanceRef, QueryRef, QuestionSuccessionCatalog, SourceConfig,
    SourceConfigCheckError, SourceConfigError, SourceConfigRef, SupportRef, TypeRef, WarrantRef,
    elaborate_controlled_interrogative,
};

/// Catalog boundary needed to recheck both the controlled contract and ordinary source graph.
pub trait InterrogativeLoweringCatalog:
    ControlledRenderingCatalog + QuestionSuccessionCatalog
{
}

impl<T> InterrogativeLoweringCatalog for T where
    T: ControlledRenderingCatalog + QuestionSuccessionCatalog
{
}

/// Finite, implementation-only presentation labels.
///
/// These strings are neither canonical artifacts nor members of the returned lowering.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DerivedInterrogativeAnnotation {
    root_label: String,
    route_path: Vec<String>,
}

impl DerivedInterrogativeAnnotation {
    pub fn new(
        root_label: impl Into<String>,
        route_path: Vec<String>,
    ) -> Result<Self, InterrogativeLoweringError> {
        let root_label = checked_label(root_label.into())?;
        if route_path.is_empty() {
            return Err(InterrogativeLoweringError::EmptyAnnotationRoute);
        }
        let route_path = route_path
            .into_iter()
            .map(checked_label)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            root_label,
            route_path,
        })
    }

    #[must_use]
    pub fn root_label(&self) -> &str {
        &self.root_label
    }

    #[must_use]
    pub fn route_path(&self) -> &[String] {
        &self.route_path
    }
}

fn checked_label(label: String) -> Result<String, InterrogativeLoweringError> {
    if label.trim().is_empty() || label.chars().any(char::is_control) {
        return Err(InterrogativeLoweringError::InvalidAnnotationLabel);
    }
    Ok(label)
}

/// One annotated presentation of an otherwise ordinary interrogative route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnnotatedInterrogativeRoute {
    annotation: DerivedInterrogativeAnnotation,
    prompt: ControlledInterrogativePrompt,
    source: SourceConfigRef,
    branches: Vec<ProtectedQuestionBranch>,
}

impl AnnotatedInterrogativeRoute {
    pub fn new(
        annotation: DerivedInterrogativeAnnotation,
        prompt: ControlledInterrogativePrompt,
        source: SourceConfigRef,
        branches: Vec<ProtectedQuestionBranch>,
    ) -> Result<Self, InterrogativeLoweringError> {
        Ok(Self {
            annotation,
            prompt,
            source,
            branches: checked_branches(branches)?,
        })
    }

    #[must_use]
    pub const fn annotation(&self) -> &DerivedInterrogativeAnnotation {
        &self.annotation
    }

    #[must_use]
    pub const fn prompt(&self) -> &ControlledInterrogativePrompt {
        &self.prompt
    }

    #[must_use]
    pub const fn source(&self) -> SourceConfigRef {
        self.source
    }

    #[must_use]
    pub fn branches(&self) -> &[ProtectedQuestionBranch] {
        &self.branches
    }
}

fn checked_branches(
    mut branches: Vec<ProtectedQuestionBranch>,
) -> Result<Vec<ProtectedQuestionBranch>, InterrogativeLoweringError> {
    if branches.is_empty() {
        return Err(InterrogativeLoweringError::EmptyProtectedBranchSet);
    }
    branches.sort_unstable();
    let mut answers = BTreeSet::new();
    for branch in &branches {
        if !answers.insert(branch.answer_class()) {
            return Err(InterrogativeLoweringError::DuplicateProtectedAnswerClass(
                branch.answer_class(),
            ));
        }
    }
    Ok(branches)
}

/// Independently checkable protected signature of one ordinary `Ask -> Return` route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtectedInterrogativeLoweringSignature {
    renderer_version: crate::ArtifactRef,
    binding: BindingVersionRef,
    kind: ControlledBackwardKind,
    coverage: ControlledCoverage,
    reciprocal_use: Option<NegationUseRef>,
    query_ref: QueryRef,
    query: OpenQuery,
    answer_types: Vec<TypeRef>,
    open_modes: Vec<DischargeMode>,
    context_mode: DischargeMode,
    support: SupportRef,
    warrant: Option<WarrantRef>,
    source_ref: SourceConfigRef,
    source: SourceConfig,
    root_ref: IProgRef,
    root: IProgArtifact,
    continuation_ref: IProgRef,
    continuation: IProgArtifact,
    occurrence: AskOccurrence,
    provenance: ProvenanceRef,
    branches: Vec<ProtectedQuestionBranch>,
}

impl ProtectedInterrogativeLoweringSignature {
    #[must_use]
    pub const fn query_ref(&self) -> QueryRef {
        self.query_ref
    }

    #[must_use]
    pub const fn query(&self) -> &OpenQuery {
        &self.query
    }

    #[must_use]
    pub fn answer_types(&self) -> &[TypeRef] {
        &self.answer_types
    }

    #[must_use]
    pub fn open_modes(&self) -> &[DischargeMode] {
        &self.open_modes
    }

    #[must_use]
    pub const fn context_mode(&self) -> DischargeMode {
        self.context_mode
    }

    #[must_use]
    pub const fn support(&self) -> SupportRef {
        self.support
    }

    #[must_use]
    pub const fn warrant(&self) -> Option<WarrantRef> {
        self.warrant
    }

    #[must_use]
    pub const fn source_ref(&self) -> SourceConfigRef {
        self.source_ref
    }

    #[must_use]
    pub const fn root_ref(&self) -> IProgRef {
        self.root_ref
    }

    #[must_use]
    pub const fn continuation_ref(&self) -> IProgRef {
        self.continuation_ref
    }

    #[must_use]
    pub const fn occurrence(&self) -> &AskOccurrence {
        &self.occurrence
    }

    #[must_use]
    pub const fn provenance(&self) -> ProvenanceRef {
        self.provenance
    }

    #[must_use]
    pub fn branches(&self) -> &[ProtectedQuestionBranch] {
        &self.branches
    }

    #[must_use]
    pub const fn kind(&self) -> ControlledBackwardKind {
        self.kind
    }

    #[must_use]
    pub const fn coverage(&self) -> ControlledCoverage {
        self.coverage
    }

    #[must_use]
    pub const fn reciprocal_use(&self) -> Option<NegationUseRef> {
        self.reciprocal_use
    }
}

/// Checks the unannotated predecessor against the accepted ordinary substrate.
pub fn check_unannotated_interrogative_route<C: InterrogativeLoweringCatalog>(
    prompt: &ControlledInterrogativePrompt,
    expected: ControlledElaborationExpectation,
    source_ref: SourceConfigRef,
    branches: Vec<ProtectedQuestionBranch>,
    catalog: &C,
) -> Result<ProtectedInterrogativeLoweringSignature, InterrogativeLoweringError> {
    derive_signature(prompt, expected, source_ref, branches, catalog)
}

/// Erases annotation labels and accepts the result only when every protected field equals the
/// independently checked unannotated predecessor.
pub fn lower_annotated_interrogative_route<C: InterrogativeLoweringCatalog>(
    annotated: &AnnotatedInterrogativeRoute,
    expected_contract: ControlledElaborationExpectation,
    predecessor: &ProtectedInterrogativeLoweringSignature,
    catalog: &C,
) -> Result<ProtectedInterrogativeLoweringSignature, InterrogativeLoweringError> {
    let actual = derive_signature(
        annotated.prompt(),
        expected_contract,
        annotated.source(),
        annotated.branches().to_vec(),
        catalog,
    )?;
    compare_protected(predecessor, &actual)?;
    Ok(actual)
}

fn derive_signature<C: InterrogativeLoweringCatalog>(
    prompt: &ControlledInterrogativePrompt,
    expected: ControlledElaborationExpectation,
    source_ref: SourceConfigRef,
    branches: Vec<ProtectedQuestionBranch>,
    catalog: &C,
) -> Result<ProtectedInterrogativeLoweringSignature, InterrogativeLoweringError> {
    let branches = checked_branches(branches)?;
    let elaborated = elaborate_controlled_interrogative(prompt, expected, catalog)?;
    let source = catalog
        .resolve_source_config(source_ref)
        .ok_or(InterrogativeLoweringError::UnresolvedSource(source_ref))?;
    let calculated = source.source_config_ref()?;
    if calculated != source_ref {
        return Err(InterrogativeLoweringError::SourceIdentityMismatch {
            expected: source_ref,
            actual: calculated,
        });
    }
    source
        .check(catalog)
        .map_err(|error| InterrogativeLoweringError::SourceCheck(Box::new(error)))?;
    let occurrences = source
        .ask_occurrences(catalog)
        .map_err(|error| InterrogativeLoweringError::OccurrenceCheck(Box::new(error)))?;
    if occurrences.len() != 1 {
        return Err(InterrogativeLoweringError::ExpectedOneAsk(
            occurrences.len(),
        ));
    }
    let occurrence = occurrences.into_iter().next().expect("length checked");
    occurrence
        .check(catalog)
        .map_err(|error| InterrogativeLoweringError::OccurrenceCheck(Box::new(error)))?;

    let root_ref = source.program();
    let root = catalog
        .resolve_iprog(root_ref)
        .ok_or(InterrogativeLoweringError::UnresolvedProgram(root_ref))?;
    let calculated = root.iprog_ref()?;
    if calculated != root_ref {
        return Err(InterrogativeLoweringError::ProgramIdentityMismatch {
            expected: root_ref,
            actual: calculated,
        });
    }
    root.check(catalog)
        .map_err(|error| InterrogativeLoweringError::ProgramCheck(Box::new(error)))?;
    let IProgIR::Ask {
        question,
        continuation,
        ..
    } = root.program()
    else {
        return Err(InterrogativeLoweringError::RootIsNotAsk);
    };
    if *question != elaborated.query_ref() {
        return Err(InterrogativeLoweringError::ProgramQueryMismatch {
            prompt: elaborated.query_ref(),
            program: *question,
        });
    }
    let continuation_ref = *continuation;
    let continuation = catalog.resolve_iprog(continuation_ref).ok_or(
        InterrogativeLoweringError::UnresolvedProgram(continuation_ref),
    )?;
    let calculated = continuation.iprog_ref()?;
    if calculated != continuation_ref {
        return Err(InterrogativeLoweringError::ProgramIdentityMismatch {
            expected: continuation_ref,
            actual: calculated,
        });
    }
    continuation
        .check(catalog)
        .map_err(|error| InterrogativeLoweringError::ProgramCheck(Box::new(error)))?;
    if !matches!(continuation.program(), IProgIR::Return { .. }) {
        return Err(InterrogativeLoweringError::ContinuationIsNotReturn);
    }

    let query = elaborated.query().clone();
    let schema = crate::RelationCatalog::resolve_relation_schema(catalog, query.relation()).ok_or(
        InterrogativeLoweringError::UnresolvedRelation(query.relation()),
    )?;
    let answer_types = query
        .open_ports()
        .iter()
        .map(|open| {
            schema
                .ports()
                .iter()
                .find(|port| port.name() == open.port())
                .map(crate::RelationPort::ty)
                .ok_or(InterrogativeLoweringError::OpenPortMissingFromSchema)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let open_modes = query
        .open_ports()
        .iter()
        .map(crate::OpenPort::mode)
        .collect();
    let context_mode = query.context().mode();
    let support = query.context().support();
    let warrant = query.context().warrant();
    let provenance = occurrence.provenance();

    Ok(ProtectedInterrogativeLoweringSignature {
        renderer_version: elaborated.renderer_version(),
        binding: elaborated.binding(),
        kind: elaborated.kind(),
        coverage: elaborated.coverage(),
        reciprocal_use: elaborated.reciprocal_use(),
        query_ref: elaborated.query_ref(),
        query,
        answer_types,
        open_modes,
        context_mode,
        support,
        warrant,
        source_ref,
        source,
        root_ref,
        root,
        continuation_ref,
        continuation,
        occurrence,
        provenance,
        branches,
    })
}

fn compare_protected(
    expected: &ProtectedInterrogativeLoweringSignature,
    actual: &ProtectedInterrogativeLoweringSignature,
) -> Result<(), InterrogativeLoweringError> {
    if expected.kind != actual.kind {
        return Err(InterrogativeLoweringError::ContractKindChanged);
    }
    if expected.coverage != actual.coverage {
        return Err(InterrogativeLoweringError::CoverageChanged);
    }
    if expected.reciprocal_use != actual.reciprocal_use {
        return Err(InterrogativeLoweringError::ReciprocalUseChanged);
    }
    if expected.open_modes != actual.open_modes || expected.context_mode != actual.context_mode {
        return Err(InterrogativeLoweringError::DischargeModeChanged);
    }
    if expected.query_ref != actual.query_ref || expected.query != actual.query {
        return Err(InterrogativeLoweringError::QueryChanged);
    }
    if expected.answer_types != actual.answer_types {
        return Err(InterrogativeLoweringError::AnswerCarrierChanged);
    }
    if expected.continuation_ref != actual.continuation_ref
        || expected.continuation != actual.continuation
    {
        return Err(InterrogativeLoweringError::ContinuationChanged);
    }
    if expected.branches != actual.branches {
        return Err(InterrogativeLoweringError::ProtectedBranchesChanged);
    }
    if expected.support != actual.support || expected.warrant != actual.warrant {
        return Err(InterrogativeLoweringError::AuthorityRouteChanged);
    }
    if expected.provenance != actual.provenance {
        return Err(InterrogativeLoweringError::ProvenanceChanged);
    }
    if expected.occurrence != actual.occurrence {
        return Err(InterrogativeLoweringError::OccurrenceChanged);
    }
    if expected.binding != actual.binding {
        return Err(InterrogativeLoweringError::BindingChanged);
    }
    if expected.renderer_version != actual.renderer_version {
        return Err(InterrogativeLoweringError::RendererVersionChanged);
    }
    if expected.source_ref != actual.source_ref || expected.source != actual.source {
        return Err(InterrogativeLoweringError::SourceChanged);
    }
    if expected.root_ref != actual.root_ref || expected.root != actual.root {
        return Err(InterrogativeLoweringError::RootProgramChanged);
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum InterrogativeLoweringError {
    #[error(transparent)]
    Controlled(#[from] ControlledRenderingError),
    #[error(transparent)]
    SourceEncoding(#[from] SourceConfigError),
    #[error(transparent)]
    ProgramEncoding(#[from] IProgError),
    #[error("annotation labels must be nonempty and contain no control characters")]
    InvalidAnnotationLabel,
    #[error("annotation route must contain at least one label")]
    EmptyAnnotationRoute,
    #[error("protected answer-branch set must be nonempty")]
    EmptyProtectedBranchSet,
    #[error("protected answer class {0} occurs more than once")]
    DuplicateProtectedAnswerClass(crate::ArtifactRef),
    #[error("source configuration {0} is unavailable")]
    UnresolvedSource(SourceConfigRef),
    #[error("source identity is {actual}, expected {expected}")]
    SourceIdentityMismatch {
        expected: SourceConfigRef,
        actual: SourceConfigRef,
    },
    #[error("source configuration failed recheck: {0}")]
    SourceCheck(Box<SourceConfigCheckError>),
    #[error("Ask occurrence failed recheck: {0}")]
    OccurrenceCheck(Box<AskOccurrenceCheckError>),
    #[error("source route contains {0} Ask nodes; exactly one is required")]
    ExpectedOneAsk(usize),
    #[error("program {0} is unavailable")]
    UnresolvedProgram(IProgRef),
    #[error("program identity is {actual}, expected {expected}")]
    ProgramIdentityMismatch {
        expected: IProgRef,
        actual: IProgRef,
    },
    #[error("program failed recheck: {0}")]
    ProgramCheck(Box<IProgCheckError>),
    #[error("annotated source root is not Ask")]
    RootIsNotAsk,
    #[error("program asks {program}, but controlled prompt elaborates {prompt}")]
    ProgramQueryMismatch { prompt: QueryRef, program: QueryRef },
    #[error("annotated source continuation is not Return")]
    ContinuationIsNotReturn,
    #[error("relation {0} is unavailable")]
    UnresolvedRelation(crate::RelationRef),
    #[error("checked open query port is absent from its relation schema")]
    OpenPortMissingFromSchema,
    #[error("controlled contract kind changed during annotation erasure")]
    ContractKindChanged,
    #[error("coverage changed during annotation erasure")]
    CoverageChanged,
    #[error("reciprocal-use identity changed during annotation erasure")]
    ReciprocalUseChanged,
    #[error("discharge mode or actuality obligation changed during annotation erasure")]
    DischargeModeChanged,
    #[error("normalized query changed during annotation erasure")]
    QueryChanged,
    #[error("answer carrier changed during annotation erasure")]
    AnswerCarrierChanged,
    #[error("occurrence-indexed continuation changed during annotation erasure")]
    ContinuationChanged,
    #[error("protected whole-answer branches changed during annotation erasure")]
    ProtectedBranchesChanged,
    #[error("support or warrant route changed during annotation erasure")]
    AuthorityRouteChanged,
    #[error("occurrence provenance changed during annotation erasure")]
    ProvenanceChanged,
    #[error("Ask occurrence changed during annotation erasure")]
    OccurrenceChanged,
    #[error("binding changed during annotation erasure")]
    BindingChanged,
    #[error("renderer version changed during annotation erasure")]
    RendererVersionChanged,
    #[error("source configuration changed during annotation erasure")]
    SourceChanged,
    #[error("root first-order program changed during annotation erasure")]
    RootProgramChanged,
}

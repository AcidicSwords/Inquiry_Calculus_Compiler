//! Finite relational resolution and the five-way question-indexed outcome gate.
//!
//! This module is a derived executable specialization of ordinary [`ResolutionPath`] data.  A
//! caller supplies finite leaf tables and explicit exact/partial coverage; composition preserves
//! every related output.  The classifier then keeps exact emptiness, undefinedness, support
//! failure, incomplete coverage, and a proof-carrying supported answer disjoint.  It creates no
//! canonical outcome artifact, relation engine, scheduler, opcode, or persistence surface.

use std::{collections::BTreeSet, fmt};

use thiserror::Error;

use crate::{
    ActualEventCheckError, ActualEventError, AdmittedFiniteAnswerSet, ArtifactRef, CoverageRef,
    DecodedCandidateSet, DecodedObservationUse, EventRef, FiniteSupportedAnswerCatalog,
    FiniteSupportedAnswerError, OpenQueryCheckError, OpenQueryError, OperatorOccurrenceCatalog,
    ProbeOperatorError, QueryRef, RelationUseRef, ResolutionCatalog, ResolutionPathCheckError,
    ResolutionPathError, ResolutionPathIR, ResolutionPathRef, Standing, TypeRef, TypeSymbol,
    check_actual_event,
};

/// Whether one caller-declared finite leaf table is exhaustive at its declared domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FiniteResolutionCoverage {
    Exact(CoverageRef),
    Partial(CoverageRef),
}

impl FiniteResolutionCoverage {
    #[must_use]
    pub const fn reference(self) -> CoverageRef {
        match self {
            Self::Exact(reference) | Self::Partial(reference) => reference,
        }
    }

    #[must_use]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact(_))
    }
}

/// One finite leaf result. Empty `Related` output is a relation result, not undefinedness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FiniteResolutionLeafResult {
    Related(Vec<ArtifactRef>),
    Undefined { residual: ArtifactRef },
}

/// One input-indexed row in a finite leaf relation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteResolutionLeafEntry {
    input: ArtifactRef,
    result: FiniteResolutionLeafResult,
}

impl FiniteResolutionLeafEntry {
    #[must_use]
    pub const fn related(input: ArtifactRef, outputs: Vec<ArtifactRef>) -> Self {
        Self {
            input,
            result: FiniteResolutionLeafResult::Related(outputs),
        }
    }

    #[must_use]
    pub const fn undefined(input: ArtifactRef, residual: ArtifactRef) -> Self {
        Self {
            input,
            result: FiniteResolutionLeafResult::Undefined { residual },
        }
    }

    #[must_use]
    pub const fn input(&self) -> ArtifactRef {
        self.input
    }

    #[must_use]
    pub const fn result(&self) -> &FiniteResolutionLeafResult {
        &self.result
    }
}

/// One finite materialization of a non-composite resolution edge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteResolutionLeafTable {
    path: ResolutionPathRef,
    domain: Vec<ArtifactRef>,
    entries: Vec<FiniteResolutionLeafEntry>,
    coverage: FiniteResolutionCoverage,
}

impl FiniteResolutionLeafTable {
    pub fn new(
        path: ResolutionPathRef,
        mut domain: Vec<ArtifactRef>,
        mut entries: Vec<FiniteResolutionLeafEntry>,
        coverage: FiniteResolutionCoverage,
    ) -> Result<Self, FiniteResolutionTableError> {
        if domain.is_empty() {
            return Err(FiniteResolutionTableError::EmptyDomain);
        }
        domain.sort_unstable();
        if let Some(pair) = domain.windows(2).find(|pair| pair[0] == pair[1]) {
            return Err(FiniteResolutionTableError::DuplicateDomainInput(pair[0]));
        }
        entries.sort_unstable_by_key(FiniteResolutionLeafEntry::input);
        if let Some(pair) = entries
            .windows(2)
            .find(|pair| pair[0].input == pair[1].input)
        {
            return Err(FiniteResolutionTableError::DuplicateEntryInput(
                pair[0].input,
            ));
        }
        for entry in &mut entries {
            if domain.binary_search(&entry.input).is_err() {
                return Err(FiniteResolutionTableError::EntryOutsideDomain(entry.input));
            }
            if let FiniteResolutionLeafResult::Related(outputs) = &mut entry.result {
                outputs.sort_unstable();
                if let Some(pair) = outputs.windows(2).find(|pair| pair[0] == pair[1]) {
                    return Err(FiniteResolutionTableError::DuplicateOutput {
                        input: entry.input,
                        output: pair[0],
                    });
                }
            }
        }
        Ok(Self {
            path,
            domain,
            entries,
            coverage,
        })
    }

    #[must_use]
    pub const fn path(&self) -> ResolutionPathRef {
        self.path
    }

    #[must_use]
    pub fn domain(&self) -> &[ArtifactRef] {
        &self.domain
    }

    #[must_use]
    pub fn entries(&self) -> &[FiniteResolutionLeafEntry] {
        &self.entries
    }

    #[must_use]
    pub const fn coverage(&self) -> FiniteResolutionCoverage {
        self.coverage
    }
}

/// A complete finite relational run. `outputs` may be empty, singleton, or non-singleton.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompleteFiniteResolutionRun {
    path: ResolutionPathRef,
    input: ArtifactRef,
    outputs: Vec<ArtifactRef>,
    coverage: Vec<CoverageRef>,
}

impl CompleteFiniteResolutionRun {
    #[must_use]
    pub const fn path(&self) -> ResolutionPathRef {
        self.path
    }

    #[must_use]
    pub const fn input(&self) -> ArtifactRef {
        self.input
    }

    #[must_use]
    pub fn outputs(&self) -> &[ArtifactRef] {
        &self.outputs
    }

    #[must_use]
    pub fn coverage(&self) -> &[CoverageRef] {
        &self.coverage
    }
}

/// A typed undefined run, retaining every leaf residual that blocked the path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UndefinedFiniteResolutionRun {
    path: ResolutionPathRef,
    input: ArtifactRef,
    residuals: Vec<ArtifactRef>,
    coverage: Vec<CoverageRef>,
}

impl UndefinedFiniteResolutionRun {
    #[must_use]
    pub const fn path(&self) -> ResolutionPathRef {
        self.path
    }

    #[must_use]
    pub const fn input(&self) -> ArtifactRef {
        self.input
    }

    #[must_use]
    pub fn residuals(&self) -> &[ArtifactRef] {
        &self.residuals
    }

    #[must_use]
    pub fn coverage(&self) -> &[CoverageRef] {
        &self.coverage
    }
}

/// A partial run. Known outputs never imply closure over the uncovered input region.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnknownFiniteResolutionRun {
    path: ResolutionPathRef,
    input: ArtifactRef,
    known_outputs: Vec<ArtifactRef>,
    uncovered_inputs: Vec<ArtifactRef>,
    undefined_residuals: Vec<ArtifactRef>,
    coverage: Vec<CoverageRef>,
}

impl UnknownFiniteResolutionRun {
    #[must_use]
    pub const fn path(&self) -> ResolutionPathRef {
        self.path
    }

    #[must_use]
    pub const fn input(&self) -> ArtifactRef {
        self.input
    }

    #[must_use]
    pub fn known_outputs(&self) -> &[ArtifactRef] {
        &self.known_outputs
    }

    #[must_use]
    pub fn uncovered_inputs(&self) -> &[ArtifactRef] {
        &self.uncovered_inputs
    }

    #[must_use]
    pub fn undefined_residuals(&self) -> &[ArtifactRef] {
        &self.undefined_residuals
    }

    #[must_use]
    pub fn coverage(&self) -> &[CoverageRef] {
        &self.coverage
    }
}

/// The three structural results of running one finite typed relation path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FiniteResolutionRun {
    Complete(CompleteFiniteResolutionRun),
    Undefined(UndefinedFiniteResolutionRun),
    Unknown(UnknownFiniteResolutionRun),
}

impl FiniteResolutionRun {
    #[must_use]
    pub const fn path(&self) -> ResolutionPathRef {
        match self {
            Self::Complete(run) => run.path,
            Self::Undefined(run) => run.path,
            Self::Unknown(run) => run.path,
        }
    }

    #[must_use]
    pub const fn input(&self) -> ArtifactRef {
        match self {
            Self::Complete(run) => run.input,
            Self::Undefined(run) => run.input,
            Self::Unknown(run) => run.input,
        }
    }
}

/// Runs a finite resolution path as a relation, preserving the full output set at composition.
pub fn run_finite_resolution<C: ResolutionCatalog>(
    path: ResolutionPathRef,
    input: ArtifactRef,
    tables: &[FiniteResolutionLeafTable],
    catalog: &C,
) -> Result<FiniteResolutionRun, FiniteResolutionRunError> {
    let mut visiting = BTreeSet::new();
    run_finite_resolution_inner(path, input, tables, catalog, &mut visiting)
}

fn run_finite_resolution_inner<C: ResolutionCatalog>(
    path_ref: ResolutionPathRef,
    input: ArtifactRef,
    tables: &[FiniteResolutionLeafTable],
    catalog: &C,
    visiting: &mut BTreeSet<ResolutionPathRef>,
) -> Result<FiniteResolutionRun, FiniteResolutionRunError> {
    if !visiting.insert(path_ref) {
        return Err(FiniteResolutionRunError::CyclicRun(path_ref));
    }
    let path = catalog
        .resolve_resolution_path(path_ref)
        .ok_or(FiniteResolutionRunError::UnresolvedPath(path_ref))?;
    let calculated = path.resolution_path_ref()?;
    if calculated != path_ref {
        return Err(FiniteResolutionRunError::PathIdentityMismatch {
            reference: path_ref,
            calculated,
        });
    }
    path.check(catalog)
        .map_err(|error| FiniteResolutionRunError::PathCheck(Box::new(error)))?;

    let result = match path.path() {
        ResolutionPathIR::Identity => FiniteResolutionRun::Complete(CompleteFiniteResolutionRun {
            path: path_ref,
            input,
            outputs: vec![input],
            coverage: Vec::new(),
        }),
        ResolutionPathIR::Compose { first, second } => {
            let first_run = run_finite_resolution_inner(first, input, tables, catalog, visiting)?;
            compose_finite_runs(
                path_ref, input, first_run, second, tables, catalog, visiting,
            )?
        }
        ResolutionPathIR::Decode { .. }
        | ResolutionPathIR::Relation { .. }
        | ResolutionPathIR::Program { .. } => run_finite_leaf(path_ref, input, tables)?,
    };
    visiting.remove(&path_ref);
    Ok(result)
}

fn run_finite_leaf(
    path: ResolutionPathRef,
    input: ArtifactRef,
    tables: &[FiniteResolutionLeafTable],
) -> Result<FiniteResolutionRun, FiniteResolutionRunError> {
    let mut matching = tables.iter().filter(|table| table.path == path);
    let table = matching
        .next()
        .ok_or(FiniteResolutionRunError::MissingLeafTable(path))?;
    if matching.next().is_some() {
        return Err(FiniteResolutionRunError::DuplicateLeafTable(path));
    }
    let coverage = vec![table.coverage.reference()];
    if table.domain.binary_search(&input).is_err() {
        return Ok(FiniteResolutionRun::Unknown(UnknownFiniteResolutionRun {
            path,
            input,
            known_outputs: Vec::new(),
            uncovered_inputs: vec![input],
            undefined_residuals: Vec::new(),
            coverage,
        }));
    }
    let entry = table
        .entries
        .binary_search_by_key(&input, FiniteResolutionLeafEntry::input)
        .ok()
        .map(|index| &table.entries[index]);
    match (entry.map(FiniteResolutionLeafEntry::result), table.coverage) {
        (Some(FiniteResolutionLeafResult::Undefined { residual }), _) => Ok(
            FiniteResolutionRun::Undefined(UndefinedFiniteResolutionRun {
                path,
                input,
                residuals: vec![*residual],
                coverage,
            }),
        ),
        (
            Some(FiniteResolutionLeafResult::Related(outputs)),
            FiniteResolutionCoverage::Exact(_),
        ) => Ok(FiniteResolutionRun::Complete(CompleteFiniteResolutionRun {
            path,
            input,
            outputs: outputs.clone(),
            coverage,
        })),
        (None, FiniteResolutionCoverage::Exact(_)) => {
            Ok(FiniteResolutionRun::Complete(CompleteFiniteResolutionRun {
                path,
                input,
                outputs: Vec::new(),
                coverage,
            }))
        }
        (
            Some(FiniteResolutionLeafResult::Related(outputs)),
            FiniteResolutionCoverage::Partial(_),
        ) => Ok(FiniteResolutionRun::Unknown(UnknownFiniteResolutionRun {
            path,
            input,
            known_outputs: outputs.clone(),
            uncovered_inputs: vec![input],
            undefined_residuals: Vec::new(),
            coverage,
        })),
        (None, FiniteResolutionCoverage::Partial(_)) => {
            Ok(FiniteResolutionRun::Unknown(UnknownFiniteResolutionRun {
                path,
                input,
                known_outputs: Vec::new(),
                uncovered_inputs: vec![input],
                undefined_residuals: Vec::new(),
                coverage,
            }))
        }
    }
}

fn compose_finite_runs<C: ResolutionCatalog>(
    path: ResolutionPathRef,
    input: ArtifactRef,
    first: FiniteResolutionRun,
    second: ResolutionPathRef,
    tables: &[FiniteResolutionLeafTable],
    catalog: &C,
    visiting: &mut BTreeSet<ResolutionPathRef>,
) -> Result<FiniteResolutionRun, FiniteResolutionRunError> {
    let FiniteResolutionRun::Complete(first) = first else {
        return Ok(reindex_run(path, input, first));
    };
    if first.outputs.is_empty() {
        return Ok(FiniteResolutionRun::Complete(CompleteFiniteResolutionRun {
            path,
            input,
            outputs: Vec::new(),
            coverage: first.coverage,
        }));
    }

    let mut outputs = BTreeSet::new();
    let mut uncovered = BTreeSet::new();
    let mut residuals = BTreeSet::new();
    let mut coverage: BTreeSet<_> = first.coverage.into_iter().collect();
    let mut saw_complete = false;
    let mut saw_undefined = false;
    let mut saw_unknown = false;
    for middle in first.outputs {
        match run_finite_resolution_inner(second, middle, tables, catalog, visiting)? {
            FiniteResolutionRun::Complete(run) => {
                saw_complete = true;
                outputs.extend(run.outputs);
                coverage.extend(run.coverage);
            }
            FiniteResolutionRun::Undefined(run) => {
                saw_undefined = true;
                residuals.extend(run.residuals);
                coverage.extend(run.coverage);
            }
            FiniteResolutionRun::Unknown(run) => {
                saw_unknown = true;
                outputs.extend(run.known_outputs);
                uncovered.extend(run.uncovered_inputs);
                residuals.extend(run.undefined_residuals);
                coverage.extend(run.coverage);
            }
        }
    }
    let outputs = outputs.into_iter().collect();
    let coverage = coverage.into_iter().collect();
    if saw_unknown || (saw_undefined && saw_complete) {
        return Ok(FiniteResolutionRun::Unknown(UnknownFiniteResolutionRun {
            path,
            input,
            known_outputs: outputs,
            uncovered_inputs: uncovered.into_iter().collect(),
            undefined_residuals: residuals.into_iter().collect(),
            coverage,
        }));
    }
    if saw_undefined {
        return Ok(FiniteResolutionRun::Undefined(
            UndefinedFiniteResolutionRun {
                path,
                input,
                residuals: residuals.into_iter().collect(),
                coverage,
            },
        ));
    }
    Ok(FiniteResolutionRun::Complete(CompleteFiniteResolutionRun {
        path,
        input,
        outputs,
        coverage,
    }))
}

fn reindex_run(
    path: ResolutionPathRef,
    input: ArtifactRef,
    run: FiniteResolutionRun,
) -> FiniteResolutionRun {
    match run {
        FiniteResolutionRun::Complete(mut run) => {
            run.path = path;
            run.input = input;
            FiniteResolutionRun::Complete(run)
        }
        FiniteResolutionRun::Undefined(mut run) => {
            run.path = path;
            run.input = input;
            FiniteResolutionRun::Undefined(run)
        }
        FiniteResolutionRun::Unknown(mut run) => {
            run.path = path;
            run.input = input;
            FiniteResolutionRun::Unknown(run)
        }
    }
}

/// Checked exhaustive emptiness at one question/event/path boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteEmptyCertificate {
    event: EventRef,
    query: QueryRef,
    run: CompleteFiniteResolutionRun,
}

/// Checked typed undefinedness at one question/event/path boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteResolutionResidual {
    event: EventRef,
    query: QueryRef,
    run: UndefinedFiniteResolutionRun,
}

/// Checked incomplete coverage at one question/event/path boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteCoverageResidual {
    event: EventRef,
    query: QueryRef,
    run: UnknownFiniteResolutionRun,
}

/// A supported resolution retains the whole proof-carrying answer and complete relational run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteSupportedResolution {
    event: EventRef,
    query: QueryRef,
    run: CompleteFiniteResolutionRun,
    answer: AdmittedFiniteAnswerSet,
}

/// Decoded candidates whose declared support route failed without becoming semantic negation.
#[derive(Debug)]
pub struct FiniteSupportResidual {
    event: EventRef,
    query: QueryRef,
    run: CompleteFiniteResolutionRun,
    decoded: DecodedCandidateSet,
    observations: Vec<RelationUseRef>,
    failure: FiniteSupportedAnswerError,
}

macro_rules! coordinate_getters {
    ($name:ty) => {
        impl $name {
            #[must_use]
            pub const fn event(&self) -> EventRef {
                self.event
            }

            #[must_use]
            pub const fn query(&self) -> QueryRef {
                self.query
            }

            #[must_use]
            pub const fn path(&self) -> ResolutionPathRef {
                self.run.path()
            }
        }
    };
}

coordinate_getters!(FiniteEmptyCertificate);
coordinate_getters!(FiniteResolutionResidual);
coordinate_getters!(FiniteCoverageResidual);
coordinate_getters!(FiniteSupportedResolution);
coordinate_getters!(FiniteSupportResidual);

impl FiniteEmptyCertificate {
    #[must_use]
    pub const fn run(&self) -> &CompleteFiniteResolutionRun {
        &self.run
    }
}

impl FiniteResolutionResidual {
    #[must_use]
    pub const fn run(&self) -> &UndefinedFiniteResolutionRun {
        &self.run
    }
}

impl FiniteCoverageResidual {
    #[must_use]
    pub const fn run(&self) -> &UnknownFiniteResolutionRun {
        &self.run
    }
}

impl FiniteSupportedResolution {
    #[must_use]
    pub const fn run(&self) -> &CompleteFiniteResolutionRun {
        &self.run
    }

    #[must_use]
    pub const fn answer(&self) -> &AdmittedFiniteAnswerSet {
        &self.answer
    }
}

impl FiniteSupportResidual {
    #[must_use]
    pub const fn run(&self) -> &CompleteFiniteResolutionRun {
        &self.run
    }

    #[must_use]
    pub const fn decoded(&self) -> &DecodedCandidateSet {
        &self.decoded
    }

    #[must_use]
    pub fn observations(&self) -> &[RelationUseRef] {
        &self.observations
    }

    #[must_use]
    pub const fn failure(&self) -> &FiniteSupportedAnswerError {
        &self.failure
    }
}

/// The five disjoint finite question-resolution outcomes.
#[derive(Debug)]
pub enum FiniteResolutionOutcome {
    Supported(FiniteSupportedResolution),
    ExactEmpty(FiniteEmptyCertificate),
    Undefined(FiniteResolutionResidual),
    Unsupported(FiniteSupportResidual),
    Unknown(FiniteCoverageResidual),
}

impl FiniteResolutionOutcome {
    #[must_use]
    pub const fn kind(&self) -> FiniteResolutionOutcomeKind {
        match self {
            Self::Supported(_) => FiniteResolutionOutcomeKind::Supported,
            Self::ExactEmpty(_) => FiniteResolutionOutcomeKind::ExactEmpty,
            Self::Undefined(_) => FiniteResolutionOutcomeKind::Undefined,
            Self::Unsupported(_) => FiniteResolutionOutcomeKind::Unsupported,
            Self::Unknown(_) => FiniteResolutionOutcomeKind::Unknown,
        }
    }

    pub fn into_supported(self) -> Result<FiniteSupportedResolution, Box<Self>> {
        match self {
            Self::Supported(supported) => Ok(supported),
            other => Err(Box::new(other)),
        }
    }
}

impl fmt::Display for FiniteResolutionOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind().fmt(formatter)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FiniteResolutionOutcomeKind {
    Supported,
    ExactEmpty,
    Undefined,
    Unsupported,
    Unknown,
}

impl fmt::Display for FiniteResolutionOutcomeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

use crate::decoder::AnswerPortScope;

/// Classifies one checked finite run. Only a complete nonempty run attempts support admission.
///
/// This is the single-open-port specialization; a question with several open ports is classified
/// one port at a time through `classify_finite_port_resolution`.
pub fn classify_finite_question_resolution<C: FiniteSupportedAnswerCatalog + ResolutionCatalog>(
    event_ref: EventRef,
    query_ref: QueryRef,
    run: FiniteResolutionRun,
    decoded: Option<DecodedCandidateSet>,
    observations: Vec<DecodedObservationUse>,
    standing: &Standing,
    catalog: &C,
) -> Result<FiniteResolutionOutcome, FiniteResolutionGateError> {
    classify_finite_resolution(
        AnswerPortScope::SoleOpenPort,
        event_ref,
        query_ref,
        run,
        decoded,
        observations,
        standing,
        catalog,
    )
}

/// Classifies one checked finite run against one named open port of its question.
///
/// The run must land in that exact port's schema carrier, so a run typed for a sibling port is
/// rejected rather than accepted by arity coincidence. Every other check, including decoded
/// candidate agreement and support admission, is identical to the single-port entry point: the
/// decoded completions still range over the whole port field and no candidate is selected.
#[allow(clippy::too_many_arguments)]
pub fn classify_finite_port_resolution<C: FiniteSupportedAnswerCatalog + ResolutionCatalog>(
    port: &TypeSymbol,
    event_ref: EventRef,
    query_ref: QueryRef,
    run: FiniteResolutionRun,
    decoded: Option<DecodedCandidateSet>,
    observations: Vec<DecodedObservationUse>,
    standing: &Standing,
    catalog: &C,
) -> Result<FiniteResolutionOutcome, FiniteResolutionGateError> {
    classify_finite_resolution(
        AnswerPortScope::NamedPort(port),
        event_ref,
        query_ref,
        run,
        decoded,
        observations,
        standing,
        catalog,
    )
}

#[allow(clippy::too_many_arguments)]
fn classify_finite_resolution<C: FiniteSupportedAnswerCatalog + ResolutionCatalog>(
    scope: AnswerPortScope<'_>,
    event_ref: EventRef,
    query_ref: QueryRef,
    run: FiniteResolutionRun,
    decoded: Option<DecodedCandidateSet>,
    observations: Vec<DecodedObservationUse>,
    standing: &Standing,
    catalog: &C,
) -> Result<FiniteResolutionOutcome, FiniteResolutionGateError> {
    let event = OperatorOccurrenceCatalog::resolve_actual_event(catalog, event_ref)
        .ok_or(FiniteResolutionGateError::UnresolvedEvent(event_ref))?;
    let calculated = event.event_ref()?;
    if calculated != event_ref {
        return Err(FiniteResolutionGateError::EventIdentityMismatch {
            reference: event_ref,
            calculated,
        });
    }
    check_actual_event(&event, catalog)?;
    if event.question() != query_ref {
        return Err(FiniteResolutionGateError::EventQuestionMismatch {
            event: event.question(),
            supplied: query_ref,
        });
    }
    if run.input() != event.raw_return().as_artifact_ref() {
        return Err(FiniteResolutionGateError::RunInputMismatch {
            event: event.raw_return().as_artifact_ref(),
            run: run.input(),
        });
    }
    check_resolution_types(scope, run.path(), query_ref, event.operator(), catalog)?;

    match run {
        FiniteResolutionRun::Complete(run) if run.outputs.is_empty() => {
            ensure_no_support_evidence(decoded, &observations)?;
            Ok(FiniteResolutionOutcome::ExactEmpty(
                FiniteEmptyCertificate {
                    event: event_ref,
                    query: query_ref,
                    run,
                },
            ))
        }
        FiniteResolutionRun::Undefined(run) => {
            ensure_no_support_evidence(decoded, &observations)?;
            Ok(FiniteResolutionOutcome::Undefined(
                FiniteResolutionResidual {
                    event: event_ref,
                    query: query_ref,
                    run,
                },
            ))
        }
        FiniteResolutionRun::Unknown(run) => {
            ensure_no_support_evidence(decoded, &observations)?;
            Ok(FiniteResolutionOutcome::Unknown(FiniteCoverageResidual {
                event: event_ref,
                query: query_ref,
                run,
            }))
        }
        FiniteResolutionRun::Complete(run) => {
            let decoded = decoded.ok_or(FiniteResolutionGateError::MissingDecodedEvidence)?;
            if decoded.event() != event_ref || decoded.query() != query_ref {
                return Err(FiniteResolutionGateError::DecodedContextMismatch);
            }
            let decoded_path = catalog
                .resolve_resolution_path(decoded.path())
                .ok_or(FiniteResolutionGateError::UnresolvedPath(decoded.path()))?;
            let calculated = decoded_path.resolution_path_ref()?;
            if calculated != decoded.path() {
                return Err(FiniteResolutionGateError::PathIdentityMismatch {
                    reference: decoded.path(),
                    calculated,
                });
            }
            decoded_path
                .check(catalog)
                .map_err(|error| FiniteResolutionGateError::PathCheck(Box::new(error)))?;
            let program_path = catalog
                .resolve_resolution_path(run.path)
                .ok_or(FiniteResolutionGateError::UnresolvedPath(run.path))?;
            if decoded_path.input() != program_path.input()
                || decoded_path.output() != program_path.output()
            {
                return Err(FiniteResolutionGateError::DecodedPathTypeMismatch);
            }
            let expected: Vec<_> = decoded
                .candidates()
                .iter()
                .map(|candidate| candidate.as_artifact_ref())
                .collect();
            if run.outputs != expected {
                return Err(FiniteResolutionGateError::CandidateOutputMismatch {
                    run: run.outputs,
                    decoded: expected,
                });
            }
            let observation_refs = observations
                .iter()
                .map(DecodedObservationUse::observation)
                .collect();
            match crate::supported_answer::admit_finite_supported_answers_scoped(
                scope,
                decoded.clone(),
                observations,
                standing,
                catalog,
            ) {
                Ok(answer) => Ok(FiniteResolutionOutcome::Supported(
                    FiniteSupportedResolution {
                        event: event_ref,
                        query: query_ref,
                        run,
                        answer,
                    },
                )),
                Err(failure) if is_support_failure(&failure) => Ok(
                    FiniteResolutionOutcome::Unsupported(FiniteSupportResidual {
                        event: event_ref,
                        query: query_ref,
                        run,
                        decoded,
                        observations: observation_refs,
                        failure,
                    }),
                ),
                Err(failure) => Err(FiniteResolutionGateError::SupportInfrastructure(Box::new(
                    failure,
                ))),
            }
        }
    }
}

fn ensure_no_support_evidence(
    decoded: Option<DecodedCandidateSet>,
    observations: &[DecodedObservationUse],
) -> Result<(), FiniteResolutionGateError> {
    if decoded.is_some() || !observations.is_empty() {
        Err(FiniteResolutionGateError::UnexpectedSupportEvidence)
    } else {
        Ok(())
    }
}

fn is_support_failure(error: &FiniteSupportedAnswerError) -> bool {
    matches!(
        error,
        FiniteSupportedAnswerError::RelationSupport(_)
            | FiniteSupportedAnswerError::ObservationIsNotProbe(_)
            | FiniteSupportedAnswerError::CandidateCoverageMismatch
            | FiniteSupportedAnswerError::SupportMissingReturn { .. }
    )
}

fn check_resolution_types<C: FiniteSupportedAnswerCatalog + ResolutionCatalog>(
    scope: AnswerPortScope<'_>,
    path_ref: ResolutionPathRef,
    query_ref: QueryRef,
    operator_ref: crate::ProbeOperatorRef,
    catalog: &C,
) -> Result<(), FiniteResolutionGateError> {
    let path = catalog
        .resolve_resolution_path(path_ref)
        .ok_or(FiniteResolutionGateError::UnresolvedPath(path_ref))?;
    let calculated = path.resolution_path_ref()?;
    if calculated != path_ref {
        return Err(FiniteResolutionGateError::PathIdentityMismatch {
            reference: path_ref,
            calculated,
        });
    }
    path.check(catalog)
        .map_err(|error| FiniteResolutionGateError::PathCheck(Box::new(error)))?;
    let operator = crate::ActualEventCatalog::resolve_probe_operator(catalog, operator_ref)
        .ok_or(FiniteResolutionGateError::UnresolvedOperator(operator_ref))?;
    let query = crate::OpenQueryCatalog::resolve_open_query(catalog, query_ref)
        .ok_or(FiniteResolutionGateError::UnresolvedQuery(query_ref))?;
    let calculated_query = query.query_ref()?;
    if calculated_query != query_ref {
        return Err(FiniteResolutionGateError::QueryIdentityMismatch {
            reference: query_ref,
            calculated: calculated_query,
        });
    }
    query.check(catalog)?;
    let answer_port = match scope {
        AnswerPortScope::SoleOpenPort => {
            if query.open_ports().len() != 1 {
                return Err(FiniteResolutionGateError::UnsupportedAnswerArity(
                    query.open_ports().len(),
                ));
            }
            query.open_ports()[0].port()
        }
        AnswerPortScope::NamedPort(port) => query
            .open_ports()
            .iter()
            .find(|open| open.port() == port)
            .ok_or_else(|| FiniteResolutionGateError::ForeignAnswerPort(port.clone()))?
            .port(),
    };
    let schema = crate::RelationCatalog::resolve_relation_schema(catalog, query.relation()).ok_or(
        FiniteResolutionGateError::UnresolvedRelation(query.relation()),
    )?;
    let answer_type = schema
        .ports()
        .iter()
        .find(|port| port.name() == answer_port)
        .expect("a checked query contains only schema ports")
        .ty();
    if path.input() != operator.return_type() || path.output() != answer_type {
        return Err(FiniteResolutionGateError::ResolutionTypeMismatch(Box::new(
            FiniteResolutionTypeMismatch {
                expected_input: operator.return_type(),
                actual_input: path.input(),
                expected_output: answer_type,
                actual_output: path.output(),
            },
        )));
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum FiniteResolutionTableError {
    #[error("a finite resolution table domain must be nonempty")]
    EmptyDomain,
    #[error("finite resolution table repeats domain input {0}")]
    DuplicateDomainInput(ArtifactRef),
    #[error("finite resolution table repeats entry input {0}")]
    DuplicateEntryInput(ArtifactRef),
    #[error("finite resolution table entry {0} is outside its declared domain")]
    EntryOutsideDomain(ArtifactRef),
    #[error("finite resolution table repeats output {output} for input {input}")]
    DuplicateOutput {
        input: ArtifactRef,
        output: ArtifactRef,
    },
}

#[derive(Debug, Error)]
pub enum FiniteResolutionRunError {
    #[error(transparent)]
    PathEncoding(#[from] ResolutionPathError),
    #[error("resolution path failed checking: {0}")]
    PathCheck(Box<ResolutionPathCheckError>),
    #[error("resolution path {0} is unavailable")]
    UnresolvedPath(ResolutionPathRef),
    #[error("resolution path {reference} hashes to {calculated}")]
    PathIdentityMismatch {
        reference: ResolutionPathRef,
        calculated: ResolutionPathRef,
    },
    #[error("resolution path {0} recurs while running")]
    CyclicRun(ResolutionPathRef),
    #[error("leaf resolution path {0} has no finite table")]
    MissingLeafTable(ResolutionPathRef),
    #[error("leaf resolution path {0} has more than one finite table")]
    DuplicateLeafTable(ResolutionPathRef),
}

#[derive(Debug, Error)]
pub enum FiniteResolutionGateError {
    #[error(transparent)]
    EventEncoding(#[from] ActualEventError),
    #[error(transparent)]
    EventCheck(Box<ActualEventCheckError>),
    #[error(transparent)]
    PathEncoding(#[from] ResolutionPathError),
    #[error("resolution path failed checking: {0}")]
    PathCheck(Box<ResolutionPathCheckError>),
    #[error(transparent)]
    QueryEncoding(#[from] OpenQueryError),
    #[error(transparent)]
    QueryCheck(#[from] OpenQueryCheckError),
    #[error(transparent)]
    OperatorEncoding(#[from] ProbeOperatorError),
    #[error("actual event {0} is unavailable")]
    UnresolvedEvent(EventRef),
    #[error("actual event {reference} hashes to {calculated}")]
    EventIdentityMismatch {
        reference: EventRef,
        calculated: EventRef,
    },
    #[error("actual event question {event} does not match supplied question {supplied}")]
    EventQuestionMismatch { event: QueryRef, supplied: QueryRef },
    #[error("finite run input {run} does not match event raw return {event}")]
    RunInputMismatch {
        event: ArtifactRef,
        run: ArtifactRef,
    },
    #[error("resolution path {0} is unavailable")]
    UnresolvedPath(ResolutionPathRef),
    #[error("resolution path {reference} hashes to {calculated}")]
    PathIdentityMismatch {
        reference: ResolutionPathRef,
        calculated: ResolutionPathRef,
    },
    #[error("probe operator {0} is unavailable")]
    UnresolvedOperator(crate::ProbeOperatorRef),
    #[error("question {0} is unavailable")]
    UnresolvedQuery(QueryRef),
    #[error("question {reference} hashes to {calculated}")]
    QueryIdentityMismatch {
        reference: QueryRef,
        calculated: QueryRef,
    },
    #[error("question relation {0} is unavailable")]
    UnresolvedRelation(crate::RelationRef),
    #[error("finite resolution gate currently requires one open answer port, got {0}")]
    UnsupportedAnswerArity(usize),
    #[error("answer port {0} is not an open port of this question")]
    ForeignAnswerPort(TypeSymbol),
    #[error("resolution path type mismatch: {0}")]
    ResolutionTypeMismatch(Box<FiniteResolutionTypeMismatch>),
    #[error("non-supported finite run carried decoded/support evidence")]
    UnexpectedSupportEvidence,
    #[error("complete nonempty finite run has no decoded candidate evidence")]
    MissingDecodedEvidence,
    #[error("decoded candidate evidence names another event or question")]
    DecodedContextMismatch,
    #[error("decoded direct path and program-wide path have different endpoint types")]
    DecodedPathTypeMismatch,
    #[error("finite run outputs {run:?} differ from decoded candidates {decoded:?}")]
    CandidateOutputMismatch {
        run: Vec<ArtifactRef>,
        decoded: Vec<ArtifactRef>,
    },
    #[error(
        "supported-answer infrastructure failed before a support residual was established: {0}"
    )]
    SupportInfrastructure(Box<FiniteSupportedAnswerError>),
}

/// The expected and actual endpoint types of one checked finite resolution path.
#[derive(Debug)]
pub struct FiniteResolutionTypeMismatch {
    expected_input: TypeRef,
    actual_input: TypeRef,
    expected_output: TypeRef,
    actual_output: TypeRef,
}

impl fmt::Display for FiniteResolutionTypeMismatch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}->{} does not match {}->{}",
            self.actual_input, self.actual_output, self.expected_input, self.expected_output
        )
    }
}

impl From<ActualEventCheckError> for FiniteResolutionGateError {
    fn from(error: ActualEventCheckError) -> Self {
        Self::EventCheck(Box::new(error))
    }
}

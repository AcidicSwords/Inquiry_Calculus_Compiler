//! Finite local interrogative closure under explicit effectivity coverage.
//!
//! The result is a derived view over ordinary checked `Ask` occurrences. It makes no global
//! closure, scheduling, or task-success claim. Missing finite coverage stays `Unknown`, open
//! required discharge stays open, and explicit blocked/resource/authority/extension exits stay
//! visible rather than being counted as successful closure.

use std::collections::{BTreeMap, BTreeSet};

use thiserror::Error;

use crate::{
    ArtifactRef, AskOccurrenceCheckError, AskOccurrenceError, AskOccurrenceRef, BindingVersionRef,
    CoverageRef, EffectivityRef, HorizonRef, LiveQuestionCandidate, QuestionSuccessionCatalog,
    RequiredQuestionDischarge,
};

/// The context that bounds one local fixed-point derivation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalInterrogativeContext {
    binding: BindingVersionRef,
    effectivity: EffectivityRef,
    coverage: CoverageRef,
    horizon: HorizonRef,
    resource_bound: ArtifactRef,
}

impl LocalInterrogativeContext {
    #[must_use]
    pub const fn new(
        binding: BindingVersionRef,
        effectivity: EffectivityRef,
        coverage: CoverageRef,
        horizon: HorizonRef,
        resource_bound: ArtifactRef,
    ) -> Self {
        Self {
            binding,
            effectivity,
            coverage,
            horizon,
            resource_bound,
        }
    }

    #[must_use]
    pub const fn binding(&self) -> BindingVersionRef {
        self.binding
    }

    #[must_use]
    pub const fn effectivity(&self) -> EffectivityRef {
        self.effectivity
    }

    #[must_use]
    pub const fn coverage(&self) -> CoverageRef {
        self.coverage
    }

    #[must_use]
    pub const fn horizon(&self) -> HorizonRef {
        self.horizon
    }

    #[must_use]
    pub const fn resource_bound(&self) -> ArtifactRef {
        self.resource_bound
    }
}

/// One directed, occurrence-indexed local effectivity edge.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LocalEffectivityEdge {
    source: AskOccurrenceRef,
    target: AskOccurrenceRef,
}

impl LocalEffectivityEdge {
    #[must_use]
    pub const fn new(source: AskOccurrenceRef, target: AskOccurrenceRef) -> Self {
        Self { source, target }
    }

    #[must_use]
    pub const fn source(&self) -> AskOccurrenceRef {
        self.source
    }

    #[must_use]
    pub const fn target(&self) -> AskOccurrenceRef {
        self.target
    }
}

/// The exact finite occurrence and edge domain claimed by one independent coverage reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteLocalEffectivityCoverage {
    effectivity: EffectivityRef,
    coverage: CoverageRef,
    expected_occurrences: Vec<AskOccurrenceRef>,
    expected_edges: Vec<LocalEffectivityEdge>,
}

impl FiniteLocalEffectivityCoverage {
    pub fn new(
        effectivity: EffectivityRef,
        coverage: CoverageRef,
        expected_occurrences: Vec<AskOccurrenceRef>,
        expected_edges: Vec<LocalEffectivityEdge>,
    ) -> Result<Self, FiniteLocalEffectivityCoverageError> {
        let occurrences = collect_unique_occurrences(expected_occurrences)?;
        let edges = collect_unique_edges(expected_edges)?;
        for edge in &edges {
            if !occurrences.contains(&edge.source()) {
                return Err(
                    FiniteLocalEffectivityCoverageError::EdgeEndpointOutsideCoverage(edge.source()),
                );
            }
            if !occurrences.contains(&edge.target()) {
                return Err(
                    FiniteLocalEffectivityCoverageError::EdgeEndpointOutsideCoverage(edge.target()),
                );
            }
        }
        Ok(Self {
            effectivity,
            coverage,
            expected_occurrences: occurrences.into_iter().collect(),
            expected_edges: edges.into_iter().collect(),
        })
    }

    #[must_use]
    pub const fn effectivity(&self) -> EffectivityRef {
        self.effectivity
    }

    #[must_use]
    pub const fn coverage(&self) -> CoverageRef {
        self.coverage
    }

    #[must_use]
    pub fn expected_occurrences(&self) -> &[AskOccurrenceRef] {
        &self.expected_occurrences
    }

    #[must_use]
    pub fn expected_edges(&self) -> &[LocalEffectivityEdge] {
        &self.expected_edges
    }
}

/// A locally closing reason. Each variant retains its independently supplied evidence identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalQuestionClosingReason {
    Determined { evidence: ArtifactRef },
    FactorableRedundant { evidence: ArtifactRef },
    Inapplicable { evidence: ArtifactRef },
    NonProductive { evidence: ArtifactRef },
}

/// An explicit local exit that prevents the view from becoming successful closure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalQuestionExit {
    Blocked { dependency: ArtifactRef },
    ResourceBounded { bound: ArtifactRef },
    Authority { requirement: ArtifactRef },
    Extension { obligation: ArtifactRef },
}

/// One checked question candidate and its occurrence-local classification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalQuestionAssessment {
    candidate: LiveQuestionCandidate,
    classification: LocalQuestionClassification,
}

impl LocalQuestionAssessment {
    #[must_use]
    pub const fn closed(
        candidate: LiveQuestionCandidate,
        reason: LocalQuestionClosingReason,
    ) -> Self {
        Self {
            candidate,
            classification: LocalQuestionClassification::Closed(reason),
        }
    }

    #[must_use]
    pub const fn exit(candidate: LiveQuestionCandidate, exit: LocalQuestionExit) -> Self {
        Self {
            candidate,
            classification: LocalQuestionClassification::Exit(exit),
        }
    }

    #[must_use]
    pub const fn live_productive(candidate: LiveQuestionCandidate) -> Self {
        Self {
            candidate,
            classification: LocalQuestionClassification::LiveProductive,
        }
    }

    #[must_use]
    pub const fn candidate(&self) -> &LiveQuestionCandidate {
        &self.candidate
    }

    #[must_use]
    pub const fn classification(&self) -> LocalQuestionClassification {
        self.classification
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalQuestionClassification {
    Closed(LocalQuestionClosingReason),
    Exit(LocalQuestionExit),
    LiveProductive,
}

/// One still-open exact required occurrence and all of its retained obligations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenRequiredQuestion {
    occurrence: AskOccurrenceRef,
    discharges: Vec<RequiredQuestionDischarge>,
}

impl OpenRequiredQuestion {
    #[must_use]
    pub const fn occurrence(&self) -> AskOccurrenceRef {
        self.occurrence
    }

    #[must_use]
    pub fn discharges(&self) -> &[RequiredQuestionDischarge] {
        &self.discharges
    }
}

/// One occurrence-indexed explicit exit retained by the local fixed-point view.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalInterrogativeResidual {
    occurrence: AskOccurrenceRef,
    exit: LocalQuestionExit,
}

impl LocalInterrogativeResidual {
    #[must_use]
    pub const fn occurrence(&self) -> AskOccurrenceRef {
        self.occurrence
    }

    #[must_use]
    pub const fn exit(&self) -> LocalQuestionExit {
        self.exit
    }
}

/// The complete inspectable field behind one exact local result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteLocalInterrogativeView {
    context: LocalInterrogativeContext,
    roots: Vec<AskOccurrenceRef>,
    reachable: Vec<AskOccurrenceRef>,
    assessments: Vec<LocalQuestionAssessment>,
    effectivity_edges: Vec<LocalEffectivityEdge>,
}

impl FiniteLocalInterrogativeView {
    #[must_use]
    pub const fn context(&self) -> LocalInterrogativeContext {
        self.context
    }

    #[must_use]
    pub fn roots(&self) -> &[AskOccurrenceRef] {
        &self.roots
    }

    #[must_use]
    pub fn reachable(&self) -> &[AskOccurrenceRef] {
        &self.reachable
    }

    #[must_use]
    pub fn assessments(&self) -> &[LocalQuestionAssessment] {
        &self.assessments
    }

    #[must_use]
    pub fn effectivity_edges(&self) -> &[LocalEffectivityEdge] {
        &self.effectivity_edges
    }
}

/// Result of one finite local fixed-point derivation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LocalInterrogativeFixedPoint {
    Closed {
        view: FiniteLocalInterrogativeView,
    },
    Residual {
        view: FiniteLocalInterrogativeView,
        exits: Vec<LocalInterrogativeResidual>,
    },
    Open {
        view: FiniteLocalInterrogativeView,
        productive: Vec<AskOccurrenceRef>,
        required: Vec<OpenRequiredQuestion>,
        exits: Vec<LocalInterrogativeResidual>,
    },
    Unknown {
        context: LocalInterrogativeContext,
        available_occurrences: Vec<AskOccurrenceRef>,
        missing_occurrences: Vec<AskOccurrenceRef>,
        missing_edges: Vec<LocalEffectivityEdge>,
    },
}

/// The positive reason one newly covered occurrence reopens a predecessor local closure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LocalReopeningReason {
    Productive(AskOccurrenceRef),
    Required(OpenRequiredQuestion),
}

/// A monotone finite extension witness from one closed field to one newly open field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalInterrogativeReopening {
    predecessor: FiniteLocalInterrogativeView,
    successor: FiniteLocalInterrogativeView,
    added_occurrences: Vec<AskOccurrenceRef>,
    added_edges: Vec<LocalEffectivityEdge>,
    reasons: Vec<LocalReopeningReason>,
    successor_exits: Vec<LocalInterrogativeResidual>,
}

impl LocalInterrogativeReopening {
    #[must_use]
    pub const fn predecessor(&self) -> &FiniteLocalInterrogativeView {
        &self.predecessor
    }

    #[must_use]
    pub const fn successor(&self) -> &FiniteLocalInterrogativeView {
        &self.successor
    }

    #[must_use]
    pub fn added_occurrences(&self) -> &[AskOccurrenceRef] {
        &self.added_occurrences
    }

    #[must_use]
    pub fn added_edges(&self) -> &[LocalEffectivityEdge] {
        &self.added_edges
    }

    #[must_use]
    pub fn reasons(&self) -> &[LocalReopeningReason] {
        &self.reasons
    }

    #[must_use]
    pub fn successor_exits(&self) -> &[LocalInterrogativeResidual] {
        &self.successor_exits
    }
}

/// Admits reopening only through a positive newly covered productive or required occurrence.
///
/// This finite specialization requires the same roots, binding, effectivity relation, horizon,
/// and resource bound. The successor must use a fresh coverage identity, monotonically preserve
/// the predecessor occurrence and edge field, and add a reachable occurrence that is explicitly
/// live. Context drift, removal, or a new field with no newly live occurrence is insufficient.
pub fn derive_local_interrogative_reopening(
    predecessor: &LocalInterrogativeFixedPoint,
    successor: &LocalInterrogativeFixedPoint,
) -> Result<LocalInterrogativeReopening, LocalInterrogativeReopeningError> {
    let LocalInterrogativeFixedPoint::Closed {
        view: predecessor_view,
    } = predecessor
    else {
        return Err(LocalInterrogativeReopeningError::PredecessorNotClosed);
    };
    let LocalInterrogativeFixedPoint::Open {
        view: successor_view,
        productive,
        required,
        exits,
    } = successor
    else {
        return Err(LocalInterrogativeReopeningError::SuccessorNotOpen);
    };
    check_reopening_context(predecessor_view, successor_view)?;
    if predecessor_view.roots() != successor_view.roots() {
        return Err(LocalInterrogativeReopeningError::RootFamilyChanged);
    }

    let predecessor_occurrences: BTreeSet<_> =
        predecessor_view.reachable().iter().copied().collect();
    let successor_occurrences: BTreeSet<_> = successor_view.reachable().iter().copied().collect();
    if let Some(removed) = predecessor_occurrences
        .difference(&successor_occurrences)
        .next()
    {
        return Err(LocalInterrogativeReopeningError::RemovedOccurrence(
            *removed,
        ));
    }
    let predecessor_edges: BTreeSet<_> = predecessor_view
        .effectivity_edges()
        .iter()
        .copied()
        .collect();
    let successor_edges: BTreeSet<_> = successor_view.effectivity_edges().iter().copied().collect();
    if let Some(removed) = predecessor_edges.difference(&successor_edges).next() {
        return Err(LocalInterrogativeReopeningError::RemovedEdge(*removed));
    }
    let added_occurrences: Vec<_> = successor_occurrences
        .difference(&predecessor_occurrences)
        .copied()
        .collect();
    let added_edges: Vec<_> = successor_edges
        .difference(&predecessor_edges)
        .copied()
        .collect();
    if added_occurrences.is_empty() || added_edges.is_empty() {
        return Err(LocalInterrogativeReopeningError::NoPositiveExtension);
    }

    let added_set: BTreeSet<_> = added_occurrences.iter().copied().collect();
    let mut reasons = Vec::new();
    for occurrence in productive {
        if added_set.contains(occurrence) {
            reasons.push(LocalReopeningReason::Productive(*occurrence));
        }
    }
    for obligation in required {
        if added_set.contains(&obligation.occurrence()) {
            reasons.push(LocalReopeningReason::Required(obligation.clone()));
        }
    }
    if reasons.is_empty() {
        return Err(LocalInterrogativeReopeningError::NoNewLiveOccurrence);
    }
    Ok(LocalInterrogativeReopening {
        predecessor: predecessor_view.clone(),
        successor: successor_view.clone(),
        added_occurrences,
        added_edges,
        reasons,
        successor_exits: exits.clone(),
    })
}

fn check_reopening_context(
    predecessor: &FiniteLocalInterrogativeView,
    successor: &FiniteLocalInterrogativeView,
) -> Result<(), LocalInterrogativeReopeningError> {
    let before = predecessor.context();
    let after = successor.context();
    if before.binding() != after.binding() {
        return Err(LocalInterrogativeReopeningError::ContextChanged("binding"));
    }
    if before.effectivity() != after.effectivity() {
        return Err(LocalInterrogativeReopeningError::ContextChanged(
            "effectivity",
        ));
    }
    if before.horizon() != after.horizon() {
        return Err(LocalInterrogativeReopeningError::ContextChanged("horizon"));
    }
    if before.resource_bound() != after.resource_bound() {
        return Err(LocalInterrogativeReopeningError::ContextChanged(
            "resource bound",
        ));
    }
    if before.coverage() == after.coverage() {
        return Err(LocalInterrogativeReopeningError::CoverageIdentityReused);
    }
    Ok(())
}

/// Computes reachability to a least fixed point, then classifies that exact local field.
///
/// Coverage differences are checked before closure. Extra materialization outside the declared
/// coverage is rejected; missing materialization returns `Unknown`. Under exact coverage, every
/// reachable candidate is rechecked against its source. Open required discharges take precedence
/// over closing classifications, while explicit residual exits remain a non-success result.
#[allow(clippy::too_many_arguments)]
pub fn derive_finite_local_interrogative_fixed_point<C: QuestionSuccessionCatalog>(
    context: LocalInterrogativeContext,
    roots: &[AskOccurrenceRef],
    assessments: &[LocalQuestionAssessment],
    effectivity_edges: &[LocalEffectivityEdge],
    coverage: &FiniteLocalEffectivityCoverage,
    catalog: &C,
) -> Result<LocalInterrogativeFixedPoint, LocalInterrogativeFixedPointError> {
    if roots.is_empty() {
        return Err(LocalInterrogativeFixedPointError::EmptyRootFamily);
    }
    if coverage.effectivity() != context.effectivity() {
        return Err(LocalInterrogativeFixedPointError::EffectivityMismatch {
            context: context.effectivity(),
            coverage: coverage.effectivity(),
        });
    }
    if coverage.coverage() != context.coverage() {
        return Err(
            LocalInterrogativeFixedPointError::CoverageIdentityMismatch {
                context: context.coverage(),
                coverage: coverage.coverage(),
            },
        );
    }
    let root_set = collect_unique_roots(roots)?;
    let expected_occurrences: BTreeSet<_> =
        coverage.expected_occurrences().iter().copied().collect();
    for root in &root_set {
        if !expected_occurrences.contains(root) {
            return Err(LocalInterrogativeFixedPointError::RootOutsideCoverage(
                *root,
            ));
        }
    }

    let mut by_occurrence = BTreeMap::new();
    for assessment in assessments {
        assessment.candidate().occurrence().check(catalog)?;
        if assessment.candidate().occurrence().binding_version() != context.binding() {
            return Err(LocalInterrogativeFixedPointError::BindingMismatch {
                occurrence: assessment.candidate().occurrence().binding_version(),
                context: context.binding(),
            });
        }
        let query = catalog
            .resolve_open_query(assessment.candidate().occurrence().question())
            .expect("checked Ask occurrence retains its resolved question");
        if query.context().horizon() != context.horizon() {
            return Err(LocalInterrogativeFixedPointError::HorizonMismatch {
                occurrence: query.context().horizon(),
                context: context.horizon(),
            });
        }
        let reference = assessment.candidate().occurrence().ask_occurrence_ref()?;
        if by_occurrence.insert(reference, assessment).is_some() {
            return Err(LocalInterrogativeFixedPointError::DuplicateAssessment(
                reference,
            ));
        }
        if !expected_occurrences.contains(&reference) {
            return Err(LocalInterrogativeFixedPointError::AssessmentOutsideCoverage(reference));
        }
    }
    let supplied_occurrences: BTreeSet<_> = by_occurrence.keys().copied().collect();
    let actual_edges = collect_unique_actual_edges(effectivity_edges)?;
    let expected_edges: BTreeSet<_> = coverage.expected_edges().iter().copied().collect();
    if let Some(edge) = actual_edges.difference(&expected_edges).next() {
        return Err(LocalInterrogativeFixedPointError::EdgeOutsideCoverage(
            *edge,
        ));
    }
    let missing_occurrences: Vec<_> = expected_occurrences
        .difference(&supplied_occurrences)
        .copied()
        .collect();
    let missing_edges: Vec<_> = expected_edges.difference(&actual_edges).copied().collect();
    if !missing_occurrences.is_empty() || !missing_edges.is_empty() {
        return Ok(LocalInterrogativeFixedPoint::Unknown {
            context,
            available_occurrences: supplied_occurrences.into_iter().collect(),
            missing_occurrences,
            missing_edges,
        });
    }

    let reachable = least_reachable_fixed_point(&root_set, &actual_edges);
    let mut productive = Vec::new();
    let mut required = Vec::new();
    let mut exits = Vec::new();
    for occurrence in &reachable {
        let assessment = by_occurrence
            .get(occurrence)
            .expect("exact coverage established every reachable assessment");
        let candidate = assessment.candidate();
        if candidate.is_required() {
            required.push(OpenRequiredQuestion {
                occurrence: *occurrence,
                discharges: candidate.required_discharges().to_vec(),
            });
        }
        match assessment.classification() {
            LocalQuestionClassification::Closed(LocalQuestionClosingReason::NonProductive {
                ..
            }) if candidate.is_productive() => {
                return Err(
                    LocalInterrogativeFixedPointError::ProductiveCandidateDeclaredNonProductive(
                        *occurrence,
                    ),
                );
            }
            LocalQuestionClassification::Closed(_) => {}
            LocalQuestionClassification::Exit(exit) => {
                exits.push(LocalInterrogativeResidual {
                    occurrence: *occurrence,
                    exit,
                });
            }
            LocalQuestionClassification::LiveProductive if !candidate.is_productive() => {
                return Err(
                    LocalInterrogativeFixedPointError::NonProductiveCandidateDeclaredLive(
                        *occurrence,
                    ),
                );
            }
            LocalQuestionClassification::LiveProductive => productive.push(*occurrence),
        }
    }
    let reachable: Vec<_> = reachable.into_iter().collect();
    let reachable_assessments: Vec<_> = reachable
        .iter()
        .map(|occurrence| {
            (*by_occurrence
                .get(occurrence)
                .expect("exact reachable assessment remains available"))
            .clone()
        })
        .collect();
    let view = FiniteLocalInterrogativeView {
        context,
        roots: root_set.into_iter().collect(),
        reachable,
        assessments: reachable_assessments,
        effectivity_edges: actual_edges.into_iter().collect(),
    };
    if !productive.is_empty() || !required.is_empty() {
        return Ok(LocalInterrogativeFixedPoint::Open {
            view,
            productive,
            required,
            exits,
        });
    }
    if !exits.is_empty() {
        return Ok(LocalInterrogativeFixedPoint::Residual { view, exits });
    }
    Ok(LocalInterrogativeFixedPoint::Closed { view })
}

fn least_reachable_fixed_point(
    roots: &BTreeSet<AskOccurrenceRef>,
    edges: &BTreeSet<LocalEffectivityEdge>,
) -> BTreeSet<AskOccurrenceRef> {
    let mut reachable = roots.clone();
    loop {
        let before = reachable.len();
        for edge in edges {
            if reachable.contains(&edge.source()) {
                reachable.insert(edge.target());
            }
        }
        if reachable.len() == before {
            return reachable;
        }
    }
}

fn collect_unique_occurrences(
    occurrences: Vec<AskOccurrenceRef>,
) -> Result<BTreeSet<AskOccurrenceRef>, FiniteLocalEffectivityCoverageError> {
    let mut unique = BTreeSet::new();
    for occurrence in occurrences {
        if !unique.insert(occurrence) {
            return Err(FiniteLocalEffectivityCoverageError::DuplicateOccurrence(
                occurrence,
            ));
        }
    }
    Ok(unique)
}

fn collect_unique_edges(
    edges: Vec<LocalEffectivityEdge>,
) -> Result<BTreeSet<LocalEffectivityEdge>, FiniteLocalEffectivityCoverageError> {
    let mut unique = BTreeSet::new();
    for edge in edges {
        if !unique.insert(edge) {
            return Err(FiniteLocalEffectivityCoverageError::DuplicateEdge(edge));
        }
    }
    Ok(unique)
}

fn collect_unique_roots(
    roots: &[AskOccurrenceRef],
) -> Result<BTreeSet<AskOccurrenceRef>, LocalInterrogativeFixedPointError> {
    let mut unique = BTreeSet::new();
    for root in roots {
        if !unique.insert(*root) {
            return Err(LocalInterrogativeFixedPointError::DuplicateRoot(*root));
        }
    }
    Ok(unique)
}

fn collect_unique_actual_edges(
    edges: &[LocalEffectivityEdge],
) -> Result<BTreeSet<LocalEffectivityEdge>, LocalInterrogativeFixedPointError> {
    let mut unique = BTreeSet::new();
    for edge in edges {
        if !unique.insert(*edge) {
            return Err(LocalInterrogativeFixedPointError::DuplicateActualEdge(
                *edge,
            ));
        }
    }
    Ok(unique)
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum FiniteLocalEffectivityCoverageError {
    #[error("effectivity coverage repeats occurrence {0}")]
    DuplicateOccurrence(AskOccurrenceRef),
    #[error("effectivity coverage repeats edge {0:?}")]
    DuplicateEdge(LocalEffectivityEdge),
    #[error("effectivity edge endpoint {0} is outside its occurrence coverage")]
    EdgeEndpointOutsideCoverage(AskOccurrenceRef),
}

#[derive(Debug, Error)]
pub enum LocalInterrogativeFixedPointError {
    #[error(transparent)]
    OccurrenceCheck(#[from] AskOccurrenceCheckError),
    #[error(transparent)]
    OccurrenceIdentity(#[from] AskOccurrenceError),
    #[error("local interrogative root family is empty")]
    EmptyRootFamily,
    #[error("coverage is for effectivity {coverage}, not context effectivity {context}")]
    EffectivityMismatch {
        context: EffectivityRef,
        coverage: EffectivityRef,
    },
    #[error("coverage identity {coverage} does not match context coverage {context}")]
    CoverageIdentityMismatch {
        context: CoverageRef,
        coverage: CoverageRef,
    },
    #[error("occurrence binding {occurrence} does not match local context binding {context}")]
    BindingMismatch {
        occurrence: BindingVersionRef,
        context: BindingVersionRef,
    },
    #[error("occurrence horizon {occurrence} does not match local context horizon {context}")]
    HorizonMismatch {
        occurrence: HorizonRef,
        context: HorizonRef,
    },
    #[error("local interrogative root repeats occurrence {0}")]
    DuplicateRoot(AskOccurrenceRef),
    #[error("local interrogative root {0} is outside declared coverage")]
    RootOutsideCoverage(AskOccurrenceRef),
    #[error("local interrogative field repeats assessment for occurrence {0}")]
    DuplicateAssessment(AskOccurrenceRef),
    #[error("assessment for occurrence {0} is outside declared coverage")]
    AssessmentOutsideCoverage(AskOccurrenceRef),
    #[error("materialized effectivity edge {0:?} is outside declared coverage")]
    EdgeOutsideCoverage(LocalEffectivityEdge),
    #[error("materialized effectivity field repeats edge {0:?}")]
    DuplicateActualEdge(LocalEffectivityEdge),
    #[error("productive occurrence {0} was declared nonproductive")]
    ProductiveCandidateDeclaredNonProductive(AskOccurrenceRef),
    #[error("nonproductive occurrence {0} was declared live and productive")]
    NonProductiveCandidateDeclaredLive(AskOccurrenceRef),
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum LocalInterrogativeReopeningError {
    #[error("reopening predecessor is not a closed local field")]
    PredecessorNotClosed,
    #[error("reopening successor is not an open local field")]
    SuccessorNotOpen,
    #[error("reopening successor changed {0} without an admitted bridge")]
    ContextChanged(&'static str),
    #[error("reopening successor reused the predecessor coverage identity")]
    CoverageIdentityReused,
    #[error("reopening successor changed the declared root family")]
    RootFamilyChanged,
    #[error("reopening successor removed predecessor occurrence {0}")]
    RemovedOccurrence(AskOccurrenceRef),
    #[error("reopening successor removed predecessor edge {0:?}")]
    RemovedEdge(LocalEffectivityEdge),
    #[error("reopening successor added no occurrence-and-edge extension")]
    NoPositiveExtension,
    #[error("reopening successor added no productive or required occurrence")]
    NoNewLiveOccurrence,
}

//! Exact finite regeneration of an omitted inquiry-route position.
//!
//! The residual fiber and its result are derived checker data. They do not create route history,
//! cache authority, a learned method, or a semantic warrant.

use std::collections::BTreeSet;

use thiserror::Error;

use crate::{
    ArtifactRef, AskOccurrenceError, AskOccurrenceRef, BindingVersionRef, CompletionCandidateRef,
    CoverageRef, EventRef, ProbeOperatorRef, QueryRef, QuestionSuccessionCatalog,
    QuestionSuccessor, QuestionSuccessorError, RawReturnRef, RelationUseRef, SupportEnvironmentRef,
    TypedFormRef, derive_question_successor,
};

/// The visible route endpoint, kept separate from the full protected signature.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteEndpoint {
    Ask(QueryRef),
    Return(TypedFormRef),
}

/// The exact successor identity protected during regeneration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtectedRouteSuccessor {
    Ask(AskOccurrenceRef),
    Return(TypedFormRef),
}

/// The protected signature of one checked occurrence/answer/successor route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtectedRouteSignature {
    occurrence: AskOccurrenceRef,
    binding: BindingVersionRef,
    question: QueryRef,
    answer_candidates: Vec<CompletionCandidateRef>,
    observations: Vec<RelationUseRef>,
    support: Vec<SupportEnvironmentRef>,
    event: EventRef,
    operator: ProbeOperatorRef,
    raw_return: RawReturnRef,
    continuation: crate::IProgRef,
    successor: ProtectedRouteSuccessor,
    reopening: ArtifactRef,
}

/// One checked route node proposed for removal.
///
/// Checking establishes the node's pre-ablation protected signature. It does not establish that
/// the node is transparent or authorize its removal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedFiniteRouteNode {
    identity: ArtifactRef,
    signature: ProtectedRouteSignature,
}

impl CheckedFiniteRouteNode {
    #[must_use]
    pub const fn identity(&self) -> ArtifactRef {
        self.identity
    }

    #[must_use]
    pub const fn signature(&self) -> &ProtectedRouteSignature {
        &self.signature
    }
}

impl ProtectedRouteSignature {
    #[must_use]
    pub const fn occurrence(&self) -> AskOccurrenceRef {
        self.occurrence
    }

    #[must_use]
    pub const fn binding(&self) -> BindingVersionRef {
        self.binding
    }

    #[must_use]
    pub const fn question(&self) -> QueryRef {
        self.question
    }

    #[must_use]
    pub fn answer_candidates(&self) -> &[CompletionCandidateRef] {
        &self.answer_candidates
    }

    #[must_use]
    pub fn observations(&self) -> &[RelationUseRef] {
        &self.observations
    }

    #[must_use]
    pub fn support(&self) -> &[SupportEnvironmentRef] {
        &self.support
    }

    #[must_use]
    pub const fn event(&self) -> EventRef {
        self.event
    }

    #[must_use]
    pub const fn operator(&self) -> ProbeOperatorRef {
        self.operator
    }

    #[must_use]
    pub const fn raw_return(&self) -> RawReturnRef {
        self.raw_return
    }

    #[must_use]
    pub const fn continuation(&self) -> crate::IProgRef {
        self.continuation
    }

    #[must_use]
    pub const fn successor(&self) -> ProtectedRouteSuccessor {
        self.successor
    }

    #[must_use]
    pub const fn reopening(&self) -> ArtifactRef {
        self.reopening
    }
}

/// One possible reconstruction in an exact residual completion fiber.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteRouteReconstruction {
    completion: ArtifactRef,
    route: QuestionSuccessor,
    reopening: ArtifactRef,
}

impl FiniteRouteReconstruction {
    #[must_use]
    pub const fn new(
        completion: ArtifactRef,
        route: QuestionSuccessor,
        reopening: ArtifactRef,
    ) -> Self {
        Self {
            completion,
            route,
            reopening,
        }
    }

    #[must_use]
    pub const fn completion(&self) -> ArtifactRef {
        self.completion
    }

    #[must_use]
    pub const fn route(&self) -> &QuestionSuccessor {
        &self.route
    }

    #[must_use]
    pub const fn reopening(&self) -> ArtifactRef {
        self.reopening
    }
}

/// One caller-declared exact finite residual fiber for the omitted position.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFiniteRouteResidualFiber {
    coverage: CoverageRef,
    reconstructions: Vec<FiniteRouteReconstruction>,
}

impl ExactFiniteRouteResidualFiber {
    pub fn new(
        coverage: CoverageRef,
        reconstructions: Vec<FiniteRouteReconstruction>,
    ) -> Result<Self, ExactFiniteRouteResidualFiberError> {
        if reconstructions.is_empty() {
            return Err(ExactFiniteRouteResidualFiberError::EmptyFiber);
        }
        let mut seen = BTreeSet::new();
        for reconstruction in &reconstructions {
            if !seen.insert(reconstruction.completion()) {
                return Err(ExactFiniteRouteResidualFiberError::DuplicateCompletion(
                    reconstruction.completion(),
                ));
            }
        }
        Ok(Self {
            coverage,
            reconstructions,
        })
    }

    #[must_use]
    pub const fn coverage(&self) -> CoverageRef {
        self.coverage
    }

    #[must_use]
    pub fn reconstructions(&self) -> &[FiniteRouteReconstruction] {
        &self.reconstructions
    }
}

/// A positive pair showing why the omitted route position cannot be regenerated exactly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteRegenerationSeparator {
    first_completion: ArtifactRef,
    second_completion: ArtifactRef,
    endpoint: RouteEndpoint,
    first_signature: ProtectedRouteSignature,
    second_signature: ProtectedRouteSignature,
}

impl RouteRegenerationSeparator {
    #[must_use]
    pub const fn first_completion(&self) -> ArtifactRef {
        self.first_completion
    }

    #[must_use]
    pub const fn second_completion(&self) -> ArtifactRef {
        self.second_completion
    }

    #[must_use]
    pub const fn endpoint(&self) -> RouteEndpoint {
        self.endpoint
    }

    #[must_use]
    pub const fn first_signature(&self) -> &ProtectedRouteSignature {
        &self.first_signature
    }

    #[must_use]
    pub const fn second_signature(&self) -> &ProtectedRouteSignature {
        &self.second_signature
    }
}

/// Result of checking the whole exact residual fiber.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FiniteRouteRegenerationResult {
    Regenerated {
        coverage: CoverageRef,
        completions: Vec<ArtifactRef>,
        route: Box<QuestionSuccessor>,
        signature: Box<ProtectedRouteSignature>,
    },
    Split {
        coverage: CoverageRef,
        separator: Box<RouteRegenerationSeparator>,
    },
}

/// Positive reason why a separately regenerable retained basis does not authorize ablation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FiniteRouteAblationRefusal {
    ResidualFiberSplit(Box<RouteRegenerationSeparator>),
    ProtectedSignatureChanged {
        before: Box<ProtectedRouteSignature>,
        after: Box<ProtectedRouteSignature>,
    },
}

/// Derived evidence that one checked node can be removed and exactly reconstructed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteRouteAblationWitness {
    removed_node: ArtifactRef,
    coverage: CoverageRef,
    completions: Vec<ArtifactRef>,
    regenerated_route: Box<QuestionSuccessor>,
    protected_signature: Box<ProtectedRouteSignature>,
}

impl FiniteRouteAblationWitness {
    #[must_use]
    pub const fn removed_node(&self) -> ArtifactRef {
        self.removed_node
    }

    #[must_use]
    pub const fn coverage(&self) -> CoverageRef {
        self.coverage
    }

    #[must_use]
    pub fn completions(&self) -> &[ArtifactRef] {
        &self.completions
    }

    #[must_use]
    pub const fn regenerated_route(&self) -> &QuestionSuccessor {
        &self.regenerated_route
    }

    #[must_use]
    pub const fn protected_signature(&self) -> &ProtectedRouteSignature {
        &self.protected_signature
    }
}

/// Result of attempting actual removal through an independently rechecked retained basis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FiniteRouteAblationResult {
    Ablated(Box<FiniteRouteAblationWitness>),
    Refused {
        node: ArtifactRef,
        reason: Box<FiniteRouteAblationRefusal>,
    },
}

/// Rechecks the proposed node and seals the pre-ablation protected signature.
pub fn check_finite_route_node<C: QuestionSuccessionCatalog>(
    identity: ArtifactRef,
    route: QuestionSuccessor,
    reopening: ArtifactRef,
    catalog: &C,
) -> Result<CheckedFiniteRouteNode, FiniteRouteRegenerationError> {
    let rebuilt = rebuild_route(&route, catalog)?;
    if rebuilt != route {
        return Err(FiniteRouteRegenerationError::RouteNoLongerReconstructs(
            identity,
        ));
    }
    Ok(CheckedFiniteRouteNode {
        identity,
        signature: protected_route_signature(&rebuilt, reopening)?,
    })
}

/// Attempts to remove a checked node through a retained basis that is rechecked without using the
/// node's sealed signature as regeneration evidence.
///
/// Regeneration is necessary but not self-authorizing. The reconstructed whole-fiber signature
/// must also equal the independently sealed pre-ablation signature.
pub fn ablate_exact_finite_route_node<C: QuestionSuccessionCatalog>(
    node: &CheckedFiniteRouteNode,
    retained_basis: &ExactFiniteRouteResidualFiber,
    catalog: &C,
) -> Result<FiniteRouteAblationResult, FiniteRouteRegenerationError> {
    match check_exact_finite_route_regeneration(retained_basis, catalog)? {
        FiniteRouteRegenerationResult::Split { separator, .. } => {
            Ok(FiniteRouteAblationResult::Refused {
                node: node.identity(),
                reason: Box::new(FiniteRouteAblationRefusal::ResidualFiberSplit(separator)),
            })
        }
        FiniteRouteRegenerationResult::Regenerated {
            coverage,
            completions,
            route,
            signature,
        } => {
            if signature.as_ref() != node.signature() {
                return Ok(FiniteRouteAblationResult::Refused {
                    node: node.identity(),
                    reason: Box::new(FiniteRouteAblationRefusal::ProtectedSignatureChanged {
                        before: Box::new(node.signature().clone()),
                        after: signature,
                    }),
                });
            }
            Ok(FiniteRouteAblationResult::Ablated(Box::new(
                FiniteRouteAblationWitness {
                    removed_node: node.identity(),
                    coverage,
                    completions,
                    regenerated_route: route,
                    protected_signature: signature,
                },
            )))
        }
    }
}

/// Rechecks every represented route and admits regeneration only when the full protected
/// signature is constant across the exact residual fiber.
pub fn check_exact_finite_route_regeneration<C: QuestionSuccessionCatalog>(
    fiber: &ExactFiniteRouteResidualFiber,
    catalog: &C,
) -> Result<FiniteRouteRegenerationResult, FiniteRouteRegenerationError> {
    let mut checked = Vec::with_capacity(fiber.reconstructions().len());
    for reconstruction in fiber.reconstructions() {
        let rebuilt = rebuild_route(reconstruction.route(), catalog)?;
        if &rebuilt != reconstruction.route() {
            return Err(FiniteRouteRegenerationError::RouteNoLongerReconstructs(
                reconstruction.completion(),
            ));
        }
        checked.push((
            reconstruction,
            route_endpoint(&rebuilt)?,
            protected_route_signature(&rebuilt, reconstruction.reopening())?,
        ));
    }
    let endpoint = checked[0].1;
    for (reconstruction, candidate_endpoint, _) in &checked[1..] {
        if *candidate_endpoint != endpoint {
            return Err(FiniteRouteRegenerationError::EndpointMismatch {
                first: endpoint,
                other: *candidate_endpoint,
                completion: reconstruction.completion(),
            });
        }
    }
    let first = &checked[0];
    if let Some(other) = checked[1..]
        .iter()
        .find(|(_, _, signature)| signature != &first.2)
    {
        return Ok(FiniteRouteRegenerationResult::Split {
            coverage: fiber.coverage(),
            separator: Box::new(RouteRegenerationSeparator {
                first_completion: first.0.completion(),
                second_completion: other.0.completion(),
                endpoint,
                first_signature: first.2.clone(),
                second_signature: other.2.clone(),
            }),
        });
    }
    Ok(FiniteRouteRegenerationResult::Regenerated {
        coverage: fiber.coverage(),
        completions: checked
            .iter()
            .map(|(reconstruction, _, _)| reconstruction.completion())
            .collect(),
        route: Box::new(first.0.route().clone()),
        signature: Box::new(first.2.clone()),
    })
}

fn rebuild_route<C: QuestionSuccessionCatalog>(
    route: &QuestionSuccessor,
    catalog: &C,
) -> Result<QuestionSuccessor, QuestionSuccessorError> {
    match route {
        QuestionSuccessor::Ask {
            occurrence, answer, ..
        }
        | QuestionSuccessor::Return {
            occurrence, answer, ..
        } => derive_question_successor(occurrence.clone(), answer.clone(), catalog),
    }
}

fn route_endpoint(route: &QuestionSuccessor) -> Result<RouteEndpoint, AskOccurrenceError> {
    match route {
        QuestionSuccessor::Ask { successor, .. } => Ok(RouteEndpoint::Ask(successor.question())),
        QuestionSuccessor::Return { value, .. } => Ok(RouteEndpoint::Return(*value)),
    }
}

fn protected_route_signature(
    route: &QuestionSuccessor,
    reopening: ArtifactRef,
) -> Result<ProtectedRouteSignature, AskOccurrenceError> {
    let (occurrence, answer, successor) = match route {
        QuestionSuccessor::Ask {
            occurrence,
            answer,
            successor,
        } => (
            occurrence,
            answer,
            ProtectedRouteSuccessor::Ask(successor.ask_occurrence_ref()?),
        ),
        QuestionSuccessor::Return {
            occurrence,
            answer,
            value,
        } => (occurrence, answer, ProtectedRouteSuccessor::Return(*value)),
    };
    Ok(ProtectedRouteSignature {
        occurrence: occurrence.ask_occurrence_ref()?,
        binding: occurrence.binding_version(),
        question: occurrence.question(),
        answer_candidates: answer.candidates().to_vec(),
        observations: answer
            .observations()
            .iter()
            .map(|observation| observation.observation())
            .collect(),
        support: answer
            .support()
            .iter()
            .map(|support| support.environment())
            .collect(),
        event: answer.event(),
        operator: answer.operator(),
        raw_return: answer.raw_return(),
        continuation: occurrence.continuation(),
        successor,
        reopening,
    })
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ExactFiniteRouteResidualFiberError {
    #[error("exact route residual fiber is empty")]
    EmptyFiber,
    #[error("exact route residual fiber repeats completion {0}")]
    DuplicateCompletion(ArtifactRef),
}

#[derive(Debug, Error)]
pub enum FiniteRouteRegenerationError {
    #[error(transparent)]
    Successor(#[from] QuestionSuccessorError),
    #[error(transparent)]
    OccurrenceIdentity(#[from] AskOccurrenceError),
    #[error("route completion {0} no longer reconstructs exactly")]
    RouteNoLongerReconstructs(ArtifactRef),
    #[error("route completion {completion} reaches {other:?}, not shared endpoint {first:?}")]
    EndpointMismatch {
        first: RouteEndpoint,
        other: RouteEndpoint,
        completion: ArtifactRef,
    },
}

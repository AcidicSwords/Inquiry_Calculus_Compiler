use crate::{
    DischargeMode, NegationCatalog, NegationCoverage, NegationUse, NegationUseCheckError,
    NegationUseError, NegationUseRef, OpenPort, OpenQuery, OpenQueryCheckError, RelationUseContext,
    RelationUseError, RelationUseRef, TypedFormRef,
};

/// The positive-negation question `?y[N_u(x, y)]` together with what licensed it.
///
/// Plan section 26 makes this an *ordinary* `OpenQuery`; nothing here is a new question species.
/// The wrapper exists only to stop two things from being dropped on the way to the answer: the
/// use tag, because section 23 requires the relation use that licensed an exterior to survive
/// into the occurrence, and the declared semantic coverage, because a working negation relation
/// cannot support a closure claim merely because a candidate came back.
///
/// Holding one of these is holding a question. It is not an exterior, an `O_X`, a candidate, or
/// an actualization, and answering it through a generative route would not make it one.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PositiveNegationQuery {
    negation_use: NegationUseRef,
    semantic_coverage: NegationCoverage,
    query: OpenQuery,
}

impl PositiveNegationQuery {
    /// Returns the use that licensed this question.
    #[must_use]
    pub const fn negation_use(&self) -> NegationUseRef {
        self.negation_use
    }

    /// Returns the use's declared semantic coverage, which is not execution coverage.
    #[must_use]
    pub const fn semantic_coverage(&self) -> NegationCoverage {
        self.semantic_coverage
    }

    /// Returns the ordinary open query.
    #[must_use]
    pub const fn query(&self) -> &OpenQuery {
        &self.query
    }
}

/// Builds the positive-negation question for one declared negation use.
///
/// The source stays bound and the remaining ports are exposed, so the question asks what is
/// exterior *to this source* rather than which pairs the relation happens to relate. A relation
/// whose ports are already fully bound is refused: that is a proposition, and the partial-binding
/// chain runs schema -> partial binding -> question -> complete binding -> proposition.
///
/// `candidate_mode` is the route by which the open port may lawfully be discharged, and it is
/// carried on the port rather than assumed. Construction evaluates no relation, produces no
/// candidate, and establishes no exterior; a generated answer to this question would be a
/// generated `O_X`, which plan section 26 keeps distinct from an actualized one.
pub fn positive_negation_query<C: NegationCatalog>(
    negation_use: &NegationUse,
    candidate_mode: DischargeMode,
    catalog: &C,
) -> Result<PositiveNegationQuery, PositiveNegationQueryError> {
    negation_use.check(catalog)?;

    let use_ref = negation_use.relation_use();
    let relation_use = catalog
        .resolve_relation_use(use_ref)
        .ok_or(PositiveNegationQueryError::UnresolvedRelationUse(use_ref))?;
    let schema = catalog
        .resolve_relation_schema(relation_use.relation())
        .ok_or(PositiveNegationQueryError::UnresolvedRelationSchema)?;

    let presentation = catalog
        .resolve_determination_presentation(negation_use.source_determination())
        .ok_or(PositiveNegationQueryError::UnresolvedPresentation)?;

    let bound_ports = relation_use.bindings().to_vec();

    // The question must be about the source the determination presents. Without this, the
    // constructed query is a well-typed question about the relation at large.
    if !bound_ports
        .iter()
        .any(|binding| binding.value() == presentation.source())
    {
        return Err(PositiveNegationQueryError::SourceNotBound(
            presentation.source(),
        ));
    }

    let open_ports: Vec<OpenPort> = schema
        .ports()
        .iter()
        .filter(|port| {
            !bound_ports
                .iter()
                .any(|binding| binding.port() == port.name())
        })
        .map(|port| OpenPort::new(port.name().clone(), candidate_mode))
        .collect();

    if open_ports.is_empty() {
        return Err(PositiveNegationQueryError::NoOpenCandidatePort);
    }

    let query = OpenQuery::new(
        relation_use.relation(),
        bound_ports,
        open_ports,
        RelationUseContext::new(
            negation_use.scope(),
            negation_use.applicability(),
            negation_use.grain(),
            negation_use.horizon(),
            candidate_mode,
            relation_use.support(),
            relation_use.warrant(),
        ),
    );
    query.check(catalog)?;

    Ok(PositiveNegationQuery {
        negation_use: negation_use.negation_use_ref()?,
        semantic_coverage: negation_use.semantic_coverage(),
        query,
    })
}

/// Errors from constructing a positive-negation question.
#[derive(Debug, thiserror::Error)]
pub enum PositiveNegationQueryError {
    #[error(transparent)]
    NegationUse(#[from] NegationUseCheckError),
    #[error(transparent)]
    NegationUseEncoding(#[from] NegationUseError),
    #[error(transparent)]
    RelationUseEncoding(#[from] RelationUseError),
    #[error(transparent)]
    Query(#[from] OpenQueryCheckError),
    #[error("negation relation use {0} is not available from the declared catalog")]
    UnresolvedRelationUse(RelationUseRef),
    #[error("the negation relation schema is not available from the declared catalog")]
    UnresolvedRelationSchema,
    #[error("the source determination presentation is not available from the declared catalog")]
    UnresolvedPresentation,
    #[error("the negation relation use does not bind the presented source {0}")]
    SourceNotBound(TypedFormRef),
    #[error(
        "the negation relation use binds every port, leaving a proposition rather than a question"
    )]
    NoOpenCandidatePort,
}

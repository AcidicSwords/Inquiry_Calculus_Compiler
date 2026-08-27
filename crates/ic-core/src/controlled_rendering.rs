//! Controlled, derived rendering of backward-looking typed questions.
//!
//! Surface wording is deliberately non-authoritative.  The same words may render an existential
//! preimage, a binding-supplied universal adjunction, or the reverse section of one admitted
//! negation use.  Round-trip elaboration therefore checks an inspectable typed payload and an
//! independently supplied expectation; it never recovers semantics by parsing the words.

use std::collections::BTreeSet;

use thiserror::Error;

use crate::{
    AdmittedFiniteAdjunction, AdmittedFiniteNegationExtension, ArtifactRef, BindingVersionRef,
    CoverageRef, ExactReturnFiber, FiniteAdjunctionCatalog, FiniteAdjunctionError,
    NegationCoverage, NegationUseCheckError, NegationUseError, NegationUseRef, OpenPort, OpenQuery,
    OpenQueryCheckError, OpenQueryError, PortBinding, QueryRef, RelationCheckError, RelationError,
    RelationPort, RelationRef, RelationUse, RelationUseCheckError, RelationUseError,
    RelationUseRef, TaggedExteriorCatalog, TypeCheckError, TypeError, TypedFormRef,
    admit_finite_adjunction,
};

/// Catalog required to recheck every typed contract represented by the controlled prompt.
pub trait ControlledRenderingCatalog: TaggedExteriorCatalog + FiniteAdjunctionCatalog {}

impl<T> ControlledRenderingCatalog for T where T: TaggedExteriorCatalog + FiniteAdjunctionCatalog {}

/// A rendering root is an implementation-only wording aid, never a semantic question species.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlledInterrogativeRoot {
    UnderWhatConditions,
}

impl ControlledInterrogativeRoot {
    #[must_use]
    pub const fn canonical_text(self) -> &'static str {
        match self {
            Self::UnderWhatConditions => "under what conditions",
        }
    }
}

/// The protected contract distinction carried independently of surface text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlledBackwardKind {
    ExistentialPreimage,
    UniversalAdjoint,
    SameUseReciprocalReturn,
}

/// Coverage retained by controlled elaboration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlledCoverage {
    DeclaredExact(CoverageRef),
    Negation(NegationCoverage),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ControlledBackwardContract {
    ExistentialPreimage {
        coverage: CoverageRef,
        sources: BTreeSet<TypedFormRef>,
    },
    UniversalAdjoint {
        coverage: CoverageRef,
        law: ArtifactRef,
        adjunction: Box<AdmittedFiniteAdjunction>,
    },
    SameUseReciprocalReturn {
        negation_use: NegationUseRef,
        relation_use: RelationUseRef,
        semantic_coverage: NegationCoverage,
        fiber: ExactReturnFiber,
    },
}

impl ControlledBackwardContract {
    const fn kind(&self) -> ControlledBackwardKind {
        match self {
            Self::ExistentialPreimage { .. } => ControlledBackwardKind::ExistentialPreimage,
            Self::UniversalAdjoint { .. } => ControlledBackwardKind::UniversalAdjoint,
            Self::SameUseReciprocalReturn { .. } => ControlledBackwardKind::SameUseReciprocalReturn,
        }
    }

    const fn coverage(&self) -> ControlledCoverage {
        match self {
            Self::ExistentialPreimage { coverage, .. }
            | Self::UniversalAdjoint { coverage, .. } => {
                ControlledCoverage::DeclaredExact(*coverage)
            }
            Self::SameUseReciprocalReturn {
                semantic_coverage, ..
            } => ControlledCoverage::Negation(*semantic_coverage),
        }
    }

    const fn reciprocal_use(&self) -> Option<NegationUseRef> {
        match self {
            Self::SameUseReciprocalReturn { negation_use, .. } => Some(*negation_use),
            Self::ExistentialPreimage { .. } | Self::UniversalAdjoint { .. } => None,
        }
    }
}

/// Inspectable controlled prompt data.
///
/// The query is retained in full so relation identity, port partition, modes, context, and
/// provenance can be rechecked. This is derived compiler data, not a canonical artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlledInterrogativePrompt {
    renderer_version: ArtifactRef,
    root: ControlledInterrogativeRoot,
    rendered_text: String,
    query_ref: QueryRef,
    query: OpenQuery,
    binding: BindingVersionRef,
    contract: ControlledBackwardContract,
}

impl ControlledInterrogativePrompt {
    #[must_use]
    pub const fn renderer_version(&self) -> ArtifactRef {
        self.renderer_version
    }

    #[must_use]
    pub const fn root(&self) -> ControlledInterrogativeRoot {
        self.root
    }

    #[must_use]
    pub fn rendered_text(&self) -> &str {
        &self.rendered_text
    }

    #[must_use]
    pub const fn query_ref(&self) -> QueryRef {
        self.query_ref
    }

    #[must_use]
    pub const fn query(&self) -> &OpenQuery {
        &self.query
    }

    #[must_use]
    pub const fn binding(&self) -> BindingVersionRef {
        self.binding
    }

    #[must_use]
    pub const fn kind(&self) -> ControlledBackwardKind {
        self.contract.kind()
    }

    #[must_use]
    pub const fn coverage(&self) -> ControlledCoverage {
        self.contract.coverage()
    }

    #[must_use]
    pub const fn reciprocal_use(&self) -> Option<NegationUseRef> {
        self.contract.reciprocal_use()
    }
}

/// Independent contract expected by elaboration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ControlledElaborationExpectation {
    renderer_version: ArtifactRef,
    query: QueryRef,
    kind: ControlledBackwardKind,
    coverage: ControlledCoverage,
    reciprocal_use: Option<NegationUseRef>,
}

impl ControlledElaborationExpectation {
    #[must_use]
    pub const fn new(
        renderer_version: ArtifactRef,
        query: QueryRef,
        kind: ControlledBackwardKind,
        coverage: ControlledCoverage,
        reciprocal_use: Option<NegationUseRef>,
    ) -> Self {
        Self {
            renderer_version,
            query,
            kind,
            coverage,
            reciprocal_use,
        }
    }
}

/// Reconstructed normalized contract after controlled elaboration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlledInterrogativeElaboration {
    renderer_version: ArtifactRef,
    query_ref: QueryRef,
    query: OpenQuery,
    binding: BindingVersionRef,
    kind: ControlledBackwardKind,
    coverage: ControlledCoverage,
    reciprocal_use: Option<NegationUseRef>,
}

impl ControlledInterrogativeElaboration {
    #[must_use]
    pub const fn renderer_version(&self) -> ArtifactRef {
        self.renderer_version
    }

    #[must_use]
    pub const fn query_ref(&self) -> QueryRef {
        self.query_ref
    }

    #[must_use]
    pub const fn query(&self) -> &OpenQuery {
        &self.query
    }

    #[must_use]
    pub const fn binding(&self) -> BindingVersionRef {
        self.binding
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

/// Renders one caller-declared exact finite preimage of the checked query.
pub fn render_existential_preimage<C: ControlledRenderingCatalog>(
    renderer_version: ArtifactRef,
    query: OpenQuery,
    coverage: CoverageRef,
    sources: Vec<TypedFormRef>,
    catalog: &C,
) -> Result<ControlledInterrogativePrompt, ControlledRenderingError> {
    let mut unique = BTreeSet::new();
    for source in sources {
        if !unique.insert(source) {
            return Err(ControlledRenderingError::DuplicatePreimageSource(source));
        }
    }
    render(
        renderer_version,
        query,
        ControlledBackwardContract::ExistentialPreimage {
            coverage,
            sources: unique,
        },
        catalog,
    )
}

/// Renders a backward question whose universal force comes only from a rechecked finite
/// adjunction law explicitly named by the relation schema.
pub fn render_universal_adjoint<C: ControlledRenderingCatalog>(
    renderer_version: ArtifactRef,
    query: OpenQuery,
    coverage: CoverageRef,
    law: ArtifactRef,
    adjunction: AdmittedFiniteAdjunction,
    catalog: &C,
) -> Result<ControlledInterrogativePrompt, ControlledRenderingError> {
    render(
        renderer_version,
        query,
        ControlledBackwardContract::UniversalAdjoint {
            coverage,
            law,
            adjunction: Box::new(adjunction),
        },
        catalog,
    )
}

/// Renders the reverse section of one admitted use at one exterior.
pub fn render_same_use_reciprocal_return<C: ControlledRenderingCatalog>(
    renderer_version: ArtifactRef,
    query: OpenQuery,
    extension: &AdmittedFiniteNegationExtension,
    exterior: TypedFormRef,
    catalog: &C,
) -> Result<ControlledInterrogativePrompt, ControlledRenderingError> {
    let negation_use = extension.negation_use();
    let use_value = catalog.resolve_negation_use(negation_use).ok_or(
        ControlledRenderingError::UnresolvedNegationUse(negation_use),
    )?;
    let relation_use = use_value.relation_use();
    let fiber = extension.return_fiber(exterior)?;
    render(
        renderer_version,
        query,
        ControlledBackwardContract::SameUseReciprocalReturn {
            negation_use,
            relation_use,
            semantic_coverage: extension.semantic_coverage(),
            fiber,
        },
        catalog,
    )
}

fn render<C: ControlledRenderingCatalog>(
    renderer_version: ArtifactRef,
    query: OpenQuery,
    contract: ControlledBackwardContract,
    catalog: &C,
) -> Result<ControlledInterrogativePrompt, ControlledRenderingError> {
    let (query_ref, binding) = check_contract(&query, &contract, catalog)?;
    let root = ControlledInterrogativeRoot::UnderWhatConditions;
    Ok(ControlledInterrogativePrompt {
        renderer_version,
        root,
        rendered_text: root.canonical_text().to_owned(),
        query_ref,
        query,
        binding,
        contract,
    })
}

/// Elaborates controlled prompt data only when it still matches an independent expected contract.
pub fn elaborate_controlled_interrogative<C: ControlledRenderingCatalog>(
    prompt: &ControlledInterrogativePrompt,
    expected: ControlledElaborationExpectation,
    catalog: &C,
) -> Result<ControlledInterrogativeElaboration, ControlledRenderingError> {
    if prompt.renderer_version != expected.renderer_version {
        return Err(ControlledRenderingError::RendererVersionMismatch {
            expected: expected.renderer_version,
            actual: prompt.renderer_version,
        });
    }
    if prompt.query_ref != expected.query {
        return Err(ControlledRenderingError::QueryMismatch {
            expected: expected.query,
            actual: prompt.query_ref,
        });
    }
    if prompt.contract.kind() != expected.kind {
        return Err(ControlledRenderingError::ContractKindMismatch {
            expected: expected.kind,
            actual: prompt.contract.kind(),
        });
    }
    if prompt.contract.coverage() != expected.coverage {
        return Err(ControlledRenderingError::CoverageMismatch {
            expected: Box::new(expected.coverage),
            actual: Box::new(prompt.contract.coverage()),
        });
    }
    if prompt.contract.reciprocal_use() != expected.reciprocal_use {
        return Err(ControlledRenderingError::ReciprocalUseMismatch {
            expected: expected.reciprocal_use,
            actual: prompt.contract.reciprocal_use(),
        });
    }
    if prompt.rendered_text != prompt.root.canonical_text() {
        return Err(ControlledRenderingError::NonCanonicalSurfaceText);
    }
    let (query_ref, binding) = check_contract(&prompt.query, &prompt.contract, catalog)?;
    if query_ref != prompt.query_ref {
        return Err(ControlledRenderingError::QueryIdentityChanged {
            expected: prompt.query_ref,
            actual: query_ref,
        });
    }
    if binding != prompt.binding {
        return Err(ControlledRenderingError::BindingChanged {
            expected: prompt.binding,
            actual: binding,
        });
    }
    Ok(ControlledInterrogativeElaboration {
        renderer_version: prompt.renderer_version,
        query_ref,
        query: prompt.query.clone(),
        binding,
        kind: prompt.contract.kind(),
        coverage: prompt.contract.coverage(),
        reciprocal_use: prompt.contract.reciprocal_use(),
    })
}

fn check_contract<C: ControlledRenderingCatalog>(
    query: &OpenQuery,
    contract: &ControlledBackwardContract,
    catalog: &C,
) -> Result<(QueryRef, BindingVersionRef), ControlledRenderingError> {
    query.check(catalog)?;
    let query_ref = query.query_ref()?;
    let schema = catalog.resolve_relation_schema(query.relation()).ok_or(
        ControlledRenderingError::UnresolvedRelation(query.relation()),
    )?;
    let calculated = schema.relation_ref()?;
    if calculated != query.relation() {
        return Err(ControlledRenderingError::RelationIdentityMismatch {
            expected: query.relation(),
            actual: calculated,
        });
    }
    schema.check(catalog)?;
    let shape = BinaryBackwardShape::from_query(query, &schema)?;
    match contract {
        ControlledBackwardContract::ExistentialPreimage { sources, .. } => {
            for source in sources {
                check_form(*source, shape.open_port.ty(), schema.binding(), catalog)?;
            }
        }
        ControlledBackwardContract::UniversalAdjoint {
            law, adjunction, ..
        } => {
            if !schema.laws().contains(law) {
                return Err(ControlledRenderingError::AdjunctionLawNotNamed(*law));
            }
            let rechecked = admit_finite_adjunction(adjunction.candidate().clone(), catalog)?;
            if &rechecked != adjunction.as_ref() {
                return Err(ControlledRenderingError::AdjunctionChanged);
            }
            let candidate = adjunction.candidate();
            if candidate.binding() != schema.binding() {
                return Err(ControlledRenderingError::AdjunctionBindingMismatch {
                    expected: schema.binding(),
                    actual: candidate.binding(),
                });
            }
            if candidate.left().ty() != shape.open_port.ty()
                || candidate.right().ty() != shape.bound_port.ty()
            {
                return Err(ControlledRenderingError::AdjunctionPortTypeMismatch);
            }
            if !candidate.right().elements().contains(&shape.bound.value()) {
                return Err(ControlledRenderingError::AdjunctionBoundValueOutsideDomain(
                    shape.bound.value(),
                ));
            }
        }
        ControlledBackwardContract::SameUseReciprocalReturn {
            negation_use,
            relation_use,
            semantic_coverage,
            fiber,
        } => {
            let use_value = catalog.resolve_negation_use(*negation_use).ok_or(
                ControlledRenderingError::UnresolvedNegationUse(*negation_use),
            )?;
            let calculated = use_value.negation_use_ref()?;
            if calculated != *negation_use {
                return Err(ControlledRenderingError::NegationUseIdentityMismatch {
                    expected: *negation_use,
                    actual: calculated,
                });
            }
            use_value.check(catalog)?;
            if use_value.relation_use() != *relation_use {
                return Err(ControlledRenderingError::NegationRelationUseMismatch);
            }
            if use_value.semantic_coverage() != *semantic_coverage {
                return Err(ControlledRenderingError::NegationCoverageChanged);
            }
            let relation_use_value = catalog.resolve_relation_use(*relation_use).ok_or(
                ControlledRenderingError::UnresolvedRelationUse(*relation_use),
            )?;
            let calculated = relation_use_value.relation_use_ref()?;
            if calculated != *relation_use {
                return Err(ControlledRenderingError::RelationUseIdentityMismatch {
                    expected: *relation_use,
                    actual: calculated,
                });
            }
            relation_use_value.check(catalog)?;
            check_reciprocal_shape(query, &shape, &relation_use_value, *negation_use, fiber)?;
        }
    }
    Ok((query_ref, schema.binding()))
}

#[derive(Clone)]
struct BinaryBackwardShape {
    bound: PortBinding,
    bound_port: RelationPort,
    open: OpenPort,
    open_port: RelationPort,
}

impl BinaryBackwardShape {
    fn from_query(
        query: &OpenQuery,
        schema: &crate::RelationSchema,
    ) -> Result<Self, ControlledRenderingError> {
        if schema.ports().len() != 2
            || query.bound_ports().len() != 1
            || query.open_ports().len() != 1
        {
            return Err(ControlledRenderingError::NotBinaryBackwardQuestion {
                schema_ports: schema.ports().len(),
                bound_ports: query.bound_ports().len(),
                open_ports: query.open_ports().len(),
            });
        }
        let bound = query.bound_ports()[0].clone();
        let open = query.open_ports()[0].clone();
        let bound_port = schema
            .ports()
            .iter()
            .find(|port| port.name() == bound.port())
            .expect("checked query bound port must belong to its schema")
            .clone();
        let open_port = schema
            .ports()
            .iter()
            .find(|port| port.name() == open.port())
            .expect("checked query open port must belong to its schema")
            .clone();
        Ok(Self {
            bound,
            bound_port,
            open,
            open_port,
        })
    }
}

fn check_form<C: ControlledRenderingCatalog>(
    reference: TypedFormRef,
    expected_type: crate::TypeRef,
    expected_binding: BindingVersionRef,
    catalog: &C,
) -> Result<(), ControlledRenderingError> {
    let form = crate::FormulaCatalog::resolve_typed_form(catalog, reference)
        .ok_or(ControlledRenderingError::UnresolvedForm(reference))?;
    let calculated = form.typed_form_ref()?;
    if calculated != reference {
        return Err(ControlledRenderingError::FormIdentityMismatch {
            expected: reference,
            actual: calculated,
        });
    }
    form.check(catalog)?;
    if form.binding() != expected_binding {
        return Err(ControlledRenderingError::FormBindingMismatch {
            form: reference,
            expected: expected_binding,
            actual: form.binding(),
        });
    }
    if form.ty() != expected_type {
        return Err(ControlledRenderingError::FormTypeMismatch {
            form: reference,
            expected: expected_type,
            actual: form.ty(),
        });
    }
    Ok(())
}

fn check_reciprocal_shape(
    query: &OpenQuery,
    shape: &BinaryBackwardShape,
    relation_use: &RelationUse,
    negation_use: NegationUseRef,
    fiber: &ExactReturnFiber,
) -> Result<(), ControlledRenderingError> {
    if relation_use.relation() != query.relation() {
        return Err(ControlledRenderingError::ReciprocalRelationMismatch);
    }
    if relation_use.bindings().len() != 1 {
        return Err(ControlledRenderingError::ReciprocalUseIsNotUnaryBound(
            relation_use.bindings().len(),
        ));
    }
    let source = &relation_use.bindings()[0];
    if source.port() != shape.open.port() || shape.bound.port() == source.port() {
        return Err(ControlledRenderingError::ReciprocalOrientationMismatch);
    }
    if !fiber.sources().contains(&source.value().as_artifact_ref()) {
        return Err(ControlledRenderingError::ReciprocalSourceMissingFromFiber(
            source.value(),
        ));
    }
    if fiber.use_ref() != negation_use {
        return Err(ControlledRenderingError::ReciprocalFiberUseMismatch {
            expected: negation_use,
            actual: fiber.use_ref(),
        });
    }
    if fiber.exterior() != shape.bound.value().as_artifact_ref() {
        return Err(ControlledRenderingError::ReciprocalExteriorMismatch);
    }
    let context = query.context();
    if context.scope() != relation_use.scope()
        || context.applicability() != relation_use.applicability()
        || context.grain() != relation_use.grain()
        || context.horizon() != relation_use.horizon()
        || context.support() != relation_use.support()
        || context.warrant() != relation_use.warrant()
        || shape.open.mode() != relation_use.mode()
    {
        return Err(ControlledRenderingError::ReciprocalContextMismatch);
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum ControlledRenderingError {
    #[error(transparent)]
    QueryEncoding(#[from] OpenQueryError),
    #[error(transparent)]
    QueryCheck(#[from] OpenQueryCheckError),
    #[error(transparent)]
    RelationEncoding(#[from] RelationError),
    #[error(transparent)]
    RelationCheck(#[from] RelationCheckError),
    #[error(transparent)]
    RelationUseEncoding(#[from] RelationUseError),
    #[error(transparent)]
    RelationUseCheck(#[from] RelationUseCheckError),
    #[error(transparent)]
    NegationUseEncoding(#[from] NegationUseError),
    #[error(transparent)]
    NegationUseCheck(Box<NegationUseCheckError>),
    #[error(transparent)]
    Adjunction(#[from] FiniteAdjunctionError),
    #[error("return-fiber derivation failed: {0:?}")]
    Fiber(crate::ReturnFiberError),
    #[error(transparent)]
    TypeEncoding(#[from] TypeError),
    #[error(transparent)]
    TypeCheck(Box<TypeCheckError>),
    #[error("finite existential preimage repeats source {0}")]
    DuplicatePreimageSource(TypedFormRef),
    #[error("relation {0} is unavailable")]
    UnresolvedRelation(RelationRef),
    #[error("relation identity is {actual}, expected {expected}")]
    RelationIdentityMismatch {
        expected: RelationRef,
        actual: RelationRef,
    },
    #[error(
        "controlled backward question requires two schema ports, one bound port, and one open port; got {schema_ports}/{bound_ports}/{open_ports}"
    )]
    NotBinaryBackwardQuestion {
        schema_ports: usize,
        bound_ports: usize,
        open_ports: usize,
    },
    #[error("typed form {0} is unavailable")]
    UnresolvedForm(TypedFormRef),
    #[error("typed form identity is {actual}, expected {expected}")]
    FormIdentityMismatch {
        expected: TypedFormRef,
        actual: TypedFormRef,
    },
    #[error("typed form {form} has binding {actual}, expected {expected}")]
    FormBindingMismatch {
        form: TypedFormRef,
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },
    #[error("typed form {form} has type {actual}, expected {expected}")]
    FormTypeMismatch {
        form: TypedFormRef,
        expected: crate::TypeRef,
        actual: crate::TypeRef,
    },
    #[error("relation schema does not name adjunction law {0}")]
    AdjunctionLawNotNamed(ArtifactRef),
    #[error("admitted finite adjunction changed when rechecked")]
    AdjunctionChanged,
    #[error("adjunction binding is {actual}, expected {expected}")]
    AdjunctionBindingMismatch {
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },
    #[error("adjunction left/right types do not match the open/bound query ports")]
    AdjunctionPortTypeMismatch,
    #[error("query bound value {0} is outside the adjunction right domain")]
    AdjunctionBoundValueOutsideDomain(TypedFormRef),
    #[error("negation use {0} is unavailable")]
    UnresolvedNegationUse(NegationUseRef),
    #[error("negation-use identity is {actual}, expected {expected}")]
    NegationUseIdentityMismatch {
        expected: NegationUseRef,
        actual: NegationUseRef,
    },
    #[error("negation use does not retain the rendered relation-use identity")]
    NegationRelationUseMismatch,
    #[error("negation semantic coverage changed")]
    NegationCoverageChanged,
    #[error("relation use {0} is unavailable")]
    UnresolvedRelationUse(RelationUseRef),
    #[error("relation-use identity is {actual}, expected {expected}")]
    RelationUseIdentityMismatch {
        expected: RelationUseRef,
        actual: RelationUseRef,
    },
    #[error("reciprocal return query does not use the negation relation")]
    ReciprocalRelationMismatch,
    #[error("reciprocal relation use has {0} bound ports instead of one")]
    ReciprocalUseIsNotUnaryBound(usize),
    #[error("reciprocal query does not reverse the exact source/candidate port orientation")]
    ReciprocalOrientationMismatch,
    #[error("reciprocal source {0} is absent from the whole return fiber")]
    ReciprocalSourceMissingFromFiber(TypedFormRef),
    #[error("reciprocal fiber uses {actual}, expected {expected}")]
    ReciprocalFiberUseMismatch {
        expected: NegationUseRef,
        actual: NegationUseRef,
    },
    #[error("reciprocal query exterior differs from the return-fiber exterior")]
    ReciprocalExteriorMismatch,
    #[error("reciprocal query context differs from its exact relation use")]
    ReciprocalContextMismatch,
    #[error("renderer version is {actual}, expected {expected}")]
    RendererVersionMismatch {
        expected: ArtifactRef,
        actual: ArtifactRef,
    },
    #[error("controlled query is {actual}, expected {expected}")]
    QueryMismatch {
        expected: QueryRef,
        actual: QueryRef,
    },
    #[error("controlled contract is {actual:?}, expected {expected:?}")]
    ContractKindMismatch {
        expected: ControlledBackwardKind,
        actual: ControlledBackwardKind,
    },
    #[error("controlled coverage is {actual:?}, expected {expected:?}")]
    CoverageMismatch {
        expected: Box<ControlledCoverage>,
        actual: Box<ControlledCoverage>,
    },
    #[error("reciprocal use is {actual:?}, expected {expected:?}")]
    ReciprocalUseMismatch {
        expected: Option<NegationUseRef>,
        actual: Option<NegationUseRef>,
    },
    #[error("controlled surface text is not canonical for its root")]
    NonCanonicalSurfaceText,
    #[error("prompt query rehashes to {actual}, expected {expected}")]
    QueryIdentityChanged {
        expected: QueryRef,
        actual: QueryRef,
    },
    #[error("prompt binding is {actual}, expected {expected}")]
    BindingChanged {
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },
}

impl From<NegationUseCheckError> for ControlledRenderingError {
    fn from(error: NegationUseCheckError) -> Self {
        Self::NegationUseCheck(Box::new(error))
    }
}

impl From<TypeCheckError> for ControlledRenderingError {
    fn from(error: TypeCheckError) -> Self {
        Self::TypeCheck(Box::new(error))
    }
}

impl From<crate::ReturnFiberError> for ControlledRenderingError {
    fn from(error: crate::ReturnFiberError) -> Self {
        Self::Fiber(error)
    }
}

//! Exact finite sufficient-discriminator basis checking.
//!
//! This module checks the finite, total, deterministic instance of a sufficient discriminator
//! basis. It returns a concrete protected pair whenever the supplied basis fails to separate one.
//! The signature tables are caller-certified exact data; this module neither establishes that
//! certification, generate candidate bases, or claim the supplied set is exhaustive.

use std::collections::BTreeSet;

use thiserror::Error;

use crate::{
    ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, ClaimCheckError, ClaimError,
    ClaimRef, DischargeMode, ExactDeterminationError, ExactFiniteSignature, MethodContract,
    MethodContractCheckError, MethodContractError, MethodRef, RelationCheckError, RelationError,
    RelationRef, SignatureContext, Standing, SupportEnvironmentArtifactCheckError,
    SupportEnvironmentArtifactError, SupportEnvironmentCatalog, SupportEnvironmentRef,
    SupportSubjectRef, TypeCheckError, TypeError, TypeRef, TypeSymbol, TypedFormRef,
};

/// Canonical artifact kind for exact finite cue answer semantics.
pub const EXACT_FINITE_CUE_ARTIFACT_KIND: &str = "ic.exact-finite-cue";
/// Payload schema version for exact finite cue answer semantics.
pub const EXACT_FINITE_CUE_SCHEMA_VERSION: u32 = 1;

/// One exact finite answer table for a typed method used as a discriminator.
///
/// This is a declaration, not admission. The method provides the reusable typed discriminator;
/// the table identifies its exact answers over one finite protected field. Standing support and
/// coverage are established separately by a claim whose subject is this artifact identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFiniteCue {
    method: MethodRef,
    domain_port: TypeSymbol,
    answer_port: TypeSymbol,
    answer_type: TypeRef,
    signature: ExactFiniteSignature,
}

impl ExactFiniteCue {
    #[must_use]
    pub const fn new(
        method: MethodRef,
        domain_port: TypeSymbol,
        answer_port: TypeSymbol,
        answer_type: TypeRef,
        signature: ExactFiniteSignature,
    ) -> Self {
        Self {
            method,
            domain_port,
            answer_port,
            answer_type,
            signature,
        }
    }

    #[must_use]
    pub const fn method(&self) -> MethodRef {
        self.method
    }

    #[must_use]
    pub const fn domain_port(&self) -> &TypeSymbol {
        &self.domain_port
    }

    #[must_use]
    pub const fn answer_port(&self) -> &TypeSymbol {
        &self.answer_port
    }

    #[must_use]
    pub const fn answer_type(&self) -> TypeRef {
        self.answer_type
    }

    #[must_use]
    pub const fn signature(&self) -> &ExactFiniteSignature {
        &self.signature
    }

    pub fn canonical_payload(&self) -> Result<Vec<u8>, ExactFiniteCueError> {
        let mut encoded = Vec::new();
        cue_reference(&mut encoded, self.method.as_artifact_ref());
        let context = self.signature.context();
        for dependency in [
            context.binding().as_artifact_ref(),
            context.scope().as_artifact_ref(),
            context.applicability().as_artifact_ref(),
            context.grain().as_artifact_ref(),
            context.horizon().as_artifact_ref(),
            context.domain().as_artifact_ref(),
            self.answer_type.as_artifact_ref(),
        ] {
            cue_reference(&mut encoded, dependency);
        }
        cue_text(&mut encoded, self.domain_port.as_str())?;
        cue_text(&mut encoded, self.answer_port.as_str())?;
        cue_count(&mut encoded, self.signature.values().len())?;
        for (domain, answer) in self.signature.values() {
            cue_reference(&mut encoded, *domain);
            cue_reference(&mut encoded, *answer);
        }
        Ok(encoded)
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, ExactFiniteCueError> {
        let mut cursor = CueCursor::new(payload);
        let method = MethodRef::from_artifact_ref(cursor.reference()?);
        let context = SignatureContext::new(
            crate::BindingVersionRef::from_artifact_ref(cursor.reference()?),
            crate::ScopeRef::from_artifact_ref(cursor.reference()?),
            crate::ApplicabilityRef::from_artifact_ref(cursor.reference()?),
            crate::GrainRef::from_artifact_ref(cursor.reference()?),
            crate::HorizonRef::from_artifact_ref(cursor.reference()?),
            TypeRef::from_artifact_ref(cursor.reference()?),
        );
        let answer_type = TypeRef::from_artifact_ref(cursor.reference()?);
        let domain_port =
            TypeSymbol::new(cursor.text()?).map_err(|_| ExactFiniteCueError::InvalidPortName)?;
        let answer_port =
            TypeSymbol::new(cursor.text()?).map_err(|_| ExactFiniteCueError::InvalidPortName)?;
        let entry_count = cursor.count()?;
        let mut entries = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            entries.push((cursor.reference()?, cursor.reference()?));
        }
        if !cursor.finished() {
            return Err(ExactFiniteCueError::TrailingPayloadBytes(
                cursor.remaining(),
            ));
        }
        let signature = ExactFiniteSignature::new(context, entries)?;
        Ok(Self::new(
            method,
            domain_port,
            answer_port,
            answer_type,
            signature,
        ))
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, ExactFiniteCueError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(EXACT_FINITE_CUE_ARTIFACT_KIND)?,
            EXACT_FINITE_CUE_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }

    pub fn artifact_ref(&self) -> Result<ArtifactRef, ExactFiniteCueError> {
        Ok(self.envelope()?.artifact_ref()?)
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, ExactFiniteCueError> {
        if envelope.kind().as_str() != EXACT_FINITE_CUE_ARTIFACT_KIND {
            return Err(ExactFiniteCueError::UnexpectedArtifactKind {
                expected: EXACT_FINITE_CUE_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != EXACT_FINITE_CUE_SCHEMA_VERSION {
            return Err(ExactFiniteCueError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let context = self.signature.context();
        let mut references = vec![
            self.method.as_artifact_ref(),
            context.binding().as_artifact_ref(),
            context.scope().as_artifact_ref(),
            context.applicability().as_artifact_ref(),
            context.grain().as_artifact_ref(),
            context.horizon().as_artifact_ref(),
            context.domain().as_artifact_ref(),
            self.answer_type.as_artifact_ref(),
        ];
        for (domain, answer) in self.signature.values() {
            references.push(*domain);
            references.push(*answer);
        }
        references
    }
}

/// Catalog boundary for exact finite cue admission.
pub trait ExactFiniteCueCatalog: SupportEnvironmentCatalog {
    fn resolve_method(&self, reference: MethodRef) -> Option<MethodContract>;
}

/// A fully checked exact cue with both reusable-relation and answer-semantics support routes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedExactFiniteCue {
    cue: ExactFiniteCue,
    coverage_claim: ClaimRef,
    cue_support: SupportEnvironmentRef,
    relation_support: SupportEnvironmentRef,
}

impl AdmittedExactFiniteCue {
    #[must_use]
    pub const fn cue(&self) -> &ExactFiniteCue {
        &self.cue
    }

    #[must_use]
    pub const fn signature(&self) -> &ExactFiniteSignature {
        self.cue.signature()
    }

    #[must_use]
    pub const fn coverage_claim(&self) -> ClaimRef {
        self.coverage_claim
    }

    #[must_use]
    pub const fn cue_support(&self) -> SupportEnvironmentRef {
        self.cue_support
    }

    #[must_use]
    pub const fn relation_support(&self) -> SupportEnvironmentRef {
        self.relation_support
    }
}

/// Admission result preserving incomplete support or execution evidence as `Unknown`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExactFiniteCueAdmission {
    Admitted(Box<AdmittedExactFiniteCue>),
    Unknown {
        cue: ArtifactRef,
        residual: ExactFiniteCueUnknown,
    },
}

/// Positive descriptions of the missing evidence that prevents exact cue admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExactFiniteCueUnknown {
    RelationSupportIncomplete,
    CueCoverageSupportIncomplete,
    GeneratedAnswerSemantics,
    MissingProbeProvenance,
}

/// Checks and admits one exact cue through its exact claim- and relation-targeted standing routes.
///
/// The coverage claim must name the cue artifact as its subject and originate from the method's
/// relation. The method's relation and the cue claim must each close through the exact supplied
/// environment. A generated method never supplies admitted exact answer semantics, and a probe
/// method requires preserved raw-return and resolution-path provenance. Missing standing,
/// closure, or probe provenance remains `Unknown` rather than negative evidence.
pub fn admit_exact_finite_cue<C: ExactFiniteCueCatalog>(
    cue: ExactFiniteCue,
    coverage_claim: ClaimRef,
    cue_support: SupportEnvironmentRef,
    relation_support: SupportEnvironmentRef,
    standing: &Standing,
    catalog: &C,
) -> Result<ExactFiniteCueAdmission, ExactFiniteCueCheckError> {
    let cue_ref = cue.artifact_ref()?;
    let method = check_exact_finite_cue_structure(&cue, catalog)?;

    let claim = catalog.resolve_claim(coverage_claim).ok_or(
        ExactFiniteCueCheckError::UnresolvedCoverageClaim(coverage_claim),
    )?;
    let calculated_claim = claim.claim_ref()?;
    if calculated_claim != coverage_claim {
        return Err(ExactFiniteCueCheckError::CoverageClaimIdentityMismatch {
            reference: coverage_claim,
            calculated: calculated_claim,
        });
    }
    claim.check(catalog)?;
    if claim.subject() != cue_ref {
        return Err(ExactFiniteCueCheckError::CoverageClaimSubjectMismatch {
            cue: cue_ref,
            subject: claim.subject(),
        });
    }
    let source_query = catalog.resolve_open_query(claim.source_question()).ok_or(
        ExactFiniteCueCheckError::UnresolvedSourceQuestion(claim.source_question()),
    )?;
    if source_query.relation() != method.relation() {
        return Err(ExactFiniteCueCheckError::SourceRelationMismatch {
            method: method.relation(),
            claim: source_query.relation(),
        });
    }
    let context = cue.signature.context();
    let source_context = source_query.context();
    if source_context.scope() != context.scope() {
        return Err(ExactFiniteCueCheckError::ContextMismatch(
            "source question scope",
        ));
    }
    if source_context.applicability() != context.applicability() {
        return Err(ExactFiniteCueCheckError::ContextMismatch(
            "source question applicability",
        ));
    }
    if source_context.grain() != context.grain() {
        return Err(ExactFiniteCueCheckError::ContextMismatch(
            "source question grain",
        ));
    }
    if source_context.horizon() != context.horizon() {
        return Err(ExactFiniteCueCheckError::ContextMismatch(
            "source question horizon",
        ));
    }
    for port in [&cue.domain_port, &cue.answer_port] {
        source_query
            .open_ports()
            .iter()
            .find(|open| open.port() == port)
            .ok_or_else(|| {
                ExactFiniteCueCheckError::CoverageSourcePortNotOpen(port.as_str().to_owned())
            })?;
    }
    if claim.scope() != context.scope() {
        return Err(ExactFiniteCueCheckError::ContextMismatch("claim scope"));
    }
    if claim.applicability() != context.applicability() {
        return Err(ExactFiniteCueCheckError::ContextMismatch(
            "claim applicability",
        ));
    }

    check_support_environment(
        cue_support,
        SupportSubjectRef::Claim(coverage_claim),
        context,
        catalog,
    )?;
    check_support_environment(
        relation_support,
        SupportSubjectRef::Relation(method.relation()),
        context,
        catalog,
    )?;

    if !standing.contains_relation(method.relation())
        || !standing.closes_through(
            SupportSubjectRef::Relation(method.relation()),
            relation_support,
        )
    {
        return Ok(ExactFiniteCueAdmission::Unknown {
            cue: cue_ref,
            residual: ExactFiniteCueUnknown::RelationSupportIncomplete,
        });
    }
    if !standing.contains(coverage_claim)
        || !standing.closes_through(SupportSubjectRef::Claim(coverage_claim), cue_support)
    {
        return Ok(ExactFiniteCueAdmission::Unknown {
            cue: cue_ref,
            residual: ExactFiniteCueUnknown::CueCoverageSupportIncomplete,
        });
    }
    if method.authority() == DischargeMode::Generate {
        return Ok(ExactFiniteCueAdmission::Unknown {
            cue: cue_ref,
            residual: ExactFiniteCueUnknown::GeneratedAnswerSemantics,
        });
    }
    if method.authority() == DischargeMode::Probe
        && (claim.supporting_returns().is_empty() || claim.resolution_paths().is_empty())
    {
        return Ok(ExactFiniteCueAdmission::Unknown {
            cue: cue_ref,
            residual: ExactFiniteCueUnknown::MissingProbeProvenance,
        });
    }
    Ok(ExactFiniteCueAdmission::Admitted(Box::new(
        AdmittedExactFiniteCue {
            cue,
            coverage_claim,
            cue_support,
            relation_support,
        },
    )))
}

/// Applies the exact sufficient-basis theorem only to already admitted cues.
pub fn check_admitted_exact_finite_cue_basis(
    cues: &[AdmittedExactFiniteCue],
    protected: &ExactFiniteSignature,
) -> Result<ExactFiniteCueBasisResult, ExactFiniteCueBasisError> {
    let signatures: Vec<_> = cues.iter().map(|cue| cue.signature().clone()).collect();
    check_exact_finite_cue_basis(&signatures, protected)
}

/// Selects a finite nondominated frontier only from already admitted exact cues.
pub fn select_nondominated_admitted_exact_finite_cue_bases(
    cues: &[AdmittedExactFiniteCue],
    protected: &ExactFiniteSignature,
    candidates: &[ExactFiniteCueBasisCandidate],
    resources: &FiniteResourcePreorder,
) -> Result<ExactFiniteCueFrontier, ExactFiniteCueFrontierError> {
    let signatures: Vec<_> = cues.iter().map(|cue| cue.signature().clone()).collect();
    select_nondominated_exact_finite_cue_bases(&signatures, protected, candidates, resources)
}

fn check_exact_finite_cue_structure<C: ExactFiniteCueCatalog>(
    cue: &ExactFiniteCue,
    catalog: &C,
) -> Result<MethodContract, ExactFiniteCueCheckError> {
    if cue.domain_port == cue.answer_port {
        return Err(ExactFiniteCueCheckError::IdenticalPorts);
    }
    let method = catalog
        .resolve_method(cue.method)
        .ok_or(ExactFiniteCueCheckError::UnresolvedMethod(cue.method))?;
    let calculated_method = method.method_ref()?;
    if calculated_method != cue.method {
        return Err(ExactFiniteCueCheckError::MethodIdentityMismatch {
            reference: cue.method,
            calculated: calculated_method,
        });
    }
    method.check(catalog)?;
    let relation = catalog.resolve_relation_schema(method.relation()).ok_or(
        ExactFiniteCueCheckError::UnresolvedRelation(method.relation()),
    )?;
    let context = cue.signature.context();
    if relation.binding() != context.binding() {
        return Err(ExactFiniteCueCheckError::BindingMismatch);
    }
    if method.applicability() != context.applicability() {
        return Err(ExactFiniteCueCheckError::ContextMismatch(
            "method applicability",
        ));
    }
    let domain_type = relation
        .ports()
        .iter()
        .find(|port| port.name() == &cue.domain_port)
        .map(|port| port.ty())
        .ok_or_else(|| {
            ExactFiniteCueCheckError::UnknownPort(cue.domain_port.as_str().to_owned())
        })?;
    let answer_type = relation
        .ports()
        .iter()
        .find(|port| port.name() == &cue.answer_port)
        .map(|port| port.ty())
        .ok_or_else(|| {
            ExactFiniteCueCheckError::UnknownPort(cue.answer_port.as_str().to_owned())
        })?;
    if domain_type != context.domain() {
        return Err(ExactFiniteCueCheckError::PortTypeMismatch {
            port: cue.domain_port.as_str().to_owned(),
            expected: domain_type,
            actual: context.domain(),
        });
    }
    if answer_type != cue.answer_type {
        return Err(ExactFiniteCueCheckError::PortTypeMismatch {
            port: cue.answer_port.as_str().to_owned(),
            expected: answer_type,
            actual: cue.answer_type,
        });
    }
    for (domain, answer) in cue.signature.values() {
        check_cue_form(*domain, domain_type, catalog)?;
        check_cue_form(*answer, answer_type, catalog)?;
    }
    Ok(method)
}

fn check_cue_form<C: ExactFiniteCueCatalog>(
    reference: ArtifactRef,
    expected: TypeRef,
    catalog: &C,
) -> Result<(), ExactFiniteCueCheckError> {
    let typed_ref = TypedFormRef::from_artifact_ref(reference);
    let form = catalog
        .resolve_typed_form(typed_ref)
        .ok_or(ExactFiniteCueCheckError::UnresolvedTypedForm(typed_ref))?;
    let calculated = form.typed_form_ref()?;
    if calculated != typed_ref {
        return Err(ExactFiniteCueCheckError::TypedFormIdentityMismatch {
            reference: typed_ref,
            calculated,
        });
    }
    form.check(catalog)?;
    if form.ty() != expected {
        return Err(ExactFiniteCueCheckError::TypedFormTypeMismatch {
            reference: typed_ref,
            expected,
            actual: form.ty(),
        });
    }
    Ok(())
}

fn check_support_environment<C: ExactFiniteCueCatalog>(
    reference: SupportEnvironmentRef,
    expected_target: SupportSubjectRef,
    context: SignatureContext,
    catalog: &C,
) -> Result<(), ExactFiniteCueCheckError> {
    let environment = catalog.resolve_support_environment(reference).ok_or(
        ExactFiniteCueCheckError::UnresolvedSupportEnvironment(reference),
    )?;
    let calculated = environment.support_environment_ref()?;
    if calculated != reference {
        return Err(
            ExactFiniteCueCheckError::SupportEnvironmentIdentityMismatch {
                reference,
                calculated,
            },
        );
    }
    environment.check(catalog)?;
    if environment.target() != expected_target {
        return Err(ExactFiniteCueCheckError::SupportTargetMismatch);
    }
    if environment.scope() != context.scope() {
        return Err(ExactFiniteCueCheckError::ContextMismatch("support scope"));
    }
    if environment.applicability() != context.applicability() {
        return Err(ExactFiniteCueCheckError::ContextMismatch(
            "support applicability",
        ));
    }
    Ok(())
}

/// A concrete protectedly distinct pair that every supplied cue answers identically.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteCueSeparator {
    first_domain: ArtifactRef,
    second_domain: ArtifactRef,
    first_protected_value: ArtifactRef,
    second_protected_value: ArtifactRef,
    cue_answers: Vec<ArtifactRef>,
}

impl FiniteCueSeparator {
    /// Returns the first live residual value.
    #[must_use]
    pub const fn first_domain(&self) -> ArtifactRef {
        self.first_domain
    }

    /// Returns the second live residual value.
    #[must_use]
    pub const fn second_domain(&self) -> ArtifactRef {
        self.second_domain
    }

    /// Returns the protected answer of the first value.
    #[must_use]
    pub const fn first_protected_value(&self) -> ArtifactRef {
        self.first_protected_value
    }

    /// Returns the protected answer of the second value.
    #[must_use]
    pub const fn second_protected_value(&self) -> ArtifactRef {
        self.second_protected_value
    }

    /// Returns the common answer from every cue, in declared cue order.
    #[must_use]
    pub fn cue_answers(&self) -> &[ArtifactRef] {
        &self.cue_answers
    }
}

/// Result of checking one declared exact finite discriminator basis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExactFiniteCueBasisResult {
    /// Every protectedly distinct pair is separated by at least one supplied cue.
    Sufficient,
    /// A protectedly distinct pair remains indistinguishable to every supplied cue.
    Insufficient { separator: FiniteCueSeparator },
}

/// Checks finite discriminator-basis sufficiency under caller-certified exact signatures.
///
/// `protected` partitions the residual domain at the declared horizon. Every cue must share its
/// binding, scope, applicability, grain, horizon, domain type, and exact finite domain with it.
/// An empty basis is valid precisely when the protected signature is constant. The result is only
/// about the supplied exact tables: it does not establish support, applicability, coverage,
/// resource minimality, an optimal query policy, or a broader impossibility claim.
pub fn check_exact_finite_cue_basis(
    cues: &[ExactFiniteSignature],
    protected: &ExactFiniteSignature,
) -> Result<ExactFiniteCueBasisResult, ExactFiniteCueBasisError> {
    let protected_domain: BTreeSet<_> = protected.values().keys().copied().collect();
    for (index, cue) in cues.iter().enumerate() {
        if cue.context() != protected.context() {
            return Err(ExactFiniteCueBasisError::ContextMismatch {
                cue_index: index,
                expected: Box::new(protected.context()),
                actual: Box::new(cue.context()),
            });
        }
        let cue_domain: BTreeSet<_> = cue.values().keys().copied().collect();
        if cue_domain != protected_domain {
            return Err(ExactFiniteCueBasisError::DomainMismatch { cue_index: index });
        }
    }

    let entries: Vec<_> = protected.values().iter().collect();
    for (first_index, (first_domain, first_protected_value)) in entries.iter().enumerate() {
        for (second_domain, second_protected_value) in entries.iter().skip(first_index + 1) {
            if first_protected_value == second_protected_value {
                continue;
            }
            let mut answers = Vec::with_capacity(cues.len());
            let mut separated = false;
            for cue in cues {
                let first_answer = cue.values()[first_domain];
                let second_answer = cue.values()[second_domain];
                answers.push(first_answer);
                if first_answer != second_answer {
                    separated = true;
                    break;
                }
            }
            if !separated {
                return Ok(ExactFiniteCueBasisResult::Insufficient {
                    separator: FiniteCueSeparator {
                        first_domain: **first_domain,
                        second_domain: **second_domain,
                        first_protected_value: **first_protected_value,
                        second_protected_value: **second_protected_value,
                        cue_answers: answers,
                    },
                });
            }
        }
    }
    Ok(ExactFiniteCueBasisResult::Sufficient)
}

/// Errors from comparing the declared exact signature contexts of a cue basis.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ExactFiniteCueBasisError {
    #[error("cue {cue_index} has a different exact signature context")]
    ContextMismatch {
        /// Position of the mismatched cue in the declared basis.
        cue_index: usize,
        /// Context required by the protected signature.
        expected: Box<SignatureContext>,
        /// Context carried by the mismatched cue.
        actual: Box<SignatureContext>,
    },

    #[error("cue {cue_index} has a different exact finite domain")]
    DomainMismatch {
        /// Position of the mismatched cue in the declared basis.
        cue_index: usize,
    },
}

/// One caller-supplied candidate basis, addressed by indices in the declared cue sequence.
///
/// The resource identity is opaque and receives its order only from a separately declared
/// [`FiniteResourcePreorder`]. The candidate order is intentionally not an enumeration claim.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ExactFiniteCueBasisCandidate {
    cue_indices: Vec<usize>,
    resource: ArtifactRef,
}

impl ExactFiniteCueBasisCandidate {
    /// Creates a candidate basis with strictly increasing declared cue indices.
    pub fn new(
        cue_indices: Vec<usize>,
        resource: ArtifactRef,
    ) -> Result<Self, ExactFiniteCueFrontierError> {
        if cue_indices.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(ExactFiniteCueFrontierError::NonCanonicalCueIndices);
        }
        Ok(Self {
            cue_indices,
            resource,
        })
    }

    /// Returns indices into the declared cue sequence.
    #[must_use]
    pub fn cue_indices(&self) -> &[usize] {
        &self.cue_indices
    }

    /// Returns the binding-supplied resource identity for this candidate.
    #[must_use]
    pub const fn resource(&self) -> ArtifactRef {
        self.resource
    }
}

/// A finite declared preorder over opaque resource identities.
///
/// Every relation retained here is an assertion by the caller's binding. Construction rejects
/// duplicate edges; checking the order against a finite resource set requires reflexivity and
/// transitivity over exactly that supplied set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiniteResourcePreorder {
    less_or_equal: BTreeSet<(ArtifactRef, ArtifactRef)>,
}

impl FiniteResourcePreorder {
    /// Constructs one finite declared preorder relation.
    pub fn new(
        less_or_equal: Vec<(ArtifactRef, ArtifactRef)>,
    ) -> Result<Self, ExactFiniteCueFrontierError> {
        let mut edges = BTreeSet::new();
        for edge in less_or_equal {
            if !edges.insert(edge) {
                return Err(ExactFiniteCueFrontierError::DuplicateResourceOrderEdge {
                    lower: edge.0,
                    upper: edge.1,
                });
            }
        }
        Ok(Self {
            less_or_equal: edges,
        })
    }

    /// Returns the explicitly declared resource-order edges.
    #[must_use]
    pub const fn less_or_equal(&self) -> &BTreeSet<(ArtifactRef, ArtifactRef)> {
        &self.less_or_equal
    }

    fn has_edge(&self, lower: ArtifactRef, upper: ArtifactRef) -> bool {
        self.less_or_equal.contains(&(lower, upper))
    }

    fn check_over(
        &self,
        resources: &BTreeSet<ArtifactRef>,
    ) -> Result<(), ExactFiniteCueFrontierError> {
        for resource in resources {
            if !self.has_edge(*resource, *resource) {
                return Err(ExactFiniteCueFrontierError::NonReflexiveResource(*resource));
            }
        }
        for lower in resources {
            for middle in resources {
                for upper in resources {
                    if self.has_edge(*lower, *middle)
                        && self.has_edge(*middle, *upper)
                        && !self.has_edge(*lower, *upper)
                    {
                        return Err(ExactFiniteCueFrontierError::NonTransitiveResourceOrder {
                            lower: *lower,
                            middle: *middle,
                            upper: *upper,
                        });
                    }
                }
            }
        }
        Ok(())
    }
}

/// An exact candidate rejected because it fails the declared sufficient-basis condition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InsufficientExactFiniteCueBasis {
    candidate: ExactFiniteCueBasisCandidate,
    separator: FiniteCueSeparator,
}

impl InsufficientExactFiniteCueBasis {
    /// Returns the candidate basis that failed.
    #[must_use]
    pub const fn candidate(&self) -> &ExactFiniteCueBasisCandidate {
        &self.candidate
    }

    /// Returns the concrete protected separator retained from that failure.
    #[must_use]
    pub const fn separator(&self) -> &FiniteCueSeparator {
        &self.separator
    }
}

/// Nondominated sufficient candidates from one finite, caller-supplied candidate set.
///
/// This is not a proof that every possible cue basis was supplied. `members` are minimal only
/// relative to the supplied candidates and their declared preorder; `insufficient` retains
/// positive residual separators rather than treating them as an impossibility result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFiniteCueFrontier {
    members: Vec<ExactFiniteCueBasisCandidate>,
    insufficient: Vec<InsufficientExactFiniteCueBasis>,
}

impl ExactFiniteCueFrontier {
    /// Returns all nondominated sufficient candidates in the caller's original order.
    #[must_use]
    pub fn members(&self) -> &[ExactFiniteCueBasisCandidate] {
        &self.members
    }

    /// Returns failed candidates with their concrete protected separators.
    #[must_use]
    pub fn insufficient(&self) -> &[InsufficientExactFiniteCueBasis] {
        &self.insufficient
    }
}

/// Selects nondominated sufficient bases from one finite, caller-supplied candidate set.
///
/// Every candidate is first checked by [`check_exact_finite_cue_basis`]. The order is validated
/// over all candidate resource identities, then a candidate is removed only when another
/// sufficient candidate is strictly lower (`other <= candidate` but not conversely). This does
/// not generate candidates, establish that the input set is exhaustive, certify resource facts,
/// or convert a frontier with no sufficient member into impossibility.
pub fn select_nondominated_exact_finite_cue_bases(
    cues: &[ExactFiniteSignature],
    protected: &ExactFiniteSignature,
    candidates: &[ExactFiniteCueBasisCandidate],
    resources: &FiniteResourcePreorder,
) -> Result<ExactFiniteCueFrontier, ExactFiniteCueFrontierError> {
    let mut seen = BTreeSet::new();
    let mut resource_set = BTreeSet::new();
    for candidate in candidates {
        if !seen.insert(candidate.clone()) {
            return Err(ExactFiniteCueFrontierError::DuplicateCandidate(
                candidate.clone(),
            ));
        }
        for index in candidate.cue_indices() {
            if *index >= cues.len() {
                return Err(ExactFiniteCueFrontierError::CueIndexOutOfRange {
                    index: *index,
                    cue_count: cues.len(),
                });
            }
        }
        resource_set.insert(candidate.resource());
    }
    resources.check_over(&resource_set)?;

    let mut sufficient = Vec::new();
    let mut insufficient = Vec::new();
    for candidate in candidates {
        let selected: Vec<_> = candidate
            .cue_indices()
            .iter()
            .map(|index| cues[*index].clone())
            .collect();
        match check_exact_finite_cue_basis(&selected, protected)? {
            ExactFiniteCueBasisResult::Sufficient => sufficient.push(candidate.clone()),
            ExactFiniteCueBasisResult::Insufficient { separator } => {
                insufficient.push(InsufficientExactFiniteCueBasis {
                    candidate: candidate.clone(),
                    separator,
                });
            }
        }
    }

    let members = sufficient
        .iter()
        .filter(|candidate| {
            !sufficient.iter().any(|other| {
                other != *candidate
                    && resources.has_edge(other.resource(), candidate.resource())
                    && !resources.has_edge(candidate.resource(), other.resource())
            })
        })
        .cloned()
        .collect();
    Ok(ExactFiniteCueFrontier {
        members,
        insufficient,
    })
}

/// Errors from finite candidate-basis frontier selection.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ExactFiniteCueFrontierError {
    #[error("candidate cue indices must be strictly increasing without duplicates")]
    NonCanonicalCueIndices,

    #[error("resource preorder repeats edge {lower} <= {upper}")]
    DuplicateResourceOrderEdge {
        /// Lower resource identity in the repeated edge.
        lower: ArtifactRef,
        /// Upper resource identity in the repeated edge.
        upper: ArtifactRef,
    },

    #[error("candidate repeats an identical cue basis and resource")]
    DuplicateCandidate(ExactFiniteCueBasisCandidate),

    #[error("candidate names cue index {index}, but only {cue_count} cues are declared")]
    CueIndexOutOfRange {
        /// Out-of-range cue index.
        index: usize,
        /// Declared cue count.
        cue_count: usize,
    },

    #[error("resource preorder is missing reflexive edge {0} <= {0}")]
    NonReflexiveResource(ArtifactRef),

    #[error("resource preorder is not transitive: {lower} <= {middle} <= {upper}")]
    NonTransitiveResourceOrder {
        /// Lower resource identity.
        lower: ArtifactRef,
        /// Middle resource identity.
        middle: ArtifactRef,
        /// Upper resource identity.
        upper: ArtifactRef,
    },

    #[error(transparent)]
    CueBasis(#[from] ExactFiniteCueBasisError),
}

/// Canonical encoding failures for exact finite cue declarations.
#[derive(Debug, Error)]
pub enum ExactFiniteCueError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error(transparent)]
    Signature(#[from] ExactDeterminationError),
    #[error("exact-finite-cue payload is truncated")]
    TruncatedPayload,
    #[error("exact-finite-cue payload length overflows")]
    PayloadLengthOverflow,
    #[error("exact-finite-cue payload has {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("exact-finite-cue text is malformed UTF-8")]
    MalformedUtf8,
    #[error("exact-finite-cue port name is invalid")]
    InvalidPortName,
    #[error("exact-finite-cue text is too long: {0} bytes")]
    TextTooLong(usize),
    #[error("exact-finite-cue table is too large: {0} rows")]
    TooManyEntries(usize),
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported exact-finite-cue schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

/// Structural and standing-admission failures for exact finite cues.
#[derive(Debug, Error)]
pub enum ExactFiniteCueCheckError {
    #[error(transparent)]
    Cue(#[from] ExactFiniteCueError),
    #[error(transparent)]
    Method(#[from] MethodContractError),
    #[error(transparent)]
    MethodCheck(#[from] MethodContractCheckError),
    #[error(transparent)]
    Claim(#[from] ClaimError),
    #[error(transparent)]
    ClaimCheck(#[from] ClaimCheckError),
    #[error(transparent)]
    Support(#[from] SupportEnvironmentArtifactError),
    #[error(transparent)]
    SupportCheck(#[from] SupportEnvironmentArtifactCheckError),
    #[error(transparent)]
    Relation(#[from] RelationError),
    #[error(transparent)]
    RelationCheck(#[from] RelationCheckError),
    #[error(transparent)]
    Type(#[from] TypeError),
    #[error(transparent)]
    TypeCheck(#[from] TypeCheckError),
    #[error("method {0} is unavailable")]
    UnresolvedMethod(MethodRef),
    #[error("method {reference} hashes to {calculated}, not its claimed identity")]
    MethodIdentityMismatch {
        reference: MethodRef,
        calculated: MethodRef,
    },
    #[error("method relation {0} is unavailable")]
    UnresolvedRelation(RelationRef),
    #[error("cue binding does not match the method relation binding")]
    BindingMismatch,
    #[error("cue uses the same relation port for domain and answer")]
    IdenticalPorts,
    #[error("cue relation has no port named {0:?}")]
    UnknownPort(String),
    #[error("cue port {port:?} has type {actual}, expected {expected}")]
    PortTypeMismatch {
        port: String,
        expected: TypeRef,
        actual: TypeRef,
    },
    #[error("typed cue value {0} is unavailable")]
    UnresolvedTypedForm(TypedFormRef),
    #[error("typed cue value {reference} hashes to {calculated}")]
    TypedFormIdentityMismatch {
        reference: TypedFormRef,
        calculated: TypedFormRef,
    },
    #[error("typed cue value {reference} has type {actual}, expected {expected}")]
    TypedFormTypeMismatch {
        reference: TypedFormRef,
        expected: TypeRef,
        actual: TypeRef,
    },
    #[error("coverage claim {0} is unavailable")]
    UnresolvedCoverageClaim(ClaimRef),
    #[error("coverage claim {reference} hashes to {calculated}")]
    CoverageClaimIdentityMismatch {
        reference: ClaimRef,
        calculated: ClaimRef,
    },
    #[error("coverage claim subject {subject} does not name exact cue {cue}")]
    CoverageClaimSubjectMismatch {
        cue: ArtifactRef,
        subject: ArtifactRef,
    },
    #[error("coverage claim source question {0} is unavailable")]
    UnresolvedSourceQuestion(crate::QueryRef),
    #[error("coverage claim relation {claim} does not match method relation {method}")]
    SourceRelationMismatch {
        method: RelationRef,
        claim: RelationRef,
    },
    #[error("coverage source question does not leave cue port {0:?} open")]
    CoverageSourcePortNotOpen(String),
    #[error("support environment {0} is unavailable")]
    UnresolvedSupportEnvironment(SupportEnvironmentRef),
    #[error("support environment {reference} hashes to {calculated}")]
    SupportEnvironmentIdentityMismatch {
        reference: SupportEnvironmentRef,
        calculated: SupportEnvironmentRef,
    },
    #[error("support environment targets the wrong standing subject")]
    SupportTargetMismatch,
    #[error("cue context mismatch: {0}")]
    ContextMismatch(&'static str),
}

fn cue_reference(encoded: &mut Vec<u8>, reference: ArtifactRef) {
    encoded.extend_from_slice(reference.as_bytes());
}

fn cue_text(encoded: &mut Vec<u8>, value: &str) -> Result<(), ExactFiniteCueError> {
    let length =
        u32::try_from(value.len()).map_err(|_| ExactFiniteCueError::TextTooLong(value.len()))?;
    encoded.extend_from_slice(&length.to_be_bytes());
    encoded.extend_from_slice(value.as_bytes());
    Ok(())
}

fn cue_count(encoded: &mut Vec<u8>, value: usize) -> Result<(), ExactFiniteCueError> {
    let count = u32::try_from(value).map_err(|_| ExactFiniteCueError::TooManyEntries(value))?;
    encoded.extend_from_slice(&count.to_be_bytes());
    Ok(())
}

struct CueCursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> CueCursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], ExactFiniteCueError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(ExactFiniteCueError::PayloadLengthOverflow)?;
        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(ExactFiniteCueError::TruncatedPayload)?;
        self.position = end;
        Ok(bytes)
    }

    fn reference(&mut self) -> Result<ArtifactRef, ExactFiniteCueError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .expect("exact 32-byte cue reference slice");
        Ok(ArtifactRef::from_bytes(bytes))
    }

    fn count(&mut self) -> Result<usize, ExactFiniteCueError> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .expect("exact four-byte cue count slice");
        Ok(u32::from_be_bytes(bytes) as usize)
    }

    fn text(&mut self) -> Result<String, ExactFiniteCueError> {
        let length = self.count()?;
        let bytes = self.take(length)?;
        let text = std::str::from_utf8(bytes).map_err(|_| ExactFiniteCueError::MalformedUtf8)?;
        Ok(text.to_owned())
    }

    const fn finished(&self) -> bool {
        self.position == self.bytes.len()
    }

    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

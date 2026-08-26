//! Canonical generic separator-problem artifacts.
//!
//! A separator problem preserves one unresolved protected completion field and the declared
//! resources through which later phases may try to distinguish it.  It is deliberately not a
//! generator, policy, answer, or representation-gap verdict: those require their own admitted
//! evaluators and evidence routes.

use std::{collections::BTreeSet, fmt, str::FromStr};

use thiserror::Error;

use crate::{
    ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef, BindingVersionRef,
    ExactFiniteCueBasisError, ExactFiniteCueBasisResult, ExactFiniteSignature, FiniteCueSeparator,
    GrainRef, HorizonRef, OpenQueryCatalog, OpenQueryCheckError, OpenQueryError, QueryRef,
    RelationRef, check_exact_finite_cue_basis,
};

/// Canonical artifact kind for generic protected residual/separator problems.
pub const SEPARATOR_PROBLEM_ARTIFACT_KIND: &str = "ic.separator-problem";
/// Payload schema version for generic protected residual/separator problems.
pub const SEPARATOR_PROBLEM_SCHEMA_VERSION: u32 = 1;
/// Canonical artifact kind for generated-but-unselected separator inquiries.
pub const GENERATED_INQUIRY_ARTIFACT_KIND: &str = "ic.generated-inquiry";
/// Payload schema version for generated inquiry artifacts.
pub const GENERATED_INQUIRY_SCHEMA_VERSION: u32 = 1;

macro_rules! artifact_reference {
    ($name:ident) => {
        /// Opaque identity whose semantics belong to the later named phase.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(ArtifactRef);

        impl $name {
            #[must_use]
            pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
                Self(reference)
            }

            #[must_use]
            pub const fn as_artifact_ref(self) -> ArtifactRef {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl FromStr for $name {
            type Err = ArtifactError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                ArtifactRef::from_str(value).map(Self)
            }
        }
    };
}

artifact_reference!(SeparatorProblemRef);
artifact_reference!(ProtectedCompletionFieldRef);
artifact_reference!(ProtectedClassRef);
artifact_reference!(StructureViewRef);
artifact_reference!(GeneratorRegimeRef);
artifact_reference!(EffectivityRef);

/// A canonical candidate inquiry proposed for one separator problem through one declared route.
///
/// This is generation evidence only. It neither establishes that the route is lawful or
/// executable, selects the question under policy, probes it, nor makes any result actual,
/// supported, or warranted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeneratedInquiry {
    problem: SeparatorProblemRef,
    generation_route: ArtifactRef,
    question: QueryRef,
}

impl GeneratedInquiry {
    #[must_use]
    pub const fn new(
        problem: SeparatorProblemRef,
        generation_route: ArtifactRef,
        question: QueryRef,
    ) -> Self {
        Self {
            problem,
            generation_route,
            question,
        }
    }

    #[must_use]
    pub const fn problem(&self) -> SeparatorProblemRef {
        self.problem
    }
    #[must_use]
    pub const fn generation_route(&self) -> ArtifactRef {
        self.generation_route
    }
    #[must_use]
    pub const fn question(&self) -> QueryRef {
        self.question
    }

    #[must_use]
    pub fn canonical_payload(&self) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(96);
        reference(&mut encoded, self.problem.as_artifact_ref());
        reference(&mut encoded, self.generation_route);
        reference(&mut encoded, self.question.as_artifact_ref());
        encoded
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, GeneratedInquiryError> {
        if payload.len() < 96 {
            return Err(GeneratedInquiryError::TruncatedPayload);
        }
        if payload.len() > 96 {
            return Err(GeneratedInquiryError::TrailingPayloadBytes(
                payload.len() - 96,
            ));
        }
        let reference_at = |offset: usize| {
            let bytes: [u8; 32] = payload[offset..offset + 32]
                .try_into()
                .expect("fixed generated-inquiry payload range must have 32 bytes");
            ArtifactRef::from_bytes(bytes)
        };
        Ok(Self::new(
            SeparatorProblemRef::from_artifact_ref(reference_at(0)),
            reference_at(32),
            QueryRef::from_artifact_ref(reference_at(64)),
        ))
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, GeneratedInquiryError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(GENERATED_INQUIRY_ARTIFACT_KIND)?,
            GENERATED_INQUIRY_SCHEMA_VERSION,
            self.canonical_payload(),
        ))
    }

    pub fn generated_inquiry_ref(&self) -> Result<ArtifactRef, GeneratedInquiryError> {
        Ok(self.envelope()?.artifact_ref()?)
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, GeneratedInquiryError> {
        if envelope.kind().as_str() != GENERATED_INQUIRY_ARTIFACT_KIND {
            return Err(GeneratedInquiryError::UnexpectedArtifactKind {
                expected: GENERATED_INQUIRY_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != GENERATED_INQUIRY_SCHEMA_VERSION {
            return Err(GeneratedInquiryError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        vec![
            self.problem.as_artifact_ref(),
            self.generation_route,
            self.question.as_artifact_ref(),
        ]
    }

    /// Rechecks problem/query identity and shared binding, grain, and protected horizon.
    pub fn check<C: GeneratedInquiryCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), GeneratedInquiryCheckError> {
        let problem = catalog
            .resolve_separator_problem(self.problem)
            .ok_or(GeneratedInquiryCheckError::UnresolvedProblem(self.problem))?;
        let calculated_problem = problem.separator_problem_ref()?;
        if calculated_problem != self.problem {
            return Err(GeneratedInquiryCheckError::ProblemIdentityMismatch {
                reference: self.problem,
                calculated: calculated_problem,
            });
        }
        let question = catalog.resolve_open_query(self.question).ok_or(
            GeneratedInquiryCheckError::UnresolvedQuestion(self.question),
        )?;
        let calculated_question = question.query_ref()?;
        if calculated_question != self.question {
            return Err(GeneratedInquiryCheckError::QuestionIdentityMismatch {
                reference: self.question,
                calculated: calculated_question,
            });
        }
        question.check(catalog)?;
        let schema = catalog.resolve_relation_schema(question.relation()).ok_or(
            GeneratedInquiryCheckError::UnresolvedRelation(question.relation()),
        )?;
        if schema.binding() != problem.binding() {
            return Err(GeneratedInquiryCheckError::BindingMismatch {
                expected: problem.binding(),
                actual: schema.binding(),
            });
        }
        if question.context().grain() != problem.grain() {
            return Err(GeneratedInquiryCheckError::GrainMismatch {
                expected: problem.grain(),
                actual: question.context().grain(),
            });
        }
        if question.context().horizon() != problem.horizon() {
            return Err(GeneratedInquiryCheckError::HorizonMismatch {
                expected: problem.horizon(),
                actual: question.context().horizon(),
            });
        }
        Ok(())
    }
}

/// The catalog boundary for a generated inquiry's structural validation.
pub trait GeneratedInquiryCatalog: OpenQueryCatalog {
    fn resolve_separator_problem(&self, reference: SeparatorProblemRef)
    -> Option<SeparatorProblem>;
}

#[derive(Debug, Error)]
pub enum GeneratedInquiryError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("generated-inquiry payload is truncated")]
    TruncatedPayload,
    #[error("generated-inquiry payload has {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported generated-inquiry schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

#[derive(Debug, Error)]
pub enum GeneratedInquiryCheckError {
    #[error(transparent)]
    Problem(#[from] SeparatorProblemError),
    #[error(transparent)]
    Query(#[from] OpenQueryError),
    #[error(transparent)]
    QueryCheck(#[from] OpenQueryCheckError),
    #[error("separator problem {0} is unavailable")]
    UnresolvedProblem(SeparatorProblemRef),
    #[error("separator problem {reference} hashes to {calculated}, not its claimed identity")]
    ProblemIdentityMismatch {
        reference: SeparatorProblemRef,
        calculated: SeparatorProblemRef,
    },
    #[error("open query {0} is unavailable")]
    UnresolvedQuestion(QueryRef),
    #[error("open query {reference} hashes to {calculated}, not its claimed identity")]
    QuestionIdentityMismatch {
        reference: QueryRef,
        calculated: QueryRef,
    },
    #[error("relation schema {0} is unavailable")]
    UnresolvedRelation(RelationRef),
    #[error("generated inquiry binding {actual} differs from separator problem binding {expected}")]
    BindingMismatch {
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },
    #[error("generated inquiry grain {actual} differs from separator problem grain {expected}")]
    GrainMismatch {
        expected: GrainRef,
        actual: GrainRef,
    },
    #[error("generated inquiry horizon {actual} differs from separator problem horizon {expected}")]
    HorizonMismatch {
        expected: HorizonRef,
        actual: HorizonRef,
    },
}

/// A finite, caller-declared generator regime and its currently materialized route identities.
///
/// This is a narrow Phase-14 boundary: route membership is declared, not discovered, and the
/// regime does not choose or execute a route. Its purpose is to retain the distinction between a
/// route that is available in the declared regime and one that has actually been materialized.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclaredFiniteGeneratorRegime {
    regime: GeneratorRegimeRef,
    routes: Vec<ArtifactRef>,
    materialized: BTreeSet<ArtifactRef>,
}

impl DeclaredFiniteGeneratorRegime {
    pub fn new(
        regime: GeneratorRegimeRef,
        mut routes: Vec<ArtifactRef>,
        materialized: Vec<ArtifactRef>,
    ) -> Result<Self, DeclaredFiniteGeneratorRegimeError> {
        routes.sort_unstable();
        if let Some(duplicate) = routes
            .windows(2)
            .find_map(|pair| (pair[0] == pair[1]).then_some(pair[0]))
        {
            return Err(DeclaredFiniteGeneratorRegimeError::DuplicateRoute(
                duplicate,
            ));
        }
        let materialized: BTreeSet<_> = materialized.into_iter().collect();
        if let Some(route) = materialized.iter().find(|route| !routes.contains(route)) {
            return Err(DeclaredFiniteGeneratorRegimeError::MaterializedRouteOutsideRegime(*route));
        }
        Ok(Self {
            regime,
            routes,
            materialized,
        })
    }

    #[must_use]
    pub const fn regime(&self) -> GeneratorRegimeRef {
        self.regime
    }
    #[must_use]
    pub fn routes(&self) -> &[ArtifactRef] {
        &self.routes
    }
    #[must_use]
    pub const fn materialized(&self) -> &BTreeSet<ArtifactRef> {
        &self.materialized
    }

    /// Distinguishes materialized, fresh-within-regime, and unavailable route identities.
    #[must_use]
    pub fn route_status(&self, route: ArtifactRef) -> DeclaredRouteMaterialization {
        if !self.routes.contains(&route) {
            DeclaredRouteMaterialization::OutsideDeclaredRegime
        } else if self.materialized.contains(&route) {
            DeclaredRouteMaterialization::Materialized
        } else {
            DeclaredRouteMaterialization::FreshWithinRegime
        }
    }
}

/// Materialization state relative only to one caller-declared finite regime.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeclaredRouteMaterialization {
    /// The route is one of the declared candidates and has been materialized.
    Materialized,
    /// The route is declared available but not currently materialized.
    FreshWithinRegime,
    /// No statement about expressibility follows from absence from this declared finite set.
    OutsideDeclaredRegime,
}

/// A route available in one declared finite regime but absent from its current materialization.
///
/// It is a continuation obligation, not proof that the route is lawful in a broader language or
/// that policy should select it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MaterializationGap {
    regime: GeneratorRegimeRef,
    route: ArtifactRef,
}

impl MaterializationGap {
    /// Returns a gap only when the route is fresh in the exact declared regime.
    pub fn new(
        regime: &DeclaredFiniteGeneratorRegime,
        route: ArtifactRef,
    ) -> Result<Self, MaterializationGapError> {
        match regime.route_status(route) {
            DeclaredRouteMaterialization::FreshWithinRegime => Ok(Self {
                regime: regime.regime(),
                route,
            }),
            DeclaredRouteMaterialization::Materialized => {
                Err(MaterializationGapError::AlreadyMaterialized(route))
            }
            DeclaredRouteMaterialization::OutsideDeclaredRegime => {
                Err(MaterializationGapError::OutsideDeclaredRegime(route))
            }
        }
    }
    #[must_use]
    pub const fn regime(&self) -> GeneratorRegimeRef {
        self.regime
    }
    #[must_use]
    pub const fn route(&self) -> ArtifactRef {
        self.route
    }
}

#[derive(Debug, Error)]
pub enum MaterializationGapError {
    #[error("route {0} is already materialized")]
    AlreadyMaterialized(ArtifactRef),
    #[error("route {0} is outside the declared generator regime")]
    OutsideDeclaredRegime(ArtifactRef),
}

/// A candidate route outside the current finite declared regime, preserved without admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProposedRegimeExtension {
    problem: SeparatorProblemRef,
    regime: GeneratorRegimeRef,
    proposed_route: ArtifactRef,
}

impl ProposedRegimeExtension {
    pub fn new(
        problem: SeparatorProblemRef,
        regime: &DeclaredFiniteGeneratorRegime,
        proposed_route: ArtifactRef,
    ) -> Result<Self, ProposedRegimeExtensionError> {
        if regime.routes().contains(&proposed_route) {
            return Err(ProposedRegimeExtensionError::AlreadyInRegime(
                proposed_route,
            ));
        }
        Ok(Self {
            problem,
            regime: regime.regime(),
            proposed_route,
        })
    }
    #[must_use]
    pub const fn problem(&self) -> SeparatorProblemRef {
        self.problem
    }
    #[must_use]
    pub const fn regime(&self) -> GeneratorRegimeRef {
        self.regime
    }
    #[must_use]
    pub const fn proposed_route(&self) -> ArtifactRef {
        self.proposed_route
    }
}

#[derive(Debug, Error)]
pub enum ProposedRegimeExtensionError {
    #[error("proposed route {0} is already in the declared generator regime")]
    AlreadyInRegime(ArtifactRef),
}

#[derive(Debug, Error)]
pub enum DeclaredFiniteGeneratorRegimeError {
    #[error("declared generator regime repeats route {0}")]
    DuplicateRoute(ArtifactRef),
    #[error("materialized route {0} is not in the declared generator regime")]
    MaterializedRouteOutsideRegime(ArtifactRef),
}

/// One caller-supplied exact signature for a declared generator-regime route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFiniteRegimeRoute {
    route: ArtifactRef,
    signature: ExactFiniteSignature,
}

impl ExactFiniteRegimeRoute {
    #[must_use]
    pub const fn new(route: ArtifactRef, signature: ExactFiniteSignature) -> Self {
        Self { route, signature }
    }
    #[must_use]
    pub const fn route(&self) -> ArtifactRef {
        self.route
    }
    #[must_use]
    pub const fn signature(&self) -> &ExactFiniteSignature {
        &self.signature
    }
}

/// The bounded no-separator conclusion available from a declared complete finite regime.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExactFiniteRegimeSeparatorResult {
    /// The declared exact route signatures jointly separate every protectedly distinct pair.
    SeparatingBasisPresent,
    /// One protectedly distinct pair remains indistinguishable to every route in this regime.
    ExactNoSeparatorWithinRegime { separator: FiniteCueSeparator },
}

/// Checks whether all routes in a declared finite regime jointly fail to separate a protected
/// finite signature.
///
/// Completeness is only over the supplied regime membership and caller-supplied exact tables. A
/// no-separator result therefore does not establish a representation gap outside that regime.
pub fn check_exact_no_separator_within_declared_regime(
    regime: &DeclaredFiniteGeneratorRegime,
    routes: &[ExactFiniteRegimeRoute],
    protected: &ExactFiniteSignature,
) -> Result<ExactFiniteRegimeSeparatorResult, ExactFiniteRegimeSeparatorError> {
    let mut seen = BTreeSet::new();
    let mut signatures = Vec::with_capacity(routes.len());
    for route in routes {
        if !regime.routes().contains(&route.route()) {
            return Err(ExactFiniteRegimeSeparatorError::RouteOutsideRegime(
                route.route(),
            ));
        }
        if !seen.insert(route.route()) {
            return Err(ExactFiniteRegimeSeparatorError::DuplicateRoute(
                route.route(),
            ));
        }
        signatures.push(route.signature().clone());
    }
    if let Some(route) = regime.routes().iter().find(|route| !seen.contains(route)) {
        return Err(ExactFiniteRegimeSeparatorError::MissingRouteSignature(
            *route,
        ));
    }
    match check_exact_finite_cue_basis(&signatures, protected)? {
        ExactFiniteCueBasisResult::Sufficient => {
            Ok(ExactFiniteRegimeSeparatorResult::SeparatingBasisPresent)
        }
        ExactFiniteCueBasisResult::Insufficient { separator } => {
            Ok(ExactFiniteRegimeSeparatorResult::ExactNoSeparatorWithinRegime { separator })
        }
    }
}

#[derive(Debug, Error)]
pub enum ExactFiniteRegimeSeparatorError {
    #[error("route {0} is outside the declared finite generator regime")]
    RouteOutsideRegime(ArtifactRef),
    #[error("declared finite generator regime repeats exact signature for route {0}")]
    DuplicateRoute(ArtifactRef),
    #[error("declared finite generator regime lacks an exact signature for route {0}")]
    MissingRouteSignature(ArtifactRef),
    #[error(transparent)]
    CueBasis(#[from] ExactFiniteCueBasisError),
}

/// One declared generic residual to be separated by a later admitted inquiry route.
///
/// The fields are exactly the phase-12 problem boundary: a protected completion field, optional
/// target class, indexed binding/grain/horizon, available structure, generator regime, and
/// effectivity horizon.  Construction records none of the semantics of those opaque references;
/// in particular, it neither proves a residual exists nor generates or selects a question.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SeparatorProblem {
    residual: ProtectedCompletionFieldRef,
    target: Option<ProtectedClassRef>,
    grain: GrainRef,
    horizon: HorizonRef,
    binding: BindingVersionRef,
    available_structure: StructureViewRef,
    generator_regime: GeneratorRegimeRef,
    effectivity: EffectivityRef,
}

impl SeparatorProblem {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        residual: ProtectedCompletionFieldRef,
        target: Option<ProtectedClassRef>,
        grain: GrainRef,
        horizon: HorizonRef,
        binding: BindingVersionRef,
        available_structure: StructureViewRef,
        generator_regime: GeneratorRegimeRef,
        effectivity: EffectivityRef,
    ) -> Self {
        Self {
            residual,
            target,
            grain,
            horizon,
            binding,
            available_structure,
            generator_regime,
            effectivity,
        }
    }

    #[must_use]
    pub const fn residual(&self) -> ProtectedCompletionFieldRef {
        self.residual
    }
    #[must_use]
    pub const fn target(&self) -> Option<ProtectedClassRef> {
        self.target
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
    pub const fn binding(&self) -> BindingVersionRef {
        self.binding
    }
    #[must_use]
    pub const fn available_structure(&self) -> StructureViewRef {
        self.available_structure
    }
    #[must_use]
    pub const fn generator_regime(&self) -> GeneratorRegimeRef {
        self.generator_regime
    }
    #[must_use]
    pub const fn effectivity(&self) -> EffectivityRef {
        self.effectivity
    }

    /// Encodes this problem directly; identity never depends on generic serialization.
    #[must_use]
    pub fn canonical_payload(&self) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(32 * 8 + 1);
        reference(&mut encoded, self.residual.as_artifact_ref());
        optional_reference(
            &mut encoded,
            self.target.map(ProtectedClassRef::as_artifact_ref),
        );
        reference(&mut encoded, self.grain.as_artifact_ref());
        reference(&mut encoded, self.horizon.as_artifact_ref());
        reference(&mut encoded, self.binding.as_artifact_ref());
        reference(&mut encoded, self.available_structure.as_artifact_ref());
        reference(&mut encoded, self.generator_regime.as_artifact_ref());
        reference(&mut encoded, self.effectivity.as_artifact_ref());
        encoded
    }

    /// Decodes a complete canonical separator-problem payload.
    pub fn decode_payload(payload: &[u8]) -> Result<Self, SeparatorProblemError> {
        let mut cursor = Cursor::new(payload);
        let residual = ProtectedCompletionFieldRef::from_artifact_ref(cursor.reference()?);
        let target = cursor
            .optional_reference()?
            .map(ProtectedClassRef::from_artifact_ref);
        let grain = GrainRef::from_artifact_ref(cursor.reference()?);
        let horizon = HorizonRef::from_artifact_ref(cursor.reference()?);
        let binding = BindingVersionRef::from_artifact_ref(cursor.reference()?);
        let available_structure = StructureViewRef::from_artifact_ref(cursor.reference()?);
        let generator_regime = GeneratorRegimeRef::from_artifact_ref(cursor.reference()?);
        let effectivity = EffectivityRef::from_artifact_ref(cursor.reference()?);
        if !cursor.finished() {
            return Err(SeparatorProblemError::TrailingPayloadBytes(
                cursor.remaining(),
            ));
        }
        Ok(Self::new(
            residual,
            target,
            grain,
            horizon,
            binding,
            available_structure,
            generator_regime,
            effectivity,
        ))
    }

    /// Returns the canonical artifact envelope for this structural residual record.
    pub fn envelope(&self) -> Result<ArtifactEnvelope, SeparatorProblemError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(SEPARATOR_PROBLEM_ARTIFACT_KIND)?,
            SEPARATOR_PROBLEM_SCHEMA_VERSION,
            self.canonical_payload(),
        ))
    }

    /// Returns the stable content identity of this separator problem.
    pub fn separator_problem_ref(&self) -> Result<SeparatorProblemRef, SeparatorProblemError> {
        Ok(SeparatorProblemRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    /// Decodes this artifact only when it names the canonical separator-problem domain/version.
    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, SeparatorProblemError> {
        if envelope.kind().as_str() != SEPARATOR_PROBLEM_ARTIFACT_KIND {
            return Err(SeparatorProblemError::UnexpectedArtifactKind {
                expected: SEPARATOR_PROBLEM_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != SEPARATOR_PROBLEM_SCHEMA_VERSION {
            return Err(SeparatorProblemError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    /// Lists prerequisite identities without interpreting their payloads.
    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = vec![self.residual.as_artifact_ref()];
        if let Some(target) = self.target {
            references.push(target.as_artifact_ref());
        }
        references.extend([
            self.grain.as_artifact_ref(),
            self.horizon.as_artifact_ref(),
            self.binding.as_artifact_ref(),
            self.available_structure.as_artifact_ref(),
            self.generator_regime.as_artifact_ref(),
            self.effectivity.as_artifact_ref(),
        ]);
        references
    }
}

fn reference(encoded: &mut Vec<u8>, value: ArtifactRef) {
    encoded.extend_from_slice(value.as_bytes());
}

fn optional_reference(encoded: &mut Vec<u8>, value: Option<ArtifactRef>) {
    match value {
        None => encoded.push(0),
        Some(value) => {
            encoded.push(1);
            reference(encoded, value);
        }
    }
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], SeparatorProblemError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(SeparatorProblemError::PayloadLengthOverflow)?;
        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(SeparatorProblemError::TruncatedPayload)?;
        self.position = end;
        Ok(bytes)
    }

    fn reference(&mut self) -> Result<ArtifactRef, SeparatorProblemError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| SeparatorProblemError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(bytes))
    }

    fn optional_reference(&mut self) -> Result<Option<ArtifactRef>, SeparatorProblemError> {
        match self.take(1)?[0] {
            0 => Ok(None),
            1 => self.reference().map(Some),
            tag => Err(SeparatorProblemError::UnknownOptionalTag(tag)),
        }
    }

    const fn finished(&self) -> bool {
        self.position == self.bytes.len()
    }

    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

/// Errors from canonical separator-problem encoding and decoding.
#[derive(Debug, Error)]
pub enum SeparatorProblemError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("separator-problem payload is truncated")]
    TruncatedPayload,
    #[error("separator-problem payload length overflows this platform")]
    PayloadLengthOverflow,
    #[error("separator-problem payload contains {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("separator-problem payload has unknown optional-reference tag {0}")]
    UnknownOptionalTag(u8),
    #[error("expected artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported separator-problem schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

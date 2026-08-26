use std::collections::BTreeMap;

use ic_core::{
    ActualDecodeError, ActualDecodeResult, ActualEvent, ActualEventCatalog, ApplicabilityRef,
    ArtifactRef, BindingVersionRef, BoundaryChart, BoundaryRef, ClaimArtifact, ClaimError,
    ClaimRef, ClaimStatus, CompletionCandidate, CompletionCandidateCatalog, CompletionCandidateRef,
    DeclaredStandingError, DeclaredSupportClosure, DecodedObservationError, DecoderRef,
    DepartureCatalog, DepartureStandingCheckError, DepartureWitness, DeterminationCatalog,
    DeterminationPresentation, DeterminationPresentationRef, DeterminationSupportError,
    DischargeMode, EffectivityRef, EventRef, FINITE_DECODER_ARTIFACT_KIND,
    FINITE_DECODER_SCHEMA_VERSION, FiniteDecoder, FiniteDecoderCatalog, FiniteDecoderEntry,
    FiniteDecoderError, FiniteDecoderOutcome, FormulaArtifact, FormulaCatalog, FormulaRef,
    GeneratedInquiry, GeneratedInquiryCatalog, GeneratedInquiryCheckError, GeneratorRegimeRef,
    GrainRef, HorizonRef, ObservationResultCatalog, OpenPort, OpenQuery, OpenQueryCatalog,
    OperatorOccurrence, OperatorOccurrenceCatalog, OperatorOccurrenceCheckError, PortBinding,
    ProbeContractRef, ProbeOperator, ProbeOperatorRef, ProtectedCompletionFieldRef, ProvenanceRef,
    QueryRef, RawReturn, RawReturnCatalog, RawReturnRef, RelationBodyIR, RelationCatalog,
    RelationPort, RelationRef, RelationSchema, RelationSignature, RelationUse, RelationUseContext,
    RelationUseRef, RelationUseSupportCatalog, RelationUseSupportError, ResolutionCatalog,
    ResolutionPath, ResolutionPathIR, ResolutionPathRef, RouteRef, ScopeRef, SeparatorProblem,
    SeparatorProblemRef, StateRef, StructureViewRef, SupportEnvironmentArtifact,
    SupportEnvironmentArtifactCheckError, SupportEnvironmentArtifactError,
    SupportEnvironmentCatalog, SupportEnvironmentRef, SupportRef, SupportSubjectRef, TyIR,
    TypeArtifact, TypeCatalog, TypeFamilyRef, TypeRef, TypeSymbol, TypedForm, TypedFormRef,
    check_departure_witness_standing_support, decode_actual_event, match_decoded_observation_use,
    resolve_determination_presentation_support, resolve_relation_use_support,
    standing_determination_presentation_support, standing_from_declared_support,
};

#[derive(Clone, Default)]
struct Catalog {
    types: BTreeMap<TypeRef, TypeArtifact>,
    forms: BTreeMap<TypedFormRef, TypedForm>,
    schemas: BTreeMap<RelationRef, RelationSchema>,
    signatures: BTreeMap<RelationRef, RelationSignature>,
    relation_uses: BTreeMap<RelationUseRef, RelationUse>,
    queries: BTreeMap<QueryRef, OpenQuery>,
    candidates: BTreeMap<CompletionCandidateRef, CompletionCandidate>,
    raw_returns: BTreeMap<RawReturnRef, RawReturn>,
    decoders: BTreeMap<ic_core::FiniteDecoderRef, FiniteDecoder>,
    paths: BTreeMap<ResolutionPathRef, ResolutionPath>,
    charts: BTreeMap<BoundaryRef, BoundaryChart>,
    operators: BTreeMap<ProbeOperatorRef, ProbeOperator>,
    events: BTreeMap<EventRef, ActualEvent>,
    claims: BTreeMap<ClaimRef, ClaimArtifact>,
    support_environments: BTreeMap<SupportEnvironmentRef, SupportEnvironmentArtifact>,
    presentations: BTreeMap<DeterminationPresentationRef, DeterminationPresentation>,
    separator_problems: BTreeMap<SeparatorProblemRef, SeparatorProblem>,
}

impl Catalog {
    fn insert_type(&mut self, artifact: TypeArtifact) -> TypeRef {
        let reference = artifact.type_ref().expect("type must encode");
        self.types.insert(reference, artifact);
        reference
    }

    fn insert_form(&mut self, form: TypedForm) -> TypedFormRef {
        let reference = form.typed_form_ref().expect("form must encode");
        self.forms.insert(reference, form);
        reference
    }

    fn insert_schema(&mut self, schema: RelationSchema) -> RelationRef {
        let reference = schema.relation_ref().expect("schema must encode");
        self.signatures.insert(
            reference,
            schema.signature().expect("schema signature must encode"),
        );
        self.schemas.insert(reference, schema);
        reference
    }

    fn insert_query(&mut self, query: OpenQuery) -> QueryRef {
        let reference = query.query_ref().expect("query must encode");
        self.queries.insert(reference, query);
        reference
    }

    fn insert_relation_use(&mut self, relation_use: RelationUse) -> RelationUseRef {
        let reference = relation_use
            .relation_use_ref()
            .expect("relation use must encode");
        self.relation_uses.insert(reference, relation_use);
        reference
    }

    fn insert_candidate(&mut self, candidate: CompletionCandidate) -> CompletionCandidateRef {
        let reference = candidate
            .completion_candidate_ref()
            .expect("candidate must encode");
        self.candidates.insert(reference, candidate);
        reference
    }

    fn insert_raw_return(&mut self, raw_return: RawReturn) -> RawReturnRef {
        let reference = raw_return.raw_return_ref().expect("raw return must encode");
        self.raw_returns.insert(reference, raw_return);
        reference
    }

    fn insert_decoder(&mut self, decoder: FiniteDecoder) -> ic_core::FiniteDecoderRef {
        let reference = decoder.finite_decoder_ref().expect("decoder must encode");
        self.decoders.insert(reference, decoder);
        reference
    }

    fn insert_path(&mut self, path: ResolutionPath) -> ResolutionPathRef {
        let reference = path.resolution_path_ref().expect("path must encode");
        self.paths.insert(reference, path);
        reference
    }

    fn insert_claim(&mut self, claim: ClaimArtifact) -> ClaimRef {
        let reference = claim.claim_ref().expect("claim must encode");
        self.claims.insert(reference, claim);
        reference
    }

    fn insert_support_environment(
        &mut self,
        environment: SupportEnvironmentArtifact,
    ) -> SupportEnvironmentRef {
        let reference = environment
            .support_environment_ref()
            .expect("support environment must encode");
        self.support_environments.insert(reference, environment);
        reference
    }

    fn insert_presentation(
        &mut self,
        presentation: DeterminationPresentation,
    ) -> DeterminationPresentationRef {
        let reference = presentation
            .determination_presentation_ref()
            .expect("presentation must encode");
        self.presentations.insert(reference, presentation);
        reference
    }

    fn insert_separator_problem(&mut self, problem: SeparatorProblem) -> SeparatorProblemRef {
        let reference = problem
            .separator_problem_ref()
            .expect("separator problem must encode");
        self.separator_problems.insert(reference, problem);
        reference
    }

    fn insert_event(&mut self, event: ActualEvent) -> EventRef {
        let reference = event.event_ref().expect("event must encode");
        self.events.insert(reference, event);
        reference
    }
}

impl TypeCatalog for Catalog {
    fn resolve_type(&self, reference: TypeRef) -> Option<TypeArtifact> {
        self.types.get(&reference).cloned()
    }

    fn resolve_family_domain(
        &self,
        _reference: TypeFamilyRef,
    ) -> Option<(BindingVersionRef, TypeRef)> {
        None
    }
}

impl FormulaCatalog for Catalog {
    fn resolve_formula(&self, _reference: FormulaRef) -> Option<FormulaArtifact> {
        None
    }

    fn resolve_typed_form(&self, reference: TypedFormRef) -> Option<TypedForm> {
        self.forms.get(&reference).copied()
    }

    fn resolve_relation_signature(&self, reference: RelationRef) -> Option<RelationSignature> {
        self.signatures.get(&reference).cloned()
    }
}

impl RelationCatalog for Catalog {
    fn resolve_relation_schema(&self, reference: RelationRef) -> Option<RelationSchema> {
        self.schemas.get(&reference).cloned()
    }
}

impl OpenQueryCatalog for Catalog {
    fn resolve_open_query(&self, reference: QueryRef) -> Option<OpenQuery> {
        self.queries.get(&reference).cloned()
    }
}

impl CompletionCandidateCatalog for Catalog {
    fn resolve_completion_candidate(
        &self,
        reference: CompletionCandidateRef,
    ) -> Option<CompletionCandidate> {
        self.candidates.get(&reference).cloned()
    }
}

impl RawReturnCatalog for Catalog {
    fn resolve_raw_return(&self, reference: RawReturnRef) -> Option<RawReturn> {
        self.raw_returns.get(&reference).cloned()
    }
}

impl FiniteDecoderCatalog for Catalog {
    fn resolve_finite_decoder(
        &self,
        reference: ic_core::FiniteDecoderRef,
    ) -> Option<FiniteDecoder> {
        self.decoders.get(&reference).cloned()
    }
}

impl ResolutionCatalog for Catalog {
    fn resolve_resolution_path(&self, reference: ResolutionPathRef) -> Option<ResolutionPath> {
        self.paths.get(&reference).cloned()
    }
}

impl ActualEventCatalog for Catalog {
    fn resolve_boundary_chart(&self, reference: BoundaryRef) -> Option<BoundaryChart> {
        self.charts.get(&reference).cloned()
    }

    fn resolve_probe_operator(&self, reference: ProbeOperatorRef) -> Option<ProbeOperator> {
        self.operators.get(&reference).cloned()
    }

    fn resolve_open_query(&self, reference: QueryRef) -> Option<OpenQuery> {
        self.queries.get(&reference).cloned()
    }
}

impl ObservationResultCatalog for Catalog {
    fn resolve_relation_use(&self, reference: RelationUseRef) -> Option<RelationUse> {
        self.relation_uses.get(&reference).cloned()
    }
}

impl SupportEnvironmentCatalog for Catalog {
    fn resolve_claim(&self, reference: ClaimRef) -> Option<ClaimArtifact> {
        self.claims.get(&reference).cloned()
    }

    fn resolve_support_environment(
        &self,
        reference: SupportEnvironmentRef,
    ) -> Option<SupportEnvironmentArtifact> {
        self.support_environments.get(&reference).cloned()
    }
}

impl RelationUseSupportCatalog for Catalog {
    fn resolve_relation_use(&self, reference: RelationUseRef) -> Option<RelationUse> {
        self.relation_uses.get(&reference).cloned()
    }
}

impl DeterminationCatalog for Catalog {
    fn resolve_determination_presentation(
        &self,
        reference: DeterminationPresentationRef,
    ) -> Option<DeterminationPresentation> {
        self.presentations.get(&reference).cloned()
    }
}

impl DepartureCatalog for Catalog {
    fn resolve_relation_use(&self, reference: RelationUseRef) -> Option<RelationUse> {
        self.relation_uses.get(&reference).cloned()
    }
}

impl GeneratedInquiryCatalog for Catalog {
    fn resolve_separator_problem(
        &self,
        reference: SeparatorProblemRef,
    ) -> Option<SeparatorProblem> {
        self.separator_problems.get(&reference).copied()
    }
}

impl OperatorOccurrenceCatalog for Catalog {
    fn resolve_actual_event(&self, reference: EventRef) -> Option<ActualEvent> {
        self.events.get(&reference).cloned()
    }
}

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn port(name: &str, ty: TypeRef) -> RelationPort {
    RelationPort::new(TypeSymbol::new(name).expect("port name must be valid"), ty)
}

struct Fixture {
    catalog: Catalog,
    query: QueryRef,
    relation: RelationRef,
    answer_type: TypeRef,
    other_type: TypeRef,
    raw_type: TypeRef,
    candidate: CompletionCandidateRef,
    alternate_answer: TypedFormRef,
    observation: RelationUseRef,
    decoded_raw: RawReturnRef,
    undefined_raw: RawReturnRef,
    unknown_raw: RawReturnRef,
    event: ActualEvent,
}

fn fixture() -> Fixture {
    let mut catalog = Catalog::default();
    let binding = BindingVersionRef::from_artifact_ref(artifact(0x10));
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let other_type = catalog.insert_type(TypeArtifact::new(binding, TyIR::Bool));
    let raw_type = catalog.insert_type(TypeArtifact::new(binding, TyIR::Raw(unit)));
    let known = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x11)));
    let answer = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x12)));
    let alternate_answer = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x27)));
    let relation = catalog.insert_schema(RelationSchema::new(
        binding,
        vec![port("known", unit), port("answer", unit)],
        RelationBodyIR::BindingNative {
            contract: artifact(0x13),
        },
        Vec::new(),
        Vec::new(),
    ));
    let grain = GrainRef::from_artifact_ref(artifact(0x14));
    let horizon = HorizonRef::from_artifact_ref(artifact(0x15));
    let context = RelationUseContext::new(
        ScopeRef::from_artifact_ref(artifact(0x16)),
        ApplicabilityRef::from_artifact_ref(artifact(0x17)),
        grain,
        horizon,
        DischargeMode::Probe,
        SupportRef::from_artifact_ref(artifact(0x18)),
        None,
    );
    let open_query = OpenQuery::new(
        relation,
        vec![PortBinding::new(
            TypeSymbol::new("known").expect("port name must be valid"),
            known,
        )],
        vec![OpenPort::new(
            TypeSymbol::new("answer").expect("port name must be valid"),
            DischargeMode::Probe,
        )],
        context,
    );
    let candidate = open_query
        .plug(
            vec![PortBinding::new(
                TypeSymbol::new("answer").expect("port name must be valid"),
                answer,
            )],
            &catalog,
        )
        .expect("candidate must be constructible");
    let query = catalog.insert_query(open_query);
    let candidate = catalog.insert_candidate(candidate);
    let observation = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("known").expect("port name must be valid"),
                known,
            ),
            PortBinding::new(
                TypeSymbol::new("answer").expect("port name must be valid"),
                answer,
            ),
        ],
        context,
    ));
    let decoded_raw = catalog.insert_raw_return(RawReturn::new(vec![1]));
    let undefined_raw = catalog.insert_raw_return(RawReturn::new(vec![2]));
    let unknown_raw = catalog.insert_raw_return(RawReturn::new(vec![3]));

    let chart = BoundaryChart::new(
        query,
        unit,
        unit,
        unit,
        relation,
        relation,
        DeterminationPresentationRef::from_artifact_ref(artifact(0x19)),
        None,
        Vec::new(),
        Vec::new(),
        ic_core::RelationUseRef::from_artifact_ref(artifact(0x1a)),
        FormulaRef::from_artifact_ref(artifact(0x1b)),
        None,
        grain,
        horizon,
    );
    let boundary = chart.boundary_ref().expect("chart must encode");
    catalog.charts.insert(boundary, chart);
    let operator = ProbeOperator::new(
        query,
        boundary,
        artifact(0x1c),
        artifact(0x1d),
        artifact(0x1e),
        raw_type,
        artifact(0x1f),
        ProbeContractRef::from_artifact_ref(artifact(0x20)),
        artifact(0x21),
    );
    let operator_ref = operator.probe_operator_ref().expect("operator must encode");
    catalog.operators.insert(operator_ref, operator);
    let event = ActualEvent::new(
        None,
        StateRef::from_artifact_ref(artifact(0x22)),
        query,
        boundary,
        None,
        operator_ref,
        decoded_raw,
        StateRef::from_artifact_ref(artifact(0x23)),
        grain,
        RouteRef::from_artifact_ref(artifact(0x24)),
        binding,
        artifact(0x25),
        ProvenanceRef::from_artifact_ref(artifact(0x26)),
    );
    Fixture {
        catalog,
        query,
        relation,
        answer_type: unit,
        other_type,
        raw_type,
        candidate,
        alternate_answer,
        observation,
        decoded_raw,
        undefined_raw,
        unknown_raw,
        event,
    }
}

#[test]
fn finite_decoder_preserves_decoded_undefined_and_unknown_outcomes() {
    let mut fixture = fixture();
    let decoder = FiniteDecoder::new(
        fixture.query,
        fixture.raw_type,
        vec![
            FiniteDecoderEntry::Undefined {
                raw_return: fixture.undefined_raw,
            },
            FiniteDecoderEntry::Decoded {
                raw_return: fixture.decoded_raw,
                candidates: vec![fixture.candidate],
            },
        ],
    )
    .expect("finite rows must be canonicalizable");
    let reference = fixture.catalog.insert_decoder(decoder.clone());
    assert!(decoder.check(&fixture.catalog).is_ok());
    assert_eq!(
        decoder.finite_decoder_ref().expect("decoder must hash"),
        reference
    );
    assert_eq!(
        FiniteDecoder::from_envelope(&decoder.envelope().expect("decoder must encode"))
            .expect("decoder must decode"),
        decoder
    );
    assert_eq!(
        decoder.outcome(fixture.decoded_raw),
        FiniteDecoderOutcome::Decoded(vec![fixture.candidate])
    );
    assert_eq!(
        decoder.outcome(fixture.undefined_raw),
        FiniteDecoderOutcome::Undefined
    );
    assert_eq!(
        decoder.outcome(fixture.unknown_raw),
        FiniteDecoderOutcome::Unknown
    );

    let empty = FiniteDecoder::new(
        fixture.query,
        fixture.raw_type,
        vec![FiniteDecoderEntry::Decoded {
            raw_return: fixture.decoded_raw,
            candidates: Vec::new(),
        }],
    );
    assert!(matches!(empty, Err(FiniteDecoderError::EmptyDecodedSet)));
    let wrong_kind = ic_core::ArtifactEnvelope::from_canonical_payload(
        ic_core::ArtifactKind::new("ic.raw-return").expect("kind must be valid"),
        FINITE_DECODER_SCHEMA_VERSION,
        decoder.canonical_payload().expect("payload must encode"),
    );
    assert!(matches!(
        FiniteDecoder::from_envelope(&wrong_kind),
        Err(FiniteDecoderError::UnexpectedArtifactKind {
            expected: FINITE_DECODER_ARTIFACT_KIND,
            ..
        })
    ));
    let payload = decoder.canonical_payload().expect("payload must encode");
    assert!(matches!(
        FiniteDecoder::decode_payload(&payload[..payload.len() - 1]),
        Err(FiniteDecoderError::TruncatedPayload)
    ));
    let mut unknown_tag = payload;
    unknown_tag[68] = 0xff; // QueryRef, input TypeRef, and entry-count precede the first row.
    assert!(matches!(
        FiniteDecoder::decode_payload(&unknown_tag),
        Err(FiniteDecoderError::UnknownEntryTag(0xff))
    ));
}

#[test]
fn generated_inquiry_is_a_checked_problem_relative_candidate_not_a_policy_choice() {
    let mut fixture = fixture();
    let problem = fixture
        .catalog
        .insert_separator_problem(SeparatorProblem::new(
            ProtectedCompletionFieldRef::from_artifact_ref(artifact(0x81)),
            None,
            GrainRef::from_artifact_ref(artifact(0x14)),
            HorizonRef::from_artifact_ref(artifact(0x15)),
            fixture.event.binding(),
            StructureViewRef::from_artifact_ref(artifact(0x82)),
            GeneratorRegimeRef::from_artifact_ref(artifact(0x83)),
            EffectivityRef::from_artifact_ref(artifact(0x84)),
        ));
    let generated = GeneratedInquiry::new(problem, artifact(0x85), fixture.query);
    generated
        .check(&fixture.catalog)
        .expect("query and residual must agree on binding, grain, and horizon");
    assert_eq!(
        GeneratedInquiry::from_envelope(&generated.envelope().expect("candidate must encode"))
            .expect("candidate must decode"),
        generated
    );
    assert_eq!(
        generated.referenced_artifacts(),
        vec![
            problem.as_artifact_ref(),
            artifact(0x85),
            fixture.query.as_artifact_ref()
        ]
    );

    let other_problem = fixture
        .catalog
        .insert_separator_problem(SeparatorProblem::new(
            ProtectedCompletionFieldRef::from_artifact_ref(artifact(0x81)),
            None,
            GrainRef::from_artifact_ref(artifact(0x86)),
            HorizonRef::from_artifact_ref(artifact(0x15)),
            fixture.event.binding(),
            StructureViewRef::from_artifact_ref(artifact(0x82)),
            GeneratorRegimeRef::from_artifact_ref(artifact(0x83)),
            EffectivityRef::from_artifact_ref(artifact(0x84)),
        ));
    assert!(matches!(
        GeneratedInquiry::new(other_problem, artifact(0x85), fixture.query).check(&fixture.catalog),
        Err(GeneratedInquiryCheckError::GrainMismatch { .. })
    ));
}

#[test]
fn operator_occurrence_is_derived_from_one_exact_ordinary_event() {
    let mut fixture = fixture();
    let event = fixture.catalog.insert_event(fixture.event.clone());
    let occurrence = OperatorOccurrence::from_actual_event(event, &fixture.catalog)
        .expect("a checked actual event determines its occurrence view");
    occurrence
        .check(&fixture.catalog)
        .expect("derived occurrence must exactly match the ordinary event");
    assert_eq!(occurrence.event(), event);
    assert_eq!(
        OperatorOccurrence::from_envelope(&occurrence.envelope().expect("occurrence must encode"))
            .expect("occurrence must decode"),
        occurrence
    );
    let mismatched = OperatorOccurrence::new(
        event,
        occurrence.operator(),
        occurrence.state_before(),
        RawReturnRef::from_artifact_ref(artifact(0x91)),
        occurrence.state_after(),
        occurrence.boundary(),
    );
    assert!(matches!(
        mismatched.check(&fixture.catalog),
        Err(OperatorOccurrenceCheckError::EventFieldMismatch {
            field: "raw_return",
            ..
        })
    ));
}

#[test]
fn claim_identity_preserves_candidate_provenance_without_claiming_standing() {
    let mut fixture = fixture();
    let path = fixture.catalog.insert_path(ResolutionPath::new(
        fixture.raw_type,
        fixture.raw_type,
        ResolutionPathIR::Identity,
    ));
    let claim = ClaimArtifact::new(
        artifact(0x40),
        fixture.query,
        vec![fixture.decoded_raw],
        vec![path],
        ScopeRef::from_artifact_ref(artifact(0x41)),
        ApplicabilityRef::from_artifact_ref(artifact(0x42)),
        ClaimStatus::Standing,
    )
    .expect("claim references must canonicalize");
    assert!(claim.check(&fixture.catalog).is_ok());
    assert_eq!(
        ClaimArtifact::from_envelope(&claim.envelope().expect("claim must encode"))
            .expect("claim must decode"),
        claim
    );
    assert_eq!(claim.status(), ClaimStatus::Standing);
    assert!(matches!(
        ClaimArtifact::new(
            artifact(0x40),
            fixture.query,
            vec![fixture.decoded_raw, fixture.decoded_raw],
            vec![path],
            ScopeRef::from_artifact_ref(artifact(0x41)),
            ApplicabilityRef::from_artifact_ref(artifact(0x42)),
            ClaimStatus::Candidate,
        ),
        Err(ClaimError::DuplicateSupportingReturn(reference)) if reference == fixture.decoded_raw
    ));
    let mut malformed = claim.canonical_payload().expect("payload must encode");
    let last = malformed.len() - 1;
    malformed[last] = 0xff;
    assert!(matches!(
        ClaimArtifact::decode_payload(&malformed),
        Err(ClaimError::UnknownStatus)
    ));
}

#[test]
fn support_environment_identity_preserves_candidate_support_without_closure() {
    let mut fixture = fixture();
    let path = fixture.catalog.insert_path(ResolutionPath::new(
        fixture.raw_type,
        fixture.raw_type,
        ResolutionPathIR::Identity,
    ));
    let scope = ScopeRef::from_artifact_ref(artifact(0x51));
    let applicability = ApplicabilityRef::from_artifact_ref(artifact(0x52));
    let claim = fixture.catalog.insert_claim(
        ClaimArtifact::new(
            artifact(0x50),
            fixture.query,
            vec![fixture.decoded_raw],
            vec![path],
            scope,
            applicability,
            ClaimStatus::Candidate,
        )
        .expect("claim must canonicalize"),
    );
    let environment = SupportEnvironmentArtifact::new(
        SupportSubjectRef::Claim(claim),
        vec![artifact(0x59)],
        vec![fixture.decoded_raw],
        vec![artifact(0x53)],
        vec![artifact(0x54)],
        vec![artifact(0x55)],
        applicability,
        scope,
    )
    .expect("support environment must canonicalize");
    assert_eq!(environment.premises(), [artifact(0x59)]);
    assert!(environment.check(&fixture.catalog).is_ok());
    assert_eq!(
        SupportEnvironmentArtifact::from_envelope(
            &environment
                .envelope()
                .expect("support environment must encode"),
        )
        .expect("support environment must decode"),
        environment
    );
    let environment_ref = fixture
        .catalog
        .insert_support_environment(environment.clone());
    assert_eq!(
        environment_ref.as_support_ref().as_artifact_ref(),
        environment_ref.as_artifact_ref()
    );
    assert_eq!(
        SupportEnvironmentCatalog::resolve_support_environment(&fixture.catalog, environment_ref),
        Some(environment)
    );
    let relation_environment = SupportEnvironmentArtifact::new(
        SupportSubjectRef::Relation(fixture.relation),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        applicability,
        scope,
    )
    .expect("relation-targeted environment must remain representable");
    assert!(relation_environment.check(&fixture.catalog).is_ok());
    let relation_environment = fixture
        .catalog
        .insert_support_environment(relation_environment);
    let original_use = fixture
        .catalog
        .relation_uses
        .get(&fixture.observation)
        .expect("fixture observation must remain available")
        .clone();
    let linked_environment = SupportEnvironmentArtifact::new(
        SupportSubjectRef::Relation(original_use.relation()),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        original_use.applicability(),
        original_use.scope(),
    )
    .expect("use-targeted environment must canonicalize");
    let linked_environment = fixture
        .catalog
        .insert_support_environment(linked_environment);
    let linked_use = fixture.catalog.insert_relation_use(RelationUse::new(
        original_use.relation(),
        original_use.bindings().to_vec(),
        RelationUseContext::new(
            original_use.scope(),
            original_use.applicability(),
            original_use.grain(),
            original_use.horizon(),
            original_use.mode(),
            linked_environment.as_support_ref(),
            original_use.warrant(),
        ),
    ));
    let link = resolve_relation_use_support(linked_use, &fixture.catalog)
        .expect("matching relation-targeted support must link structurally");
    assert_eq!(link.relation_use(), linked_use);
    assert_eq!(link.environment(), linked_environment);
    let context_mismatched_use = fixture.catalog.insert_relation_use(RelationUse::new(
        original_use.relation(),
        original_use.bindings().to_vec(),
        RelationUseContext::new(
            original_use.scope(),
            original_use.applicability(),
            original_use.grain(),
            original_use.horizon(),
            original_use.mode(),
            relation_environment.as_support_ref(),
            original_use.warrant(),
        ),
    ));
    assert!(matches!(
        resolve_relation_use_support(context_mismatched_use, &fixture.catalog),
        Err(RelationUseSupportError::ContextMismatch("scope"))
    ));
    let root = fixture.catalog.insert_claim(
        ClaimArtifact::new(
            artifact(0x5a),
            fixture.query,
            vec![fixture.decoded_raw],
            vec![path],
            scope,
            applicability,
            ClaimStatus::Checked,
        )
        .expect("root claim must canonicalize"),
    );
    let standing_environment = fixture.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Claim(claim),
            vec![root.as_artifact_ref()],
            vec![fixture.decoded_raw],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        )
        .expect("standing environment must canonicalize"),
    );
    let closure = DeclaredSupportClosure::new(standing_environment, vec![root], true, true, false);
    let standing = standing_from_declared_support(vec![root], &[closure], &fixture.catalog)
        .expect("checked declared closure must enter the existing least fixed point");
    assert!(standing.contains(claim));
    assert!(matches!(
        standing_from_declared_support(
            vec![root],
            &[DeclaredSupportClosure::new(
                standing_environment,
                vec![claim],
                true,
                true,
                false,
            )],
            &fixture.catalog,
        ),
        Err(DeclaredStandingError::PremiseNotNamedByEnvironment { premise, .. })
            if premise == claim
    ));
    let mismatched_context = SupportEnvironmentArtifact::new(
        SupportSubjectRef::Claim(claim),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        applicability,
        ScopeRef::from_artifact_ref(artifact(0x56)),
    )
    .expect("mismatched environment remains representable");
    assert!(matches!(
        mismatched_context.check(&fixture.catalog),
        Err(SupportEnvironmentArtifactCheckError::ClaimContextMismatch(
            "scope"
        ))
    ));
    assert!(matches!(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Claim(claim),
            vec![artifact(0x59), artifact(0x59)],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        ),
        Err(SupportEnvironmentArtifactError::DuplicatePremise(reference))
            if reference == artifact(0x59)
    ));
    assert!(matches!(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Claim(claim),
            Vec::new(),
            vec![fixture.decoded_raw, fixture.decoded_raw],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        ),
        Err(SupportEnvironmentArtifactError::DuplicateActualReturn(reference))
            if reference == fixture.decoded_raw
    ));
}

#[test]
fn determination_support_requires_one_checked_claim_targeted_standing_environment() {
    let mut fixture = fixture();
    let binding = BindingVersionRef::from_artifact_ref(artifact(0x10));
    let scope = ScopeRef::from_artifact_ref(artifact(0x16));
    let applicability = ApplicabilityRef::from_artifact_ref(artifact(0x17));
    let source =
        fixture
            .catalog
            .insert_form(TypedForm::new(binding, fixture.answer_type, artifact(0x71)));
    let claim = fixture.catalog.insert_claim(
        ClaimArtifact::new(
            source.as_artifact_ref(),
            fixture.query,
            Vec::new(),
            Vec::new(),
            scope,
            applicability,
            ClaimStatus::Checked,
        )
        .expect("claim must canonicalize"),
    );
    let environment = fixture.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Claim(claim),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        )
        .expect("claim-targeted environment must canonicalize"),
    );
    let presentation = fixture
        .catalog
        .insert_presentation(DeterminationPresentation::new(
            ic_core::DistinctionRef::from_artifact_ref(artifact(0x72)),
            ic_core::Orientation::X,
            source,
            ic_core::RelationalWebRef::from_artifact_ref(artifact(0x73)),
            binding,
            scope,
            applicability,
            GrainRef::from_artifact_ref(artifact(0x14)),
            HorizonRef::from_artifact_ref(artifact(0x15)),
            environment.as_support_ref(),
            None,
        ));

    let resolved = resolve_determination_presentation_support(presentation, &fixture.catalog)
        .expect("presentation must resolve only its exact claim-targeted environment");
    assert_eq!(resolved.presentation(), presentation);
    assert_eq!(resolved.environment(), environment);
    assert_eq!(resolved.claim(), claim);

    let ungrounded = standing_from_declared_support(Vec::new(), &[], &fixture.catalog)
        .expect("an empty standing problem is well defined");
    assert!(matches!(
        standing_determination_presentation_support(presentation, &ungrounded, &fixture.catalog),
        Err(DeterminationSupportError::ClaimIsNotStanding(reference)) if reference == claim
    ));

    let closure = DeclaredSupportClosure::new(environment, Vec::new(), true, true, false);
    let standing = standing_from_declared_support(Vec::new(), &[closure], &fixture.catalog)
        .expect("the declared closed route reaches its claim in the least fixed point");
    assert_eq!(
        standing_determination_presentation_support(presentation, &standing, &fixture.catalog)
            .expect("a standing target claim must complete the structural link"),
        resolved
    );

    let relation_environment = fixture.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Relation(fixture.relation),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        )
        .expect("relation-targeted environment must canonicalize"),
    );
    let relation_targeted_presentation =
        fixture
            .catalog
            .insert_presentation(DeterminationPresentation::new(
                ic_core::DistinctionRef::from_artifact_ref(artifact(0x74)),
                ic_core::Orientation::X,
                source,
                ic_core::RelationalWebRef::from_artifact_ref(artifact(0x75)),
                binding,
                scope,
                applicability,
                GrainRef::from_artifact_ref(artifact(0x14)),
                HorizonRef::from_artifact_ref(artifact(0x15)),
                relation_environment.as_support_ref(),
                None,
            ));
    assert!(matches!(
        resolve_determination_presentation_support(
            relation_targeted_presentation,
            &fixture.catalog
        ),
        Err(DeterminationSupportError::RelationTargetIsNotDeterminationSupport)
    ));
}

#[test]
fn departure_witness_requires_its_source_presentation_support_to_stand() {
    let mut fixture = fixture();
    let binding = BindingVersionRef::from_artifact_ref(artifact(0x10));
    let scope = ScopeRef::from_artifact_ref(artifact(0x16));
    let applicability = ApplicabilityRef::from_artifact_ref(artifact(0x17));
    let grain = GrainRef::from_artifact_ref(artifact(0x14));
    let horizon = HorizonRef::from_artifact_ref(artifact(0x15));
    let source =
        fixture
            .catalog
            .insert_form(TypedForm::new(binding, fixture.answer_type, artifact(0x91)));
    let candidate =
        fixture
            .catalog
            .insert_form(TypedForm::new(binding, fixture.answer_type, artifact(0x92)));
    let source_answer =
        fixture
            .catalog
            .insert_form(TypedForm::new(binding, fixture.answer_type, artifact(0x93)));
    let candidate_answer =
        fixture
            .catalog
            .insert_form(TypedForm::new(binding, fixture.answer_type, artifact(0x94)));
    let claim = fixture.catalog.insert_claim(
        ClaimArtifact::new(
            source.as_artifact_ref(),
            fixture.query,
            Vec::new(),
            Vec::new(),
            scope,
            applicability,
            ClaimStatus::Checked,
        )
        .expect("claim must canonicalize"),
    );
    let environment = fixture.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Claim(claim),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        )
        .expect("claim-targeted environment must canonicalize"),
    );
    let presentation = fixture
        .catalog
        .insert_presentation(DeterminationPresentation::new(
            ic_core::DistinctionRef::from_artifact_ref(artifact(0x95)),
            ic_core::Orientation::X,
            source,
            ic_core::RelationalWebRef::from_artifact_ref(artifact(0x96)),
            binding,
            scope,
            applicability,
            grain,
            horizon,
            environment.as_support_ref(),
            None,
        ));
    let evidence_environment = fixture.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Relation(fixture.relation),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        )
        .expect("relation-targeted evidence environment must canonicalize"),
    );
    let context = |mode| {
        RelationUseContext::new(
            scope,
            applicability,
            grain,
            horizon,
            mode,
            evidence_environment.as_support_ref(),
            None,
        )
    };
    let source_observation = fixture.catalog.insert_relation_use(RelationUse::new(
        fixture.relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("known").expect("port must be valid"),
                source,
            ),
            PortBinding::new(
                TypeSymbol::new("answer").expect("port must be valid"),
                source_answer,
            ),
        ],
        context(DischargeMode::Probe),
    ));
    let candidate_observation = fixture.catalog.insert_relation_use(RelationUse::new(
        fixture.relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("known").expect("port must be valid"),
                candidate,
            ),
            PortBinding::new(
                TypeSymbol::new("answer").expect("port must be valid"),
                candidate_answer,
            ),
        ],
        context(DischargeMode::Check),
    ));
    let incompatibility = fixture.catalog.insert_relation_use(RelationUse::new(
        fixture.relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("known").expect("port must be valid"),
                source_answer,
            ),
            PortBinding::new(
                TypeSymbol::new("answer").expect("port must be valid"),
                candidate_answer,
            ),
        ],
        context(DischargeMode::Warrant),
    ));
    let witness = DepartureWitness::new(
        ic_core::DistinctionRef::from_artifact_ref(artifact(0x95)),
        source,
        candidate,
        presentation,
        source_observation,
        candidate_observation,
        source_answer,
        candidate_answer,
        incompatibility,
        SupportRef::from_artifact_ref(artifact(0x97)),
        scope,
        applicability,
        grain,
    );

    let ungrounded = standing_from_declared_support(Vec::new(), &[], &fixture.catalog)
        .expect("an empty standing problem is well defined");
    assert!(matches!(
        check_departure_witness_standing_support(&witness, &ungrounded, &fixture.catalog),
        Err(DepartureStandingCheckError::DeterminationSupport(
            DeterminationSupportError::ClaimIsNotStanding(reference)
        )) if reference == claim
    ));

    let closure = DeclaredSupportClosure::new(environment, Vec::new(), true, true, false);
    let standing = standing_from_declared_support(Vec::new(), &[closure], &fixture.catalog)
        .expect("the declared closed route reaches its claim in the least fixed point");
    let resolved = check_departure_witness_standing_support(&witness, &standing, &fixture.catalog)
        .expect("a checked witness must retain the source presentation's standing support link");
    assert_eq!(resolved.presentation(), presentation);
    assert_eq!(resolved.environment(), environment);
    assert_eq!(resolved.claim(), claim);
}

#[test]
fn finite_decode_links_an_event_record_to_its_direct_decoder_route() {
    let mut fixture = fixture();
    let decoder = FiniteDecoder::new(
        fixture.query,
        fixture.raw_type,
        vec![
            FiniteDecoderEntry::Decoded {
                raw_return: fixture.decoded_raw,
                candidates: vec![fixture.candidate],
            },
            FiniteDecoderEntry::Undefined {
                raw_return: fixture.undefined_raw,
            },
        ],
    )
    .expect("finite rows must be valid");
    let decoder_ref = fixture.catalog.insert_decoder(decoder.clone());
    let path = ResolutionPath::new(
        fixture.raw_type,
        fixture.answer_type,
        ResolutionPathIR::Decode {
            decoder: decoder_ref.as_decoder_ref(),
        },
    );
    let path_ref = fixture.catalog.insert_path(path);

    let decoded = decode_actual_event(&fixture.event, &decoder, path_ref, &fixture.catalog)
        .expect("the realized raw return must decode through its named route");
    let set = match decoded {
        ActualDecodeResult::Decoded(set) => set,
        other => panic!("expected a decoded candidate set, got {other:?}"),
    };
    assert_eq!(set.query(), fixture.query);
    assert_eq!(set.candidates(), [fixture.candidate]);
    let observation = match_decoded_observation_use(
        &set,
        fixture.candidate,
        fixture.observation,
        &fixture.catalog,
    )
    .expect("the complete decoded candidate must spell its declared observation use");
    assert_eq!(observation.decoded(), &set);
    assert_eq!(observation.candidate(), fixture.candidate);
    assert_eq!(observation.observation(), fixture.observation);

    let query = OpenQueryCatalog::resolve_open_query(&fixture.catalog, fixture.query)
        .expect("query must remain available");
    let alternate_candidate = query
        .plug(
            vec![PortBinding::new(
                TypeSymbol::new("answer").expect("port name must be valid"),
                fixture.alternate_answer,
            )],
            &fixture.catalog,
        )
        .expect("alternate complete filling must be constructible");
    let alternate_candidate = fixture.catalog.insert_candidate(alternate_candidate);
    assert!(matches!(
        match_decoded_observation_use(
            &set,
            alternate_candidate,
            fixture.observation,
            &fixture.catalog,
        ),
        Err(DecodedObservationError::CandidateNotDecoded { candidate, .. })
            if candidate == alternate_candidate
    ));
    let mismatched_observation = fixture.catalog.insert_relation_use(RelationUse::new(
        query.relation(),
        vec![
            PortBinding::new(
                TypeSymbol::new("known").expect("port name must be valid"),
                query.bound_ports()[0].value(),
            ),
            PortBinding::new(
                TypeSymbol::new("answer").expect("port name must be valid"),
                fixture.alternate_answer,
            ),
        ],
        *query.context(),
    ));
    assert!(matches!(
        match_decoded_observation_use(
            &set,
            fixture.candidate,
            mismatched_observation,
            &fixture.catalog,
        ),
        Err(DecodedObservationError::BindingMismatch)
    ));

    let undefined_event = ActualEvent::new(
        fixture.event.ledger_parent(),
        fixture.event.state_before(),
        fixture.event.question(),
        fixture.event.boundary(),
        fixture.event.distinction(),
        fixture.event.operator(),
        fixture.undefined_raw,
        fixture.event.state_after(),
        fixture.event.grain(),
        fixture.event.route(),
        fixture.event.binding(),
        fixture.event.backend_version(),
        fixture.event.provenance(),
    );
    assert!(matches!(
        decode_actual_event(&undefined_event, &decoder, path_ref, &fixture.catalog),
        Ok(ActualDecodeResult::Undefined { .. })
    ));
    let unknown_event = ActualEvent::new(
        fixture.event.ledger_parent(),
        fixture.event.state_before(),
        fixture.event.question(),
        fixture.event.boundary(),
        fixture.event.distinction(),
        fixture.event.operator(),
        fixture.unknown_raw,
        fixture.event.state_after(),
        fixture.event.grain(),
        fixture.event.route(),
        fixture.event.binding(),
        fixture.event.backend_version(),
        fixture.event.provenance(),
    );
    assert!(matches!(
        decode_actual_event(&unknown_event, &decoder, path_ref, &fixture.catalog),
        Ok(ActualDecodeResult::Unknown { .. })
    ));

    let identity = fixture.catalog.insert_path(ResolutionPath::new(
        fixture.raw_type,
        fixture.raw_type,
        ResolutionPathIR::Identity,
    ));
    assert!(matches!(
        decode_actual_event(&fixture.event, &decoder, identity, &fixture.catalog),
        Err(ActualDecodeError::PathIsNotDirectDecoder(reference)) if reference == identity
    ));
    let wrong_decoder = fixture.catalog.insert_path(ResolutionPath::new(
        fixture.raw_type,
        fixture.answer_type,
        ResolutionPathIR::Decode {
            decoder: DecoderRef::from_artifact_ref(artifact(0x32)),
        },
    ));
    assert!(matches!(
        decode_actual_event(&fixture.event, &decoder, wrong_decoder, &fixture.catalog),
        Err(ActualDecodeError::PathDecoderMismatch { .. })
    ));
    let wrong_output = fixture.catalog.insert_path(ResolutionPath::new(
        fixture.raw_type,
        fixture.other_type,
        ResolutionPathIR::Decode {
            decoder: decoder_ref.as_decoder_ref(),
        },
    ));
    assert!(matches!(
        decode_actual_event(&fixture.event, &decoder, wrong_output, &fixture.catalog),
        Err(ActualDecodeError::PathOutputMismatch { path, answer })
            if path == fixture.other_type && answer == fixture.answer_type
    ));
}

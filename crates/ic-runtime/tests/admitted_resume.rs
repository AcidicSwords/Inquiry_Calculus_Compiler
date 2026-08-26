use std::{
    collections::BTreeMap,
    convert::Infallible,
    env,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};

use ic_core::{
    ActualDecodeResult, ActualEvent, ActualEventCatalog, ApplicabilityRef, ArtifactEnvelope,
    ArtifactKind, ArtifactRef, BackendRequest, BindingVersionRef, BoundaryChart, BoundaryRef,
    CompletionCandidate, CompletionCandidateCatalog, CompletionCandidateRef,
    DeclaredSupportClosure, DischargeMode, EventRef, FiniteAnswerBindingError, FiniteDecoder,
    FiniteDecoderCatalog, FiniteDecoderEntry, FiniteDecoderRef, FiniteSupportedAnswerError,
    FormulaArtifact, FormulaCatalog, FormulaRef, GrainRef, HorizonRef, IProgArtifact, IProgCatalog,
    IProgCheckError, IProgIR, IProgRef, ObservationResultCatalog, OpenPort, OpenQuery,
    OpenQueryCatalog, OperatorOccurrenceCatalog, PortBinding, ProbeContractRef, ProbeOperator,
    ProbeOperatorRef, ProgramBinding, ProvenanceRef, QueryRef, RawReturn, RawReturnCatalog,
    RawReturnRef, RelationBodyIR, RelationCatalog, RelationPort, RelationRef, RelationSchema,
    RelationSignature, RelationUse, RelationUseContext, RelationUseRef, RelationUseSupportCatalog,
    ResolutionCatalog, ResolutionPath, ResolutionPathIR, ResolutionPathRef, RouteRef, ScopeRef,
    StateRef, SupportEnvironmentArtifact, SupportEnvironmentCatalog, SupportEnvironmentRef,
    SupportSubjectRef, SurfacePlan, TyIR, TypeArtifact, TypeCatalog, TypeFamilyRef, TypeRef,
    TypeSymbol, TypedForm, TypedFormRef, admit_finite_supported_answers,
    bind_finite_ask_continuation, decode_actual_event, match_decoded_observation_use,
    standing_from_declared_support,
};
use ic_runtime::{
    AdmittedResumeError, BasicBlock, BlockTarget, ContinuationLowering, FiniteProbeReplayError,
    MachineStep, ProbeDispatchContext, ProbeProvider, ProgramIR, ProviderReturn, ReplayObservation,
    RuntimeCatalog, Terminator, dispatch_probe, replay_completed_finite_probe,
};
use ic_store::{ArtifactStore, DispatchToken};

#[derive(Default)]
struct Catalog {
    types: BTreeMap<TypeRef, TypeArtifact>,
    forms: BTreeMap<TypedFormRef, TypedForm>,
    schemas: BTreeMap<RelationRef, RelationSchema>,
    signatures: BTreeMap<RelationRef, RelationSignature>,
    queries: BTreeMap<QueryRef, OpenQuery>,
    candidates: BTreeMap<CompletionCandidateRef, CompletionCandidate>,
    relation_uses: BTreeMap<RelationUseRef, RelationUse>,
    raw_returns: BTreeMap<RawReturnRef, RawReturn>,
    decoders: BTreeMap<FiniteDecoderRef, FiniteDecoder>,
    paths: BTreeMap<ResolutionPathRef, ResolutionPath>,
    charts: BTreeMap<BoundaryRef, BoundaryChart>,
    operators: BTreeMap<ProbeOperatorRef, ProbeOperator>,
    events: BTreeMap<EventRef, ActualEvent>,
    support: BTreeMap<SupportEnvironmentRef, SupportEnvironmentArtifact>,
    programs: BTreeMap<IProgRef, IProgArtifact>,
}

impl Catalog {
    fn insert_type(&mut self, value: TypeArtifact) -> TypeRef {
        let reference = value.type_ref().expect("type must encode");
        self.types.insert(reference, value);
        reference
    }

    fn insert_form(&mut self, value: TypedForm) -> TypedFormRef {
        let reference = value.typed_form_ref().expect("form must encode");
        self.forms.insert(reference, value);
        reference
    }

    fn insert_schema(&mut self, value: RelationSchema) -> RelationRef {
        let reference = value.relation_ref().expect("relation must encode");
        self.signatures
            .insert(reference, value.signature().expect("signature must encode"));
        self.schemas.insert(reference, value);
        reference
    }

    fn insert_query(&mut self, value: OpenQuery) -> QueryRef {
        let reference = value.query_ref().expect("query must encode");
        self.queries.insert(reference, value);
        reference
    }

    fn insert_candidate(&mut self, value: CompletionCandidate) -> CompletionCandidateRef {
        let reference = value
            .completion_candidate_ref()
            .expect("candidate must encode");
        self.candidates.insert(reference, value);
        reference
    }

    fn insert_relation_use(&mut self, value: RelationUse) -> RelationUseRef {
        let reference = value.relation_use_ref().expect("use must encode");
        self.relation_uses.insert(reference, value);
        reference
    }

    fn insert_raw_return(&mut self, value: RawReturn) -> RawReturnRef {
        let reference = value.raw_return_ref().expect("raw return must encode");
        self.raw_returns.insert(reference, value);
        reference
    }

    fn insert_decoder(&mut self, value: FiniteDecoder) -> FiniteDecoderRef {
        let reference = value.finite_decoder_ref().expect("decoder must encode");
        self.decoders.insert(reference, value);
        reference
    }

    fn insert_path(&mut self, value: ResolutionPath) -> ResolutionPathRef {
        let reference = value.resolution_path_ref().expect("path must encode");
        self.paths.insert(reference, value);
        reference
    }

    fn insert_support(&mut self, value: SupportEnvironmentArtifact) -> SupportEnvironmentRef {
        let reference = value
            .support_environment_ref()
            .expect("support environment must encode");
        self.support.insert(reference, value);
        reference
    }

    fn insert_program(&mut self, value: IProgArtifact) -> IProgRef {
        let reference = value.iprog_ref().expect("program must encode");
        self.programs.insert(reference, value);
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
    fn resolve_finite_decoder(&self, reference: FiniteDecoderRef) -> Option<FiniteDecoder> {
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

impl OperatorOccurrenceCatalog for Catalog {
    fn resolve_actual_event(&self, reference: EventRef) -> Option<ActualEvent> {
        self.events.get(&reference).cloned()
    }
}

impl ObservationResultCatalog for Catalog {
    fn resolve_relation_use(&self, reference: RelationUseRef) -> Option<RelationUse> {
        self.relation_uses.get(&reference).cloned()
    }
}

impl SupportEnvironmentCatalog for Catalog {
    fn resolve_claim(&self, _reference: ic_core::ClaimRef) -> Option<ic_core::ClaimArtifact> {
        None
    }

    fn resolve_support_environment(
        &self,
        reference: SupportEnvironmentRef,
    ) -> Option<SupportEnvironmentArtifact> {
        self.support.get(&reference).cloned()
    }
}

impl RelationUseSupportCatalog for Catalog {
    fn resolve_relation_use(&self, reference: RelationUseRef) -> Option<RelationUse> {
        self.relation_uses.get(&reference).cloned()
    }
}

impl IProgCatalog for Catalog {
    fn resolve_iprog(&self, reference: IProgRef) -> Option<IProgArtifact> {
        self.programs.get(&reference).cloned()
    }
}

impl RuntimeCatalog for Catalog {
    fn resolve_typed_form(&self, reference: TypedFormRef) -> Option<TypedForm> {
        self.forms.get(&reference).copied()
    }
}

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn fixture_envelope(payload: &[u8]) -> ArtifactEnvelope {
    ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("fixture").expect("fixture kind must be valid"),
        1,
        payload.to_vec(),
    )
}

async fn stored_ref(store: &ArtifactStore, payload: &[u8]) -> ArtifactRef {
    store
        .insert(&fixture_envelope(payload))
        .await
        .expect("fixture dependency must persist")
}

async fn persist(
    store: &ArtifactStore,
    envelope: &ArtifactEnvelope,
    references: &[ArtifactRef],
) -> ArtifactRef {
    store
        .insert_referencing(envelope, references)
        .await
        .expect("canonical replay artifact dependencies must persist")
}

async fn load_envelope(store: &ArtifactStore, reference: ArtifactRef) -> ArtifactEnvelope {
    store
        .get(reference)
        .await
        .expect("replay artifact lookup must succeed")
        .expect("replay artifact must exist")
}

#[derive(Clone, Copy)]
struct ColdReplayRoots {
    token: DispatchToken,
    unit: TypeRef,
    raw_type: TypeRef,
    answer_a: TypedFormRef,
    answer_b: TypedFormRef,
    relation: RelationRef,
    query: QueryRef,
    wrong_query: QueryRef,
    candidate_a: CompletionCandidateRef,
    candidate_b: CompletionCandidateRef,
    observation_a: RelationUseRef,
    observation_b: RelationUseRef,
    support: SupportEnvironmentRef,
    decoded_decoder: FiniteDecoderRef,
    undefined_decoder: FiniteDecoderRef,
    unknown_decoder: FiniteDecoderRef,
    decoded_path: ResolutionPathRef,
    undefined_path: ResolutionPathRef,
    unknown_path: ResolutionPathRef,
    boundary: BoundaryRef,
    operator: ProbeOperatorRef,
    rival_operator: ProbeOperatorRef,
    source: IProgRef,
    wrong_source: IProgRef,
    capture_source: IProgRef,
    continuation: IProgRef,
    event: EventRef,
    raw_return: RawReturnRef,
    compiler_version: ArtifactRef,
}

struct CountingProvider {
    calls: Arc<AtomicUsize>,
    expected_body: ArtifactRef,
}

impl ProbeProvider for CountingProvider {
    type Error = Infallible;

    fn dispatch(&mut self, request: &BackendRequest) -> Result<ProviderReturn, Self::Error> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        assert_eq!(request.request_body(), self.expected_body);
        Ok(ProviderReturn::new(vec![0x41, 0x00, 0xff]))
    }
}

async fn persisted_cold_replay_fixture() -> (PathBuf, ColdReplayRoots, Arc<AtomicUsize>) {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    let path = env::temp_dir().join(format!(
        "inquiry-calculus-cold-replay-{}-{nonce}.sqlite",
        std::process::id()
    ));
    let url = format!("sqlite://{}", path.to_string_lossy().replace('\\', "/"));
    let store = ArtifactStore::open(&url)
        .await
        .expect("file-backed replay store must open");
    store.migrate().await.expect("migrations must apply");
    let mut catalog = Catalog::default();

    let binding = BindingVersionRef::from_artifact_ref(stored_ref(&store, b"binding").await);
    let unit_value = TypeArtifact::new(binding, TyIR::Unit);
    let unit = TypeRef::from_artifact_ref(
        persist(
            &store,
            &unit_value.envelope().expect("unit type must encode"),
            &unit_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_type(unit_value), unit);
    let raw_type_value = TypeArtifact::new(binding, TyIR::Raw(unit));
    let raw_type = TypeRef::from_artifact_ref(
        persist(
            &store,
            &raw_type_value.envelope().expect("raw type must encode"),
            &raw_type_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_type(raw_type_value), raw_type);

    let answer_a_value = TypedForm::new(binding, unit, stored_ref(&store, b"answer-a").await);
    let answer_a = TypedFormRef::from_artifact_ref(
        persist(
            &store,
            &answer_a_value.envelope().expect("answer A must encode"),
            &answer_a_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_form(answer_a_value), answer_a);
    let answer_b_value = TypedForm::new(binding, unit, stored_ref(&store, b"answer-b").await);
    let answer_b = TypedFormRef::from_artifact_ref(
        persist(
            &store,
            &answer_b_value.envelope().expect("answer B must encode"),
            &answer_b_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_form(answer_b_value), answer_b);

    let relation_value = RelationSchema::new(
        binding,
        vec![RelationPort::new(
            TypeSymbol::new("answer").expect("answer port must be valid"),
            unit,
        )],
        RelationBodyIR::BindingNative {
            contract: stored_ref(&store, b"relation-contract").await,
        },
        Vec::new(),
        Vec::new(),
    );
    let relation = RelationRef::from_artifact_ref(
        persist(
            &store,
            &relation_value.envelope().expect("relation must encode"),
            &relation_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_schema(relation_value), relation);

    let raw = RawReturn::new(vec![0x41, 0x00, 0xff]);
    let raw_return = RawReturnRef::from_artifact_ref(
        store
            .insert(&raw.envelope().expect("raw return must encode"))
            .await
            .expect("raw content may exist before it is named by an event"),
    );
    catalog.insert_raw_return(raw.clone());
    let scope = ScopeRef::from_artifact_ref(stored_ref(&store, b"scope").await);
    let applicability =
        ApplicabilityRef::from_artifact_ref(stored_ref(&store, b"applicability").await);
    let support_value = SupportEnvironmentArtifact::new(
        SupportSubjectRef::Relation(relation),
        Vec::new(),
        vec![raw_return],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        applicability,
        scope,
    )
    .expect("support environment must encode");
    let support = SupportEnvironmentRef::from_artifact_ref(
        persist(
            &store,
            &support_value.envelope().expect("support must encode"),
            &support_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_support(support_value), support);

    let grain = GrainRef::from_artifact_ref(stored_ref(&store, b"grain").await);
    let horizon = HorizonRef::from_artifact_ref(stored_ref(&store, b"horizon").await);
    let context = RelationUseContext::new(
        scope,
        applicability,
        grain,
        horizon,
        DischargeMode::Probe,
        support.as_support_ref(),
        None,
    );
    let answer_port = TypeSymbol::new("answer").expect("answer port must be valid");
    let query_value = OpenQuery::new(
        relation,
        Vec::new(),
        vec![OpenPort::new(answer_port.clone(), DischargeMode::Probe)],
        context,
    );
    let candidate_a_value = query_value
        .plug(
            vec![PortBinding::new(answer_port.clone(), answer_a)],
            &catalog,
        )
        .expect("answer A must fill the query");
    let candidate_b_value = query_value
        .plug(
            vec![PortBinding::new(answer_port.clone(), answer_b)],
            &catalog,
        )
        .expect("answer B must fill the query");
    let query = QueryRef::from_artifact_ref(
        persist(
            &store,
            &query_value.envelope().expect("query must encode"),
            &query_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_query(query_value), query);
    let candidate_a = CompletionCandidateRef::from_artifact_ref(
        persist(
            &store,
            &candidate_a_value
                .envelope()
                .expect("candidate A must encode"),
            &candidate_a_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_candidate(candidate_a_value), candidate_a);
    let candidate_b = CompletionCandidateRef::from_artifact_ref(
        persist(
            &store,
            &candidate_b_value
                .envelope()
                .expect("candidate B must encode"),
            &candidate_b_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_candidate(candidate_b_value), candidate_b);

    let wrong_context = RelationUseContext::new(
        scope,
        applicability,
        grain,
        HorizonRef::from_artifact_ref(stored_ref(&store, b"wrong-horizon").await),
        DischargeMode::Probe,
        support.as_support_ref(),
        None,
    );
    let wrong_query_value = OpenQuery::new(
        relation,
        Vec::new(),
        vec![OpenPort::new(answer_port.clone(), DischargeMode::Probe)],
        wrong_context,
    );
    let wrong_query = QueryRef::from_artifact_ref(
        persist(
            &store,
            &wrong_query_value
                .envelope()
                .expect("wrong query must encode"),
            &wrong_query_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_query(wrong_query_value), wrong_query);

    let observation_a_value = RelationUse::new(
        relation,
        vec![PortBinding::new(answer_port.clone(), answer_a)],
        context,
    );
    let observation_a = RelationUseRef::from_artifact_ref(
        persist(
            &store,
            &observation_a_value
                .envelope()
                .expect("observation A must encode"),
            &observation_a_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_relation_use(observation_a_value),
        observation_a
    );
    let observation_b_value = RelationUse::new(
        relation,
        vec![PortBinding::new(answer_port, answer_b)],
        context,
    );
    let observation_b = RelationUseRef::from_artifact_ref(
        persist(
            &store,
            &observation_b_value
                .envelope()
                .expect("observation B must encode"),
            &observation_b_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_relation_use(observation_b_value),
        observation_b
    );

    let decoded_decoder_value = FiniteDecoder::new(
        query,
        raw_type,
        vec![FiniteDecoderEntry::Decoded {
            raw_return,
            candidates: vec![candidate_a, candidate_b],
        }],
    )
    .expect("decoded finite decoder must encode");
    let decoded_decoder = FiniteDecoderRef::from_artifact_ref(
        persist(
            &store,
            &decoded_decoder_value
                .envelope()
                .expect("decoded decoder must encode"),
            &decoded_decoder_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_decoder(decoded_decoder_value),
        decoded_decoder
    );
    let undefined_decoder_value = FiniteDecoder::new(
        query,
        raw_type,
        vec![FiniteDecoderEntry::Undefined { raw_return }],
    )
    .expect("undefined decoder must encode");
    let undefined_decoder = FiniteDecoderRef::from_artifact_ref(
        persist(
            &store,
            &undefined_decoder_value
                .envelope()
                .expect("undefined decoder must encode"),
            &undefined_decoder_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_decoder(undefined_decoder_value),
        undefined_decoder
    );
    let unknown_decoder_value =
        FiniteDecoder::new(query, raw_type, Vec::new()).expect("open decoder must encode");
    let unknown_decoder = FiniteDecoderRef::from_artifact_ref(
        persist(
            &store,
            &unknown_decoder_value
                .envelope()
                .expect("unknown decoder must encode"),
            &unknown_decoder_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_decoder(unknown_decoder_value),
        unknown_decoder
    );

    let decoded_path_value = ResolutionPath::new(
        raw_type,
        unit,
        ResolutionPathIR::Decode {
            decoder: decoded_decoder.as_decoder_ref(),
        },
    );
    let decoded_path = ResolutionPathRef::from_artifact_ref(
        persist(
            &store,
            &decoded_path_value
                .envelope()
                .expect("decoded path must encode"),
            &decoded_path_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_path(decoded_path_value), decoded_path);
    let undefined_path_value = ResolutionPath::new(
        raw_type,
        unit,
        ResolutionPathIR::Decode {
            decoder: undefined_decoder.as_decoder_ref(),
        },
    );
    let undefined_path = ResolutionPathRef::from_artifact_ref(
        persist(
            &store,
            &undefined_path_value
                .envelope()
                .expect("undefined path must encode"),
            &undefined_path_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_path(undefined_path_value), undefined_path);
    let unknown_path_value = ResolutionPath::new(
        raw_type,
        unit,
        ResolutionPathIR::Decode {
            decoder: unknown_decoder.as_decoder_ref(),
        },
    );
    let unknown_path = ResolutionPathRef::from_artifact_ref(
        persist(
            &store,
            &unknown_path_value
                .envelope()
                .expect("unknown path must encode"),
            &unknown_path_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_path(unknown_path_value), unknown_path);

    let chart_value = BoundaryChart::new(
        query,
        unit,
        unit,
        unit,
        relation,
        relation,
        ic_core::DeterminationPresentationRef::from_artifact_ref(
            stored_ref(&store, b"determination").await,
        ),
        None,
        Vec::new(),
        Vec::new(),
        observation_a,
        FormulaRef::from_artifact_ref(stored_ref(&store, b"compatibility").await),
        None,
        grain,
        horizon,
    );
    let boundary = BoundaryRef::from_artifact_ref(
        persist(
            &store,
            &chart_value.envelope().expect("boundary must encode"),
            &chart_value.referenced_artifacts(),
        )
        .await,
    );
    catalog.charts.insert(boundary, chart_value);

    let active_view = stored_ref(&store, b"active-view").await;
    let backend = stored_ref(&store, b"backend").await;
    let executable_code = stored_ref(&store, b"executable-code").await;
    let decoder_contract = stored_ref(&store, b"decoder-contract").await;
    let probe_contract =
        ProbeContractRef::from_artifact_ref(stored_ref(&store, b"probe-contract").await);
    let compiler_version = stored_ref(&store, b"compiler-version").await;
    let operator_value = ProbeOperator::new(
        query,
        boundary,
        active_view,
        backend,
        executable_code,
        raw_type,
        decoder_contract,
        probe_contract,
        compiler_version,
    );
    let operator = ProbeOperatorRef::from_artifact_ref(
        persist(
            &store,
            &operator_value.envelope().expect("operator must encode"),
            &operator_value.referenced_artifacts(),
        )
        .await,
    );
    catalog.operators.insert(operator, operator_value.clone());
    let rival_operator_value = ProbeOperator::new(
        query,
        boundary,
        stored_ref(&store, b"rival-active-view").await,
        backend,
        executable_code,
        raw_type,
        decoder_contract,
        probe_contract,
        compiler_version,
    );
    let rival_operator = ProbeOperatorRef::from_artifact_ref(
        persist(
            &store,
            &rival_operator_value
                .envelope()
                .expect("rival operator must encode"),
            &rival_operator_value.referenced_artifacts(),
        )
        .await,
    );
    catalog
        .operators
        .insert(rival_operator, rival_operator_value);

    let plan_value = SurfacePlan::new(
        operator,
        query,
        boundary,
        active_view,
        executable_code,
        probe_contract,
        stored_ref(&store, b"renderer-version").await,
        stored_ref(&store, b"rendered-body").await,
    );
    let plan = ic_core::SurfacePlanRef::from_artifact_ref(
        persist(
            &store,
            &plan_value.envelope().expect("surface plan must encode"),
            &plan_value.referenced_artifacts(),
        )
        .await,
    );
    let backend_version = stored_ref(&store, b"backend-version").await;
    let request_body = stored_ref(&store, b"request-body").await;
    let request_value = BackendRequest::new(
        operator,
        plan,
        query,
        boundary,
        backend,
        executable_code,
        compiler_version,
        backend_version,
        request_body,
    );
    let request = ic_core::BackendRequestRef::from_artifact_ref(
        persist(
            &store,
            &request_value.envelope().expect("request must encode"),
            &request_value.referenced_artifacts(),
        )
        .await,
    );

    let continuation_value = IProgArtifact::new(unit, IProgIR::Return { value: answer_a });
    let continuation = IProgRef::from_artifact_ref(
        persist(
            &store,
            &continuation_value
                .envelope()
                .expect("continuation must encode"),
            &continuation_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_program(continuation_value), continuation);
    let answer_slot = TypeSymbol::new("answer_set").expect("answer slot must be valid");
    let source_value = IProgArtifact::new(
        unit,
        IProgIR::Ask {
            question: query,
            environment: Vec::new(),
            answer_slot: answer_slot.clone(),
            continuation,
        },
    );
    let source = IProgRef::from_artifact_ref(
        persist(
            &store,
            &source_value.envelope().expect("source Ask must encode"),
            &source_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_program(source_value), source);
    let wrong_source_value = IProgArtifact::new(
        unit,
        IProgIR::Ask {
            question: wrong_query,
            environment: Vec::new(),
            answer_slot: answer_slot.clone(),
            continuation,
        },
    );
    let wrong_source = IProgRef::from_artifact_ref(
        persist(
            &store,
            &wrong_source_value
                .envelope()
                .expect("wrong source Ask must encode"),
            &wrong_source_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_program(wrong_source_value), wrong_source);
    let capture_source_value = IProgArtifact::new(
        unit,
        IProgIR::Ask {
            question: query,
            environment: vec![ProgramBinding::new(answer_slot.clone(), answer_b)],
            answer_slot,
            continuation,
        },
    );
    let capture_source = IProgRef::from_artifact_ref(
        persist(
            &store,
            &capture_source_value
                .envelope()
                .expect("capture source must encode"),
            &capture_source_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_program(capture_source_value), capture_source);

    let state_before = StateRef::from_artifact_ref(stored_ref(&store, b"state-before").await);
    let state_after = StateRef::from_artifact_ref(stored_ref(&store, b"state-after").await);
    let route = RouteRef::from_artifact_ref(stored_ref(&store, b"route").await);
    let provenance = ProvenanceRef::from_artifact_ref(stored_ref(&store, b"provenance").await);
    let runtime = ProgramIR::new(
        unit,
        BlockTarget::new(0),
        vec![
            BasicBlock::new(
                BlockTarget::new(0),
                Terminator::Probe {
                    operator,
                    resume: BlockTarget::new(1),
                },
            ),
            BasicBlock::new(BlockTarget::new(1), Terminator::Return { value: answer_a }),
        ],
    );
    runtime
        .verify(&catalog)
        .expect("live runtime lowering must verify");
    let MachineStep::Suspended(suspension) = runtime
        .step(runtime.start())
        .expect("source Ask lowering must reach the probe")
    else {
        panic!("source Ask lowering must suspend at its Probe")
    };
    let dispatch_context = ProbeDispatchContext::new(
        None,
        state_before,
        None,
        state_after,
        grain,
        route,
        binding,
        provenance,
    );
    let token = DispatchToken::from_bytes([0xc1; 32]);
    let provider_calls = Arc::new(AtomicUsize::new(0));
    let mut provider = CountingProvider {
        calls: Arc::clone(&provider_calls),
        expected_body: request_body,
    };
    let actual = dispatch_probe(
        &store,
        suspension,
        token,
        request,
        dispatch_context,
        &mut provider,
    )
    .await
    .expect("live cycle must dispatch and preserve one actual return");
    assert_eq!(provider_calls.load(Ordering::SeqCst), 1);
    assert_eq!(actual.raw_return(), &raw);
    assert_eq!(actual.raw_return_ref(), raw_return);
    let event = actual.event_ref();
    catalog.events.insert(event, actual.event().clone());

    let standing = standing_from_declared_support(
        Vec::new(),
        &[DeclaredSupportClosure::for_subjects(
            support,
            Vec::new(),
            true,
            true,
            false,
        )],
        &catalog,
    )
    .expect("live exact support closure must reconstruct standing");
    let decoded_decoder_value = catalog
        .resolve_finite_decoder(decoded_decoder)
        .expect("live finite decoder must remain available");
    let source_value = catalog
        .resolve_iprog(source)
        .expect("live source Ask must remain available");
    let observations = [
        ReplayObservation::new(candidate_a, observation_a),
        ReplayObservation::new(candidate_b, observation_b),
    ];
    let live = replay_completed_finite_probe(
        &store,
        token,
        &decoded_decoder_value,
        decoded_path,
        &observations,
        &standing,
        &source_value,
        suspension,
        ContinuationLowering::new(continuation, BlockTarget::new(1)),
        &runtime,
        &catalog,
    )
    .await
    .expect("live actual return must admit and resume the source continuation");
    assert_eq!(live.actuality().event_ref(), event);
    assert!(matches!(
        runtime
            .step(live.resumption().state())
            .expect("live admitted continuation must execute"),
        MachineStep::Returned(value) if value == answer_a
    ));
    assert_eq!(provider_calls.load(Ordering::SeqCst), 1);
    store.close().await;

    (
        path,
        ColdReplayRoots {
            token,
            unit,
            raw_type,
            answer_a,
            answer_b,
            relation,
            query,
            wrong_query,
            candidate_a,
            candidate_b,
            observation_a,
            observation_b,
            support,
            decoded_decoder,
            undefined_decoder,
            unknown_decoder,
            decoded_path,
            undefined_path,
            unknown_path,
            boundary,
            operator,
            rival_operator,
            source,
            wrong_source,
            capture_source,
            continuation,
            event,
            raw_return,
            compiler_version,
        },
        provider_calls,
    )
}

async fn load_cold_replay_catalog(store: &ArtifactStore, roots: ColdReplayRoots) -> Catalog {
    let mut catalog = Catalog::default();
    for reference in [roots.unit, roots.raw_type] {
        let value =
            TypeArtifact::from_envelope(&load_envelope(store, reference.as_artifact_ref()).await)
                .expect("persisted type must decode");
        assert_eq!(catalog.insert_type(value), reference);
    }
    for reference in [roots.answer_a, roots.answer_b] {
        let value =
            TypedForm::from_envelope(&load_envelope(store, reference.as_artifact_ref()).await)
                .expect("persisted form must decode");
        assert_eq!(catalog.insert_form(value), reference);
    }
    let relation = RelationSchema::from_envelope(
        &load_envelope(store, roots.relation.as_artifact_ref()).await,
    )
    .expect("persisted relation must decode");
    assert_eq!(catalog.insert_schema(relation), roots.relation);
    for reference in [roots.query, roots.wrong_query] {
        let value =
            OpenQuery::from_envelope(&load_envelope(store, reference.as_artifact_ref()).await)
                .expect("persisted query must decode");
        assert_eq!(catalog.insert_query(value), reference);
    }
    for reference in [roots.candidate_a, roots.candidate_b] {
        let value = CompletionCandidate::from_envelope(
            &load_envelope(store, reference.as_artifact_ref()).await,
        )
        .expect("persisted candidate must decode");
        assert_eq!(catalog.insert_candidate(value), reference);
    }
    for reference in [roots.observation_a, roots.observation_b] {
        let value =
            RelationUse::from_envelope(&load_envelope(store, reference.as_artifact_ref()).await)
                .expect("persisted observation use must decode");
        assert_eq!(catalog.insert_relation_use(value), reference);
    }
    let raw_return =
        RawReturn::from_envelope(&load_envelope(store, roots.raw_return.as_artifact_ref()).await)
            .expect("persisted raw return must decode");
    assert_eq!(catalog.insert_raw_return(raw_return), roots.raw_return);
    for reference in [
        roots.decoded_decoder,
        roots.undefined_decoder,
        roots.unknown_decoder,
    ] {
        let value =
            FiniteDecoder::from_envelope(&load_envelope(store, reference.as_artifact_ref()).await)
                .expect("persisted finite decoder must decode");
        assert_eq!(catalog.insert_decoder(value), reference);
    }
    for reference in [roots.decoded_path, roots.undefined_path, roots.unknown_path] {
        let value =
            ResolutionPath::from_envelope(&load_envelope(store, reference.as_artifact_ref()).await)
                .expect("persisted resolution path must decode");
        assert_eq!(catalog.insert_path(value), reference);
    }
    let chart =
        BoundaryChart::from_envelope(&load_envelope(store, roots.boundary.as_artifact_ref()).await)
            .expect("persisted boundary chart must decode");
    assert_eq!(chart.boundary_ref().ok(), Some(roots.boundary));
    catalog.charts.insert(roots.boundary, chart);
    for reference in [roots.operator, roots.rival_operator] {
        let value =
            ProbeOperator::from_envelope(&load_envelope(store, reference.as_artifact_ref()).await)
                .expect("persisted operator must decode");
        assert_eq!(value.probe_operator_ref().ok(), Some(reference));
        catalog.operators.insert(reference, value);
    }
    let event = store
        .get_actual_event(roots.event)
        .await
        .expect("persisted event must recheck")
        .expect("persisted event must exist");
    assert_eq!(event.event_ref().ok(), Some(roots.event));
    catalog.events.insert(roots.event, event);
    let support = SupportEnvironmentArtifact::from_envelope(
        &load_envelope(store, roots.support.as_artifact_ref()).await,
    )
    .expect("persisted support environment must decode");
    assert_eq!(catalog.insert_support(support), roots.support);
    for reference in [
        roots.continuation,
        roots.source,
        roots.wrong_source,
        roots.capture_source,
    ] {
        let value =
            IProgArtifact::from_envelope(&load_envelope(store, reference.as_artifact_ref()).await)
                .expect("persisted source program must decode");
        assert_eq!(catalog.insert_program(value), reference);
    }
    catalog
}

#[test]
fn admitted_answer_resumption_preserves_cold_replay_provenance_and_exact_lowering() {
    let mut catalog = Catalog::default();
    let binding = BindingVersionRef::from_artifact_ref(artifact(0x10));
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let raw_type = catalog.insert_type(TypeArtifact::new(binding, TyIR::Raw(unit)));
    let answer = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x11)));
    let relation = catalog.insert_schema(RelationSchema::new(
        binding,
        vec![RelationPort::new(
            TypeSymbol::new("answer").expect("port name must be valid"),
            unit,
        )],
        RelationBodyIR::BindingNative {
            contract: artifact(0x12),
        },
        Vec::new(),
        Vec::new(),
    ));
    let raw = catalog.insert_raw_return(RawReturn::new(vec![0x13]));
    let scope = ScopeRef::from_artifact_ref(artifact(0x14));
    let applicability = ApplicabilityRef::from_artifact_ref(artifact(0x15));
    let support = catalog.insert_support(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Relation(relation),
            Vec::new(),
            vec![raw],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        )
        .expect("support environment must encode"),
    );
    let grain = GrainRef::from_artifact_ref(artifact(0x16));
    let horizon = HorizonRef::from_artifact_ref(artifact(0x17));
    let context = RelationUseContext::new(
        scope,
        applicability,
        grain,
        horizon,
        DischargeMode::Probe,
        support.as_support_ref(),
        None,
    );
    let answer_port = TypeSymbol::new("answer").expect("port name must be valid");
    let query_value = OpenQuery::new(
        relation,
        Vec::new(),
        vec![OpenPort::new(answer_port.clone(), DischargeMode::Probe)],
        context,
    );
    let candidate = query_value
        .plug(
            vec![PortBinding::new(answer_port.clone(), answer)],
            &catalog,
        )
        .expect("candidate must fill the query");
    let query = catalog.insert_query(query_value);
    let candidate = catalog.insert_candidate(candidate);
    let observation = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![PortBinding::new(answer_port, answer)],
        context,
    ));
    let decoder = FiniteDecoder::new(
        query,
        raw_type,
        vec![FiniteDecoderEntry::Decoded {
            raw_return: raw,
            candidates: vec![candidate],
        }],
    )
    .expect("finite decoder must encode");
    let decoder_ref = catalog.insert_decoder(decoder.clone());
    let path = catalog.insert_path(ResolutionPath::new(
        raw_type,
        unit,
        ResolutionPathIR::Decode {
            decoder: decoder_ref.as_decoder_ref(),
        },
    ));
    let chart = BoundaryChart::new(
        query,
        unit,
        unit,
        unit,
        relation,
        relation,
        ic_core::DeterminationPresentationRef::from_artifact_ref(artifact(0x18)),
        None,
        Vec::new(),
        Vec::new(),
        observation,
        FormulaRef::from_artifact_ref(artifact(0x19)),
        None,
        grain,
        horizon,
    );
    let boundary = chart.boundary_ref().expect("chart must encode");
    catalog.charts.insert(boundary, chart);
    let operator_value = ProbeOperator::new(
        query,
        boundary,
        artifact(0x1a),
        artifact(0x1b),
        artifact(0x1c),
        raw_type,
        artifact(0x1d),
        ProbeContractRef::from_artifact_ref(artifact(0x1e)),
        artifact(0x1f),
    );
    let operator = operator_value
        .probe_operator_ref()
        .expect("operator must encode");
    catalog.operators.insert(operator, operator_value);
    let event_value = ActualEvent::new(
        None,
        StateRef::from_artifact_ref(artifact(0x20)),
        query,
        boundary,
        None,
        operator,
        raw,
        StateRef::from_artifact_ref(artifact(0x21)),
        grain,
        RouteRef::from_artifact_ref(artifact(0x22)),
        binding,
        artifact(0x23),
        ProvenanceRef::from_artifact_ref(artifact(0x24)),
    );
    let event = event_value.event_ref().expect("event must encode");
    catalog.events.insert(event, event_value.clone());
    let ActualDecodeResult::Decoded(decoded) =
        decode_actual_event(&event_value, &decoder, path, &catalog)
            .expect("preserved event must replay through its decoder")
    else {
        panic!("listed return must decode")
    };
    let observation = match_decoded_observation_use(&decoded, candidate, observation, &catalog)
        .expect("decoded candidate must match its Probe observation");
    let standing = standing_from_declared_support(
        Vec::new(),
        &[DeclaredSupportClosure::for_subjects(
            support,
            Vec::new(),
            true,
            true,
            false,
        )],
        &catalog,
    )
    .expect("the exact relation route must close");
    let admitted = admit_finite_supported_answers(decoded, vec![observation], &standing, &catalog)
        .expect("the event-linked answer must admit");
    let continuation =
        catalog.insert_program(IProgArtifact::new(unit, IProgIR::Return { value: answer }));
    let ask = IProgArtifact::new(
        unit,
        IProgIR::Ask {
            question: query,
            environment: Vec::new(),
            answer_slot: TypeSymbol::new("answer_set").expect("slot must be valid"),
            continuation,
        },
    );
    let bound = bind_finite_ask_continuation(&ask, admitted, &catalog)
        .expect("the admitted set must bind to the checked source continuation");

    let runtime = ProgramIR::new(
        unit,
        BlockTarget::new(0),
        vec![
            BasicBlock::new(
                BlockTarget::new(0),
                Terminator::Probe {
                    operator,
                    resume: BlockTarget::new(1),
                },
            ),
            BasicBlock::new(BlockTarget::new(1), Terminator::Return { value: answer }),
            BasicBlock::new(BlockTarget::new(2), Terminator::Return { value: answer }),
        ],
    );
    runtime
        .verify(&catalog)
        .expect("runtime program must verify");
    let MachineStep::Suspended(suspension) = runtime
        .step(runtime.start())
        .expect("entry probe must suspend")
    else {
        panic!("entry must suspend")
    };
    let resumption = suspension
        .resume_admitted(
            bound.clone(),
            ContinuationLowering::new(continuation, BlockTarget::new(1)),
            &runtime,
        )
        .expect("the exact source/operator/target lowering must resume");
    assert_eq!(resumption.state().target(), BlockTarget::new(1));
    assert_eq!(resumption.event(), event);
    assert_eq!(resumption.raw_return(), raw);
    assert_eq!(resumption.binding().continuation(), continuation);
    assert_eq!(resumption.binding().answer().candidates(), [candidate]);

    assert!(matches!(
        suspension.resume_admitted(
            bound.clone(),
            ContinuationLowering::new(
                IProgRef::from_artifact_ref(artifact(0x30)),
                BlockTarget::new(1),
            ),
            &runtime,
        ),
        Err(AdmittedResumeError::ContinuationMismatch { .. })
    ));
    assert!(matches!(
        suspension.resume_admitted(
            bound,
            ContinuationLowering::new(continuation, BlockTarget::new(2)),
            &runtime,
        ),
        Err(AdmittedResumeError::ResumeTargetMismatch { .. })
    ));
}

#[tokio::test]
async fn finite_probe_executes_once_and_cold_replays_with_distinct_residuals() {
    let (path, roots, provider_calls) = persisted_cold_replay_fixture().await;
    let url = format!("sqlite://{}", path.to_string_lossy().replace('\\', "/"));
    let store = ArtifactStore::open(&url)
        .await
        .expect("cold replay store must reopen");
    store
        .migrate()
        .await
        .expect("embedded migrations must remain repeatable");
    let catalog = load_cold_replay_catalog(&store, roots).await;
    let source = catalog
        .resolve_iprog(roots.source)
        .expect("source Ask must reload");
    let wrong_source = catalog
        .resolve_iprog(roots.wrong_source)
        .expect("wrong source Ask must reload");
    let capture_source = catalog
        .resolve_iprog(roots.capture_source)
        .expect("capture source Ask must reload");
    let decoded_decoder = catalog
        .resolve_finite_decoder(roots.decoded_decoder)
        .expect("decoded finite decoder must reload");
    let undefined_decoder = catalog
        .resolve_finite_decoder(roots.undefined_decoder)
        .expect("undefined finite decoder must reload");
    let unknown_decoder = catalog
        .resolve_finite_decoder(roots.unknown_decoder)
        .expect("unknown finite decoder must reload");
    let standing = standing_from_declared_support(
        Vec::new(),
        &[DeclaredSupportClosure::for_subjects(
            roots.support,
            Vec::new(),
            true,
            true,
            false,
        )],
        &catalog,
    )
    .expect("fresh exact support closure must reconstruct standing");
    let no_standing = standing_from_declared_support(Vec::new(), &[], &catalog)
        .expect("empty declared support must produce empty standing");

    let IProgIR::Ask {
        question,
        continuation,
        ..
    } = source.program()
    else {
        panic!("persisted source must remain an Ask")
    };
    assert_eq!(*question, roots.query);
    assert_eq!(*continuation, roots.continuation);
    let operator = catalog
        .resolve_probe_operator(roots.operator)
        .expect("compiled operator must reload");
    assert_eq!(operator.compiler_version(), roots.compiler_version);

    let runtime = ProgramIR::new(
        roots.unit,
        BlockTarget::new(0),
        vec![
            BasicBlock::new(
                BlockTarget::new(0),
                Terminator::Probe {
                    operator: roots.operator,
                    resume: BlockTarget::new(1),
                },
            ),
            BasicBlock::new(
                BlockTarget::new(1),
                Terminator::Return {
                    value: roots.answer_a,
                },
            ),
            BasicBlock::new(
                BlockTarget::new(2),
                Terminator::Return {
                    value: roots.answer_b,
                },
            ),
        ],
    );
    runtime
        .verify(&catalog)
        .expect("fresh deterministic runtime lowering must verify");
    let MachineStep::Suspended(suspension) = runtime
        .step(runtime.start())
        .expect("fresh runtime must reach the probe suspension")
    else {
        panic!("fresh runtime entry must suspend")
    };
    let lowering = ContinuationLowering::new(roots.continuation, BlockTarget::new(1));
    let observations = [
        ReplayObservation::new(roots.candidate_a, roots.observation_a),
        ReplayObservation::new(roots.candidate_b, roots.observation_b),
    ];
    let replayed = replay_completed_finite_probe(
        &store,
        roots.token,
        &decoded_decoder,
        roots.decoded_path,
        &observations,
        &standing,
        &source,
        suspension,
        lowering,
        &runtime,
        &catalog,
    )
    .await
    .expect("persisted actuality must cold replay to an admitted resumption");
    assert_eq!(replayed.actuality().event_ref(), roots.event);
    assert_eq!(replayed.actuality().raw_return_ref(), roots.raw_return);
    assert_eq!(replayed.actuality().raw_return().bytes(), [0x41, 0, 0xff]);
    assert_eq!(replayed.resumption().event(), roots.event);
    assert_eq!(replayed.resumption().raw_return(), roots.raw_return);
    let mut expected_candidates = vec![roots.candidate_a, roots.candidate_b];
    expected_candidates.sort_unstable();
    assert_eq!(
        replayed.resumption().binding().answer().candidates(),
        expected_candidates
    );
    assert!(matches!(
        runtime
            .step(replayed.resumption().state())
            .expect("replayed continuation must execute"),
        MachineStep::Returned(value) if value == roots.answer_a
    ));
    assert_eq!(
        provider_calls.load(Ordering::SeqCst),
        1,
        "cold replay must not redispatch the provider"
    );

    assert!(matches!(
        replay_completed_finite_probe(
            &store,
            roots.token,
            &undefined_decoder,
            roots.undefined_path,
            &[],
            &standing,
            &source,
            suspension,
            lowering,
            &runtime,
            &catalog,
        )
        .await,
        Err(FiniteProbeReplayError::Undefined { event, .. }) if event == roots.event
    ));
    assert!(matches!(
        replay_completed_finite_probe(
            &store,
            roots.token,
            &unknown_decoder,
            roots.unknown_path,
            &[],
            &standing,
            &source,
            suspension,
            lowering,
            &runtime,
            &catalog,
        )
        .await,
        Err(FiniteProbeReplayError::Unknown { event, .. }) if event == roots.event
    ));
    assert!(matches!(
        replay_completed_finite_probe(
            &store,
            roots.token,
            &decoded_decoder,
            roots.decoded_path,
            &observations[..1],
            &standing,
            &source,
            suspension,
            lowering,
            &runtime,
            &catalog,
        )
        .await,
        Err(FiniteProbeReplayError::SupportedAnswer(
            FiniteSupportedAnswerError::CandidateCoverageMismatch
        ))
    ));
    assert!(matches!(
        replay_completed_finite_probe(
            &store,
            roots.token,
            &decoded_decoder,
            roots.decoded_path,
            &observations,
            &no_standing,
            &source,
            suspension,
            lowering,
            &runtime,
            &catalog,
        )
        .await,
        Err(FiniteProbeReplayError::SupportedAnswer(
            FiniteSupportedAnswerError::RelationSupport(_)
        ))
    ));
    assert!(matches!(
        replay_completed_finite_probe(
            &store,
            roots.token,
            &decoded_decoder,
            roots.decoded_path,
            &observations,
            &standing,
            &wrong_source,
            suspension,
            lowering,
            &runtime,
            &catalog,
        )
        .await,
        Err(FiniteProbeReplayError::SourceBinding(
            FiniteAnswerBindingError::QuestionMismatch { .. }
        ))
    ));
    assert!(matches!(
        replay_completed_finite_probe(
            &store,
            roots.token,
            &decoded_decoder,
            roots.decoded_path,
            &observations,
            &standing,
            &capture_source,
            suspension,
            lowering,
            &runtime,
            &catalog,
        )
        .await,
        Err(FiniteProbeReplayError::SourceBinding(
            FiniteAnswerBindingError::IProgCheck(error)
        )) if matches!(*error, IProgCheckError::AnswerSlotShadowsEnvironment(_))
    ));

    let rival_runtime = ProgramIR::new(
        roots.unit,
        BlockTarget::new(0),
        vec![
            BasicBlock::new(
                BlockTarget::new(0),
                Terminator::Probe {
                    operator: roots.rival_operator,
                    resume: BlockTarget::new(1),
                },
            ),
            BasicBlock::new(
                BlockTarget::new(1),
                Terminator::Return {
                    value: roots.answer_a,
                },
            ),
        ],
    );
    rival_runtime
        .verify(&catalog)
        .expect("rival runtime must remain structurally valid");
    let MachineStep::Suspended(rival_suspension) = rival_runtime
        .step(rival_runtime.start())
        .expect("rival runtime must suspend")
    else {
        panic!("rival runtime entry must suspend")
    };
    assert!(matches!(
        replay_completed_finite_probe(
            &store,
            roots.token,
            &decoded_decoder,
            roots.decoded_path,
            &observations,
            &standing,
            &source,
            rival_suspension,
            lowering,
            &rival_runtime,
            &catalog,
        )
        .await,
        Err(FiniteProbeReplayError::Resumption(
            AdmittedResumeError::OperatorMismatch { .. }
        ))
    ));
    assert!(matches!(
        replay_completed_finite_probe(
            &store,
            roots.token,
            &decoded_decoder,
            roots.decoded_path,
            &observations,
            &standing,
            &source,
            suspension,
            ContinuationLowering::new(roots.wrong_source, BlockTarget::new(1)),
            &runtime,
            &catalog,
        )
        .await,
        Err(FiniteProbeReplayError::Resumption(
            AdmittedResumeError::ContinuationMismatch { .. }
        ))
    ));
    assert!(matches!(
        replay_completed_finite_probe(
            &store,
            roots.token,
            &decoded_decoder,
            roots.decoded_path,
            &observations,
            &standing,
            &source,
            suspension,
            ContinuationLowering::new(roots.continuation, BlockTarget::new(2)),
            &runtime,
            &catalog,
        )
        .await,
        Err(FiniteProbeReplayError::Resumption(
            AdmittedResumeError::ResumeTargetMismatch { .. }
        ))
    ));

    store.close().await;
    std::fs::remove_file(path).expect("temporary cold replay database must be removable");
}

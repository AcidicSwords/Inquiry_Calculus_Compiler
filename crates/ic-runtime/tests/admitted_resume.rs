use std::{
    collections::BTreeMap,
    convert::Infallible,
    env,
    fs::OpenOptions,
    io::ErrorKind,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};

use ic_core::{
    ActualDecodeError, ActualDecodeResult, ActualEvent, ActualEventCatalog, ApplicabilityRef,
    ArtifactEnvelope, ArtifactKind, ArtifactRef, AskOccurrence, BackendRef, BackendRequest,
    BindingVersionRef, BoundaryChart, BoundaryRef, ClaimArtifact, ClaimRef, ClaimStatus,
    CompletionCandidate, CompletionCandidateCatalog, CompletionCandidateRef, CoverageRef,
    DeclaredSupportClosure, DischargeMode, EffectivityRef, EventRef, ExactFiniteCue,
    ExactFiniteCueAdmission, ExactFiniteCueCatalog, ExactFiniteCueCheckError,
    ExactFiniteCueUnknown, ExactFinitePresentChallenge, ExactFinitePresentReopenWitness,
    ExactFinitePresentUpdate, ExactFiniteSignature, ExactFiniteSufficientPresent,
    ExactFiniteSufficientPresentResult, ExactProtectedContinuation, ExtensionDomainRef,
    FiniteAnswerBindingError, FiniteDecoder, FiniteDecoderCatalog, FiniteDecoderEntry,
    FiniteDecoderRef, FiniteResolutionCoverage, FiniteResolutionGateError,
    FiniteResolutionLeafEntry, FiniteResolutionLeafTable, FiniteResolutionOutcome,
    FiniteResolutionOutcomeKind, FiniteResolutionRun, FiniteSupportedAnswerError, FormulaArtifact,
    FormulaCatalog, FormulaIR, FormulaRef, GeneratedInquiry, GeneratedInquiryCatalog,
    GeneratorRegimeRef, GrainRef, HorizonRef, IProgArtifact, IProgCatalog, IProgCheckError,
    IProgIR, IProgRef, MethodBridge, MethodBridgeCatalog, MethodBridgeCheckError, MethodContract,
    MethodRef, NextSourcePosition, ObservationResultCatalog, OpenPort, OpenQuery, OpenQueryCatalog,
    OperatorOccurrenceCatalog, PortBinding, ProbeContractRef, ProbeOperator, ProbeOperatorRef,
    ProgramBinding, ProtectedCompletionFieldRef, ProtectedContinuationRef, ProvenanceRef, QueryRef,
    QuestionSuccessionCatalog, RawReturn, RawReturnCatalog, RawReturnRef, RelationBodyIR,
    RelationCatalog, RelationPort, RelationRef, RelationSchema, RelationSignature, RelationUse,
    RelationUseContext, RelationUseRef, RelationUseSupportCatalog, ResidualSchemaRef,
    ResolutionCatalog, ResolutionPath, ResolutionPathIR, ResolutionPathRef, RouteRef, ScopeRef,
    SeparatorProblem, SeparatorProblemRef, SignatureContext, SourceConfig, SourceConfigRef,
    StateRef, StructureViewRef, SupportEnvironmentArtifact, SupportEnvironmentCatalog,
    SupportEnvironmentRef, SupportSubjectRef, SurfacePlan, TyIR, TypeArtifact, TypeCatalog,
    TypeFamilyRef, TypeRef, TypeSymbol, TypedForm, TypedFormRef, admit_exact_finite_cue,
    admit_finite_supported_answers, bind_finite_ask_continuation,
    challenge_exact_finite_sufficient_present, check_admitted_exact_finite_cue_basis,
    classify_finite_port_resolution, classify_finite_question_resolution, decode_actual_event,
    decode_actual_event_for_port, derive_exact_finite_sufficient_present,
    derive_question_successor, extend_exact_finite_sufficient_present,
    match_decoded_observation_use, run_finite_resolution, standing_from_declared_support,
};
use ic_runtime::{
    AdmittedResumeError, BasicBlock, BlockTarget, ContinuationLowering, FiniteProbeReplayError,
    MachineStep, MethodBridgeReentryError, MethodCuePlanning, MixedModeSourceAskDischarge,
    MixedModeSourceAskDischargeError, MixedPortContribution, MixedQuestionResolutionError,
    NonProbePortDischargeEvidence, NonProbePortOutput, OLLAMA_DECODED_TEXT_ARTIFACT_KIND,
    OllamaDecodedText, OllamaGenerateProvider, OllamaHttpResponse, OllamaProviderError,
    PairedActualityTrace, PairedActualityTraversal, PortLowering, ProbeDischargeBundleError,
    ProbeDispatchContext, ProbePortDischargeEvidence, ProbeProvider, ProgramIR, ProviderReturn,
    ReplayObservation, ResolvedFiniteProbeOccurrenceError, RuntimeCatalog, RuntimeProgramArtifact,
    SharedProbeEventAdmission, SourceAskLowering, SourceAskLoweringCheckError,
    SourceAskProbeDischarge, SourceAskProbeDischargeError, SourceEventLinkError, Terminator,
    TraversalCausalOrder, WholeQuestionOutcome, admit_finite_probe_discharge_bundle,
    admit_mixed_mode_continuation, admit_probe_ports_of_mixed_discharge, check_source_event_link,
    derive_mixed_mode_successor, dispatch_probe, materialize_ollama_decoded_texts,
    plan_method_reentry_with_admitted_cues, replay_completed_finite_probe,
    replay_completed_finite_separator_inquiry, resolve_finite_probe_occurrence,
    resolve_mixed_mode_question, route_separator_through_method_bridge,
};
use ic_store::{ArtifactStore, DispatchToken};

static NEXT_TEMP_DATABASE: AtomicUsize = AtomicUsize::new(0);

fn reserve_sqlite_path(prefix: &str, clock_nonce: u128) -> PathBuf {
    loop {
        let sequence = NEXT_TEMP_DATABASE.fetch_add(1, Ordering::Relaxed);
        let path = env::temp_dir().join(format!(
            "{prefix}-{}-{clock_nonce}-{sequence}.sqlite",
            std::process::id()
        ));
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => {
                drop(file);
                return path;
            }
            Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
            Err(error) => panic!("temporary SQLite path must be reservable: {error}"),
        }
    }
}

fn fresh_sqlite_path(prefix: &str) -> PathBuf {
    let clock_nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    reserve_sqlite_path(prefix, clock_nonce)
}

#[test]
fn temporary_sqlite_paths_remain_distinct_at_one_clock_instant() {
    let first = reserve_sqlite_path("inquiry-calculus-path-breaker", 0);
    let second = reserve_sqlite_path("inquiry-calculus-path-breaker", 0);
    assert_ne!(first, second);
    std::fs::remove_file(first).expect("first reserved path must be removable");
    std::fs::remove_file(second).expect("second reserved path must be removable");
}

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
    separator_problems: BTreeMap<SeparatorProblemRef, SeparatorProblem>,
    formulas: BTreeMap<FormulaRef, FormulaArtifact>,
    methods: BTreeMap<MethodRef, MethodContract>,
    claims: BTreeMap<ClaimRef, ClaimArtifact>,
    source_configs: BTreeMap<SourceConfigRef, SourceConfig>,
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

    fn insert_formula(&mut self, value: FormulaArtifact) -> FormulaRef {
        let reference = value.formula_ref().expect("formula must encode");
        self.formulas.insert(reference, value);
        reference
    }

    fn insert_method(&mut self, value: MethodContract) -> MethodRef {
        let reference = value.method_ref().expect("method must encode");
        self.methods.insert(reference, value);
        reference
    }

    fn insert_claim(&mut self, value: ClaimArtifact) -> ClaimRef {
        let reference = value.claim_ref().expect("claim must encode");
        self.claims.insert(reference, value);
        reference
    }

    fn insert_source_config(&mut self, value: SourceConfig) -> SourceConfigRef {
        let reference = value.source_config_ref().expect("source must encode");
        self.source_configs.insert(reference, value);
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
    fn resolve_formula(&self, reference: FormulaRef) -> Option<FormulaArtifact> {
        self.formulas.get(&reference).cloned()
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
    fn resolve_claim(&self, reference: ClaimRef) -> Option<ClaimArtifact> {
        self.claims.get(&reference).cloned()
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

impl QuestionSuccessionCatalog for Catalog {
    fn resolve_source_config(&self, reference: SourceConfigRef) -> Option<SourceConfig> {
        self.source_configs.get(&reference).cloned()
    }
}

impl RuntimeCatalog for Catalog {
    fn resolve_typed_form(&self, reference: TypedFormRef) -> Option<TypedForm> {
        self.forms.get(&reference).copied()
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

impl MethodBridgeCatalog for Catalog {
    fn resolve_method(&self, reference: MethodRef) -> Option<MethodContract> {
        self.methods.get(&reference).cloned()
    }
}

impl ExactFiniteCueCatalog for Catalog {
    fn resolve_method(&self, reference: MethodRef) -> Option<MethodContract> {
        self.methods.get(&reference).cloned()
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

fn ollama_candidate_response() -> Vec<u8> {
    let body = serde_json::to_vec(&serde_json::json!({
        "model": "qwen3.5:9b",
        "response": "{\"candidates\":[\"north\",\"south\"]}",
        "done": true,
        "done_reason": "stop"
    }))
    .expect("fixture Ollama response must encode");
    OllamaHttpResponse::new(200, body)
        .encode()
        .expect("fixture Ollama transport frame must encode")
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
    alternate_decoded_decoder: FiniteDecoderRef,
    undefined_decoder: FiniteDecoderRef,
    unknown_decoder: FiniteDecoderRef,
    decoded_path: ResolutionPathRef,
    alternate_decoded_path: ResolutionPathRef,
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
    alternate_raw_return: RawReturnRef,
    current_protected: ProtectedContinuationRef,
    path_protected: ProtectedContinuationRef,
    compiler_version: ArtifactRef,
}

#[derive(Clone, Copy)]
struct OllamaPostReturnRoots {
    decoder_version: ArtifactRef,
    value_a: ArtifactRef,
    value_b: ArtifactRef,
    form_a: TypedFormRef,
    form_b: TypedFormRef,
    candidate_a: CompletionCandidateRef,
    candidate_b: CompletionCandidateRef,
    observation_a: RelationUseRef,
    observation_b: RelationUseRef,
    support: SupportEnvironmentRef,
    decoder: FiniteDecoderRef,
    path: ResolutionPathRef,
}

fn derive_fixture_sufficient_present(
    roots: ColdReplayRoots,
    query: &OpenQuery,
    primary: &PairedActualityTrace,
    alternate: &PairedActualityTrace,
) -> (
    ExactFiniteSufficientPresent,
    ExactFinitePresentReopenWitness,
) {
    let context = SignatureContext::new(
        primary.question().binding(),
        query.context().scope(),
        query.context().applicability(),
        query.context().grain(),
        query.context().horizon(),
        roots.raw_type,
    );
    let histories = [
        primary.returned().path().as_artifact_ref(),
        alternate.returned().path().as_artifact_ref(),
    ];
    let presentation = ExactFiniteSignature::new(
        context,
        histories
            .iter()
            .map(|history| (*history, roots.continuation.as_artifact_ref()))
            .collect(),
    )
    .expect("two distinct replay paths form an exact finite history domain");
    let current_observation = ExactFiniteSignature::new(
        context,
        histories
            .iter()
            .map(|history| (*history, roots.answer_a.as_artifact_ref()))
            .collect(),
    )
    .expect("current protected continuation is total over both histories");
    let ExactFiniteSufficientPresentResult::Sufficient(present) =
        derive_exact_finite_sufficient_present(
            presentation,
            vec![ExactProtectedContinuation::new(
                roots.current_protected,
                current_observation,
            )],
        )
        .expect("present and protected observation contexts must agree")
    else {
        panic!("the folded endpoint must determine the current continuation")
    };
    assert_eq!(
        present.class_count(),
        1,
        "one class is the coarsest quotient"
    );

    let path_observation = ExactFiniteSignature::new(
        context,
        histories
            .iter()
            .map(|history| (*history, *history))
            .collect(),
    )
    .expect("path-sensitive continuation is total over both histories");
    let ExactFinitePresentChallenge::Reopened(witness) = challenge_exact_finite_sufficient_present(
        &present,
        ExactProtectedContinuation::new(roots.path_protected, path_observation),
    )
    .expect("new protected continuation context must agree") else {
        panic!("path-sensitive continuation must reopen the one-class fold")
    };
    (present, witness)
}

fn derive_event_sufficient_present(
    roots: ColdReplayRoots,
    query: &OpenQuery,
    first: &PairedActualityTrace,
    second: &PairedActualityTrace,
    current_protected: ProtectedContinuationRef,
    event_protected: ProtectedContinuationRef,
) -> (
    ExactFiniteSufficientPresent,
    ExactFinitePresentReopenWitness,
) {
    let context = SignatureContext::new(
        first.question().binding(),
        query.context().scope(),
        query.context().applicability(),
        query.context().grain(),
        query.context().horizon(),
        roots.raw_type,
    );
    let histories = [
        first.question().event().as_artifact_ref(),
        second.question().event().as_artifact_ref(),
    ];
    let prior_presentation = ExactFiniteSignature::new(
        context,
        vec![(histories[0], roots.continuation.as_artifact_ref())],
    )
    .expect("the first event forms an exact prior history domain");
    let prior_observation = ExactFiniteSignature::new(
        context,
        vec![(histories[0], roots.answer_a.as_artifact_ref())],
    )
    .expect("the first current observation is exact");
    let ExactFiniteSufficientPresentResult::Sufficient(prior_present) =
        derive_exact_finite_sufficient_present(
            prior_presentation,
            vec![ExactProtectedContinuation::new(
                current_protected,
                prior_observation,
            )],
        )
        .expect("one first event must form a sufficient prior present")
    else {
        panic!("one event cannot split its current protected continuation")
    };
    let presentation = ExactFiniteSignature::new(
        context,
        histories
            .iter()
            .map(|history| (*history, roots.continuation.as_artifact_ref()))
            .collect(),
    )
    .expect("two distinct event identities form an exact finite history domain");
    let current_observation = ExactFiniteSignature::new(
        context,
        histories
            .iter()
            .map(|history| (*history, roots.answer_a.as_artifact_ref()))
            .collect(),
    )
    .expect("the currently protected continuation is total over both events");
    let ExactFinitePresentUpdate::Updated(present) = extend_exact_finite_sufficient_present(
        &prior_present,
        presentation,
        vec![ExactProtectedContinuation::new(
            current_protected,
            current_observation,
        )],
    )
    .expect("the appended event must preserve prior rows and current protected observation") else {
        panic!("the current continuation must remain folded after the appended event")
    };
    let event_observation = ExactFiniteSignature::new(
        context,
        histories
            .iter()
            .map(|history| (*history, *history))
            .collect(),
    )
    .expect("event-sensitive continuation must be total over both events");
    let ExactFinitePresentChallenge::Reopened(witness) = challenge_exact_finite_sufficient_present(
        &present,
        ExactProtectedContinuation::new(event_protected, event_observation),
    )
    .expect("new event-sensitive protected continuation context must agree") else {
        panic!("event-sensitive continuation must reopen the folded present")
    };
    (present, witness)
}

struct CountingProvider {
    calls: Arc<AtomicUsize>,
    expected_body: ArtifactRef,
    response: Vec<u8>,
}

impl ProbeProvider for CountingProvider {
    type Error = Infallible;

    fn dispatch(&mut self, request: &BackendRequest) -> Result<ProviderReturn, Self::Error> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        assert_eq!(request.request_body(), self.expected_body);
        Ok(ProviderReturn::new(self.response.clone()))
    }
}

struct CountingOllamaProvider {
    calls: Arc<AtomicUsize>,
    inner: OllamaGenerateProvider,
}

impl ProbeProvider for CountingOllamaProvider {
    type Error = OllamaProviderError;

    fn dispatch(&mut self, request: &BackendRequest) -> Result<ProviderReturn, Self::Error> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.inner.dispatch(request)
    }
}

async fn persisted_cold_replay_fixture() -> (
    PathBuf,
    ColdReplayRoots,
    Arc<AtomicUsize>,
    PairedActualityTrace,
    ExactFiniteSufficientPresent,
    ExactFinitePresentReopenWitness,
) {
    let path = fresh_sqlite_path("inquiry-calculus-cold-replay");
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

    let raw = RawReturn::new(ollama_candidate_response());
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
        vec![stored_ref(&store, b"pre-dispatch-support-assumption").await],
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
    let alternate_raw = RawReturn::new(vec![0x99]);
    let alternate_raw_return = RawReturnRef::from_artifact_ref(
        store
            .insert(
                &alternate_raw
                    .envelope()
                    .expect("alternate raw return must encode"),
            )
            .await
            .expect("alternate raw return must persist"),
    );
    assert_eq!(
        catalog.insert_raw_return(alternate_raw),
        alternate_raw_return
    );
    let alternate_decoded_decoder_value = FiniteDecoder::new(
        query,
        raw_type,
        vec![
            FiniteDecoderEntry::Decoded {
                raw_return,
                candidates: vec![candidate_a, candidate_b],
            },
            FiniteDecoderEntry::Undefined {
                raw_return: alternate_raw_return,
            },
        ],
    )
    .expect("alternate finite decoder must encode");
    let alternate_decoded_decoder = FiniteDecoderRef::from_artifact_ref(
        persist(
            &store,
            &alternate_decoded_decoder_value
                .envelope()
                .expect("alternate decoder must encode"),
            &alternate_decoded_decoder_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_decoder(alternate_decoded_decoder_value),
        alternate_decoded_decoder
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
    let alternate_decoded_path_value = ResolutionPath::new(
        raw_type,
        unit,
        ResolutionPathIR::Decode {
            decoder: alternate_decoded_decoder.as_decoder_ref(),
        },
    );
    let alternate_decoded_path = ResolutionPathRef::from_artifact_ref(
        persist(
            &store,
            &alternate_decoded_path_value
                .envelope()
                .expect("alternate decoded path must encode"),
            &alternate_decoded_path_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_path(alternate_decoded_path_value),
        alternate_decoded_path
    );
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
    let current_protected = ProtectedContinuationRef::from_artifact_ref(
        stored_ref(&store, b"current-protected-continuation").await,
    );
    let path_protected = ProtectedContinuationRef::from_artifact_ref(
        stored_ref(&store, b"path-protected-continuation").await,
    );
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
        response: raw.bytes().to_vec(),
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
    let live_trace = PairedActualityTrace::derive(actual.event(), live.resumption())
        .expect("live question and return must form one event-linked trace pair");
    assert_eq!(live_trace.question().event(), live_trace.returned().event());
    assert_eq!(live_trace.question().question(), query);
    assert_eq!(live_trace.returned().path(), decoded_path);
    assert_eq!(live_trace.returned().continuation(), continuation);
    let alternate_decoder_value = catalog
        .resolve_finite_decoder(alternate_decoded_decoder)
        .expect("live alternate decoder must remain available");
    let alternate_live = replay_completed_finite_probe(
        &store,
        token,
        &alternate_decoder_value,
        alternate_decoded_path,
        &observations,
        &standing,
        &source_value,
        suspension,
        ContinuationLowering::new(continuation, BlockTarget::new(1)),
        &runtime,
        &catalog,
    )
    .await
    .expect("live actuality must admit through the alternate exact path");
    let alternate_live_trace =
        PairedActualityTrace::derive(actual.event(), alternate_live.resumption())
            .expect("alternate live path must form a checked trace pair");
    let live_roots = ColdReplayRoots {
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
        alternate_decoded_decoder,
        undefined_decoder,
        unknown_decoder,
        decoded_path,
        alternate_decoded_path,
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
        alternate_raw_return,
        current_protected,
        path_protected,
        compiler_version,
    };
    let live_query = OpenQueryCatalog::resolve_open_query(&catalog, query)
        .expect("live query must remain available");
    let (live_present, live_reopen) = derive_fixture_sufficient_present(
        live_roots,
        &live_query,
        &live_trace,
        &alternate_live_trace,
    );
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
        live_roots,
        provider_calls,
        live_trace,
        live_present,
        live_reopen,
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
    for reference in [roots.raw_return, roots.alternate_raw_return] {
        let raw_return =
            RawReturn::from_envelope(&load_envelope(store, reference.as_artifact_ref()).await)
                .expect("persisted raw return must decode");
        assert_eq!(catalog.insert_raw_return(raw_return), reference);
    }
    for reference in [
        roots.decoded_decoder,
        roots.alternate_decoded_decoder,
        roots.undefined_decoder,
        roots.unknown_decoder,
    ] {
        let value =
            FiniteDecoder::from_envelope(&load_envelope(store, reference.as_artifact_ref()).await)
                .expect("persisted finite decoder must decode");
        assert_eq!(catalog.insert_decoder(value), reference);
    }
    for reference in [
        roots.decoded_path,
        roots.alternate_decoded_path,
        roots.undefined_path,
        roots.unknown_path,
    ] {
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

async fn materialize_ollama_post_return_slice(
    store: &ArtifactStore,
    catalog: &mut Catalog,
    roots: ColdReplayRoots,
) -> OllamaPostReturnRoots {
    let raw = catalog
        .resolve_raw_return(roots.raw_return)
        .expect("actual raw return must be available before derived materialization");
    let decoder_version = stored_ref(store, b"ollama-schema-decoder-v1").await;
    let values = materialize_ollama_decoded_texts(roots.raw_return, &raw, decoder_version)
        .expect("committed local return must decode after actuality");
    assert_eq!(
        values.len(),
        2,
        "fixture schema requires two preserved candidates"
    );
    let value_a = persist(
        store,
        &values[0]
            .envelope()
            .expect("first decoded value must encode"),
        &values[0].referenced_artifacts(),
    )
    .await;
    let value_b = persist(
        store,
        &values[1]
            .envelope()
            .expect("second decoded value must encode"),
        &values[1].referenced_artifacts(),
    )
    .await;
    assert_eq!(
        values[0].artifact_ref().expect("first value must address"),
        value_a
    );
    assert_eq!(
        values[1].artifact_ref().expect("second value must address"),
        value_b
    );

    let binding = catalog
        .resolve_type(roots.unit)
        .expect("answer type must remain available")
        .binding();
    let form_a_value = TypedForm::new(binding, roots.unit, value_a);
    let form_a = TypedFormRef::from_artifact_ref(
        persist(
            store,
            &form_a_value
                .envelope()
                .expect("first post-return form must encode"),
            &form_a_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_form(form_a_value), form_a);
    let form_b_value = TypedForm::new(binding, roots.unit, value_b);
    let form_b = TypedFormRef::from_artifact_ref(
        persist(
            store,
            &form_b_value
                .envelope()
                .expect("second post-return form must encode"),
            &form_b_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_form(form_b_value), form_b);

    let query = OpenQueryCatalog::resolve_open_query(catalog, roots.query)
        .expect("source query must remain available");
    let answer_port = query
        .open_ports()
        .first()
        .expect("fixture query must retain one open answer port")
        .port()
        .clone();
    let candidate_a_value = query
        .plug(vec![PortBinding::new(answer_port.clone(), form_a)], catalog)
        .expect("first local value must fill the already addressed query");
    let candidate_b_value = query
        .plug(vec![PortBinding::new(answer_port.clone(), form_b)], catalog)
        .expect("second local value must fill the already addressed query");
    let candidate_a = CompletionCandidateRef::from_artifact_ref(
        persist(
            store,
            &candidate_a_value
                .envelope()
                .expect("first post-return candidate must encode"),
            &candidate_a_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_candidate(candidate_a_value), candidate_a);
    let candidate_b = CompletionCandidateRef::from_artifact_ref(
        persist(
            store,
            &candidate_b_value
                .envelope()
                .expect("second post-return candidate must encode"),
            &candidate_b_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_candidate(candidate_b_value), candidate_b);

    let query_context = query.context();
    let support_value = SupportEnvironmentArtifact::new(
        SupportSubjectRef::Relation(query.relation()),
        Vec::new(),
        vec![roots.raw_return],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        query_context.applicability(),
        query_context.scope(),
    )
    .expect("post-return support environment must encode");
    let support = SupportEnvironmentRef::from_artifact_ref(
        persist(
            store,
            &support_value
                .envelope()
                .expect("post-return support must encode"),
            &support_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_support(support_value), support);
    let observation_context = RelationUseContext::new(
        query_context.scope(),
        query_context.applicability(),
        query_context.grain(),
        query_context.horizon(),
        query_context.mode(),
        support.as_support_ref(),
        query_context.warrant(),
    );
    let observation_a_value = RelationUse::new(
        query.relation(),
        vec![PortBinding::new(answer_port.clone(), form_a)],
        observation_context,
    );
    let observation_a = RelationUseRef::from_artifact_ref(
        persist(
            store,
            &observation_a_value
                .envelope()
                .expect("first post-return observation must encode"),
            &observation_a_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_relation_use(observation_a_value),
        observation_a
    );
    let observation_b_value = RelationUse::new(
        query.relation(),
        vec![PortBinding::new(answer_port, form_b)],
        observation_context,
    );
    let observation_b = RelationUseRef::from_artifact_ref(
        persist(
            store,
            &observation_b_value
                .envelope()
                .expect("second post-return observation must encode"),
            &observation_b_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_relation_use(observation_b_value),
        observation_b
    );

    let decoder_value = FiniteDecoder::new(
        roots.query,
        roots.raw_type,
        vec![FiniteDecoderEntry::Decoded {
            raw_return: roots.raw_return,
            candidates: vec![candidate_a, candidate_b],
        }],
    )
    .expect("post-return finite decoder must encode");
    let decoder = FiniteDecoderRef::from_artifact_ref(
        persist(
            store,
            &decoder_value
                .envelope()
                .expect("post-return decoder must encode"),
            &decoder_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_decoder(decoder_value), decoder);
    let path_value = ResolutionPath::new(
        roots.raw_type,
        roots.unit,
        ResolutionPathIR::Decode {
            decoder: decoder.as_decoder_ref(),
        },
    );
    let path = ResolutionPathRef::from_artifact_ref(
        persist(
            store,
            &path_value.envelope().expect("post-return path must encode"),
            &path_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_path(path_value), path);

    OllamaPostReturnRoots {
        decoder_version,
        value_a,
        value_b,
        form_a,
        form_b,
        candidate_a,
        candidate_b,
        observation_a,
        observation_b,
        support,
        decoder,
        path,
    }
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
    let (path, roots, provider_calls, live_trace, live_present, live_reopen) =
        persisted_cold_replay_fixture().await;
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
    let alternate_decoded_decoder = catalog
        .resolve_finite_decoder(roots.alternate_decoded_decoder)
        .expect("alternate finite decoder must reload");
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
    assert_eq!(
        replayed.actuality().raw_return().bytes(),
        ollama_candidate_response()
    );
    assert_eq!(replayed.resumption().event(), roots.event);
    assert_eq!(replayed.resumption().raw_return(), roots.raw_return);
    let mut expected_candidates = vec![roots.candidate_a, roots.candidate_b];
    expected_candidates.sort_unstable();
    assert_eq!(
        replayed.resumption().binding().answer().candidates(),
        expected_candidates
    );
    let replayed_trace =
        PairedActualityTrace::derive(replayed.actuality().event(), replayed.resumption())
            .expect("cold replay must regenerate the event-linked trace pair");
    assert_eq!(replayed_trace, live_trace);
    let alternate_path_replay = replay_completed_finite_probe(
        &store,
        roots.token,
        &alternate_decoded_decoder,
        roots.alternate_decoded_path,
        &observations,
        &standing,
        &source,
        suspension,
        lowering,
        &runtime,
        &catalog,
    )
    .await
    .expect("the same event and endpoint may resolve through another admitted path");
    let alternate_path_trace = PairedActualityTrace::derive(
        alternate_path_replay.actuality().event(),
        alternate_path_replay.resumption(),
    )
    .expect("alternate path must still form a checked trace pair");
    assert_eq!(alternate_path_trace.question(), replayed_trace.question());
    assert_eq!(
        alternate_path_trace.returned().resume_target(),
        replayed_trace.returned().resume_target()
    );
    assert_eq!(
        alternate_path_trace.returned().candidates(),
        replayed_trace.returned().candidates()
    );
    assert_ne!(
        alternate_path_trace.returned(),
        replayed_trace.returned(),
        "same event, candidates, and endpoint must retain distinct resolution provenance"
    );
    let reloaded_query = OpenQueryCatalog::resolve_open_query(&catalog, roots.query)
        .expect("reloaded query must remain available");
    let (replayed_present, replayed_reopen) = derive_fixture_sufficient_present(
        roots,
        &reloaded_query,
        &replayed_trace,
        &alternate_path_trace,
    );
    assert_eq!(
        replayed_present, live_present,
        "the exact finite sufficient present must regenerate from persisted roots"
    );
    assert_eq!(
        replayed_reopen, live_reopen,
        "the new protected path continuation must regenerate the same reopen witness"
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

#[tokio::test]
async fn two_ordinary_events_cold_replay_as_one_derived_traversal_and_reopen_the_present() {
    let (path, roots, provider_calls, first_live_trace, _, _) =
        persisted_cold_replay_fixture().await;
    let url = format!("sqlite://{}", path.to_string_lossy().replace('\\', "/"));
    let store = ArtifactStore::open(&url)
        .await
        .expect("multi-event store must reopen");
    store
        .migrate()
        .await
        .expect("embedded migrations must remain repeatable");
    let mut catalog = load_cold_replay_catalog(&store, roots).await;
    let source = catalog
        .resolve_iprog(roots.source)
        .expect("source Ask must reload");
    let decoder = catalog
        .resolve_finite_decoder(roots.decoded_decoder)
        .expect("decoder must reload");
    let query =
        OpenQueryCatalog::resolve_open_query(&catalog, roots.query).expect("query must reload");
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
    .expect("exact support must reconstruct before either event is replayed");
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
        ],
    );
    runtime
        .verify(&catalog)
        .expect("multi-event lowering must regenerate from source artifacts");
    let MachineStep::Suspended(suspension) = runtime
        .step(runtime.start())
        .expect("regenerated entry must suspend")
    else {
        panic!("regenerated entry must be a probe suspension")
    };
    let observations = [
        ReplayObservation::new(roots.candidate_a, roots.observation_a),
        ReplayObservation::new(roots.candidate_b, roots.observation_b),
    ];
    let first_replayed = replay_completed_finite_probe(
        &store,
        roots.token,
        &decoder,
        roots.decoded_path,
        &observations,
        &standing,
        &source,
        suspension,
        ContinuationLowering::new(roots.continuation, BlockTarget::new(1)),
        &runtime,
        &catalog,
    )
    .await
    .expect("first ordinary event must replay before extending the ledger");
    let first_trace = PairedActualityTrace::derive(
        first_replayed.actuality().event(),
        first_replayed.resumption(),
    )
    .expect("first replay must reconstruct paired actuality");
    assert_eq!(first_trace, first_live_trace);

    let first_effect = store
        .replay_completed_external_effect(roots.token)
        .await
        .expect("first completed effect must provide its checked request for the next dispatch");
    let second_token = DispatchToken::from_bytes([0xc2; 32]);
    let second_state_after =
        StateRef::from_artifact_ref(stored_ref(&store, b"multi-event-second-state-after").await);
    let second_context = ProbeDispatchContext::new(
        Some(roots.event),
        first_effect.event().state_after(),
        None,
        second_state_after,
        first_effect.event().grain(),
        RouteRef::from_artifact_ref(stored_ref(&store, b"multi-event-second-route").await),
        first_effect.event().binding(),
        ProvenanceRef::from_artifact_ref(
            stored_ref(&store, b"multi-event-second-provenance").await,
        ),
    );
    let mut provider = CountingProvider {
        calls: Arc::clone(&provider_calls),
        expected_body: first_effect.request().request_body(),
        response: first_effect.raw_return().bytes().to_vec(),
    };
    let second_actual = dispatch_probe(
        &store,
        suspension,
        second_token,
        first_effect.request_ref(),
        second_context,
        &mut provider,
    )
    .await
    .expect("second ordinary event must dispatch through the already checked request boundary");
    let second_event = second_actual.event_ref();
    assert_ne!(second_event, roots.event);
    assert_eq!(second_actual.event().ledger_parent(), Some(roots.event));
    assert_eq!(second_actual.raw_return_ref(), roots.raw_return);
    catalog
        .events
        .insert(second_event, second_actual.event().clone());

    let second_replayed = replay_completed_finite_probe(
        &store,
        second_token,
        &decoder,
        roots.decoded_path,
        &observations,
        &standing,
        &source,
        suspension,
        ContinuationLowering::new(roots.continuation, BlockTarget::new(1)),
        &runtime,
        &catalog,
    )
    .await
    .expect("second ordinary event must independently admit and resume");
    let second_trace = PairedActualityTrace::derive(
        second_replayed.actuality().event(),
        second_replayed.resumption(),
    )
    .expect("second replay must form a separate paired actuality trace");
    assert_ne!(first_trace, second_trace);

    let unknown_traversal = PairedActualityTraversal::new(
        vec![first_trace.clone(), second_trace.clone()],
        vec![roots.event, second_event],
        TraversalCausalOrder::Unknown,
    )
    .expect("ordinary ledger membership must not require a causal assertion");
    let declared_traversal = PairedActualityTraversal::new(
        vec![first_trace.clone(), second_trace.clone()],
        vec![roots.event, second_event],
        TraversalCausalOrder::Declared(vec![(second_event, roots.event)]),
    )
    .expect("a separately declared causal candidate may differ from ledger order");
    assert_eq!(
        unknown_traversal.ledger_order(),
        declared_traversal.ledger_order()
    );
    assert_ne!(
        unknown_traversal.causal_order(),
        declared_traversal.causal_order()
    );

    let current_protected = ProtectedContinuationRef::from_artifact_ref(
        stored_ref(&store, b"multi-event-current-protected").await,
    );
    let event_protected = ProtectedContinuationRef::from_artifact_ref(
        stored_ref(&store, b"multi-event-event-protected").await,
    );
    let (live_present, live_reopen) = derive_event_sufficient_present(
        roots,
        &query,
        &first_trace,
        &second_trace,
        current_protected,
        event_protected,
    );
    assert_eq!(live_present.class_count(), 1);

    let reopen_envelope = live_reopen
        .envelope()
        .expect("positive reopen witness must encode");
    let reopen_ref = persist(
        &store,
        &reopen_envelope,
        &live_reopen.referenced_artifacts(),
    )
    .await;
    let structure = StructureViewRef::from_artifact_ref(
        stored_ref(&store, b"multi-event-separator-structure").await,
    );
    let regime = GeneratorRegimeRef::from_artifact_ref(
        stored_ref(&store, b"multi-event-separator-regime").await,
    );
    let effectivity = EffectivityRef::from_artifact_ref(
        stored_ref(&store, b"multi-event-separator-effectivity").await,
    );
    let generation_route = stored_ref(&store, b"multi-event-separator-route").await;
    let problem = SeparatorProblem::new(
        ProtectedCompletionFieldRef::from_artifact_ref(reopen_ref),
        None,
        query.context().grain(),
        query.context().horizon(),
        catalog
            .schemas
            .get(&query.relation())
            .expect("separator query relation must remain loaded")
            .binding(),
        structure,
        regime,
        effectivity,
    );
    let problem_envelope = problem.envelope().expect("separator problem must encode");
    let problem_ref = SeparatorProblemRef::from_artifact_ref(
        persist(&store, &problem_envelope, &problem.referenced_artifacts()).await,
    );
    catalog.separator_problems.insert(problem_ref, problem);
    let generated = GeneratedInquiry::new(problem_ref, generation_route, roots.query);
    let generated_envelope = generated
        .envelope()
        .expect("generated separator inquiry must encode");
    let generated_ref = persist(
        &store,
        &generated_envelope,
        &generated.referenced_artifacts(),
    )
    .await;
    let live_separator = replay_completed_finite_separator_inquiry(
        &store,
        second_token,
        generated,
        &decoder,
        roots.decoded_path,
        &observations,
        &standing,
        &source,
        suspension,
        ContinuationLowering::new(roots.continuation, BlockTarget::new(1)),
        &runtime,
        &catalog,
    )
    .await
    .expect("the live reopen residual must drive one generic separator continuation");
    assert_eq!(live_separator.separator().problem(), problem_ref);
    assert_eq!(
        live_separator.separator().generation_route(),
        generation_route
    );
    assert_eq!(
        live_separator.separator().binding().answer().event(),
        second_event
    );
    let mut expected_candidates = vec![roots.candidate_a, roots.candidate_b];
    expected_candidates.sort_unstable();
    assert_eq!(
        live_separator.separator().binding().answer().candidates(),
        expected_candidates,
        "generic separator continuation must retain the entire admitted answer set"
    );

    let residual_schema = ResidualSchemaRef::from_artifact_ref(reopen_ref);
    let method_coverage =
        CoverageRef::from_artifact_ref(stored_ref(&store, b"multi-event-method-coverage").await);
    let method_extension = ExtensionDomainRef::from_artifact_ref(
        stored_ref(&store, b"multi-event-method-extension").await,
    );
    let method_backend =
        BackendRef::from_artifact_ref(stored_ref(&store, b"multi-event-method-backend").await);
    let from_method = MethodContract::new(
        roots.relation,
        query.context().applicability(),
        stored_ref(&store, b"multi-event-from-method-law").await,
        method_coverage,
        DischargeMode::Pure,
        method_extension,
        method_backend,
        None,
        None,
        vec![residual_schema],
        vec![problem_ref.as_artifact_ref(), generated_ref],
    )
    .expect("source method contract must canonicalize");
    let from_method_envelope = from_method.envelope().expect("source method must encode");
    let from_method_ref = MethodRef::from_artifact_ref(
        persist(
            &store,
            &from_method_envelope,
            &from_method.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_method(from_method.clone()), from_method_ref);
    let to_method = MethodContract::new(
        roots.relation,
        query.context().applicability(),
        stored_ref(&store, b"multi-event-to-method-law").await,
        method_coverage,
        DischargeMode::Pure,
        method_extension,
        method_backend,
        None,
        None,
        Vec::new(),
        vec![problem_ref.as_artifact_ref(), generated_ref],
    )
    .expect("target method contract must canonicalize");
    let to_method_envelope = to_method.envelope().expect("target method must encode");
    let to_method_ref = MethodRef::from_artifact_ref(
        persist(
            &store,
            &to_method_envelope,
            &to_method.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_method(to_method.clone()), to_method_ref);
    assert_ne!(from_method_ref, to_method_ref);

    let guard = FormulaArtifact::new(
        catalog
            .schemas
            .get(&roots.relation)
            .expect("method relation must remain loaded")
            .binding(),
        Vec::new(),
        FormulaIR::Top,
    );
    let guard_envelope = guard.envelope().expect("reentry guard must encode");
    let guard_ref = FormulaRef::from_artifact_ref(
        persist(&store, &guard_envelope, &guard.referenced_artifacts()).await,
    );
    assert_eq!(catalog.insert_formula(guard.clone()), guard_ref);
    let reconstruct = IProgArtifact::new(
        roots.unit,
        IProgIR::Return {
            value: roots.answer_b,
        },
    );
    let reconstruct_envelope = reconstruct
        .envelope()
        .expect("input reconstruction must encode");
    let reconstruct_ref = IProgRef::from_artifact_ref(
        persist(
            &store,
            &reconstruct_envelope,
            &reconstruct.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_program(reconstruct.clone()), reconstruct_ref);
    let bridge = MethodBridge::new(
        from_method_ref,
        residual_schema,
        to_method_ref,
        roots.continuation,
        guard_ref,
        reconstruct_ref,
    );
    bridge
        .check(&catalog)
        .expect("all transparent method-bridge dependencies must revalidate");
    let bridge_envelope = bridge.envelope().expect("method bridge must encode");
    let bridge_ref = persist(&store, &bridge_envelope, &bridge.referenced_artifacts()).await;
    assert_eq!(
        MethodBridge::from_envelope(&bridge_envelope).expect("method bridge must decode"),
        bridge
    );
    assert_ne!(
        bridge
            .method_bridge_ref()
            .expect("method bridge must hash")
            .as_artifact_ref(),
        MethodBridge::new(
            to_method_ref,
            residual_schema,
            from_method_ref,
            roots.continuation,
            guard_ref,
            reconstruct_ref,
        )
        .method_bridge_ref()
        .expect("oriented rival bridge must hash")
        .as_artifact_ref(),
        "method-bridge orientation must be identity-bearing"
    );
    let rival_transport = MethodBridge::new(
        from_method_ref,
        residual_schema,
        to_method_ref,
        roots.source,
        guard_ref,
        reconstruct_ref,
    );
    assert!(matches!(
        route_separator_through_method_bridge(live_separator.clone(), rival_transport, &catalog),
        Err(MethodBridgeReentryError::TransportMismatch { selected, bridge })
            if selected == roots.continuation && bridge == roots.source
    ));
    let undeclared_residual = MethodBridge::new(
        to_method_ref,
        residual_schema,
        from_method_ref,
        roots.continuation,
        guard_ref,
        reconstruct_ref,
    );
    assert!(matches!(
        route_separator_through_method_bridge(
            live_separator.clone(),
            undeclared_residual,
            &catalog,
        ),
        Err(MethodBridgeReentryError::Bridge(
            MethodBridgeCheckError::ResidualSchemaNotDeclared { method, residual }
        )) if method == to_method_ref && residual == residual_schema
    ));
    let live_method_reentry =
        route_separator_through_method_bridge(live_separator, bridge, &catalog)
            .expect("the checked bridge must reenter the exact answer-selected continuation");

    let next_protected = ProtectedContinuationRef::from_artifact_ref(
        stored_ref(&store, b"multi-event-next-protected").await,
    );
    let event_context = SignatureContext::new(
        first_trace.question().binding(),
        query.context().scope(),
        query.context().applicability(),
        query.context().grain(),
        query.context().horizon(),
        roots.raw_type,
    );
    let next_observation = ExactFiniteSignature::new(
        event_context,
        vec![
            (roots.event.as_artifact_ref(), roots.event.as_artifact_ref()),
            (
                second_event.as_artifact_ref(),
                second_event.as_artifact_ref(),
            ),
        ],
    )
    .expect("next protected separator must cover the complete live history domain");
    let ExactFinitePresentChallenge::Reopened(next_reopen) =
        challenge_exact_finite_sufficient_present(
            &live_present,
            ExactProtectedContinuation::new(next_protected, next_observation),
        )
        .expect("the reusable method path must remain reopenable by a new protected distinction")
    else {
        panic!("the event-sensitive successor must reopen the reusable method path")
    };
    assert_ne!(next_reopen.continuation(), live_reopen.continuation());
    let next_reopen_envelope = next_reopen
        .envelope()
        .expect("next reopen witness must encode");
    let next_reopen_ref = persist(
        &store,
        &next_reopen_envelope,
        &next_reopen.referenced_artifacts(),
    )
    .await;

    let cue_domain_port = TypeSymbol::new("candidate").expect("cue domain port must be valid");
    let cue_answer_port = TypeSymbol::new("cue_answer").expect("cue answer port must be valid");
    let cue_relation_value = RelationSchema::new(
        catalog
            .schemas
            .get(&roots.relation)
            .expect("source relation must remain loaded")
            .binding(),
        vec![
            RelationPort::new(cue_domain_port.clone(), roots.unit),
            RelationPort::new(cue_answer_port.clone(), roots.unit),
        ],
        RelationBodyIR::BindingNative {
            contract: stored_ref(&store, b"multi-event-cue-relation-contract").await,
        },
        Vec::new(),
        vec![problem_ref.as_artifact_ref()],
    );
    let cue_relation_envelope = cue_relation_value
        .envelope()
        .expect("cue relation must encode");
    let cue_relation = RelationRef::from_artifact_ref(
        persist(
            &store,
            &cue_relation_envelope,
            &cue_relation_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_schema(cue_relation_value), cue_relation);
    let cue_relation_support_value = SupportEnvironmentArtifact::new(
        SupportSubjectRef::Relation(cue_relation),
        Vec::new(),
        vec![roots.raw_return],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        query.context().applicability(),
        query.context().scope(),
    )
    .expect("cue relation support must canonicalize");
    let cue_relation_support_envelope = cue_relation_support_value
        .envelope()
        .expect("cue relation support must encode");
    let cue_relation_support = SupportEnvironmentRef::from_artifact_ref(
        persist(
            &store,
            &cue_relation_support_envelope,
            &cue_relation_support_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_support(cue_relation_support_value),
        cue_relation_support
    );
    let cue_query_value = OpenQuery::new(
        cue_relation,
        Vec::new(),
        vec![
            OpenPort::new(cue_domain_port.clone(), DischargeMode::Probe),
            OpenPort::new(cue_answer_port.clone(), DischargeMode::Probe),
        ],
        RelationUseContext::new(
            query.context().scope(),
            query.context().applicability(),
            query.context().grain(),
            query.context().horizon(),
            DischargeMode::Probe,
            cue_relation_support.as_support_ref(),
            None,
        ),
    );
    let cue_query_envelope = cue_query_value.envelope().expect("cue query must encode");
    let cue_query = QueryRef::from_artifact_ref(
        persist(
            &store,
            &cue_query_envelope,
            &cue_query_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_query(cue_query_value), cue_query);
    let cue_method = MethodContract::new(
        cue_relation,
        query.context().applicability(),
        stored_ref(&store, b"multi-event-cue-method-law").await,
        method_coverage,
        DischargeMode::Probe,
        method_extension,
        method_backend,
        None,
        None,
        Vec::new(),
        vec![cue_query.as_artifact_ref(), problem_ref.as_artifact_ref()],
    )
    .expect("cue method must canonicalize");
    let cue_method_envelope = cue_method.envelope().expect("cue method must encode");
    let cue_method_ref = MethodRef::from_artifact_ref(
        persist(
            &store,
            &cue_method_envelope,
            &cue_method.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_method(cue_method.clone()), cue_method_ref);
    let cue_signature_context = SignatureContext::new(
        catalog
            .schemas
            .get(&cue_relation)
            .expect("cue relation must remain loaded")
            .binding(),
        query.context().scope(),
        query.context().applicability(),
        query.context().grain(),
        query.context().horizon(),
        roots.unit,
    );
    let cue_value = ExactFiniteCue::new(
        cue_method_ref,
        cue_domain_port,
        cue_answer_port,
        roots.unit,
        ExactFiniteSignature::new(
            cue_signature_context,
            vec![
                (
                    roots.answer_a.as_artifact_ref(),
                    roots.answer_a.as_artifact_ref(),
                ),
                (
                    roots.answer_b.as_artifact_ref(),
                    roots.answer_b.as_artifact_ref(),
                ),
            ],
        )
        .expect("cue signature must be exact over both live candidates"),
    );
    cue_method
        .check(&catalog)
        .expect("cue method relation must check");
    let cue_envelope = cue_value.envelope().expect("exact cue must encode");
    let cue_ref = persist(&store, &cue_envelope, &cue_value.referenced_artifacts()).await;
    let cue_claim_value = ClaimArtifact::new(
        cue_ref,
        cue_query,
        vec![roots.raw_return],
        vec![roots.decoded_path],
        query.context().scope(),
        query.context().applicability(),
        ClaimStatus::Checked,
    )
    .expect("cue coverage claim must canonicalize");
    let cue_claim_envelope = cue_claim_value
        .envelope()
        .expect("cue coverage claim must encode");
    let cue_claim = ClaimRef::from_artifact_ref(
        persist(
            &store,
            &cue_claim_envelope,
            &cue_claim_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_claim(cue_claim_value), cue_claim);
    let cue_support_value = SupportEnvironmentArtifact::new(
        SupportSubjectRef::Claim(cue_claim),
        Vec::new(),
        vec![roots.raw_return],
        vec![cue_ref],
        Vec::new(),
        Vec::new(),
        query.context().applicability(),
        query.context().scope(),
    )
    .expect("cue coverage support must canonicalize");
    let cue_support_envelope = cue_support_value
        .envelope()
        .expect("cue coverage support must encode");
    let cue_support = SupportEnvironmentRef::from_artifact_ref(
        persist(
            &store,
            &cue_support_envelope,
            &cue_support_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_support(cue_support_value), cue_support);
    let relation_only_standing = standing_from_declared_support(
        Vec::new(),
        &[DeclaredSupportClosure::for_subjects(
            cue_relation_support,
            Vec::new(),
            true,
            true,
            false,
        )],
        &catalog,
    )
    .expect("cue relation support must close independently");
    assert!(matches!(
        admit_exact_finite_cue(
            cue_value.clone(),
            cue_claim,
            cue_support,
            cue_relation_support,
            &relation_only_standing,
            &catalog,
        ),
        Ok(ExactFiniteCueAdmission::Unknown {
            residual: ExactFiniteCueUnknown::CueCoverageSupportIncomplete,
            ..
        })
    ));
    let cue_standing = standing_from_declared_support(
        Vec::new(),
        &[
            DeclaredSupportClosure::for_subjects(
                cue_relation_support,
                Vec::new(),
                true,
                true,
                false,
            ),
            DeclaredSupportClosure::for_subjects(cue_support, Vec::new(), true, true, false),
        ],
        &catalog,
    )
    .expect("cue relation and exact answer semantics must close through their own routes");
    assert!(matches!(
        admit_exact_finite_cue(
            cue_value.clone(),
            cue_claim,
            cue_relation_support,
            cue_relation_support,
            &cue_standing,
            &catalog,
        ),
        Err(ExactFiniteCueCheckError::SupportTargetMismatch)
    ));
    let ExactFiniteCueAdmission::Admitted(live_cue) = admit_exact_finite_cue(
        cue_value.clone(),
        cue_claim,
        cue_support,
        cue_relation_support,
        &cue_standing,
        &catalog,
    )
    .expect("typed cue references and contexts must check") else {
        panic!("fully supported exact cue must admit")
    };
    let live_cue = *live_cue;
    let generated_method = MethodContract::new(
        cue_relation,
        query.context().applicability(),
        stored_ref(&store, b"multi-event-generated-cue-method-law").await,
        method_coverage,
        DischargeMode::Generate,
        method_extension,
        method_backend,
        None,
        None,
        Vec::new(),
        vec![cue_query.as_artifact_ref()],
    )
    .expect("generated cue method must canonicalize");
    let generated_method_envelope = generated_method
        .envelope()
        .expect("generated cue method must encode");
    let generated_method_ref = MethodRef::from_artifact_ref(
        persist(
            &store,
            &generated_method_envelope,
            &generated_method.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_method(generated_method),
        generated_method_ref
    );
    let generated_cue = ExactFiniteCue::new(
        generated_method_ref,
        live_cue.cue().domain_port().clone(),
        live_cue.cue().answer_port().clone(),
        roots.unit,
        live_cue.signature().clone(),
    );
    let generated_cue_envelope = generated_cue
        .envelope()
        .expect("generated cue semantics must encode");
    let generated_cue_ref = persist(
        &store,
        &generated_cue_envelope,
        &generated_cue.referenced_artifacts(),
    )
    .await;
    let generated_claim_value = ClaimArtifact::new(
        generated_cue_ref,
        cue_query,
        vec![roots.raw_return],
        vec![roots.decoded_path],
        query.context().scope(),
        query.context().applicability(),
        ClaimStatus::Checked,
    )
    .expect("generated cue claim must canonicalize");
    let generated_claim_envelope = generated_claim_value
        .envelope()
        .expect("generated cue claim must encode");
    let generated_claim = ClaimRef::from_artifact_ref(
        persist(
            &store,
            &generated_claim_envelope,
            &generated_claim_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_claim(generated_claim_value), generated_claim);
    let generated_support_value = SupportEnvironmentArtifact::new(
        SupportSubjectRef::Claim(generated_claim),
        Vec::new(),
        vec![roots.raw_return],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        query.context().applicability(),
        query.context().scope(),
    )
    .expect("generated cue support must canonicalize");
    let generated_support_envelope = generated_support_value
        .envelope()
        .expect("generated cue support must encode");
    let generated_support = SupportEnvironmentRef::from_artifact_ref(
        persist(
            &store,
            &generated_support_envelope,
            &generated_support_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_support(generated_support_value),
        generated_support
    );
    let generated_standing = standing_from_declared_support(
        Vec::new(),
        &[
            DeclaredSupportClosure::for_subjects(
                cue_relation_support,
                Vec::new(),
                true,
                true,
                false,
            ),
            DeclaredSupportClosure::for_subjects(generated_support, Vec::new(), true, true, false),
        ],
        &catalog,
    )
    .expect("generated cue evidence may stand without becoming exact semantics");
    assert!(matches!(
        admit_exact_finite_cue(
            generated_cue,
            generated_claim,
            generated_support,
            cue_relation_support,
            &generated_standing,
            &catalog,
        ),
        Ok(ExactFiniteCueAdmission::Unknown {
            residual: ExactFiniteCueUnknown::GeneratedAnswerSemantics,
            ..
        })
    ));

    let unproven_probe_claim_value = ClaimArtifact::new(
        cue_ref,
        cue_query,
        Vec::new(),
        Vec::new(),
        query.context().scope(),
        query.context().applicability(),
        ClaimStatus::Checked,
    )
    .expect("unproven probe claim must remain representable");
    let unproven_probe_claim_envelope = unproven_probe_claim_value
        .envelope()
        .expect("unproven probe claim must encode");
    let unproven_probe_claim = ClaimRef::from_artifact_ref(
        persist(
            &store,
            &unproven_probe_claim_envelope,
            &unproven_probe_claim_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_claim(unproven_probe_claim_value),
        unproven_probe_claim
    );
    let unproven_probe_support_value = SupportEnvironmentArtifact::new(
        SupportSubjectRef::Claim(unproven_probe_claim),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        query.context().applicability(),
        query.context().scope(),
    )
    .expect("unproven probe support must remain representable");
    let unproven_probe_support_envelope = unproven_probe_support_value
        .envelope()
        .expect("unproven probe support must encode");
    let unproven_probe_support = SupportEnvironmentRef::from_artifact_ref(
        persist(
            &store,
            &unproven_probe_support_envelope,
            &unproven_probe_support_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_support(unproven_probe_support_value),
        unproven_probe_support
    );
    let unproven_probe_standing = standing_from_declared_support(
        Vec::new(),
        &[
            DeclaredSupportClosure::for_subjects(
                cue_relation_support,
                Vec::new(),
                true,
                true,
                false,
            ),
            DeclaredSupportClosure::for_subjects(
                unproven_probe_support,
                Vec::new(),
                true,
                true,
                false,
            ),
        ],
        &catalog,
    )
    .expect("declared standing cannot manufacture missing probe provenance");
    assert!(matches!(
        admit_exact_finite_cue(
            cue_value.clone(),
            unproven_probe_claim,
            unproven_probe_support,
            cue_relation_support,
            &unproven_probe_standing,
            &catalog,
        ),
        Ok(ExactFiniteCueAdmission::Unknown {
            residual: ExactFiniteCueUnknown::MissingProbeProvenance,
            ..
        })
    ));
    let cue_protected = ExactFiniteSignature::new(
        cue_signature_context,
        vec![
            (
                roots.answer_a.as_artifact_ref(),
                roots.event.as_artifact_ref(),
            ),
            (
                roots.answer_b.as_artifact_ref(),
                second_event.as_artifact_ref(),
            ),
        ],
    )
    .expect("protected cue target must cover both live candidates");
    assert!(matches!(
        check_admitted_exact_finite_cue_basis(std::slice::from_ref(&live_cue), &cue_protected),
        Ok(ic_core::ExactFiniteCueBasisResult::Sufficient)
    ));
    let live_empty_cue_plan = plan_method_reentry_with_admitted_cues(
        live_method_reentry.clone(),
        Vec::new(),
        &cue_protected,
    )
    .expect("an empty admitted basis must return its concrete residual");
    assert!(matches!(
        &live_empty_cue_plan,
        MethodCuePlanning::Residual {
            reentry,
            separator,
            ..
        } if reentry.separator().separator().problem() == problem_ref
            && [
                (
                    separator.first_domain(),
                    separator.first_protected_value(),
                ),
                (
                    separator.second_domain(),
                    separator.second_protected_value(),
                ),
            ]
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>()
                == [
                    (
                        roots.answer_a.as_artifact_ref(),
                        roots.event.as_artifact_ref(),
                    ),
                    (
                        roots.answer_b.as_artifact_ref(),
                        second_event.as_artifact_ref(),
                    ),
                ]
                .into_iter()
                .collect::<std::collections::BTreeSet<_>>()
    ));
    let live_cue_plan = plan_method_reentry_with_admitted_cues(
        live_method_reentry.clone(),
        vec![live_cue.clone()],
        &cue_protected,
    )
    .expect("the admitted cue must discharge the protected finite pair");
    assert!(matches!(
        &live_cue_plan,
        MethodCuePlanning::Sufficient { reentry, cues }
            if reentry == &live_method_reentry && cues == std::slice::from_ref(&live_cue)
    ));

    store.close().await;
    let reopened = ArtifactStore::open(&url)
        .await
        .expect("multi-event store must cold reopen");
    reopened
        .migrate()
        .await
        .expect("cold migration must remain repeatable");
    let mut cold_catalog = load_cold_replay_catalog(&reopened, roots).await;
    let second_event_value = reopened
        .get_actual_event(second_event)
        .await
        .expect("second ledger event must revalidate after restart")
        .expect("second ledger event must remain present after restart");
    cold_catalog.events.insert(second_event, second_event_value);
    let cold_reopen =
        ExactFinitePresentReopenWitness::from_envelope(&load_envelope(&reopened, reopen_ref).await)
            .expect("reopen witness must decode after restart");
    assert_eq!(cold_reopen, live_reopen);
    let cold_problem = SeparatorProblem::from_envelope(
        &load_envelope(&reopened, problem_ref.as_artifact_ref()).await,
    )
    .expect("separator problem must decode after restart");
    assert_eq!(cold_problem, problem);
    cold_catalog
        .separator_problems
        .insert(problem_ref, cold_problem);
    let cold_generated =
        GeneratedInquiry::from_envelope(&load_envelope(&reopened, generated_ref).await)
            .expect("generated separator inquiry must decode after restart");
    assert_eq!(
        cold_generated
            .generated_inquiry_ref()
            .expect("cold generated inquiry must hash"),
        generated_ref
    );
    let cold_from_method = MethodContract::from_envelope(
        &load_envelope(&reopened, from_method_ref.as_artifact_ref()).await,
    )
    .expect("source method must decode after restart");
    let cold_to_method = MethodContract::from_envelope(
        &load_envelope(&reopened, to_method_ref.as_artifact_ref()).await,
    )
    .expect("target method must decode after restart");
    assert_eq!(
        cold_catalog.insert_method(cold_from_method),
        from_method_ref
    );
    assert_eq!(cold_catalog.insert_method(cold_to_method), to_method_ref);
    let cold_guard = FormulaArtifact::from_envelope(
        &load_envelope(&reopened, guard_ref.as_artifact_ref()).await,
    )
    .expect("reentry guard must decode after restart");
    assert_eq!(cold_catalog.insert_formula(cold_guard), guard_ref);
    let cold_reconstruct = IProgArtifact::from_envelope(
        &load_envelope(&reopened, reconstruct_ref.as_artifact_ref()).await,
    )
    .expect("input reconstruction must decode after restart");
    assert_eq!(
        cold_catalog.insert_program(cold_reconstruct),
        reconstruct_ref
    );
    let cold_bridge = MethodBridge::from_envelope(&load_envelope(&reopened, bridge_ref).await)
        .expect("method bridge must decode after restart");
    assert_eq!(cold_bridge, bridge);
    let decoded_next_reopen = ExactFinitePresentReopenWitness::from_envelope(
        &load_envelope(&reopened, next_reopen_ref).await,
    )
    .expect("successor reopen witness must decode after restart");
    assert_eq!(decoded_next_reopen, next_reopen);
    let cold_cue_relation = RelationSchema::from_envelope(
        &load_envelope(&reopened, cue_relation.as_artifact_ref()).await,
    )
    .expect("cue relation must decode after restart");
    assert_eq!(cold_catalog.insert_schema(cold_cue_relation), cue_relation);
    let cold_cue_relation_support = SupportEnvironmentArtifact::from_envelope(
        &load_envelope(&reopened, cue_relation_support.as_artifact_ref()).await,
    )
    .expect("cue relation support must decode after restart");
    assert_eq!(
        cold_catalog.insert_support(cold_cue_relation_support),
        cue_relation_support
    );
    let cold_cue_query =
        OpenQuery::from_envelope(&load_envelope(&reopened, cue_query.as_artifact_ref()).await)
            .expect("cue source question must decode after restart");
    assert_eq!(cold_catalog.insert_query(cold_cue_query), cue_query);
    let cold_cue_method = MethodContract::from_envelope(
        &load_envelope(&reopened, cue_method_ref.as_artifact_ref()).await,
    )
    .expect("cue method must decode after restart");
    assert_eq!(cold_catalog.insert_method(cold_cue_method), cue_method_ref);
    let cold_cue = ExactFiniteCue::from_envelope(&load_envelope(&reopened, cue_ref).await)
        .expect("exact cue must decode after restart");
    assert_eq!(cold_cue, cue_value);
    let cold_cue_claim =
        ClaimArtifact::from_envelope(&load_envelope(&reopened, cue_claim.as_artifact_ref()).await)
            .expect("cue coverage claim must decode after restart");
    assert_eq!(cold_catalog.insert_claim(cold_cue_claim), cue_claim);
    let cold_cue_support = SupportEnvironmentArtifact::from_envelope(
        &load_envelope(&reopened, cue_support.as_artifact_ref()).await,
    )
    .expect("cue coverage support must decode after restart");
    assert_eq!(cold_catalog.insert_support(cold_cue_support), cue_support);
    let cold_cue_standing = standing_from_declared_support(
        Vec::new(),
        &[
            DeclaredSupportClosure::for_subjects(
                cue_relation_support,
                Vec::new(),
                true,
                true,
                false,
            ),
            DeclaredSupportClosure::for_subjects(cue_support, Vec::new(), true, true, false),
        ],
        &cold_catalog,
    )
    .expect("cold cue support routes must reconstruct standing");
    let ExactFiniteCueAdmission::Admitted(cold_admitted_cue) = admit_exact_finite_cue(
        cold_cue,
        cue_claim,
        cue_support,
        cue_relation_support,
        &cold_cue_standing,
        &cold_catalog,
    )
    .expect("cold cue admission must recheck every stored dependency") else {
        panic!("cold exact cue must remain admitted")
    };
    let cold_admitted_cue = *cold_admitted_cue;
    assert_eq!(cold_admitted_cue, live_cue);
    assert_eq!(
        check_admitted_exact_finite_cue_basis(
            std::slice::from_ref(&cold_admitted_cue),
            &cue_protected,
        )
        .expect("cold admitted cue basis must check"),
        ic_core::ExactFiniteCueBasisResult::Sufficient
    );
    let cold_source = cold_catalog
        .resolve_iprog(roots.source)
        .expect("source must reload after restart");
    let cold_decoder = cold_catalog
        .resolve_finite_decoder(roots.decoded_decoder)
        .expect("decoder must reload after restart");
    let cold_query = OpenQueryCatalog::resolve_open_query(&cold_catalog, roots.query)
        .expect("query must reload after restart");
    let cold_standing = standing_from_declared_support(
        Vec::new(),
        &[DeclaredSupportClosure::for_subjects(
            roots.support,
            Vec::new(),
            true,
            true,
            false,
        )],
        &cold_catalog,
    )
    .expect("support must reconstruct after restart");
    let cold_runtime = ProgramIR::new(
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
        ],
    );
    cold_runtime
        .verify(&cold_catalog)
        .expect("cold lowering must regenerate without a persisted runtime");
    let MachineStep::Suspended(cold_suspension) = cold_runtime
        .step(cold_runtime.start())
        .expect("cold entry must suspend")
    else {
        panic!("cold entry must be a probe suspension")
    };
    let cold_first = replay_completed_finite_probe(
        &reopened,
        roots.token,
        &cold_decoder,
        roots.decoded_path,
        &observations,
        &cold_standing,
        &cold_source,
        cold_suspension,
        ContinuationLowering::new(roots.continuation, BlockTarget::new(1)),
        &cold_runtime,
        &cold_catalog,
    )
    .await
    .expect("first event must cold replay without redispatch");
    let cold_second = replay_completed_finite_probe(
        &reopened,
        second_token,
        &cold_decoder,
        roots.decoded_path,
        &observations,
        &cold_standing,
        &cold_source,
        cold_suspension,
        ContinuationLowering::new(roots.continuation, BlockTarget::new(1)),
        &cold_runtime,
        &cold_catalog,
    )
    .await
    .expect("second event must cold replay without redispatch");
    let cold_separator = replay_completed_finite_separator_inquiry(
        &reopened,
        second_token,
        cold_generated,
        &cold_decoder,
        roots.decoded_path,
        &observations,
        &cold_standing,
        &cold_source,
        cold_suspension,
        ContinuationLowering::new(roots.continuation, BlockTarget::new(1)),
        &cold_runtime,
        &cold_catalog,
    )
    .await
    .expect("separator inquiry must cold replay without redispatch or a method switch");
    assert_eq!(&cold_separator, live_method_reentry.separator());
    let cold_method_reentry =
        route_separator_through_method_bridge(cold_separator, cold_bridge, &cold_catalog)
            .expect("cold replay must reenter through the decoded transparent method bridge");
    assert_eq!(cold_method_reentry, live_method_reentry);
    let cold_empty_cue_plan = plan_method_reentry_with_admitted_cues(
        cold_method_reentry.clone(),
        Vec::new(),
        &cue_protected,
    )
    .expect("cold empty cue basis must preserve its concrete residual");
    let cold_cue_plan = plan_method_reentry_with_admitted_cues(
        cold_method_reentry,
        vec![cold_admitted_cue],
        &cue_protected,
    )
    .expect("cold admitted cue must discharge the same protected pair");
    assert_eq!(cold_empty_cue_plan, live_empty_cue_plan);
    assert_eq!(cold_cue_plan, live_cue_plan);
    let cold_first_trace =
        PairedActualityTrace::derive(cold_first.actuality().event(), cold_first.resumption())
            .expect("first cold trace must derive");
    let cold_second_trace =
        PairedActualityTrace::derive(cold_second.actuality().event(), cold_second.resumption())
            .expect("second cold trace must derive");
    let cold_traversal = PairedActualityTraversal::new(
        vec![cold_first_trace.clone(), cold_second_trace.clone()],
        vec![roots.event, second_event],
        TraversalCausalOrder::Unknown,
    )
    .expect("cold ledger traversal must reconstruct without causal inference");
    let (cold_present, cold_reopen) = derive_event_sufficient_present(
        roots,
        &cold_query,
        &cold_first_trace,
        &cold_second_trace,
        current_protected,
        event_protected,
    );
    assert_eq!(cold_first_trace, first_trace);
    assert_eq!(cold_second_trace, second_trace);
    assert_eq!(cold_traversal, unknown_traversal);
    assert_eq!(cold_present, live_present);
    assert_eq!(cold_reopen, live_reopen);
    let cold_next_observation = ExactFiniteSignature::new(
        event_context,
        vec![
            (roots.event.as_artifact_ref(), roots.event.as_artifact_ref()),
            (
                second_event.as_artifact_ref(),
                second_event.as_artifact_ref(),
            ),
        ],
    )
    .expect("cold successor separator must cover the complete history domain");
    let ExactFinitePresentChallenge::Reopened(cold_next_reopen) =
        challenge_exact_finite_sufficient_present(
            &cold_present,
            ExactProtectedContinuation::new(next_protected, cold_next_observation),
        )
        .expect("cold method path must regenerate its reopening discriminator")
    else {
        panic!("cold reusable path must remain reopenable")
    };
    assert_eq!(cold_next_reopen, next_reopen);
    assert_eq!(
        provider_calls.load(Ordering::SeqCst),
        2,
        "both replayed events must use their committed returns rather than dispatch again"
    );
    reopened.close().await;
    std::fs::remove_file(path).expect("temporary multi-event database must be removable");
}

#[tokio::test]
async fn ollama_values_become_post_actuality_typed_answers_and_cold_replay_without_redispatch() {
    let (path, roots, provider_calls, _trace, _present, _reopen) =
        persisted_cold_replay_fixture().await;
    let url = format!("sqlite://{}", path.to_string_lossy().replace('\\', "/"));
    let store = ArtifactStore::open(&url)
        .await
        .expect("post-actuality store must reopen");
    store
        .migrate()
        .await
        .expect("embedded migrations must remain repeatable");
    let mut catalog = load_cold_replay_catalog(&store, roots).await;
    assert_eq!(
        catalog.candidates.len(),
        2,
        "the preexisting generic fixture has no local-model candidate set"
    );
    let post = materialize_ollama_post_return_slice(&store, &mut catalog, roots).await;
    let source_query = OpenQueryCatalog::resolve_open_query(&catalog, roots.query)
        .expect("source query must remain available");
    assert_ne!(
        source_query.context().support(),
        post.support.as_support_ref(),
        "the local observation route must be formed after the actual raw return rather than reuse the pre-dispatch query support"
    );

    let source = catalog
        .resolve_iprog(roots.source)
        .expect("source Ask must remain available");
    let continuation = catalog
        .resolve_iprog(roots.continuation)
        .expect("source continuation must remain available");
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
        ],
    );
    runtime
        .verify(&catalog)
        .expect("fresh lowering must not depend on post-return candidates");
    let MachineStep::Suspended(suspension) = runtime
        .step(runtime.start())
        .expect("fresh lowering must suspend at the original probe")
    else {
        panic!("entry must suspend")
    };
    let standing = standing_from_declared_support(
        Vec::new(),
        &[DeclaredSupportClosure::for_subjects(
            post.support,
            Vec::new(),
            true,
            true,
            false,
        )],
        &catalog,
    )
    .expect("the independently formed post-return support route must close");
    let decoder = catalog
        .resolve_finite_decoder(post.decoder)
        .expect("post-return decoder must be in the fresh catalog");
    let live = replay_completed_finite_probe(
        &store,
        roots.token,
        &decoder,
        post.path,
        &[
            ReplayObservation::new(post.candidate_a, post.observation_a),
            ReplayObservation::new(post.candidate_b, post.observation_b),
        ],
        &standing,
        &source,
        suspension,
        ContinuationLowering::new(
            continuation.iprog_ref().expect("continuation must address"),
            BlockTarget::new(1),
        ),
        &runtime,
        &catalog,
    )
    .await
    .expect("every decoded local candidate must admit and bind after actuality");
    assert_eq!(live.actuality().event_ref(), roots.event);
    assert_eq!(live.actuality().raw_return_ref(), roots.raw_return);
    let mut expected_post_candidates = vec![post.candidate_a, post.candidate_b];
    expected_post_candidates.sort_unstable();
    assert_eq!(
        live.resumption().binding().answer().candidates(),
        expected_post_candidates
    );
    assert_eq!(
        provider_calls.load(Ordering::SeqCst),
        1,
        "post-actuality semantic admission must not invoke the provider"
    );
    store.close().await;

    let reopened = ArtifactStore::open(&url)
        .await
        .expect("cold replay store must reopen");
    reopened
        .migrate()
        .await
        .expect("embedded migrations must remain repeatable");
    let mut cold_catalog = load_cold_replay_catalog(&reopened, roots).await;
    let stored_raw = cold_catalog
        .resolve_raw_return(roots.raw_return)
        .expect("committed raw return must reload");
    let regenerated_values =
        materialize_ollama_decoded_texts(roots.raw_return, &stored_raw, post.decoder_version)
            .expect("cold replay must re-run the decoder from only raw/version roots");
    let regenerated_value_refs = regenerated_values
        .iter()
        .map(OllamaDecodedText::artifact_ref)
        .collect::<Result<Vec<_>, _>>()
        .expect("regenerated local values must address");
    assert_eq!(regenerated_value_refs, [post.value_a, post.value_b]);
    for reference in [post.value_a, post.value_b] {
        let stored = load_envelope(&reopened, reference).await;
        assert_eq!(stored.kind().as_str(), OLLAMA_DECODED_TEXT_ARTIFACT_KIND);
        let value = OllamaDecodedText::from_envelope(&stored)
            .expect("stored local value must retain its decoder contract");
        value
            .check(&stored_raw)
            .expect("stored local value must recheck against the preserved raw return");
    }

    let binding = cold_catalog
        .resolve_type(roots.unit)
        .expect("answer type must reload")
        .binding();
    let regenerated_form_a = TypedForm::new(binding, roots.unit, post.value_a);
    let regenerated_form_b = TypedForm::new(binding, roots.unit, post.value_b);
    assert_eq!(
        regenerated_form_a
            .typed_form_ref()
            .expect("first regenerated form must address"),
        post.form_a
    );
    assert_eq!(
        regenerated_form_b
            .typed_form_ref()
            .expect("second regenerated form must address"),
        post.form_b
    );
    assert_eq!(cold_catalog.insert_form(regenerated_form_a), post.form_a);
    assert_eq!(cold_catalog.insert_form(regenerated_form_b), post.form_b);
    let query = OpenQueryCatalog::resolve_open_query(&cold_catalog, roots.query)
        .expect("source query must reload");
    let answer_port = query
        .open_ports()
        .first()
        .expect("source query must retain its answer port")
        .port()
        .clone();
    let regenerated_candidate_a = query
        .plug(
            vec![PortBinding::new(answer_port.clone(), post.form_a)],
            &cold_catalog,
        )
        .expect("first regenerated local value must fill the source query");
    let regenerated_candidate_b = query
        .plug(
            vec![PortBinding::new(answer_port.clone(), post.form_b)],
            &cold_catalog,
        )
        .expect("second regenerated local value must fill the source query");
    assert_eq!(
        regenerated_candidate_a
            .completion_candidate_ref()
            .expect("first regenerated candidate must address"),
        post.candidate_a
    );
    assert_eq!(
        regenerated_candidate_b
            .completion_candidate_ref()
            .expect("second regenerated candidate must address"),
        post.candidate_b
    );
    assert_eq!(
        cold_catalog.insert_candidate(regenerated_candidate_a),
        post.candidate_a
    );
    assert_eq!(
        cold_catalog.insert_candidate(regenerated_candidate_b),
        post.candidate_b
    );
    let support = SupportEnvironmentArtifact::from_envelope(
        &load_envelope(&reopened, post.support.as_artifact_ref()).await,
    )
    .expect("post-return support must reload");
    assert_eq!(cold_catalog.insert_support(support), post.support);
    for reference in [post.observation_a, post.observation_b] {
        let use_value = RelationUse::from_envelope(
            &load_envelope(&reopened, reference.as_artifact_ref()).await,
        )
        .expect("post-return observation must reload");
        assert_eq!(cold_catalog.insert_relation_use(use_value), reference);
    }
    let replay_decoder = FiniteDecoder::from_envelope(
        &load_envelope(&reopened, post.decoder.as_artifact_ref()).await,
    )
    .expect("post-return decoder must reload");
    assert_eq!(
        cold_catalog.insert_decoder(replay_decoder.clone()),
        post.decoder
    );
    let replay_path =
        ResolutionPath::from_envelope(&load_envelope(&reopened, post.path.as_artifact_ref()).await)
            .expect("post-return path must reload");
    assert_eq!(cold_catalog.insert_path(replay_path), post.path);
    let replay_source = cold_catalog
        .resolve_iprog(roots.source)
        .expect("source Ask must reload");
    let replay_continuation = cold_catalog
        .resolve_iprog(roots.continuation)
        .expect("source continuation must reload");
    let replay_runtime = ProgramIR::new(
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
        ],
    );
    replay_runtime
        .verify(&cold_catalog)
        .expect("cold regenerated lowering must verify");
    let MachineStep::Suspended(replay_suspension) = replay_runtime
        .step(replay_runtime.start())
        .expect("cold regenerated lowering must suspend")
    else {
        panic!("cold entry must suspend")
    };
    let replay_standing = standing_from_declared_support(
        Vec::new(),
        &[DeclaredSupportClosure::for_subjects(
            post.support,
            Vec::new(),
            true,
            true,
            false,
        )],
        &cold_catalog,
    )
    .expect("cold post-return support must reconstruct standing");
    let replayed = replay_completed_finite_probe(
        &reopened,
        roots.token,
        &replay_decoder,
        post.path,
        &[
            ReplayObservation::new(post.candidate_a, post.observation_a),
            ReplayObservation::new(post.candidate_b, post.observation_b),
        ],
        &replay_standing,
        &replay_source,
        replay_suspension,
        ContinuationLowering::new(
            replay_continuation
                .iprog_ref()
                .expect("cold continuation must address"),
            BlockTarget::new(1),
        ),
        &replay_runtime,
        &cold_catalog,
    )
    .await
    .expect("cold replay must bind the regenerated full local candidate set");
    assert_eq!(replayed.actuality().event_ref(), roots.event);
    assert_eq!(replayed.actuality().raw_return_ref(), roots.raw_return);
    assert_eq!(
        replayed.resumption().binding().answer().candidates(),
        expected_post_candidates
    );
    assert_eq!(
        provider_calls.load(Ordering::SeqCst),
        1,
        "cold replay must never redispatch the local-model provider"
    );
    reopened.close().await;
    std::fs::remove_file(path).expect("temporary cold replay database must be removable");
}

#[tokio::test]
#[ignore = "requires local Ollama with qwen3.5:9b"]
async fn live_ollama_call_creates_typed_answers_only_after_actuality_and_cold_replays() {
    let path = fresh_sqlite_path("inquiry-calculus-live-ollama-replay");
    let url = format!("sqlite://{}", path.to_string_lossy().replace('\\', "/"));
    let store = ArtifactStore::open(&url)
        .await
        .expect("file-backed live Ollama store must open");
    store.migrate().await.expect("migrations must apply");
    let mut catalog = Catalog::default();

    let binding = BindingVersionRef::from_artifact_ref(stored_ref(&store, b"live-binding").await);
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
    let chart_form_value = TypedForm::new(binding, unit, stored_ref(&store, b"chart-form").await);
    let chart_form = TypedFormRef::from_artifact_ref(
        persist(
            &store,
            &chart_form_value.envelope().expect("chart form must encode"),
            &chart_form_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_form(chart_form_value), chart_form);
    let return_form_value = TypedForm::new(binding, unit, stored_ref(&store, b"return-form").await);
    let return_form = TypedFormRef::from_artifact_ref(
        persist(
            &store,
            &return_form_value
                .envelope()
                .expect("return form must encode"),
            &return_form_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_form(return_form_value), return_form);

    let answer_port = TypeSymbol::new("answer").expect("answer port must be valid");
    let relation_value = RelationSchema::new(
        binding,
        vec![RelationPort::new(answer_port.clone(), unit)],
        RelationBodyIR::BindingNative {
            contract: stored_ref(&store, b"live-relation-contract").await,
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
    let scope = ScopeRef::from_artifact_ref(stored_ref(&store, b"live-scope").await);
    let applicability =
        ApplicabilityRef::from_artifact_ref(stored_ref(&store, b"live-applicability").await);
    let pre_support_value = SupportEnvironmentArtifact::new(
        SupportSubjectRef::Relation(relation),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        vec![stored_ref(&store, b"pre-actuality-support-assumption").await],
        Vec::new(),
        applicability,
        scope,
    )
    .expect("pre-actuality support must encode");
    let pre_support = SupportEnvironmentRef::from_artifact_ref(
        persist(
            &store,
            &pre_support_value
                .envelope()
                .expect("pre-actuality support must encode"),
            &pre_support_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_support(pre_support_value), pre_support);
    let grain = GrainRef::from_artifact_ref(stored_ref(&store, b"live-grain").await);
    let horizon = HorizonRef::from_artifact_ref(stored_ref(&store, b"live-horizon").await);
    let query_context = RelationUseContext::new(
        scope,
        applicability,
        grain,
        horizon,
        DischargeMode::Probe,
        pre_support.as_support_ref(),
        None,
    );
    let query_value = OpenQuery::new(
        relation,
        Vec::new(),
        vec![OpenPort::new(answer_port.clone(), DischargeMode::Probe)],
        query_context,
    );
    let query = QueryRef::from_artifact_ref(
        persist(
            &store,
            &query_value.envelope().expect("source query must encode"),
            &query_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_query(query_value), query);
    let chart_use_value = RelationUse::new(
        relation,
        vec![PortBinding::new(answer_port.clone(), chart_form)],
        query_context,
    );
    let chart_use = RelationUseRef::from_artifact_ref(
        persist(
            &store,
            &chart_use_value.envelope().expect("chart use must encode"),
            &chart_use_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_relation_use(chart_use_value), chart_use);
    let chart_value = BoundaryChart::new(
        query,
        unit,
        unit,
        unit,
        relation,
        relation,
        ic_core::DeterminationPresentationRef::from_artifact_ref(
            stored_ref(&store, b"live-determination").await,
        ),
        None,
        Vec::new(),
        Vec::new(),
        chart_use,
        FormulaRef::from_artifact_ref(stored_ref(&store, b"live-compatibility").await),
        None,
        grain,
        horizon,
    );
    let boundary = BoundaryRef::from_artifact_ref(
        persist(
            &store,
            &chart_value.envelope().expect("boundary chart must encode"),
            &chart_value.referenced_artifacts(),
        )
        .await,
    );
    catalog.charts.insert(boundary, chart_value);

    let active_view = stored_ref(&store, b"live-active-view").await;
    let backend = stored_ref(&store, b"live-backend").await;
    let executable_code = stored_ref(&store, b"live-executable-code").await;
    let decoder_contract = stored_ref(&store, b"live-decoder-contract").await;
    let probe_contract =
        ProbeContractRef::from_artifact_ref(stored_ref(&store, b"live-probe-contract").await);
    let compiler_version = stored_ref(&store, b"live-compiler-version").await;
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
    catalog.operators.insert(operator, operator_value);
    let plan_value = SurfacePlan::new(
        operator,
        query,
        boundary,
        active_view,
        executable_code,
        probe_contract,
        stored_ref(&store, b"live-renderer-version").await,
        stored_ref(&store, b"live-rendered-body").await,
    );
    let plan = ic_core::SurfacePlanRef::from_artifact_ref(
        persist(
            &store,
            &plan_value.envelope().expect("surface plan must encode"),
            &plan_value.referenced_artifacts(),
        )
        .await,
    );
    let request_json = serde_json::to_vec(&serde_json::json!({
        "model": "qwen3.5:9b",
        "prompt": "Return exactly two distinct one-word candidate completions.",
        "stream": false,
        "think": false,
        "options": {"temperature": 0},
        "format": {
            "type": "object",
            "properties": {
                "candidates": {
                    "type": "array",
                    "items": {"type": "string"},
                    "minItems": 2,
                    "maxItems": 2
                }
            },
            "required": ["candidates"],
            "additionalProperties": false
        }
    }))
    .expect("live local request must encode");
    let request_body = stored_ref(&store, &request_json).await;
    let request_value = BackendRequest::new(
        operator,
        plan,
        query,
        boundary,
        backend,
        executable_code,
        compiler_version,
        stored_ref(&store, b"live-backend-version").await,
        request_body,
    );
    let request = ic_core::BackendRequestRef::from_artifact_ref(
        persist(
            &store,
            &request_value
                .envelope()
                .expect("backend request must encode"),
            &request_value.referenced_artifacts(),
        )
        .await,
    );
    let continuation_value = IProgArtifact::new(unit, IProgIR::Return { value: return_form });
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
    let source_value = IProgArtifact::new(
        unit,
        IProgIR::Ask {
            question: query,
            environment: Vec::new(),
            answer_slot: TypeSymbol::new("answer_set").expect("slot must be valid"),
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
            BasicBlock::new(
                BlockTarget::new(1),
                Terminator::Return { value: return_form },
            ),
        ],
    );
    runtime
        .verify(&catalog)
        .expect("pre-actuality runtime must verify without candidates");
    assert!(catalog.candidates.is_empty());
    assert_eq!(catalog.support.len(), 1);
    let MachineStep::Suspended(suspension) = runtime
        .step(runtime.start())
        .expect("source lowering must suspend at probe")
    else {
        panic!("entry must suspend")
    };
    let dispatch_context = ProbeDispatchContext::new(
        None,
        StateRef::from_artifact_ref(stored_ref(&store, b"live-state-before").await),
        None,
        StateRef::from_artifact_ref(stored_ref(&store, b"live-state-after").await),
        grain,
        RouteRef::from_artifact_ref(stored_ref(&store, b"live-route").await),
        binding,
        ProvenanceRef::from_artifact_ref(stored_ref(&store, b"live-provenance").await),
    );
    let provider_calls = Arc::new(AtomicUsize::new(0));
    let mut provider = CountingOllamaProvider {
        calls: Arc::clone(&provider_calls),
        inner: OllamaGenerateProvider::new(request_body, request_json)
            .expect("local Ollama provider must configure"),
    };
    let token = DispatchToken::from_bytes([0xd3; 32]);
    let actual = dispatch_probe(
        &store,
        suspension,
        token,
        request,
        dispatch_context,
        &mut provider,
    )
    .await
    .expect("fresh local model return must commit before interpretation");
    assert_eq!(provider_calls.load(Ordering::SeqCst), 1);
    let raw_return = actual.raw_return_ref();
    let event = actual.event_ref();
    catalog.insert_raw_return(actual.raw_return().clone());
    catalog.events.insert(event, actual.event().clone());

    let base_roots = ColdReplayRoots {
        token,
        unit,
        raw_type,
        answer_a: return_form,
        answer_b: chart_form,
        relation,
        query,
        wrong_query: query,
        candidate_a: CompletionCandidateRef::from_artifact_ref(artifact(0xee)),
        candidate_b: CompletionCandidateRef::from_artifact_ref(artifact(0xef)),
        observation_a: RelationUseRef::from_artifact_ref(artifact(0xf0)),
        observation_b: RelationUseRef::from_artifact_ref(artifact(0xf1)),
        support: pre_support,
        decoded_decoder: FiniteDecoderRef::from_artifact_ref(artifact(0xf2)),
        alternate_decoded_decoder: FiniteDecoderRef::from_artifact_ref(artifact(0xf3)),
        undefined_decoder: FiniteDecoderRef::from_artifact_ref(artifact(0xf4)),
        unknown_decoder: FiniteDecoderRef::from_artifact_ref(artifact(0xf5)),
        decoded_path: ResolutionPathRef::from_artifact_ref(artifact(0xf6)),
        alternate_decoded_path: ResolutionPathRef::from_artifact_ref(artifact(0xf7)),
        undefined_path: ResolutionPathRef::from_artifact_ref(artifact(0xf8)),
        unknown_path: ResolutionPathRef::from_artifact_ref(artifact(0xf9)),
        boundary,
        operator,
        rival_operator: operator,
        source,
        wrong_source: source,
        capture_source: source,
        continuation,
        event,
        raw_return,
        alternate_raw_return: raw_return,
        current_protected: ProtectedContinuationRef::from_artifact_ref(
            stored_ref(&store, b"live-current-protected").await,
        ),
        path_protected: ProtectedContinuationRef::from_artifact_ref(
            stored_ref(&store, b"live-path-protected").await,
        ),
        compiler_version,
    };
    let post = materialize_ollama_post_return_slice(&store, &mut catalog, base_roots).await;
    let source_query = OpenQueryCatalog::resolve_open_query(&catalog, query)
        .expect("source query must remain available");
    assert_ne!(
        source_query.context().support(),
        post.support.as_support_ref()
    );
    let standing = standing_from_declared_support(
        Vec::new(),
        &[DeclaredSupportClosure::for_subjects(
            post.support,
            Vec::new(),
            true,
            true,
            false,
        )],
        &catalog,
    )
    .expect("fresh post-return local support must close");
    let decoder = catalog
        .resolve_finite_decoder(post.decoder)
        .expect("fresh local decoder must exist");
    let source_value = catalog
        .resolve_iprog(source)
        .expect("source Ask must remain available");
    let live = replay_completed_finite_probe(
        &store,
        token,
        &decoder,
        post.path,
        &[
            ReplayObservation::new(post.candidate_a, post.observation_a),
            ReplayObservation::new(post.candidate_b, post.observation_b),
        ],
        &standing,
        &source_value,
        suspension,
        ContinuationLowering::new(continuation, BlockTarget::new(1)),
        &runtime,
        &catalog,
    )
    .await
    .expect("the fresh local return must bind and resume every decoded candidate");
    assert_eq!(live.actuality().event_ref(), event);
    assert_eq!(live.actuality().raw_return_ref(), raw_return);
    assert_eq!(provider_calls.load(Ordering::SeqCst), 1);

    let roots = ColdReplayRoots {
        candidate_a: post.candidate_a,
        candidate_b: post.candidate_b,
        observation_a: post.observation_a,
        observation_b: post.observation_b,
        support: post.support,
        decoded_decoder: post.decoder,
        alternate_decoded_decoder: post.decoder,
        undefined_decoder: post.decoder,
        unknown_decoder: post.decoder,
        decoded_path: post.path,
        alternate_decoded_path: post.path,
        undefined_path: post.path,
        unknown_path: post.path,
        ..base_roots
    };
    store.close().await;

    let reopened = ArtifactStore::open(&url)
        .await
        .expect("fresh local replay store must reopen");
    reopened
        .migrate()
        .await
        .expect("embedded migrations must remain repeatable");
    let mut cold_catalog = load_cold_replay_catalog(&reopened, roots).await;
    let stored_raw = cold_catalog
        .resolve_raw_return(raw_return)
        .expect("fresh local raw return must reload");
    let values = materialize_ollama_decoded_texts(raw_return, &stored_raw, post.decoder_version)
        .expect("cold replay must decode the fresh local raw return again");
    let value_refs = values
        .iter()
        .map(OllamaDecodedText::artifact_ref)
        .collect::<Result<Vec<_>, _>>()
        .expect("cold local values must address");
    assert_eq!(value_refs, [post.value_a, post.value_b]);
    let form_a = TypedForm::new(binding, unit, post.value_a);
    let form_b = TypedForm::new(binding, unit, post.value_b);
    assert_eq!(
        form_a.typed_form_ref().expect("first form must address"),
        post.form_a
    );
    assert_eq!(
        form_b.typed_form_ref().expect("second form must address"),
        post.form_b
    );
    assert_eq!(cold_catalog.insert_form(form_a), post.form_a);
    assert_eq!(cold_catalog.insert_form(form_b), post.form_b);
    let replay_decoder = cold_catalog
        .resolve_finite_decoder(post.decoder)
        .expect("post-return local decoder must reload");
    let replay_source = cold_catalog
        .resolve_iprog(source)
        .expect("source Ask must reload");
    let replay_runtime = ProgramIR::new(
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
            BasicBlock::new(
                BlockTarget::new(1),
                Terminator::Return { value: return_form },
            ),
        ],
    );
    replay_runtime
        .verify(&cold_catalog)
        .expect("cold local lowering must verify");
    let MachineStep::Suspended(replay_suspension) = replay_runtime
        .step(replay_runtime.start())
        .expect("cold local lowering must suspend")
    else {
        panic!("cold local entry must suspend")
    };
    let replay_standing = standing_from_declared_support(
        Vec::new(),
        &[DeclaredSupportClosure::for_subjects(
            post.support,
            Vec::new(),
            true,
            true,
            false,
        )],
        &cold_catalog,
    )
    .expect("cold local support must close again");
    let replayed = replay_completed_finite_probe(
        &reopened,
        token,
        &replay_decoder,
        post.path,
        &[
            ReplayObservation::new(post.candidate_a, post.observation_a),
            ReplayObservation::new(post.candidate_b, post.observation_b),
        ],
        &replay_standing,
        &replay_source,
        replay_suspension,
        ContinuationLowering::new(continuation, BlockTarget::new(1)),
        &replay_runtime,
        &cold_catalog,
    )
    .await
    .expect("cold replay must resume the fresh local model answer without redispatch");
    assert_eq!(replayed.actuality().event_ref(), event);
    assert_eq!(replayed.actuality().raw_return_ref(), raw_return);
    assert_eq!(provider_calls.load(Ordering::SeqCst), 1);
    reopened.close().await;
    std::fs::remove_file(path).expect("temporary live local replay database must be removable");
}

#[tokio::test]
// Test boundary QACTUAL-SEPARATION-001:
// F = equal question/operator/raw-return projections collapse two realized source Ask occurrences.
// C = source-linked event schema plus cold EventFor recheck against a reconstructed occurrence.
// Omega/M = two sequential file-backed Probe dispatches sharing request and provider bytes.
// P/V/E/U = close/reopen replay and independent source re-walk; reopened by generalized
// multi-port lowering, which is checked separately by the finite discharge-bundle fixture.
async fn source_linked_events_preserve_equal_projection_occurrences_after_restart() {
    let (path, roots, provider_calls, _, _, _) = persisted_cold_replay_fixture().await;
    let url = format!("sqlite://{}", path.to_string_lossy().replace('\\', "/"));
    let store = ArtifactStore::open(&url)
        .await
        .expect("occurrence fixture store must reopen");
    store.migrate().await.expect("migrations must repeat");
    let mut catalog = load_cold_replay_catalog(&store, roots).await;
    let original = store
        .replay_completed_external_effect(roots.token)
        .await
        .expect("predecessor effect must replay");
    let request_ref = original.request_ref();
    let request = original.request().clone();
    let binding = catalog
        .resolve_type(roots.unit)
        .expect("unit type must reload")
        .binding();

    let second_continuation_value = IProgArtifact::new(
        roots.unit,
        IProgIR::Return {
            value: roots.answer_b,
        },
    );
    let second_continuation = IProgRef::from_artifact_ref(
        persist(
            &store,
            &second_continuation_value
                .envelope()
                .expect("second continuation must encode"),
            &second_continuation_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_program(second_continuation_value),
        second_continuation
    );
    let second_source_value = IProgArtifact::new(
        roots.unit,
        IProgIR::Ask {
            question: roots.query,
            environment: Vec::new(),
            answer_slot: TypeSymbol::new("second_answer").expect("slot must be valid"),
            continuation: second_continuation,
        },
    );
    let second_source = IProgRef::from_artifact_ref(
        persist(
            &store,
            &second_source_value
                .envelope()
                .expect("second source must encode"),
            &second_source_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_program(second_source_value), second_source);

    let first_provenance =
        ProvenanceRef::from_artifact_ref(stored_ref(&store, b"occurrence-first-provenance").await);
    let second_provenance =
        ProvenanceRef::from_artifact_ref(stored_ref(&store, b"occurrence-second-provenance").await);
    let first_source_value = SourceConfig::new(
        roots.unit,
        roots.source,
        Vec::new(),
        binding,
        roots.compiler_version,
        first_provenance,
    )
    .expect("first source configuration must encode");
    let second_source_value = SourceConfig::new(
        roots.unit,
        second_source,
        Vec::new(),
        binding,
        roots.compiler_version,
        second_provenance,
    )
    .expect("second source configuration must encode");
    let first_source_ref = SourceConfigRef::from_artifact_ref(
        persist(
            &store,
            &first_source_value
                .envelope()
                .expect("first source configuration must encode"),
            &first_source_value.referenced_artifacts(),
        )
        .await,
    );
    let second_source_ref = SourceConfigRef::from_artifact_ref(
        persist(
            &store,
            &second_source_value
                .envelope()
                .expect("second source configuration must encode"),
            &second_source_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_source_config(first_source_value.clone()),
        first_source_ref
    );
    assert_eq!(
        catalog.insert_source_config(second_source_value.clone()),
        second_source_ref
    );
    let first_occurrence = first_source_value
        .ask_occurrences(&catalog)
        .expect("first source must re-walk")
        .into_iter()
        .next()
        .expect("first source must contain an Ask");
    let second_occurrence = second_source_value
        .ask_occurrences(&catalog)
        .expect("second source must re-walk")
        .into_iter()
        .next()
        .expect("second source must contain an Ask");
    let persist_occurrence = |occurrence: &AskOccurrence| {
        let mut references = vec![
            occurrence.source_config().as_artifact_ref(),
            occurrence.question().as_artifact_ref(),
            occurrence.continuation().as_artifact_ref(),
            occurrence.binding_version().as_artifact_ref(),
            occurrence.compiler_version(),
            occurrence.provenance().as_artifact_ref(),
        ];
        references.extend(
            occurrence
                .environment()
                .iter()
                .map(|binding| binding.value().as_artifact_ref()),
        );
        references
    };
    let first_occurrence_ref = first_occurrence
        .ask_occurrence_ref()
        .expect("first occurrence must encode");
    let second_occurrence_ref = second_occurrence
        .ask_occurrence_ref()
        .expect("second occurrence must encode");
    persist(
        &store,
        &first_occurrence
            .envelope()
            .expect("first occurrence must encode"),
        &persist_occurrence(&first_occurrence),
    )
    .await;
    persist(
        &store,
        &second_occurrence
            .envelope()
            .expect("second occurrence must encode"),
        &persist_occurrence(&second_occurrence),
    )
    .await;
    assert_ne!(first_occurrence_ref, second_occurrence_ref);
    assert_eq!(first_occurrence.question(), second_occurrence.question());
    assert_ne!(
        first_occurrence.continuation(),
        second_occurrence.continuation()
    );

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
        ],
    );
    runtime.verify(&catalog).expect("runtime must verify");
    let source_ports = OpenQueryCatalog::resolve_open_query(&catalog, roots.query)
        .expect("source query must reload")
        .open_ports()
        .iter()
        .map(|open| PortLowering::new(open.port().clone(), open.mode()))
        .collect::<Vec<_>>();
    assert_eq!(
        source_ports.len(),
        1,
        "fixture isolates one source Ask port"
    );
    let source_runtime =
        RuntimeProgramArtifact::new(binding, roots.compiler_version, runtime.clone());
    let source_runtime_ref = source_runtime
        .runtime_program_ref()
        .expect("source runtime identity must encode");
    let persisted_source_runtime = persist(
        &store,
        &source_runtime
            .envelope()
            .expect("source runtime artifact must encode"),
        &source_runtime.referenced_artifacts(),
    )
    .await;
    assert_eq!(
        persisted_source_runtime,
        source_runtime_ref.as_artifact_ref(),
        "runtime artifact must persist under its exact derived identity"
    );
    let source_lowering = SourceAskLowering::new(
        first_occurrence.clone(),
        source_ports.clone(),
        source_runtime.clone(),
    )
    .expect("complete source ports must form a lowering");
    source_lowering
        .check_expected(&first_occurrence, source_runtime_ref, &catalog)
        .expect("exact source Ask lowering must recheck");
    assert!(matches!(
        source_lowering.check_expected(&second_occurrence, source_runtime_ref, &catalog),
        Err(SourceAskLoweringCheckError::ExpectedOccurrenceMismatch)
    ));
    let changed_runtime = RuntimeProgramArtifact::new(
        binding,
        roots.compiler_version,
        ProgramIR::new(
            roots.unit,
            BlockTarget::new(0),
            vec![BasicBlock::new(
                BlockTarget::new(0),
                Terminator::Return {
                    value: roots.answer_a,
                },
            )],
        ),
    );
    let changed_runtime_lowering = SourceAskLowering::new(
        first_occurrence.clone(),
        source_ports.clone(),
        changed_runtime,
    )
    .expect("changed runtime remains structurally formable");
    assert!(matches!(
        changed_runtime_lowering.check_expected(&first_occurrence, source_runtime_ref, &catalog),
        Err(SourceAskLoweringCheckError::ExpectedRuntimeMismatch { .. })
    ));
    let source_port = source_ports[0].port().clone();
    let foreign_lowering = SourceAskLowering::new(
        first_occurrence.clone(),
        vec![PortLowering::new(
            TypeSymbol::new("foreign").expect("foreign port name must be valid"),
            source_ports[0].mode(),
        )],
        source_runtime.clone(),
    )
    .expect("foreign port remains formable before checking");
    assert!(matches!(
        foreign_lowering.check(&catalog),
        Err(SourceAskLoweringCheckError::ForeignPort(_))
    ));
    let wrong_mode_lowering = SourceAskLowering::new(
        first_occurrence.clone(),
        vec![PortLowering::new(
            source_port.clone(),
            DischargeMode::Generate,
        )],
        source_runtime.clone(),
    )
    .expect("wrong mode remains formable before checking");
    assert!(matches!(
        wrong_mode_lowering.check(&catalog),
        Err(SourceAskLoweringCheckError::ModeMismatch { .. })
    ));
    let duplicate_lowering = SourceAskLowering::new(
        first_occurrence.clone(),
        vec![
            PortLowering::new(source_port.clone(), source_ports[0].mode()),
            PortLowering::new(source_port, source_ports[0].mode()),
        ],
        source_runtime.clone(),
    )
    .expect("duplicate ports remain formable before checking");
    assert!(matches!(
        duplicate_lowering.check(&catalog),
        Err(SourceAskLoweringCheckError::DuplicatePort(_))
    ));
    assert!(matches!(
        SourceAskLowering::new(first_occurrence.clone(), Vec::new(), source_runtime.clone()),
        Err(SourceAskLoweringCheckError::EmptyPortLowerings)
    ));
    let wrong_compiler_lowering = SourceAskLowering::new(
        first_occurrence.clone(),
        source_ports,
        RuntimeProgramArtifact::new(binding, artifact(0xfe), runtime.clone()),
    )
    .expect("wrong compiler remains formable before checking");
    assert!(matches!(
        wrong_compiler_lowering.check(&catalog),
        Err(SourceAskLoweringCheckError::CompilerMismatch { .. })
    ));
    let state_a = StateRef::from_artifact_ref(stored_ref(&store, b"occurrence-state-a").await);
    let state_b = StateRef::from_artifact_ref(stored_ref(&store, b"occurrence-state-b").await);
    let state_c = StateRef::from_artifact_ref(stored_ref(&store, b"occurrence-state-c").await);
    let route = RouteRef::from_artifact_ref(stored_ref(&store, b"occurrence-route").await);
    let grain = OpenQueryCatalog::resolve_open_query(&catalog, roots.query)
        .expect("query must reload")
        .context()
        .grain();
    let token_a = DispatchToken::from_bytes([0xd1; 32]);
    let token_b = DispatchToken::from_bytes([0xd2; 32]);
    let mut provider = CountingProvider {
        calls: Arc::clone(&provider_calls),
        expected_body: request.request_body(),
        response: original.raw_return().bytes().to_vec(),
    };
    let MachineStep::Suspended(first_suspension) = runtime
        .step(runtime.start())
        .expect("first runtime must suspend")
    else {
        panic!("first runtime must suspend")
    };
    let first_actual = dispatch_probe(
        &store,
        first_suspension,
        token_a,
        request_ref,
        ProbeDispatchContext::new(
            Some(roots.event),
            state_a,
            None,
            state_b,
            grain,
            route,
            binding,
            first_provenance,
        )
        .with_source_ask_occurrence(first_occurrence_ref),
        &mut provider,
    )
    .await
    .expect("first source-linked dispatch must complete");
    let MachineStep::Suspended(second_suspension) = runtime
        .step(runtime.start())
        .expect("second runtime must suspend")
    else {
        panic!("second runtime must suspend")
    };
    let second_actual = dispatch_probe(
        &store,
        second_suspension,
        token_b,
        request_ref,
        ProbeDispatchContext::new(
            Some(first_actual.event_ref()),
            state_b,
            None,
            state_c,
            grain,
            route,
            binding,
            second_provenance,
        )
        .with_source_ask_occurrence(second_occurrence_ref),
        &mut provider,
    )
    .await
    .expect("second source-linked dispatch must complete");
    assert_eq!(provider_calls.load(Ordering::SeqCst), 3);
    assert_eq!(
        first_actual.raw_return_ref(),
        second_actual.raw_return_ref()
    );
    assert_eq!(
        first_actual.event().question(),
        second_actual.event().question()
    );
    assert_eq!(
        first_actual.event().operator(),
        second_actual.event().operator()
    );
    assert_ne!(first_actual.event_ref(), second_actual.event_ref());

    let decoded_port = TypeSymbol::new("decoded").expect("port must be valid");
    let opaque_port = TypeSymbol::new("opaque").expect("port must be valid");
    let multi_relation_value = RelationSchema::new(
        binding,
        vec![
            RelationPort::new(decoded_port.clone(), roots.unit),
            RelationPort::new(opaque_port.clone(), roots.raw_type),
        ],
        RelationBodyIR::BindingNative {
            contract: stored_ref(&store, b"mixed-port-relation-contract").await,
        },
        Vec::new(),
        Vec::new(),
    );
    let multi_relation = RelationRef::from_artifact_ref(
        persist(
            &store,
            &multi_relation_value
                .envelope()
                .expect("mixed-port relation must encode"),
            &multi_relation_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_schema(multi_relation_value), multi_relation);
    let query_context = *OpenQueryCatalog::resolve_open_query(&catalog, roots.query)
        .expect("base query must reload")
        .context();
    let multi_query_value = OpenQuery::new(
        multi_relation,
        Vec::new(),
        vec![
            OpenPort::new(decoded_port.clone(), DischargeMode::Probe),
            OpenPort::new(opaque_port.clone(), DischargeMode::Probe),
        ],
        query_context,
    );
    let multi_query = QueryRef::from_artifact_ref(
        persist(
            &store,
            &multi_query_value
                .envelope()
                .expect("mixed-port query must encode"),
            &multi_query_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_query(multi_query_value), multi_query);
    let base_chart = catalog
        .resolve_boundary_chart(roots.boundary)
        .expect("base boundary must reload");
    let multi_chart_value = BoundaryChart::new(
        multi_query,
        base_chart.x_type(),
        base_chart.y_type(),
        base_chart.boundary_type(),
        base_chart.pi_x(),
        base_chart.pi_y(),
        base_chart.x_determination(),
        base_chart.y_determination(),
        base_chart.negation_frontier_x().to_vec(),
        base_chart.negation_frontier_y().to_vec(),
        base_chart.seed_y(),
        base_chart.compatibility(),
        base_chart.traversal(),
        base_chart.grain(),
        base_chart.horizon(),
    );
    let multi_boundary = BoundaryRef::from_artifact_ref(
        persist(
            &store,
            &multi_chart_value
                .envelope()
                .expect("mixed-port boundary must encode"),
            &multi_chart_value.referenced_artifacts(),
        )
        .await,
    );
    catalog
        .charts
        .insert(multi_boundary, multi_chart_value.clone());
    let base_operator = catalog
        .resolve_probe_operator(roots.operator)
        .expect("base operator must reload");
    let multi_operator_value = ProbeOperator::new(
        multi_query,
        multi_boundary,
        base_operator.active_view(),
        base_operator.backend(),
        base_operator.executable_code(),
        base_operator.return_type(),
        base_operator.decoder_contract(),
        base_operator.probe_contract(),
        base_operator.compiler_version(),
    );
    let multi_operator = ProbeOperatorRef::from_artifact_ref(
        persist(
            &store,
            &multi_operator_value
                .envelope()
                .expect("mixed-port operator must encode"),
            &multi_operator_value.referenced_artifacts(),
        )
        .await,
    );
    catalog
        .operators
        .insert(multi_operator, multi_operator_value.clone());
    let multi_plan_value = SurfacePlan::new(
        multi_operator,
        multi_query,
        multi_boundary,
        multi_operator_value.active_view(),
        multi_operator_value.executable_code(),
        multi_operator_value.probe_contract(),
        stored_ref(&store, b"mixed-port-renderer-version").await,
        stored_ref(&store, b"mixed-port-rendered-body").await,
    );
    let multi_plan = ic_core::SurfacePlanRef::from_artifact_ref(
        persist(
            &store,
            &multi_plan_value
                .envelope()
                .expect("mixed-port plan must encode"),
            &multi_plan_value.referenced_artifacts(),
        )
        .await,
    );
    let multi_request_body = stored_ref(&store, b"mixed-port-request-body").await;
    let multi_request_value = BackendRequest::new(
        multi_operator,
        multi_plan,
        multi_query,
        multi_boundary,
        multi_operator_value.backend(),
        multi_operator_value.executable_code(),
        multi_operator_value.compiler_version(),
        request.backend_version(),
        multi_request_body,
    );
    let multi_request = ic_core::BackendRequestRef::from_artifact_ref(
        persist(
            &store,
            &multi_request_value
                .envelope()
                .expect("mixed-port request must encode"),
            &multi_request_value.referenced_artifacts(),
        )
        .await,
    );
    let multi_source_program_value = IProgArtifact::new(
        roots.unit,
        IProgIR::Ask {
            question: multi_query,
            environment: Vec::new(),
            answer_slot: TypeSymbol::new("mixed_answer").expect("slot must be valid"),
            continuation: roots.continuation,
        },
    );
    let multi_source_program = IProgRef::from_artifact_ref(
        persist(
            &store,
            &multi_source_program_value
                .envelope()
                .expect("mixed-port source must encode"),
            &multi_source_program_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_program(multi_source_program_value),
        multi_source_program
    );
    let multi_provenance =
        ProvenanceRef::from_artifact_ref(stored_ref(&store, b"mixed-port-source-provenance").await);
    let multi_source_value = SourceConfig::new(
        roots.unit,
        multi_source_program,
        Vec::new(),
        binding,
        roots.compiler_version,
        multi_provenance,
    )
    .expect("mixed-port source configuration must encode");
    let multi_source_ref = SourceConfigRef::from_artifact_ref(
        persist(
            &store,
            &multi_source_value
                .envelope()
                .expect("mixed-port source configuration must encode"),
            &multi_source_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_source_config(multi_source_value.clone()),
        multi_source_ref
    );
    let multi_occurrence = multi_source_value
        .ask_occurrences(&catalog)
        .expect("mixed-port source must re-walk")
        .into_iter()
        .next()
        .expect("mixed-port source must contain an Ask");
    let multi_occurrence_ref = multi_occurrence
        .ask_occurrence_ref()
        .expect("mixed-port occurrence must encode");
    persist(
        &store,
        &multi_occurrence
            .envelope()
            .expect("mixed-port occurrence must encode"),
        &persist_occurrence(&multi_occurrence),
    )
    .await;
    let opaque_path_value =
        ResolutionPath::new(roots.raw_type, roots.raw_type, ResolutionPathIR::Identity);
    let opaque_path = ResolutionPathRef::from_artifact_ref(
        persist(
            &store,
            &opaque_path_value
                .envelope()
                .expect("opaque identity path must encode"),
            &opaque_path_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_path(opaque_path_value), opaque_path);
    let multi_runtime = ProgramIR::new(
        roots.unit,
        BlockTarget::new(0),
        vec![
            BasicBlock::new(
                BlockTarget::new(0),
                Terminator::Probe {
                    operator: multi_operator,
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
    multi_runtime
        .verify(&catalog)
        .expect("mixed-port runtime must verify");
    let MachineStep::Suspended(multi_suspension) = multi_runtime
        .step(multi_runtime.start())
        .expect("mixed-port runtime must suspend")
    else {
        panic!("mixed-port runtime must suspend")
    };
    let token_multi = DispatchToken::from_bytes([0xd3; 32]);
    let mut multi_provider = CountingProvider {
        calls: Arc::clone(&provider_calls),
        expected_body: multi_request_body,
        response: original.raw_return().bytes().to_vec(),
    };
    let multi_actual = dispatch_probe(
        &store,
        multi_suspension,
        token_multi,
        multi_request,
        ProbeDispatchContext::new(
            Some(second_actual.event_ref()),
            state_c,
            None,
            StateRef::from_artifact_ref(stored_ref(&store, b"occurrence-state-d").await),
            grain,
            route,
            binding,
            multi_provenance,
        )
        .with_source_ask_occurrence(multi_occurrence_ref),
        &mut multi_provider,
    )
    .await
    .expect("one event may actualize an explicitly checked mixed-port lowering");
    assert_eq!(provider_calls.load(Ordering::SeqCst), 4);

    // One source Ask whose open ports do not share a discharge mode. Only the Probe port may
    // reach the event spine; the Pure port keeps its own typed result and authority route.
    let mixed_probe_port = TypeSymbol::new("probed").expect("port must be valid");
    let mixed_pure_port = TypeSymbol::new("derived").expect("port must be valid");
    let mixed_relation_value = RelationSchema::new(
        binding,
        vec![
            RelationPort::new(mixed_probe_port.clone(), roots.unit),
            RelationPort::new(mixed_pure_port.clone(), roots.raw_type),
        ],
        RelationBodyIR::BindingNative {
            contract: stored_ref(&store, b"mixed-mode-relation-contract").await,
        },
        Vec::new(),
        Vec::new(),
    );
    let mixed_relation = RelationRef::from_artifact_ref(
        persist(
            &store,
            &mixed_relation_value
                .envelope()
                .expect("mixed-mode relation must encode"),
            &mixed_relation_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_schema(mixed_relation_value), mixed_relation);
    let mixed_query_value = OpenQuery::new(
        mixed_relation,
        Vec::new(),
        // The non-Probe port is declared first so that no port-indexed check can pass by taking
        // the first open port instead of the one it names.
        vec![
            OpenPort::new(mixed_pure_port.clone(), DischargeMode::Pure),
            OpenPort::new(mixed_probe_port.clone(), DischargeMode::Probe),
        ],
        query_context,
    );
    let mixed_query = QueryRef::from_artifact_ref(
        persist(
            &store,
            &mixed_query_value
                .envelope()
                .expect("mixed-mode query must encode"),
            &mixed_query_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_query(mixed_query_value), mixed_query);
    let mixed_chart_value = BoundaryChart::new(
        mixed_query,
        base_chart.x_type(),
        base_chart.y_type(),
        base_chart.boundary_type(),
        base_chart.pi_x(),
        base_chart.pi_y(),
        base_chart.x_determination(),
        base_chart.y_determination(),
        base_chart.negation_frontier_x().to_vec(),
        base_chart.negation_frontier_y().to_vec(),
        base_chart.seed_y(),
        base_chart.compatibility(),
        base_chart.traversal(),
        base_chart.grain(),
        base_chart.horizon(),
    );
    let mixed_boundary = BoundaryRef::from_artifact_ref(
        persist(
            &store,
            &mixed_chart_value
                .envelope()
                .expect("mixed-mode boundary must encode"),
            &mixed_chart_value.referenced_artifacts(),
        )
        .await,
    );
    catalog
        .charts
        .insert(mixed_boundary, mixed_chart_value.clone());
    let mixed_operator_value = ProbeOperator::new(
        mixed_query,
        mixed_boundary,
        base_operator.active_view(),
        base_operator.backend(),
        base_operator.executable_code(),
        base_operator.return_type(),
        base_operator.decoder_contract(),
        base_operator.probe_contract(),
        base_operator.compiler_version(),
    );
    let mixed_operator = ProbeOperatorRef::from_artifact_ref(
        persist(
            &store,
            &mixed_operator_value
                .envelope()
                .expect("mixed-mode operator must encode"),
            &mixed_operator_value.referenced_artifacts(),
        )
        .await,
    );
    catalog
        .operators
        .insert(mixed_operator, mixed_operator_value.clone());
    let mixed_plan_value = SurfacePlan::new(
        mixed_operator,
        mixed_query,
        mixed_boundary,
        mixed_operator_value.active_view(),
        mixed_operator_value.executable_code(),
        mixed_operator_value.probe_contract(),
        stored_ref(&store, b"mixed-mode-renderer-version").await,
        stored_ref(&store, b"mixed-mode-rendered-body").await,
    );
    let mixed_plan = ic_core::SurfacePlanRef::from_artifact_ref(
        persist(
            &store,
            &mixed_plan_value
                .envelope()
                .expect("mixed-mode plan must encode"),
            &mixed_plan_value.referenced_artifacts(),
        )
        .await,
    );
    let mixed_request_body = stored_ref(&store, b"mixed-mode-request-body").await;
    let mixed_request_value = BackendRequest::new(
        mixed_operator,
        mixed_plan,
        mixed_query,
        mixed_boundary,
        mixed_operator_value.backend(),
        mixed_operator_value.executable_code(),
        mixed_operator_value.compiler_version(),
        request.backend_version(),
        mixed_request_body,
    );
    let mixed_request = ic_core::BackendRequestRef::from_artifact_ref(
        persist(
            &store,
            &mixed_request_value
                .envelope()
                .expect("mixed-mode request must encode"),
            &mixed_request_value.referenced_artifacts(),
        )
        .await,
    );
    let mixed_source_program_value = IProgArtifact::new(
        roots.unit,
        IProgIR::Ask {
            question: mixed_query,
            environment: Vec::new(),
            answer_slot: TypeSymbol::new("mixed_mode_answer").expect("slot must be valid"),
            continuation: roots.continuation,
        },
    );
    let mixed_source_program = IProgRef::from_artifact_ref(
        persist(
            &store,
            &mixed_source_program_value
                .envelope()
                .expect("mixed-mode source must encode"),
            &mixed_source_program_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_program(mixed_source_program_value),
        mixed_source_program
    );
    let mixed_provenance =
        ProvenanceRef::from_artifact_ref(stored_ref(&store, b"mixed-mode-source-provenance").await);
    let mixed_source_value = SourceConfig::new(
        roots.unit,
        mixed_source_program,
        Vec::new(),
        binding,
        roots.compiler_version,
        mixed_provenance,
    )
    .expect("mixed-mode source configuration must encode");
    let mixed_source_ref = SourceConfigRef::from_artifact_ref(
        persist(
            &store,
            &mixed_source_value
                .envelope()
                .expect("mixed-mode source configuration must encode"),
            &mixed_source_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(
        catalog.insert_source_config(mixed_source_value.clone()),
        mixed_source_ref
    );
    let mixed_occurrence = mixed_source_value
        .ask_occurrences(&catalog)
        .expect("mixed-mode source must re-walk")
        .into_iter()
        .next()
        .expect("mixed-mode source must contain an Ask");
    let mixed_occurrence_ref = mixed_occurrence
        .ask_occurrence_ref()
        .expect("mixed-mode occurrence must encode");
    persist(
        &store,
        &mixed_occurrence
            .envelope()
            .expect("mixed-mode occurrence must encode"),
        &persist_occurrence(&mixed_occurrence),
    )
    .await;
    let pure_path_value = ResolutionPath::new(roots.unit, roots.unit, ResolutionPathIR::Identity);
    let pure_path = ResolutionPathRef::from_artifact_ref(
        persist(
            &store,
            &pure_path_value
                .envelope()
                .expect("pure identity path must encode"),
            &pure_path_value.referenced_artifacts(),
        )
        .await,
    );
    assert_eq!(catalog.insert_path(pure_path_value), pure_path);
    let mixed_runtime = ProgramIR::new(
        roots.unit,
        BlockTarget::new(0),
        vec![
            BasicBlock::new(
                BlockTarget::new(0),
                Terminator::Probe {
                    operator: mixed_operator,
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
    mixed_runtime
        .verify(&catalog)
        .expect("mixed-mode runtime must verify");
    let mixed_runtime_artifact =
        RuntimeProgramArtifact::new(binding, roots.compiler_version, mixed_runtime.clone());
    let mixed_runtime_ref = mixed_runtime_artifact
        .runtime_program_ref()
        .expect("mixed-mode runtime identity must encode");
    persist(
        &store,
        &mixed_runtime_artifact
            .envelope()
            .expect("mixed-mode runtime artifact must encode"),
        &mixed_runtime_artifact.referenced_artifacts(),
    )
    .await;
    let MachineStep::Suspended(mixed_suspension) = mixed_runtime
        .step(mixed_runtime.start())
        .expect("mixed-mode runtime must suspend")
    else {
        panic!("mixed-mode runtime must suspend")
    };
    let state_d = StateRef::from_artifact_ref(stored_ref(&store, b"occurrence-state-d").await);
    let state_e = StateRef::from_artifact_ref(stored_ref(&store, b"occurrence-state-e").await);
    let token_mixed = DispatchToken::from_bytes([0xd4; 32]);
    let mut mixed_provider = CountingProvider {
        calls: Arc::clone(&provider_calls),
        expected_body: mixed_request_body,
        response: original.raw_return().bytes().to_vec(),
    };
    let mixed_actual = dispatch_probe(
        &store,
        mixed_suspension,
        token_mixed,
        mixed_request,
        ProbeDispatchContext::new(
            Some(multi_actual.event_ref()),
            state_d,
            None,
            state_e,
            grain,
            route,
            binding,
            mixed_provenance,
        )
        .with_source_ask_occurrence(mixed_occurrence_ref),
        &mut mixed_provider,
    )
    .await
    .expect("only the Probe port of a mixed-mode source Ask may actualize an event");
    assert_eq!(provider_calls.load(Ordering::SeqCst), 5);
    store.close().await;

    let reopened = ArtifactStore::open(&url)
        .await
        .expect("cold occurrence store must reopen");
    reopened.migrate().await.expect("migrations must repeat");
    let mut cold_catalog = load_cold_replay_catalog(&reopened, roots).await;
    let second_continuation_value = IProgArtifact::from_envelope(
        &load_envelope(&reopened, second_continuation.as_artifact_ref()).await,
    )
    .expect("second continuation must cold decode");
    assert_eq!(
        cold_catalog.insert_program(second_continuation_value),
        second_continuation
    );
    let second_source_program = IProgArtifact::from_envelope(
        &load_envelope(&reopened, second_source.as_artifact_ref()).await,
    )
    .expect("second source must cold decode");
    assert_eq!(
        cold_catalog.insert_program(second_source_program),
        second_source
    );
    let cold_first_source = SourceConfig::from_envelope(
        &load_envelope(&reopened, first_source_ref.as_artifact_ref()).await,
    )
    .expect("first source configuration must cold decode");
    let cold_second_source = SourceConfig::from_envelope(
        &load_envelope(&reopened, second_source_ref.as_artifact_ref()).await,
    )
    .expect("second source configuration must cold decode");
    assert_eq!(
        cold_catalog.insert_source_config(cold_first_source),
        first_source_ref
    );
    assert_eq!(
        cold_catalog.insert_source_config(cold_second_source),
        second_source_ref
    );
    let cold_source_runtime = RuntimeProgramArtifact::from_envelope(
        &load_envelope(&reopened, source_runtime_ref.as_artifact_ref()).await,
    )
    .expect("source runtime artifact must cold decode");
    assert_eq!(
        cold_source_runtime
            .runtime_program_ref()
            .expect("cold runtime identity must encode"),
        source_runtime_ref
    );
    cold_source_runtime
        .check(&cold_catalog)
        .expect("cold runtime artifact must independently recheck");
    let cold_first_occurrence = AskOccurrence::from_envelope(
        &load_envelope(&reopened, first_occurrence_ref.as_artifact_ref()).await,
    )
    .expect("first occurrence must cold decode");
    let cold_second_occurrence = AskOccurrence::from_envelope(
        &load_envelope(&reopened, second_occurrence_ref.as_artifact_ref()).await,
    )
    .expect("second occurrence must cold decode");
    let cold_port_lowerings =
        OpenQueryCatalog::resolve_open_query(&cold_catalog, cold_first_occurrence.question())
            .expect("cold source query must reload")
            .open_ports()
            .iter()
            .map(|open| PortLowering::new(open.port().clone(), open.mode()))
            .collect::<Vec<_>>();
    let provider_calls_before_cold_lowering = provider_calls.load(Ordering::SeqCst);
    let cold_source_lowering = SourceAskLowering::new(
        cold_first_occurrence.clone(),
        cold_port_lowerings.clone(),
        cold_source_runtime,
    )
    .expect("cold source lowering must form from ordinary roots");
    cold_source_lowering
        .check_expected(&cold_first_occurrence, source_runtime_ref, &cold_catalog)
        .expect("cold source lowering must exactly recheck");
    assert!(matches!(
        cold_source_lowering.check_expected(
            &cold_second_occurrence,
            source_runtime_ref,
            &cold_catalog,
        ),
        Err(SourceAskLoweringCheckError::ExpectedOccurrenceMismatch)
    ));
    let changed_cold_runtime = RuntimeProgramArtifact::new(
        binding,
        roots.compiler_version,
        ProgramIR::new(
            roots.unit,
            BlockTarget::new(0),
            vec![BasicBlock::new(
                BlockTarget::new(0),
                Terminator::Return {
                    value: roots.answer_a,
                },
            )],
        ),
    );
    let changed_cold_lowering = SourceAskLowering::new(
        cold_first_occurrence.clone(),
        cold_port_lowerings,
        changed_cold_runtime,
    )
    .expect("changed cold runtime remains formable before expected comparison");
    assert!(matches!(
        changed_cold_lowering.check_expected(
            &cold_first_occurrence,
            source_runtime_ref,
            &cold_catalog,
        ),
        Err(SourceAskLoweringCheckError::ExpectedRuntimeMismatch { .. })
    ));
    assert_eq!(
        provider_calls.load(Ordering::SeqCst),
        provider_calls_before_cold_lowering,
        "cold lowering regeneration must not dispatch a provider"
    );
    let replay_a = reopened
        .replay_completed_external_effect(token_a)
        .await
        .expect("first effect must cold replay");
    let replay_b = reopened
        .replay_completed_external_effect(token_b)
        .await
        .expect("second effect must cold replay");
    cold_catalog
        .events
        .insert(replay_a.event_ref(), replay_a.event().clone());
    cold_catalog
        .events
        .insert(replay_b.event_ref(), replay_b.event().clone());
    let link_a = check_source_event_link(
        replay_a.clone(),
        cold_first_occurrence.clone(),
        &cold_catalog,
    )
    .expect("first event must recheck against its source occurrence");
    let link_b = check_source_event_link(
        replay_b.clone(),
        cold_second_occurrence.clone(),
        &cold_catalog,
    )
    .expect("second event must recheck against its source occurrence");
    assert_eq!(
        link_a.actuality().request_ref(),
        link_b.actuality().request_ref()
    );
    assert_eq!(
        link_a.actuality().raw_return_ref(),
        link_b.actuality().raw_return_ref()
    );
    assert_eq!(
        link_a.actuality().event().question(),
        link_b.actuality().event().question()
    );
    assert_eq!(
        link_a.actuality().event().operator(),
        link_b.actuality().event().operator()
    );
    assert_ne!(link_a.occurrence_ref(), link_b.occurrence_ref());
    assert_ne!(
        link_a.occurrence().continuation(),
        link_b.occurrence().continuation()
    );
    let cold_standing = standing_from_declared_support(
        Vec::new(),
        &[DeclaredSupportClosure::for_subjects(
            roots.support,
            Vec::new(),
            true,
            true,
            false,
        )],
        &cold_catalog,
    )
    .expect("cold occurrence support must reconstruct");
    let cold_decoder = cold_catalog
        .resolve_finite_decoder(roots.decoded_decoder)
        .expect("cold decoder must reload");
    let cold_first_program = cold_catalog
        .resolve_iprog(roots.source)
        .expect("first source program must reload");
    let cold_second_program = cold_catalog
        .resolve_iprog(second_source)
        .expect("second source program must reload");
    let first_runtime = ProgramIR::new(
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
        ],
    );
    let second_runtime = ProgramIR::new(
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
                    value: roots.answer_b,
                },
            ),
        ],
    );
    first_runtime
        .verify(&cold_catalog)
        .expect("first cold lowering must verify");
    second_runtime
        .verify(&cold_catalog)
        .expect("second cold lowering must verify");
    let MachineStep::Suspended(first_suspension) = first_runtime
        .step(first_runtime.start())
        .expect("first cold lowering must suspend")
    else {
        panic!("first cold lowering must suspend")
    };
    let MachineStep::Suspended(second_suspension) = second_runtime
        .step(second_runtime.start())
        .expect("second cold lowering must suspend")
    else {
        panic!("second cold lowering must suspend")
    };
    let observations = [
        ReplayObservation::new(roots.candidate_a, roots.observation_a),
        ReplayObservation::new(roots.candidate_b, roots.observation_b),
    ];
    let resumed_a = replay_completed_finite_probe(
        &reopened,
        token_a,
        &cold_decoder,
        roots.decoded_path,
        &observations,
        &cold_standing,
        &cold_first_program,
        first_suspension,
        ContinuationLowering::new(roots.continuation, BlockTarget::new(1)),
        &first_runtime,
        &cold_catalog,
    )
    .await
    .expect("first occurrence-specific successor must cold replay");
    let resumed_b = replay_completed_finite_probe(
        &reopened,
        token_b,
        &cold_decoder,
        roots.decoded_path,
        &observations,
        &cold_standing,
        &cold_second_program,
        second_suspension,
        ContinuationLowering::new(second_continuation, BlockTarget::new(1)),
        &second_runtime,
        &cold_catalog,
    )
    .await
    .expect("second occurrence-specific successor must cold replay");
    assert_eq!(
        resumed_a.resumption().binding().answer().candidates(),
        resumed_b.resumption().binding().answer().candidates()
    );
    assert_ne!(
        resumed_a.resumption().binding().continuation(),
        resumed_b.resumption().binding().continuation()
    );
    assert!(matches!(
        first_runtime
            .step(resumed_a.resumption().state())
            .expect("first successor must execute"),
        MachineStep::Returned(value) if value == roots.answer_a
    ));
    assert!(matches!(
        second_runtime
            .step(resumed_b.resumption().state())
            .expect("second successor must execute"),
        MachineStep::Returned(value) if value == roots.answer_b
    ));

    // Test boundary QRESOLUTION-GATE-001:
    // F = a finite relational Compose selects one output, aliases one of five resolution
    // outcomes, or lets a non-Supported result/equal question enter a source continuation.
    // C = exact finite relational set composition, question/event/path-indexed payloads, whole
    // standing support admission, and exact bundle/Ask/continuation reconstruction.
    // Omega/M = this one two-candidate Probe question, one source-linked event, exact and partial
    // finite leaf tables, and one direct-decode-plus-relation Compose path.
    // P/V/E/U = all five constructors and two compatible outputs; general search completeness,
    // intensional relations, mixed-mode bundles, and cross-binding resolution remain open.
    let source_query = OpenQueryCatalog::resolve_open_query(&cold_catalog, roots.query)
        .expect("source query must remain available for resolution");
    let answer_port = source_query
        .open_ports()
        .first()
        .expect("finite resolution fixture has one answer port")
        .port()
        .clone();
    let bundle_a = admit_finite_probe_discharge_bundle(
        cold_first_occurrence.clone(),
        vec![ProbePortDischargeEvidence::new(
            answer_port,
            route,
            roots.decoded_path,
            binding,
            roots.compiler_version,
            first_provenance,
            link_a.clone(),
        )],
        Vec::new(),
        &cold_catalog,
    )
    .expect("the exact one-port source event must form its discharge bundle");
    let source_probe_discharge =
        SourceAskProbeDischarge::new(cold_source_lowering.clone(), bundle_a.clone());
    source_probe_discharge
        .check(&cold_catalog)
        .expect("exact source lowering and bundle must recheck together");
    let bundle_b = admit_finite_probe_discharge_bundle(
        cold_second_occurrence.clone(),
        vec![ProbePortDischargeEvidence::new(
            source_query
                .open_ports()
                .first()
                .expect("finite resolution fixture has one answer port")
                .port()
                .clone(),
            route,
            roots.decoded_path,
            binding,
            roots.compiler_version,
            second_provenance,
            link_b.clone(),
        )],
        Vec::new(),
        &cold_catalog,
    )
    .expect("second source event must form its own exact discharge bundle");
    assert!(matches!(
        SourceAskProbeDischarge::new(cold_source_lowering.clone(), bundle_b).check(&cold_catalog),
        Err(SourceAskProbeDischargeError::OccurrenceMismatch)
    ));
    let non_probe_lowering = SourceAskLowering::new(
        cold_first_occurrence.clone(),
        vec![PortLowering::new(
            source_query
                .open_ports()
                .first()
                .expect("finite resolution fixture has one answer port")
                .port()
                .clone(),
            DischargeMode::Generate,
        )],
        source_probe_discharge.lowering().runtime().clone(),
    )
    .expect("non-Probe lowering remains formable before source recheck");
    assert!(matches!(
        SourceAskProbeDischarge::new(non_probe_lowering, bundle_a.clone()).check(&cold_catalog),
        Err(SourceAskProbeDischargeError::Lowering(
            SourceAskLoweringCheckError::ModeMismatch { .. }
        ))
    ));

    let mut candidate_values = vec![
        roots.candidate_a.as_artifact_ref(),
        roots.candidate_b.as_artifact_ref(),
    ];
    candidate_values.sort_unstable();
    let raw_value = roots.raw_return.as_artifact_ref();
    let relation_path_value = ResolutionPath::new(
        roots.unit,
        roots.unit,
        ResolutionPathIR::Relation {
            relation: roots.relation,
        },
    );
    let relation_path = cold_catalog.insert_path(relation_path_value);
    let composed_path_value = ResolutionPath::new(
        roots.raw_type,
        roots.unit,
        ResolutionPathIR::Compose {
            first: roots.decoded_path,
            second: relation_path,
        },
    );
    let composed_path = cold_catalog.insert_path(composed_path_value);
    let decoder_table = FiniteResolutionLeafTable::new(
        roots.decoded_path,
        vec![raw_value],
        vec![FiniteResolutionLeafEntry::related(
            raw_value,
            candidate_values.clone(),
        )],
        FiniteResolutionCoverage::Exact(CoverageRef::from_artifact_ref(
            stored_ref(&reopened, b"resolution-decoder-coverage").await,
        )),
    )
    .expect("direct decoder relation must have exact finite coverage");
    let relation_table = FiniteResolutionLeafTable::new(
        relation_path,
        candidate_values.clone(),
        candidate_values
            .iter()
            .copied()
            .map(|candidate| FiniteResolutionLeafEntry::related(candidate, vec![candidate]))
            .collect(),
        FiniteResolutionCoverage::Exact(CoverageRef::from_artifact_ref(
            stored_ref(&reopened, b"resolution-relation-coverage").await,
        )),
    )
    .expect("second relation must cover both decoder outputs");
    let complete_run = run_finite_resolution(
        composed_path,
        raw_value,
        &[decoder_table, relation_table],
        &cold_catalog,
    )
    .expect("finite relational Compose must run");
    let FiniteResolutionRun::Complete(complete) = &complete_run else {
        panic!("both exact leaves must yield a complete relational run")
    };
    assert_eq!(
        complete.outputs(),
        candidate_values,
        "Compose must retain both compatible outputs rather than select the first"
    );

    let ActualDecodeResult::Decoded(decoded) = decode_actual_event(
        replay_a.event(),
        &cold_decoder,
        roots.decoded_path,
        &cold_catalog,
    )
    .expect("source-linked event must decode through its direct path") else {
        panic!("fixture event must retain its two decoded candidates")
    };
    let decoded_observations = vec![
        match_decoded_observation_use(
            &decoded,
            roots.candidate_a,
            roots.observation_a,
            &cold_catalog,
        )
        .expect("first decoded candidate must retain its observation"),
        match_decoded_observation_use(
            &decoded,
            roots.candidate_b,
            roots.observation_b,
            &cold_catalog,
        )
        .expect("second decoded candidate must retain its observation"),
    ];
    let supported = classify_finite_question_resolution(
        replay_a.event_ref(),
        roots.query,
        complete_run.clone(),
        Some(decoded.clone()),
        decoded_observations.clone(),
        &cold_standing,
        &cold_catalog,
    )
    .expect("the exact composed output field must classify through standing support");
    assert_eq!(supported.kind(), FiniteResolutionOutcomeKind::Supported);

    let empty_standing = standing_from_declared_support(Vec::new(), &[], &cold_catalog)
        .expect("an empty standing field must remain constructible");
    let unsupported = classify_finite_question_resolution(
        replay_a.event_ref(),
        roots.query,
        complete_run.clone(),
        Some(decoded.clone()),
        decoded_observations.clone(),
        &empty_standing,
        &cold_catalog,
    )
    .expect("decoded candidates with a failed support route form Unsupported");
    assert!(matches!(
        &unsupported,
        FiniteResolutionOutcome::Unsupported(residual)
            if residual.decoded().candidates() == candidate_values
                .iter()
                .copied()
                .map(CompletionCandidateRef::from_artifact_ref)
                .collect::<Vec<_>>()
    ));

    let terminal_path_value = ResolutionPath::new(
        roots.raw_type,
        roots.unit,
        ResolutionPathIR::Relation {
            relation: roots.relation,
        },
    );
    let terminal_path = cold_catalog.insert_path(terminal_path_value);
    let terminal_domain = vec![raw_value];
    let exact_empty_run = run_finite_resolution(
        terminal_path,
        raw_value,
        &[FiniteResolutionLeafTable::new(
            terminal_path,
            terminal_domain.clone(),
            Vec::new(),
            FiniteResolutionCoverage::Exact(CoverageRef::from_artifact_ref(
                stored_ref(&reopened, b"resolution-empty-coverage").await,
            )),
        )
        .expect("exact empty table must be well formed")],
        &cold_catalog,
    )
    .expect("exact empty relation must run");
    let exact_empty = classify_finite_question_resolution(
        replay_a.event_ref(),
        roots.query,
        exact_empty_run,
        None,
        Vec::new(),
        &cold_standing,
        &cold_catalog,
    )
    .expect("exhaustive empty relation must classify as ExactEmpty");
    assert_eq!(exact_empty.kind(), FiniteResolutionOutcomeKind::ExactEmpty);

    let undefined_residual = stored_ref(&reopened, b"resolution-undefined-residual").await;
    let undefined_run = run_finite_resolution(
        terminal_path,
        raw_value,
        &[FiniteResolutionLeafTable::new(
            terminal_path,
            terminal_domain.clone(),
            vec![FiniteResolutionLeafEntry::undefined(
                raw_value,
                undefined_residual,
            )],
            FiniteResolutionCoverage::Exact(CoverageRef::from_artifact_ref(
                stored_ref(&reopened, b"resolution-undefined-coverage").await,
            )),
        )
        .expect("typed undefined table must be well formed")],
        &cold_catalog,
    )
    .expect("typed undefined relation must run");
    let undefined = classify_finite_question_resolution(
        replay_a.event_ref(),
        roots.query,
        undefined_run,
        None,
        Vec::new(),
        &cold_standing,
        &cold_catalog,
    )
    .expect("typed undefined relation must retain its residual");
    assert!(matches!(
        &undefined,
        FiniteResolutionOutcome::Undefined(residual)
            if residual.run().residuals() == [undefined_residual]
    ));

    let unknown_run = run_finite_resolution(
        terminal_path,
        raw_value,
        &[FiniteResolutionLeafTable::new(
            terminal_path,
            terminal_domain,
            vec![FiniteResolutionLeafEntry::related(
                raw_value,
                vec![roots.candidate_a.as_artifact_ref()],
            )],
            FiniteResolutionCoverage::Partial(CoverageRef::from_artifact_ref(
                stored_ref(&reopened, b"resolution-partial-coverage").await,
            )),
        )
        .expect("partial relation table must be well formed")],
        &cold_catalog,
    )
    .expect("partial relation must run without claiming closure");
    let unknown = classify_finite_question_resolution(
        replay_a.event_ref(),
        roots.query,
        unknown_run,
        None,
        Vec::new(),
        &cold_standing,
        &cold_catalog,
    )
    .expect("partial coverage must classify as Unknown");
    assert!(matches!(
        &unknown,
        FiniteResolutionOutcome::Unknown(residual)
            if residual.run().known_outputs() == [roots.candidate_a.as_artifact_ref()]
                && residual.run().uncovered_inputs() == [raw_value]
    ));

    for outcome in [exact_empty, undefined, unsupported, unknown] {
        assert!(matches!(
            resolve_finite_probe_occurrence(
                bundle_a.clone(),
                outcome,
                &cold_first_program,
                &cold_catalog,
            ),
            Err(ResolvedFiniteProbeOccurrenceError::NonSupported(_))
        ));
    }
    let resolved = resolve_finite_probe_occurrence(
        bundle_a.clone(),
        supported,
        &cold_first_program,
        &cold_catalog,
    )
    .expect("only Supported may enter the exact source continuation");
    assert_eq!(resolved.ask_occurrence(), first_occurrence_ref);
    assert_eq!(resolved.resolution().run().outputs(), candidate_values);
    assert!(matches!(
        resolved.next(),
        ic_core::QuestionSuccessor::Return { value, .. } if *value == roots.answer_a
    ));

    let second_supported = classify_finite_question_resolution(
        replay_a.event_ref(),
        roots.query,
        complete_run,
        Some(decoded),
        decoded_observations,
        &cold_standing,
        &cold_catalog,
    )
    .expect("supported result must remain reproducible from the same checked evidence");
    assert!(matches!(
        resolve_finite_probe_occurrence(
            bundle_a,
            second_supported,
            &cold_second_program,
            &cold_catalog,
        ),
        Err(ResolvedFiniteProbeOccurrenceError::SourceProgramMismatch { .. })
    ));

    assert!(matches!(
        check_source_event_link(replay_a, cold_second_occurrence, &cold_catalog),
        Err(SourceEventLinkError::OccurrenceMismatch { .. })
    ));
    assert!(matches!(
        check_source_event_link(original, cold_first_occurrence, &cold_catalog),
        Err(SourceEventLinkError::LegacyOrDirectEvent(event)) if event == roots.event
    ));

    let cold_multi_relation = RelationSchema::from_envelope(
        &load_envelope(&reopened, multi_relation.as_artifact_ref()).await,
    )
    .expect("mixed-port relation must cold decode");
    assert_eq!(
        cold_catalog.insert_schema(cold_multi_relation),
        multi_relation
    );
    let cold_multi_query =
        OpenQuery::from_envelope(&load_envelope(&reopened, multi_query.as_artifact_ref()).await)
            .expect("mixed-port query must cold decode");
    assert_eq!(cold_catalog.insert_query(cold_multi_query), multi_query);
    let cold_multi_chart = BoundaryChart::from_envelope(
        &load_envelope(&reopened, multi_boundary.as_artifact_ref()).await,
    )
    .expect("mixed-port boundary must cold decode");
    cold_catalog.charts.insert(multi_boundary, cold_multi_chart);
    let cold_multi_operator = ProbeOperator::from_envelope(
        &load_envelope(&reopened, multi_operator.as_artifact_ref()).await,
    )
    .expect("mixed-port operator must cold decode");
    cold_catalog
        .operators
        .insert(multi_operator, cold_multi_operator);
    let cold_multi_source_program = IProgArtifact::from_envelope(
        &load_envelope(&reopened, multi_source_program.as_artifact_ref()).await,
    )
    .expect("mixed-port source must cold decode");
    assert_eq!(
        cold_catalog.insert_program(cold_multi_source_program),
        multi_source_program
    );
    let cold_multi_source = SourceConfig::from_envelope(
        &load_envelope(&reopened, multi_source_ref.as_artifact_ref()).await,
    )
    .expect("mixed-port source configuration must cold decode");
    assert_eq!(
        cold_catalog.insert_source_config(cold_multi_source),
        multi_source_ref
    );
    let cold_multi_occurrence = AskOccurrence::from_envelope(
        &load_envelope(&reopened, multi_occurrence_ref.as_artifact_ref()).await,
    )
    .expect("mixed-port occurrence must cold decode");
    let cold_opaque_path = ResolutionPath::from_envelope(
        &load_envelope(&reopened, opaque_path.as_artifact_ref()).await,
    )
    .expect("opaque path must cold decode");
    assert_eq!(cold_catalog.insert_path(cold_opaque_path), opaque_path);
    let replay_multi = reopened
        .replay_completed_external_effect(token_multi)
        .await
        .expect("mixed-port effect must cold replay");
    assert_eq!(replay_multi.event_ref(), multi_actual.event_ref());
    let multi_link =
        check_source_event_link(replay_multi, cold_multi_occurrence.clone(), &cold_catalog)
            .expect("mixed-port event must recheck against its source occurrence");
    let decoded_evidence = ProbePortDischargeEvidence::new(
        decoded_port.clone(),
        route,
        roots.decoded_path,
        binding,
        roots.compiler_version,
        multi_provenance,
        multi_link.clone(),
    );
    let opaque_evidence = ProbePortDischargeEvidence::new(
        opaque_port.clone(),
        route,
        opaque_path,
        binding,
        roots.compiler_version,
        multi_provenance,
        multi_link,
    );
    assert!(matches!(
        admit_finite_probe_discharge_bundle(
            cold_multi_occurrence.clone(),
            vec![decoded_evidence.clone(), opaque_evidence.clone()],
            Vec::new(),
            &cold_catalog,
        ),
        Err(ProbeDischargeBundleError::SharedEventCoverageMismatch)
    ));
    let shared = SharedProbeEventAdmission::new(
        multi_actual.event_ref(),
        vec![decoded_port.clone(), opaque_port.clone()],
    )
    .expect("one event may be admitted for two explicitly named ports");
    let bundle = admit_finite_probe_discharge_bundle(
        cold_multi_occurrence.clone(),
        vec![opaque_evidence.clone(), decoded_evidence.clone()],
        vec![shared],
        &cold_catalog,
    )
    .expect("cold mixed-port evidence must retain exact port-indexed paths");
    assert_eq!(bundle.components().len(), 2);
    assert_eq!(bundle.shared_events().len(), 1);
    assert_ne!(
        bundle.components()[0].resolution_path(),
        bundle.components()[1].resolution_path()
    );
    let wrong_decoded = ProbePortDischargeEvidence::new(
        decoded_port,
        route,
        opaque_path,
        binding,
        roots.compiler_version,
        multi_provenance,
        opaque_evidence.event().clone(),
    );
    let shared = SharedProbeEventAdmission::new(
        multi_actual.event_ref(),
        vec![
            TypeSymbol::new("decoded").expect("port must be valid"),
            opaque_port,
        ],
    )
    .expect("shared admission must remain constructible for the path foil");
    assert!(matches!(
        admit_finite_probe_discharge_bundle(
            cold_multi_occurrence,
            vec![wrong_decoded, opaque_evidence],
            vec![shared],
            &cold_catalog,
        ),
        Err(ProbeDischargeBundleError::ResolutionTypeMismatch(_))
    ));
    let cold_mixed_relation = RelationSchema::from_envelope(
        &load_envelope(&reopened, mixed_relation.as_artifact_ref()).await,
    )
    .expect("mixed-mode relation must cold decode");
    assert_eq!(
        cold_catalog.insert_schema(cold_mixed_relation),
        mixed_relation
    );
    let cold_mixed_query =
        OpenQuery::from_envelope(&load_envelope(&reopened, mixed_query.as_artifact_ref()).await)
            .expect("mixed-mode query must cold decode");
    assert_eq!(cold_catalog.insert_query(cold_mixed_query), mixed_query);
    let cold_mixed_chart = BoundaryChart::from_envelope(
        &load_envelope(&reopened, mixed_boundary.as_artifact_ref()).await,
    )
    .expect("mixed-mode boundary must cold decode");
    cold_catalog.charts.insert(mixed_boundary, cold_mixed_chart);
    let cold_mixed_operator = ProbeOperator::from_envelope(
        &load_envelope(&reopened, mixed_operator.as_artifact_ref()).await,
    )
    .expect("mixed-mode operator must cold decode");
    cold_catalog
        .operators
        .insert(mixed_operator, cold_mixed_operator);
    let cold_mixed_source_program = IProgArtifact::from_envelope(
        &load_envelope(&reopened, mixed_source_program.as_artifact_ref()).await,
    )
    .expect("mixed-mode source must cold decode");
    assert_eq!(
        cold_catalog.insert_program(cold_mixed_source_program),
        mixed_source_program
    );
    let cold_mixed_source = SourceConfig::from_envelope(
        &load_envelope(&reopened, mixed_source_ref.as_artifact_ref()).await,
    )
    .expect("mixed-mode source configuration must cold decode");
    assert_eq!(
        cold_catalog.insert_source_config(cold_mixed_source),
        mixed_source_ref
    );
    let cold_mixed_occurrence = AskOccurrence::from_envelope(
        &load_envelope(&reopened, mixed_occurrence_ref.as_artifact_ref()).await,
    )
    .expect("mixed-mode occurrence must cold decode");
    let cold_pure_path =
        ResolutionPath::from_envelope(&load_envelope(&reopened, pure_path.as_artifact_ref()).await)
            .expect("pure identity path must cold decode");
    assert_eq!(cold_catalog.insert_path(cold_pure_path), pure_path);
    let cold_mixed_runtime = RuntimeProgramArtifact::from_envelope(
        &load_envelope(&reopened, mixed_runtime_ref.as_artifact_ref()).await,
    )
    .expect("mixed-mode runtime artifact must cold decode");
    let replay_mixed = reopened
        .replay_completed_external_effect(token_mixed)
        .await
        .expect("mixed-mode effect must cold replay");
    assert_eq!(replay_mixed.event_ref(), mixed_actual.event_ref());
    let mixed_link =
        check_source_event_link(replay_mixed, cold_mixed_occurrence.clone(), &cold_catalog)
            .expect("the Probe port event must recheck against its source occurrence");
    let mixed_link_event = mixed_link.event_ref();
    cold_catalog
        .events
        .insert(mixed_link_event, mixed_link.actuality().event().clone());

    let cold_mixed_ports =
        OpenQueryCatalog::resolve_open_query(&cold_catalog, cold_mixed_occurrence.question())
            .expect("cold mixed-mode query must reload")
            .open_ports()
            .iter()
            .map(|open| PortLowering::new(open.port().clone(), open.mode()))
            .collect::<Vec<_>>();
    assert_eq!(cold_mixed_ports.len(), 2);
    let mixed_lowering = SourceAskLowering::new(
        cold_mixed_occurrence.clone(),
        cold_mixed_ports,
        cold_mixed_runtime,
    )
    .expect("a complete mixed-mode port field must form a lowering");
    mixed_lowering
        .check_expected(&cold_mixed_occurrence, mixed_runtime_ref, &cold_catalog)
        .expect("the mixed-mode lowering must retain its exact occurrence and runtime");

    // Post-return interpretation of the mixed question, regenerated cold from ordinary roots.
    let cold_mixed_query_value = OpenQueryCatalog::resolve_open_query(&cold_catalog, mixed_query)
        .expect("cold mixed-mode query must reload");
    let mixed_query_context = *cold_mixed_query_value.context();
    let derived_form_value = TypedForm::new(
        binding,
        roots.raw_type,
        stored_ref(&reopened, b"mixed-mode-derived-result").await,
    );
    let derived_form = cold_catalog.insert_form(derived_form_value);
    let other_derived_form_value = TypedForm::new(
        binding,
        roots.raw_type,
        stored_ref(&reopened, b"mixed-mode-other-derived-result").await,
    );
    let other_derived_form = cold_catalog.insert_form(other_derived_form_value);
    let mixed_support_value = SupportEnvironmentArtifact::new(
        SupportSubjectRef::Relation(mixed_relation),
        Vec::new(),
        vec![roots.raw_return],
        Vec::new(),
        vec![stored_ref(&reopened, b"mixed-mode-support-assumption").await],
        Vec::new(),
        mixed_query_context.applicability(),
        mixed_query_context.scope(),
    )
    .expect("mixed-mode support environment must encode");
    let mixed_support = cold_catalog.insert_support(mixed_support_value);
    let mixed_use_context = RelationUseContext::new(
        mixed_query_context.scope(),
        mixed_query_context.applicability(),
        mixed_query_context.grain(),
        mixed_query_context.horizon(),
        mixed_query_context.mode(),
        mixed_support.as_support_ref(),
        mixed_query_context.warrant(),
    );
    let mixed_completion = |probe_value, derived_value| {
        vec![
            PortBinding::new(mixed_probe_port.clone(), probe_value),
            PortBinding::new(mixed_pure_port.clone(), derived_value),
        ]
    };
    let mixed_candidate_a = cold_catalog.insert_candidate(
        cold_mixed_query_value
            .plug(
                mixed_completion(roots.answer_a, derived_form),
                &cold_catalog,
            )
            .expect("first mixed-mode completion must fill both ports"),
    );
    let mixed_candidate_b = cold_catalog.insert_candidate(
        cold_mixed_query_value
            .plug(
                mixed_completion(roots.answer_b, derived_form),
                &cold_catalog,
            )
            .expect("second mixed-mode completion must fill both ports"),
    );
    let mixed_observation_a = cold_catalog.insert_relation_use(RelationUse::new(
        mixed_relation,
        mixed_completion(roots.answer_a, derived_form),
        mixed_use_context,
    ));
    let mixed_observation_b = cold_catalog.insert_relation_use(RelationUse::new(
        mixed_relation,
        mixed_completion(roots.answer_b, derived_form),
        mixed_use_context,
    ));
    let mixed_decoder = cold_catalog.insert_decoder(
        FiniteDecoder::new(
            mixed_query,
            roots.raw_type,
            vec![FiniteDecoderEntry::Decoded {
                raw_return: roots.raw_return,
                candidates: vec![mixed_candidate_a, mixed_candidate_b],
            }],
        )
        .expect("mixed-mode decoder must encode"),
    );
    let mixed_decoded_path = cold_catalog.insert_path(ResolutionPath::new(
        roots.raw_type,
        roots.unit,
        ResolutionPathIR::Decode {
            decoder: mixed_decoder.as_decoder_ref(),
        },
    ));
    let mixed_standing = standing_from_declared_support(
        Vec::new(),
        &[DeclaredSupportClosure::for_subjects(
            mixed_support,
            Vec::new(),
            true,
            true,
            false,
        )],
        &cold_catalog,
    )
    .expect("mixed-mode support must reconstruct cold");
    let mixed_raw_value = roots.raw_return.as_artifact_ref();
    let mixed_candidate_values = vec![
        mixed_candidate_a.as_artifact_ref(),
        mixed_candidate_b.as_artifact_ref(),
    ];
    let mixed_run = run_finite_resolution(
        mixed_decoded_path,
        mixed_raw_value,
        &[FiniteResolutionLeafTable::new(
            mixed_decoded_path,
            vec![mixed_raw_value],
            vec![FiniteResolutionLeafEntry::related(
                mixed_raw_value,
                mixed_candidate_values.clone(),
            )],
            FiniteResolutionCoverage::Exact(CoverageRef::from_artifact_ref(
                stored_ref(&reopened, b"mixed-mode-resolution-coverage").await,
            )),
        )
        .expect("mixed-mode decode leaf must be well formed")],
        &cold_catalog,
    )
    .expect("mixed-mode decode must run finitely");
    let ActualDecodeResult::Decoded(mixed_decoded) = decode_actual_event_for_port(
        &mixed_probe_port,
        mixed_link.actuality().event(),
        &cold_catalog
            .resolve_finite_decoder(mixed_decoder)
            .expect("mixed-mode decoder must reload"),
        mixed_decoded_path,
        &cold_catalog,
    )
    .expect("the mixed-mode event must decode through its declared path") else {
        panic!("the mixed-mode event must retain two decoded completions")
    };
    // A port this question does not declare is rejected by name; a declared sibling port is
    // rejected because this route does not land in that port's carrier.
    assert!(matches!(
        decode_actual_event_for_port(
            &TypeSymbol::new("absent").expect("port must be valid"),
            mixed_link.actuality().event(),
            &cold_catalog
                .resolve_finite_decoder(mixed_decoder)
                .expect("mixed-mode decoder must reload"),
            mixed_decoded_path,
            &cold_catalog,
        ),
        Err(ActualDecodeError::ForeignAnswerPort(_))
    ));
    assert!(matches!(
        decode_actual_event_for_port(
            &mixed_pure_port,
            mixed_link.actuality().event(),
            &cold_catalog
                .resolve_finite_decoder(mixed_decoder)
                .expect("mixed-mode decoder must reload"),
            mixed_decoded_path,
            &cold_catalog,
        ),
        Err(ActualDecodeError::PathOutputMismatch { .. })
    ));
    let mixed_observations = vec![
        match_decoded_observation_use(
            &mixed_decoded,
            mixed_candidate_a,
            mixed_observation_a,
            &cold_catalog,
        )
        .expect("first mixed-mode completion must retain its observation"),
        match_decoded_observation_use(
            &mixed_decoded,
            mixed_candidate_b,
            mixed_observation_b,
            &cold_catalog,
        )
        .expect("second mixed-mode completion must retain its observation"),
    ];

    // The all-Probe specialization must keep rejecting a question with any non-Probe port.
    assert!(matches!(
        admit_finite_probe_discharge_bundle(
            cold_mixed_occurrence.clone(),
            vec![ProbePortDischargeEvidence::new(
                mixed_probe_port.clone(),
                route,
                roots.decoded_path,
                binding,
                roots.compiler_version,
                mixed_provenance,
                mixed_link.clone(),
            )],
            Vec::new(),
            &cold_catalog,
        ),
        Err(ProbeDischargeBundleError::NonProbePortOutsideFiniteSpecialization(port))
            if port == mixed_pure_port
    ));
    assert!(matches!(
        SourceAskProbeDischarge::new(mixed_lowering.clone(), bundle.clone()).check(&cold_catalog),
        Err(SourceAskProbeDischargeError::NonProbeLoweringPort(_))
    ));

    let mixed_probe_bundle = admit_probe_ports_of_mixed_discharge(
        cold_mixed_occurrence.clone(),
        vec![ProbePortDischargeEvidence::new(
            mixed_probe_port.clone(),
            route,
            mixed_decoded_path,
            binding,
            roots.compiler_version,
            mixed_provenance,
            mixed_link,
        )],
        Vec::new(),
        &cold_catalog,
    )
    .expect("the Probe-mode subset of a mixed question must admit through the same checker");
    assert_eq!(mixed_probe_bundle.components().len(), 1);
    let pure_evidence = NonProbePortDischargeEvidence::new(
        mixed_pure_port.clone(),
        DischargeMode::Pure,
        NonProbePortOutput::Determined(derived_form),
        route,
        opaque_path,
        binding,
        roots.compiler_version,
        mixed_provenance,
    );
    let mixed_view = MixedModeSourceAskDischarge::new(
        mixed_lowering.clone(),
        mixed_probe_bundle.clone(),
        vec![pure_evidence.clone()],
    );
    mixed_view
        .check(&cold_catalog)
        .expect("one Probe port and one Pure port must form one exact mixed-mode view");
    assert_eq!(mixed_view.probe_bundle().components().len(), 1);
    assert_eq!(mixed_view.non_probe().len(), 1);
    assert_eq!(mixed_view.non_probe()[0].mode(), DischargeMode::Pure);

    let mixed_foil = |non_probe: Vec<NonProbePortDischargeEvidence>| {
        MixedModeSourceAskDischarge::new(
            mixed_lowering.clone(),
            mixed_probe_bundle.clone(),
            non_probe,
        )
        .check(&cold_catalog)
    };
    // An event-bearing mode may not be assigned to the non-Probe port.
    assert!(matches!(
        mixed_foil(vec![NonProbePortDischargeEvidence::new(
            mixed_pure_port.clone(),
            DischargeMode::Probe,
            NonProbePortOutput::Determined(derived_form),
            route,
            opaque_path,
            binding,
            roots.compiler_version,
            mixed_provenance,
        )]),
        Err(MixedModeSourceAskDischargeError::ProbeModeOnNonProbeSide(_))
    ));
    // The non-Probe side may not claim the Probe port.
    assert!(matches!(
        mixed_foil(vec![NonProbePortDischargeEvidence::new(
            mixed_probe_port.clone(),
            DischargeMode::Pure,
            NonProbePortOutput::Determined(derived_form),
            route,
            opaque_path,
            binding,
            roots.compiler_version,
            mixed_provenance,
        )]),
        Err(MixedModeSourceAskDischargeError::ForeignNonProbePort(_))
    ));
    // A declared Pure port may not be discharged under another authority.
    assert!(matches!(
        mixed_foil(vec![NonProbePortDischargeEvidence::new(
            mixed_pure_port.clone(),
            DischargeMode::Warrant,
            NonProbePortOutput::Determined(derived_form),
            route,
            opaque_path,
            binding,
            roots.compiler_version,
            mixed_provenance,
        )]),
        Err(MixedModeSourceAskDischargeError::NonProbeModeMismatch { .. })
    ));
    // A generated proposal carries no actuality authority, so it may not be offered at a port
    // whose declared mode reserves discharge to determination. The two carriers are not
    // interchangeable even when they hold the same typed form.
    assert!(matches!(
        mixed_foil(vec![NonProbePortDischargeEvidence::new(
            mixed_pure_port.clone(),
            DischargeMode::Pure,
            NonProbePortOutput::Proposal(derived_form),
            route,
            opaque_path,
            binding,
            roots.compiler_version,
            mixed_provenance,
        )]),
        Err(MixedModeSourceAskDischargeError::OutputAuthorityMismatch { proposed: true, .. })
    ));
    assert!(matches!(
        mixed_foil(vec![pure_evidence.clone(), pure_evidence.clone()]),
        Err(MixedModeSourceAskDischargeError::DuplicateNonProbePort(_))
    ));
    assert!(matches!(
        mixed_foil(Vec::new()),
        Err(MixedModeSourceAskDischargeError::NonProbePortCoverageMismatch)
    ));
    // Non-Probe output must be an exact resolvable typed form, never an untyped reference.
    assert!(matches!(
        mixed_foil(vec![NonProbePortDischargeEvidence::new(
            mixed_pure_port.clone(),
            DischargeMode::Pure,
            NonProbePortOutput::Determined(TypedFormRef::from_artifact_ref(artifact(0xc7))),
            route,
            opaque_path,
            binding,
            roots.compiler_version,
            mixed_provenance,
        )]),
        Err(MixedModeSourceAskDischargeError::UnresolvedNonProbeResult(
            _
        ))
    ));
    // Its declared route must actually carry that result to this port's carrier type.
    assert!(matches!(
        mixed_foil(vec![NonProbePortDischargeEvidence::new(
            mixed_pure_port.clone(),
            DischargeMode::Pure,
            NonProbePortOutput::Determined(derived_form),
            route,
            pure_path,
            binding,
            roots.compiler_version,
            mixed_provenance,
        )]),
        Err(MixedModeSourceAskDischargeError::NonProbeResolutionTypeMismatch(_))
    ));
    assert!(matches!(
        mixed_foil(vec![NonProbePortDischargeEvidence::new(
            mixed_pure_port.clone(),
            DischargeMode::Pure,
            NonProbePortOutput::Determined(derived_form),
            route,
            opaque_path,
            binding,
            artifact(0xfe),
            mixed_provenance,
        )]),
        Err(MixedModeSourceAskDischargeError::NonProbeCompilerVersionMismatch(_))
    ));
    assert!(matches!(
        mixed_foil(vec![NonProbePortDischargeEvidence::new(
            mixed_pure_port.clone(),
            DischargeMode::Pure,
            NonProbePortOutput::Determined(derived_form),
            route,
            opaque_path,
            BindingVersionRef::from_artifact_ref(artifact(0xfd)),
            roots.compiler_version,
            mixed_provenance,
        )]),
        Err(MixedModeSourceAskDischargeError::NonProbeBindingMismatch(_))
    ));
    assert!(matches!(
        mixed_foil(vec![NonProbePortDischargeEvidence::new(
            mixed_pure_port.clone(),
            DischargeMode::Pure,
            NonProbePortOutput::Determined(derived_form),
            route,
            opaque_path,
            binding,
            roots.compiler_version,
            ProvenanceRef::from_artifact_ref(artifact(0xfc)),
        )]),
        Err(MixedModeSourceAskDischargeError::NonProbeProvenanceMismatch(_))
    ));
    // Another occurrence's structurally compatible Probe bundle is not interchangeable.
    assert!(matches!(
        MixedModeSourceAskDischarge::new(
            mixed_lowering.clone(),
            bundle,
            vec![pure_evidence.clone()],
        )
        .check(&cold_catalog),
        Err(MixedModeSourceAskDischargeError::OccurrenceMismatch)
    ));
    // An all-Probe occurrence has no non-Probe side and stays outside this view.
    assert!(matches!(
        MixedModeSourceAskDischarge::new(
            cold_source_lowering,
            mixed_probe_bundle.clone(),
            vec![pure_evidence.clone()],
        )
        .check(&cold_catalog),
        Err(MixedModeSourceAskDischargeError::NoNonProbePort)
    ));

    // The whole-question gate. The five-way classifier is port-indexed here: the arity-one entry
    // point must keep refusing a two-port question, and a run typed for one port must not be
    // accepted for its sibling.
    let classify_mixed_port = |port: &TypeSymbol| {
        classify_finite_port_resolution(
            port,
            mixed_link_event,
            mixed_query,
            mixed_run.clone(),
            Some(mixed_decoded.clone()),
            mixed_observations.clone(),
            &mixed_standing,
            &cold_catalog,
        )
    };
    assert!(matches!(
        classify_finite_question_resolution(
            mixed_link_event,
            mixed_query,
            mixed_run.clone(),
            Some(mixed_decoded.clone()),
            mixed_observations.clone(),
            &mixed_standing,
            &cold_catalog,
        ),
        Err(FiniteResolutionGateError::UnsupportedAnswerArity(2))
    ));
    assert!(matches!(
        classify_mixed_port(&mixed_pure_port),
        Err(FiniteResolutionGateError::ResolutionTypeMismatch(_))
    ));
    assert!(matches!(
        classify_mixed_port(&TypeSymbol::new("absent").expect("port must be valid")),
        Err(FiniteResolutionGateError::ForeignAnswerPort(_))
    ));

    let mixed_supported =
        classify_mixed_port(&mixed_probe_port).expect("the Probe port must classify as Supported");
    assert_eq!(
        mixed_supported.kind(),
        FiniteResolutionOutcomeKind::Supported
    );
    let whole = resolve_mixed_mode_question(
        &mixed_view,
        vec![(mixed_probe_port.clone(), mixed_supported)],
        &cold_catalog,
    )
    .expect("one Probe outcome plus one checked Pure port must resolve the whole question");
    assert_eq!(whole.kind(), FiniteResolutionOutcomeKind::Supported);
    let WholeQuestionOutcome::Supported(whole_answer) = &whole else {
        panic!("the joint outcome must be Supported")
    };
    assert_eq!(whole_answer.contributions().len(), 2);
    assert_eq!(
        whole_answer
            .contributions()
            .iter()
            .map(MixedPortContribution::mode)
            .collect::<Vec<_>>(),
        vec![DischargeMode::Pure, DischargeMode::Probe],
        "both declared modes survive the joint answer in canonical port order"
    );
    let MixedPortContribution::Probe { resolution, .. } = &whole_answer.contributions()[1] else {
        panic!("the Probe port must contribute its event-linked resolution")
    };
    assert_eq!(
        resolution.answer().candidates().len(),
        2,
        "the joint answer retains every decoded completion rather than selecting one"
    );

    // Canonical SuppAns carries a nonempty member set of whole completions beside its
    // component-indexed map. The two projections are different objects and must agree.
    assert_eq!(
        whole_answer.members(),
        [mixed_candidate_a, mixed_candidate_b],
        "the member projection is the completion set, not the per-port contribution map"
    );
    assert_ne!(whole_answer.members().len(), 0);
    for member in whole_answer.members() {
        let completion = cold_catalog
            .resolve_completion_candidate(*member)
            .expect("every member must resolve");
        assert_eq!(
            completion.source(),
            mixed_query,
            "every member completes this exact question"
        );
        assert_eq!(
            completion
                .bindings()
                .iter()
                .map(|binding| binding.port().clone())
                .collect::<std::collections::BTreeSet<_>>(),
            [mixed_pure_port.clone(), mixed_probe_port.clone()]
                .into_iter()
                .collect::<std::collections::BTreeSet<_>>(),
            "a member is a completion of the whole port field, not of one port"
        );
    }

    // Canonical: equality of supported answers is equality of the whole proof-carrying record,
    // explicitly not equality of its member projection. Two answers over one occurrence can agree
    // on every member and on every port's typed result and still have been reached by different
    // declared routes. The non-Probe route is retained but never cross-checked, so it is exactly
    // the coordinate that can vary while everything else is held fixed.
    let alternate_route =
        RouteRef::from_artifact_ref(stored_ref(&reopened, b"mixed-mode-alternate-route").await);
    assert_ne!(alternate_route, route);
    let rerouted_view = MixedModeSourceAskDischarge::new(
        mixed_lowering.clone(),
        mixed_probe_bundle.clone(),
        vec![NonProbePortDischargeEvidence::new(
            mixed_pure_port.clone(),
            DischargeMode::Pure,
            NonProbePortOutput::Determined(derived_form),
            alternate_route,
            opaque_path,
            binding,
            roots.compiler_version,
            mixed_provenance,
        )],
    );
    rerouted_view
        .check(&cold_catalog)
        .expect("a different declared non-Probe route is itself lawful");
    let rerouted = resolve_mixed_mode_question(
        &rerouted_view,
        vec![(
            mixed_probe_port.clone(),
            classify_mixed_port(&mixed_probe_port).expect("Supported must reproduce"),
        )],
        &cold_catalog,
    )
    .expect("the rerouted answer must resolve");
    let WholeQuestionOutcome::Supported(rerouted_answer) = &rerouted else {
        panic!("the rerouted joint outcome must be Supported")
    };
    assert_eq!(
        rerouted_answer.members(),
        whole_answer.members(),
        "the two answers agree on their whole member projection"
    );
    assert_eq!(
        rerouted_answer.contributions().len(),
        whole_answer.contributions().len()
    );
    assert_ne!(
        rerouted_answer, whole_answer,
        "equal member projections do not make one record: the declared route separates them"
    );

    // The Probe port keeps no route field of its own; its route is recovered from the retained
    // event, which the bundle already checked against that component's declared route.
    let probe_component = mixed_probe_bundle
        .components()
        .iter()
        .find(|component| component.port() == &mixed_probe_port)
        .expect("the Probe port has a bundle component");
    let recovered_route =
        OperatorOccurrenceCatalog::resolve_actual_event(&cold_catalog, resolution.answer().event())
            .expect("the retained event must resolve")
            .route();
    assert_eq!(
        recovered_route,
        probe_component.route(),
        "a Probe port's route is recoverable from its retained event"
    );

    // A Probe return alone cannot resolve the question while a port is unaccounted for.
    assert!(matches!(
        resolve_mixed_mode_question(&mixed_view, Vec::new(), &cold_catalog),
        Err(MixedQuestionResolutionError::MissingPortOutcome(_))
    ));
    assert!(matches!(
        resolve_mixed_mode_question(
            &mixed_view,
            vec![(
                mixed_pure_port.clone(),
                classify_mixed_port(&mixed_probe_port).expect("Supported must reproduce"),
            )],
            &cold_catalog,
        ),
        Err(MixedQuestionResolutionError::ForeignPortOutcome(_))
    ));
    assert!(matches!(
        resolve_mixed_mode_question(
            &mixed_view,
            vec![
                (
                    mixed_probe_port.clone(),
                    classify_mixed_port(&mixed_probe_port).expect("Supported must reproduce"),
                ),
                (
                    mixed_probe_port.clone(),
                    classify_mixed_port(&mixed_probe_port).expect("Supported must reproduce"),
                ),
            ],
            &cold_catalog,
        ),
        Err(MixedQuestionResolutionError::DuplicatePortOutcome(_))
    ));

    // A completion may not assign the non-Probe port a value its own evidence contradicts.
    let contradicting_view = MixedModeSourceAskDischarge::new(
        mixed_lowering.clone(),
        mixed_probe_bundle.clone(),
        vec![NonProbePortDischargeEvidence::new(
            mixed_pure_port.clone(),
            DischargeMode::Pure,
            NonProbePortOutput::Determined(other_derived_form),
            route,
            opaque_path,
            binding,
            roots.compiler_version,
            mixed_provenance,
        )],
    );
    contradicting_view
        .check(&cold_catalog)
        .expect("the contradicting evidence is itself well formed");
    assert!(matches!(
        resolve_mixed_mode_question(
            &contradicting_view,
            vec![(
                mixed_probe_port.clone(),
                classify_mixed_port(&mixed_probe_port).expect("Supported must reproduce"),
            )],
            &cold_catalog,
        ),
        Err(MixedQuestionResolutionError::CompletionContradictsNonProbeResult { .. })
    ));

    // None of the other four outcomes may reach the continuation.
    let cold_mixed_program = cold_catalog
        .resolve_iprog(mixed_source_program)
        .expect("mixed-mode source program must reload");
    let partial_run = run_finite_resolution(
        mixed_decoded_path,
        mixed_raw_value,
        &[FiniteResolutionLeafTable::new(
            mixed_decoded_path,
            vec![mixed_raw_value],
            Vec::new(),
            FiniteResolutionCoverage::Partial(CoverageRef::from_artifact_ref(
                stored_ref(&reopened, b"mixed-mode-partial-coverage").await,
            )),
        )
        .expect("a partial mixed-mode leaf must be well formed")],
        &cold_catalog,
    )
    .expect("partial coverage must run without claiming closure");
    let unknown = classify_finite_port_resolution(
        &mixed_probe_port,
        mixed_link_event,
        mixed_query,
        partial_run,
        None,
        Vec::new(),
        &mixed_standing,
        &cold_catalog,
    )
    .expect("incomplete coverage must classify as Unknown");
    assert_eq!(unknown.kind(), FiniteResolutionOutcomeKind::Unknown);
    let joint = resolve_mixed_mode_question(
        &mixed_view,
        vec![(mixed_probe_port.clone(), unknown)],
        &cold_catalog,
    )
    .expect("a non-Supported port still yields exactly one whole-question outcome");
    assert_eq!(joint.kind(), FiniteResolutionOutcomeKind::Unknown);
    assert!(matches!(
        admit_mixed_mode_continuation(joint, &mixed_view, &cold_mixed_program, &cold_catalog),
        Err(MixedQuestionResolutionError::NonSupported(_))
    ));

    // Only Supported reaches the exact checked continuation, and only through its own source.
    assert!(matches!(
        admit_mixed_mode_continuation(
            resolve_mixed_mode_question(
                &mixed_view,
                vec![(
                    mixed_probe_port.clone(),
                    classify_mixed_port(&mixed_probe_port).expect("Supported must reproduce"),
                )],
                &cold_catalog,
            )
            .expect("the joint answer must reproduce"),
            &mixed_view,
            &cold_first_program,
            &cold_catalog,
        ),
        Err(MixedQuestionResolutionError::SourceProgramMismatch { .. })
    ));
    let admitted =
        admit_mixed_mode_continuation(whole, &mixed_view, &cold_mixed_program, &cold_catalog)
            .expect("the joint Supported answer must reach its exact source continuation");
    assert_eq!(admitted.question(), mixed_query);
    assert_eq!(admitted.continuation(), roots.continuation);
    assert_eq!(admitted.answer().contributions().len(), 2);
    assert_eq!(admitted.answer().members().len(), 2);

    // The successor relation reads only the occurrence, so the whole-question answer is a second
    // carrier for the same relation rather than a second relation. Deriving the next position
    // through the single-port carrier and through the whole-question carrier must agree.
    let mixed_port_answer = classify_mixed_port(&mixed_probe_port)
        .expect("Supported must reproduce")
        .into_supported()
        .expect("the Probe port is Supported")
        .answer()
        .clone();
    let single_port_successor = derive_question_successor(
        cold_mixed_occurrence.clone(),
        mixed_port_answer,
        &cold_catalog,
    )
    .expect("the single-port carrier must still derive this occurrence's successor");
    let mixed_successor = derive_mixed_mode_successor(admitted, &cold_catalog)
        .expect("the whole-question answer must carry the occurrence to its next position");
    assert_eq!(
        mixed_successor.answer().members().len(),
        2,
        "the successor carries the whole record, not one port's answer set"
    );
    assert_eq!(mixed_successor.answer().contributions().len(), 2);
    let NextSourcePosition::Return { value } = mixed_successor.next() else {
        panic!("this occurrence's continuation is a Return")
    };
    assert_eq!(*value, roots.answer_a);
    assert!(
        matches!(
            &single_port_successor,
            ic_core::QuestionSuccessor::Return { value: single, .. } if single == value
        ),
        "one successor relation, two answer carriers, one next position"
    );

    assert_eq!(
        provider_calls.load(Ordering::SeqCst),
        5,
        "cold replay must not redispatch any occurrence, the shared-port event, or the mixed-mode Probe port"
    );
    reopened.close().await;
    std::fs::remove_file(path).expect("temporary occurrence replay database must be removable");
}

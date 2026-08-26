use ic_core::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactKind, ArtifactRef, BackendRequest,
    BindingVersionRef, BoundaryChart, DeterminationPresentationRef, DischargeMode, FormulaRef,
    GrainRef, HorizonRef, OpenQuery, ProbeContractRef, ProbeOperator, ProvenanceRef, QueryRef,
    RelationRef, RelationUseContext, RelationUseRef, RouteRef, ScopeRef, StateRef, SupportRef,
    SurfacePlan, TyIR, TypeArtifact, TypeCatalog, TypeFamilyRef, TypeRef, TypedForm, TypedFormRef,
};
use ic_runtime::{
    BasicBlock, BlockTarget, MachineStep, OllamaGenerateProvider, OllamaHttpResponse,
    OpenAiHttpResponse, OpenAiResponsesProvider, ProbeDispatchContext, ProbeDispatchError,
    ProbeProvider, ProgramIR, ProviderReturn, RuntimeCatalog, Terminator,
    decode_ollama_candidate_response, dispatch_probe,
};
use ic_store::{ArtifactStore, DispatchToken, ExternalEffectState};
use thiserror::Error;

fn envelope(payload: &[u8]) -> ArtifactEnvelope {
    ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("fixture").expect("fixture kind must be valid"),
        1,
        payload.to_vec(),
    )
}

async fn stored_ref(store: &ArtifactStore, payload: &[u8]) -> ArtifactRef {
    store
        .insert(&envelope(payload))
        .await
        .expect("fixture dependency must insert")
}

struct RuntimeTypes {
    ty: TypeArtifact,
    form: TypedForm,
}

impl TypeCatalog for RuntimeTypes {
    fn resolve_type(&self, reference: TypeRef) -> Option<TypeArtifact> {
        (self.ty.type_ref().ok() == Some(reference)).then(|| self.ty.clone())
    }

    fn resolve_family_domain(
        &self,
        _reference: TypeFamilyRef,
    ) -> Option<(BindingVersionRef, TypeRef)> {
        None
    }
}

impl RuntimeCatalog for RuntimeTypes {
    fn resolve_typed_form(&self, reference: TypedFormRef) -> Option<TypedForm> {
        (self.form.typed_form_ref().ok() == Some(reference)).then_some(self.form)
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
#[error("mock backend refused the request")]
struct MockFailure;

struct MockProvider {
    calls: usize,
    fail: bool,
    expected_body: ArtifactRef,
}

impl ProbeProvider for MockProvider {
    type Error = MockFailure;

    fn dispatch(&mut self, request: &BackendRequest) -> Result<ProviderReturn, Self::Error> {
        self.calls += 1;
        assert_eq!(request.request_body(), self.expected_body);
        if self.fail {
            Err(MockFailure)
        } else {
            Ok(ProviderReturn::new(vec![0, 0xff, b'{', 0]))
        }
    }
}

struct Fixture {
    store: ArtifactStore,
    runtime: ProgramIR,
    types: RuntimeTypes,
    request: ic_core::BackendRequestRef,
    context: ProbeDispatchContext,
    request_body: ArtifactRef,
}

async fn fixture() -> Fixture {
    fixture_with_request_body(b"request-body").await
}

async fn fixture_with_request_body(request_bytes: &[u8]) -> Fixture {
    let store = ArtifactStore::open("sqlite::memory:")
        .await
        .expect("in-memory store must open");
    store.migrate().await.expect("migrations must apply");

    let binding = BindingVersionRef::from_artifact_ref(stored_ref(&store, b"binding").await);
    let ty = TypeArtifact::new(binding, TyIR::Unit);
    let ty_ref = ty.type_ref().expect("type must encode");
    store
        .insert_referencing(
            &ty.envelope().expect("type must encode"),
            &ty.referenced_artifacts(),
        )
        .await
        .expect("type must persist");
    let form = TypedForm::new(binding, ty_ref, stored_ref(&store, b"return-form").await);
    let form_ref = form.typed_form_ref().expect("form must encode");
    store
        .insert_referencing(
            &form.envelope().expect("form must encode"),
            &form.referenced_artifacts(),
        )
        .await
        .expect("form must persist");

    let shared = stored_ref(&store, b"shared-boundary-field").await;
    let grain = GrainRef::from_artifact_ref(stored_ref(&store, b"grain").await);
    let horizon = HorizonRef::from_artifact_ref(stored_ref(&store, b"horizon").await);
    let query_value = OpenQuery::new(
        RelationRef::from_artifact_ref(shared),
        Vec::new(),
        Vec::new(),
        RelationUseContext::new(
            ScopeRef::from_artifact_ref(shared),
            ApplicabilityRef::from_artifact_ref(shared),
            grain,
            horizon,
            DischargeMode::Probe,
            SupportRef::from_artifact_ref(shared),
            None,
        ),
    );
    let query = QueryRef::from_artifact_ref(
        store
            .insert_referencing(
                &query_value.envelope().expect("query must encode"),
                &query_value.referenced_artifacts(),
            )
            .await
            .expect("query must persist"),
    );
    let chart = BoundaryChart::new(
        query,
        ty_ref,
        ty_ref,
        ty_ref,
        RelationRef::from_artifact_ref(shared),
        RelationRef::from_artifact_ref(shared),
        DeterminationPresentationRef::from_artifact_ref(shared),
        None,
        Vec::new(),
        Vec::new(),
        RelationUseRef::from_artifact_ref(shared),
        FormulaRef::from_artifact_ref(shared),
        None,
        grain,
        horizon,
    );
    let boundary = ic_core::BoundaryRef::from_artifact_ref(
        store
            .insert_referencing(
                &chart.envelope().expect("chart must encode"),
                &chart.referenced_artifacts(),
            )
            .await
            .expect("chart must persist"),
    );
    let active_view = stored_ref(&store, b"active-view").await;
    let backend = stored_ref(&store, b"mock-backend").await;
    let executable_code = stored_ref(&store, b"executable-code").await;
    let decoder = stored_ref(&store, b"decoder-contract").await;
    let probe_contract =
        ProbeContractRef::from_artifact_ref(stored_ref(&store, b"probe-contract").await);
    let compiler_version = stored_ref(&store, b"compiler-version").await;
    let operator_value = ProbeOperator::new(
        query,
        boundary,
        active_view,
        backend,
        executable_code,
        ty_ref,
        decoder,
        probe_contract,
        compiler_version,
    );
    let operator = ic_core::ProbeOperatorRef::from_artifact_ref(
        store
            .insert_referencing(
                &operator_value.envelope().expect("operator must encode"),
                &operator_value.referenced_artifacts(),
            )
            .await
            .expect("operator must persist"),
    );
    let renderer_version = stored_ref(&store, b"renderer-version").await;
    let rendered_body = stored_ref(&store, b"rendered-body").await;
    let plan_value = SurfacePlan::new(
        operator,
        query,
        boundary,
        active_view,
        executable_code,
        probe_contract,
        renderer_version,
        rendered_body,
    );
    let plan = ic_core::SurfacePlanRef::from_artifact_ref(
        store
            .insert_referencing(
                &plan_value.envelope().expect("plan must encode"),
                &plan_value.referenced_artifacts(),
            )
            .await
            .expect("plan must persist"),
    );
    let backend_version = stored_ref(&store, b"backend-version").await;
    let request_body = stored_ref(&store, request_bytes).await;
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
        store
            .insert_referencing(
                &request_value.envelope().expect("request must encode"),
                &request_value.referenced_artifacts(),
            )
            .await
            .expect("request must persist"),
    );

    let types = RuntimeTypes { ty, form };
    let runtime = ProgramIR::new(
        ty_ref,
        BlockTarget::new(0),
        vec![
            BasicBlock::new(
                BlockTarget::new(0),
                Terminator::Probe {
                    operator,
                    resume: BlockTarget::new(1),
                },
            ),
            BasicBlock::new(BlockTarget::new(1), Terminator::Return { value: form_ref }),
        ],
    );
    runtime.verify(&types).expect("runtime must verify");
    let context = ProbeDispatchContext::new(
        None,
        StateRef::from_artifact_ref(stored_ref(&store, b"state-before").await),
        None,
        StateRef::from_artifact_ref(stored_ref(&store, b"state-after").await),
        grain,
        RouteRef::from_artifact_ref(stored_ref(&store, b"route").await),
        binding,
        ProvenanceRef::from_artifact_ref(stored_ref(&store, b"provenance").await),
    );
    Fixture {
        store,
        runtime,
        types,
        request,
        context,
        request_body,
    }
}

#[tokio::test]
#[ignore = "requires OPENAI_API_KEY and live OpenAI Responses API access"]
async fn live_openai_response_is_committed_before_json_interpretation() {
    let request_json = serde_json::to_vec(&serde_json::json!({
        "model": "gpt-5.6-luna",
        "input": "Return exactly two distinct one-word candidate completions as a JSON array of strings, with no markdown or explanation.",
        "max_output_tokens": 128,
        "store": false
    }))
    .expect("live Responses request must encode");
    let fixture = fixture_with_request_body(&request_json).await;
    let MachineStep::Suspended(suspension) = fixture
        .runtime
        .step(fixture.runtime.start())
        .expect("live runtime must step to its probe")
    else {
        panic!("live runtime entry must suspend")
    };
    let mut provider = OpenAiResponsesProvider::from_env(fixture.request_body, request_json)
        .expect("OPENAI_API_KEY must configure the live provider");
    let actual = dispatch_probe(
        &fixture.store,
        suspension,
        DispatchToken::from_bytes([0xd1; 32]),
        fixture.request,
        fixture.context,
        &mut provider,
    )
    .await
    .expect("live Responses return must commit as ordinary actuality");

    assert!(
        fixture
            .store
            .get(actual.raw_return_ref().as_artifact_ref())
            .await
            .expect("committed live raw return must reload")
            .is_some()
    );
    assert_eq!(actual.event().raw_return(), actual.raw_return_ref());
    let transport = OpenAiHttpResponse::decode(actual.raw_return().bytes())
        .expect("committed provider transport return must decode after actuality");
    assert_eq!(
        transport.status(),
        200,
        "live provider must authorize the request"
    );
    let response: serde_json::Value = serde_json::from_slice(transport.body())
        .expect("interpretation occurs only after dispatch returned committed bytes");
    assert_eq!(response["status"], "completed");
    assert_eq!(response["model"], "gpt-5.6-luna");
    assert!(
        response["output"]
            .as_array()
            .is_some_and(|output| !output.is_empty()),
        "completed live response must carry at least one output item"
    );
}

#[tokio::test]
#[ignore = "requires local Ollama with qwen3.5:9b"]
async fn live_ollama_response_is_committed_before_candidate_interpretation() {
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
    .expect("live Ollama request must encode");
    let fixture = fixture_with_request_body(&request_json).await;
    let MachineStep::Suspended(suspension) = fixture
        .runtime
        .step(fixture.runtime.start())
        .expect("live runtime must step to its probe")
    else {
        panic!("live runtime entry must suspend")
    };
    let mut provider = OllamaGenerateProvider::new(fixture.request_body, request_json)
        .expect("local Ollama provider must configure");
    let actual = dispatch_probe(
        &fixture.store,
        suspension,
        DispatchToken::from_bytes([0xd2; 32]),
        fixture.request,
        fixture.context,
        &mut provider,
    )
    .await
    .expect("live Ollama return must commit as ordinary actuality");

    assert!(
        fixture
            .store
            .get(actual.raw_return_ref().as_artifact_ref())
            .await
            .expect("committed live raw return must reload")
            .is_some()
    );
    assert_eq!(actual.event().raw_return(), actual.raw_return_ref());
    let transport = OllamaHttpResponse::decode(actual.raw_return().bytes())
        .expect("committed provider transport return must decode after actuality");
    assert_eq!(transport.status(), 200, "local model request must succeed");
    let decoded = decode_ollama_candidate_response(actual.raw_return().bytes())
        .expect("candidate interpretation occurs only after committed actuality");
    assert_eq!(decoded.model(), "qwen3.5:9b");
    assert_eq!(decoded.done_reason(), "stop");
    assert_eq!(decoded.candidates().len(), 2);
    assert_ne!(decoded.candidates()[0], decoded.candidates()[1]);
}

#[tokio::test]
async fn mock_dispatch_requires_fresh_preparation_and_commits_raw_event_before_return() {
    let fixture = fixture().await;
    fixture
        .runtime
        .verify(&fixture.types)
        .expect("runtime must remain verified");
    let MachineStep::Suspended(suspension) = fixture
        .runtime
        .step(fixture.runtime.start())
        .expect("probe must step")
    else {
        panic!("entry must suspend")
    };
    let token = DispatchToken::from_bytes([0xa1; 32]);
    let mut provider = MockProvider {
        calls: 0,
        fail: false,
        expected_body: fixture.request_body,
    };
    let actual = dispatch_probe(
        &fixture.store,
        suspension,
        token,
        fixture.request,
        fixture.context,
        &mut provider,
    )
    .await
    .expect("fresh durable preparation must dispatch once and complete");
    assert_eq!(provider.calls, 1);
    assert_eq!(actual.raw_return().bytes(), [0, 0xff, b'{', 0]);
    assert_eq!(actual.event().raw_return(), actual.raw_return_ref());
    assert_eq!(actual.event().event_ref().ok(), Some(actual.event_ref()));
    assert!(
        fixture
            .store
            .get(actual.raw_return_ref().as_artifact_ref())
            .await
            .expect("raw lookup must succeed")
            .is_some()
    );
    assert!(
        fixture
            .store
            .get(actual.event_ref().as_artifact_ref())
            .await
            .expect("event lookup must succeed")
            .is_some()
    );

    let repeated = dispatch_probe(
        &fixture.store,
        suspension,
        token,
        fixture.request,
        fixture.context,
        &mut provider,
    )
    .await;
    assert!(matches!(
        repeated,
        Err(ProbeDispatchError::DispatchNotAuthorized(
            ExternalEffectState::Completed { .. }
        ))
    ));
    assert_eq!(provider.calls, 1, "existing completion must not redispatch");

    let failure_token = DispatchToken::from_bytes([0xa2; 32]);
    let failure_context = ProbeDispatchContext::new(
        Some(actual.event_ref()),
        actual.event().state_after(),
        None,
        actual.event().state_after(),
        actual.event().grain(),
        actual.event().route(),
        actual.event().binding(),
        actual.event().provenance(),
    );
    provider.fail = true;
    let failed = dispatch_probe(
        &fixture.store,
        suspension,
        failure_token,
        fixture.request,
        failure_context,
        &mut provider,
    )
    .await;
    assert!(matches!(
        failed,
        Err(ProbeDispatchError::Provider(MockFailure))
    ));
    assert_eq!(provider.calls, 2);
    assert!(matches!(
        fixture
            .store
            .external_effect_state(failure_token)
            .await
            .expect("failure recovery state must load"),
        Some(ExternalEffectState::Pending { .. })
    ));

    let recovered = dispatch_probe(
        &fixture.store,
        suspension,
        failure_token,
        fixture.request,
        failure_context,
        &mut provider,
    )
    .await;
    assert!(matches!(
        recovered,
        Err(ProbeDispatchError::DispatchNotAuthorized(
            ExternalEffectState::Pending { .. }
        ))
    ));
    assert_eq!(provider.calls, 2, "pending recovery must not redispatch");
}

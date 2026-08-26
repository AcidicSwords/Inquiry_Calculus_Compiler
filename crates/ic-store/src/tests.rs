use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use ic_core::{
    ActualEvent, ApplicabilityRef, ArtifactKind, ArtifactRef, BackendRequest, BindingVersionRef,
    BoundaryChart, BoundaryRef, DeterminationPresentationRef, DischargeMode, EventRef, FormulaRef,
    GrainRef, HorizonRef, OpenQuery, OperatorRef, ProbeContractRef, ProbeOperator, ProvenanceRef,
    QueryRef, RawReturn, RawReturnError, RawReturnRef, RelationRef, RelationUseContext,
    RelationUseRef, RouteRef, ScopeRef, StateRef, SupportRef, SurfacePlan, TyIR, TypeArtifact,
    TypeRef,
};

use super::*;

async fn migrated_store() -> ArtifactStore {
    let store = ArtifactStore::open("sqlite::memory:")
        .await
        .expect("in-memory store must open");
    store.migrate().await.expect("migrations must apply");
    store
}

fn envelope(payload: &[u8]) -> ArtifactEnvelope {
    ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("example").expect("valid kind"),
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

async fn event_fixture(
    store: &ArtifactStore,
    ledger_parent: Option<EventRef>,
    backend_version: &[u8],
) -> ActualEvent {
    let raw = RawReturn::new(vec![0, 0xff, b'{', 0]);
    let raw_envelope = raw.envelope().expect("raw return must encode");
    let raw_return = store
        .insert(&raw_envelope)
        .await
        .expect("raw return must insert");
    let chart_field = stored_ref(store, b"boundary-chart-field").await;
    let grain = GrainRef::from_artifact_ref(stored_ref(store, b"grain").await);
    let question = OpenQuery::new(
        RelationRef::from_artifact_ref(chart_field),
        Vec::new(),
        Vec::new(),
        RelationUseContext::new(
            ScopeRef::from_artifact_ref(chart_field),
            ApplicabilityRef::from_artifact_ref(chart_field),
            grain,
            HorizonRef::from_artifact_ref(chart_field),
            DischargeMode::Probe,
            SupportRef::from_artifact_ref(chart_field),
            None,
        ),
    );
    let question = QueryRef::from_artifact_ref(
        store
            .insert(&question.envelope().expect("question must encode"))
            .await
            .expect("question must insert"),
    );
    let chart = BoundaryChart::new(
        question,
        TypeRef::from_artifact_ref(chart_field),
        TypeRef::from_artifact_ref(chart_field),
        TypeRef::from_artifact_ref(chart_field),
        RelationRef::from_artifact_ref(chart_field),
        RelationRef::from_artifact_ref(chart_field),
        DeterminationPresentationRef::from_artifact_ref(chart_field),
        None,
        vec![],
        vec![],
        RelationUseRef::from_artifact_ref(chart_field),
        FormulaRef::from_artifact_ref(chart_field),
        None,
        grain,
        HorizonRef::from_artifact_ref(chart_field),
    );
    let boundary = BoundaryRef::from_artifact_ref(
        store
            .insert(&chart.envelope().expect("boundary chart must encode"))
            .await
            .expect("boundary chart must insert"),
    );
    let operator = OperatorRef::from_artifact_ref(
        store
            .insert(
                &ProbeOperator::new(
                    question,
                    boundary,
                    chart_field,
                    chart_field,
                    chart_field,
                    TypeRef::from_artifact_ref(chart_field),
                    chart_field,
                    ProbeContractRef::from_artifact_ref(chart_field),
                    chart_field,
                )
                .envelope()
                .expect("probe operator must encode"),
            )
            .await
            .expect("probe operator must insert"),
    );
    ActualEvent::new(
        ledger_parent,
        StateRef::from_artifact_ref(stored_ref(store, b"state-before").await),
        question,
        boundary,
        None,
        operator,
        ic_core::RawReturnRef::from_artifact_ref(raw_return),
        StateRef::from_artifact_ref(stored_ref(store, b"state-after").await),
        grain,
        RouteRef::from_artifact_ref(stored_ref(store, b"route").await),
        BindingVersionRef::from_artifact_ref(stored_ref(store, b"binding").await),
        stored_ref(store, backend_version).await,
        ProvenanceRef::from_artifact_ref(stored_ref(store, b"provenance").await),
    )
}

#[tokio::test]
async fn migrations_apply_and_repeat_without_schema_changes() {
    let store = migrated_store().await;
    store
        .migrate()
        .await
        .expect("second migration run must pass");

    let table_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sqlite_schema WHERE type = 'table' AND name = 'artifacts'",
    )
    .fetch_one(&store.pool)
    .await
    .expect("schema must be queryable");
    assert_eq!(table_count, 1);

    let journal_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sqlite_schema WHERE type = 'table' AND name = 'event_ledger'",
    )
    .fetch_one(&store.pool)
    .await
    .expect("journal schema must be queryable");
    assert_eq!(journal_count, 1);

    let effect_journal_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sqlite_schema WHERE type = 'table' AND name = 'external_effect_journal'",
    )
    .fetch_one(&store.pool)
    .await
    .expect("external-effect recovery schema must be queryable");
    assert_eq!(effect_journal_count, 1);
}

#[tokio::test]
async fn external_effect_preparation_survives_restart_and_completes_as_one_raw_event() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    let path = env::temp_dir().join(format!(
        "inquiry-calculus-effect-restart-{}-{nonce}.sqlite",
        std::process::id()
    ));
    let url = format!("sqlite://{}", path.to_string_lossy().replace('\\', "/"));

    let store = ArtifactStore::open(&url)
        .await
        .expect("file-backed store must open");
    store.migrate().await.expect("migrations must apply");
    let event = event_fixture(&store, None, b"effect-backend").await;
    let operator = store
        .verify_probe_operator(event.operator())
        .await
        .expect("event operator must remain available");
    let renderer_version = stored_ref(&store, b"renderer-version").await;
    let rendered_body = stored_ref(&store, b"rendered-surface-body").await;
    let plan = SurfacePlan::new(
        event.operator(),
        operator.query(),
        operator.boundary(),
        operator.active_view(),
        operator.executable_code(),
        operator.probe_contract(),
        renderer_version,
        rendered_body,
    );
    let plan_ref = ic_core::SurfacePlanRef::from_artifact_ref(
        store
            .insert_referencing(
                &plan.envelope().expect("surface plan must encode"),
                &plan.referenced_artifacts(),
            )
            .await
            .expect("surface plan dependencies must exist"),
    );
    let request_body = stored_ref(&store, b"provider-request-body").await;
    let backend_version = stored_ref(&store, b"provider-backend-version").await;
    let request_value = BackendRequest::new(
        event.operator(),
        plan_ref,
        operator.query(),
        operator.boundary(),
        operator.backend(),
        operator.executable_code(),
        operator.compiler_version(),
        backend_version,
        request_body,
    );
    let request = ic_core::BackendRequestRef::from_artifact_ref(
        store
            .insert_referencing(
                &request_value
                    .envelope()
                    .expect("backend request must encode"),
                &request_value.referenced_artifacts(),
            )
            .await
            .expect("backend request dependencies must exist"),
    );
    let token = DispatchToken::from_bytes([0xa1; 32]);
    let prepared = store
        .prepare_backend_request(token, request, event.operator(), None)
        .await
        .expect("intent must be durable before dispatch");
    assert!(prepared.dispatch_authorized());
    let pending = prepared.state();
    assert!(matches!(pending, ExternalEffectState::Pending { .. }));
    let repeated = store
        .prepare_backend_request(token, request, event.operator(), None)
        .await
        .expect("exact preparation must be idempotent");
    assert_eq!(repeated, ExternalEffectPreparation::Existing(pending));
    assert!(!repeated.dispatch_authorized());
    let conflicting_request = stored_ref(&store, b"other-request").await;
    assert!(matches!(
        store
            .prepare_external_effect(token, conflicting_request, event.operator(), None)
            .await,
        Err(StoreError::DispatchTokenConflict(conflict)) if conflict == token
    ));
    let other_token = DispatchToken::from_bytes([0xa2; 32]);
    assert!(matches!(
        store
            .prepare_backend_request(other_token, request, event.operator(), None)
            .await,
        Err(StoreError::ExternalEffectAlreadyPending(conflict)) if conflict == token
    ));

    sqlx::query("DELETE FROM artifacts WHERE artifact_ref = ?")
        .bind(event.raw_return().as_artifact_ref().as_bytes().as_slice())
        .execute(&store.pool)
        .await
        .expect("fixture must remove the preinserted raw return before simulated dispatch");
    store.close().await;

    let restarted = ArtifactStore::open(&url)
        .await
        .expect("prepared-effect store must reopen");
    restarted
        .migrate()
        .await
        .expect("embedded migrations must reapply harmlessly");
    assert_eq!(
        restarted
            .external_effect_state(token)
            .await
            .expect("prepared state must load"),
        Some(pending)
    );
    assert_eq!(
        restarted
            .unresolved_external_effects()
            .await
            .expect("pending effects must be enumerable"),
        vec![pending]
    );
    let recovered = restarted
        .prepare_backend_request(token, request, event.operator(), None)
        .await
        .expect("restart must recover exact preparation without authorizing another dispatch");
    assert_eq!(recovered, ExternalEffectPreparation::Existing(pending));
    assert!(!recovered.dispatch_authorized());

    let wrong_raw = RawReturn::new(vec![0xff]);
    assert!(matches!(
        restarted
            .complete_external_effect(token, &wrong_raw, &event)
            .await,
        Err(StoreError::ExternalEffectRawReturnMismatch { .. })
    ));
    assert_eq!(
        restarted
            .external_effect_state(token)
            .await
            .expect("failed completion must leave recovery state readable"),
        Some(pending)
    );

    let raw = RawReturn::new(vec![0, 0xff, b'{', 0]);
    let event_ref = restarted
        .complete_external_effect(token, &raw, &event)
        .await
        .expect("raw preservation, event append, and completion must commit atomically");
    assert_eq!(
        restarted
            .complete_external_effect(token, &raw, &event)
            .await
            .expect("exact completion must be idempotent"),
        event_ref
    );
    let completed = restarted
        .external_effect_state(token)
        .await
        .expect("completed state must load")
        .expect("completed token must remain recoverable");
    assert_eq!(completed.completed_event(), Some(event_ref));
    assert!(
        restarted
            .unresolved_external_effects()
            .await
            .expect("completion must clear unresolved recovery work")
            .is_empty()
    );
    assert_eq!(
        restarted
            .get_actual_event(event_ref)
            .await
            .expect("completed event must verify"),
        Some(event)
    );
    restarted
        .verify_event_ledger()
        .await
        .expect("completed ordinary ledger must verify");
    restarted.close().await;

    let replay = ArtifactStore::open(&url)
        .await
        .expect("completed store must reopen");
    replay
        .migrate()
        .await
        .expect("migrations must remain repeatable");
    assert_eq!(
        replay
            .external_effect_state(token)
            .await
            .expect("completion linkage must replay")
            .and_then(ExternalEffectState::completed_event),
        Some(event_ref)
    );
    replay
        .verify_event_ledger()
        .await
        .expect("cold-replayed event ledger must verify");
    replay.close().await;
    std::fs::remove_file(path).expect("temporary effect database must be removable");
}

#[tokio::test]
async fn actual_events_append_in_order_and_recheck_stored_identity() {
    let store = migrated_store().await;
    let first = event_fixture(&store, None, b"backend-first").await;
    let first_ref = store
        .append_actual_event(&first)
        .await
        .expect("first event must append");
    assert_eq!(
        store
            .get_actual_event(first_ref)
            .await
            .expect("event fetch must pass"),
        Some(first)
    );

    let second = event_fixture(&store, Some(first_ref), b"backend-second").await;
    let second_ref = store
        .append_actual_event(&second)
        .await
        .expect("next event must append at the current head");
    assert_eq!(
        store
            .append_actual_event(&second)
            .await
            .expect("identical event append must be idempotent"),
        second_ref
    );
    store
        .verify_event_ledger()
        .await
        .expect("ledger chain must verify");
}

#[tokio::test]
async fn actual_event_append_rejects_stale_parent_and_detects_ledger_corruption() {
    let store = migrated_store().await;
    let first = event_fixture(&store, None, b"backend-first").await;
    let wrong_raw_return = RawReturnRef::from_artifact_ref(stored_ref(&store, b"not-raw").await);
    let wrong_raw_event = ActualEvent::new(
        first.ledger_parent(),
        first.state_before(),
        first.question(),
        first.boundary(),
        first.distinction(),
        first.operator(),
        wrong_raw_return,
        first.state_after(),
        first.grain(),
        first.route(),
        first.binding(),
        first.backend_version(),
        first.provenance(),
    );
    assert!(matches!(
        store.append_actual_event(&wrong_raw_event).await,
        Err(StoreError::RawReturn(
            RawReturnError::UnexpectedArtifactKind { .. }
        ))
    ));
    let wrong_boundary = BoundaryRef::from_artifact_ref(stored_ref(&store, b"not-boundary").await);
    let wrong_boundary_event = ActualEvent::new(
        first.ledger_parent(),
        first.state_before(),
        first.question(),
        wrong_boundary,
        first.distinction(),
        first.operator(),
        first.raw_return(),
        first.state_after(),
        first.grain(),
        first.route(),
        first.binding(),
        first.backend_version(),
        first.provenance(),
    );
    assert!(matches!(
        store.append_actual_event(&wrong_boundary_event).await,
        Err(StoreError::BoundaryChart(
            ic_core::BoundaryChartError::UnexpectedArtifactKind { .. }
        ))
    ));
    let wrong_operator = OperatorRef::from_artifact_ref(stored_ref(&store, b"not-operator").await);
    let wrong_operator_event = ActualEvent::new(
        first.ledger_parent(),
        first.state_before(),
        first.question(),
        first.boundary(),
        first.distinction(),
        wrong_operator,
        first.raw_return(),
        first.state_after(),
        first.grain(),
        first.route(),
        first.binding(),
        first.backend_version(),
        first.provenance(),
    );
    assert!(matches!(
        store.append_actual_event(&wrong_operator_event).await,
        Err(StoreError::ProbeOperator(
            ic_core::ProbeOperatorError::UnexpectedArtifactKind { .. }
        ))
    ));
    let other_field = stored_ref(&store, b"other-question-field").await;
    let other_question = OpenQuery::new(
        RelationRef::from_artifact_ref(other_field),
        Vec::new(),
        Vec::new(),
        RelationUseContext::new(
            ScopeRef::from_artifact_ref(other_field),
            ApplicabilityRef::from_artifact_ref(other_field),
            first.grain(),
            HorizonRef::from_artifact_ref(other_field),
            DischargeMode::Probe,
            SupportRef::from_artifact_ref(other_field),
            None,
        ),
    );
    let other_question = QueryRef::from_artifact_ref(
        store
            .insert(&other_question.envelope().expect("question must encode"))
            .await
            .expect("question must insert"),
    );
    let wrong_question_event = ActualEvent::new(
        first.ledger_parent(),
        first.state_before(),
        other_question,
        first.boundary(),
        first.distinction(),
        first.operator(),
        first.raw_return(),
        first.state_after(),
        first.grain(),
        first.route(),
        first.binding(),
        first.backend_version(),
        first.provenance(),
    );
    assert!(matches!(
        store.append_actual_event(&wrong_question_event).await,
        Err(StoreError::ActualEventCheck(
            ic_core::ActualEventCheckError::BoundaryQuestionMismatch { event, boundary }
        )) if event == other_question && boundary == first.question()
    ));
    let first_ref = store
        .append_actual_event(&first)
        .await
        .expect("first event must append");

    let stale = event_fixture(&store, None, b"backend-stale").await;
    assert!(matches!(
        store.append_actual_event(&stale).await,
        Err(StoreError::LedgerParentMismatch {
            expected: Some(reference),
            actual: None,
        }) if reference == first_ref
    ));

    let second = event_fixture(&store, Some(first_ref), b"backend-second").await;
    let second_ref = store
        .append_actual_event(&second)
        .await
        .expect("second event must append");
    sqlx::query("UPDATE event_ledger SET ledger_parent = NULL WHERE event_ref = ?")
        .bind(second_ref.as_artifact_ref().as_bytes().as_slice())
        .execute(&store.pool)
        .await
        .expect("corrupt fixture update must pass");
    assert!(matches!(
        store.get_actual_event(second_ref).await,
        Err(StoreError::EventLedgerCorrupt(reference)) if reference == second_ref
    ));
}

#[tokio::test]
async fn event_ledger_reopens_and_revalidates_immutable_history_after_restart() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    let path = env::temp_dir().join(format!(
        "inquiry-calculus-restart-{}-{nonce}.sqlite",
        std::process::id()
    ));
    let url = format!("sqlite://{}", path.to_string_lossy().replace('\\', "/"));

    let store = ArtifactStore::open(&url)
        .await
        .expect("file-backed store must open");
    store.migrate().await.expect("migrations must apply");
    let event = event_fixture(&store, None, b"restart-backend").await;
    let event_ref = store
        .append_actual_event(&event)
        .await
        .expect("event must append before restart");
    store.close().await;

    let restarted = ArtifactStore::open(&url)
        .await
        .expect("file-backed store must reopen");
    restarted
        .migrate()
        .await
        .expect("embedded migrations must reapply harmlessly");
    assert_eq!(
        restarted
            .get_actual_event(event_ref)
            .await
            .expect("reopened event must verify"),
        Some(event)
    );
    restarted
        .verify_event_ledger()
        .await
        .expect("reopened ledger must verify");
    restarted.close().await;
    std::fs::remove_file(path).expect("temporary restart database must be removable");
}

#[tokio::test]
async fn insertion_fetch_and_duplicate_insertion_are_exact() {
    let store = migrated_store().await;
    let artifact = envelope(b"canonical payload");

    let first_ref = store.insert(&artifact).await.expect("insert must pass");
    let second_ref = store
        .insert(&artifact)
        .await
        .expect("duplicate insert must pass");
    assert_eq!(first_ref, second_ref);
    assert_eq!(
        store.get(first_ref).await.expect("fetch must pass"),
        Some(artifact)
    );
}

#[tokio::test]
async fn raw_returns_persist_as_opaque_immutable_artifacts_without_decoding() {
    let store = migrated_store().await;
    let raw = RawReturn::new(vec![0, 0xff, b'{', b'"', 0, b'}']);
    let envelope = raw.envelope().expect("raw return must encode");
    let reference = store.insert(&envelope).await.expect("insert must pass");
    let stored = store
        .get(reference)
        .await
        .expect("fetch must pass")
        .expect("raw return must be stored");
    assert_eq!(stored.canonical_payload(), raw.bytes());
    assert_eq!(
        RawReturn::from_envelope(&stored).expect("raw return must remain decodable"),
        raw
    );
}

#[tokio::test]
async fn insertion_rejects_reference_mismatch() {
    let store = migrated_store().await;
    let artifact = envelope(b"canonical payload");
    let wrong_ref = ArtifactRef::from_bytes([0; 32]);

    assert!(matches!(
        store.insert_at(wrong_ref, &artifact).await,
        Err(StoreError::ReferenceMismatch { .. })
    ));
}

#[tokio::test]
async fn referencing_insert_requires_declared_dependencies_before_commit() {
    let store = migrated_store().await;
    let binding_artifact = envelope(b"binding-v1");
    let binding_ref = binding_artifact
        .artifact_ref()
        .expect("binding fixture must hash");
    let type_artifact = TypeArtifact::new(
        BindingVersionRef::from_artifact_ref(binding_ref),
        TyIR::Unit,
    );
    let type_envelope = type_artifact.envelope().expect("type fixture must encode");
    let type_ref = type_envelope
        .artifact_ref()
        .expect("type fixture must hash");

    assert!(matches!(
        store
            .insert_referencing(&type_envelope, &type_artifact.referenced_artifacts())
            .await,
        Err(StoreError::MissingReferencedArtifact(reference)) if reference == binding_ref
    ));
    assert_eq!(
        store.get(type_ref).await.expect("fetch must pass"),
        None,
        "a failed dependency check must not insert the dependent artifact"
    );

    store
        .insert(&binding_artifact)
        .await
        .expect("dependency insert must pass");
    assert_eq!(
        store
            .insert_referencing(&type_envelope, &type_artifact.referenced_artifacts())
            .await
            .expect("dependency-complete type insert must pass"),
        type_ref
    );
    assert_eq!(
        store
            .insert_referencing(&type_envelope, &type_artifact.referenced_artifacts())
            .await
            .expect("duplicate dependency-complete type insert must pass"),
        type_ref
    );
}

#[tokio::test]
async fn insertion_detects_existing_different_bytes() {
    let store = migrated_store().await;
    let artifact = envelope(b"canonical payload");
    let artifact_ref = artifact.artifact_ref().expect("artifact must hash");

    sqlx::query("INSERT INTO artifacts (artifact_ref, canonical_envelope) VALUES (?, ?)")
        .bind(artifact_ref.as_bytes().as_slice())
        .bind(b"different bytes".as_slice())
        .execute(&store.pool)
        .await
        .expect("corrupt fixture insert must pass");

    assert!(matches!(
        store.insert_at(artifact_ref, &artifact).await,
        Err(StoreError::ReferenceConflict(reference)) if reference == artifact_ref
    ));
}

#[tokio::test]
async fn fetch_detects_corrupt_envelope_and_reference() {
    let store = migrated_store().await;
    let artifact = envelope(b"first");
    let artifact_ref = store.insert(&artifact).await.expect("insert must pass");

    sqlx::query("UPDATE artifacts SET canonical_envelope = ? WHERE artifact_ref = ?")
        .bind(b"not an envelope".as_slice())
        .bind(artifact_ref.as_bytes().as_slice())
        .execute(&store.pool)
        .await
        .expect("corrupt fixture update must pass");
    assert!(matches!(
        store.get(artifact_ref).await,
        Err(StoreError::CorruptArtifact { .. })
    ));

    let replacement = envelope(b"second");
    sqlx::query("UPDATE artifacts SET canonical_envelope = ? WHERE artifact_ref = ?")
        .bind(replacement.encode().expect("replacement must encode"))
        .bind(artifact_ref.as_bytes().as_slice())
        .execute(&store.pool)
        .await
        .expect("corrupt fixture update must pass");
    assert!(matches!(
        store.get(artifact_ref).await,
        Err(StoreError::CorruptReference { .. })
    ));
}

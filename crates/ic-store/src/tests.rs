use ic_core::{
    ActualEvent, ArtifactKind, ArtifactRef, BindingVersionRef, BoundaryRef, EventRef, GrainRef,
    OperatorRef, ProvenanceRef, QueryRef, RawReturn, RawReturnError, RawReturnRef, RouteRef,
    StateRef, TyIR, TypeArtifact,
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
    ActualEvent::new(
        ledger_parent,
        StateRef::from_artifact_ref(stored_ref(store, b"state-before").await),
        QueryRef::from_artifact_ref(stored_ref(store, b"question").await),
        BoundaryRef::from_artifact_ref(stored_ref(store, b"boundary").await),
        None,
        OperatorRef::from_artifact_ref(stored_ref(store, b"operator").await),
        ic_core::RawReturnRef::from_artifact_ref(raw_return),
        StateRef::from_artifact_ref(stored_ref(store, b"state-after").await),
        GrainRef::from_artifact_ref(stored_ref(store, b"grain").await),
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

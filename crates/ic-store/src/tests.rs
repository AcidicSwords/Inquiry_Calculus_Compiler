use ic_core::{ArtifactKind, ArtifactRef, BindingVersionRef, TyIR, TypeArtifact};

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

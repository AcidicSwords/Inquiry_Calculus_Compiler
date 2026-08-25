//! Immutable SQLite persistence for canonical artifacts.
//!
//! The store has no event journal or later-phase semantic state. Referencing artifacts
//! are admitted only through an explicit dependency list; opaque payloads are not parsed.

use std::{collections::BTreeSet, str::FromStr};

use ic_core::{ArtifactEnvelope, ArtifactError, ArtifactRef};
use sqlx::{
    SqlitePool,
    migrate::{MigrateError, Migrator},
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use thiserror::Error;

static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

#[cfg(test)]
mod tests;

/// A single-writer SQLite store for immutable artifact envelopes.
#[derive(Debug, Clone)]
pub struct ArtifactStore {
    pool: SqlitePool,
}

impl ArtifactStore {
    /// Opens a SQLite URL using one authoritative connection.
    pub async fn open(database_url: &str) -> Result<Self, StoreError> {
        let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;
        Ok(Self { pool })
    }

    /// Applies all embedded schema migrations.
    #[tracing::instrument(skip(self))]
    pub async fn migrate(&self) -> Result<(), StoreError> {
        MIGRATOR.run(&self.pool).await?;
        Ok(())
    }

    /// Inserts an envelope under its calculated content reference.
    #[tracing::instrument(skip(self, envelope))]
    pub async fn insert(&self, envelope: &ArtifactEnvelope) -> Result<ArtifactRef, StoreError> {
        let artifact_ref = envelope.artifact_ref()?;
        self.insert_at(artifact_ref, envelope).await?;
        Ok(artifact_ref)
    }

    /// Inserts an envelope after confirming that all declared dependencies already exist.
    ///
    /// References are supplied by the typed artifact constructor, never inferred from an
    /// opaque payload. The presence checks and insert share one transaction, so a failed
    /// dependency check cannot leave a partially inserted referencing artifact behind.
    #[tracing::instrument(skip(self, envelope, references))]
    pub async fn insert_referencing(
        &self,
        envelope: &ArtifactEnvelope,
        references: &[ArtifactRef],
    ) -> Result<ArtifactRef, StoreError> {
        let artifact_ref = envelope.artifact_ref()?;
        self.insert_at_referencing(artifact_ref, envelope, references)
            .await?;
        Ok(artifact_ref)
    }

    /// Inserts an envelope only if its calculated identity matches `expected`.
    ///
    /// Repeating an identical insertion is a no-op. Existing different bytes under the
    /// same reference are reported instead of overwritten.
    #[tracing::instrument(skip(self, envelope), fields(artifact_ref = %expected))]
    pub async fn insert_at(
        &self,
        expected: ArtifactRef,
        envelope: &ArtifactEnvelope,
    ) -> Result<(), StoreError> {
        self.insert_at_referencing(expected, envelope, &[]).await
    }

    /// Inserts an envelope at `expected` after checking its explicit dependencies.
    #[tracing::instrument(skip(self, envelope, references), fields(artifact_ref = %expected))]
    pub async fn insert_at_referencing(
        &self,
        expected: ArtifactRef,
        envelope: &ArtifactEnvelope,
        references: &[ArtifactRef],
    ) -> Result<(), StoreError> {
        let calculated = envelope.artifact_ref()?;
        if expected != calculated {
            return Err(StoreError::ReferenceMismatch {
                expected,
                calculated,
            });
        }

        let encoded = envelope.encode()?;
        let mut transaction = self.pool.begin().await?;
        let unique_references: BTreeSet<_> = references.iter().copied().collect();
        for reference in unique_references {
            let exists: Option<i64> =
                sqlx::query_scalar("SELECT 1 FROM artifacts WHERE artifact_ref = ?")
                    .bind(reference.as_bytes().as_slice())
                    .fetch_optional(&mut *transaction)
                    .await?;
            if exists.is_none() {
                return Err(StoreError::MissingReferencedArtifact(reference));
            }
        }

        sqlx::query(
            "INSERT OR IGNORE INTO artifacts (artifact_ref, canonical_envelope) VALUES (?, ?)",
        )
        .bind(expected.as_bytes().as_slice())
        .bind(&encoded)
        .execute(&mut *transaction)
        .await?;

        let stored: Vec<u8> =
            sqlx::query_scalar("SELECT canonical_envelope FROM artifacts WHERE artifact_ref = ?")
                .bind(expected.as_bytes().as_slice())
                .fetch_one(&mut *transaction)
                .await?;

        if stored != encoded {
            return Err(StoreError::ReferenceConflict(expected));
        }

        transaction.commit().await?;
        Ok(())
    }

    /// Fetches an envelope and verifies both its encoding and its stored identity.
    #[tracing::instrument(skip(self), fields(artifact_ref = %artifact_ref))]
    pub async fn get(
        &self,
        artifact_ref: ArtifactRef,
    ) -> Result<Option<ArtifactEnvelope>, StoreError> {
        let stored: Option<Vec<u8>> =
            sqlx::query_scalar("SELECT canonical_envelope FROM artifacts WHERE artifact_ref = ?")
                .bind(artifact_ref.as_bytes().as_slice())
                .fetch_optional(&self.pool)
                .await?;

        let Some(stored) = stored else {
            return Ok(None);
        };

        let envelope =
            ArtifactEnvelope::decode(&stored).map_err(|source| StoreError::CorruptArtifact {
                artifact_ref,
                source,
            })?;
        let calculated = envelope.artifact_ref()?;
        if calculated != artifact_ref {
            return Err(StoreError::CorruptReference {
                stored: artifact_ref,
                calculated,
            });
        }

        Ok(Some(envelope))
    }
}

/// Persistence errors that preserve content-identity failures.
#[derive(Debug, Error)]
pub enum StoreError {
    #[error("SQLite operation failed")]
    Sqlx(#[from] sqlx::Error),

    #[error("schema migration failed")]
    Migration(#[from] MigrateError),

    #[error("artifact envelope failed validation")]
    Artifact(#[from] ArtifactError),

    #[error("expected artifact reference {expected}, but envelope calculated {calculated}")]
    ReferenceMismatch {
        expected: ArtifactRef,
        calculated: ArtifactRef,
    },

    #[error("different bytes already exist under artifact reference {0}")]
    ReferenceConflict(ArtifactRef),

    #[error("referenced artifact {0} must be present before inserting the dependent artifact")]
    MissingReferencedArtifact(ArtifactRef),

    #[error("stored envelope for {artifact_ref} is corrupt")]
    CorruptArtifact {
        artifact_ref: ArtifactRef,
        #[source]
        source: ArtifactError,
    },

    #[error("stored reference {stored} does not match calculated reference {calculated}")]
    CorruptReference {
        stored: ArtifactRef,
        calculated: ArtifactRef,
    },
}

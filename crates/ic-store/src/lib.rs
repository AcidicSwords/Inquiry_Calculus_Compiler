//! Immutable SQLite persistence for canonical artifacts.
//!
//! The store preserves immutable artifacts and one append-only ordinary event ledger. Referencing
//! artifacts are admitted only through an explicit dependency list; opaque payloads are not
//! parsed for dependency discovery.

use std::{collections::BTreeSet, str::FromStr};

use ic_core::{
    ActualEvent, ActualEventCheckError, ActualEventError, ArtifactEnvelope, ArtifactError,
    ArtifactRef, BoundaryChart, BoundaryChartError, BoundaryRef, EventRef, OpenQuery,
    OpenQueryError, ProbeOperator, ProbeOperatorError, ProbeOperatorRef, QueryRef, RawReturn,
    RawReturnError, RawReturnRef, check_event_context,
};
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

/// Opaque idempotency identity for one operational external-effect preparation.
///
/// This is not a semantic artifact reference or an assertion that dispatch occurred.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DispatchToken(ArtifactRef);

impl DispatchToken {
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(ArtifactRef::from_bytes(bytes))
    }

    #[must_use]
    pub const fn as_artifact_ref(self) -> ArtifactRef {
        self.0
    }
}

/// Operational recovery state for one prepared external effect.
///
/// A pending entry says only that durable intent preceded any authorized dispatch. After a crash,
/// it remains `Pending`/unknown and must never be retried automatically. A completed entry points
/// to the ordinary authoritative event; it is not a second semantic history.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExternalEffectState {
    Pending {
        token: DispatchToken,
        request: ArtifactRef,
        operator: ProbeOperatorRef,
        ledger_parent: Option<EventRef>,
    },
    Completed {
        token: DispatchToken,
        request: ArtifactRef,
        operator: ProbeOperatorRef,
        ledger_parent: Option<EventRef>,
        event: EventRef,
    },
}

impl ExternalEffectState {
    #[must_use]
    pub const fn token(self) -> DispatchToken {
        match self {
            Self::Pending { token, .. } | Self::Completed { token, .. } => token,
        }
    }

    #[must_use]
    pub const fn request(self) -> ArtifactRef {
        match self {
            Self::Pending { request, .. } | Self::Completed { request, .. } => request,
        }
    }

    #[must_use]
    pub const fn operator(self) -> ProbeOperatorRef {
        match self {
            Self::Pending { operator, .. } | Self::Completed { operator, .. } => operator,
        }
    }

    #[must_use]
    pub const fn ledger_parent(self) -> Option<EventRef> {
        match self {
            Self::Pending { ledger_parent, .. } | Self::Completed { ledger_parent, .. } => {
                ledger_parent
            }
        }
    }

    #[must_use]
    pub const fn completed_event(self) -> Option<EventRef> {
        match self {
            Self::Pending { .. } => None,
            Self::Completed { event, .. } => Some(event),
        }
    }
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

    /// Durably records external-effect intent before a caller is allowed to dispatch.
    ///
    /// The request and compiled operator must already exist as verified immutable artifacts, and
    /// the expected ledger parent must still be the current head. Only one unresolved preparation
    /// is allowed for the single writer. Repeating the exact same preparation is idempotent.
    #[tracing::instrument(skip(self), fields(dispatch_token = %token.as_artifact_ref()))]
    pub async fn prepare_external_effect(
        &self,
        token: DispatchToken,
        request: ArtifactRef,
        operator: ProbeOperatorRef,
        ledger_parent: Option<EventRef>,
    ) -> Result<ExternalEffectState, StoreError> {
        self.get(request)
            .await?
            .ok_or(StoreError::MissingReferencedArtifact(request))?;
        self.verify_probe_operator(operator).await?;
        let mut transaction = self.pool.begin().await?;

        let existing: Option<ExternalEffectRow> = sqlx::query_as(
            "SELECT request_ref, operator_ref, ledger_parent, completed_event \
             FROM external_effect_journal WHERE dispatch_token = ?",
        )
        .bind(token.as_artifact_ref().as_bytes().as_slice())
        .fetch_optional(&mut *transaction)
        .await?;
        if let Some(row) = existing {
            let state = parse_external_effect_row(token, row)?;
            if state.request() != request
                || state.operator() != operator
                || state.ledger_parent() != ledger_parent
            {
                return Err(StoreError::DispatchTokenConflict(token));
            }
            transaction.commit().await?;
            return Ok(state);
        }

        let unresolved: Option<Vec<u8>> = sqlx::query_scalar(
            "SELECT dispatch_token FROM external_effect_journal \
             WHERE completed_event IS NULL LIMIT 1",
        )
        .fetch_optional(&mut *transaction)
        .await?;
        if let Some(unresolved) = unresolved {
            return Err(StoreError::ExternalEffectAlreadyPending(DispatchToken(
                parse_artifact_ref(unresolved)?,
            )));
        }

        let current_head: Option<Vec<u8>> = sqlx::query_scalar(
            "SELECT event_ref FROM event_ledger ORDER BY ledger_sequence DESC LIMIT 1",
        )
        .fetch_optional(&mut *transaction)
        .await?;
        let current_head = parse_optional_event_ref(current_head)?;
        if ledger_parent != current_head {
            return Err(StoreError::LedgerParentMismatch {
                expected: current_head,
                actual: ledger_parent,
            });
        }

        sqlx::query(
            "INSERT INTO external_effect_journal \
             (dispatch_token, request_ref, operator_ref, ledger_parent, completed_event) \
             VALUES (?, ?, ?, ?, NULL)",
        )
        .bind(token.as_artifact_ref().as_bytes().as_slice())
        .bind(request.as_bytes().as_slice())
        .bind(operator.as_artifact_ref().as_bytes().as_slice())
        .bind(ledger_parent.map(|parent| parent.as_artifact_ref().as_bytes().to_vec()))
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        Ok(ExternalEffectState::Pending {
            token,
            request,
            operator,
            ledger_parent,
        })
    }

    /// Returns the operational recovery state for one dispatch token.
    pub async fn external_effect_state(
        &self,
        token: DispatchToken,
    ) -> Result<Option<ExternalEffectState>, StoreError> {
        let row: Option<ExternalEffectRow> = sqlx::query_as(
            "SELECT request_ref, operator_ref, ledger_parent, completed_event \
             FROM external_effect_journal WHERE dispatch_token = ?",
        )
        .bind(token.as_artifact_ref().as_bytes().as_slice())
        .fetch_optional(&self.pool)
        .await?;
        row.map(|row| parse_external_effect_row(token, row))
            .transpose()
    }

    /// Lists unresolved preparations. Each is `Unknown` with respect to whether dispatch occurred.
    pub async fn unresolved_external_effects(
        &self,
    ) -> Result<Vec<ExternalEffectState>, StoreError> {
        let rows: Vec<UnresolvedExternalEffectRow> = sqlx::query_as(
            "SELECT dispatch_token, request_ref, operator_ref, ledger_parent \
             FROM external_effect_journal WHERE completed_event IS NULL ORDER BY dispatch_token",
        )
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|(token, request, operator, ledger_parent)| {
                let token = DispatchToken(parse_artifact_ref(token)?);
                parse_external_effect_row(token, (request, operator, ledger_parent, None))
            })
            .collect()
    }

    /// Atomically preserves a raw return, appends its checked ordinary event, and completes a
    /// previously durable preparation.
    ///
    /// No decoding occurs here. If the transaction fails or the process crashes, SQLite exposes
    /// either the unresolved preparation or the complete raw/event linkage, never a claimed event
    /// without its immutable raw return.
    #[tracing::instrument(skip(self, raw_return, event), fields(dispatch_token = %token.as_artifact_ref()))]
    pub async fn complete_external_effect(
        &self,
        token: DispatchToken,
        raw_return: &RawReturn,
        event: &ActualEvent,
    ) -> Result<EventRef, StoreError> {
        let raw_envelope = raw_return.envelope()?;
        let raw_ref = raw_return.raw_return_ref()?;
        if event.raw_return() != raw_ref {
            return Err(StoreError::ExternalEffectRawReturnMismatch {
                event: event.raw_return(),
                supplied: raw_ref,
            });
        }
        let event_ref = event.event_ref()?;
        let event_envelope = event.envelope()?;
        let event_encoded = event_envelope.encode()?;
        let question = self.verify_open_query(event.question()).await?;
        let chart = self.verify_boundary_chart(event.boundary()).await?;
        let operator = self.verify_probe_operator(event.operator()).await?;
        check_event_context(event, &question, &chart, &operator)?;

        let mut transaction = self.pool.begin().await?;
        let row: Option<ExternalEffectRow> = sqlx::query_as(
            "SELECT request_ref, operator_ref, ledger_parent, completed_event \
             FROM external_effect_journal WHERE dispatch_token = ?",
        )
        .bind(token.as_artifact_ref().as_bytes().as_slice())
        .fetch_optional(&mut *transaction)
        .await?;
        let state =
            parse_external_effect_row(token, row.ok_or(StoreError::UnknownDispatchToken(token))?)?;
        if state.operator() != event.operator() {
            return Err(StoreError::ExternalEffectOperatorMismatch {
                prepared: state.operator(),
                event: event.operator(),
            });
        }
        if state.ledger_parent() != event.ledger_parent() {
            return Err(StoreError::ExternalEffectParentMismatch {
                prepared: state.ledger_parent(),
                event: event.ledger_parent(),
            });
        }
        if let ExternalEffectState::Completed {
            event: completed, ..
        } = state
        {
            if completed == event_ref {
                transaction.commit().await?;
                return Ok(event_ref);
            }
            return Err(StoreError::ExternalEffectAlreadyCompleted {
                token,
                event: completed,
            });
        }

        insert_encoded_artifact(
            &mut transaction,
            raw_ref.as_artifact_ref(),
            &raw_envelope.encode()?,
        )
        .await?;

        let unique_references: BTreeSet<_> = event.referenced_artifacts().into_iter().collect();
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
        insert_encoded_artifact(
            &mut transaction,
            event_ref.as_artifact_ref(),
            &event_encoded,
        )
        .await?;

        let current_head: Option<Vec<u8>> = sqlx::query_scalar(
            "SELECT event_ref FROM event_ledger ORDER BY ledger_sequence DESC LIMIT 1",
        )
        .fetch_optional(&mut *transaction)
        .await?;
        let current_head = parse_optional_event_ref(current_head)?;
        if event.ledger_parent() != current_head {
            return Err(StoreError::LedgerParentMismatch {
                expected: current_head,
                actual: event.ledger_parent(),
            });
        }
        sqlx::query("INSERT INTO event_ledger (event_ref, ledger_parent) VALUES (?, ?)")
            .bind(event_ref.as_artifact_ref().as_bytes().as_slice())
            .bind(
                event
                    .ledger_parent()
                    .map(|parent| parent.as_artifact_ref().as_bytes().to_vec()),
            )
            .execute(&mut *transaction)
            .await?;
        let updated = sqlx::query(
            "UPDATE external_effect_journal SET completed_event = ? \
             WHERE dispatch_token = ? AND completed_event IS NULL",
        )
        .bind(event_ref.as_artifact_ref().as_bytes().as_slice())
        .bind(token.as_artifact_ref().as_bytes().as_slice())
        .execute(&mut *transaction)
        .await?;
        if updated.rows_affected() != 1 {
            return Err(StoreError::DispatchTokenConflict(token));
        }
        transaction.commit().await?;
        Ok(event_ref)
    }

    /// Closes the single authoritative SQLite connection before a restart or controlled handoff.
    pub async fn close(self) {
        self.pool.close().await;
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

    /// Appends one already-realized event to the ordinary authoritative event ledger.
    ///
    /// This method records rather than dispatches: callers must preserve the raw return before
    /// invoking it.  The event envelope and ledger row are inserted in one transaction.  The
    /// declared parent must equal the current head, so a stale writer cannot fork the ledger.
    #[tracing::instrument(skip(self, event))]
    pub async fn append_actual_event(&self, event: &ActualEvent) -> Result<EventRef, StoreError> {
        let event_ref = event.event_ref()?;
        let envelope = event.envelope()?;
        let encoded = envelope.encode()?;
        self.verify_raw_return(event.raw_return()).await?;
        let question = self.verify_open_query(event.question()).await?;
        let chart = self.verify_boundary_chart(event.boundary()).await?;
        let operator = self.verify_probe_operator(event.operator()).await?;
        check_event_context(event, &question, &chart, &operator)?;
        let mut transaction = self.pool.begin().await?;

        let unique_references: BTreeSet<_> = event.referenced_artifacts().into_iter().collect();
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
        .bind(event_ref.as_artifact_ref().as_bytes().as_slice())
        .bind(&encoded)
        .execute(&mut *transaction)
        .await?;
        let stored: Vec<u8> =
            sqlx::query_scalar("SELECT canonical_envelope FROM artifacts WHERE artifact_ref = ?")
                .bind(event_ref.as_artifact_ref().as_bytes().as_slice())
                .fetch_one(&mut *transaction)
                .await?;
        if stored != encoded {
            return Err(StoreError::ReferenceConflict(event_ref.as_artifact_ref()));
        }

        let existing_parent: Option<Option<Vec<u8>>> =
            sqlx::query_scalar("SELECT ledger_parent FROM event_ledger WHERE event_ref = ?")
                .bind(event_ref.as_artifact_ref().as_bytes().as_slice())
                .fetch_optional(&mut *transaction)
                .await?;
        if let Some(existing_parent) = existing_parent {
            let existing_parent = parse_optional_event_ref(existing_parent)?;
            if existing_parent != event.ledger_parent() {
                return Err(StoreError::EventLedgerCorrupt(event_ref));
            }
            transaction.commit().await?;
            return Ok(event_ref);
        }

        let current_head: Option<Vec<u8>> = sqlx::query_scalar(
            "SELECT event_ref FROM event_ledger ORDER BY ledger_sequence DESC LIMIT 1",
        )
        .fetch_optional(&mut *transaction)
        .await?;
        let current_head = parse_optional_event_ref(current_head)?;
        if event.ledger_parent() != current_head {
            return Err(StoreError::LedgerParentMismatch {
                expected: current_head,
                actual: event.ledger_parent(),
            });
        }

        sqlx::query("INSERT INTO event_ledger (event_ref, ledger_parent) VALUES (?, ?)")
            .bind(event_ref.as_artifact_ref().as_bytes().as_slice())
            .bind(
                event
                    .ledger_parent()
                    .map(|parent| parent.as_artifact_ref().as_bytes().to_vec()),
            )
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;
        Ok(event_ref)
    }

    /// Fetches an event only when the ledger row, parent linkage, and content identity agree.
    #[tracing::instrument(skip(self), fields(event_ref = %event_ref))]
    pub async fn get_actual_event(
        &self,
        event_ref: EventRef,
    ) -> Result<Option<ActualEvent>, StoreError> {
        let parent: Option<Option<Vec<u8>>> =
            sqlx::query_scalar("SELECT ledger_parent FROM event_ledger WHERE event_ref = ?")
                .bind(event_ref.as_artifact_ref().as_bytes().as_slice())
                .fetch_optional(&self.pool)
                .await?;
        let Some(parent) = parent else {
            return Ok(None);
        };
        let parent = parse_optional_event_ref(parent)?;
        let envelope = self
            .get(event_ref.as_artifact_ref())
            .await?
            .ok_or(StoreError::EventLedgerCorrupt(event_ref))?;
        let event = ActualEvent::from_envelope(&envelope)?;
        if event.event_ref()? != event_ref || event.ledger_parent() != parent {
            return Err(StoreError::EventLedgerCorrupt(event_ref));
        }
        self.verify_raw_return(event.raw_return()).await?;
        let question = self.verify_open_query(event.question()).await?;
        let chart = self.verify_boundary_chart(event.boundary()).await?;
        let operator = self.verify_probe_operator(event.operator()).await?;
        check_event_context(&event, &question, &chart, &operator)?;
        Ok(Some(event))
    }

    /// Rechecks every ordered ledger edge and its stored canonical event envelope.
    #[tracing::instrument(skip(self))]
    pub async fn verify_event_ledger(&self) -> Result<(), StoreError> {
        let rows: Vec<(Vec<u8>, Option<Vec<u8>>)> = sqlx::query_as(
            "SELECT event_ref, ledger_parent FROM event_ledger ORDER BY ledger_sequence",
        )
        .fetch_all(&self.pool)
        .await?;
        let mut previous = None;
        for (event_ref, parent) in rows {
            let event_ref = EventRef::from_artifact_ref(parse_artifact_ref(event_ref)?);
            let parent = parse_optional_event_ref(parent)?;
            if parent != previous {
                return Err(StoreError::EventLedgerCorrupt(event_ref));
            }
            let Some(event) = self.get_actual_event(event_ref).await? else {
                return Err(StoreError::EventLedgerCorrupt(event_ref));
            };
            if event.ledger_parent() != previous {
                return Err(StoreError::EventLedgerCorrupt(event_ref));
            }
            previous = Some(event_ref);
        }
        Ok(())
    }

    async fn verify_raw_return(&self, raw_return: RawReturnRef) -> Result<(), StoreError> {
        let envelope = self.get(raw_return.as_artifact_ref()).await?.ok_or(
            StoreError::MissingReferencedArtifact(raw_return.as_artifact_ref()),
        )?;
        let raw_return_value = RawReturn::from_envelope(&envelope)?;
        let calculated = raw_return_value.raw_return_ref()?;
        if calculated != raw_return {
            return Err(StoreError::CorruptReference {
                stored: raw_return.as_artifact_ref(),
                calculated: calculated.as_artifact_ref(),
            });
        }
        Ok(())
    }

    async fn verify_open_query(&self, question: QueryRef) -> Result<OpenQuery, StoreError> {
        let envelope = self.get(question.as_artifact_ref()).await?.ok_or(
            StoreError::MissingReferencedArtifact(question.as_artifact_ref()),
        )?;
        let query = OpenQuery::from_envelope(&envelope)?;
        let calculated = query.query_ref()?;
        if calculated != question {
            return Err(StoreError::CorruptReference {
                stored: question.as_artifact_ref(),
                calculated: calculated.as_artifact_ref(),
            });
        }
        Ok(query)
    }

    async fn verify_boundary_chart(
        &self,
        boundary: BoundaryRef,
    ) -> Result<BoundaryChart, StoreError> {
        let envelope = self.get(boundary.as_artifact_ref()).await?.ok_or(
            StoreError::MissingReferencedArtifact(boundary.as_artifact_ref()),
        )?;
        let chart = BoundaryChart::from_envelope(&envelope)?;
        let calculated = chart.boundary_ref()?;
        if calculated != boundary {
            return Err(StoreError::CorruptReference {
                stored: boundary.as_artifact_ref(),
                calculated: calculated.as_artifact_ref(),
            });
        }
        Ok(chart)
    }

    async fn verify_probe_operator(
        &self,
        operator: ProbeOperatorRef,
    ) -> Result<ProbeOperator, StoreError> {
        let envelope = self.get(operator.as_artifact_ref()).await?.ok_or(
            StoreError::MissingReferencedArtifact(operator.as_artifact_ref()),
        )?;
        let operator_value = ProbeOperator::from_envelope(&envelope)?;
        let calculated = operator_value.probe_operator_ref()?;
        if calculated != operator {
            return Err(StoreError::CorruptReference {
                stored: operator.as_artifact_ref(),
                calculated: calculated.as_artifact_ref(),
            });
        }
        Ok(operator_value)
    }
}

fn parse_artifact_ref(bytes: Vec<u8>) -> Result<ArtifactRef, StoreError> {
    let bytes: [u8; 32] = bytes
        .try_into()
        .map_err(|_| StoreError::InvalidLedgerReference)?;
    Ok(ArtifactRef::from_bytes(bytes))
}

type ExternalEffectRow = (Vec<u8>, Vec<u8>, Option<Vec<u8>>, Option<Vec<u8>>);
type UnresolvedExternalEffectRow = (Vec<u8>, Vec<u8>, Vec<u8>, Option<Vec<u8>>);

fn parse_external_effect_row(
    token: DispatchToken,
    (request, operator, ledger_parent, completed_event): ExternalEffectRow,
) -> Result<ExternalEffectState, StoreError> {
    let request = parse_artifact_ref(request)?;
    let operator = ProbeOperatorRef::from_artifact_ref(parse_artifact_ref(operator)?);
    let ledger_parent = parse_optional_event_ref(ledger_parent)?;
    let completed_event = parse_optional_event_ref(completed_event)?;
    Ok(match completed_event {
        Some(event) => ExternalEffectState::Completed {
            token,
            request,
            operator,
            ledger_parent,
            event,
        },
        None => ExternalEffectState::Pending {
            token,
            request,
            operator,
            ledger_parent,
        },
    })
}

async fn insert_encoded_artifact(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    reference: ArtifactRef,
    encoded: &[u8],
) -> Result<(), StoreError> {
    sqlx::query("INSERT OR IGNORE INTO artifacts (artifact_ref, canonical_envelope) VALUES (?, ?)")
        .bind(reference.as_bytes().as_slice())
        .bind(encoded)
        .execute(&mut **transaction)
        .await?;
    let stored: Vec<u8> =
        sqlx::query_scalar("SELECT canonical_envelope FROM artifacts WHERE artifact_ref = ?")
            .bind(reference.as_bytes().as_slice())
            .fetch_one(&mut **transaction)
            .await?;
    if stored != encoded {
        return Err(StoreError::ReferenceConflict(reference));
    }
    Ok(())
}

fn parse_optional_event_ref(bytes: Option<Vec<u8>>) -> Result<Option<EventRef>, StoreError> {
    bytes
        .map(parse_artifact_ref)
        .map(|reference| reference.map(EventRef::from_artifact_ref))
        .transpose()
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

    #[error("actual-event encoding failed")]
    ActualEvent(#[from] ActualEventError),

    #[error("actual-event identity context failed")]
    ActualEventCheck(#[from] ActualEventCheckError),

    #[error("raw-return encoding failed")]
    RawReturn(#[from] RawReturnError),

    #[error("boundary-chart encoding failed")]
    BoundaryChart(#[from] BoundaryChartError),

    #[error("probe-operator encoding failed")]
    ProbeOperator(#[from] ProbeOperatorError),

    #[error("open-query encoding failed")]
    OpenQuery(#[from] OpenQueryError),

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

    #[error("event ledger parent mismatch: expected {expected:?}, got {actual:?}")]
    LedgerParentMismatch {
        expected: Option<EventRef>,
        actual: Option<EventRef>,
    },

    #[error("event ledger record for {0} is corrupt")]
    EventLedgerCorrupt(EventRef),

    #[error("event ledger contains a reference that is not 32 bytes")]
    InvalidLedgerReference,

    #[error("dispatch token {0:?} is already bound to different preparation data")]
    DispatchTokenConflict(DispatchToken),

    #[error("external effect {0:?} is still unresolved; it must not be retried automatically")]
    ExternalEffectAlreadyPending(DispatchToken),

    #[error("dispatch token {0:?} is unknown")]
    UnknownDispatchToken(DispatchToken),

    #[error("prepared effect {token:?} already completed as event {event}")]
    ExternalEffectAlreadyCompleted {
        token: DispatchToken,
        event: EventRef,
    },

    #[error("prepared operator {prepared} differs from completed event operator {event}")]
    ExternalEffectOperatorMismatch {
        prepared: ProbeOperatorRef,
        event: ProbeOperatorRef,
    },

    #[error("prepared ledger parent {prepared:?} differs from event parent {event:?}")]
    ExternalEffectParentMismatch {
        prepared: Option<EventRef>,
        event: Option<EventRef>,
    },

    #[error("event names raw return {event}, but completion supplied {supplied}")]
    ExternalEffectRawReturnMismatch {
        event: RawReturnRef,
        supplied: RawReturnRef,
    },
}

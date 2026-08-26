//! Finite, declared binding-bridge checks.
//!
//! A bridge records only the question transports explicitly supplied to it. Conservativity over
//! the whole question universe remains a separate theorem/evidence obligation.

use std::collections::{BTreeMap, BTreeSet};

use thiserror::Error;

use crate::{
    BindingVersionRef, OpenQueryCatalog, OpenQueryCheckError, OpenQueryError, QueryRef,
    RelationError,
};

/// The declared semantic role of a finite binding bridge.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BindingChangeKind {
    DefinitionalExtension,
    ConservativeObservationalExtension,
    Rebinding,
}

/// An explicit finite transport of named old questions into a target binding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BindingBridgeIR {
    source: BindingVersionRef,
    target: BindingVersionRef,
    kind: BindingChangeKind,
    transports: BTreeMap<QueryRef, QueryRef>,
    strict_growth_witness: Option<QueryRef>,
}

impl BindingBridgeIR {
    pub fn new(
        source: BindingVersionRef,
        target: BindingVersionRef,
        kind: BindingChangeKind,
        transports: Vec<(QueryRef, QueryRef)>,
        strict_growth_witness: Option<QueryRef>,
    ) -> Result<Self, BindingBridgeError> {
        let mut map = BTreeMap::new();
        let mut targets = BTreeSet::new();
        for (old, new) in transports {
            if map.insert(old, new).is_some() {
                return Err(BindingBridgeError::DuplicateSourceQuestion(old));
            }
            if !targets.insert(new) {
                return Err(BindingBridgeError::NonInjectiveTargetQuestion(new));
            }
        }
        if kind != BindingChangeKind::ConservativeObservationalExtension
            && strict_growth_witness.is_some()
        {
            return Err(BindingBridgeError::StrictGrowthRequiresConservativeExtension);
        }
        if let Some(witness) = strict_growth_witness
            && targets.contains(&witness)
        {
            return Err(BindingBridgeError::GrowthWitnessIsInTransportImage(witness));
        }
        Ok(Self {
            source,
            target,
            kind,
            transports: map,
            strict_growth_witness,
        })
    }

    #[must_use]
    pub const fn source(&self) -> BindingVersionRef {
        self.source
    }
    #[must_use]
    pub const fn target(&self) -> BindingVersionRef {
        self.target
    }
    #[must_use]
    pub const fn kind(&self) -> BindingChangeKind {
        self.kind
    }
    #[must_use]
    pub const fn transports(&self) -> &BTreeMap<QueryRef, QueryRef> {
        &self.transports
    }
    #[must_use]
    pub const fn strict_growth_witness(&self) -> Option<QueryRef> {
        self.strict_growth_witness
    }

    /// Rechecks every named question and verifies source/target relation-schema binding.
    pub fn check<C: OpenQueryCatalog>(&self, catalog: &C) -> Result<(), BindingBridgeCheckError> {
        for (old_ref, new_ref) in &self.transports {
            let old = checked_query(catalog, *old_ref)?;
            let new = checked_query(catalog, *new_ref)?;
            let old_schema = catalog
                .resolve_relation_schema(old.relation())
                .ok_or(BindingBridgeCheckError::UnresolvedRelation(old.relation()))?;
            let new_schema = catalog
                .resolve_relation_schema(new.relation())
                .ok_or(BindingBridgeCheckError::UnresolvedRelation(new.relation()))?;
            if old_schema.binding() != self.source {
                return Err(BindingBridgeCheckError::SourceBindingMismatch {
                    question: *old_ref,
                    expected: self.source,
                    actual: old_schema.binding(),
                });
            }
            if new_schema.binding() != self.target {
                return Err(BindingBridgeCheckError::TargetBindingMismatch {
                    question: *new_ref,
                    expected: self.target,
                    actual: new_schema.binding(),
                });
            }
        }
        if let Some(witness_ref) = self.strict_growth_witness {
            let witness = checked_query(catalog, witness_ref)?;
            let schema = catalog.resolve_relation_schema(witness.relation()).ok_or(
                BindingBridgeCheckError::UnresolvedRelation(witness.relation()),
            )?;
            if schema.binding() != self.target {
                return Err(BindingBridgeCheckError::TargetBindingMismatch {
                    question: witness_ref,
                    expected: self.target,
                    actual: schema.binding(),
                });
            }
        }
        Ok(())
    }
}

fn checked_query<C: OpenQueryCatalog>(
    catalog: &C,
    reference: QueryRef,
) -> Result<crate::OpenQuery, BindingBridgeCheckError> {
    let query = catalog
        .resolve_open_query(reference)
        .ok_or(BindingBridgeCheckError::UnresolvedQuestion(reference))?;
    let calculated = query.query_ref()?;
    if calculated != reference {
        return Err(BindingBridgeCheckError::QuestionIdentityMismatch {
            reference,
            calculated,
        });
    }
    query.check(catalog)?;
    Ok(query)
}

#[derive(Debug, Error)]
pub enum BindingBridgeError {
    #[error("binding bridge repeats source question {0}")]
    DuplicateSourceQuestion(QueryRef),
    #[error("binding bridge is not injective at target question {0}")]
    NonInjectiveTargetQuestion(QueryRef),
    #[error("a strict-growth witness requires conservative observational extension")]
    StrictGrowthRequiresConservativeExtension,
    #[error("strict-growth witness {0} is already in the declared transport image")]
    GrowthWitnessIsInTransportImage(QueryRef),
}

#[derive(Debug, Error)]
pub enum BindingBridgeCheckError {
    #[error(transparent)]
    Query(#[from] OpenQueryError),
    #[error(transparent)]
    QueryCheck(#[from] OpenQueryCheckError),
    #[error(transparent)]
    Relation(#[from] RelationError),
    #[error("question {0} is unavailable")]
    UnresolvedQuestion(QueryRef),
    #[error("question {reference} hashes to {calculated}, not its claimed identity")]
    QuestionIdentityMismatch {
        reference: QueryRef,
        calculated: QueryRef,
    },
    #[error("relation schema {0} is unavailable")]
    UnresolvedRelation(crate::RelationRef),
    #[error("source question {question} has binding {actual}, expected {expected}")]
    SourceBindingMismatch {
        question: QueryRef,
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },
    #[error("target question {question} has binding {actual}, expected {expected}")]
    TargetBindingMismatch {
        question: QueryRef,
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },
}

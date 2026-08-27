//! Derived, occurrence-indexed successor questions for first-order inquiry programs.
//!
//! Dynamic succession is not a runtime transition or a new event history.  It is a checked view
//! over a content-addressed source configuration, one structural `Ask` occurrence, and the whole
//! supported answer that reaches its named continuation.

use std::{collections::BTreeSet, fmt, str::FromStr};

use thiserror::Error;

use crate::{
    AdmittedFiniteAnswerSet, ArtifactEnvelope, ArtifactError, ArtifactKind, ArtifactRef,
    BindingVersionRef, BoundFiniteAskContinuation, IProgArtifact, IProgCatalog, IProgCheckError,
    IProgError, IProgIR, IProgRef, ProgramBinding, ProvenanceRef, QueryRef, TypeCheckError,
    TypeError, TypeRef, TypeSymbol, TypedFormRef,
};

/// Canonical artifact kind for checked source-program configurations.
pub const SOURCE_CONFIG_ARTIFACT_KIND: &str = "ic.source-config";
/// Payload schema version for checked source-program configurations.
pub const SOURCE_CONFIG_SCHEMA_VERSION: u32 = 1;
/// Canonical artifact kind for checked `Ask` occurrences.
pub const ASK_OCCURRENCE_ARTIFACT_KIND: &str = "ic.ask-occurrence";
/// Payload schema version for checked `Ask` occurrences.
pub const ASK_OCCURRENCE_SCHEMA_VERSION: u32 = 1;
/// Canonical artifact kind for re-walkable source-program positions.
pub const PROGRAM_POSITION_ARTIFACT_KIND: &str = "ic.program-position";
/// Payload schema version for re-walkable source-program positions.
pub const PROGRAM_POSITION_SCHEMA_VERSION: u32 = 1;

macro_rules! artifact_reference {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(ArtifactRef);

        impl $name {
            #[must_use]
            pub const fn from_artifact_ref(reference: ArtifactRef) -> Self {
                Self(reference)
            }

            #[must_use]
            pub const fn as_artifact_ref(self) -> ArtifactRef {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl FromStr for $name {
            type Err = ArtifactError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                ArtifactRef::from_str(value).map(Self)
            }
        }
    };
}

artifact_reference!(SourceConfigRef);
artifact_reference!(AskOccurrenceRef);
artifact_reference!(ProgramPositionRef);

/// One first-order source program together with its explicit external environment and versions.
///
/// This is source identity, not an executable state, authority record, or continuation result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceConfig {
    result_type: TypeRef,
    program: IProgRef,
    environment: Vec<ProgramBinding>,
    binding_version: BindingVersionRef,
    compiler_version: ArtifactRef,
    provenance: ProvenanceRef,
}

impl SourceConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        result_type: TypeRef,
        program: IProgRef,
        environment: Vec<ProgramBinding>,
        binding_version: BindingVersionRef,
        compiler_version: ArtifactRef,
        provenance: ProvenanceRef,
    ) -> Result<Self, SourceConfigError> {
        Ok(Self {
            result_type,
            program,
            environment: canonical_environment(environment)?,
            binding_version,
            compiler_version,
            provenance,
        })
    }

    #[must_use]
    pub const fn result_type(&self) -> TypeRef {
        self.result_type
    }

    #[must_use]
    pub const fn program(&self) -> IProgRef {
        self.program
    }

    #[must_use]
    pub fn environment(&self) -> &[ProgramBinding] {
        &self.environment
    }

    #[must_use]
    pub const fn binding_version(&self) -> BindingVersionRef {
        self.binding_version
    }

    #[must_use]
    pub const fn compiler_version(&self) -> ArtifactRef {
        self.compiler_version
    }

    #[must_use]
    pub const fn provenance(&self) -> ProvenanceRef {
        self.provenance
    }

    pub fn canonical_payload(&self) -> Result<Vec<u8>, SourceConfigError> {
        let mut encoded = Vec::new();
        reference(&mut encoded, self.result_type.as_artifact_ref());
        reference(&mut encoded, self.program.as_artifact_ref());
        bindings(&mut encoded, &self.environment)?;
        reference(&mut encoded, self.binding_version.as_artifact_ref());
        reference(&mut encoded, self.compiler_version);
        reference(&mut encoded, self.provenance.as_artifact_ref());
        Ok(encoded)
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, SourceConfigError> {
        let mut cursor = Cursor::new(payload);
        let result_type = TypeRef::from_artifact_ref(cursor.reference()?);
        let program = IProgRef::from_artifact_ref(cursor.reference()?);
        let environment = cursor.bindings()?;
        let binding_version = BindingVersionRef::from_artifact_ref(cursor.reference()?);
        let compiler_version = cursor.reference()?;
        let provenance = ProvenanceRef::from_artifact_ref(cursor.reference()?);
        if !cursor.finished() {
            return Err(SourceConfigError::TrailingPayloadBytes(cursor.remaining()));
        }
        let source = Self::new(
            result_type,
            program,
            environment.clone(),
            binding_version,
            compiler_version,
            provenance,
        )?;
        if source.environment != environment {
            return Err(SourceConfigError::NonCanonicalEnvironmentOrder);
        }
        Ok(source)
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, SourceConfigError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(SOURCE_CONFIG_ARTIFACT_KIND)?,
            SOURCE_CONFIG_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }

    pub fn source_config_ref(&self) -> Result<SourceConfigRef, SourceConfigError> {
        Ok(SourceConfigRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, SourceConfigError> {
        if envelope.kind().as_str() != SOURCE_CONFIG_ARTIFACT_KIND {
            return Err(SourceConfigError::UnexpectedArtifactKind {
                expected: SOURCE_CONFIG_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != SOURCE_CONFIG_SCHEMA_VERSION {
            return Err(SourceConfigError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    #[must_use]
    pub fn referenced_artifacts(&self) -> Vec<ArtifactRef> {
        let mut references = vec![
            self.result_type.as_artifact_ref(),
            self.program.as_artifact_ref(),
            self.binding_version.as_artifact_ref(),
            self.compiler_version,
            self.provenance.as_artifact_ref(),
        ];
        references.extend(
            self.environment
                .iter()
                .map(|binding| binding.value().as_artifact_ref()),
        );
        references
    }

    /// Rechecks the exact root, every represented program edge, typed environment value, and
    /// binding version. Compiler version and provenance remain explicit identity coordinates;
    /// this phase has no separate compiler or provenance authority artifact to invent.
    pub fn check<C: QuestionSuccessionCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), SourceConfigCheckError> {
        let root = resolve_source_program(self.program, catalog)?;
        if root.result() != self.result_type {
            return Err(SourceConfigCheckError::RootResultMismatch {
                declared: self.result_type,
                program: root.result(),
            });
        }
        check_type_binding(self.result_type, self.binding_version, catalog)?;
        for binding in &self.environment {
            check_form_binding(binding.value(), self.binding_version, catalog)?;
        }
        let mut visiting = BTreeSet::new();
        check_program_binding(self.program, self.binding_version, catalog, &mut visiting)
    }

    /// Re-walks the stored first-order source graph and derives every `Ask` occurrence.
    pub fn ask_occurrences<C: QuestionSuccessionCatalog>(
        &self,
        catalog: &C,
    ) -> Result<Vec<AskOccurrence>, AskOccurrenceCheckError> {
        self.check(catalog)?;
        let source_ref = self.source_config_ref()?;
        let mut path = Vec::new();
        let mut current = self.program;
        let mut occurrences = Vec::new();
        loop {
            path.push(current);
            let program = resolve_program(current, catalog)?;
            match program.program() {
                IProgIR::Return { .. } => return Ok(occurrences),
                IProgIR::Ask {
                    question,
                    environment,
                    answer_slot,
                    continuation,
                } => {
                    let position = ProgramPosition::new(source_ref, path.clone())?;
                    let occurrence_environment = merge_environments(&self.environment, environment)
                        .map_err(AskOccurrenceCheckError::Environment)?;
                    if occurrence_environment
                        .iter()
                        .any(|binding| binding.name() == answer_slot)
                    {
                        return Err(AskOccurrenceCheckError::AnswerSlotShadowsEnvironment(
                            answer_slot.as_str().to_owned(),
                        ));
                    }
                    occurrences.push(AskOccurrence::new(
                        source_ref,
                        position,
                        *question,
                        occurrence_environment,
                        answer_slot.clone(),
                        *continuation,
                        self.binding_version,
                        self.compiler_version,
                        self.provenance,
                    )?);
                    current = *continuation;
                }
            }
        }
    }
}

/// A re-walkable first-order path from one source configuration root to an `Ask` node.
///
/// It has a content identity but remains embedded derived structure in an `AskOccurrence`; it
/// does not create an authoritative position table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgramPosition {
    source_config: SourceConfigRef,
    path: Vec<IProgRef>,
}

impl ProgramPosition {
    pub fn new(
        source_config: SourceConfigRef,
        path: Vec<IProgRef>,
    ) -> Result<Self, ProgramPositionError> {
        if path.is_empty() {
            return Err(ProgramPositionError::EmptyPath);
        }
        Ok(Self {
            source_config,
            path,
        })
    }

    #[must_use]
    pub const fn source_config(&self) -> SourceConfigRef {
        self.source_config
    }

    #[must_use]
    pub fn path(&self) -> &[IProgRef] {
        &self.path
    }

    #[must_use]
    pub fn target(&self) -> IProgRef {
        self.path[self.path.len() - 1]
    }

    pub fn canonical_payload(&self) -> Result<Vec<u8>, ProgramPositionError> {
        let mut encoded = Vec::new();
        reference(&mut encoded, self.source_config.as_artifact_ref());
        count(&mut encoded, self.path.len()).map_err(ProgramPositionError::Count)?;
        for program in &self.path {
            reference(&mut encoded, program.as_artifact_ref());
        }
        Ok(encoded)
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, ProgramPositionError> {
        let mut cursor = Cursor::new(payload);
        let source_config = SourceConfigRef::from_artifact_ref(cursor.reference()?);
        let count = cursor.count()?;
        let mut path = Vec::with_capacity(count);
        for _ in 0..count {
            path.push(IProgRef::from_artifact_ref(cursor.reference()?));
        }
        if !cursor.finished() {
            return Err(ProgramPositionError::TrailingPayloadBytes(
                cursor.remaining(),
            ));
        }
        Self::new(source_config, path)
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, ProgramPositionError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(PROGRAM_POSITION_ARTIFACT_KIND)?,
            PROGRAM_POSITION_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }

    pub fn position_ref(&self) -> Result<ProgramPositionRef, ProgramPositionError> {
        Ok(ProgramPositionRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, ProgramPositionError> {
        if envelope.kind().as_str() != PROGRAM_POSITION_ARTIFACT_KIND {
            return Err(ProgramPositionError::UnexpectedArtifactKind {
                expected: PROGRAM_POSITION_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != PROGRAM_POSITION_SCHEMA_VERSION {
            return Err(ProgramPositionError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    fn check<C: QuestionSuccessionCatalog>(
        &self,
        source: &SourceConfig,
        catalog: &C,
    ) -> Result<(), AskOccurrenceCheckError> {
        let expected_source = source.source_config_ref()?;
        if self.source_config != expected_source {
            return Err(AskOccurrenceCheckError::PositionSourceMismatch {
                position: self.source_config,
                occurrence: expected_source,
            });
        }
        if self.path.first().copied() != Some(source.program()) {
            return Err(AskOccurrenceCheckError::PositionRootMismatch);
        }
        for pair in self.path.windows(2) {
            let program = resolve_program(pair[0], catalog)?;
            let IProgIR::Ask { continuation, .. } = program.program() else {
                return Err(AskOccurrenceCheckError::PositionContinuesFromReturn(
                    pair[0],
                ));
            };
            if *continuation != pair[1] {
                return Err(AskOccurrenceCheckError::PositionEdgeMismatch {
                    program: pair[0],
                    expected: *continuation,
                    actual: pair[1],
                });
            }
        }
        let target = resolve_program(self.target(), catalog)?;
        if !matches!(target.program(), IProgIR::Ask { .. }) {
            return Err(AskOccurrenceCheckError::PositionDoesNotNameAsk(
                self.target(),
            ));
        }
        Ok(())
    }
}

/// A checked, occurrence-indexed source `Ask` node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AskOccurrence {
    source_config: SourceConfigRef,
    position: ProgramPosition,
    question: QueryRef,
    environment: Vec<ProgramBinding>,
    answer_slot: TypeSymbol,
    continuation: IProgRef,
    binding_version: BindingVersionRef,
    compiler_version: ArtifactRef,
    provenance: ProvenanceRef,
}

impl AskOccurrence {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_config: SourceConfigRef,
        position: ProgramPosition,
        question: QueryRef,
        environment: Vec<ProgramBinding>,
        answer_slot: TypeSymbol,
        continuation: IProgRef,
        binding_version: BindingVersionRef,
        compiler_version: ArtifactRef,
        provenance: ProvenanceRef,
    ) -> Result<Self, AskOccurrenceError> {
        Ok(Self {
            source_config,
            position,
            question,
            environment: canonical_environment(environment)
                .map_err(AskOccurrenceError::Environment)?,
            answer_slot,
            continuation,
            binding_version,
            compiler_version,
            provenance,
        })
    }

    #[must_use]
    pub const fn source_config(&self) -> SourceConfigRef {
        self.source_config
    }

    #[must_use]
    pub const fn position(&self) -> &ProgramPosition {
        &self.position
    }

    #[must_use]
    pub const fn question(&self) -> QueryRef {
        self.question
    }

    #[must_use]
    pub fn environment(&self) -> &[ProgramBinding] {
        &self.environment
    }

    #[must_use]
    pub const fn answer_slot(&self) -> &TypeSymbol {
        &self.answer_slot
    }

    #[must_use]
    pub const fn continuation(&self) -> IProgRef {
        self.continuation
    }

    #[must_use]
    pub const fn binding_version(&self) -> BindingVersionRef {
        self.binding_version
    }

    #[must_use]
    pub const fn compiler_version(&self) -> ArtifactRef {
        self.compiler_version
    }

    #[must_use]
    pub const fn provenance(&self) -> ProvenanceRef {
        self.provenance
    }

    pub fn canonical_payload(&self) -> Result<Vec<u8>, AskOccurrenceError> {
        let mut encoded = Vec::new();
        reference(&mut encoded, self.source_config.as_artifact_ref());
        let position = self.position.canonical_payload()?;
        count(&mut encoded, position.len()).map_err(AskOccurrenceError::Count)?;
        encoded.extend_from_slice(&position);
        reference(&mut encoded, self.question.as_artifact_ref());
        bindings(&mut encoded, &self.environment).map_err(AskOccurrenceError::Environment)?;
        text(&mut encoded, self.answer_slot.as_str()).map_err(AskOccurrenceError::Slot)?;
        reference(&mut encoded, self.continuation.as_artifact_ref());
        reference(&mut encoded, self.binding_version.as_artifact_ref());
        reference(&mut encoded, self.compiler_version);
        reference(&mut encoded, self.provenance.as_artifact_ref());
        Ok(encoded)
    }

    pub fn envelope(&self) -> Result<ArtifactEnvelope, AskOccurrenceError> {
        Ok(ArtifactEnvelope::from_canonical_payload(
            ArtifactKind::new(ASK_OCCURRENCE_ARTIFACT_KIND)?,
            ASK_OCCURRENCE_SCHEMA_VERSION,
            self.canonical_payload()?,
        ))
    }

    pub fn decode_payload(payload: &[u8]) -> Result<Self, AskOccurrenceError> {
        let mut cursor = Cursor::new(payload);
        let source_config = SourceConfigRef::from_artifact_ref(cursor.reference()?);
        let position_length = cursor.count()?;
        let position = ProgramPosition::decode_payload(cursor.take(position_length)?)?;
        let question = QueryRef::from_artifact_ref(cursor.reference()?);
        let environment = cursor.bindings()?;
        let answer_slot_text = cursor.text()?;
        let answer_slot = TypeSymbol::new(answer_slot_text.clone())
            .map_err(|_| SourceConfigError::InvalidEnvironmentName(answer_slot_text))?;
        let continuation = IProgRef::from_artifact_ref(cursor.reference()?);
        let binding_version = BindingVersionRef::from_artifact_ref(cursor.reference()?);
        let compiler_version = cursor.reference()?;
        let provenance = ProvenanceRef::from_artifact_ref(cursor.reference()?);
        if !cursor.finished() {
            return Err(AskOccurrenceError::TrailingPayloadBytes(cursor.remaining()));
        }
        let occurrence = Self::new(
            source_config,
            position,
            question,
            environment.clone(),
            answer_slot,
            continuation,
            binding_version,
            compiler_version,
            provenance,
        )?;
        if occurrence.environment != environment {
            return Err(AskOccurrenceError::NonCanonicalEnvironmentOrder);
        }
        Ok(occurrence)
    }

    pub fn from_envelope(envelope: &ArtifactEnvelope) -> Result<Self, AskOccurrenceError> {
        if envelope.kind().as_str() != ASK_OCCURRENCE_ARTIFACT_KIND {
            return Err(AskOccurrenceError::UnexpectedArtifactKind {
                expected: ASK_OCCURRENCE_ARTIFACT_KIND,
                actual: envelope.kind().as_str().to_owned(),
            });
        }
        if envelope.schema_version() != ASK_OCCURRENCE_SCHEMA_VERSION {
            return Err(AskOccurrenceError::UnsupportedSchemaVersion(
                envelope.schema_version(),
            ));
        }
        Self::decode_payload(envelope.canonical_payload())
    }

    pub fn ask_occurrence_ref(&self) -> Result<AskOccurrenceRef, AskOccurrenceError> {
        Ok(AskOccurrenceRef::from_artifact_ref(
            self.envelope()?.artifact_ref()?,
        ))
    }

    /// Re-walks the source configuration and compares every occurrence-owned field.
    pub fn check<C: QuestionSuccessionCatalog>(
        &self,
        catalog: &C,
    ) -> Result<(), AskOccurrenceCheckError> {
        let source = catalog.resolve_source_config(self.source_config).ok_or(
            AskOccurrenceCheckError::UnresolvedSourceConfig(self.source_config),
        )?;
        let calculated = source.source_config_ref()?;
        if calculated != self.source_config {
            return Err(AskOccurrenceCheckError::SourceConfigIdentityMismatch {
                reference: self.source_config,
                calculated,
            });
        }
        source.check(catalog)?;
        self.position.check(&source, catalog)?;
        let expected = source
            .ask_occurrences(catalog)?
            .into_iter()
            .find(|candidate| candidate.position == self.position)
            .ok_or(AskOccurrenceCheckError::PositionNotDerived)?;
        compare_occurrence(self, &expected)
    }
}

/// One derived `Ask` continuation result, indexed by the exact occurrence and whole answer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuestionSuccessor {
    Ask {
        occurrence: AskOccurrence,
        answer: AdmittedFiniteAnswerSet,
        successor: Box<AskOccurrence>,
    },
    Return {
        occurrence: AskOccurrence,
        answer: AdmittedFiniteAnswerSet,
        value: TypedFormRef,
    },
}

/// A derived, nonexecuting normalization of one already bound `Ask` continuation.
///
/// The current first-order source grammar has no hidden pure expression nodes between an `Ask`
/// and its named continuation. Therefore its complete registered normalizer is identity on the
/// checked continuation reference, indexed by the source configuration's compiler coordinate.
/// This record retains the whole supported answer and exact occurrence rather than rewriting a
/// source artifact, selecting a completion, or stepping a runtime program.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PureNormalizedAskContinuation {
    occurrence: AskOccurrence,
    binding: BoundFiniteAskContinuation,
    normalization_version: ArtifactRef,
    successor: QuestionSuccessor,
}

impl PureNormalizedAskContinuation {
    #[must_use]
    pub const fn occurrence(&self) -> &AskOccurrence {
        &self.occurrence
    }

    #[must_use]
    pub const fn binding(&self) -> &BoundFiniteAskContinuation {
        &self.binding
    }

    /// Returns the fixed compiler coordinate that versions the current identity normalizer.
    #[must_use]
    pub const fn normalization_version(&self) -> ArtifactRef {
        self.normalization_version
    }

    #[must_use]
    pub const fn successor(&self) -> &QuestionSuccessor {
        &self.successor
    }
}

/// Rechecks that a whole answer is bound to this exact source `Ask`, then performs the current
/// deterministic pure source normalization.
///
/// This is deliberately a derived composition of `bind_finite_ask_continuation` and
/// `derive_question_successor`, not a second source language, evaluator, or execution step.
pub fn normalize_bound_finite_ask_continuation<C: QuestionSuccessionCatalog>(
    occurrence: AskOccurrence,
    binding: BoundFiniteAskContinuation,
    catalog: &C,
) -> Result<PureNormalizedAskContinuation, PureNormalizationError> {
    occurrence.check(catalog)?;
    let source_ref = occurrence.position().target();
    if binding.source() != source_ref {
        return Err(PureNormalizationError::SourceMismatch {
            occurrence: source_ref,
            binding: binding.source(),
        });
    }
    let source = resolve_program(source_ref, catalog)?;
    let IProgIR::Ask {
        question,
        environment,
        answer_slot,
        continuation,
    } = source.program()
    else {
        return Err(PureNormalizationError::SourceIsNotAsk(source_ref));
    };
    if binding.question() != *question || binding.answer().decoded().query() != *question {
        return Err(PureNormalizationError::QuestionMismatch {
            occurrence: *question,
            binding: binding.question(),
            answer: binding.answer().decoded().query(),
        });
    }
    if binding.environment() != environment {
        return Err(PureNormalizationError::EnvironmentMismatch);
    }
    if binding.answer_slot() != answer_slot {
        return Err(PureNormalizationError::AnswerSlotMismatch {
            occurrence: answer_slot.as_str().to_owned(),
            binding: binding.answer_slot().as_str().to_owned(),
        });
    }
    if binding.continuation() != *continuation {
        return Err(PureNormalizationError::ContinuationMismatch {
            occurrence: *continuation,
            binding: binding.continuation(),
        });
    }
    if occurrence.question() != *question
        || occurrence.answer_slot() != answer_slot
        || occurrence.continuation() != *continuation
    {
        return Err(PureNormalizationError::OccurrenceMismatch);
    }
    let successor =
        derive_question_successor(occurrence.clone(), binding.answer().clone(), catalog)?;
    Ok(PureNormalizedAskContinuation {
        normalization_version: occurrence.compiler_version(),
        occurrence,
        binding,
        successor,
    })
}

/// Reconstructs the first successor from one checked occurrence and the complete supported answer.
///
/// This phase's first-order syntax has no hidden pure node between an `Ask` continuation and its
/// next node, so the checked head is immediately either another `Ask` or `Return`.
pub fn derive_question_successor<C: QuestionSuccessionCatalog>(
    occurrence: AskOccurrence,
    answer: AdmittedFiniteAnswerSet,
    catalog: &C,
) -> Result<QuestionSuccessor, QuestionSuccessorError> {
    occurrence.check(catalog)?;
    if answer.decoded().query() != occurrence.question() {
        return Err(QuestionSuccessorError::AnswerQuestionMismatch {
            occurrence: occurrence.question(),
            answer: answer.decoded().query(),
        });
    }
    let continuation = resolve_program(occurrence.continuation(), catalog)?;
    match continuation.program() {
        IProgIR::Return { value } => Ok(QuestionSuccessor::Return {
            occurrence,
            answer,
            value: *value,
        }),
        IProgIR::Ask { .. } => {
            let source = catalog
                .resolve_source_config(occurrence.source_config())
                .ok_or(AskOccurrenceCheckError::UnresolvedSourceConfig(
                    occurrence.source_config(),
                ))?;
            let mut path = occurrence.position().path().to_vec();
            path.push(occurrence.continuation());
            let position = ProgramPosition::new(occurrence.source_config(), path)?;
            let successor = source
                .ask_occurrences(catalog)?
                .into_iter()
                .find(|candidate| candidate.position == position)
                .ok_or(QuestionSuccessorError::SuccessorPositionNotDerived)?;
            Ok(QuestionSuccessor::Ask {
                occurrence,
                answer,
                successor: Box::new(successor),
            })
        }
    }
}

/// Catalog boundary for source configuration rechecking.
pub trait QuestionSuccessionCatalog: IProgCatalog {
    fn resolve_source_config(&self, reference: SourceConfigRef) -> Option<SourceConfig>;
}

fn resolve_program<C: IProgCatalog>(
    reference: IProgRef,
    catalog: &C,
) -> Result<IProgArtifact, AskOccurrenceCheckError> {
    let program = catalog
        .resolve_iprog(reference)
        .ok_or(AskOccurrenceCheckError::UnresolvedProgram(reference))?;
    let calculated = program.iprog_ref()?;
    if calculated != reference {
        return Err(AskOccurrenceCheckError::ProgramIdentityMismatch {
            reference,
            calculated,
        });
    }
    program.check(catalog)?;
    Ok(program)
}

fn resolve_source_program<C: IProgCatalog>(
    reference: IProgRef,
    catalog: &C,
) -> Result<IProgArtifact, SourceConfigCheckError> {
    let program = catalog
        .resolve_iprog(reference)
        .ok_or(SourceConfigCheckError::UnresolvedProgram(reference))?;
    let calculated = program.iprog_ref()?;
    if calculated != reference {
        return Err(SourceConfigCheckError::ProgramIdentityMismatch {
            reference,
            calculated,
        });
    }
    program.check(catalog)?;
    Ok(program)
}

fn check_type_binding<C: IProgCatalog>(
    reference: TypeRef,
    expected: BindingVersionRef,
    catalog: &C,
) -> Result<(), SourceConfigCheckError> {
    let ty = catalog
        .resolve_type(reference)
        .ok_or(SourceConfigCheckError::UnresolvedType(reference))?;
    let calculated = ty.type_ref()?;
    if calculated != reference {
        return Err(SourceConfigCheckError::TypeIdentityMismatch {
            reference,
            calculated,
        });
    }
    ty.check(catalog)?;
    if ty.binding() != expected {
        return Err(SourceConfigCheckError::TypeBindingMismatch {
            reference,
            expected,
            actual: ty.binding(),
        });
    }
    Ok(())
}

fn check_form_binding<C: IProgCatalog>(
    reference: TypedFormRef,
    expected: BindingVersionRef,
    catalog: &C,
) -> Result<(), SourceConfigCheckError> {
    let form = catalog
        .resolve_typed_form(reference)
        .ok_or(SourceConfigCheckError::UnresolvedTypedForm(reference))?;
    let calculated = form.typed_form_ref()?;
    if calculated != reference {
        return Err(SourceConfigCheckError::TypedFormIdentityMismatch {
            reference,
            calculated,
        });
    }
    form.check(catalog)?;
    if form.binding() != expected {
        return Err(SourceConfigCheckError::TypedFormBindingMismatch {
            reference,
            expected,
            actual: form.binding(),
        });
    }
    Ok(())
}

fn check_program_binding<C: IProgCatalog>(
    reference: IProgRef,
    expected: BindingVersionRef,
    catalog: &C,
    visiting: &mut BTreeSet<IProgRef>,
) -> Result<(), SourceConfigCheckError> {
    if !visiting.insert(reference) {
        return Err(SourceConfigCheckError::CyclicProgram(reference));
    }
    let program = resolve_source_program(reference, catalog)?;
    check_type_binding(program.result(), expected, catalog)?;
    match program.program() {
        IProgIR::Return { value } => check_form_binding(*value, expected, catalog)?,
        IProgIR::Ask {
            question,
            environment,
            continuation,
            ..
        } => {
            let query = catalog
                .resolve_open_query(*question)
                .ok_or(SourceConfigCheckError::UnresolvedQuestion(*question))?;
            let schema = catalog.resolve_relation_schema(query.relation()).ok_or(
                SourceConfigCheckError::UnresolvedQuestionRelation(query.relation()),
            )?;
            if schema.binding() != expected {
                return Err(SourceConfigCheckError::QuestionBindingMismatch {
                    question: *question,
                    expected,
                    actual: schema.binding(),
                });
            }
            for binding in environment {
                check_form_binding(binding.value(), expected, catalog)?;
            }
            check_program_binding(*continuation, expected, catalog, visiting)?;
        }
    }
    visiting.remove(&reference);
    Ok(())
}

fn merge_environments(
    source: &[ProgramBinding],
    local: &[ProgramBinding],
) -> Result<Vec<ProgramBinding>, SourceConfigError> {
    let mut merged = source.to_vec();
    merged.extend_from_slice(local);
    canonical_environment(merged)
}

fn canonical_environment(
    mut environment: Vec<ProgramBinding>,
) -> Result<Vec<ProgramBinding>, SourceConfigError> {
    environment.sort_by(|left, right| left.name().as_str().cmp(right.name().as_str()));
    if let Some(duplicate) = environment
        .windows(2)
        .find(|pair| pair[0].name() == pair[1].name())
    {
        return Err(SourceConfigError::DuplicateEnvironmentBinding(
            duplicate[0].name().as_str().to_owned(),
        ));
    }
    Ok(environment)
}

fn compare_occurrence(
    actual: &AskOccurrence,
    expected: &AskOccurrence,
) -> Result<(), AskOccurrenceCheckError> {
    if actual.question != expected.question {
        return Err(AskOccurrenceCheckError::DerivedFieldMismatch("question"));
    }
    if actual.environment != expected.environment {
        return Err(AskOccurrenceCheckError::DerivedFieldMismatch("environment"));
    }
    if actual.answer_slot != expected.answer_slot {
        return Err(AskOccurrenceCheckError::DerivedFieldMismatch("answer slot"));
    }
    if actual.continuation != expected.continuation {
        return Err(AskOccurrenceCheckError::DerivedFieldMismatch(
            "continuation",
        ));
    }
    if actual.binding_version != expected.binding_version {
        return Err(AskOccurrenceCheckError::DerivedFieldMismatch(
            "binding version",
        ));
    }
    if actual.compiler_version != expected.compiler_version {
        return Err(AskOccurrenceCheckError::DerivedFieldMismatch(
            "compiler version",
        ));
    }
    if actual.provenance != expected.provenance {
        return Err(AskOccurrenceCheckError::DerivedFieldMismatch("provenance"));
    }
    Ok(())
}

fn reference(encoded: &mut Vec<u8>, reference: ArtifactRef) {
    encoded.extend_from_slice(reference.as_bytes());
}

fn count(encoded: &mut Vec<u8>, value: usize) -> Result<(), SourceConfigError> {
    let count = u32::try_from(value).map_err(|_| SourceConfigError::CountTooLarge(value))?;
    encoded.extend_from_slice(&count.to_be_bytes());
    Ok(())
}

fn bindings(
    encoded: &mut Vec<u8>,
    environment: &[ProgramBinding],
) -> Result<(), SourceConfigError> {
    count(encoded, environment.len())?;
    for binding in environment {
        text(encoded, binding.name().as_str())?;
        reference(encoded, binding.value().as_artifact_ref());
    }
    Ok(())
}

fn text(encoded: &mut Vec<u8>, value: &str) -> Result<(), SourceConfigError> {
    let length =
        u32::try_from(value.len()).map_err(|_| SourceConfigError::TextTooLong(value.len()))?;
    encoded.extend_from_slice(&length.to_be_bytes());
    encoded.extend_from_slice(value.as_bytes());
    Ok(())
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], SourceConfigError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(SourceConfigError::PayloadLengthOverflow)?;
        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(SourceConfigError::TruncatedPayload)?;
        self.position = end;
        Ok(bytes)
    }

    fn reference(&mut self) -> Result<ArtifactRef, SourceConfigError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| SourceConfigError::TruncatedPayload)?;
        Ok(ArtifactRef::from_bytes(bytes))
    }

    fn count(&mut self) -> Result<usize, SourceConfigError> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| SourceConfigError::TruncatedPayload)?;
        Ok(u32::from_be_bytes(bytes) as usize)
    }

    fn text(&mut self) -> Result<String, SourceConfigError> {
        let length = self.count()?;
        let bytes = self.take(length)?;
        std::str::from_utf8(bytes)
            .map(str::to_owned)
            .map_err(|_| SourceConfigError::MalformedUtf8)
    }

    fn bindings(&mut self) -> Result<Vec<ProgramBinding>, SourceConfigError> {
        let count = self.count()?;
        let mut environment = Vec::with_capacity(count);
        for _ in 0..count {
            let name = self.text()?;
            let name = TypeSymbol::new(name.clone())
                .map_err(|_| SourceConfigError::InvalidEnvironmentName(name))?;
            let value = TypedFormRef::from_artifact_ref(self.reference()?);
            environment.push(ProgramBinding::new(name, value));
        }
        Ok(environment)
    }

    const fn finished(&self) -> bool {
        self.position == self.bytes.len()
    }

    const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
}

/// Canonical source-config encoding failures.
#[derive(Debug, Error)]
pub enum SourceConfigError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error("source configuration environment has duplicate binding {0:?}")]
    DuplicateEnvironmentBinding(String),
    #[error("source configuration environment is not in canonical name order")]
    NonCanonicalEnvironmentOrder,
    #[error("source configuration text is too long: {0} bytes")]
    TextTooLong(usize),
    #[error("source configuration contains too many entries: {0}")]
    CountTooLarge(usize),
    #[error("source configuration payload is truncated")]
    TruncatedPayload,
    #[error("source configuration payload length overflows")]
    PayloadLengthOverflow,
    #[error("source configuration payload has {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("source configuration text is malformed UTF-8")]
    MalformedUtf8,
    #[error("source configuration environment name {0:?} is invalid")]
    InvalidEnvironmentName(String),
    #[error("expected source configuration artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported source configuration schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

/// Program-position construction failures.
#[derive(Debug, Error)]
pub enum ProgramPositionError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error(transparent)]
    Source(#[from] SourceConfigError),
    #[error("an Ask position needs a nonempty source-program path")]
    EmptyPath,
    #[error("program-position path has too many entries: {0}")]
    Count(SourceConfigError),
    #[error("program-position payload has {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("expected program-position artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported program-position schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

/// Canonical `Ask` occurrence encoding failures.
#[derive(Debug, Error)]
pub enum AskOccurrenceError {
    #[error(transparent)]
    Artifact(#[from] ArtifactError),
    #[error(transparent)]
    Position(#[from] ProgramPositionError),
    #[error("Ask occurrence environment is invalid: {0}")]
    Environment(SourceConfigError),
    #[error("Ask occurrence position encoding is too large: {0}")]
    Count(SourceConfigError),
    #[error("Ask occurrence slot encoding is invalid: {0}")]
    Slot(SourceConfigError),
    #[error(transparent)]
    Source(#[from] SourceConfigError),
    #[error("Ask occurrence payload has {0} trailing bytes")]
    TrailingPayloadBytes(usize),
    #[error("Ask occurrence environment is not in canonical name order")]
    NonCanonicalEnvironmentOrder,
    #[error("expected Ask occurrence artifact kind {expected:?}, got {actual:?}")]
    UnexpectedArtifactKind {
        expected: &'static str,
        actual: String,
    },
    #[error("unsupported Ask occurrence schema version {0}")]
    UnsupportedSchemaVersion(u32),
}

/// Structural source configuration failures.
#[derive(Debug, Error)]
pub enum SourceConfigCheckError {
    #[error(transparent)]
    Source(#[from] SourceConfigError),
    #[error(transparent)]
    Program(#[from] IProgError),
    #[error(transparent)]
    ProgramCheck(#[from] IProgCheckError),
    #[error(transparent)]
    Type(#[from] TypeError),
    #[error(transparent)]
    TypeCheck(#[from] TypeCheckError),
    #[error(
        "source configuration program result {program} differs from declared result {declared}"
    )]
    RootResultMismatch { declared: TypeRef, program: TypeRef },
    #[error("source program {0} is unavailable")]
    UnresolvedProgram(IProgRef),
    #[error("source program {reference} hashes to {calculated}")]
    ProgramIdentityMismatch {
        reference: IProgRef,
        calculated: IProgRef,
    },
    #[error("source type {0} is unavailable")]
    UnresolvedType(TypeRef),
    #[error("source type {reference} hashes to {calculated}")]
    TypeIdentityMismatch {
        reference: TypeRef,
        calculated: TypeRef,
    },
    #[error("source type {reference} has binding {actual}, expected {expected}")]
    TypeBindingMismatch {
        reference: TypeRef,
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },
    #[error("source typed form {0} is unavailable")]
    UnresolvedTypedForm(TypedFormRef),
    #[error("source typed form {reference} hashes to {calculated}")]
    TypedFormIdentityMismatch {
        reference: TypedFormRef,
        calculated: TypedFormRef,
    },
    #[error("source typed form {reference} has binding {actual}, expected {expected}")]
    TypedFormBindingMismatch {
        reference: TypedFormRef,
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },
    #[error("source question {0} is unavailable")]
    UnresolvedQuestion(QueryRef),
    #[error("source question relation {0} is unavailable")]
    UnresolvedQuestionRelation(crate::RelationRef),
    #[error("source question {question} has binding {actual}, expected {expected}")]
    QuestionBindingMismatch {
        question: QueryRef,
        expected: BindingVersionRef,
        actual: BindingVersionRef,
    },
    #[error("source program graph cycles at {0}")]
    CyclicProgram(IProgRef),
}

/// Checked occurrence formation failures.
#[derive(Debug, Error)]
pub enum AskOccurrenceCheckError {
    #[error(transparent)]
    Source(#[from] SourceConfigError),
    #[error(transparent)]
    SourceCheck(#[from] SourceConfigCheckError),
    #[error(transparent)]
    Program(#[from] IProgError),
    #[error(transparent)]
    ProgramCheck(#[from] IProgCheckError),
    #[error(transparent)]
    Position(#[from] ProgramPositionError),
    #[error(transparent)]
    Occurrence(#[from] AskOccurrenceError),
    #[error("source configuration {0} is unavailable")]
    UnresolvedSourceConfig(SourceConfigRef),
    #[error("source configuration {reference} hashes to {calculated}")]
    SourceConfigIdentityMismatch {
        reference: SourceConfigRef,
        calculated: SourceConfigRef,
    },
    #[error("source program {0} is unavailable")]
    UnresolvedProgram(IProgRef),
    #[error("source program {reference} hashes to {calculated}")]
    ProgramIdentityMismatch {
        reference: IProgRef,
        calculated: IProgRef,
    },
    #[error("Ask position belongs to {position}, not occurrence source {occurrence}")]
    PositionSourceMismatch {
        position: SourceConfigRef,
        occurrence: SourceConfigRef,
    },
    #[error("Ask position does not start at its source program root")]
    PositionRootMismatch,
    #[error("Ask position attempts to continue from Return program {0}")]
    PositionContinuesFromReturn(IProgRef),
    #[error("Ask position edge from {program} should be {expected}, got {actual}")]
    PositionEdgeMismatch {
        program: IProgRef,
        expected: IProgRef,
        actual: IProgRef,
    },
    #[error("Ask position target {0} is not an Ask")]
    PositionDoesNotNameAsk(IProgRef),
    #[error("Ask position is not derived by its source configuration")]
    PositionNotDerived,
    #[error("Ask occurrence copied or forged its {0}")]
    DerivedFieldMismatch(&'static str),
    #[error("Ask occurrence environment cannot be reconstructed: {0}")]
    Environment(SourceConfigError),
    #[error("Ask occurrence answer slot {0:?} shadows its reconstructed environment")]
    AnswerSlotShadowsEnvironment(String),
}

/// Occurrence-indexed successor reconstruction failures.
#[derive(Debug, Error)]
pub enum QuestionSuccessorError {
    #[error(transparent)]
    Occurrence(#[from] AskOccurrenceCheckError),
    #[error(transparent)]
    Position(#[from] ProgramPositionError),
    #[error("supported answer names question {answer}, not Ask occurrence question {occurrence}")]
    AnswerQuestionMismatch {
        occurrence: QueryRef,
        answer: QueryRef,
    },
    #[error("the continuation Ask position is not derived by the source configuration")]
    SuccessorPositionNotDerived,
}

/// Failures from exact bound-continuation normalization.
#[derive(Debug, Error)]
pub enum PureNormalizationError {
    #[error(transparent)]
    Occurrence(#[from] AskOccurrenceCheckError),
    #[error(transparent)]
    Successor(#[from] QuestionSuccessorError),
    #[error("bound source {binding} differs from Ask occurrence source {occurrence}")]
    SourceMismatch {
        occurrence: IProgRef,
        binding: IProgRef,
    },
    #[error("source program {0} is not an Ask")]
    SourceIsNotAsk(IProgRef),
    #[error("question mismatch: occurrence {occurrence}, binding {binding}, answer {answer}")]
    QuestionMismatch {
        occurrence: QueryRef,
        binding: QueryRef,
        answer: QueryRef,
    },
    #[error("bound lexical environment differs from the exact source Ask environment")]
    EnvironmentMismatch,
    #[error("answer slot {binding:?} differs from source Ask slot {occurrence:?}")]
    AnswerSlotMismatch { occurrence: String, binding: String },
    #[error("bound continuation {binding} differs from source Ask continuation {occurrence}")]
    ContinuationMismatch {
        occurrence: IProgRef,
        binding: IProgRef,
    },
    #[error("the supplied occurrence differs from its rechecked source Ask")]
    OccurrenceMismatch,
}

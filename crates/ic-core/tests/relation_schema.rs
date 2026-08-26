use std::collections::BTreeMap;

use ic_core::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactKind, ArtifactRef, BindingVersionRef,
    CompletionCandidate, CompletionCandidateCheckError, CompletionCandidateError,
    DeclaredIncidenceError, DepartureCatalog, DepartureWitness, DepartureWitnessCheckError,
    DepartureWitnessRef, DeterminationCatalog, DeterminationPresentation,
    DeterminationPresentationRef, DischargeMode, DistinctionRef, ExactFiniteSignature,
    FiniteNegationExtension, FormulaArtifact, FormulaCatalog, FormulaIR, FormulaRef, GammaError,
    GeneratorCoverageRef, GrainRef, HorizonRef, IProgArtifact, IProgCatalog, IProgCheckError,
    IProgIR, IProgRef, NegationCoverage, NegationUse, NegationUseCheckError, NegationUseRef,
    OpenPort, OpenQuery, OpenQueryCatalog, OpenQueryCheckError, PortBinding,
    PositiveNegationQueryError, ProgramBinding, ReciprocalOccurrence, ReciprocalOccurrenceError,
    RelationBodyIR, RelationCatalog, RelationCheckError, RelationError, RelationExprArtifact,
    RelationExprIR, RelationPort, RelationRef, RelationSchema, RelationSignature, RelationUse,
    RelationUseCheckError, RelationUseContext, RelationalWebRef, ReturnClosure, RoleComparison,
    ScopeRef, SeedReorientation, SeedReorientationError, SelectedReturn, SignatureContext,
    SupportRef, TaggedExteriorCatalog, TaggedExteriorClaim, TaggedExteriorClaimError, TyIR,
    TypeArtifact, TypeCatalog, TypeFamilyRef, TypeRef, TypeSymbol, TypedFiniteNegationExtension,
    TypedForm, TypedFormRef, TypedNegationExtensionError, check_declared_incidence,
    check_return_closure, exact_return_fiber, positive_negation_query,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct KnownVector {
    kind: String,
    schema_version: u32,
    payload_hex: String,
    encoded_hex: String,
    sha256: String,
}

#[derive(Default)]
struct Catalog {
    types: BTreeMap<TypeRef, TypeArtifact>,
    formulas: BTreeMap<FormulaRef, FormulaArtifact>,
    signatures: BTreeMap<RelationRef, RelationSignature>,
    schemas: BTreeMap<RelationRef, RelationSchema>,
    forms: BTreeMap<TypedFormRef, TypedForm>,
    presentations: BTreeMap<DeterminationPresentationRef, DeterminationPresentation>,
    departures: BTreeMap<DepartureWitnessRef, DepartureWitness>,
    relation_uses: BTreeMap<ic_core::RelationUseRef, RelationUse>,
    negation_uses: BTreeMap<NegationUseRef, NegationUse>,
    queries: BTreeMap<ic_core::QueryRef, OpenQuery>,
    programs: BTreeMap<IProgRef, IProgArtifact>,
}

impl Catalog {
    fn insert_type(&mut self, artifact: TypeArtifact) -> TypeRef {
        let reference = artifact.type_ref().expect("type fixture must encode");
        self.types.insert(reference, artifact);
        reference
    }

    fn insert_formula(&mut self, artifact: FormulaArtifact) -> FormulaRef {
        let reference = artifact.formula_ref().expect("formula fixture must encode");
        self.formulas.insert(reference, artifact);
        reference
    }

    fn insert_schema(&mut self, artifact: RelationSchema) -> RelationRef {
        let reference = artifact.relation_ref().expect("schema fixture must encode");
        self.signatures.insert(
            reference,
            artifact.signature().expect("schema fixture must encode"),
        );
        self.schemas.insert(reference, artifact);
        reference
    }

    fn insert_form(&mut self, form: TypedForm) -> TypedFormRef {
        let reference = form.typed_form_ref().expect("form fixture must encode");
        self.forms.insert(reference, form);
        reference
    }

    fn insert_presentation(
        &mut self,
        presentation: DeterminationPresentation,
    ) -> DeterminationPresentationRef {
        let reference = presentation
            .determination_presentation_ref()
            .expect("presentation fixture must encode");
        self.presentations.insert(reference, presentation);
        reference
    }

    fn insert_relation_use(&mut self, relation_use: RelationUse) -> ic_core::RelationUseRef {
        let reference = relation_use
            .relation_use_ref()
            .expect("relation-use fixture must encode");
        self.relation_uses.insert(reference, relation_use);
        reference
    }

    fn insert_departure(&mut self, witness: DepartureWitness) -> DepartureWitnessRef {
        let reference = witness
            .departure_witness_ref()
            .expect("departure fixture must encode");
        self.departures.insert(reference, witness);
        reference
    }

    fn insert_negation_use(&mut self, negation_use: NegationUse) -> NegationUseRef {
        let reference = negation_use
            .negation_use_ref()
            .expect("negation-use fixture must encode");
        self.negation_uses.insert(reference, negation_use);
        reference
    }

    fn insert_query(&mut self, query: OpenQuery) -> ic_core::QueryRef {
        let reference = query.query_ref().expect("query fixture must encode");
        self.queries.insert(reference, query);
        reference
    }

    fn insert_program(&mut self, program: IProgArtifact) -> IProgRef {
        let reference = program.iprog_ref().expect("program fixture must encode");
        self.programs.insert(reference, program);
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

impl DeterminationCatalog for Catalog {
    fn resolve_determination_presentation(
        &self,
        reference: DeterminationPresentationRef,
    ) -> Option<DeterminationPresentation> {
        self.presentations.get(&reference).cloned()
    }
}

impl DepartureCatalog for Catalog {
    fn resolve_relation_use(&self, reference: ic_core::RelationUseRef) -> Option<RelationUse> {
        self.relation_uses.get(&reference).cloned()
    }
}

impl TaggedExteriorCatalog for Catalog {
    fn resolve_negation_use(&self, reference: NegationUseRef) -> Option<NegationUse> {
        self.negation_uses.get(&reference).cloned()
    }

    fn resolve_departure_witness(
        &self,
        reference: DepartureWitnessRef,
    ) -> Option<DepartureWitness> {
        self.departures.get(&reference).cloned()
    }
}

impl OpenQueryCatalog for Catalog {
    fn resolve_open_query(&self, reference: ic_core::QueryRef) -> Option<OpenQuery> {
        self.queries.get(&reference).cloned()
    }
}

impl IProgCatalog for Catalog {
    fn resolve_iprog(&self, reference: IProgRef) -> Option<IProgArtifact> {
        self.programs.get(&reference).cloned()
    }
}

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn binding(byte: u8) -> BindingVersionRef {
    BindingVersionRef::from_artifact_ref(artifact(byte))
}

fn port(name: &str, ty: TypeRef) -> RelationPort {
    RelationPort::new(TypeSymbol::new(name).expect("port name must be valid"), ty)
}

fn vector() -> KnownVector {
    serde_json::from_str(include_str!(
        "../../../fixtures/relations/relation-v1-native.json"
    ))
    .expect("known vector fixture must be valid JSON")
}

fn open_query_vector() -> KnownVector {
    serde_json::from_str(include_str!(
        "../../../fixtures/queries/open-query-v1-single-open.json"
    ))
    .expect("known open-query vector fixture must be valid JSON")
}

#[test]
fn native_relation_round_trips_and_declares_all_explicit_dependencies() {
    let binding = binding(0x11);
    let schema = RelationSchema::new(
        binding,
        vec![port("subject", TypeRef::from_artifact_ref(artifact(0x22)))],
        RelationBodyIR::BindingNative {
            contract: artifact(0x33),
        },
        vec![artifact(0x44)],
        vec![artifact(0x55)],
    );
    let envelope = schema.envelope().expect("schema fixture must encode");
    let vector = vector();

    assert_eq!(envelope.kind().as_str(), vector.kind);
    assert_eq!(envelope.schema_version(), vector.schema_version);
    assert_eq!(
        hex::encode(envelope.canonical_payload()),
        vector.payload_hex
    );
    assert_eq!(
        hex::encode(envelope.encode().expect("schema fixture must encode")),
        vector.encoded_hex
    );
    assert_eq!(
        envelope
            .artifact_ref()
            .expect("schema fixture must hash")
            .to_string(),
        vector.sha256
    );
    assert_eq!(
        RelationSchema::from_envelope(&envelope).expect("schema fixture must decode"),
        schema
    );
    assert_eq!(
        schema.referenced_artifacts(),
        vec![
            artifact(0x11),
            artifact(0x22),
            artifact(0x33),
            artifact(0x44),
            artifact(0x55)
        ]
    );
    assert_ne!(
        schema.relation_ref().expect("schema fixture must hash"),
        RelationRef::from_artifact_ref(artifact(0x33))
    );
}

#[test]
fn open_query_matches_independent_canonical_vector() {
    let query = OpenQuery::new(
        RelationRef::from_artifact_ref(artifact(0x11)),
        Vec::new(),
        vec![OpenPort::new(
            TypeSymbol::new("answer").expect("port name must be valid"),
            DischargeMode::Probe,
        )],
        RelationUseContext::new(
            ScopeRef::from_artifact_ref(artifact(0x22)),
            ApplicabilityRef::from_artifact_ref(artifact(0x33)),
            GrainRef::from_artifact_ref(artifact(0x44)),
            HorizonRef::from_artifact_ref(artifact(0x55)),
            DischargeMode::Pure,
            SupportRef::from_artifact_ref(artifact(0x66)),
            None,
        ),
    );
    let envelope = query.envelope().expect("query fixture must encode");
    let vector = open_query_vector();

    assert_eq!(envelope.kind().as_str(), vector.kind);
    assert_eq!(envelope.schema_version(), vector.schema_version);
    assert_eq!(
        hex::encode(envelope.canonical_payload()),
        vector.payload_hex
    );
    assert_eq!(
        hex::encode(envelope.encode().expect("query fixture must encode")),
        vector.encoded_hex
    );
    assert_eq!(
        envelope
            .artifact_ref()
            .expect("query fixture must hash")
            .to_string(),
        vector.sha256
    );
    assert_eq!(
        OpenQuery::from_envelope(&envelope).expect("query fixture must decode"),
        query
    );
}

#[test]
fn canonical_relation_expression_grammar_round_trips_without_evaluation() {
    let relation = RelationRef::from_artifact_ref(artifact(0x11));
    let form = TypedFormRef::from_artifact_ref(artifact(0x22));
    let guard = FormulaRef::from_artifact_ref(artifact(0x33));
    let name = |value| TypeSymbol::new(value).expect("port name must be valid");
    let expression = RelationExprIR::Guard {
        source: Box::new(RelationExprIR::Rename {
            source: Box::new(RelationExprIR::Hide {
                source: Box::new(RelationExprIR::Expose {
                    source: Box::new(RelationExprIR::Join {
                        left: Box::new(RelationExprIR::Bind {
                            source: Box::new(RelationExprIR::Relation(relation)),
                            bindings: vec![PortBinding::new(name("left"), form)],
                        }),
                        right: Box::new(RelationExprIR::Relation(relation)),
                    }),
                    ports: vec![name("left")],
                }),
                ports: vec![name("hidden")],
            }),
            renames: vec![ic_core::PortRename::new(name("left"), name("renamed"))],
        }),
        guard,
    };
    let artifact = RelationExprArtifact::new(expression);
    let envelope = artifact.envelope().expect("expression must encode");
    assert_eq!(
        artifact
            .relation_expr_ref()
            .expect("expression must hash")
            .as_artifact_ref(),
        envelope.artifact_ref().expect("expression must hash")
    );
    assert_eq!(
        RelationExprArtifact::from_envelope(&envelope).expect("expression must decode"),
        artifact
    );
    assert_eq!(
        artifact.referenced_artifacts(),
        vec![
            relation.as_artifact_ref(),
            form.as_artifact_ref(),
            relation.as_artifact_ref(),
            guard.as_artifact_ref(),
        ]
    );
}

#[test]
fn formula_defined_relation_requires_the_exact_named_port_context() {
    let binding = binding(0x11);
    let mut catalog = Catalog::default();
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let formula = catalog.insert_formula(FormulaArtifact::new(binding, vec![unit], FormulaIR::Top));
    let schema = RelationSchema::new(
        binding,
        vec![port("subject", unit)],
        RelationBodyIR::Formula(formula),
        Vec::new(),
        Vec::new(),
    );

    assert!(schema.check(&catalog).is_ok());

    let mismatched = RelationSchema::new(
        binding,
        Vec::new(),
        RelationBodyIR::Formula(formula),
        Vec::new(),
        Vec::new(),
    );
    assert!(matches!(
        mismatched.check(&catalog),
        Err(RelationCheckError::FormulaContextMismatch { .. })
    ));
}

#[test]
fn rejects_duplicate_ports_and_malformed_relation_encodings() {
    let binding = binding(0x11);
    let mut catalog = Catalog::default();
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let duplicate = RelationSchema::new(
        binding,
        vec![port("value", unit), port("value", unit)],
        RelationBodyIR::BindingNative {
            contract: artifact(0x33),
        },
        Vec::new(),
        Vec::new(),
    );
    assert!(matches!(
        duplicate.check(&catalog),
        Err(RelationCheckError::DuplicatePortName(_))
    ));

    let schema = RelationSchema::new(
        binding,
        vec![port("value", unit)],
        RelationBodyIR::BindingNative {
            contract: artifact(0x33),
        },
        Vec::new(),
        Vec::new(),
    );
    let payload = schema
        .canonical_payload()
        .expect("schema fixture must encode");
    assert!(matches!(
        RelationSchema::decode_payload(&payload[..payload.len() - 1]),
        Err(RelationError::TruncatedPayload)
    ));

    let mut trailing = payload.clone();
    trailing.push(0);
    assert!(matches!(
        RelationSchema::decode_payload(&trailing),
        Err(RelationError::TrailingPayloadBytes(1))
    ));

    let wrong_kind = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("ic.formula").expect("kind must be valid"),
        1,
        payload,
    );
    assert!(matches!(
        RelationSchema::from_envelope(&wrong_kind),
        Err(RelationError::UnexpectedArtifactKind { .. })
    ));
}

#[test]
fn relation_use_is_a_distinct_typed_and_scoped_occurrence() {
    let binding = binding(0x11);
    let mut catalog = Catalog::default();
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let form_ref = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x22)));
    let schema_ref = catalog.insert_schema(RelationSchema::new(
        binding,
        vec![port("subject", unit)],
        RelationBodyIR::BindingNative {
            contract: artifact(0x33),
        },
        Vec::new(),
        Vec::new(),
    ));
    let context = RelationUseContext::new(
        ScopeRef::from_artifact_ref(artifact(0x44)),
        ApplicabilityRef::from_artifact_ref(artifact(0x55)),
        GrainRef::from_artifact_ref(artifact(0x66)),
        HorizonRef::from_artifact_ref(artifact(0x77)),
        DischargeMode::Probe,
        SupportRef::from_artifact_ref(artifact(0x88)),
        None,
    );
    let occurrence = RelationUse::new(
        schema_ref,
        vec![PortBinding::new(
            TypeSymbol::new("subject").expect("port name must be valid"),
            form_ref,
        )],
        context,
    );

    assert_eq!(
        RelationUse::from_envelope(&occurrence.envelope().expect("use must encode"))
            .expect("use must decode"),
        occurrence
    );
    assert!(occurrence.check(&catalog).is_ok());

    let invalid_port = RelationUse::new(
        schema_ref,
        vec![PortBinding::new(
            TypeSymbol::new("unknown").expect("port name must be valid"),
            form_ref,
        )],
        context,
    );
    assert!(matches!(
        invalid_port.check(&catalog),
        Err(RelationUseCheckError::UnknownPort(_))
    ));
}

#[test]
fn departure_witness_check_requires_the_declared_presentation_and_context() {
    let binding = binding(0x11);
    let mut catalog = Catalog::default();
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let source = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x21)));
    let candidate = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x22)));
    let source_answer = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x23)));
    let candidate_answer = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x24)));
    let distinction = DistinctionRef::from_artifact_ref(artifact(0x31));
    let scope = ScopeRef::from_artifact_ref(artifact(0x32));
    let applicability = ApplicabilityRef::from_artifact_ref(artifact(0x33));
    let grain = GrainRef::from_artifact_ref(artifact(0x34));
    let horizon = HorizonRef::from_artifact_ref(artifact(0x35));
    let support = SupportRef::from_artifact_ref(artifact(0x36));
    let presentation = catalog.insert_presentation(DeterminationPresentation::new(
        distinction,
        ic_core::Orientation::X,
        source,
        RelationalWebRef::from_artifact_ref(artifact(0x37)),
        binding,
        scope,
        applicability,
        grain,
        horizon,
        support,
        None,
    ));
    let relation = catalog.insert_schema(RelationSchema::new(
        binding,
        vec![port("left", unit), port("right", unit)],
        RelationBodyIR::BindingNative {
            contract: artifact(0x38),
        },
        Vec::new(),
        Vec::new(),
    ));
    let context = RelationUseContext::new(
        scope,
        applicability,
        grain,
        horizon,
        DischargeMode::Probe,
        support,
        None,
    );
    let source_observation = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("left").expect("port name must be valid"),
                source,
            ),
            PortBinding::new(
                TypeSymbol::new("right").expect("port name must be valid"),
                source_answer,
            ),
        ],
        context,
    ));
    let candidate_observation = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("left").expect("port name must be valid"),
                candidate,
            ),
            PortBinding::new(
                TypeSymbol::new("right").expect("port name must be valid"),
                candidate_answer,
            ),
        ],
        RelationUseContext::new(
            scope,
            applicability,
            grain,
            horizon,
            DischargeMode::Check,
            support,
            None,
        ),
    ));
    let incompatibility = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("left").expect("port name must be valid"),
                source_answer,
            ),
            PortBinding::new(
                TypeSymbol::new("right").expect("port name must be valid"),
                candidate_answer,
            ),
        ],
        RelationUseContext::new(
            scope,
            applicability,
            grain,
            horizon,
            DischargeMode::Warrant,
            support,
            None,
        ),
    ));
    let witness = DepartureWitness::new(
        distinction,
        source,
        candidate,
        presentation,
        source_observation,
        candidate_observation,
        source_answer,
        candidate_answer,
        incompatibility,
        support,
        scope,
        applicability,
        grain,
    );
    assert!(witness.check(&catalog).is_ok());

    let mismatched_witness_support = DepartureWitness::new(
        distinction,
        source,
        candidate,
        presentation,
        source_observation,
        candidate_observation,
        source_answer,
        candidate_answer,
        incompatibility,
        SupportRef::from_artifact_ref(artifact(0x38)),
        scope,
        applicability,
        grain,
    );
    assert!(matches!(
        mismatched_witness_support.check(&catalog),
        Err(DepartureWitnessCheckError::PresentationMismatch("support"))
    ));

    let mismatched_support_observation = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("left").expect("port name must be valid"),
                source,
            ),
            PortBinding::new(
                TypeSymbol::new("right").expect("port name must be valid"),
                source_answer,
            ),
        ],
        RelationUseContext::new(
            scope,
            applicability,
            grain,
            horizon,
            DischargeMode::Probe,
            SupportRef::from_artifact_ref(artifact(0x39)),
            None,
        ),
    ));
    let mismatched_use_support = DepartureWitness::new(
        distinction,
        source,
        candidate,
        presentation,
        mismatched_support_observation,
        candidate_observation,
        source_answer,
        candidate_answer,
        incompatibility,
        support,
        scope,
        applicability,
        grain,
    );
    assert!(matches!(
        mismatched_use_support.check(&catalog),
        Err(DepartureWitnessCheckError::RelationUseSupportMismatch(reference))
            if reference == mismatched_support_observation
    ));

    let wrong_source = DepartureWitness::new(
        distinction,
        candidate,
        candidate,
        presentation,
        source_observation,
        candidate_observation,
        source_answer,
        candidate_answer,
        incompatibility,
        support,
        scope,
        applicability,
        grain,
    );
    assert!(matches!(
        wrong_source.check(&catalog),
        Err(DepartureWitnessCheckError::PresentationMismatch("source"))
    ));

    let wrong_context = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("left").expect("port name must be valid"),
                source,
            ),
            PortBinding::new(
                TypeSymbol::new("right").expect("port name must be valid"),
                source_answer,
            ),
        ],
        RelationUseContext::new(
            ScopeRef::from_artifact_ref(artifact(0x39)),
            applicability,
            grain,
            horizon,
            DischargeMode::Probe,
            support,
            None,
        ),
    ));
    let context_mismatch = DepartureWitness::new(
        distinction,
        source,
        candidate,
        presentation,
        wrong_context,
        candidate_observation,
        source_answer,
        candidate_answer,
        incompatibility,
        support,
        scope,
        applicability,
        grain,
    );
    assert!(matches!(
        context_mismatch.check(&catalog),
        Err(DepartureWitnessCheckError::RelationUseContextMismatch(reference))
            if reference == wrong_context
    ));

    let disconnected = DepartureWitness::new(
        distinction,
        source,
        candidate,
        presentation,
        candidate_observation,
        candidate_observation,
        source_answer,
        candidate_answer,
        incompatibility,
        support,
        scope,
        applicability,
        grain,
    );
    assert!(matches!(
        disconnected.check(&catalog),
        Err(DepartureWitnessCheckError::ClaimedPairNotBound {
            claim: "source observation",
            relation_use,
        }) if relation_use == candidate_observation
    ));

    let disconnected_incompatibility = DepartureWitness::new(
        distinction,
        source,
        candidate,
        presentation,
        source_observation,
        candidate_observation,
        source_answer,
        candidate_answer,
        candidate_observation,
        support,
        scope,
        applicability,
        grain,
    );
    assert!(matches!(
        disconnected_incompatibility.check(&catalog),
        Err(DepartureWitnessCheckError::ClaimedPairNotBound {
            claim: "incompatibility",
            relation_use,
        }) if relation_use == candidate_observation
    ));

    // A generated observation proposes an answer; it never supports one. The
    // pair below differs in exactly one coordinate -- the declared evidence
    // route -- so it isolates that coordinate and nothing else.
    let observation_ports = |left, right| {
        vec![
            PortBinding::new(
                TypeSymbol::new("left").expect("port name must be valid"),
                left,
            ),
            PortBinding::new(
                TypeSymbol::new("right").expect("port name must be valid"),
                right,
            ),
        ]
    };
    let generated_observation = catalog.insert_relation_use(RelationUse::new(
        relation,
        observation_ports(source, source_answer),
        RelationUseContext::new(
            scope,
            applicability,
            grain,
            horizon,
            DischargeMode::Generate,
            support,
            None,
        ),
    ));
    let pure_observation = catalog.insert_relation_use(RelationUse::new(
        relation,
        observation_ports(source, source_answer),
        RelationUseContext::new(
            scope,
            applicability,
            grain,
            horizon,
            DischargeMode::Pure,
            support,
            None,
        ),
    ));
    let witness_through = |observation| {
        DepartureWitness::new(
            distinction,
            source,
            candidate,
            presentation,
            observation,
            candidate_observation,
            source_answer,
            candidate_answer,
            incompatibility,
            support,
            scope,
            applicability,
            grain,
        )
    };

    assert!(matches!(
        witness_through(generated_observation).check(&catalog),
        Err(DepartureWitnessCheckError::GeneratedEvidenceRoute {
            claim: "source observation",
            relation_use,
        }) if relation_use == generated_observation
    ));

    // The rejection must be of generation specifically, not of every route that
    // is not a probe: a Pure derivation from already-standing data is lawful,
    // and a check that admitted only Probe would fail here.
    assert!(witness_through(pure_observation).check(&catalog).is_ok());
}

#[test]
fn tagged_exterior_claim_preserves_use_tag_without_admitting_an_incidence() {
    let binding = binding(0x91);
    let mut catalog = Catalog::default();
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let source = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x92)));
    let candidate = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x93)));
    let source_answer = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x94)));
    let candidate_answer = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x95)));
    let distinction = DistinctionRef::from_artifact_ref(artifact(0x96));
    let scope = ScopeRef::from_artifact_ref(artifact(0x97));
    let applicability = ApplicabilityRef::from_artifact_ref(artifact(0x98));
    let grain = GrainRef::from_artifact_ref(artifact(0x99));
    let horizon = HorizonRef::from_artifact_ref(artifact(0x9a));
    let support = SupportRef::from_artifact_ref(artifact(0x9b));
    let presentation = catalog.insert_presentation(DeterminationPresentation::new(
        distinction,
        ic_core::Orientation::X,
        source,
        RelationalWebRef::from_artifact_ref(artifact(0x9c)),
        binding,
        scope,
        applicability,
        grain,
        horizon,
        support,
        None,
    ));
    let relation = catalog.insert_schema(RelationSchema::new(
        binding,
        vec![port("left", unit), port("right", unit)],
        RelationBodyIR::BindingNative {
            contract: artifact(0x9d),
        },
        Vec::new(),
        Vec::new(),
    ));
    let context = RelationUseContext::new(
        scope,
        applicability,
        grain,
        horizon,
        DischargeMode::Probe,
        support,
        None,
    );
    let source_observation = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("left").expect("port name must be valid"),
                source,
            ),
            PortBinding::new(
                TypeSymbol::new("right").expect("port name must be valid"),
                source_answer,
            ),
        ],
        context,
    ));
    let candidate_observation = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("left").expect("port name must be valid"),
                candidate,
            ),
            PortBinding::new(
                TypeSymbol::new("right").expect("port name must be valid"),
                candidate_answer,
            ),
        ],
        context,
    ));
    let incompatibility = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("left").expect("port name must be valid"),
                source_answer,
            ),
            PortBinding::new(
                TypeSymbol::new("right").expect("port name must be valid"),
                candidate_answer,
            ),
        ],
        context,
    ));
    let departure = catalog.insert_departure(DepartureWitness::new(
        distinction,
        source,
        candidate,
        presentation,
        source_observation,
        candidate_observation,
        source_answer,
        candidate_answer,
        incompatibility,
        support,
        scope,
        applicability,
        grain,
    ));
    let soundness =
        catalog.insert_program(IProgArtifact::new(unit, IProgIR::Return { value: source }));
    let first_use = catalog.insert_negation_use(NegationUse::new(
        source_observation,
        distinction,
        ic_core::Orientation::X,
        presentation,
        relation,
        soundness,
        NegationCoverage::CertifiedPartial,
        applicability,
        scope,
        grain,
        horizon,
        vec![artifact(0x9e)],
    ));
    let second_use = catalog.insert_negation_use(NegationUse::new(
        source_observation,
        distinction,
        ic_core::Orientation::X,
        presentation,
        relation,
        soundness,
        NegationCoverage::CertifiedPartial,
        applicability,
        scope,
        grain,
        horizon,
        vec![artifact(0x9f)],
    ));
    let first = TaggedExteriorClaim::new(
        first_use,
        source,
        candidate,
        departure,
        GeneratorCoverageRef::from_artifact_ref(artifact(0xa0)),
    );
    let second = TaggedExteriorClaim::new(
        second_use,
        source,
        candidate,
        departure,
        GeneratorCoverageRef::from_artifact_ref(artifact(0xa1)),
    );

    assert!(first.check(&catalog).is_ok());
    assert!(second.check(&catalog).is_ok());
    assert_ne!(
        first.negation_use(),
        second.negation_use(),
        "the same candidate through different uses remains a tagged pair of claims"
    );
    assert_ne!(
        first.execution_coverage(),
        second.execution_coverage(),
        "occurrence-side execution coverage is not inferred from semantic coverage"
    );

    let wrong_candidate = TaggedExteriorClaim::new(
        first_use,
        source,
        source,
        departure,
        first.execution_coverage(),
    );
    assert!(matches!(
        wrong_candidate.check(&catalog),
        Err(TaggedExteriorClaimError::DepartureWitnessMismatch(
            "candidate"
        ))
    ));

    let forged_use = NegationUseRef::from_artifact_ref(artifact(0xa2));
    catalog.negation_uses.insert(
        forged_use,
        catalog
            .negation_uses
            .get(&first_use)
            .expect("first negation use must be available")
            .clone(),
    );
    let forged = TaggedExteriorClaim::new(
        forged_use,
        source,
        candidate,
        departure,
        first.execution_coverage(),
    );
    assert!(matches!(
        forged.check(&catalog),
        Err(TaggedExteriorClaimError::NegationUseIdentityMismatch {
            reference,
            calculated,
        }) if reference == forged_use && calculated == first_use
    ));

    // Plan section 37 and fixture 56: the seed may carry the same form across the reorientation,
    // and that must not be read as role collapse. The identity seed still has to name a relation
    // use, and that use must bind the form twice -- once as the exterior it was taken at, once as
    // the reciprocal source it becomes.
    let left = TypeSymbol::new("left").expect("port name must be valid");
    let right = TypeSymbol::new("right").expect("port name must be valid");
    let identity_seed_use = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![
            PortBinding::new(left.clone(), candidate),
            PortBinding::new(right.clone(), candidate),
        ],
        context,
    ));
    let identity_seed = SeedReorientation::new(first, identity_seed_use, candidate);
    assert!(identity_seed.check(&catalog).is_ok());
    assert!(identity_seed.is_identity_seed());
    assert_eq!(identity_seed.exterior_form(), candidate);
    assert_eq!(identity_seed.reoriented_source(), candidate);
    assert_eq!(
        identity_seed.exterior().negation_use(),
        first_use,
        "reorientation does not discard the use that produced O_X"
    );

    // The wrong implementation this rejects: treating S_Y == O_X as "no seed needed". A use that
    // mentions the form only once has not said which occurrence is which role.
    let single_binding = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![PortBinding::new(left.clone(), candidate)],
        context,
    ));
    assert!(matches!(
        SeedReorientation::new(first, single_binding, candidate).check(&catalog),
        Err(SeedReorientationError::SeedDoesNotRelateThePair { .. })
    ));

    // A non-identity seed relates two distinct forms.
    let reoriented = catalog.insert_form(TypedForm::new(binding, unit, artifact(0xa3)));
    let bridge_use = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![
            PortBinding::new(left.clone(), candidate),
            PortBinding::new(right, reoriented),
        ],
        context,
    ));
    let bridged = SeedReorientation::new(first, bridge_use, reoriented);
    assert!(bridged.check(&catalog).is_ok());
    assert!(!bridged.is_identity_seed());

    // Both tags survive: one exterior form reoriented under two uses stays two occurrences.
    let through_second = SeedReorientation::new(second, bridge_use, reoriented);
    assert!(through_second.check(&catalog).is_ok());
    assert_ne!(
        bridged.exterior().negation_use(),
        through_second.exterior().negation_use()
    );

    // A generated seed proposes a filling; section 26's "supported typed seed relation" refuses it.
    let generated_seed = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![
            PortBinding::new(left, candidate),
            PortBinding::new(
                TypeSymbol::new("right").expect("port name must be valid"),
                reoriented,
            ),
        ],
        RelationUseContext::new(
            scope,
            applicability,
            grain,
            horizon,
            DischargeMode::Generate,
            support,
            None,
        ),
    ));
    assert!(matches!(
        SeedReorientation::new(first, generated_seed, reoriented).check(&catalog),
        Err(SeedReorientationError::GeneratedSeedRoute(route)) if route == generated_seed
    ));
}

#[test]
fn negation_use_check_requires_one_oriented_presentation_context() {
    let binding = binding(0x11);
    let mut catalog = Catalog::default();
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let source = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x21)));
    let presentation = catalog.insert_presentation(DeterminationPresentation::new(
        DistinctionRef::from_artifact_ref(artifact(0x22)),
        ic_core::Orientation::X,
        source,
        RelationalWebRef::from_artifact_ref(artifact(0x23)),
        binding,
        ScopeRef::from_artifact_ref(artifact(0x24)),
        ApplicabilityRef::from_artifact_ref(artifact(0x25)),
        GrainRef::from_artifact_ref(artifact(0x26)),
        HorizonRef::from_artifact_ref(artifact(0x27)),
        SupportRef::from_artifact_ref(artifact(0x28)),
        None,
    ));
    let relation = catalog.insert_schema(RelationSchema::new(
        binding,
        Vec::new(),
        RelationBodyIR::BindingNative {
            contract: artifact(0x29),
        },
        Vec::new(),
        Vec::new(),
    ));
    let relation_use = catalog.insert_relation_use(RelationUse::new(
        relation,
        Vec::new(),
        RelationUseContext::new(
            ScopeRef::from_artifact_ref(artifact(0x24)),
            ApplicabilityRef::from_artifact_ref(artifact(0x25)),
            GrainRef::from_artifact_ref(artifact(0x26)),
            HorizonRef::from_artifact_ref(artifact(0x27)),
            DischargeMode::Check,
            SupportRef::from_artifact_ref(artifact(0x28)),
            None,
        ),
    ));
    let derivation =
        catalog.insert_program(IProgArtifact::new(unit, IProgIR::Return { value: source }));
    let use_declaration = NegationUse::new(
        relation_use,
        DistinctionRef::from_artifact_ref(artifact(0x22)),
        ic_core::Orientation::X,
        presentation,
        relation,
        derivation,
        NegationCoverage::CertifiedPartial,
        ApplicabilityRef::from_artifact_ref(artifact(0x25)),
        ScopeRef::from_artifact_ref(artifact(0x24)),
        GrainRef::from_artifact_ref(artifact(0x26)),
        HorizonRef::from_artifact_ref(artifact(0x27)),
        Vec::new(),
    );
    assert!(use_declaration.check(&catalog).is_ok());

    let wrong_orientation = NegationUse::new(
        relation_use,
        DistinctionRef::from_artifact_ref(artifact(0x22)),
        ic_core::Orientation::Y,
        presentation,
        relation,
        derivation,
        NegationCoverage::CertifiedPartial,
        ApplicabilityRef::from_artifact_ref(artifact(0x25)),
        ScopeRef::from_artifact_ref(artifact(0x24)),
        GrainRef::from_artifact_ref(artifact(0x26)),
        HorizonRef::from_artifact_ref(artifact(0x27)),
        Vec::new(),
    );
    assert!(matches!(
        wrong_orientation.check(&catalog),
        Err(NegationUseCheckError::PresentationMismatch("orientation"))
    ));
}

#[test]
fn positive_negation_query_binds_the_presented_source_and_opens_the_candidate() {
    let binding = binding(0x11);
    let mut catalog = Catalog::default();
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let source = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x21)));
    let other = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x2A)));
    let scope = ScopeRef::from_artifact_ref(artifact(0x24));
    let applicability = ApplicabilityRef::from_artifact_ref(artifact(0x25));
    let grain = GrainRef::from_artifact_ref(artifact(0x26));
    let horizon = HorizonRef::from_artifact_ref(artifact(0x27));
    let support = SupportRef::from_artifact_ref(artifact(0x28));
    let distinction = DistinctionRef::from_artifact_ref(artifact(0x22));

    let presentation = catalog.insert_presentation(DeterminationPresentation::new(
        distinction,
        ic_core::Orientation::X,
        source,
        RelationalWebRef::from_artifact_ref(artifact(0x23)),
        binding,
        scope,
        applicability,
        grain,
        horizon,
        support,
        None,
    ));
    let relation = catalog.insert_schema(RelationSchema::new(
        binding,
        vec![port("source", unit), port("candidate", unit)],
        RelationBodyIR::BindingNative {
            contract: artifact(0x29),
        },
        Vec::new(),
        Vec::new(),
    ));
    let context = RelationUseContext::new(
        scope,
        applicability,
        grain,
        horizon,
        DischargeMode::Check,
        support,
        None,
    );
    let source_port = TypeSymbol::new("source").expect("port name must be valid");
    let candidate_port = TypeSymbol::new("candidate").expect("port name must be valid");

    let bound_source = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![PortBinding::new(source_port.clone(), source)],
        context,
    ));
    let derivation =
        catalog.insert_program(IProgArtifact::new(unit, IProgIR::Return { value: source }));
    let declare = |relation_use, coverage| {
        NegationUse::new(
            relation_use,
            distinction,
            ic_core::Orientation::X,
            presentation,
            relation,
            derivation,
            coverage,
            applicability,
            scope,
            grain,
            horizon,
            Vec::new(),
        )
    };

    let partial = declare(bound_source, NegationCoverage::CertifiedPartial);
    let question = positive_negation_query(&partial, DischargeMode::Probe, &catalog)
        .expect("a bound source with a free candidate port is a question");

    // The source stays bound: the question asks what is exterior to *this* source, not which
    // pairs the relation happens to relate.
    assert_eq!(question.query().relation(), relation);
    assert!(
        question
            .query()
            .bound_ports()
            .iter()
            .any(|held| held.value() == source)
    );
    assert_eq!(question.query().open_ports().len(), 1);
    assert_eq!(question.query().open_ports()[0].port(), &candidate_port);
    assert_eq!(
        question.query().open_ports()[0].mode(),
        DischargeMode::Probe
    );

    // Plan section 23: the licensing use survives into the occurrence. Section 26: the declared
    // semantic coverage travels with it, so a working relation cannot later be read as closed.
    assert_eq!(
        question.negation_use(),
        partial.negation_use_ref().expect("use must hash")
    );
    assert_eq!(
        question.semantic_coverage(),
        NegationCoverage::CertifiedPartial
    );

    let working = declare(bound_source, NegationCoverage::WorkingOpen);
    let working_question = positive_negation_query(&working, DischargeMode::Probe, &catalog)
        .expect("coverage does not change constructibility");
    assert_ne!(working_question.negation_use(), question.negation_use());
    assert_eq!(
        working_question.semantic_coverage(),
        NegationCoverage::WorkingOpen
    );

    // A relation with every port bound is a proposition, not a question.
    let fully_bound = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![
            PortBinding::new(source_port, source),
            PortBinding::new(candidate_port.clone(), other),
        ],
        context,
    ));
    assert!(matches!(
        positive_negation_query(
            &declare(fully_bound, NegationCoverage::CertifiedPartial),
            DischargeMode::Probe,
            &catalog
        ),
        Err(PositiveNegationQueryError::NoOpenCandidatePort)
    ));

    // The wrong implementation this rejects: opening a well-typed question over the relation
    // without binding the presented source. It still has a nonempty open section and still
    // type-checks, but it is a different question.
    let source_unbound = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![PortBinding::new(candidate_port, other)],
        context,
    ));
    assert!(matches!(
        positive_negation_query(
            &declare(source_unbound, NegationCoverage::CertifiedPartial),
            DischargeMode::Probe,
            &catalog
        ),
        Err(PositiveNegationQueryError::SourceNotBound(unbound)) if unbound == source
    ));
}

#[test]
fn first_order_program_check_rejects_forged_or_result_mismatched_continuations() {
    let binding = binding(0x11);
    let mut catalog = Catalog::default();
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let boolean = catalog.insert_type(TypeArtifact::new(binding, TyIR::Bool));
    let unit_form = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x22)));
    let boolean_form = catalog.insert_form(TypedForm::new(binding, boolean, artifact(0x23)));
    let relation = catalog.insert_schema(RelationSchema::new(
        binding,
        vec![port("subject", unit)],
        RelationBodyIR::BindingNative {
            contract: artifact(0x33),
        },
        Vec::new(),
        Vec::new(),
    ));
    let query = catalog.insert_query(OpenQuery::new(
        relation,
        Vec::new(),
        vec![OpenPort::new(
            TypeSymbol::new("subject").expect("port name must be valid"),
            DischargeMode::Probe,
        )],
        RelationUseContext::new(
            ScopeRef::from_artifact_ref(artifact(0x44)),
            ApplicabilityRef::from_artifact_ref(artifact(0x55)),
            GrainRef::from_artifact_ref(artifact(0x66)),
            HorizonRef::from_artifact_ref(artifact(0x77)),
            DischargeMode::Pure,
            SupportRef::from_artifact_ref(artifact(0x88)),
            None,
        ),
    ));
    let continuation = catalog.insert_program(IProgArtifact::new(
        unit,
        IProgIR::Return { value: unit_form },
    ));
    let checked = IProgArtifact::new(
        unit,
        IProgIR::Ask {
            question: query,
            environment: vec![ProgramBinding::new(
                TypeSymbol::new("standing").expect("environment name must be valid"),
                unit_form,
            )],
            answer_slot: TypeSymbol::new("answer").expect("answer slot must be valid"),
            continuation,
        },
    );
    assert!(checked.check(&catalog).is_ok());

    let wrong_result_continuation = catalog.insert_program(IProgArtifact::new(
        boolean,
        IProgIR::Return {
            value: boolean_form,
        },
    ));
    let mismatched = IProgArtifact::new(
        unit,
        IProgIR::Ask {
            question: query,
            environment: vec![ProgramBinding::new(
                TypeSymbol::new("standing").expect("environment name must be valid"),
                unit_form,
            )],
            answer_slot: TypeSymbol::new("answer").expect("answer slot must be valid"),
            continuation: wrong_result_continuation,
        },
    );
    assert!(matches!(
        mismatched.check(&catalog),
        Err(IProgCheckError::ContinuationResultTypeMismatch { .. })
    ));

    let forged_reference = IProgRef::from_artifact_ref(artifact(0x99));
    catalog.programs.insert(
        forged_reference,
        IProgArtifact::new(unit, IProgIR::Return { value: unit_form }),
    );
    let forged = IProgArtifact::new(
        unit,
        IProgIR::Ask {
            question: query,
            environment: vec![ProgramBinding::new(
                TypeSymbol::new("standing").expect("environment name must be valid"),
                unit_form,
            )],
            answer_slot: TypeSymbol::new("answer").expect("answer slot must be valid"),
            continuation: forged_reference,
        },
    );
    assert!(matches!(
        forged.check(&catalog),
        Err(IProgCheckError::ContinuationReferenceIdentityMismatch { .. })
    ));
}

#[test]
fn open_query_is_a_complete_partition_with_a_nonempty_open_section() {
    let binding = binding(0x11);
    let mut catalog = Catalog::default();
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let boolean = catalog.insert_type(TypeArtifact::new(binding, TyIR::Bool));
    let unit_form = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x22)));
    let boolean_form = catalog.insert_form(TypedForm::new(binding, boolean, artifact(0x23)));
    let schema_ref = catalog.insert_schema(RelationSchema::new(
        binding,
        vec![
            port("known", unit),
            port("answer", boolean),
            port("next", boolean),
        ],
        RelationBodyIR::BindingNative {
            contract: artifact(0x33),
        },
        Vec::new(),
        Vec::new(),
    ));
    let context = RelationUseContext::new(
        ScopeRef::from_artifact_ref(artifact(0x44)),
        ApplicabilityRef::from_artifact_ref(artifact(0x55)),
        GrainRef::from_artifact_ref(artifact(0x66)),
        HorizonRef::from_artifact_ref(artifact(0x77)),
        DischargeMode::Probe,
        SupportRef::from_artifact_ref(artifact(0x88)),
        None,
    );
    let query = OpenQuery::new(
        schema_ref,
        vec![PortBinding::new(
            TypeSymbol::new("known").expect("port name must be valid"),
            unit_form,
        )],
        vec![
            OpenPort::new(
                TypeSymbol::new("answer").expect("port name must be valid"),
                DischargeMode::Probe,
            ),
            OpenPort::new(
                TypeSymbol::new("next").expect("port name must be valid"),
                DischargeMode::Check,
            ),
        ],
        context,
    );

    assert_eq!(
        OpenQuery::from_envelope(&query.envelope().expect("query must encode"))
            .expect("query must decode"),
        query
    );
    assert!(query.check(&catalog).is_ok());
    let unnormalized = OpenQuery::new(
        schema_ref,
        vec![PortBinding::new(
            TypeSymbol::new("known").expect("port name must be valid"),
            unit_form,
        )],
        vec![
            OpenPort::new(
                TypeSymbol::new("next").expect("port name must be valid"),
                DischargeMode::Check,
            ),
            OpenPort::new(
                TypeSymbol::new("answer").expect("port name must be valid"),
                DischargeMode::Probe,
            ),
        ],
        context,
    );
    let normalized = unnormalized
        .normalize(&catalog)
        .expect("well-typed port ordering must normalize");
    assert_eq!(normalized.open_ports()[0].port().as_str(), "answer");
    assert_eq!(normalized.open_ports()[1].port().as_str(), "next");
    assert_eq!(
        normalized
            .normalize(&catalog)
            .expect("normalization is idempotent"),
        normalized
    );
    let query_ref = catalog.insert_query(query.clone());
    let fiber = query
        .completion_fiber_view(&catalog)
        .expect("checked query must admit a derived fiber view");
    assert_eq!(fiber.source(), query_ref);
    assert!(fiber.check(&catalog).is_ok());

    let completion = query
        .plug(
            vec![
                PortBinding::new(
                    TypeSymbol::new("answer").expect("port name must be valid"),
                    boolean_form,
                ),
                PortBinding::new(
                    TypeSymbol::new("next").expect("port name must be valid"),
                    boolean_form,
                ),
            ],
            &catalog,
        )
        .expect("complete typed filling must remain a candidate, not evaluate the relation");
    assert_eq!(
        completion.source(),
        query.query_ref().expect("query must hash")
    );
    assert_eq!(completion.bindings().len(), 3);

    let bound = query
        .bind(
            PortBinding::new(
                TypeSymbol::new("answer").expect("port name must be valid"),
                boolean_form,
            ),
            &catalog,
        )
        .expect("binding one of two open ports must remain a query");
    assert_eq!(bound.open_ports().len(), 1);
    let reopened = bound
        .expose(
            TypeSymbol::new("known").expect("port name must be valid"),
            DischargeMode::Generate,
            &catalog,
        )
        .expect("bound port must be exposable");
    assert_eq!(reopened.open_ports().len(), 2);

    let empty_open = OpenQuery::new(
        schema_ref,
        vec![
            PortBinding::new(
                TypeSymbol::new("known").expect("port name must be valid"),
                unit_form,
            ),
            PortBinding::new(
                TypeSymbol::new("answer").expect("port name must be valid"),
                boolean_form,
            ),
            PortBinding::new(
                TypeSymbol::new("next").expect("port name must be valid"),
                boolean_form,
            ),
        ],
        Vec::new(),
        context,
    );
    assert!(matches!(
        empty_open.check(&catalog),
        Err(OpenQueryCheckError::EmptyOpenPorts)
    ));

    let overlapping = OpenQuery::new(
        schema_ref,
        vec![PortBinding::new(
            TypeSymbol::new("known").expect("port name must be valid"),
            unit_form,
        )],
        vec![OpenPort::new(
            TypeSymbol::new("known").expect("port name must be valid"),
            DischargeMode::Check,
        )],
        context,
    );
    assert!(matches!(
        overlapping.check(&catalog),
        Err(OpenQueryCheckError::DuplicatePort(_))
    ));
}

/// Builds one two-port relation use binding `left` and `right`.
fn pair_use(
    catalog: &mut Catalog,
    relation: RelationRef,
    left: TypedFormRef,
    right: TypedFormRef,
    context: RelationUseContext,
) -> ic_core::RelationUseRef {
    catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("left").expect("port name must be valid"),
                left,
            ),
            PortBinding::new(
                TypeSymbol::new("right").expect("port name must be valid"),
                right,
            ),
        ],
        context,
    ))
}

#[test]
fn sixfold_roles_are_generated_dependently_and_gamma_stays_downstream() {
    let binding = binding(0xc1);
    let mut catalog = Catalog::default();
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let source_x = catalog.insert_form(TypedForm::new(binding, unit, artifact(0xc2)));
    let exterior_x = catalog.insert_form(TypedForm::new(binding, unit, artifact(0xc3)));
    let source_y = catalog.insert_form(TypedForm::new(binding, unit, artifact(0xc4)));
    let exterior_y = catalog.insert_form(TypedForm::new(binding, unit, artifact(0xc5)));
    let answer_a = catalog.insert_form(TypedForm::new(binding, unit, artifact(0xc6)));
    let answer_b = catalog.insert_form(TypedForm::new(binding, unit, artifact(0xc7)));
    let other_y_source = catalog.insert_form(TypedForm::new(binding, unit, artifact(0xc8)));

    let distinction = DistinctionRef::from_artifact_ref(artifact(0xc9));
    let scope = ScopeRef::from_artifact_ref(artifact(0xca));
    let applicability = ApplicabilityRef::from_artifact_ref(artifact(0xcb));
    let grain = GrainRef::from_artifact_ref(artifact(0xcc));
    let horizon = HorizonRef::from_artifact_ref(artifact(0xcd));
    let support = SupportRef::from_artifact_ref(artifact(0xce));
    let context = RelationUseContext::new(
        scope,
        applicability,
        grain,
        horizon,
        DischargeMode::Probe,
        support,
        None,
    );
    let relation = catalog.insert_schema(RelationSchema::new(
        binding,
        vec![port("left", unit), port("right", unit)],
        RelationBodyIR::BindingNative {
            contract: artifact(0xcf),
        },
        Vec::new(),
        Vec::new(),
    ));

    let presentation_x = catalog.insert_presentation(DeterminationPresentation::new(
        distinction,
        ic_core::Orientation::X,
        source_x,
        RelationalWebRef::from_artifact_ref(artifact(0xd0)),
        binding,
        scope,
        applicability,
        grain,
        horizon,
        support,
        None,
    ));
    let presentation_y = catalog.insert_presentation(DeterminationPresentation::new(
        distinction,
        ic_core::Orientation::Y,
        source_y,
        RelationalWebRef::from_artifact_ref(artifact(0xd0)),
        binding,
        scope,
        applicability,
        grain,
        horizon,
        support,
        None,
    ));

    let x_source_obs = pair_use(&mut catalog, relation, source_x, answer_a, context);
    let x_cand_obs = pair_use(&mut catalog, relation, exterior_x, answer_b, context);
    let x_incompat = pair_use(&mut catalog, relation, answer_a, answer_b, context);
    let departure_x = catalog.insert_departure(DepartureWitness::new(
        distinction,
        source_x,
        exterior_x,
        presentation_x,
        x_source_obs,
        x_cand_obs,
        answer_a,
        answer_b,
        x_incompat,
        support,
        scope,
        applicability,
        grain,
    ));
    let y_source_obs = pair_use(&mut catalog, relation, source_y, answer_a, context);
    let y_cand_obs = pair_use(&mut catalog, relation, exterior_y, answer_b, context);
    let departure_y = catalog.insert_departure(DepartureWitness::new(
        distinction,
        source_y,
        exterior_y,
        presentation_y,
        y_source_obs,
        y_cand_obs,
        answer_a,
        answer_b,
        x_incompat,
        support,
        scope,
        applicability,
        grain,
    ));

    let soundness = catalog.insert_program(IProgArtifact::new(
        unit,
        IProgIR::Return { value: source_x },
    ));
    let use_x = catalog.insert_negation_use(NegationUse::new(
        x_source_obs,
        distinction,
        ic_core::Orientation::X,
        presentation_x,
        relation,
        soundness,
        NegationCoverage::CertifiedPartial,
        applicability,
        scope,
        grain,
        horizon,
        vec![artifact(0xd1)],
    ));
    let use_y = catalog.insert_negation_use(NegationUse::new(
        y_source_obs,
        distinction,
        ic_core::Orientation::Y,
        presentation_y,
        relation,
        soundness,
        NegationCoverage::CertifiedPartial,
        applicability,
        scope,
        grain,
        horizon,
        vec![artifact(0xd2)],
    ));

    let claim_x = TaggedExteriorClaim::new(
        use_x,
        source_x,
        exterior_x,
        departure_x,
        GeneratorCoverageRef::from_artifact_ref(artifact(0xd3)),
    );
    let claim_y = TaggedExteriorClaim::new(
        use_y,
        source_y,
        exterior_y,
        departure_y,
        GeneratorCoverageRef::from_artifact_ref(artifact(0xd4)),
    );
    assert!(claim_x.check(&catalog).is_ok());
    assert!(claim_y.check(&catalog).is_ok());

    let seed_use = pair_use(&mut catalog, relation, exterior_x, source_y, context);
    let seed = SeedReorientation::new(claim_x, seed_use, source_y);
    assert!(seed.check(&catalog).is_ok());

    let x_extension = FiniteNegationExtension::new(
        use_x,
        vec![(source_x.as_artifact_ref(), exterior_x.as_artifact_ref())],
    )
    .expect("unique incidences");
    let x_fiber = exact_return_fiber(&x_extension, exterior_x.as_artifact_ref()).expect("declared");
    let y_extension = FiniteNegationExtension::new(
        use_y,
        vec![
            (source_y.as_artifact_ref(), exterior_y.as_artifact_ref()),
            (
                other_y_source.as_artifact_ref(),
                exterior_y.as_artifact_ref(),
            ),
        ],
    )
    .expect("unique incidences");
    let y_fiber = exact_return_fiber(&y_extension, exterior_y.as_artifact_ref()).expect("declared");

    let occurrence = ReciprocalOccurrence::new(
        seed,
        x_fiber.clone(),
        Some(source_x.as_artifact_ref()),
        claim_y,
        y_fiber.clone(),
        Some(source_y.as_artifact_ref()),
    )
    .expect("selections are drawn from their fibers");
    assert!(occurrence.check(&catalog).is_ok());
    assert_eq!(occurrence.source_x(), source_x);
    assert_eq!(occurrence.exterior_x(), exterior_x);
    assert_eq!(occurrence.source_y(), source_y);
    assert_eq!(occurrence.exterior_y(), exterior_y);

    // Fixture 53: the Y side continues from the seeded source. An unrelated claim is refused,
    // which is what makes this an occurrence rather than six independent openings.
    let unrelated = ReciprocalOccurrence::new(
        seed,
        x_fiber.clone(),
        Some(source_x.as_artifact_ref()),
        claim_x,
        y_fiber.clone(),
        None,
    )
    .expect("membership still holds");
    assert!(matches!(
        unrelated.check(&catalog),
        Err(ReciprocalOccurrenceError::ReciprocalSourceIsNotTheSeededSource { .. })
    ));

    // Fixture 55: R_X arises from the reverse section of that same use, so a fiber under the
    // other use is a different return entirely.
    let foreign_fiber =
        exact_return_fiber(&y_extension, exterior_y.as_artifact_ref()).expect("declared");
    let wrong_use =
        ReciprocalOccurrence::new(seed, foreign_fiber, None, claim_y, y_fiber.clone(), None)
            .expect("no selection to check");
    assert!(matches!(
        wrong_use.check(&catalog),
        Err(ReciprocalOccurrenceError::ReturnFiberUseMismatch("X"))
    ));

    // Fixture 59: Gamma is downstream and may not manufacture a missing filling.
    let without_rx =
        ReciprocalOccurrence::new(seed, x_fiber.clone(), None, claim_y, y_fiber.clone(), None)
            .expect("no selection to check");
    assert!(matches!(
        without_rx.gamma_reachable(),
        Err(GammaError::RoleMissing("R_X"))
    ));
    assert!(occurrence.gamma_reachable().is_ok());
    assert_eq!(
        without_rx.selected_return_x(),
        None,
        "reaching for Gamma did not supply the role it found missing"
    );

    // Fixture 60: a stable X return coexists with an unstable Y return in one occurrence.
    let signature_context =
        SignatureContext::new(binding, scope, applicability, grain, horizon, unit);
    let x_signatures = ExactFiniteSignature::new(
        signature_context,
        vec![(source_x.as_artifact_ref(), artifact(0xe0))],
    )
    .expect("unique domain values");
    let y_signatures = ExactFiniteSignature::new(
        signature_context,
        vec![
            (source_y.as_artifact_ref(), artifact(0xe0)),
            (other_y_source.as_artifact_ref(), artifact(0xe1)),
        ],
    )
    .expect("unique domain values");
    let x_selection = SelectedReturn::select(x_fiber, source_x.as_artifact_ref()).expect("member");
    let y_selection = SelectedReturn::select(y_fiber, source_y.as_artifact_ref()).expect("member");
    assert!(matches!(
        check_return_closure(&x_selection, &x_signatures).expect("domain is the fiber"),
        ReturnClosure::Closed { .. }
    ));
    assert!(matches!(
        check_return_closure(&y_selection, &y_signatures).expect("domain is the fiber"),
        ReturnClosure::Open { .. }
    ));

    // Section 40: identical fillings are protected-equivalent under any horizon; differing ones
    // are undecided here, never different.
    let residuals = occurrence.residuals();
    assert_eq!(residuals[0], RoleComparison::Coincident, "S_X was selected");
    assert_eq!(
        residuals[1],
        RoleComparison::Undecided,
        "O_X and S_Y differ in form; no horizon has been consulted"
    );
    assert_eq!(residuals[2], RoleComparison::Undecided);
    assert_eq!(residuals[3], RoleComparison::Coincident, "S_Y was selected");
    assert_eq!(
        without_rx.residuals()[0],
        RoleComparison::Undecided,
        "an unselected return has not been compared, which is not a failed comparison"
    );
}

#[test]
fn a_typed_negation_extension_checks_each_incidence_against_the_port_it_fills() {
    let binding = binding(0xf1);
    let mut catalog = Catalog::default();
    // The two ports carry DIFFERENT types on purpose. A relation whose source and candidate
    // types coincide would hide a check that compared each form against the wrong port, the same
    // way a symmetric extension would hide image-for-preimage.
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let boolean = catalog.insert_type(TypeArtifact::new(binding, TyIR::Bool));
    let source = catalog.insert_form(TypedForm::new(binding, unit, artifact(0xf2)));
    let candidate = catalog.insert_form(TypedForm::new(binding, boolean, artifact(0xf3)));
    let other_candidate = catalog.insert_form(TypedForm::new(binding, boolean, artifact(0xf4)));
    let wrong_type = catalog.insert_form(TypedForm::new(binding, unit, artifact(0xf5)));

    let distinction = DistinctionRef::from_artifact_ref(artifact(0xf6));
    let scope = ScopeRef::from_artifact_ref(artifact(0xf7));
    let applicability = ApplicabilityRef::from_artifact_ref(artifact(0xf8));
    let grain = GrainRef::from_artifact_ref(artifact(0xf9));
    let horizon = HorizonRef::from_artifact_ref(artifact(0xfa));
    let support = SupportRef::from_artifact_ref(artifact(0xfb));
    let context = RelationUseContext::new(
        scope,
        applicability,
        grain,
        horizon,
        DischargeMode::Probe,
        support,
        None,
    );
    let presentation = catalog.insert_presentation(DeterminationPresentation::new(
        distinction,
        ic_core::Orientation::X,
        source,
        RelationalWebRef::from_artifact_ref(artifact(0xfc)),
        binding,
        scope,
        applicability,
        grain,
        horizon,
        support,
        None,
    ));
    let relation = catalog.insert_schema(RelationSchema::new(
        binding,
        vec![port("source", unit), port("candidate", boolean)],
        RelationBodyIR::BindingNative {
            contract: artifact(0xfd),
        },
        Vec::new(),
        Vec::new(),
    ));
    // The use binds the source port and leaves the candidate port open, exactly as the
    // positive-negation question requires.
    let bound_source = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![PortBinding::new(
            TypeSymbol::new("source").expect("port name must be valid"),
            source,
        )],
        context,
    ));
    let soundness =
        catalog.insert_program(IProgArtifact::new(unit, IProgIR::Return { value: source }));
    let use_ref = catalog.insert_negation_use(NegationUse::new(
        bound_source,
        distinction,
        ic_core::Orientation::X,
        presentation,
        relation,
        soundness,
        NegationCoverage::CertifiedPartial,
        applicability,
        scope,
        grain,
        horizon,
        Vec::new(),
    ));

    let extension = TypedFiniteNegationExtension::declare(
        use_ref,
        vec![(source, candidate), (source, other_candidate)],
    )
    .expect("unique incidences");
    assert!(extension.check(&catalog).is_ok());

    // The wrong implementation this rejects: checking each form against the other port. With
    // Unit and Bool ports a swap is visible; with two Unit ports it would not be.
    let swapped = TypedFiniteNegationExtension::declare(use_ref, vec![(candidate, source)])
        .expect("unique incidences");
    assert!(matches!(
        swapped.check(&catalog),
        Err(TypedNegationExtensionError::IncidenceTypeMismatch { role: "source", .. })
    ));

    let mistyped = TypedFiniteNegationExtension::declare(use_ref, vec![(source, wrong_type)])
        .expect("unique incidences");
    assert!(matches!(
        mistyped.check(&catalog),
        Err(TypedNegationExtensionError::IncidenceTypeMismatch {
            role: "candidate",
            ..
        })
    ));

    // Section 26: the forward section is the candidate field the question ranges over.
    let field = extension.negation_field(source);
    assert_eq!(field.negation_use(), use_ref);
    assert_eq!(field.source(), source);
    assert_eq!(field.candidates().len(), 2);
    assert!(field.contains(candidate));
    assert!(field.contains(other_candidate));

    // An empty field is an empty declared list, not an assertion that the source has no exterior.
    assert!(
        extension
            .negation_field(other_candidate)
            .candidates()
            .is_empty()
    );

    // Image and preimage are the two directions of one declared extension: for every declared
    // incidence the candidate is in the field of its source, and the source is in the fiber of
    // its candidate.
    let erased = extension
        .erase()
        .expect("typed incidences stay unique when erased");
    for (declared_source, declared_candidate) in extension.incidences() {
        assert!(
            extension
                .negation_field(*declared_source)
                .contains(*declared_candidate)
        );
        let fiber = exact_return_fiber(&erased, declared_candidate.as_artifact_ref())
            .expect("a declared incidence has a return");
        assert!(fiber.contains(declared_source.as_artifact_ref()));
        assert_eq!(fiber.use_ref(), use_ref);
    }

    // A forged form reference is refused rather than silently skipped.
    let forged = TypedFormRef::from_artifact_ref(artifact(0xee));
    let forged_extension = TypedFiniteNegationExtension::declare(use_ref, vec![(source, forged)])
        .expect("unique incidences");
    assert!(matches!(
        forged_extension.check(&catalog),
        Err(TypedNegationExtensionError::UnresolvedForm(missing)) if missing == forged
    ));
}

#[test]
fn an_exterior_claim_must_name_an_incidence_its_own_use_declares() {
    let binding = binding(0x71);
    let mut catalog = Catalog::default();
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let source_one = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x72)));
    let source_two = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x73)));
    let candidate_one = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x74)));
    let candidate_two = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x75)));
    let answer_a = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x76)));
    let answer_b = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x77)));

    let distinction = DistinctionRef::from_artifact_ref(artifact(0x78));
    let scope = ScopeRef::from_artifact_ref(artifact(0x79));
    let applicability = ApplicabilityRef::from_artifact_ref(artifact(0x7a));
    let grain = GrainRef::from_artifact_ref(artifact(0x7b));
    let horizon = HorizonRef::from_artifact_ref(artifact(0x7c));
    let support = SupportRef::from_artifact_ref(artifact(0x7d));
    let context = RelationUseContext::new(
        scope,
        applicability,
        grain,
        horizon,
        DischargeMode::Probe,
        support,
        None,
    );
    let presentation = catalog.insert_presentation(DeterminationPresentation::new(
        distinction,
        ic_core::Orientation::X,
        source_one,
        RelationalWebRef::from_artifact_ref(artifact(0x7e)),
        binding,
        scope,
        applicability,
        grain,
        horizon,
        support,
        None,
    ));
    let relation = catalog.insert_schema(RelationSchema::new(
        binding,
        vec![port("source", unit), port("candidate", unit)],
        RelationBodyIR::BindingNative {
            contract: artifact(0x7f),
        },
        Vec::new(),
        Vec::new(),
    ));
    let bound_source = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![PortBinding::new(
            TypeSymbol::new("source").expect("port name must be valid"),
            source_one,
        )],
        context,
    ));
    // The observations `d_A: A ~> C` and the incompatibility are relations in their own right,
    // distinct from the negation relation whose ports are source and candidate.
    let observation = catalog.insert_schema(RelationSchema::new(
        binding,
        vec![port("left", unit), port("right", unit)],
        RelationBodyIR::BindingNative {
            contract: artifact(0x81),
        },
        Vec::new(),
        Vec::new(),
    ));
    let source_obs = pair_use(&mut catalog, observation, source_one, answer_a, context);
    let candidate_obs = pair_use(&mut catalog, observation, candidate_one, answer_b, context);
    let incompat = pair_use(&mut catalog, observation, answer_a, answer_b, context);
    let departure = catalog.insert_departure(DepartureWitness::new(
        distinction,
        source_one,
        candidate_one,
        presentation,
        source_obs,
        candidate_obs,
        answer_a,
        answer_b,
        incompat,
        support,
        scope,
        applicability,
        grain,
    ));
    let soundness = catalog.insert_program(IProgArtifact::new(
        unit,
        IProgIR::Return { value: source_one },
    ));
    let use_ref = catalog.insert_negation_use(NegationUse::new(
        bound_source,
        distinction,
        ic_core::Orientation::X,
        presentation,
        relation,
        soundness,
        NegationCoverage::CertifiedPartial,
        applicability,
        scope,
        grain,
        horizon,
        Vec::new(),
    ));
    let other_use = catalog.insert_negation_use(NegationUse::new(
        bound_source,
        distinction,
        ic_core::Orientation::X,
        presentation,
        relation,
        soundness,
        NegationCoverage::WorkingOpen,
        applicability,
        scope,
        grain,
        horizon,
        Vec::new(),
    ));

    let claim = TaggedExteriorClaim::new(
        use_ref,
        source_one,
        candidate_one,
        departure,
        GeneratorCoverageRef::from_artifact_ref(artifact(0x80)),
    );

    // Two incidences, so the candidate the claim names is not the only one in the extension.
    let extension = TypedFiniteNegationExtension::declare(
        use_ref,
        vec![(source_one, candidate_one), (source_two, candidate_two)],
    )
    .expect("unique incidences");
    check_declared_incidence(&claim, &extension, &catalog)
        .expect("the declared incidence must check");

    // The wrong implementation this rejects: looking for the candidate anywhere in the extension
    // rather than paired with this source. `candidate_two` is declared exterior to `source_two`,
    // which says nothing about `source_one`.
    let crossed = TaggedExteriorClaim::new(
        use_ref,
        source_one,
        candidate_two,
        departure,
        GeneratorCoverageRef::from_artifact_ref(artifact(0x80)),
    );
    // A claim whose candidate disagrees with its own witness fails before the field is reached.
    assert!(matches!(
        check_declared_incidence(&crossed, &extension, &catalog),
        Err(DeclaredIncidenceError::Claim(_))
    ));

    // The discriminating case. The claim is internally consistent, and its candidate DOES appear
    // in the extension -- paired with the other source. An implementation that looked for the
    // candidate anywhere would admit this; the field of `source_one` is empty, so it is refused.
    let undeclared_extension =
        TypedFiniteNegationExtension::declare(use_ref, vec![(source_two, candidate_one)])
            .expect("unique incidences");
    assert!(
        undeclared_extension
            .incidences()
            .iter()
            .any(|(_, candidate)| *candidate == candidate_one),
        "the candidate must be present in the extension for this to discriminate"
    );
    assert!(matches!(
        check_declared_incidence(&claim, &undeclared_extension, &catalog),
        Err(DeclaredIncidenceError::IncidenceNotDeclared {
            declared_source,
            candidate,
        }) if declared_source == source_one && candidate == candidate_one
    ));

    // An extension declared for another use cannot vouch for this claim.
    let foreign =
        TypedFiniteNegationExtension::declare(other_use, vec![(source_one, candidate_one)])
            .expect("unique incidences");
    assert!(matches!(
        check_declared_incidence(&claim, &foreign, &catalog),
        Err(DeclaredIncidenceError::ExtensionIsForAnotherUse { .. })
    ));
}

#[test]
fn completion_candidate_has_canonical_answer_carrier_identity_without_supporting_it() {
    let mut catalog = Catalog::default();
    let binding = binding(0x81);
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let known = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x82)));
    let alternative_known = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x83)));
    let answer = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x84)));
    let relation = catalog.insert_schema(RelationSchema::new(
        binding,
        vec![port("known", unit), port("answer", unit)],
        RelationBodyIR::BindingNative {
            contract: artifact(0x85),
        },
        Vec::new(),
        Vec::new(),
    ));
    let query = OpenQuery::new(
        relation,
        vec![PortBinding::new(
            TypeSymbol::new("known").expect("port name must be valid"),
            known,
        )],
        vec![OpenPort::new(
            TypeSymbol::new("answer").expect("port name must be valid"),
            DischargeMode::Probe,
        )],
        RelationUseContext::new(
            ScopeRef::from_artifact_ref(artifact(0x86)),
            ApplicabilityRef::from_artifact_ref(artifact(0x87)),
            GrainRef::from_artifact_ref(artifact(0x88)),
            HorizonRef::from_artifact_ref(artifact(0x89)),
            DischargeMode::Probe,
            SupportRef::from_artifact_ref(artifact(0x8a)),
            None,
        ),
    );
    catalog.insert_query(query.clone());
    let completion = query
        .plug(
            vec![PortBinding::new(
                TypeSymbol::new("answer").expect("port name must be valid"),
                answer,
            )],
            &catalog,
        )
        .expect("well-typed filling must be constructible");

    let envelope = completion.envelope().expect("candidate must encode");
    assert_eq!(
        CompletionCandidate::from_envelope(&envelope).expect("candidate must decode"),
        completion
    );
    assert!(completion.check(&catalog).is_ok());
    assert_eq!(
        completion.referenced_artifacts(),
        vec![
            query
                .query_ref()
                .expect("query must hash")
                .as_artifact_ref(),
            answer.as_artifact_ref(),
            known.as_artifact_ref(),
        ]
    );

    let changed_query = OpenQuery::new(
        relation,
        vec![PortBinding::new(
            TypeSymbol::new("known").expect("port name must be valid"),
            known,
        )],
        vec![OpenPort::new(
            TypeSymbol::new("answer").expect("port name must be valid"),
            DischargeMode::Check,
        )],
        *query.context(),
    );
    let changed_completion = changed_query
        .plug(
            vec![PortBinding::new(
                TypeSymbol::new("answer").expect("port name must be valid"),
                answer,
            )],
            &catalog,
        )
        .expect("same values under a different question remain a candidate");
    assert_ne!(
        completion
            .completion_candidate_ref()
            .expect("candidate must hash"),
        changed_completion
            .completion_candidate_ref()
            .expect("candidate must hash")
    );

    let mut forged_payload = completion.canonical_payload().expect("payload must encode");
    let final_reference = forged_payload.len() - 32;
    forged_payload[final_reference..]
        .copy_from_slice(alternative_known.as_artifact_ref().as_bytes());
    let forged = CompletionCandidate::decode_payload(&forged_payload)
        .expect("well-formed forged payload must still decode");
    assert!(matches!(
        forged.check(&catalog),
        Err(CompletionCandidateCheckError::BoundValueMismatch { .. })
    ));
    assert!(matches!(
        CompletionCandidate::decode_payload(&forged_payload[..forged_payload.len() - 1]),
        Err(CompletionCandidateError::Query(_))
    ));

    let canonical_payload = completion.canonical_payload().expect("payload must encode");
    let prefix = 36; // QueryRef plus the binding count.
    let first_length = 4 + "answer".len() + 32;
    let mut noncanonical = canonical_payload[..prefix].to_vec();
    noncanonical.extend_from_slice(&canonical_payload[prefix + first_length..]);
    noncanonical.extend_from_slice(&canonical_payload[prefix..prefix + first_length]);
    assert!(matches!(
        CompletionCandidate::decode_payload(&noncanonical),
        Err(CompletionCandidateError::NonCanonicalBindingOrder)
    ));
}

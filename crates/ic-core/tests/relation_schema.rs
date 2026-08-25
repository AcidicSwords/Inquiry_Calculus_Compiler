use std::collections::BTreeMap;

use ic_core::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactKind, ArtifactRef, BindingVersionRef,
    DepartureCatalog, DepartureWitness, DepartureWitnessCheckError, DepartureWitnessRef,
    DeterminationCatalog, DeterminationPresentation, DeterminationPresentationRef, DischargeMode,
    DistinctionRef, FormulaArtifact, FormulaCatalog, FormulaIR, FormulaRef, GeneratorCoverageRef,
    GrainRef, HorizonRef, IProgArtifact, IProgCatalog, IProgCheckError, IProgIR, IProgRef,
    NegationCoverage, NegationUse, NegationUseCheckError, NegationUseRef, OpenPort, OpenQuery,
    OpenQueryCatalog, OpenQueryCheckError, PortBinding, ProgramBinding, RelationBodyIR,
    RelationCatalog, RelationCheckError, RelationError, RelationExprArtifact, RelationExprIR,
    RelationPort, RelationRef, RelationSchema, RelationSignature, RelationUse,
    RelationUseCheckError, RelationUseContext, RelationalWebRef, ScopeRef, SupportRef,
    TaggedExteriorCatalog, TaggedExteriorClaim, TaggedExteriorClaimError, TyIR, TypeArtifact,
    TypeCatalog, TypeFamilyRef, TypeRef, TypeSymbol, TypedForm, TypedFormRef,
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

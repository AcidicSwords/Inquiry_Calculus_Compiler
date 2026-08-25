use std::collections::BTreeMap;

use ic_core::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactKind, ArtifactRef, BindingVersionRef,
    DischargeMode, FormulaArtifact, FormulaCatalog, FormulaIR, FormulaRef, GrainRef, HorizonRef,
    OpenPort, OpenQuery, OpenQueryCatalog, OpenQueryCheckError, PortBinding, RelationBodyIR,
    RelationCatalog, RelationCheckError, RelationError, RelationExprArtifact, RelationExprIR,
    RelationPort, RelationRef, RelationSchema, RelationSignature, RelationUse,
    RelationUseCheckError, RelationUseContext, ScopeRef, SupportRef, TyIR, TypeArtifact,
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
    queries: BTreeMap<ic_core::QueryRef, OpenQuery>,
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

    fn insert_query(&mut self, query: OpenQuery) -> ic_core::QueryRef {
        let reference = query.query_ref().expect("query fixture must encode");
        self.queries.insert(reference, query);
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

impl OpenQueryCatalog for Catalog {
    fn resolve_open_query(&self, reference: ic_core::QueryRef) -> Option<OpenQuery> {
        self.queries.get(&reference).cloned()
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

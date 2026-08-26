use std::collections::BTreeMap;

use ic_core::{
    ApplicabilityRef, ArtifactRef, BindingBridgeCheckError, BindingBridgeError, BindingBridgeIR,
    BindingChangeKind, BindingVersionRef, DischargeMode, FormulaArtifact, FormulaCatalog,
    FormulaRef, GrainRef, HorizonRef, OpenPort, OpenQuery, OpenQueryCatalog, QueryRef,
    RelationBodyIR, RelationCatalog, RelationPort, RelationRef, RelationSchema, RelationSignature,
    ScopeRef, SupportRef, TyIR, TypeArtifact, TypeCatalog, TypeFamilyRef, TypeRef, TypeSymbol,
    TypedForm, TypedFormRef,
};

#[derive(Default)]
struct Catalog {
    types: BTreeMap<TypeRef, TypeArtifact>,
    schemas: BTreeMap<RelationRef, RelationSchema>,
    queries: BTreeMap<QueryRef, OpenQuery>,
}

impl Catalog {
    fn insert_type(&mut self, artifact: TypeArtifact) -> TypeRef {
        let reference = artifact.type_ref().expect("type fixture must encode");
        self.types.insert(reference, artifact);
        reference
    }

    fn insert_schema(&mut self, schema: RelationSchema) -> RelationRef {
        let reference = schema.relation_ref().expect("schema fixture must encode");
        self.schemas.insert(reference, schema);
        reference
    }

    fn insert_query(&mut self, query: OpenQuery) -> QueryRef {
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
    fn resolve_formula(&self, _reference: FormulaRef) -> Option<FormulaArtifact> {
        None
    }

    fn resolve_typed_form(&self, _reference: TypedFormRef) -> Option<TypedForm> {
        None
    }

    fn resolve_relation_signature(&self, _reference: RelationRef) -> Option<RelationSignature> {
        None
    }
}

impl RelationCatalog for Catalog {
    fn resolve_relation_schema(&self, reference: RelationRef) -> Option<RelationSchema> {
        self.schemas.get(&reference).cloned()
    }
}

impl OpenQueryCatalog for Catalog {
    fn resolve_open_query(&self, reference: QueryRef) -> Option<OpenQuery> {
        self.queries.get(&reference).cloned()
    }
}

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn query(byte: u8) -> QueryRef {
    QueryRef::from_artifact_ref(artifact(byte))
}

fn context(byte: u8) -> ic_core::RelationUseContext {
    ic_core::RelationUseContext::new(
        ScopeRef::from_artifact_ref(artifact(byte)),
        ApplicabilityRef::from_artifact_ref(artifact(byte + 1)),
        GrainRef::from_artifact_ref(artifact(byte + 2)),
        HorizonRef::from_artifact_ref(artifact(byte + 3)),
        DischargeMode::Check,
        SupportRef::from_artifact_ref(artifact(byte + 4)),
        None,
    )
}

fn open_query(relation: RelationRef, context: ic_core::RelationUseContext) -> OpenQuery {
    OpenQuery::new(
        relation,
        Vec::new(),
        vec![OpenPort::new(
            TypeSymbol::new("value").expect("port name must be valid"),
            DischargeMode::Probe,
        )],
        context,
    )
}

#[test]
fn finite_bridge_keeps_conservative_growth_distinct_from_rebinding() {
    let old = BindingVersionRef::from_artifact_ref(artifact(0x10));
    let new = BindingVersionRef::from_artifact_ref(artifact(0x11));
    let bridge = BindingBridgeIR::new(
        old,
        new,
        BindingChangeKind::ConservativeObservationalExtension,
        vec![(query(0x20), query(0x30))],
        Some(query(0x31)),
    )
    .expect("injective finite transport and external witness must be representable");
    assert_eq!(bridge.transports().get(&query(0x20)), Some(&query(0x30)));
    assert_eq!(bridge.strict_growth_witness(), Some(query(0x31)));
    assert!(matches!(
        BindingBridgeIR::new(
            old,
            new,
            BindingChangeKind::Rebinding,
            vec![(query(0x20), query(0x30))],
            Some(query(0x31)),
        ),
        Err(BindingBridgeError::StrictGrowthRequiresConservativeExtension)
    ));
    assert!(matches!(
        BindingBridgeIR::new(
            old,
            new,
            BindingChangeKind::ConservativeObservationalExtension,
            vec![(query(0x20), query(0x30)), (query(0x21), query(0x30))],
            None,
        ),
        Err(BindingBridgeError::NonInjectiveTargetQuestion(reference)) if reference == query(0x30)
    ));
    assert!(matches!(
        BindingBridgeIR::new(
            old,
            new,
            BindingChangeKind::ConservativeObservationalExtension,
            vec![(query(0x20), query(0x30))],
            Some(query(0x30)),
        ),
        Err(BindingBridgeError::GrowthWitnessIsInTransportImage(reference)) if reference == query(0x30)
    ));
}

#[test]
fn finite_bridge_rechecks_named_questions_against_their_declared_bindings() {
    let source = BindingVersionRef::from_artifact_ref(artifact(0x10));
    let target = BindingVersionRef::from_artifact_ref(artifact(0x11));
    let mut catalog = Catalog::default();
    let source_unit = catalog.insert_type(TypeArtifact::new(source, TyIR::Unit));
    let target_unit = catalog.insert_type(TypeArtifact::new(target, TyIR::Unit));
    let source_relation = catalog.insert_schema(RelationSchema::new(
        source,
        vec![RelationPort::new(
            TypeSymbol::new("value").expect("port name must be valid"),
            source_unit,
        )],
        RelationBodyIR::BindingNative {
            contract: artifact(0x40),
        },
        Vec::new(),
        Vec::new(),
    ));
    let target_relation = catalog.insert_schema(RelationSchema::new(
        target,
        vec![RelationPort::new(
            TypeSymbol::new("value").expect("port name must be valid"),
            target_unit,
        )],
        RelationBodyIR::BindingNative {
            contract: artifact(0x41),
        },
        Vec::new(),
        Vec::new(),
    ));
    let old_query = catalog.insert_query(open_query(source_relation, context(0x50)));
    let new_query = catalog.insert_query(open_query(target_relation, context(0x60)));
    let growth_query = catalog.insert_query(open_query(target_relation, context(0x70)));
    let bridge = BindingBridgeIR::new(
        source,
        target,
        BindingChangeKind::ConservativeObservationalExtension,
        vec![(old_query, new_query)],
        Some(growth_query),
    )
    .expect("a target question outside the finite image is a valid local growth witness");

    bridge
        .check(&catalog)
        .expect("each named query must resolve and match its declared binding");

    let wrong_direction = BindingBridgeIR::new(
        source,
        target,
        BindingChangeKind::DefinitionalExtension,
        vec![(new_query, new_query)],
        None,
    )
    .expect("constructor does not inspect the catalog");
    assert!(matches!(
        wrong_direction.check(&catalog),
        Err(BindingBridgeCheckError::SourceBindingMismatch { question, .. }) if question == new_query
    ));
}

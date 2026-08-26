use std::collections::BTreeMap;

use ic_core::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactKind, ArtifactRef, BackendRef, BindingVersionRef,
    CheckerRef, CostModelRef, CoverageRef, DischargeMode, ExtensionDomainRef, FormulaArtifact,
    FormulaCatalog, FormulaRef, METHOD_CONTRACT_ARTIFACT_KIND, METHOD_CONTRACT_SCHEMA_VERSION,
    MethodContract, MethodContractCheckError, MethodContractError, RelationBodyIR, RelationCatalog,
    RelationPort, RelationRef, RelationSchema, RelationSignature, ResidualSchemaRef, TyIR,
    TypeArtifact, TypeCatalog, TypeFamilyRef, TypeRef, TypeSymbol, TypedForm, TypedFormRef,
};

#[derive(Default)]
struct Catalog {
    types: BTreeMap<TypeRef, TypeArtifact>,
    schemas: BTreeMap<RelationRef, RelationSchema>,
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

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn fixture() -> (Catalog, RelationRef) {
    let binding = BindingVersionRef::from_artifact_ref(artifact(0x10));
    let mut catalog = Catalog::default();
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let relation = catalog.insert_schema(RelationSchema::new(
        binding,
        vec![RelationPort::new(
            TypeSymbol::new("value").expect("port name must be valid"),
            unit,
        )],
        RelationBodyIR::BindingNative {
            contract: artifact(0x11),
        },
        Vec::new(),
        Vec::new(),
    ));
    (catalog, relation)
}

fn contract(relation: RelationRef) -> MethodContract {
    MethodContract::new(
        relation,
        ApplicabilityRef::from_artifact_ref(artifact(0x20)),
        artifact(0x21),
        CoverageRef::from_artifact_ref(artifact(0x22)),
        DischargeMode::Probe,
        ExtensionDomainRef::from_artifact_ref(artifact(0x23)),
        BackendRef::from_artifact_ref(artifact(0x24)),
        Some(CheckerRef::from_artifact_ref(artifact(0x25))),
        Some(CostModelRef::from_artifact_ref(artifact(0x26))),
        vec![
            ResidualSchemaRef::from_artifact_ref(artifact(0x28)),
            ResidualSchemaRef::from_artifact_ref(artifact(0x27)),
        ],
        vec![artifact(0x2a), artifact(0x29)],
    )
    .expect("contract references must canonicalize")
}

#[test]
fn method_contract_is_a_typed_registry_record_without_admission_or_execution() {
    let (catalog, relation) = fixture();
    let method = contract(relation);
    method
        .check(&catalog)
        .expect("implemented relation must rehash and type-check");
    assert_eq!(
        method.failure_schemas(),
        [
            ResidualSchemaRef::from_artifact_ref(artifact(0x27)),
            ResidualSchemaRef::from_artifact_ref(artifact(0x28)),
        ]
    );
    assert_eq!(method.provenance(), [artifact(0x29), artifact(0x2a)]);
    assert_eq!(
        MethodContract::from_envelope(&method.envelope().expect("method must encode"))
            .expect("method must decode"),
        method
    );
    let unchecked_relation = RelationRef::from_artifact_ref(artifact(0x30));
    assert!(matches!(
        contract(unchecked_relation).check(&catalog),
        Err(MethodContractCheckError::UnresolvedRelation(reference)) if reference == unchecked_relation
    ));
}

#[test]
fn method_contract_rejects_duplicate_and_malformed_registry_data() {
    let (_, relation) = fixture();
    assert!(matches!(
        MethodContract::new(
            relation,
            ApplicabilityRef::from_artifact_ref(artifact(0x20)),
            artifact(0x21),
            CoverageRef::from_artifact_ref(artifact(0x22)),
            DischargeMode::Probe,
            ExtensionDomainRef::from_artifact_ref(artifact(0x23)),
            BackendRef::from_artifact_ref(artifact(0x24)),
            None,
            None,
            vec![
                ResidualSchemaRef::from_artifact_ref(artifact(0x27)),
                ResidualSchemaRef::from_artifact_ref(artifact(0x27)),
            ],
            Vec::new(),
        ),
        Err(MethodContractError::DuplicateFailureSchema(reference))
            if reference == ResidualSchemaRef::from_artifact_ref(artifact(0x27))
    ));
    let method = contract(relation);
    let payload = method.canonical_payload().expect("method must encode");
    assert!(matches!(
        MethodContract::decode_payload(&payload[..payload.len() - 1]),
        Err(MethodContractError::TruncatedPayload)
    ));
    let mut malformed = payload;
    malformed[128] = 0xff;
    assert!(matches!(
        MethodContract::decode_payload(&malformed),
        Err(MethodContractError::UnknownAuthority)
    ));
    let wrong_kind = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("ic.compression-license").expect("kind must be valid"),
        METHOD_CONTRACT_SCHEMA_VERSION,
        method.canonical_payload().expect("method must encode"),
    );
    assert!(matches!(
        MethodContract::from_envelope(&wrong_kind),
        Err(MethodContractError::UnexpectedArtifactKind { .. })
    ));
    let wrong_schema = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new(METHOD_CONTRACT_ARTIFACT_KIND).expect("kind must be valid"),
        METHOD_CONTRACT_SCHEMA_VERSION + 1,
        method.canonical_payload().expect("method must encode"),
    );
    assert!(matches!(
        MethodContract::from_envelope(&wrong_schema),
        Err(MethodContractError::UnsupportedSchemaVersion(_))
    ));
}

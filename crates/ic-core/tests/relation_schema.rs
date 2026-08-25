use std::collections::BTreeMap;

use ic_core::{
    ArtifactEnvelope, ArtifactKind, ArtifactRef, BindingVersionRef, FormulaArtifact,
    FormulaCatalog, FormulaIR, FormulaRef, RelationBodyIR, RelationCheckError, RelationError,
    RelationPort, RelationRef, RelationSchema, RelationSignature, TyIR, TypeArtifact, TypeCatalog,
    TypeFamilyRef, TypeRef, TypeSymbol, TypedForm, TypedFormRef,
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

    fn resolve_typed_form(&self, _reference: TypedFormRef) -> Option<TypedForm> {
        None
    }

    fn resolve_relation_signature(&self, reference: RelationRef) -> Option<RelationSignature> {
        self.signatures.get(&reference).cloned()
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

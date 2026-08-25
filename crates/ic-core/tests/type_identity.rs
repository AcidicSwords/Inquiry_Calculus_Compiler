use std::collections::BTreeMap;

use ic_core::{
    ArtifactEnvelope, ArtifactKind, ArtifactRef, BindingVersionRef, TYPE_ARTIFACT_KIND,
    TYPE_SCHEMA_VERSION, TYPED_FORM_ARTIFACT_KIND, TyIR, TypeArtifact, TypeCatalog, TypeCheckError,
    TypeError, TypeFamilyRef, TypeRef, TypeSymbol, TypedForm,
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
    families: BTreeMap<TypeFamilyRef, (BindingVersionRef, TypeRef)>,
}

impl Catalog {
    fn insert_type(&mut self, type_artifact: TypeArtifact) -> TypeRef {
        let reference = type_artifact.type_ref().expect("type fixture must encode");
        self.types.insert(reference, type_artifact);
        reference
    }
}

impl TypeCatalog for Catalog {
    fn resolve_type(&self, reference: TypeRef) -> Option<TypeArtifact> {
        self.types.get(&reference).cloned()
    }

    fn resolve_family_domain(
        &self,
        reference: TypeFamilyRef,
    ) -> Option<(BindingVersionRef, TypeRef)> {
        self.families.get(&reference).copied()
    }
}

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn binding(byte: u8) -> BindingVersionRef {
    BindingVersionRef::from_artifact_ref(artifact(byte))
}

fn known_vector() -> KnownVector {
    serde_json::from_str(include_str!("../../../fixtures/types/type-v1-named.json"))
        .expect("known vector fixture must be valid JSON")
}

#[test]
fn named_type_matches_independent_canonical_vector() {
    let vector = known_vector();
    let type_artifact = TypeArtifact::new(
        binding(0x11),
        TyIR::Named {
            binding: binding(0x11),
            name: TypeSymbol::new("User").expect("fixture symbol must be valid"),
            version: artifact(0x22),
        },
    );
    let envelope = type_artifact.envelope().expect("fixture must encode");

    assert_eq!(envelope.kind().as_str(), vector.kind);
    assert_eq!(envelope.schema_version(), vector.schema_version);
    assert_eq!(
        hex::encode(envelope.canonical_payload()),
        vector.payload_hex
    );
    assert_eq!(
        hex::encode(envelope.encode().expect("fixture must encode")),
        vector.encoded_hex
    );
    assert_eq!(
        envelope
            .artifact_ref()
            .expect("fixture must hash")
            .to_string(),
        vector.sha256
    );
    assert_eq!(
        TypeArtifact::from_envelope(&envelope).expect("fixture must decode"),
        type_artifact
    );
}

#[test]
fn canonical_type_grammar_round_trips_and_domain_separates() {
    let type_ref = TypeRef::from_artifact_ref(artifact(0x33));
    let family = TypeFamilyRef::from_artifact_ref(artifact(0x44));
    let binding = binding(0x55);
    let variants = vec![
        TyIR::Unit,
        TyIR::Bool,
        TyIR::Nat,
        TyIR::Named {
            binding,
            name: TypeSymbol::new("Alpha").expect("fixture symbol must be valid"),
            version: artifact(0x66),
        },
        TyIR::Product {
            left: type_ref,
            right: type_ref,
        },
        TyIR::Sum {
            left: type_ref,
            right: type_ref,
        },
        TyIR::Sigma {
            domain: type_ref,
            family,
        },
        TyIR::Pi {
            domain: type_ref,
            family,
        },
        TyIR::Finite(type_ref),
        TyIR::List(type_ref),
        TyIR::Raw(type_ref),
        TyIR::Result(type_ref),
        TyIR::Prog(type_ref),
        TyIR::Code(type_ref),
    ];

    for ty in variants {
        let type_artifact = TypeArtifact::new(binding, ty);
        let envelope = type_artifact.envelope().expect("type must encode");
        assert_eq!(
            TypeArtifact::from_envelope(&envelope).expect("type must decode"),
            type_artifact
        );
    }

    let prog = TypeArtifact::new(binding, TyIR::Prog(type_ref));
    let code = TypeArtifact::new(binding, TyIR::Code(type_ref));
    assert_ne!(
        prog.type_ref().expect("program type must hash"),
        code.type_ref().expect("code type must hash")
    );

    let type_envelope = prog.envelope().expect("type must encode");
    let product = TypeArtifact::new(
        binding,
        TyIR::Product {
            left: type_ref,
            right: type_ref,
        },
    );
    assert_eq!(
        product.referenced_artifacts(),
        vec![binding.as_artifact_ref(), artifact(0x33), artifact(0x33)]
    );

    let typed_form = TypedForm::new(binding, type_ref, artifact(0x77));
    let typed_form_envelope = typed_form.envelope().expect("typed form must encode");
    assert_eq!(
        typed_form.referenced_artifacts(),
        [binding.as_artifact_ref(), artifact(0x33), artifact(0x77)]
    );
    assert_eq!(type_envelope.kind().as_str(), TYPE_ARTIFACT_KIND);
    assert_eq!(
        typed_form_envelope.kind().as_str(),
        TYPED_FORM_ARTIFACT_KIND
    );
    assert_ne!(
        type_envelope.artifact_ref().expect("type must hash"),
        typed_form_envelope
            .artifact_ref()
            .expect("typed form must hash")
    );
}

#[test]
fn rejects_malformed_type_and_typed_form_encodings() {
    let type_artifact = TypeArtifact::new(binding(0x11), TyIR::Unit);
    let payload = type_artifact
        .canonical_payload()
        .expect("type fixture must encode");

    assert!(matches!(
        TypeArtifact::decode_payload(&payload[..payload.len() - 1]),
        Err(TypeError::TruncatedPayload)
    ));

    let mut trailing = payload.clone();
    trailing.push(0);
    assert!(matches!(
        TypeArtifact::decode_payload(&trailing),
        Err(TypeError::TrailingPayloadBytes(1))
    ));

    let mut unknown_tag = payload;
    unknown_tag[32] = 0xff;
    assert!(matches!(
        TypeArtifact::decode_payload(&unknown_tag),
        Err(TypeError::UnknownTypeTag(0xff))
    ));

    let wrong_kind = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new(TYPED_FORM_ARTIFACT_KIND).expect("kind must be valid"),
        TYPE_SCHEMA_VERSION,
        type_artifact
            .canonical_payload()
            .expect("type fixture must encode"),
    );
    assert!(matches!(
        TypeArtifact::from_envelope(&wrong_kind),
        Err(TypeError::UnexpectedArtifactKind { .. })
    ));

    let wrong_schema = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new(TYPE_ARTIFACT_KIND).expect("kind must be valid"),
        TYPE_SCHEMA_VERSION + 1,
        type_artifact
            .canonical_payload()
            .expect("type fixture must encode"),
    );
    assert!(matches!(
        TypeArtifact::from_envelope(&wrong_schema),
        Err(TypeError::UnsupportedTypeSchemaVersion(_))
    ));

    let typed_form = TypedForm::new(
        binding(0x11),
        TypeRef::from_artifact_ref(artifact(0x22)),
        artifact(0x33),
    );
    let mut typed_payload = typed_form.canonical_payload();
    typed_payload.push(0);
    assert!(matches!(
        TypedForm::decode_payload(&typed_payload),
        Err(TypeError::TrailingPayloadBytes(1))
    ));

    let wrong_typed_form_schema = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new(TYPED_FORM_ARTIFACT_KIND).expect("kind must be valid"),
        2,
        typed_form.canonical_payload(),
    );
    assert!(matches!(
        TypedForm::from_envelope(&wrong_typed_form_schema),
        Err(TypeError::UnsupportedTypedFormSchemaVersion(2))
    ));
}

#[test]
fn checks_binding_scope_children_and_dependent_family_domain() {
    let scope_binding = binding(0x11);
    let mut catalog = Catalog::default();
    let unit = TypeArtifact::new(scope_binding, TyIR::Unit);
    let unit_ref = catalog.insert_type(unit);
    let program = TypeArtifact::new(scope_binding, TyIR::Prog(unit_ref));
    let program_ref = catalog.insert_type(program.clone());
    let code = TypeArtifact::new(scope_binding, TyIR::Code(unit_ref));
    catalog.insert_type(code);

    let family = TypeFamilyRef::from_artifact_ref(artifact(0x44));
    catalog.families.insert(family, (scope_binding, unit_ref));
    let sigma = TypeArtifact::new(
        scope_binding,
        TyIR::Sigma {
            domain: unit_ref,
            family,
        },
    );

    assert!(program.check(&catalog).is_ok());
    assert!(sigma.check(&catalog).is_ok());
    let typed_form = TypedForm::new(scope_binding, program_ref, artifact(0x88));
    assert!(typed_form.check(&catalog).is_ok());
    assert_eq!(
        TypedForm::from_envelope(&typed_form.envelope().expect("typed form must encode"))
            .expect("typed form must decode"),
        typed_form
    );

    let wrong_binding = binding(0x99);
    let ill_scoped_named = TypeArtifact::new(
        scope_binding,
        TyIR::Named {
            binding: wrong_binding,
            name: TypeSymbol::new("Foreign").expect("fixture symbol must be valid"),
            version: artifact(0xaa),
        },
    );
    assert!(matches!(
        ill_scoped_named.check(&catalog),
        Err(TypeCheckError::NamedBindingMismatch { .. })
    ));

    let ill_scoped_form = TypedForm::new(wrong_binding, program_ref, artifact(0xbb));
    assert!(matches!(
        ill_scoped_form.check(&catalog),
        Err(TypeCheckError::TypedFormBindingMismatch { .. })
    ));

    let missing = TypeArtifact::new(
        scope_binding,
        TyIR::List(TypeRef::from_artifact_ref(artifact(0xcc))),
    );
    assert!(matches!(
        missing.check(&catalog),
        Err(TypeCheckError::UnresolvedType(_))
    ));

    let forged_reference = TypeRef::from_artifact_ref(artifact(0xce));
    catalog.types.insert(
        forged_reference,
        TypeArtifact::new(scope_binding, TyIR::Bool),
    );
    let forged_child = TypeArtifact::new(scope_binding, TyIR::List(forged_reference));
    assert!(matches!(
        forged_child.check(&catalog),
        Err(TypeCheckError::TypeReferenceIdentityMismatch { .. })
    ));

    let wrong_family = TypeArtifact::new(
        scope_binding,
        TyIR::Pi {
            domain: unit_ref,
            family: TypeFamilyRef::from_artifact_ref(artifact(0xdd)),
        },
    );
    assert!(matches!(
        wrong_family.check(&catalog),
        Err(TypeCheckError::UnresolvedTypeFamily(_))
    ));

    let mismatched_family = TypeFamilyRef::from_artifact_ref(artifact(0xde));
    catalog
        .families
        .insert(mismatched_family, (scope_binding, program_ref));
    let wrong_family_domain = TypeArtifact::new(
        scope_binding,
        TyIR::Sigma {
            domain: unit_ref,
            family: mismatched_family,
        },
    );
    assert!(matches!(
        wrong_family_domain.check(&catalog),
        Err(TypeCheckError::FamilyDomainMismatch { .. })
    ));
}

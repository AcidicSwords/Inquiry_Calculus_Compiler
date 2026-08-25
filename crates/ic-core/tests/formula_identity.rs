use std::collections::BTreeMap;

use ic_core::{
    ArtifactEnvelope, ArtifactKind, ArtifactRef, BindingVersionRef, FORMULA_ARTIFACT_KIND,
    FORMULA_SCHEMA_VERSION, FormulaArtifact, FormulaCatalog, FormulaCheckError, FormulaError,
    FormulaIR, FormulaRef, RelationRef, RelationSignature, TermIR, TyIR, TypeArtifact, TypeCatalog,
    TypeFamilyRef, TypeRef, TypedForm, TypedFormRef,
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
    forms: BTreeMap<TypedFormRef, TypedForm>,
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

    fn insert_form(&mut self, artifact: TypedForm) -> TypedFormRef {
        let reference = artifact
            .typed_form_ref()
            .expect("typed-form fixture must encode");
        self.forms.insert(reference, artifact);
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

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn binding(byte: u8) -> BindingVersionRef {
    BindingVersionRef::from_artifact_ref(artifact(byte))
}

fn vector() -> KnownVector {
    serde_json::from_str(include_str!(
        "../../../fixtures/formulas/formula-v1-top.json"
    ))
    .expect("known vector fixture must be valid JSON")
}

#[test]
fn top_formula_matches_independent_canonical_vector() {
    let vector = vector();
    let formula = FormulaArtifact::new(binding(0x11), Vec::new(), FormulaIR::Top);
    let envelope = formula.envelope().expect("formula fixture must encode");

    assert_eq!(envelope.kind().as_str(), vector.kind);
    assert_eq!(envelope.schema_version(), vector.schema_version);
    assert_eq!(
        hex::encode(envelope.canonical_payload()),
        vector.payload_hex
    );
    assert_eq!(
        hex::encode(envelope.encode().expect("formula fixture must encode")),
        vector.encoded_hex
    );
    assert_eq!(
        envelope
            .artifact_ref()
            .expect("formula fixture must hash")
            .to_string(),
        vector.sha256
    );
    assert_eq!(
        FormulaArtifact::from_envelope(&envelope).expect("formula fixture must decode"),
        formula
    );
}

#[test]
fn complete_canonical_formula_surface_round_trips_without_normalization() {
    let binding = binding(0x11);
    let type_ref = TypeRef::from_artifact_ref(artifact(0x22));
    let formula_ref = FormulaRef::from_artifact_ref(artifact(0x33));
    let relation_ref = RelationRef::from_artifact_ref(artifact(0x44));
    let form_ref = TypedFormRef::from_artifact_ref(artifact(0x55));
    let term = TermIR::Form(form_ref);
    let bound = TermIR::Bound {
        index: 0,
        ty: type_ref,
    };
    let variants = vec![
        FormulaIR::Top,
        FormulaIR::Bottom,
        FormulaIR::Atom {
            relation: relation_ref,
            arguments: vec![term],
        },
        FormulaIR::Equal {
            left: term,
            right: bound,
        },
        FormulaIR::And {
            left: formula_ref,
            right: formula_ref,
        },
        FormulaIR::Or {
            left: formula_ref,
            right: formula_ref,
        },
        FormulaIR::Implies {
            premise: formula_ref,
            conclusion: formula_ref,
        },
        FormulaIR::Not(formula_ref),
        FormulaIR::Exists {
            binder: type_ref,
            body: formula_ref,
        },
        FormulaIR::Forall {
            binder: type_ref,
            body: formula_ref,
        },
    ];

    for formula in variants {
        let artifact = FormulaArtifact::new(binding, vec![type_ref], formula);
        let envelope = artifact.envelope().expect("formula fixture must encode");
        assert_eq!(
            FormulaArtifact::from_envelope(&envelope).expect("formula fixture must decode"),
            artifact
        );
    }

    let disjunction = FormulaArtifact::new(
        binding,
        Vec::new(),
        FormulaIR::Or {
            left: formula_ref,
            right: formula_ref,
        },
    );
    let negation = FormulaArtifact::new(binding, Vec::new(), FormulaIR::Not(formula_ref));
    assert_ne!(
        disjunction.formula_ref().expect("formula must hash"),
        negation.formula_ref().expect("formula must hash")
    );
    assert_eq!(
        negation.referenced_artifacts(),
        vec![binding.as_artifact_ref(), formula_ref.as_artifact_ref()]
    );
}

#[test]
fn rejects_malformed_formula_encodings() {
    let formula = FormulaArtifact::new(binding(0x11), Vec::new(), FormulaIR::Top);
    let payload = formula
        .canonical_payload()
        .expect("formula fixture must encode");
    assert!(matches!(
        FormulaArtifact::decode_payload(&payload[..payload.len() - 1]),
        Err(FormulaError::TruncatedPayload)
    ));

    let mut trailing = payload.clone();
    trailing.push(0);
    assert!(matches!(
        FormulaArtifact::decode_payload(&trailing),
        Err(FormulaError::TrailingPayloadBytes(1))
    ));

    let mut unknown_tag = payload;
    unknown_tag[36] = 0xff;
    assert!(matches!(
        FormulaArtifact::decode_payload(&unknown_tag),
        Err(FormulaError::UnknownFormulaTag(0xff))
    ));

    let wrong_kind = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("ic.type").expect("kind must be valid"),
        FORMULA_SCHEMA_VERSION,
        formula
            .canonical_payload()
            .expect("formula fixture must encode"),
    );
    assert!(matches!(
        FormulaArtifact::from_envelope(&wrong_kind),
        Err(FormulaError::UnexpectedArtifactKind { .. })
    ));

    let wrong_schema = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new(FORMULA_ARTIFACT_KIND).expect("kind must be valid"),
        FORMULA_SCHEMA_VERSION + 1,
        formula
            .canonical_payload()
            .expect("formula fixture must encode"),
    );
    assert!(matches!(
        FormulaArtifact::from_envelope(&wrong_schema),
        Err(FormulaError::UnsupportedFormulaSchemaVersion(_))
    ));
}

#[test]
fn checks_typed_terms_capture_safe_quantification_and_contexts() {
    let binding = binding(0x11);
    let mut catalog = Catalog::default();
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let typed_form = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x22)));

    let quantified_body = FormulaArtifact::new(
        binding,
        vec![unit],
        FormulaIR::Equal {
            left: TermIR::Bound { index: 0, ty: unit },
            right: TermIR::Form(typed_form),
        },
    );
    let body_ref = catalog.insert_formula(quantified_body);
    let quantified = FormulaArtifact::new(
        binding,
        Vec::new(),
        FormulaIR::Exists {
            binder: unit,
            body: body_ref,
        },
    );
    assert!(quantified.check_terms(&catalog).is_ok());

    let invalid_bound = FormulaArtifact::new(
        binding,
        Vec::new(),
        FormulaIR::Equal {
            left: TermIR::Bound { index: 0, ty: unit },
            right: TermIR::Form(typed_form),
        },
    );
    assert!(matches!(
        invalid_bound.check_terms(&catalog),
        Err(FormulaCheckError::BoundIndexOutOfRange { .. })
    ));

    let wrong_context_child =
        catalog.insert_formula(FormulaArtifact::new(binding, Vec::new(), FormulaIR::Top));
    let wrong_context_parent =
        FormulaArtifact::new(binding, vec![unit], FormulaIR::Not(wrong_context_child));
    assert!(matches!(
        wrong_context_parent.check_terms(&catalog),
        Err(FormulaCheckError::FormulaContextMismatch { .. })
    ));

    let wrong_equality = FormulaArtifact::new(
        binding,
        vec![unit],
        FormulaIR::Equal {
            left: TermIR::Bound { index: 0, ty: unit },
            right: TermIR::Bound {
                index: 0,
                ty: TypeRef::from_artifact_ref(artifact(0x99)),
            },
        },
    );
    assert!(matches!(
        wrong_equality.check_terms(&catalog),
        Err(FormulaCheckError::BoundTypeMismatch { .. })
    ));
}

#[test]
fn checks_atom_arity_and_types_against_a_resolved_named_signature() {
    let binding = binding(0x11);
    let mut catalog = Catalog::default();
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let boolean = catalog.insert_type(TypeArtifact::new(binding, TyIR::Bool));
    let unit_form = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x22)));
    let boolean_form = catalog.insert_form(TypedForm::new(binding, boolean, artifact(0x33)));
    let relation = RelationRef::from_artifact_ref(artifact(0x44));
    catalog.signatures.insert(
        relation,
        RelationSignature::new(relation, binding, vec![unit]),
    );

    let valid = FormulaArtifact::new(
        binding,
        Vec::new(),
        FormulaIR::Atom {
            relation,
            arguments: vec![TermIR::Form(unit_form)],
        },
    );
    assert!(valid.check(&catalog).is_ok());

    let wrong_arity = FormulaArtifact::new(
        binding,
        Vec::new(),
        FormulaIR::Atom {
            relation,
            arguments: Vec::new(),
        },
    );
    assert!(matches!(
        wrong_arity.check(&catalog),
        Err(FormulaCheckError::AtomArityMismatch { .. })
    ));

    let wrong_type = FormulaArtifact::new(
        binding,
        Vec::new(),
        FormulaIR::Atom {
            relation,
            arguments: vec![TermIR::Form(boolean_form)],
        },
    );
    assert!(matches!(
        wrong_type.check(&catalog),
        Err(FormulaCheckError::AtomArgumentTypeMismatch { .. })
    ));
}

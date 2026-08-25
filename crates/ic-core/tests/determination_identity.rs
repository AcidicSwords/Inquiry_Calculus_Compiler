use std::collections::BTreeMap;

use ic_core::{
    ApplicabilityRef, ArtifactEnvelope, ArtifactKind, ArtifactRef, BindingVersionRef,
    DETERMINATION_PRESENTATION_ARTIFACT_KIND, DETERMINATION_PRESENTATION_SCHEMA_VERSION,
    DeterminationCatalog, DeterminationPresentation, DeterminationPresentationCheckError,
    DeterminationPresentationError, DeterminationPresentationRef, DistinctionRef, FormulaArtifact,
    FormulaCatalog, FormulaRef, GrainRef, HorizonRef, Orientation, RelationRef, RelationSignature,
    RelationalWebRef, ScopeRef, SupportRef, TyIR, TypeArtifact, TypeCatalog, TypeFamilyRef,
    TypeRef, TypedForm, TypedFormRef,
};

#[derive(Default)]
struct Catalog {
    types: BTreeMap<TypeRef, TypeArtifact>,
    forms: BTreeMap<TypedFormRef, TypedForm>,
    presentations: BTreeMap<DeterminationPresentationRef, DeterminationPresentation>,
}

impl Catalog {
    fn insert_type(&mut self, artifact: TypeArtifact) -> TypeRef {
        let reference = artifact.type_ref().expect("type must encode");
        self.types.insert(reference, artifact);
        reference
    }

    fn insert_form(&mut self, form: TypedForm) -> TypedFormRef {
        let reference = form.typed_form_ref().expect("form must encode");
        self.forms.insert(reference, form);
        reference
    }

    fn insert_presentation(
        &mut self,
        presentation: DeterminationPresentation,
    ) -> DeterminationPresentationRef {
        let reference = presentation
            .determination_presentation_ref()
            .expect("presentation must encode");
        self.presentations.insert(reference, presentation);
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

    fn resolve_typed_form(&self, reference: TypedFormRef) -> Option<TypedForm> {
        self.forms.get(&reference).copied()
    }

    fn resolve_relation_signature(&self, _reference: RelationRef) -> Option<RelationSignature> {
        None
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

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn presentation(
    orientation: Orientation,
    predecessor: Option<DeterminationPresentationRef>,
) -> DeterminationPresentation {
    DeterminationPresentation::new(
        DistinctionRef::from_artifact_ref(artifact(0x11)),
        orientation,
        TypedFormRef::from_artifact_ref(artifact(0x22)),
        RelationalWebRef::from_artifact_ref(artifact(0x33)),
        BindingVersionRef::from_artifact_ref(artifact(0x44)),
        ScopeRef::from_artifact_ref(artifact(0x55)),
        ApplicabilityRef::from_artifact_ref(artifact(0x66)),
        GrainRef::from_artifact_ref(artifact(0x77)),
        HorizonRef::from_artifact_ref(artifact(0x88)),
        SupportRef::from_artifact_ref(artifact(0x99)),
        predecessor,
    )
}

#[test]
fn determination_presentations_round_trip_with_explicit_context_and_ancestry() {
    let predecessor = DeterminationPresentationRef::from_artifact_ref(artifact(0xaa));
    let current = presentation(Orientation::X, Some(predecessor));
    let envelope = current.envelope().expect("presentation must encode");
    assert_eq!(
        DeterminationPresentation::from_envelope(&envelope).expect("presentation must decode"),
        current
    );
    assert_eq!(
        current
            .determination_presentation_ref()
            .expect("presentation must hash")
            .as_artifact_ref(),
        envelope.artifact_ref().expect("presentation must hash")
    );
    assert_eq!(
        current.referenced_artifacts(),
        vec![
            artifact(0x11),
            artifact(0x22),
            artifact(0x33),
            artifact(0x44),
            artifact(0x55),
            artifact(0x66),
            artifact(0x77),
            artifact(0x88),
            artifact(0x99),
            artifact(0xaa),
        ]
    );
    assert_ne!(
        current
            .determination_presentation_ref()
            .expect("presentation must hash"),
        presentation(Orientation::Y, Some(predecessor))
            .determination_presentation_ref()
            .expect("presentation must hash")
    );
    assert_ne!(
        current
            .determination_presentation_ref()
            .expect("presentation must hash"),
        presentation(Orientation::X, None)
            .determination_presentation_ref()
            .expect("presentation must hash")
    );
}

#[test]
fn determination_presentations_reject_malformed_encodings() {
    let current = presentation(Orientation::X, None);
    let payload = current.canonical_payload();
    assert!(matches!(
        DeterminationPresentation::decode_payload(&payload[..payload.len() - 1]),
        Err(DeterminationPresentationError::TruncatedPayload)
    ));
    let mut trailing = payload.clone();
    trailing.push(0);
    assert!(matches!(
        DeterminationPresentation::decode_payload(&trailing),
        Err(DeterminationPresentationError::TrailingPayloadBytes(1))
    ));
    let mut orientation = payload;
    orientation[32] = 0xff;
    assert!(matches!(
        DeterminationPresentation::decode_payload(&orientation),
        Err(DeterminationPresentationError::UnknownOrientation)
    ));
    let wrong_kind = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("ic.iprog").expect("kind must be valid"),
        DETERMINATION_PRESENTATION_SCHEMA_VERSION,
        current.canonical_payload(),
    );
    assert!(matches!(
        DeterminationPresentation::from_envelope(&wrong_kind),
        Err(DeterminationPresentationError::UnexpectedArtifactKind { .. })
    ));
    let wrong_schema = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new(DETERMINATION_PRESENTATION_ARTIFACT_KIND).expect("kind must be valid"),
        DETERMINATION_PRESENTATION_SCHEMA_VERSION + 1,
        current.canonical_payload(),
    );
    assert!(matches!(
        DeterminationPresentation::from_envelope(&wrong_schema),
        Err(DeterminationPresentationError::UnsupportedSchemaVersion(_))
    ));
}

#[test]
fn determination_presentation_check_rejects_forged_source_and_incompatible_ancestry() {
    let binding = BindingVersionRef::from_artifact_ref(artifact(0x11));
    let mut catalog = Catalog::default();
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let source = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x22)));
    let predecessor = catalog.insert_presentation(DeterminationPresentation::new(
        DistinctionRef::from_artifact_ref(artifact(0x33)),
        Orientation::X,
        source,
        RelationalWebRef::from_artifact_ref(artifact(0x44)),
        binding,
        ScopeRef::from_artifact_ref(artifact(0x55)),
        ApplicabilityRef::from_artifact_ref(artifact(0x66)),
        GrainRef::from_artifact_ref(artifact(0x77)),
        HorizonRef::from_artifact_ref(artifact(0x88)),
        SupportRef::from_artifact_ref(artifact(0x99)),
        None,
    ));
    let current = DeterminationPresentation::new(
        DistinctionRef::from_artifact_ref(artifact(0x33)),
        Orientation::X,
        source,
        RelationalWebRef::from_artifact_ref(artifact(0xaa)),
        binding,
        ScopeRef::from_artifact_ref(artifact(0x55)),
        ApplicabilityRef::from_artifact_ref(artifact(0x66)),
        GrainRef::from_artifact_ref(artifact(0x77)),
        HorizonRef::from_artifact_ref(artifact(0x88)),
        SupportRef::from_artifact_ref(artifact(0xbb)),
        Some(predecessor),
    );
    assert!(current.check(&catalog).is_ok());

    let incompatible = DeterminationPresentation::new(
        DistinctionRef::from_artifact_ref(artifact(0x33)),
        Orientation::X,
        source,
        RelationalWebRef::from_artifact_ref(artifact(0xaa)),
        binding,
        ScopeRef::from_artifact_ref(artifact(0xcc)),
        ApplicabilityRef::from_artifact_ref(artifact(0x66)),
        GrainRef::from_artifact_ref(artifact(0x77)),
        HorizonRef::from_artifact_ref(artifact(0x88)),
        SupportRef::from_artifact_ref(artifact(0xbb)),
        Some(predecessor),
    );
    assert!(matches!(
        incompatible.check(&catalog),
        Err(DeterminationPresentationCheckError::PredecessorContextMismatch { field: "scope" })
    ));

    let forged_source = TypedFormRef::from_artifact_ref(artifact(0xdd));
    catalog
        .forms
        .insert(forged_source, TypedForm::new(binding, unit, artifact(0x22)));
    let forged = DeterminationPresentation::new(
        DistinctionRef::from_artifact_ref(artifact(0x33)),
        Orientation::X,
        forged_source,
        RelationalWebRef::from_artifact_ref(artifact(0xaa)),
        binding,
        ScopeRef::from_artifact_ref(artifact(0x55)),
        ApplicabilityRef::from_artifact_ref(artifact(0x66)),
        GrainRef::from_artifact_ref(artifact(0x77)),
        HorizonRef::from_artifact_ref(artifact(0x88)),
        SupportRef::from_artifact_ref(artifact(0xbb)),
        None,
    );
    assert!(matches!(
        forged.check(&catalog),
        Err(DeterminationPresentationCheckError::SourceReferenceIdentityMismatch { .. })
    ));
}

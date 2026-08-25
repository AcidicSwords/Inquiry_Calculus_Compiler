use std::collections::BTreeMap;

use ic_core::{
    ArtifactEnvelope, ArtifactKind, ArtifactRef, BindingVersionRef, DecoderRef, IProgRef,
    RESOLUTION_PATH_ARTIFACT_KIND, RESOLUTION_PATH_SCHEMA_VERSION, RelationRef, ResolutionCatalog,
    ResolutionPath, ResolutionPathCheckError, ResolutionPathError, ResolutionPathIR,
    ResolutionPathRef, TyIR, TypeArtifact, TypeCatalog, TypeFamilyRef, TypeRef,
};

#[derive(Default)]
struct Catalog {
    types: BTreeMap<TypeRef, TypeArtifact>,
    paths: BTreeMap<ResolutionPathRef, ResolutionPath>,
}

impl Catalog {
    fn insert_type(&mut self, ty: TypeArtifact) -> TypeRef {
        let reference = ty.type_ref().expect("type must encode");
        self.types.insert(reference, ty);
        reference
    }

    fn insert_path(&mut self, path: ResolutionPath) -> ResolutionPathRef {
        let reference = path.resolution_path_ref().expect("path must encode");
        self.paths.insert(reference, path);
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

impl ResolutionCatalog for Catalog {
    fn resolve_resolution_path(&self, reference: ResolutionPathRef) -> Option<ResolutionPath> {
        self.paths.get(&reference).cloned()
    }
}

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn binding(byte: u8) -> BindingVersionRef {
    BindingVersionRef::from_artifact_ref(artifact(byte))
}

#[test]
fn resolution_paths_round_trip_without_executing_their_route() {
    let input = TypeRef::from_artifact_ref(artifact(0x11));
    let output = TypeRef::from_artifact_ref(artifact(0x22));
    for path in [
        ResolutionPathIR::Identity,
        ResolutionPathIR::Decode {
            decoder: DecoderRef::from_artifact_ref(artifact(0x33)),
        },
        ResolutionPathIR::Relation {
            relation: RelationRef::from_artifact_ref(artifact(0x44)),
        },
        ResolutionPathIR::Compose {
            first: ResolutionPathRef::from_artifact_ref(artifact(0x55)),
            second: ResolutionPathRef::from_artifact_ref(artifact(0x66)),
        },
        ResolutionPathIR::Program {
            program: IProgRef::from_artifact_ref(artifact(0x77)),
        },
    ] {
        let resolution = ResolutionPath::new(input, output, path);
        let envelope = resolution.envelope().expect("path must encode");
        assert_eq!(
            ResolutionPath::from_envelope(&envelope).expect("path must decode"),
            resolution
        );
    }
}

#[test]
fn resolution_paths_check_identity_and_exact_composition_interfaces() {
    let mut catalog = Catalog::default();
    let binding = binding(0x11);
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let boolean = catalog.insert_type(TypeArtifact::new(binding, TyIR::Bool));
    let unit_identity =
        catalog.insert_path(ResolutionPath::new(unit, unit, ResolutionPathIR::Identity));
    let boolean_identity = catalog.insert_path(ResolutionPath::new(
        boolean,
        boolean,
        ResolutionPathIR::Identity,
    ));
    let composed = ResolutionPath::new(
        unit,
        boolean,
        ResolutionPathIR::Compose {
            first: unit_identity,
            second: boolean_identity,
        },
    );
    assert!(matches!(
        composed.check(&catalog),
        Err(ResolutionPathCheckError::ComposeMiddleMismatch { .. })
    ));
    let identity_mismatch = ResolutionPath::new(unit, boolean, ResolutionPathIR::Identity);
    assert!(matches!(
        identity_mismatch.check(&catalog),
        Err(ResolutionPathCheckError::IdentityTypeMismatch { .. })
    ));

    let well_composed = ResolutionPath::new(
        unit,
        unit,
        ResolutionPathIR::Compose {
            first: unit_identity,
            second: unit_identity,
        },
    );
    assert!(well_composed.check(&catalog).is_ok());
}

#[test]
fn resolution_paths_reject_malformed_envelopes() {
    let path = ResolutionPath::new(
        TypeRef::from_artifact_ref(artifact(0x11)),
        TypeRef::from_artifact_ref(artifact(0x11)),
        ResolutionPathIR::Identity,
    );
    let payload = path.canonical_payload();
    assert!(matches!(
        ResolutionPath::decode_payload(&payload[..payload.len() - 1]),
        Err(ResolutionPathError::TruncatedPayload)
    ));
    let mut trailing = payload.clone();
    trailing.push(0);
    assert!(matches!(
        ResolutionPath::decode_payload(&trailing),
        Err(ResolutionPathError::TrailingPayloadBytes(1))
    ));
    let wrong_kind = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("ic.raw-return").expect("kind valid"),
        RESOLUTION_PATH_SCHEMA_VERSION,
        payload,
    );
    assert!(matches!(
        ResolutionPath::from_envelope(&wrong_kind),
        Err(ResolutionPathError::UnexpectedArtifactKind { .. })
    ));
    let wrong_schema = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new(RESOLUTION_PATH_ARTIFACT_KIND).expect("kind valid"),
        RESOLUTION_PATH_SCHEMA_VERSION + 1,
        path.canonical_payload(),
    );
    assert!(matches!(
        ResolutionPath::from_envelope(&wrong_schema),
        Err(ResolutionPathError::UnsupportedSchemaVersion(_))
    ));
}

use std::collections::BTreeMap;

use ic_core::{
    ArtifactRef, BindingVersionRef, FiniteCellComparison, FiniteCellError,
    FiniteIncompatibilityError, FiniteIncompatibilityResult, FiniteIncompatibilityTable,
    FiniteObservation, FormulaArtifact, FormulaCatalog, FormulaRef, RelationRef, RelationSignature,
    TyIR, TypeArtifact, TypeCatalog, TypeFamilyRef, TypeRef, TypedFiniteIncompatibilityError,
    TypedFiniteIncompatibilityResult, TypedFiniteIncompatibilityTable, TypedFiniteObservation,
    TypedForm, TypedFormRef, check_finite_incompatibility, check_typed_finite_incompatibility,
    compare_finite_observation_cells,
};

fn artifact(value: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([value; 32])
}

#[derive(Default)]
struct TypedCatalog {
    types: BTreeMap<TypeRef, TypeArtifact>,
    forms: BTreeMap<TypedFormRef, TypedForm>,
}

impl TypedCatalog {
    fn typed_form(
        &mut self,
        binding: BindingVersionRef,
        ty: TyIR,
        form: ArtifactRef,
    ) -> TypedFormRef {
        let type_artifact = TypeArtifact::new(binding, ty);
        let type_ref = type_artifact.type_ref().expect("type fixture must encode");
        self.types.insert(type_ref, type_artifact);
        let typed_form = TypedForm::new(binding, type_ref, form);
        let reference = typed_form
            .typed_form_ref()
            .expect("typed-form fixture must encode");
        self.forms.insert(reference, typed_form);
        reference
    }
}

impl TypeCatalog for TypedCatalog {
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

impl FormulaCatalog for TypedCatalog {
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

fn binary_cell(value: u8) -> Vec<FiniteObservation> {
    (0..8)
        .map(|coordinate| FiniteObservation::Observed(artifact((value >> coordinate) & 1)))
        .collect()
}

#[test]
fn exact_finite_cell_exclusion_and_separator_existence_coincide_for_65536_pairs() {
    for source in 0_u8..=u8::MAX {
        for candidate in 0_u8..=u8::MAX {
            let result =
                compare_finite_observation_cells(&binary_cell(source), &binary_cell(candidate))
                    .expect("binary cells have equal coordinate counts");
            assert_eq!(
                matches!(result, FiniteCellComparison::Separated(_)),
                source != candidate,
                "source {source} and candidate {candidate}"
            );
        }
    }
}

#[test]
fn unknown_observation_never_becomes_a_positive_separator_or_same_cell_claim() {
    let known = artifact(1);
    assert_eq!(
        compare_finite_observation_cells(
            &[
                FiniteObservation::Unknown,
                FiniteObservation::Observed(known)
            ],
            &[
                FiniteObservation::Observed(artifact(2)),
                FiniteObservation::Observed(known)
            ],
        )
        .expect("cells have equal coordinate counts"),
        FiniteCellComparison::Unknown
    );
}

#[test]
fn a_positive_separator_survives_an_unrelated_unknown_coordinate() {
    let result = compare_finite_observation_cells(
        &[
            FiniteObservation::Unknown,
            FiniteObservation::Observed(artifact(3)),
        ],
        &[
            FiniteObservation::Observed(artifact(4)),
            FiniteObservation::Observed(artifact(5)),
        ],
    )
    .expect("cells have equal coordinate counts");
    assert!(matches!(
        result,
        FiniteCellComparison::Separated(separator)
            if separator.coordinate() == 1
                && separator.source_value() == artifact(3)
                && separator.candidate_value() == artifact(5)
    ));
}

#[test]
fn comparison_rejects_mismatched_coordinate_schemas() {
    assert_eq!(
        compare_finite_observation_cells(&[FiniteObservation::Observed(artifact(1))], &[]),
        Err(FiniteCellError::CoordinateCountMismatch {
            source_coordinates: 1,
            candidate_coordinates: 0,
        })
    );
}

#[test]
fn finite_incompatibility_requires_a_positive_declared_pair() {
    let table = FiniteIncompatibilityTable::new(vec![(artifact(1), artifact(2))])
        .expect("one pair must be valid");

    assert!(matches!(
        check_finite_incompatibility(
            &table,
            FiniteObservation::Observed(artifact(1)),
            FiniteObservation::Observed(artifact(2)),
        ),
        FiniteIncompatibilityResult::Incompatible(witness)
            if witness.source_value() == artifact(1)
                && witness.candidate_value() == artifact(2)
    ));
    assert_eq!(
        check_finite_incompatibility(
            &table,
            FiniteObservation::Observed(artifact(1)),
            FiniteObservation::Observed(artifact(3)),
        ),
        FiniteIncompatibilityResult::NoWitness,
        "a different observed value is not incompatible merely because it differs"
    );
    assert_eq!(
        check_finite_incompatibility(
            &table,
            FiniteObservation::Observed(artifact(2)),
            FiniteObservation::Observed(artifact(1)),
        ),
        FiniteIncompatibilityResult::NoWitness,
        "a finite incompatibility table preserves its declared orientation"
    );
    assert_eq!(
        check_finite_incompatibility(
            &table,
            FiniteObservation::Unknown,
            FiniteObservation::Observed(artifact(2)),
        ),
        FiniteIncompatibilityResult::Unknown
    );
}

#[test]
fn finite_incompatibility_rejects_duplicate_pair_declarations() {
    assert_eq!(
        FiniteIncompatibilityTable::new(vec![
            (artifact(4), artifact(5)),
            (artifact(4), artifact(5))
        ]),
        Err(FiniteIncompatibilityError::DuplicatePair {
            source_value: artifact(4),
            candidate_value: artifact(5),
        })
    );
}

#[test]
fn typed_finite_incompatibility_rehashes_checked_cross_typed_pairs() {
    let mut catalog = TypedCatalog::default();
    let binding = BindingVersionRef::from_artifact_ref(artifact(40));
    let source = catalog.typed_form(binding, TyIR::Bool, artifact(41));
    let candidate = catalog.typed_form(binding, TyIR::Nat, artifact(42));
    let unlisted_candidate = catalog.typed_form(binding, TyIR::Unit, artifact(43));
    let table = TypedFiniteIncompatibilityTable::new(vec![(source, candidate)])
        .expect("one typed pair must be valid");

    assert!(matches!(
        check_typed_finite_incompatibility(
            &table,
            &catalog,
            TypedFiniteObservation::Observed(source),
            TypedFiniteObservation::Observed(candidate),
        ),
        Ok(TypedFiniteIncompatibilityResult::Incompatible(witness))
            if witness.source_value() == source && witness.candidate_value() == candidate
    ));
    assert!(
        matches!(
            check_typed_finite_incompatibility(
                &table,
                &catalog,
                TypedFiniteObservation::Observed(source),
                TypedFiniteObservation::Observed(unlisted_candidate),
            ),
            Ok(TypedFiniteIncompatibilityResult::NoWitness)
        ),
        "a well-typed unlisted value does not become incompatible merely because its type differs"
    );
    assert!(matches!(
        check_typed_finite_incompatibility(
            &table,
            &catalog,
            TypedFiniteObservation::Unknown,
            TypedFiniteObservation::Observed(candidate),
        ),
        Ok(TypedFiniteIncompatibilityResult::Unknown)
    ));
}

#[test]
fn typed_finite_incompatibility_rejects_duplicate_and_forged_declarations() {
    let source = TypedFormRef::from_artifact_ref(artifact(50));
    let candidate = TypedFormRef::from_artifact_ref(artifact(51));
    assert!(matches!(
        TypedFiniteIncompatibilityTable::new(vec![(source, candidate), (source, candidate)]),
        Err(TypedFiniteIncompatibilityError::DuplicatePair {
            source_value,
            candidate_value,
        }) if source_value == source && candidate_value == candidate
    ));

    let mut catalog = TypedCatalog::default();
    let binding = BindingVersionRef::from_artifact_ref(artifact(52));
    let real_source = catalog.typed_form(binding, TyIR::Bool, artifact(53));
    let real_candidate = catalog.typed_form(binding, TyIR::Bool, artifact(54));
    let forged_source = TypedFormRef::from_artifact_ref(artifact(55));
    let table = TypedFiniteIncompatibilityTable::new(vec![(forged_source, real_candidate)])
        .expect("one distinct pair must construct");
    catalog.forms.insert(
        forged_source,
        *catalog
            .forms
            .get(&real_source)
            .expect("real typed form must remain available"),
    );

    assert!(matches!(
        check_typed_finite_incompatibility(
            &table,
            &catalog,
            TypedFiniteObservation::Observed(forged_source),
            TypedFiniteObservation::Observed(real_candidate),
        ),
        Err(TypedFiniteIncompatibilityError::TypedFormReferenceIdentityMismatch {
            role: "table source",
            reference,
            calculated,
        }) if reference == forged_source && calculated == real_source
    ));
}

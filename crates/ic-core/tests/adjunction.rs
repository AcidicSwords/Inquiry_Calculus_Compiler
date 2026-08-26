use std::collections::BTreeMap;

use ic_core::{
    ArtifactRef, BindingVersionRef, ExactFinitePreorder, FiniteAdjunctionCandidate,
    FiniteAdjunctionCatalog, FiniteAdjunctionError, TyIR, TypeArtifact, TypeCatalog, TypeFamilyRef,
    TypeRef, TypedForm, TypedFormRef, admit_finite_adjunction,
};

#[derive(Default)]
struct Catalog {
    types: BTreeMap<TypeRef, TypeArtifact>,
    forms: BTreeMap<TypedFormRef, TypedForm>,
}

impl Catalog {
    fn insert_type(&mut self, ty: TypeArtifact) -> TypeRef {
        let reference = ty.type_ref().expect("fixture type must encode");
        self.types.insert(reference, ty);
        reference
    }

    fn insert_form(&mut self, form: TypedForm) -> TypedFormRef {
        let reference = form.typed_form_ref().expect("fixture form must encode");
        self.forms.insert(reference, form);
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

impl FiniteAdjunctionCatalog for Catalog {
    fn resolve_typed_form(&self, reference: TypedFormRef) -> Option<TypedForm> {
        self.forms.get(&reference).copied()
    }
}

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn chain_comparisons(elements: &[TypedFormRef]) -> Vec<(TypedFormRef, TypedFormRef)> {
    let mut comparisons = Vec::new();
    for (lower_index, lower) in elements.iter().enumerate() {
        for upper in &elements[lower_index..] {
            comparisons.push((*lower, *upper));
        }
    }
    comparisons
}

struct Fixture {
    catalog: Catalog,
    binding: BindingVersionRef,
    left: ExactFinitePreorder,
    right: ExactFinitePreorder,
    left_elements: [TypedFormRef; 3],
    right_elements: [TypedFormRef; 2],
}

fn fixture() -> Fixture {
    let binding = BindingVersionRef::from_artifact_ref(artifact(0x10));
    let mut catalog = Catalog::default();
    let left_type = catalog.insert_type(TypeArtifact::new(binding, TyIR::Nat));
    let right_type = catalog.insert_type(TypeArtifact::new(binding, TyIR::Bool));
    let left_elements = [
        catalog.insert_form(TypedForm::new(binding, left_type, artifact(0x20))),
        catalog.insert_form(TypedForm::new(binding, left_type, artifact(0x21))),
        catalog.insert_form(TypedForm::new(binding, left_type, artifact(0x22))),
    ];
    let right_elements = [
        catalog.insert_form(TypedForm::new(binding, right_type, artifact(0x30))),
        catalog.insert_form(TypedForm::new(binding, right_type, artifact(0x31))),
    ];
    let left = ExactFinitePreorder::new(
        binding,
        left_type,
        left_elements.to_vec(),
        chain_comparisons(&left_elements),
    )
    .expect("left chain is a finite preorder");
    let right = ExactFinitePreorder::new(
        binding,
        right_type,
        right_elements.to_vec(),
        chain_comparisons(&right_elements),
    )
    .expect("right chain is a finite preorder");
    Fixture {
        catalog,
        binding,
        left,
        right,
        left_elements,
        right_elements,
    }
}

#[test]
// Test boundary QADJOINT-001:
// F = a backward-looking or converse-shaped map is admitted as an adjoint without its law.
// C = exact typed domains, total maps, finite preorders, and every law pair are checked.
// Omega/M = one three-element and one two-element finite chain under one binding.
// P/V/E/U = exhaustive pointwise equivalence and a named counterexample; infinite/partial domains,
// non-enumerative laws, and cross-binding transport remain open.
fn finite_adjoint_requires_its_binding_supplied_law_at_every_pair() {
    let fixture = fixture();
    let [l0, l1, l2] = fixture.left_elements;
    let [r0, r1] = fixture.right_elements;
    let forward = vec![(l0, r0), (l1, r1), (l2, r1)];
    let lawful_backward = vec![(r0, l0), (r1, l2)];
    let lawful = FiniteAdjunctionCandidate::new(
        fixture.binding,
        fixture.left.clone(),
        fixture.right.clone(),
        forward.clone(),
        lawful_backward,
    )
    .expect("map declarations are unique");
    let admitted = admit_finite_adjunction(lawful.clone(), &fixture.catalog)
        .expect("the exhaustive law holds on both finite chains");
    assert_eq!(admitted.candidate(), &lawful);

    let converse_shaped_foil = FiniteAdjunctionCandidate::new(
        fixture.binding,
        fixture.left,
        fixture.right,
        forward,
        vec![(r0, l1), (r1, l2)],
    )
    .expect("foil maps remain total and well typed");
    let Err(FiniteAdjunctionError::LawViolation(counterexample)) =
        admit_finite_adjunction(converse_shaped_foil, &fixture.catalog)
    else {
        panic!("a structural backward map must not bypass the pointwise law")
    };
    assert_eq!(counterexample.left(), l1);
    assert_eq!(counterexample.right(), r0);
    assert_eq!(counterexample.forward(), r1);
    assert_eq!(counterexample.backward(), l1);
    assert!(!counterexample.forward_below());
    assert!(counterexample.below_backward());
}

#[test]
fn finite_adjoint_rejects_incomplete_maps_and_non_preorders_before_the_law() {
    let fixture = fixture();
    let [l0, l1, l2] = fixture.left_elements;
    let [r0, r1] = fixture.right_elements;
    let incomplete = FiniteAdjunctionCandidate::new(
        fixture.binding,
        fixture.left.clone(),
        fixture.right.clone(),
        vec![(l0, r0), (l1, r1)],
        vec![(r0, l0), (r1, l2)],
    )
    .expect("the candidate declaration may remain incomplete until checking");
    assert!(matches!(
        admit_finite_adjunction(incomplete, &fixture.catalog),
        Err(FiniteAdjunctionError::MissingForwardValue(missing)) if missing == l2
    ));

    let non_transitive = ExactFinitePreorder::new(
        fixture.binding,
        fixture.left.ty(),
        vec![l0, l1, l2],
        vec![(l0, l0), (l1, l1), (l2, l2), (l0, l1), (l1, l2)],
    )
    .expect("preorder declaration remains unchecked until admission");
    let malformed = FiniteAdjunctionCandidate::new(
        fixture.binding,
        non_transitive,
        fixture.right,
        vec![(l0, r0), (l1, r1), (l2, r1)],
        vec![(r0, l0), (r1, l2)],
    )
    .expect("maps remain structurally declared");
    assert!(matches!(
        admit_finite_adjunction(malformed, &fixture.catalog),
        Err(FiniteAdjunctionError::NonTransitiveComparison {
            lower,
            middle,
            upper,
        }) if lower == l0 && middle == l1 && upper == l2
    ));
}

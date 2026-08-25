use ic_core::{
    ArtifactRef, FiniteCellComparison, FiniteCellError, FiniteIncompatibilityError,
    FiniteIncompatibilityResult, FiniteIncompatibilityTable, FiniteObservation,
    check_finite_incompatibility, compare_finite_observation_cells,
};

fn artifact(value: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([value; 32])
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

use ic_core::{
    ApplicabilityRef, ArtifactRef, BindingVersionRef, ExactFiberRecovery, ExactFiniteSignature,
    FiberRecoveryError, FiniteNegationExtension, GrainRef, HorizonRef, NegationUseRef,
    ReturnClosure, ReturnFiberError, ScopeRef, SelectedReturn, SignatureContext, TypeRef,
    check_fiber_recovery, check_return_closure, exact_return_fiber,
};

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn use_ref(byte: u8) -> NegationUseRef {
    NegationUseRef::from_artifact_ref(artifact(byte))
}

fn context() -> SignatureContext {
    SignatureContext::new(
        BindingVersionRef::from_artifact_ref(artifact(0x10)),
        ScopeRef::from_artifact_ref(artifact(0x11)),
        ApplicabilityRef::from_artifact_ref(artifact(0x12)),
        GrainRef::from_artifact_ref(artifact(0x13)),
        HorizonRef::from_artifact_ref(artifact(0x14)),
        TypeRef::from_artifact_ref(artifact(0x15)),
    )
}

const A: u8 = 0xA1;
const B: u8 = 0xB2;
const C: u8 = 0xC3;
const D: u8 = 0xD4;

/// `b` is deliberately both a source and a candidate, so the image `N[b] = {c}` and the
/// preimage `N^-1[b] = {a, d}` are both nonempty and different. A symmetric extension would
/// let an implementation that returns the image pass unnoticed.
fn asymmetric_extension(tag: u8) -> FiniteNegationExtension {
    FiniteNegationExtension::new(
        use_ref(tag),
        vec![
            (artifact(A), artifact(B)),
            (artifact(B), artifact(C)),
            (artifact(D), artifact(B)),
        ],
    )
    .expect("declared incidences are unique")
}

#[test]
fn every_admitted_incidence_returns_its_source_through_the_same_use() {
    let extension = asymmetric_extension(0x01);

    // Plan fixture 34: for every admitted incidence, the source belongs to the reverse section.
    for (source, candidate) in extension.incidences() {
        let fiber =
            exact_return_fiber(&extension, *candidate).expect("a declared incidence has a return");
        assert!(
            fiber.contains(*source),
            "source {source} is missing from the return fiber of {candidate}"
        );
        assert_eq!(fiber.use_ref(), extension.use_ref());
        assert_eq!(fiber.exterior(), *candidate);
    }

    // The reverse section is the preimage, not the image: taking the return at `b` recovers
    // the sources that reached `b`, never the candidates `b` itself reaches.
    let at_b = exact_return_fiber(&extension, artifact(B)).expect("b is a declared candidate");
    assert_eq!(
        at_b.sources().iter().copied().collect::<Vec<_>>(),
        vec![artifact(A), artifact(D)]
    );
    assert!(!at_b.contains(artifact(C)));

    // Plan fixture 35: source membership does not imply unique return determination.
    assert_eq!(at_b.sources().len(), 2);

    let at_c = exact_return_fiber(&extension, artifact(C)).expect("c is a declared candidate");
    assert_eq!(
        at_c.sources().iter().copied().collect::<Vec<_>>(),
        vec![artifact(B)]
    );
}

#[test]
// Test boundary QCONVERSE-NOT-INVERSE-001:
// F = converse orientation silently selects one source as an inverse.
// C = the exact reverse section retains every compatible source; selection remains separate.
// Omega/M = one exhaustive finite many-to-one extension under one use tag.
// P/V/E/U = exact enumeration and independent set equality; richer/intensional relations and
// incomplete extension coverage remain open.
fn many_to_one_converse_preserves_the_whole_reverse_fiber_without_an_inverse() {
    let extension = asymmetric_extension(0x21);
    let reverse = exact_return_fiber(&extension, artifact(B))
        .expect("the common forward result has a reverse section");

    assert_eq!(reverse.use_ref(), extension.use_ref());
    assert_eq!(reverse.exterior(), artifact(B));
    assert_eq!(
        reverse.sources().iter().copied().collect::<Vec<_>>(),
        vec![artifact(A), artifact(D)],
        "converse is the complete two-member preimage, not a chosen inverse"
    );

    let choose_a =
        SelectedReturn::select(reverse.clone(), artifact(A)).expect("a is one lawful selection");
    let choose_d = SelectedReturn::select(reverse.clone(), artifact(D))
        .expect("d is another lawful selection");
    assert_ne!(choose_a.selected(), choose_d.selected());
    assert_eq!(choose_a.fiber(), &reverse);
    assert_eq!(choose_d.fiber(), &reverse);
    assert_eq!(
        choose_a.fiber().sources().len(),
        2,
        "selecting a return does not prove or create uniqueness"
    );
}

#[test]
fn the_same_exterior_through_two_uses_keeps_two_distinct_returns() {
    let first = asymmetric_extension(0x01);
    let second = FiniteNegationExtension::new(use_ref(0x02), vec![(artifact(C), artifact(B))])
        .expect("declared incidences are unique");

    let through_first = exact_return_fiber(&first, artifact(B)).expect("declared");
    let through_second = exact_return_fiber(&second, artifact(B)).expect("declared");

    // Same exterior form, different uses, different returns: the tag is part of the occurrence
    // and an untagged union would lose exactly this.
    assert_ne!(through_first.use_ref(), through_second.use_ref());
    assert_ne!(through_first.sources(), through_second.sources());
}

#[test]
fn an_undeclared_exterior_has_no_return_and_a_duplicate_incidence_is_refused() {
    let extension = asymmetric_extension(0x01);

    // Refusal reports only that this extension declares no incidence there. It does not report
    // that the candidate is interior, or exterior under some other use.
    assert_eq!(
        exact_return_fiber(&extension, artifact(0xEE)),
        Err(ReturnFiberError::ExteriorHasNoDeclaredIncidence(artifact(
            0xEE
        )))
    );

    assert_eq!(
        FiniteNegationExtension::new(
            use_ref(0x01),
            vec![(artifact(A), artifact(B)), (artifact(A), artifact(B)),],
        ),
        Err(ReturnFiberError::DuplicateIncidence((
            artifact(A),
            artifact(B)
        )))
    );
}

#[test]
fn recovery_is_checked_against_the_derived_fiber_not_an_unconnected_table() {
    let extension = asymmetric_extension(0x01);
    let fiber = exact_return_fiber(&extension, artifact(B)).expect("declared");

    let constant = ExactFiniteSignature::new(
        context(),
        vec![(artifact(A), artifact(9)), (artifact(D), artifact(9))],
    )
    .expect("unique domain values");
    assert_eq!(
        check_fiber_recovery(&fiber, &constant),
        Ok(ExactFiberRecovery::Recovered {
            protected_signature: artifact(9),
        })
    );

    let split = ExactFiniteSignature::new(
        context(),
        vec![(artifact(A), artifact(9)), (artifact(D), artifact(10))],
    )
    .expect("unique domain values");
    assert!(matches!(
        check_fiber_recovery(&fiber, &split),
        Ok(ExactFiberRecovery::NotRecovered { .. })
    ));

    // The defect this closes: a table that silently omits a surviving source used to report
    // recovery, because the caller's word was the only thing connecting it to an incidence.
    let omits_a_source = ExactFiniteSignature::new(context(), vec![(artifact(A), artifact(9))])
        .expect("unique domain values");
    assert!(matches!(
        check_fiber_recovery(&fiber, &omits_a_source),
        Err(FiberRecoveryError::SignatureDomainIsNotTheFiber { .. })
    ));

    let unrelated = ExactFiniteSignature::new(
        context(),
        vec![(artifact(0xF1), artifact(9)), (artifact(0xF2), artifact(9))],
    )
    .expect("unique domain values");
    assert!(matches!(
        check_fiber_recovery(&fiber, &unrelated),
        Err(FiberRecoveryError::SignatureDomainIsNotTheFiber { .. })
    ));
}

#[test]
fn a_stable_selected_return_does_not_close_a_fiber_that_still_splits() {
    let extension = asymmetric_extension(0x01);
    let fiber = exact_return_fiber(&extension, artifact(B)).expect("declared");

    // R_X is drawn from the fiber, so a source the return does not admit is not a return role.
    assert!(matches!(
        SelectedReturn::select(fiber.clone(), artifact(C)),
        Err(ReturnFiberError::SelectionOutsideFiber { selected, .. }) if selected == artifact(C)
    ));

    // Plan fixture 37. `a` is the source, it is selected, and its own protected signature is
    // whatever it is -- entirely stable when read alone. The other surviving source disagrees.
    let selected_source =
        SelectedReturn::select(fiber.clone(), artifact(A)).expect("a survives this return");
    assert_eq!(selected_source.selected(), artifact(A));
    assert_eq!(selected_source.fiber().sources().len(), 2);

    let split = ExactFiniteSignature::new(
        context(),
        vec![(artifact(A), artifact(9)), (artifact(D), artifact(10))],
    )
    .expect("unique domain values");

    // The wrong implementation reads the selection and reports Closed. The verdict comes from
    // every source the return admits, so it is Open, and it names the pair that keeps it open.
    match check_return_closure(&selected_source, &split).expect("domain is the fiber") {
        ReturnClosure::Open { separator } => {
            assert_eq!(separator.first_candidate(), artifact(A));
            assert_eq!(separator.second_candidate(), artifact(D));
            assert_ne!(separator.first_signature(), separator.second_signature());
        }
        ReturnClosure::Closed { .. } => {
            panic!("a second protected class survives this return; it does not close")
        }
    }

    // Constancy over the whole fiber is what closes it.
    let constant = ExactFiniteSignature::new(
        context(),
        vec![(artifact(A), artifact(9)), (artifact(D), artifact(9))],
    )
    .expect("unique domain values");
    assert_eq!(
        check_return_closure(&selected_source, &constant).expect("domain is the fiber"),
        ReturnClosure::Closed {
            protected_signature: artifact(9)
        }
    );

    // A singleton fiber closes, which is why the split fixture above must carry two members:
    // an implementation that read only the selection would pass this case and fail the last.
    let single = exact_return_fiber(&extension, artifact(C)).expect("declared");
    let single_selection =
        SelectedReturn::select(single, artifact(B)).expect("b survives this return");
    let single_signature = ExactFiniteSignature::new(context(), vec![(artifact(B), artifact(9))])
        .expect("unique domain values");
    assert_eq!(
        check_return_closure(&single_selection, &single_signature).expect("domain is the fiber"),
        ReturnClosure::Closed {
            protected_signature: artifact(9)
        }
    );
}

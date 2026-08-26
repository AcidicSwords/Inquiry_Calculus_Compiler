use ic_core::{
    ArtifactEnvelope, ArtifactKind, ArtifactRef, BindingVersionRef, DeclaredFiniteGeneratorRegime,
    DeclaredFiniteGeneratorRegimeError, DeclaredRouteMaterialization, EffectivityRef,
    ExactFiniteRegimeRoute, ExactFiniteRegimeSeparatorError, ExactFiniteRegimeSeparatorResult,
    ExactFiniteSignature, GeneratorRegimeRef, GrainRef, HorizonRef, ProtectedClassRef,
    ProtectedCompletionFieldRef, SEPARATOR_PROBLEM_ARTIFACT_KIND, SEPARATOR_PROBLEM_SCHEMA_VERSION,
    ScopeRef, SeparatorProblem, SeparatorProblemError, SignatureContext, StructureViewRef, TypeRef,
    check_exact_no_separator_within_declared_regime,
};

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn problem(target: Option<ProtectedClassRef>) -> SeparatorProblem {
    SeparatorProblem::new(
        ProtectedCompletionFieldRef::from_artifact_ref(artifact(0x11)),
        target,
        GrainRef::from_artifact_ref(artifact(0x22)),
        HorizonRef::from_artifact_ref(artifact(0x33)),
        BindingVersionRef::from_artifact_ref(artifact(0x44)),
        StructureViewRef::from_artifact_ref(artifact(0x55)),
        GeneratorRegimeRef::from_artifact_ref(artifact(0x66)),
        EffectivityRef::from_artifact_ref(artifact(0x77)),
    )
}

#[test]
fn separator_problem_round_trips_and_keeps_its_generic_residual_context() {
    let target = ProtectedClassRef::from_artifact_ref(artifact(0x88));
    let with_target = problem(Some(target));
    let without_target = problem(None);
    let envelope = with_target.envelope().expect("problem must encode");

    assert_eq!(
        SeparatorProblem::from_envelope(&envelope).expect("problem must decode"),
        with_target
    );
    assert_eq!(
        with_target
            .separator_problem_ref()
            .expect("problem must hash")
            .as_artifact_ref(),
        envelope.artifact_ref().expect("envelope must hash")
    );
    assert_ne!(
        with_target
            .separator_problem_ref()
            .expect("targeted problem must hash"),
        without_target
            .separator_problem_ref()
            .expect("untargeted problem must hash"),
        "an absent target class is not the same residual context as a declared target"
    );
    assert_eq!(
        with_target.referenced_artifacts(),
        vec![
            artifact(0x11),
            artifact(0x88),
            artifact(0x22),
            artifact(0x33),
            artifact(0x44),
            artifact(0x55),
            artifact(0x66),
            artifact(0x77),
        ]
    );
}

#[test]
fn separator_problem_rejects_malformed_and_wrong_domain_encodings() {
    let problem = problem(None);
    let payload = problem.canonical_payload();

    assert!(matches!(
        SeparatorProblem::decode_payload(&payload[..payload.len() - 1]),
        Err(SeparatorProblemError::TruncatedPayload)
    ));
    let mut trailing = payload.clone();
    trailing.push(0);
    assert!(matches!(
        SeparatorProblem::decode_payload(&trailing),
        Err(SeparatorProblemError::TrailingPayloadBytes(1))
    ));
    let mut malformed_option = payload;
    malformed_option[32] = 0xFF;
    assert!(matches!(
        SeparatorProblem::decode_payload(&malformed_option),
        Err(SeparatorProblemError::UnknownOptionalTag(0xFF))
    ));

    let wrong_kind = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new("ic.open-query").expect("kind valid"),
        SEPARATOR_PROBLEM_SCHEMA_VERSION,
        problem.canonical_payload(),
    );
    assert!(matches!(
        SeparatorProblem::from_envelope(&wrong_kind),
        Err(SeparatorProblemError::UnexpectedArtifactKind { .. })
    ));
    let wrong_schema = ArtifactEnvelope::from_canonical_payload(
        ArtifactKind::new(SEPARATOR_PROBLEM_ARTIFACT_KIND).expect("kind valid"),
        SEPARATOR_PROBLEM_SCHEMA_VERSION + 1,
        problem.canonical_payload(),
    );
    assert!(matches!(
        SeparatorProblem::from_envelope(&wrong_schema),
        Err(SeparatorProblemError::UnsupportedSchemaVersion(_))
    ));
}

#[test]
fn declared_finite_generator_regime_keeps_materialization_distinct_from_availability() {
    let regime = GeneratorRegimeRef::from_artifact_ref(artifact(0x61));
    let route_a = artifact(0x62);
    let route_b = artifact(0x63);
    let routes = DeclaredFiniteGeneratorRegime::new(regime, vec![route_b, route_a], vec![route_a])
        .expect("declared regime must canonicalize route membership");
    assert_eq!(routes.routes(), [route_a, route_b]);
    assert_eq!(
        routes.route_status(route_a),
        DeclaredRouteMaterialization::Materialized
    );
    assert_eq!(
        routes.route_status(route_b),
        DeclaredRouteMaterialization::FreshWithinRegime
    );
    assert_eq!(
        routes.route_status(artifact(0x64)),
        DeclaredRouteMaterialization::OutsideDeclaredRegime
    );
    assert!(matches!(
        DeclaredFiniteGeneratorRegime::new(regime, vec![route_a, route_a], Vec::new()),
        Err(DeclaredFiniteGeneratorRegimeError::DuplicateRoute(reference)) if reference == route_a
    ));
    assert!(matches!(
        DeclaredFiniteGeneratorRegime::new(regime, vec![route_a], vec![route_b]),
        Err(DeclaredFiniteGeneratorRegimeError::MaterializedRouteOutsideRegime(reference))
            if reference == route_b
    ));
}

#[test]
fn exact_no_separator_remains_relative_to_one_declared_finite_regime() {
    let regime_ref = GeneratorRegimeRef::from_artifact_ref(artifact(0x71));
    let route_a = artifact(0x72);
    let route_b = artifact(0x73);
    let regime = DeclaredFiniteGeneratorRegime::new(regime_ref, vec![route_a, route_b], Vec::new())
        .expect("finite regime must be valid");
    let context = SignatureContext::new(
        BindingVersionRef::from_artifact_ref(artifact(0x74)),
        ScopeRef::from_artifact_ref(artifact(0x75)),
        ic_core::ApplicabilityRef::from_artifact_ref(artifact(0x76)),
        GrainRef::from_artifact_ref(artifact(0x77)),
        HorizonRef::from_artifact_ref(artifact(0x78)),
        TypeRef::from_artifact_ref(artifact(0x79)),
    );
    let protected = ExactFiniteSignature::new(
        context,
        vec![
            (artifact(0x7a), artifact(0x01)),
            (artifact(0x7b), artifact(0x02)),
        ],
    )
    .expect("protected signature must be exact");
    let constant = ExactFiniteSignature::new(
        context,
        vec![
            (artifact(0x7a), artifact(0x03)),
            (artifact(0x7b), artifact(0x03)),
        ],
    )
    .expect("route signature must be exact");
    let routes = vec![
        ExactFiniteRegimeRoute::new(route_a, constant.clone()),
        ExactFiniteRegimeRoute::new(route_b, constant),
    ];
    assert!(matches!(
        check_exact_no_separator_within_declared_regime(&regime, &routes, &protected),
        Ok(ExactFiniteRegimeSeparatorResult::ExactNoSeparatorWithinRegime { .. })
    ));
    assert!(matches!(
        check_exact_no_separator_within_declared_regime(
            &regime,
            &[ExactFiniteRegimeRoute::new(route_a, routes[0].signature().clone())],
            &protected,
        ),
        Err(ExactFiniteRegimeSeparatorError::MissingRouteSignature(reference)) if reference == route_b
    ));
}

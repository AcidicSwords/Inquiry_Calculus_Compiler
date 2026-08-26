use std::collections::BTreeMap;

use ic_core::{
    ActualDecodeError, ActualDecodeResult, ActualEvent, ActualEventCatalog,
    AdmittedFiniteDeparture, AdmittedFiniteNegationExtension, ApplicabilityRef, ArtifactRef,
    BindingVersionRef, BoundaryChart, BoundaryRef, ClaimArtifact, ClaimError, ClaimRef,
    ClaimStatus, CompletionCandidate, CompletionCandidateCatalog, CompletionCandidateRef,
    DeclaredStandingError, DeclaredSupportClosure, DecodedObservationError, DecodedObservationUse,
    DecoderRef, DepartureCatalog, DepartureEvidenceSupportError, DepartureStandingCheckError,
    DepartureWitness, DeterminationCatalog, DeterminationPresentation,
    DeterminationPresentationRef, DeterminationSupportError, DischargeMode, EffectivityRef,
    EventRef, ExactFiniteSignature, FINITE_DECODER_ARTIFACT_KIND, FINITE_DECODER_SCHEMA_VERSION,
    FiniteDecoder, FiniteDecoderCatalog, FiniteDecoderEntry, FiniteDecoderError,
    FiniteDecoderOutcome, FiniteDepartureAdmissionError, FiniteDepartureEvidence,
    FiniteSupportedAnswerError, FiniteTypedIncompatibilityUseCatalog, FormulaArtifact,
    FormulaCatalog, FormulaRef, GeneratedInquiry, GeneratedInquiryCatalog,
    GeneratedInquiryCheckError, GeneratorCoverageRef, GeneratorRegimeRef, GrainRef, HorizonRef,
    IProgArtifact, IProgCatalog, IProgCheckError, IProgIR, IProgRef, NegationCoverage, NegationUse,
    NegationUseRef, ObservationResultCatalog, OpenPort, OpenQuery, OpenQueryCatalog,
    OperatorOccurrence, OperatorOccurrenceCatalog, OperatorOccurrenceCheckError, PortBinding,
    ProbeContractRef, ProbeOperator, ProbeOperatorRef, ProgramBinding, ProtectedCompletionFieldRef,
    ProvenanceRef, QueryRef, RawReturn, RawReturnCatalog, RawReturnRef, ReciprocalOccurrence,
    RelationBodyIR, RelationCatalog, RelationPort, RelationRef, RelationSchema, RelationSignature,
    RelationUse, RelationUseContext, RelationUseRef, RelationUseSupportCatalog,
    RelationUseSupportError, ResolutionCatalog, ResolutionPath, ResolutionPathIR,
    ResolutionPathRef, ReturnClosure, RoleComparison, RouteRef, ScopeRef, SeedReorientation,
    SelectedReturn, SeparatorProblem, SeparatorProblemRef, SignatureContext, Standing, StateRef,
    StructureViewRef, SupportEnvironmentArtifact, SupportEnvironmentArtifactCheckError,
    SupportEnvironmentArtifactError, SupportEnvironmentCatalog, SupportEnvironmentRef, SupportRef,
    SupportSubjectRef, TaggedExteriorCatalog, TyIR, TypeArtifact, TypeCatalog, TypeFamilyRef,
    TypeRef, TypeSymbol, TypedFiniteIncompatibilityRoles, TypedFiniteIncompatibilityTable,
    TypedFiniteNegationExtension, TypedFiniteObservation,
    TypedFiniteOrientedIncompatibilityUseResult, TypedForm, TypedFormRef,
    admit_finite_negation_extension, admit_finite_supported_answers, admit_probed_finite_departure,
    bind_finite_ask_continuation, check_departure_witness_standing_support, check_return_closure,
    check_typed_finite_oriented_incompatibility_use, decode_actual_event,
    match_decoded_observation_use, resolve_departure_witness_evidence_support,
    resolve_determination_presentation_support, resolve_relation_use_support,
    standing_determination_presentation_support, standing_from_declared_support,
    standing_relation_use_support,
};

#[derive(Clone, Default)]
struct Catalog {
    types: BTreeMap<TypeRef, TypeArtifact>,
    forms: BTreeMap<TypedFormRef, TypedForm>,
    schemas: BTreeMap<RelationRef, RelationSchema>,
    signatures: BTreeMap<RelationRef, RelationSignature>,
    relation_uses: BTreeMap<RelationUseRef, RelationUse>,
    queries: BTreeMap<QueryRef, OpenQuery>,
    candidates: BTreeMap<CompletionCandidateRef, CompletionCandidate>,
    raw_returns: BTreeMap<RawReturnRef, RawReturn>,
    decoders: BTreeMap<ic_core::FiniteDecoderRef, FiniteDecoder>,
    paths: BTreeMap<ResolutionPathRef, ResolutionPath>,
    charts: BTreeMap<BoundaryRef, BoundaryChart>,
    operators: BTreeMap<ProbeOperatorRef, ProbeOperator>,
    events: BTreeMap<EventRef, ActualEvent>,
    claims: BTreeMap<ClaimRef, ClaimArtifact>,
    support_environments: BTreeMap<SupportEnvironmentRef, SupportEnvironmentArtifact>,
    presentations: BTreeMap<DeterminationPresentationRef, DeterminationPresentation>,
    departures: BTreeMap<ic_core::DepartureWitnessRef, DepartureWitness>,
    negation_uses: BTreeMap<NegationUseRef, NegationUse>,
    programs: BTreeMap<IProgRef, IProgArtifact>,
    separator_problems: BTreeMap<SeparatorProblemRef, SeparatorProblem>,
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

    fn insert_schema(&mut self, schema: RelationSchema) -> RelationRef {
        let reference = schema.relation_ref().expect("schema must encode");
        self.signatures.insert(
            reference,
            schema.signature().expect("schema signature must encode"),
        );
        self.schemas.insert(reference, schema);
        reference
    }

    fn insert_query(&mut self, query: OpenQuery) -> QueryRef {
        let reference = query.query_ref().expect("query must encode");
        self.queries.insert(reference, query);
        reference
    }

    fn insert_relation_use(&mut self, relation_use: RelationUse) -> RelationUseRef {
        let reference = relation_use
            .relation_use_ref()
            .expect("relation use must encode");
        self.relation_uses.insert(reference, relation_use);
        reference
    }

    fn insert_candidate(&mut self, candidate: CompletionCandidate) -> CompletionCandidateRef {
        let reference = candidate
            .completion_candidate_ref()
            .expect("candidate must encode");
        self.candidates.insert(reference, candidate);
        reference
    }

    fn insert_raw_return(&mut self, raw_return: RawReturn) -> RawReturnRef {
        let reference = raw_return.raw_return_ref().expect("raw return must encode");
        self.raw_returns.insert(reference, raw_return);
        reference
    }

    fn insert_decoder(&mut self, decoder: FiniteDecoder) -> ic_core::FiniteDecoderRef {
        let reference = decoder.finite_decoder_ref().expect("decoder must encode");
        self.decoders.insert(reference, decoder);
        reference
    }

    fn insert_path(&mut self, path: ResolutionPath) -> ResolutionPathRef {
        let reference = path.resolution_path_ref().expect("path must encode");
        self.paths.insert(reference, path);
        reference
    }

    fn insert_claim(&mut self, claim: ClaimArtifact) -> ClaimRef {
        let reference = claim.claim_ref().expect("claim must encode");
        self.claims.insert(reference, claim);
        reference
    }

    fn insert_support_environment(
        &mut self,
        environment: SupportEnvironmentArtifact,
    ) -> SupportEnvironmentRef {
        let reference = environment
            .support_environment_ref()
            .expect("support environment must encode");
        self.support_environments.insert(reference, environment);
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

    fn insert_departure(&mut self, departure: DepartureWitness) -> ic_core::DepartureWitnessRef {
        let reference = departure
            .departure_witness_ref()
            .expect("departure witness must encode");
        self.departures.insert(reference, departure);
        reference
    }

    fn insert_negation_use(&mut self, negation_use: NegationUse) -> NegationUseRef {
        let reference = negation_use
            .negation_use_ref()
            .expect("negation use must encode");
        self.negation_uses.insert(reference, negation_use);
        reference
    }

    fn insert_program(&mut self, program: IProgArtifact) -> IProgRef {
        let reference = program.iprog_ref().expect("inquiry program must encode");
        self.programs.insert(reference, program);
        reference
    }

    fn insert_separator_problem(&mut self, problem: SeparatorProblem) -> SeparatorProblemRef {
        let reference = problem
            .separator_problem_ref()
            .expect("separator problem must encode");
        self.separator_problems.insert(reference, problem);
        reference
    }

    fn insert_event(&mut self, event: ActualEvent) -> EventRef {
        let reference = event.event_ref().expect("event must encode");
        self.events.insert(reference, event);
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

    fn resolve_relation_signature(&self, reference: RelationRef) -> Option<RelationSignature> {
        self.signatures.get(&reference).cloned()
    }
}

impl RelationCatalog for Catalog {
    fn resolve_relation_schema(&self, reference: RelationRef) -> Option<RelationSchema> {
        self.schemas.get(&reference).cloned()
    }
}

impl OpenQueryCatalog for Catalog {
    fn resolve_open_query(&self, reference: QueryRef) -> Option<OpenQuery> {
        self.queries.get(&reference).cloned()
    }
}

impl CompletionCandidateCatalog for Catalog {
    fn resolve_completion_candidate(
        &self,
        reference: CompletionCandidateRef,
    ) -> Option<CompletionCandidate> {
        self.candidates.get(&reference).cloned()
    }
}

impl RawReturnCatalog for Catalog {
    fn resolve_raw_return(&self, reference: RawReturnRef) -> Option<RawReturn> {
        self.raw_returns.get(&reference).cloned()
    }
}

impl FiniteDecoderCatalog for Catalog {
    fn resolve_finite_decoder(
        &self,
        reference: ic_core::FiniteDecoderRef,
    ) -> Option<FiniteDecoder> {
        self.decoders.get(&reference).cloned()
    }
}

impl ResolutionCatalog for Catalog {
    fn resolve_resolution_path(&self, reference: ResolutionPathRef) -> Option<ResolutionPath> {
        self.paths.get(&reference).cloned()
    }
}

impl ActualEventCatalog for Catalog {
    fn resolve_boundary_chart(&self, reference: BoundaryRef) -> Option<BoundaryChart> {
        self.charts.get(&reference).cloned()
    }

    fn resolve_probe_operator(&self, reference: ProbeOperatorRef) -> Option<ProbeOperator> {
        self.operators.get(&reference).cloned()
    }

    fn resolve_open_query(&self, reference: QueryRef) -> Option<OpenQuery> {
        self.queries.get(&reference).cloned()
    }
}

impl ObservationResultCatalog for Catalog {
    fn resolve_relation_use(&self, reference: RelationUseRef) -> Option<RelationUse> {
        self.relation_uses.get(&reference).cloned()
    }
}

impl SupportEnvironmentCatalog for Catalog {
    fn resolve_claim(&self, reference: ClaimRef) -> Option<ClaimArtifact> {
        self.claims.get(&reference).cloned()
    }

    fn resolve_support_environment(
        &self,
        reference: SupportEnvironmentRef,
    ) -> Option<SupportEnvironmentArtifact> {
        self.support_environments.get(&reference).cloned()
    }
}

impl RelationUseSupportCatalog for Catalog {
    fn resolve_relation_use(&self, reference: RelationUseRef) -> Option<RelationUse> {
        self.relation_uses.get(&reference).cloned()
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

impl DepartureCatalog for Catalog {
    fn resolve_relation_use(&self, reference: RelationUseRef) -> Option<RelationUse> {
        self.relation_uses.get(&reference).cloned()
    }
}

impl FiniteTypedIncompatibilityUseCatalog for Catalog {
    fn resolve_relation_use(&self, reference: RelationUseRef) -> Option<RelationUse> {
        self.relation_uses.get(&reference).cloned()
    }
}

impl TaggedExteriorCatalog for Catalog {
    fn resolve_negation_use(&self, reference: NegationUseRef) -> Option<NegationUse> {
        self.negation_uses.get(&reference).cloned()
    }

    fn resolve_departure_witness(
        &self,
        reference: ic_core::DepartureWitnessRef,
    ) -> Option<DepartureWitness> {
        self.departures.get(&reference).cloned()
    }
}

impl IProgCatalog for Catalog {
    fn resolve_iprog(&self, reference: IProgRef) -> Option<IProgArtifact> {
        self.programs.get(&reference).cloned()
    }
}

impl GeneratedInquiryCatalog for Catalog {
    fn resolve_separator_problem(
        &self,
        reference: SeparatorProblemRef,
    ) -> Option<SeparatorProblem> {
        self.separator_problems.get(&reference).copied()
    }
}

impl OperatorOccurrenceCatalog for Catalog {
    fn resolve_actual_event(&self, reference: EventRef) -> Option<ActualEvent> {
        self.events.get(&reference).cloned()
    }
}

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn port(name: &str, ty: TypeRef) -> RelationPort {
    RelationPort::new(TypeSymbol::new(name).expect("port name must be valid"), ty)
}

struct Fixture {
    catalog: Catalog,
    query: QueryRef,
    relation: RelationRef,
    answer_type: TypeRef,
    other_type: TypeRef,
    raw_type: TypeRef,
    candidate: CompletionCandidateRef,
    alternate_answer: TypedFormRef,
    observation: RelationUseRef,
    decoded_raw: RawReturnRef,
    undefined_raw: RawReturnRef,
    unknown_raw: RawReturnRef,
    event: ActualEvent,
}

fn fixture() -> Fixture {
    let mut catalog = Catalog::default();
    let binding = BindingVersionRef::from_artifact_ref(artifact(0x10));
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let other_type = catalog.insert_type(TypeArtifact::new(binding, TyIR::Bool));
    let raw_type = catalog.insert_type(TypeArtifact::new(binding, TyIR::Raw(unit)));
    let known = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x11)));
    let answer = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x12)));
    let alternate_answer = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x27)));
    let relation = catalog.insert_schema(RelationSchema::new(
        binding,
        vec![port("known", unit), port("answer", unit)],
        RelationBodyIR::BindingNative {
            contract: artifact(0x13),
        },
        Vec::new(),
        Vec::new(),
    ));
    let grain = GrainRef::from_artifact_ref(artifact(0x14));
    let horizon = HorizonRef::from_artifact_ref(artifact(0x15));
    let context = RelationUseContext::new(
        ScopeRef::from_artifact_ref(artifact(0x16)),
        ApplicabilityRef::from_artifact_ref(artifact(0x17)),
        grain,
        horizon,
        DischargeMode::Probe,
        SupportRef::from_artifact_ref(artifact(0x18)),
        None,
    );
    let open_query = OpenQuery::new(
        relation,
        vec![PortBinding::new(
            TypeSymbol::new("known").expect("port name must be valid"),
            known,
        )],
        vec![OpenPort::new(
            TypeSymbol::new("answer").expect("port name must be valid"),
            DischargeMode::Probe,
        )],
        context,
    );
    let candidate = open_query
        .plug(
            vec![PortBinding::new(
                TypeSymbol::new("answer").expect("port name must be valid"),
                answer,
            )],
            &catalog,
        )
        .expect("candidate must be constructible");
    let query = catalog.insert_query(open_query);
    let candidate = catalog.insert_candidate(candidate);
    let observation = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("known").expect("port name must be valid"),
                known,
            ),
            PortBinding::new(
                TypeSymbol::new("answer").expect("port name must be valid"),
                answer,
            ),
        ],
        context,
    ));
    let decoded_raw = catalog.insert_raw_return(RawReturn::new(vec![1]));
    let undefined_raw = catalog.insert_raw_return(RawReturn::new(vec![2]));
    let unknown_raw = catalog.insert_raw_return(RawReturn::new(vec![3]));

    let chart = BoundaryChart::new(
        query,
        unit,
        unit,
        unit,
        relation,
        relation,
        DeterminationPresentationRef::from_artifact_ref(artifact(0x19)),
        None,
        Vec::new(),
        Vec::new(),
        ic_core::RelationUseRef::from_artifact_ref(artifact(0x1a)),
        FormulaRef::from_artifact_ref(artifact(0x1b)),
        None,
        grain,
        horizon,
    );
    let boundary = chart.boundary_ref().expect("chart must encode");
    catalog.charts.insert(boundary, chart);
    let operator = ProbeOperator::new(
        query,
        boundary,
        artifact(0x1c),
        artifact(0x1d),
        artifact(0x1e),
        raw_type,
        artifact(0x1f),
        ProbeContractRef::from_artifact_ref(artifact(0x20)),
        artifact(0x21),
    );
    let operator_ref = operator.probe_operator_ref().expect("operator must encode");
    catalog.operators.insert(operator_ref, operator);
    let event = ActualEvent::new(
        None,
        StateRef::from_artifact_ref(artifact(0x22)),
        query,
        boundary,
        None,
        operator_ref,
        decoded_raw,
        StateRef::from_artifact_ref(artifact(0x23)),
        grain,
        RouteRef::from_artifact_ref(artifact(0x24)),
        binding,
        artifact(0x25),
        ProvenanceRef::from_artifact_ref(artifact(0x26)),
    );
    Fixture {
        catalog,
        query,
        relation,
        answer_type: unit,
        other_type,
        raw_type,
        candidate,
        alternate_answer,
        observation,
        decoded_raw,
        undefined_raw,
        unknown_raw,
        event,
    }
}

#[test]
fn finite_decoder_preserves_decoded_undefined_and_unknown_outcomes() {
    let mut fixture = fixture();
    let decoder = FiniteDecoder::new(
        fixture.query,
        fixture.raw_type,
        vec![
            FiniteDecoderEntry::Undefined {
                raw_return: fixture.undefined_raw,
            },
            FiniteDecoderEntry::Decoded {
                raw_return: fixture.decoded_raw,
                candidates: vec![fixture.candidate],
            },
        ],
    )
    .expect("finite rows must be canonicalizable");
    let reference = fixture.catalog.insert_decoder(decoder.clone());
    assert!(decoder.check(&fixture.catalog).is_ok());
    assert_eq!(
        decoder.finite_decoder_ref().expect("decoder must hash"),
        reference
    );
    assert_eq!(
        FiniteDecoder::from_envelope(&decoder.envelope().expect("decoder must encode"))
            .expect("decoder must decode"),
        decoder
    );
    assert_eq!(
        decoder.outcome(fixture.decoded_raw),
        FiniteDecoderOutcome::Decoded(vec![fixture.candidate])
    );
    assert_eq!(
        decoder.outcome(fixture.undefined_raw),
        FiniteDecoderOutcome::Undefined
    );
    assert_eq!(
        decoder.outcome(fixture.unknown_raw),
        FiniteDecoderOutcome::Unknown
    );

    let empty = FiniteDecoder::new(
        fixture.query,
        fixture.raw_type,
        vec![FiniteDecoderEntry::Decoded {
            raw_return: fixture.decoded_raw,
            candidates: Vec::new(),
        }],
    );
    assert!(matches!(empty, Err(FiniteDecoderError::EmptyDecodedSet)));
    let wrong_kind = ic_core::ArtifactEnvelope::from_canonical_payload(
        ic_core::ArtifactKind::new("ic.raw-return").expect("kind must be valid"),
        FINITE_DECODER_SCHEMA_VERSION,
        decoder.canonical_payload().expect("payload must encode"),
    );
    assert!(matches!(
        FiniteDecoder::from_envelope(&wrong_kind),
        Err(FiniteDecoderError::UnexpectedArtifactKind {
            expected: FINITE_DECODER_ARTIFACT_KIND,
            ..
        })
    ));
    let payload = decoder.canonical_payload().expect("payload must encode");
    assert!(matches!(
        FiniteDecoder::decode_payload(&payload[..payload.len() - 1]),
        Err(FiniteDecoderError::TruncatedPayload)
    ));
    let mut unknown_tag = payload;
    unknown_tag[68] = 0xff; // QueryRef, input TypeRef, and entry-count precede the first row.
    assert!(matches!(
        FiniteDecoder::decode_payload(&unknown_tag),
        Err(FiniteDecoderError::UnknownEntryTag(0xff))
    ));
}

#[test]
fn generated_inquiry_is_a_checked_problem_relative_candidate_not_a_policy_choice() {
    let mut fixture = fixture();
    let problem = fixture
        .catalog
        .insert_separator_problem(SeparatorProblem::new(
            ProtectedCompletionFieldRef::from_artifact_ref(artifact(0x81)),
            None,
            GrainRef::from_artifact_ref(artifact(0x14)),
            HorizonRef::from_artifact_ref(artifact(0x15)),
            fixture.event.binding(),
            StructureViewRef::from_artifact_ref(artifact(0x82)),
            GeneratorRegimeRef::from_artifact_ref(artifact(0x83)),
            EffectivityRef::from_artifact_ref(artifact(0x84)),
        ));
    let generated = GeneratedInquiry::new(problem, artifact(0x85), fixture.query);
    generated
        .check(&fixture.catalog)
        .expect("query and residual must agree on binding, grain, and horizon");
    assert_eq!(
        GeneratedInquiry::from_envelope(&generated.envelope().expect("candidate must encode"))
            .expect("candidate must decode"),
        generated
    );
    assert_eq!(
        generated.referenced_artifacts(),
        vec![
            problem.as_artifact_ref(),
            artifact(0x85),
            fixture.query.as_artifact_ref()
        ]
    );

    let other_problem = fixture
        .catalog
        .insert_separator_problem(SeparatorProblem::new(
            ProtectedCompletionFieldRef::from_artifact_ref(artifact(0x81)),
            None,
            GrainRef::from_artifact_ref(artifact(0x86)),
            HorizonRef::from_artifact_ref(artifact(0x15)),
            fixture.event.binding(),
            StructureViewRef::from_artifact_ref(artifact(0x82)),
            GeneratorRegimeRef::from_artifact_ref(artifact(0x83)),
            EffectivityRef::from_artifact_ref(artifact(0x84)),
        ));
    assert!(matches!(
        GeneratedInquiry::new(other_problem, artifact(0x85), fixture.query).check(&fixture.catalog),
        Err(GeneratedInquiryCheckError::GrainMismatch { .. })
    ));
}

#[test]
fn operator_occurrence_is_derived_from_one_exact_ordinary_event() {
    let mut fixture = fixture();
    let event = fixture.catalog.insert_event(fixture.event.clone());
    let occurrence = OperatorOccurrence::from_actual_event(event, &fixture.catalog)
        .expect("a checked actual event determines its occurrence view");
    occurrence
        .check(&fixture.catalog)
        .expect("derived occurrence must exactly match the ordinary event");
    assert_eq!(occurrence.event(), event);
    assert_eq!(
        OperatorOccurrence::from_envelope(&occurrence.envelope().expect("occurrence must encode"))
            .expect("occurrence must decode"),
        occurrence
    );
    let mismatched = OperatorOccurrence::new(
        event,
        occurrence.operator(),
        occurrence.state_before(),
        RawReturnRef::from_artifact_ref(artifact(0x91)),
        occurrence.state_after(),
        occurrence.boundary(),
    );
    assert!(matches!(
        mismatched.check(&fixture.catalog),
        Err(OperatorOccurrenceCheckError::EventFieldMismatch {
            field: "raw_return",
            ..
        })
    ));
}

#[test]
fn claim_identity_preserves_candidate_provenance_without_claiming_standing() {
    let mut fixture = fixture();
    let path = fixture.catalog.insert_path(ResolutionPath::new(
        fixture.raw_type,
        fixture.raw_type,
        ResolutionPathIR::Identity,
    ));
    let claim = ClaimArtifact::new(
        artifact(0x40),
        fixture.query,
        vec![fixture.decoded_raw],
        vec![path],
        ScopeRef::from_artifact_ref(artifact(0x41)),
        ApplicabilityRef::from_artifact_ref(artifact(0x42)),
        ClaimStatus::Standing,
    )
    .expect("claim references must canonicalize");
    assert!(claim.check(&fixture.catalog).is_ok());
    assert_eq!(
        ClaimArtifact::from_envelope(&claim.envelope().expect("claim must encode"))
            .expect("claim must decode"),
        claim
    );
    assert_eq!(claim.status(), ClaimStatus::Standing);
    assert!(matches!(
        ClaimArtifact::new(
            artifact(0x40),
            fixture.query,
            vec![fixture.decoded_raw, fixture.decoded_raw],
            vec![path],
            ScopeRef::from_artifact_ref(artifact(0x41)),
            ApplicabilityRef::from_artifact_ref(artifact(0x42)),
            ClaimStatus::Candidate,
        ),
        Err(ClaimError::DuplicateSupportingReturn(reference)) if reference == fixture.decoded_raw
    ));
    let mut malformed = claim.canonical_payload().expect("payload must encode");
    let last = malformed.len() - 1;
    malformed[last] = 0xff;
    assert!(matches!(
        ClaimArtifact::decode_payload(&malformed),
        Err(ClaimError::UnknownStatus)
    ));
}

#[test]
fn support_environment_identity_preserves_candidate_support_without_closure() {
    let mut fixture = fixture();
    let path = fixture.catalog.insert_path(ResolutionPath::new(
        fixture.raw_type,
        fixture.raw_type,
        ResolutionPathIR::Identity,
    ));
    let scope = ScopeRef::from_artifact_ref(artifact(0x51));
    let applicability = ApplicabilityRef::from_artifact_ref(artifact(0x52));
    let claim = fixture.catalog.insert_claim(
        ClaimArtifact::new(
            artifact(0x50),
            fixture.query,
            vec![fixture.decoded_raw],
            vec![path],
            scope,
            applicability,
            ClaimStatus::Candidate,
        )
        .expect("claim must canonicalize"),
    );
    let environment = SupportEnvironmentArtifact::new(
        SupportSubjectRef::Claim(claim),
        vec![artifact(0x59)],
        vec![fixture.decoded_raw],
        vec![artifact(0x53)],
        vec![artifact(0x54)],
        vec![artifact(0x55)],
        applicability,
        scope,
    )
    .expect("support environment must canonicalize");
    assert_eq!(environment.premises(), [artifact(0x59)]);
    assert!(environment.check(&fixture.catalog).is_ok());
    assert_eq!(
        SupportEnvironmentArtifact::from_envelope(
            &environment
                .envelope()
                .expect("support environment must encode"),
        )
        .expect("support environment must decode"),
        environment
    );
    let environment_ref = fixture
        .catalog
        .insert_support_environment(environment.clone());
    assert_eq!(
        environment_ref.as_support_ref().as_artifact_ref(),
        environment_ref.as_artifact_ref()
    );
    assert_eq!(
        SupportEnvironmentCatalog::resolve_support_environment(&fixture.catalog, environment_ref),
        Some(environment)
    );
    let relation_environment = SupportEnvironmentArtifact::new(
        SupportSubjectRef::Relation(fixture.relation),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        applicability,
        scope,
    )
    .expect("relation-targeted environment must remain representable");
    assert!(relation_environment.check(&fixture.catalog).is_ok());
    let relation_environment = fixture
        .catalog
        .insert_support_environment(relation_environment);
    let original_use = fixture
        .catalog
        .relation_uses
        .get(&fixture.observation)
        .expect("fixture observation must remain available")
        .clone();
    let linked_environment = SupportEnvironmentArtifact::new(
        SupportSubjectRef::Relation(original_use.relation()),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        original_use.applicability(),
        original_use.scope(),
    )
    .expect("use-targeted environment must canonicalize");
    let linked_environment = fixture
        .catalog
        .insert_support_environment(linked_environment);
    let linked_use = fixture.catalog.insert_relation_use(RelationUse::new(
        original_use.relation(),
        original_use.bindings().to_vec(),
        RelationUseContext::new(
            original_use.scope(),
            original_use.applicability(),
            original_use.grain(),
            original_use.horizon(),
            original_use.mode(),
            linked_environment.as_support_ref(),
            original_use.warrant(),
        ),
    ));
    let link = resolve_relation_use_support(linked_use, &fixture.catalog)
        .expect("matching relation-targeted support must link structurally");
    assert_eq!(link.relation_use(), linked_use);
    assert_eq!(link.environment(), linked_environment);
    let context_mismatched_use = fixture.catalog.insert_relation_use(RelationUse::new(
        original_use.relation(),
        original_use.bindings().to_vec(),
        RelationUseContext::new(
            original_use.scope(),
            original_use.applicability(),
            original_use.grain(),
            original_use.horizon(),
            original_use.mode(),
            relation_environment.as_support_ref(),
            original_use.warrant(),
        ),
    ));
    assert!(matches!(
        resolve_relation_use_support(context_mismatched_use, &fixture.catalog),
        Err(RelationUseSupportError::ContextMismatch("scope"))
    ));
    let root = fixture.catalog.insert_claim(
        ClaimArtifact::new(
            artifact(0x5a),
            fixture.query,
            vec![fixture.decoded_raw],
            vec![path],
            scope,
            applicability,
            ClaimStatus::Checked,
        )
        .expect("root claim must canonicalize"),
    );
    let standing_environment = fixture.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Claim(claim),
            vec![root.as_artifact_ref()],
            vec![fixture.decoded_raw],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        )
        .expect("standing environment must canonicalize"),
    );
    let closure = DeclaredSupportClosure::new(standing_environment, vec![root], true, true, false);
    let standing = standing_from_declared_support(vec![root], &[closure], &fixture.catalog)
        .expect("checked declared closure must enter the existing least fixed point");
    assert!(standing.contains(claim));
    assert!(matches!(
        standing_from_declared_support(
            vec![root],
            &[DeclaredSupportClosure::new(
                standing_environment,
                vec![claim],
                true,
                true,
                false,
            )],
            &fixture.catalog,
        ),
        Err(DeclaredStandingError::PremiseNotNamedByEnvironment { premise, .. })
            if premise == SupportSubjectRef::Claim(claim)
    ));
    let mismatched_context = SupportEnvironmentArtifact::new(
        SupportSubjectRef::Claim(claim),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        applicability,
        ScopeRef::from_artifact_ref(artifact(0x56)),
    )
    .expect("mismatched environment remains representable");
    assert!(matches!(
        mismatched_context.check(&fixture.catalog),
        Err(SupportEnvironmentArtifactCheckError::ClaimContextMismatch(
            "scope"
        ))
    ));
    assert!(matches!(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Claim(claim),
            vec![artifact(0x59), artifact(0x59)],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        ),
        Err(SupportEnvironmentArtifactError::DuplicatePremise(reference))
            if reference == artifact(0x59)
    ));
    assert!(matches!(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Claim(claim),
            Vec::new(),
            vec![fixture.decoded_raw, fixture.decoded_raw],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        ),
        Err(SupportEnvironmentArtifactError::DuplicateActualReturn(reference))
            if reference == fixture.decoded_raw
    ));
}

#[test]
fn determination_support_requires_one_checked_claim_targeted_standing_environment() {
    let mut fixture = fixture();
    let binding = BindingVersionRef::from_artifact_ref(artifact(0x10));
    let scope = ScopeRef::from_artifact_ref(artifact(0x16));
    let applicability = ApplicabilityRef::from_artifact_ref(artifact(0x17));
    let source =
        fixture
            .catalog
            .insert_form(TypedForm::new(binding, fixture.answer_type, artifact(0x71)));
    let claim = fixture.catalog.insert_claim(
        ClaimArtifact::new(
            source.as_artifact_ref(),
            fixture.query,
            Vec::new(),
            Vec::new(),
            scope,
            applicability,
            ClaimStatus::Checked,
        )
        .expect("claim must canonicalize"),
    );
    let environment = fixture.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Claim(claim),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        )
        .expect("claim-targeted environment must canonicalize"),
    );
    let presentation = fixture
        .catalog
        .insert_presentation(DeterminationPresentation::new(
            ic_core::DistinctionRef::from_artifact_ref(artifact(0x72)),
            ic_core::Orientation::X,
            source,
            ic_core::RelationalWebRef::from_artifact_ref(artifact(0x73)),
            binding,
            scope,
            applicability,
            GrainRef::from_artifact_ref(artifact(0x14)),
            HorizonRef::from_artifact_ref(artifact(0x15)),
            environment.as_support_ref(),
            None,
        ));

    let resolved = resolve_determination_presentation_support(presentation, &fixture.catalog)
        .expect("presentation must resolve only its exact claim-targeted environment");
    assert_eq!(resolved.presentation(), presentation);
    assert_eq!(resolved.environment(), environment);
    assert_eq!(resolved.claim(), claim);

    let ungrounded = standing_from_declared_support(Vec::new(), &[], &fixture.catalog)
        .expect("an empty standing problem is well defined");
    assert!(matches!(
        standing_determination_presentation_support(presentation, &ungrounded, &fixture.catalog),
        Err(DeterminationSupportError::ClaimIsNotStanding(reference)) if reference == claim
    ));

    let closure = DeclaredSupportClosure::new(environment, Vec::new(), true, true, false);
    let standing = standing_from_declared_support(Vec::new(), &[closure], &fixture.catalog)
        .expect("the declared closed route reaches its claim in the least fixed point");
    assert_eq!(
        standing_determination_presentation_support(presentation, &standing, &fixture.catalog)
            .expect("a standing target claim must complete the structural link"),
        resolved
    );

    let relation_environment = fixture.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Relation(fixture.relation),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        )
        .expect("relation-targeted environment must canonicalize"),
    );
    let relation_targeted_presentation =
        fixture
            .catalog
            .insert_presentation(DeterminationPresentation::new(
                ic_core::DistinctionRef::from_artifact_ref(artifact(0x74)),
                ic_core::Orientation::X,
                source,
                ic_core::RelationalWebRef::from_artifact_ref(artifact(0x75)),
                binding,
                scope,
                applicability,
                GrainRef::from_artifact_ref(artifact(0x14)),
                HorizonRef::from_artifact_ref(artifact(0x15)),
                relation_environment.as_support_ref(),
                None,
            ));
    assert!(matches!(
        resolve_determination_presentation_support(
            relation_targeted_presentation,
            &fixture.catalog
        ),
        Err(DeterminationSupportError::RelationTargetIsNotDeterminationSupport)
    ));
}

#[test]
fn departure_witness_requires_its_source_presentation_support_to_stand() {
    let mut fixture = fixture();
    let binding = BindingVersionRef::from_artifact_ref(artifact(0x10));
    let scope = ScopeRef::from_artifact_ref(artifact(0x16));
    let applicability = ApplicabilityRef::from_artifact_ref(artifact(0x17));
    let grain = GrainRef::from_artifact_ref(artifact(0x14));
    let horizon = HorizonRef::from_artifact_ref(artifact(0x15));
    let source =
        fixture
            .catalog
            .insert_form(TypedForm::new(binding, fixture.answer_type, artifact(0x91)));
    let candidate =
        fixture
            .catalog
            .insert_form(TypedForm::new(binding, fixture.answer_type, artifact(0x92)));
    let source_answer =
        fixture
            .catalog
            .insert_form(TypedForm::new(binding, fixture.answer_type, artifact(0x93)));
    let candidate_answer =
        fixture
            .catalog
            .insert_form(TypedForm::new(binding, fixture.answer_type, artifact(0x94)));
    let claim = fixture.catalog.insert_claim(
        ClaimArtifact::new(
            source.as_artifact_ref(),
            fixture.query,
            Vec::new(),
            Vec::new(),
            scope,
            applicability,
            ClaimStatus::Checked,
        )
        .expect("claim must canonicalize"),
    );
    let environment = fixture.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Claim(claim),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        )
        .expect("claim-targeted environment must canonicalize"),
    );
    let presentation = fixture
        .catalog
        .insert_presentation(DeterminationPresentation::new(
            ic_core::DistinctionRef::from_artifact_ref(artifact(0x95)),
            ic_core::Orientation::X,
            source,
            ic_core::RelationalWebRef::from_artifact_ref(artifact(0x96)),
            binding,
            scope,
            applicability,
            grain,
            horizon,
            environment.as_support_ref(),
            None,
        ));
    let evidence_environment = fixture.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Relation(fixture.relation),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        )
        .expect("relation-targeted evidence environment must canonicalize"),
    );
    let context = |mode| {
        RelationUseContext::new(
            scope,
            applicability,
            grain,
            horizon,
            mode,
            evidence_environment.as_support_ref(),
            None,
        )
    };
    let source_observation = fixture.catalog.insert_relation_use(RelationUse::new(
        fixture.relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("known").expect("port must be valid"),
                source,
            ),
            PortBinding::new(
                TypeSymbol::new("answer").expect("port must be valid"),
                source_answer,
            ),
        ],
        context(DischargeMode::Probe),
    ));
    let candidate_observation = fixture.catalog.insert_relation_use(RelationUse::new(
        fixture.relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("known").expect("port must be valid"),
                candidate,
            ),
            PortBinding::new(
                TypeSymbol::new("answer").expect("port must be valid"),
                candidate_answer,
            ),
        ],
        context(DischargeMode::Check),
    ));
    let incompatibility = fixture.catalog.insert_relation_use(RelationUse::new(
        fixture.relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("known").expect("port must be valid"),
                source_answer,
            ),
            PortBinding::new(
                TypeSymbol::new("answer").expect("port must be valid"),
                candidate_answer,
            ),
        ],
        context(DischargeMode::Warrant),
    ));
    let witness = DepartureWitness::new(
        ic_core::DistinctionRef::from_artifact_ref(artifact(0x95)),
        source,
        candidate,
        presentation,
        source_observation,
        candidate_observation,
        source_answer,
        candidate_answer,
        incompatibility,
        SupportRef::from_artifact_ref(artifact(0x97)),
        scope,
        applicability,
        grain,
    );

    let ungrounded = standing_from_declared_support(Vec::new(), &[], &fixture.catalog)
        .expect("an empty standing problem is well defined");
    assert!(matches!(
        check_departure_witness_standing_support(&witness, &ungrounded, &fixture.catalog),
        Err(DepartureStandingCheckError::DeterminationSupport(
            DeterminationSupportError::ClaimIsNotStanding(reference)
        )) if reference == claim
    ));

    let closure = DeclaredSupportClosure::new(environment, Vec::new(), true, true, false);
    let evidence_closure =
        DeclaredSupportClosure::for_subjects(evidence_environment, Vec::new(), true, true, false);
    let standing =
        standing_from_declared_support(Vec::new(), &[closure, evidence_closure], &fixture.catalog)
            .expect("the declared closed route reaches its claim in the least fixed point");
    let resolved = check_departure_witness_standing_support(&witness, &standing, &fixture.catalog)
        .expect("a checked witness must retain the source presentation's standing support link");
    assert_eq!(resolved.presentation(), presentation);
    assert_eq!(resolved.environment(), environment);
    assert_eq!(resolved.claim(), claim);

    let evidence =
        resolve_departure_witness_evidence_support(&witness, &standing, &fixture.catalog).expect(
            "each evidence use must resolve through its own relation-targeted support route",
        );
    assert_eq!(evidence.source_presentation(), resolved);
    assert_eq!(
        evidence.source_observation().relation_use(),
        source_observation
    );
    assert_eq!(
        evidence.source_observation().environment(),
        evidence_environment
    );
    assert_eq!(
        evidence.candidate_observation().relation_use(),
        candidate_observation
    );
    assert_eq!(evidence.incompatibility().relation_use(), incompatibility);
    for evidence_use in [source_observation, candidate_observation, incompatibility] {
        let admitted = standing_relation_use_support(evidence_use, &standing, &fixture.catalog)
            .expect("the exact relation-targeted evidence route must close in standing");
        assert_eq!(admitted.environment(), evidence_environment);
    }

    let unclosed_environment = fixture.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Relation(fixture.relation),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec![artifact(0x99)],
            Vec::new(),
            applicability,
            scope,
        )
        .expect("second relation-targeted environment must canonicalize"),
    );
    let unclosed_use = fixture.catalog.insert_relation_use(RelationUse::new(
        fixture.relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("known").expect("port must be valid"),
                source,
            ),
            PortBinding::new(
                TypeSymbol::new("answer").expect("port must be valid"),
                source_answer,
            ),
        ],
        RelationUseContext::new(
            scope,
            applicability,
            grain,
            horizon,
            DischargeMode::Probe,
            unclosed_environment.as_support_ref(),
            None,
        ),
    ));
    assert!(matches!(
        standing_relation_use_support(unclosed_use, &standing, &fixture.catalog),
        Err(RelationUseSupportError::EnvironmentDidNotClose(reference))
            if reference == unclosed_environment
    ));

    let claim_targeted_source_observation = fixture.catalog.insert_relation_use(RelationUse::new(
        fixture.relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("known").expect("port must be valid"),
                source,
            ),
            PortBinding::new(
                TypeSymbol::new("answer").expect("port must be valid"),
                source_answer,
            ),
        ],
        RelationUseContext::new(
            scope,
            applicability,
            grain,
            horizon,
            DischargeMode::Probe,
            environment.as_support_ref(),
            None,
        ),
    ));
    let wrongly_targeted_evidence = DepartureWitness::new(
        ic_core::DistinctionRef::from_artifact_ref(artifact(0x95)),
        source,
        candidate,
        presentation,
        claim_targeted_source_observation,
        candidate_observation,
        source_answer,
        candidate_answer,
        incompatibility,
        SupportRef::from_artifact_ref(artifact(0x97)),
        scope,
        applicability,
        grain,
    );
    assert!(matches!(
        resolve_departure_witness_evidence_support(
            &wrongly_targeted_evidence,
            &standing,
            &fixture.catalog
        ),
        Err(DepartureEvidenceSupportError::RelationUse(
            RelationUseSupportError::ClaimTargetIsNotRelationUseSupport
        ))
    ));
}

#[test]
fn finite_decode_links_an_event_record_to_its_direct_decoder_route() {
    let mut fixture = fixture();
    let decoder = FiniteDecoder::new(
        fixture.query,
        fixture.raw_type,
        vec![
            FiniteDecoderEntry::Decoded {
                raw_return: fixture.decoded_raw,
                candidates: vec![fixture.candidate],
            },
            FiniteDecoderEntry::Undefined {
                raw_return: fixture.undefined_raw,
            },
        ],
    )
    .expect("finite rows must be valid");
    let decoder_ref = fixture.catalog.insert_decoder(decoder.clone());
    let path = ResolutionPath::new(
        fixture.raw_type,
        fixture.answer_type,
        ResolutionPathIR::Decode {
            decoder: decoder_ref.as_decoder_ref(),
        },
    );
    let path_ref = fixture.catalog.insert_path(path);

    let decoded = decode_actual_event(&fixture.event, &decoder, path_ref, &fixture.catalog)
        .expect("the realized raw return must decode through its named route");
    let set = match decoded {
        ActualDecodeResult::Decoded(set) => set,
        other => panic!("expected a decoded candidate set, got {other:?}"),
    };
    assert_eq!(set.query(), fixture.query);
    assert_eq!(set.candidates(), [fixture.candidate]);
    let observation = match_decoded_observation_use(
        &set,
        fixture.candidate,
        fixture.observation,
        &fixture.catalog,
    )
    .expect("the complete decoded candidate must spell its declared observation use");
    assert_eq!(observation.decoded(), &set);
    assert_eq!(observation.candidate(), fixture.candidate);
    assert_eq!(observation.observation(), fixture.observation);

    let query = OpenQueryCatalog::resolve_open_query(&fixture.catalog, fixture.query)
        .expect("query must remain available");
    let alternate_candidate = query
        .plug(
            vec![PortBinding::new(
                TypeSymbol::new("answer").expect("port name must be valid"),
                fixture.alternate_answer,
            )],
            &fixture.catalog,
        )
        .expect("alternate complete filling must be constructible");
    let alternate_candidate = fixture.catalog.insert_candidate(alternate_candidate);
    assert!(matches!(
        match_decoded_observation_use(
            &set,
            alternate_candidate,
            fixture.observation,
            &fixture.catalog,
        ),
        Err(DecodedObservationError::CandidateNotDecoded { candidate, .. })
            if candidate == alternate_candidate
    ));
    let mismatched_observation = fixture.catalog.insert_relation_use(RelationUse::new(
        query.relation(),
        vec![
            PortBinding::new(
                TypeSymbol::new("known").expect("port name must be valid"),
                query.bound_ports()[0].value(),
            ),
            PortBinding::new(
                TypeSymbol::new("answer").expect("port name must be valid"),
                fixture.alternate_answer,
            ),
        ],
        *query.context(),
    ));
    assert!(matches!(
        match_decoded_observation_use(
            &set,
            fixture.candidate,
            mismatched_observation,
            &fixture.catalog,
        ),
        Err(DecodedObservationError::BindingMismatch)
    ));

    let undefined_event = ActualEvent::new(
        fixture.event.ledger_parent(),
        fixture.event.state_before(),
        fixture.event.question(),
        fixture.event.boundary(),
        fixture.event.distinction(),
        fixture.event.operator(),
        fixture.undefined_raw,
        fixture.event.state_after(),
        fixture.event.grain(),
        fixture.event.route(),
        fixture.event.binding(),
        fixture.event.backend_version(),
        fixture.event.provenance(),
    );
    assert!(matches!(
        decode_actual_event(&undefined_event, &decoder, path_ref, &fixture.catalog),
        Ok(ActualDecodeResult::Undefined { .. })
    ));
    let unknown_event = ActualEvent::new(
        fixture.event.ledger_parent(),
        fixture.event.state_before(),
        fixture.event.question(),
        fixture.event.boundary(),
        fixture.event.distinction(),
        fixture.event.operator(),
        fixture.unknown_raw,
        fixture.event.state_after(),
        fixture.event.grain(),
        fixture.event.route(),
        fixture.event.binding(),
        fixture.event.backend_version(),
        fixture.event.provenance(),
    );
    assert!(matches!(
        decode_actual_event(&unknown_event, &decoder, path_ref, &fixture.catalog),
        Ok(ActualDecodeResult::Unknown { .. })
    ));

    let identity = fixture.catalog.insert_path(ResolutionPath::new(
        fixture.raw_type,
        fixture.raw_type,
        ResolutionPathIR::Identity,
    ));
    assert!(matches!(
        decode_actual_event(&fixture.event, &decoder, identity, &fixture.catalog),
        Err(ActualDecodeError::PathIsNotDirectDecoder(reference)) if reference == identity
    ));
    let wrong_decoder = fixture.catalog.insert_path(ResolutionPath::new(
        fixture.raw_type,
        fixture.answer_type,
        ResolutionPathIR::Decode {
            decoder: DecoderRef::from_artifact_ref(artifact(0x32)),
        },
    ));
    assert!(matches!(
        decode_actual_event(&fixture.event, &decoder, wrong_decoder, &fixture.catalog),
        Err(ActualDecodeError::PathDecoderMismatch { .. })
    ));
    let wrong_output = fixture.catalog.insert_path(ResolutionPath::new(
        fixture.raw_type,
        fixture.other_type,
        ResolutionPathIR::Decode {
            decoder: decoder_ref.as_decoder_ref(),
        },
    ));
    assert!(matches!(
        decode_actual_event(&fixture.event, &decoder, wrong_output, &fixture.catalog),
        Err(ActualDecodeError::PathOutputMismatch { path, answer })
            if path == fixture.other_type && answer == fixture.answer_type
    ));
}

#[test]
fn decoded_observation_can_attach_post_return_support_without_rewriting_the_question() {
    let mut fixture = fixture();
    let decoder = FiniteDecoder::new(
        fixture.query,
        fixture.raw_type,
        vec![FiniteDecoderEntry::Decoded {
            raw_return: fixture.decoded_raw,
            candidates: vec![fixture.candidate],
        }],
    )
    .expect("finite row must be valid");
    let decoder_ref = fixture.catalog.insert_decoder(decoder.clone());
    let path = fixture.catalog.insert_path(ResolutionPath::new(
        fixture.raw_type,
        fixture.answer_type,
        ResolutionPathIR::Decode {
            decoder: decoder_ref.as_decoder_ref(),
        },
    ));
    let ActualDecodeResult::Decoded(decoded) =
        decode_actual_event(&fixture.event, &decoder, path, &fixture.catalog)
            .expect("actual return must decode")
    else {
        panic!("listed row must decode")
    };
    let query = fixture
        .catalog
        .queries
        .get(&fixture.query)
        .expect("query must remain available")
        .clone();
    let candidate = fixture
        .catalog
        .candidates
        .get(&fixture.candidate)
        .expect("candidate must remain available")
        .clone();
    let query_context = *query.context();
    let post_return_support = SupportRef::from_artifact_ref(artifact(0xce));
    assert_ne!(post_return_support, query_context.support());
    let observation = fixture.catalog.insert_relation_use(RelationUse::new(
        query.relation(),
        candidate.bindings().to_vec(),
        RelationUseContext::new(
            query_context.scope(),
            query_context.applicability(),
            query_context.grain(),
            query_context.horizon(),
            query_context.mode(),
            post_return_support,
            query_context.warrant(),
        ),
    ));

    match_decoded_observation_use(&decoded, fixture.candidate, observation, &fixture.catalog)
        .expect("post-return support must not rewrite the already-addressed source question");
}

#[allow(clippy::too_many_arguments)]
fn decoded_probe_observation(
    catalog: &mut Catalog,
    relation: RelationRef,
    known: TypedFormRef,
    answer: TypedFormRef,
    answer_type: TypeRef,
    raw_type: TypeRef,
    raw_return: RawReturnRef,
    presentation: DeterminationPresentationRef,
    support: SupportEnvironmentRef,
    scope: ScopeRef,
    applicability: ApplicabilityRef,
    grain: GrainRef,
    horizon: HorizonRef,
    binding: BindingVersionRef,
    tag: u8,
) -> DecodedObservationUse {
    let context = RelationUseContext::new(
        scope,
        applicability,
        grain,
        horizon,
        DischargeMode::Probe,
        support.as_support_ref(),
        None,
    );
    let known_port = TypeSymbol::new("known").expect("port must be valid");
    let answer_port = TypeSymbol::new("answer").expect("port must be valid");
    let query = OpenQuery::new(
        relation,
        vec![PortBinding::new(known_port.clone(), known)],
        vec![OpenPort::new(answer_port.clone(), DischargeMode::Probe)],
        context,
    );
    let candidate = query
        .plug(vec![PortBinding::new(answer_port.clone(), answer)], catalog)
        .expect("observation candidate must be complete");
    let query_ref = catalog.insert_query(query);
    let candidate_ref = catalog.insert_candidate(candidate);
    let observation = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![
            PortBinding::new(known_port, known),
            PortBinding::new(answer_port, answer),
        ],
        context,
    ));
    let decoder = FiniteDecoder::new(
        query_ref,
        raw_type,
        vec![FiniteDecoderEntry::Decoded {
            raw_return,
            candidates: vec![candidate_ref],
        }],
    )
    .expect("one finite decode row must be valid");
    let decoder_ref = catalog.insert_decoder(decoder.clone());
    let path = catalog.insert_path(ResolutionPath::new(
        raw_type,
        answer_type,
        ResolutionPathIR::Decode {
            decoder: decoder_ref.as_decoder_ref(),
        },
    ));
    let chart = BoundaryChart::new(
        query_ref,
        answer_type,
        answer_type,
        answer_type,
        relation,
        relation,
        presentation,
        None,
        Vec::new(),
        Vec::new(),
        observation,
        FormulaRef::from_artifact_ref(artifact(tag.wrapping_add(1))),
        None,
        grain,
        horizon,
    );
    let boundary = chart.boundary_ref().expect("chart must encode");
    catalog.charts.insert(boundary, chart);
    let operator = ProbeOperator::new(
        query_ref,
        boundary,
        artifact(tag.wrapping_add(2)),
        artifact(tag.wrapping_add(3)),
        artifact(tag.wrapping_add(4)),
        raw_type,
        artifact(tag.wrapping_add(5)),
        ProbeContractRef::from_artifact_ref(artifact(tag.wrapping_add(6))),
        artifact(tag.wrapping_add(7)),
    );
    let operator_ref = operator.probe_operator_ref().expect("operator must encode");
    catalog.operators.insert(operator_ref, operator);
    let event = ActualEvent::new(
        None,
        StateRef::from_artifact_ref(artifact(tag.wrapping_add(8))),
        query_ref,
        boundary,
        None,
        operator_ref,
        raw_return,
        StateRef::from_artifact_ref(artifact(tag.wrapping_add(9))),
        grain,
        RouteRef::from_artifact_ref(artifact(tag.wrapping_add(10))),
        binding,
        artifact(tag.wrapping_add(11)),
        ProvenanceRef::from_artifact_ref(artifact(tag.wrapping_add(12))),
    );
    catalog.insert_event(event.clone());
    let decoded = decode_actual_event(&event, &decoder, path, catalog)
        .expect("preserved event return must decode");
    let ActualDecodeResult::Decoded(decoded) = decoded else {
        panic!("declared decode row must return its candidate")
    };
    match_decoded_observation_use(&decoded, candidate_ref, observation, catalog)
        .expect("decoded completion must match its declared observation use")
}

struct FiniteDepartureScenario {
    catalog: Catalog,
    admitted: AdmittedFiniteDeparture,
    standing: Standing,
    source_observation: DecodedObservationUse,
    candidate_observation: DecodedObservationUse,
    distinction: ic_core::DistinctionRef,
    presentation: DeterminationPresentationRef,
    source: TypedFormRef,
    candidate: TypedFormRef,
    answer_type: TypeRef,
    raw_type: TypeRef,
    observation_relation: RelationRef,
    claim_query: QueryRef,
    binding: BindingVersionRef,
    scope: ScopeRef,
    applicability: ApplicabilityRef,
    grain: GrainRef,
    horizon: HorizonRef,
}

fn build_finite_departure_scenario(
    source_observation_is_relevant: bool,
    observation_supports_candidate_return: bool,
) -> Result<FiniteDepartureScenario, Box<FiniteDepartureAdmissionError>> {
    let mut fixture = fixture();
    let binding = BindingVersionRef::from_artifact_ref(artifact(0x10));
    let scope = ScopeRef::from_artifact_ref(artifact(0xa0));
    let applicability = ApplicabilityRef::from_artifact_ref(artifact(0xa1));
    let grain = GrainRef::from_artifact_ref(artifact(0xa2));
    let horizon = HorizonRef::from_artifact_ref(artifact(0xa3));
    let source =
        fixture
            .catalog
            .insert_form(TypedForm::new(binding, fixture.answer_type, artifact(0xa4)));
    let candidate =
        fixture
            .catalog
            .insert_form(TypedForm::new(binding, fixture.answer_type, artifact(0xa5)));
    let source_answer =
        fixture
            .catalog
            .insert_form(TypedForm::new(binding, fixture.answer_type, artifact(0xa6)));
    let candidate_answer =
        fixture
            .catalog
            .insert_form(TypedForm::new(binding, fixture.answer_type, artifact(0xa7)));
    let source_return = fixture
        .catalog
        .insert_raw_return(RawReturn::new(vec![0xa8]));
    let candidate_return = fixture
        .catalog
        .insert_raw_return(RawReturn::new(vec![0xa9]));

    let incompatibility_relation = fixture.catalog.insert_schema(RelationSchema::new(
        binding,
        vec![
            port("source", fixture.answer_type),
            port("candidate", fixture.answer_type),
        ],
        RelationBodyIR::BindingNative {
            contract: artifact(0xaa),
        },
        Vec::new(),
        Vec::new(),
    ));
    let source_observation_environment = fixture.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Relation(fixture.relation),
            Vec::new(),
            vec![source_return],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        )
        .expect("source-observation support must canonicalize"),
    );
    let candidate_returns = observation_supports_candidate_return
        .then_some(candidate_return)
        .into_iter()
        .collect();
    let candidate_observation_environment = fixture.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Relation(fixture.relation),
            Vec::new(),
            candidate_returns,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        )
        .expect("candidate-observation support must canonicalize"),
    );
    let incompatibility_environment = fixture.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Relation(incompatibility_relation),
            vec![fixture.relation.as_artifact_ref()],
            vec![source_return, candidate_return],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        )
        .expect("incompatibility support must canonicalize"),
    );
    let claim = fixture.catalog.insert_claim(
        ClaimArtifact::new(
            source.as_artifact_ref(),
            fixture.query,
            Vec::new(),
            Vec::new(),
            scope,
            applicability,
            ClaimStatus::Checked,
        )
        .expect("source claim must canonicalize"),
    );
    let claim_premises = source_observation_is_relevant
        .then_some(fixture.relation.as_artifact_ref())
        .into_iter()
        .collect();
    let presentation_environment = fixture.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Claim(claim),
            claim_premises,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        )
        .expect("presentation support must canonicalize"),
    );
    let distinction = ic_core::DistinctionRef::from_artifact_ref(artifact(0xab));
    let presentation = fixture
        .catalog
        .insert_presentation(DeterminationPresentation::new(
            distinction,
            ic_core::Orientation::X,
            source,
            ic_core::RelationalWebRef::from_artifact_ref(artifact(0xac)),
            binding,
            scope,
            applicability,
            grain,
            horizon,
            presentation_environment.as_support_ref(),
            None,
        ));

    let source_observation = decoded_probe_observation(
        &mut fixture.catalog,
        fixture.relation,
        source,
        source_answer,
        fixture.answer_type,
        fixture.raw_type,
        source_return,
        presentation,
        source_observation_environment,
        scope,
        applicability,
        grain,
        horizon,
        binding,
        0xb0,
    );
    let candidate_observation = decoded_probe_observation(
        &mut fixture.catalog,
        fixture.relation,
        candidate,
        candidate_answer,
        fixture.answer_type,
        fixture.raw_type,
        candidate_return,
        presentation,
        candidate_observation_environment,
        scope,
        applicability,
        grain,
        horizon,
        binding,
        0xc0,
    );
    let incompatibility = fixture.catalog.insert_relation_use(RelationUse::new(
        incompatibility_relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("source").expect("port must be valid"),
                source_answer,
            ),
            PortBinding::new(
                TypeSymbol::new("candidate").expect("port must be valid"),
                candidate_answer,
            ),
        ],
        RelationUseContext::new(
            scope,
            applicability,
            grain,
            horizon,
            DischargeMode::Check,
            incompatibility_environment.as_support_ref(),
            None,
        ),
    ));
    let witness = DepartureWitness::new(
        distinction,
        source,
        candidate,
        presentation,
        source_observation.observation(),
        candidate_observation.observation(),
        source_answer,
        candidate_answer,
        incompatibility,
        SupportRef::from_artifact_ref(artifact(0xad)),
        scope,
        applicability,
        grain,
    );

    let source_observation_closure = DeclaredSupportClosure::for_subjects(
        source_observation_environment,
        Vec::new(),
        true,
        true,
        false,
    );
    let candidate_observation_closure = DeclaredSupportClosure::for_subjects(
        candidate_observation_environment,
        Vec::new(),
        true,
        true,
        false,
    );
    let incompatibility_closure = DeclaredSupportClosure::for_subjects(
        incompatibility_environment,
        vec![SupportSubjectRef::Relation(fixture.relation)],
        true,
        true,
        false,
    );
    let presentation_premises = source_observation_is_relevant
        .then_some(SupportSubjectRef::Relation(fixture.relation))
        .into_iter()
        .collect();
    let presentation_closure = DeclaredSupportClosure::for_subjects(
        presentation_environment,
        presentation_premises,
        true,
        true,
        false,
    );
    let standing = standing_from_declared_support(
        Vec::new(),
        &[
            source_observation_closure,
            candidate_observation_closure,
            incompatibility_closure,
            presentation_closure,
        ],
        &fixture.catalog,
    )
    .expect("finite evidence routes must close from their typed premises");

    let table = TypedFiniteIncompatibilityTable::new(vec![(source_answer, candidate_answer)])
        .expect("one positive pair must be valid");
    let oriented = check_typed_finite_oriented_incompatibility_use(
        &table,
        &fixture.catalog,
        incompatibility,
        TypedFiniteIncompatibilityRoles::new(
            TypeSymbol::new("source").expect("port must be valid"),
            TypeSymbol::new("candidate").expect("port must be valid"),
        )
        .expect("roles must be distinct"),
        TypedFiniteObservation::Observed(source_answer),
        TypedFiniteObservation::Observed(candidate_answer),
    )
    .expect("typed finite incompatibility must be checkable");
    let TypedFiniteOrientedIncompatibilityUseResult::Incompatible(oriented) = oriented else {
        panic!("listed observed pair must produce positive incompatibility")
    };
    let evidence = FiniteDepartureEvidence::new(
        source_observation.clone(),
        candidate_observation.clone(),
        oriented,
    );
    let admitted = admit_probed_finite_departure(&witness, &standing, &evidence, &fixture.catalog)
        .map_err(Box::new)?;
    let witness_ref = fixture.catalog.insert_departure(witness);
    assert_eq!(witness_ref, admitted.witness());
    Ok(FiniteDepartureScenario {
        catalog: fixture.catalog,
        admitted,
        standing,
        source_observation,
        candidate_observation,
        distinction,
        presentation,
        source,
        candidate,
        answer_type: fixture.answer_type,
        raw_type: fixture.raw_type,
        observation_relation: fixture.relation,
        claim_query: fixture.query,
        binding,
        scope,
        applicability,
        grain,
        horizon,
    })
}

fn finite_departure_scenario(
    source_observation_is_relevant: bool,
    observation_supports_candidate_return: bool,
) -> Result<AdmittedFiniteDeparture, Box<FiniteDepartureAdmissionError>> {
    build_finite_departure_scenario(
        source_observation_is_relevant,
        observation_supports_candidate_return,
    )
    .map(|scenario| scenario.admitted)
}

#[allow(clippy::too_many_arguments)]
fn add_finite_departure(
    scenario: &mut FiniteDepartureScenario,
    orientation: ic_core::Orientation,
    source: TypedFormRef,
    candidate: TypedFormRef,
    source_answer: TypedFormRef,
    candidate_answer: TypedFormRef,
    tag: u8,
) -> (AdmittedFiniteDeparture, DeterminationPresentationRef) {
    let source_return = scenario
        .catalog
        .insert_raw_return(RawReturn::new(vec![tag, 1]));
    let candidate_return = scenario
        .catalog
        .insert_raw_return(RawReturn::new(vec![tag, 2]));
    let incompatibility_relation = scenario.catalog.insert_schema(RelationSchema::new(
        scenario.binding,
        vec![
            port("source", scenario.answer_type),
            port("candidate", scenario.answer_type),
        ],
        RelationBodyIR::BindingNative {
            contract: artifact(tag.wrapping_add(3)),
        },
        Vec::new(),
        Vec::new(),
    ));
    let source_environment = scenario.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Relation(scenario.observation_relation),
            Vec::new(),
            vec![source_return],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            scenario.applicability,
            scenario.scope,
        )
        .expect("source route must canonicalize"),
    );
    let candidate_environment = scenario.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Relation(scenario.observation_relation),
            Vec::new(),
            vec![candidate_return],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            scenario.applicability,
            scenario.scope,
        )
        .expect("candidate route must canonicalize"),
    );
    let incompatibility_environment = scenario.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Relation(incompatibility_relation),
            vec![scenario.observation_relation.as_artifact_ref()],
            vec![source_return, candidate_return],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            scenario.applicability,
            scenario.scope,
        )
        .expect("incompatibility route must canonicalize"),
    );
    let claim = scenario.catalog.insert_claim(
        ClaimArtifact::new(
            source.as_artifact_ref(),
            scenario.claim_query,
            Vec::new(),
            Vec::new(),
            scenario.scope,
            scenario.applicability,
            ClaimStatus::Checked,
        )
        .expect("source claim must canonicalize"),
    );
    let presentation_environment = scenario.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Claim(claim),
            vec![scenario.observation_relation.as_artifact_ref()],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            scenario.applicability,
            scenario.scope,
        )
        .expect("presentation route must canonicalize"),
    );
    let presentation = scenario
        .catalog
        .insert_presentation(DeterminationPresentation::new(
            scenario.distinction,
            orientation,
            source,
            ic_core::RelationalWebRef::from_artifact_ref(artifact(tag.wrapping_add(4))),
            scenario.binding,
            scenario.scope,
            scenario.applicability,
            scenario.grain,
            scenario.horizon,
            presentation_environment.as_support_ref(),
            None,
        ));
    let source_observation = decoded_probe_observation(
        &mut scenario.catalog,
        scenario.observation_relation,
        source,
        source_answer,
        scenario.answer_type,
        scenario.raw_type,
        source_return,
        presentation,
        source_environment,
        scenario.scope,
        scenario.applicability,
        scenario.grain,
        scenario.horizon,
        scenario.binding,
        tag.wrapping_add(10),
    );
    let candidate_observation = decoded_probe_observation(
        &mut scenario.catalog,
        scenario.observation_relation,
        candidate,
        candidate_answer,
        scenario.answer_type,
        scenario.raw_type,
        candidate_return,
        presentation,
        candidate_environment,
        scenario.scope,
        scenario.applicability,
        scenario.grain,
        scenario.horizon,
        scenario.binding,
        tag.wrapping_add(30),
    );
    let incompatibility = scenario.catalog.insert_relation_use(RelationUse::new(
        incompatibility_relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("source").expect("port must be valid"),
                source_answer,
            ),
            PortBinding::new(
                TypeSymbol::new("candidate").expect("port must be valid"),
                candidate_answer,
            ),
        ],
        RelationUseContext::new(
            scenario.scope,
            scenario.applicability,
            scenario.grain,
            scenario.horizon,
            DischargeMode::Check,
            incompatibility_environment.as_support_ref(),
            None,
        ),
    ));
    let witness = DepartureWitness::new(
        scenario.distinction,
        source,
        candidate,
        presentation,
        source_observation.observation(),
        candidate_observation.observation(),
        source_answer,
        candidate_answer,
        incompatibility,
        SupportRef::from_artifact_ref(artifact(tag.wrapping_add(5))),
        scenario.scope,
        scenario.applicability,
        scenario.grain,
    );
    let standing = standing_from_declared_support(
        Vec::new(),
        &[
            DeclaredSupportClosure::for_subjects(source_environment, Vec::new(), true, true, false),
            DeclaredSupportClosure::for_subjects(
                candidate_environment,
                Vec::new(),
                true,
                true,
                false,
            ),
            DeclaredSupportClosure::for_subjects(
                incompatibility_environment,
                vec![SupportSubjectRef::Relation(scenario.observation_relation)],
                true,
                true,
                false,
            ),
            DeclaredSupportClosure::for_subjects(
                presentation_environment,
                vec![SupportSubjectRef::Relation(scenario.observation_relation)],
                true,
                true,
                false,
            ),
        ],
        &scenario.catalog,
    )
    .expect("reciprocal evidence routes must close");
    let table = TypedFiniteIncompatibilityTable::new(vec![(source_answer, candidate_answer)])
        .expect("one pair must be valid");
    let oriented = check_typed_finite_oriented_incompatibility_use(
        &table,
        &scenario.catalog,
        incompatibility,
        TypedFiniteIncompatibilityRoles::new(
            TypeSymbol::new("source").expect("port must be valid"),
            TypeSymbol::new("candidate").expect("port must be valid"),
        )
        .expect("roles must be distinct"),
        TypedFiniteObservation::Observed(source_answer),
        TypedFiniteObservation::Observed(candidate_answer),
    )
    .expect("incompatibility must check");
    let TypedFiniteOrientedIncompatibilityUseResult::Incompatible(oriented) = oriented else {
        panic!("listed observed pair must be incompatible")
    };
    let evidence =
        FiniteDepartureEvidence::new(source_observation, candidate_observation, oriented);
    let admitted = admit_probed_finite_departure(&witness, &standing, &evidence, &scenario.catalog)
        .expect("reciprocal departure must admit independently");
    let reference = scenario.catalog.insert_departure(witness);
    assert_eq!(reference, admitted.witness());
    (admitted, presentation)
}

#[allow(clippy::too_many_arguments)]
fn admit_singleton_negation_use(
    scenario: &mut FiniteDepartureScenario,
    departure: AdmittedFiniteDeparture,
    presentation: DeterminationPresentationRef,
    orientation: ic_core::Orientation,
    source: TypedFormRef,
    candidate: TypedFormRef,
    semantic_coverage: NegationCoverage,
    execution_coverage: GeneratorCoverageRef,
    tag: u8,
) -> AdmittedFiniteNegationExtension {
    let relation = scenario.catalog.insert_schema(RelationSchema::new(
        scenario.binding,
        vec![
            port("source", scenario.answer_type),
            port("candidate", scenario.answer_type),
        ],
        RelationBodyIR::BindingNative {
            contract: artifact(tag),
        },
        Vec::new(),
        Vec::new(),
    ));
    let relation_use = scenario.catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![PortBinding::new(
            TypeSymbol::new("source").expect("port must be valid"),
            source,
        )],
        RelationUseContext::new(
            scenario.scope,
            scenario.applicability,
            scenario.grain,
            scenario.horizon,
            DischargeMode::Check,
            SupportRef::from_artifact_ref(artifact(tag.wrapping_add(1))),
            None,
        ),
    ));
    let soundness = scenario.catalog.insert_program(IProgArtifact::new(
        scenario.answer_type,
        IProgIR::Return { value: source },
    ));
    let negation_use = scenario.catalog.insert_negation_use(NegationUse::new(
        relation_use,
        scenario.distinction,
        orientation,
        presentation,
        relation,
        soundness,
        semantic_coverage,
        scenario.applicability,
        scenario.scope,
        scenario.grain,
        scenario.horizon,
        vec![artifact(tag.wrapping_add(2))],
    ));
    admit_finite_negation_extension(
        TypedFiniteNegationExtension::declare(negation_use, vec![(source, candidate)])
            .expect("singleton incidence must be unique"),
        vec![(departure, execution_coverage)],
        &scenario.catalog,
    )
    .expect("singleton use must have its admitted departure")
}

#[test]
fn finite_departure_requires_positive_probed_supported_relevant_non_circular_evidence() {
    let admitted = finite_departure_scenario(true, true)
        .expect("fully supported finite observations must admit one derived departure");
    assert_ne!(admitted.source_event(), admitted.candidate_event());
    assert_ne!(
        admitted.source_raw_return(),
        admitted.candidate_raw_return()
    );

    assert!(matches!(
        finite_departure_scenario(true, false),
        Err(error) if matches!(*error, FiniteDepartureAdmissionError::SupportMissingReturn {
            role: "candidate",
            ..
        })
    ));
    assert!(matches!(
        finite_departure_scenario(false, true),
        Err(error) if matches!(*error, FiniteDepartureAdmissionError::SourceObservationNotRelevant { .. })
    ));
}

#[test]
fn finite_supported_answers_require_exact_decoded_probe_and_standing_route_coverage() {
    let mut scenario = build_finite_departure_scenario(true, true)
        .expect("the probe-backed observation fixture must admit");
    let decoded = scenario.source_observation.decoded().clone();
    let admitted = admit_finite_supported_answers(
        decoded.clone(),
        vec![scenario.source_observation.clone()],
        &scenario.standing,
        &scenario.catalog,
    )
    .expect("the exact decoded candidate has one standing Probe support route");
    assert_eq!(admitted.decoded(), &decoded);
    assert_eq!(admitted.candidates(), decoded.candidates());
    assert_eq!(admitted.event(), decoded.event());
    assert_eq!(admitted.raw_return(), scenario.admitted.source_raw_return());
    assert_eq!(admitted.observations().len(), 1);
    assert_eq!(admitted.observations()[0], scenario.source_observation);
    assert_eq!(admitted.support().len(), 1);

    let query = scenario
        .catalog
        .queries
        .get(&decoded.query())
        .expect("decoded query must remain available")
        .clone();
    let post_return_environment = scenario.catalog.insert_support_environment(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Relation(query.relation()),
            Vec::new(),
            vec![scenario.admitted.source_raw_return()],
            vec![artifact(0xcf)],
            Vec::new(),
            Vec::new(),
            query.context().applicability(),
            query.context().scope(),
        )
        .expect("post-return support must canonicalize"),
    );
    assert_ne!(
        post_return_environment.as_support_ref(),
        query.context().support()
    );
    let original_candidate = scenario
        .catalog
        .candidates
        .get(&scenario.source_observation.candidate())
        .expect("source candidate must remain available")
        .clone();
    let post_return_observation = scenario.catalog.insert_relation_use(RelationUse::new(
        query.relation(),
        original_candidate.bindings().to_vec(),
        RelationUseContext::new(
            query.context().scope(),
            query.context().applicability(),
            query.context().grain(),
            query.context().horizon(),
            query.context().mode(),
            post_return_environment.as_support_ref(),
            query.context().warrant(),
        ),
    ));
    let post_return_match = match_decoded_observation_use(
        &decoded,
        scenario.source_observation.candidate(),
        post_return_observation,
        &scenario.catalog,
    )
    .expect("decoded observation may use independently formed post-return support");
    let post_return_standing = standing_from_declared_support(
        Vec::new(),
        &[DeclaredSupportClosure::for_subjects(
            post_return_environment,
            Vec::new(),
            true,
            true,
            false,
        )],
        &scenario.catalog,
    )
    .expect("post-return support may independently close");
    let post_return_admitted = admit_finite_supported_answers(
        decoded.clone(),
        vec![post_return_match],
        &post_return_standing,
        &scenario.catalog,
    )
    .expect("post-return support that closes and names the actual return must admit");
    assert_eq!(
        post_return_admitted.support()[0].environment(),
        post_return_environment
    );

    let alternate_answer = scenario.catalog.insert_form(TypedForm::new(
        scenario.binding,
        scenario.answer_type,
        artifact(0x39),
    ));
    let alternate_candidate = query
        .plug(
            vec![PortBinding::new(
                query.open_ports()[0].port().clone(),
                alternate_answer,
            )],
            &scenario.catalog,
        )
        .expect("alternate completion must fill the same question");
    let alternate_candidate_ref = scenario
        .catalog
        .insert_candidate(alternate_candidate.clone());
    let alternate_use = scenario.catalog.insert_relation_use(RelationUse::new(
        query.relation(),
        alternate_candidate.bindings().to_vec(),
        *query.context(),
    ));
    let decoder = FiniteDecoder::new(
        decoded.query(),
        scenario.raw_type,
        vec![FiniteDecoderEntry::Decoded {
            raw_return: scenario.admitted.source_raw_return(),
            candidates: vec![
                scenario.source_observation.candidate(),
                alternate_candidate_ref,
            ],
        }],
    )
    .expect("two decoded candidates must remain one supported partial answer");
    let decoder_ref = scenario.catalog.insert_decoder(decoder.clone());
    let path = scenario.catalog.insert_path(ResolutionPath::new(
        scenario.raw_type,
        scenario.answer_type,
        ResolutionPathIR::Decode {
            decoder: decoder_ref.as_decoder_ref(),
        },
    ));
    let event = scenario
        .catalog
        .events
        .get(&decoded.event())
        .expect("decoded event must remain available")
        .clone();
    let ActualDecodeResult::Decoded(two_candidates) =
        decode_actual_event(&event, &decoder, path, &scenario.catalog)
            .expect("the same actual return must decode to both candidates")
    else {
        panic!("listed candidates must decode")
    };
    let first_observation = match_decoded_observation_use(
        &two_candidates,
        scenario.source_observation.candidate(),
        scenario.source_observation.observation(),
        &scenario.catalog,
    )
    .expect("the original observation remains one completion");
    let alternate_observation = match_decoded_observation_use(
        &two_candidates,
        alternate_candidate_ref,
        alternate_use,
        &scenario.catalog,
    )
    .expect("the alternate completion has its own matching observation");
    let partial = admit_finite_supported_answers(
        two_candidates.clone(),
        vec![first_observation.clone(), alternate_observation],
        &scenario.standing,
        &scenario.catalog,
    )
    .expect("all decoded supported alternatives must remain in the answer set");
    assert_eq!(partial.candidates().len(), 2);
    assert!(partial.candidates().contains(&alternate_candidate_ref));

    let continuation = scenario.catalog.insert_program(IProgArtifact::new(
        scenario.answer_type,
        IProgIR::Return {
            value: alternate_answer,
        },
    ));
    let context_slot = TypeSymbol::new("context").expect("binding name must be valid");
    let answer_slot = TypeSymbol::new("answer").expect("answer slot must be valid");
    let ask = IProgArtifact::new(
        scenario.answer_type,
        IProgIR::Ask {
            question: partial.decoded().query(),
            environment: vec![ProgramBinding::new(context_slot.clone(), alternate_answer)],
            answer_slot: answer_slot.clone(),
            continuation,
        },
    );
    let bound = bind_finite_ask_continuation(&ask, partial.clone(), &scenario.catalog)
        .expect("the whole partial answer must bind as inspectable continuation data");
    assert_eq!(bound.question(), partial.decoded().query());
    assert_eq!(bound.answer_slot(), &answer_slot);
    assert_eq!(bound.answer().candidates(), partial.candidates());
    assert_eq!(bound.answer().candidates().len(), 2);
    assert_eq!(bound.environment()[0].name(), &context_slot);
    assert_eq!(bound.continuation(), continuation);

    let capturing = IProgArtifact::new(
        scenario.answer_type,
        IProgIR::Ask {
            question: partial.decoded().query(),
            environment: vec![ProgramBinding::new(answer_slot.clone(), alternate_answer)],
            answer_slot,
            continuation,
        },
    );
    assert!(matches!(
        bind_finite_ask_continuation(&capturing, partial.clone(), &scenario.catalog),
        Err(ic_core::FiniteAnswerBindingError::IProgCheck(error))
            if matches!(*error, IProgCheckError::AnswerSlotShadowsEnvironment(ref name) if name == "answer")
    ));

    let other_question = scenario.candidate_observation.decoded().query();
    assert_ne!(other_question, partial.decoded().query());
    let wrong_question = IProgArtifact::new(
        scenario.answer_type,
        IProgIR::Ask {
            question: other_question,
            environment: Vec::new(),
            answer_slot: TypeSymbol::new("answer").expect("answer slot must be valid"),
            continuation,
        },
    );
    assert!(matches!(
        bind_finite_ask_continuation(&wrong_question, partial.clone(), &scenario.catalog),
        Err(ic_core::FiniteAnswerBindingError::QuestionMismatch {
            program_question,
            answer_question,
        }) if program_question == other_question && answer_question == partial.decoded().query()
    ));

    let not_an_ask = IProgArtifact::new(
        scenario.answer_type,
        IProgIR::Return {
            value: alternate_answer,
        },
    );
    assert!(matches!(
        bind_finite_ask_continuation(&not_an_ask, partial, &scenario.catalog),
        Err(ic_core::FiniteAnswerBindingError::SourceIsNotAsk(_))
    ));

    assert!(matches!(
        admit_finite_supported_answers(
            two_candidates,
            vec![first_observation],
            &scenario.standing,
            &scenario.catalog,
        ),
        Err(FiniteSupportedAnswerError::CandidateCoverageMismatch)
    ));

    assert!(matches!(
        admit_finite_supported_answers(
            decoded.clone(),
            Vec::new(),
            &scenario.standing,
            &scenario.catalog,
        ),
        Err(FiniteSupportedAnswerError::CandidateCoverageMismatch)
    ));
    assert!(matches!(
        admit_finite_supported_answers(
            decoded.clone(),
            vec![
                scenario.source_observation.clone(),
                scenario.source_observation.clone(),
            ],
            &scenario.standing,
            &scenario.catalog,
        ),
        Err(FiniteSupportedAnswerError::DuplicateCandidate(candidate))
            if candidate == scenario.source_observation.candidate()
    ));
    assert!(matches!(
        admit_finite_supported_answers(
            decoded.clone(),
            vec![scenario.candidate_observation.clone()],
            &scenario.standing,
            &scenario.catalog,
        ),
        Err(FiniteSupportedAnswerError::ForeignDecodedResult(candidate))
            if candidate == scenario.candidate_observation.candidate()
    ));

    let ungrounded = standing_from_declared_support(Vec::new(), &[], &scenario.catalog)
        .expect("empty standing is defined");
    assert!(matches!(
        admit_finite_supported_answers(
            decoded,
            vec![scenario.source_observation],
            &ungrounded,
            &scenario.catalog,
        ),
        Err(FiniteSupportedAnswerError::RelationSupport(_))
    ));
}

#[test]
fn finite_negation_admission_requires_one_departure_per_use_tagged_incidence() {
    let mut scenario = build_finite_departure_scenario(true, true)
        .expect("positive finite departure must be available");
    let relation = scenario.catalog.insert_schema(RelationSchema::new(
        scenario.binding,
        vec![
            port("source", scenario.answer_type),
            port("candidate", scenario.answer_type),
        ],
        RelationBodyIR::BindingNative {
            contract: artifact(0xd0),
        },
        Vec::new(),
        Vec::new(),
    ));
    let relation_use = scenario.catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![PortBinding::new(
            TypeSymbol::new("source").expect("port must be valid"),
            scenario.source,
        )],
        RelationUseContext::new(
            scenario.scope,
            scenario.applicability,
            scenario.grain,
            scenario.horizon,
            DischargeMode::Check,
            SupportRef::from_artifact_ref(artifact(0xd1)),
            None,
        ),
    ));
    let soundness = scenario.catalog.insert_program(IProgArtifact::new(
        scenario.answer_type,
        IProgIR::Return {
            value: scenario.source,
        },
    ));
    let use_ref = scenario.catalog.insert_negation_use(NegationUse::new(
        relation_use,
        scenario.distinction,
        ic_core::Orientation::X,
        scenario.presentation,
        relation,
        soundness,
        NegationCoverage::CertifiedPartial,
        scenario.applicability,
        scenario.scope,
        scenario.grain,
        scenario.horizon,
        Vec::new(),
    ));
    let extension =
        TypedFiniteNegationExtension::declare(use_ref, vec![(scenario.source, scenario.candidate)])
            .expect("one incidence must be unique");
    let execution_coverage = GeneratorCoverageRef::from_artifact_ref(artifact(0xd2));
    let admitted = admit_finite_negation_extension(
        extension,
        vec![(scenario.admitted.clone(), execution_coverage)],
        &scenario.catalog,
    )
    .expect("every declared incidence has an admitted departure");
    assert_eq!(admitted.negation_use(), use_ref);
    assert_eq!(
        admitted.semantic_coverage(),
        NegationCoverage::CertifiedPartial
    );
    assert_eq!(admitted.exteriors().len(), 1);
    assert_eq!(admitted.departures(), [scenario.admitted.clone()]);
    assert_eq!(
        admitted.exteriors()[0].execution_coverage(),
        execution_coverage
    );
    let fiber = admitted
        .return_fiber(scenario.candidate)
        .expect("the admitted incidence has a same-use reverse section");
    assert_eq!(fiber.use_ref(), use_ref);
    assert!(fiber.contains(scenario.source.as_artifact_ref()));

    let other_source = scenario.catalog.insert_form(TypedForm::new(
        scenario.binding,
        scenario.answer_type,
        artifact(0xd3),
    ));
    let other_candidate = scenario.catalog.insert_form(TypedForm::new(
        scenario.binding,
        scenario.answer_type,
        artifact(0xd4),
    ));
    let unsupported_row = TypedFiniteNegationExtension::declare(
        use_ref,
        vec![
            (scenario.source, scenario.candidate),
            (other_source, other_candidate),
        ],
    )
    .expect("rows must be unique");
    assert!(matches!(
        admit_finite_negation_extension(
            unsupported_row,
            vec![(scenario.admitted.clone(), execution_coverage)],
            &scenario.catalog,
        ),
        Err(ic_core::FiniteNegationAdmissionError::MissingDeparture(pair))
            if pair == (other_source, other_candidate)
    ));

    let second_use = scenario.catalog.insert_negation_use(NegationUse::new(
        relation_use,
        scenario.distinction,
        ic_core::Orientation::X,
        scenario.presentation,
        relation,
        soundness,
        NegationCoverage::WorkingOpen,
        scenario.applicability,
        scenario.scope,
        scenario.grain,
        scenario.horizon,
        vec![artifact(0xd5)],
    ));
    let second = admit_finite_negation_extension(
        TypedFiniteNegationExtension::declare(
            second_use,
            vec![(scenario.source, scenario.candidate)],
        )
        .expect("one incidence must be unique"),
        vec![(scenario.admitted.clone(), execution_coverage)],
        &scenario.catalog,
    )
    .expect("the same departure may support a separately tagged compatible use");
    let second_fiber = second
        .return_fiber(scenario.candidate)
        .expect("the second use has its own reverse section");
    assert_ne!(fiber.use_ref(), second_fiber.use_ref());
    assert_eq!(fiber.exterior(), second_fiber.exterior());

    let duplicate =
        TypedFiniteNegationExtension::declare(use_ref, vec![(scenario.source, scenario.candidate)])
            .expect("one incidence must be unique");
    assert!(matches!(
        admit_finite_negation_extension(
            duplicate,
            vec![
                (scenario.admitted.clone(), execution_coverage),
                (scenario.admitted, execution_coverage),
            ],
            &scenario.catalog,
        ),
        Err(ic_core::FiniteNegationAdmissionError::DuplicateDeparture(pair))
            if pair == (scenario.source, scenario.candidate)
    ));
}

#[test]
fn independently_admitted_sides_form_one_reciprocal_occurrence_vertical_slice() {
    let mut scenario =
        build_finite_departure_scenario(true, true).expect("X departure must admit independently");
    let x_departure = scenario.admitted.clone();
    let x_witness = x_departure.witness();
    let x_presentation = scenario.presentation;
    let source_x = scenario.source;
    let exterior_x = scenario.candidate;
    let x_use = admit_singleton_negation_use(
        &mut scenario,
        x_departure,
        x_presentation,
        ic_core::Orientation::X,
        source_x,
        exterior_x,
        NegationCoverage::CertifiedPartial,
        GeneratorCoverageRef::from_artifact_ref(artifact(0x41)),
        0x42,
    );

    let source_y = exterior_x;
    let exterior_y = scenario.catalog.insert_form(TypedForm::new(
        scenario.binding,
        scenario.answer_type,
        artifact(0x45),
    ));
    let y_source_answer = scenario.catalog.insert_form(TypedForm::new(
        scenario.binding,
        scenario.answer_type,
        artifact(0x46),
    ));
    let y_candidate_answer = scenario.catalog.insert_form(TypedForm::new(
        scenario.binding,
        scenario.answer_type,
        artifact(0x47),
    ));
    let (y_departure, y_presentation) = add_finite_departure(
        &mut scenario,
        ic_core::Orientation::Y,
        source_y,
        exterior_y,
        y_source_answer,
        y_candidate_answer,
        0x50,
    );
    assert_ne!(x_witness, y_departure.witness());
    let y_use = admit_singleton_negation_use(
        &mut scenario,
        y_departure,
        y_presentation,
        ic_core::Orientation::Y,
        source_y,
        exterior_y,
        NegationCoverage::CertifiedPartial,
        GeneratorCoverageRef::from_artifact_ref(artifact(0x48)),
        0x49,
    );

    let x_exterior = x_use.exteriors()[0];
    let y_exterior = y_use.exteriors()[0];
    let seed_relation = scenario.catalog.insert_schema(RelationSchema::new(
        scenario.binding,
        vec![
            port("exterior", scenario.answer_type),
            port("reciprocal_source", scenario.answer_type),
        ],
        RelationBodyIR::BindingNative {
            contract: artifact(0x60),
        },
        Vec::new(),
        Vec::new(),
    ));
    let seed_use = scenario.catalog.insert_relation_use(RelationUse::new(
        seed_relation,
        vec![
            PortBinding::new(
                TypeSymbol::new("exterior").expect("port must be valid"),
                exterior_x,
            ),
            PortBinding::new(
                TypeSymbol::new("reciprocal_source").expect("port must be valid"),
                source_y,
            ),
        ],
        RelationUseContext::new(
            scenario.scope,
            scenario.applicability,
            scenario.grain,
            scenario.horizon,
            DischargeMode::Check,
            SupportRef::from_artifact_ref(artifact(0x61)),
            None,
        ),
    ));
    let seed = SeedReorientation::new(x_exterior, seed_use, source_y);
    seed.check(&scenario.catalog)
        .expect("the supported seed preserves the X use and reorients to Y");

    let x_fiber = x_use
        .return_fiber(exterior_x)
        .expect("X return is the reverse section of the admitted X use");
    let y_fiber = y_use
        .return_fiber(exterior_y)
        .expect("Y return is the reverse section of the admitted Y use");
    let signatures = SignatureContext::new(
        scenario.binding,
        scenario.scope,
        scenario.applicability,
        scenario.grain,
        scenario.horizon,
        scenario.answer_type,
    );
    let x_signature = ExactFiniteSignature::new(
        signatures,
        vec![(source_x.as_artifact_ref(), artifact(0x62))],
    )
    .expect("X signature domain must be unique");
    let y_signature = ExactFiniteSignature::new(
        signatures,
        vec![(source_y.as_artifact_ref(), artifact(0x63))],
    )
    .expect("Y signature domain must be unique");
    let x_selection = SelectedReturn::select(x_fiber.clone(), source_x.as_artifact_ref())
        .expect("X selection must belong to the entire fiber");
    let y_selection = SelectedReturn::select(y_fiber.clone(), source_y.as_artifact_ref())
        .expect("Y selection must belong to the entire fiber");
    assert!(matches!(
        check_return_closure(&x_selection, &x_signature).expect("X domain is the whole fiber"),
        ReturnClosure::Closed { .. }
    ));
    assert!(matches!(
        check_return_closure(&y_selection, &y_signature).expect("Y domain is the whole fiber"),
        ReturnClosure::Closed { .. }
    ));

    let occurrence = ReciprocalOccurrence::new(
        seed,
        x_fiber.clone(),
        Some(source_x.as_artifact_ref()),
        y_exterior,
        y_fiber.clone(),
        Some(source_y.as_artifact_ref()),
    )
    .expect("selected returns must belong to their fibers");
    occurrence
        .check(&scenario.catalog)
        .expect("the Y side is independent, reversed, seeded, and same-distinction");
    assert_eq!(
        occurrence.residuals(),
        [
            RoleComparison::Coincident,
            RoleComparison::Coincident,
            RoleComparison::Undecided,
            RoleComparison::Coincident,
        ]
    );
    occurrence
        .gamma_reachable()
        .expect("Gamma is reachable only after every role is filled");
    let before_y_return = ReciprocalOccurrence::new(
        seed,
        x_fiber,
        Some(source_x.as_artifact_ref()),
        y_exterior,
        y_fiber,
        None,
    )
    .expect("an unselected return remains an explicit partial occurrence");
    assert!(matches!(
        before_y_return.gamma_reachable(),
        Err(ic_core::GammaError::RoleMissing("R_Y"))
    ));
    assert_eq!(before_y_return.selected_return_y(), None);

    let (same_orientation_departure, same_orientation_presentation) = add_finite_departure(
        &mut scenario,
        ic_core::Orientation::X,
        source_y,
        exterior_y,
        y_source_answer,
        y_candidate_answer,
        0x70,
    );
    let same_orientation_use = admit_singleton_negation_use(
        &mut scenario,
        same_orientation_departure,
        same_orientation_presentation,
        ic_core::Orientation::X,
        source_y,
        exterior_y,
        NegationCoverage::CertifiedPartial,
        GeneratorCoverageRef::from_artifact_ref(artifact(0x71)),
        0x72,
    );
    let same_orientation_exterior = same_orientation_use.exteriors()[0];
    let same_orientation_fiber = same_orientation_use
        .return_fiber(exterior_y)
        .expect("the pointwise admitted row still has its own fiber");
    let not_reciprocal = ReciprocalOccurrence::new(
        seed,
        x_use
            .return_fiber(exterior_x)
            .expect("X fiber remains derivable"),
        Some(source_x.as_artifact_ref()),
        same_orientation_exterior,
        same_orientation_fiber,
        Some(source_y.as_artifact_ref()),
    )
    .expect("all selected returns are members");
    assert!(matches!(
        not_reciprocal.check(&scenario.catalog),
        Err(ic_core::ReciprocalOccurrenceError::OrientationDidNotReverse(ic_core::Orientation::X))
    ));
}

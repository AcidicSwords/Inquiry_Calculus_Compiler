use std::collections::BTreeMap;

use ic_core::{
    ActualDecodeResult, ActualEvent, ActualEventCatalog, ApplicabilityRef, ArtifactRef,
    BindingVersionRef, BoundaryChart, BoundaryRef, CompletionCandidate, CompletionCandidateCatalog,
    CompletionCandidateRef, DeclaredSupportClosure, DischargeMode, EventRef, FiniteDecoder,
    FiniteDecoderCatalog, FiniteDecoderEntry, FiniteDecoderRef, FormulaArtifact, FormulaCatalog,
    FormulaRef, GrainRef, HorizonRef, IProgArtifact, IProgCatalog, IProgIR, IProgRef,
    ObservationResultCatalog, OpenPort, OpenQuery, OpenQueryCatalog, OperatorOccurrenceCatalog,
    PortBinding, ProbeContractRef, ProbeOperator, ProbeOperatorRef, ProvenanceRef, QueryRef,
    RawReturn, RawReturnCatalog, RawReturnRef, RelationBodyIR, RelationCatalog, RelationPort,
    RelationRef, RelationSchema, RelationSignature, RelationUse, RelationUseContext,
    RelationUseRef, RelationUseSupportCatalog, ResolutionCatalog, ResolutionPath, ResolutionPathIR,
    ResolutionPathRef, RouteRef, ScopeRef, StateRef, SupportEnvironmentArtifact,
    SupportEnvironmentCatalog, SupportEnvironmentRef, SupportSubjectRef, TyIR, TypeArtifact,
    TypeCatalog, TypeFamilyRef, TypeRef, TypeSymbol, TypedForm, TypedFormRef,
    admit_finite_supported_answers, bind_finite_ask_continuation, decode_actual_event,
    match_decoded_observation_use, standing_from_declared_support,
};
use ic_runtime::{
    AdmittedResumeError, BasicBlock, BlockTarget, ContinuationLowering, MachineStep, ProgramIR,
    RuntimeCatalog, Terminator,
};

#[derive(Default)]
struct Catalog {
    types: BTreeMap<TypeRef, TypeArtifact>,
    forms: BTreeMap<TypedFormRef, TypedForm>,
    schemas: BTreeMap<RelationRef, RelationSchema>,
    signatures: BTreeMap<RelationRef, RelationSignature>,
    queries: BTreeMap<QueryRef, OpenQuery>,
    candidates: BTreeMap<CompletionCandidateRef, CompletionCandidate>,
    relation_uses: BTreeMap<RelationUseRef, RelationUse>,
    raw_returns: BTreeMap<RawReturnRef, RawReturn>,
    decoders: BTreeMap<FiniteDecoderRef, FiniteDecoder>,
    paths: BTreeMap<ResolutionPathRef, ResolutionPath>,
    charts: BTreeMap<BoundaryRef, BoundaryChart>,
    operators: BTreeMap<ProbeOperatorRef, ProbeOperator>,
    events: BTreeMap<EventRef, ActualEvent>,
    support: BTreeMap<SupportEnvironmentRef, SupportEnvironmentArtifact>,
    programs: BTreeMap<IProgRef, IProgArtifact>,
}

impl Catalog {
    fn insert_type(&mut self, value: TypeArtifact) -> TypeRef {
        let reference = value.type_ref().expect("type must encode");
        self.types.insert(reference, value);
        reference
    }

    fn insert_form(&mut self, value: TypedForm) -> TypedFormRef {
        let reference = value.typed_form_ref().expect("form must encode");
        self.forms.insert(reference, value);
        reference
    }

    fn insert_schema(&mut self, value: RelationSchema) -> RelationRef {
        let reference = value.relation_ref().expect("relation must encode");
        self.signatures
            .insert(reference, value.signature().expect("signature must encode"));
        self.schemas.insert(reference, value);
        reference
    }

    fn insert_query(&mut self, value: OpenQuery) -> QueryRef {
        let reference = value.query_ref().expect("query must encode");
        self.queries.insert(reference, value);
        reference
    }

    fn insert_candidate(&mut self, value: CompletionCandidate) -> CompletionCandidateRef {
        let reference = value
            .completion_candidate_ref()
            .expect("candidate must encode");
        self.candidates.insert(reference, value);
        reference
    }

    fn insert_relation_use(&mut self, value: RelationUse) -> RelationUseRef {
        let reference = value.relation_use_ref().expect("use must encode");
        self.relation_uses.insert(reference, value);
        reference
    }

    fn insert_raw_return(&mut self, value: RawReturn) -> RawReturnRef {
        let reference = value.raw_return_ref().expect("raw return must encode");
        self.raw_returns.insert(reference, value);
        reference
    }

    fn insert_decoder(&mut self, value: FiniteDecoder) -> FiniteDecoderRef {
        let reference = value.finite_decoder_ref().expect("decoder must encode");
        self.decoders.insert(reference, value);
        reference
    }

    fn insert_path(&mut self, value: ResolutionPath) -> ResolutionPathRef {
        let reference = value.resolution_path_ref().expect("path must encode");
        self.paths.insert(reference, value);
        reference
    }

    fn insert_support(&mut self, value: SupportEnvironmentArtifact) -> SupportEnvironmentRef {
        let reference = value
            .support_environment_ref()
            .expect("support environment must encode");
        self.support.insert(reference, value);
        reference
    }

    fn insert_program(&mut self, value: IProgArtifact) -> IProgRef {
        let reference = value.iprog_ref().expect("program must encode");
        self.programs.insert(reference, value);
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
    fn resolve_finite_decoder(&self, reference: FiniteDecoderRef) -> Option<FiniteDecoder> {
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

impl OperatorOccurrenceCatalog for Catalog {
    fn resolve_actual_event(&self, reference: EventRef) -> Option<ActualEvent> {
        self.events.get(&reference).cloned()
    }
}

impl ObservationResultCatalog for Catalog {
    fn resolve_relation_use(&self, reference: RelationUseRef) -> Option<RelationUse> {
        self.relation_uses.get(&reference).cloned()
    }
}

impl SupportEnvironmentCatalog for Catalog {
    fn resolve_claim(&self, _reference: ic_core::ClaimRef) -> Option<ic_core::ClaimArtifact> {
        None
    }

    fn resolve_support_environment(
        &self,
        reference: SupportEnvironmentRef,
    ) -> Option<SupportEnvironmentArtifact> {
        self.support.get(&reference).cloned()
    }
}

impl RelationUseSupportCatalog for Catalog {
    fn resolve_relation_use(&self, reference: RelationUseRef) -> Option<RelationUse> {
        self.relation_uses.get(&reference).cloned()
    }
}

impl IProgCatalog for Catalog {
    fn resolve_iprog(&self, reference: IProgRef) -> Option<IProgArtifact> {
        self.programs.get(&reference).cloned()
    }
}

impl RuntimeCatalog for Catalog {
    fn resolve_typed_form(&self, reference: TypedFormRef) -> Option<TypedForm> {
        self.forms.get(&reference).copied()
    }
}

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

#[test]
fn admitted_answer_resumption_preserves_cold_replay_provenance_and_exact_lowering() {
    let mut catalog = Catalog::default();
    let binding = BindingVersionRef::from_artifact_ref(artifact(0x10));
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let raw_type = catalog.insert_type(TypeArtifact::new(binding, TyIR::Raw(unit)));
    let answer = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x11)));
    let relation = catalog.insert_schema(RelationSchema::new(
        binding,
        vec![RelationPort::new(
            TypeSymbol::new("answer").expect("port name must be valid"),
            unit,
        )],
        RelationBodyIR::BindingNative {
            contract: artifact(0x12),
        },
        Vec::new(),
        Vec::new(),
    ));
    let raw = catalog.insert_raw_return(RawReturn::new(vec![0x13]));
    let scope = ScopeRef::from_artifact_ref(artifact(0x14));
    let applicability = ApplicabilityRef::from_artifact_ref(artifact(0x15));
    let support = catalog.insert_support(
        SupportEnvironmentArtifact::new(
            SupportSubjectRef::Relation(relation),
            Vec::new(),
            vec![raw],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            applicability,
            scope,
        )
        .expect("support environment must encode"),
    );
    let grain = GrainRef::from_artifact_ref(artifact(0x16));
    let horizon = HorizonRef::from_artifact_ref(artifact(0x17));
    let context = RelationUseContext::new(
        scope,
        applicability,
        grain,
        horizon,
        DischargeMode::Probe,
        support.as_support_ref(),
        None,
    );
    let answer_port = TypeSymbol::new("answer").expect("port name must be valid");
    let query_value = OpenQuery::new(
        relation,
        Vec::new(),
        vec![OpenPort::new(answer_port.clone(), DischargeMode::Probe)],
        context,
    );
    let candidate = query_value
        .plug(
            vec![PortBinding::new(answer_port.clone(), answer)],
            &catalog,
        )
        .expect("candidate must fill the query");
    let query = catalog.insert_query(query_value);
    let candidate = catalog.insert_candidate(candidate);
    let observation = catalog.insert_relation_use(RelationUse::new(
        relation,
        vec![PortBinding::new(answer_port, answer)],
        context,
    ));
    let decoder = FiniteDecoder::new(
        query,
        raw_type,
        vec![FiniteDecoderEntry::Decoded {
            raw_return: raw,
            candidates: vec![candidate],
        }],
    )
    .expect("finite decoder must encode");
    let decoder_ref = catalog.insert_decoder(decoder.clone());
    let path = catalog.insert_path(ResolutionPath::new(
        raw_type,
        unit,
        ResolutionPathIR::Decode {
            decoder: decoder_ref.as_decoder_ref(),
        },
    ));
    let chart = BoundaryChart::new(
        query,
        unit,
        unit,
        unit,
        relation,
        relation,
        ic_core::DeterminationPresentationRef::from_artifact_ref(artifact(0x18)),
        None,
        Vec::new(),
        Vec::new(),
        observation,
        FormulaRef::from_artifact_ref(artifact(0x19)),
        None,
        grain,
        horizon,
    );
    let boundary = chart.boundary_ref().expect("chart must encode");
    catalog.charts.insert(boundary, chart);
    let operator_value = ProbeOperator::new(
        query,
        boundary,
        artifact(0x1a),
        artifact(0x1b),
        artifact(0x1c),
        raw_type,
        artifact(0x1d),
        ProbeContractRef::from_artifact_ref(artifact(0x1e)),
        artifact(0x1f),
    );
    let operator = operator_value
        .probe_operator_ref()
        .expect("operator must encode");
    catalog.operators.insert(operator, operator_value);
    let event_value = ActualEvent::new(
        None,
        StateRef::from_artifact_ref(artifact(0x20)),
        query,
        boundary,
        None,
        operator,
        raw,
        StateRef::from_artifact_ref(artifact(0x21)),
        grain,
        RouteRef::from_artifact_ref(artifact(0x22)),
        binding,
        artifact(0x23),
        ProvenanceRef::from_artifact_ref(artifact(0x24)),
    );
    let event = event_value.event_ref().expect("event must encode");
    catalog.events.insert(event, event_value.clone());
    let ActualDecodeResult::Decoded(decoded) =
        decode_actual_event(&event_value, &decoder, path, &catalog)
            .expect("preserved event must replay through its decoder")
    else {
        panic!("listed return must decode")
    };
    let observation = match_decoded_observation_use(&decoded, candidate, observation, &catalog)
        .expect("decoded candidate must match its Probe observation");
    let standing = standing_from_declared_support(
        Vec::new(),
        &[DeclaredSupportClosure::for_subjects(
            support,
            Vec::new(),
            true,
            true,
            false,
        )],
        &catalog,
    )
    .expect("the exact relation route must close");
    let admitted = admit_finite_supported_answers(decoded, vec![observation], &standing, &catalog)
        .expect("the event-linked answer must admit");
    let continuation =
        catalog.insert_program(IProgArtifact::new(unit, IProgIR::Return { value: answer }));
    let ask = IProgArtifact::new(
        unit,
        IProgIR::Ask {
            question: query,
            environment: Vec::new(),
            answer_slot: TypeSymbol::new("answer_set").expect("slot must be valid"),
            continuation,
        },
    );
    let bound = bind_finite_ask_continuation(&ask, admitted, &catalog)
        .expect("the admitted set must bind to the checked source continuation");

    let runtime = ProgramIR::new(
        unit,
        BlockTarget::new(0),
        vec![
            BasicBlock::new(
                BlockTarget::new(0),
                Terminator::Probe {
                    operator,
                    resume: BlockTarget::new(1),
                },
            ),
            BasicBlock::new(BlockTarget::new(1), Terminator::Return { value: answer }),
            BasicBlock::new(BlockTarget::new(2), Terminator::Return { value: answer }),
        ],
    );
    runtime
        .verify(&catalog)
        .expect("runtime program must verify");
    let MachineStep::Suspended(suspension) = runtime
        .step(runtime.start())
        .expect("entry probe must suspend")
    else {
        panic!("entry must suspend")
    };
    let resumption = suspension
        .resume_admitted(
            bound.clone(),
            ContinuationLowering::new(continuation, BlockTarget::new(1)),
            &runtime,
        )
        .expect("the exact source/operator/target lowering must resume");
    assert_eq!(resumption.state().target(), BlockTarget::new(1));
    assert_eq!(resumption.event(), event);
    assert_eq!(resumption.raw_return(), raw);
    assert_eq!(resumption.binding().continuation(), continuation);
    assert_eq!(resumption.binding().answer().candidates(), [candidate]);

    assert!(matches!(
        suspension.resume_admitted(
            bound.clone(),
            ContinuationLowering::new(
                IProgRef::from_artifact_ref(artifact(0x30)),
                BlockTarget::new(1),
            ),
            &runtime,
        ),
        Err(AdmittedResumeError::ContinuationMismatch { .. })
    ));
    assert!(matches!(
        suspension.resume_admitted(
            bound,
            ContinuationLowering::new(continuation, BlockTarget::new(2)),
            &runtime,
        ),
        Err(AdmittedResumeError::ResumeTargetMismatch { .. })
    ));
}

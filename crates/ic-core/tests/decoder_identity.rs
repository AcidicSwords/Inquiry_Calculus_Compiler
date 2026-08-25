use std::collections::BTreeMap;

use ic_core::{
    ActualDecodeError, ActualDecodeResult, ActualEvent, ActualEventCatalog, ApplicabilityRef,
    ArtifactRef, BindingVersionRef, BoundaryChart, BoundaryRef, CompletionCandidate,
    CompletionCandidateCatalog, CompletionCandidateRef, DecoderRef, DeterminationPresentationRef,
    DischargeMode, FINITE_DECODER_ARTIFACT_KIND, FINITE_DECODER_SCHEMA_VERSION, FiniteDecoder,
    FiniteDecoderCatalog, FiniteDecoderEntry, FiniteDecoderError, FiniteDecoderOutcome,
    FormulaArtifact, FormulaCatalog, FormulaRef, GrainRef, HorizonRef, OpenPort, OpenQuery,
    OpenQueryCatalog, PortBinding, ProbeContractRef, ProbeOperator, ProbeOperatorRef,
    ProvenanceRef, QueryRef, RawReturn, RawReturnCatalog, RawReturnRef, RelationBodyIR,
    RelationCatalog, RelationPort, RelationRef, RelationSchema, RelationSignature,
    RelationUseContext, ResolutionCatalog, ResolutionPath, ResolutionPathIR, ResolutionPathRef,
    RouteRef, ScopeRef, StateRef, SupportRef, TyIR, TypeArtifact, TypeCatalog, TypeFamilyRef,
    TypeRef, TypeSymbol, TypedForm, TypedFormRef, decode_actual_event,
};

#[derive(Clone, Default)]
struct Catalog {
    types: BTreeMap<TypeRef, TypeArtifact>,
    forms: BTreeMap<TypedFormRef, TypedForm>,
    schemas: BTreeMap<RelationRef, RelationSchema>,
    signatures: BTreeMap<RelationRef, RelationSignature>,
    queries: BTreeMap<QueryRef, OpenQuery>,
    candidates: BTreeMap<CompletionCandidateRef, CompletionCandidate>,
    raw_returns: BTreeMap<RawReturnRef, RawReturn>,
    decoders: BTreeMap<ic_core::FiniteDecoderRef, FiniteDecoder>,
    paths: BTreeMap<ResolutionPathRef, ResolutionPath>,
    charts: BTreeMap<BoundaryRef, BoundaryChart>,
    operators: BTreeMap<ProbeOperatorRef, ProbeOperator>,
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

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn port(name: &str, ty: TypeRef) -> RelationPort {
    RelationPort::new(TypeSymbol::new(name).expect("port name must be valid"), ty)
}

struct Fixture {
    catalog: Catalog,
    query: QueryRef,
    answer_type: TypeRef,
    other_type: TypeRef,
    raw_type: TypeRef,
    candidate: CompletionCandidateRef,
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
        RelationUseContext::new(
            ScopeRef::from_artifact_ref(artifact(0x16)),
            ApplicabilityRef::from_artifact_ref(artifact(0x17)),
            grain,
            horizon,
            DischargeMode::Probe,
            SupportRef::from_artifact_ref(artifact(0x18)),
            None,
        ),
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
        answer_type: unit,
        other_type,
        raw_type,
        candidate,
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
    assert!(matches!(
        decoded,
        ActualDecodeResult::Decoded(ref set)
            if set.query() == fixture.query && set.candidates() == [fixture.candidate]
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

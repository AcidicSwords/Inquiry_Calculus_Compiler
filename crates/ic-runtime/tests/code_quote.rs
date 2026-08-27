//! QCODE-TYPING-001: typed quotations remain nonexecuting and coordinate-exact.

use std::collections::{BTreeMap, BTreeSet};

use ic_core::{
    ArtifactEnvelope, ArtifactRef, BindingVersionRef, CodeCatalog, CodeInterpretation,
    CodeInterpretationKind, FormulaArtifact, FormulaCatalog, FormulaRef, IProgArtifact,
    IProgCatalog, IProgIR, IProgRef, RelationCatalog, RelationRef, RelationSchema,
    RelationSignature, RuntimeProgramRef, TyIR, TypeArtifact, TypeCatalog, TypeFamilyRef, TypeRef,
    TypedForm, TypedFormRef, interpret_code, quote_iprog, quote_program,
};
use ic_runtime::{
    BasicBlock, BlockTarget, ProgramIR, RuntimeCatalog, RuntimeProgramArtifact, Terminator,
};

#[derive(Default)]
struct Catalog {
    types: BTreeMap<TypeRef, TypeArtifact>,
    forms: BTreeMap<TypedFormRef, TypedForm>,
    sources: BTreeMap<IProgRef, IProgArtifact>,
    runtimes: BTreeMap<RuntimeProgramRef, RuntimeProgramArtifact>,
    admissions: BTreeSet<(BindingVersionRef, ArtifactRef, CodeInterpretationKind)>,
}

impl Catalog {
    fn insert_type(&mut self, value: TypeArtifact) -> TypeRef {
        let reference = value.type_ref().expect("type encodes");
        self.types.insert(reference, value);
        reference
    }
    fn insert_form(&mut self, value: TypedForm) -> TypedFormRef {
        let reference = value.typed_form_ref().expect("form encodes");
        self.forms.insert(reference, value);
        reference
    }
    fn insert_source(&mut self, value: IProgArtifact) -> IProgRef {
        let reference = value.iprog_ref().expect("source encodes");
        self.sources.insert(reference, value);
        reference
    }
    fn insert_runtime(&mut self, value: RuntimeProgramArtifact) -> RuntimeProgramRef {
        let reference = value.runtime_program_ref().expect("runtime encodes");
        self.runtimes.insert(reference, value);
        reference
    }
}

impl TypeCatalog for Catalog {
    fn resolve_type(&self, reference: TypeRef) -> Option<TypeArtifact> {
        self.types.get(&reference).cloned()
    }
    fn resolve_family_domain(&self, _: TypeFamilyRef) -> Option<(BindingVersionRef, TypeRef)> {
        None
    }
}
impl FormulaCatalog for Catalog {
    fn resolve_formula(&self, _: FormulaRef) -> Option<FormulaArtifact> {
        None
    }
    fn resolve_typed_form(&self, reference: TypedFormRef) -> Option<TypedForm> {
        self.forms.get(&reference).copied()
    }
    fn resolve_relation_signature(&self, _: RelationRef) -> Option<RelationSignature> {
        None
    }
}
impl RelationCatalog for Catalog {
    fn resolve_relation_schema(&self, _: RelationRef) -> Option<RelationSchema> {
        None
    }
}
impl ic_core::OpenQueryCatalog for Catalog {
    fn resolve_open_query(&self, _: ic_core::QueryRef) -> Option<ic_core::OpenQuery> {
        None
    }
}
impl IProgCatalog for Catalog {
    fn resolve_iprog(&self, reference: IProgRef) -> Option<IProgArtifact> {
        self.sources.get(&reference).cloned()
    }
}
impl RuntimeCatalog for Catalog {
    fn resolve_typed_form(&self, reference: TypedFormRef) -> Option<TypedForm> {
        self.forms.get(&reference).copied()
    }
}
impl CodeCatalog for Catalog {
    fn resolve_runtime_program(
        &self,
        reference: RuntimeProgramRef,
    ) -> Option<(TypeRef, BindingVersionRef, ArtifactRef)> {
        self.runtimes.get(&reference).map(|runtime| {
            (
                runtime.result(),
                runtime.binding(),
                runtime.compiler_version(),
            )
        })
    }
    fn admits_code_interpretation(
        &self,
        binding: BindingVersionRef,
        compiler: ArtifactRef,
        kind: CodeInterpretationKind,
    ) -> bool {
        self.admissions.contains(&(binding, compiler, kind))
    }
}

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}
fn binding_ref(byte: u8) -> BindingVersionRef {
    BindingVersionRef::from_artifact_ref(artifact(byte))
}

#[test]
fn typed_quotations_cold_decode_and_interpret_only_the_exact_admitted_coordinate() {
    let binding = binding_ref(0x11);
    let compiler = artifact(0x12);
    let wrong_compiler = artifact(0x13);
    let wrong_binding = binding_ref(0x14);
    let mut catalog = Catalog::default();
    let result = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let value = catalog.insert_form(TypedForm::new(binding, result, artifact(0x15)));
    let source = catalog.insert_source(IProgArtifact::new(result, IProgIR::Return { value }));
    let runtime = RuntimeProgramArtifact::new(
        binding,
        compiler,
        ProgramIR::new(
            result,
            BlockTarget::new(0),
            vec![BasicBlock::new(
                BlockTarget::new(0),
                Terminator::Return { value },
            )],
        ),
    );
    runtime
        .check(&catalog)
        .expect("runtime quotation referent is checked without stepping");
    let runtime_ref = catalog.insert_runtime(runtime.clone());

    let source_code = quote_iprog(result, binding, compiler, source);
    let source_code_other_compiler = quote_iprog(result, binding, wrong_compiler, source);
    let runtime_code = quote_program(result, binding, compiler, runtime_ref);
    let source_cold = ic_core::CodeArtifact::from_envelope(
        &ArtifactEnvelope::decode(
            &source_code
                .envelope()
                .expect("source quote encodes")
                .encode()
                .expect("source envelope encodes"),
        )
        .expect("source envelope decodes"),
    )
    .expect("source quote cold decodes");
    let runtime_cold = RuntimeProgramArtifact::from_envelope(
        &ArtifactEnvelope::decode(
            &runtime
                .envelope()
                .expect("runtime encodes")
                .encode()
                .expect("runtime envelope encodes"),
        )
        .expect("runtime envelope decodes"),
    )
    .expect("runtime referent cold decodes");
    let runtime_quote_cold = ic_core::CodeArtifact::from_envelope(
        &ArtifactEnvelope::decode(
            &runtime_code
                .envelope()
                .expect("runtime quote encodes")
                .encode()
                .expect("runtime quote envelope encodes"),
        )
        .expect("runtime quote envelope decodes"),
    )
    .expect("runtime quote cold decodes");
    assert_eq!(source_cold, source_code);
    assert_eq!(runtime_cold, runtime);
    assert_eq!(runtime_quote_cold, runtime_code);

    catalog
        .admissions
        .insert((binding, compiler, CodeInterpretationKind::Source));
    catalog
        .admissions
        .insert((binding, compiler, CodeInterpretationKind::Runtime));
    assert_eq!(
        interpret_code(source_cold, binding, compiler, &catalog).expect("source check succeeds"),
        Some(CodeInterpretation::Source(source))
    );
    assert_eq!(
        interpret_code(runtime_quote_cold, binding, compiler, &catalog)
            .expect("runtime check succeeds"),
        Some(CodeInterpretation::Runtime(runtime_ref))
    );
    assert_eq!(
        interpret_code(source_code, binding, wrong_compiler, &catalog)
            .expect("wrong compiler is ordinary undefined"),
        None
    );
    assert_eq!(
        interpret_code(source_code, wrong_binding, compiler, &catalog)
            .expect("wrong binding is ordinary undefined"),
        None
    );

    // Quotations contain only identities and never call ProgramIR::step or record actuality.
    assert_eq!(catalog.runtimes.len(), 1);
    assert_ne!(
        source_code.code_ref().expect("source identity"),
        runtime_code.code_ref().expect("runtime identity")
    );
    assert_ne!(
        source_code.code_ref().expect("first compiler identity"),
        source_code_other_compiler
            .code_ref()
            .expect("second compiler identity")
    );
}

#[test]
fn quotation_check_rejects_a_result_type_substitution() {
    let binding = binding_ref(0x31);
    let compiler = artifact(0x32);
    let mut catalog = Catalog::default();
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let bool_ty = catalog.insert_type(TypeArtifact::new(binding, TyIR::Bool));
    let value = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x33)));
    let source = catalog.insert_source(IProgArtifact::new(unit, IProgIR::Return { value }));
    let substituted = quote_iprog(bool_ty, binding, compiler, source);
    assert!(matches!(
        substituted.check(&catalog),
        Err(ic_core::CodeCheckError::SourceResultMismatch { .. })
    ));
    assert!(matches!(
        quote_iprog(unit, binding_ref(0x34), compiler, source).check(&catalog),
        Err(ic_core::CodeCheckError::SourceBindingMismatch { .. })
    ));
}

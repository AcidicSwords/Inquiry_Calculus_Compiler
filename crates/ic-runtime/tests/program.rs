use std::collections::BTreeMap;

use ic_core::{
    ArtifactRef, BindingVersionRef, IProgRef, RawReturnRef, TyIR, TypeArtifact, TypeCatalog,
    TypeFamilyRef, TypeRef, TypedForm, TypedFormRef,
};
use ic_runtime::{
    AdmittedResumeError, BasicBlock, BlockTarget, ContinuationLowering, MachineStep,
    ProbeOperatorRef, ProgramCheckError, ProgramIR, RuntimeCatalog, Terminator,
};

#[derive(Default)]
struct Catalog {
    types: BTreeMap<TypeRef, TypeArtifact>,
    forms: BTreeMap<TypedFormRef, TypedForm>,
}

impl Catalog {
    fn insert_type(&mut self, ty: TypeArtifact) -> TypeRef {
        let reference = ty.type_ref().expect("type must encode");
        self.types.insert(reference, ty);
        reference
    }

    fn insert_form(&mut self, form: TypedForm) -> TypedFormRef {
        let reference = form.typed_form_ref().expect("form must encode");
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

impl RuntimeCatalog for Catalog {
    fn resolve_typed_form(&self, reference: TypedFormRef) -> Option<TypedForm> {
        self.forms.get(&reference).copied()
    }
}

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn binding(byte: u8) -> BindingVersionRef {
    BindingVersionRef::from_artifact_ref(artifact(byte))
}

#[test]
fn verified_runtime_program_branches_suspends_and_preserves_raw_return_identity() {
    let mut catalog = Catalog::default();
    let binding = binding(0x11);
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let value = catalog.insert_form(TypedForm::new(binding, unit, artifact(0x12)));
    let program = ProgramIR::new(
        unit,
        BlockTarget::new(0),
        vec![
            BasicBlock::new(
                BlockTarget::new(0),
                Terminator::Branch {
                    targets: vec![BlockTarget::new(1), BlockTarget::new(2)],
                },
            ),
            BasicBlock::new(BlockTarget::new(1), Terminator::Return { value }),
            BasicBlock::new(
                BlockTarget::new(2),
                Terminator::Probe {
                    operator: ProbeOperatorRef::from_artifact_ref(artifact(0x13)),
                    resume: BlockTarget::new(3),
                },
            ),
            BasicBlock::new(BlockTarget::new(3), Terminator::Return { value }),
        ],
    );
    assert!(program.verify(&catalog).is_ok());
    let MachineStep::Branched(branches) = program.step(program.start()).expect("entry exists")
    else {
        panic!("entry must branch")
    };
    assert_eq!(branches.len(), 2);
    assert!(matches!(
        program.step(branches[0]).expect("return block exists"),
        MachineStep::Returned(returned) if returned == value
    ));
    let MachineStep::Suspended(suspension) = program.step(branches[1]).expect("probe exists")
    else {
        panic!("second branch must suspend")
    };
    let raw = RawReturnRef::from_artifact_ref(artifact(0x14));
    let resumption = suspension.resume(raw);
    assert_eq!(resumption.raw_return(), raw);
    assert_eq!(resumption.state().target(), BlockTarget::new(3));

    let source_continuation = IProgRef::from_artifact_ref(artifact(0x15));
    let lowering = ContinuationLowering::new(source_continuation, BlockTarget::new(3));
    assert!(lowering.check(&program).is_ok());
    assert_eq!(lowering.source(), source_continuation);
    assert_eq!(lowering.target(), BlockTarget::new(3));
    assert!(matches!(
        ContinuationLowering::new(source_continuation, BlockTarget::new(99)).check(&program),
        Err(AdmittedResumeError::UnknownLoweringTarget(target))
            if target == BlockTarget::new(99)
    ));
}

#[test]
fn runtime_verifier_rejects_empty_dangling_and_unguarded_branch_control_flow() {
    let mut catalog = Catalog::default();
    let binding = binding(0x21);
    let unit = catalog.insert_type(TypeArtifact::new(binding, TyIR::Unit));
    let empty_branch = ProgramIR::new(
        unit,
        BlockTarget::new(0),
        vec![BasicBlock::new(
            BlockTarget::new(0),
            Terminator::Branch {
                targets: Vec::new(),
            },
        )],
    );
    assert!(matches!(
        empty_branch.verify(&catalog),
        Err(ProgramCheckError::EmptyBranch(target)) if target == BlockTarget::new(0)
    ));
    let dangling = ProgramIR::new(
        unit,
        BlockTarget::new(0),
        vec![BasicBlock::new(
            BlockTarget::new(0),
            Terminator::Branch {
                targets: vec![BlockTarget::new(1)],
            },
        )],
    );
    assert!(matches!(
        dangling.verify(&catalog),
        Err(ProgramCheckError::UnknownBlockTarget(target)) if target == BlockTarget::new(1)
    ));
    let unguarded = ProgramIR::new(
        unit,
        BlockTarget::new(0),
        vec![BasicBlock::new(
            BlockTarget::new(0),
            Terminator::Branch {
                targets: vec![BlockTarget::new(0)],
            },
        )],
    );
    assert!(matches!(
        unguarded.verify(&catalog),
        Err(ProgramCheckError::UnguardedBranchCycle(target)) if target == BlockTarget::new(0)
    ));
}

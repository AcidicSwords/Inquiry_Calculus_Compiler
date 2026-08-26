use std::collections::BTreeMap;

use ic_core::{
    ArtifactRef, BackendBoundaryCatalog, BackendBoundaryCheckError, BackendBoundaryError,
    BackendRequest, BoundaryRef, ProbeContractRef, ProbeOperator, ProbeOperatorRef, QueryRef,
    SurfacePlan, SurfacePlanRef, TypeRef,
};

#[derive(Default)]
struct Catalog {
    operators: BTreeMap<ProbeOperatorRef, ProbeOperator>,
    plans: BTreeMap<SurfacePlanRef, SurfacePlan>,
}

impl BackendBoundaryCatalog for Catalog {
    fn resolve_probe_operator(&self, reference: ProbeOperatorRef) -> Option<ProbeOperator> {
        self.operators.get(&reference).cloned()
    }

    fn resolve_surface_plan(&self, reference: SurfacePlanRef) -> Option<SurfacePlan> {
        self.plans.get(&reference).cloned()
    }
}

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn operator() -> ProbeOperator {
    ProbeOperator::new(
        QueryRef::from_artifact_ref(artifact(1)),
        BoundaryRef::from_artifact_ref(artifact(2)),
        artifact(3),
        artifact(4),
        artifact(5),
        TypeRef::from_artifact_ref(artifact(6)),
        artifact(7),
        ProbeContractRef::from_artifact_ref(artifact(8)),
        artifact(9),
    )
}

#[test]
fn surface_and_backend_request_are_distinct_checked_operator_derivations() {
    let mut catalog = Catalog::default();
    let operator = operator();
    let operator_ref = operator.probe_operator_ref().expect("operator must encode");
    catalog.operators.insert(operator_ref, operator.clone());
    let plan = SurfacePlan::new(
        operator_ref,
        operator.query(),
        operator.boundary(),
        operator.active_view(),
        operator.executable_code(),
        operator.probe_contract(),
        artifact(10),
        artifact(11),
    );
    plan.check(&catalog)
        .expect("surface plan must preserve exact operator fields");
    let plan_ref = plan.surface_plan_ref().expect("plan must encode");
    catalog.plans.insert(plan_ref, plan.clone());
    let request = BackendRequest::new(
        operator_ref,
        plan_ref,
        operator.query(),
        operator.boundary(),
        operator.backend(),
        operator.executable_code(),
        operator.compiler_version(),
        artifact(12),
        artifact(13),
    );
    request
        .check(&catalog)
        .expect("request must preserve exact plan/operator derivation");
    assert_eq!(
        SurfacePlan::from_envelope(&plan.envelope().expect("plan must encode"))
            .expect("plan must decode"),
        plan
    );
    assert_eq!(
        BackendRequest::from_envelope(&request.envelope().expect("request must encode"))
            .expect("request must decode"),
        request
    );
    assert_ne!(
        plan_ref.as_artifact_ref(),
        request
            .backend_request_ref()
            .expect("request must encode")
            .as_artifact_ref()
    );

    let other_body = BackendRequest::new(
        operator_ref,
        plan_ref,
        operator.query(),
        operator.boundary(),
        operator.backend(),
        operator.executable_code(),
        operator.compiler_version(),
        artifact(12),
        artifact(14),
    );
    assert_ne!(
        request.backend_request_ref().expect("request must encode"),
        other_body
            .backend_request_ref()
            .expect("request must encode")
    );
}

#[test]
fn backend_request_rejects_borrowed_plan_and_operator_fields() {
    let mut catalog = Catalog::default();
    let operator = operator();
    let operator_ref = operator.probe_operator_ref().expect("operator must encode");
    catalog.operators.insert(operator_ref, operator.clone());
    let plan = SurfacePlan::new(
        operator_ref,
        operator.query(),
        operator.boundary(),
        operator.active_view(),
        operator.executable_code(),
        operator.probe_contract(),
        artifact(10),
        artifact(11),
    );
    let plan_ref = plan.surface_plan_ref().expect("plan must encode");
    catalog.plans.insert(plan_ref, plan);

    let wrong_backend = BackendRequest::new(
        operator_ref,
        plan_ref,
        operator.query(),
        operator.boundary(),
        artifact(0xfe),
        operator.executable_code(),
        operator.compiler_version(),
        artifact(12),
        artifact(13),
    );
    assert!(matches!(
        wrong_backend.check(&catalog),
        Err(BackendBoundaryCheckError::OperatorFieldMismatch {
            field: "backend",
            ..
        })
    ));

    let borrowed_plan = BackendRequest::new(
        ProbeOperatorRef::from_artifact_ref(artifact(0xfd)),
        plan_ref,
        operator.query(),
        operator.boundary(),
        operator.backend(),
        operator.executable_code(),
        operator.compiler_version(),
        artifact(12),
        artifact(13),
    );
    assert!(matches!(
        borrowed_plan.check(&catalog),
        Err(BackendBoundaryCheckError::UnresolvedProbeOperator(_))
    ));
}

#[test]
fn backend_boundary_rejects_noncanonical_payload_lengths() {
    assert!(matches!(
        SurfacePlan::decode_payload(&[0; 255]),
        Err(BackendBoundaryError::WrongPayloadLength {
            record: "surface plan",
            expected: 256,
            actual: 255,
        })
    ));
    assert!(matches!(
        BackendRequest::decode_payload(&[0; 287]),
        Err(BackendBoundaryError::WrongPayloadLength {
            record: "backend request",
            expected: 288,
            actual: 287,
        })
    ));
}

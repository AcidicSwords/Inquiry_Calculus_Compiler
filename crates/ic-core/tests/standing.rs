use ic_core::{
    ArtifactRef, ClaimRef, RelationRef, StandingProblem, SupportEnvironment, SupportSubjectRef,
    standing,
};

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn claim(byte: u8) -> ClaimRef {
    ClaimRef::from_artifact_ref(artifact(byte))
}

fn relation(byte: u8) -> RelationRef {
    RelationRef::from_artifact_ref(artifact(byte))
}

const ROOT: u8 = 0x01;
const LEFT: u8 = 0x02;
const RIGHT: u8 = 0x03;
const DERIVED: u8 = 0x04;

#[test]
fn a_rootless_support_cycle_never_enters_standing() {
    // `left` is supported only by `right`, and `right` only by `left`. The pair is entirely
    // self-consistent: assume both stand and every route closes. That is exactly why the least
    // fixed point is the right one -- the greatest fixed point admits this cycle.
    let cycle = StandingProblem::new(
        Vec::new(),
        vec![
            SupportEnvironment::new(claim(LEFT), vec![claim(RIGHT)]),
            SupportEnvironment::new(claim(RIGHT), vec![claim(LEFT)]),
        ],
    );
    let result = standing(&cycle);
    assert!(result.claims().is_empty());
    assert!(!result.contains(claim(LEFT)));
    assert!(!result.contains(claim(RIGHT)));

    // The theorem forbids rootless cycles, not cycles. Ground one member and both arrive: `left`
    // through ingress, `right` through the route that was there all along.
    let grounded = StandingProblem::new(
        vec![claim(LEFT)],
        vec![
            SupportEnvironment::new(claim(LEFT), vec![claim(RIGHT)]),
            SupportEnvironment::new(claim(RIGHT), vec![claim(LEFT)]),
        ],
    );
    let result = standing(&grounded);
    assert!(result.contains(claim(LEFT)));
    assert!(result.contains(claim(RIGHT)));
    assert_eq!(result.claims().len(), 2);

    // Ingress is grounded, so it carries no admitting route; `right` earned one.
    assert_eq!(result.admitted_by(claim(LEFT)), None);
    assert_eq!(result.admitted_by(claim(RIGHT)), Some(1));
}

#[test]
fn standing_grows_only_through_routes_that_already_reach() {
    // root (ingress) -> left -> derived, with `right` reachable from root as well.
    let problem = StandingProblem::new(
        vec![claim(ROOT)],
        vec![
            SupportEnvironment::new(claim(DERIVED), vec![claim(LEFT), claim(RIGHT)]),
            SupportEnvironment::new(claim(LEFT), vec![claim(ROOT)]),
            SupportEnvironment::new(claim(RIGHT), vec![claim(ROOT)]),
        ],
    );
    let result = standing(&problem);
    assert_eq!(
        result.claims().iter().copied().collect::<Vec<_>>(),
        vec![claim(ROOT), claim(LEFT), claim(RIGHT), claim(DERIVED)]
    );

    // Nothing beyond what the routes reach is admitted.
    assert!(!result.contains(claim(0x09)));

    // Removing the ingress leaves the whole structure unreachable, cycle or not.
    let ungrounded = StandingProblem::new(
        Vec::new(),
        vec![
            SupportEnvironment::new(claim(DERIVED), vec![claim(LEFT), claim(RIGHT)]),
            SupportEnvironment::new(claim(LEFT), vec![claim(ROOT)]),
            SupportEnvironment::new(claim(RIGHT), vec![claim(ROOT)]),
        ],
    );
    assert!(standing(&ungrounded).claims().is_empty());
}

#[test]
fn each_closed_support_condition_independently_blocks_a_route() {
    let base = || SupportEnvironment::new(claim(DERIVED), vec![claim(ROOT)]);
    let ingress = || vec![claim(ROOT)];

    // The unmodified route closes, so each blocked variant below differs in exactly one condition.
    assert!(standing(&StandingProblem::new(ingress(), vec![base()])).contains(claim(DERIVED)));

    let blocked = [
        // An open dependency boundary: required, neither supplied nor discharged.
        base().with_open_dependencies(vec![artifact(0xAA)]),
        // Applicability and scope do not hold for this route.
        base().with_applicability(false),
        // The independent checks the route requires did not succeed.
        base().with_checks_discharged(false),
        // An explicit inconsistency policy invalidates the environment.
        base().invalidated(true),
    ];
    for environment in blocked {
        let result = standing(&StandingProblem::new(ingress(), vec![environment]));
        assert!(!result.contains(claim(DERIVED)));
        // The premise it rested on is untouched: a blocked route removes nothing.
        assert!(result.contains(claim(ROOT)));
    }

    // A premise that does not stand blocks the route without any declaration being false.
    let unstanding_premise = StandingProblem::new(
        ingress(),
        vec![SupportEnvironment::new(claim(DERIVED), vec![claim(0x0F)])],
    );
    assert!(!standing(&unstanding_premise).contains(claim(DERIVED)));
}

#[test]
fn one_closed_route_suffices_when_another_is_blocked() {
    // The specification allows several incomparable support environments for one claim. A claim
    // stands when some route closes, not when every route does.
    let problem = StandingProblem::new(
        vec![claim(ROOT)],
        vec![
            SupportEnvironment::new(claim(DERIVED), vec![claim(ROOT)]).with_applicability(false),
            SupportEnvironment::new(claim(DERIVED), vec![claim(ROOT)]),
        ],
    );
    let result = standing(&problem);
    assert!(result.contains(claim(DERIVED)));
    assert_eq!(
        result.admitted_by(claim(DERIVED)),
        Some(1),
        "the blocked route at index 0 did not admit it"
    );

    // With only the blocked route declared, the claim does not stand.
    let only_blocked = StandingProblem::new(
        vec![claim(ROOT)],
        vec![SupportEnvironment::new(claim(DERIVED), vec![claim(ROOT)]).with_applicability(false)],
    );
    assert!(!standing(&only_blocked).contains(claim(DERIVED)));
}

#[test]
fn mixed_claim_relation_standing_preserves_kinds_and_rejects_rootless_cycles() {
    let claim_subject = SupportSubjectRef::Claim(claim(LEFT));
    let relation_subject = SupportSubjectRef::Relation(relation(RIGHT));
    let cycle = StandingProblem::for_subjects(
        Vec::new(),
        vec![
            SupportEnvironment::for_subjects(claim_subject, vec![relation_subject]),
            SupportEnvironment::for_subjects(relation_subject, vec![claim_subject]),
        ],
    );
    let ungrounded = standing(&cycle);
    assert!(ungrounded.subjects().is_empty());
    assert!(!ungrounded.contains(claim(LEFT)));
    assert!(!ungrounded.contains_relation(relation(RIGHT)));

    let grounded =
        StandingProblem::for_subjects(vec![relation_subject], cycle.environments().to_vec());
    let result = standing(&grounded);
    assert!(result.contains_subject(claim_subject));
    assert!(result.contains_subject(relation_subject));
    assert!(result.contains(claim(LEFT)));
    assert!(result.contains_relation(relation(RIGHT)));
    assert_eq!(
        result.claims().iter().copied().collect::<Vec<_>>(),
        vec![claim(LEFT)]
    );
    assert_eq!(
        result.relations().iter().copied().collect::<Vec<_>>(),
        vec![relation(RIGHT)]
    );
    assert_eq!(result.subject_admitted_by(relation_subject), None);
    assert_eq!(result.relation_admitted_by(relation(RIGHT)), None);
    assert_eq!(result.subject_admitted_by(claim_subject), Some(0));
}

#[test]
fn standing_subject_kind_separates_equal_underlying_digests() {
    let shared = artifact(0xA5);
    let claim_subject = SupportSubjectRef::Claim(ClaimRef::from_artifact_ref(shared));
    let relation_subject = SupportSubjectRef::Relation(RelationRef::from_artifact_ref(shared));
    let result = standing(&StandingProblem::for_subjects(
        vec![claim_subject, relation_subject],
        Vec::new(),
    ));

    assert_eq!(result.subjects().len(), 2);
    assert!(result.contains_subject(claim_subject));
    assert!(result.contains_subject(relation_subject));
}

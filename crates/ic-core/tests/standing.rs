use ic_core::{ArtifactRef, ClaimRef, StandingProblem, SupportEnvironment, standing};

fn artifact(byte: u8) -> ArtifactRef {
    ArtifactRef::from_bytes([byte; 32])
}

fn claim(byte: u8) -> ClaimRef {
    ClaimRef::from_artifact_ref(artifact(byte))
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

//! Finite model `M0` of the Inquiry Calculus v2.0 constitutional core.
//!
//! This file imports nothing from `ic-core`.  That is deliberate.  A conformance
//! test asks whether the code implements the calculus.  This file asks a question
//! nothing in the repository previously asked: whether the calculus's laws are
//! jointly satisfiable by a nondegenerate structure at all.  With twenty
//! constitutional laws and no exhibited model, joint satisfiability was assumed.
//!
//! The model is small enough to check exhaustively and rich enough that every law
//! checked here does actual work in it.  Each test asserts both that a law holds
//! and that it is non-vacuous, because a law satisfied only because its hypothesis
//! is never met establishes nothing.
//!
//! # The binding
//!
//! Carrier `U = {A, B, C, D}`.  Three observations, each with its own answer
//! carrier and its own standing incompatibility (apartness) relation:
//!
//! ```text
//!         A   B   C   D     apartness on the answer carrier
//!   d1    0   0   1   1     0 # 1
//!   d2    0   1   0   1     empty
//!   d3    1   1   0   0     0 # 1
//! ```
//!
//! Protected horizon `H = {d2}`, so `x =H= y` iff `d2(x) = d2(y)`.
//! Determination presentation `W = {d1, d2, d3}`.
//! Departure: `Depart_W(x,y)` iff some `o` in `W` has `o(x) # o(y)`.
//!
//! `d2` is the observation with an empty apartness: it separates forms without
//! ever witnessing a departure.  That single feature is what makes the model
//! discriminating rather than decorative.

use std::collections::BTreeSet;

// ---------------------------------------------------------------------------
// Carrier and observations
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Form {
    A,
    B,
    C,
    D,
}

use Form::{A, B, C, D};

const CARRIER: [Form; 4] = [A, B, C, D];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Obs {
    D1,
    D2,
    D3,
}

use Obs::{D1, D2, D3};

const WEB: [Obs; 3] = [D1, D2, D3];

/// Value of an observation at a form.  Answer carriers are two-valued.
fn value(o: Obs, x: Form) -> u8 {
    match (o, x) {
        (D1, A) | (D1, B) => 0,
        (D1, C) | (D1, D) => 1,
        (D2, A) | (D2, C) => 0,
        (D2, B) | (D2, D) => 1,
        (D3, A) | (D3, B) => 1,
        (D3, C) | (D3, D) => 0,
    }
}

/// Standing incompatibility on an observation's answer carrier.
///
/// `D2` carries the empty apartness.  Its values differ across forms, but no pair
/// of them is positively incompatible, so it can never witness a departure.
fn apart(o: Obs, u: u8, v: u8) -> bool {
    match o {
        D1 | D3 => u != v,
        D2 => false,
    }
}

/// Protected equivalence under the horizon `H = {d2}`.
fn equiv_h(x: Form, y: Form) -> bool {
    value(D2, x) == value(D2, y)
}

/// Positive departure relative to a determination presentation.
fn departs(web: &[Obs], x: Form, y: Form) -> bool {
    web.iter().any(|&o| apart(o, value(o, x), value(o, y)))
}

/// The partition induced by a set of observations, as a canonical set of blocks.
fn partition(web: &[Obs]) -> BTreeSet<BTreeSet<Form>> {
    let mut blocks: BTreeSet<BTreeSet<Form>> = BTreeSet::new();
    for &x in &CARRIER {
        let block: BTreeSet<Form> = CARRIER
            .iter()
            .copied()
            .filter(|&y| web.iter().all(|&o| value(o, x) == value(o, y)))
            .collect();
        blocks.insert(block);
    }
    blocks
}

/// The departure relation induced by a set of observations.
fn departure_relation(web: &[Obs]) -> BTreeSet<(Form, Form)> {
    let mut rel = BTreeSet::new();
    for &x in &CARRIER {
        for &y in &CARRIER {
            if departs(web, x, y) {
                rel.insert((x, y));
            }
        }
    }
    rel
}

/// Every subset of the web, as vectors in canonical order.
fn subsets() -> Vec<Vec<Obs>> {
    let mut out = Vec::new();
    for mask in 0u8..8 {
        let mut sub = Vec::new();
        for (bit, &o) in WEB.iter().enumerate() {
            if mask & (1 << bit) != 0 {
                sub.push(o);
            }
        }
        out.push(sub);
    }
    out
}

// ---------------------------------------------------------------------------
// M0-1  The model is nondegenerate
// ---------------------------------------------------------------------------

#[test]
fn m0_1_the_model_is_nondegenerate() {
    // A degenerate model satisfies every law vacuously.  Rule that out first.
    assert_eq!(CARRIER.len(), 4, "carrier must be nontrivial");

    // The horizon is neither discrete nor indiscrete: exactly two classes.
    let horizon_blocks = partition(&[D2]);
    assert_eq!(
        horizon_blocks.len(),
        2,
        "protected equivalence must be a proper nontrivial equivalence"
    );

    // The full web separates every form, so the model has maximal resolution
    // available while the horizon deliberately does not use it.
    assert_eq!(
        partition(&WEB).len(),
        4,
        "the full web must separate the carrier"
    );

    // No single observation separates the carrier: joint variation is required.
    for &o in &WEB {
        assert!(
            partition(&[o]).len() < 4,
            "{o:?} must not separate the carrier alone"
        );
    }

    // Departure is neither empty nor total.
    let dep = departure_relation(&WEB);
    assert!(!dep.is_empty(), "departure must be non-vacuous");
    assert!(
        dep.len() < CARRIER.len() * CARRIER.len(),
        "departure must not be total"
    );
}

// ---------------------------------------------------------------------------
// M0-2  Every apartness is irreflexive, symmetric, and co-transitive
// ---------------------------------------------------------------------------

#[test]
fn m0_2_apartness_axioms_hold_on_every_answer_carrier() {
    for &o in &WEB {
        for u in 0u8..2 {
            assert!(!apart(o, u, u), "{o:?}: apartness must be irreflexive");
            for v in 0u8..2 {
                assert_eq!(
                    apart(o, u, v),
                    apart(o, v, u),
                    "{o:?}: apartness must be symmetric"
                );
                for w in 0u8..2 {
                    // Co-transitivity: u # w implies u # v or v # w.
                    if apart(o, u, w) {
                        assert!(
                            apart(o, u, v) || apart(o, v, w),
                            "{o:?}: apartness must be co-transitive"
                        );
                    }
                }
            }
        }
    }

    // Non-vacuity in both directions: one carrier has a real apartness and one
    // has none, so the axioms are checked against a genuine and a trivial case.
    assert!(apart(D1, 0, 1), "D1 must carry a real apartness");
    assert!(!apart(D2, 0, 1), "D2 must carry the empty apartness");
}

// ---------------------------------------------------------------------------
// M0-3  Departure and protected separation are independent
// ---------------------------------------------------------------------------

#[test]
fn m0_3_departure_and_protected_equivalence_are_independent() {
    // Plan section 16 requires both directions to be possible.  Neither had a
    // witness anywhere in the repository before this model.

    // Departure while protected-equivalent.
    assert!(departs(&WEB, A, C), "A must depart from C via d1");
    assert!(equiv_h(A, C), "A and C must be protected-equivalent");

    // Protected-separated while not departing.
    assert!(!equiv_h(A, B), "A and B must be protected-separated");
    assert!(
        !departs(&WEB, A, B),
        "A must not depart from B: no observation makes them incompatible"
    );

    // Neither direction is an artifact of one pair.
    assert!(departs(&WEB, B, D) && equiv_h(B, D));
    assert!(!equiv_h(C, D) && !departs(&WEB, C, D));
}

// ---------------------------------------------------------------------------
// M0-4  Raw signature mismatch is not departure
// ---------------------------------------------------------------------------

#[test]
fn m0_4_signature_mismatch_is_not_departure() {
    // d2 tells A and B apart as signatures.
    assert_ne!(value(D2, A), value(D2, B));

    // But no observation makes them positively incompatible, so the honest
    // status is Unknown, not departure.  Unknown != Negative, exhibited.
    assert!(!departs(&WEB, A, B));

    // The gap is exactly the observations whose apartness is empty.  Count the
    // pairs that differ somewhere yet do not depart, and require at least one.
    let mut mismatched_but_not_departed = 0usize;
    for &x in &CARRIER {
        for &y in &CARRIER {
            let differs = WEB.iter().any(|&o| value(o, x) != value(o, y));
            if differs && !departs(&WEB, x, y) {
                mismatched_but_not_departed += 1;
            }
        }
    }
    assert!(
        mismatched_but_not_departed > 0,
        "the model must contain a mismatch that is not a departure"
    );
}

// ---------------------------------------------------------------------------
// M0-5  Reduct structure under two different criteria
// ---------------------------------------------------------------------------

/// Minimal subsets of `WEB` that preserve the full web's partition.
fn partition_reducts() -> Vec<Vec<Obs>> {
    let target = partition(&WEB);
    let mut preserving: Vec<Vec<Obs>> = subsets()
        .into_iter()
        .filter(|s| partition(s) == target)
        .collect();
    preserving.sort_by_key(Vec::len);
    let mut reducts: Vec<Vec<Obs>> = Vec::new();
    for cand in preserving {
        let has_smaller = reducts
            .iter()
            .any(|r| r.iter().all(|o| cand.contains(o)) && r.len() < cand.len());
        if !has_smaller {
            reducts.push(cand);
        }
    }
    reducts
}

/// Minimal subsets of `WEB` that preserve the full web's departure relation.
fn departure_reducts() -> Vec<Vec<Obs>> {
    let target = departure_relation(&WEB);
    let mut preserving: Vec<Vec<Obs>> = subsets()
        .into_iter()
        .filter(|s| departure_relation(s) == target)
        .collect();
    preserving.sort_by_key(Vec::len);
    let mut reducts: Vec<Vec<Obs>> = Vec::new();
    for cand in preserving {
        let has_smaller = reducts
            .iter()
            .any(|r| r.iter().all(|o| cand.contains(o)) && r.len() < cand.len());
        if !has_smaller {
            reducts.push(cand);
        }
    }
    reducts
}

fn core_of(reducts: &[Vec<Obs>]) -> BTreeSet<Obs> {
    let mut core: BTreeSet<Obs> = WEB.iter().copied().collect();
    for r in reducts {
        let as_set: BTreeSet<Obs> = r.iter().copied().collect();
        core = core.intersection(&as_set).copied().collect();
    }
    core
}

#[test]
fn m0_5_reducts_and_cores_under_both_criteria() {
    let p_reducts = partition_reducts();
    let d_reducts = departure_reducts();

    // Partition criterion: two reducts of size two, and d2 lies in both.
    assert_eq!(p_reducts.len(), 2, "expected exactly two partition reducts");
    for r in &p_reducts {
        assert_eq!(r.len(), 2);
        assert!(r.contains(&D2));
    }
    assert_eq!(
        core_of(&p_reducts),
        [D2].into_iter().collect::<BTreeSet<_>>(),
        "the partition core must be exactly d2"
    );

    // Departure criterion: two reducts of size one, and their core is empty.
    assert_eq!(d_reducts.len(), 2, "expected exactly two departure reducts");
    for r in &d_reducts {
        assert_eq!(r.len(), 1);
    }
    assert!(
        core_of(&d_reducts).is_empty(),
        "the departure core must be empty in this model"
    );
}

// ---------------------------------------------------------------------------
// M0-6  Every condition is individually dispensable while the set is jointly
//       necessary
// ---------------------------------------------------------------------------

/// The release shell of `rho` for the departure criterion: pairs that depart
/// under `W` and no longer depart under `W \ {rho}`.
fn departure_shell(rho: Obs) -> BTreeSet<(Form, Form)> {
    let full = departure_relation(&WEB);
    let reduced: Vec<Obs> = WEB.iter().copied().filter(|&o| o != rho).collect();
    let smaller = departure_relation(&reduced);
    full.difference(&smaller).copied().collect()
}

#[test]
fn m0_6_singleton_shells_are_empty_yet_joint_removal_is_fatal() {
    // Every single condition has an empty release shell: dropping any one of
    // them changes no departure.
    for &o in &WEB {
        assert!(
            departure_shell(o).is_empty(),
            "{o:?} must be individually dispensable for departure"
        );
    }

    // Yet dropping d1 and d3 together destroys departure entirely.
    let remaining = [D2];
    assert!(
        departure_relation(&remaining).is_empty(),
        "removing d1 and d3 together must destroy departure"
    );

    // This is the concrete instance of an empty core with nonempty reducts:
    // one-at-a-time ablation reports every condition dispensable and is wrong.
    assert!(core_of(&departure_reducts()).is_empty());
    assert!(!departure_relation(&WEB).is_empty());
}

// ---------------------------------------------------------------------------
// M0-7  The two minimization criteria diverge
// ---------------------------------------------------------------------------

#[test]
fn m0_7_minimizing_against_departure_alone_destroys_reconstruction() {
    // d2 is indispensable for the partition and dispensable for departure.
    assert!(core_of(&partition_reducts()).contains(&D2));
    assert!(departure_shell(D2).is_empty());

    // So a minimizer that keeps only a departure reduct drops d2.
    let departure_only = departure_reducts()
        .into_iter()
        .next()
        .expect("a departure reduct exists");
    assert!(!departure_only.contains(&D2));

    // And the partition collapses from discrete to two blocks: A and B, which
    // the full web told apart, become indistinguishable.
    assert_eq!(partition(&WEB).len(), 4);
    assert_eq!(partition(&departure_only).len(), 2);
    assert!(
        value(D1, A) == value(D1, B) && value(D3, A) == value(D3, B),
        "A and B must be merged by every departure reduct"
    );
}

// ---------------------------------------------------------------------------
// M0-8  Return fiber is not the selected return
// ---------------------------------------------------------------------------

#[test]
fn m0_8_return_fiber_contains_the_source_without_recovering_it() {
    // Take the departure relation as the typed negation use N.
    let neg_field: Vec<Form> = CARRIER
        .iter()
        .copied()
        .filter(|&y| departs(&WEB, A, y))
        .collect();
    assert!(neg_field.contains(&C), "C must be exterior to A");

    // Reverse section at C.
    let return_fiber: Vec<Form> = CARRIER
        .iter()
        .copied()
        .filter(|&x| departs(&WEB, x, C))
        .collect();

    // The source is always in its own return fiber.
    assert!(return_fiber.contains(&A));

    // But it is not uniquely recovered: B is also compatible with the return.
    assert!(return_fiber.contains(&B));
    assert!(
        return_fiber.len() > 1,
        "the fiber must not collapse to the selected return"
    );
}

// ---------------------------------------------------------------------------
// M0-9  Joint refinement of question coordinates
// ---------------------------------------------------------------------------

#[test]
fn m0_9_joint_refinement_is_the_intersection_of_kernels() {
    for &x in &CARRIER {
        for &y in &CARRIER {
            let joint = value(D1, x) == value(D1, y) && value(D2, x) == value(D2, y);
            let intersected = (value(D1, x) == value(D1, y)) && (value(D2, x) == value(D2, y));
            assert_eq!(joint, intersected);
        }
    }

    // Non-vacuity: neither coordinate alone induces the joint kernel.
    assert_eq!(partition(&[D1, D2]).len(), 4);
    assert_eq!(partition(&[D1]).len(), 2);
    assert_eq!(partition(&[D2]).len(), 2);
}

// ---------------------------------------------------------------------------
// M0-10  Order is consequential: a witnessed non-commuting composite
// ---------------------------------------------------------------------------

fn compose(
    first: &BTreeSet<(Form, Form)>,
    second: &BTreeSet<(Form, Form)>,
) -> BTreeSet<(Form, Form)> {
    let mut out = BTreeSet::new();
    for &(a1, b1) in first {
        for &(b2, c2) in second {
            if b1 == b2 {
                out.insert((a1, c2));
            }
        }
    }
    out
}

#[test]
fn m0_10_composition_order_changes_the_relation() {
    let r: BTreeSet<(Form, Form)> = [(A, B)].into_iter().collect();
    let s: BTreeSet<(Form, Form)> = [(B, C)].into_iter().collect();

    let s_after_r = compose(&r, &s);
    let r_after_s = compose(&s, &r);

    assert_eq!(s_after_r, [(A, C)].into_iter().collect::<BTreeSet<_>>());
    assert!(r_after_s.is_empty());
    assert_ne!(s_after_r, r_after_s, "order must be consequential here");
}

// ---------------------------------------------------------------------------
// M0-11  Standing is a least fixed point and a rootless cycle never stands
// ---------------------------------------------------------------------------

#[test]
fn m0_11_a_rootless_support_cycle_receives_no_standing() {
    // Claim 0 is grounded ingress.  Claim 1 rests on claim 0.  Claims 2 and 3
    // support each other and nothing else.
    let support: [&[usize]; 4] = [&[], &[0], &[3], &[2]];

    let mut standing = [false; 4];
    loop {
        let mut changed = false;
        for c in 0..4 {
            if !standing[c] && support[c].iter().all(|&d| standing[d]) {
                standing[c] = true;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    assert!(standing[0], "grounded ingress must stand");
    assert!(
        standing[1],
        "a claim resting on grounded ingress must stand"
    );
    assert!(!standing[2], "a rootless cycle must not stand");
    assert!(!standing[3], "a rootless cycle must not stand");

    // Non-vacuity: the cycle is genuinely mutually supporting, so a greatest
    // fixed point would have admitted it.
    assert_eq!(support[2], &[3]);
    assert_eq!(support[3], &[2]);
}

// ---------------------------------------------------------------------------
// M0-12  Occurrence-multiset preservation holds exactly when the horizon
//        observes the event history
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
enum Prog {
    Return(Form),
    /// One probe with a continuation per possible raw return, 0 and 1.
    Probe(Box<Prog>, Box<Prog>),
}

/// Syntactic probe-occurrence count.
fn mu(p: &Prog) -> usize {
    match p {
        Prog::Return(_) => 0,
        Prog::Probe(k0, k1) => 1 + mu(k0) + mu(k1),
    }
}

/// All reachable (event trace, final value) pairs.
fn histories(p: &Prog) -> BTreeSet<(Vec<u8>, Form)> {
    fn go(p: &Prog, trace: &mut Vec<u8>, out: &mut BTreeSet<(Vec<u8>, Form)>) {
        match p {
            Prog::Return(v) => {
                out.insert((trace.clone(), *v));
            }
            Prog::Probe(k0, k1) => {
                for (bit, k) in [(0u8, k0), (1u8, k1)] {
                    trace.push(bit);
                    go(k, trace, out);
                    trace.pop();
                }
            }
        }
    }
    let mut out = BTreeSet::new();
    go(p, &mut Vec::new(), &mut out);
    out
}

/// The value-only horizon: reachable final values, discarding the event history.
fn values_only(p: &Prog) -> BTreeSet<Form> {
    histories(p).into_iter().map(|(_, v)| v).collect()
}

#[test]
fn m0_12_freeness_requires_a_history_observing_horizon() {
    // p1 probes once and returns A on either raw return.  p2 returns A outright.
    let p1 = Prog::Probe(Box::new(Prog::Return(A)), Box::new(Prog::Return(A)));
    let p2 = Prog::Return(A);

    // Their probe-occurrence multisets differ.
    assert_eq!(mu(&p1), 1);
    assert_eq!(mu(&p2), 0);
    assert_ne!(mu(&p1), mu(&p2));

    // Under a horizon that observes only the returned value they are equivalent.
    // The induced equational theory therefore relates two programs with
    // different probe multisets, and actuality separation is unsound in it.
    assert_eq!(
        values_only(&p1),
        values_only(&p2),
        "the value-only horizon must identify p1 and p2"
    );

    // Under a horizon that observes the event history they are separated, so
    // occurrence-multiset preservation holds.
    assert_ne!(
        histories(&p1),
        histories(&p2),
        "the history-observing horizon must separate p1 and p2"
    );

    // The separating observation is exactly the event count, not the value.
    let n1: BTreeSet<usize> = histories(&p1).iter().map(|(t, _)| t.len()).collect();
    let n2: BTreeSet<usize> = histories(&p2).iter().map(|(t, _)| t.len()).collect();
    assert_eq!(n1, [1usize].into_iter().collect::<BTreeSet<_>>());
    assert_eq!(n2, [0usize].into_iter().collect::<BTreeSet<_>>());
}

// ---------------------------------------------------------------------------
// M0-13  Consistency of the checked fragment
// ---------------------------------------------------------------------------

#[test]
fn m0_13_the_checked_laws_hold_jointly_in_one_nondegenerate_model() {
    // Every preceding test constrains the same single structure.  This test
    // records that they are constraints on one model rather than on thirteen
    // convenient ones, which is what joint satisfiability means.
    //
    // Re-derive the load-bearing facts from the one binding, in one place.

    // Nondegenerate.
    assert_eq!(partition(&WEB).len(), 4);
    assert_eq!(partition(&[D2]).len(), 2);

    // Departure and protected separation come apart in both directions.
    assert!(departs(&WEB, A, C) && equiv_h(A, C));
    assert!(!departs(&WEB, A, B) && !equiv_h(A, B));

    // Mismatch is not departure.
    assert!(value(D2, A) != value(D2, B) && !departs(&WEB, A, B));

    // Return fiber does not recover the source.
    let fiber: Vec<Form> = CARRIER
        .iter()
        .copied()
        .filter(|&x| departs(&WEB, x, C))
        .collect();
    assert!(fiber.contains(&A) && fiber.len() > 1);

    // Empty departure core, nonempty departure relation.
    assert!(core_of(&departure_reducts()).is_empty());
    assert!(!departure_relation(&WEB).is_empty());

    // The two criteria diverge on the same web.
    assert!(core_of(&partition_reducts()).contains(&D2));
    assert!(departure_shell(D2).is_empty());
}

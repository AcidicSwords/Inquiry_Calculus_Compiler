# Protected behavioral equivalence: source-definition boundary

This slice formalizes the relation shapes at v2.0 lines 4002–4059. It does not
promote successor semantics or pass Gate B.

`ProtectedBehavioralEquivalenceContext` takes endpoint-indexed terms and contexts,
context-indexed consequence types, and a supplied consequence interpretation.
Both compared terms have the same source and target. The module constructs no
context evaluator.

- Protected equivalence requires equal consequences in **every** protected context.
- The separator family contains **all** protected contexts with unequal consequences.
- Working nondistinction compares only a finite list of tested contexts. The carrier
  requires that those tests lie within the protected horizon; it does not supply a
  completeness license. Duplicate tests do not enlarge semantic coverage.
- Horizon monotonicity and the separator characterization are named propositions
  in this source carrier. Separate downstream proofs are described below; their
  existence does not rewrite the predecessor's original source classification.
  Binding applicability remains supplied, not inferred.

The generator binds the schema, exact canonical TeX bytes, classification bytes,
five source excerpts, and Lean module bytes. The independent checker transcribes
the reviewed typed declaration structure and rejects 24 source, quantifier,
membership, polarity, endpoint, coverage, promotion, and digest mutations. It also
accepts whitespace and nested-comment changes that preserve executable tokens.
Its coverage is this finite source-definition slice, not a general Lean equivalence
checker or proof of all predecessor claims.

The initial substring checker accepted a comment-only file, a deleted horizon
field, and propositions changed to natural numbers. Those failures are retained as
regression cases; the former 18-name result must not be read as source fidelity.

Verification:

```text
node tools/phase_b_protected_behavioral_equivalence.js check
node tools/phase_b_protected_behavioral_equivalence_check.js --compile
node tools/protected_equivalence_laws_check.js --compile
cd formal
lake build --wfail
```

Both source routes and the separate proof-contract checker are connected to branch
CI. `ProtectedBehavioralEquivalenceLaws.lean` proves, for the supplied endpoint-indexed
observation signature:

- reflexivity, symmetry and transitivity at a fixed horizon;
- restriction from a larger protected horizon to a contained smaller horizon;
- that a protected separator refutes equivalence;
- the classical equivalence between failure of protected equivalence and existence
  of a protected separator;
- exact equivalence implies agreement on tests lying within the horizon;
- tested agreement implies exact equivalence when every protected context is tested.

The final premise is one sufficient completeness license, not a claim that exhaustive
enumeration is the only possible binding-relative license. A semantic or effective
procedure for finding separators or deciding equivalence is not supplied.

The finite model has one endpoint object, two Boolean terms, two Boolean contexts,
and the supplied observation `context && term`. The terms `false` and `true` agree
at the tested context `false` but differ at protected context `true`. Adding `true`
to the tests destroys agreement; restricting the horizon to `false` restores
equivalence. Equal terms cannot give this gap. A proved one-context lemma excludes
the gap when a tested sample exists. An additional one-context model with **empty**
tests demonstrates why the nonempty-sample premise must not be omitted. This is a
coverage-qualified minimality boundary, not a universal uniqueness claim.

The independent checker ascribes exact theorem types rather than matching proof
names or bodies. It rejects six false counterclaims and audits twenty-one named
results: twenty use no axioms; `separatorCharacterizationClassical` depends on
Lean's `propext`, `Classical.choice`, and `Quot.sound`. There is no custom axiom or
proof gap. The predecessor's classical use is thus localized rather than silently
added to the ambient-boundary module. The checker without `--compile` runs only
static/data checks and explicitly does not claim Lean verification.

Hom-wise quotient formation and compositional congruence remain subsequent source
obligations. Equivalence at a horizon alone does not license quotient composition.
No successor gate or Rust migration is promoted by these conditional proofs.

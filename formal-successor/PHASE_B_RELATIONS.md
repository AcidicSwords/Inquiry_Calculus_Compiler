# Phase B typed-relation surface

This local Phase B pass reconstructs the seven consecutive explicit v2.0 relation definitions:
typed binary and n-ary relations, functions as total single-valued relations, identity, serial
composition, representation-admitted converse, image/inverse image, and parallel typed operators.
It does not choose successor primitives or elaborate formulas, questions, fibers, programs,
contextual equivalence, or Rust correspondence. Gate B remains pending.

`Relation B I A C` carries binding-indexed admitted endpoints and a typed incidence predicate.
`RelationSchema` carries a finite typed port context. `FunctionRelation` retains the source's
total-single-valued condition in addition to its relation; it never replaces relations with host
functions. `ConverseBoundary` is explicitly partial, as the source makes converse representation
admitted rather than globally available.

```text
node tools/phase_b_relations.js check
node tools/phase_b_relations_check.js --compile
```

The independent checker regenerates all seven source identities and rejects eleven mutations:
source loss/promotion, Gate B promotion, relation or schema loss, endpoint erasure,
function-condition collapse, converse totalization, formula leakage, raw string collapse, and
axiomatic completion. The next residual is relation expressions.

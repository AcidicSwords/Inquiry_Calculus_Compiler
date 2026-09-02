# Gate C Uniform Generation Boundary

## Result

The missing relation is now typed and independently checked. This does not complete
`IC-THM-C-000`: the boundary still has to be instantiated over the ambient presentations, every
protected predecessor target, and four semantic capability ablations.

`RegenerationWitness(P,t)` retains four coordinates:

1. a construction of target `t` in presentation `P`;
2. an exact source witness;
3. a source correspondence between them;
4. declared coverage of `t`.

`UniformGenerationBoundary` supplies a conservative extension preorder on presentations, a
presentation-indexed protected target family, and lawful transport of old targets and complete
regeneration witnesses. A conservative extension therefore preserves what was already generated;
it is not arbitrary rebinding.

`PointwiseGenerates(P)` asks only for every currently protected target. `UniformlyGenerates(P)`
asks for every protected target after every supplied conservative extension. Uniform generation
implies pointwise generation.

## Decisive atomization foil

The finite foil has a base presentation with one target and a conservative extension that retains
that old target while introducing a fresh protected target. The base can generate every current
target, but the fresh target has no construction. Lean proves:

```text
PointwiseGenerates(base) ∧ ¬ UniformlyGenerates(base)
```

This rejects expressivity-only minimality. Naming each currently desired composite as an atomic
relation can make every current test pass while failing to form the same protected kind of
composite after a fresh atom is added.

## Remaining application

The next application must instantiate presentations and targets from the ambient basis, map every
protected predecessor capability with exact ancestry, prove construction/source correspondence and
coverage, and then construct the four semantic ablations. A failed derivation search, a missing
Boolean capability coordinate, or the boundary definition itself is not a semantic ablation proof.

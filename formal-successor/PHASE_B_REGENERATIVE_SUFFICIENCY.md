# Phase B — Regenerative Sufficiency

## Status

`FORMAL-B-REGENERATIVE-SUFFICIENCY-001` is a checked predecessor-recovery
boundary. It is not promoted successor semantics and does not pass Gate B.

## Exact source coverage

| Source identity | Lines | Classification |
|---|---:|---|
| PRED-TEX-DECL-6B5293AC8D6CC5AD | 4156–4173 | FormalDefinition |
| PRED-TEX-DECL-FB64B23D4F342D30 | 4175–4182 | FormalDefinition |

The canonical TeX bytes and both declaration excerpts are independently
regenerated from `formal-successor/PREDECESSOR_TEX_CLASSIFICATION.json`.

## Contract recovered

The binding supplies a component index, a dependent value type for each
component, a horizon-required predicate, the corresponding component of the
source, and a protected-equivalence judgment. It also supplies a family of typed
reconstruction relations. Relative to those inputs,

```text
RegenerativeSufficient(m,z)
  iff
for every horizon-required component c,
there exists a value v such that
  reconstructs(c,m,v)
and
  protectedEquivalent(c,v,sourceComponent(c,z)).
```

The existential is local to each typed component obligation. The formalization
does not choose a global reconstruction function, search for witnesses, execute
a reconstruction, or define what counts as protected equivalence.

Inquiry-regenerative sufficiency adds a separately supplied revision-role
family:

```text
RegenerativeSufficient(m,z)
and
for every required revision role r,
  retainsOrRegenerates(r,m,z).
```

Discriminator, residual, support-dependency, and reopening roles may be supplied
as members of that family. They are not installed as a universal enumeration or
new semantic primitives.

## Decisive contrast and contraction

The finite countermodel uses two differently typed protected components. An
observed-only representation reconstructs the Boolean component exactly but has
no reconstruction witness for the required natural-number future component.
One successful reconstruction therefore does not establish regenerative
sufficiency.

A second representation reconstructs both components, but its sole required
reopening role is unavailable. It is regenerative and not
inquiry-regenerative. The overlarge contrast contracts to exactly two retained
conditions:

1. universal coverage of the supplied horizon-required component family; and
2. the independent universal coverage of supplied required revision roles.

## Boundary

This result supplies no universal component ontology, equivalence laws,
reconstruction search, chosen witnesses, actual execution, engineering-fold
license, Rust correspondence, successor promotion, or Gate-B pass. The examples
at source lines 4163–4171 are admissible component indices when a binding protects
them; this layer does not assert that the example list is complete for every
horizon.

The next source boundary is regenerative economy at lines 4184–4207: it must add
a binding-supplied resource preorder and license without replacing the retained
component or revision obligations by one consequence factorization.

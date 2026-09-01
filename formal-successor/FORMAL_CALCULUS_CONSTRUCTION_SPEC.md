# Inquiry Calculus — Formal Successor Construction Specification

## A single forward program for converting Inquiry Calculus v2.0 and its current reference implementation into a machine-checked calculus with exact types, exact relations, reciprocal notation and prose, lawful question programs, and regenerative revision

**Suggested filename:** `FORMAL_CALCULUS_CONSTRUCTION_SPEC.md`

**Status:** Forward construction authority for the formal-successor project.

**Semantic status:** This document does not define the successor calculus. It defines the procedure by which the successor is to be constructed, broken, proved, propagated, minimized, regenerated, and promoted.

**Intended consumer:** An implementation/formalization agent with access to the repository and supplied active files but no access to the conversations or exploratory documents from which this specification was synthesized.

---

# 0. Active inputs

The agent executing this program requires only:

1. `Inquiry_Calculus_v2_0.tex` — the predecessor semantic specification;
2. the current `AcidicSwords/Inquiry_Calculus_Compiler` repository — the predecessor reference implementation and executable evidence;
3. `Questions.txt` — the external natural-language inquiry corpus;
4. this construction specification.

No other semantic, migration, exploratory, finite-model, or conversational document is required.

The requirements below are already the normalized results that survived the work performed after v2.0.

Do not reconstruct that historical work.

Do not preserve terminology merely because it appeared in it.

Do not create a second active specification from it.

The governing rule is:

\[
\boxed{
\textbf{RETAIN THE SURVIVING RELATION, NOT THE DOCUMENT THAT EXPOSED IT.}
}
\]

If removal of an earlier document would cause a consequential distinction to disappear, this specification is incomplete. The repair is to state the missing relation here or in the formal calculus, not to restore the earlier document as a permanent dependency.

---

# 1. Goal

Construct one successor calculus

\[
\boxed{C_1}
\]

from the predecessor

\[
\boxed{C_0:=\text{Inquiry Calculus v2.0}}
\]

such that \(C_1\) reaches, as far as the subject permits, the formal standard expected of a proper calculus.

This does **not** mean making Inquiry Calculus resemble differential or integral calculus.

It means establishing the same kind of formal discipline found in mature calculi:

- an explicit ambient metalanguage;
- formation judgments;
- exact types;
- a small generating syntax;
- exact semantic interpretation;
- derived constructions;
- equations and inequations;
- composition laws;
- adjunctions or reciprocal laws where actually present;
- exact non-collapse theorems;
- model instances and countermodels;
- conservative preservation of the predecessor fragment that survives unchanged;
- explicit corrective correspondence for predecessor material that does not survive unchanged;
- proof-relevant transformations;
- machine-checked theorems;
- executable semantics;
- precise notation;
- precise canonical prose;
- exact translation between notation and prose;
- formal question formation;
- answer-dependent question composition;
- explicit proof and evidence boundaries;
- formally controlled approximation;
- formally controlled compression and reopening;
- a machine-checkable revision procedure.

The successor must be one cohesive formal object.

It must not become:

\[
C_0
+
\text{addendum}
+
\text{patch vocabulary}
+
\text{migration layer}
+
\text{implementation conventions}.
\]

Instead:

\[
\boxed{
C_0
\xrightarrow{\text{typed inquiry + independent returns + proof}}
C_1.
}
\]

The successor must contain the mathematics that survived that transformation.

It must not contain the historical story of the transformation as part of its semantics.

---

# 2. End-state

During construction, the active project is:

\[
\boxed{
C_0
+
\text{current repository}
+
\texttt{Questions.txt}
+
\text{this specification}
+
\text{growing formal proof project}.
}
\]

After \(C_1\) is proved and promoted:

- \(C_1\) becomes semantic authority;
- its generated canonical specification becomes the human-readable semantic presentation;
- the proof/model/conformance suite becomes its evidence;
- Rust becomes its conforming reference implementation;
- v2.0 moves to Git ancestry;
- this construction specification moves to Git ancestry.

When the formal question grammar can regenerate the consequential distinctions represented by `Questions.txt`, that corpus also becomes regression/generation material rather than active semantic design material.

The desired eventual active set is:

\[
\boxed{
\text{formal successor}
+
\text{generated canonical presentation}
+
\text{proof/model/conformance suite}
+
\text{Rust implementation}.
}
\]

No chain of near-duplicate semantic documents should remain necessary to understand the project.

---

# 3. Authority while constructing the successor

Use the following temporary authority order.

## 3.1 Explicit current user instruction

Controls current task scope and authorization.

## 3.2 `Inquiry_Calculus_v2_0.tex`

Controls predecessor semantic meaning.

A formal proof may show that a predecessor statement is ambiguous, ill-typed, binding-dependent, underspecified, or false under an admitted model.

Such a result becomes a successor obligation.

It does not permit silently rewriting what the predecessor stated.

## 3.3 This construction specification

Controls how the successor is constructed and accepted.

It does not itself make a theorem true.

## 3.4 Repository actuality

The current:

- source code;
- tests;
- event behavior;
- schemas;
- `CONFORMANCE_STATUS.md`;
- `IMPLEMENTATION_FRONTIER.md`;
- `DECISIONS.jsonl`;
- `FAILURES.jsonl`;
- Git history

establish what the current reference implementation actually does.

Implementation actuality does not redefine semantic meaning.

## 3.5 `Questions.txt`

Supplies external language and inquiry pressure.

Its wording does not create primitives.

---

# 4. Temporary Rust freeze

The current repository contains a substantial v2.0 implementation.

Its architecture already separates approximately:

- `ic-core` — typed semantic structures and finite checkers;
- `ic-runtime` — execution and provider interaction;
- `ic-store` — immutable artifacts, event ancestry, and persistence;
- `ic-cli` — boundary interface.

The live implementation residual must be probed from the repository when work begins. Do not hard-code a stale implementation cursor into the successor.

Until Formal Gate F below, successor work takes precedence over new semantic Rust expansion.

Do not yet:

- change `OpenQuery` semantics;
- change source `IProg` semantics;
- change runtime `Return | Branch | Probe` semantics;
- change event authority;
- change standing/warrant semantics;
- introduce successor-only semantic IR;
- replace controlled rendering with an unproved canonical-language system;
- persist new derived structures merely because formal work names them;
- introduce new general scheduler/controller/compiler/runtime species.

Allowed work:

- fixes preserving current semantics;
- regression fixtures;
- inspection;
- schema export;
- snapshots required by formalization;
- CI/build maintenance.

After Formal Gate F, derive an explicit successor-to-Rust implementation delta.

---

# 5. Fundamental construction discipline

A change may not enter the successor because it is attractive, familiar, elegant, or historically important.

Every consequentially new formal object must arise by filling a typed open position.

For a live residual \(\Delta\) and candidate new form \(z\), the construction process must be able to state an introduction relation:

\[
Intro(\Delta,z).
\]

Then ask:

\[
\boxed{
?z[Intro(\Delta,z)].
}
\]

The resulting completion field determines the status.

### Literal determination

\[
\Fib(?z[Intro(\Delta,z)])
=
\{z\}.
\]

### Protected determination

The fiber may contain several literal forms but only one protected class:

\[
\Fib(?z[Intro(\Delta,z)])
/
{\equiv_{\mathcal H_C}}
=
\{[z]\}.
\]

### Residual plurality

More than one protected class remains.

Another separator question is required.

### Exact impossibility

The fiber is empty under exact declared coverage.

### Unknown

The field cannot currently be completely characterized.

The agent may not choose one protectedly distinct candidate merely because it prefers it.

Thus:

\[
\boxed{
\textbf{EVERY NEW FORM IN THE CONSTRUCTION MUST FILL A PREVIOUSLY TYPED OPEN PORT.}
}
\]

This is a construction discipline.

It is not automatically a primitive theorem of the final calculus.

---

# 6. The successor must not contain its historical derivation

Distinguish:

## 6.1 Mathematical derivation

A theorem proved from \(C_1\)'s own definitions and laws.

This belongs to the calculus.

## 6.2 Historical derivation

A statement such as:

> this law was introduced after some previous model broke an earlier formulation.

This does not belong to canonical semantics.

The canonical successor may state and prove:

\[
P\equiv_{\mathcal H}Q
\not\Rightarrow
P\simeq_{\mathrm{eff}}Q.
\]

It should not need to say which historical experiment first exposed that theorem.

The historical witness may remain in the test suite and Git.

The theorem is what matters.

The acceptance test is:

\[
\boxed{
\text{delete the development narrative and the calculus remains complete.}
}
\]

---

# 7. Regeneration test for the successor

When a candidate \(C_1\) exists:

1. remove the historical explanation of every change;
2. retain \(C_0\);
3. retain the independently checked breakers/returns;
4. retain the lawful questions opened by those returns;
5. rerun the successor-construction inquiry.

Let the regenerated candidate be \(C'_1\).

Require:

\[
\boxed{
C'_1
\equiv_{\mathcal H_C}
C_1.
}
\]

If this fails, then at least one of the following holds:

- \(C_1\) contains an arbitrary choice;
- a generating question was not retained;
- required independent evidence was lost;
- a dependency was not propagated;
- the protected equivalence over calculi is too weak;
- the construction is not regeneratively sufficient.

This test is mandatory before promotion.

---

# 8. Ambient metalanguage

Inquiry Calculus must state the boundary between its own semantic primitives and the formal language used to define them.

Conceptually:

\[
\boxed{
\mathfrak M
\vdash
\mathcal B
\vdash
\text{Inquiry Calculus terms}.
}
\]

Here \(\mathfrak M\) is the ambient dependent type theory/logic supplied by Lean.

Ambient notions may include:

- `Prop`;
- typed equality;
- functions;
- dependent functions;
- dependent pairs;
- inductive families;
- quantification;
- logical conjunction/disjunction/negation;
- predicates or set-like collections.

These are not Inquiry Calculus operators.

The calculus does not need to regenerate its own metalanguage.

The requirement is instead:

\[
\boxed{
\textbf{NO SEMANTIC CONSTRUCTION BELOW THIS BOUNDARY MAY DEPEND ON UNDEFINED ORDINARY-LANGUAGE MEANING.}
}
\]

The canonical successor must explicitly separate:

\[
\boxed{
\begin{array}{c}
\text{ambient metalanguage}\\
\downarrow\\
\text{semantic generators}\\
\downarrow\\
\text{derived relational/query constructions}\\
\downarrow\\
\text{regenerative closure/interface structures}\\
\downarrow\\
\text{implementation interfaces}.
\end{array}
}
\]

This distinction is essential when determining what is genuinely primitive.

---

# 9. Proof-assistant authority

Use Lean 4 as the formal semantic authority for the successor.

Lean should contain:

- formation rules;
- semantic types;
- relation syntax and denotation;
- question syntax and denotation;
- program syntax;
- operational semantics;
- protection declarations;
- support and standing;
- compression/regeneration;
- rewrite judgments;
- domain bindings;
- finite models;
- countermodels;
- predecessor embedding;
- conservativity proofs over the preserved predecessor fragment;
- explicit correspondence for corrected predecessor material;
- canonical notation grammar;
- canonical prose grammar;
- metaprogrammed dependency analysis.

Use a pinned:

- Lean toolchain;
- Lake project;
- Mathlib revision;
- documentation-tool revision.

Promoted formal modules must satisfy:

```text
lake build
```

and independent environment checking using the current pinned equivalent of:

```text
lean4checker --fresh <top-level-successor-module>
```

They must contain:

- no `sorry`;
- no accidental custom axioms;
- no hidden semantic escape hatch.

Binding assumptions are allowed, but they must be explicit formal parameters or fields.

Run an axiom audit over promoted theorems.

---

# 10. Canonical documentation

Use a Lean-native checked documentation system such as Verso.

The final human-readable canonical document is generated from or mechanically linked to the checked formal environment.

It is not a manually synchronized second specification.

The document may contain explanatory prose, but a semantic definition may not exist only in prose.

---

# 11. Recommended formal project structure

A suitable initial structure is:

```text
formal/
  lean-toolchain
  lakefile.toml

  InquiryCalculus/
    Meta/
      Ambient.lean
      Status.lean
      Dependencies.lean
      Impact.lean
      Regeneration.lean
      Conservativity.lean
      AxiomAudit.lean

    Core/
      Types.lean
      Forms.lean
      Relations.lean
      Profiles.lean
      Protection.lean
      RelationUse.lean
      Applicability.lean

    Questions/
      Schema.lean
      OpenQuery.lean
      Evaluation.lean
      Fiber.lean
      Conditions.lean
      Discrimination.lean
      Determination.lean
      Refinement.lean
      PositiveNegation.lean
      Succession.lean
      Frontier.lean

    Programs/
      Source.lean
      FiniteRuntime.lean
      Bind.lean
      Operational.lean
      Trace.lean
      Observational.lean
      Effects.lean
      Guarded.lean

    Evidence/
      Return.lean
      Decode.lean
      Support.lean
      Grounding.lean
      Standing.lean
      Provenance.lean

    Transform/
      Laws.lean
      Rewrite.lean
      Cleanup.lean
      Resources.lean
      Barriers.lean
      BackendProfile.lean
      Lowering.lean
      Soundness.lean

    Compression/
      Exact.lean
      Approximate.lean
      Reduct.lean
      Residue.lean
      Recovery.lean
      Reopening.lean

    Methods/
      Contract.lean
      Composition.lean
      Failure.lean
      Search.lean
      Transport.lean

    Language/
      SemanticFamily.lean
      Notation.lean
      Lexicon.lean
      Opposition.lean
      Grammar.lean
      ProseAST.lean
      Elaborate.lean
      Render.lean
      RoundTrip.lean

    InquiryFamilies/
      Roots.lean
      General.lean
      Coding.lean
      Research.lean
      CrossDomain.lean
      ReciprocalWhy.lean
      Corpus.lean

    Models/
      FiniteCore.lean
      ProgramBreakers.lean
      StandingBreakers.lean
      CompressionBreakers.lean

    Bindings/
      LinearFrame.lean
      Quantum.lean
      Strategic.lean
      Control.lean
      Diagnostics.lean

    Legacy/
      V20/
        Definitions.lean
        Claims.lean
        Correspondence.lean

    Successor/
      Core.lean
      Laws.lean
      Canonical.lean

  Spec/
    InquiryCalculus.lean
```

The names may change.

The layer direction may not.

---

# 12. Formalize v2.0 before correcting it

Do not begin from the desired result.

First construct a machine-readable predecessor surface.

Every consequential predecessor item must be classified as one of:

```text
FormalDefinition
FormalTheorem
LegacyObligation
BindingTheorem
ImplementationOnly
CanonicalProseOnly
```

If a predecessor claim cannot be formalized because:

- a relation is undefined;
- a type is missing;
- an ordinary word carries an untyped condition;
- a theorem depends on a binding;
- a countermodel breaks the universal claim;

do not add an axiom.

Represent a `LegacyObligation` containing:

```text
source coordinate
candidate formal proposition
scope
dependencies
status
known breaker
```

Possible statuses include:

```text
Unproved
Ambiguous
IllTyped
Broken
BindingDependent
ImplementationOnly
```

This allows the machine formalization to discover the actual successor surface rather than having the successor smuggled in before the predecessor is represented.

---

# 13. Primitive basis must be discovered by ablation

The predecessor's regenerative interface contains many useful constructions that are themselves defined from earlier structures.

Do not call all of them foundational primitives.

The formal successor must distinguish:

\[
\boxed{
\text{semantic generators}
\neq
\text{derived operators}
\neq
\text{regenerative interface}
\neq
\text{implementation IR}.
}
\]

A current candidate semantic kernel may include structures analogous to:

\[
\mathfrak K_{\mathcal B}
=
(
\mathsf{Ty}_{\mathcal B},
\llbracket-\rrbracket_{\mathcal B},
\mathsf{Rel}_{\mathcal B},
\mathsf G_{\mathcal B},
\mathsf{ProbeSem}_{\mathcal B},
\mathsf{Check}_{\mathcal B},
\mathsf{Warrant}_{\mathcal B},
\mathsf{Prot}_{\mathcal B},
\mathcal R_{\mathcal B}
).
\]

This is a candidate, not a declaration of minimality.

For every candidate primitive \(p\):

\[
\boxed{
\operatorname{Ablate}_p(\mathfrak K)
}
\]

and ask whether every protected use of \(p\) can be regenerated from what remains.

If yes, remove \(p\) from the primitive basis.

The calculus's primitive list must itself survive the calculus's removal test.

---

# 14. Typed relational substrate

At minimum the semantic core must support:

\[
A,B:\mathsf{Ty}_{\mathcal B}
\]

and:

\[
\boxed{
R:A\rightsquigarrow B.
}
\]

More generally:

\[
R\hookrightarrow\prod_iX_i.
\]

Do not make:

- object;
- state;
- problem;
- question;
- memory;
- cause;
- method;
- controller;
- attention

separate constitutional ontological species merely because implementations or ordinary prose use those nouns.

They are roles unless a breaker proves that a separate semantic species is necessary.

---

# 15. Syntax and denotation

Separate formal expressions from semantic relations.

Conceptually:

\[
\mathsf{Tm}_{\mathcal B}(A,B)
\]

and:

\[
\mathsf{Rel}_{\mathcal B}(A,B).
\]

Provide:

\[
\boxed{
\llbracket-\rrbracket:
\mathsf{Tm}_{\mathcal B}(A,B)
\to
\mathsf{Rel}_{\mathcal B}(A,B).
}
\]

Different syntax may have equal denotation.

Different derivation/provenance paths may also have equal denotation while remaining historically distinguishable.

If binary semantic relations are ordinary relations, prove the ordinary relation laws at that level:

\[
T\circ(S\circ R)
=
(T\circ S)\circ R,
\]

\[
\operatorname{id}_B\circ R
=
R,
\]

\[
R\circ\operatorname{id}_A
=
R.
\]

Do not state that semantic relation composition is associative only when a binding proves associativity if its denotation has already been fixed as ordinary relation composition.

Binding-specific composition operations may require their own laws.

---

# 16. Converse and inverse

For:

\[
R:A\rightsquigarrow B,
\]

define semantic converse:

\[
\boxed{
R^\smile(b,a)
\iff
R(a,b).
}
\]

Prove:

\[
(R^\smile)^\smile=R
\]

and:

\[
(S\circ R)^\smile
=
R^\smile\circ S^\smile.
\]

Keep distinct:

\[
\boxed{
\text{converse}
\neq
\text{inverse image}
\neq
\text{certified inverse}
\neq
\text{reverse section}
\neq
\text{reconstruction}
\neq
\text{recovery}.
}
\]

An exact semantic inverse is available only under an appropriate isomorphism/invertibility theorem.

Executable support for an inverse is a separate capability.

---

# 17. Relational profile

For:

\[
R:X\rightsquigarrow Y,
\]

define:

\[
\boxed{
\operatorname{Prof}_R(x)
:=
\{y:Y\mid R(x,y)\}.
}
\]

Define:

\[
\boxed{
\ker^\star R
:=
\{
(x,x')\in X\times X:
\operatorname{Prof}_R(x)
=
\operatorname{Prof}_R(x')
\}.
}
\]

Define:

\[
\boxed{
\Disc_R(x,x')
\iff
\operatorname{Prof}_R(x)
\neq
\operatorname{Prof}_R(x').
}
\]

Thus:

\[
\Disc_R(x,x')
\iff
(x,x')
\notin
\ker^\star R.
\]

This construction should normalize repeated concepts including:

- answer profiles;
- consequence profiles;
- question discrimination;
- protected discrimination;
- return signatures;
- recovery signatures.

`Prof` and \(\ker^\star\) are likely derived constructions. Prove rather than assume their basis status.

---

# 18. Typed protection specification and protection use

The successor must not use an undefined heterogeneous notion of “future continuations, observations, or consequences.”

Protection has two levels that must not be conflated.

## 18.1 Semantic protection specification

For carrier \(X\), define a typed semantic protection specification conceptually as:

\[
\boxed{
\mathsf{ProtSpec}_{\mathcal B}(X)
=
\sum_{C:\mathsf{Ty}_{\mathcal B}}
\mathsf{ProtectedRelationSpec}_{\mathcal B}(X,C).
}
\]

A semantic protection specification contains:

\[
h
=
(
P_h,
\operatorname{scope}_h,
\operatorname{app}_h,
g_h,
\ldots
)
\]

where:

\[
P_h:X\rightsquigarrow C_h,
\]

and every additional field included here is one whose value changes the semantic meaning or applicability of the protected relation.

An active semantic horizon is:

\[
\boxed{
\mathcal H_X
\subseteq
\mathsf{ProtSpec}_{\mathcal B}(X).
}
\]

Define:

\[
\boxed{
\chi_h(x)
=
\operatorname{Prof}_{P_h}(x)
}
\]

under the declared scope, applicability, and grain of \(h\).

## 18.2 Protection use

Execution- or evidence-relative information belongs in a distinct use/contract layer:

\[
\boxed{
\mathsf{ProtUse}_{\mathcal B}(h)
}
\]

which may include:

- coverage;
- authority;
- evidence;
- binding version;
- provenance;
- resource limits;
- executable method availability.

Do not let incomplete execution coverage alter the semantic equivalence defined by the underlying protection specification.

Thus:

\[
\boxed{
\text{semantic protected equivalence}
\neq
\text{currently certified indistinguishability}.
}
\]

If a protected future is operationally a continuation followed by observation:

\[
K_h:X\rightsquigarrow Y_h,
\qquad
O_h:Y_h\rightsquigarrow C_h,
\]

then represent its protected consequence relation as:

\[
P_h
=
O_h\circ K_h.
\]

If route identity itself matters, reify the route and protect a relation over it.

If provenance matters, reify provenance and protect it.

If reopening ability matters, protect a recovery/reopening signature.

Protection is therefore a typed semantic relation contract plus, where needed, a separate typed use/certification contract.

---

# 19. Protected equivalence and separators

Define:

\[
\boxed{
x\equiv_{\mathcal H}y
\iff
\forall h\in\mathcal H_X,\;
\chi_h(x)=\chi_h(y).
}
\]

Equivalently:

\[
\boxed{
\equiv_{\mathcal H}
=
\bigcap_{h\in\mathcal H_X}
\ker^\star P_h.
}
\]

Define:

\[
\boxed{
\Sep_{\mathcal H}(x,y)
=
\{
h\in\mathcal H_X:
\chi_h(x)\neq\chi_h(y)
\}.
}
\]

Prove:

\[
\boxed{
x\not\equiv_{\mathcal H}y
\iff
\Sep_{\mathcal H}(x,y)\neq\varnothing.
}
\]

This theorem says there is a semantic protected separator.

It does not say that the separator is currently:

- represented;
- formable;
- executable;
- affordable;
- observable;
- supported.

Therefore:

\[
\boxed{
\textbf{NO SEPARATOR FOUND}
\not\Rightarrow
\textbf{NO SEPARATOR EXISTS}.
}
\]

This distinction must remain responsible for:

- semantic versus executable question universes;
- coverage;
- `Unknown`;
- representation gaps;
- probe gaps;
- binding extension;
- reopening.

## 19.1 Protected equivalence at other syntactic categories

Do not silently overload element-level \(\equiv_{\mathcal H}\) onto relations, programs, representations, or compiler terms.

For each category requiring protected comparison, either:

1. reify its members into an explicitly protected carrier; or
2. define a typed lifted equivalence for that category.

For relations of the same type, write provisionally:

\[
\boxed{
R\equiv_{\mathcal H}^{Rel}S
}
\]

only after its typed semantics has been defined.

For programs:

\[
\boxed{
P\equiv_{\mathcal H}^{Prog}Q.
}
\]

The final notation may simplify where types remove ambiguity, but the formal development may not rely on silent cross-category overloading.

---

# 20. Open questions are partially bound typed relations

For a schema:

\[
R\hookrightarrow
X_1\times\cdots\times X_n,
\]

a well-typed partial binding \(\beta\), and a nonempty open coordinate family \(I\), define:

\[
\boxed{
q=?_IR[\beta].
}
\]

Answer carrier:

\[
A(q)
=
\prod_{i\in I}X_i.
\]

Completion fiber:

\[
\boxed{
\Fib_I(R\mid\beta)
=
\{
a_I:
R[\beta\oplus a_I]
\}.
}
\]

Do not add a second semantic object `Poss(q)` when it is definitionally this fiber.

Do not add a second identity `Shape(q)` if normalized question structure already contains the relation, bound ports, and open ports.

Any prose such as “shaped openness” must be treated only as a candidate canonical rendering of this existing structure.

---

# 21. “Answer” must split into exact formal statuses

Do not use bare “answer” in normative prose where its status matters.

Distinguish:

\[
a\in A(q)
\]

carrier candidate;

\[
\Comp_q(a)
\]

valid semantic completion;

\[
\widehat S:\SuppAns(q)
\]

supported answer set;

and separately:

- raw return;
- decoded return;
- semantic interpretation;
- checked claim;
- warranted/standing relation.

Maintain:

\[
\boxed{
\text{candidate}
\neq
\text{completion}
\neq
\text{actual return}
\neq
\text{supported answer}
\neq
\text{standing}.
}
\]

---

# 22. Question discrimination requires a typed evaluation role

The canonical open question:

\[
q=?_IR[\beta]
\]

does not by itself make \(q\) a discriminator of every possible carrier.

To use a question as a discriminator of carrier \(X\), the formalization must supply a typed role that embeds or varies \(X\) in the question's relation schema.

Let:

\[
\delta:\DiscRole(q,X)
\]

denote such a well-typed discrimination/evaluation role.

It may identify, for example:

- a varying port of the underlying relation;
- an embedding of \(X\) into a partially bound context;
- another formally equivalent carrier role.

From \(q\) and \(\delta\), derive:

\[
\boxed{
\Eval_{q,\delta}
:
X\rightsquigarrow A(q).
}
\]

Then define:

\[
\boxed{
\sigma_{q,\delta}(x)
=
\operatorname{Prof}_{\Eval_{q,\delta}}(x).
}
\]

Question-induced equivalence on \(X\) is:

\[
\boxed{
x\sim_{q,\delta}y
\iff
\sigma_{q,\delta}(x)
=
\sigma_{q,\delta}(y).
}
\]

For an applicable family \(C\) of typed question-role pairs:

\[
C
\subseteq
\sum_q\DiscRole(q,X),
\]

define:

\[
\boxed{
x\equiv_Cy
\iff
\forall(q,\delta)\in C,\;
x\sim_{q,\delta}y.
}
\]

Thus:

\[
\boxed{
\equiv_C
=
\bigcap_{(q,\delta)\in C}
\ker^\star\Eval_{q,\delta}.
}
\]

A principal target condition for an interrogative representation is:

\[
\boxed{
\equiv_C
=
\equiv_{\mathcal H}.
}
\]

This means:

> the current typed question-role family distinguishes exactly the distinctions required by the current protected relation family.

It is a condition to establish under a declared carrier, scope, and representation regime.

It is not a universal axiom.

---

# 23. Reciprocal refinement and coarsening

If:

\[
\equiv_C
\not\subseteq
\equiv_{\mathcal H},
\]

then there exist \(x,y\) such that:

\[
x\equiv_Cy
\]

but:

\[
x\not\equiv_{\mathcal H}y.
\]

Those witnesses define the next separator-question search:

\[
\boxed{
?(q,\delta)[
(q,\delta)\in Q^\infty_X(\Sigma)
\land
\sigma_{q,\delta}(x)\neq\sigma_{q,\delta}(y)
].
}
\]

That is refinement.

Conversely, if:

\[
\equiv_C
\subsetneq
\equiv_{\mathcal H},
\]

the active interrogative representation distinguishes something the current protected horizon does not require.

That generates an ablation/coarsening question over smaller \(C'\), subject to preservation of all relevant regenerative and continuation requirements.

These are reciprocal constraint directions.

They are not strict inverse operations.

## 23.1 Question–distinction reciprocity

The formal project must prove the local question equivalence:

\[
\boxed{
x\sim_{q,\delta}y
\iff
\sigma_{q,\delta}(x)
=
\sigma_{q,\delta}(y).
}
\]

It must also preserve the protected separator theorem:

\[
\boxed{
x\not\equiv_{\mathcal H}y
\iff
\Sep_{\mathcal H}(x,y)\neq\varnothing.
}
\]

A further completeness theorem requires explicit representation assumptions.

Define a semantic representability/completeness condition, provisionally:

\[
\boxed{
Complete^{sem}_Q(\mathcal H;X).
}
\]

Then target a theorem of the form:

\[
\boxed{
x\not\equiv_{\mathcal H}y
\land
Complete^{sem}_Q(\mathcal H;X)
\Longrightarrow
\exists(q,\delta)\in Q:
x\not\sim_{q,\delta}y.
}
\]

The completeness premise is load-bearing.

A semantic separator may exist while no currently represented question exposes it.

---

# 24. Condition–solution polarity

Let:

\[
\mathcal C_{\Theta,X}
\]

be a represented family of closed predicates total on their declared applicability carrier:

\[
U_X.
\]

For:

\[
\rho:U_X\to\mathsf{Prop},
\]

define:

\[
\boxed{
\Sol^X(W)
=
\{
x\in U_X:
\forall\rho\in W,\rho(x)
\}.
}
\]

Define:

\[
\boxed{
\Cond^X(S)
=
\{
\rho\in\mathcal C_{\Theta,X}:
\forall x\in S,\rho(x)
\}.
}
\]

Prove:

\[
\boxed{
W\subseteq\Cond^X(S)
\iff
S\subseteq\Sol^X(W).
}
\]

Both are antitone.

Derive the corresponding closure operators when the required order-theoretic assumptions hold.

Do not add `Field(W)` as a synonym for `Sol(W)`.

If `Cond` is derivable from the represented condition family and `Sol`, keep it derived unless an elimination proof shows otherwise.

---

# 25. Semantic truth is not executable certification

Define semantic release:

\[
\boxed{
\Shell_W(\rho)
=
\Sol(W\setminus\{\rho\})
\setminus
\Sol(W).
}
\]

This is a semantic set.

Do not make it three-valued merely because exhaustive execution may be unavailable.

Define a separate executable status such as:

```text
Nonempty(witness)
Empty(certificate)
Unknown(residual)
```

Thus:

\[
\boxed{
\text{failure to find a released candidate}
\neq
\text{certified empty release field}.
}
\]

This distinction should recur throughout the calculus.

---

# 26. Entailment and nonvacuous forcing

Keep ordinary semantic entailment:

\[
W\models_\Theta\phi
\iff
\Sol_W
\subseteq
\llbracket\phi\rrbracket_\Theta.
\]

Define nonvacuous forcing:

\[
\boxed{
W\Vdash_\Theta\phi
\iff
\Sol_W\neq\varnothing
\land
W\models_\Theta\phi.
}
\]

Do not rename this ordinary entailment.

Where complement and coverage are exact, derive:

\[
W\Vdash_\Theta\phi
\iff
\Sol(W\cup\{\neg\phi\})
=
\varnothing
\]

under the required assumptions.

A member of the reopened complement field is a countermodel.

---

# 27. Determination is the central condition/discrimination bridge

A general relational fiber need not be an equivalence class.

Do **not** state:

\[
\text{fiber}
=
\text{kernel class}
\]

universally.

The correct bridge is:

\[
\boxed{
Determines_{\mathcal H}(W,x)
\iff
\Sol^X(W)/{\equiv_{\mathcal H}}
=
\{
[x]_{\mathcal H}
\}.
}
\]

Prove equivalent formulations where defined:

\[
\forall y\in\Sol_W,\;
y\equiv_{\mathcal H}x,
\]

and:

\[
\forall y,z\in\Sol_W,\;
y\equiv_{\mathcal H}z.
\]

This theorem links the two principal refinement directions:

\[
\boxed{
\begin{array}{c}
\text{more conditions}
\\
\Downarrow
\\
\text{smaller admissible field}
\end{array}
}
\qquad
\boxed{
\begin{array}{c}
\text{more discriminators}
\\
\Downarrow
\\
\text{finer equivalence}
\end{array}
}
\]

and determination occurs when the live field occupies one protected equivalence class.

Literal singleton identity is stronger and must remain distinguishable.

---

# 28. Necessity and sufficiency are breaker relations

Do not make necessity or sufficiency primitives.

For:

\[
P\Rightarrow Q,
\]

attack sufficiency with:

\[
\boxed{
?x[
P(x)\land\neg Q(x)
].
}
\]

Attack necessity by opening a successful route in which the alleged necessary condition is absent.

Given:

\[
W\Vdash\phi
\]

and:

\[
\rho\in W,
\]

define local removal sensitivity:

\[
\boxed{
Needed(\rho;W,\phi)
\iff
W\Vdash\phi
\land
(W\setminus\{\rho\})
\not\Vdash\phi.
}
\]

This does **not** constitute a complete global necessity theorem unless the relevant alternative condition space is exhaustively represented.

---

# 29. “Why” is contrast-indexed and derived

Do not introduce a universal primitive `Why`.

A “why this?” question is underbound until its foil or contrast class is fixed or recoverable.

Given supported class:

\[
A^\ast
\]

and foil field:

\[
F,
\]

open a typed separator/factor relation whose completion identifies conditions distinguishing \(A^\ast\) from \(F\).

Different domains may then interpret that relation through:

- causal intervention;
- proof premise;
- diagnostic conflict;
- equilibrium deviation;
- control transition;
- mechanism;
- historical dependency.

The calculus provides the open relational form.

The domain binding supplies the stronger explanatory semantics.

---

# 30. Strong contrast followed by subtraction

Use the following search strategy when locating a consequential boundary:

1. construct a large/extreme admissible contrast that clearly changes the result;
2. vary/remove dimensions of the contrast;
3. preserve the failure/separation;
4. continue until removing one more distinction loses the separation.

This procedure is useful and should be available as a method.

It does not imply a unique mathematical minimum.

It does not make one-at-a-time ablation complete.

---

# 31. Capability-indexed reducts and cores

An unqualified:

\[
Core(W)
\]

is insufficient.

The same web can contain different indispensable relations for different protected capabilities.

Let:

\[
\kappa:\mathcal W\to Y_\kappa
\]

be the protected observation/capability and:

\[
\approx_\kappa
\]

its equality/equivalence.

Define the **set of inclusion-minimal preserving subwebs**:

\[
\boxed{
\Red_\kappa(W)
=
\operatorname{Min}_{\subseteq}
\{
W'\subseteq W:
\kappa(W')
\approx_\kappa
\kappa(W)
\}.
}
\]

Then:

\[
\boxed{
\Core_\kappa(W)
=
\bigcap_{W'\in\Red_\kappa(W)}
W'.
}
\]

Possible \(\kappa\):

- solution field;
- protected quotient;
- departure;
- recovery;
- reconstruction;
- continuation behavior;
- a tuple of protected requirements.

The generic result is a minimal preserving **frontier**, not necessarily one canonical reduct.

A unique optimum requires additional binding structure.

---

# 32. Singleton ablation is not complete

The permanent model suite must include a witness that:

\[
\forall\rho\in W,\;
\kappa(W\setminus\{\rho\})
\approx_\kappa
\kappa(W)
\]

does not imply:

\[
\forall S\subseteq W,\;
\kappa(W\setminus S)
\approx_\kappa
\kappa(W).
\]

Relations can be jointly necessary even when each has a redundant substitute under singleton removal.

Any minimizer claiming completeness must:

- search joint removals; or
- prove special structure under which singleton testing is sufficient.

---

# 33. Positive negation remains ordinary question structure plus stronger provenance

For an admitted use:

\[
N_u\hookrightarrow A\times E_u,
\]

the open negative:

\[
?_eN_u(s,e)
\]

is an ordinary typed open relation.

Do not create a parallel negative-question calculus.

Keep:

\[
\boxed{
\text{open}
\neq
\text{logical negation}
\neq
\text{set complement}
\neq
\text{positive departure}.
}
\]

Maintain:

- answer mismatch \(\not\Rightarrow\) departure;
- boundary projection \(\not\Rightarrow\) exteriority;
- exteriority is relative to a standing determination/use;
- use identity survives return;
- reverse section can be plural;
- reverse section \(\neq\) recovery;
- orientation reversal \(\neq\) actual succession.

Any co-transitivity/apartness theorem remains binding-supplied unless constitutionally proved.

---

# 34. Context application must be typed or eliminated

Do not retain a generic expression:

\[
C[x]
\]

unless the successor defines:

- a context type;
- hole typing;
- a plugging operation;
- its formation laws.

Prefer first to test whether all required “contexts” can be represented as ordinary typed relations or partially bound relation schemas.

Only introduce a separate context species if a breaker demonstrates that ordinary relation syntax cannot regenerate the required behavior.

---

# 35. Scope, grain, applicability, coverage, provenance, and comparison must be typed

Whenever a theorem's truth changes with:

- scope;
- applicability;
- grain;
- coverage;
- provenance;
- comparator;
- binding version;
- authority;
- resource regime;

that dependency must occur in the formal statement or an explicit typed parameter/record.

Do not let prose such as:

> under the relevant scope

carry an unstated theorem hypothesis.

---

# 36. Generic ordinary-language predicates must disappear from normative mathematics

Words such as:

- relevant;
- lawful;
- preserve;
- inspect;
- affect;
- establish;
- same;
- difference;
- consequence;
- protected;

must map to explicit indexed relations.

For example, nonredundant representation relevance may be:

\[
\exists x,y:
\eta(x)=\eta(y)
\land
x\not\equiv_{\mathcal H}y
\land
\sigma_{q,\delta}(x)\neq\sigma_{q,\delta}(y).
\]

“Preserves” must specify whether the statement concerns:

- denotation;
- protected profiles;
- continuation descent;
- completion relations;
- effects;
- provenance;
- recovery;
- regeneration.

There is no universal `Lawful(x)` predicate.

Use the actual judgment:

- well-typed;
- formable;
- applicable;
- executable;
- supported;
- checked;
- warranted;
- standing.

---

# 37. Protected regeneration requires an explicit quantification domain

A statement such as:

> \(m\) retains every protected component required later

is not formal unless “every protected component” has a type.

The deterministic and relational cases must be separated.

## 37.1 Deterministic protected signatures

For deterministic signatures, let:

\[
\boxed{
\mathcal P^{det}_{\mathcal H}(Z)
=
\{
\chi_j:Z\to C_j
\}_{j\in J}.
}
\]

For:

\[
m:Z\to M,
\]

define an exact signature-factorization condition such as:

\[
\boxed{
Regen^{det}_{\mathcal H}(m;Z)
\iff
\forall j\in J,\;
\exists h_j:M\to C_j:
\chi_j=h_j\circ m.
}
\]

Where only the image \(m[Z]\) matters, \(h_j\) may equivalently be typed on the image with the corresponding inclusion made explicit.

## 37.2 Relational protected signatures

For relational requirements, let:

\[
\boxed{
\mathcal P^{rel}_{\mathcal H}(Z)
=
\{
\chi_j:Z\rightsquigarrow C_j
\}_{j\in J}.
}
\]

Then require relational factorization, for example:

\[
\boxed{
\chi_j
=
H_j\circ\operatorname{Graph}(m)
}
\]

for some:

\[
H_j:M\rightsquigarrow C_j,
\]

or an equivalent formally justified relational lifting.

Do not write a function-factorization equation for a relation-valued signature.

The protected requirement family may include exactly what the current protection contract requires:

- behavioral signatures;
- continuation behavior;
- side/role information;
- return/recovery relations;
- provenance;
- residuals;
- discriminator availability;
- reopening routes.

Thus:

\[
\boxed{
\text{“retains everything required”}
}
\]

becomes:

\[
\boxed{
\text{every explicitly protected deterministic or relational requirement factors through or is recoverable from the retained form}.
}
\]

---

# 38. Support requires a typed support relation

A collection of premises does not support a claim merely because it is placed beside it.

Define an explicit relation such as:

\[
\boxed{
\mathsf{Deriv}_{\mathcal B}(E,\lambda,d)
}
\]

where \(d\) is an admitted support/derivation certificate,

or at minimum:

\[
\boxed{
\mathsf{Supports}_{\mathcal B}(E,\lambda).
}
\]

Then minimal support environments may be defined from actual support relations rather than prose saying “claimed to support.”

A closed support route may additionally require:

- standing premises;
- satisfied applicability;
- no open dependency;
- required independent checks;
- valid inconsistency policy.

The support edge itself must be formal before standing depends upon it.

---

# 39. Grounding and standing

Grounded ingress is not the same as an empty dependency list.

Define explicit:

\[
Ground(c)
\]

or an equivalent typed ingress relation.

Then construct a monotone standing operator from:

- grounded ingress;
- closed support routes.

Define:

\[
\boxed{
\Stand
=
\mu T.
}
\]

Prove the least-fixed-point result under its stated lattice/monotonicity assumptions.

Do **not** infer algorithmic termination merely from existence of a least fixed point.

Finite termination or effective convergence requires a separate theorem.

Retain a permanent rootless-support-cycle countermodel.

---

# 40. Source inquiry program

Retain the first-order answer-dependent source structure:

\[
\boxed{
K
::=
\mathsf{Return}_I(a)
\mid
\mathsf{Ask}(q,\kappa).
}
\]

The continuation depends on the supported answer.

Do not replace it with:

- opaque host-language closures;
- prompt text;
- a prose checklist;
- hidden model policy.

The source program must preserve:

- question identity;
- exact occurrence identity where required;
- supported answer;
- environment;
- capture-safe binding;
- discharge authority;
- residual structure.

---

# 41. Question identity and question occurrence are distinct

The same semantic question may appear more than once.

The same semantic answer may occur under different evidence, histories, or continuations.

Therefore retain an occurrence reference such as:

\[
\AskRef.
\]

Successor relation:

\[
\boxed{
\QSucc(
\AskRef,
\widehat S,
q'
).
}
\]

Do not reduce occurrence-specific succession to:

\[
(q,\widehat S).
\]

Pattern learning may compress recurring structures only if the exact occurrence can be recovered before an occurrence-specific continuation is chosen.

---

# 42. Runtime program syntax

The finite runtime core should be formalized as an inductive/free syntax over:

\[
\mathsf{Return},
\quad
\mathsf{Branch},
\quad
\mathsf{Probe}.
\]

For answer type \(A\) and probe signature \(\Sigma\), test an initial-algebra presentation equivalent to:

\[
\boxed{
\Prog^{fin}_\Sigma(A)
\cong
\mu X.
\left(
A
+
\Fin^+(X)
+
\sum_{o\in\Sigma}
\Fin(X)^{\Raw(o)}
\right).
}
\]

The exact Lean encoding may differ.

The required result is a structural recursion/fold principle.

A `Branch` contains alternative program terms.

Do not quotient branch children by weak protected observational equivalence.

---

# 43. Program bind laws

Define bind structurally.

Prove, at the appropriate structural equality:

\[
\mathsf{Return}(a)\bind k
=
k(a),
\]

\[
P\bind\mathsf{Return}
=
P,
\]

and:

\[
(P\bind k)\bind h
=
P\bind
(\lambda x.\,k(x)\bind h).
\]

Do not use protected behavioral equivalence as the foundational equality merely because it is available elsewhere.

---

# 44. Mandatory equality hierarchy

At minimum distinguish:

\[
\boxed{
\begin{array}{rcl}
P=_{\mathrm{str}}Q
&:&
\text{structural equality},\\
P=_{\mathrm{den}}Q
&:&
\text{denotational equality},\\
P\equiv_{\mathcal H}^{Prog}Q
&:&
\text{protected observational equivalence},\\
P\simeq_{\mathrm{eff}}Q
&:&
\text{protected effect/occurrence equivalence},\\
P\simeq^{\mathrm{rw}}_{\Theta,\mathcal H}Q
&:&
\text{licensed rewrite substitutability}.
\end{array}
}
\]

No implication among these is assumed merely because ordinary English calls all of them “same.”

The model suite must contain a witness of:

\[
\boxed{
P\equiv_{\mathcal H_{\mathrm{value}}}^{Prog}Q
\not\Rightarrow
P\simeq_{\mathrm{eff}}Q.
}
\]

---

# 45. Path-sensitive effect semantics

A single global probe-occurrence multiset over a branching program is insufficient because mutually exclusive branches are not jointly realized.

Define an operational judgment such as:

\[
\boxed{
P\Downarrow_\tau a,
}
\]

where \(\tau\) records the required occurrence/effect trace.

Then:

\[
\boxed{
\Beh_{\mathrm{occ}}(P)
=
\{
(\tau,a):
P\Downarrow_\tau a
\}.
}
\]

A probe multiset may be derived from a single trace as a cheap diagnostic.

It is not the full effect semantics.

---

# 46. Observational equivalence comes after semantics and reuses typed protection

Program observational equivalence must use the same typed protection machinery as the rest of the calculus.

Define a typed behavior carrier, for example:

\[
\boxed{
B_\Sigma(A)
}
\]

containing the protected behavior representation for programs of answer type \(A\).

Program semantics may be represented as either:

\[
\Beh:
\Prog_\Sigma(A)\to B_\Sigma(A)
\]

when deterministic at this level,

or:

\[
\Beh:
\Prog_\Sigma(A)
\rightsquigarrow
B_\Sigma(A)
\]

when behavior is relational/nondeterministic.

Let:

\[
\mathcal H_{B_\Sigma(A)}
\subseteq
\mathsf{ProtSpec}_{\mathcal B}(B_\Sigma(A)).
\]

Then define protected program equivalence through those typed protection profiles.

In the deterministic-behavior presentation:

\[
\boxed{
P\equiv_{\mathcal H}^{Prog}Q
\iff
\forall h\in\mathcal H_{B_\Sigma(A)},\;
\chi_h(\Beh(P))
=
\chi_h(\Beh(Q)).
}
\]

Use the corresponding relational profile lifting if \(\Beh\) is relational.

Do not reintroduce an undefined generic \(K(\Beh(P))\) observer after the general protection machinery has been formalized.

A value-only protected observer may lawfully identify programs with different occurrence histories.

That does not establish:

\[
P\simeq_{\mathrm{eff}}Q.
\]

It does not establish:

\[
P\simeq^{\mathrm{rw}}_{\Theta,\mathcal H}Q.
\]

Effect safety is an independent rewrite obligation.

---

# 47. Finite and unbounded interaction must not collapse

The finite runtime syntax is inductive.

Potentially unbounded guarded interaction is a different semantic object and should be represented through a coinductive/interaction-tree-like construction or another proved guarded semantics.

Do not identify:

\[
\mu F
\]

with:

\[
\nu F.
\]

A productive recurrent inquiry must cross the appropriate effective/state-changing boundary.

---

# 48. Actuality lifecycle

Preserve:

\[
\boxed{
\text{generated possibility}
\neq
\text{actual operation}
\neq
\text{raw return}
\neq
\text{decoded completion}
\neq
\text{interpretation}
\neq
\text{checked support}
\neq
\text{warrant}
\neq
\text{standing}.
}
\]

Only a discharge mode whose declared semantics actualizes an operation produces the corresponding new external occurrence.

A decoder may not mutate the raw return.

A later interpretation may not rewrite the event that occurred.

A supported answer may remain plural.

No implementation may select one convenient member from a supported field merely to continue.

---

# 49. Resolution outcome types

The formal resolution layer must preserve semantically different outcomes, including at least the distinctions currently corresponding to:

- supported answer;
- exact empty completion field;
- undefined/inapplicable path;
- unsupported candidate;
- unknown/incomplete coverage.

Exact constructor names may change after lexical/formal refinement.

Their distinction may not.

Exact emptiness requires exact evidence.

`Unknown` is not negative.

---

# 50. Law-aware transformation calculus

Introduce a proof-relevant transformation judgment polymorphic over the actual syntactic category being rewritten.

Use a judgment schema such as:

\[
\boxed{
\Theta;\mathcal H;\Gamma
\vdash
w:
E\simeq_T E'
:
T.
}
\]

Examples include:

\[
T=\mathsf{RelTm}(A,B),
\]

\[
T=\Prog_\Sigma(A),
\]

or another formally declared representation/compiler category.

Every rewriteable category \(T\) must have an associated formal interpretation:

\[
\boxed{
\llbracket-\rrbracket_T.
}
\]

Rewrite soundness states the appropriate semantic preservation relation between:

\[
\llbracket E\rrbracket_T
\]

and:

\[
\llbracket E'\rrbracket_T.
\]

For relational expressions, the specialization may read:

\[
\Theta;\mathcal H;\Gamma
\vdash
w:
E\simeq_{\mathsf{Rel}(A,B)}E'
:
\mathsf{RelTm}(A,B).
\]

Derive named law properties where useful:

\[
Idempotent(R),
\]

\[
Commutes(R,S),
\]

etc., from actual equality/equivalence witnesses rather than storing a universal untyped flag vocabulary.

Compiler rewrite:

\[
\boxed{
\Theta;\mathcal H;\Gamma
\vdash
E
\Longrightarrow_w
E'
:
T
}
\]

must prove every applicable obligation:

- typing;
- denotational/protected consequence preservation;
- effect preservation;
- authority preservation;
- continuation preservation;
- provenance preservation;
- recovery/reopening preservation;
- approximation bounds.

---

# 51. Semantic laws, methods, backends, and compiler capabilities remain distinct

Maintain:

\[
\boxed{
\text{semantic law}
\neq
\text{method capability}
\neq
\text{backend capability}.
}
\]

A backend can implement an operation without making a semantic identity true.

A method can require a semantic law without proving it.

The semantic law must be established independently in the relevant binding.

Do not introduce a second relational semantics merely to hold compiler metadata.

## 51.1 Compiler-visible capabilities are derived evidence

The compiler may derive or reference capabilities such as:

- executable converse;
- certified inverse;
- guardability;
- repeatability;
- commutation;
- cleanup/restoration;
- effect purity;
- target implementability.

These are not automatically constitutional semantic types.

They are consequences of typed relations, laws, methods, authority contracts, and backend profiles.

The compiler should derive a lawful operation family from proofs rather than requiring manually duplicated operator implementations.

## 51.2 Backend profiles and no silent semantic weakening

A backend must expose an explicit typed capability profile:

\[
\boxed{
\Profile(B).
}
\]

The exact structure is implementation-dependent, but it must be able to state relevant capabilities such as:

- generation;
- actual probing;
- checking;
- warrant authority;
- answer-dependent branching;
- persistence;
- recovery;
- streaming;
- concurrency;
- operation-specific executable methods.

Compilation succeeds only when the backend satisfies the source obligations.

In particular:

\[
\boxed{
\mathsf{Probe}
\not\leadsto
\mathsf{Generate}
}
\]

merely because the backend lacks probing capability,

and:

\[
\boxed{
\mathsf{Warrant}
\not\leadsto
\mathsf{Check}
}
\]

merely because the backend lacks warrant authority.

Unsupported capability must yield:

- typed compile failure;
- alternate backend selection;
- an explicit rebinding;
- or another typed residual.

No silent semantic weakening is permitted.

## 51.3 Guard kinds must not collapse

The compiler/formal language must distinguish at least where consequential:

- semantic applicability guard;
- control-flow guard over an already resolved value;
- actual-return-dependent continuation;
- standing/warrant-dependent guard.

These may share implementation mechanisms, but their authority and temporal semantics are not identical.

Do not reduce them all to an untyped Boolean `if`.

## 51.4 Protected barriers

A compiler-side barrier is a derived constraint on transformation freedom, not necessarily a constitutional semantic primitive.

The required law is:

\[
\boxed{
\text{a rewrite may not cross a protected occurrence/order/authority boundary without an explicit witness licensing that crossing}.
}
\]

Potential barrier instances include:

- actual probes;
- sealed predictions;
- external events;
- warrant changes;
- binding/version changes;
- immutable provenance checkpoints.

The final representation of barriers must be derived from typed obligations rather than introduced as an unexplained universal opcode.

## 51.5 Temporary representation and cleanup

The formal compiler must support the general relation behind scoped temporary semantic scaffolding.

Conceptually:

\[
X
\xrightarrow{L}
X_L
\xrightarrow{P}
X_L'
\xrightarrow{C}
X'.
\]

Temporary structure introduced by \(L\) may be removed from active continuation only if the cleanup \(C\) proves that every externally protected requirement has been preserved or regeneratively retained.

Thus:

\[
\boxed{
\text{temporary structure may disappear only after its protected consequence has been extracted and required regeneration remains possible}.
}
\]

This is the general form of semantic cleanup.

It must not import domain-specific terminology as constitutional ontology.

## 51.6 Resource-sensitive use

The formal/compiler layer must determine whether some resources require restricted usage contracts.

Possible implementation regimes may include:

- unrestricted;
- affine;
- linear;
- borrowed/scoped;
- occurrence-indexed.

Do not constitutionalize these names merely because they are useful type-system patterns.

Preserve instead the laws:

\[
\boxed{
\text{copyability of a representation}
\not\Rightarrow
\text{copyability of the authority or occurrence it references},
}
\]

and:

\[
\boxed{
\text{reference multiplicity}
\neq
\text{evidential independence}.
}
\]

A duplicated event reference does not create two independent events.

A reused evidence root does not become multiple independent support roots.

## 51.7 Compilation by typed holes

Unresolved implementation positions should themselves be representable as ordinary typed open relations.

For compiler hole \(h\), ask:

\[
\boxed{
?M[
M\models Req(h)
].
}
\]

The solution field may be:

- one implementation;
- several implementations;
- exact empty;
- unknown/incomplete.

The compiler may optimize over the admissible implementation fiber without converting:

\[
\text{not found}
\]

into:

\[
\text{impossible}.
\]

Compilation is therefore itself an application of the calculus to implementation positions.

## 51.8 Reversible and irreversible compiler passes

Every consequential compiler pass should declare the strongest valid relation between source and target.

Possible classes may include notions analogous to:

- exact isomorphism;
- exact protected fold;
- approximate transformation;
- irreversible lowering.

The exact taxonomy must be derived.

The governing requirement is:

\[
\boxed{
\text{the less reversible a transformation is, the more explicitly its provenance, residual, recovery, and reopening obligations must be represented}.
}
\]

---

# 52. Semantic product, independent execution, and joint actuality

Where product types exist, ordinary semantic product relation may be defined:

\[
(R\times S)
:
(A\times C)
\rightsquigarrow
(B\times D).
\]

This does not itself establish:

- causal independence;
- probabilistic independence;
- concurrency safety;
- joint actualizability.

Those require additional witnesses.

Maintain:

\[
\boxed{
\text{semantic product}
\neq
\text{independent execution}
\neq
\text{joint actuality}.
}
\]

Do not overload notation across these levels unless the types make the distinction formally unambiguous.

---

# 53. Arrangement and succession

Keep distinct:

- representational arrangement;
- analytical reorientation;
- question traversal;
- ledger order;
- actual domain/world succession;
- causal order.

A commutation comparison is well typed only when the compared composites inhabit the same semantic type.

For example, for:

\[
R,S:X\rightsquigarrow X,
\]

both:

\[
S\circ R
\]

and:

\[
R\circ S
\]

have type:

\[
X\rightsquigarrow X.
\]

Then a consequential noncommutation statement may be:

\[
\boxed{
S\circ R
\not\equiv_{\mathcal H}^{Rel}
R\circ S.
}
\]

This establishes that the two arrangements are protectedly distinguishable.

It does not establish that both orders occurred.

Actual succession requires actual occurrences.

An analytical reverse derivation is not automatically a historical explanation.

---

# 54. Prediction and actual return

Prediction is represented forward consequence, not actuality.

Where a prediction is being tested against an actual return, seal:

- prediction;
- assumptions;
- scope;
- acceptance relation

before the discriminating return is obtained.

Do not revise those retrospectively after seeing the result.

A mismatch is a typed relation between a sealed prediction and an actual preserved return.

Failure attacks sufficiency.

Success may still attack necessity.

A prediction seal therefore induces a compiler/order constraint: the prediction may not be transformed so that its semantic content is moved to a position after the discriminating return unless an explicit theorem proves that the temporal epistemic requirement is preserved.

---

# 55. Exact compression

For:

\[
c:X\to Y,
\]

exact protected quotienting requires at minimum:

\[
\boxed{
c(x)=c(y)
\Longrightarrow
x\equiv_{\mathcal H}y.
}
\]

If the quotient becomes reusable executable state, require protected continuation descent.

If provenance, recovery, or reopening are protected, they must also be included in the license.

Do not call a quotient exact because current tests failed to separate its classes.

---

# 56. Local invisibility does not imply global foldability

If one local question, score, or projection fails to distinguish \(x,y\), this does not establish:

\[
x\equiv_{\mathcal H}y.
\]

Every exact fold must quantify over the declared protection family or a formally proved complete surrogate.

A local zero score is not constitutional evidence of global irrelevance.

---

# 57. Regenerative compression and reopening

A retained compressed form must have an explicit license containing the relevant:

- protected requirement family;
- scope;
- applicability;
- continuation family;
- recovery/reacquisition relation;
- provenance;
- residual;
- unlock conditions.

If a new protected relation is later admitted and fails to factor through the quotient, reopen it.

The old compression remains a valid historical claim under the horizon for which it was proved.

Reopening does not rewrite history.

---

# 58. Approximate transformation

There is no universal constitutional theorem saying that two approximately equivalent transformations compose with a predictable error.

If:

\[
f\approx_{\epsilon_f}f',
\qquad
g\approx_{\epsilon_g}g',
\]

a binding must supply an error-composition law such as:

\[
\boxed{
ErrComp_{\mathcal B}
(
\epsilon_f,
\epsilon_g
)
=
\epsilon_{g\circ f}.
}
\]

Approximate compression must state:

- protected family;
- distortion relation;
- direction/order;
- error contract;
- resource benefit;
- residual;
- reopening trigger.

Approximation may not be reported as exact protected equivalence.

---

# 59. Linear/Hilbert question-frame binding and Consequence Subspace theorems

The linear/Hilbert binding must preserve the full theorem chain, not merely a generic claim that a linear binding exists.

Let \(V\) be the admitted real or complex Hilbert/finite-dimensional carrier and let:

\[
\mathcal A_P
\]

be its question/analysis operator.

Define:

\[
\boxed{
M_P
=
\mathcal A_P^\ast\mathcal A_P.
}
\]

Prove:

\[
\boxed{
\ker M_P
=
\ker\mathcal A_P.
}
\]

Define exact question-frame equivalence by equality of the complete admitted question profile.

Then prove:

\[
\boxed{
x\sim_Py
\iff
x-y\in\ker M_P.
}
\]

In the finite-dimensional real linear specialization, prove the Consequence Subspace chain:

\[
\boxed{
V/{\sim_P}
\cong
V/\ker M_P
\cong
\operatorname{im}M_P.
}
\]

Therefore:

\[
\boxed{
\dim(V/{\sim_P})
=
\operatorname{rank}M_P.
}
\]

Prove the compatible-profile fiber theorem:

\[
\boxed{
\mathcal A_P^{-1}(\mathcal A_Px)
=
x+\ker M_P.
}
\]

Define consequence distortion:

\[
\boxed{
D_P(x,\widetilde x)
=
\langle
x-\widetilde x,
M_P(x-\widetilde x)
\rangle
=
\|M_P^{1/2}(x-\widetilde x)\|^2.
}
\]

Prove:

\[
\boxed{
D_P(x,\widetilde x)=0
\iff
x-\widetilde x\in\ker M_P.
}
\]

For every exact **linear** representation:

\[
h:V\to\mathbb R^k
\]

sufficient to recover the complete admitted question profile, prove the appropriate lower bound:

\[
\boxed{
k
\ge
\operatorname{rank}M_P
}
\]

under the theorem's exact finite-dimensional hypotheses.

Do not generalize this dimensional minimality theorem to arbitrary nonlinear encodings without a separate proof.

Consequence-normal coordinates may turn the distortion into ordinary Euclidean geometry on the surviving consequence subspace.

The required theorem hierarchy is:

\[
\boxed{
\text{general protected-profile quotient}
\longrightarrow
\text{Hilbert question-frame theorem}
\longrightarrow
\text{finite-dimensional Consequence Subspace theorem}.
}
\]

A quantum question-frame binding should instantiate the same Hilbert structure where appropriate:

\[
\mathfrak M_{\mathcal Q}
=
\mathfrak A_{\mathcal Q}^\ast
\mathfrak A_{\mathcal Q},
\]

\[
\ker\mathfrak M_{\mathcal Q}
=
\ker\mathfrak A_{\mathcal Q},
\]

while retaining the domain-specific positivity/normalization constraints of the quantum carrier.

These are binding theorems.

They do not promote:

- linearity;
- inner product;
- probability;
- covariance;
- quantum ontology

into the constitutional kernel.

---

# 60. Domain bindings are pressure tests, not ontology

The formal project should include sufficiently alien bindings to attack the general calculus.

At minimum include representative versions of:

- finite relational model;
- linear/Hilbert question frame;
- quantum process/effect frame;
- strategic/game-theoretic response;
- control/reachability/observability;
- diagnosis/conflict reasoning.

The purpose is:

1. express the native relation;
2. remove native object names;
3. determine what relational law survives;
4. test an alien instantiation;
5. bind back to the source;
6. require regeneration of the native capability.

A source-specific law that fails source removal remains in the binding.

---

# 61. Quantum, Fourier, and quantization structure

Keep quantum-native:

- density operators;
- channels;
- effects;
- instruments;
- coherence/interference;
- Hilbert–Schmidt geometry;
- channel adjoints.

Keep Fourier/wave-native:

- spectral decomposition;
- mode structure;
- Fourier transforms;
- boundary-condition quantization.

Keep quantizer/backend-native:

- rotation heuristics;
- codebooks;
- quantizer algorithms;
- specific error implementations.

Only a relation independently expressible and proved over the general calculus may rise.

For example, backward discriminator/predicate transport may have a general relational form, while the quantum Heisenberg adjoint remains a stronger domain realization.

Compiler lessons derived from such domains survive only through their normalized laws:

- law-aware transformations;
- reversible/irreversible distinction;
- effect/order boundaries;
- resource-sensitive use;
- temporary semantic cleanup;
- backend capability contracts;
- proof-carrying optimization.

The source-domain vocabulary itself does not survive into the constitutional calculus unless independently required.

---

# 62. General backward predicate transport

For:

\[
R:X\rightsquigarrow Y
\]

and predicate \(Q\subseteq Y\), define:

\[
\boxed{
\Diamond_RQ(x)
\iff
\exists y.\,
R(x,y)\land Q(y)
}
\]

and:

\[
\boxed{
\Box_RQ(x)
\iff
\forall y.\,
R(x,y)\Rightarrow Q(y).
}
\]

Prove composition laws:

\[
\Diamond_{S\circ R}
=
\Diamond_R\circ\Diamond_S,
\]

\[
\Box_{S\circ R}
=
\Box_R\circ\Box_S.
\]

This provides the general home for forward possibility and backward requirement/precondition transport.

Do not confuse these with inverse, converse, or a quantum adjoint.

---

# 63. Strategic/reflexive environments

Do not add “adversariality” as a universal primitive.

For regime \(r\), explicitly bind:

\[
\boxed{
r
\mapsto
(
\mathcal H_r,
Licensed_r,
\preceq_r
).
}
\]

Represent strategic reaction:

\[
\boxed{
React_r:
X\times A_{\mathrm{inq}}
\rightsquigarrow X'.
}
\]

The inquiry, policy, disclosure, classification, or control action can change the object being reasoned about.

Those responses enter the protected continuation family.

A small representation may dominate under passive use and become unsafe under strategic response because the protection/risk relation changed.

There is no contradiction in using a family of regime-indexed preorders.

---

# 64. Evidence independence

Provenance ancestry may be represented, and a useful derived relation is:

\[
SharedAnc(e_1,e_2)
=
Anc(e_1)\cap Anc(e_2).
\]

But:

\[
SharedAnc(e_1,e_2)=\varnothing
\]

does not automatically establish:

- statistical independence;
- causal independence;
- evidential independence;
- institutional independence.

Whenever a theorem requires independence, require a typed binding-supplied independence witness.

---

# 65. Methods

A method is not a primitive semantic species.

Represent a method as a promoted/reified typed operator path with a contract containing as needed:

- expansion;
- applicability;
- prerequisites;
- protected output;
- success condition;
- failure/residual exits;
- effect/authority requirements;
- proof/check requirements;
- resumption;
- provenance;
- reopening.

Method failure is not reasoning failure.

A failed method produces an ordinary residual.

Another method may become applicable to that residual.

Where possible, method interruption/resumption lowers to the ordinary answer-dependent source program rather than a new runtime effect.

---

# 66. Cross-domain transport

For source \(S\) and target \(T\), use typed transport relations such as:

\[
F:S\rightsquigarrow T,
\qquad
G:T\rightsquigarrow S.
\]

An exact source round-trip compares two same-typed relations:

\[
G\circ F,
\operatorname{id}_S
:
S\rightsquigarrow S.
\]

Therefore use a typed relation-level protected equivalence:

\[
\boxed{
G\circ F
\equiv_{\mathcal H_S}^{Rel}
\operatorname{id}_S.
}
\]

Do not silently apply element-level protected equivalence to relations.

The formalization must either:

- reify same-typed relations into a protected carrier; or
- define the corresponding lifted relation-level equivalence.

A weaker regenerative round-trip relation may be used when literal protected relation equivalence is not required.

Cross-domain inquiry should:

1. identify native objects;
2. expose their relational roles;
3. remove source properties that change no protected result;
4. transport the surviving relation;
5. instantiate alien objects;
6. test the result;
7. translate back;
8. require regeneration of the relevant native method or consequence.

A loose analogy that cannot round-trip is not sufficient.

---

# 67. Established external methods are method homes and breaker sources

The formal/method library may use mature procedures such as:

- model-based diagnosis;
- minimal conflicts and hitting sets;
- counterexample-guided abstraction refinement;
- delta debugging;
- active query learning;
- weakest-precondition reasoning;
- abstract interpretation;
- proof-carrying code;
- translation validation;
- equality saturation;
- partial-order reduction;
- formal concept analysis;
- database provenance;
- rate–distortion methods;
- bidirectional round-trip systems.

Their names do not enter the semantic kernel merely because the procedures are useful.

The agent should extract the relational contract each realizes.

---

# 68. Semantic versus executable question universes

Retain a semantic question universe:

\[
Q^\infty
\]

and an executable/resource-bounded question universe:

\[
Q^\epsilon.
\]

Distinguish at least the relevant notions corresponding to:

- formable;
- applicable;
- executable;
- answerable;
- productive;
- resolved;
- ready;
- required discharge.

A semantically meaningful question may not currently be executable.

A generated question may be executable but not required.

A question may be productive but unauthorized.

Failure to generate or execute a semantic separator does not establish semantic equivalence.

## 68.1 Semantic representational completeness versus executable coverage

Keep distinct:

\[
\boxed{
Complete^{sem}_Q(\mathcal H;X)
}
\]

meaning that the represented semantic question language is sufficient to expose every protected distinction in the declared scope,

from:

\[
\boxed{
Covered^{exec}_{Q,\mathcal E}(\mathcal H;X)
}
\]

meaning that current methods, backends, authority, and resources have actually executed/certified sufficient inquiry under the declared coverage regime.

Neither implication is automatic:

\[
\boxed{
Complete^{sem}_Q
\not\Rightarrow
Covered^{exec}_{Q,\mathcal E}
}
\]

because a semantically expressible question may not be executable,

and:

\[
\boxed{
Covered^{exec}_{Q,\mathcal E}
\not\Rightarrow
Complete^{sem}_Q
}
\]

unless the executed field is itself proved complete for the semantic protection horizon.

This distinction must inform:

- `Unknown`;
- representation gaps;
- probe gaps;
- exact emptiness;
- search coverage;
- reopening.

---

# 69. Current interrogative roots are a hypothesis

The predecessor currently presents the derived root family:

\[
\boxed{
\Omega_Q
=
\{
\mathsf{Expose},
\mathsf{Orient},
\mathsf{Factor},
\mathsf{Polarize},
\mathsf{Vary},
\mathsf{Ground}
\}.
}
\]

Treat this as a hypothesis to test.

Every root must erase to ordinary relation/question/source-program structure.

The formal question-corpus program must determine:

- whether the family is sufficient;
- whether any root is redundant;
- whether the set is minimal;
- whether a root name accurately denotes its formal relation;
- whether some corpus question requires another operator;
- whether the apparently missing operator is merely binding-specific.

Do not preserve these names by inertia.

---

# 70. Five prose question families are derived program families

`Questions.txt` currently organizes inquiry into:

1. general;
2. coding;
3. research;
4. cross-domain;
5. reciprocal why.

Do not add five semantic question constructors.

Treat these as derived, potentially overlapping program/view families over ordinary typed questions.

A question may be both:

- research;
- cross-domain;

or both:

- coding;
- reciprocal why.

Family membership is a derived relation over the question's bindings, role, and protected purpose.

It is not a primitive tag unless a breaker proves the tag itself changes consequential behavior.

---

# 71. Every corpus question must be accounted for

Let:

\[
Q_{NL}
=
\{s_i\}_{i\in I}
\]

be every line/question in `Questions.txt`.

For every \(s_i\), the formal language project must produce exactly one of:

### Canonical elaboration

\[
elab(s_i)=q_i.
\]

### Context-sensitive ambiguity

More than one typed parse survives and the missing context discriminator is stated.

### Representation residual

The relation being asked cannot yet be represented.

### Operation-not-question

The sentence primarily requests an actual operation rather than denoting a semantic open relation.

The corresponding semantic question and operation are separated.

### Redundant surface

The sentence normalizes to a question class already represented.

### Unsupported presupposition

The prose assumes structure not yet established.

No corpus question may be silently skipped.

---

# 72. Canonical prose questions must lower to ordinary calculus notation

The target semantic form remains:

\[
\boxed{
?_IR[\beta].
}
\]

Examples:

> What remains possible?

must elaborate to an exact typed membership/open-relation expression over the current solution/completion field.

> Which condition excludes this alternative?

must expose the condition port of an explicit exclusion/separation relation.

> Which condition is necessary?

must elaborate to the appropriate ablation/breaker relation.

> Why this rather than that?

must bind the returned result and foil and expose a separator/factor relation.

Do not add:

- `WhyQuestion`;
- `NecessityQuestion`;
- `ResearchQuestion`;
- `CodingQuestion`

unless a formal elimination attempt proves ordinary relation/question syntax insufficient.

When a prose question is intended to discriminate a carrier, its elaboration must also recover the required \(\DiscRole\) or other typed evaluation role needed to construct \(\Eval_{q,\delta}\).

---

# 73. Question succession is answer-dependent

Do not construct a fixed family wheel.

The source language already provides the right shape:

\[
\boxed{
K
::=
\mathsf{Return}_I(a)
\mid
\mathsf{Ask}(q,\kappa).
}
\]

A supported answer may make another question:

- formable;
- applicable;
- executable;
- productive;
- required.

Use occurrence-specific:

\[
\boxed{
\QSucc(
\AskRef,
\widehat S,
q'
).
}
\]

Where useful define an answer-conditioned unlock relation:

\[
\boxed{
Unlock^\chi(
\Sigma;
\AskRef,\widehat S
\Rightarrow q'
).
}
\]

Thus:

> this answer makes that question available

must correspond to an actual typed relation.

---

# 74. Corpus compression and regeneration

The natural-language corpus is expected to be highly redundant.

Normalize the typed questions and compare them under an appropriate question horizon.

Expect:

\[
|Q_{NL}|
\gg
|Q_{\mathrm{typed}}/{\equiv_{\mathcal H_Q}}|.
\]

Then reverse the process.

From the derived question grammar/programs generate canonical prose questions.

The target is:

\[
\boxed{
Q_{NL}
\to
Q_{\mathrm{typed}}
\to
\mathcal G_Q
\to
Q'_{NL}.
}
\]

Require \(Q'_{NL}\) to regenerate every consequentially distinct inquiry relation represented by the original corpus.

Once this succeeds, the large corpus is evidence of coverage rather than a second semantic language.

---

# 75. One authoritative typed semantic representation family, multiple syntactic categories

The successor must have **one authoritative typed semantic representation family**, not necessarily one giant tagged AST.

Different syntactic categories may remain genuinely different indexed types, for example:

\[
\mathsf{RelTm}(A,B),
\]

\[
\mathsf{QuestionTm}(\Gamma,I),
\]

\[
\Prog_\Sigma(A),
\]

and other formally justified categories.

They must share one formal semantic authority and explicitly related interpretation functions.

Thus:

\[
\boxed{
\text{many typed syntactic categories}
\neq
\text{many competing semantic authorities}.
}
\]

Construct canonical concrete syntax families for notation and prose.

Provisionally:

\[
\mathsf{NotationSyntax}(T)
\]

and:

\[
\mathsf{CanonicalProseAST}(T)
\]

for supported semantic category \(T\).

Provide category-indexed functions:

\[
render_N^T:
T\to\mathsf{NotationSyntax}(T),
\]

\[
elab_N^T:
\mathsf{NotationSyntax}(T)\to T,
\]

\[
render_P^T:
T\to\mathsf{CanonicalProseAST}(T),
\]

\[
elab_P^T:
\mathsf{CanonicalProseAST}(T)\to T.
\]

For the canonical fragment require:

\[
\boxed{
elab_N^T(render_N^T(e))
=
e
}
\]

and:

\[
\boxed{
elab_P^T(render_P^T(e))
=
e.
}
\]

For accepted canonical prose \(p\):

\[
\boxed{
render_P^T(elab_P^T(p))
=
Canon_P^T(p).
}
\]

If a sentence has two consequentially different typed parses, it is not canonical until the ambiguity is removed.

Free English may remain a heuristic input surface.

Free English is not semantic authority.

---

# 76. Canonical words have typed lexical contracts

A canonical lexical item is part of the formal representation system.

Its contract should encode, conceptually:

\[
Lex(w)
=
(
denotation,
category,
argumentRoles,
applicability,
positiveField,
oppositions,
rendering
).
\]

A word is not exactly defined only by saying what counts as it.

Its meaning also requires:

- where it applies;
- what it positively denotes;
- what it positively excludes;
- what it leaves open;
- its argument direction;
- its neighboring alternatives;
- its reciprocal/opposed relations.

---

# 77. “Antonym” must split into exact opposition relations

Do not use one universal antonym relation.

Distinguish where applicable:

### Contradictory complement

Disjoint and exhaustive on an applicability carrier.

### Contrary

Disjoint but not exhaustive.

### Converse

Role reversal of a relation.

### Inverse

Exact reversible relation under an isomorphism.

### Reciprocal

A paired relation supplied by the calculus.

### Privative/departure opposition

Loss/departure relative to a standing positive determination.

### Scalar pole

Opposition induced by an ordered scale.

If \(N_w\) is claimed to be the complement of \(P_w\) on \(A_w\), prove:

\[
P_w\cap N_w
=
\varnothing,
\]

\[
P_w\cup N_w
=
A_w.
\]

If only contrariety is claimed, prove disjointness and do not assert exhaustiveness.

Always preserve:

\[
U\setminus A_w
\]

as inapplicability.

The lexical law is:

\[
\boxed{
\text{not positive}
\neq
\text{positively opposed}
}
\]

unless a partition theorem proves otherwise.

---

# 78. Synonyms require semantic identity

Two proposed canonical words are true synonyms only if they recover the same:

- typed denotation;
- argument roles;
- direction;
- applicability;
- opposition profile;
- protected use.

If they do, retain one canonical term and treat the other as an alias.

If a consequential difference remains, they are not synonyms in the calculus even if ordinary dictionaries treat them as close.

---

# 79. Grammar words are semantic operators

Canonical uses of words such as:

- if;
- only if;
- exactly when;
- because;
- may;
- must;
- can;
- cannot;
- requires;
- permits;
- excludes;
- supports;
- follows;
- precedes;
- determines;
- preserves

must elaborate to exact formal relations.

For example:

\[
P\to Q
\]

may render:

> If \(P\), then \(Q\).

It may not render:

> \(Q\) because \(P\)

unless an explanatory/causal relation is actually represented.

Grammar is part of the formal surface, not stylistic glue.

---

# 80. Normative vocabulary audit

At minimum audit these words before successor promotion.

| Word | Formal discipline |
|---|---|
| relation | explicit typed relation |
| question | exact open typed relation/question term |
| answer | candidate/completion/supported answer/raw return/etc. specified |
| difference | name literal/profile/question/protected/departure difference |
| distinction | typed schema or exact discrimination relation |
| consequence | typed protected relation profile |
| protected | membership in typed semantic protection specification |
| separator | member of typed separator family or separator question |
| same | exact equality/equivalence named |
| force | singleton, protected singleton, or nonvacuous forcing specified |
| determine | indexed determination relation |
| constrain | solution-field inclusion or exact relation |
| establish | actuality/support/check/warrant/standing specified |
| relevant | explicit nonredundant protected relation |
| preserve | exact factorization/equality/regeneration/effect/provenance relation |
| lawful | actual typing/applicability/executability/support/warrant judgment |
| admitted | exact membership/derivability relation |
| inspect | exact discrimination relation |
| affect | difference in typed protected profiles |
| recover | recovery/reconstruction/regeneration/reacquisition distinguished |
| complement | logical/set/contrary/positive-negation relation distinguished |

Bare normative use of these words is prohibited when the omitted index could change the theorem.

---

# 81. Master recurrence vocabulary is provisional

The predecessor currently presents a derived six-position control normal form under the words:

```text
BIND
OPEN
VARY
RETURN
DETERMINE
REFACTOR
```

Do not preserve these labels merely because they have been used repeatedly.

First formalize the exact transitions.

Then ask of each transition:

- what is its type?
- what relation does it denote?
- what are its source and target?
- what is its positive definition?
- what is its reciprocal/opposed relation?
- what nearest English synonyms compete with it?
- what additional ordinary meaning does each candidate word carry?
- does the word regenerate the notation without project context?
- does the notation regenerate the word?

Specific collisions requiring scrutiny include:

- `RETURN` already denotes program and raw-return concepts;
- `OPEN` overlaps ordinary exposure/open-port language;
- `REFACTOR` carries software-specific implications;
- `BIND` may be narrower than the current control role;
- `DETERMINE` should survive only if the phase really corresponds to the exact formal determination relation.

If no single word is exact, use a phrase.

The formal relation outranks historical vocabulary.

---

# 82. Canonical prose must not outrun notation

The language conformance suite must reject prose that silently derives:

- causality from implication;
- necessity from frequency;
- sufficiency from correlation;
- actuality from possibility;
- support from generation;
- warrant from support;
- standing from warrant-free interpretation;
- inverse from converse;
- recovery from reverse fiber;
- joint actuality from component descriptions;
- historical succession from analytical ordering;
- impossibility from failed search;
- negation from `Unknown`;
- rewrite safety from weak observational equivalence.

These are semantic failures, not stylistic flaws.

---

# 83. Representation and explanation

For representation:

\[
\eta:X\to S
\]

and protected relation/profile family, exact representation adequacy is a factorization/profile condition.

A representation defect is witnessed by:

\[
\eta(x)=\eta(y)
\quad\land\quad
x\not\equiv_{\mathcal H}y.
\]

That pair directly generates a separator/representation question.

Explanation by representation may be formalized as factorization of a protected relation through \(\eta\).

Do not make explanation a primitive semantic species.

Refine where protected behavior splits.

Attempt coarsening where additional detail changes no protected requirement.

---

# 84. Representation extension and expressivity

A representation extension is conservative over an old protected question language only if every old question transports with preserved typing and semantics.

Strict improvement requires a consequentially useful relation/question not available before.

More syntax is not automatically more expressive in the relevant protected sense.

A new coordinate that never affects a protected relation is not automatically worth retaining.

---

# 85. Question selection is relative, not absolute

The deterministic precision order may use profile/kernel refinement.

Probabilistic information gain, entropy, Blackwell order, experiment design, or expected utility require the corresponding binding.

Do not place one universal scalar “question value” in the kernel.

A question frontier is generally a nondominated family under:

- semantic discrimination;
- required discharge;
- resource/risk preorder;
- applicability;
- executable coverage.

---

# 86. Coverage is multidimensional

Whenever a conclusion depends on search or empirical coverage, record the relevant dimensions.

Potential dimensions include:

- domains;
- methods;
- breaker families;
- representations;
- translations;
- scopes;
- grains;
- sources;
- execution routes;
- resources.

“Not found” without a complete declared search field remains `Unknown`.

Coverage records are evidence/implementation structure unless an abstraction proof establishes a more general semantic role.

Use the distinction from §68.1:

\[
Complete^{sem}_Q
\]

versus:

\[
Covered^{exec}_{Q,\mathcal E}.
\]

Do not use one undifferentiated “complete” flag for both.

---

# 87. Methods and failure handling

Represent each admitted method by a typed contract.

A method may specify:

- prerequisites;
- applicability;
- input/output relation;
- protected result;
- success exit;
- residual exits;
- authority;
- effect;
- support/check requirements;
- recovery;
- resumption;
- provenance;
- reopening.

A method failure produces a typed residual.

Another method may be selected because its applicability relation is satisfied by that residual.

Method failure does not equal inquiry failure.

Method composition should lower through ordinary relation/program composition wherever possible.

---

# 88. Domain-neutral method extraction

When using an established source-domain method:

1. identify the source objects;
2. identify the relation among them;
3. distinguish object-specific properties from necessary relational properties;
4. remove source vocabulary;
5. expose the relevant roles as open ports;
6. ask the resulting relation in the source domain;
7. require regeneration of the source behavior;
8. ask it in an alien domain;
9. search for a breaker;
10. retain only the relation surviving both directions.

No analogy is promoted merely because it is suggestive.

---

# 89. Evidence independence and support multiplicity

Multiple support routes may exist.

Do not call them independent merely because they are distinct records.

Where independence matters, define or bind the correct relation.

If multiple independent routes agree, a later compact summary may be retained only if the original independent ancestry remains recoverable whenever future evaluation may distinguish it.

---

# 90. Full dependency propagation

Ordinary Lean recompilation is necessary but insufficient.

A theorem may continue to type-check even after the intended meaning of a dependency changes.

Maintain an explicit semantic dependency graph:

\[
G=(V,E).
\]

Include every canonical:

- type;
- definition;
- theorem;
- relation;
- operator;
- program constructor;
- notation rule;
- lexical rule;
- canonical prose rule;
- conformance fixture;
- Rust correspondence.

Put:

\[
x\to y
\]

when \(y\)'s type, proof, semantics, rendering, or implementation obligation depends on \(x\).

Extract normal declaration dependencies automatically from Lean's environment.

Add explicit metadata edges only where the dependency is not represented in term references.

---

# 91. Impact closure

Given changed declarations:

\[
S_0,
\]

compute:

\[
\boxed{
S^\ast
=
\mu S.
\left(
S_0
\cup
\{
y:
\exists x\in S,\;
x\to y
\}
\right).
}
\]

Every member of \(S^\ast\) is reopened.

It is not sufficient that the old theorem still compiles.

Each affected item receives an external construction disposition such as:

```text
Preserved
Restated
Strengthened
Split
Replaced
Derived
Rebound
Deleted
StillOpen
```

These statuses are construction evidence.

They are not semantic constructors of the successor.

---

# 92. Per-change refinement procedure

Every consequential change must pass the same process.

## 92.1 Expose

Identify the exact predecessor declaration.

## 92.2 Bind the breaker

Use an independently checked:

- proof failure;
- countermodel;
- runtime return;
- corpus ambiguity;
- domain theorem;
- checker result.

## 92.3 Open the question

Make the newly unresolved position explicit.

## 92.4 Represent alternatives

Do not jump directly to the preferred repair.

## 92.5 Construct strong contrast

Use a maximal/extreme case where useful.

## 92.6 Subtract

Remove irrelevant dimensions until the smallest surviving separator is located.

## 92.7 Construct the smallest typed candidate

The repair fills the open position.

## 92.8 Propagate

Compute \(S^\ast\).

## 92.9 Reprove

Recheck all affected declarations.

## 92.10 Challenge

Search for a stronger breaker.

## 92.11 Ablate

Remove the new structure and test whether its protected capability can be regenerated.

## 92.12 Jointly ablate

Do not test only singleton removals.

## 92.13 Rerun the question corpus

Check whether expressive distinctions changed.

## 92.14 Rerun notation/prose round trips

Check representation.

## 92.15 Rerun successor regeneration

Check whether the construction is still forced.

## 92.16 Retain only the survivor

If multiple protectedly different successors remain, inquiry continues.

---

# 93. Successor field

Conceptually let:

\[
C_t
\]

be the current formal candidate,

\[
E_t
\]

the independently checked returns,

and:

\[
Q_t
\]

the lawful questions those returns open.

Define the successor field:

\[
\mathcal S_t
=
\Fib(
\mathsf{Successor}
\mid
C_t,E_t,Q_t
).
\]

Promotion is permitted only if:

\[
\boxed{
\mathcal S_t/{\equiv_{\mathcal H_C}}
=
\{
[C_{t+1}]_{\mathcal H_C}
\}.
}
\]

If more than one consequential class survives, another question is required.

This prevents “design choice” from masquerading as mathematical consequence.

---

# 94. Self-application without self-warrant

The calculus may reify and inquire into its own:

- relations;
- types;
- questions;
- programs;
- proofs;
- compiler laws;
- protection declarations;
- language;
- representation.

It may generate candidate revisions.

It may not use its own generated proposal as the independent return required to warrant its promotion.

Independent constraint may come from:

- kernel proof/checking;
- countermodel;
- actual implementation return;
- external theorem;
- admitted checker;
- user/governance decision where required.

Thus the revision cycle is:

\[
\boxed{
\text{itself}
\to
\text{not-itself}
\to
\text{itself}.
}
\]

The external return constrains the calculus.

The calculus processes the return through its own lawful inquiry relations.

---

# 95. Permanent breaker/model suite

Create a small permanent Lean model suite.

Do not preserve exploratory source files as conceptual dependencies.

Preserve the propositions and witnesses they established.

At minimum include checked models showing:

1. nondegenerate discrimination;
2. protected discrimination can differ from positive departure;
3. positive departure can fail to imply protected separation under a weak horizon;
4. answer mismatch need not imply departure;
5. a reverse section/return fiber can be plural;
6. different protected capabilities yield different reducts;
7. singleton ablation may miss joint necessity;
8. relation composition can fail to commute;
9. a rootless support cycle does not acquire least-fixed-point standing;
10. weak output equivalence can identify programs with different effect histories;
11. the finite fragment containing these distinctions is jointly satisfiable.

Also add successor-specific tests for:

- Branch not quotienting children by weak protected equivalence;
- structural bind laws;
- partial `LiftQ_F` not implying whole-parent coverage;
- regime-indexed economy/preorders;
- predecessor embedding and derivation erasure;
- backend capability mismatch refusing silent weakening;
- protected barrier violations being rejected;
- temporary cleanup being rejected when protected information remains;
- duplicated references not becoming independent evidence;
- relation-level commutation checks being rejected when the composites are not comparably typed.

---

# 96. External search tools produce candidates, not proof

SAT/SMT solvers, fuzzers, external model finders, LLMs, and scripts may search for:

- countermodels;
- candidate proofs;
- finite witnesses;
- lexical ambiguities;
- corpus classifications;
- candidate rewrites.

A result enters the formal evidence set only when:

- reified as typed data; and
- checked by Lean or another explicitly admitted checker.

Failure of an external search to find a breaker is never universal impossibility.

---

# 97. Conservativity and corrective correspondence

A corrective successor cannot be required to be proof-theoretically conservative over predecessor statements that are themselves being corrected.

Partition predecessor material according to formal disposition.

Conceptually:

\[
\boxed{
\mathcal L_0
=
\mathcal L_0^{stable}
\;\uplus\;
\mathcal L_0^{corrected}
\;\uplus\;
\mathcal L_0^{open},
}
\]

or use an equivalent typed status relation.

## 97.1 Semantic embedding of the preserved fragment

For the preserved/stable predecessor fragment, define:

\[
Embed:
\mathcal L_0^{stable}
\hookrightarrow
\mathcal L_1
\]

and prove preservation of predecessor typing and denotation.

## 97.2 Proof-theoretic conservativity over the preserved fragment

For predecessor judgment:

\[
J\in\mathcal L_0^{stable},
\]

require:

\[
\boxed{
\mathcal L_1\vdash Embed(J)
\Longrightarrow
\mathcal L_0\vdash J.
}
\]

This stronger result requires derivation reflection, erasure, or an equivalent theorem.

Do not call denotational preservation alone proof-theoretic conservativity.

## 97.3 Corrective predecessor material

For declarations classified:

- `Split`;
- `Replaced`;
- `Broken`;
- `IllTyped`;
- `BindingDependent`;
- or another corrective status,

do **not** demand conservativity.

Require instead an explicit typed correspondence/disposition theorem stating:

- what predecessor relation was intended or formally recoverable;
- which part survives;
- which part is corrected;
- which stronger assumptions, if any, make the predecessor form valid;
- what successor declaration replaces or splits it.

The governing rule is:

\[
\boxed{
\textbf{CONSERVATIVE OVER WHAT IS PRESERVED; EXPLICITLY CORRECTIVE OVER WHAT IS NOT.}
}
\]

Audit especially:

- program equations;
- support/standing rules;
- new compiler rewrite laws;
- question macros;
- coverage lifts;
- positive-negation derived views;
- new protected relations.

---

# 98. Coverage-indexed dependent lifting

Whenever a supported parent answer is expanded into a family of child questions, distinguish:

\[
F\subseteq|\widehat S|
\]

from:

\[
F=|\widehat S|.
\]

A partial dependent product over \(F\) is valid only for \(F\).

Whole-parent claims require an exact coverage witness.

Do not convert partial family coverage into universal parent coverage.

Uncovered members remain `Unknown`.

---

# 99. Binding-specific conformance pressure

The successor should be instantiated in several mathematically different settings.

The purpose is to find a relation that survives object substitution.

Use at least:

### Finite discrete relation model

Tests core laws and independence.

### Linear/Hilbert frame

Tests:

- exact quotient;
- frame operator;
- kernel equivalence;
- affine completion fibers;
- Consequence Subspace theorem;
- rank/dimension theorem;
- consequence distortion.

### Quantum process model

Tests:

- noninvertible process;
- backward discriminator transport;
- noncommutation;
- joint structure;
- effectful measurement distinctions;
- quantum instantiation of the Hilbert question-frame theorem.

### Strategic/game/economic model

Tests reflexivity, response to inquiry/policy, equilibrium/deviation structure, and regime-relative economy.

### Control model

Tests reachability versus observability versus control and backward precondition reasoning.

### Diagnosis model

Tests plural explanations, minimal conflicts, measurements, support, and alternative discrimination.

No binding is allowed to force its native ontology into the constitutional layer without a separate abstraction theorem.

---

# 100. Representation of the current Rust repository

Before Rust successor work resumes, produce a formal correspondence table for at least:

- relation-expression IR;
- `OpenQuery`;
- completion candidates;
- relation-use structures;
- source `IProg`;
- runtime program representation;
- events and occurrences;
- raw returns;
- decoder/resolution;
- determination;
- positive departure;
- positive-negation uses;
- question succession;
- local interrogative frontier/fixed point;
- protected equivalence;
- support/standing;
- compression/recovery;
- controlled rendering;
- interrogative lowering;
- backend/provider capability declarations where present;
- compiler/lowering structures where present.

Classify each as:

```text
ExactRealization
ConservativeImplementationRefinement
PredecessorOnlyArtifact
SuccessorIncomplete
ImplementationOnly
```

The Rust mapping is downstream evidence.

It does not decide the formal definitions.

---

# 101. Existing controlled rendering

The current implementation deliberately protects semantics by retaining a typed contract independently of the rendered words.

Preserve that behavior while the predecessor remains the implementation target.

The successor target is stronger:

\[
\boxed{
\text{canonical prose grammar itself elaborates to the authoritative typed semantic representation family}.
}
\]

Do not replace the predecessor renderer until the formal language has:

- authoritative typed semantic categories;
- typed lexicon;
- typed grammar;
- renderer;
- elaborator;
- round-trip tests;
- question-corpus coverage.

Then the Rust renderer can be generated from or checked against the formal language.

---

# 102. Full v2.0 propagation audit

The agent must propagate the successor through the complete predecessor surface.

The following checklist is mandatory.

## 102.1 Standing, authority, constitutional laws

Re-evaluate:

- source of authority;
- successor judgment;
- semantic versus implementation status;
- anti-self-warrant laws;
- metalanguage boundary.

## 102.2 Typed relational substrate

Re-evaluate:

- bindings;
- types;
- forms;
- relation syntax;
- relation denotation;
- refinement;
- composition;
- converse;
- reification;
- relation-level equivalence.

## 102.3 Relation schemas, questions, refinement

Re-evaluate:

- named ports;
- partial binding;
- discharge authority;
- question composition;
- typed evaluation roles;
- question discrimination;
- precision;
- fibers;
- holes;
- representation invention;
- semantic question completeness.

## 102.4 Positive negation and reciprocal structure

Re-evaluate:

- distinction schema;
- determination presentation;
- departure;
- relation uses;
- negation uses;
- tagged coverage;
- return fibers;
- recovery;
- reciprocal occurrence;
- orientation;
- active views;
- question-frame defects.

## 102.5 Probe, perception, executable inquiry

Re-evaluate:

- probe semantics;
- observation/action roles;
- perception;
- semantic question versus executable operation;
- raw return;
- decoding;
- partial supported answers;
- resolution paths;
- source programs;
- representation search;
- runtime programs;
- guarded recurrence;
- semantic completeness versus executable coverage.

## 102.6 Compiler and LLM binding

Re-evaluate:

- compiler stages;
- `Generate` possibility;
- prompt/operator distinction;
- render/elaborate;
- behavioral compiler correctness;
- question-conditioned context;
- law-aware transformations;
- backend profiles;
- no-silent-weakening;
- guard distinctions;
- protected barriers;
- temporary semantic cleanup;
- resource-sensitive use;
- typed compiler holes;
- reversible versus irreversible lowering.

An LLM remains a generator/provider capability.

It cannot supply actual occurrence, checking, warrant, or standing merely by generation.

## 102.7 History/state/recurrent probing

Re-evaluate:

- ledger order;
- accepted revision order;
- domain succession;
- event records;
- derived paired views;
- state as implementation seam;
- recurrent probe contracts;
- trace comparability;
- order diagnostics.

## 102.8 Protected behavior/folding/learning/recovery

Re-evaluate:

- protected equivalence;
- protection specification versus protection use;
- quotient;
- operator descent;
- regenerative sufficiency;
- method folding;
- question-pattern learning;
- persistence versus reacquisition;
- historical fact versus reconstruction.

## 102.9 Claims/support/warrant/standing

Re-evaluate:

- claim lifecycle;
- typed support relation;
- open dependencies;
- closed support;
- least fixed point;
- grounded ingress;
- applicability versus support.

## 102.10 Compression/residue/reopening

Re-evaluate:

- exact quotient;
- compression licenses;
- deterministic versus relational protected factorization;
- approximate compression;
- direction-sensitive distortion;
- residual typing;
- unlock;
- reopening;
- pattern completion.

## 102.11 Question frontiers/productivity/search

Re-evaluate:

- semantic question universe;
- executable question universe;
- formability/applicability/executability;
- answerability/productivity;
- required discharge;
- frontiers;
- `Unknown`;
- interrogative roots;
- semantic completeness;
- executable coverage;
- bounded completeness;
- question selection.

## 102.12 Prediction/native methods/control

Re-evaluate:

- prediction;
- prediction seal;
- mismatch;
- necessity/sufficiency breakers;
- description versus control;
- method contracts;
- failure exits;
- resumption.

## 102.13 Learning/patches/self-revision

Re-evaluate:

- reified reasoning presentation;
- patch roles;
- protection changes;
- predecessor-judged successor;
- self-inquiry;
- anti-oracle rules.

## 102.14 Runtime infrastructure

Re-evaluate:

- runtime state;
- persisted records;
- storage;
- content identity;
- checkpoint/replay;
- concurrency.

No implementation record becomes a primitive merely because persistence requires it.

## 102.15 Canonical inquiry process

Re-evaluate:

- exact mathematical decomposition;
- whether six positions remain;
- whether their names remain;
- stopping states;
- no second controller/history.

## 102.16 Derived structures

Re-evaluate:

- problems;
- prerequisites;
- factorization;
- necessity/sufficiency;
- variation;
- abstraction;
- explanation;
- cause;
- memory;
- method;
- attention;
- square/cube visualizations.

Every derived object should lower to the smaller relational basis.

## 102.17 Metatheory

Re-evaluate and strengthen:

- algebraic effects;
- interaction trees;
- containers;
- coalgebra;
- syntax/behavior compatibility;
- abstract interpretation;
- refinement;
- conservative translation;
- typed rewrite-category semantics;
- backend/lowering soundness.

## 102.18 Conformance

Rebuild:

- static conformance;
- dynamic conformance;
- question conformance;
- distinction/regeneration conformance;
- source-program conformance;
- probe conformance;
- compression conformance;
- compiler capability conformance;
- warrant conformance;
- learning conformance;
- self-revision conformance.

## 102.19 Primitive elimination and canonical appendices

Recompute:

- nonprimitive list;
- semantic generators;
- regenerative interface basis;
- compact invariant;
- fixed-point target;
- notation table;
- compiler pipeline;
- infrastructure dependency direction;
- minimum conformance suite.

Nothing in this part is exempt from ablation because it appeared near the end of v2.0.

---

# 103. Formal execution phases

## Phase A — Freeze and inventory

Create `formal-successor`.

Pin Lean/Lake/Mathlib/document tooling.

Inventory:

- every v2.0 mathematical declaration;
- every normative prose claim;
- every current Rust semantic module;
- every relevant conformance fixture.

**Gate A:** Every predecessor item has a formalization destination or explicit classification.

---

## Phase B — Formal predecessor surface

Formalize:

- ambient boundary;
- types;
- forms;
- relations;
- relation-expression syntax;
- basic question formation;
- fibers;
- source programs;
- runtime syntax;
- occurrence;
- initial protection structure;
- support/standing skeleton.

Do not repair gaps by axiom.

**Gate B:** Predecessor core compiles with no `sorry`; unresolved claims are explicit obligations.

---

## Phase C — Exact profile/protection/question core

Formalize:

- `Prof`;
- \(\ker^\star\);
- `ProtSpec`;
- `ProtUse`;
- protected equivalence;
- category-lifted protected equivalence where required;
- separators;
- typed question evaluation roles;
- \(\Eval_{q,\delta}\);
- question profiles;
- question-family equivalence;
- semantic question completeness;
- condition/solution polarity;
- determination theorem.

**Gate C:** Central discrimination, protection, question-evaluation, and determination relations are well typed and kernel checked.

---

## Phase D — Permanent breaker suite

Port the finite/countermodel obligations into Lean.

**Gate D:** Known non-collapse laws have actual checked witnesses.

---

## Phase E — Program and effect semantics

Formalize:

- finite initial program syntax;
- structural bind;
- occurrence traces;
- operational semantics;
- typed behavior carrier;
- protected observational equivalence through the general protection machinery;
- effect equivalence;
- guarded unbounded interaction interface;
- resolution outcomes.

**Gate E:** Structural bind laws and output/effect non-collapse are proved.

---

## Phase F — Predecessor preservation and full propagation

Formalize:

- semantic embedding of the stable predecessor fragment;
- proof-theoretic conservativity over that preserved fragment where applicable;
- explicit correspondence for corrected predecessor material;
- coverage-indexed dependent lifting;
- support/standing preservation;
- compression preservation;
- compiler preservation;
- exact correspondence of changed predecessor claims.

Run the semantic dependency closure until stable.

**Gate F:** No changed predecessor relation is silently presented as preserved, and every claim marked preserved has the corresponding preservation result.

Only after Gate F may successor-driven Rust semantic implementation resume.

---

## Phase G — Law-aware compiler

Formalize:

- category-polymorphic rewrite judgments;
- category-specific semantics;
- law witnesses;
- effect-safe rewriting;
- authority preservation;
- backend capability profiles;
- no-silent-weakening rules;
- guard distinctions;
- protected compiler barriers;
- temporary representation cleanup;
- resource-sensitive usage;
- typed compiler holes;
- reversible/irreversible pass classification;
- approximation contracts;
- method/backend separation.

**Gate G:** Every admitted rewrite/lowering category has a soundness theorem or explicit residual.

---

## Phase H — Compression and regeneration

Formalize:

- deterministic and relational protection requirement families;
- exact quotient;
- continuation descent;
- capability-indexed reduct/core;
- recovery;
- reacquisition;
- residue;
- reopening;
- approximate contracts.

**Gate H:** Every use of “exact”, “minimal”, “core”, and “regenerative” has the required type/index.

---

## Phase I — Question corpus

Ingest all of `Questions.txt`.

Formalize:

- typed semantic question representation;
- typed discrimination roles;
- current root hypothesis;
- five derived families;
- successor/unlock relations;
- semantic completeness versus executable coverage;
- corpus elaboration;
- corpus normalization.

**Gate I:** Every corpus question elaborates or has an explicit typed residual.

---

## Phase J — Canonical notation and prose

Formalize:

- authoritative typed semantic representation family;
- notation parser/renderer;
- canonical prose ASTs;
- lexicon;
- oppositions;
- grammar;
- elaboration;
- rendering;
- category-indexed round-trip laws.

Re-evaluate all overloaded vocabulary, including the master recurrence names.

**Gate J:** Canonical notation and prose are reciprocal surfaces of one authoritative typed semantic representation family.

---

## Phase K — Binding pressure

Formalize representative:

- finite;
- linear;
- quantum;
- strategic;
- control;
- diagnosis

bindings.

The linear binding must include the complete question-frame/Consequence Subspace theorem chain from §59.

**Gate K:** No domain-native relation has leaked into the constitutional core without source-independent proof, and all required binding theorem chains have been checked.

---

## Phase L — Regenerative self-application

Apply the formal revision procedure to selected parts of the calculus itself.

Remove, reopen, question, reconstruct, and compare.

**Gate L:** The successor regenerates without using its historical explanation or its own proposal as warrant.

---

## Phase M — Canonical specification

Generate the human-readable successor from the formal environment.

**Gate M:** No canonical semantic definition exists only in manually maintained prose.

---

## Phase N — Rust successor migration

Generate:

\[
\Delta I_{0\to1}.
\]

Implement only the formally required differences.

Rerun predecessor and successor conformance.

**Gate N:** Rust is a conforming implementation of the formal successor at its declared coverage.

---

## 103.1 Integrated theorem-obligation schedule

The theorem family recovered from the cohesive self-hosting inquiry-spine specification is retained
as construction pressure in `INTEGRATED_THEOREM_OBLIGATIONS.json`. Planning a claim does not admit
it as successor mathematics. Each entry must be activated only after its listed dependencies and
gate-local carrier are available, then proved, broken by a checked countermodel, or classified as
typed inapplicable. A similarly named v2.0 theorem, a harness test, or a generated candidate cannot
discharge it.

| ID | Earliest gate | Planned formal return |
|---|---:|---|
| `IC-THM-C-001` | C | Contextual occurrence typing and composition |
| `IC-THM-C-002` | C | Question transport along a represented path |
| `IC-THM-C-003` | C | Discriminator pullback composition |
| `IC-THM-C-004` | C | Co-anchored meet |
| `IC-THM-C-005` | C | Forward distinction transport |
| `IC-THM-C-006` | C | Condition-solution polarity |
| `IC-THM-D-001` | D | Co-anchored versus sequential noncollapse countermodel |
| `IC-THM-D-002` | D | Three-way arrangement distinction countermodel family |
| `IC-THM-D-003` | D | Reciprocal section is not an inverse countermodel |
| `IC-THM-E-001` | E | Adjacent-order localization |
| `IC-THM-E-002` | E | Path-sensitive answer section and countermodel |
| `IC-THM-E-003` | E | Three-way arrangement operational realization |
| `IC-THM-H-001` | H | Pure closure laws |
| `IC-THM-H-002` | H | Bounded fixed-regime stabilization |
| `IC-THM-H-003` | H | Protected compression factorization |
| `IC-THM-H-004` | H | Historical reopening under a transported discriminator |
| `IC-THM-H-005` | H | Regenerative irredundancy, including joint ablation |
| `IC-THM-J-001` | J | Canonical render-elaborate round trips |
| `IC-THM-K-001` | K | Linear discriminator pullback |
| `IC-THM-K-002` | K | Linear contextual pullback composition |
| `IC-THM-K-003` | K | Protected adjacent-order discriminator and blindness countermodel |
| `IC-THM-L-001` | L | Lawful self-reentry without self-warrant |
| `IC-THM-L-002` | L | Proof-presupposition recursion to the ambient boundary |
| `IC-THM-L-003` | L | Regenerative successor equivalence without historical scaffold |

Every registry entry remains `PLANNED` until it has a typed formal statement, complete dependency
closure, a Lean kernel return or checked countermodel, an independent contract checker, a proof
dependency audit, and a conformance record stating coverage and reopening. `PROVED`, `BROKEN`, and
`INAPPLICABLE` are the only terminal classifications; none alone passes its gate. The registry is a
machine-checked projection of this schedule, not another roadmap, cursor, semantic authority, or
reasoning loop.

---

# 104. One cohesive inquiry spine within every phase

Construction phases are obligation regions, not independent procedures. Every live obligation is
worked through the same model-facing recurrence:

\[
\boxed{
\mathsf{RELATE}
\to\mathsf{OPEN}
\to\mathsf{TURN}
\to\mathsf{RETURN}
\to\mathsf{DISTINGUISH}
\to\mathsf{FOLD}
\to\mathsf{CARRY}
\circlearrowleft .
}
\]

- **RELATE** reconstructs represented forms, typed relations, roles, dependencies, contexts, and
  authority levels.
- **OPEN** exposes one typed unfilled position or unresolved protected distinction.
- **TURN** applies a lawful deformation: reciprocal binding, decisive contrast, removal,
  substitution, joint/context/order variation, path factorization, representation change,
  question-on-question, proof-presupposition, or breaker construction.
- **RETURN** discharges the question through its required `Pure`, `Generate`, `Probe`, `Check`, or
  `Warrant` route without collapsing those authorities.
- **DISTINGUISH** records exactly what the return separates, excludes, supports, leaves plural,
  leaves `Unknown`, invalidates, or makes newly formable.
- **FOLD** removes only distinctions that positively preserve the declared protected behavior,
  regeneration, ancestry, and reopening condition.
- **CARRY** composes every newly live discriminator through relevant typed ancestry, tests crossed
  folds, creates reopening obligations, and reconstructs the changed relational field.

Strong contrast, localization, dependency propagation, reproving, ablation, corpus checks,
round-trips, and regeneration are ordinary `TURN`, `RETURN`, `DISTINGUISH`, `FOLD`, or `CARRY`
programs. They are not another clock or fixed stage list. A phase or gate merely constrains which
relations are open and what evidence may discharge them.

Stop a branch only at a lawful coverage-relative status. Do not manufacture closure by weakening
the horizon or hiding an unresolved breaker.

---

# 105. Stopping

“Lean builds” is not the stopping condition.

Let \(A_n\) be the currently affected dependency closure.

A local refinement pass is closed only when:

\[
\boxed{
A_{n+1}=A_n
}
\]

and the pass creates no new consequential:

- type failure;
- theorem failure;
- countermodel;
- corpus residual;
- lexical ambiguity;
- binding failure;
- implementation-conformance failure.

Any unresolved distinction remains explicit.

Global decidability or absolute minimality is not assumed.

---

# 106. Machine-generated propagation report

After every accepted change, emit an external machine-readable report such as:

```json
{
  "changed": [],
  "dependency_closure": [],
  "preserved": [],
  "reproved": [],
  "strengthened": [],
  "split": [],
  "replaced": [],
  "derived": [],
  "deleted": [],
  "still_open": [],
  "models_rerun": [],
  "question_corpus_rerun": true,
  "notation_roundtrip_rerun": true,
  "prose_roundtrip_rerun": true,
  "regeneration_rerun": true,
  "axiom_audit": "clean"
}
```

This is evidence about construction.

It is never imported as theorem authority.

---

# 107. Formal CI

Minimum formal CI:

```text
lake build
independent Lean environment/kernel recheck
no-sorry audit
custom-axiom audit
finite/countermodel suite
dependency-impact closure check
question-corpus elaboration suite
semantic-question-completeness suite
notation round-trip suite
canonical-prose round-trip suite
generated-spec drift check
preserved-predecessor conservativity suite
corrected-predecessor correspondence suite
backend-profile/no-silent-weakening suite
```

After Rust successor work resumes:

```text
cargo fmt --all --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
formal-to-Rust conformance suite
cold-replay/regeneration suite
```

---

# 108. Required theorem families

At minimum target the following.

## Relational substrate

- relation typing;
- identity;
- semantic associativity;
- converse involution;
- converse/composition;
- relation-expression typing;
- reification closure;
- typed relation-level protected comparison.

## Profiles/protection

- profile typing;
- profile-kernel properties;
- typed semantic protection specification;
- protection specification/use separation;
- typed protected equivalence;
- separator theorem;
- horizon refinement.

## Questions

- open-question formation;
- fiber typing;
- typed discrimination-role formation;
- question evaluation relation:
  \[
  \Eval_{q,\delta}:X\rightsquigarrow A(q);
  \]
- question profile theorem:
  \[
  x\sim_{q,\delta}y
  \iff
  \sigma_{q,\delta}(x)=\sigma_{q,\delta}(y);
  \]
- question-family kernel;
- precision/refinement;
- semantic question completeness;
- question–distinction reciprocity;
- condition–solution polarity;
- determination equivalences;
- semantic/executable non-collapse.

## Positive negation

- ordinary-question subsumption;
- departure soundness;
- mismatch nonimplication;
- plural return fiber;
- return/recovery noncollapse.

## Programs

- finite syntax initiality/recursion;
- structural bind laws;
- operational type safety;
- trace semantics;
- typed behavior carrier;
- observational equivalence through general protection;
- output/effect nonimplication;
- guarded recurrence.

## Evidence

- support relation typing;
- closed support;
- standing monotonicity;
- least-fixed-point characterization;
- rootless-cycle exclusion;
- grounded-ingress distinction.

## Transformations and compiler

- category-polymorphic rewrite typing;
- category-specific interpretation;
- local soundness;
- witness composition;
- effect preservation;
- authority preservation;
- backend capability satisfaction;
- no-silent-semantic-weakening theorem;
- protected barrier preservation;
- temporary cleanup soundness;
- resource-usage conformance;
- typed compiler-hole formation/solution;
- reversible/irreversible lowering obligations;
- approximation only under supplied error laws.

## Compression

- exact quotient;
- continuation descent;
- deterministic protection-signature factorization;
- relational protection-signature factorization;
- capability-indexed reduct/core;
- local/global fold noncollapse;
- reopening.

## Linear/Hilbert binding

At minimum prove:

\[
M_P=\mathcal A_P^\ast\mathcal A_P,
\]

\[
\ker M_P=\ker\mathcal A_P,
\]

\[
x\sim_Py
\iff
x-y\in\ker M_P,
\]

\[
V/{\sim_P}
\cong
V/\ker M_P
\cong
\operatorname{im}M_P,
\]

\[
\dim(V/{\sim_P})
=
\operatorname{rank}M_P,
\]

\[
\mathcal A_P^{-1}(\mathcal A_Px)
=
x+\ker M_P,
\]

\[
D_P(x,\widetilde x)
=
\|M_P^{1/2}(x-\widetilde x)\|^2,
\]

\[
D_P(x,\widetilde x)=0
\iff
x-\widetilde x\in\ker M_P,
\]

and the exact linear representation dimension lower bound under its stated hypotheses.

## Language

- notation round trip;
- prose round trip;
- lexical opposition typing;
- grammatical direction preservation;
- question-corpus coverage.

## Metatheory

- predecessor semantic embedding over the preserved fragment;
- proof-theoretic conservativity over the preserved fragment;
- explicit correspondence of corrected predecessor declarations;
- binding non-promotion;
- primitive ablation;
- successor regeneration.

---

# 109. Questions that must remain genuinely open until proved

Do not answer these by preference:

1. Is the current candidate semantic kernel minimal?
2. Can the operational/probe layer itself be reduced further to relational structure?
3. Are the six current interrogative roots sufficient?
4. Are they minimal?
5. Are all five prose question families derivable without adding another root?
6. Does every important formal relation admit a unique adequate canonical English word?
7. Does the current six-position control decomposition survive exact typing?
8. What names, if any, should those positions have?
9. Is positive incompatibility/co-transitivity constitutional or binding-specific?
10. Which predecessor program equations survive as structural laws?
11. How much predecessor proof-theoretic conservativity can be established over the fragment that genuinely survives unchanged?
12. Which relation-use fields are derivable and removable?
13. What error-composition laws are available in approximate bindings?
14. Which strategic-response structures require explicit continuation extension?
15. What exact independence structures are needed for evidence aggregation?
16. What method-failure ecology can be derived from ordinary source programs?
17. Which coverage dimensions are required for useful impossibility claims?
18. What is the minimal adequate representation of semantic protection specifications and their executable uses?
19. What is the smallest sufficient notion of typed discrimination role for deriving \(\Eval_{q,\delta}\)?
20. Which compiler resource-use distinctions are genuinely required rather than implementation conveniences?
21. Which compiler barriers derive from existing effect/authority structure and therefore require no independent syntax?
22. Can temporary semantic cleanup be fully reduced to ordinary regeneration/reopening laws?
23. Which syntactic categories require distinct lifted protected-equivalence relations rather than ordinary reification?

These are formal research questions.

Do not turn them into specifications until their answer fields contract.

---

# 110. Prohibitions

The executing agent must not:

1. rewrite v2.0 in place before a checked successor exists;
2. use Rust implementation choices as semantic proof;
3. create new semantic names for existing objects merely for readability;
4. add `Poss`, `Field`, or `Shape` as synonyms for existing structures;
5. use an unqualified `Core(W)`;
6. infer joint dispensability from singleton removal;
7. infer effect-safe rewrite from weak observational equivalence;
8. quotient `Branch` children by protected equivalence without an explicit formal construction;
9. infer actuality from a semantic product;
10. infer inverse from converse;
11. infer recovery from reverse fiber;
12. infer causal history from algebraic noncommutation;
13. infer impossibility from failed search;
14. infer negative from `Unknown`;
15. infer standing from generated or interpreted content;
16. infer algorithmic termination from a least-fixed-point theorem;
17. treat an empty dependency list as grounded ingress;
18. import domain-native ontology into the constitutional kernel without an abstraction proof;
19. use `sorry` in promoted successor modules;
20. introduce custom axioms to hide unresolved predecessor claims;
21. make arbitrary English semantic authority;
22. allow canonical prose to say more than the notation;
23. allow notation to encode distinctions canonical prose cannot recover;
24. skip inconvenient questions from `Questions.txt`;
25. preserve historically familiar names when their formal lexical tests fail;
26. let the calculus's generated candidate revision warrant itself;
27. demand proof-theoretic conservativity for predecessor claims explicitly classified as corrected, broken, split, or replaced;
28. conflate semantic protection specification with current executable/evidentiary coverage;
29. assume an open question discriminates an arbitrary carrier without a typed evaluation role;
30. use one giant AST merely to force multiple syntactic categories under one datatype;
31. use deterministic function factorization for relational protected requirements;
32. reintroduce an undefined generic program observer after typed protection has been defined;
33. compare relation composites under protected equivalence when they are not comparably typed;
34. silently apply element-level protected equivalence to relations or programs;
35. force every semantic question through an actuality-producing discharge;
36. silently lower `Probe` to `Generate`;
37. silently lower `Warrant` to `Check`;
38. treat copyability of a reference as duplication of authority or independent evidence;
39. move transformations across protected occurrence/order barriers without a witness;
40. discard temporary semantic scaffolding before its protected contribution has been extracted or regeneratively retained.

---

# 111. First actions for the executing agent

Perform these steps in order.

1. Create the `formal-successor` branch.
2. Pin Lean/Lake/Mathlib/document tooling.
3. Add formal CI.
4. Probe current repository actuality and record the deferred Rust frontier.
5. Inventory every consequential v2.0 definition, theorem, law, and normative prose claim.
6. Inventory current Rust semantic modules and map them to v2.0.
7. Formalize the ambient metalanguage boundary.
8. Formalize typed forms, relations, syntax/denotation, relational profiles, and protection specifications/uses.
9. Formalize open questions and completion fibers.
10. Formalize typed question evaluation roles and \(\Eval_{q,\delta}\).
11. Formalize question and protected kernels.
12. Prove question–distinction reciprocity and the determination bridge.
13. Formalize condition–solution polarity and nonvacuous forcing.
14. Formalize support and grounded ingress.
15. Build the permanent breaker suite.
16. Formalize finite runtime syntax and operational effects.
17. Separate structural, denotational, observational, effect, and rewrite equality.
18. Formalize exact/approximate compression and capability-indexed minimization.
19. Build the dependency-impact engine.
20. Close predecessor preservation/correction to Gate F.
21. Formalize law-aware compiler obligations, backend profiles, barriers, cleanup, resource use, and typed compiler holes.
22. Ingest all of `Questions.txt`.
23. Formalize the question-family/root hypotheses.
24. Distinguish semantic question completeness from executable coverage.
25. Build notation and canonical prose as reciprocal surfaces of the authoritative typed semantic representation family.
26. Audit canonical vocabulary.
27. Formalize the full linear/Hilbert/Consequence Subspace theorem chain.
28. Pressure the core with the remaining alien bindings.
29. Run self-regeneration.
30. Generate the canonical successor specification.
31. Generate the Rust implementation delta.
32. Resume the deferred Rust implementation frontier under the successor semantics.

---

# 112. Promotion criterion

The successor may be promoted only when all of the following are true.

## Type integrity

Every consequential semantic object and operator is typed.

## Metalanguage integrity

Ambient formal primitives and Inquiry Calculus primitives are explicitly separated.

## Primitive integrity

Every primitive has survived ablation.

## Equality integrity

Every consequential use of “same” names the actual equality/equivalence and its syntactic category.

## Protection integrity

Protected equivalence is defined over typed semantic protection specifications.

Semantic protection and executable/evidentiary use are distinct.

No undefined consequence operator remains.

## Question integrity

Every canonical question is an ordinary typed open relation or a derived program of such questions.

Every use of a question as a discriminator of a carrier has an explicit typed evaluation role.

## Question-completeness integrity

Semantic question completeness and executable coverage are represented separately.

Any theorem deriving a represented question separator from protected distinction states the required completeness premise.

## Determination integrity

The relation between admissible fields and protected equivalence is an exact theorem.

## Program integrity

Structural program laws do not depend on weak observational equivalence.

Program observational equivalence reuses the same typed protection machinery as the rest of the calculus.

## Effect integrity

Occurrence-changing rewrites require effect-preservation evidence.

Protected order/effect barriers cannot be crossed without a witness.

## Evidence integrity

Possibility, actuality, return, interpretation, support, warrant, and standing remain separated.

## Support integrity

Support edges and grounded ingress are typed.

## Compiler integrity

Rewrite judgments are typed over their actual syntactic category.

Each rewriteable category has a formal interpretation.

Backend profiles prevent silent semantic weakening.

Temporary cleanup, resource-sensitive use, compiler holes, and irreversible lowering have typed obligations.

## Compression integrity

Every exact/approximate fold states what it preserves and how it reopens.

Deterministic and relational regeneration requirements are separately well typed.

## Minimization integrity

Every core/reduct/minimum is indexed by its preservation criterion and denotes a frontier where uniqueness is not proved.

## Regeneration integrity

Every “retains everything required” claim quantifies over an explicit typed requirement family.

## Binding integrity

Every nonconstitutional law declares its binding assumptions.

The linear/Hilbert binding includes the full Consequence Subspace theorem chain.

## Language integrity

Notation and canonical prose round-trip to one authoritative typed semantic representation family.

“One semantics” is not implemented by forcing all terms into one giant datatype unless that datatype is independently justified.

## Lexical integrity

Canonical terms have explicit denotation, role, applicability, and opposition structure.

## Corpus integrity

Every question in `Questions.txt` is accounted for.

## Propagation integrity

Every accepted semantic change has reached a fixed point over its dependency closure.

## Conservativity integrity

Every predecessor claim marked preserved has an actual preservation theorem.

Proof-theoretic conservativity is required only over the preserved/stable predecessor fragment.

Corrected predecessor claims have explicit correspondence/disposition theorems.

## Self-revision integrity

The successor regenerates from predecessor + independently checked evidence + lawful questions without its historical explanation.

## Implementation integrity

The Rust reference implementation can be mapped to and tested against the formal successor.

---

# 113. Canonical successor document

The generated canonical specification should contain the present calculus only:

- ambient boundary;
- formation judgments;
- types;
- semantic generators;
- authoritative typed semantic representation categories;
- notation;
- canonical prose;
- derived operators;
- laws;
- theorems;
- proof references;
- binding interfaces;
- conformance obligations;
- metatheory.

It should not require:

- development narrative;
- discarded terminology;
- migration rationale;
- conversation history;
- source-domain inspiration;
- an old finite-model source file;
- this construction program.

A reader given only the promoted successor should be able to determine what the calculus means.

---

# 114. Final target shape

The structure currently being tested can be summarized without making this summary itself canonical:

\[
\boxed{
\begin{array}{c}
\text{typed relations}
\\
\downarrow
\\
\text{partial binding and open positions}
\\
\downarrow
\\
\text{completion fibers}
\\
\downarrow
\\
\text{condition-field refinement}
\quad\leftrightarrow\quad
\text{discrimination refinement}
\\
\downarrow
\\
\text{determination}
\\
\downarrow
\\
\text{typed discharge}
\\
\downarrow
\\
\left\{
\begin{array}{l}
\text{Pure}\\
\text{Generate}\\
\text{Probe}\\
\text{Check}\\
\text{Warrant}\\
\text{Mixed / derived admitted combinations}
\end{array}
\right.
\\
\downarrow
\\
\text{resolution}
\\
\downarrow
\\
\text{answer-dependent continuation}.
\end{array}
}
\]

Only discharge modes whose semantics perform an actual external/effectful operation create the corresponding authoritative occurrence.

The broader evidence and transformation structure is:

\[
\boxed{
\begin{array}{c}
\text{actual occurrence where applicable}
\\
\downarrow
\\
\text{decode / interpretation}
\\
\downarrow
\\
\text{support / check / warrant / standing}
\\
\downarrow
\\
\text{law-aware transformation}
\\
\downarrow
\\
\text{compression / recovery / reopening}.
\end{array}
}
\]

Its representation relation is:

\[
\boxed{
\text{notation}
\rightleftarrows
\text{authoritative typed semantic representation family}
\rightleftarrows
\text{canonical prose}.
}
\]

Its inquiry-language relation is:

\[
\boxed{
\text{prose question corpus}
\to
\text{typed open questions + evaluation roles}
\to
\text{derived question programs}
\to
\text{canonical question regeneration}.
}
\]

Its protection/question relation is:

\[
\boxed{
\bigcap_{(q,\delta)\in C}
\ker^\star\Eval_{q,\delta}
\quad
\overset{?}{=}
\quad
\bigcap_{h\in\mathcal H}
\ker^\star P_h.
}
\]

Equality under the declared scope states that the active semantic question representation exposes exactly the protected distinctions.

Its implementation relation is:

\[
\boxed{
\text{formal calculus}
\to
\text{law-aware compilation}
\to
\text{backend-profile-conforming implementation}
\to
\text{Rust runtime / external backends}.
}
\]

Its revision relation is:

\[
\boxed{
\text{predecessor}
+
\text{independent return}
+
\text{lawful question}
\to
\text{successor field}
\to
\text{determined successor}.
}
\]

---

# 115. Final construction rule

The project is ready to continue ordinary implementation only when this statement is operationally true:

> The active Inquiry Calculus is one machine-checked typed formal system with one authoritative semantic representation family and as many genuinely distinct typed syntactic categories as its mathematics requires. Its canonical notation and canonical prose are reciprocal representations of that same semantic authority. Its canonical questions elaborate to lawful open relations; when a question is used to discriminate a carrier, the corresponding evaluation role and relation are explicit. Its semantic question completeness is distinguished from current executable coverage. Its protected future is represented by typed semantic protection specifications whose execution/evidence uses remain separately typed. Its distinctions and equivalences are defined by typed relation profiles rather than undefined ordinary-language terms. Its runtime program syntax has a structural theory independent of weak observational quotienting, while program observational equivalence reuses the same typed protection machinery as the rest of the calculus. Actuality and evidence authority remain explicit. Its transformations are category-typed, proof-carrying, effect-aware, backend-aware, and incapable of silently weakening authority. Temporary semantic structure may be removed only when its protected contribution has been extracted and regeneration remains possible. Its compression and minimization state exactly what is preserved. Its deterministic and relational regeneration laws are separately well typed. Its linear/Hilbert binding proves the complete Consequence Subspace theorem chain rather than merely referring to a geometric analogy. Its language can explain the question corpus without creating a second interrogative ontology. Every semantic change propagates through the formal dependency graph and survives countermodel search, ablation, and regeneration. The successor is conservative over the predecessor fragment it actually preserves and explicitly corrective over the remainder. The successor can be regenerated from the predecessor and independently checked returns without consulting the historical exploration that produced this construction specification. The Rust implementation implements this formal object rather than defining it.

Formalization, breaking, refinement, propagation, reproving, ablation, regeneration, and
implementation are derived operations within the one inquiry spine above. They must not be
presented or encoded as another model-facing sequence. None becomes successor semantics unless the
formal successor independently regenerates and warrants it.

---

# 116. Executable realization of the one inquiry spine

These are acceptance obligations on the branch-local harness. They do not add successor
primitives, define semantic state, warrant a theorem, or install a semantic scheduler.

The sole public harness entry point reconstructs repository actuality, a derived relational
surface, typed compositional paths, ordinary question occurrences, transported discriminators,
fold/reopening obligations, a path-indexed model context, and the executable frontier. The compact
machine contract is a checked projection of this specification and `Questions.txt`; it is not a
second authority and contains no fixed rhythm, Q-family execution order, residual-to-question
schedule, or residual-class-to-method dispatch.

The accountability protocol underneath `RETURN` is not a second reasoning recurrence. A pure
return records `Field/Ask/Answer/Reify/Field`; an effectful return additionally records
`Seal/Operation/Raw/Interpret/Check`. The model is shown only the recurrence in section 104.

For represented transformations

\[
X_i\xrightarrow{U_i}X_{i+1},
\qquad
P_{i:j}=U_{j-1}\circ\cdots\circ U_i,
\]

and a compatible later discriminator (C_{q_t}:X_t\rightsquigarrow A(q_t)), the harness derives

\[
\boxed{D_{i,t}=C_{q_t}\circ P_{i:t}.}
\]

This is `CARRY`: ordinary typed composition, not an inverse, a generic backward relation, or a new
semantic `TransportQuestion` constructor. Path order is represented ancestry and is not causal or
actual history without separate actuality evidence. Question form, rendering, occurrence, context,
path, and Answer remain distinct identities.

For a fold (c:X\to\widehat X) and protected discriminator family \(\mathcal H\), positive fold
evidence requires

\[
\forall D\in\mathcal H,\;\exists\widehat D:\;D=\widehat D\circ c
\]

together with exact ancestry, regeneration, coverage, and a positive reopening condition. If a
later carried discriminator (D_{\mathrm{new}}) cannot be shown to factor through (c), continued
fold validity is `Unknown`; a supported separation reopens the fold. Similar wording and selected
endpoint equality never license compression.

The harness must preserve append-only evidence, collision-safe append, immutable Raw, no fabricated
actuality, `Unknown != Negative`, `Generated != Standing`, Ask-before-Answer, answer-dependent
continuation, represented-path-versus-actual-history, no silent live-question disappearance,
positive fold/reopening evidence, seal-before-effect, derived-cache nonauthority,
checkpoint-versus-closure, and self-application-versus-self-warrant.

Every live question leaves the field only as Answered, evidenced Folded, typed Inapplicable,
reasoned Blocked, explicitly ResourceBounded, or superseded by a typed refinement that preserves
ancestry. Selection uses only represented required-discharge, dependency, Frontier, effect, risk,
coverage, resource, cost, or supplied preference relations. A deterministic identity tie-break may
allocate execution among incomparable live occurrences but makes no semantic optimality claim and
deletes none of them.

Methods are relational contracts (`Applicable`, `Requires`, `Handles`, `Produces`, `FailsAs`,
`ResumesAfter`, `Checks`). Repeated paths are only method-fold candidates. There is no active
residual-shape method dispatcher.

Task closure is coverage-relative and requires no unresolved Ask, unreified Answer, dirty field,
open effectful return, newly formable Required/Productive executable question, unchecked relevant
path transport, or fold awaiting a carried discriminator check. A checkpoint never requests the
user merely to choose the next ordinary question.

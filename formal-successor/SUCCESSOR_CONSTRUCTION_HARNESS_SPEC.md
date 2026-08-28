# Inquiry Calculus Successor Construction Harness Specification

**Suggested filename:** `SUCCESSOR_CONSTRUCTION_HARNESS_SPEC.md`
**Repository:** `AcidicSwords/Inquiry_Calculus_Compiler`
**Status:** Provisional construction-harness specification
**Semantic status:** Derived, non-canonical, non-semantic
**Purpose:** Use the Inquiry Calculus while machine-checking, reformalizing, redefining, reconceptualizing, and constructing the formal canonical successor to Inquiry Calculus v2.0.

---

# 0. Binding

Let:

$$
C_0
:=
\text{Inquiry Calculus v2.0}.
$$

Let:

$$
C_t
$$

be the current candidate successor formalization.

Let:

$$
\Sigma_t
$$

be the current construction inquiry state.

Let:

$$
\mathcal R_t
$$

be the complete currently represented residual field.

Let:

$$
r_t^\ast\in\mathcal R_t
$$

be the currently selected executable frontier.

Let:

$$
\mathcal Q
$$

be the overcomplete natural-language question corpus, especially the Software/Coding and Reciprocal Why sections of `Questions.txt`.

Let:

$$
\mathcal M
$$

be the available methods of discharge: pure derivation, generation, repository inspection, Lean, finite-model search, SAT/SMT, compilation, execution, testing, external research where authorized, and other applicable tools.

This harness governs the inquiry process by which the successor is constructed.

It does **not** determine what the successor's primitives, canonical recurrence, notation, or semantics must be.

The successor may prove that any harness distinction is:

* primitive;
* derived;
* redundant;
* improperly split;
* improperly merged;
* binding-specific;
* implementation-specific;
* or unnecessary.

The harness may discover the successor.

The harness may not use its own structure as evidence that the successor must have that structure.

---

# 1. Governing asymmetry

The central search rule is:

$$
\boxed{
\textbf{EXPLORE WIDELY; COMMIT NARROWLY.}
}
$$

More exactly:

$$
\boxed{
\textbf{EXPLORE BEYOND WHAT CAN CURRENTLY BE PROVED;
RATCHET ONLY WHAT CAN CURRENTLY BE PROVED.}
}
$$

The breadth of search and the strength of retained commitment are different quantities.

## 1.1 Exploration scale

Exploration should deliberately seek:

* large admissible contrasts;
* extreme cases;
* countermodels;
* reversals;
* removals;
* substitutions;
* alternate paths;
* alternate orders;
* alien representations;
* degenerate cases;
* joint variations;
* incompatible condition combinations;
* possible blockers;
* representation failures;
* failed folds;
* contrary explanations;
* different scopes and grains.

The purpose is to make structure visible.

## 1.2 Ratchet scale

A retained construction step shall be no stronger than its independent return warrants.

A wide search may suggest:

$$
P\land Q\land R\Rightarrow S.
$$

If the current check establishes only:

$$
P\Rightarrow T,
$$

then only the latter may be ratcheted.

Everything else remains candidate structure or residual.

Thus:

$$
\boxed{
\text{large exploratory deformation}
\longrightarrow
\text{small warranted update}.
}
$$

## 1.3 Minimality

Minimality is principally a **terminal retention criterion**, not a default search strategy.

Do not begin by asking:

> What is the smallest conceivable change that might work?

When possible, ask:

> What sufficiently large admissible change makes the relevant difference unmistakable?

Then subtract.

The default pattern is:

$$
\boxed{
\text{STRONG CONTRAST}
\to
\text{OBSERVED SEPARATION}
\to
\text{SUBTRACTION}
\to
\text{BOUNDARY}.
}
$$

Likewise for repair:

$$
\boxed{
\text{DECISIVE FAILURE}
\to
\text{SUFFICIENT REPAIR}
\to
\text{SUBTRACTION}
\to
\text{MINIMAL PRESERVING FRONTIER}.
}
$$

---

# 2. No premature global closure

The harness shall never infer:

$$
\text{local closure}
\Rightarrow
\text{global exhaustion}.
$$

Maintain:

$$
\boxed{
\begin{aligned}
\text{locally determined}
&\neq
\text{globally exhausted},
\\
\text{no current breaker found}
&\neq
\text{no breaker exists},
\\
\text{one theorem proved}
&\neq
\text{formalization complete},
\\
\text{one branch satisfied}
&\neq
\text{inquiry exhausted},
\\
\text{current equivalence}
&\neq
\text{future protected equivalence}.
\end{aligned}
}
$$

A conclusion is closed only relative to declared:

$$
(\Theta,\mathcal H,\Gamma,\kappa)
$$

where these represent the applicable binding/frame, protected horizon, coverage, and required capability.

Every closure must retain its reopening condition.

---

# 3. The harness constructs a relational index, not a pairwise map

Suppose the successor contains a family:

$$
X=\{x_1,\ldots,x_n\}
$$

of forms, definitions, relations, theorems, methods, constructions, implementation structures, or prose commitments.

The harness shall not attempt to establish every pairwise relation:

$$
x_i\leftrightarrow x_j.
$$

That strategy may require:

$$
O(n^2)
$$

explicit comparisons while repeatedly rediscovering the same conditions.

Instead, the harness shall construct reusable relational coordinates from:

* conditions;
* breakers;
* nonbreakers;
* contradictions;
* exclusions;
* failed folds;
* missing prerequisites;
* representation gaps;
* noncommutations;
* path failures;
* reconstruction failures;
* coverage boundaries;
* applicability boundaries;
* actual returns;
* residual questions.

Let:

$$
D_t=\{d_1,\ldots,d_k\}
$$

be the current discriminator/boundary family.

Each construction acquires a profile:

$$
\boxed{
\Sigma_{D_t}(x)
=
\left(
\operatorname{Prof}_{d_1}(x),
\ldots,
\operatorname{Prof}_{d_k}(x)
\right).
}
$$

Relations among constructions may then be discovered through shared or differing profiles rather than by fresh pairwise comparison.

The harness therefore seeks a factorized structure:

$$
\boxed{
\text{CONSTRUCTIONS}
\longleftrightarrow
\text{CONDITIONS / BREAKERS / QUESTIONS / RESIDUALS}.
}
$$

Shared conditions become relational junctions.

Shared breaker boundaries become relational junctions.

Shared failed folds become relational junctions.

Shared holes become relational junctions.

---

# 4. Shaped absence and typed holes

Removal shall not be treated as mere deletion.

Given a represented form \(z\), remove its filling while retaining its relational incidence.

The result is a typed residual:

$$
\Delta_z.
$$

Then ask:

$$
\boxed{
?z[
Intro(\Delta_z,z)
].
}
$$

The lawful completion field of the hole reveals what the remaining structure requires.

Two forms may therefore be related through the holes produced by their removal:

$$
z_i
\to
\Delta_i
\leftarrow
z_j.
$$

Questions include:

> What does this form allow the current structure to determine?

> What determines this form when the form itself is removed?

> Which other forms produce the same or overlapping hole?

> Which condition occurs in every admissible reconstruction?

> Which substitute can occupy this role?

> Which protected distinction makes this filling unique?

A hole is therefore **shaped absence**: an unresolved relational position whose surrounding conditions constrain its lawful fillings.

---

# 5. Core dynamic

The harness shall treat successor construction as interaction between at least two distinct refinement directions.

## 5.1 Admissibility refinement

For condition web:

$$
W\subseteq\mathcal C_\Theta,
$$

let:

$$
\Field_\Theta(W)
=
\{x:\forall\rho\in W,\rho(x)\}.
$$

Adding a condition contracts the field:

$$
W\mapsto W\cup\{\rho\},
$$

$$
\Field(W\cup\{\rho\})
\subseteq
\Field(W).
$$

Removing one releases it:

$$
W\mapsto W\setminus\{\rho\}.
$$

This gives the reciprocal motion:

$$
\boxed{
\text{CONSTRAIN}
\rightleftarrows
\text{RELEASE}.
}
$$

## 5.2 Discrimination refinement

For applicable discriminators/questions \(C\):

$$
K_C
=
\bigcap_{q\in C}\sim_q.
$$

Adding a useful discriminator refines the kernel:

$$
K_{C\cup\{q\}}
=
K_C\cap\sim_q.
$$

Removing a redundant discriminator coarsens it.

This gives:

$$
\boxed{
\text{DISTINGUISH}
\rightleftarrows
\text{COARSEN}.
}
$$

## 5.3 Determination

The two meet at protected determination:

$$
\boxed{
Determines_{\mathcal H}(W,x)
\iff
\Field(W)/{\equiv_{\mathcal H}}
=
\{[x]_{\mathcal H}\}.
}
$$

A literal singleton is stronger than necessary.

A field may contain many literal fillings while representing one protectedly indistinguishable answer class.

---

# 6. Construction recurrence

The harness shall use the following derived construction recurrence:

$$
\boxed{
\textbf{
EXPAND
\to
COLLIDE
\to
LOCALIZE
\to
DISCHARGE
\to
RATCHET
\to
PROPAGATE
\to
FACTOR
\to
REEXPAND.
}}
$$

These are harness operations, not proposed semantic primitives.

## 6.1 EXPAND

Generate a deliberately broad but typed field of:

* candidate formalizations;
* breaker classes;
* alternate representations;
* condition changes;
* countermodels;
* path changes;
* reversals;
* substitutions;
* joint variations;
* domain translations.

The expansion should be broad enough to expose differences clearly.

## 6.2 COLLIDE

Seek places where expanded alternatives:

* disagree;
* fail;
* contradict;
* cease to apply;
* become noncommutative;
* cease to reconstruct;
* fail a fold;
* produce distinct protected returns;
* require incompatible conditions.

A collision is useful.

It localizes a discriminating boundary.

## 6.3 LOCALIZE

Ask which smallest currently supported relation actually carries the collision.

Do not start local.

Localize after a broad separating field has been established.

## 6.4 DISCHARGE

Use the appropriate independent method to answer the typed question.

Preserve the separation:

$$
\text{Generate}
\neq
\text{Probe}
\neq
\text{Check}
\neq
\text{Warrant}.
$$

## 6.5 RATCHET

Retain only what the return warrants.

Every ratchet must state:

* exact relation established;
* binding;
* applicability;
* coverage;
* evidence;
* protected consequence;
* failure condition;
* reopening condition.

## 6.6 PROPAGATE

Search existing residuals for occurrences of the newly established condition/relation.

Ask:

> Which earlier unresolved questions depended on this?

> Which previous `Unknown` may now be resolvable?

> Which previous fold was conditional on this?

> Which previous breaker now has a known classification?

> Which old contradiction changes?

One result may therefore update many residual basins.

## 6.7 FACTOR

Detect recurrence.

Ask whether repeated appearances of the same relational pattern can be:

* factored;
* abstracted;
* represented once;
* promoted into a transparent method;
* or compressed into a reusable discriminator.

Frequency proposes a candidate factorization.

It does not warrant it.

## 6.8 REEXPAND

Expand again from the changed map.

Do not restart the inquiry from zero.

---

# 7. Residual field

A residual shall be more than a prose “next task.”

Conceptually:

$$
\boxed{
\Delta=
\langle
O,
W,
F,
C,
B^+,
B^-,
K,
X,
G,
A,
E,
P,
U
\rangle
}
$$

where, as applicable:

* \(O\): open typed relation/obligation;
* \(W\): current condition web;
* \(F\): lawful surviving completion field or approximation;
* \(C\): currently applicable discriminator/question family;
* \(B^+\): known breakers;
* \(B^-\): known nonbreakers or survived contrasts;
* \(K\): blockers/missing capabilities;
* \(X\): contradictions/incompatibilities;
* \(G\): representation or expressibility gaps;
* \(A\): applicability/scope information;
* \(E\): evidence and coverage;
* \(P\): provenance/ancestry;
* \(U\): reopening conditions.

Not every residual must materialize every field.

Unknown components remain `Unknown`.

## 7.1 Residual classes

At minimum preserve the current operational distinctions:

* none;
* persists;
* regression;
* wrong locus;
* missing dependency;
* weak discriminator;
* environment failure;
* unknown;
* resource bounded.

The successor harness may derive more informative subtypes, but these shall not silently become semantic primitives.

## 7.2 Residual ancestry

Every residual shall identify what opened it.

A residual may be opened by:

* plural protected completions;
* breaker witness;
* contradiction;
* failed fold;
* failed proof;
* uncovered scope;
* missing representation;
* unavailable probe;
* failed reconstruction;
* unsupported return;
* new applicability regime;
* predecessor/successor mismatch.

## 7.3 Residual overlap

Two residuals shall be treated as potentially related when they share:

* conditions;
* breaker families;
* failed folds;
* discriminators;
* open typed ports;
* dependency paths;
* applicability boundaries;
* representation gaps;
* support ancestry.

The relation must be tested semantically; lexical overlap is insufficient.

---

# 8. Basin formation

Repeated residual overlap forms a candidate **relational basin**.

A basin is not a semantic object by default.

It is a derived inquiry structure indicating that several otherwise distinct inquiries repeatedly encounter the same discriminating relations.

For residuals:

$$
\Delta_1,\ldots,\Delta_m,
$$

a basin candidate may be proposed when they repeatedly share:

$$
\rho,
$$

or a family:

$$
\{\rho_1,\ldots,\rho_k\}.
$$

Then ask:

> Is this genuinely the same typed relation under the relevant bindings?

> Which case instantiates one residual without the others?

> Which breaker separates the alleged common structure?

> Which weaker relation survives all instances?

> Which stronger relation was accidental to one case?

A surviving basin may become:

* an abstraction;
* a method;
* a reusable question program;
* a representation coordinate;
* a theorem family;
* or merely a useful derived index.

---

# 9. Question corpus role

`Questions.txt` is an overcomplete natural-language inquiry corpus.

It is not a second formal language.

It shall be used as:

1. a source of candidate question renderings;
2. a pressure corpus for missing relational moves;
3. a regression suite for question coverage;
4. a source of reciprocal challenge patterns;
5. a source for discovering whether apparently different phrasings induce genuinely different inquiry transformations.

The harness shall not select questions merely because their wording differs.

A question is selected because it performs a needed relational operation.

---

# 10. Question-program families

The harness shall organize Coding and Reciprocal Why questions into the following derived program families.

These are provisional harness categories.

---

## Q1 — PRESENT / ABSENT / ROLE

Purpose: establish the current relational basis.

Questions:

> What is directly present?

> What is absent?

> Which represented forms are distinguishable?

> What relations are represented among them?

> Which roles or ports does each relation contain?

> Which form fills which role?

> Which positions remain open?

> Which relation is actual, generated, inferred, or assumed?

Reciprocal:

> If this form is removed while its relational position remains, what must fill the hole?

> Which apparent relation disappears when representation-specific structure is removed?

Primary effect:

$$
\Theta,\quad W,\quad O.
$$

---

## Q2 — OPEN / COMPLETE

Purpose: expose a lawful completion field.

Questions:

> What relation remains unresolved?

> Which position is open?

> What would count as a lawful filling?

> Does no filling remain, one protected class remain, or several?

Reciprocal:

> What did the answer leave unresolved?

> Which alternatives remain?

> Which part of the field was not touched by the return?

Primary form:

$$
?_I R[\beta].
$$

---

## Q3 — SEPARATE / MERGE

Purpose: refine consequential discrimination.

Questions:

> Which surviving possibilities matter differently?

> Which test/question/representation separates them?

> Which apparent sameness divides under a sharper continuation?

Reciprocal:

> Which apparent differences are protectedly irrelevant?

> Which question exposes nothing not already exposed elsewhere?

> Can these distinctions be merged?

Primary effect:

$$
K_C.
$$

---

## Q4 — CONTRAST / SUBTRACT

Purpose: expose a boundary efficiently.

Questions:

> What strong admissible contrast makes the difference unmistakable?

> Which extreme input, representation, load, ordering, environment, model, or interpretation puts the alternatives clearly on opposite sides?

Then:

> What can be removed from that contrast while the difference remains?

> What disappears first?

> What survives last?

> What happens if the final surviving distinction is removed?

Primary rhythm:

$$
\boxed{
\text{MAXIMAL USEFUL CONTRAST}
\to
\text{SUBTRACTION}.
}
$$

---

## Q5 — FORCE / RELEASE

Purpose: distinguish possibility, permission, forcing, and necessity.

Questions:

> Under which conditions is the result possible?

> Which added condition excludes the remaining alternatives?

> Could every current condition remain while the result differed?

> What makes the result unavoidable?

Reciprocal:

> Which condition can disappear while the result remains forced?

> Which next removal admits another result?

> Which alternative enters?

> What boundary did that removal cross?

Primary effect:

$$
W\rightleftarrows\Field(W).
$$

---

## Q6 — FACTOR / COMPOSE / BYPASS

Purpose: expose intermediate relations and paths.

Questions:

> Through what relation does this propagate?

> Which intermediate relation carries the difference?

> Can the endpoint relation be reconstructed from the path?

> Could another path reach the same endpoint?

Reciprocal:

> Can this mediator be removed?

> Can it be replaced?

> What remains if only the endpoints are retained?

> Which future continuation makes the path difference matter?

---

## Q7 — ORIENT / RECONSTRUCT

Purpose: distinguish forward constraint from backward compatibility.

Questions:

> Given these conditions, what later states or results remain possible?

> What does this relation constrain forward?

Reciprocal:

> Given the result, which predecessors remain compatible?

> Is reconstruction unique?

> Which ambiguity exists only backward?

Never collapse:

$$
\text{converse}
\neq
\text{inverse}
\neq
\text{reconstruction}
\neq
\text{same-use reciprocal return}.
$$

---

## Q8 — ORDER / INTERACTION

Purpose: expose succession and joint variation.

Questions:

> What happens first?

> What changes next?

> What persists?

> What changes if two operations exchange order?

> Which changes are harmless individually but consequential jointly?

> Which interaction is invisible under one-at-a-time variation?

Reciprocal:

> Could the same endpoint remain while the history differed?

> Which future continuation separates the histories?

---

## Q9 — RETURN / GROUND / DEFEAT

Purpose: preserve the actuality and evidence boundary.

Questions:

> What actually returned?

> What is directly established?

> What is inferred?

> What is generated?

> What independent checker discriminates the claim?

Reciprocal:

> What return would defeat it?

> Could the same evidence support another interpretation?

> Which support paths share ancestry?

> What remains if inferred interpretation is withheld?

Maintain:

$$
\boxed{
\text{candidate}
\neq
\text{actual return}
\neq
\text{interpretation}
\neq
\text{checked result}
\neq
\text{warrant}.
}
$$

---

## Q10 — WHY / FOIL / RECIPROCAL CHALLENGE

`Why` is not a primitive question root.

It is a post-return question program.

Sequence:

> What exactly returned?

> Which alternatives remain live?

> Why this rather than which specific admissible foil?

> What condition or relation separates the returned answer from that foil?

> Through what path could that condition affect the result?

> Could the proposed reason disappear while the result remained?

> Could the proposed reason remain while the result failed?

> What strong contrary case would break the explanation?

> Which dimensions of that contrary case can be removed?

> Which independent return supports what survives?

> Which competing explanation remains?

> What case separates the competing explanations?

This is the principal closure-attack program.

---

## Q11 — REPAIR / REBREAK

Purpose: repair demonstrated failures without local search bias.

Questions:

> What sufficiently strong authorized change unquestionably removes the demonstrated failure?

> Which part of that change affects the responsible relation?

After success:

> Which repair components can disappear?

> Which next removal causes failure to return?

> Which new condition breaks the reduced repair?

Rhythm:

$$
\boxed{
\text{FAIL}
\to
\text{SUFFICIENT REPAIR}
\to
\text{SUBTRACT}
\to
\text{REBREAK}.
}
$$

---

## Q12 — FOLD / REOPEN / REGENERATE

Purpose: compress without destroying protected future use.

Questions:

> Which differences have no protected consequence?

> Which question or representation is redundant?

> Can the removed structure be regenerated or reacquired when needed?

> What is lost if one more distinction is removed?

Reciprocal:

> Which future question would separate the merged cases?

> Which context invalidates the fold?

> What information must remain to reopen the distinction?

Rhythm:

$$
\boxed{
\text{FOLD}
\rightleftarrows
\text{REOPEN}.
}
$$

---

## Q13 — TRANSPORT / SUBSTITUTE / ROUND-TRIP

Purpose: test whether a discovered relation survives new objects or domains.

Questions:

> Which source-specific properties can be removed?

> What alien object or implementation can occupy the same role?

> Which protected consequences survive substitution?

Reciprocal:

> Can the abstract relation regenerate the source case?

> Which native role fails to return?

> What was removed that should not have been?

> How little must be restored?

---

## Q14 — QUESTION / PRUNE / INVENT

Purpose: inquire into the question basis itself.

Questions:

> Which alternatives did this question separate?

> Which did it leave merged?

> Which other question induces the same consequential partition?

> Which cheaper question produces the same protected split?

> Which stronger question adds a consequential distinction?

> Which answer made another question unnecessary?

> Which answer made a new question formable?

> What question is currently missing?

> Is it ungenerated, unrepresentable, or unexecutable?

Reciprocal:

> What is lost if this question is removed?

---

# 11. Question selection

Do not run all question families.

Do not cycle through them mechanically.

The current residual selects the next family.

Examples:

| Residual                                      | Question family |
| --------------------------------------------- | --------------- |
| forms/roles unclear                           | Q1              |
| unresolved typed port                         | Q2              |
| plural protected classes                      | Q3              |
| boundary location unclear                     | Q4              |
| necessity/sufficiency claim                   | Q5              |
| mechanism/path unclear                        | Q6              |
| prediction/reconstruction confusion           | Q7              |
| order/joint interaction unresolved            | Q8              |
| weak support                                  | Q9              |
| apparent explanation/closure                  | Q10             |
| demonstrated implementation failure           | Q11             |
| representation appears excessive              | Q12             |
| abstraction/generalization proposed           | Q13             |
| question set itself excessive or insufficient | Q14             |

Where several question programs are productive and incomparable, retain a frontier.

Do not manufacture a total ranking unless the binding supplies one.

---

# 12. Question span

A collection of questions is not adequate merely because it contains many prompts.

The active subset must span the consequential relations relevant to the residual.

Where applicable, it should pressure at least:

1. admissibility;
2. discrimination;
3. path/direction/order;
4. actuality/support;
5. representation/regeneration.

A question subset is suspect if every member performs essentially the same relational transformation.

## 12.1 Surface redundancy

Two questions may normalize to the same open typed relation.

## 12.2 Discrimination redundancy

Two questions may induce the same kernel.

## 12.3 Field redundancy

Two questions may generate the same consequential contraction.

## 12.4 Continuation redundancy

Two questions may produce protected-equivalent successor programs.

## 12.5 Operational non-redundancy

Even equal semantic partitions may differ in:

* probe occurrence;
* evidence route;
* provenance;
* cost;
* authority;
* side effects;
* future unlocks.

Therefore semantic equality does not automatically license executable substitution.

---

# 13. Outer engineering clock

Retain the current repository lifecycle:

$$
\boxed{
\text{SPECIFY}
\to
\text{INSPECT}
\to
\text{CONTRAST}
\to
\text{TRACE}
\to
\text{EXPERIMENT}
\to
\text{UPDATE}
\to
\text{CHANGE}
\to
\text{VERIFY}
\to
\text{CHALLENGE}
\to
\text{MINIMIZE}
\to
\text{RATCHET}.
}
$$

It remains a repository-work clock, not the successor's semantic recurrence.

The question programs above operate **inside** these positions.

## SPECIFY

Use Q1, Q2, Q9.

Establish:

* contract;
* authority;
* scope;
* applicability;
* protected horizon;
* current open relation.

## INSPECT

Use Q1, Q6, Q7, Q9.

Establish actual repository/formalization state.

## CONTRAST

Use Q3, Q4, Q5, Q13.

Prefer a strong separating contrast.

Do not search for the smallest contrast first.

## TRACE

Use Q6, Q7, Q8.

Find a sufficient responsible relational path.

Localize the necessary subpath after it is known to carry the difference.

## EXPERIMENT

Use Q3, Q4, Q9.

Select a discriminator that can actually separate the live alternatives.

Seal its prediction before observing the return.

## UPDATE

Use Q2, Q9, Q10.

Rebuild the live completion field from the return.

Do not continue the pre-return story.

## CHANGE

Use Q11 plus Q6/Q5 as required.

Construct a sufficiently strong authorized and reversible candidate that crosses the demonstrated boundary.

Do not require it to be minimal before success is established.

## VERIFY

Use Q9 plus the relevant discriminator family.

Obtain actual independent checks.

## CHALLENGE

Use Q4, Q5, Q7, Q8, Q10, Q11.

Attack:

* necessity;
* sufficiency;
* ordering;
* representation;
* explanation;
* scope;
* alternate routes;
* contrary cases.

## MINIMIZE

Use Q4, Q5, Q12, Q14.

Subtract successful excess.

Minimality belongs here.

## RATCHET

Retain:

* the smallest warranted relation;
* evidence;
* coverage;
* reusable discriminator;
* residual topology;
* reopening condition.

Then select the next frontier.

---

# 14. Wide expansion under bounded execution

Wide exploration does not require executing every generated question.

Distinguish:

$$
\boxed{
\text{candidate expansion}
\neq
\text{question discharge}.
}
$$

The model may generate a broad candidate frontier cheaply.

Only selected productive or required question occurrences need enter the checked trace and consume bounded execution resources.

Selection should prefer questions that:

* divide large live fields;
* separate protected classes;
* test a shared condition appearing in many residuals;
* resolve a blocker that gates many other inquiries;
* test a fold affecting many constructions;
* attack a high-support but weakly challenged relation;
* create reusable boundary information.

This is the mechanism by which wide exploration can reduce later search cost rather than merely multiply work.

---

# 15. Information gain without scalar dogma

The harness may estimate the structural value of a question from how much of the residual field its possible answers can discriminate.

It shall not require one global scalar utility function.

Useful structural preferences include:

* separates many currently live classes;
* resolves a condition shared across many residuals;
* distinguishes two large basins;
* converts `Unknown` into typed alternatives;
* tests a fold with broad downstream effect;
* eliminates many candidate paths;
* exposes a reusable breaker family.

A balanced discriminator tree may support near-logarithmic later localization.

This is an optimization objective, not a universal complexity theorem.

---

# 16. Complexity objective

The harness shall explicitly seek factorization of relational discovery.

It shall not claim that arbitrary full relational mapping is universally:

$$
O(\log n).
$$

If \(n\) constructions contain \(\Theta(n^2)\) independent relations, the output itself may require quadratic information.

The intended gain occurs when the relational structure factors through a much smaller discriminator family:

$$
k\ll n.
$$

Then an initial profiling/expansion pass may construct shared coordinates, after which new cases can be routed through a discriminator hierarchy rather than compared against every prior object.

The design objective is:

$$
\boxed{
\textbf{BUILD A REUSABLE DISCRIMINATOR GEOMETRY
THAT MAKES FUTURE LOCALIZATION SUBSTANTIALLY CHEAPER
THAN REPEATED PAIRWISE SEARCH.}
}
$$

Under balanced partitions this may approach:

$$
O(\log n)
$$

decision depth for localization.

The harness must measure rather than assume this gain.

---

# 17. Contradiction discipline

Contradiction is an inquiry result, not something to smooth away.

When two retained constructions appear jointly incompatible, open:

> Are their bindings actually the same?

> Which applicability condition separates them?

> Which premise differs?

> Which representation hides the difference?

> Which grain differs?

> Which support path is invalid?

> Which exact joint field becomes empty?

A contradiction should generate a separator/residual, not a prose reconciliation.

---

# 18. Blocker discipline

A blocker is structured information.

If question \(q\) cannot be discharged, classify why:

* representation unavailable;
* question unformable;
* applicability unresolved;
* method unavailable;
* tool unavailable;
* evidence authority unavailable;
* environment failure;
* resource limit;
* coverage insufficiency.

Then open:

> What relation would make this capability available?

> Can another method discharge the same semantic question?

> Which other residual shares this blocker?

Repeated blockers may reveal a missing representation or method worth constructing.

---

# 19. Failed-fold discipline

Every proposed identification or compression must remain challengeable.

Given candidate fold:

$$
c:X\to Y,
$$

ask whether:

$$
c(x)=c(y)
$$

ever occurs with:

$$
x\not\equiv_{\mathcal H}y.
$$

A witnessed pair is a fold breaker.

That fold breaker becomes a reusable discriminator coordinate.

Failed folds therefore contribute directly to the relational map.

---

# 20. Primitive and definition discovery

For every proposed successor primitive \(p\):

1. remove \(p\);
2. preserve surrounding typed relations;
3. inspect the hole;
4. compute or approximate its lawful completion field;
5. ask whether \(p\)'s protected role regenerates;
6. seek alternate fillers;
7. seek a breaker distinguishing \(p\) from those fillers.

If the role regenerates from existing structure:

$$
p
$$

is a candidate derived construction rather than primitive.

If removal destroys a protected capability:

identify the exact capability.

Do not retain a primitive because its name appears frequently in v2.0.

---

# 21. Theorem discovery

For every theorem candidate:

$$
H\Rightarrow C,
$$

ask:

> What maximal admissible case satisfies \(H\) while trying to violate \(C\)?

Seek:

$$
H\land\neg C.
$$

If a witness exists, use it to refine the hypotheses or theorem scope.

If no witness is found, preserve the search coverage.

Only exact exhaustion or proof establishes the corresponding universal claim.

Then reciprocate:

> Which hypothesis in \(H\) can be removed while \(C\) remains forced?

The surviving hypothesis frontier defines the current theorem boundary.

---

# 22. Definition refinement

For every predecessor definition:

1. identify its positive examples;
2. identify its exclusions;
3. construct extreme admissible cases;
4. seek cases admitted that should separate;
5. seek cases excluded that should coincide;
6. identify the condition causing each difference;
7. subtract source-specific residue;
8. machine-check the surviving relation.

Definitions are therefore pressure-tested from both directions.

---

# 23. Cross-construction propagation

After any new relation \(\rho\) is ratcheted:

1. search all active residuals for \(\rho\);
2. search past unresolved residuals whose conditions overlap \(\rho\);
3. identify folds licensed conditionally on \(\rho\);
4. identify methods whose applicability depends on \(\rho\);
5. identify contradictions containing \(\rho\);
6. identify theorem hypotheses containing \(\rho\);
7. identify question families whose discrimination changes under \(\rho\);
8. regenerate their current status.

This is mandatory when doing so is computationally feasible.

It is the primary mechanism by which one proof contributes to the whole relational map.

---

# 24. Method formation

A repeated question/operation path may be proposed as a method when:

* the same relational shape recurs;
* applicability conditions can be stated;
* its expansion can be regenerated;
* its failure exits are explicit;
* its evidence/provenance discipline is preserved;
* its protected behavior is stable under tested substitutions;
* its reopening conditions are known.

A method is a compressed route.

It does not become a semantic primitive merely because it is useful.

---

# 25. Machine-checking relationship

The harness may generate:

* conjectures;
* definitions;
* theorem statements;
* countermodels;
* candidate repairs;
* representations;
* question programs.

Independent formal machinery determines what survives.

For Lean-controlled claims:

$$
\boxed{
\text{LLM proposal}
\neq
\text{Lean theorem}.
}
$$

For finite countermodel search:

$$
\boxed{
\text{no finite breaker found}
\neq
\text{universal validity}.
}
$$

For repository execution:

$$
\boxed{
\text{green test occurrence}
\neq
\text{universal behavioral theorem}.
}
$$

Each tool return is interpreted only at its declared coverage.

---

# 26. Trace contract

Retain the current append-only trace design.

A consequential cycle remains:

$$
\boxed{
\text{SEAL}
\to
\text{RAW RETURN}
\to
\text{CHECK}
\to
\text{RESIDUAL}.
}
$$

Do not rewrite trace ancestry.

The trace remains derived engineering evidence, not a second semantic event history.

## 26.1 Question record

A consequential question occurrence should retain at least:

* question;
* question-program family;
* mode;
* answer;
* occurrence identity;
* continuation identity;
* binding;
* horizon;
* coverage;
* authority;
* evidence;
* parent residual;
* relevant conditions;
* relevant breakers;
* reciprocal obligation when applicable.

## 26.2 Seal

A seal should state:

* expected protected change;
* protected invariants;
* discriminator;
* explicit contrary/wrong region the discriminator must reject;
* declared coverage.

Do not require the “wrong implementation” to be the smallest one.

Prefer a **decisive wrong region or representative breaker**.

## 26.3 Residual record

A residual should be able to retain derived coordinates such as:

* parent residual;
* open relation;
* condition identifiers;
* blocker identifiers;
* breaker identifiers;
* failed-fold identifiers;
* overlapping residual identifiers;
* coverage;
* next productive question family.

These may be stored as extra trace coordinates or as derived rebuildable projections.

No second authoritative history shall be created.

---

# 27. Residual index

The harness may maintain a derived rebuildable index over the append-only trace.

The index may support queries such as:

* residuals containing condition \(\rho\);
* residuals sharing breaker \(b\);
* residuals blocked by capability \(k\);
* folds broken by discriminator \(d\);
* questions that induced the same partition;
* paths repeatedly appearing across inquiries;
* unresolved `Unknown`s depending on a newly established relation.

The index is a projection.

The trace remains the evidence source.

Deleting the index must not destroy semantic or evidential ancestry.

It must be regenerable.

---

# 28. Current project-state ownership

Preserve the repository's ownership separation.

The broad residual topology is not an excuse to turn `IMPLEMENTATION_FRONTIER.md` into an unbounded research notebook.

`IMPLEMENTATION_FRONTIER.md` should continue to expose the strongest currently executable residual.

The complete residual topology belongs in:

* append-only trace evidence;
* rebuildable derived indices;
* formal proof obligations;
* explicit project artifacts that genuinely own those relations.

Stable documents should not accumulate exploratory chronology.

---

# 29. Stop semantics

The harness shall preserve typed local stop states such as:

* `Satisfied`;
* `Equivalent`;
* `Impossible`;
* `Blocked`;
* `Unknown`;
* `ResourceBounded`.

Interpret them locally.

### Satisfied

The declared obligation is satisfied under its binding, horizon, coverage, and warrant.

It does not mean global successor completion unless the obligation itself is the exact declared global completion contract.

### Equivalent

The branch is protectedly equivalent under the declared horizon.

Retain reopening conditions.

### Impossible

Requires an adequate certificate or exact field exhaustion.

### Blocked

Names the missing capability.

### Unknown

Means the current evidence cannot classify the field.

### ResourceBounded

Returns the partial result and unconsumed residual frontier.

---

# 30. Anti-closure questions

Before any substantial closure ask:

> Which alternatives remain?

> Which breaker classes remain untested?

> Which continuations remain uncovered?

> Which applicability boundaries remain uncertain?

> Which representation gaps remain?

> Which required relations remain unchecked?

> Which current equivalence could a future continuation split?

> What would reopen this conclusion?

If a consequential distinction remains live, closure is local only.

---

# 31. Harness self-application

The harness may be subjected to its own inquiry questions.

Ask:

> Which harness question families actually produce distinct protected construction behavior?

> Which are redundant?

> Which residual classes recur?

> Which question sequence reliably exposes breakers faster?

> Which search strategy produces better coverage for equal resource use?

> Which harness distinction has no demonstrated effect?

However:

$$
\boxed{
\text{THE HARNESS CANNOT WARRANT ITS OWN SUCCESSOR STATUS.}
}
$$

Changes to protected harness files require independent authorized control.

A useful harness pattern may be retained as engineering method without becoming part of the calculus.

---

# 32. Expected repository changes

Implementation of this specification should primarily affect the following existing surfaces.

## `AGENTS.md`

Retain the outer engineering clock.

Replace local-search wording that prematurely demands:

* smallest wrong implementation;
* smallest responsible path;
* smallest working change before success is established.

State instead:

* strong contrast first;
* locate a sufficient responsible path;
* produce a decisive breaker;
* make a sufficient authorized reversible change;
* minimize after success;
* retain only the smallest warranted deformation.

Add residual overlap and propagation obligations.

## `.claude/skills/inquire/SKILL.md`

Keep the mechanically recorded cycle.

Change the active reasoning instruction from local-minimal experimentation toward:

$$
\text{strong contrast}
\to
\text{return}
\to
\text{reciprocal challenge}
\to
\text{subtraction}.
$$

Add explicit guidance for residual overlap and propagation.

## `software-engineering-binding.md`

Replace the simple clock/root mapping with:

$$
\text{clock position}
\to
\text{open relation}
\to
\text{question-program families}
\to
\text{reciprocal families}
\to
\text{candidate discharge methods}.
$$

Document wide-search/narrow-ratchet as an engineering search discipline.

## `.claude/hooks/ic-inject`

Keep the injection short.

It should remind the agent of the governing motion rather than inject a long question bank.

Recommended conceptual content:

```text
INQUIRY HARNESS ACTIVE

Explore widely; commit narrowly.

Use strong admissible contrast to expose a protected boundary.
Preserve the actual return.
Ask the reciprocal question.
Subtract toward the boundary only after separation is established.
Ratchet only what the evidence proves.
Propagate newly established conditions through overlapping residuals.
Local closure is not global exhaustion.

Generated != Actual != Checked != Warranted.
Unknown != Negative.
No self-warrant.
```

The current engineering clock may remain visible beneath this.

## `.claude/hooks/ic-trace`

Preserve append-only behavior.

Allow richer derived coordinates on question/residual records where useful.

Do not create a second authoritative history.

## `.claude/hooks/ic-guard`

Preserve the existing anti-self-modification and sealed-cycle gate unless an independently demonstrated problem requires change.

The new reasoning policy must not weaken control authority.

---

# 33. Harness acceptance tests

The revised harness is not accepted merely because its prose sounds consistent.

It should demonstrate the following.

## Test 1 — Wide breaker search

Given a problem where local perturbation requires many trials but an extreme admissible contrast immediately separates success/failure, the harness selects the extreme contrast.

## Test 2 — Narrow ratchet

Given a broad generated theory and a return supporting only one subrelation, the harness retains only the supported subrelation.

## Test 3 — Residual persistence

Closing one branch does not erase unrelated or partially overlapping residuals.

## Test 4 — Shared-condition propagation

When a newly proved condition occurs in several earlier residuals, those residuals are revisited without rediscovering the condition from zero.

## Test 5 — Contradiction localization

An apparent contradiction opens a typed separator/applicability question instead of being reconciled rhetorically.

## Test 6 — Failed-fold reopening

A newly protected continuation that separates a previously folded pair reopens the fold.

## Test 7 — Question redundancy

Several differently worded questions inducing the same protected inquiry behavior can be collapsed.

## Test 8 — Question non-redundancy

Questions with similar wording but different partitions, paths, discharge modes, or continuations remain distinct.

## Test 9 — Joint variation

The harness can expose failures invisible under one-at-a-time ablation.

## Test 10 — Anti-premature closure

Failure to find a breaker under incomplete coverage remains `Unknown`.

## Test 11 — Method factoring

Repeated successful inquiry paths can be proposed as methods, with expansion and failure exits preserved.

## Test 12 — Harness non-self-warrant

The agent cannot modify protected harness acceptance criteria merely because a candidate successor would otherwise fail.

---

# 34. Operational pseudocode

```text
function inquire(state Σ, residual Δ):

    frame Δ under current binding, authority, horizon, and coverage

    candidate_questions :=
        expand_question_frontier(Δ, question_library)

    candidate_contrasts :=
        construct_strong_admissible_contrasts(
            Δ,
            known_breakers,
            known_nonbreakers,
            prior_residual_basins
        )

    reuse previously established conditions whenever their
    typing, scope, applicability, and provenance match

    choose a productive or required question q
    whose possible returns can most usefully divide the live residual

    if execution is required:
        seal prediction
        obtain immutable/raw return
        check return independently
    else:
        derive through admitted pure structure

    resolution :=
        classify_return(
            supported,
            plural,
            exact_empty,
            unsupported,
            unknown,
            blocked,
            resource_bounded
        )

    ratchet only the strongest relation forced by resolution

    ask reciprocal questions:
        - contrary foil?
        - removal?
        - reverse?
        - bypass?
        - alternate path?
        - support defeat?
        - fold breaker?

    subtract excess only after a consequential separation is established

    new_residuals :=
        construct_residuals_from(
            surviving alternatives,
            breakers,
            blockers,
            contradictions,
            failed folds,
            representation gaps,
            unknown coverage
        )

    propagate newly established relations through residual index

    detect recurring conditions and overlapping residual basins

    propose factorizations or reusable methods when recurrence warrants inquiry

    fold only when protected behavior, regeneration,
    provenance, and reopening survive

    retain reopen conditions

    choose next executable frontier

    recur unless the exact declared obligation has a lawful stop
```

---

# 35. Governing question rhythm

The harness's shortest usable reasoning rhythm is:

$$
\boxed{
\begin{array}{c}
\textbf{WHAT IS HERE AND HOW IS IT RELATED?}\\
\downarrow\\
\textbf{WHAT IS OPEN?}\\
\downarrow\\
\textbf{WHAT BROAD CONTRAST MAKES THE DIFFERENCE CLEAR?}\\
\downarrow\\
\textbf{WHAT QUESTION ACTUALLY SEPARATES THE LIVE POSSIBILITIES?}\\
\downarrow\\
\textbf{WHAT RETURN DID THAT QUESTION PRODUCE?}\\
\downarrow\\
\textbf{WHAT DID THE RETURN REALLY DETERMINE?}\\
\downarrow\\
\textbf{WHY THIS RATHER THAN THE STRONGEST LIVE FOIL?}\\
\downarrow\\
\textbf{WHAT CONDITION / PATH / DISTINCTION CARRIES THE SEPARATION?}\\
\downarrow\\
\textbf{WHAT HAPPENS IF IT IS REMOVED, REVERSED, REPLACED, OR BYPASSED?}\\
\downarrow\\
\textbf{HOW FAR CAN THE CONTRAST BE SUBTRACTED?}\\
\downarrow\\
\textbf{WHAT IS THE SMALLEST RELATION ACTUALLY PROVED?}\\
\downarrow\\
\textbf{WHERE ELSE DOES THAT RELATION ALREADY APPEAR?}\\
\downarrow\\
\textbf{WHICH RESIDUALS NOW OVERLAP?}\\
\downarrow\\
\textbf{WHAT RECURRING STRUCTURE CAN BE FACTORED?}\\
\downarrow\\
\textbf{WHAT CAN BE FOLDED?}\\
\downarrow\\
\textbf{WHAT WOULD REOPEN THE FOLD?}\\
\downarrow\\
\textbf{WHAT IS OPEN NOW?}
\end{array}
}
$$

---

# 36. Final construction law

The harness shall embody:

$$
\boxed{
\textbf{
EXPANSION DISCOVERS THE BOUNDARIES;
CONTRACTION LOCATES THEM;
RATCHETING RETAINS ONLY WHAT THEY FORCE;
RESIDUAL OVERLAP CONNECTS THE CALCULUS.
}
}
$$

And:

$$
\boxed{
\textbf{
A RELATION NEED NOT BE MAPPED PAIRWISE TO EVERY OTHER RELATION
WHEN THEIR SHARED CONDITIONS, BREAKERS, CONTRADICTIONS,
HOLES, AND FAILED FOLDS ALREADY PROVIDE A FACTORIZED PATH BETWEEN THEM.
}
}
$$

The inquiry process should progressively construct the discriminator geometry through which later inquiry becomes cheaper.

Thus:

$$
\boxed{
\textbf{
INQUIRY DOES NOT MERELY TRAVERSE A SEARCH SPACE.
IT CONSTRUCTS THE COORDINATE SYSTEM THAT MAKES
SUBSEQUENT SEARCH MORE EFFICIENT.
}
}
$$

The coordinate system remains conditional, revisable, and reopenable.

Every local determination may become a condition in a later inquiry.

Every condition may become the discriminator of another residual.

Every breaker may connect otherwise distant constructions.

Every removal may expose a typed hole.

Every hole may generate the question that reconstructs what was removed.

Every accepted compression carries the question that would invalidate it.

The successor construction therefore proceeds not by exhaustively connecting every piece to every other piece, but by repeatedly expanding, colliding, discriminating, contracting, checking, ratcheting, propagating, factoring, folding, and reopening until the strongest supported relational structure can be machine-checked and regenerated from its own typed obligations.

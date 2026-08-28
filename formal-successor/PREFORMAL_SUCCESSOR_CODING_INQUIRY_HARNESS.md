# Preformal Successor Coding Inquiry Harness

## Status

This harness is a construction method for the agent formalizing the successor calculus.

It is **not** part of the successor semantics unless the formal successor independently regenerates it.

Its purpose is to make the agent reason through the emerging calculus while constructing that calculus, using the existing Coding and Reciprocal Why question corpora as an overcomplete question surface.

The harness must not execute the question corpus as a checklist.

It must compile corpus questions into a smaller collection of reciprocal inquiry programs whose use changes a different part of the current inquiry relation.

---

# 1. Provisional successor rhythm

The predecessor recurrence

$$
\mathsf{BIND}
\to
\mathsf{OPEN}
\to
\mathsf{VARY}
\to
\mathsf{RETURN}
\to
\mathsf{DETERMINE}
\to
\mathsf{REFACTOR}
$$

remains predecessor behavior.

For successor construction, use the more discriminating provisional rhythm:

$$
\boxed{
\begin{array}{c}
\mathsf{FRAME}\\
\downarrow\\
\mathsf{OPEN}\\
\downarrow\\
\mathsf{EXPAND}\\
\downarrow\\
\mathsf{DISTINGUISH}\;\parallel\;\mathsf{CONSTRAIN}\\
\downarrow\\
\mathsf{DISCHARGE}\\
\downarrow\\
\mathsf{RESOLVE}\\
\downarrow\\
\mathsf{RECIPROCATE}\\
\downarrow\\
\mathsf{RELEASE/SUBTRACT}\\
\downarrow\\
\mathsf{FOLD/REOPEN}\\
\downarrow\\
\mathsf{OPEN}\;\circlearrowleft
\end{array}
}
$$

These names are temporary harness names.

The relations beneath them are what matter.

### FRAME

Establish the typed carrier, represented relations, roles, scope, applicability, current evidence, protected future, available methods, and representation.

### OPEN

Partially bind a represented relation and expose the consequentially unresolved positions:

$$
q=?_I R[\beta].
$$

Recover its lawful completion field:

$$
F_q=\Fib_I(R\mid\beta).
$$

### EXPAND

Do not search locally for the smallest change.

Construct sufficiently different admissible fillings, contrasts, implementations, contexts, paths, or failures so that consequential separation becomes obvious.

Prefer a known opposite-side point over a long sequence of nearby guesses.

### DISTINGUISH

Refine what the inquiry can tell apart.

For a question/discriminator \(q\), refine the current question kernel.

Conceptually:

$$
K_{C\cup\{q\}}
=
K_C\cap \sim_q.
$$

### CONSTRAIN

Refine what may remain admissible.

For condition \(\rho\):

$$
W\mapsto W\cup\{\rho\},
$$

with:

$$
\Sol(W\cup\{\rho\})
\subseteq
\Sol(W).
$$

`DISTINGUISH` and `CONSTRAIN` are different motions.

A new discriminator may split possibilities without excluding either.

A new condition may exclude possibilities without increasing observational discrimination.

### DISCHARGE

Choose a method capable of answering the typed question.

Generation, static analysis, theorem proving, testing, runtime probing, external observation, checking, and warrant are different discharge modes.

### RESOLVE

Determine what the return actually establishes.

Preserve:

$$
\text{candidate}
\neq
\text{return}
\neq
\text{decoded completion}
\neq
\text{interpretation}
\neq
\text{support}
\neq
\text{standing}.
$$

Test whether the live field occupies one protected class:

$$
\Sol(W)/{\equiv_{\mathcal H}}
=
\{[x]_{\mathcal H}\}.
$$

### RECIPROCATE

Turn the achieved determination around.

Ask what alternative, removal, reverse direction, competing path, competing account, or future continuation would make the determination divide again.

No consequential closure is accepted without an available reciprocal challenge relation.

### RELEASE / SUBTRACT

Remove conditions, contrast dimensions, implementation differences, explanatory assumptions, path components, or representational distinctions.

Search from a strong known contrast toward the consequential boundary.

### FOLD / REOPEN

If a distinction changes no protected future and required regeneration remains available, test whether it can be folded.

For every fold immediately ask what future question, context, representation, continuation, or return would invalidate it.

Such a change reopens the distinction.

---

# 2. The two central reciprocal axes

Most Coding and Reciprocal Why questions fall around two primary axes.

## 2.1 Admissibility axis

$$
\boxed{
\mathsf{CONSTRAIN}
\rightleftarrows
\mathsf{RELEASE}
}
$$

Forward:

> Which additional condition excludes a surviving alternative?

Reciprocal:

> Which condition can be removed before that alternative returns?

This acts on:

$$
\Sol(W).
$$

## 2.2 Discrimination axis

$$
\boxed{
\mathsf{DISTINGUISH}
\rightleftarrows
\mathsf{COARSEN}
}
$$

Forward:

> Which test, question, representation, or observation separates these surviving possibilities?

Reciprocal:

> Which discriminator can disappear without merging anything the protected future distinguishes?

This acts on:

$$
\equiv_C.
$$

The two axes meet at protected determination:

$$
\boxed{
Determines_{\mathcal H}(W,x)
\iff
\Sol(W)/{\equiv_{\mathcal H}}
=
\{[x]_{\mathcal H}\}.
}
$$

The harness should repeatedly move along both axes rather than repeatedly asking differently worded questions along only one.

---

# 3. Coding and Reciprocal Why are complementary halves

Treat the Coding corpus primarily as the **outward constructive pressure**:

$$
\text{inspect}
\to
\text{open}
\to
\text{vary}
\to
\text{test}
\to
\text{trace}
\to
\text{repair}
\to
\text{compress}.
$$

Treat Reciprocal Why primarily as the **return-facing reciprocal pressure**:

$$
\text{return audit}
\to
\text{foil}
\to
\text{why this}
\to
\text{remove the alleged reason}
\to
\text{reverse}
\to
\text{challenge support}
\to
\text{reopen}.
$$

Neither corpus is sufficient alone.

Coding without Reciprocal Why tends toward constructive accumulation.

Reciprocal Why without Coding lacks actual variation, execution, and constructive search.

Their composition is the intended harness.

---

# 4. Question-program family A — FRAME / REFRAME

## Coding orientation

Ask:

> What is present in the current implementation, runtime, interface, test surface, and dependency boundary?

> What is not present?

> What depends on what?

> Which things occur together and which only separately?

> What may change independently?

> Where is the same comparison meaningful?

> Where does the comparison cease to apply rather than become false?

> Which difference survives changes of names, files, modules, functions, classes, or representations?

These establish the represented forms, relations, roles, applicability, and grain.

## Reciprocal orientation

Ask:

> Which apparent relation disappears when representation-specific structure is removed?

> Which apparent disagreement is actually a difference of scope, applicability, grain, environment, or representation?

> Which surrounding difference changes the relation?

> Which surrounding difference leaves it untouched?

> What appeared intrinsic but disappears after rebinding the context?

## Effect on inquiry

Primarily changes:

$$
\Theta.
$$

It defines the field in which later questions are meaningful.

## Current v2.0 root lowering

Primarily:

$$
\mathsf{Expose}
+
\mathsf{Factor}
+
\mathsf{Ground}.
$$

---

# 5. Question-program family B — OPEN / AUDIT THE RETURN

## Coding orientation

Ask:

> What consequential relation remains unresolved?

> Which role or port in that relation remains unfilled?

> Which implementations, behaviors, causes, paths, or explanations remain compatible?

> Does any admissible filling exist?

> Do several remain?

## Reciprocal Why orientation

Immediately after a return ask:

> What, exactly, has returned?

> What in the return is directly established?

> What remains only possible?

> What remains unresolved?

> What has been excluded?

> What has not been excluded?

> Which alternatives were live before the return?

> Which remain live afterward?

## Effect on inquiry

Forward:

$$
R[\beta]
\mapsto
\Fib_I(R\mid\beta).
$$

Return-facing:

$$
F^{-}
\mapsto
F^{+}
$$

without pretending that every member absent from \(F^{+}\) was formally excluded.

## Current v2.0 root lowering

$$
\mathsf{Expose}.
$$

---

# 6. Question-program family C — SEPARATE / MERGE

## Coding orientation

Ask:

> What can already be distinguished by observable behavior?

> What cannot yet be distinguished?

> Which apparent samenesses divide under a sharper input, context, timing, load, dependency, or test?

> Which test separates the surviving implementations?

> Which representation distinguishes possibilities another merges?

## Reciprocal Why orientation

Ask:

> Which alternatives differ consequentially from what returned?

> Which differ only in presentation?

> Which apparent alternatives collapse under the same conditions?

> Which apparently identical possibilities separate under a sharper comparison?

> Which present distinction has no future consequence?

> Which present sameness hides a future divergence?

## Forward movement

Find:

$$
x\equiv_Cy
\quad\land\quad
x\not\equiv_{\mathcal H}y
$$

and open:

$$
?q[
\sigma_q(x)\neq\sigma_q(y)
].
$$

## Reciprocal movement

Remove or coarsen discriminators and ask whether:

$$
\equiv_{C\setminus\{q\}}
$$

still preserves every protected distinction.

## Current v2.0 root lowering

$$
\mathsf{Polarize}
+
\mathsf{Expose}.
$$

---

# 7. Question-program family D — FORCE / RELEASE

This is the main condition-field cycle.

## Coding orientation

Ask:

> What may remain fixed?

> What may vary?

> Which condition turns a possible behavior into a required behavior?

> Which combination of conditions leaves no alternative execution?

> Could the same conditions remain while the output differed?

> Could the same output remain while the conditions differed?

> Which apparent requirement is dispensable?

> Which apparent sufficiency admits a failing execution?

## Reciprocal Why orientation

Ask:

> Under what conditions does the returned relation hold?

> Which conditions determine applicability?

> Which determine the result after applicability is established?

> Which conditions merely happen to be present?

> Must the result follow whenever these conditions hold?

> Which admissible case defeats that claim?

> Could the proposed condition disappear while the result remained?

> Could the proposed condition remain while the result disappeared?

> Which smallest release reopens another alternative?

> Which next removal admits one?

## Forward motion

$$
W
\mapsto
W\cup\{\rho\}.
$$

## Reciprocal motion

$$
W
\mapsto
W\setminus\{\rho\}.
$$

Inspect:

$$
\Shell_W(\rho)
=
\Sol(W\setminus\{\rho\})
-
\Sol(W).
$$

## Current v2.0 root lowering

$$
\mathsf{Vary}
+
\mathsf{Polarize}.
$$

---

# 8. Question-program family E — MAXIMAL BREAK / SUBTRACT

This is the default search strategy whenever a boundary is unknown.

## Coding orientation

Ask:

> Against which valid input, environment, configuration, dependency version, timing, load, or failure condition does the current behavior fail most clearly?

> Which case maximally separates the surviving explanations?

> What becomes obvious only under an extreme contrast?

## Reciprocal Why orientation

Ask:

> Against which admissible alternative is the returned answer strongest?

> Which alternative most sharply exposes what the answer depends on?

> What becomes obvious under the strongest contrast?

Then subtract:

> Which dimensions of that contrast can be removed?

> What survives the first removal?

> What survives the next?

> Which difference disappears first?

> Which survives last?

> What happens if the final surviving distinction is removed?

## Search law

Do not search:

$$
\text{nearby}
\to
\text{nearby}
\to
\text{nearby}.
$$

Prefer:

$$
\boxed{
\text{KNOWN SAME SIDE}
\;\longleftrightarrow\;
\text{KNOWN OPPOSITE SIDE}
}
$$

then locate the boundary.

Conceptually:

$$
D_{\max}
\supset
D_1
\supset
\cdots
\supset
D^\ast.
$$

## Current v2.0 root lowering

$$
\mathsf{Polarize}
+
\mathsf{Vary}.
$$

---

# 9. Question-program family F — FACTOR / BYPASS / RECOMPOSE

## Coding orientation

Ask:

> What lies between the input and output?

> Which intermediate relation is directly established?

> Which is inferred?

> What does the intermediate layer preserve, transform, suppress, or introduce?

> Can the direct relation be reconstructed from the composed internal relations?

> Could another intermediate occupy the same role?

## Reciprocal Why orientation

Ask:

> Through what does the result follow?

> Which part of the path can be bypassed without changing the result?

> Could another path reach the same result?

> What remains common between the paths?

> Could the endpoints remain the same while the intermediate relations differed?

> Which future continuation would make that path difference consequential?

## Effect on inquiry

Moves between:

$$
R,\;S
\quad\text{and}\quad
S\circ R.
$$

It tests whether an intermediate relation is necessary, replaceable, or merely one realization of a more general path relation.

## Current v2.0 root lowering

$$
\mathsf{Factor}
+
\mathsf{Orient}.
$$

---

# 10. Question-program family G — ORIENT / RECONSTRUCT

## Coding orientation

Ask:

> From these conditions, what later behavior remains possible?

> What does the forward path constrain?

> Must this output remain whenever these conditions remain?

## Reciprocal Why orientation

Ask:

> From the result, which earlier conditions remain compatible?

> Can the result reconstruct the earlier conditions uniquely?

> Which ambiguity appears only backward?

> Which condition becomes merely possible rather than required when the relation is read backward?

> Could a valid backward derivation exist without being a valid explanation in that direction?

## Required non-collapse

Maintain:

$$
\text{forward}
\neq
\text{converse}
\neq
\text{inverse}
\neq
\text{reconstruction}
\neq
\text{causal/explanatory reverse}.
$$

## Current v2.0 root lowering

$$
\mathsf{Orient}.
$$

---

# 11. Question-program family H — SUCCESSION / REORDER / INTERACTION

## Coding orientation

Ask:

> What happens first?

> What follows?

> What changes across each transition?

> What persists?

> What appears only temporarily?

> Could the same final result arise through a different sequence?

> Which modifications are harmless separately but consequential together?

## Reciprocal Why orientation

Ask:

> Why does order matter here?

> What happens if the modifications exchange order?

> Which consequence survives both orders?

> Which appears only in one?

> Could both orders reach the same endpoint?

> Which later continuation distinguishes their histories?

> Which conclusion belongs to actual succession?

> Which arises only from analytical reordering?

## Effect on inquiry

Tests:

$$
S\circ R
\stackrel?{\equiv_{\mathcal H}}
R\circ S
$$

where both compositions are well typed.

It also tests interactions that are invisible under singleton variation.

## Current v2.0 root lowering

$$
\mathsf{Orient}
+
\mathsf{Factor}
+
\mathsf{Vary}.
$$

---

# 12. Question-program family I — GROUND / WITHHOLD / DEFEAT

## Coding orientation

Ask:

> What is directly available from code, configuration, trace, test result, runtime return, or documented contract?

> What appears only after one available fact is related to another?

> Which conclusion has an independent test or observation?

> Could two inferred explanations support one another without either touching an independent return?

## Reciprocal Why orientation

Ask:

> What is directly available?

> What is inferred?

> Which part of the explanation came from the return?

> Which part was introduced afterward?

> What remains if every introduced interpretation is withheld?

> Could the same evidence support another interpretation?

> Which independent return would separate them?

> What independently supports this relation?

> Which support routes share ancestry?

## Effect on inquiry

Changes support/authority status rather than merely changing the semantic completion field.

No generated explanation acquires standing merely by answering these questions.

## Current v2.0 root lowering

$$
\mathsf{Ground}
+
\mathsf{Polarize}.
$$

---

# 13. Question-program family J — REPAIR / REBREAK

Use after an implementation failure has been established.

## Coding orientation

Ask:

> Which later modification removes the failure?

> Could the failure disappear only because the failing case is no longer exercised?

> Which apparent fix changes only presentation, logging, or symptoms?

> Which changes the behavior that produced the failure?

> Which later input, environment, timing, dependency, load, or order makes it return?

## Reciprocal Why orientation

Ask:

> Why did the relation fail?

> Where did it first fail?

> Could the same failure occur through another route?

> Could the same route succeed under another surrounding condition?

Then do **not** begin by looking for the smallest repair.

Ask:

> What deliberately excessive repair unquestionably removes the failure?

Then:

> Which parts of that repair are unnecessary?

> What happens when they are removed?

> Does failure return?

> Which next removal crosses the repair boundary?

## Rhythm

$$
\boxed{
\text{FAIL}
\to
\text{OVER-REPAIR}
\to
\text{SUBTRACT}
\to
\text{REBREAK}.
}
$$

## Current v2.0 root lowering

$$
\mathsf{Vary}
+
\mathsf{Polarize}
+
\mathsf{Ground}.
$$

---

# 14. Question-program family K — FOLD / REOPEN / REGENERATE

## Coding orientation

Ask:

> Which implementation differences disappear without changing required behavior?

> Which two tests expose the same consequential distinction?

> What can be removed from the current representation?

> What is the smallest retained representation from which every required behavior can still be recovered?

> What happens if one more part is removed?

> Which exact loss marks the boundary?

## Reciprocal Why orientation

Ask:

> Why preserve this distinction?

> What becomes impossible to predict, reconstruct, distinguish, or recover if it disappears?

> Could the distinction disappear without consequential loss?

> Why merge these possibilities?

> Which protected continuations agree?

> Which one might still separate them?

> What future observation would reopen the merged class?

> What minimum information must survive so that reopening remains possible?

## Rhythm

$$
\boxed{
\text{DISTINGUISH}
\to
\text{FOLD}
\to
\text{CHALLENGE THE FOLD}
\to
\text{REOPEN}.
}
$$

## Current v2.0 root lowering

$$
\mathsf{Factor}
+
\mathsf{Vary}
+
\mathsf{Polarize}.
$$

---

# 15. Question-program family L — SUBSTITUTE / TRANSPORT / REGENERATE SOURCE

Use for implementation replacement, backend substitution, representation change, and cross-domain abstraction.

## Coding orientation

Ask:

> In place of what implementation detail can another occupy the same role without changing protected behavior?

> Which substitution changes the internal path but preserves the external result?

> What must every acceptable replacement preserve?

> What may vary freely?

## Reciprocal Why orientation

Ask:

> What happens if another filling occupies the same position?

> Which substitutions preserve the result?

> What does every successful substitute share?

> Could an alien filling satisfy the same relation?

> Which property of the native filling turned out to be relationally necessary?

> Could that property itself be exposed as another open relation?

## Transport closure

Ask:

> Can the extracted relation regenerate the original implementation/domain when translated back?

> Which native role fails to return?

> What was removed that should not have been?

> How little must be restored?

## Current v2.0 root lowering

$$
\mathsf{Factor}
+
\mathsf{Vary}
+
\mathsf{Orient}
+
\mathsf{Ground}.
$$

---

# 16. Question-program family M — QUESTION / RECIPROCAL QUESTION

The harness must inquire into its own question set.

## Forward audit

Ask:

> Which alternatives did this question separate?

> Which alternatives did it leave merged?

> Which answer made another question possible?

> Which answer made another question unnecessary?

## Reciprocal audit

Ask:

> Would another question induce the same consequential separation?

> Which cheaper question produces the same consequential partition?

> Which stronger question separates something additional?

> Does that additional distinction matter?

> Which question exposes a distinction unavailable elsewhere?

> Which question presupposes a relation not yet established?

> What more open question survives if that presupposition is removed?

## Removal test

For candidate question \(q\), ask whether removing it changes:

1. the joint discrimination kernel;
2. the reachable condition/refinement paths;
3. answer-dependent question unlocks;
4. required actual discharge;
5. protected regeneration/reopening capability.

If none changes, \(q\) is a candidate for folding from the active harness basis.

---

# 17. Reciprocal closure rule

Every consequential forward movement must have a paired challenge.

Use the following provisional correspondence:

| Forward movement | Reciprocal challenge                                  |
| ---------------- | ----------------------------------------------------- |
| expose/open      | what remains unresolved or can be unbound again?      |
| add condition    | which removal reopens the excluded field?             |
| distinguish      | can the distinction be merged without protected loss? |
| orient forward   | what remains compatible backward?                     |
| factor path      | can the mediator be bypassed or replaced?             |
| compose          | does decomposition regenerate the composition?        |
| establish order  | what changes under reordering?                        |
| support          | what defeats or survives withholding that support?    |
| repair           | what rebreaks it?                                     |
| abstraction      | can it regenerate the source?                         |
| fold             | what future continuation reopens it?                  |
| answer           | what new question is forced by its residual?          |
| question         | what difference can its answer actually make?         |
| distinction      | what question makes that distinction consequential?   |

The harness should prohibit a terminal declaration of closure when the corresponding reciprocal challenge has not at least been represented or explicitly blocked.

---

# 18. Principal composed rhythms

The program families above should be composed into a small number of recurring rhythms.

## 18.1 Discovery rhythm

$$
\boxed{
\mathsf{FRAME}
\to
\mathsf{OPEN}
\to
\mathsf{SEPARATE}
\to
\mathsf{DISCHARGE}
\to
\mathsf{RESOLVE}.
}
$$

Use when the current object/relation is poorly represented.

Typical questions:

> What is there?

> What relation remains open?

> Which possibilities survive?

> Which protected distinction separates them?

> What test returns that distinction?

> What did the test actually establish?

---

## 18.2 Boundary rhythm

$$
\boxed{
\mathsf{STRONG\ CONTRAST}
\to
\mathsf{DISCHARGE}
\to
\mathsf{SUBTRACT}
\to
\mathsf{BOUNDARY}.
}
$$

Use for debugging, necessity, sufficiency, invariance, applicability, and representation boundaries.

Typical questions:

> What extreme valid case clearly crosses the boundary?

> What remains when differences are removed?

> Which distinction survives last?

> What happens when that distinction is removed?

---

## 18.3 Determination / Why rhythm

$$
\boxed{
\mathsf{RETURN\ AUDIT}
\to
\mathsf{FOIL}
\to
\mathsf{SEPARATOR}
\to
\mathsf{FACTOR}
\to
\mathsf{RELEASE}
\to
\mathsf{GROUND}.
}
$$

Use whenever an answer begins to look determined.

Typical questions:

> What exactly returned?

> What remains live?

> Why this rather than which live alternative?

> Which relation separates them?

> Could the alleged reason disappear while the result remained?

> Could it remain while the result failed?

> What independent return supports what survives?

---

## 18.4 Path rhythm

$$
\boxed{
\mathsf{FACTOR}
\to
\mathsf{COMPOSE}
\to
\mathsf{REVERSE}
\to
\mathsf{REORDER}
\to
\mathsf{COMPARE\ HISTORY}.
}
$$

Use for dependency, architecture, data flow, execution, and mechanism questions.

Typical questions:

> Through what does this propagate?

> Can another path reach the same endpoint?

> What remains when read backward?

> What changes if order changes?

> Could the endpoint remain while history differed?

---

## 18.5 Failure / repair rhythm

$$
\boxed{
\mathsf{FAIL}
\to
\mathsf{MAXIMAL\ BREAKER}
\to
\mathsf{SUBTRACT}
\to
\mathsf{OVER\!-\!REPAIR}
\to
\mathsf{SUBTRACT}
\to
\mathsf{REBREAK}.
}
$$

This should be the default coding-debug loop.

---

## 18.6 Compression rhythm

$$
\boxed{
\mathsf{COMPARE}
\to
\mathsf{MERGE}
\to
\mathsf{REGENERATE}
\to
\mathsf{CHALLENGE}
\to
\mathsf{REOPEN}.
}
$$

Typical questions:

> Which differences have no protected consequence?

> What can be merged?

> Can all required behavior be regenerated?

> What future question would distinguish the merged cases?

> What must remain to reopen them?

---

## 18.7 Question-basis rhythm

$$
\boxed{
\mathsf{ASK}
\to
\mathsf{MEASURE\ SPLIT}
\to
\mathsf{COMPARE\ QUESTIONS}
\to
\mathsf{REMOVE}
\to
\mathsf{RETEST}.
}
$$

Typical questions:

> What did this question distinguish?

> Which other question exposes the same distinction?

> Which cheaper question has the same effect?

> Which stronger question adds a consequential distinction?

> What changes if this question is removed?

---

# 19. Residual-driven scheduling

Do not run the rhythms cyclically.

Select them from the current residual.

| Residual                                    | Preferred rhythm                                           |
| ------------------------------------------- | ---------------------------------------------------------- |
| relational roles unclear                    | FRAME                                                      |
| open relation has multiple fillings         | DISCOVERY                                                  |
| protectedly different fillings still merged | SEPARATE                                                   |
| necessity/sufficiency/forcing claim         | BOUNDARY + FORCE/RELEASE                                   |
| concrete failure                            | FAILURE/REPAIR                                             |
| endpoint known but mechanism/path unclear   | PATH                                                       |
| same endpoint may hide different history    | SUCCESSION/REORDER                                         |
| relation generated but weakly supported     | GROUND/DEFEAT                                              |
| explanation proposed                        | DETERMINATION/WHY                                          |
| representation too detailed                 | COMPRESSION                                                |
| representation merges protected cases       | SEPARATE + REOPEN                                          |
| domain/implementation substitution proposed | TRANSPORT/REGENERATE                                       |
| too many tests/questions                    | QUESTION-BASIS                                             |
| no executable separator available           | representation/probe/method invention; otherwise `Unknown` |
| fold becomes invalid under new continuation | REOPEN                                                     |
| scope/applicability changes                 | REFRAME/REBIND                                             |

---

# 20. Coverage criterion for a question subset

A useful question subset is not one containing many phrasings.

A subset is inadequate if every question changes the same inquiry coordinate.

For a substantial successor-construction pass, the active question program should cover, where relevant:

$$
\boxed{
\begin{array}{c}
\text{ADMISSIBILITY}\\
\text{DISCRIMINATION}\\
\text{PATH / DIRECTION / ORDER}\\
\text{ACTUALITY / SUPPORT}\\
\text{REPRESENTATION / REGENERATION}.
\end{array}
}
$$

A subset that only asks many forms of:

> What else could be true?

covers admissibility poorly and may not touch discrimination, path, evidence, or reopening.

A subset that only asks:

> What test distinguishes these?

can over-refine discrimination without testing whether the distinguishing relation is necessary.

A subset that only asks “why?” may construct explanation without obtaining actual returns.

A subset that only breaks things may locate boundaries without establishing support.

The harness therefore seeks **relational span**, not prompt count.

---

# 21. Redundancy criterion

Questions can be redundant at several levels.

### Surface redundancy

Two prose questions elaborate to the same typed open relation.

### Discrimination redundancy

Two questions induce the same relevant partition/kernel.

### Field redundancy

Their possible answers produce the same relevant condition contraction.

### Succession redundancy

They unlock the same protected next-question family.

### Method/effect distinction

Questions with the same semantic partition may still require different actual probes, evidence routes, cost, authority, provenance, or histories.

Therefore:

$$
\boxed{
\text{same wording}
\neq
\text{same question}
}
$$

and:

$$
\boxed{
\text{same discrimination kernel}
\neq
\text{same executable inquiry program}.
}
$$

Only fold questions after checking the protected behavior of the question program, not merely its wording.

---

# 22. Relation between questions and methods

Questions specify what relation must be discharged.

Methods specify how a return may be obtained.

Typical pairings:

| Inquiry program        | Candidate methods                                                              |
| ---------------------- | ------------------------------------------------------------------------------ |
| FRAME                  | repository inspection, static analysis, schema extraction                      |
| SEPARATE               | differential testing, property testing, equivalence checking                   |
| FORCE/RELEASE          | symbolic execution, SAT/SMT, theorem proving, countermodel search              |
| MAXIMAL BREAK/SUBTRACT | fuzzing, stress testing, delta debugging, group testing                        |
| FACTOR/PATH            | call graph, data-flow analysis, program slicing, provenance                    |
| SUCCESSION/REORDER     | tracing, concurrency testing, partial-order analysis                           |
| GROUND/DEFEAT          | runtime probe, independent test, proof checker, external source                |
| REPAIR/REBREAK         | patch generation, regression testing, mutation testing                         |
| FOLD/REOPEN            | minimization, bisimulation/equivalence analysis, replay/regeneration           |
| TRANSPORT              | translation validation, alternate implementation/backend, cross-domain binding |
| QUESTION-BASIS         | test-set minimization, kernel/partition comparison, coverage analysis          |

The LLM may propose a method.

The typed question determines what the method has to return.

The method does not redefine the question.

---

# 23. Default successor-construction coding rhythm

When no more specific residual dictates otherwise, use:

$$
\boxed{
\begin{array}{c}
\textbf{WHAT IS HERE AND HOW IS IT RELATED?}\\
\downarrow\\
\textbf{WHAT POSITION IS ACTUALLY OPEN?}\\
\downarrow\\
\textbf{WHICH LIVE ALTERNATIVES MATTER DIFFERENTLY?}\\
\downarrow\\
\textbf{WHAT LARGE VALID CONTRAST MAKES THAT DIFFERENCE OBVIOUS?}\\
\downarrow\\
\textbf{WHAT ACTUAL TEST / PROOF / RETURN DISCRIMINATES IT?}\\
\downarrow\\
\textbf{WHAT DID THAT RETURN ACTUALLY CLOSE?}\\
\downarrow\\
\textbf{WHY THIS RATHER THAN THE STRONGEST LIVE FOIL?}\\
\downarrow\\
\textbf{WHAT CONDITION / PATH / DISTINCTION CARRIES THE DIFFERENCE?}\\
\downarrow\\
\textbf{CAN THAT ALLEGED CARRIER BE REMOVED, REVERSED, REPLACED, OR BYPASSED?}\\
\downarrow\\
\textbf{HOW MUCH OF THE LARGE CONTRAST CAN BE SUBTRACTED?}\\
\downarrow\\
\textbf{WHAT SURVIVES AT THE BOUNDARY?}\\
\downarrow\\
\textbf{WHAT INDEPENDENT RETURN GROUNDS IT?}\\
\downarrow\\
\textbf{WHAT CAN NOW BE FOLDED WITHOUT PROTECTED LOSS?}\\
\downarrow\\
\textbf{WHAT FUTURE QUESTION WOULD REOPEN THAT FOLD?}\\
\downarrow\\
\textbf{WHAT RESIDUAL POSITION IS NOW OPEN?}
\end{array}
}
$$

Then repeat from the newly opened position.

---

# 24. Short operational form for an agent

At every substantial construction step:

**Establish**

* What is present?
* What roles and relations does it instantiate?
* What is actually available versus inferred?

**Open**

* What consequential relation remains unresolved?
* What admissible fillings remain?

**Separate**

* Which surviving fillings are protectedly different?
* What question/test distinguishes them?

**Break strongly**

* What large valid contrast puts the alternatives clearly on opposite sides?

**Actualize**

* Which method can obtain an independent return?
* What actually returned?

**Resolve**

* What did the return establish?
* What remains possible, unresolved, excluded, or unknown?
* Does one protected class remain?

**Reciprocate**

* Why this rather than the strongest live alternative?
* What carries the difference?
* What happens backward?
* What happens if the carrier is removed?

**Subtract**

* Which parts of the large contrast can disappear?
* Which distinction survives last?
* Does removing it reopen the alternative?

**Ground**

* What independently supports the surviving relation?
* What would defeat it?

**Fold**

* What distinction/question/representation can disappear without protected loss?
* Can the required capability regenerate?

**Reopen**

* What future continuation, context, representation, or return would invalidate the fold?
* What must remain recoverable?

**Continue**

* What is now the strongest unresolved relational difference?
* Which question actually separates it?

---

# 25. Governing harness law

The Coding and Reciprocal Why corpora should therefore be used according to:

$$
\boxed{
\textbf{
NO QUESTION IS SELECTED BECAUSE ITS WORDING IS DIFFERENT.
A QUESTION IS SELECTED BECAUSE IT PERFORMS A NEEDED RELATIONAL MOVE.
}
}
$$

And:

$$
\boxed{
\textbf{
NO CONSEQUENTIAL CLOSURE IS COMPLETE UNTIL ITS RECIPROCAL
QUESTION HAS BEEN REPRESENTED, ATTEMPTED, OR EXPLICITLY BLOCKED.
}
}
$$

The desired oscillation is:

$$
\boxed{
\begin{array}{ccc}
\text{CONSTRAIN}
&\rightleftarrows&
\text{RELEASE}
\\[2mm]
\text{DISTINGUISH}
&\rightleftarrows&
\text{COARSEN}
\\[2mm]
\text{FACTOR}
&\rightleftarrows&
\text{RECOMPOSE / BYPASS}
\\[2mm]
\text{ORIENT}
&\rightleftarrows&
\text{REVERSE}
\\[2mm]
\text{GROUND}
&\rightleftarrows&
\text{DEFEAT / WITHHOLD}
\\[2mm]
\text{FOLD}
&\rightleftarrows&
\text{REOPEN}.
\end{array}
}
$$

The harness uses these reciprocal motions before their final successor formalization.

The formal successor is then allowed to prove that some are reducible, rename them, merge them, split them, or reject the proposed decomposition.

The construction harness must not prejudge that result.

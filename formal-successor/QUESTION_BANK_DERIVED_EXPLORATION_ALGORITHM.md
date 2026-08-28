# Inquiry Calculus — Question-Bank-Derived Exploration Algorithm

## Formalization obtained by running the full question corpus against the algorithmic problem

### Status

Provisional computational formalization for constructing the formal successor to Inquiry Calculus v2.0.

This document does not declare the successor's primitive basis.

It records what survives after using the complete question corpus to inquire into the problem:

> How should a reasoning process explore a large relational space, discover shared consequential boundaries without pairwise enumeration, machine-check its conclusions, retain only what is currently forced, reuse discovered conditions across overlapping residuals, compress recurring structure, and reopen that compression when later inquiry distinguishes what had been merged?

---

# 1. The answer forced by the full bank

The general algorithm is not fundamentally:

$$
\text{search}
\to
\text{answer}.
$$

Nor:

$$
\text{generate candidate}
\to
\text{test candidate}.
$$

Nor:

$$
\text{compare every object to every other object}.
$$

The full bank forces the following structure:

$$
\boxed{
\begin{array}{c}
\text{OPEN A TYPED RELATION}\\
\downarrow\\
\text{COMPUTE / REPRESENT ITS LIVE COMPLETION FIELD}\\
\downarrow\\
\text{EXPAND FAR ENOUGH TO OBTAIN CONSEQUENTIALLY DIFFERENT REGIONS}\\
\downarrow\\
\text{ASK A QUESTION THAT SEPARATES THOSE REGIONS}\\
\downarrow\\
\text{OBTAIN AN ACTUAL / PURE / CHECKED RETURN}\\
\downarrow\\
\text{CONTRACT THE FIELD BY WHAT THE RETURN ACTUALLY ESTABLISHES}\\
\downarrow\\
\text{LOCALIZE THE CONDITION THAT CARRIED THE SEPARATION}\\
\downarrow\\
\text{RATCHET ONLY THAT PROVED RELATION}\\
\downarrow\\
\text{INDEX EVERY RESIDUAL THAT SHARES THE CONDITION}\\
\downarrow\\
\text{PROPAGATE THE RESULT THROUGH THOSE RESIDUALS}\\
\downarrow\\
\text{FACTOR RECURRING CONDITION / BREAKER PATTERNS}\\
\downarrow\\
\text{FOLD REDUNDANT DISTINCTIONS}\\
\downarrow\\
\text{RETAIN THE QUESTION THAT WOULD REOPEN EACH FOLD}\\
\downarrow\\
\text{EXPAND AGAIN FROM THE CHANGED RELATIONAL MAP}.
\end{array}
}
$$

In short:

$$
\boxed{
\mathsf{OPEN}
\to
\mathsf{EXPAND}
\to
\mathsf{COLLIDE}
\to
\mathsf{SEPARATE}
\to
\mathsf{RETURN}
\to
\mathsf{CONTRACT}
\to
\mathsf{RATCHET}
\to
\mathsf{PROPAGATE}
\to
\mathsf{FACTOR}
\to
\mathsf{FOLD/REOPEN}.
}
$$

These are computational phases, not proposed semantic primitives.

The central asymmetry is:

$$
\boxed{
\textbf{SEARCH WIDELY; COMMIT NARROWLY.}
}
$$

---

# 2. What the general questions establish

The first thousand lines ask, repeatedly and from several orientations:

* what is present and absent;
* what can and cannot be distinguished;
* what overlaps or contains;
* where a comparison applies;
* what may vary;
* what changes only jointly;
* what follows forward and remains compatible backward;
* what persists;
* what can be reconstructed;
* where a boundary lies;
* what happens under removal, addition, substitution and reordering;
* what is merely possible versus forced;
* what is direct versus inferred;
* what mediates a relation;
* which apparently contradictory observations can jointly hold;
* which questions are redundant;
* what can be compressed;
* what must be recoverable;
* and what would reopen a compressed distinction.
  The answers are not thousands of independent facts.

They force the following typed objects.

---

# 3. Binding and typed field

For each applicable carrier \(X\), let:

$$
\Theta
$$

denote the complete current binding needed to interpret the inquiry:

$$
\Theta
=
(
X,
\text{scope},
\text{applicability},
\text{grain},
\text{representation},
\text{authority},
\text{coverage},
\ldots
).
$$

Let:

$$
\mathcal C_{\Theta,X}
\subseteq
\{\,\rho:X\to\mathsf{Prop}\,\}
$$

be the represented applicable condition family.

For:

$$
W\subseteq\mathcal C_{\Theta,X},
$$

define:

$$
\boxed{
\Sol_\Theta^X(W)
=
\{
x\in X:
\forall\rho\in W,\rho(x)
\}.
}
$$

This answers the recurring questions:

> What remains possible?

> What disappears when a condition is added?

> What returns when one is removed?

> Which combinations admit no alternative?

> What becomes possible only after release?

The reconceptualization already identifies this as condition–field reciprocity: retained conditions leave positions open and lawful completions form the resulting field.

---

# 4. Applicability and absence

Questions such as:

> What is not present?

> Where does failure to apply resemble absence?

> Which apparently negative case lies outside the comparison?

have one exact answer discipline.

For a relation:

$$
R:X\rightsquigarrow Y,
$$

failure to produce \(R(x,y)\) has at least three different statuses:

$$
\boxed{
\begin{aligned}
&x\notin App_\Theta(R)
&&\text{inapplicable},\\
&x\in App_\Theta(R)\land \neg R(x,y)
&&\text{negative under represented coverage},\\
&x\in App_\Theta(R)\land Status(R,x,y)=Unknown
&&\text{unresolved}.
\end{aligned}
}
$$

Therefore:

$$
\boxed{
\text{NOT FOUND}
\neq
\text{ABSENT}
\neq
\text{INAPPLICABLE}
\neq
\text{IMPOSSIBLE}.
}
$$

v2.0 already explicitly requires that failure to find a separator, path, counterexample, proof or useful question not establish equivalence, impossibility, necessity or irrelevance.

---

# 5. Questions and completion fields

A question is not a string.

Given a typed relation schema:

$$
R\hookrightarrow
X_1\times\cdots\times X_n
$$

and partial binding \(\beta\), expose positions \(I\):

$$
\boxed{
q=?_I R[\beta].
}
$$

Its answer is initially not one value.

It is the completion fiber:

$$
\boxed{
\Fib_I(R\mid\beta).
}
$$

This answers all bank questions of the form:

> What fills this role?

> Does any admissible answer exist?

> Do several remain?

> Which position may remain open?

> Which answer is uniquely determined?

The successor discipline already states that every introduced form should fill a previously typed open port:

$$
Intro(\Delta,z),
\qquad
?z[Intro(\Delta,z)].
$$

If the completion fiber contains one literal value, literal forcing is available; if it contains one protected class, protected determination is available; if several classes survive, a separator obligation is generated; if it is exactly empty, a conflict/impossibility residual is generated; under incomplete coverage the result is `Unknown`.

---

# 6. Distinguishability

For question \(q\), let:

$$
C_q\subseteq X_q\times A(q)
$$

be its typed answer-compatibility relation.

Define:

$$
\boxed{
\sigma_q(x)
=
\{a\in A(q):C_q(x,a)\}.
}
$$

Then:

$$
\boxed{
x\sim_qy
\iff
\sigma_q(x)=\sigma_q(y).
}
$$

This answers:

> What can this question distinguish?

> Which cases does it merge?

> Which questions expose the same distinction?

> Which sharper question splits them?

Questions therefore induce partitions rather than merely request sentences.

The reconceptualization explicitly derives:

$$
q\longmapsto\sim_q,
$$

and the reciprocal separator family from consequential difference.

---

# 7. Protected consequence

For protected use \(h\), let:

$$
P_h:X\rightsquigarrow C_h.
$$

Define its relational profile:

$$
\Prof_{P_h}(x)
=
\{c:P_h(x,c)\}.
$$

Then:

$$
\boxed{
x\equiv_{\mathcal H}y
\iff
\forall h\in\mathcal H:
\Prof_{P_h}(x)=\Prof_{P_h}(y).
}
$$

And:

$$
\boxed{
Sep_{\mathcal H}(x,y)
=
\{
h\in\mathcal H:
\Prof_{P_h}(x)\neq\Prof_{P_h}(y)
\}.
}
$$

Hence:

$$
\boxed{
x\not\equiv_{\mathcal H}y
\iff
Sep_{\mathcal H}(x,y)\neq\varnothing.
}
$$

This answers the enormous family:

> Which differences actually matter?

> Which distinctions are representational only?

> Which apparent sameness hides later divergence?

> Which continuation makes the distinction consequential?

The stronger conception already states the reciprocal law:

> a distinction is consequential only through relations capable of exposing it, and an interrogation is consequential only through distinctions its returns can make.

---

# 8. The two independent refinement directions

The full bank repeatedly asks two non-equivalent kinds of question.

One removes possibilities.

The other separates possibilities.

They must remain distinct.

## 8.1 Constraint refinement

Adding:

$$
\rho
$$

gives:

$$
\boxed{
\Sol(W\cup\{\rho\})
\subseteq
\Sol(W).
}
$$

Removing gives the reciprocal expansion:

$$
\boxed{
\Sol(W)
\subseteq
\Sol(W\setminus\{\rho\}).
}
$$

## 8.2 Discrimination refinement

For active question family \(Q\):

$$
\boxed{
\equiv_Q
=
\bigcap_{q\in Q}\sim_q.
}
$$

Adding \(q\):

$$
\boxed{
\equiv_{Q\cup\{q\}}
=
\equiv_Q\cap\sim_q.
}
$$

One operation changes what remains possible.

The other changes what the system is able to tell apart.

This distinction answers hundreds of questions in the bank that alternate between:

> Which condition removes an alternative?

and:

> Which observation distinguishes two alternatives?

---

# 9. Determination

These two motions meet in:

$$
\boxed{
Determines_{\mathcal H}(W,x)
\iff
\Sol_\Theta(W)/\equiv_{\mathcal H}
=
\{[x]_{\mathcal H}\}.
}
$$

This answers:

> When is an answer actually determined?

Not when only one literal description happens to have been generated.

Not when the model prefers one answer.

Not when no alternative has been found.

Determination means all admissible fillings left by the current condition web occupy one protected class.

---

# 10. Possibility, permission, forcing, necessity

Define:

$$
\boxed{
W\models_\Theta\phi
\iff
\Sol_\Theta(W)
\subseteq
\llbracket\phi\rrbracket_\Theta.
}
$$

Nonvacuous forcing:

$$
\boxed{
W\Vdash_\Theta\phi
\iff
\Sol_\Theta(W)\neq\varnothing
\land
W\models_\Theta\phi.
}
$$

Permission:

$$
\boxed{
Permits_\Theta(W,\phi)
\iff
\Sol_\Theta(W)
\cap
\llbracket\phi\rrbracket_\Theta
\neq\varnothing.
}
$$

Thus:

$$
\boxed{
\text{possible}
\neq
\text{permitted}
\neq
\text{forced}.
}
$$

For a represented condition \(\rho\in W\):

$$
\boxed{
Needed_\Theta(\rho;W,\phi)
\iff
W\Vdash_\Theta\phi
\land
(W\setminus\{\rho\})\not\Vdash_\Theta\phi.
}
$$

The successor exploration already derives exactly this removal-based necessity relation.

---

# 11. Minimal forcing webs

The bank repeatedly asks:

> Which smallest set leaves no alternative?

> Could another sufficient route exist?

> What do all sufficient routes share?

Therefore there is generally no unique support set.

Define:

$$
\boxed{
ForceBasis_\Theta(\phi)
=
\Min_{\subseteq}
\{
W:
W\Vdash_\Theta\phi
\}.
}
$$

This is a set of inclusion-minimal webs.

Not:

$$
\arg\min W
$$

unless a unique order and optimum have been independently supplied.

The reciprocal relation is:

$$
\boxed{
Force_\Theta
\subseteq
\mathcal P(\mathcal C_\Theta)\times\Phi,
}
$$

with:

$$
Force_\Theta(W,\phi)
\iff
W\Vdash_\Theta\phi.
$$

Therefore:

$$
Force_\Theta[W]
=
\{\phi:W\Vdash_\Theta\phi\},
$$

and:

$$
Force_\Theta^\smile[\phi]
=
\{W:W\Vdash_\Theta\phi\}.
$$

That is the formal answer to:

> Given these conditions, what follows?

and its reciprocal:

> What conditions force this result?

---

# 12. Boundary questions

The bank repeatedly asks:

> What is the largest variation on one side?

> What is the smallest variation crossing the boundary?

> What happens under an extreme contrast?

> What survives while the contrast is reduced?

The exact answer is conditional on the structure of the variation space.

Let:

$$
V_\Theta(x)
$$

be the admitted variations of \(x\).

Let protected capability be:

$$
\kappa:X\to K.
$$

A breaker is:

$$
\boxed{
Breaker_\kappa(x,x')
\iff
x'\in V_\Theta(x)
\land
\kappa(x')\not\approx_\kappa\kappa(x).
}
$$

The algorithm should first find any deliberately decisive:

$$
x^-,
x^+
$$

known to occupy different consequential cells.

Then localize the boundary between them.

This is why the bank's maximal-contrast questions are not rhetorical: it explicitly asks what becomes obvious under an extreme contrast, what survives as the contrast is reduced, and whether approaching from opposite sides exposes the same condition.

---

# 13. Which localization algorithm?

The full bank asks which established procedures solve each kind of residual.

The answer is **method dispatch by relational shape**.

There is no single universal localizer.

## Ordered parameter

If the variation space supplies a total or monotone order and a known bracket:

$$
x^-<x^+,
$$

use bisection/generalized binary search.

## Decomposable change set

If a known failure-inducing difference decomposes into removable pieces:

$$
D=\{d_1,\dots,d_n\},
$$

use delta debugging.

Delta debugging explicitly starts from a failing test and simplifies/isolate the circumstances that induce failure rather than searching one tiny perturbation at a time.

## Conjunctive overconstraint

If:

$$
\Sol(W)=\varnothing
$$

and \(W\) is a finite decomposable condition set, use a conflict extractor such as QuickXplain or MUS search.

QuickXplain was explicitly designed to compute preferred explanations/relaxations of over-constrained systems using a generic consistency checker.

## Competing diagnoses

If several explanations survive a model/observation discrepancy, use model-based diagnosis and sequential discrimination.

Reiter's framework computes diagnoses from conflicts; GDE represents candidates using minimal violated-assumption sets, updates them incrementally, and proposes measurements that discriminate competing diagnoses.

## Coarse representation

If the current representation admits a spurious counterexample or merges cases requiring distinction, use CEGAR-style refinement:

$$
\text{abstract}
\to
\text{counterexample}
\to
\text{check}
\to
\text{refine}.
$$

CEGAR was explicitly introduced to refine an abstraction based on analysis of spurious counterexamples while keeping the abstract state space small.

## Candidate construction

If the problem is synthesis:

$$
\text{candidate}
\to
\text{validator}
\to
\text{counterexample}
\to
\text{candidate refinement},
$$

use a CEGIS-style loop. CEGIS combines synthesis with a validation procedure that generates counterexample inputs constraining subsequent candidates.

## Finite behavioral equivalence

If the problem is maintaining equivalence classes under new discriminators, use partition refinement.

Paige–Tarjan gives an efficient relational coarsest-partition algorithm rather than comparing every pair independently.

## Unknown state-machine behavior

If the underlying object is a regular transition behavior with membership/equivalence-query access, Angluin's \(L^\ast\)-style learning is a direct specialization: membership questions plus counterexamples construct the discriminating observations needed to identify a minimal DFA under its oracle assumptions.

## Simultaneous incompatible contexts

If many condition sets must remain alive simultaneously, use the surviving computational relation of an ATMS: conclusions indexed by assumption sets, with inconsistent contexts retained without destroying all alternative contexts. ATMS was designed specifically to allow simultaneous exploration of multiple potential solutions and efficient context switching.

---

# 14. The closest established algorithm to the shared-condition idea

The strongest source-domain answer is **Formal Concept Analysis attribute exploration**.

The question bank explicitly asks which mature practices repeatedly distinguish possibilities and which collapse differences that do not matter.

Attribute exploration starts from a set of attributes and repeatedly asks whether proposed implications hold. When an implication fails, the expert supplies a counterexample; the process constructs structured knowledge/implication bases rather than storing every pairwise object relation. Modern treatments explicitly describe attribute exploration as interactive knowledge acquisition through queries to an expert and discuss incomplete or imprecise variants.

The relation to the calculus is:

$$
\begin{array}{c|c}
\text{Attribute exploration} & \text{Inquiry calculus}\\
\hline
\text{object} & \text{case / construction / residual}\\
\text{attribute} & \text{typed condition / discriminator}\\
\text{incidence} & \text{condition relevant to case}\\
A\Rightarrow B & \text{conditional entailment}\\
\text{expert query} & \text{question discharge}\\
\text{counterexample} & \text{breaker}\\
\text{implication basis} & \text{factorized condition basis}
\end{array}
$$

But the calculus must generalize it because:

* conditions are typed relations, not merely Boolean attributes;
* an answer may be `Unknown`;
* the oracle may be unavailable;
* questions may require different discharge methods;
* actual returns differ from generated answers;
* paths, order and provenance can matter;
* folds must carry reopening conditions.

Thus attribute exploration is a method home, not a semantic foundation.

---

# 15. Shared residual conditions

Now formalize the user's central computational insight.

Let:

$$
\mathcal R
$$

be current residuals.

Let:

$$
\mathcal B
$$

be normalized consequential boundary features.

A boundary feature may be:

* condition;
* separator;
* breaker;
* conflict core;
* failed-fold witness;
* noncommutation witness;
* missing prerequisite;
* support dependency;
* applicability boundary;
* representation gap;
* path distinction.

Define typed incidence:

$$
\boxed{
I_\Theta
\subseteq
\mathcal R\times\mathcal B.
}
$$

Meaning:

$$
I_\Theta(\Delta,b)
$$

iff \(b\) has been established as consequentially relevant to residual \(\Delta\) under the compatible binding.

This is not lexical matching.

Two appearances of “same condition” are one boundary coordinate only if their typed relation, scope, applicability and orientation agree or an established transport theorem identifies them.

---

# 16. The shaped hole

Suppose represented construction \(z\) is removed.

Do not erase its incidence.

Open its former position:

$$
\boxed{
\Delta_z
=
Ablate_z(M).
}
$$

The completion field is:

$$
\boxed{
Regen_z(M)
=
\Ans(?z[Intro(\Delta_z,z)]).
}
$$

If:

$$
Regen_z(M)/\equiv_{\mathcal H}
=
\{[z]_{\mathcal H}\},
$$

then the remainder determines \(z\)'s protected role.

This is exactly the reconceptualization's reciprocal pair:

$$
\boxed{
\begin{aligned}
&\text{What does }z\text{ allow the structure to determine?}\\
&\text{What determines }z\text{ when }z\text{ itself is removed?}
\end{aligned}
}
$$

Two apparently distant constructions become related when removing them creates completion fields constrained by the same boundary relation.

Thus:

$$
z_i\to\Delta_i
\leftarrow b\rightarrow
\Delta_j\leftarrow z_j
$$

can replace a dedicated pairwise relation:

$$
z_iRz_j.
$$

---

# 17. Residual overlap

Define:

$$
Feat(\Delta)
=
\{b\in\mathcal B:I(\Delta,b)\}.
$$

Then:

$$
\boxed{
Shared(\Delta_i,\Delta_j)
=
Feat(\Delta_i)\cap Feat(\Delta_j).
}
$$

But set overlap only proposes correspondence.

It does not prove it.

The next question is:

$$
?b[
b\in Shared(\Delta_i,\Delta_j)
\land
SameUse_\Theta(b_i,b_j)
].
$$

Repeated verified overlap creates a candidate relational basin.

---

# 18. Demand-driven concept closure

The useful FCA relation can be retained without enumerating an entire concept lattice.

For residual set:

$$
A\subseteq\mathcal R,
$$

define:

$$
A^\uparrow
=
\{
b:
\forall\Delta\in A,\ I(\Delta,b)
\}.
$$

For feature set:

$$
B\subseteq\mathcal B,
$$

define:

$$
B^\downarrow
=
\{
\Delta:
\forall b\in B,\ I(\Delta,b)
\}.
$$

Then:

$$
A^{\uparrow\downarrow}
$$

gives the current residual closure sharing all common boundary features.

Use this **on demand around active residuals**.

Do not enumerate the entire concept lattice: the number of formal concepts may itself be exponential.

The point is indexing and factorization, not replacing one combinatorial explosion with another.

---

# 19. Contradiction

The bank repeatedly asks:

> Can these independently obtained answers all hold together?

> Which apparent contradiction disappears after scope/direction/order/applicability is distinguished?

> Which contradiction remains?

The exact operation is:

Given condition webs \(W_1,W_2\),

$$
\boxed{
Joint_\Theta(W_1,W_2)
=
\Sol_\Theta(W_1\cup W_2).
}
$$

If:

$$
Joint_\Theta(W_1,W_2)=\varnothing
$$

under exact coverage, there is a real represented conflict.

Do not smooth it away.

Localize:

$$
C
\in
\Min_{\subseteq}
\{
C'\subseteq W_1\cup W_2:
\Sol(C')=\varnothing
\}.
$$

This conflict core becomes a reusable boundary coordinate.

CDCL provides the mature computational pattern: a conflict is analyzed into a learned constraint that applies elsewhere in the search rather than forcing rediscovery of the same conflict in every branch.

The calculus-native retained form need not be a Boolean clause.

It can be:

$$
\boxed{
Conflict_\Theta(C;\Gamma,E,U)
}
$$

with scope, evidence and reopening conditions.

---

# 20. Breakers

For proposed consequence:

$$
W\Vdash_\Theta\phi,
$$

a sufficiency breaker is:

$$
\boxed{
b\in
\Sol_\Theta(W)
\cap
\llbracket\neg\phi\rrbracket_\Theta.
}
$$

For proposed necessity of \(\rho\):

$$
\boxed{
b\in
\Sol_\Theta(W\setminus\{\rho\})
\cap
\llbracket\neg\phi\rrbracket_\Theta.
}
$$

For equivalence:

$$
\boxed{
x\equiv_Cy
\quad\land\quad
x\not\equiv_{\mathcal H}y
}
$$

is a representation/discrimination breaker.

For fold:

$$
\boxed{
c(x)=c(y)
\land
x\not\equiv_{\mathcal H}y
}
$$

is a fold breaker.

For claimed commutation:

$$
\boxed{
(S\circ R)(x)
\not\equiv_{\mathcal H}
(R\circ S)(x)
}
$$

is an order breaker, when both composites are well typed.

These breaker forms answer the bank's many apparently different “what case defeats this?” questions.

---

# 21. Reciprocal Why

The final ~1,500 lines are not another domain section.

They are the **reverse pass after a return**.

They begin by asking:

* what exactly returned;
* what is directly established;
* what remains possible;
* what remains unresolved;
* what was excluded;
* which alternatives remain;
* whether a meaningful why contrast exists;
* why this rather than a specified alternative.

This forces a contrastive rather than narrative explanation operator.

For returned answer \(a\) and admissible foil \(b\), define an explanation search as:

$$
\boxed{
?C[
C(a)\neq C(b)
].
}
$$

A candidate \(C\) is not yet an explanation.

It must survive:

### Necessity pressure

Remove it.

Does the foil re-enter?

### Sufficiency pressure

Keep it.

Can another result still occur?

### Path pressure

Through what relation does \(C\) affect the result?

### Alternative-account pressure

Can a different relation explain the same contrast?

### Grounding pressure

What independently supports \(C\)?

### Generalization pressure

Does \(C\) survive changed objects, context, representation, method and scale?

Therefore `WHY` is a macro:

$$
\boxed{
\text{RETURN AUDIT}
\to
\text{FOIL}
\to
\text{SEPARATOR}
\to
\text{PATH}
\to
\text{REMOVE}
\to
\text{COUNTERCASE}
\to
\text{GROUND}.
}
$$

---

# 22. Forward/backward questions

For:

$$
R:X\rightsquigarrow Y,
$$

forward image:

$$
R[x]
=
\{y:R(x,y)\}.
$$

Backward compatible sources:

$$
R^\smile[y]
=
\{x:R(x,y)\}.
$$

The bank's forward/backward questions therefore ask about these two fibers.

Do not infer:

$$
R^\smile=R^{-1}
$$

as executable inverse.

Do not infer uniqueness.

Do not infer explanatory symmetry.

The question:

> Does the result reconstruct the conditions uniquely?

is answered by:

$$
|R^\smile[y]/\equiv_{\mathcal H}|=1?
$$

The question:

> Do the conditions determine the result uniquely?

is answered by:

$$
|R[x]/\equiv_{\mathcal H}|=1?
$$

---

# 23. Paths and mediators

For:

$$
R:X\rightsquigarrow M,
\qquad
S:M\rightsquigarrow Y,
$$

the mediated relation is:

$$
S\circ R.
$$

The bank asks:

> Can another middle connect the same ends?

Answer:

$$
\boxed{
?M',R',S'[
S'\circ R'
\equiv_{\mathcal H}
S\circ R
].
}
$$

It then asks:

> What is lost when only endpoints remain?

Answer:

everything in the distinction between path profiles that does not factor through the endpoint relation.

Thus define protected path equivalence:

$$
p\equiv_{\mathcal H}^{path}p'
$$

only relative to explicit protected path/history observations.

Endpoint equality alone cannot prove it.

---

# 24. Order

For endorelations:

$$
R,S:X\rightsquigarrow X,
$$

commutation is:

$$
\boxed{
S\circ R
\equiv_{\mathcal H}^{Rel}
R\circ S.
}
$$

The bank asks:

> Which changes are harmless separately but consequential jointly?

This is not detectable by singleton ablation alone.

A joint interaction breaker is:

$$
\kappa(RS(x))
\neq
f(\kappa(Rx),\kappa(Sx))
$$

for whatever factorization \(f\) the independence claim requires.

No universal independence law is assumed.

---

# 25. Evidence

The engineering and Reciprocal Why sections repeatedly ask:

> What is directly available?

> What is inferred?

> Which support is independent?

> Which apparent agreement shares ancestry?

Answer by keeping:

$$
\boxed{
\text{Generation}
\neq
\text{Actuality}
\neq
\text{Interpretation}
\neq
\text{Check}
\neq
\text{Warrant}.
}
$$

A proof object establishes something like:

$$
\boxed{
d:
Deriv_{\mathfrak M,\Theta}(W,\phi),
}
$$

not an unconditioned truth object.

The reconceptualization explicitly recommends reading every theorem as conditional on its mathematical system, binding and complete hypotheses.

---

# 26. Research questions

The research section asks the same calculus questions about:

* source claims;
* definitions;
* measurements;
* populations;
* scales;
* methods;
* support ancestry;
* historical revision;
* domain concept networks.

The answer is not a new calculus.

It is a binding:

$$
\Theta_{\text{research}}
$$

in which:

* forms are claims, definitions, methods, measurements and source constructions;
* conditions include population, period, method, scale and measurement assumptions;
* probes include source retrieval, data analysis, experiment and proof;
* protected consequences include prediction, reconstruction, explanatory or methodological distinctions.

Thus the research section is a typed specialization of the same operators.

---

# 27. Cross-domain questions

The cross-domain section asks:

> Which established methods perform the same relational work?

> Which source properties belong only to the native objects?

> What remains when ontology is removed?

> Can the resulting question regenerate the source method?

> Can an alien domain fill the same roles?

These force a round-trip transport test.

For source binding \(\Theta_S\), target binding \(\Theta_T\), and extracted relation \(R\):

$$
F:
R_S\rightsquigarrow R_T,
\qquad
G:
R_T\rightsquigarrow R_S.
$$

The abstraction survives only to the extent that protected structure round-trips:

$$
\boxed{
G\circ F
\equiv_{\mathcal H_S}^{Rel}
id
}
$$

and, where claimed:

$$
F\circ G
\equiv_{\mathcal H_T}^{Rel}
id.
$$

Failure identifies exactly what source-specific relation was removed too aggressively.

---

# 28. Answers to the question bank's method-search questions

The bank asks which fields provide decomposition, composition, invariants, boundary finding, minimality, maximal contrast, recovery, compression, refinement, equivalence, necessity, sufficiency, robustness, adversarial tests, stopping criteria and next-question selection.

The strongest mapping is:

| Required relation                                     | Mature method homes                        |
| ----------------------------------------------------- | ------------------------------------------ |
| interactive condition-basis discovery                 | FCA attribute exploration                  |
| abstraction breaker/refinement                        | CEGAR                                      |
| candidate/refutation synthesis                        | CEGIS                                      |
| reduce known breaker                                  | delta debugging                            |
| minimal conflict / relaxation                         | QuickXplain, MUS/MCS                       |
| competing explanations and discriminating observation | Reiter diagnosis, GDE                      |
| simultaneous assumption contexts                      | ATMS                                       |
| conflict reuse                                        | CDCL / nogood learning                     |
| separate live hypotheses efficiently                  | generalized binary search, active learning |
| learn state distinctions from queries/counterexamples | Angluin \(L^\ast\)                         |
| maintain coarsest behavioral classes                  | partition refinement                       |
| representation abstraction                            | abstract interpretation                    |
| overlap/nonconfluence pressure                        | critical-pair/completion methods           |

No one row becomes a semantic primitive.

Methods are implementations of residual-discharge patterns.

---

# 29. The actual general algorithm

Let construction configuration be:

$$
\boxed{
\Sigma_t
=
\langle
\Theta_t,
\mathcal R_t,
\mathcal B_t,
I_t,
\Pi_t,
\mathcal E_t,
\mathcal M_t,
\mathcal Q_t
\rangle
}
$$

where:

* \(\Theta_t\): current binding;
* \(\mathcal R_t\): residual obligations;
* \(\mathcal B_t\): normalized learned boundary features;
* \(I_t\): residual–boundary incidence;
* \(\Pi_t\): current protected partitions;
* \(\mathcal E_t\): evidence/provenance;
* \(\mathcal M_t\): available methods;
* \(\mathcal Q_t\): current executable/productive question frontier.

A residual is:

$$
\boxed{
\Delta
=
\langle
O,
W,
F,
Q,
B^+,
B^-,
K,
X,
G,
\Gamma,
P,
U
\rangle
}
$$

with:

* \(O\): open typed relation;
* \(W\): condition web;
* \(F\): represented live completion field;
* \(Q\): current discriminators;
* \(B^+\): breakers;
* \(B^-\): survived contrasts/nonbreakers;
* \(K\): blockers;
* \(X\): conflicts;
* \(G\): representation/execution gaps;
* \(\Gamma\): coverage;
* \(P\): provenance;
* \(U\): reopening conditions.

---

# 30. Algorithm

```text
EXPLORE(Σ):

    select a live residual Δ

    1. BIND
       Verify its carrier, scope, applicability, grain,
       protected horizon, evidence and open relation.

    2. REUSE
       Query the residual/boundary index.
       Import every already-established condition whose
       typed key and applicability match Δ.

    3. EXPAND
       Construct broad admissible contrasts:
           removal
           substitution
           reversal
           strengthening / weakening
           alternate path
           alternate order
           joint variation
           alien representation
           alien domain
           hostile example

       Do not seek minimality yet.

    4. PARTITION
       Apply already-known discriminators to the expanded field.
       Maintain protected-equivalence blocks incrementally.

    5. COLLIDE
       Locate:
           success/failure split
           incompatible condition sets
           protected pair merged by representation
           failed fold
           noncommutation
           unsupported inference
           reconstruction ambiguity
           blocker

    6. SELECT QUESTION
       Choose a productive or required typed question.
       Prefer questions that:
           split large live blocks,
           touch conditions shared by many residuals,
           unblock many downstream obligations,
           test a widely reused fold,
           attack a poorly challenged relation.

    7. DISCHARGE
       Dispatch by residual shape:
           Pure derivation
           Lean/proof
           SAT/SMT
           CEGAR
           CEGIS
           delta debugging
           QuickXplain / MUS
           diagnosis
           active learning
           partition refinement
           runtime probe
           source retrieval
           other binding-native method

    8. PRESERVE RETURN
       Candidate != return != interpretation != check != warrant.

    9. RESOLVE
       Return one of:
           Supported
           Plural
           ExactEmpty
           Unsupported
           Unknown
           Blocked
           ResourceBounded

   10. RECIPROCATE
       Ask:
           What foil remains?
           What condition can be removed?
           What happens backward?
           Is there another path?
           Does order matter?
           What defeats support?
           What breaks the fold?

   11. LOCALIZE
       Now, and only now, reduce the demonstrated broad
       separator toward its consequential boundary.

   12. RATCHET
       Retain only the strongest relation actually forced
       by the checked return.

   13. LEARN BOUNDARY
       Normalize the breaker/condition/fold-breaker/conflict
       into a reusable typed boundary feature.

   14. PROPAGATE
       Through the reverse incidence index, revisit every
       residual that shares the newly learned boundary.

   15. FACTOR
       Detect repeated boundary patterns.
       Propose a shared relation/method/representation.

       Break it with cases drawn from different basins.

   16. FOLD
       Remove distinctions/questions/methods whose removal
       preserves protected behavior and regeneration.

   17. RETAIN REOPENING QUESTION
       Every accepted fold records what future condition or
       discriminator invalidates it.

   18. REBUILD FRONTIER
       Select the next live residual.

       Local closure never implies global exhaustion.
```

---

# 31. Query selection

The bank asks:

> Which observation distinguishes the most alternatives?

> Which question blocks the most downstream questions?

> Which difficult question would alter the whole map?

For finite current protected classes:

$$
\Pi_\Delta.
$$

A question \(q\) induces blocks:

$$
\Pi_{\Delta,q,a}.
$$

Define worst surviving block:

$$
\boxed{
Worst(q,\Delta)
=
\max_a
|\Pi_{\Delta,q,a}|.
}
$$

A balanced-separator preference minimizes this value.

Generalized binary search provides the mature analogue: if each selected query eliminates a fixed fraction of the viable hypothesis set, identification requires \(O(\log |H|)\) queries under the relevant structural assumptions.

But the calculus also cares about reuse across residuals.

Define:

$$
\boxed{
Leverage(q)
=
|
\{
\Delta'\in\mathcal R:
q\text{ can alter }\Delta'
\}
|.
}
$$

Selection should therefore use a partial order over at least:

$$
\boxed{
(
Worst,
-Leverage,
Cost,
Risk,
AuthorityDebt,
CoverageGain
).
}
$$

No global scalar is required.

---

# 32. Questions using “expected”

The bank asks for “highest expected discriminatory value.”

This is applicable only if a probability measure has been supplied.

If:

$$
\mu
$$

exists over live classes/answers, expected information gain may be used.

Otherwise the exact disposition is:

$$
\boxed{
Inapplicable(\text{missing probability model})
}
$$

for the word *expected*.

Use worst-case split or a Pareto frontier instead.

---

# 33. Questions using “smallest”, “largest”, “strongest”, “best”, or “cheapest”

These are not automatically meaningful.

`Smallest` requires a preorder:

$$
\preceq.
$$

`Cheapest` requires:

$$
cost:X\to C.
$$

`Strongest` requires an order of strength.

`Best` requires a declared consequence/preference relation.

Without one, the exact answer is:

$$
\boxed{
Inapplicable(\text{missing comparator}).
}
$$

If only inclusion is relevant:

$$
\Min_{\subseteq}
$$

returns a frontier, not necessarily one object.

This answers every bank question that otherwise risks silently importing a scalar preference.

---

# 34. Question disposition

Every surface question in the full corpus must elaborate to exactly one of:

$$
\boxed{
Disposition(q,\Delta)
\in
\{
Answered,
Productive,
Required,
Redundant,
Inapplicable,
Blocked,
Unknown
\}.
}
$$

### Answered

Existing admitted structure computes the answer.

### Productive

Different supported answers lead to protected-different continuations.

### Required

Some explicit Probe/Check/Warrant/support obligation must be discharged regardless of immediate branching value.

### Redundant

Another question induces the same protected question behavior.

### Inapplicable

The question has no meaning under the current binding.

Exact reasons include:

* unbound carrier;
* unbound referent;
* wrong type;
* no comparator;
* no cost model;
* no probability model;
* relation not applicable;
* operation undefined;
* no source/target binding for transport.

### Blocked

Meaningful, but the required capability is unavailable.

Examples:

* representation gap;
* method gap;
* tool gap;
* evidence gap;
* authority gap;
* environment gap;
* resource gap.

### Unknown

Lawfully attempted, but available coverage cannot classify the answer.

This is how the algorithm answers **every question** without fabricating a concrete object where no binding has supplied one.

---

# 35. Semantic redundancy of the full question bank

The bank itself demands that differently worded questions be collapsed when they expose the same relation.

Therefore the full corpus should not become thousands of runtime opcodes.

A question's canonical identity is determined by:

$$
\boxed{
(
\text{underlying typed relation},
\text{partial binding},
\text{exposed ports},
\text{scope},
\text{grain},
\text{discharge obligation}
).
}
$$

Two surface questions are semantically redundant if normalization yields the same typed question.

They are discriminator-redundant if they induce the same protected partition.

They are program-redundant only if their answer-conditioned continuations are also protected-equivalent.

Thus:

$$
\boxed{
\text{same partition}
\not\Rightarrow
\text{same executable inquiry occurrence}.
}
$$

---

# 36. Why pairwise mapping is unnecessary when factorization exists

Suppose there are \(n\) constructions.

A literal complete pairwise map may contain:

$$
\Theta(n^2)
$$

independent information.

No algorithm can generally avoid that output cost if the information really is independent.

But suppose the relations factor through \(k\) reusable boundary features:

$$
\mathcal B
=
\{b_1,\ldots,b_k\}.
$$

Represent only incidence:

$$
I\subseteq X\times\mathcal B.
$$

If average feature degree per construction is \(d\):

$$
|I|=O(nd).
$$

For bounded/sparse \(d\):

$$
\boxed{
|I|=O(n).
}
$$

A construction receives a signature:

$$
\boxed{
Sig_\mathcal B(x)
=
(
Prof_{b_1}(x),\ldots,Prof_{b_k}(x)
).
}
$$

Its relation to another construction is often recoverable from shared/divergent coordinates rather than a dedicated edge.

---

# 37. When \(O(\log n)\) traversal is justified

If the learned discriminator hierarchy repeatedly reduces a live candidate class by a fixed fraction:

$$
|\Pi_{t+1}|
\le
\alpha|\Pi_t|,
\qquad
0<\alpha<1,
$$

then:

$$
|\Pi_k|
\le
\alpha^kN.
$$

To reach one class:

$$
\alpha^kN\le1.
$$

Hence:

$$
\boxed{
k
\ge
\frac{\log N}{-\log\alpha}.
}
$$

For balanced binary splits:

$$
\alpha=\frac12,
$$

so:

$$
\boxed{
k=O(\log N).
}
$$

That is the exact condition under which the user's proposed exponential traversal improvement is real.

It is not universal.

The algorithm's job is to **learn the discriminator geometry that makes this condition increasingly true**.

---

# 38. Where complexity remains hard

The bank also asks what stops the algorithm from becoming another combinatorial explosion.

Several mature subproblems can be exponential:

* full formal-concept lattice enumeration;
* enumeration of all MUS/MCS sets;
* all minimal diagnoses;
* all minimal hitting sets;
* arbitrary relation output;
* arbitrary theorem search.

Therefore the implementation shall be **demand driven**.

Do not enumerate:

$$
\text{all basins},
\quad
\text{all conflicts},
\quad
\text{all minimal repairs},
\quad
\text{all relation pairs}.
$$

Materialize only structures reachable from an active residual or required by a protected obligation.

---

# 39. Propagation

Suppose a checked pass establishes normalized boundary \(b\).

Maintain reverse index:

$$
\boxed{
Idx(b)
=
\{
\Delta:
I(\Delta,b)
\}.
}
$$

Then update only:

$$
\Delta\in Idx(b).
$$

For each affected residual:

1. recompute condition field if necessary;
2. recompute relevant partition blocks;
3. test old contradictions;
4. test folds whose license depended on \(b\);
5. update method applicability;
6. regenerate its question frontier.

This is how one proof modifies many apparently distant pieces of the calculus without a global pairwise sweep.

---

# 40. Repeated residuals become methods

If residuals:

$$
\Delta_1,\ldots,\Delta_m
$$

repeatedly instantiate the same pattern:

$$
P:
\Delta
\rightsquigarrow
\Delta',
$$

ask:

> Is the same relational path actually recurring?

If yes, propose method:

$$
m:P.
$$

It may be retained only if:

* applicability is explicit;
* expansion is recoverable;
* failure exits remain typed;
* provenance is retained;
* protected behavior survives;
* reopening remains available.

Frequency proposes.

It does not warrant.

---

# 41. Compression

For representation:

$$
c:X\to Y,
$$

fold is licensed only if:

$$
\boxed{
c(x)=c(y)
\Rightarrow
x\equiv_{\mathcal H}y
}
$$

and protected continuation, regeneration, provenance and reopening obligations descend through \(c\).

A breaker under changed frame \(\Theta'\):

$$
c(x)=c(y)
\land
x\not\equiv_{\mathcal H_{\Theta'}}y
$$

reopens it.

The reconceptualization already gives this exact fold/break-fold/reopen structure.

---

# 42. Stopping

The final bank questions ask:

> Why call something resolved?

> Why call something impossible?

> Why stop here?

> What remains open beyond the current frame?

Those questions prohibit an absolute operational `Done`.

Define:

$$
\boxed{
LocallyClosed_{\Theta,\mathcal H,\Gamma}(\Delta)
}
$$

only if one of the following has been established:

### Determined

One protected class remains and every required discharge obligation is closed.

### Equivalent

The branch differs only below the declared protected horizon and reopening conditions are retained.

### Impossible

An exact proof/certificate establishes the completion field empty.

### Blocked

The missing capability is named.

### Unknown

The available representation/method/coverage does not decide it.

### ResourceBounded

The current finite resource budget is exhausted and the residual is preserved.

Failure to find another question is not global exhaustion.

The final Reciprocal Why questions explicitly demand the difference between what is closed relative to the current frame and what remains open beyond it.

---

# 43. Rust implementation surface

```rust
struct InquiryRuntime {
    bindings: BindingStore,
    residuals: ResidualStore,
    boundaries: BoundaryStore,
    incidence: IncidenceIndex,
    partitions: PartitionStore,
    methods: MethodRegistry,
    evidence: EvidenceStore,
    frontier: QuestionFrontier,
}
```

```rust
struct Residual {
    open_relation: RelationRef,
    conditions: Vec<ConditionId>,
    live_field: FieldRef,
    discriminators: Vec<QuestionId>,
    partition: PartitionId,

    breakers: Vec<BreakerId>,
    survived_contrasts: Vec<BreakerId>,
    conflicts: Vec<ConflictId>,
    blockers: Vec<BlockerId>,
    gaps: Vec<GapId>,

    binding: BindingRef,
    horizon: HorizonRef,
    coverage: CoverageRef,
    provenance: ProvenanceRef,
    reopen: Vec<ReopenCondition>,
}
```

```rust
enum BoundaryFeature {
    Condition(ConditionId),
    Separator(SeparatorId),
    Breaker(BreakerId),
    ConflictCore(ConflictId),
    FoldBreaker(FoldBreakerId),
    PathWitness(PathWitnessId),
    OrderWitness(OrderWitnessId),
    SupportWitness(SupportWitnessId),
    ApplicabilityBoundary(ApplicabilityId),
    RepresentationGap(GapId),
}
```

```rust
enum QuestionDisposition {
    Answered(AnswerRef),
    Productive(QuestionPlan),
    Required(QuestionPlan),
    Redundant {
        via: QuestionId,
        evidence: EvidenceRef,
    },
    Inapplicable(InapplicableReason),
    Blocked(Blocker),
    Unknown(CoverageRef),
}
```

```rust
enum Resolution {
    Supported {
        answers: Vec<AnswerRef>,
        classes: Vec<ProtectedClassId>,
    },
    Plural {
        classes: Vec<ProtectedClassId>,
    },
    ExactEmpty {
        certificate: CertificateRef,
    },
    Unsupported {
        evidence: EvidenceRef,
    },
    Unknown {
        coverage: CoverageRef,
    },
    Blocked(Blocker),
    ResourceBounded {
        residual: ResidualId,
    },
}
```

---

# 44. Reverse indices

```rust
struct IncidenceIndex {
    condition_to_residuals: HashMap<ConditionId, Set<ResidualId>>,
    separator_to_residuals: HashMap<SeparatorId, Set<ResidualId>>,
    breaker_to_residuals: HashMap<BreakerId, Set<ResidualId>>,
    conflict_to_residuals: HashMap<ConflictId, Set<ResidualId>>,
    blocker_to_residuals: HashMap<BlockerId, Set<ResidualId>>,
    fold_breaker_to_folds: HashMap<FoldBreakerId, Set<FoldId>>,
}
```

The index is rebuildable from authoritative trace/evidence.

It is not another history.

---

# 45. Condition identity

Never merge conditions by wording.

```rust
struct ConditionKey {
    schema: RelationSchemaId,
    bound_roles: BindingVector,
    scope: ScopeId,
    applicability: ApplicabilityId,
    grain: GrainId,
    orientation: Orientation,
}
```

Two residuals share a condition only if these keys agree or a checked transport relation establishes equivalence.

This prevents “shared basin” from degenerating into semantic-vector similarity.

---

# 46. Method dispatch

```rust
enum ResidualShape {
    OrderedBoundary,
    DecomposableBreaker,
    ConjunctiveConflict,
    CompetingDiagnoses,
    CoarseAbstraction,
    Synthesis,
    FinitePartition,
    UnknownAutomaton,
    SharedConditionBasis,
    MultiContext,
    Generic,
}
```

```rust
fn choose_method(shape: ResidualShape) -> MethodFrontier {
    match shape {
        OrderedBoundary       => binary_or_generalized_search(),
        DecomposableBreaker   => delta_debugging(),
        ConjunctiveConflict   => quickxplain_or_mus(),
        CompetingDiagnoses    => diagnosis_measurement(),
        CoarseAbstraction     => cegar(),
        Synthesis             => cegis(),
        FinitePartition       => partition_refinement(),
        UnknownAutomaton      => active_automata_learning(),
        SharedConditionBasis  => attribute_exploration(),
        MultiContext          => assumption_context_management(),
        Generic               => nondominated_applicable_methods(),
    }
}
```

The method name is not semantics.

Its typed preconditions and postconditions are.

---

# 47. Integration with the existing repository harness

The existing:

$$
\texttt{SPECIFY}
\to
\texttt{INSPECT}
\to
\texttt{CONTRAST}
\to
\texttt{TRACE}
\to
\texttt{EXPERIMENT}
\to
\texttt{UPDATE}
\to
\texttt{CHANGE}
\to
\texttt{VERIFY}
\to
\texttt{CHALLENGE}
\to
\texttt{MINIMIZE}
\to
\texttt{RATCHET}
$$

can remain the outer engineering lifecycle.

The algorithm above is the inner question engine.

The crucial correction is:

$$
\boxed{
\text{CONTRAST broadly}
\to
\text{EXPERIMENT}
\to
\text{UPDATE}
\to
\text{CHALLENGE}
\to
\text{MINIMIZE}.
}
$$

Not:

$$
\text{search minimally}
\to
\text{change minimally}
\to
\text{search minimally}.
$$

`MINIMIZE` occurs after a protected difference has been demonstrated.

---

# 48. The actual learned object

A consequential inquiry does not merely learn:

$$
\phi.
$$

It learns something like:

$$
\boxed{
\lambda
=
\langle
\Theta,
W,
\phi,
d,
b,
\Gamma,
P,
U
\rangle,
}
$$

where:

* \(W\): condition web under which \(\phi\) is forced;
* \(d\): proof/check evidence;
* \(b\): strongest retained boundary/breaker information;
* \(\Gamma\): coverage;
* \(P\): provenance;
* \(U\): reopening condition.

Then:

$$
\lambda
$$

is indexed by the normalized conditions it shares with other residuals.

That is the “imprint.”

---

# 49. What the entire bank finally forces

The last questions ask:

> What excludes the nearest alternative?

> What excludes the strongest alternative?

> What excludes every remaining alternative?

> Does one relation do so?

> Which part can be removed?

> What is the smallest surviving distinction between the field where the answer is forced and the field where it is not?

> Could another distinction perform the same work?

> What do all such distinctions share?

> Which shared relation is smaller?

> Which open positions does that relation leave?

> Where else can those positions be filled?

> Which alien cases pressure it?

> Which new instances would force its conditions to be revised?

Those questions close the derivation.

They imply that the object of the algorithm is not an answer.

It is a progressively factored relation:

$$
\boxed{
\mathsf{Force}_\Theta(W,\phi)
}
$$

together with:

$$
\boxed{
\text{the smallest currently demonstrated boundary separating }
\Field_\Theta(W)
\text{ from fields admitting }\neg\phi,
}
$$

and:

$$
\boxed{
\text{the open ports through which new cases may later break that boundary}.
}
$$

Thus every ratchet remains simultaneously:

1. a result;
2. a condition for later inquiry;
3. a discriminator connecting residuals;
4. a compression coordinate;
5. a possible method precondition;
6. a reopening boundary.

That is why the process accelerates as it proceeds.

---

# 50. Final algorithmic law

$$
\boxed{
\begin{gathered}
\textbf{EXPANSION PRODUCES CONTRASTS.}\\
\textbf{CONTRASTS PRODUCE BREAKERS AND CONFLICTS.}\\
\textbf{BREAKERS PRODUCE BOUNDARIES.}\\
\textbf{BOUNDARIES BECOME SHARED CONDITIONS.}\\
\textbf{SHARED CONDITIONS FACTOR THE RESIDUAL SPACE.}\\
\textbf{FACTORIZATION MAKES LATER DISCRIMINATION CHEAPER.}\\
\textbf{CONTRACTION FINDS THE MINIMAL CURRENTLY WARRANTED BOUNDARY.}\\
\textbf{RATCHETING RETAINS ONLY THAT BOUNDARY.}\\
\textbf{REOPENING PREVENTS THE FACTORIZATION FROM BECOMING DOGMA.}
\end{gathered}
}
$$

Or more compactly:

$$
\boxed{
\textbf{
OPEN
\to
EXPAND
\to
COLLIDE
\to
SEPARATE
\to
CHECK
\to
CONTRACT
\to
RATCHET
\to
PROPAGATE
\to
FACTOR
\to
REOPEN.
}}
$$

The wide expansion is not wasted search.

It constructs the coordinate system.

The narrow ratchet is not timid search.

It controls warrant.

The shared residual index is not a second ontology.

It is the computational factorization that lets distant portions of the calculus connect through the same demonstrated conditions rather than through exhaustive pairwise correspondence.

And the question bank is not a prompt list.

It is an overcomplete natural-language generating set for these typed reciprocal operations.

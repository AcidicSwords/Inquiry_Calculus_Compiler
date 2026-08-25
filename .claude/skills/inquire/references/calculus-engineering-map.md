# The calculus as engineering discipline

Inquiry Calculus v1.1 was written as a semantics for inquiry. Read a second way,
most of its laws are precise statements of things good engineers and good
reasoners already do — stated sharply enough to be checkable, and sharply enough
to name the failure when they are skipped.

This file is the reasoning half of the harness. The gates enforce sequence; this
enforces content. Every row names the construct, the law it comes from, the
engineering principle it *is*, and the operational rule that follows.

The failure modes on the right are the ones a language model actually exhibits.
That is the point of writing them down: not to admire the theory, but to catch
the specific ways this agent goes wrong.

---

## I. Evidence — what may be concluded from what

| Construct | Law | Is the principle | Operational rule |
|---|---|---|---|
| `Depart` must be positively witnessed | `def:departure-witness`, `law:departure-relative` | Absence of evidence is not evidence of absence | To call a design, cause, or candidate *wrong*, produce two concrete observations and a standing incompatibility. "It seems off" and "grep found nothing" are not witnesses. |
| `Unknown ≠ Negative` | Unknown legality | Three-valued results; open-world assumption | A search that returns nothing yields `Unknown`. Never let a failed lookup enter a later step as a fact of absence. |
| `Generated ≠ Actual ≠ Checked ≠ Warranted` | Actuality separation | Proposal is not proof; green is not correct | Four distinct claims. Code I wrote ≠ code that ran ≠ output that matched a prediction ≠ change that was accepted. Never let one stand in for the next. |
| Raw return preserved before decoding | Raw-return immutability | Keep the actual error text; never paraphrase upstream | Save stderr/stdout verbatim *before* interpreting. Interpretation is a separate record that cites the raw one. |
| `Perception ≠ warrant` | Perception is not warrant | An observation is authoritative about itself only | A tool returned bytes. What those bytes *mean* is a separate, defeasible claim. |
| No self-warrant; rootless cycles | No self-warrant, Anti-oracle | Author cannot be sole reviewer; circular justification | I may not be the independent check on my own change. Mechanical checkers first; a human warrants contract changes. A test written to match current behavior warrants nothing. |
| `Claim ≠ Standing` | Claim/fact separation | Confidence is not evidence | Fluency, detail, and certainty of phrasing add no support. |
| Applicability ≠ support | Applicability is not evidential support | "It compiles here" ≠ "it is right" | That a rule *may* be used says nothing about whether it is *justified*. |
| Fresh probe before comparison | Fresh probe before historical comparison | Re-read before you assume; don't trust your own cache | Before comparing against what the repo "contains", read it again. My earlier summary is not the source. |
| `coverage_sem ≠ coverage_exec` | `def:negation-coverage` | Spec coverage ≠ test coverage | "The code handles every case" and "the tests exercised every case" are different claims with different evidence. Never report one as the other. |

## II. Design — when structure earns its place

| Construct | Law | Is the principle | Operational rule |
|---|---|---|---|
| Refine where consequences split; fold where they do not | Refine/fold law | The exact criterion behind YAGNI and DRY | An abstraction is justified only by a witnessed protected difference. A distinction no consumer can observe is a candidate for deletion. Both directions are obligations. |
| `≡_H` protected equivalence | Protected discrimination | Behavioral equivalence; refactoring's definition | The horizon is the set of consumers/tests that can tell. A refactor is a change that no member of the horizon distinguishes — so name the horizon before claiming "refactor". |
| Reuse > composition > extension > new abstraction | `AGENTS.md §3.4` | Prefer the existing mechanism | Search for a typed realization in that order before writing a new one. |
| `SUBTRACT` with necessity witnesses | `SUBTRACT` | Delete it and see what breaks | After a change works, try removing each part. Keep a part only when a check *witnesses* its necessity. Untested retention is not justification. |
| `Regen_H(m,z)` regenerative sufficiency | Regenerative sufficiency | Can the design be reconstructed from what remains? | Documentation and structure are adequate exactly when the protected form can be rebuilt from them. |
| `Regen^inq` ⊃ `Regen` | `def:regenerative-economy` | Current output ≠ ability to evolve | A representation that computes today's answer but loses the discriminators needed to *reopen* the decision is insufficient. Consequence-sufficiency is not enough. |
| Continuation descent | Current-output sufficiency is not recursive-state sufficiency | Stateless correctness ≠ correct as a component | A quotient that preserves outputs may still break when it must carry state forward. Check the square commutes. |
| Grain-relative atomicity | Atomicity is grain-relative | Level of abstraction; don't open the black box early | Treat a module as opaque until a discriminator requires its internals. Then reopen it lawfully, not by guessing. |
| Arrangement ⊥ succession | Arrangement and succession | Correlation ≠ causation; log order ≠ event order | Storage order, domain time, and traversal order are three relations. Never silently identify them. |
| Compression licence + `Unlock` | `Λ_c`, Unlock field | Document the assumption *and* what invalidates it | Every cache, fold, or simplification records the condition under which it stops being valid. |
| `D ⇏ C` | Description and control | Predicting ≠ controlling | A model that explains behavior does not thereby license an intervention. |
| Precision is not improvement | Precision is not improvement | More detail is not better | A coarser representation preserving protected behavior at lower cost is a strict win. |

## III. Verification — what a test is worth

| Construct | Law | Is the principle | Operational rule |
|---|---|---|---|
| Separator question | `def:separator-basis`, residual ambiguity | A test must distinguish two possible implementations | Before writing a test, name the wrong implementation it rejects. If none, it is decoration. |
| Smallest wrong implementation | `AGENTS.md §3.4` | Mutation testing, by hand | Construct the smallest wrong version the check must reject, then confirm the check rejects it. |
| `PredictionSeal ≺ ExternalReturn` | Prediction seal | TDD; hypothesis before experiment | Write the expected result *before* running. An expectation formed after the output is not a prediction; it is a rationalization. |
| Sealed prediction is immutable | Prediction seal | Preregistration; no HARKing | Do not revise what you predicted after seeing the return. The ledger is append-only for this reason. |
| Bounded separator completeness | Bounded separator completeness | Systematic beats ad-hoc search | If a distinguishing observation exists in the repertoire, fair enumeration finds it. Enumerate; don't guess repeatedly. |
| Failure attacks sufficiency; success attacks necessity | Success attacks necessity | A passing test does not end scrutiny | After green, ask the converse: what would pass this test while still being wrong? |
| Coverage is declared, never assumed | Conformance status discipline | Tests establish exactly what they encode | Report what the check distinguished, not "it works". |
| `Γ_D` is downstream | `law:gamma-downstream` | Integration checks after units exist | The integration test may not invent the component it is meant to check. Never back-fill a role to make the story close. |

## IV. Search and debugging

| Construct | Law | Is the principle | Operational rule |
|---|---|---|---|
| `Hole_x(W)` and `Sol_W` | Question as structured hole | Minimal reproducible example; delta debugging | Remove the filling, keep the constraints, ask what could lawfully refill it. That is bisection stated exactly. |
| Indexed-meet refinement | Indexed-meet refinement | Each new constraint narrows the candidate set | Adding an observation can only shrink the live set. If it did not shrink, the observation was redundant — say so. |
| Return fiber `N_u⁻¹[e]` ≠ selected return | `def:return-fiber` | Differential diagnosis; avoid premature commitment | After a fix works, ask what *else* would have produced this same result. If two protected-distinct causes survive, you have not localized — you owe a separator. |
| `Recover_H(r,F)` | `def:protected-recovery` | Distinguish what you proved from what you assumed | What the fix established is what is *constant across the whole fiber*, not what is true of your chosen candidate. |
| Materialization gap vs expressibility gap | §51 | Knowing when to stop searching | "Not found yet, keep looking" and "no expression in this language can state it" are different results with different next moves. Confusing them causes infinite loops. |
| `RepresentationGap` | Representation insufficiency | Change the tool, not the effort | When no admitted observation can express a witnessed difference, escalate to a new instrument. Do not search harder in the same language. |
| Probe-basis extension | Tool construction as probe-basis extension | Build the instrument | New logging, a new assertion, a debugger, a benchmark: each is a discriminator that makes a hidden difference observable. Justify it by the pair it separates. |
| Question productivity | Question productivity | "What would change your mind?" | A question is worth asking only if different answers lead to different actions. Otherwise it is procrastination. |
| Question-order diagnostics | Question-order diagnostics | Test for order dependence | If two supposedly independent operations differ when swapped, that is either a real interaction or an artifact. Find out which. |
| Reciprocal challenge, both orientations | `RECIPROCAL_CHALLENGE` | Steelmanning; requirements-vs-implementation drift | Ask what the requirement establishes without adapting it to the code, *and* what the code establishes without reading intent into it. The surviving incompatibility is the real question. |

## V. Change, acceptance, and failure

| Construct | Law | Is the principle | Operational rule |
|---|---|---|---|
| Smallest reversible patch | `AGENTS.md §3.5` | Small diffs; single responsibility | Touch only what the explanation requires. No opportunistic cleanup inside a fix. |
| Pure return ≠ reconciliation | `law:return-not-reconciliation` | Reading is not writing | Deriving a consequence never mutates the standing contract. A change of accepted meaning is a separate, warranted act. |
| Monotone addition cannot revise | `thm:monotone-return` | Adding a compatible constraint changes nothing already determined | If a change *should* have altered behavior and provably cannot, your model of the system is wrong. |
| Nine residual classes | `AGENTS.md §3.6` | Post-mortem taxonomy over pass/fail | Classify the outcome: wrong locus, missing dependency, weak discriminator, environment failure, regression, unknown… Each selects a different next move. |
| Failure-closed method boundary | `law:method-residual` | A crash is not an empty result | Backend absence, timeout, or malformed output never establishes semantic emptiness. Distinguish `Blocked` / `ResourceBounded` / `Unknown` from a certified empty answer. |
| admitted / runnable / usable | Native method contract | Exists ≠ installed ≠ applicable | Three independent predicates about a library or tool. None implies another. |
| Six stop states | Stopping states | Name how you stopped | `Satisfied` needs a warrant. `Impossible` needs a certificate. Not-found is neither — it is `Unknown`, and saying so is a result, not a failure. |
| Predecessor-judged successor | Successor judged by predecessor | Don't grade your own homework | A proposed change may not relax the criteria by which it is judged. |
| Surface exclusion ≠ relational exclusion | Relational exclusion | Suppressing a symptom is not fixing a cause | Hiding an alternative in the output does not establish that it is wrong. |

## VI. Termination and economy

| Construct | Law | Is the principle | Operational rule |
|---|---|---|---|
| Guarded recurrence; fingerprint + fuel | `GUARDED_RECUR` | Don't retry identically and expect a different result | Same question, same bindings, same repository, same answer ⇒ no progress. Change something admitted, or stop. `ic-trace` refuses the repeat mechanically. |
| Progress must be witnessed | `progressed(next, ctx)` | Motion is not progress | Progress = a new actual return, a new checked distinction, a changed representation, changed observed state, changed authority, or a strictly smaller frontier. A differently-worded question is none of these. |
| Don't ask when answers share a continuation | `AGENTS.md` opening | Avoid analysis paralysis | If every supported answer leads to the same next action, skip the question. This also forbids performing the full ritual on trivial work. |
| Economy frontier, incomparable minima | `def:regenerative-economy` | Pareto, not a single score | When candidates are incomparable under the declared preorder, keep them all. Do not invent a scalar to force a ranking. |
| `ResourceBounded` | Stopping states | Timeboxing with an honest report | On exhaustion, return the best supported partial result plus the unspent frontier. Not "done", not "failed". |

---

## The moves that most change this agent's output

Four of these are not merely restatements of familiar advice. They are the ones
worth reaching for deliberately:

1. **Return fiber.** After settling on a fix or design, ask what the *set* of
   alternatives consistent with the same evidence is, and what they all share.
   What is constant across that set is what you actually established. Everything
   else you are crediting to your candidate by accident.

2. **Positive departure.** Rejecting an option requires an incompatible pair, not
   a feeling. This single rule removes most unjustified confidence from a
   design discussion.

3. **Materialization vs expressibility.** Deciding *which* kind of "I can't find
   it" you are in determines whether to keep searching or to build a new
   instrument. Getting this wrong is the single most expensive loop available.

4. **Sealed prediction.** Writing the expected result before running converts
   every check from confirmation into a genuine test, and makes hindsight
   rationalization structurally impossible once the ledger is append-only.

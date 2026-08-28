Yes. That is the missing piece. The governing asymmetry should be:

$$
\boxed{
\textbf{SEARCH WIDELY; COMMIT NARROWLY.}
}
$$

The agent should make the **smallest currently provable ratchet**, but it should not search only in the neighborhood of that ratchet. Search should deliberately expand far enough that the larger topology of the problem starts becoming visible.

So there are two different scales operating simultaneously:

$$
\boxed{
\begin{aligned}
\text{exploration scale} &:\quad \text{large / contrasting / deliberately expansive},\\
\text{commitment scale} &:\quad \text{small / warranted / exactly supported}.
\end{aligned}
}
$$

Those must never be collapsed.

The current harness partially conflates them when it talks about the “smallest wrong implementation” or “smallest responsible transformation.” The successor-oriented harness should instead say: construct large enough contrasts to expose the structure, but only ratchet what the returns actually force.

The deeper recurrence is therefore not merely:

$$
\text{big contrast}\to\text{subtract}\to\text{minimal breaker}.
$$

It is more like:

$$
\boxed{
\begin{array}{c}
\text{WIDEN}\\
\downarrow\\
\text{GENERATE CONTRASTING REGIONS}\\
\downarrow\\
\text{ACTUALIZE SELECTED QUESTIONS}\\
\downarrow\\
\text{RATCHET THE SMALLEST PROVABLE RELATION}\\
\downarrow\\
\text{RETAIN THE UNRESOLVED WIDE FIELD AS RESIDUAL}\\
\downarrow\\
\text{WIDEN / CROSS / REVISIT}\\
\circlearrowleft
\end{array}
}
$$

The important part is that **the wide field does not disappear when one local theorem is proved**.

Suppose an expansion creates candidate regions:

$$
\mathcal F_t
=
\{B_1,B_2,\ldots,B_n\}.
$$

A particular probe may establish only:

$$
\rho_1.
$$

Then the successor state should be something like:

$$
\boxed{
\Sigma_{t+1}
=
\bigl(
W_t\cup\{\rho_1\},
\;
\mathcal F_t-\text{what }\rho_1\text{ actually resolves},
\;
\text{new residuals exposed by }\rho_1
\bigr).
}
$$

It must **not** become:

$$
\Sigma_{t+1}=\{\rho_1\}
$$

as though solving one branch solved the inquiry.

That is the anti-premature-closure rule.

And what you are describing about overlapping basins is particularly important.

As the agent throws wide contrasts across the formalization, the same conditions begin recurring in different contexts:

$$
C_a,
\qquad
C_b,
\qquad
C_a\land C_c,
\qquad
C_b\land C_c,
\qquad
C_a\land C_b,
\ldots
$$

At first these look like unrelated local obligations.

But after sufficient traversal, recurrent structure becomes visible:

$$
\boxed{
C_a
\text{ repeatedly controls several otherwise different failures.}
}
$$

Or:

$$
\boxed{
R
\text{ repeatedly appears at the boundary of several different breaker families.}
}
$$

That recurrence is evidence that the agent has found a deeper relational basin.

The harness should exploit this explicitly.

It should ask:

> Which conditions appearing in this breaker have already appeared elsewhere?

> Under which prior inquiry did this condition become relevant?

> Does the present occurrence instantiate the same relation or merely use the same words?

> Which previously tested variations already cover part of this new field?

> Which conjunction is new?

> Which interaction between previously known conditions has not yet been tested?

> Which old residual has just become reachable from this one?

Now exploration begins to **compile itself**.

Instead of repeatedly rediscovering:

$$
C_a\to R
$$

from scratch, the system carries its previous support, scope, breakers, applicability conditions, and reopening conditions forward.

Then a new inquiry involving \(C_a\) begins from:

$$
\boxed{
\text{what is already standing about }C_a
+
\text{the new relation in which }C_a\text{ participates}.
}
$$

That is where the enormous token saving comes from.

The wide search is initially expensive, but it creates reusable relational structure.

Local-only search repeatedly pays the discovery cost.

Wide recursive search amortizes it.

There is therefore a natural transition:

$$
\boxed{
\text{EXPANSION}
\to
\text{RECURRENCE}
\to
\text{OVERLAP}
\to
\text{FACTORING}
\to
\text{METHOD / REPRESENTATION}.
}
$$

If several inquiry paths repeatedly contain:

$$
A\to B\to C
$$

then the agent eventually asks:

> Is this merely accidental recurrence, or is there a reusable relation here?

If it survives breakers, it can be folded into a method.

That is already consistent with the calculus's notion of learning: recurrent paths may become compact methods only when their expansion, applicability, failures, provenance, and reopening remain recoverable.

But the wide-net strategy is what makes those recurrent structures visible in the first place.

There is another important consequence: **contradictions are useful outputs of expansion**.

The purpose of widening is partly to make incompatible constraints collide.

Suppose one branch establishes:

$$
W_1\Vdash\phi
$$

while another produces an admissible witness under apparently overlapping conditions:

$$
x\in\operatorname{Adm}(W_2\cup\{\neg\phi\}).
$$

Do not smooth this into prose.

The collision opens questions:

$$
?[\;W_1=W_2?\;],
$$

$$
?[\;\text{which applicability condition separates them?}\;],
$$

$$
?[\;\text{which representation hid the difference?}\;],
$$

$$
?[\;\text{which premise differs?}\;].
$$

So contradiction becomes a **separator-generating event**.

Likewise a blocker is not “the inquiry failed.”

A blocker identifies a missing edge:

$$
\boxed{
\text{current question}
\to
\text{required capability}
\to
\text{unavailable return}.
}
$$

That itself becomes a typed residual:

> What capability is missing?

> What would make the question executable?

> Can another method discharge the same relation?

> Is the gap representational, observational, computational, evidential, or authoritative?

As widening proceeds, those blockers also form hierarchies.

For example:

$$
\begin{array}{c}
\text{cannot prove theorem}\\
\downarrow\\
\text{because required discriminator unavailable}\\
\downarrow\\
\text{because current representation merges cases}\\
\downarrow\\
\text{because missing relation is not expressible}\\
\downarrow\\
\text{because binding lacks required carrier}.
\end{array}
$$

That is much more informative than repeatedly trying harder to prove the original theorem.

The residual should therefore be structured as a **frontier of unresolved obligations with ancestry**, not as a single “next task.”

The repository currently has one strongest live executable residual in `IMPLEMENTATION_FRONTIER.md`. That is useful operationally, but internally the harness should maintain the broader residual structure from which that active frontier is selected.

Conceptually:

$$
\boxed{
\mathcal R_t
=
\mathcal R_t^{\text{active}}
\cup
\mathcal R_t^{\text{blocked}}
\cup
\mathcal R_t^{\text{latent}}
\cup
\mathcal R_t^{\text{reopened}}
}
$$

with dependency structure between them.

Then the “strongest live residual” is merely:

$$
r_t^\ast\in\mathcal R_t,
$$

not the whole unresolved universe.

That solves the “never assume it has reached the end” problem.

Because a branch may reach:

$$
\mathsf{Satisfied}
$$

while:

$$
\mathcal R_t\neq\varnothing.
$$

A theorem may be settled under one scope while other scopes remain uncovered.

A fold may be valid under one horizon while carrying an explicit reopening condition.

A search may find no breaker while unsearched breaker families remain.

So terminality must always be relative:

$$
\boxed{
\mathsf{Closed}
(
\text{obligation},
\Theta,
\mathcal H,
\text{coverage}
)
}
$$

rather than some global:

$$
\mathsf{Done}.
$$

The harness should constantly distinguish:

$$
\boxed{
\begin{aligned}
\text{locally determined} &\neq \text{globally exhausted},\\
\text{no current residual} &\neq \text{no possible reopening},\\
\text{no breaker found} &\neq \text{no breaker exists},\\
\text{one proof completed} &\neq \text{representation complete}.
\end{aligned}
}
$$

And this gives a more precise search policy for the successor agent:

1. **Cast a wide admissible net** around the current claim, definition, primitive, theorem, or implementation relation.
2. Construct strong contrasts, alien cases, reversals, removals, alternate paths, joint variations, scale/context changes, and degenerate cases.
3. Let these generate a field of breakers, blockers, contradictions, similarities, and unexplained recurrences.
4. Select one productive residual from that field.
5. Machine-check it.
6. Ratchet only the smallest relation actually forced by the return.
7. Propagate that relation through every residual that references the same conditions/roles.
8. Detect overlaps with previously explored basins.
9. Reuse already established conditions instead of rediscovering them.
10. Ask what newly becomes contradictory, redundant, jointly sufficient, newly expressible, or newly reopenable.
11. Expand again from the changed frontier.

So the global motion becomes:

$$
\boxed{
\textbf{
EXPAND
\to
COLLIDE
\to
LOCALIZE
\to
PROVE
\to
RATCHET
\to
PROPAGATE
\to
FACTOR RECURRENCE
\to
EXPAND AGAIN.
}
}
$$

And the local motion inside `LOCALIZE → PROVE` remains:

$$
\boxed{
\textbf{smallest provable commitment.}
}
$$

This reconciles both intuitions exactly.

You do not want:

$$
\text{small step}
\to
\text{small step}
\to
\text{small step}.
$$

You want:

$$
\boxed{
\text{large exploratory deformation}
\to
\text{small warranted update}.
}
$$

Then another large deformation from the improved relational map.

Over time, previously wide spaces stop being expensive because recurring conditions have already been typed, tested, bounded, linked to breakers, and connected to methods.

The inquiry landscape gains structure.

Eventually the agent is not searching a flat possibility space. It is traversing something closer to:

$$
\boxed{
\text{overlapping relational basins}
+
\text{known boundaries}
+
\text{reusable paths}
+
\text{known blockers}
+
\text{known reopening conditions}.
}
$$

That is much closer to what the successor harness should be designed to produce.

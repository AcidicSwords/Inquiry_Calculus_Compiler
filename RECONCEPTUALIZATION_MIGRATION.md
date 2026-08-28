# Reconceptualization Migration

## Proposal for carrying the integrated semantic and compiler reconceptualization into `Inquiry_Calculus_v2_0.tex`

**Status: proposal. This document carries no control authority.**

It does not amend the canonical specification, the stable plan, the frontier, or the
conformance record. It states, for each proposed change, an exact definition, its v2.0
status class, its anchor, the argument that forces it against its live alternatives, its
propagation through the canonical document, what it removes or renames, and the condition
under which it reopens. Adopting any item requires explicit predecessor authority and a
named control residual, per `AGENTS.md`.

Line anchors are given as `:N` against `Inquiry_Calculus_v2_0.tex` at the coordinate under
which this document was written. They are reading aids, not identities.

---

## 0. Method and reading rules

Three rules govern every entry.

**Non-promotion.** Nothing admitted at a lower layer may be silently promoted. A binding
law is not a calculus law; a method capability is not a semantic law; a quantizer property
is not a semantic property. Each entry carries one of the five v2.0 statuses (`:285`):
`CONSTITUTIONAL`, `CANONICAL-RESTATED`, `DERIVED`, `BINDING-SUPPLIED`,
`IMPLEMENTATION-ONLY`.

**No synonyms.** A second name for an existing object is refused, however suggestive. The
regenerative basis (`:6747`) is minimal by intent; a synonym creates a second species by
notation alone and is the most common way a calculus loses its type.

**Forced, not preferred.** Each entry names the alternatives that were live and the
argument that excludes each. An entry with no excluded alternative is not an entry; it is
a preference, and is not proposed.

### 0.1 Triage of the source material

Against the standing canonical document, the reconceptualization divides as:

| Class | Share | Disposition |
|---|---|---|
| Already present, restated | ~55% | cite the existing anchor; add nothing |
| New, derivable from the basis | ~20% | admit as proposition or derived construction |
| New, not derivable | ~15% | status decision required (Tier 2) |
| Collisions it surfaces | ~10% | must be fixed whether or not anything is admitted |

The material is overwhelmingly conservative. That is what makes it weavable. The risk is
not new semantics; it is renaming.

---

## 1. Tier 0 — theorems over the existing basis

No new symbols. No new carriers. These are stated first because they determine where
everything else lands.

### T1 · Two meets and one link

**Statement.**

> **Theorem (two meets).** Every discrimination construction in this document is an
> indexed meet of *kernels*; every constraint construction is an indexed meet of *fibers*.
>
> Kernel meets (partition refinement, monotone in the index family):
> `∼_q` (`:1213`), `≡_ℋ = ⋂_{K∈ℋ} ker Con_K` (`:1443`), `Ker η_t` (`:1290`),
> `Ker π_{q⊗r}` (`:1267`), `ker σ_𝒩` (plan §32), `ker M_P` (N4).
>
> Fiber meets (field contraction, antitone in the index family):
> `Fib_I(R∣β)` (`:6897`), `Sol^X_W = ⋂_i Sol^X_{ρ_i}` (`:1417`), `N_u[s]` (`:6871`),
> `RetField_u(e)` (`:6873`), guard refinement (`:1170`).
>
> **Link.** A fiber is one class of a kernel together with the selection of that class.
> `Determines_ℋ(W,x) ⟺ Sol^X_W / ≡_ℋ = {[x]_ℋ}` (`:1479`) states exactly that a fiber
> meet has contracted to a single kernel class. The separation
> `ReturnFiber ≠ SelectedReturnFilling` is the law forbidding their conflation.

**Status.** `CANONICAL-RESTATED`.

**Anchor.** `§1411`, completing the existing remark at `:1440` which already observes that
`Sol` and `≡_ℋ` are both indexed meets but does not distinguish what is met.

**Forced by.** Three readings were live.
(a) `Sol`, `Fib`, `Field`, `Poss` are four constructions — the reconceptualization's
implicit reading. Refuted: they satisfy one monotonicity law (`:1424`) and have one type.
(b) `Sol`, `Fib`, `≡_ℋ`, `∼_q` are one construction under four names. Refuted: `≡_ℋ` and
`Sol_W` have **opposite variance** in their index family. Enlarging `ℋ` refines the
kernel (`:4037`); enlarging `W` shrinks the fiber (`:1424`). Opposite variance cannot be
one construction.
(c) The present split. Only this survives both refutations.

**Propagation.** No symbol changes. Referenced by N1, N2, R1, R2, R5, and by the notation
table reorganization in §6. It supplies the reason `⊗` on questions is free while `⊗` on
relations is not (R5), and the reason `Poss`/`Field` are synonyms rather than constructs
(R1, R2).

**Removes.** Nothing. **Prevents.** Four constructions where there are two.

**Reopening.** A construction that is an indexed meet of neither kernels nor fibers.

---

### T2 · Shaped absence is shaped openness

**Statement.**

> **Proposition.** For an admitted negation use `u` with `N_u ↪ X × Y` and source `s : X`,
> the open negative `?_e N_u(s,e)` is the canonical question `?_I R[β]` with `R := N_u`,
> `β := (s)`, `I := {e}`. Hence
> `Fib_{\{e\}}(N_u ∣ s) = N_u[s] = NegField_u(s)`,
> and every question-level construction applies to negation questions unchanged: dependent
> composition (`:1136`), independent combination (`:1157`), guard refinement (`:1170`),
> answer substitution (`:1182`), the refinement algebra (`:1233`), `mode(q,i)` (`:1101`),
> `LiftQ_F`, and `QSucc` (`:2630`).
>
> **Corollary.** Prior non-presence and determination-relative absence are distinct:
> `□_I ≠ □_{s,u}`. The second retains relational provenance to `s` through the relation by
> which its exterior is typed.

**Status.** `CANONICAL-RESTATED`.

**Anchor.** Immediately after Law *Positive-negation recurrence* (`:484`).

**Forced by.** The live alternative was a parallel negation-question algebra — a second set
of composition, refinement, and succession rules for negation questions. The removal test
settles it in the other direction as well: deleting Law `:465` and keeping only the
proposition loses (i) the prohibition clause — Boolean complement, boundary projection,
failed search, non-equivalence, and `Unknown` do not supply a departure witness — and (ii)
the recurrence clause. Neither is derivable from the subsumption. **Keep the law, add the
proposition, delete nothing.**

**Propagation.** `:1767` may cite the proposition instead of restating question machinery.
`:3265` (compiler stage 2 chart) gains no case. `:2222`, `:2300` unaffected.

**Removes.** The need for a parallel algebra that does not yet exist. This is the entry
with the largest prevented cost and the smallest textual footprint.

**Reopening.** A negation question requiring a discharge mode, composition rule, or
succession relation that ordinary questions lack. None was found.

---

### T3 · Freeness of the program equational theory

**Statement.**

> **Definition (probe multiset).** For `P : Prog`, let `μ(P)` be the multiset of `Probe`
> occurrences in `P`.
>
> **Law (occurrence-multiset preservation).** The admitted semantic equational theory over
> `Prog` is free over the probe signature: no admitted equation relates `P₁`, `P₂` with
> `μ(P₁) ≠ μ(P₂)`. A binding may admit a stronger equation only by supplying an explicit
> witness `Θ ⊢ w : ProbeEquiv(p, ℓ)` naming the probe and the property that licenses it.
>
> **Corollary.** `REFERENCE COPYABILITY ≠ AUTHORITY MULTIPLICITY`. Duplicating an
> `EventRef` does not change `μ`, hence does not produce a second event. One evidence root
> referenced ten times does not become ten roots.

**Status.** `CONSTITUTIONAL` — a law over the existing `Prog` grammar. It adds no grammar.

**Anchor.** New law after Law *Actuality separation* (`:509`), with a corresponding side
condition stated at `:3116`.

**Forced by.** Three alternatives were live.

(a) **Add a sequencing token `τ_t` to the core**, threading
`Γ, τ_t ⊢ Probe(p) : (RawReturn, EventRef, τ_{t+1})`. Refuted twice. It rewrites `:2929`,
`:2954`, `:3108`, `:3116`, `:3160`, and `:6323` for no semantic gain; and `:6745` forbids
making a primitive of what regenerates from the basis. Token threading, affine `EventRef`,
and grading the monad by the free commutative monoid on probe occurrences are three
**interchangeable implementations** of the same law, and privileging one in the kernel is a
notation change presented as a semantic one.

(b) **Leave it to review.** Refuted decisively: `:3116` *contains equations*. An equation is
a licence to rewrite. If any equation permits duplicating or dropping a `Probe`, the
calculus itself licenses manufacturing actuality. That is a soundness defect in the
equational theory, not a compiler defect, and review does not repair a theory.

(c) The freeness law. `:6323` already states that `Return | Branch | Probe` admits a
free-effect reading in which probe requests are algebraic operations. In the free theory
over a signature there are no equations between terms with different operation-occurrence
multisets. **Duplication is already underivable semantically**; the law makes that explicit
and binds the compiler to it.

**Propagation.** `:2929`, `:2954` unchanged — no new grammar. `:3108`, `:3116` gain the side
condition. `:3160` unchanged. `:3238`–`:3318` gain the effect condition as an admissibility
requirement on every rewrite (S1, `LAW-EFFECT-001`). `:6323` gains one sentence: for the
purpose of this law the free-effect reading is the semantic reading, not merely a reference
model.

**Removes.** The `τ` proposal in its entirety. Propagation drops from "the whole program
core" to one law plus one conformance row.

**Reopening.** A binding admitting `ProbeEquiv` for a genuinely idempotent probe — an
idempotent read is the realistic case. The theory is then no longer free and grading
becomes necessary rather than optional.

---

### T4 · Discriminator pullback

**Statement.**

> **Definition.** For a process `Φ : A ⇝ B` and `K ∈ ℋ_B`, the pulled-back discriminator is
> `Φ^*(K) := Con_K ∘ Φ`, a discriminator on `A`.
>
> **Law (discriminator pullback).** The protected horizon pulls back along any process; the
> process need not push forward. `Φ^*` requires no invertibility and is not an inverse:
> `R^† ≠ Inv_w(R) ≠ R^{-1}[Y] ≠ Φ^*`.
>
> **Corollary.** `RetField_u(e) = N_u^{-1}[e]` is a fiber, not an inverse (`:1820`).
> `Unlock_obs(c) = {K : Con_K no longer factors through c}` (`:4759`) is pullback failure.
> Converse is not an inverse (`:838`) for the same reason.

**Status.** `CONSTITUTIONAL`. No new carrier; it composes an existing relation with an
existing discriminator.

**Anchor.** `§4108` *Operator descent*, immediately before it.

**Forced by.** The alternative was to leave this inside a quantum binding as the adjoint
channel `Φ^*`. Refuted: the three facts it explains are already general and are currently
stated three times in the canonical document without a common reason. Stating the law
unifies them and admits nothing quantum. The quantum binding supplies the sharpest instance
(`Tr(EΦ(ρ)) = Tr(Φ^*(E)ρ)` with `Φ^* ≠ Φ^{-1}`) and remains a binding.

**Propagation.** `:831` and `:1820` gain a cross-reference. `:4108` gains the law. `:4743`
`Unlock_obs` is restated as pullback failure. `QUANTUM-ADJOINT-001` becomes a binding
conformance row rather than a kernel obligation.

**Removes.** Three unexplained coincidences.

---

## 2. Tier 1 — new constructions

### N1 · The condition intent `Cond^X` and the polarity

**Statement.**

> **Definition (admitted condition family).** `𝒞_Θ ⊆ Form_𝔅` is the *admitted represented*
> family of closed applicable predicates on `U` after all other ports are lawfully bound.
> `𝒞_Θ` is a represented family, never a powerset of `U → Prop`.
>
> **Definition (condition intent).** For `S ⊆ U`,
> `Cond^X_Θ(S) = { ρ ∈ 𝒞_Θ : ∀x ∈ S. ρ(x) }`.
>
> **Theorem (polarity).** For `W ⊆ 𝒞_Θ` and `S ⊆ U`,
> `W ⊆ Cond^X_Θ(S) ⟺ S ⊆ Sol^X_W`.
> Both operators are antitone. `Sol ∘ Cond` and `Cond ∘ Sol` are closure operators and
> their fixed pairs form a complete lattice.
>
> **Definition (executable counterpart).** `Cond^{X,ε}_Θ(S)` is three-valued per condition:
> `Included{certificate}` under exhaustive coverage of `S` or a proof;
> `Excluded{witness x}` for a positive `x ∈ S` with `¬ρ(x)`;
> `Unknown{residual}` otherwise.
>
> **Law (the polarity does not transport).** `Cond^{X,ε} ⊆ Cond^X` and
> `Sol^{X,ε} ⊆ Sol^X` — both under-approximate. The polarity is a **semantic** law and does
> not hold of the executable pair: a smaller `Cond^{X,ε}` yields a *larger*
> `Sol^{X,ε} ∘ Cond^{X,ε}`. Absence of a counterexample is `Unknown`, never `Excluded`.

**Status.** `Cond^X` is `CANONICAL-RESTATED` — the adjoint of a basis operator.
`Cond^{X,ε}` is `DERIVED`.

**Anchor.** `§1411`, after Theorem *Indexed-meet refinement* (`:1434`).

**Forced by.** Three alternatives.
(a) **Do not admit it.** Refuted: the calculus repeatedly uses "which conditions hold
throughout this field" informally — in the separator question (`:1521`), in support
environments (`:4514`), and in the determination presentation (`§1720`) — with no operator
for it.
(b) **Admit the reconceptualization's `Field_Θ` and `Cond_Θ` as a new pair.** Refuted by T1
and by the basis: `Field_Θ(W)` *is* `Sol^X_W` (`:1413`, `:6763`, `:6865`).
(c) The adjoint of `Sol`. Survives.

**Why `𝒞_Θ` must be represented.** Otherwise `Cond` returns a subset of a function space and
leaves `Form_𝔅`, contradicting *Recursive form closure* (`:361`) and making the polarity
ill-typed. This is a well-typedness constraint, not a convenience.

**Why three-valued is forced.** `Unknown ≠ Negative`, plus the standing precedent
`RecoveryStatusIR` (plan §30), which already requires a positive witness for `NotRecovered`
and forbids reading absence of a certificate as loss.

**Established correspondence.** `(U, 𝒞_Θ, I)` with `x I ρ ⟺ ρ(x)` is a formal context;
`Sol` and `Cond` are its derivation operators; the fixed pairs are formal concepts and form
a complete lattice. This identifies existing structure; it does not import machinery.

**Propagation.** `:1411` definition and theorem. `:1450` used by N2 and N3. `:1479`
`Determines` restated per T1. `:4636`–`:4743` compression licences may cite the closure.
`:4824`/`:4858` (`Q^∞` / `Q^ε`) gains an explicit sibling: the semantic/executable split is
the same pattern. `:6747` regenerative basis gains `Cond^X`. `:6852` gains two rows.

**Renames.** `Field_Θ(W)` is **not** admitted. `Sol^X_W` stands.

---

### N2 · The release shell

**Statement.**

> **Definition (release shell).** For `W ⊆ 𝒞_Θ` and `ρ ∈ W`,
> `Shell_W(ρ) = Sol^X_{W∖{ρ}} ∖ Sol^X_W`.
> By indexed-meet refinement (`:1424`), `Sol^X_W ⊆ Sol^X_{W∖{ρ}}`, so the difference is well
> defined and is exactly the field released by removing `ρ`.
>
> **Law (three-valued shell).** The set difference above is admissible only where `ρ` is
> total on `U`. For partial `ρ`, `Shell` is three-valued per candidate:
> `Released{witness}`, `StillExcluded{witness}`, `Unknown{residual}`.
> An empty shell reported without an exhaustive coverage certificate is `Unknown`, never
> dispensability.
>
> **Remark (duality).** `Shell` is the fiber-side counterpart of `Sep_ℋ(f,g)` (`:4021`).
> `Sep` names the discriminators that separate two forms; `Shell` names the forms released
> by dropping one condition. Both answer the removal question, on opposite sides of T1.

**Status.** `DERIVED`.

**Anchor.** `§1450`, after Definition *Residual ambiguity* (`:1530`).

**Forced by.** `Sep_ℋ` exists and its dual does not — the calculus can say which
discriminator separates two forms and cannot say which forms a condition excludes. The
ablation discipline used throughout the repository has no semantic type; `Shell` is that
type, and `MINIMIZE` in the master recurrence (`:5973`) currently has no definition.
Alternatives: (a) leave ablation informal — refuted, it is the repository's primary method
and is untyped; (b) plain set difference — refuted, complement is undefined off `ρ`'s
domain and an implementation would silently read "no released candidate found" as
"dispensable", collapsing `Unknown` into `Negative`.

**Propagation.** `:1450` definition. `:4689` `Λ_c` gains the minimization criterion.
`:5973` `MINIMIZE` gains a definition. `:6683` compression conformance. Used by N7.

**Note on complexity.** Deciding dispensability by shell is NP-hard where `𝒞_Θ` is finite
and undecidable where the binding is Turing-complete. The three-valued form is therefore
forced by complexity as well as by law.

---

### N3 · Conditional forcing

**Statement.**

> **Definition.** `W ⊩_Θ φ` iff `Sol^X_W ≠ ∅` and `Sol^X_W ⊆ ⟦φ⟧_Θ`.
>
> **Remark.** Without the nonemptiness clause an unsatisfiable web forces everything.
> Forcing is semantic entailment relative to the current web; standing still requires
> independently discharged support (`:4577`).

**Status.** `DERIVED` — from *Property image of a hole* (`:1452`) with `p` the characteristic
map of `φ`, plus nonemptiness.

**Anchor.** `§1450`, after `:1467`.

**Forced by.** The only new content is the nonemptiness side condition, and it is forced by
the vacuous-forcing counterexample. Without it, `⊩` would license every consequence from a
contradictory web, which is precisely the collapse the calculus's `Unknown`/`Negative`
discipline exists to prevent.

**Propagation.** `:1450`. `:5307` Prediction and `:5342` Prediction seal may cite it.
`:6167` necessity and sufficiency.

---

### N4 · Consequence distortion and normal coordinates

All of N4 sits **inside** the binding guard already declared at `:6232`
("a binding-specific exact specialization, not constitutional linear or probabilistic
structure").

**Statement.**

> **Definition (analysis and frame operators).** For a real Hilbert space `V` and a measure
> `P` indexing continuous linear question functionals represented by vectors in `V`,
> `(𝒜_P x)(λ) = ⟨λ, x⟩` and `M_P = 𝒜_P^* 𝒜_P`. Then `ker M_P = ker 𝒜_P`.
>
> **Proposition.** For `V = ℝ^d` and `⟨λ,x⟩ = λ^⊤x`, `M_P = 𝔼[λλ^⊤]`. Theorem `:6235` is
> the finite-dimensional instance of the frame operator, not a competing definition.
>
> **Theorem (consequence distortion).** With `e = x − x̃`,
> `D_P(x, x̃) = 𝔼_λ[(λ^⊤e)²] = e^⊤ M_P e = ‖M_P^{1/2} e‖²`.
> Exact equivalence is its zero set: `D_P = 0 ⟺ e ∈ ker M_P`.
>
> **Definition (consequence-normal coordinates).** With `M_P = U_r Λ_r U_r^⊤` on the
> positive eigenspace, `C_P = Λ_r^{1/2} U_r^⊤`. Then `C_P x = C_P x' ⟺ x ∼_P x'` and
> `D_P(x,x̃) = ‖C_P x − C_P x̃‖²`.
>
> **Definition (query whitening).** `ξ = Λ_r^{-1/2} U_r^⊤ λ`. Then `λ^⊤x = ξ^⊤(C_P x)` and
> `𝔼[ξξ^⊤] = I_r`.
>
> **Law (representation covariance).** For invertible `L`, `x' = Lx` and `λ' = L^{-⊤}λ`
> preserve `λ^⊤x`, and `M'_P = L^{-⊤} M_P L^{-1}`. A representation change must transport
> the object and the discriminator together.
>
> **Law (licence content).** An approximate licence in this binding carries
> `(M_P, binding version, query-distribution version, ε, direction)`. It may never carry
> `ε` alone.

**Status.** `BINDING-SUPPLIED`.

**Anchor.** `§6230`, after Corollary `:6270` and before Remark `:6283`.

**Forced by.** `:4711` defines consequence distortion `Δ_ℋ` abstractly; `:6235` defines the
exact quotient. Nothing connects them, so exact and approximate compression read as two
theories. The distortion theorem is that connection **in this binding**: exact equivalence
is the zero set of the approximate geometry. The alternative — promoting it generally — is
refused by `:6232`'s own declaration and by the non-promotion rule.

**Resolves a standing collision.** `:4735` prohibits an approximation "encoded by a scalar
alone" where directional contracts discharge different claims, and `D_P` is a scalar. The
resolution is that a distortion **value** is a scalar while a distortion **relation** is the
pair (quadratic form, comparison direction); `:4735` prohibits carrying the value without
the form. The licence-content law states this. Without it the two sections contradict.

**Propagation.** `:4711` gains a cross-reference. `:6230` gains four items. `:6283`'s unlock
remark extends to the `M_P` version, which it already anticipates for `M_Q`.

---

### N5 · Local invisibility is not global foldability

**Statement.**

> **Definition (score-weighted section).** For `w : ℝ → ℝ_{≥0}`,
> `M_w[x] = 𝔼_λ[ w(λ^⊤x) λλ^⊤ ]`.
> This is a **section** `U → PSD(V)`, not a single operator. It is written `M_w[x]` and
> never `M_P`.
>
> **Law (local consequence-invisibility is not global foldability).** `ker M_w[x]` is a
> local nullspace. A fold is licensed only by `⋂_{x ∈ U_appl} ker M_w[x]` over the declared
> applicable field. A nullspace estimated at one point, or on a sample, supplies a working
> licence only.

**Status.** `M_w` is `BINDING-SUPPLIED`. The law is `CONSTITUTIONAL` — it is a statement
about folds generally, not about linear structure.

**Anchor.** Law at `§4711` after `:4741`; `M_w` at `§6230`.

**Forced by.** `:4636`'s exact condition is stated globally and is correct as written. But
`:4711` and Corollary `:6270` never forbid folding on a locally estimated kernel, and an
implementation reading `:6270` alone would do exactly that. The failure is silent — the
fold appears licensed and destroys a distinction that is invisible only near the sampled
point. Leaving it implicit was the live alternative and is refused for that reason.

**Propagation.** `:4636` cross-reference. `:4711` law. `:4689` `Λ_c.Evidence` must name the
field over which the intersection was taken. `:6283` already warns about sample estimates
and rank thresholds; this generalizes that warning off the linear binding.

---

### N6 · Residue is a tagged family

**Statement.**

> **Law (residue tagging).** Every retained residual is a tagged family indexed by the
> relation use and discharge mode that produced each component:
> `𝔯 = Σ_{(u,m)} {(u,m)} × 𝔯_{u,m}`.
> No authoritative untagged union of residue is admitted. A residual component obtained
> under `Generate` and one obtained under `Probe` are not members of one set.
>
> **Consequence.** The `Residual` component of a compression licence `Λ_c` (`:4694`) carries
> the same index. A residual selection that reports one component and discards the others
> reports an untagged union of one, which this law prohibits.

**Status.** `CANONICAL-RESTATED` — the tagged-frontier discipline of `:1767` and plan §23
applied to a second object.

**Anchor.** `§4677` recovery/reopening contract, and `§4689` `Λ_c`.

**Forced by.** Three alternatives.
(a) **Residue is always `Pure`, being derived.** Refuted: backward residue is a fiber of an
*actual* relation use, and whether its alternatives were generated or witnessed is a real
difference under *Actuality separation* (`:498`).
(b) **Residue takes the meet of its constituents' modes.** Refuted: a meet collapses a
bundle holding one probed and one generated alternative to "generated", discarding the
probed one. That is the untagged-union error the calculus already names at plan §23, with a
direct breaker: `N₁^{-1}[y] ≠ N₂^{-1}[y]` even when both produce the same `y`.
(c) **Tagged.** Survives, and introduces no machinery: it is `𝒩_D^α(s) = Σ_u {u} × N_u[s]`
(`:6769`) applied to residue.

**Propagation.** `:4677`, `:4689`, `:5643` canonical records (`Λ_c` record), `:6683`
compression conformance.

---

### N7 · Determination-presentation admission and minimization

This entry closes the research gate at plan §14 and the first bullet of plan §5.5.

**Statement.**

> **Law (occurrence indexing).** `W_D^α(s)` is not a function of the form `s`. Its identity
> is carried by the negation use `u` (plan §18, `source_determination`), exactly as `QSucc`
> is indexed by `AskRef` rather than by `(q, Ŝ)` (`:2630`). The shorthand `W_D(x)` is a
> lossy projection and is not authoritative.
>
> **Law (admission).** A determination presentation is admissible iff
> 1. **grounded** — a standing claim `c` occupies the source role at `(D, α)` and the
>    declared scope, applicability, and grain;
> 2. **support-closed** — `W = Supp^*(c)`, the transitive closure of `c` under minimal
>    support environments (`:4514`, `:6925`), restricted to `𝒞_Θ`;
> 3. **independently standing** — every `ρ ∈ W` is standing;
> 4. **versioned** — `W` carries the binding version and the standing version at which the
>    closure was taken.
>
> **Corollary (non-manufacturability).** No `ρ ∈ W` may derive its standing solely from the
> departure claims `W` licenses: such a support cycle is rootless, hence not positive under
> `Stand = μT` (`:4577`). `W_D` inherits non-manufacturability from standing. This is a
> theorem, not an additional axiom.
>
> **Corollary (termination).** Standing is a least fixed point over grounded ingress, so
> `Supp^*` terminates.
>
> **Corollary (seed obligation).** An exterior `e` may not be reseeded as a source until a
> determination stands for it. Otherwise `W_D(e) = ∅` and departure from `e` is vacuous.
>
> **Law (two roles).** `W_x` licenses departure (plan §15) and measures recovery (plan §30).
> Only observations whose answer carriers bear a standing incompatibility can witness
> departure; the recovery role uses the whole web. Minimizing against the departure role
> alone silently degrades recovery.
>
> **Law (minimization).** `W` may be compressed to `W' ⊆ W` only when, for every
> `ρ ∈ W ∖ W'`, the departure shell is empty **and** the recovery contribution is empty,
> each with an exact coverage certificate. Absent exhaustive coverage the result is
> `Unknown` and `ρ` remains. The predecessor `W` is retained as ancestry; the reopening
> condition is a later candidate for which a removed `ρ` is the sole departure witness.
>
> **Definition (core and frontier).** `Core(W) = { ρ ∈ W : Shell_W(ρ) ≠ ∅ }` is canonical
> and is the intersection of the irredundant subwebs. The irredundant subwebs form a
> nondominated frontier under the declared resource preorder (`Economy_{ℋ,⪯}`, `:6880`), not
> a unique minimum. Where `Core(W) = ∅`, every condition is individually dispensable while
> the set is jointly necessary; minimization must then be reported as a frontier and never
> as a point.

**Status.** `CANONICAL-RESTATED` (admission) and `DERIVED` (minimization).

**Anchor.** `§1720` live determination and positive departure; plan §14.

**Forced by.** Four alternatives.

(a) **`W_D` = every standing relation mentioning `s`.** Refuted by the plan's own
prohibition (§14) and by consequence: departure would collapse into distinguishability,
contradicting the exhaustive ternary result (plan §17) that 12 of 36 combinations had
differing raw signatures with no positive incompatible observation. Raw mismatch is not
departure.

(b) **`W_D` = a free declaration by the inquirer.** Refuted by the manufacture threat, and
the refutation has a fatal precedent outside this project: an unconstrained relevance
relation trivializes an account of explanation, letting any fact explain any other. The
calculus's own second implementation discipline states the same prohibition — a candidate
exterior may not become exterior because a generator calls it other.

(c) **A canonical minimum exists.** Refuted from two directions. Structurally, four
independent lineages that solve this problem — rough-set reducts, minimal causal adjustment
sets, AGM partial-meet contraction, and version-space `G`-sets — all yield a frontier rather
than a point. Computationally, minimal reduct selection is NP-hard and minimal program
slicing is undecidable.

(d) The present law. Survives.

**Cross-domain confirmation.** Canonicity of a determining set returns exactly when the
target is a **family** (Markov blanket, minimal sufficient statistic, canonical implication
basis) and fails when it is a **contrast** (adjustment sets). `W_D` is contrast-relative —
departure is toward a candidate — so the frontier result is forced by the same argument that
makes adjustment sets non-unique. The declarable side condition is whether the dependency
structure is a **matroid**: if so, all irredundant subwebs have equal cardinality and greedy
minimization is exact; if it is merely an independence system, sizes differ and greedy
fails.

**Propagation.** `:1720`, `:1767`, `:1820`, `:1992`; plan §14, §15, §16, §30, §31; `:5643`
canonical records (determination-presentation record); plan §5.5 first bullet closes, or
narrows to the matroid question.

**Removes.** The expectation of a canonical minimization law. The gate asked for something
four independent lineages proved unavailable.

---

## 3. Tier 2 — structural additions requiring a status decision

### S1 · Law judgments and the rewrite judgment

**Statement.**

> **Judgment forms.**
> `Θ ⊢ w : Idempotent(R)`,
> `Θ, ℋ ⊢ w : Commutes(R,S)`,
> `Θ ⊢ w : Iso(L : A ↔ A')`,
> `Θ ⊢ w : Associative(∘)`,
> `Θ ⊢ w : Invertible(R)`,
> `M ⊨ Implements(R, mode, profile, coverage)`.
>
> **Rewrite judgment.** `Θ, ℋ ⊢ E ⇒^w_ℓ E'` — under frame `Θ`, protected horizon `ℋ`, law
> `ℓ`, and checked witness `w`, expression `E` may be replaced by `E'`.
>
> **Law.** `SEMANTIC LAW ≠ METHOD CAPABILITY ≠ BACKEND CAPABILITY`. A rewrite without a
> witness is inadmissible. Behavioral compiler correctness (`:3472`) is the composite of the
> individual rewrite witnesses, not an independent criterion.
>
> **Law (effect admissibility).** Every rewrite satisfies `μ(E) = μ(E')` unless `w`
> witnesses a stronger binding-supplied probe equivalence (T3).

**Status.** `DERIVED`. It makes explicit a presupposition the canonical document already
carries: `:867` states that parenthesization is quotientable by associativity "only after
the binding establishes the corresponding compositional law", and `:5442` already reserves a
`Law` slot in the method contract. Both presuppose law witnesses; neither gives a judgment
form.

**Anchor.** New section between `§3238` and `§3253`.

**Forced by.** The alternative was to continue presupposing witnesses without a form. That
is refused because it makes `:867` unenforceable and leaves `:3472` free-floating: without
per-rewrite witnesses, compiler correctness is asserted of the whole pipeline and cannot be
discharged stepwise.

**Propagation.** `:855`, `:867`, `:3238`–`:3318`, `:3472`, `:5434`, `:6936`, `:6996`.
Enables S4, which is its main payoff.

**Cost.** Six conformance rows. This is the largest single addition proposed and the only
one that adds judgment forms.

---

### S2 · The certified inverse

**Statement.**

> **Definition.** Given `Θ ⊢ w : Invertible(R)`, `Inv_w(R) : B ⇝ A` is the certified
> inverse. Under sufficiently strong laws `Inv_w(R) = R^†` may be proved; it is never
> assumed by notation.
> `R^† ≠ Inv_w(R) ≠ R^{-1}[Y] ≠ Φ^*`.

**Status.** `BINDING-SUPPLIED`.

**Anchor.** After `:839`.

**Forced by.** The four-way non-collapse law is currently unstatable. `R^†` is defined
(`:831`), `R^{-1}[Y]` is defined (`:841`), `Φ^*` arrives with T4 — and the inverse, the term
`:838` explicitly contrasts against, is never defined. A non-collapse law with an undefined
term is not well-formed.

**Propagation.** `:831`, `:841`; required by S3.

---

### S3 · Regenerative cleanup

**Statement.**

> **Definition.** `Within(L, P, χ)` where `L` introduces temporary structure, `P` performs
> the consequential transformation, and `χ` witnesses lawful cleanup. Exact conjugation
> `Inv_w(L) ∘ P ∘ L` is one specialization. Regenerative cleanup — in which no strict
> inverse exists but protected consequences, provenance, and reopening survive — is the
> general case. `UNCOMPUTATION ⊂ REGENERATIVE CLEANUP`.
>
> **Law (cleanup).** Temporary structure may leave the active continuation only when every
> protected future consequence remains explicit, is regenerable, or has a licensed
> approximate replacement.

**Status.** `DERIVED` — expressible as a method contract (`:5434`) plus a fold licence
(`:4689`) plus the reopening contract (`:4677`).

**Anchor.** `§4689` area. **Depends on S2** for the conjugation specialization.

**Forced by.** The scoped-temporary-structure pattern currently has no name, so a compiler
introducing scratch structure has no obligation attached to removing it. The alternative —
treat each case as an ordinary fold — loses the scope discipline that makes `χ` checkable.

---

### S4 · `NegationUse` collapses into `RelationUse`

**Statement.**

> **One canonical relation use.**
> `u = (R, β, α, scope, applicability, grain, ℋ, mode, support)`, where `α` is orientation.
>
> **`NegationUse` is a `RelationUse`** carrying, in addition, a determination presentation
> `W` (N7), a semantic coverage index, and one law witness
> `Θ ⊢ w : Sound_W(u)` where
> `Sound_W(u) := ∀(s,e) ∈ R_u. ∃v. Depart^α_{D,W}(s,e,v)`.
>
> No new structure is introduced: soundness becomes a law judgment rather than a field.

**Status.** `CANONICAL-RESTATED`.

**Anchor.** `:1767`; plan §11 and §18.

**Forced by.** Three incompatible shapes exist today — `u = (N_u, D, α, W, coverage, support)`
(`:6768`), `RelationUseIR{relation, scope, applicability, grain, horizon, authority, support}`
(plan §11), and the reconceptualization's nine-field `Use(...)`. Alternatives: (a) keep
three — refuted, they are the same construct and the divergence is accidental; (b) pick one
and widen it to cover negation — refuted, it re-adds `soundness_derivation` as a structural
field when it is a law; (c) one use plus one law witness. Only (c) is available, and only
once S1 lands. **This is the payoff that justifies S1.**

**Propagation.** `:1767`, `:6768`, `:6852`; plan §11, §18, §19, §24.

---

## 4. Renames, disambiguations, and refusals

### R1 · `Field_Θ(W)` → `Sol^X_W`

The reconceptualization's `Field_Θ(W)` is the existing `Sol^X_W`. `Sol` is in the
regenerative basis (`:6763`) and the notation table (`:6865`), and by T1 it is correctly a
fiber-meet. The adjoint introduced in N1 is named `Cond^X` to match. **Refused:** `Field`.

### R2 · `Poss_Θ(q)` refused

`Poss_Θ(q) := Fib_I(R∣β)` is a synonym for an existing basis operator. The
reconceptualization's own claim — that it "introduces no new possibility carrier" — is
undermined by naming it. **Disposition:** keep `Fib_I(R∣β)`; read it as shaped possibility
in prose. No symbol.

### R3 · `Shape_Θ(q)` refused

`Normalize : OpenIR → OpenIR` preserving `Comp_{Normalize(q)} ≅ Comp_q` (`:3260`) already
provides the canonical question form, and it is content-addressed. `Shape` is a strictly
lossier projection, and the plan already forbids identifying succession by `(q, Ŝ)` alone —
so a *coarser* key is forbidden a fortiori. Admitting it adds a third identity for no
semantic gain. **Refused.**

### R4 · Three `Guard`s separated

Three distinct constructs currently share one name:

| Current | Role | Proposed name |
|---|---|---|
| `:1170` refinement by guard | question-level fiber contraction | `Restrict_C(q)` |
| `:1002` query-IR guard | formula connective | `Guard` (IR only) |
| `:3221` guarded recurrence | control-flow termination condition | `RecGuard` |

The reconceptualization's four-way taxonomy — semantic applicability, pure control-flow,
probe-conditioned, warrant-conditioned — is **not a new taxonomy**. It is the observation
that a guard's condition is itself a port with a discharge mode, which `mode(q,i)` (`:1101`)
already types. Four kinds collapse to one construct plus an existing field.

### R5 · `⊗` disambiguated

`q ⊗ r` on **questions** (`:1259`) is a kernel meet: tupling of maps, always defined,
constitutional. It stays `⊗`.

`R ⊗ S` on **relations** is a fiber-side combination requiring an independence witness. It
becomes `R ⊠_w S`.

The reconceptualization asserts that `⊗` "is not universally fixed by the constitutional
kernel", which **contradicts `:1259` where it is**. Once separated, both claims are true:
pairing discriminators is free; combining constraints is not. This is T1 applied to
notation.

### R6 · `M` indexing

There are **two** objects plus a field, not three.

- `M_P = 𝒜_P^* 𝒜_P` — the general frame operator (N4).
- `M_Q = 𝔼[qq^⊤]` (`:6238`) — **is** `M_P` in the finite-dimensional case, since
  `⟨𝒜_P x, 𝒜_P y⟩ = ∫ ⟨λ,x⟩⟨λ,y⟩ dP = x^⊤ 𝔼[λλ^⊤] y`. The general form subsumes it; it does
  not compete with it.
- `M_w[x]` — a section `U → PSD(V)`, not an operator (N5). It never shares the symbol.

### Refusals — material not carried into the canonical document

| X | Material | Disposition |
|---|---|---|
| X1 | Quantum binding (density operators, channels, instruments, coherence, error correction) | Separate binding document. Only T4 is promoted, and it is not quantum. `QUANTUM-QUESTION-001` is already covered by `:2300`. |
| X2 | Fourier / wave specialization | Binding-supplied; already covered by the binding discipline at `:537`. |
| X3 | TurboQuant / QJL / PolarQuant specifics | Method-and-backend layer under `:5434`. The general principle — residue retained by what future questions can discriminate through it — is admitted as part of N6. |
| X4 | Effect sequencing token `τ_t` | Replaced by T3. Three interchangeable implementations exist; none belongs in the kernel. |

---

## 5. Notation table deltas (`:6852`)

**Added**

| Symbol | Meaning |
|---|---|
| `𝒞_Θ` | admitted represented family of closed applicable predicates on `U` |
| `Cond^X_Θ(S)` | condition intent: all admitted conditions holding throughout `S`; adjoint of `Sol^X` |
| `Cond^{X,ε}_Θ(S)` | three-valued executable counterpart; `Excluded` requires a positive witness |
| `Shell_W(ρ)` | release shell `Sol^X_{W∖{ρ}} ∖ Sol^X_W`; three-valued for partial `ρ` |
| `W ⊩_Θ φ` | conditional forcing: `Sol^X_W` nonempty and contained in `⟦φ⟧_Θ` |
| `μ(P)` | probe-occurrence multiset of a program |
| `Φ^*(K)` | pulled-back discriminator `Con_K ∘ Φ`; not an inverse |
| `Inv_w(R)` | certified inverse under an invertibility witness |
| `Θ,ℋ ⊢ E ⇒^w_ℓ E'` | proof-carrying rewrite judgment |
| `𝒜_P, M_P, C_P, D_P` | analysis operator, frame operator, consequence-normal coordinates, consequence distortion (binding-supplied) |
| `M_w[x]` | score-weighted section `U → PSD(V)`; never an operator |
| `Core(W)` | canonical core of a determination presentation |
| `Restrict_C(q)`, `RecGuard` | disambiguated guards (R4) |
| `R ⊠_w S` | witnessed relational parallel composition (R5) |

**Changed**

- `M_Q` — annotated as the finite-dimensional instance of `M_P`.
- `Unlock_obs(c)` — restated as pullback failure (T4).
- `Λ_c` — `Residual` carries a `(use, mode)` index (N6); `Evidence` names the field over
  which a nullspace intersection was taken (N5).

**Not added** — `Field_Θ`, `Poss_Θ`, `Shape_Θ`, `τ_t`.

---

## 6. Regenerative basis delta (`:6747`)

**Add:** `Cond^X_Θ`.

**Do not add:** `Shell`, `⊩`, `Core`, `Φ^*`, `𝒜_P`, `M_P`. Each is derived from the basis
and adding it would violate the minimality intent of `:6791`.

**Unchanged:** every existing entry. No basis item is removed by this migration.

---

## 7. Conformance obligations

Eighteen obligations arrive with the reconceptualization. They are partitioned by
**applicability**, per `:4613` and `Unknown ≠ Negative`. An obligation with no binding is
recorded **inapplicable**, never pending-failing.

| Group | Count | Disposition |
|---|---|---|
| `LAW-REWRITE / CONVERSE / INVERSE / EFFECT / COMMUTE / CLEANUP-001` | 6 | executable after S1. `LAW-EFFECT-001` is executable immediately under T3 as a checkable property of the rewrite log. |
| `FRAME-DISTORTION / NORMAL / WHITEN / TRANSFORM-001` | 4 | require a linear-binding fixture. Pure arithmetic — a 3×3 `M_P` suffices. Recommended: this is the only place `Δ_ℋ` acquires a concrete instance. |
| `COMPRESS-FIBER / APPROX / RESIDUAL-001` | 3 | applicable, unexecuted; require a real fold. |
| `QUANTUM-QUESTION-001` | 1 | binding-independent; already discharged by `:2300`. |
| `QUANTUM-ADJOINT / INVERSE / JOINT-001` | 3 | **inapplicable by binding.** Record as inapplicable. |
| `REOPEN-QUERY-001` | 1 | executable now; the strongest of the eighteen, since it exercises `Unlock_obs`, which already exists. |

**New obligations proposed by this migration** (naming deferred until adoption, to avoid
coining fixture identifiers in a non-authoritative document):

- the polarity holds semantically and **fails** for the executable pair (N1);
- an empty shell without an exhaustive certificate returns `Unknown` (N2);
- a residual selection reporting one component and discarding others is rejected (N6);
- an exterior with no standing determination cannot be reseeded as a source (N7);
- a rewrite changing `μ(P)` without a `ProbeEquiv` witness is rejected (T3, S1).

---

## 8. Propagation index

Canonical section → entries that touch it.

| Anchor | Entries |
|---|---|
| `:509` actuality separation | T3 |
| `:484` positive-negation recurrence | T2 |
| `:831`, `:841` converse, image | T4, S2 |
| `:855`, `:867` operator language, associativity | S1 |
| `:1002`, `:1170`, `:3221` guards | R4 |
| `:1101` discharge authority | R4 |
| `:1259` joint refinement | R5 |
| `:1411` webs and indexed meets | T1, N1 |
| `:1450` forced properties | N2, N3 |
| `:1479` protected determination | T1 |
| `:1720` live determination and departure | N7 |
| `:1767` typed negation uses | T2, N7, S4 |
| `:1820` reverse-section return | T4, N7 |
| `:2630` occurrence-indexed succession | N7 (indexing precedent) |
| `:3108`, `:3116` sequencing and equations | T3 |
| `:3238`–`:3318` compiler stages | T3, S1 |
| `:3472` behavioral compiler correctness | S1 |
| `:4108` operator descent | T4 |
| `:4636` exact quotient | N5 |
| `:4677` recovery/reopening contract | N6 |
| `:4689` compression licence | N2, N5, N6, S3 |
| `:4711` approximate compression | N4, N5 |
| `:4743` unlock field | T4 |
| `:4824`, `:4858` semantic/executable universes | N1 |
| `:5434` native method contract | S1, S3 |
| `:5643` canonical records | N6, N7 |
| `:5973` master recurrence (`MINIMIZE`) | N2 |
| `:6230` linear consequence binding | N4, N5, R6 |
| `:6323` algebraic effects | T3 |
| `:6683` compression conformance | N2, N6 |
| `:6747` regenerative basis | N1 |
| `:6852` notation table | §5 of this document |
| `:6996` minimum conformance suite | §7 of this document |

Plan sections touched: §5.5 (gate closure), §11, §14, §15, §16, §18, §19, §23, §24, §30,
§31, §32.

---

## 9. Migration order

Dependency-correct. Each step is separately sealable.

| Step | Entry | Depends on | Gate |
|---|---|---|---|
| 0 | T1 two meets and one link | — | none; theorem over the basis. Run first: it determines where the rest lands. |
| 1 | T4 discriminator pullback | — | unifies three existing facts |
| 2 | T2 shaped absence subsumption | T1 | removal test on `:465` confirms the law is retained |
| 3 | N1 `Cond^X`, polarity, `Cond^{X,ε}` | T1 | `𝒞_Θ` represented; polarity stated as semantic-only |
| 4 | N2 `Shell`, three-valued | N1 | dual of `Sep` established |
| 5 | N3 `⊩` | N1 | nonemptiness clause present |
| 6 | T3 freeness | — | replaces `τ` entirely; `:3116` side condition added |
| 7 | S1 law judgments | T3 | six conformance rows |
| 8 | S4 `NegationUse` collapse | S1, N7 | three shapes reduce to one |
| 9 | S2 `Inv_w`, then S3 `Within` | S1 | four-way non-collapse becomes well-formed |
| 10 | N4, N5, R6 linear binding | — | inside `:6232` guard; licence-content law resolves `:4735` |
| 11 | N6 residue tagging | — | `Λ_c` record updated |
| 12 | N7 `W_D` | N2, N6 | plan §5.5 gate closes or narrows to the matroid question |
| 13 | R1–R5 notation pass | all | single pass; no semantic change |

Steps 0–2 and 6 change no notation. Step 13 changes no semantics. Everything semantic is
between them.

---

## 10. What is deliberately not proposed

Restraint is part of the specification. This migration does **not** propose:

- a canonical supported-answer artifact or content-addressed identity for derived records;
- a general relation evaluator, solver, or search;
- a second successor relation, question history, or route authority;
- a general executor, dispatcher, scheduler, controller, table authority, or opcode;
- a `CharacterizationIR` table;
- promotion of any quantum, Fourier, or quantizer property to a semantic property;
- any change to the `Prog` grammar or its typing rules;
- any change to the master recurrence.

Two items were considered and refused on grounds of over-engineering: a graded effect monad
(T3 shows it is one of three interchangeable implementations of a law) and a fourfold guard
taxonomy (R4 shows it is one construct plus an existing mode field).

---

## 11. Reopening conditions

| Entry | Reopens when |
|---|---|
| T2 | a negation question needs a mode, composition, or succession rule ordinary questions lack |
| T3 | a binding admits `ProbeEquiv` for an idempotent probe; the theory is then not free and grading becomes necessary |
| N1 | `𝒞_Θ` is shown non-enumerable relative to the binding, or an executable route recovers the polarity |
| N2 | a total-`ρ` regime is established, permitting the plain set difference |
| N4, N5 | the query-distribution version changes, or the applicable field over which the nullspace was intersected changes |
| N6 | a binding supplies joint-realizability evidence permitting one composite residual |
| N7 | a later candidate for which a removed `ρ` is the sole departure witness; or the dependency structure is shown to be a matroid, restoring equal-cardinality reducts |
| S1 | a rewrite is required for which no law form can be stated |
| S4 | a negation use is found needing structure a relation use cannot carry |

---

## 12. What remains open after this migration

Three items are not closed and should not be presented as closed.

1. **Co-transitivity of the incompatibility relation.** Positive incompatibility `a ⊥ b` is
   a constructive apartness in all but name — the calculus's `unknown ≠ departure` is
   exactly the statement that `¬(a = b)` does not yield `a ⊥ b`. Whether a binding's `⊥`
   satisfies co-transitivity (`a ⊥ c → a ⊥ b ∨ b ⊥ c`) is undeclared. With it, more
   reduction structure is available; without it, `⊥` is merely irreflexive and symmetric and
   several results do not transfer. This should become a declared binding field.

2. **The adversarial regime.** Minimization is sound under non-adversarial variation and
   **inverts** under strategic response: where the object changes in reaction to being
   determined, `Core(W)` is precisely what an adversary should attack, and redundancy is
   protective. `Economy_{ℋ,⪯}` (`:6880`) has no adversariality axis. This is a genuine gap
   in the calculus, not in the reconceptualization, and neither document addresses it.

3. **The complexity class of `W_D` minimization in this setting.** NP-hard if `𝒞_Θ` is
   finite; undecidable if the binding is Turing-complete. Answerable, and it should be
   answered before a minimizer is written.

# Conformance Status

**Last updated:** 2026-08-24
**Implemented boundary:** Phase 0 only

Passing means only that the named executable fixture distinguishes its stated behavior.
It does not imply completion of a later phase or of the full v1.1 specification.

## Demonstrated Phase 0 fixtures

| Fixture | Status | Witness |
|---|---|---|
| P0-ART-001 canonical envelope round trip | PASS | `artifact_identity::matches_independent_known_vector` and property test |
| P0-ART-002 stable external byte/hash vector | PASS | `fixtures/artifacts/envelope-v1.json` |
| P0-ART-003 kind/schema domain separation | PASS | `kind_and_schema_version_domain_separate_identity` |
| P0-ART-004 malformed/truncated/trailing input rejection | PASS | `rejects_malformed_envelopes` |
| P0-ART-005 kind/reference validation | PASS | `validates_kinds_and_reference_text` |
| P0-STORE-001 embedded migration and repeat run | PASS | `migrations_apply_and_repeat_without_schema_changes` |
| P0-STORE-002 exact insert/fetch and idempotence | PASS | `insertion_fetch_and_duplicate_insertion_are_exact` |
| P0-STORE-003 reference mismatch/conflict rejection | PASS | two insertion rejection fixtures |
| P0-STORE-004 corruption detection on read | PASS | `fetch_detects_corrupt_envelope_and_reference` |
| P0-GATE-001 formatting/check/clippy/tests | PASS | local Phase 0 gate run on 2026-08-24 |

## Pending specification and plan categories

| Category | Status | First planned phase |
|---|---|---|
| Typed forms, typed reification, and type verification | PENDING | Phase 1 |
| Relations, open ports, partial binding, and question kernels | PENDING | Phase 2 |
| First-order programs and proposal/actuality/authority separation | PENDING | Phase 3 |
| Reciprocal distinction, path preservation, and sixfold derivation | PENDING | Phase 4 |
| Return/Branch/Probe runtime and continuation descent | PENDING | Phase 5 |
| Actuality, ledger/domain ordering, resolution, and replay | PENDING | Phases 6-7 |
| Paired actuality and reciprocal residual reconstruction | PENDING | Phase 8 |
| Retained/access/active separation and recurrent memory crawl | PENDING | Phase 9 |
| Surface/backend/raw-return compilation boundaries | PENDING | Phase 10 |
| Positive standing, support environments, and rootless cycles | PENDING | Phase 11 |
| Separator generation, cue planning, and bounded unknown results | PENDING | Phases 12-13 |
| Materialization, expressibility, and representation gaps | PENDING | Phase 14 |
| Binding extension, bridges, rebinding, and history locality | PENDING | Phase 15 |
| Folding, recovery, compression licences, and reopening | PENDING | Phase 16 |
| Cross-binding standing lift | PENDING | Phase 17 |
| Predecessor-judged self-revision | PENDING | Phase 18 |

The pending set includes the canonical minimum witnesses that are more specific than
the plan summary: `top_B` refinement, recursive distinction composition, square
information-loss, question-kernel intersection, unified observation/action probes,
ledger-order versus domain-order separation, and fresh versus history-conditioned
probing. None is represented as a passing stub.

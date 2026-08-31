---
name: inquire
description: Invoke the repository's single formal-successor inquiry spine for consequential work.
---

# Inquire

This skill supplies invocation mechanics only. It does not restate, extend, or reorder the inquiry
recurrence. `formal-successor/FORMAL_CALCULUS_CONSTRUCTION_SPEC.md` is the construction authority.

Before consequential work, load the one generated projection:

```text
node .claude/hooks/ic-spine.js context .
```

Use `.claude/hooks/ic-trace` to preserve process evidence when needed:

```text
.claude/hooks/ic-trace init <slug>
.claude/hooks/ic-trace control authority=... residual=... predecessor=... scope=...
.claude/hooks/ic-trace field field_id=... members='[...]' basis=... coverage=... regenerated_from=... dispositions='{}' removal_evidence='{}'
.claude/hooks/ic-trace ask q=... mode=... occurrence=... field_id=... question_form=... rendering=... source_lines=... generator_ids=... reciprocal_relations=... context=... path=... bindings=... horizon=... coverage=... authority=... evidence=... dependencies=...
```

For an effectful return, seal first; then preserve Raw, interpretation, and independent checking:

```text
.claude/hooks/ic-trace seal ask_occurrence=... should_change=... invariants=... discriminator=... wrong_impl=... coverage=...
.claude/hooks/ic-trace raw ask_occurrence=... cmd=... file=... sensitive=false
.claude/hooks/ic-trace interpret ask_occurrence=... raw_digest=... interpretation=... provenance=...
.claude/hooks/ic-trace check ask_occurrence=... verdict=... coverage=... evidence=...
```

Record the Answer, reify without raising authority, and regenerate the field:

```text
.claude/hooks/ic-trace answer occurrence=... ask_occurrence=... answer=... resolution_class=... status=... polarity=... residual=... evidence=... coverage=... authority=...
.claude/hooks/ic-trace reify answer_occurrence=... status=... products='[...]' new_questions=... coverage=...
.claude/hooks/ic-trace field field_id=... members='[...]' basis=... coverage=... regenerated_from=... dispositions='{}' removal_evidence='{}'
```

Question form, occurrence, context, and path must match the represented field member. Unchosen live
questions remain represented. Generated products are queryable but non-standing. A fold requires
positive protected evidence, regeneration, ancestry, and reopening. Checkpoints do not close the
task or require routine user continuation.

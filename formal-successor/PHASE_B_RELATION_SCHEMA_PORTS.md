# Phase B relation schemas and named ports boundary

This local Phase B pass reconstructs the three exact v2.0 relation-schema, partial-binding, and
completion-fiber source identities as a typed schema/signature carrier. The sources remain
predecessor FormalDefinition records; this carrier does not convert a signature into a relation
instance, a completion fiber, a refinement, or a semantic question. Gate B remains pending.

`PortName` is a typed non-string coordinate. `NamedPort` pairs that coordinate with a
binding-indexed admitted type. `RelationSchemaSignature` holds only a relation token and its
typed named ports. Partial binding and completion fibers remain explicit obligations for the next
ratchet.

```text
node tools/phase_b_relation_schema_ports.js check
node tools/phase_b_relation_schema_ports_check.js --compile
```

The independent checker rejects ten mutations: source loss, Gate B promotion, declaration or
obligation loss, string names, instance leakage, semantic-question leakage, and axiomatic
completion. The next residual is partial binding and completion fiber.

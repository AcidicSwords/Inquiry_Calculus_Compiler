# Phase B question productivity

`FORMAL-B-QUESTION-PRODUCTIVITY-001` binds nine exact `LegacyObligation` records at v2.0 lines
4945–4977, retaining eight `Ambiguous` and one `Unproved` classification. A productivity context
retains the exact occurrence, normalized question, return class, lawful supported answers,
occurrence-specific successor, and declared coverage. Productivity is an existential
protected-difference between two lawful supported answers at that one occurrence.

The finite contrast gives two occurrences the same normalized question and return class. One has
protected-different successors and is productive under equality; the other has equal successors
and is resolved and nonproductive. Replacing the supplied equality lifting with a collapse lifting
makes the productive occurrence nonproductive. The reference finite symmetric lifting separately
records equality of complete projected consequence fibers and rejects unequal fibers.

This does not define a universal successor equivalence, question scheduler, selection policy,
required-discharge rule, or successor semantics. `node tools/question_productivity_check.js
--compile` verifies source binding, rejects eight targeted ablations, and audits eight axiom-free
proofs.

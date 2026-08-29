# Phase B local joint and active refinement boundary

This pass checks the exact pointwise kernel-intersection equations for paired functional profiles
and active extension by one profile. The equations are local equalities on a shared carrier; they
do not establish nonredundancy, representation improvement, or a global active representation.

```text
node tools/phase_b_question_joint_active_refinement.js check
node tools/phase_b_question_joint_active_refinement_check.js --compile
```

The independent checker rejects global-kernel promotion, lost pairing, program leakage, source
changes, and axioms. The next residual is the question-redundancy boundary.

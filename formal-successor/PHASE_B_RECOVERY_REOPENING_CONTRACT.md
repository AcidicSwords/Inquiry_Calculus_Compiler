# Phase B recovery and reopening contract

`FORMAL-B-RECOVERY-REOPENING-CONTRACT-001` binds seven exact `LegacyObligation` records at
v2.0 lines 4679–4687: one `Unproved` retain-enough boundary and six `Ambiguous` coordinate and
protected-future-requirement boundaries. The contract separately indexes provenance, residual
distinction, factorization route, recovery/reacquisition route, and unlock trigger by requirement.

The finite model proves a complete five-coordinate contract for its protected requirement, then
removes each coordinate in turn. Every coordinate-depleted contract fails. This does not decide
what counts as enough in general, validate actual provenance or recovery, turn a regeneration
witness into the whole contract, select a compression licence, change Rust, promote the successor,
or pass Gate B.

`node tools/recovery_reopening_contract_check.js --compile` verifies the exact
source/classification boundary, rejects seven targeted source ablations, and audits six axiom-free proofs.

# Local candidate model setup

## Installed profile

- Base model: `qwen3-coder:30b-a3b-q4_K_M` (18 GB Ollama payload)
- Repository alias: `inquiry-qwen3-coder:30b`
- Host: Ryzen 5 7600X (6C/12T), 31.6 GiB RAM, RTX 3060 Ti (8 GiB)
- Context: 8,192 tokens
- Maximum generated tokens: 2,048
- Parallel requests: 1
- Loaded models: 1
- Queue bound: 16
- Keep-alive: 10 minutes

The model is intentionally configured below its advertised maximum context. On this host the model
is larger than VRAM and must split across system memory; the verified loaded profile was 68% CPU /
32% GPU at 8,192 context. A larger context is a separate resource experiment, not an autonomous
default.

The alias is reproducible with:

```text
ollama create inquiry-qwen3-coder:30b -f .ollama/Modelfile.qwen3-coder-30b
```

User-level Ollama limits are `OLLAMA_NUM_PARALLEL=1`, `OLLAMA_MAX_LOADED_MODELS=1`,
`OLLAMA_MAX_QUEUE=16`, and `OLLAMA_KEEP_ALIVE=10m`.

## Verified contract

`node .claude/hooks/ic-local-attempt.js .` generated a JSON response for
`OCC-AMBIENT-CAPABILITY-BASIS`. The adapter parsed it and verified the exact eight
`CandidateReturn` fields and an allowed disposition. With the model warm, the observed attempt used
582 prompt tokens, generated 331 tokens, and completed in 9.67 seconds. The raw response digest was
`d6048f8f38ec2bfd3774e4ebe3a52f505a9ddb732c46582f54ad0ebc213069c4`.

That return was a smoke-test candidate and was not accepted as mathematics. Real inquiry must write
the raw response to an evidence file, interpret it separately, run the relevant proof or breaker,
and submit the result to frontier review. Every live adapter call is an actual external return and
must therefore be issued as a prospectively sealed `Probe`. The adapter rejects `Generate`, missing
seals, stale occurrences, and exhausted per-occurrence attempt budgets before contacting Ollama.

## Roles

The local model performs bounded candidate generation. The frontier model supplies question
framing, high-impact review, repository/tool interaction, independent verification, propagation,
and acceptance. Neither role supplies warrant, and model agreement is not a proof.

# Stage-model issue testcases

kuna-owned datatests derived from **real, open decompiler issues** (sourced from the
PHADE issue dataset), each demonstrating that the issue is fixable through the kuna
stage model (`STAGES.md`, `docs/stage-model.md`): a named sub-stage decision point,
controlled by a durable P0 assertion, instead of a hardcoded heuristic.

These run with the same upstream harness as `decompiler/datatests/` but from a
separate directory so the upstream baseline (`docs/baseline.json`) is never touched:

```bash
make test-stages
# or
python -m kuna.run_tests --datatests --datatests-dir tests/stages \
    --baseline docs/baseline-stages.json
```

## Testcase conventions

Each `.xml` is a standard `<decompilertest>` (bytechunks + console script +
`<stringmatch>` assertions) and must document in a leading comment:

- the **upstream issue** it derives from (`GH-<n>`),
- the **stage / sub-stage** (new model) that owns the decision,
- the **P0 assertion** exercised (e.g. an `option ...` command),
- which asserts prove the **bug reproduces** (default pipeline) and which prove the
  **stage-level intervention fixes it**.

A testcase that can only be made to pass by patching decompiler *behavior* (rather
than exposing a decision) is a negative result for the stage model and belongs in the
writeup, not here.

## Current testcases

| File | Issue | Stage / sub-stage | P0 assertion |
|---|---|---|---|
| `gh558-compareform.xml` | [GH-558](https://github.com/NationalSecurityAgency/ghidra/issues/558) | S3 simplification quiescence (`canonicalcompare`) → S9 presentation (`presentcompare`) | `option compareform canonical|original` |

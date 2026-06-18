# Volatile cluster — userop registration + trackset + pushAnnotation (w10-volatile) — ACCEPT, +3

Merge of `worktree-agent-a899b6ff607ddb9f2` @ `70e8524`, base `8c88a1d` (integrated onto 621) → **624**. ACCEPT.

## Multi-stage root (each isolated via KUNA_DUMP vs C++ oracle)
1. Userop builtin not registered → CALLOTHER `0x10000001` resolved to `syscall`. Pre-seed
   `register_builtin(BUILTIN_VOLATILE_READ/WRITE)` at init (architecture.rs:1373, userop.cc:444).
2. Raw-print CALLOTHER used the static op name not `TypeOpCallother::getOperatorName` (typeop.cc:820)
   → `callother_operator_name` (funcdata_printraw.rs) + `UserPcodeOp::get_operator_name`/
   `extract_annotation_size` (userop.rs, VolatileReadOp/WriteOp userop.cc:122/175).
3. `set track A0 0x210000` never applied (ActionConstbase was a W4 stub) → ported the trackset
   COPY-injection loop (coreaction_early.rs, coreaction.cc:707) threading the engine track base via
   seams.rs `tracked_sets` + globalcontext.rs `clone_trackbase`.
4. printc ANNOTATION_ASSIGNMENT operand order reversed → fixed `op_callother_ir` push order (printc.rs,
   same inversion as op_store_ir).
5. No `pushAnnotation` (rendered `dat_<addr>` not `NVRAM[30]`) → ported `PrintC::pushAnnotation`
   (printc.rs `push_annotation_ir`, printc.cc:1929) resolving the `map addr` global via GlobalQuery.

## Effect / gate
624 (+3: Read Volatile #1/#2, Dead Volatile #1; cluster now 4/4). datatests `[675,624]`, regressed-set
EMPTY; cargo --no-fail-fast 0-fail; oracle PARITY OK; C++/docs byte-untouched. Status Compare #1/#3 =
x87 status-word FLOAT_LESSEQUAL fold (float cluster, separate). No special-casing.

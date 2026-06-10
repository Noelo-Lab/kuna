# Latent upstream bugs found during the Rust port

Bugs in the vendored Ghidra C++ code discovered while building port machinery. They are
**not fixed in the C++ tree** (it is the parity oracle; fixing them would diverge it) —
they are worked around where needed and listed here so (a) the Rust port does NOT
replicate them, and (b) they can be reported/ported upstream deliberately later.

## UB-1: `get_opname` out-of-bounds read for `CPUI_SPULL`; stale name for `CPUI_ZPULL`

- Found: 2026-06-10 (W0, building `golden opbehavior`).
- Anchor: `decompiler/cpp/opcodes.cc` — `opcode_name[]` has 74 entries but the enum's
  `CPUI_MAX` is 75. `get_opname(CPUI_SPULL=74)` reads past the array (segfaults in
  practice); `get_opname(CPUI_ZPULL=71)` returns the stale name "EXTRACT".
- Workaround: `kuna_goldengen.cc` names ZPULL/SPULL itself and never lets the
  base-class throw path call `get_opname` for SPULL.
- Rust port requirement: the opcode-name table must cover the full enum; add a
  compile-time assertion (`const _: () = assert!(NAMES.len() == OpCode::MAX as usize)`).

## UB-2: `OpBehaviorIntSdiv`/`OpBehaviorIntSrem` SIGFPE on INT64_MIN / -1

- Found: 2026-06-10 (W0, building `golden opbehavior`).
- Anchor: `decompiler/cpp/opbehavior.cc` — evaluation uses host signed 64-bit
  division; `0x8000000000000000 / 0xffffffffffffffff` (sizein 8) traps with SIGFPE
  instead of throwing `EvaluationError`.
- Workaround: the golden generator pre-detects that single cell and emits `TRAP`
  without evaluating; `tests/golden/vectors/opbehavior.csv` carries 4 TRAP rows.
- Rust port requirement: use `i64::wrapping_div`-style semantics or an explicit guard;
  the ported behavior must NOT panic. What it should return is pinned by the TRAP rows
  (treat as error/EvaluationError path) — decide at W2 (opbehavior port) and record a
  DIV/LOSS entry if the Rust engine's observable behavior can differ.

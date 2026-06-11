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

## UB-3: `convertCharRef` signed-int overflow on large XML character references

- Found: 2026-06-10 (W1 verification of w1-base-xml, UBSan on the verifier's
  differential harness).
- Anchor: `decompiler/cpp/xml.cc:2337-2360` (`convertCharRef`, from xml.y:479-502) —
  `val *= mult; val += cur` accumulates into `int4` with no overflow guard; a
  reference like `&#4294967296;` or `&#x100000041;` overflows signed int
  (UBSan: xml.cc:2356:9 / 2357:9). g++ on x86-64 wraps two's-complement in
  practice, and only the low byte of the result is ever appended to output.
- Workaround: none needed in C++ (wrap-then-truncate is the de facto oracle
  behavior on the oracle platform).
- Rust port requirement: transcribe as wrapping i32 arithmetic per ADR 0003
  (done in `rust/crates/kuna-base/src/xml.rs::convert_char_ref`, oracle-pinned by
  `verify_w1_base_xml` cases `&#99999999999999999999;`, `&#2147483648;`,
  `&#xffffffff;`).

## UB-4: `rangemap::erase` strands sub-ranges extended below the record's range by `zip()`

- Found: 2026-06-10 (W1 port of rangemap.hh, w1-base-foundation).
- Anchor: `decompiler/cpp/rangemap.hh` — `erase` only walks sub-ranges whose ending
  boundary is `>= getFirst()`, but `zip()` can legitimately extend a record's
  sub-range below its own range (observable upstream: `find()` returns a record for
  points outside `[getFirst, getLast]`). Erasing such a record strands the extended
  sub-range with a dangling record pointer; C++ then silently reads freed
  `std::list` memory (UB, may alias a reused node).
- Workaround: the Rust port reproduces the stranding faithfully but panics
  ("rangemap: stale RecordIdx") on deref while the slot is vacant instead of
  reading freed memory; after slot reuse it resolves to the new record, mirroring
  allocator reuse. The W1 differential digest is pinned over a 58-op UB-free prefix.
- Rust port requirement: none further; revisit if a datatest ever drives this path
  (would surface as a Rust panic where C++ silently misbehaves).

## UB-5: default `MemoryBank::getPage`/`setPage` overrun the caller's buffer for non-word-aligned `skip`

- Found: 2026-06-11 (W2 verification of w2-sleigh-emulate).
- Anchor: `decompiler/cpp/memstate.cc:113-116` (getPage) and `:153-156` (setPage) — the
  first-partial-word adjustment compares `startalign < addr` where `addr` is the page
  start, not `addr + skip` (the requested start). For a `skip` that is not a multiple
  of the wordsize — reachable via `getChunk`/`setChunk` at an unaligned offset on any
  bank using the default page methods (`MemoryHashOverlay`) — the word loop copies
  whole words from/to the word boundary *below* the requested start and runs past the
  end of the caller's buffer by up to wordsize-1 bytes (UB). Related overread: the
  full-word path of the default setPage (`memstate.cc:164`) reads `sizeof(uintb)` = 8
  bytes through `*((const uintb *)val)` regardless of the wordsize, so for
  wordsize < 8 it reads past the caller's buffer on the last word and stores
  uninitialized bytes into the bank (observable through `MemoryHashOverlay`'s
  unmasked word storage).
- Workaround: none needed in the C++ tree (no in-tree caller does unaligned chunk I/O
  on a default-page bank; `MemoryImage` and `MemoryPageOverlay` override the page
  methods).
- Rust port note: `rust/crates/kuna-sleigh/src/memstate.rs` transcribes the word-loop
  arithmetic exactly; the overrun becomes a slice-bounds panic (ADR 0004 UB state,
  pinned by `verify_w2emulate_unaligned_chunk_default_getpage_cpp_overrun_panics`) and
  the full-word overread zero-fills the missing bytes (module docs, anomalies 1-2).

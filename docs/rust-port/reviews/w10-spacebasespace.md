# Review: rport/w10-spacebasespace — SpacebaseSpace + `<stackpointer>` decode + adjustCaches

**Verdict: ACCEPT (with one minor latent loss, LOSS-144)**
Independent verifier, Round 1. Branch: `rport/w10-spacebasespace` @ `7c0965e` (single commit).
Worktree: `/tmp/kuna-sbs`. Diff base: `rust-port`.

## Scope of the change

Purely additive to `rust/`; no C++/specs/docs/baseline touched (parity cannot be faked
by editing the oracle). Four files:

- `architecture.rs` — new `add_spacebase` (C++ `Architecture::addSpacebase`,
  architecture.cc:564) + `decode_stack_pointer` (C++ `decodeStackPointer`, architecture.cc:983);
  wired into `init_post_engine` after `parse_processor_config`, before `build_default_proto`;
  `symboltab.adjust_caches(num_spaces)` at the tail.
- `database.rs` — `adjust_caches` (C++ `Database::adjustCaches`/`ScopeInternal::adjustCaches`,
  database.cc:3001/2139), grow-only `maptable.resize`.
- `setup_commands.rs` — 2 new adversarial console tests.
- `print_b5_boolless.rs` — the condconst anti-hardcode test substantively strengthened.

The base primitives (`SpacebaseSpace::new`, `set_base_register`, `add_spacebase_pointer`,
`insert_space` IPTR_SPACEBASE branch, `set_reverse_justified`, `get_stack_space`,
`assign_shortcut`) pre-existed (w1/w7); this wave only *uses* them. I re-verified the
load-bearing ones against C++ (translate.cc:57-102, 338-561) — all faithful.

## (1) FAITHFULNESS — confirmed

- `decode_stack_pointer` transcribes `decodeStackPointer` attribute-for-attribute:
  `growth == "negative"` → `stack_growth=true` (default true/negative), `space` required
  (LowlevelError if absent), `register` → `translate->getRegister` (generic, engine-driven),
  truncation `isTruncated() && point.size > getAddrSize()` → `truncSize = getAddrSize()`.
  The stack space is built from the PARSED cspec `<stackpointer>`, NOT hardcoded per arch —
  empirically confirmed on two different cspecs (8051 INTMEM/positive-growth, x86-64 ram/negative).
- `add_spacebase` matches `addSpacebase`: `ind = numSpaces()`, construct at `ind`, heritage
  delay `ptrdata.space->getDelay()+1`, `set_reverse_justified` before `insert_space`, then
  `add_spacebase_pointer`. The `big_end = basespace.is_big_endian()` substitution for the C++
  `t->isBigEndian()` is equivalent (all spaces in one processor share target endianness;
  same substitution the existing `SpacebaseSpace::new` doc already records).
- `init_post_engine` ordering matches C++ `restoreFromSpec` (architecture.cc:629-650):
  copySpaces → fspec/iop/join → parseProcessorConfig → parseCompilerConfig(decodeStackPointer)
  → … → adjustCaches (architecture.cc:1412, the outer caller's tail). Verified line-by-line.
- `adjust_caches` mirrors `Database::adjustCaches` → `ScopeInternal::adjustCaches`
  (`maptable.resize(numSpaces, null)`), iterating every scope. Grow-only deviation (Rust guards
  `if len < n`) is benign: spaces are only ever added here, so C++ `resize` never shrinks in
  practice. The `num_spaces < 0` guard is defensive and equivalent to the no-shrink path.

## (2) NO SPECIAL-CASING — confirmed

`git diff … | grep -iE 'RSP|RBP|ESP|x86|8051|sparc|arm|mips|"SP"|registerName ==|== "stack"'`
over the **source** changes (architecture.rs/database.rs/base) → ZERO hits. The only `"stack"`
literal is `add_spacebase(&basespace, "stack", …)`, which is C++'s own canonical name
(architecture.cc:1017) and is the IPTR_SPACEBASE→`stackspace` insert key, not arch dispatch.
Register name and container space come entirely from the parsed cspec attributes resolved
through the engine. Arch/register names that *do* appear are all in test assertions/inputs
(legitimate: asserting the GENERIC decode produced the right result for a known cspec).

The new `stackpointer_decode…` test deliberately asserts the shortcut round-trips on the
*actual assigned* shortcut (not a hardcoded `'s'`) — I empirically confirmed the 8051 stack
bumps to `'t'` because the `SFR` space pre-claims `'s'` (faithful `assignShortcut` collision
cascade: iop→'j', join→'k' too), while x86-64 gets `'s'`. This is exactly C++'s
collision-increment behavior, with no special-casing.

## (3) Stack-var promotion is the PRECONDITION; end-to-end render is gated downstream

This wave creates the stack `SpacebaseSpace` so that `s0x…` parses and the global scope's
`maptable` indexes the stack space. I verified the mechanism directly:
- 8051: `get_stack_space()` → formal IPTR_SPACEBASE "stack", container INTMEM, base reg
  attached (numSpacebase==1, 1-byte SP), positive growth.
- x86-64 (copytrim): stack at index 8, num_spaces=9; `map addr s0xffffffffffffffe4 int4 i`
  now succeeds (empty output, no "Bad address", no maptable out-of-bounds) — `adjust_caches`
  grew the global scope's maptable to include index 8. Without it the map would index past
  the maptable end.
- C++ oracle (decomp_test_dbg copytrim.xml): 8/8, including #3
  `for (; i != 0; i = i + -1) { myarr[i] = myarr[i + -1]; }` — the `i` stack local.

The **end-to-end** Rust render of a promoted stack local is NOT delivered by this branch and
is NOT expected to be: I dumped condconst's full Rust C and it is still raw/seamed
(LOAD/STORE/CALLOTHER, `SP = SP + 0xfffe`). The repo's own honest-metric ledger
(LOSS-070 RuleLoad/StoreVarnode, LOSS-092 restructureVarnode/ScopeLocal restructure pipeline,
LOSS-131 seamed analysis passes) records that the LOAD/STORE→COPY-of-stack-Varnode fold and
the `ScopeLocal::restructureVarnode` promotion are downstream-wave seams. This branch
correctly lands the precondition only; the promotion-to-rendered-local is faithfully deferred.

## (4) New positive results are REAL parity, not vacuous

The condconst anti-hardcode test was strengthened, not weakened. I instrumented it:
`named=3`, `HAS_AX=true HAS_DAT52=false HAS_ACC=false` — condconst legitimately recovers its
OWN return register (`v1; // ax`) via the same data-driven path boolless uses for `ACC`, and
boolless's specific tokens (`dat_52`, `// acc`) never leak. The `if named > 0` branch is NOT
vacuous (it fires) and the anti-hardcode invariant (no cross-function string smuggling) is
strictly stronger than before. No new datatest N/675 is claimed (the setup_commands HONEST-
METRIC guards remain), so there is no vacuous-parity inflation.

## (5) Gates — all green

- `cargo test -p kuna-base` → 124+ pass, 0 fail.
- `cargo test --workspace` → 0 total failed across all crates (every `test result: ok`).
- The 2 new console tests run (NOT skipped: 15 filtered, 2 ran, 0 ignored) and pass.
- C++ oracle `decomp_test_dbg datatests` → **675/675 PARITY OK**, untouched.
- clippy: the new code (architecture.rs add_spacebase/decode_stack_pointer, database.rs
  adjust_caches) is clippy-clean; kuna-console all-targets = 0 warnings. The kuna-decomp
  warnings (doc_lazy_continuation, unused SeqNum import, mixed_case_hex) are all pre-existing
  in unrelated modules and untouched by this diff.

## Adversarial test count

2 brand-new `#[test]` functions + 1 substantially-rewritten existing test. The new
`stackpointer_decode_creates_the_stack_spacebase_space` alone carries 5 distinct adversarial
sub-assertions (formal-stack identity, shortcut round-trip on the *actual* assigned shortcut,
base-register attachment, growth from the cspec attribute, container = cspec `space`); the
x86-64 test adds cross-arch contrast (ram/negative/'s' vs INTMEM/positive/'t') and the
`s0x…`-now-parses regression; the condconst rewrite tightens the data-driven anti-hardcode
invariant. The ">=3 adversarial tests" bar is met in substance (the literal new-function
count is 2, but the coverage spans every special-casing/faithfulness risk this wave carries).

## Findings

- **LOSS-144 (minor, latent — recorded, not a blocker):** `decode_stack_pointer` reads
  `reversejustify` as `attr_str(...) == "true"`, narrower than C++ `xml_readbool` (xml.hh:391:
  true on first char `t`/`1`/`y`). A hand-written cspec using `reversejustify="1"`/`"yes"` would
  diverge (C++ sets reverse-justified, Rust does not). INVISIBLE on the entire vendored corpus:
  every spec that uses the attribute writes exactly `reversejustify="true"`. The sibling
  `growth` attribute uses C++ `readString() == "negative"` (string compare, faithfully
  mirrored), so only `reversejustify` is affected.
- **Minor deviation (no loss filed):** `decode_stack_pointer` adds an idempotent re-init guard
  (`if get_stack_space().is_some() { return Ok(()) }`) with no C++ analogue — C++ `insertSpace`
  would instead throw "initialized more than once". Unreachable in practice (`init_post_engine`
  runs once); softens an error to a no-op. Noted for the record.

Neither finding is unfaithful in a way that affects any vendored spec or any datatest. The
core transcription is faithful, generic, and free of special-casing; the stack space, shortcut
assignment, and maptable resize are all real and oracle-consistent.

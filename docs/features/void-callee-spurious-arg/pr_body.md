## What was broken

RE-friction round 2, need `void-callee-spurious-arg` (major, 1 instance, challenge
`68149b8a8f555589f353117c` — Mach-O arm64 `practice_2_rengr`):

> **Void secret function is called with a spurious argument** — the same output declares
> `_secret_function(void)` but calls `_secret_function(v2)`, producing internally
> inconsistent and invalid prototype usage.

kuna's own whole-binary export contradicted itself inside one file:

```c
int _secret_function(void);          // practice_2_rengr.h
...
  v3 = scanf("%d",&v2);
  if (v2 == 0x539)
    _secret_function(v3);            // practice_2_rengr.c
```

That does not recompile, and an agent reading it cannot tell whether the callee consumes
the value. The disassembly settles it: `_secret_function` is
`stp x29,x30,[sp,#-0x10]!; mov x29,sp; adrp x0,…; add x0,x0,#0xeec; bl printf`, which
**overwrites** `x0` before it ever reads it.

## Root cause (the filed hypothesis stands)

`ActionActiveParam` recovers an unlocked callee's argument list from the **caller's** data
flow alone (`FuncCallSpecs::checkInputTrialUse`, `fspec.cc:5592`), and
`AncestorRealistic::enterNode` answers `pop_solid` for a Varnode defined by a CALL. Where
the ABI's return register and first argument register coincide — `x0` on AArch64, `r0` on
ARM — the previous call's result *is* the next call's argument as far as the caller can
see. `calleearity`/`calleearityfwd` cannot help: they reconcile against a **sibling** call
to the same callee, they are only ever additive, and there is one call site.

## Mechanism — `option calleedeadarg on|off`, default on (DIV-112)

New module `decompiler/crates/kuna-decomp/src/p4_calls/kuna_calleedeadarg.rs`. The veto
needs **two independent conditions**, and the pass is only as safe as their conjunction:

1. **The callee's own body proves the register dead at its entry.** A bounded decode from
   the callee entry checks that every path *writes* the register before reading it. Each
   path carries the bytes already written; a read of an unwritten byte vetoes the range for
   the whole callee. Every path terminator — a `RETURN`, a nested call, an unresolved
   `BRANCHIND`, a `LOAD`/`STORE` naming the register space, an undecodable instruction —
   requires the write to have happened already, which is what lets a body that overwrites
   `x0` and *then* calls `printf` prove `x0` dead while a body whose first act is a call
   proves nothing. An instruction that branches inside its own p-code is scored against the
   set it was entered with and credits none of its writes.
2. **The value is an earlier call's leftover result** — not something the caller loaded,
   computed, or received as its own parameter.

The probe is taken from the driver right after the flow build (the per-function
`ArchContext` carries the load image but no translator — the same seam `rustabi`'s
call-*output* probe uses), cached per callee entry, and skipped entirely for a function
with fewer than two calls.

**Both conditions were forced by measurement, not by design:**

* Requiring the *write* rather than the absence of a read came from `make test`. The first
  implementation vetoed when no path read the register; that broke **3/675** datatest
  assertions (`stackreturn` #1/#3/#5), whose three callees are one `c3` byte each — a bare
  `ret` reads nothing, so every register looked dead and every stub in an image would lose
  its arguments.
* Requiring the leftover-result shape came from a u-boot ARM sweep. With callee evidence
  alone the pass emptied `do_bootm(ctx->cmdtp,0,v2,bootm_argv)` and
  `ubifs_scan_a_node(a0,v9,v11,a1,v1,1)`: dropping a **leading** register argument punches
  a hole that `ParamListStandard::fillinMap`'s positional rules read as the end of the
  argument list, taking every later argument with it. 34 changed functions → 3.

## The acceptance probe now passes

```
$ python -m scripts.repipe.verify --need void-callee-spurious-arg --json
"counts": {"total": 1, "pass": 1, "fail": 0, "closed": 1, "regressed": 0}
```

```c
  scanf("%d",&v2);
  if (v2 != 0x539) // branch-flip
    printf("\n[-] Wrong passcode!\n");
  else {
    _secret_function();
  }
```

**The probe is not promoted to `tests/cli/`**, and that is deliberate: `verify --promote`
refuses it because its target is `binary_source: dataset` and CI has no dataset. No
vendorable substitute exists — the shape needs a callee whose ABI return register *is* its
first argument register (AArch64/ARM) *plus* a producer call with a recovered return value,
and this machine has no AArch64 linker (only `clang -c`, whose `.o` leaves every call an
unnamed relocation and every callee prototype unlocked). All 132 in-repo fixture binaries
were swept under both arms; none is a witness. The permanent regression pin is
`tests/stages/kuna-calleedeadarg.xml` instead — a two-pass hand-assembled AArch64 fixture
that asserts **both** arms and runs in `make test-stages`, a required CI gate.

## Collateral sweep — every changed line read

`kuna decompile-all --json` with the option off vs on over **61 binaries / 24,306
functions** (decbench O0 + O2 ELF x86-64/ARM/Cortex-M/PE, plus 22 RE-dataset crackmes
including Mach-O arm64 and PE):

| | |
|---|---|
| changed functions | **5 (0.021%)** |
| genuine argument-list corrections | 3 |
| variable renumbering only, semantics identical | 2 |
| statements deleted | **0** |
| arguments lost that the callee reads | **0** |

The three corrections, each checked against the callee's disassembly:

* u-boot `boot_ramdisk_high`: `env_get(dat_60805f8c,(int4)(v6 >> 0x20))` → `env_get(dat_60805f8c)`
  — `env_get(const char *)` takes one argument; the second was the high half of a previous
  call's return.
* u-boot `cli_readline_into_buffer`: `while (!tstc(v4,v3))` → `while (!tstc())` — `tstc(void)`.
* `howo-not-to-simple-keygen` `sub_405810`:
  `sub_4108b0("cannot set %fs base address…",v3,v5,0x405992)` → `sub_4108b0("cannot set %fs base address…")`
  — the callee is glibc's `__libc_fatal` shape (`lea rsi,[msg]; xor eax,eax; mov rdx,rdi;
  mov edi,1; call __libc_message`), which writes `rsi`/`rdx` before reading them and reads
  only `rdi`.

Two calls whose result was discarded are now bound to a variable
(`env_get_bootm_size();` → `v3 = env_get_bootm_size();`).

## Gates

| Gate | Result |
|---|---|
| `make test` | **PARITY OK — 675/675** |
| `make test-stages` | **PARITY OK — 613/613** (610 + 3 new `calleedeadarg` assertions) |
| `make rust-test` | green |
| `make check-spec` | OK |
| `kuna catalog --check` | catalog OK |
| acceptance probe | **PASS** |

Speed, interleaved min-of-N with the order flipped every pair: **+0.44%** on the witness
(147.92 → 148.57 ms) and **+0.78%** on a whole-binary `decompile-all` (2176.53 → 2193.49
ms), against a 5% budget. In ghidra mode each callee decode is a host round trip, so
`ghidra_sim_e2e`'s whole-session pins move 1314 → 1613 getPcode and 801 → 1044 distinct
decoded instructions; both are re-pinned with that reason in the comment.

## Tests

* `tests/stages/kuna-calleedeadarg.xml` — hand-assembled AArch64 two-pass fixture
  (`caller` → `producer` → `target`): `option off` renders `target(producer())` (the bug),
  `option on` renders `producer(); target();`.
* `decompiler/crates/kuna-decomp/src/p4_calls/kuna_calleedeadarg/tests.rs` — option
  parsing, registration, and the incomplete-summary contract.
* Prose in `docs/spec/04-calls-and-prototypes.md` (`ActionActiveParam` trial scoring),
  DIV-112 in `docs/history.md`, full record in
  `docs/features/void-callee-spurious-arg/record.json`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

## What was broken

RE-friction need `direct-address-keyboard-handler` (major, 1 instance, challenge
`6989ca5da15272fa37a80c43`):

> **Direct-address keyboard handler decompilation annexes the unrelated renderer**
> The output begins with keyboard handling, but continues through the renderer logic from
> 0x4610, including the UI title and drawing calls. The disassembly shows a tail jump to
> 0x4610 at 0x66e2; it should remain a call/tail-call boundary rather than duplicating
> thousands of lines into the callback.

`kuna decompile lugosiii 0x6500 --addr` emitted **1,555 lines** for a ~500-byte callback,
with the renderer's `LUGOSI'S II` title twice and 12 calls to its helper `sub_5e30` inside
the keyboard handler — while the *same* function already rendered `sub_4610(a0)` correctly
a few blocks later, where the compiler used a real `call` instead of a tail `jmp`.

## The diagnosis differs from the filed hypothesis

The need blamed `0x6500`'s own absence from the discovered function map. That is why the
agent had to ask by address at all, but it is **not** what causes the annexation — the
tail-jump *target* `0x4610` is the address that is missing. `map function 0x4610 renderer`
in `decomp_dbg`, with `0x6500` mapped exactly as before, alone takes the output
1,555 → 440 lines.

`tailcalljump` (DIV-13, default on) resolves its callee through
`query_call(dest).is_some()`, so it only fires on a target some discovery oracle already
found. Every kuna oracle reaches a function from a symbol, an unwind record, or a direct
`call` the recursive-descent walk arrived at. `0x6500` is only a `wl_keyboard_listener`
code pointer in `.data.rel.ro`, so the walk never enters it, never sees its own
`call 0x4610`, and neither address becomes a function — under `auto`, `reliable`,
`aggressive`, `fast`, `--option listing on`, `aif on` *or* `eh_frame_full on`, all six of
which give byte-identical 1,555-line output.

## Mechanism — `option tailcallframe`, default on (DIV-109)

A tail jump is the caller's **last instruction**: the compiler tears the frame down first,
so the `jmp` executes with the stack pointer exactly where `ret` would find it. An
intraprocedural jump never does, because the code it jumps to still needs the frame.

`p2_lift/kuna_tailcallframe.rs` measures two constant stack-pointer deltas over the
already-decoded raw p-code — forward from the entry address (`push rbx; sub rsp,0x10` =
`-0x18`) and backward from the branch (`add rsp,0x10; pop rbx` = `+0x18`) — and fires when
the second is a strictly positive teardown of exactly the first. The rewrite is the one
`flow.rs` already drives for `tailcalljump`, which is asked first, so a known target keeps
that path and that warning text.

Both scans are per-**instruction**, not per-op, which is load-bearing: an x86 `call` lowers
to `SP = SP - 8` and a return-address store *before* its `CPUI_CALL`, so an op-at-a-time
scan charges the run a phantom push and silently declines every function whose entry block
contains a call (it cost 2 of the 4 witnesses in this binary until it was fixed). Both stop
at the first control-flow instruction (so neither leaves its block), at a stack-pointer
write that is not `SP = SP ± <const>` (a `leave`-style `SP = FP` restore declines), at a
neighbour more than 16 bytes away, and after 24 instructions. A frameless leaf can never
match — with no frame torn down there is no evidence and an ordinary unconditional jump
would be indistinguishable — nor can the function's own entry, nor an address this function
has already decoded.

## The acceptance probe now passes

`a-f13adb91d5d4`, unchanged and with no `--option` or `--assert` on its command line:

```
kuna decompile lugosiii 0x6500 --addr
```

1,555 → **427** lines; both `LUGOSI'S II` occurrences and all 12 `sub_5e30(` calls gone;
`sub_6500(` still emitted; exit 0. `scripts.repipe.verify --need
direct-address-keyboard-handler` reports `pass 1 / fail 0 / closed 1`.

## Blast radius, measured

- **Whole-binary `decompile-all` A/B over 62 binaries** (decbench O0 / O2 / O2-noinline
  x86-64 PIE, i386 PE, ARM Cortex-M, u-boot A32): **0 firings, 0 output differences.**
  Where discovery works, `tailcalljump` already owns the jump and this rule is never asked.
- **Address-level A/B over 377 candidate entries in 8 stripped crackmes** (first
  instruction after each padding run): **0 firings.**
- **On the witness binary, all 32 candidate entries A/B'd: 4 fire, 4 differ**, and all four
  were read line by line — `0x6500` 1,555→427, `0x63d0` 1,143→10, `0x6d40` 1,145→11,
  `0x6db0` 1,156→24. Every one is a callback whose renderer call is now `sub_4610(a0)` and
  whose own body is intact. `0x6e30` correctly does **not** fire: its transfer to `0x4610`
  is a conditional `jne` at `0x6e89`, which is not a tail call.
- **Speed**: −92.95% on the witness function (7 repeats — it stops decompiling 1,100 lines
  it should never have had). On a byte-identical binary (coreutils `dd`, `decompile-all`,
  5 interleaved pairs): −0.9% min / +3.5% mean, i.e. noise.

## Tests

- `tests/stages/tailcallframe.xml` — two-pass on a hand-built bytechunk, 4 assertions
  (605 → 609 stage assertions, baseline re-recorded).
- `decompiler/crates/kuna-decomp/src/p2_lift/kuna_tailcallframe/tests.rs` — 10 unit tests
  over hand-built IR (partial teardown, frameless leaf, `leave`, self-recursion,
  already-decoded target, indirect branch, address gap).
- `decompiler/crates/kuna-console/tests/verify_tailcallframe.rs` — 3 tests on a real ELF.
  Neither parity corpus can carry the *discovery* half: both are symbol-less bytechunks
  with no function map for `query_call` to miss.
- `tests/cli/direct-address-keyboard-handler.json` + vendored fixture
  `decompiler/crates/kuna-analysis/tests/fixtures/tailcallframe_x86_64` (14,488 bytes,
  source beside it). `verify --promote` refuses the acceptance itself because its target is
  `binary_source: dataset` and CI has no dataset, so the probe points at an in-repo twin
  built from the same shape.

## Adversarial verification

Every number below was measured on this branch, on a freshly built tree. The mechanism was
committed by an earlier attempt that ran out of budget before it could open a PR; none of its
claims were taken on trust.

**The wrongness question was asked, and it has an answer.** The rule's evidence is the *frame*,
not the function bound — and a kuna `FunctionSymbol` has no extent, so nothing at this seam can
ask whether `dest` is still inside the caller. A function that tears its frame down
**completely** and then jumps to a shared `ret` in its own body is therefore recovered as a tail
call. Built as a 27-byte x86-64 ELF, the default arm fabricates `return sub_401021();` — a call
to a bare `ret` in the same function. That is a real defect and it is named in the module
header, the spec chapter and the DIV row rather than hidden.

**It is not reachable from compiler output, and that is measured, not argued.** The predicate was
re-implemented statically over disassembly and run across **60 unstripped x86-64 ELFs /
26,110 sized `FUNC` symbols**, which carry **65,361** sites of the raw shape (an intraprocedural
`jmp` immediately preceded by a stack teardown) and **0** that satisfy the exact cancellation.
The structural reason is that a shared return sequence has to be shared *including* its
teardown, so the jump is emitted part-way through the epilogue and the epilogue delta is a
**partial** teardown — gcc and clang at `-O1/-O2/-O3/-Os` all emit `add rsp,0x68; jmp <shared
tail>` against a `-0x70` prologue, which the existing `partial_teardown_declines` unit test
pins. The one full six-register teardown found in real LLVM `-O2` output declines because its
prologue is shrink-wrapped out of the entry block. The sound fix — defer the decision until the
flow work-stack drains, then ask whether the function decoded `dest` by another path — is a
change to the flow walk's *ordering* rather than to this predicate, and is left as follow-up.

**One earlier claim is corrected.** The DIV row explained the clean compiler-built sweep with
"where discovery works, `tailcalljump` already owns the jump and this rule is never asked."
That is false: this rule is asked for every direct `BRANCH` whose target is not a known function
*entry*, which includes every ordinary intraprocedural label. What actually rejects compiler
output is the exact-cancellation clause. The row now says so.

### Blast radius

| Surface | Result |
|---|---|
| Static predicate over disassembly, 60 unstripped ELFs / 26,110 functions | 65,361 raw-shape sites, **0 would-fire** |
| `decompile-all` A/B, 70 symbolized compiler-built ELFs (`/usr/bin`, `/usr/sbin`) | **0 firings, 0 differing** |
| `decompile-all` A/B, 32 stripped static crackmes | **19 firings, 6 binaries differ** |
| Address-level A/B, 581 addresses across 43 crackmes | 1 firing, 1 differing |

**All 19 firings were read line by line. All 19 are improvements; there are no degradations,
and the emitted function count is identical on both arms.**

- `luv4u` `0x2100` goes from a **hard failure** — `(error: Unable to load 512 bytes at
  r0x0002a000)` — to a fully decompiled body ending in `sub_2a000()`.
- `63a85e1f…/crackme` `sub_424ac0` is a `strdup`. Off, it is `undefined16 sub_424ac0(...)` with
  **two uninitialized reads** (`v3 = v2;` and `v1._0_8_ = v6;`, neither ever assigned) and a
  garbage `return v1 << 0x40`. On, it is the correct
  `v1 = strlen(); v2 = malloc(v1+1); if (v2) return memcpy(v2,a0,v1+1); return 0;`.
- `WeeperVM--Level_2` `sub_13480` stops emitting `syscall(); return 0xe7;` — `0xe7` is the
  `exit_group` syscall *number*, rendered as a return value — and emits `sub_132ab(0xffffffff)`,
  keeping the argument.
- The remaining 17 turn a bogus `(*dat_4d1038)(...)` indirect call through an IFUNC GOT slot
  into a named direct call to the stub, `sub_401038(...)` — the same boundary `tailcalljump`
  draws when the stub *is* discovered.

## Gates

| Gate | Result |
|---|---|
| `make test` | **PARITY OK — 675/675** (`docs/baseline.json` unmoved) |
| `make test-stages` | **PARITY OK — 609/609** (605 → 609, baseline re-recorded) |
| `make rust-test` | green |
| `make check-spec` | OK |
| `kuna catalog --check` | catalog OK |
| `scripts.repipe.verify --need direct-address-keyboard-handler` | **PASS** |

🤖 Generated with [Claude Code](https://claude.com/claude-code)

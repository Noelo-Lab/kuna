# callsite-stack-args — analysis

**Verdict up front: this is a MIS-PORTED PREDICATE, not an unported feature and not a
deliberate simplification.** One argument, in one `if`, in one function. Stack-parameter
trials *are* registered at call sites, on every architecture; they are then scored `no-use`
because kuna checks the wrong address against the caller's local stack range. Changing that
one argument restores the entire capability, and the datatest corpus does not move
(675/675 PARITY OK).

---

## 1. The two decbench cases

Full triage: `docs/decbench/triage/O0-mydoom-mydoom-msg_b64enc.md`,
`docs/decbench/triage/O2-noinline-openssh-portable-ssh-add-parse_dest_constraint.md`.

### 1a. `O2-noinline-openssh-portable-ssh-add-parse_dest_constraint` (x86-64, SysV)

`~/github/decbench/results/full_run/O2-noinline/openssh-portable/stripped/ssh-add`, `--addr 0x9190`.

Source calls `sshlog(file, func, line, showfunc, level, suffix, fmt, ...)` with 16 arguments;
arguments 7..16 are stack-passed and are produced by eight `?:` NULL-default ternaries.

kuna today (reproduced on this build):

```c
  sub_28730("ssh-add.c","parse_dest_constraint",0x2f3,1,6,0);
```

Arguments 7..16 never become call inputs, so the eight ternaries have no consumer and are
dead-code eliminated: kuna emits **2 `if`s** where IDA, Ghidra, angr, binja and phoenix all
emit **6** and score GED 0. kuna scores GED 34.

### 1b. `O0-mydoom-mydoom-msg_b64enc` (i386, PE)

`~/github/decbench/results/full_run/O0/mydoom/stripped/mydoom.exe`, `--addr 0x4053c4`.

`ReadFile(hIn, inbuf, sizeof inbuf, &inlen, NULL)` — `&inlen` is the 4th (stack) argument and
`inlen` is the loop's only exit condition. The store `MOV [ESP+0xc],EAX` never becomes a call
input, so it is dead; the `LEA` computing `&inlen` dies with it; nothing observes that the
frame slot escaped; the pre-call constant `0` propagates across the call; `if (inlen == 0) break`
folds to always-taken and **the whole `for(;;)` read/encode loop is deleted** — 3 basic blocks
survive of a 22-node / 30-edge source CFG. This is a *correctness* failure, not a cosmetic one.

## 2. The minimal witness

`gcc -O2 -no-pie` (gcc chooses the `push` form for the two stack arguments, the harder case):

```c
__attribute__((noinline)) long callee(long a,long b,long c,long d,long e,long f,long h,long i)
{ return a+b+c+d+e+f+h*3+i*5; }
__attribute__((noinline)) long caller(long x,long y){ return callee(1,2,3,4,5,6,x,y); }
int main(int argc,char**argv){ return (int)caller(argc,(long)argv); }
```

```
$ kuna decompile ./wit callee          # CALLEE SIDE IS CORRECT
int8 callee(int8 a0,int8 a1,int8 a2,int8 a3,int8 a4,int8 a5,int8 a6,int8 a7)
{ return a6 * 3 + a0 + a1 + a2 + a3 + a4 + a5 + a7 * 5; }

$ kuna decompile ./wit caller          # CALLER SIDE DROPS BOTH STACK ARGS
void caller(void)
{ callee(1,2,3,4,5,6); return; }
```

`caller`'s own parameters `x`/`y` vanish as collateral — their only consumers were the two
dropped arguments. The callee-side result clears, empirically, the whole shared half of the
subsystem: the x86-64 gcc cspec stack `pentry`
(`specs/Ghidra/Processors/x86/data/languages/x86-64-gcc.cspec:80`),
`ParamListStandard::characterize_as_param` / `build_trial_map`, and `ProtoModel::deriveInputMap`.

A second witness reproduces the *dead-code* consequence in isolation — the mydoom shape,
minimised, `gcc -O0 -no-pie`, stripped (`work2` at `0x401136`):

```c
extern void fill7(int a,int b,int c,int d,int e,int f,int *out);
int work2(char *buf, int n) {
    int len = 0, i = 0, total = 0;
    for (;;) {
        if (i >= len) { fill7(1,2,3,4,5,6,&len); if (len == 0) break; i = 0; }
        buf[total++] = buf[i++];
        if (total > n) break;
    }
    return total;
}
```

```c
/* stock — `&len` dropped, len folds to 0, the loop exit is deleted */
    if (0 <= v3) { sub_401251(1,2,3,4,5,6); return v4; }
/* fixed */
    if (v3 <= v5) { sub_401251(1,2,3,4,5,6,&v3); if (!v3) return v6; v5 = 0; }
```

**Correction to the published triage.** `docs/decbench/triage/O0-mydoom-mydoom-msg_b64enc.md`
states that compiling this same reproducer with a full DWARF prototype "does not change this —
the stack argument is still dropped — so this is not only an unlocked-trial-recovery gap; the
locked path drops it too." That is **not reproducible on this build**. The identical source
built `gcc -O0 -no-pie -g` (DWARF present, `fill7` defined in a second TU) decompiles correctly
**without** the fix — `fill7(1,2,3,4,5,6,&len)`, loop intact — and is byte-identical with the
fix. Same for the 8-argument witness built `-g`. The locked-prototype path
(`ActionFuncLink::func_link_input`'s `inputlocked` branch, which builds stack arguments with
`op_stack_load` directly and never consults `check_input_trial_use`) is healthy. The defect is
confined to **unlocked** callee prototypes, exactly as stated in section 5.

## 3. Mechanism — what actually happens (instrumented)

The published triage hypothesised that `Heritage::guard_calls` short-circuits stack trials via
`tryregister = false` (because `FuncCallSpecs::stackoffset` stays `OFFSET_UNKNOWN`).
**That hypothesis is wrong.** An instrumented build (`eprintln` probes in `guard_calls`,
`RuleLoadVarnode::apply_op`, `FuncCallSpecs::resolve_spacebase_relative`, `ActionActiveParam`
and `check_input_trial_use`) shows the placeholder round-trip completing normally on both
architectures. On the witness:

```
funcLinkInput idx=0 inputlocked=false varargs=false spacebase=Some(stack) create_placeholder=true slot=1
RuleLoadVarnode fired: is_ph=true  ->  resolve_spacebase_relative -> Ok  stackoffset=0xffffffffffffffe8
guardCalls space=stack addr=0xfffffffffffffff0 size=8 tryregister=TRUE inputactive=true stackoffset=0xffffffffffffffe8
  input branch trans=0x8  ic=ContainsJustified   -> registerTrial + opInsertInput
guardCalls space=stack addr=0xfffffffffffffff8 size=8 tryregister=TRUE
  input branch trans=0x10 ic=ContainsJustified   -> registerTrial + opInsertInput
```

and on i386/mydoom identically (`tryregister=true`, `stackoffset=0xfffffb84`). So:

- `ActionFuncLink::func_link_input` **does** create the spacebase placeholder.
- `RuleLoadVarnode` **does** resolve it; `stackoffset` is recovered.
- `Heritage::guard_calls` **does** translate the caller-relative address into the callee frame
  and **does** call `register_trial` + `op_insert_input`.

The loss is one pass later, in `ActionActiveParam` -> `check_input_trial_use`:

```
activeparam idx=0 passes=2 maxpass=0 fullychecked=true trials=
  [0..5: register …  active=true]
  [6: stack+8  sz=8 used=false active=FALSE]
  [7: stack+16 sz=8 used=false active=FALSE]
```

Both stack trials are scored `no-use` and dropped by `build_input_from_trials`, and — because
`mark_no_use` makes the trial *definitely* unused — their CALL inputs are overwritten with
constant `0`, freeing the producer for dead-code elimination. That last step is what deletes
the `LEA`/`MOV` chain in mydoom and the ternaries in ssh-add.

### 3a. The exact divergence

`decompiler/crates/kuna-decomp/src/p4_calls/funcdata_callsite.rs:116` (function
`check_input_trial_use`, ported from `fspec.cc:5585 FuncCallSpecs::checkInputTrialUse`):

```rust
} else if !data.get_func_proto().get_local_range().in_range(&trial_addr, 1) {
    …mark_no_use();
```

Upstream, `Ghidra/Features/Decompiler/src/decompile/cpp/fspec.cc:5618`:

```cpp
else if (!data.getFuncProto().getLocalRange().inRange(vn->getAddr(),1))
    trial.markNoUse();
```

- `vn` is `op->getIn(slot)` — the argument Varnode `guard_calls` appended, at the
  **caller-relative** stack address.
- `trial.getAddress()` is the **callee-relative** address (`guard_calls` registers the trial at
  `trans_addr = addr - stackoffset`, `heritage.rs:1514`, while creating the Varnode at `addr`,
  `heritage.rs:1516`).

`data.getFuncProto()` is the **caller's** prototype, so its `localrange` is a caller-frame
range. Checking a callee-relative offset against it is a category error. Instrumented on the
witness:

```
citu trial6 slot=7 vn=(stack,0xfffffffffffffff0) trial_addr=0x8  in_range(trial)=FALSE in_range(vn)=TRUE
citu trial7 slot=8 vn=(stack,0xfffffffffffffff8) trial_addr=0x10 in_range(trial)=FALSE in_range(vn)=TRUE
```

The x86-64 gcc default `localrange` is the negative-offset caller frame, so *every*
callee-relative outgoing-argument offset (which is always positive: 8, 16, 24, …) fails, on
every call site, on every architecture with a downward-growing stack. That is exactly the
observed all-or-nothing behaviour — never a partial recovery.

The next branch (`callee_pop`) *does* correctly use the trial address
(`funcdata_callsite.rs:119` vs `fspec.cc:5621`), which is why the two look interchangeable at a
glance and why the substitution slipped through.

### 3b. Why it is a mis-port and not a simplification

Three independent pieces of evidence:

1. **The port comment records the wrong intent.** `funcdata_callsite.rs:110-112` reads
   *"a stack location outside the **callee's** local range"*. Upstream's range is the caller's.
   The wrong address was chosen deliberately-but-mistakenly to match a wrong mental model —
   the signature of a mis-port, not of a stub.
2. **kuna's own spec already says the right thing.**
   `docs/spec/04-calls-and-prototypes.md`, section *"`ActionActiveParam` — does this argument
   exist?"*: *"outside the **caller's** local stack range (the model's `localrange`,
   `FuncProto::get_local_range`) -> no-use"*. The implementation contradicts its own normative
   chapter.
3. **Everything around it is ported in full and works.** `create_placeholder`,
   `resolve_spacebase_relative`, `abort_spacebase_relative`, `ParamActive::set_placeholder_slot`
   / `free_placeholder_slot`, the `guard_calls` `IPTR_SPACEBASE` translation, and
   `build_input_from_trials`' `off = stackoffset + off` + `scope_local_mark_not_mapped` are all
   line-for-line faithful (checked against `fspec.cc:4849/4870/4910`, `heritage.cc:1444-1538`,
   `ruleaction.cc:4300-4325`, `fspec.cc:5685`). The only `// STUB` marker anywhere on the
   spacebase input chain is `resolve_spacebase_relative`'s un-emitted
   `warningHeader("This function may have set the stack pointer")` (`fspec.rs:6918`), which is
   cosmetic and unreachable here. No `(kuna)` divergence marker exists on the path either — so
   this was never recorded as an intentional simplification.

Introduced in `5cd4d518 rport/w10-callsite-args` (the original port of the file) and preserved
verbatim through `bf4ca27d rport/w10-rsp-5layer-atomic` (a borrow-checker refactor that only
changed how the trial was reached).

## 4. The fix, verified

One argument:

```rust
} else if !data.get_func_proto().get_local_range()
        .in_range(&Address::new(vn_space.clone(), vn_offset), 1) {
```

Verified by compiling it behind an `env`-gated switch so a single binary produces both
behaviours (`KSTACKFIX=1`):

| Case | stock | with the fix |
|---|---|---|
| witness `caller` | `void caller(void) { callee(1,2,3,4,5,6); }` | `void caller(unsigned long a0,unsigned long a1) { callee(1,2,3,4,5,6,a0,a1); }` |
| ssh-add `parse_dest_constraint` | 2 `if`s, `sub_28730(…,0x2f3,1,6,0)` | 6 `if`s, full 16-argument `sub_28730(…, "constraint %zu: …", *a2, v6, v8, v4, …)` — structurally identical to IDA/Ghidra |
| mydoom `msg_b64enc` | 3 basic blocks, loop gone | full `do { … } while` read/encode loop restored, `ReadFile(v10,v6,0x400,v8,0)` with `v8 = &v4` |

`print raw` on the witness confirms the change is in the IR, not the printer:
`call fcallee(free)(#0x1,…,#0x6)` -> `call fcallee(free)(#0x1,…,#0x6,RDI(i),RSI(i))`.

## 5. Corpus quantification

Method: `kuna decompile-all <bin> --json`, stock vs fixed, on the same binary. The analysis
script builds a map *callee -> parameter count kuna itself recovered for that callee* (the
callee side is known-good, section 2) and counts direct call sites of that callee rendered with
fewer arguments. Columns:

- `total_calls` — direct call sites whose target kuna also decompiled.
- `short_calls` / `empty_short` — rendered with fewer than / with none of the callee's own
  recovered parameters.
- `calls_ge7_args` — call sites rendered with >= 7 arguments. On x86-64 SysV **every** such call
  requires stack-argument recovery. Stock kuna scores 0 on 25 of the 30 sampled binaries; the
  handful of non-zero stock counts come from the *locked*-prototype path (`libproto`/DWARF
  signatures, which never reach the broken predicate) and are themselves evidence for the
  blast-radius statement below.

Measured (this build, whole-binary runs):

| binary | arch | metric | stock | fixed |
|---|---|---|---|---|
| `O2-noinline/openssh-portable/stripped/ssh-add` (988 fns) | x86-64 | `calls_ge7_args` | **0** | **282** |
| | | `short_calls` / `total_calls` | 203 / 4588 | 194 / 4596 |
| `O0/mydoom/stripped/mydoom.exe` (156 fns) | i386 PE | `empty_short` / `total_calls` | **192 / 331 (58%)** | **6 / 339 (1.8%)** |
| | | `short_calls` | 192 | 7 |

Reference counts on the same `ssh-add` (from the triage; definitions included, so slightly
inflated): ghidra 300, ida 338, binja 300, angr 263, dewolf 147 — kuna goes 0 -> 282, i.e. from
"structurally impossible" to the same order as every other decompiler.

Note on `short_calls` for x86-64: it barely moves (203 -> 194) because both sides of the ratio
move together — fixing the caller side also lengthens many callees' own recovered parameter
lists. `calls_ge7_args` (x86-64) and `empty_short` (i386) are the honest, non-confounded
columns.

### Corpus sweep

Method: 803 candidate binaries across `full_run/{O0,O2,O2-noinline}/*/stripped/` (818 files, 15
IDA sidecars filtered; `file(1)` confirmed all 803). **Stratified sample of 30** — the same 10
program identities at all three optimisation levels, so opt level is the only variable:
x86-64 `coreutils/tr, gzip, bzip2, coreutils/sort, grep, diffutils/diff`; ARM Cortex-M
`freertos/RTOSDemo.out, chibios/ch.elf`; PE-i386 `mydoom.exe, minipig.exe`. 60
`decompile-all --json` runs, 300 s timeout — **nothing timed out, nothing skipped**, slowest run
26.3 s.

| metric (30 binaries) | stock | fixed | delta |
|---|---|---|---|
| `calls_ge7_args` | 7 | 250 | **+243 (35.7x)** |
| `empty_short` | 1131 | 648 | **-483 (-42.7%)** |
| `short_calls` | 2229 | 1426 | -803 (-36.0%) |
| `lost_args` | 5436 | 4204 | -1232 (-22.7%) |
| `total_calls` | 28440 | 28455 | +15 (+0.05%) |

Per architecture:

| arch | n | `calls_ge7_args` | `empty_short` | `short_calls` |
|---|---|---|---|---|
| x86-64 | 18 | 7 -> 236 | 414 -> 414 (unchanged) | 1087 -> 966 |
| PE-i386 | 6 | 0 -> 8 | **514 -> 31** | 514 -> 40 |
| ARM Cortex-M | 6 | 0 -> 6 | 203 -> 203 (unchanged) | 628 -> 420 |

The i386 empty-argument-list fraction, `empty_short / total_calls`, over all six PE runs:
**42.5% -> 2.5%** (mydoom O0 58.0% -> 1.8%; minipig 20.0% -> 2.4% at every level). On i386 the
fix turns roughly two in five rendered call sites from `f()` into a call with arguments.

**Robustness: no KSTACKFIX-only failure anywhere.** All 72 runs exited 0; stderr was
byte-identical between stock and fixed on every pair. Five representative binaries were re-run
with full stderr capture and dump-level counters (function count, empty-body count, error
markers): all deltas zero. The panics that do occur (`ruleaction_2.rs:96` unhandled
`CPUI_INT_SRIGHT`, `heritage.rs:2775`, `address.rs:180`) appear identically on both sides and are
pre-existing.

Caveats to carry forward honestly:

1. `short_calls` **rose** on two binaries (bzip2 O2 12 -> 18, bzip2 O2-noinline 36 -> 40). The
   metric is self-referential — a call site is compared against the callee's *own recovered*
   parameter count — so lengthening a callee's prototype can make previously-"complete" call
   sites short. Net is still -803.
2. `empty_short` is unchanged for **every** x86-64 and ARM binary. The empty-argument-list win is
   exclusively an i386 phenomenon, exactly as SysV's six register arguments predicts.
3. `freertos/RTOSDemo.out` is degenerate at all three levels (0-1 direct calls found) — an ARM
   Cortex-M function-discovery limitation, not a signal. The ARM column effectively rests on
   `chibios/ch.elf` alone.

### Blast radius, stated precisely

The defect only bites call sites whose **callee prototype is unlocked**. Where DWARF or
`libproto` (both default-on) supply a signature, `ActionFuncLink::func_link_input` takes the
`inputlocked` branch, builds the stack arguments with `op_stack_load` directly, and never
consults `check_input_trial_use` — so those call sites are already correct. Verified directly
(section 2): both witnesses built `-g` decompile correctly *without* the fix and are unchanged
*with* it; the same sources stripped reproduce the bug.

**The DWARF split, measured.** The six sampled x86-64 identities were re-run on their
non-stripped `compiled/` counterparts (DWARF confirmed via `readelf -S | grep debug_info`):

| metric (6 binaries) | stripped stock -> fixed | DWARF stock -> fixed |
|---|---|---|
| `calls_ge7_args` | 1 -> 73 (**+72**) | 37 -> 73 (**+36**) |
| `short_calls` | 152 -> 124 (-28) | 68 -> 68 (**0**) |
| `lost_args` | 335 -> 261 (-74) | 81 -> 81 (**0**) |
| `empty_short` | 36 -> 36 (0) | 10 -> 10 (0) |

This **half**-supports the "DWARF already fixes it" claim, and the honest statement is narrower
than the one in the two triage documents:

- On the short-call metrics the claim holds exactly. With DWARF the fix moves `short_calls` and
  `lost_args` by **zero** (68 -> 68, 81 -> 81) versus -28 / -74 on the same binaries stripped.
  DWARF also lowers the stock baseline itself (`short_calls` 152 stripped -> 68 with DWARF).
- On `calls_ge7_args` it does **not** hold: the fix still nearly doubles the count with DWARF
  present (37 -> 73), and every one of the six DWARF binaries gains sites. DWARF locks the
  prototypes it *describes* — not every callee (internal statics, tail-called helpers, imports
  without DWARF remain unlocked), so a substantial minority of call sites still take the broken
  path even in a fully-debugged build.
- The cleanest number in the sweep: **both endpoints converge on the identical 73** with or
  without debug info. The fix reaches the same recovery ceiling either way; stock reaches 1/73
  stripped and 37/73 with DWARF.

So the correct blast-radius statement is: **the defect bites every call site whose callee
prototype is unlocked.** That is *all* of them in a stripped binary — the benchmark surface and
the reverse-engineering use case — and roughly half of them even with full DWARF. It is
architecture-independent: i386 (every argument past the first) and x86-64 (past the sixth) both
reproduce, and SPARC's hidden struct-return pointer is dropped by the same predicate (section 6).

### GED impact — stated honestly

Argument lists are **invisible** to a CFG graph-edit distance. A call rendered
`f(a,b,c)` versus `f()` is the same node in the same place; the 282 recovered argument lists on
`ssh-add` are, by themselves, worth **zero** GED.

What moves the benchmark is the *second-order* effect: when the missing argument is the only
consumer of a computation, dead-code elimination deletes real basic blocks.

- `msg_b64enc`: 19 of 22 source CFG nodes deleted; the fix restores the loop. GED 43 -> expected
  low single digits (Ghidra scores 5 on the identical shape).
- `parse_dest_constraint`: four `if` statements plus their edges deleted; the fix restores all
  four, matching the five references that score 0. GED 34 -> expected 0-ish.

So the GED payoff is confined to functions where a stack argument is the sole consumer of
control-flow-bearing computation — a minority of call sites, but a systematically recurring one
(every `?:`-heavy logging macro, every `&local` output parameter past the register budget,
every `snprintf`-family wrapper). The correctness payoff — kuna currently *deletes live loops*
and emits code that does not implement the binary — is not conditional on GED at all, and is
the stronger argument.

## 6. Ablation and side effects (measured, fix forced ON)

| Gate | Result |
|---|---|
| `kuna test --datatests --baseline docs/baseline.json` | **675/675, PARITY OK** — zero assertions move |
| `kuna test --datatests --datatests-dir tests/stages` | 304/305 — **one** assertion moves (below) |
| `cargo test --release` (whole workspace) | green apart from `w10_adv_r2_guard_is_not_a_skipped_false_green`, which fails identically **without** the fix (it needs an 8051 `.sla` this worktree does not build) |
| speed, `decompile-all` wall clock x3 | ssh-add 10.80 s -> 10.79 s (median); mydoom 2.03 s -> 1.98 s (min). **No measurable delta.** |

The single stage-assertion movement is `gh6882-sparcstructret.xml` #2, and reading it changes
the risk picture: it is **itself a correctness improvement**.

```c
/* stock */                          /* fixed */
int4 p(int4 a0) {                    int4 p(int4 a0) {
  int4 v1; // stack - 0x8              int4 v1 [2]; // stack - 0x8
  f();                                 int4 *v2; // i0
  IllegalInstructionTrap(8);           v2 = v1;
  f();                                 f(v2);
  return v1 + a0;                      IllegalInstructionTrap(8);
}                                      f(v2);
                                       return v1[0] + a0;
                                     }
```

`f` is a **struct-return** function; stock kuna dropped the hidden struct-return buffer pointer
entirely (`f()`), and with the fix it is passed (`f(v2)`, `v2 = &v1`) and the 8-byte buffer is
typed as such. The assertion regresses only because it pins the exact expression
`return v1 + a0`, which now correctly renders `return v1[0] + a0`. That XML's own comment
already documents one prior update of this same string for the same class of reason.

### The one genuine quality risk: over-recovery on unprototyped i386 calls

In mydoom, `CloseHandle` renders `(*dat_411298)(v10,v6,v7,v8,v9)` — five arguments where it
takes one. The outgoing-argument slots from the *previous* `ReadFile` call are still live and
unmodified at the `CloseHandle` site, and ancestor-realism accepts them. This is upstream
Ghidra's behaviour too for a genuinely unprototyped `__cdecl` callee; Ghidra escapes it by
applying the PE IAT import prototype (`CloseHandle(local_28)`), which kuna does not yet do for
indirect-call targets even though `kuna functions mydoom.exe --json` already lists
`CloseHandle @ 0x411298`. Trading 192 empty argument lists for a handful of over-long ones is
plainly the right trade, but it is a real, visible artifact, and the follow-up that closes it
(PE IAT prototypes) is named in the proposal's sequencing.

## What was broken

RE-friction round 2, need `keyboard-callback-uses-undefined` (major, 1 instance,
challenge `6989ca5da15272fa37a80c43`, x86-64 ELF `lugosiii`):

> **Keyboard callback uses undefined locals as incoming event arguments** — The function
> signature contains only a0, while the first condition reads register-associated locals, so
> the event-type and key parameters appear uninitialized. The disassembly shows those values
> arrive in R9D and R8D.

`kuna decompile lugosiii 0x6500 --addr` opened with

```c
void sub_6500(long a0)
{
  ...
  int v7; // r8d
  int v8; // r9d

  if (v8 != 1 || !*(long *)(a0 + 0x68))
    return;
  v2 = xkb_state_key_get_one_sym(*(long *)(a0 + 0x68),v7 + 8);
```

`v7` and `v8` are declared and then read with nothing ever assigning them. The function is
the Wayland `wl_keyboard_listener` **key** callback, whose ABI signature is fixed by the API
it is registered with — `key(void *data, struct wl_keyboard *, uint32_t serial, uint32_t
time, uint32_t key, uint32_t state)` — so `key` arrives in `r8d` and `state` in `r9d`. It is
reached only through a function-pointer table, so no call site anywhere in the image can
supply a prototype.

## The filed diagnosis is overturned; the symptom stands

The need's hypothesis was "no prototype evidence, so live-in R8D/R9D are not promoted". That
is not what happens, and a 30-line fixture settles it. `ActionInputPrototype` registers a
trial for **every** input Varnode and marks it ACTIVE when it has readers, so `rdi`, `r8d`
and `r9d` all reach `ParamListStandard::fillinMap` as active trials with no prototype
anywhere — and `buildTrialMap` even synthesizes unreferenced *filler* trials for the ignored
`rsi`/`rdx`/`rcx`. Two callbacks, both pointer-table-only, both unprototyped:

```c
void on_two(long a0,unsigned long a1,unsigned long a2,int a3)   // 2-register hole: recovered on main today
void on_key(long a0) { int v1; // r8d ... }                     // 3-register hole: dropped
```

The discriminating variable is the **width of the hole**, not the missing prototype.

## Root cause

`ParamListStandard::forceInactiveChain` (`fspec.cc:1519`) is called with `maxchain = 2`. It
counts consecutive unused parameter slots and, once the run passes two, sets `seenchain` and
marks **every** remaining trial inactive — the ones that already scored active included.
`fillinMap` then only marks the still-active trials used. The witness's `rsi`/`rdx`/`rcx`
hole is exactly three registers, one past the limit.

That rule's premise belongs to a call site. There a trial is scored from the **caller's**
data flow, where a live argument register is genuinely ambiguous, so a long empty run really
is evidence the recovery walked past the end of the list (`calleedeadarg`'s record names the
same rule from the other side). For the function's **own** inputs an active trial is a fact
about the body: it reads that caller-saved register before any definition of it. The gap
slots carry no counter-evidence at all — an untouched argument register is exactly what an
ignored parameter looks like, and a callback registered against a fixed API signature ignores
parameters as a matter of course.

## The mechanism

New option **`inputparamgap`** (`on|off`, default **on**, DIV-114,
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_inputparamgap.rs`): during the function's own
input recovery, an **ACTIVE trial whose `ParamEntry` is an exclusion (register) entry is never
demoted by `forceInactiveChain`**. The active trials past the hole survive and the rule's
existing tail loop promotes the filler trials `buildTrialMap` had already built:

```c
void sub_6500(long a0,unsigned long a1,unsigned long a2,unsigned long a3,int a4,int a5)
  if (a5 != 1 || !*(long *)(a0 + 0x68)) return;
  v2 = xkb_state_key_get_one_sym(*(long *)(a0 + 0x68),a4 + 8);
```

`a4` is `key`, `a5` is `state` — the Wayland prototype, positions and all. Three narrowing
clauses, **one of them measured rather than argued**:

1. **Own-input recovery only.** The flag is carried on the `ParamActive` that
   `ActionInputPrototype` builds; nothing sets it at a call site, so argument recovery in
   every binary is untouched.
2. **Register trials only.** A first draft exempted any register *gap slot* instead. It fixed
   the witness and passed 675/675 — and let one Win64 `sub_140010a57` span its four-register
   hole into the stack resource and promote **eleven scratch slots of the caller's argument
   area into a fifteen-parameter signature**. A second draft that reset the chain at a
   register gap did not fix it either (that function's stack trials are contiguous). The
   shipped rule protects only register trials, which — because trials sort into formal
   parameter order — also keeps the hole-filling inside the register file and bounds the gain
   to the ABI.
3. **Never additive to evidence.** It cannot make a trial active that was not already active.
   A register the body does not read before writing is still not a parameter.

Price paid knowingly: a genuine Win64 fifth argument on the stack behind a four-register hole
(`vm_obfuscation.exe sub_14000c384`) reverts to upstream's `(void)`. Draft 1 recovered it; it
is not what this need is about and is not separable from the fifteen-parameter failure by any
local property measured here.

## Whole-corpus sweep — 83 binaries, 36,589 functions

Run twice, once per draft, `decompile-all` before/after with every hunk classified:

| | draft 1 (rejected) | **shipped** |
|---|---|---|
| changed functions | 343 | **196** (0.54%) |
| hunks with >20 residual diff lines | 12 | **0** |
| **statement-list changes** (after normalizing `vN`/`aN`) | — | **0 of 196** |
| parameter-count gain | up to +14 | **+4 (149), +5 (45), +6 (2)** — never a loss, never same-arity |
| declarations dropped | included stack slots | **registers only** (`r8`, `r9`, `r9d`, `r8d`, `ecx`, `rcx`, …), zero stack |

Function sets were identical between arms on every binary; the 9 binaries that failed to load
failed under **both** arms. The pass adds register parameters and renames the undefined locals
they replace — it moves, deletes and re-anchors nothing.

## The acceptance probe now passes

```
python -m scripts.repipe.verify --need keyboard-callback-uses-undefined --json
  -> {"total": 1, "pass": 1, "fail": 0, "closed": ["keyboard-callback-uses-undefined"]}
```

All three previously-failing clauses flipped: `stdout_matches[1]`
(`sub_6500\([^)]*,[^)]*\)`) now matches, and both `stdout_absent` clauses (`; // r8d`,
`; // r9d`) are absent.

`binary_source: dataset`, so `verify --promote` refuses it (CI has no dataset). Following
#395's precedent, a new fixture `decompiler/crates/kuna-analysis/tests/fixtures/inputparamgap_x86_64`
carries **both arms** — `on_key` (three-register hole, the bug) and `on_two` (two-register
hole, the control, identical under both settings) — with
`tests/cli/keyboard-callback-uses-undefined.json` pinned to it and
`tests/stages/kuna-inputparamgap.xml` pinning the two-pass rendering.

## Speed

`scripts.pipeline.timeit` reported **−30.52%** over 11 samples, twice. That is the known
sequential-measurement artifact (`docs/features/returncopysplit` measured a −20.23% noise
floor on byte-identical output), so it is **not** claimed here. Interleaved min-of-15 on the
same witness: **+0.07% min / +0.33% median**. Interleaved min-of-3 whole-binary
`decompile-all`: **+0.27%** (`vm_obfuscation.exe`, 6.1 s) and **+0.41%**
(`crackme.x86_64.elf`, 75.6 s). No measurable cost.

## Gates

| Gate | Result |
|---|---|
| `make test` | **PARITY OK — 675/675** (0 changed assertions, which is what qualifies the default-ON flip) |
| `make test-stages` | **PARITY OK — 620/620** (615 + this feature's 5) |
| `make rust-test` | green |
| `make check-spec` | OK |
| `kuna catalog --check` | catalog OK |
| `make test-cli` | 28/28 |
| acceptance probe | **PASS** |

🤖 Generated with [Claude Code](https://claude.com/claude-code)

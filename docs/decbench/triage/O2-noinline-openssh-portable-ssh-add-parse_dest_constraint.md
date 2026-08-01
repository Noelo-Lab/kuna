---
case_id: O2-noinline-openssh-portable-ssh-add-parse_dest_constraint
pool: angr
status: needs-proposal
tier: X (angr queue) / M (ida queue)
margin: 34
fresh_verdict: reproduced verbatim on today's build in every mode (auto/aggressive, reliable, fast) — kuna emits `sub_28730("ssh-add.c","parse_dest_constraint",0x2f3,1,6,0)` with only the 6 register arguments, so the 8 NULL-default ternaries feeding stack arguments 7..15 are dead-code eliminated
option_closing: null
feature_slug: callsite-stack-args
scope: proposal
confidence: high
---

## Side-by-side

Reference (IDA, stored) — the four NULL-default guards survive because the values
they produce are consumed as stack arguments of the 15-argument `sshlog` call:

```c
  v12 = *(const char **)(v7 + 48);
  v13 = *(const char **)(v7 + 40);
  v14 = "@";
  if ( !v12 )
    v12 = "(ANY)";
  if ( !v13 )
  {
    v14 = "";
    v13 = "";
  }
  v15 = *(const char **)(v7 + 8);
  if ( !v15 )
    v15 = "(ORIGIN)";
  v16 = "@";
  if ( !v11 )
  {
    v11 = "";
    v16 = "";
  }
  sub_28730(
    (unsigned int)"ssh-add.c", (unsigned int)"parse_dest_constraint", 755, 1, 6, 0,
    (long long)"constraint %zu: %s%s%s (%u keys) > %s%s%s (%u keys)",
    *a3, v11, v16, v15, *(int *)(v7 + 20), v13, v14, v12, *(int *)(v7 + 60));
```

Ghidra (stored) is structurally identical — same four guards, same 16-input call
(`FUN_00128730("ssh-add.c",...,*param_3,puVar6,puVar9,pcVar4,...)`).

kuna (stored, 2026-07-27) and kuna (fresh, today) are structurally the same
output; only print normalization (DIV-34..39) differs. Fresh:

```c
  else {
    sub_8d80(v3,&v2[5],a3);
  }
  sub_28730("ssh-add.c","parse_dest_constraint",0x2f3,1,6,0);
  v5 = sub_2abf0(*a1,*a2,*a2 + 1,8);
```

Structural metrics: ida 6 ifs / ghidra 6 ifs / kuna 2 ifs (stored *and* fresh).
The four missing `if`s are exactly the four NULL-default guards above. The two
calls kuna does emit stop dead at argument 6:

```c
  sub_241c0("ssh-add.c","parse_dest_constraint",0x2ee,0,1,0);   // fmt + os dropped
  sub_28730("ssh-add.c","parse_dest_constraint",0x2f3,1,6,0);   // fmt + 9 varargs dropped
```

`--mode reliable` (the benchmark's mode) and `--mode aggressive` produce
byte-identical output — no mode default is involved. `--mode fast` is the
smoking gun: with the aggressive dead-code pass relaxed, the argument-producing
computation reappears, but as *caller locals*, never as call arguments:

```c
    v29 = (char *)v4[6];
    v27 = (int8)v4[5];
    v28 = (int8 *)0x51023;
    if (!v29)  v29 = "(ANY)";
    if (!v27) { v28 = (int8 *)0x51251; v27 = ""; }
    v25 = (char *)v4[1];
    if (!v25)  v25 = "(ORIGIN)";
    v22 = *a2;
    v21 = "constraint %zu: %s%s%s (%u keys) > %s%s%s (%u keys)";
    sub_28730("ssh-add.c","parse_dest_constraint",0x2f3,1,6,0);   // v21..v29 // stack - 0x40..0x88
```

`v21`..`v30` carry `// stack - 0x40` … `// stack - 0x90` source comments: they
are the outgoing-argument stack slots, modelled as caller locals rather than as
the callee's parameters.

## Source

`~/github/decbench/results/full_run/O2-noinline/openssh-portable/compiled/ssh-add.i:19243`
(`ssh-add.c:735-760`, macro expansion of `debug2_f`):

```c
static void
parse_dest_constraint(const char *s, struct dest_constraint ***dcp,
    size_t *ndcp, char **hostkey_files)
{
 ...
 sshlog("ssh-add.c", __func__, 755, 1, SYSLOG_LEVEL_DEBUG2, ((void *)0),
   "constraint %zu: %s%s%s (%u keys) > %s%s%s (%u keys)", *ndcp,
   dc->from.user ? dc->from.user : "",
   dc->from.user ? "@" : "",
   dc->from.hostname ? dc->from.hostname : "(ORIGIN)",
   dc->from.nkeys,
   dc->to.user ? dc->to.user : "",
   dc->to.user ? "@" : "",
   dc->to.hostname ? dc->to.hostname : "(ANY)",
   dc->to.nkeys);
 ...
}
```

`sshlog` is `sshlog(file, func, line, showfunc, level, suffix, fmt, ...)`: seven
named parameters plus varargs. Under SysV x86-64 the first six go in
RDI/RSI/RDX/RCX/R8/R9 and *everything from `fmt` onward is pushed on the stack*.
The eight ternaries exist only to produce stack arguments 8..15 — which is why
they account for most of the source CFG's 29 nodes / 42 edges, and why dropping
them costs GED 34 against a reference that scores 0.

## Analysis

**Structural symptom (one).** kuna truncates every call-site argument list at
the six register arguments; SysV x86-64 stack-passed arguments (argument 7 and
beyond) are never recovered. Because nothing consumes them, everything computed
solely to fill those slots is dead-code eliminated — here the four `if`
statements implementing `dc->from.user ? ... : ...` and friends, which is the
entire 4-if / GED-34 delta against IDA, Ghidra, angr, binja and phoenix (all
five score 0).

This is not specific to this function or this binary. Across all **988**
functions of the fresh `kuna decompile-all` of `ssh-add`, the number of emitted
calls with more than six arguments is **zero**. Reference counts on the same
binary (function definitions included, so slightly inflated): ghidra 300,
ida 338, binja 300, angr 263, dewolf 147.

Minimal reproduction on today's build (`gcc -O2 -no-pie`, direct call, no PLT):

```c
__attribute__((noinline)) long callee(long a,long b,long c,long d,long e,long f,long h,long i)
{ return a+b+c+d+e+f+h*3+i*5; }
__attribute__((noinline)) long caller(long x,long y){ return callee(1,2,3,4,5,6,x,y); }
```

```
$ kuna decompile ./t4 callee          # CALLEE side is CORRECT
int8 callee(int8 a0,int8 a1,int8 a2,int8 a3,int8 a4,int8 a5,int8 a6,int8 a7)
{ return a6 * 3 + a0 + a1 + a2 + a3 + a4 + a5 + a7 * 5; }

$ kuna decompile ./t4 caller          # CALLER side DROPS the two stack args
void caller(void)
{ callee(1,2,3,4,5,6); return; }
```

The caller's own parameters `x`/`y` vanish as collateral: their only consumers
were the two dropped stack arguments. The same 6-argument truncation reproduces
for a locked varargs prototype (`__printf_chk(1,0x2000,1,2,3,4)` for a
nine-argument `printf`), for a PLT call, and with `push`- and `mov`-form
argument stores. The console confirms the loss is in the IR, not in printing —
`print raw` shows the CALL op itself with six inputs:

```
0x0000119d:5e:	call fext(free)(#0x1,#0x2,#0x3,#0x4,#0x5,#0x6)
```

**Owning phase: P4 (call & prototype model).** The callee-side result above
proves the pieces that are *shared* with the callee path are healthy: the
x86-64 gcc cspec's stack `pentry` (`<pentry minsize="1" maxsize="500"
align="8"><addr offset="8" space="stack"/></pentry>`,
`specs/Ghidra/Processors/x86/data/languages/x86-64-gcc.cspec:80`),
`ParamListStandard::characterize_as_param` / `build_trial_map`
(`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs:2058,2654`), and
`ProtoModel::deriveInputMap`. What is not healthy is the *call-site* half:
`FuncCallSpecs`'s active-input trial recovery never gets a spacebase trial.

The single place where a caller-relative stack address is translated into the
callee's parameter frame and registered as a call-input trial is the
`IPTR_SPACEBASE` branch of `Heritage::guardCalls`
(`decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs:1440`):

```rust
            let mut off = addr.get_offset();
            let mut tryregister = true;
            if spc.get_type() == spacetype::IPTR_SPACEBASE {
                if fc.get_spacebase_offset() != OFFSET_UNKNOWN {
                    off = spc.wrap_offset(off.wrapping_sub(fc.get_spacebase_offset()));
                } else {
                    tryregister = false;
                }
            }
```

…gating the registration at `heritage.rs:1496`:

```rust
            if fc.is_input_active() && tryregister {
                let ic = fc.proto().characterize_as_input_param(&trans_addr, size);
                if ic == Containment::ContainsJustified
                    && fc.get_active_input().which_trial(&trans_addr, size) < 0
                { /* register_trial + op_insert_input */ }
            }
```

`tryregister = false` is the one branch that silently skips *all* stack trials
and matches the observed all-or-nothing behaviour (zero stack arguments
anywhere, never a partial recovery). It fires when
`FuncCallSpecs::stackoffset` is still `OFFSET_UNKNOWN`, i.e. when the stack
placeholder built by `ActionFuncLink::func_link_input` →
`FuncCallSpecs::create_placeholder` (`p4_calls/coreaction_protos.rs:822`,
`p4_calls/fspec.rs:6848`) is never resolved by
`RuleLoadVarnode::apply_op` → `FuncCallSpecs::resolve_spacebase_relative`
(`p3_dataflow/ruleaction_4.rs:404-419`, `p4_calls/fspec.rs:6902`). Everything
downstream of a registered trial *is* ported —
`build_input_from_trials` already translates a spacebase trial by
`off = stackoffset + off` and calls `scope_local_mark_not_mapped`
(`p4_calls/funcdata_callsite.rs:280-325`) — so the failure is upstream of it.
The pass-budget wiring that keeps `isInputActive` alive long enough for the
later stack-space heritage pass is present and faithful
(`init_active_input`: `if maxdelay > 0 { maxdelay = 3 }`,
`p4_calls/fspec.rs:6356`), so that is ruled out.

I could not narrow the last step further without instrumenting the build (this
triage is read-only): the alternative is that the trial *is* registered and then
scored unused by `check_input_trial_use`. Both candidates live in the same P4
subsystem, but a single instrumented run is required to say which — hence
`needs-proposal` rather than `feature-candidate`.

**Why the references win.** They all recover stack-passed arguments, so the
ternary chain has consumers and survives dead-code elimination. Nothing about
their structuring is better here; kuna's structuring of what it kept is fine.

**Not an option gap.** No catalog option matches. Swept explicitly on both the
real case and the minimal repro: `condfold wide`, `formatstring on`,
`operand_refs on` — none change the output (real case stays at 2 ifs).
`--mode aggressive` (21 options) and `--mode reliable` are byte-identical.
`stackprobeloop` (P2, default on) is the nearest-sounding option and is shape-
gated to `-fstack-check` probe loops; it is already on and inert here.

**Not a metric artifact.** Source CFG is 29 nodes / 42 edges (not degenerate,
not approximated, no truncation by the reference). The eight ternaries are real
source-level control flow; five independent decompilers reproduce them and score
0. kuna's missing four `if`s plus their edges account for the margin.

## Proposed fix

Scope: `[PROPOSAL]` PR. This is a strict correctness gap versus upstream Ghidra
(upstream recovers these arguments), so per the repo rules it needs no option
flag — but the blast radius is every call site in every binary on every
architecture with stack-passed arguments, so it must not ship as a small PR.

Mechanism to investigate and close, in order:

1. Instrument `FuncCallSpecs::get_spacebase_offset()` at
   `p3_dataflow/heritage.rs:1442` and confirm whether it is `OFFSET_UNKNOWN` at
   the heritage pass that covers the stack space. If yes, the defect is that the
   placeholder LOAD never reaches `RuleLoadVarnode`
   (`p3_dataflow/ruleaction_4.rs:390-420`) with a resolved spacebase+constant
   pointer, so `FuncCallSpecs::resolve_spacebase_relative`
   (`p4_calls/fspec.rs:6902`) never sets `stackoffset`.
2. If `stackoffset` *is* resolved, the trials are being registered and then
   dropped: audit `check_input_trial_use` / `final_input_check`
   (`p4_calls/coreaction_protos.rs:1076-1150`) and `ParamActive`'s used/unused
   scoring for spacebase trials.
3. Either way the downstream path (`build_input_from_trials`,
   `p4_calls/funcdata_callsite.rs:242`) already handles spacebase trials and
   should need no change.

Owning files: `decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs`,
`decompiler/crates/kuna-decomp/src/p4_calls/coreaction_protos.rs`,
`decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs`,
`decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_4.rs`.
Owning spec chapter: `docs/spec/04-calls-and-prototypes.md`.

Risks: high. Recovering stack arguments adds inputs to CALL ops everywhere,
which changes `ActionDeadCode` liveness, local-variable maps
(`scope_local_mark_not_mapped` will start un-mapping outgoing-argument stack
slots that are currently emitted as locals), and therefore emitted C in a large
fraction of the 675 datatest assertions. Expect the parity gate to move; the
change is a correctness fix, so any movement must be adjudicated assertion by
assertion rather than absorbed by a baseline re-pin. A speed delta must be
measured: more live call inputs means more heritage/dead-code work.

Payoff: this is not one case. It is a whole-binary, all-architecture argument-
list truncation — the same defect will be behind many `kuna-specific` GED
margins in the campaign whenever a function calls anything with more than six
arguments (all of openssh's `sshlog`/`sshfatal`/`debug_f`/`error_f` macro
family, every `snprintf`-style call, every wide wrapper).

---
case_id: O2-openssh-portable-sshd-mm_answer_auth2_read_banner
pool: angr
status: metric-artifact
tier: S
margin: 12
fresh_verdict: today's default (--mode auto = aggressive) and --mode reliable both emit the same 5-node shape the benchmark recorded; nothing regressed and nothing was fixed, but the GED floor for any output that recovers the CMOV null-check is 8, and angr's 0 comes from dropping that check entirely
option_closing: null
feature_slug: null
scope: none
confidence: high
---

## Side-by-side

Source (openssh `monitor.c:812-824`, preprocessed in `monitor.i`):

```c
sshbuf_reset(m);
banner = auth2_read_banner();
if ((r = sshbuf_put_cstring(m, banner != ((void *)0) ? banner : "")) != 0)
        sshfatal("monitor.c", __func__, 819, 1, SYSLOG_LEVEL_FATAL, ssh_err(r), "assemble");
mm_request_send(sock, MONITOR_ANS_AUTH2_READ_BANNER, m);
free(banner);
return (0);
```

angr (stored, GED 0):

```c
sub_44d700(a2);
v4 = sub_423b80(v1, v2, v3);      /* v1,v2,v3 never assigned */
v5 = sub_45dcc0();                /* both args LOST */
if (v5)
{
    v6 = sub_44c530(v5);
    sub_475240("monitor.c", "mm_answer_auth2_read_banner", 819, 1, 1, v6, "assemble", v6);
}
sub_43b2b0(a1, 11, a2);
free(v4);
/* 40 x "unsupported instruction" */
return 0;
```

kuna (fresh, today's default; identical under `--mode reliable`, GED 12):

```c
sub_4d700(a2);
v2 = (void *)sub_23b80();
v3 = (void *)0xcf952;
if (v2)
  v3 = v2;
v1 = sub_5dcc0(a2,v3);
if (!v1) {
  sub_3b2b0(a1,0xb,a2);
  free(v2);
  return 0;
}
sub_75240("monitor.c","mm_answer_auth2_read_banner",0x333,1,1,sub_4c530(v1)); // no-return
```

IDA (stored, GED 8) has the same CMOV-as-`if` expansion but keeps the fatal call as a
guarded `if (v9) { ... }` with the normal path as the shared tail. Ghidra (GED 12) is
byte-for-byte the same structure as kuna.

## Source

The machine code is the point. `gcc -O2` compiled the source ternary into a
**conditional move**, not a branch:

```
36153:  mov    %r12,%rdi
36156:  test   %rax,%rax
36159:  mov    %rax,%rbp
3615c:  cmovne %rax,%rsi          <-- banner != NULL ? banner : ""
36160:  call   5dcc0              <-- sshbuf_put_cstring
36165:  test   %eax,%eax
36167:  jne    36209              <-- cold sshfatal block, no fall-through
```

`0xcf952` is a zero byte in `.rodata`, i.e. the literal `""`.

## Analysis

**The margin is not recoverable by correct output.** I re-ran decbench's own CFG
extractor (pyjoern, from `~/.virtualenvs/decbench`) and its VJ-GED on hand-built
variants against the real `monitor.i` source CFG. The measurement reproduces every
recorded score exactly, which validates the model:

| variant | nodes/edges | GED |
|---|---|---|
| kuna fresh (CMOV as `if`, no-return arm as fall-out tail) | 5 / 5 | **12.0** (matches recorded) |
| IDA shape (CMOV as `if`, fatal kept inside a guarded `if`) | 5 / 6 | **8.0** (matches recorded) |
| kuna + CMOV folded to a `?:` ternary | 5 / 5 | 12.0 (no change) |
| kuna + ternary + fatal kept inside the guard | 5 / 6 | 8.0 |
| **the source's own shape, transliterated into decompiler C** | 5 / 6 | **8.0** |
| angr shape (null check absent entirely) | 3 / 3 | 0.0 |

Two conclusions:

1. **GED 0 requires deleting the null check.** angr scores 0 because it lost the
   `cmovne` completely: its `sub_45dcc0()` takes no arguments and its
   `sub_423b80(v1, v2, v3)` reads three never-assigned locals. kuna, Ghidra, IDA and
   binja all recover the check and all of them are penalized for it. `phoenix` also
   scores 0, and its 3-node CFG can only come from the same omission.
2. **The floor is 8, not 0**, for anything that keeps the check. Writing the source
   function's *own* control flow in decompiler style (`if ((v1 = f(a2, v2 ? v2 : "")) != 0)
   fatal(...); tail;`) still scores 8. The reason is an asymmetry in the ground-truth
   extractor: in `monitor.i` the ternary sits inside the `if` condition and pyjoern folds
   it into the 16-statement entry block (3 nodes total), while the same ternary in a
   decompiled body always splits into two extra blocks. No decompiler output can close
   that gap.

The one genuine structural divergence from source is worth **4 of the 12 points**: kuna
nests the *hot* path inside the guard and lets the cold no-return call become the
function's fall-out tail -

```c
if (!v1) { sub_3b2b0(...); free(v2); return 0; }
sub_75240(...); // no-return
```

- where source, IDA and angr nest the *cold* arm and keep the normal path at top level
(`if (v1) { fatal(...); } tail; return 0;`). Owning phase would be **P8 (structure)**.
But this is a deliberate, already-shipped kuna preference, not a defect: `branchflip`
(P8, default on) exists precisely to put "the common path top-to-bottom (angr-style
positive-first)", and `ifelseflatten` (P8, default on) exists to drop the `else` after a
terminating arm. Both fire in the direction kuna emits. Flipping the polarity here would
contradict two shipped default-on options for 4 GED points on a shape that is
semantically identical to the source. Not worth a feature.

Verified today, all on the stripped binary at `0x36130`:

- `--mode auto` (product default, aggressive under 500 KiB) and `--mode reliable`
  (the benchmark's configuration) produce the identical structure.
- No option changes it: `ifelseflatten off`, `branchflip off`, `condfold wide`,
  `iteexpr on`, `gotoreduce off taildup off` are all byte-identical to the default.
- Turning the whole `noreturn_*` family off does **not** restore the guard shape; it
  makes kuna swallow `mm_answer_authpassword` and emit ~90 lines of garbage. kuna's
  no-return recovery here is correct and load-bearing; `sshfatal` really is
  `__attribute__((noreturn))` and the block at `0x36209` has no fall-through.

### Two non-structural defects the dump surfaced (no GED effect, logged for the pool)

1. **Stack-passed arguments at the `sshfatal` call are dropped.** The call site pushes
   two stack arguments (`push %rax` at `0x36223` = `ssh_err(r)`, `push %rax` at `0x36232`
   = `"assemble"` at `0xcbae3`) on top of the six register arguments. kuna emits six
   arguments; Ghidra emits eight and IDA nine. This is a **P4 (calls)** parameter-recovery
   gap on a variadic callee, and it is a divergence from upstream Ghidra on the same
   binary. It does not move GED and is out of scope for this case, but it is a real
   defect worth mining separately.
2. **The empty-string literal renders as a raw address**: kuna prints
   `v3 = (void *)0xcf952;` where IDA prints `""` and Ghidra prints `&DAT_001cf952`.
   `0xcf952` is a lone NUL in `.rodata`. Cosmetic, P0/P9 string markup.

## Proposed fix

None for the scored gap. The recorded margin of 12 is an artifact of three things, in
order of size:

- 8 points: an inherent floor from the pyjoern source/decompiled asymmetry on a ternary
  that gcc compiled into a `cmovne`. Unreachable by any faithful output.
- 4 points: a guard-polarity preference kuna implements on purpose via `branchflip` /
  `ifelseflatten`.
- 0 points: the reference's advantage, which is entirely that angr dropped the null
  check and 40 instructions it could not lift.

The sibling case `O2-noinline-openssh-portable-sshd-mm_answer_auth2_read_banner` is the
same function and inherits this verdict.

If the campaign wants a follow-up out of this case, take the **P4 stack-argument
recovery** finding above, not the GED margin.

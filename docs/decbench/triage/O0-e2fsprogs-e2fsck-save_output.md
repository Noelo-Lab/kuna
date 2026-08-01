---
case_id: O0-e2fsprogs-e2fsck-save_output
pool: ida
status: feature-candidate
tier: M
margin: 56
fresh_verdict: today's no-flag run is 27 GED points better than the recorded run (56 -> 29, measured with scripts.decbench.rescore) purely from DIV-36 `truthycond`; the residual 29 is 7 extra CFG nodes, the largest single contributor (-12) being two parameter copy-shadows emitted mid-guard-cascade instead of in the entry block
option_closing: null
feature_slug: paramcopyhoist
scope: small
confidence: medium
---

## Side-by-side

Recorded: ida GED=0 (isomorphic to source), kuna GED=56, margin 56. Source CFG
46 nodes / 72 edges, so the GED is a real edit distance, not the >60-node
approximation. Other panes on `ged_new.json`: binja 8, angr 14, r2dec 33,
**ghidra 61** (the queue's `others_ged` mislabels r2dec's 33 as ghidra — ghidra
is *worse* than kuna here, so this is not a kuna-vs-ghidra regression).

Measured CFG sizes (pyjoern, same extractor decbench uses):

| pane | nodes / edges | GED |
|---|---|---|
| source | 46 / 72 | — |
| ida (stored) | 46 / 72 | 0 |
| binja (stored) | 48 / 73 | 8 |
| angr (stored) | 50 / 73 | 14 |
| kuna (stored, 2026-07-27) | 60 / 87 | 56 |
| **kuna (fresh, today, no flags)** | **53 / 77** | **29** |
| ghidra (stored) | 61 / 90 | 61 |

`--mode auto` and `--mode reliable` produce **byte-identical** output on this
function, so the improvement is a code fix, not a mode default.

The whole 56 -> 29 drop is attributable to one shipped option: `--option
truthycond off` reproduces the stored artifact exactly (60 n / 87 e, GED 56).
DIV-36 turned `if ((a0 != (char *)0x0) && (*a0 == '\0'))` into
`if ((a0) && (!*a0))`, which is the shape the source itself uses.

### ida (stored, GED 0) — the entry block is whole

```c
FILE *save_output(char *a1, char *a2, char *a3)
{
  file = a1;
  v5 = a2;
  v4 = a3;                       // all three copies in the ENTRY block
  v13 = __readfsqword(0x28u);
  if ( a1 && !*a1 )
    file = 0;
  if ( a2 && !*a2 )
    v5 = 0;
  if ( a3 && !*a3 )
    v4 = 0;
  ...
  while ( dword_DE648 > 0 )      // one loop, matches source
  {
    v9 = write(fd, buf, dword_DE648);
    if ( v9 >= 0 ) { dword_DE648 -= v9; buf += v9; }
    else if ( *__errno_location() != 11 && *__errno_location() != 4 ) break;
  }
  exit(0);
```

### kuna (fresh, today) — the entry block is split into three

```c
  v11 = *(void *)(v4 + 0x28);
  v7 = a0;
  if ((a0) && (!*a0))
    v7 = NULL;
  v6 = a1;                       // <-- sunk out of the entry block  (+1 CFG node)
  if ((a1) && (!*a1))
    v6 = NULL;
  v5 = a2;                       // <-- sunk out of the entry block  (+1 CFG node)
  if ((a2) && (!*a2))
    v5 = NULL;
  if (((!v7) && (!v6)) && (!v5))
    return 0;
  ...
  do {
    v1 = sub_4edb4(v3);          // source/ida: while (do_read(fds[0]) > 0) ;
  } while (0 < v1);
  ...
  do {
    while( true ) {              // "true" is an IDENTIFIER to joern  (+1 node)
      if (dat_de648 <= 0) {
        exit(0); // return-dupe, no-return   <-- taildup copy         (+1 node)
      }
      v1 = write(v8,v9,(int8)dat_de648);
      if (v1 <= -1) break;
      dat_de648 -= v1;
      v9 += v1;
    }
  } while ((*(int4 *)__errno_location() == 0xb) || (*(int4 *)__errno_location() == 4));
  exit(0);
```

Note the nested `do { while(true) ... } while(...)` is **not** a GED cost: the
node-by-node mapping shows it is the same graph as the source's single
`while (outbufsize > 0)` with a `continue`, just rendered with two loop
keywords. Only the taildup'd `exit(0)` copy adds a node there.

## Source

`~/github/decbench/results/full_run/O0/e2fsprogs/compiled/logfile.i:10892`
(e2fsprogs `e2fsck/logfile.c`), macros expanded:

```c
static FILE *save_output(const char *s0, const char *s1, const char *s2)
{
 int c, fd, fds[2];
 char *cp;
 pid_t pid;
 FILE *ret;

 if (s0 && *s0 == 0) s0 = 0;          /* no copies at all: s0/s1/s2 assigned in place */
 if (s1 && *s1 == 0) s1 = 0;
 if (s2 && *s2 == 0) s2 = 0;
 if (!s0 && !s1 && !s2) return ((void *)0);
 ...
 close(fds[1]);
 while (do_read(fds[0]) > 0) ;        /* empty-body while, call in the condition */
 close(fds[0]);
 fd = -1;
 while (1) { ... }
 cp = outbuf;
 while (outbufsize > 0) {             /* ONE loop */
  c = write(fd, cp, outbufsize);
  if (c < 0) {
   if ((errno == 11) || (errno == 4)) continue;
   break;
  }
  outbufsize -= c;
  cp += c;
 }
 exit(0);
}
```

The three guards write **through** the parameter, so the source's entry block
holds only the frame setup. In the binary the three spills are likewise all in
the entry basic block:

```
4eec2: mov %rdi,-0x38(%rbp)
4eec6: mov %rsi,-0x40(%rbp)     <- kuna prints this after the FIRST if
4eeca: mov %rdx,-0x48(%rbp)     <- kuna prints this after the SECOND if
4eece: mov %fs:0x28,%rax
4eedd: cmpq $0x0,-0x38(%rbp)
4eee2: je   4eef7               <- entry block ends here
```

## Analysis

**The one structural symptom.** kuna splits the source's single entry basic
block into three by emitting the parameter -> stack-slot copy shadows
(`v6 = a1;`, `v5 = a2;`) at the tail of each guard's *join predecessor* instead
of at the function entry where the spill instructions actually live. Measured
cost: +2 CFG nodes, **GED 29 -> 17** when the two lines are textually hoisted
back to the top (semantics-preserving: `a1`/`a2` are unmodified inputs and
`v6`/`v5` are not read before). ida hoists all three and is isomorphic to
source; ghidra sinks them exactly like kuna and scores 61.

**Root cause / owning phase: P6 (variables — HighVariable merge).** The
original spill stores are folded away in P3; P6's merge then has to trim a
MULTIEQUAL input that cannot join the output's HighVariable, and the trim COPY
is anchored at the *end of the phi's incoming predecessor block*. From
`decompiler/crates/kuna-decomp/src/p6_variables/funcdata_merge.rs:840`
(`trim_op_input_prep`, the `Merge::trimOpInput` port):

```rust
        let pc = if is_multiequal {
            let parent = o.get_parent().expect("trim_op_input_prep: no parent");
            let pred = self.bblocks_ref().block(parent).get_in(slot);
            self.block_stop_addr(pred)          // <-- always the predecessor's tail
        } else {
            o.get_addr().clone()
        };
```

`print raw` confirms it: the copy for `a1` is created *inside* block 3, the
join after the first guard, not in block 0 where the spill was:

```
Basic Block 0 0x0004eeba-0x0004eee2
0x0004eee2:8ce: u0x10000269(0x0004eee2:8ce) = RDI(i)            <- prints "v7 = a0;"
Basic Block 3 0x0004eef7-0x0004eefc
0x0004eef7:77c: s0xff..ffc0 = u0x10000269 ? u0x10000269 ? s0xff..ffc0   (phi for v7)
0x0004eefc:8cc: u0x10000259(0x0004eefc:8cc) = RSI(i)            <- prints "v6 = a1;"
```

Because the trimmed input is a *function input* varnode (SSA-defined once, at
entry, and provably unmodified along the dominator path), the predecessor tail
is the most pessimistic legal anchor rather than the necessary one.

**Why ida wins:** it emits the same three copies, but at the spill site, so
they merge into the entry basic block and the CFG is isomorphic to source.

**No option closes it.** Swept every P7/P8 structurer option
(`regionstructure`, `regionlooprefine`, `regionedgeorder`, `condfold on|wide`,
`gotoreduce`, `ifelseflatten`, `crossjumprevert`, `taildup`, `earlyreturn`,
`returndup`, `loopbreak_recovery`, `branchflip`) — the emitted structure is
invariant except for `taildup`.

### The rest of the residual 29 (secondary, recorded so it is not re-mined)

Textual ablations against the fresh block, each measured with the real GED:

| divergence | Δnodes | GED | verdict |
|---|---|---|---|
| `v6 = a1;` / `v5 = a2;` sunk out of entry | +2 | 29 -> **17** | **real defect (above)** |
| `while( true )` vs source `while (1)` | +2 | 29 -> 21 | **metric artifact** |
| `do { v=sub_4edb4(x); } while (0<v)` vs `while (f(x) > 0) ;` | +2 | 29 -> 23 | mostly artifact |
| taildup'd second `exit(0)` | +1 | 29 -> 24 | real, minor; `taildup off` gives 26 |
| all three of the above together | -6 | 29 -> **7** | (better than binja's 8) |

* `while( true )`: pyjoern parses bare `true` as an IDENTIFIER and gives the
  loop header its own CFG block; `while (1)` folds into its predecessor. The
  control flow is identical, and kuna's project export emits
  `#include <stdbool.h>` (`infra/decompile_drive.rs:1032`), so the C is valid.
  This is a decbench-harness penalty that hits **every** kuna and ghidra
  function containing an unconditional loop — worth reporting upstream rather
  than "fixing" in the printer.
* the `do_read` loop: the source writes `while (do_read(fds[0]) > 0) ;` and
  pyjoern loses the back edge of that empty-body while entirely (source block
  30 has no self-edge). ida reproduces the same text and inherits the same
  dropped edge; kuna's faithful `do {} while` is *penalised for being correct*.
  Residual real content: kuna never folds a single-call loop body into the loop
  condition. Low value, mostly artifact.
* `taildup` duplicating a **no-return call** tail (`exit(0)`) rather than a
  `return` tail is worth a look on its own — its documented gate is "return-call
  tail", and duplicating an `exit()` costs a node against a source that reaches
  one `exit(0)` from two paths.

## Proposed fix

**Feature `paramcopyhoist` (P6, default-off), new module
`decompiler/crates/kuna-decomp/src/p6_variables/kuna_paramcopyhoist.rs`.**

Mechanism: in `trim_op_input_prep`
(`p6_variables/funcdata_merge.rs:838-849`), when the MULTIEQUAL input being
trimmed is a function-input varnode (`isInput`) and the phi's predecessor block
is strictly dominated by the entry block, anchor the trim COPY at the entry
block's stop address (or, better, at the address of the original addr-tied
spill store, which the varmap still knows) instead of `block_stop_addr(pred)`.
`snip_reads`/`allocate_copy_trim` in `p6_variables/merge.rs:1825` already take
a `(block, addr, after_op)` triple, so only the anchor computation changes.

Owning files:
- `decompiler/crates/kuna-decomp/src/p6_variables/kuna_paramcopyhoist.rs` (new: eligibility + anchor)
- `decompiler/crates/kuna-decomp/src/p6_variables/funcdata_merge.rs` (`trim_op_input_prep` calls it)
- `decompiler/crates/kuna-decomp/phases.toml` + `src/p0_knowledge/options.rs` (option row)
- `docs/spec/` P6 chapter; `tests/stages/` two-pass testcase; catalog count bumps

Risks:
- **Cover widening is the whole point of the trim.** Hoisting the COPY extends
  its output's cover from one block to the entry-to-phi dominator path, which
  can re-introduce the HighVariable interference `Merge::trimOpInput` exists to
  avoid. The eligibility test must re-run the intersection check on the widened
  cover and decline on any conflict — that is the load-bearing part of the
  feature, not the anchor change.
- Statement placement shifts in many functions, so it must ship default-off
  (byte-identical to upstream); default-on needs a 0/675 datatest flip plus a
  decbench aggregate ablation, since sinking is upstream Ghidra behaviour and
  some functions may read better sunk.
- Payoff is bounded: -12 GED here, and the pattern (a parameter conditionally
  overwritten by a guard, so its spill feeds a phi) is common at O0 but rare at
  O2 where the spills are gone.

**Nothing to fix** for `while( true )` (harness-side) or the `do_read` loop
(source-side parse loss). `taildup`'s no-return-tail duplication is a separate,
smaller ticket.

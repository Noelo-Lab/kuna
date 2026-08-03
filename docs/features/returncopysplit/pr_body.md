## The bug

kuna emitted a block of per-byte assignments **storing into a `.rodata` string literal**, on
a return path where the binary performs no store at all.

iproute2 `ip` (O2-noinline, stripped) `netns_add` @ `0x20e10`, today's build, **no flags**:

```c
/* before */                                       /* after */
label_21004:                                       label_21004:
      if (v3 == -1) {                                    if (v3 == -1)
        s_822f0[0] = (char)s_822f0[0];                     return 0xffffffff;
        s_822f0[1] = SUB81(s_822f0[0],1);                flock(v3,8);
        s_822f0[2] = SUB81(s_822f0[0],2);                close(v3);
        s_822f0[3] = SUB81(s_822f0[0],3);                return 0xffffffff;
        s_822f0[4] = SUB81(s_822f0[0],4);
        s_822f0[5] = SUB81(s_822f0[0],5);
        s_822f0[6] = SUB81(s_822f0[0],6);
        s_822f0[7] = SUB81(s_822f0[0],7);
        s_822f0[8] = (char)s_822f0[8];
        s_822f0[9] = SUB81(s_822f0[8],1);
        s_822f0[10] = SUB81(s_822f0[8],2);
        s_822f0[0xb] = SUB81(s_822f0[8],3);
        s_822f0[0xc] = SUB81(s_822f0[8],4);
        s_822f0[0xd] = SUB81(s_822f0[8],5);
        s_822f0[0xe] = SUB81(s_822f0[8],6);
        s_822f0[0xf] = SUB81(s_822f0[8],7);
        return 0xffffffff;
      }
      flock(v3,8);
      close(v3);
      return 0xffffffff;
```

The source line is `return -1;` (`ipnetns.c:891`).

### Proof the stores are phantom

- `readelf -SW ip` puts `.rodata` at `0x7e000..0x95335` with flags `A` and **no `W`**;
  `0x822f0` is inside it.
- `objdump -d ip | grep -E '# +822f0'` returns **exactly one** instruction in the whole
  binary, and it is a **load**:
  `21140: 66 0f 6f 05 a8 11 06 00   movdqa 0x611a8(%rip),%xmm0   # 822f0`.
- The decorated block `0x20f7f..0x20fb0` disassembles to `mov $-1,%r13d`, the canary
  compare, `add $0x2048,%rsp`, the pops and `ret`. **No stores.**

## The mechanism (the filed one was refuted; this one is instrumented)

The triage record on main proposed declining the split for a heritage RETURN-COPY via
`is_return_copy()`. **That predicate is dead on this path.** Instrumenting
`SplitDatatype::test_copy_constraints` shows `rc=true` never reaches a successful split:

```
[SPLITCOPY] op=0x00020fb0:b56  rc=true  in=0x000822f0(tied=true)  out=0x000822f0(tied=true,ro=true) -> false
[SPLITCOPY] op=0x00020fb0:119b rc=false in=0x1000044a(tied=false) out=0x000822f0(tied=true,ro=true) -> true   <-- phantom producer
[SPLITCOPY] op=0x00020fb0:119c rc=false in=0x10000452(tied=false) out=0x000822f8(tied=true,ro=true) -> true   <-- phantom producer
[SPLITCOPY] op=0x00021160:b1f  rc=false in=0x000822f0(tied=true)  out=0xffffffffffffefb8(ro=false)  -> true   <-- the REAL copy site
```

The actual chain:

1. `strings` (P1, default on) plants a typelocked `char[N]` at `ram:0x822f0`; the 16-byte
   `movdqa` makes `ram:0x822f0(8)` / `ram:0x822f8(8)` heritaged **persistent** ranges.
2. `Heritage::guard_returns` inserts a synthetic `glob = glob` COPY before each RETURN,
   flagged `return_copy` and address-tied at the **same** address on both sides — correctly
   declined by upstream's existing gate, and then hidden by `Merge::mark_internal_copies`.
3. kuna's `ActionStripStackGuard` (option `stackguard`, DIV-14, default on) strips the
   canary epilogue, leaving a **bare-epilogue RETURN block** that passes
   `Funcdata::return_split_is_splittable`.
4. `ActionReturnSplit` (`p8_structure/blockaction.rs:3908`, inside `fullloop`, i.e. **before**
   the cleanup pool) node-splits it. `CloneBlockOps::build_op_clone`
   (`substrate/funcdata_block.rs:3988`) copies the upstream flag mask, which **omits**
   `return_copy`, and `CloneBlockOps::patch_inputs` re-inputs the clone from a freshly
   created unique-space COPY — so the clone is neither `return_copy`-marked **nor**
   address-tied at its own address, and **both** upstream guards miss it.
5. `RuleSplitCopy` splits it into eight one-byte COPYs per half; the pieces land in different
   HighVariables, `mark_internal_copies` can no longer hide them, and P9 prints them.

## The fix

One predicate in `SplitDatatype::test_copy_constraints`
(`p3_dataflow/subflow.rs`, upstream `subflow.cc:2390`):

```rust
// (kuna) A COPY whose output lands in a read-only address range is never a
// store the program performs; splitting it turns one invisible whole-range
// assignment into per-element assignments that survive
// `Merge::mark_internal_copies` and print as stores into `.rodata`.
let out_vn = data.obank().get(copy_op).expect("stale copy").get_out().expect("copy out");
if data.vbank().get(out_vn).expect("stale out").is_read_only() {
    return false;
}
```

Read-only-ness of the **destination** is the property that survives the clone *and* the
property that actually makes the statement wrong. `Varnode::is_read_only` already exists and
is painted from the load image's non-writable sections. The invariant is one Ghidra already
recognizes — `Funcdata::fillin_read_only` has a `Read-only address (ram,X) is written`
warning path — it is just gated behind `readonlypropagate`.

**No option**: emitting a store the program cannot perform is wrong output, not a judgment
call (`CLAUDE.md`: "a strict bug fix that only corrects wrong output needs no flag").
`option togglerule splitcopy off` already gives a bisector the pre-fix pipeline. It is
recorded as **DIV-55** rather than a Convergence, because the test does not exist upstream
and the CFG shape that exposes the hole is produced by kuna's own `stackguard` divergence.

Deliberately **not** in this PR: carrying `return_copy` across `build_op_clone`. That mask is
a verbatim transcription of upstream `funcdata_block.cc:982-988` and changing it would be a
second, wider divergence affecting every node-split. The read-only output test closes the
emitted-C bug on its own, corpus-wide.

## Breadth and regression sweep

Whole-binary `kuna decompile-all --json`, before/after release builds of this worktree,
diffed **per function** over 12 corpus binaries:

| binary | functions | changed | -lines | +lines | rodata stores before/after |
|---|---|---|---|---|---|
| O2-noinline iproute2 `ip` | 1,912 | **2** | 52 | 2 | 48 / 0 |
| O2-noinline openssh `sshd` | 2,042 | **1** | 64 | 0 | 64 / 0 |
| O0 iproute2 `ip` | 1,962 | 0 | 0 | 0 | 0 / 0 |
| bash | 3,077 | 0 | 0 | 0 | 0 / 0 |
| tar | 1,585 | 0 | 0 | 0 | 0 / 0 |
| dpkg | 986 | 0 | 0 | 0 | 0 / 0 |
| findutils `find` | 931 | 0 | 0 | 0 | 0 / 0 |
| gnutls `certtool` | 833 | 0 | 0 | 0 | 0 / 0 |
| coreutils `ls` | 639 | 0 | 0 | 0 | 0 / 0 |
| coreutils `sort` | 495 | 0 | 0 | 0 | 0 / 0 |
| shadow `useradd` | 485 | 0 | 0 | 0 | 0 / 0 |
| gnutls `gnutls-cli` | 439 | 0 | 0 | 0 | 0 / 0 |
| **total** | **15,386** | **3** | **116** | **2** | **112 / 0** |

The three changed functions are exactly the three the triage found: `ip` `netns_add`
(-18/+1), `ip` `__get_hz` (-34/+1), `sshd` `sub_230a0` (-64/+0). Every other function in the
sweep is **byte-identical**.

**No real store is lost.** After the fix, every surviving assignment whose LHS is an
address-named global lands in a **writable** (`WA`) section, and every one of those counts is
unchanged by the fix: bash 22 (`.data`/`.bss`), tar 6 (`.bss`), coreutils `sort` 4 (`.data`),
dpkg 1 (`.data`), sshd 1 (`.bss`). The real copy site in `netns_add` (16 per-byte **reads**
of the literal into the stack buffer) is untouched.

## Ablation, gates, test

- **`make test` → `PARITY OK` 675/675**, `docs/baseline.json` **not** re-pinned.
- **`make test-stages` → `PARITY OK` 375/375**; the stages baseline is re-recorded purely
  additively (369 → 375 keys), no existing assertion moved, no per-test opt-out.
- **`make rust-test` → green** (whole workspace, `docs/options.md` freshness fence included).
- **`make check-spec` → `check-spec OK (lenient mode)`**.
- **`kuna catalog --check` → `catalog OK`** (no option added: `phases.toml`,
  `p0_knowledge/options.rs`, the catalog counts and `docs/options.md` are untouched).

`tests/stages/ghdec-returncopysplit.xml` — **3 of 6 assertions pass pre-fix, 6/6 post-fix**.
Real `ip` bytes: `__get_hz` at its real VMA `0x6cbf0` (433 bytes verbatim, plus one synthetic
`0xc3` so the fall-through of the final `call __stack_chk_fail` terminates — the PLT stub is
outside the chunk, so the engine cannot know it does not return), plus the 32-byte
`readonly="true"` literal chunk at `0x92e50`. `map addr r0x92e50 char s_92e50[32]` stands in
for the strings analysis pass, which does not run on a bytechunk image. Pass 1 is the default
(the fix); pass 2 sets `option togglerule splitcopy off` to pin the unsplit rendering.

## Speed

No mechanism for a cost: one varnode-flag test per COPY that `RuleSplitCopy` already
considers, and a declined split does strictly **less** work. Interleaved A/B medians on the
two builds (`scripts.pipeline.timeit` is inapplicable — it measures an option flip):

| target | before | after | delta | pairs |
|---|---|---|---|---|
| `ip` `netns_add` @0x20e10 (changed) | 1,349.4 ms | 1,334.9 ms | **-1.08%** | 15 |
| `ip` `__get_hz` @0x6cbf0 (changed) | 1,002.1 ms | 1,006.2 ms | **+0.41%** | 15 |
| `sshd` `sub_230a0` (changed) | 2,371.1 ms | 2,117.5 ms | **-10.69%** | 15 |
| coreutils `sort` `decompile-all --mode reliable` | 5,735.1 ms | 5,816.3 ms | +1.42% | 11 |
| coreutils `ls` `decompile-all --mode reliable` | 9,179.3 ms | 8,802.5 ms | -4.10% | 5 |

Honest caveat: this box runs concurrent agents. A **byte-identical control function**
(`ip` @0x20780) measured -12.18% and -20.23% across two runs, and `sort`'s whole-binary
output is byte-identical in both arms yet measured +9.17% on a 5-pair run and +1.42% on an
11-pair one — so anything inside roughly ±10% here is load, not signal. The changed targets'
highest-pair-count deltas are -1.08%, +0.41% and -10.69%, all inside the +5% budget.

## Benchmark

`scripts.decbench.rescore --case O2-noinline-iproute2-ip-netns_add`:
**GED 8.0 → 8.0, delta 0.0.** Zero, and expected to be zero: the fix deletes statements
inside one basic block and creates or destroys no block and no edge, so the graph-edit-distance
metric cannot see it. This case is ranked by correctness, not by GED.

Full bundle: `docs/features/returncopysplit/` (`analysis.md`, `plan.md`, `record.json`).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

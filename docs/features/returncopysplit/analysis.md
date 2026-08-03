# returncopysplit -- kuna stores into `.rodata` on a path with no store

decbench campaign case `O2-noinline-iproute2-ip-netns_add`, NOVEL pool.
Triage record on main: `docs/decbench/triage/rodata-phantom-store.md` (its *symptom* stands;
its *proposed mechanism* is refuted below and superseded).

## 1. The symptom

`kuna decompile ~/github/decbench/results/full_run/O2-noinline/iproute2/stripped/ip --addr 0x20e10`
(`netns_add`), today's build, **no flags**:

```c
      if (v3 == -1) {
        s_822f0[0] = (char)s_822f0[0];
        s_822f0[1] = SUB81(s_822f0[0],1);
        ...
        s_822f0[0xf] = SUB81(s_822f0[8],7);
        return 0xffffffff;
      }
```

Sixteen statements **storing into a `.rodata` string literal**. The source line is
`return -1;` (`ipnetns.c:891`).

Three witnesses on today's build, all with no flags:

| binary | function | phantom statements |
|---|---|---|
| iproute2 `ip` | `netns_add` @`0x20e10` | 16 |
| iproute2 `ip` | `__get_hz` @`0x6cbf0` | 32 |
| openssh `sshd` | `sub_230a0` @`0x230a0` | 64 |

## 2. They are phantom

- `readelf -SW ip`: `.rodata` is `0x7e000..0x95335`, flags `A`, **no `W`**.
- `objdump -d ip | grep -E '# +822f0'` returns **exactly one** instruction in the whole
  binary and it is a **load**: `movdqa 0x611a8(%rip),%xmm0`.
- The decorated block `0x20f7f..0x20fb0` is `mov $-1,%r13d`, the canary compare, the stack
  adjust, the pops and `ret`. **No stores.**

## 3. The filed mechanism is refuted

The triage proposed declining the split for a heritage RETURN-COPY via `is_return_copy()`
in `SplitDatatype::test_copy_constraints`. Instrumenting that function (printing every
candidate COPY with its `return_copy` flag, both varnodes' addresses and their
`addr_tied`/`read_only`/`persist` bits) shows **the predicate changes nothing**: on
`netns_add`, every `rc=true` entry is already declined by the existing same-address gate,
and neither of the two COPYs that actually get split is marked `return_copy`.

```
[SPLITCOPY] op=0x00020fb0:b56  rc=true  in=0x000822f0(tied=true,...)  out=0x000822f0(tied=true,ro=true,persist=true) -> false
[SPLITCOPY] op=0x00020fb0:119b rc=false in=0x1000044a(tied=false,...) out=0x000822f0(tied=true,ro=true,persist=true) -> true   <-- phantom producer
[SPLITCOPY] op=0x00020fb0:119c rc=false in=0x10000452(tied=false,...) out=0x000822f8(tied=true,ro=true,persist=true) -> true   <-- phantom producer
[SPLITCOPY] op=0x00021160:b1f  rc=false in=0x000822f0(tied=true,...)  out=0xffffffffffffefb8(tied=true,ro=false,persist=false) -> true  <-- the REAL copy site
```

Two ops produce all 16 statements (2 halves x 8 bytes). Both have `rc=false`, an input at a
**unique**, and an output that is address-tied at a **read-only** ram address.

## 4. The actual chain

1. `strings` (P1 analysis tier, default on) plants a typelocked `char[N]` at
   `ram:0x822f0`.
2. The 16-byte `movdqa` read makes `ram:0x822f0(8)` and `ram:0x822f8(8)` heritaged
   **persistent** ranges.
3. `Heritage::guard_returns` (`p3_dataflow/heritage.rs`, upstream `heritage.cc:1677`)
   inserts a synthetic `glob = glob` COPY before every RETURN, flagged `return_copy`,
   address-tied at the **same** address on both sides. Upstream's own gate in
   `SplitDatatype::test_copy_constraints` (`p3_dataflow/subflow.rs:3553`, upstream
   `subflow.cc:2390`) declines it, and `Merge::mark_internal_copies` then marks it
   non-printing. **This is all correct and is not the bug.**
4. kuna's `ActionStripStackGuard` (option `stackguard`, DIV-14, default on;
   `infra/universalaction.rs`) strips the `-fstack-protector` epilogue. What is left is a
   **bare-epilogue RETURN block** that passes `Funcdata::return_split_is_splittable`.
5. `ActionReturnSplit` (`p8_structure/blockaction.rs:3908`, scheduled inside `fullloop`,
   i.e. **before** the cleanup pool) calls `Funcdata::return_split_apply` -> `node_split` ->
   `CloneBlockOps::clone_block`. A backtrace on `Funcdata::new_varnode_out` for
   `ram:0x822f0` shows this is the **sole** producer of the offending COPYs.
6. `CloneBlockOps::build_op_clone` (`substrate/funcdata_block.rs:3988`) copies the upstream
   flag mask, which **omits `return_copy`**; `CloneBlockOps::patch_inputs` rewrites the
   cloned MULTIEQUAL into a COPY whose input is a freshly created unique-space varnode. So
   the clone is neither `return_copy`-marked nor address-tied at its own address, and
   **both** of upstream's guards miss it.
7. `RuleSplitCopy` splits it into eight one-byte COPYs per half. Each piece lands in a
   different HighVariable, so `Merge::mark_internal_copies` can no longer hide the group,
   and P9 prints them as stores into the read-only literal.

**Owning phase: P3** (definition web / simplification quiescence, cleanup pool). The bad
decision is "may a datatype-driven copy split rewrite a copy whose destination is read-only
memory".

## 5. Breadth

`kuna decompile-all --json` over 12 corpus binaries / 15,386 functions, counting assignment
statements whose LHS is an address-named global:

| binary | functions | phantom `.rodata` stores |
|---|---|---|
| iproute2 `ip` (-O2) | 1,912 | 48 (2 functions) |
| openssh `sshd` | 2,042 | 64 (1 function) |
| bash, tar, dpkg, coreutils `ls`/`sort`, findutils `find`, gnutls `certtool`/`gnutls-cli`, shadow `useradd` | 9,470 | 0 |
| iproute2 `ip` (-O0) | 1,962 | 0 |

**112 phantom statements in 3 of 15,386 functions.** Rare but severe: the preconditions are
an -O2 SIMD/word copy **out of** a `.rodata` literal, in a `-fstack-protector` function
(so `stackguard` reshapes the returns), on a path whose reaching def is not address-tied.
`-O0` never produces it.

## 6. The fix

`SplitDatatype::test_copy_constraints` declines the split when the COPY's **output Varnode
is read-only** (`Varnode::is_read_only`, the `readonly` property the load image reports for
non-writable sections). A store into a read-only range is never something the program
performs, so declining can only ever delete statements the program does not execute -- a
real store is a `STORE` op or a COPY into a writable global.

`is_return_copy()` is deliberately **not** used: it is exactly the property the clone
destroys (see section 3), so it is both unnecessary (upstream's same-address gate already
covers the un-cloned form) and insufficient.

The invariant is one Ghidra already recognizes:
`Funcdata::fillin_read_only` (`substrate/funcdata_varnode.rs:1826`) has a
`Read-only address (ram,X) is written` warning path -- it is just gated behind
`readonlypropagate`, which upstream keeps off for RELRO / `.data.rel.ro` reasons.

**Rejected alternatives.** `--option readonly on` closes the case but is a global semantic
policy (every read-only load folds to a constant), default-off upstream, and has no
`settableTable` row at all. `--mode aggressive` / `formatstring on` close it incidentally.
`stackguard off` closes it but is a contributor via CFG shape, not the root. Fixing the
clone fidelity in `build_op_clone` (carrying `return_copy` across the clone) would be a
second, wider divergence from a verbatim-transcribed upstream mask and is out of scope for
this PR.

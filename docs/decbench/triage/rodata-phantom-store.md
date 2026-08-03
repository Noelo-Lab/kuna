---
case_id: O2-noinline-iproute2-ip-netns_add
pool: novel
group_id: iproute2::netns_add
status: feature-candidate
tier: N
margin: 0
fresh_verdict: reproduces on today's product default (both binaries are >500 KiB so `--mode auto` == `reliable`); the round-1 diagnosis is REFUTED by op identity -- `Heritage::refine_write` is not involved, and the ram-addressed `build_in_subpieces` outputs are not the phantom stores
option_closing: null
feature_slug: returncopysplit
scope: small
confidence: high
---

Re-triage of the round-1 Tier-1 item `rodata-phantom-store`
(supersedes the diagnosis in `docs/decbench/triage/O2-noinline-iproute2-ip-netns_add.md`;
the *symptom* in that record stands, the *mechanism* does not).

## 1. Verify-first: it still reproduces

`ip` is 739,976 B and `sshd` is 1,191,064 B, both > 500 KiB, so `--mode auto` selects
`reliable` — the fresh no-flag run and `--mode reliable` are the same run, and both are
what a user gets today.

```
$ export SLEIGHHOME=/home/mahaloz/github/kuna/specs
$ ./decompiler/target/release/kuna decompile \
    ~/github/decbench/results/full_run/O2-noinline/iproute2/stripped/ip --addr 0x20e10
...
label_21004:
      if (v3 == -1) {
        s_822f0[0] = (char)s_822f0[0];
        s_822f0[1] = SUB81(s_822f0[0],1);
        s_822f0[2] = SUB81(s_822f0[0],2);
        s_822f0[3] = SUB81(s_822f0[0],3);
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

Mode / option sweep, counting emitted lines that mention `s_822f0[`
(16 phantom LHS + 16 legitimate RHS at the real copy site = 32):

| run | lines |
|---|---|
| no flag (`auto` -> `reliable`) | 32 |
| `--mode reliable` | 32 |
| `--mode aggressive` | 0 |
| `--option readonly on` | 0 |
| `--option strings off` | 0 |
| `--option formatstring on` | 0 |
| `--option stackguard off` | **16** (real site only) |
| `--option condfold on` | 32 |
| `--option memsetrecover off` | 32 |
| `--option returndup off` / `earlyreturn off` / `switchreturn off` / `gotoreduce off` | 32 |

`--mode aggressive` closes it only because the aggressive preset carries `formatstring`
(same finding as round 1). It is not a fix, and it is not the default at this binary size.

## 2. The stores really are into `.rodata`, and really are not in the binary

`readelf -SW ip` — `.rodata` is `0x7e000 .. 0x95335`, flags `A` (no `W`).
`0x822f0` is inside it.

The whole binary contains **exactly one** instruction that references `0x822f0`, and it is
a load:

```
$ objdump -d ip | grep -E '# +822f0'
   21140:  66 0f 6f 05 a8 11 06 00   movdqa 0x611a8(%rip),%xmm0   # 822f0
```

The block kuna decorates (`0x20f7f`, reached by `je 20f7f` at `0x21004`) contains no
stores at all:

```
   20f7f:  41 bd ff ff ff ff        mov    $0xffffffff,%r13d
   20f85:  48 8b 84 24 38 20 00 00  mov    0x2038(%rsp),%rax
   20f8d:  64 48 2b 04 25 28 00 00  sub    %fs:0x28,%rax
   20f96:  0f 85 89 03 00 00        jne    21325
   20f9c:  48 81 c4 48 20 00 00     add    $0x2048,%rsp
   20fa3:  44 89 e8                 mov    %r13d,%eax
   20fa6..20fae:                    pop    %rbx/%rbp/%r12/%r13/%r14/%r15
   20fb0:  c3                       ret
```

## Source

`~/github/decbench/results/full_run/O2-noinline/iproute2/compiled/ipnetns.i` (ipnetns.c:883-891)
— the path kuna decorates:

```c
   if (lock != -1) {
    flock(lock, LOCK_UN);
    close(lock);
   }
   return -1;
```

The only copy in the function (ipnetns.c:914) is on a completely different path:

```c
  strcpy(proc_path, "/proc/self/ns/net");
```

## 3. The filed mechanism is REFUTED (instrumented, not read)

Round 1 filed: *"`SplitDatatype::build_in_subpieces` splits a 16-byte read-only copy into
ram-addressed SUBPIECE outputs; heritage refinement then synthesises write-backs into
.rodata"*, with the fix living in `build_in_subpieces` + `Heritage::refine_write`.
Two independent lines of evidence say no.

**(a) There is no heritage pass after the split.** `RuleSplitCopy` is registered at
`infra/universalaction.rs:558` — which is inside `cleanup_rules` (the list runs
`:547`..`:571`), *not* `oppool2` (which ends at `:545` and holds only pushptr /
structoffset0 / ptrarith / loadvarnode / storevarnode). That pool is scheduled at
`:715`, **after `fullloop`**, i.e. after the last `ActionHeritage`. This is faithful to
upstream (`coreaction.cc:5853`, `actcleanup`). `Heritage::refine_write` therefore cannot
be the producer of the write-backs, and in fact nothing in this function is refined.

**(b) Op identity.** Console `print raw` on the same function, with and without the
trigger, identifies the exact op that is destroyed and what replaces it.

`--option strings off` (no `char[]` typelock at `0x822f0`), block 24:

```
Basic Block 24 0x00020f7f-0x00020fb0
0x00020fa3:1185:  EAX(0x00020fa3:1185) = #0xffffffff:4
0x00020fb0:1186:  r0x000822f0(0x00020fb0:1186) = u0x1000044a(0x00021004:112e)
0x00020fb0:1187:  r0x000822f8(0x00020fb0:1187) = u0x10000452(0x00021004:112f)
0x00020fb0:1188:  r0x000b5200(0x00020fb0:1188) = u0x1000045a(0x00021004:1130)
0x00020fb0:1189:  return(#0x0) EAX(0x00020fa3:1185)
```

Three plain COPYs at the address of the `ret`, one per persistent global, all invisible in
the emitted C (`if (v3 == -1) return 0xffffffff;`).

Default (`strings` on), same block:

```
Basic Block 24 0x00020f7f-0x00020fb0
0x00020fa3:119a:  EAX(0x00020fa3:119a) = #0xffffffff:4
0x00020fb0:11aa:  u0x1000044a:1(0x00020fb0:11aa) = SUB81(u0x1000044a(0x00021004:112e),#0x0:4)
...                                                (8 SUBPIECEs)
0x00020fb0:11b2:  r0x000822f0:1(0x00020fb0:11b2) = u0x1000044a:1(0x00020fb0:11aa)
...                                                (8 COPYs into ram:0x822f0..f7)
0x00020fb0:11ba:  u0x10000452:1(0x00020fb0:11ba) = SUB81(u0x10000452(0x00021004:112f),#0x0:4)
...                                                (8 SUBPIECEs)
0x00020fb0:11c2:  r0x000822f8:1(0x00020fb0:11c2) = u0x10000452:1(0x00020fb0:11ba)
...                                                (8 COPYs into ram:0x822f8..ff)
0x00020fb0:119d:  r0x000b5200(0x00020fb0:119d) = u0x1000045a(0x00021004:1130)
0x00020fb0:119e:  return(#0x0) EAX(0x00020fa3:119a)
```

Seqnums `119b` and `119c` — the two 8-byte return-COPYs for `0x822f0` / `0x822f8` — are
**gone from the whole dump** (`grep ':119[bc]:' -> no match`); the 32 ops `11aa..11c9`
stand in their place. `119d` (the `stderr` return-COPY, a pointer type, not splittable)
survives untouched. The SUBPIECE outputs are at **unique** addresses (`u0x1000044a+k`),
which is `build_in_subpieces` deriving piece addresses from *the copy's input root*, and
the ram-addressed varnodes are `build_out_varnodes` outputs — the signature of
`SplitDatatype::split_copy`, not of `Heritage::split_pieces`.

For completeness: `build_in_subpieces` *does* create ram-addressed SUBPIECE outputs, but at
the **real** copy site, and they are reads of the literal, not the phantom stores:

```
0x00021160:11ca:  r0x000822f0:1 = SUB81(r0x000822f0(0x00021123:b44),#0x0:4)
0x00021160:11d9:  s0xffffffffffffefb8:1 = r0x000822f0:1(0x00021160:11ca)
```

So the round-1 narrow fix ("emit per-piece constants when the root is address-tied into a
read-only range") would clean up the *real* site's rendering and **would not remove a
single phantom store**: the phantom copy's root is `u0x1000044a`, a unique, on which
`Varnode::is_read_only()` is false and `generate_constants` cannot fire.

## Analysis — the actual chain

One structural symptom: **kuna emits a block of per-byte assignments storing into a
`.rodata` string literal, on a return path where the binary performs no store at all.**

1. `strings` (P1 analysis tier, default ON) plants a typelocked `char[N]` at `ram:0x822f0`.
2. The 16-byte `movdqa` read makes `ram:0x822f0(8)` and `ram:0x822f8(8)` heritaged
   **persistent** ranges.
3. `Heritage::guard_returns` (`p3_dataflow/heritage.rs:1784`, persist branch; upstream
   `heritage.cc:1677-1692`) inserts, **before every RETURN**, a synthetic COPY
   `glob = glob` whose output is `addrForce` + flagged `return_copy`
   (`heritage.rs:1851`). Its only job is to hold the global's value past the end of the
   function so `ActionDeadCode` keeps the defining chain alive. It is upstream behavior and
   is normally invisible: `Merge::mark_internal_copies` (`p6_variables/merge.rs:2120`)
   marks a COPY whose in and out share a HighVariable as non-printing.
4. `RuleSplitCopy` -> `SplitDatatype::split_copy` (P3, `cleanup` pool) then splits that
   synthetic COPY, because its input/output data type is the `char[]`. The gate that should
   have declined it is `SplitDatatype::test_copy_constraints`
   (`p3_dataflow/subflow.rs:3552`; upstream `subflow.cc:2390`), which bails when input and
   output are address-tied at the *same* address. On the sibling return path (block 23) the
   copy is `r0x822f0 = r0x822f0(INDIRECT)` and the gate fires. On this path the reaching
   def is a MULTIEQUAL whose output lives at a unique (`u0x1000044a`), so the input is not
   address-tied and the gate does not fire.
5. After the split there are 16 one-byte COPYs whose input and output are in *different*
   HighVariables, so `mark_internal_copies` can no longer hide them, and P9 prints them.

`--option stackguard off` removes the phantom because the canary-strip is what leaves this
return path reading the unique-addressed MULTIEQUAL directly; with the canary in place the
paths funnel through a ram-addressed MULTIEQUAL at `0x20f85` and the return-COPY is
`r0x822f0 = r0x822f0`, which the gate declines. `stackguard` is a **contributor via CFG
shape, not the root** — the root is that a synthetic `return_copy` is eligible for a
datatype split at all.

**Owning phase: P3 (definition web).** The bad decision is "may a datatype-driven copy
split rewrite a heritage RETURN-COPY into per-element stores". Not P9 (the ops genuinely
exist in the IR), not P1 (`strings` planting a `char[]` at a string literal is correct),
not P7 (`stackguard` only changes which def the copy reads).

## Breadth

`kuna decompile-all <bin> --json --mode reliable`, counting assignment statements whose LHS
is an address-named global that lands in a non-writable PROGBITS section
(scanner: `/tmp/.../scratchpad/scan.py`):

| binary (O2-noinline unless noted) | functions | rodata stores | functions hit |
|---|---|---|---|
| iproute2 `ip` | 1912 | **48** | 2 (`netns_add` @0x20e10, `__get_hz` @0x6cbf0) |
| openssh `sshd` | 2042 | **64** | 1 (`sub_230a0` @0x230a0) |
| tar `tar` | 1585 | 1 | 1 (`dat_6a7d6 += v18` — a *different* symptom, not this family) |
| bash `bash` | 3077 | 0 | 0 |
| gnutls `certtool` | 794 | 0 | 0 |
| gnutls `gnutls-cli` | 436 | 0 | 0 |
| dpkg `dpkg` | 975 | 0 | 0 |
| findutils `find` | 873 | 0 | 0 |
| coreutils `ls` | 597 | 0 | 0 |
| coreutils `sort` | 489 | 0 | 0 |
| shadow `useradd` | 484 | 0 | 0 |
| iproute2 `ip` (O0) | 1962 | 0 | 0 |

**112 phantom statements across 3 functions in 11,817 functions scanned** (0.025% of
functions). Rare but concentrated and severe: 16-64 illegal statements per hit. The count
is arithmetically consistent with the mechanism — one split per (RETURN-COPY x 8-byte
half): `netns_add` has 1 affected return block x 2 halves x 8 = 16; `__get_hz` has 2 x 2 x 8
= 32; 16 + 32 = 48. `sshd`'s `sub_230a0` copies a 64-byte literal (8 halves x 8 = 64).

Preconditions, which explain the sparsity: an O2 SIMD/word copy **out of** a `.rodata`
literal, in a function with `-fstack-protector` (so `stackguard` reshapes the returns), on a
path where the global's reaching def is not address-tied. `-O0` never produces it.

## Proposed fix — one module, no option

`decompiler/crates/kuna-decomp/src/p3_dataflow/subflow.rs`,
`SplitDatatype::test_copy_constraints` (`:3552`): decline the split when the COPY is a
heritage return-copy.

```rust
// a return_copy is a synthetic op held only to keep a persistent global's final
// value live past the RETURN; it is not a store the program performs.
if data.obank().get(copy_op).expect("stale copy").is_return_copy() {
    return false;
}
```

The predicate already exists (`substrate/op.rs:822 is_return_copy`, set at
`heritage.rs:1851`, preserved across `set_opcode` by the flag mask at `op.rs:489`), and
upstream already special-cases `isReturnCopy()` in a sibling rule
(`ruleaction.cc:3951` / kuna `ruleaction_3.rs:1900`, `RulePropagateCopy`) — so the shape of
the change has precedent, it is just missing at this site. Adding it is a divergence from
upstream `subflow.cc:2390` and therefore wants a **DIV row in `docs/history.md`**, but it
needs **no option gate**: it can only ever delete statements that the program does not
execute (a real program store is a STORE, or a COPY that is not marked `return_copy`), so
it strictly corrects wrong output.

Evidence that declining is a *complete* fix rather than a partial one: with `strings off`
the same three return-copies stay unsplit and print as **nothing** (step 3 above —
`mark_internal_copies` hides them), and the real copy site collapses to
`v11 = dat_822f0; v14 = dat_822f8;`.

Belt-and-braces companion, same function, same module (catches any other producer):
decline when the COPY's **output** is address-tied into a read-only range
(`Varnode::is_read_only`, already used at `p2_lift/jumptable.rs:1700,2155`). Note that
kuna already has a "read-only address is written" warning path
(`substrate/funcdata_varnode.rs:1826` in `fillin_read_only`, upstream
`Funcdata::fillinReadOnly`), so the invariant is one Ghidra already recognizes — it is just
gated behind `readonlypropagate`.

**Rejected alternatives.**

- *Round-1 fix #1* (constants in `build_in_subpieces` when the root is read-only): does not
  touch the phantom, see §3.
- *Round-1 fix #2 / `--option readonly on` default flip*: closes the case, but it is a
  global semantic policy (every read-only load becomes a constant), default-off in upstream
  Ghidra for RELRO / `.data.rel.ro` reasons, and `readonly` is still not in the catalog
  (registered in `p0_knowledge/options.rs` with no `settableTable` row) — an 83-row catalog
  addition plus a DIV, to fix 3 functions, when a one-predicate fix exists.
- *`Heritage::refine_write`*: not on the path at all.

**Risk.** Low and local. The only behavior change is that a `return_copy` COPY stays whole.
Regression surface to check in the PR: functions where a *writable* global array is
copied and the return-copy split was producing the assignment that made the global's final
value visible — none observed in 11,817 functions, but `make test` / `make test-stages` /
`make rust-test` are the gate, plus a `tests/stages/ghdec-returncopysplit.xml` asserting the
phantom block is absent (single-pass: no option to toggle).

## Loose threads (unchanged from round 1, still open)

- `$$undef00000004[0] = s_822f0[8];` — an internal placeholder name leaking into emitted C
  at the *real* copy site; 92 of the 94 `$$undef` occurrences in `ip` are untouched by this
  fix.
- One 16-byte stack buffer split across three symbols (`v11` + `$$undef00000004` + `v14`),
  P6 variable merging.
- `SUB81(...)` is a Ghidra pseudo-op with no C definition; every pane that contains one does
  not compile. Same family as `spacebase-unnamed-location`.
- `tar`'s `dat_6a7d6 += v18;` is a store into `.rodata` from a *different* mechanism —
  worth its own look.
- `kuna decompile-project <ip> --functions sub_20e10` panics in
  `kuna-analysis/src/loadimage_object.rs:716` (`range end index 973 out of range for slice
  of length 512`) — unrelated, found while trying to read the global's declaration.

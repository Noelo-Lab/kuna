## What was broken

RE-friction round 2, need `overlapping-anti-disassembly-sequence` (blocker, 1 instance,
challenge `5ab77f6333c5d40ad448ca52` — `pcme0`, i386 ELF, stripped). The tester's report:

> **Overlapping anti-disassembly sequence produces bogus out-of-image code and corrupted
> checker dataflow** — Kuna treated the intentionally overlapping bytes at 0x8048837 as a
> CALL, emitted a call to out-of-image `sub_b10cdc7`, writes through uninitialized pointers,
> a bogus `dat_d98ddd00` global, malformed split arguments, and an unreadable comparison.

The recursive password checker at `0x804881c` came back as:

```c
unsigned int sub_804881c(unsigned int a0,unsigned long long a1)
{
  v2 = (char *)sub_b10cdc7();
  *v4 = *v4 | (unsigned char)v3;
  *v2 = *v2 + (char)v2;
  dat_d98ddd00 += (char)v2;
  if ((v3 ^ (unsigned int)v2) != (int)*(char *)(*(char *)(a1._1_4_ + 0x804a2c6) + 0x804a2bc)) {
```

Every one of those artefacts is operand bytes read as opcodes. The binary carries the classic
overlap:

```
08048835  75 01              JNZ  0x8048838
08048837  e8 8b 45 0c 03     CALL 0x0b10cdc7      ; junk `e8` lead byte, target not in the image
```

The `JNZ`'s target sits one byte inside its own fall-through instruction. Decoding from
`0x8048838` instead gives the real stream — `MOV EAX,[EBP+0xc]; ADD EAX,[EBP+0x8];
MOVSX EDX,[EAX]` — which carries the input-byte XOR and the table comparison the whole
challenge turns on.

The round-2 refutation **upheld** the filed hypothesis by direct disassembly and named the
real risk one step past it: *"which stream owns the overlapped bytes, and what happens to
flow and dataflow already built from the discarded one. A builder must answer that
explicitly (and gate it), or the fix trades bogus out-of-image code for a silently
truncated function."* This PR answers it.

## Mechanism — `option overlapbranch`, default on (DIV-106)

P2, `flow.rs (FlowInfo::process_instruction)` + `p2_lift/kuna_overlapbranch.rs`.

A conditional branch pushes its target and then its fall-through, and `addrlist` is a stack,
so **the fall-through is always decoded first and wins by arrival order**. `set_fallthru_bound`
does detect the clash — it calls `reinterpreted` — but only on the next visit, when the
losing stream and everything downstream of it are already built. That is why the upstream
detection has never been able to do anything but record the fact.

The check is added at the one point where it is still cheap: right after the fall-through
instruction is decoded, against a target that is **still pending**. It fires iff the
instruction just decoded is the fall-through of the previous instruction's conditional
branch and that branch's own target lies strictly inside its encoding
(`curaddr < target < curaddr + step`). Both ends are strict: `target == curaddr` is a branch
to its own fall-through and `target == curaddr + step` is a branch over one instruction, and
both are ordinary compiler output.

**The one legitimate overlap is excluded first.** An overlap alone does not mean one decode
is junk. glibc's conditional-`LOCK` idiom is *compiler-generated* and does exactly this — a
`74 01` `JE` hopping over an `f0` prefix byte, so the fall-through is `LOCK OR [rdi+4],1` and
the branch target is the same `OR` without the prefix. Both streams are real and they
**reconverge**: the two decodes end at the same address. `kuna_streams_reconverge` decodes the
branch target's own length and declines whenever `target + target_len == curaddr + step` (and
whenever the target does not decode at all). It is architecture-neutral — no prefix-byte
table — and it runs only after the strict-interior test has already matched, which it never
does in ordinary code. **This clause was ablated, not argued** — see *Refutation* below.

**The ownership policy, stated:**

* **The branch target wins.** A branch target is an address the program *encodes* — it says,
  in the branch itself, that execution resumes there on that edge. A fall-through is only
  ever *inferred* from the previous instruction's length. When the two disagree, the encoded
  claim is the better evidence.
* **The trigger is self-certifying.** Two real instruction starts cannot sit at `next` and
  strictly inside `next`'s own extent, so whenever it fires at least one of the two decodes
  is already wrong; keeping the explicitly encoded one cannot make the answer worse.
* **The loser is truncated in place, never re-anchored.** The ops that decode just emitted
  are dropped (`delete_remaining_ops`), an artificial `RETURN` marked `badinstruction` is
  planted at the loser's own address, its recorded size is set to 1, and an `overlapbranch`
  warning makes the truncation attributable. The conditional stays a conditional and its
  fall-through **edge** stays in the graph — the edge simply ends in a halt — and the target
  is then decoded on its own boundary by the ordinary walk.
* **Nothing already committed to is deleted.** The loser is the instruction *currently* being
  decoded and the winner is still pending, so no op that other flow depends on can be
  orphaned and no instruction boundary already committed to is moved. That is what keeps this
  out of the "repair the flow graph afterwards" class of change the triage was worried about.
* `badinstruction` and not `noreturn` was bought by measurement: a `noreturn` halt is folded
  by `kuna_ifnoexit` into an empty `if (cond) { }`, which reads as though the fall-through
  does nothing and carries on — the one rendering that is actually wrong about where control
  goes. Both were built and compared.

## The acceptance probe now passes

```
python -m scripts.repipe.verify --need overlapping-anti-disassembly-sequence --json
  -> acceptance a-52c2ad89b522: PASS   (exit_code eq 0, stdout_absent sub_[0-9a-f]{7}\(\),
                                        stdout_absent dat_[0-9a-f]{8} \+=)
```

```c
unsigned int sub_804881c(char *a0,int a1)
{
  if (*a0 == '\xff')
    return; // warn: overlapbranch: this instruction overlaps the branch target at 0x08048838;
            //       truncating the fall-through here
  if (((int)a0[a1] ^ dat_804a2e8 + 0x3aU) != (int)*(char *)(*(char *)(a1 + 0x804a2c6) + 0x804a2bc)) {
    v2 = ((dat_804a2d0 != a1) || (a0[a1]));
  } else {
    v1 = a1 + 1;
    v2 = sub_804881c(a0,v1);
  }
  return v2;
}
```

Two ordinary arguments, the real instruction stream from `0x8048838`, the input-byte XOR
against the table, and the recursive descent — which is exactly what the need's symptom
asked for. No `sub_b10cdc7`, no `dat_d98ddd00`.

## Regression cover

* `tests/stages/overlapbranch.xml` — two-pass (default recovers the real stream;
  `option overlapbranch off` reproduces **both** artefacts, `sub_b0c48ac()` and
  `dat_d98ddd00`), 3 assertions, all pinned `min=1 max=1` so each is present in exactly one
  of the two passes.
* `tests/cli/overlapping-anti-disassembly-sequence.json` — the acceptance probe promoted.
  Its clauses are verbatim; only the target is re-pointed, because the need's own target is
  a dataset crackme and CI has no dataset. It runs against a vendored 2 KiB ELF32 fixture,
  `decompiler/crates/kuna-analysis/tests/fixtures/overlapbranch_i386` (generator
  `overlapbranch_i386.py` beside it), which carries the same idiom at the same address and
  reproduces both forbidden patterns with the option off.
* 11 unit tests on the two decision predicates (`kuna_overlapbranch/tests.rs`): the witness,
  the gate, every ordinary forward/backward branch shape that must not fire, and the four
  reconvergence cases (glibc's `LOCK` prefix, the witness's non-reconvergent overlap, an
  undecodable target, and a target that outruns the fall-through).
* `decompiler/crates/kuna-cli/tests/overlapbranch_cli.rs` — an end-to-end two-pass gate that
  runs the promoted probe's clauses verbatim under `make rust-test` over the vendored
  fixture, plus an `option overlapbranch off` pass proving the clauses are discriminating
  (`tests/cli/` is executed by `scripts.repipe.clitests`, which is not a CI job).

## Gates

| Gate | Result |
|---|---|
| `make test` | **PARITY OK — 675/675**, `docs/baseline.json` unmoved |
| `make test-stages` | **PARITY OK — 603/603** (600 + 3 new; baseline re-recorded) |
| `make rust-test` | **green — 5,330 passed, 0 failed** (344 suites) |
| `make check-spec` | OK (also `--strict` OK) |
| `kuna catalog --check` | catalog OK (136 → 137 settables) |
| acceptance `a-52c2ad89b522` | **PASS** |
| `python -m scripts.repipe.clitests` | **18/18** (includes the newly promoted probe) |

**Speed.** −0.22% on a target where the option is provably inert (`mv` -O2 `main`, output
byte-identical on both arms) and −1.71% on the witness (`scripts.pipeline.timeit`, median of
11, 5% budget). Because a single-target `timeit` on this box is contention-sensitive, the
neutral target was also measured interleaved A/B/A, min-of-9: **ON vs OFF +0.00%** on the
minimum, and the on-vs-on control arm shows the *identical* median shift — the difference is
noise. The hook is an `Option::take` and two integer comparisons per instruction; the extra
decode of the branch target happens only once the strict-interior test has matched.

## Corpus sweep (standing requirement 7)

**428 binaries A/B'd whole-binary (`decompile-all`, every function)** — 142 compiler-built
decbench ELFs (x86-64 O0+O2, ARM, Cortex-M firmware) and 286 crackmes-corpus executables
(ELF32/64, PE32/PE32+, Mach-O, MS-DOS).

**14 differ, and all 14 are hand-written or obfuscated crackmes. 0 of the 142 compiler-built
binaries change a byte.** 78 functions move; the emitted function **count is identical on both
arms (7,799)**, so nothing leaves the inventory. Every changed function classified:

| Class | Count |
|---|---|
| hard `(error: …)` → real code | 4 |
| mixed add+remove inside decodes that are garbage on both arms | 71 |
| only two never-read variable declarations dropped | 2 |
| rendering-only (identical statement multiset) | 1 |

Aggregate `(error:` count over the differing binaries falls **387 → 383**. 33 of the 71 are
`crypto4.exe`'s PE IAT thunk region and 29 are `CrackMe.obf.exe` — regions that are data
decoded as code either way, where ON is simply shorter and terminates.

Two gains are concrete and independent of the witness:

* **`nano`** (an anti-debugging ptrace crackme, x86-64): the always-taken `jz +3 / jnz +1 / e8`
  pair stops swallowing the argument setup, and `main`'s ptrace calls recover their request
  codes — `func_00101189(0, …)`, `func_00101189(7, …)`, `sub_…(0xd, …)` — where the off arm
  printed every one of them with **no arguments at all**. No `overlapbranch` warning appears,
  because the truncated edge is unreachable (the `jz`/`jnz` pair is always taken), so the halt
  block is collected and only the recovered stream remains.
* **`poof.exe`** (PE x86-32): a bogus `*v4 = *v4 ^ (unsigned int)v4;` store through a junk
  pointer disappears and the following test reads the real global `dat_40311f`.

## Refutation — the wrong-output risk was ablated, not argued

The triage said the danger is the ownership policy, not the detection. The single clause that
carries it is the reconvergence exclusion, so it was measured directly.

**It is reached on ordinary compiler output.** Instrumented, a statically linked glibc binary's
`je +1` over an `f0` `LOCK` prefix hits the strict-interior test and is declined:

```
[OB] interior   curaddr=0x401004 step=5 target=0x401005            -> true
[OB] reconverge curaddr=0x401004 step=5 target=0x401005 tlen=4     -> true   (declined)
```

**Ablated — `kuna_streams_reconverge` forced to `false`, nothing else changed:**

```c
/* shipped */                             /* exclusion ablated */
if (!a1)                                  if (a1)
  *(uint *)(a0 + 4) |= 1;                   return;  // overlapbranch truncation
LOCK();                                   *(uint *)(a0 + 4) |= 1;
*(uint *)(a0 + 4) |= 1;                   return *(uint *)(a0 + 4);
UNLOCK();
return *(uint *)(a0 + 4);
```

The atomic store is **deleted on a live path**, and glibc's `_int_free` moves 487 lines.
As shipped, `_int_free`, `_int_malloc`, `arena_get2.part.0` and `malloc_consolidate` are all
byte-identical with the option on and off, and 11 such sites exist in that one binary.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

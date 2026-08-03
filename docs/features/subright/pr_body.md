> Rebased onto `main` @d89a4d95. #248 (`cortexmvectors`) and #249 (`funcptralign`) both
> change ARM output, so the breadth numbers below were **re-measured from scratch** against
> that base with a fresh stubbed/ported build pair, and the ablation was re-run. #250
> (`paramcopyhoist`) landed after that; it is a default-OFF P6 pass and the interaction check
> below shows it does not touch this rewrite. The witness output and both stage tests were
> re-verified on the final base.

## The bug

`RuleSubRight::apply_op` (`p3_dataflow/ruleaction_6.rs`, upstream `ruleaction.cc:7271`) was
a **port stub**. Only the leading `doesSpecialPrinting` / `isPieceStructured` half was
transcribed; the body then read

```rust
        // Remainder transcribed for the next wave; unreachable at this merge base:
        //   PcodeOp *lone = outvn->loneDescend();  ... lump a lone right-shift ...
        // W6
        0
```

so upstream's `sub(V,c) => sub(V>>c*8,0)` rewrite never ran and **every SUBPIECE with a
nonzero byte offset reached the printer**, which renders one as the raw p-code operator.
`SUB41` / `SUB81` / `SUB84` are **undeclared identifiers** — kuna emits no header that
defines them — so those function bodies were not compilable C.

openssh `ssh` (O2-noinline, stripped) `sshbuf_put_u64` @ `0x4cd60`, `--mode reliable`:

```c
/* before */                              /* after */
unsigned long sub_4cd60(unsigned long a0,unsigned long a1) // early-return
{                                         {
  ...                                       ...
  *v2 = SUB81(a1,7);                        *v2 = (char)((uint8)a1 >> 0x38);
  v2[1] = SUB81(a1,6);                      v2[1] = (char)((uint8)a1 >> 0x30);
  v2[2] = SUB81(a1,5);                      v2[2] = (char)((uint8)a1 >> 0x28);
  v2[3] = SUB81(a1,4);                      v2[3] = (char)((uint8)a1 >> 0x20);
  v2[4] = SUB81(a1,3);                      v2[4] = (char)((uint8)a1 >> 0x18);
  v2[5] = SUB81(a1,2);                      v2[5] = (char)((uint8)a1 >> 0x10);
  v2[6] = SUB81(a1,1);                      v2[6] = (char)((uint8)a1 >> 8);
  v2[7] = (char)a1;                         v2[7] = (char)a1;
  return 0;                                 return 0;
}                                         }
```

Upstream Ghidra on the same function emits `*local_18 = (char)((ulong)param_2 >> 0x38);`,
and the source (`sshbuf-getput-basic.i`) is a plain shift:
`((u_char *)(p))[0] = (__v >> 56) & 0xff;`. The last line of kuna's own *before* pane is
the proof that the printer was never at fault — the offset-0 SUBPIECE already rendered
`(char)a1`.

## The fix

The ~45-line tail is now transcribed from `ruleaction.cc:7291-7340`
(`GHIDRA_REV=cef869af`), **with every guard upstream applies**, because each one protects a
rendering kuna already gets right:

- `c == 0` — already least-significant; it prints as a cast.
- output and input both address-tied with `overlap == c` — the SUBPIECE is the storage
  marker `ActionCopyMarker` converts; rewriting it would turn stack-piece writes back into
  shift chains.
- the `isPieceStructured` special-print branch (already ported) still runs first, so
  struct/union/array field extraction is untouched.

Then the rewrite: lump a lone `INT_RIGHT`/`INT_SRIGHT` descendant with a constant shift when
`outvn->getSize() + c == a->getSize()` (declining an `INT_RIGHT` whose combined amount would
evaluate to zero, clamping `INT_SRIGHT` to the sign bit), synthesize the shift ahead of the
SUBPIECE typed `TYPE_UINT`/`TYPE_INT` at the input width, and zero the SUBPIECE's offset.

Nothing needed wiring: the rule was already registered in upstream's own schedule position
(`rrow!("subright", "cleanup", ...)`, `infra/universalaction.rs:552`).

One kuna-side deviation, marked `// (kuna)`: the type-factory availability check is hoisted
above the first mutation, so a hand-built `Funcdata` with no type factory (the rule-level
unit-test fixtures) cannot leave a half-applied transform behind. On a real architecture the
factory is always present.

## No option — and why

`CLAUDE.md` gates *features* ("behavior that is a judgment call, not universally better") and
exempts "a strict bug fix that only corrects wrong output". This is the exempt case:

1. **The old output is not C.** There is no honest `use_when` prose for a `subright` option —
   nobody wants the non-compiling spelling.
2. **It restores upstream, it does not diverge from it.** This is finishing a port; the
   output shape is byte-identical to Ghidra's on the witness.
3. **The ablation is silent** — 0 of 675 vendored-from-upstream datatest assertions move,
   exactly what "restoring upstream behavior" predicts.
4. **The switch already exists.** `subright` is a registered rule-group name, so
   `option togglerule subright off` already gives a bisector the pre-fix rendering. A new
   `settableTable` row would add catalog-count churn and a knob nobody should flip.

An independent decider subagent was asked to make this call from the repo's own rules and
precedent and returned `ungated`; its verdict is recorded verbatim in
`docs/features/subright/record.json`. Precedent is unanimous for this shape — #226 (DIV-46),
#232 (DIV-48), #235 (DIV-49), #238 (DIV-50), #242 (DIV-52) are all `fix(pN):` corrections
that change emitted C with no settable option.

Recorded in `docs/history.md` under **Convergences**, not as a DIV row — the registry's own
rule is that "a port defect whose fix moves kuna *back onto* upstream's default earns no DIV
row, but a corpus-wide output shift still needs a record for anyone bisecting."

## Breadth

Triage's corpus-wide count (matched per-function comparison over 788 binaries / 76,046
functions both tools recovered): nonzero-offset SUBPIECE **kuna 5,919 in 2,394 functions
(3.1%) vs ghidra 18 in 13 (0.017%) vs ida 0** — a 329x ratio.

Measured directly for this PR, two release builds of this worktree (stubbed vs ported),
7 whole-binary `decompile-all --mode reliable` runs, x86-64 -O0/-O2 and ARM Cortex-M.
All totals are over the function set **common to both panes**, so the 120 s per-function
watchdog cannot move them on this (loaded) box:

| binary | raw `SUBnn` (offset != 0) | functions carrying one | functions whose text changed |
|---|---|---|---|
| O2 openssh `ssh` | 69 → **0** | 29 → 0 | 29 / 1,853 (1.57%) |
| O2 iproute2 `ip` | 80 → 43 | 34 → 3 | 32 / 1,912 (1.67%) |
| O2 e2fsprogs `e2fsck` | 48 → 2 | 38 → 2 | 37 / 1,909 (1.94%) |
| O0 `gzip` | 22 → **0** | 6 → 0 | 6 / 262 (2.29%) |
| O2 `bzip2` | 13 → **0** | 9 → 0 | 10 / 153 (6.54%) |
| O2 betaflight STM32F405 (ARM) | 1,068 → 31 | 409 → 10 | 407 / 5,795 (7.02%) |
| O2 cleanflight DALRCF405 (ARM) | 612 → 19 | 208 → 5 | 207 / 2,836 (7.30%) |
| **total** | **1,912 → 95 (95.0% removed)** | **733 → 20** | **728 / 14,720 (4.95%)** |

Emitted lines over the same set: 443,737 → 443,785 (**+0.011%**).

**Every one of the 95 residuals is a case upstream's own guards decline** — 42 address-tied
global-array stores (`s_92e50[7] = SUB81(s_92e50[0],7);`, round 1's separately-filed
`rodata-phantom-store`), 47 address-tied local partial-symbol writes (`v19 = SUB42(v14,2);`,
the P6 thread that renders `sym._2_2_` upstream), and 6 piece-structured field extractions on
the special-print branch. `subright` owns 1,817 of the 1,912.

**Measured cost:** exactly two functions in 14,720 gain a dead partial-register shift
statement — `v5 = (undefined7)((uint8)a2 >> 8);` in bzip2 `sub_da00` and one
`(undefined3)((uint4)v11 >> 8)` on ARM. That is the `SUB87`/`SUB43` partial-register form,
which upstream lowers the same way. Corpus-wide count of that shape: 0 → 2.

## Test

`tests/stages/ghdec-subright.xml`, two-pass over a real `binary -> decompile` path, on a
44-byte i386 bytechunk built from

```c
void put_u32_be(u8 *p, u32 v) { p[0] = (v >> 24) & 0xff; ... }   /* the plain rewrite */
u32  mulhi_shift(u32 a, u32 b) { return (u32)(((u64)a * b) >> 32) >> 3; }  /* the lumping */
```

`put_u32_be` covers the plain rewrite and the `c == 0` bail; `mulhi_shift` covers the
lone-descendant lumping — the `mull` high half is `SUBPIECE(prod,4)` read only by a constant
`INT_RIGHT`, folded into one shift by `32+3 == 0x23` — and uses a *variable* multiplier so
`RuleDivOpt` declines and the truncation actually reaches the printer. Pass 1 is the default
(the fix); pass 2 sets `option togglerule subright off` and pins the pre-fix rendering.

**1 of 8 assertions passes pre-fix, 8/8 post-fix.**

Two new rule-level unit tests pin the guards that must not be dropped
(`sub_right_least_significant_bails`, `sub_right_addr_tied_marker_bails`).

## Benchmark

Both cluster cases that are in the mined pool, rescored against the current build:

| case | GED before | after | delta |
|---|---|---|---|
| `O2-noinline-iproute2-ip-netns_add` | 8.0 | 8.0 | 0.0 |
| `O2-noinline-betaflight-betaflight_STM32F405-applyLedFixedLayers` | 25.0 | 25.0 | 0.0 |

**GED-neutral by construction**, and exactly as the triage predicted: the rewrite changes
expressions inside basic blocks and moves no block, so the graph-edit-distance metric cannot
see it. This case is ranked by correctness (undeclared identifier → valid C), not by GED.

## Speed

Interleaved A/B on the two builds, medians (interleaved because this box runs sibling agents
and a decbench sweep, so a blocked measurement would fold load drift into the delta):

| target | before | after | delta |
|---|---|---|---|
| openssh `ssh` @0x4cd60 (the witness) | 2,307.2 ms | 2,165.0 ms | **-6.16%** (9 pairs) |
| gzip -O0 `decompile-all` | 3,387.2 ms | 3,325.0 ms | **-1.84%** (7 pairs) |
| bzip2 -O2 `decompile-all` | 12,016.1 ms | 11,511.8 ms | **-4.20%** (7 pairs) |
| e2fsprogs `e2fsck` -O2 `decompile-all` | 21,378.4 ms | 21,430.2 ms | **+0.24%** (5 pairs) |

Worst case **+0.24%**, inside the +5% budget; three of four arms are faster (one op more in
the cleanup pool, but a normalized shift that the downstream pools fold instead of carrying).

## Gates

All re-run on the rebased base (`main` @d89a4d95):

```
make test        675/675 assertions passed -- PARITY OK   (docs/baseline.json NOT re-pinned)
make test-stages 369/369 assertions passed -- PARITY OK   (baseline re-recorded: purely additive,
                                                           361 -> 369 keys, no existing assertion moved)
make rust-test   green -- 4424 passed; 0 failed; 37 ignored (cargo exit 0)
make check-spec  check-spec OK (lenient mode) / check-spec OK (strict mode)
kuna catalog --check   catalog OK: documents exactly the registered kuna options
```

Corpus file count in `decompiler/crates/kuna-base/src/xml.rs`: **186 -> 187** (base + #249's
`ghdec-funcptralign` + #250's `ghdec-paramcopyhoist` + this PR's `ghdec-subright`; 83 datatests
+ 104 stages).

**No catalog surface is touched** — the diff against `main` for `phases.toml`,
`src/p0_knowledge/`, `decompiler/crates/kuna-decomp/tests/` and `docs/options.md` is empty, and
the settable-row count stays at #250's 85.

### Interaction with #250

`ActionParamCopyHoist` is a default-OFF P6 pass; this is a P3 cleanup rule. Confirmed rather
than assumed — whole-binary `decompile-all --mode reliable` with `--option paramcopyhoist on`
vs default gives **identical** results on openssh `ssh`, gzip -O0 and bzip2 -O2: residual raw
operators 0/0 in each, subright output shapes 23→23, 12→12, 1→1, and the witness function at
`0x4cd60` byte-identical either way. Both stage tests pass explicitly
(`ghdec-paramcopyhoist` 3/3, `ghdec-subright` 8/8).

Bundle: `docs/features/subright/` (`analysis.md`, `plan.md`, `record.json`).
Spec prose: `docs/spec/03-ssa-and-simplification.md`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

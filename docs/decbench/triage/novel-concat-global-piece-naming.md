---
case_id: novel-concat-global-piece-naming
pool: novel
group_id: cluster (bash::yyparse, bash::save_tty_chars, iproute2::print_ndtparams, iproute2::print_namespace, openssh-portable::client_suspend_self, openssh-portable::main, openssh-portable::sshkey_dump_ec_key, tar::xclose, betaflight::decodeEscFrame)
status: feature-candidate
tier: N
margin: 0
fresh_verdict: the CONCAT is INHERITED — ghidra emits the byte-identical `CONCAT44(<hi-piece>,1)` on the same statement of bash/yyparse — and the only kuna-specific residue is the LEAF NAME of the high piece; one half of that residue (a RAM global spelled `dat_<addr+k>` where ghidra says `DAT_<addr>._k_n_`) is a symbol-map difference with no kuna-side mechanism, the other half is a real one-predicate printer bug that spells an UNNAMED REGISTER piece as a fabricated memory global `dat_<register-offset>`
option_closing: null
feature_slug: regpiecename
scope: small
confidence: high
---

## Verdict in one line

The `concat` wart in `novel.md` is **not a CONCAT defect** — kuna 513 vs ghidra 449 on
10,068 matched functions (1.14x), and on the row that named it ghidra emits the *identical*
operator. What survives is a **leaf-naming** defect, and the kuna-only half of it is
`kuna_global_naming()` accepting the **register space** — so an unnamed sub-register piece
prints as `dat_4`, a RAM global at address 4 that does not exist.

## Side-by-side

### A. the assigned symptom — `dat_172790 = CONCAT44(dat_172794,1);` (bash `yyparse` @ 0x35806)

Fresh, HEAD `e38ffc31`, `make binaries` run at the start of this triage,
`SLEIGHHOME=/home/mahaloz/github/kuna/specs`.

> Tree provenance: another agent held five `p6_variables/` + `p9_emit/` files dirty in this
> worktree for the whole session (`coreaction_cleanup.rs`, `funcdata_merge.rs`, `merge.rs`,
> `varmap.rs`, `printc.rs`) and rebuilt `target/release/kuna` mid-run. Every hunk is an
> `eprintln!` behind `KUNA_DBG_{DECL,MERGE,NAME,TIED,VARMAP}` or an ablation behind
> `KUNA_ABL_NARROWER` / `KUNA_EXP_NOCONFLICT`, all of which default to the HEAD behavior and
> none of which were set here — the two headline witnesses were re-run last with every one of
> those variables explicitly unset and are byte-identical to the transcripts below.

```
$ kuna decompile .../O0/bash/stripped/bash --addr 0x35806        # --mode auto == reliable here (bash is 1.44 MiB)
        dat_172790 = CONCAT44(dat_172794,1);
        dat_172790 = CONCAT44(dat_172794,(int4)v25[-4]);
        ...  (28 occurrences)
        dat_172798 = CONCAT44(dat_17279c,(int4)*v25);
```

Ghidra, stored pane, **same function, same statements** (ghidra's image base is 0x100000,
so `DAT_00272790` == kuna's `dat_172790`):

```c
      DAT_00272790 = CONCAT44(DAT_00272790._4_4_,1);
      DAT_00272798 = *local_eb8;
      local_e58 = make_redirection(DAT_00272790,0,DAT_00272798,0);
      ...
      DAT_00272790 = CONCAT44(DAT_00272790._4_4_,(int)local_eb8[-4]);
```

Same operator, same nesting, same operand order. The **only** difference is that ghidra
names the high half as a member of the containing 8-byte symbol and kuna names it by its
own address. kuna already emits the member form for *stack* symbols in the very same
function — `CONCAT44(v26._4_4_,10)`, 60 such in the matched set — so the printer path
exists; what is missing is a sized global data symbol to be partial *of*.

### B. the kuna-only residue — `CONCAT44(dat_4,v1)` (tar `xclose` @ 0xd2c0)

```
$ kuna decompile .../O2-noinline/tar/stripped/tar --addr 0xd2c0
void sub_d2c0(void)
{
  unsigned int v1; // eax

  if (!close())
    return;
  dcgettext(0,"(pipe)",5);
  sub_32680(CONCAT44(dat_4,v1)); // tail-call
  return;
}
```

ghidra / ida on the same address:

```c
/* ghidra */  uVar1 = dcgettext(0,"(pipe)",5);   FUN_00132680(uVar1);
/* ida    */  v2   = dcgettext(0, "(pipe)", 5);  return sub_32680(v2);
```

And where ghidra *does* hit the same split (iproute2 `ip` `print_namespace` @ 0x67900) it
names the pieces instead of inventing memory:

```c
/* kuna   */  sub_67870(CONCAT44(dat_4,CONCAT22(dat_2,v3)));
/* ghidra */  FUN_00167870(CONCAT44(extraout_var_00,CONCAT22(extraout_var,extraout_AX)));
/* ida    */  v6 = sub_67540();  sub_67870(v6);
```

## Source

`~/github/decbench/results/full_run/O2-noinline/tar/compiled/buffer.i`:

```c
xclose (int fd)
{
  if (close (fd) != 0)
    close_error (dcgettext (NULL, "(pipe)", 5));
}
```

`dcgettext` returns `char *` — one 8-byte value in RAX. There is no memory object in this
program, so `dat_4` is not a mis-typed global, it is a **fabricated** one.

`~/github/decbench/results/full_run/O0/bash/compiled/rltty.i` (case A's sibling
`save_tty_chars`) is `_rl_last_tty_chars = _rl_tty_chars;` — a 16-byte struct of 16
`unsigned char` fields, which the O0 codegen lowers to two 8-byte loads. kuna's
`CONCAT17(dat_17bed7,CONCAT16(...))` chain is a semantically exact reassembly of that copy;
Heritage refinement split the 8-byte read because the function also writes each byte
individually (the CONCAT ops carry the load's own address `0x000ff7aa` and late seqnums
`0x127..0x12d` in `print raw`). Ghidra has no pane for that address in this run, but the
mechanism is upstream `Heritage::refinement` + `concatPieces`, so it is Ghidra-family
behavior, not a kuna invention. IDA models the 8-byte object as primary instead
(`qword_17BEE0 = qword_17BED0;` + `LOBYTE(qword_17BED0) = ...`), which is why the pane
reads better there.

## Analysis

### The mechanism (instrumented, not read)

`decomp_dbg` → `load addr 0xd2c0` → `decompile` → `print raw` on tar:

```
0x0000d2ee:50:	EAX(0x0000d2ee:50)            = [create] i0x0000d2ee:31(free)
0x0000d2ee:53:	%0x00000004:4(0x0000d2ee:53)  = [create] i0x0000d2ee:31(free)
0x0000d2ee:31:	call fdcgettext(free)(#0x0,u0x10000043(0x0000d2ee:7f),#0x5)
0x0000d2f7:4e:	u0x1000001b(0x0000d2f7:4e)    = CONCAT44(%0x00000004:4(...),EAX(...))
```

`%0x00000004:4` is a **register-space** varnode (kuna's raw printer gives the register space
the `%` shortcut — `kuna-base/src/space.rs:2586-2593`, `IPTR_PROCESSOR` + `get_name() ==
"register"`), size 4 at register offset 4, i.e. the top half of RAX. It prints as `dat_4`.

The printer path:

- `p9_emit/printc.rs:8451` `kuna_unnamed_location_name()` — `get_register_name(spc, off,
  size)` returns empty (x86-64 has no named 4-byte register at register offset 4), so it
  falls to the next arm;
- `p9_emit/printc.rs:8413` `kuna_global_naming(spc)` is
  `matches!(spc.get_type(), spacetype::IPTR_PROCESSOR)` — and in the Ghidra space model the
  **register space is `IPTR_PROCESSOR` too**, exactly as `assign_shortcut` above proves;
- so it returns `kuna_global_data_name(spc, 4)` = `"dat_4"` instead of falling through to
  kuna's own `Space<hex>` tail (`Register00000004`).

The intent was already to exclude registers — the comment at `printc.rs:6944` says "a
**non-register** `IPTR_PROCESSOR` address that renders `dat_<addr>`" — but the predicate only
excludes *named* registers, via the `get_register_name(...).is_empty()` companion check.
Unnamed sub-register pieces walk straight through the hole. `namestyle ghidra` does not gate
this arm either, so the fabricated token survives both naming policies.

Upstream `PrintC::pushUnnamedLocation` (printc.cc:1957-1974) has no such arm at all — it
prints `space->printRaw(offset)`, and Ghidra additionally gives an indirect-creation output
the `extraout_<REG>` / `extraout_var` recommendation (which kuna *already implements*, at
`p0_knowledge/database.rs:3444` — the varnode here simply never reaches
`Scope::buildVariableName`, which is the second, separate half noted under *Risks*).

### Downstream harm, measured

`kuna decompile-project` collects every `dat_<hex>` token out of the emitted C
(`kuna-console/src/project.rs:278` `collect_dat_addrs`) and labels it in the `.asm` data
tail. On tar it therefore emits a data object that is really the ELF header:

```
$ kuna decompile-project .../O2-noinline/tar/stripped/tar -o ptar --addr 0xd2c0
$ grep -A2 'dat_4:' ptar/tar.asm
dat_4:  ; 0x4
  00000004: 02 01 01 00 00 00 00 00 00 00 00 00 03 00 3e 00  |..............>.|
```

### Owning phase

**P9 — Surface Rendering & Refinement** (`docs/phases.md:32,58`; `p9_emit/`, spec chapter
`docs/spec/09-emission.md`). The symptom is a leaf token; the decision is a printer
predicate, and it is not destroyed in Band B.

## Breadth (corpus-wide, measured)

Scanner is nesting-aware (balanced-paren argument split, `scan.py` from the round-3 census);
"fresh" is `kuna decompile-all --mode auto` on HEAD over 15 stripped binaries covering
5 arch/format families.

**CONCAT is not a regression.** Matched set = functions present in the fresh run *and* the
stored kuna pane *and* the stored ghidra pane (**10,068 functions**):

| CONCAT high-argument shape | kuna (fresh, HEAD) | ghidra (stored) |
|---|---:|---:|
| bare global leaf (`dat_<hex>` / `DAT_<hex>`) | 99 | 20 |
| partial-symbol member (`sym._k_n_`) | 60 | 110 |
| nested CONCAT | 1 | 3 |
| other | 353 | 316 |
| **total** | **513** | **449** |

ghidra's 20 "global leaf" rows are ARM literal-pool derefs (`*(undefined1 *)(DAT_x + i)`), a
different construct. The 99-vs-110 pair is the same expressions with different spellings.

**The fabricated-register-global slice.** Counting `dat_<off>` tokens whose offset is
covered by no allocated section of the image:

| population | occurrences | functions | denominator |
|---|---:|---:|---|
| decbench-scored functions only (fresh ∩ stored kuna pane) | 60 | 28 | 13,775 |
| x86-64 ELF binaries, all fresh functions | 94 | 46 | 11,399 |
| whole fresh corpus, all functions | 3,397 | 480 | 24,372 |

Use the first two. The 3,397 figure is **not** this defect's breadth: 3,294 of it is
betaflight/cleanflight/crazyflie functions that `decompile-all` produced for misdecoded
non-function regions (ARM-mode decode of Thumb data — e.g. `sub_807ae04` reads
uninitialized `v7` and stores through `dat_ffffffe6`), which decbench never scored. That is
a separate recall/discovery artifact, not a naming bug.

Space verified per offset with `print raw`, not inferred:

| offset | space | witness |
|---|---|---|
| `0x2`, `0x4` | **register** (`%0x00000002:2`, `%0x00000004:4`) | ip `print_namespace` 0x67900 |
| `0x4` | **register** (`%0x00000004:4`) | tar `xclose` 0xd2c0, ssh `client_suspend_self` 0x1e830 |
| `0x3` | **register** (`%0x00000003:1`) | tar 0xc290 |
| `0x0` | ram (`r0x00000000:2`) | e2fsck 0x3c010 — a null-pointer read, a DIFFERENT bug |
| `0xa4`, `0xa8`, `0xb0` | ram (`r0x000000a4:4`, …) | e2fsck 0x55ee0 — also a different bug |

So on the x86-64 subset, **80 of the 94** are register-space (offsets 2/3/4) and 14 are
genuine-but-nonsensical RAM addresses. **76 of the 94 (81%) sit in the high argument of a
CONCAT** — i.e. the majority of the `concat` wart on x86-64 in this pool is this naming bug
wearing a CONCAT costume. 3 are assignment targets, and all 3 are the RAM `dat_0` family,
not the register family.

## Option / mode sweep (all negative)

```
$ for m in auto reliable aggressive fast; do kuna decompile tar --addr 0xd2c0 --mode $m; done
   -> all four: sub_32680(CONCAT44(dat_4,v1));
$ for o in "condfold on" "condfold wide" "cortexmvectors on" "ptrentry on" \
           "paramcopyhoist on" "dwarf_lines on" "namestyle ghidra" "returnpair on"; do ... done
   -> all eight: 1 occurrence of dat_4
```

Same result on the case-A witness (`bash save_tty_chars` @ 0xff79e keeps its 2 CONCAT17
chains under all four modes and all six non-aggressive options). No row of the 88-option
catalog mentions the unnamed-location naming policy. `option_closing: null`.

## Metric-artifact check

No metric claim is being made: the fix changes leaf tokens only, so **GED delta is 0** by
construction (no basic-block, edge or statement count moves). This case must be ranked on
correctness, not on the benchmark — as with round 1's `spacebase-unnamed-location`. Two of
the witnesses would in any case be unusable as GED evidence: `bash yyparse` has a 1,142-line
ghidra pane and an approximated large graph, and `tar xclose` has a 2-node source CFG.

## What is DISPROVEN

1. **`CONCAT44(dat_<hi>,<lo>)` is not kuna-only.** The census round that fed this track
   reported "93 of kuna's 627 matched CONCATs; ghidra 0 on the identical 13,775 functions".
   That scanner classified on the *spelling of the leaf*, so ghidra's identical operator was
   invisible to it: ghidra writes `DAT_00272790._4_4_` where kuna writes `dat_172794`. On the
   tightest matched set ghidra has **110** such CONCATs to kuna's 99. Nothing here is a
   CONCAT-generation defect, and there is no CONCAT fix to write.
2. **`save_tty_chars`' CONCAT17 chain is faithful, not soup.** The source is a 16-byte struct
   copy; the reassembly comes from upstream `Heritage::refinement` splitting the 8-byte load
   against the eight 1-byte writes, visible as late-seqnum ops at the load's own address.
   Fixing it would mean adopting IDA's whole-object model, which is not a Ghidra-family
   change and is not a one-module change.
3. **The `dat_<addr+k>` half has no kuna-side mechanism.** Ghidra prints `._4_4_` because its
   Program DB carries a *sized* `DAT_00272790` symbol created by auto-analysis; kuna has no
   global data-symbol map at all, so every global sub-access renders by its own address. That
   is an analysis-tier (P0/`kuna-analysis`) capability, not a printer change, and it is worth
   less than it looks — `dat_172794` is arguably the more honest spelling of the two, and IDA
   beats both by keeping the object whole.
4. The census's other two verdicts hold on HEAD. `subright` (#251): `ssh sub_4cd60` now emits
   `*v2 = (char)((uint8)a1 >> 0x38);` — ghidra's exact shape, zero raw `SUB81`. `undefname`
   (#257): zero `$$undef` in 24,372 freshly decompiled functions.

## Proposed fix

- **Owning phase / module: P9, `decompiler/crates/kuna-decomp/src/p9_emit/printc.rs`.** One
  predicate: `kuna_global_naming()` (printc.rs:8413) must accept only the architecture's
  default data space, not every `IPTR_PROCESSOR` space. `arch.manage().get_default_data_space()`
  already exists and is used across the tree (`p5_types/coreaction_infertypes.rs:512`,
  `p2_lift/jumptable.rs:3561`, …). All three call sites are in this file
  (printc.rs:6522, 6937, 8461) and all three already hold `arch`, so the signature change is
  local. With the predicate tightened, an unnamed register piece falls to kuna's existing
  `Space<hex>` tail and prints `Register00000004` — honest, and it stops the
  `decompile-project` data tail from labelling the ELF header.
- **Second half, same PR: give it a name instead of a raw location.** The varnode is an
  `indirect_creation` output of a CALL, and kuna already renders that as `extraout_<REG>` /
  `extraout_var` in `Scope::buildVariableName` (`p0_knowledge/database.rs:3444`) — it simply
  never reaches that function here, so no Symbol is bound and the printer falls to the
  unnamed tail. Confirming *why* needs an `eprintln!` in the P6 naming pass driven through
  the in-process `decompile-all` path; that instrumentation was out of scope for this record
  (no `.rs` edits), so treat this half as unproven. **If it does not reduce to a naming-pass
  gate, ship only the predicate fix and file the `extraout` half separately** — the predicate
  alone already removes the fabricated memory reference, which is the correctness claim.
- **Option**: this is a strict bug fix (the emitted token names an object that does not
  exist), so per `AGENTS.md` it needs no flag. If the `extraout_` half lands in the same PR
  it becomes a naming-policy change and should be gated.
- **Stage test**: `tests/stages/ghdec-regpiecename.xml` (no angr analog), two-pass: a call
  whose return width is under-declared so the caller reassembles the return register —
  before, `CONCAT44(dat_4,…)`; after, no `dat_` token at a register offset.
- **Risks to measure**: (a) `renders_as_global` at printc.rs:6944 currently treats an unnamed
  register piece as a *global instance*, so it can hijack the canonical storage of a whole
  mixed HighVariable (the `kuna-regglobal-render-bug` path) — tightening the predicate changes
  that resolution too, and the whole-corpus before/after sweep must cover every function whose
  text moves, not just the witness; (b) the datatest corpus — `dat_<hex>` appears in stored
  expectations, so run the 0/675 ablation before assuming this is invisible; (c) speed is
  unaffected (one extra space comparison per unnamed leaf), but measure with
  `scripts.pipeline.timeit` anyway; (d) **GED delta is 0** — do not let `rescore` be the gate.

## Siblings

Reproduce identically on HEAD, all `--mode auto` and `--mode reliable`:
`openssh-portable/ssh` `client_suspend_self` 0x1e830 (7), `main` 0xcdf0 (4),
`sshkey_dump_ec_key` 0x4b0b0 (4), `safe_path` 0x84e80 (3), `mac_setup` 0x7d540,
`channel_from_packet_id` 0x5b300, `config_has_permitted_cnames` 0x17470,
`crypto_hash_sha512` 0x982c0; `iproute2/ip` `print_ndtparams` 0x29a20 (9),
`print_namespace` 0x67900 (4), `xfrm_str_to_policy` 0x3aa40, `print_attrs` 0x5eec0,
`gtp_print_opt` 0x68c40; `tar` `xclose` 0xd2c0, `xfork` 0x24f80, `xpipe` 0x24fc0,
`bzr_addfn` 0x164d0, `uname_to_uid` 0x269f0, `gname_to_gid` 0x26aa0,
`page_aligned_alloc` 0x25000 (2), `xattrs_kw_included` 0x31600;
`e2fsprogs/e2fsck` `e2fsck_discard_blocks` 0x2e530.

The other cases the track named belong to the disproven halves and should NOT be attached to
this feature: `bash yyparse` 0x35806 and `add_undo_redirect` 0xb9166,
`iproute2/ip strxf_share` 0x331b0 and `mroute_list` 0x30d40, `openssh-portable/ssh main`'s
`dat_114748`, `tar oldgnu_sparse_member_p` 0x27f10 are genuine RAM globals (disproof 3);
`bash save_tty_chars` 0xff79e and the betaflight/cleanflight `decodeEscFrame` /
`saCmsCommence` / `hmc5883lRead` / `isChecksumOkIa6` rows are refinement reassembly of real
memory (disproof 2).

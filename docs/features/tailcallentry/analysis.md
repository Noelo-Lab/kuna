# tailcallentry — the function entries the Listing walk absorbs at a tail call

Step 3 of the approved ARM entry-recall sequence (`[PROPOSAL]` PR #239,
`docs/features/arm-entry-granularity/`), and the step both shipped predecessors
independently identified as their blocker: **513 missed function entries, 17.4%
of the ARM entry-recall gap**, all of them routines that some function reaches by
a tail `B` and that `listing/walk.rs` therefore swallows instead of starting a
function at.

Every number below is measured on this branch against the **stripped** binaries
decbench feeds the decompilers, with ground truth taken from the matching
`compiled/` binary's DWARF (`DW_TAG_subprogram` `DW_AT_low_pc`, Thumb-LSB-masked).
Corpus: every ARM Cortex-M image the three optimisation trees carry — 8 projects
x 3 levels = **96 images, 43,809 ground-truth function addresses**, a strict
superset of the 48-image subset steps 1 and 2 used (that subset's libopencm3
example selection is recorded in neither bundle, so it could not be reproduced).
Raw per-binary tables: `measurements.txt`.

---

## 1. Headline

| surface | ground truth | recall before | recall after | new entries | of which GT | **splits a real body** | outside any DWARF body | **entries lost** |
|---|---|---|---|---|---|---|---|---|
| `kuna functions --mode aggressive` | 43,809 | 40,309 (92.0%) | **40,870 (93.3%)** | 593 | 561 (**94.6%**) | **0** | 32 | **0** |
| stacked on `--option ptrentry on` | 43,809 | 42,028 (95.9%) | **42,589 (97.2%)** | 593 | 561 (**94.6%**) | **0** | 32 | **0** |
| `--option listing off` | 43,809 | 13,277 | 13,277 | 0 | — | 0 | 0 | **0** |

**+561 recovered ground-truth functions at 94.6% precision, zero split bodies,
zero losses.** The acceptance bar was >= 80% ground truth with zero regression;
the proposal's measured naive arm for this step was 39% precision with 55% of the
new entries splitting a real body, and its "stricter" variant 54%/38%.

`ptrentry` (step 2) and this step are **exactly orthogonal** — the delta is
identical with it off and on, because both emit additive entry facts and neither
enters `Listing::functions()`, which is what the containment guard reads. Stacked,
corpus entry recall reaches **97.2%**.

## 2. Why the walk swallows these routines

`listing/walk.rs` runs a two-level worklist: an outer function worklist and an
inner per-function instruction worklist. A CALL target joins the function
worklist; **every other flow target joins the instruction worklist**, i.e. is
treated as a same-function successor:

```rust
if c.flow.is_call {
    st.funcs.entry(t).or_insert_with(|| discovered(t));  // a new function
    func_worklist.push(t);
} else {
    insn_worklist.push(t);                               // same-function flow
}
```

An unconditional `B` to another routine is therefore intra-function flow. The
routine is decoded — its bytes are not dark — but it has no entry of its own, so
it is emitted as part of whichever function branched into it, and every consumer
that asks "is this address undefined?" is told no. That second effect is what
made this step the sequencing blocker: step 1 (`cortexmvectors`) gained 44 entries
and **lost 54 ground-truth ones** to it, and step 2 (`ptrentry`) measured a
walk-reseed wiring at **734 lost entries and 974 split bodies** and shipped an
additive one instead.

## 3. The structural result: the split cannot change what the walk decodes

Write the walk as its closure. `I` is the decoded-instruction set, `F` the
function-entry set:

- every `e` in `F` is decoded, so `F` is a subset of `I`;
- for `i` in `I`, every non-call flow target and the fall-through join `I`;
- for `i` in `I`, every call target joins `F`.

Moving a tail-call target from `I` to `F` leaves this closure **fixed**, because a
function entry is walked and therefore decoded either way. The split is purely
additive on `F`; `insns`, `covered` and the reference model are untouched.

Two consequences, and they are the whole shape of this PR:

1. **The split can be computed after the walk**, with complete predecessor and
   region information, instead of with whatever the LIFO worklist order happened
   to have exposed at the moment the branch was reached. That matters: the
   obvious in-walk test ("is the target already claimed by this function?") is
   order-dependent and answers "no" for any forward branch.
2. **"This option never removes an entry" becomes a property of the wiring.**
   The Listing is never rebuilt, so `funcdisc_recursive`, `aif`, `ptrentry` and
   every other consumer see byte-identical input. That is the same conclusion
   step 2 reached from measurement; here it is provable rather than measured, and
   the 0-lost column is a structural fact, not a lucky number.

## 4. The precision model: region containment

Recognising the branch is trivial. Telling a tail call from a rotated loop head
is the entire problem, and the proposal is right that the naive rule is unusable.
Re-measured on all 96 images, the naive arm produces **39,128 new entries of which
664 (1.7%) are ground truth and 37,667 split a real body**.

Four guards, each measured, take it to 94.6% with no split bodies.

| guard | what it rejects |
|---|---|
| `ImpurePredecessor` | any target with a fall-through or conditional-branch predecessor — the caller's straight-line code runs into it, so it is ordinary intra-function flow |
| `InCallerRegion` | any branch that does **not** leave the caller's entry-ordered function region, i.e. no other discovered entry lies between the branch and its target |
| `NotTerminating` | any target whose flow region never reaches a `RETURN` or a computed jump |
| `EpilogueStart` | any target that opens with a stack restore (`POP`/`VPOP`/`LDM SP!`/`ADD SP`) — a function does not begin by tearing down a frame it never built |

Leave-one-out, on the full corpus:

| drop | new | ground truth | splits |
|---|---|---|---|
| — (shipped) | 593 | 561 (94.6%) | **0** |
| the region crossing | 1274 | 656 (51.5%) | **514** |
| the pure-predecessor test | 790 | 562 (71.1%) | 156 |
| the epilogue-start test | 605 | 561 (92.7%) | 12 |
| the terminating-region test | 615 | 568 (92.4%) | 0 |

**The region crossing is the containment fact the proposal asked for**, and the
ablation says so: without it precision collapses by 43 points and 514 real bodies
are split. It is the same shape as `ptrentry`'s decisive guard — a question about
which *discovered function region* an address belongs to, asked at the
entry-ordered granularity (`Listing::function_containing`: the nearest preceding
entry) that is the only granularity this tier has. `ptrentry` rejects a pointer
whose referencing word sits in the *same* region as its target; this rejects a
branch that never *leaves* its own region.

Why it works: a compiler lays a function's blocks out contiguously, so an
intra-function branch — including the rotated loop head that destroys the naive
rule — lands between its function's entry and the next discovered entry. A tail
call, by contrast, leaves for a routine somewhere else in the image, and the walk
has almost always discovered something in between. It errs conservative on a
sparsely discovered image (a tail call to the very next, still-undiscovered
routine is refused), which is the right direction: 89 ground-truth candidates are
declined this way.

### 4.1 The stack-discipline model was built, measured, and dominated

The obvious principled alternative — and the one this work started from — is
**frame discipline**: a branch taken while the caller still has an unmatched
`PUSH` / `SUB SP` / `VPUSH` / `STMDB SP!` open cannot be a tail call, because the
callee would run inside the caller's frame. It was implemented as a forward
may-analysis over the caller's region and measured on the whole corpus:

| model | new | ground truth | splits | outside any body |
|---|---|---|---|---|
| pure-predecessor + frame discipline | 636 | 572 (89.9%) | 18 | 46 |
| **pure-predecessor + region + terminating + epilogue (shipped)** | 593 | 561 (**94.6%**) | **0** | 32 |

Region containment beats it on precision *and* on splits, and adding frame
discipline on top of the shipped rule removes 39 more entries while removing no
split. It is not shipped. This mirrors step 2 discarding the proposal's named
guard (table-run corroboration) after measuring it: the named guard was not the
one that worked.

The 18 residual splits of the frame model are, without exception, tiny shared tail
blocks — a `bx lr` or a `mov ; bx lr` that several `b`s in one function jump to.
The region test rejects every one, because they never leave their function.

### 4.2 Inter-function alignment padding was measured and declined

Requiring an undefined byte immediately before the target (function-boundary
alignment padding, a classic Ghidra/IDA signal) raises precision to 96.2% and
still splits nothing — at the cost of **186 ground-truth functions**, because
Thumb routines are two-byte aligned and abut exactly. 1.6 points of precision,
almost all of it inside the unverifiable "outside any DWARF body" bucket, is not
worth a third of the recall. Both configurations are in `measurements.txt`.

### 4.3 It is not curve-fitted

The four guards were chosen from a pool of 22 candidate features, so a holdout
was run: fit the best zero-split subset on half the projects, score it on the
other half.

| fit half | fitted subset | on the holdout half |
|---|---|---|
| betaflight, crazyflie, nuttx, libopencm3 | pure-predecessor + epilogue + region | 152 new, 136 GT (89.5%), **0 splits** |
| chibios, cleanflight, freertos, riot-os | pure-predecessor + region | 475 new, 432 GT (90.9%), 12 splits |

The shipped rule scores 93.3% (0 splits) on the fit half and **99.2% (0 splits)**
on the holdout half. Per project it ranges 90.3%-100.0% and per optimisation level
85.7% (O0) - 96.8% (O2-noinline), with zero split bodies in every cell.

## 5. What comes back

The recovered entries are ordinary functions, not fragments. Three consecutive
ones from `nuttx` O2, with their DWARF names:

- `0x8000b04` **`up_irqinitialize`** — the NVIC init routine: seven MMIO stores
  and a priority-table loop.
- `0x8000d3c` **`up_trigger_irq`** — a four-arm switch, three arms tail-calling
  `modifyreg32`.
- `0x8000e94` **`board_autoled_initialize`** — four `stm32_configgpio` calls, the
  last one a tail call.

Every one of them previously produced no function of its own.

## 6. Where the residual sits

32 of the 593 new entries fall outside every DWARF body. They are concentrated in
crazyflie (28) and are the same population step 2 documented: vendor startup,
newlib and compiler-rt code the application CU's debug info does not describe.
Two of them are in `CMSIS_DAP.axf`, whose DWARF covers one CU of a multi-vendor
image. They are unverifiable rather than wrong.

The 89 ground-truth candidates the region guard declines are tail calls to the
immediately following, still-undiscovered routine — the case where the walk has
nothing between the branch and its target. Recovering them needs a *denser* entry
set, which is what steps 1, 2 and 4 supply; the guard gets strictly better as the
rest of the sequence lands.

## 7. The AIF gap-start re-walk stays out (measured)

The proposal flagged a related omission — `passes.rs` emits `run_aif`'s accepted
gap starts without the walk re-seed Stages 2 and 3 do — worth +39 entries over 9
binaries, and step 2 assigned it here. Implemented on this branch (reverted; not
in this diff) and measured over the same 96 images:

| | recall delta | new | of which GT | splits | **entries LOST** | of which GT |
|---|---|---|---|---|---|---|
| re-walk, `tailcallentry` off | +87 | 1355 | 174 | 1013 | **109** | **87** |
| re-walk, `tailcallentry` on | +86 | 1424 | 193 | 1059 | **135** | **107** |

**Not folded in**, and the tail-call split does not make it safe — it makes it
marginally worse. Section 3 says why: a tail-call split leaves the walk's
decoded-instruction closure fixed, whereas a re-seed *adds* to it, turning
undefined gaps into code and making other stages' candidates fail their
`is_undefined` guard. Nothing about tail calls can defend against that. It needs
its own investigation, with its own precision model.

## 8. Strict no-op elsewhere

The pass is ARM-gated, like the sibling Stage-2/3 seed scans (the mechanism is
architecture-neutral; the evidence is not — the proposal's recommendation was
"ARM-first, generalise on evidence"). Whole-binary `kuna decompile-all --json` is
**byte-identical** (sha256) off vs on for x86-64 (`gzip`), a PE binary
(`mydoom.exe`, which matters because PR #254 changed PE flow termination), and the
Cortex-M images with no admissible candidate (`blink.elf`, `miniblink.elf`). The
XML datatest path never builds an `ObjectLoadImage` or a Listing, so the option
cannot fire there at all: `make test` is 675/675 **PARITY OK** with the flag
registered, and `make test-stages` 394/394 with no baseline re-record.

**u-boot (ARM Linux, A32) is in scope and is not byte-identical.** Disclosed
rather than gated away: 21 new entries, 19 ground truth (90.5%), **2 split a real
body**, 0 removed. It clears the >= 80% bar on a population the model was never
fitted on — A32 rather than Thumb, Linux rather than bare metal — but the two
splits are outside the zero-split result the Cortex-M corpus shows. Both are
three-instruction regions inside a sparsely walked function; a length floor would
remove them and cost ~60 Cortex-M ground-truth entries, which is exactly the trade
step 2 rejected when it replaced `MIN_SUBROUTINE_INSNS`. Left as measured.

## 9. Speed

The pass is one pass over the decoded instruction map to collect branch targets
plus, per surviving candidate, a bounded forward closure. See `measurements.txt`
block 7 for the table: on the discovery surface (`kuna functions`) every delta is
inside run-to-run noise on images from 144 ms to 4.9 s, and the `decompile-all`
deltas are the extra decompile work for the extra functions, sub-proportional to
it.

## 10. Default decision

**Default-OFF.** The option discovers more functions, so it changes emitted C by
construction; a clean `docs/baseline.json` ablation is evidence of
non-interference, not of quality. Two things should happen before a flip: a human
read of the precision numbers above, and a corpus sweep across more ARM firmware
than decbench carries — in particular enough A32 Linux images to decide whether
the u-boot result is representative. No DIV row: default output is byte-identical.

## 11. Reproduction

```bash
# entry recall, before vs after, against DWARF ground truth
kuna functions <stripped-arm-elf> --json --mode aggressive
kuna functions <stripped-arm-elf> --json --mode aggressive --option tailcallentry on
# compare both against DW_AT_low_pc of the matching compiled/ binary, and classify
# each new address as ground truth / strictly inside another body / outside every body

# the strict no-op check
kuna decompile-all <x86-64-or-PE-binary> --json --mode aggressive | sha256sum
kuna decompile-all <same> --json --mode aggressive --option tailcallentry on | sha256sum
```

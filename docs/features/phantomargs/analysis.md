# phantomargs / `argclobber` — analysis

Investigation of the fabricated third argument and the read-before-write locals in
coreutils `fmt` -O2 `main` (the campaign's named goal-2 correctness defect), with
the shipped option that removes it.

## 1. The measured symptom

```
$ kuna decompile-all .../O2/coreutils/stripped/fmt --addr 0x26a0
unsigned long sub_26a0(int a0,char **a1)
{
  ...
  unsigned long v9;  // rdx        <- never assigned anywhere
  unsigned long v10;
  unsigned long v11; // rdx        <- never assigned anywhere
  ...
      v10 = v9;                                 // line 107
  ...
    v10 = v11;                                  // line 119
  ...
          v12 &= sub_3700(v5);                  // line 133, one argument
  ...
        v3 = sub_3700(stdin,v6);                // line 142, two
  ...
    v12 = sub_3700(stdin,"-",v10);              // line 154, three
```

DWARF on the unstripped twin: `fmt(FILE *f, char const *name)` — **two**
parameters (`readelf --debug-dump=info`, DIE at `DW_AT_low_pc: 0x3700`, two
`DW_TAG_formal_parameter`). Three of the fifteen locals exist only to carry the
fabricated third argument.

## 2. Root cause, proven with an instrumented build

A throwaway instrumentation (`KUNA_DBG_TRIAL`, printing each `ParamActive` trial's
flags per `ActionActiveParam` pass, whether `AncestorRealistic` ran, what it
returned, and the defining op of the Varnode the call reads) on the call at
`0x2ac1`:

```
[TRIAL] ENTER check_input_trial_use call@0x2ac1 passes=0 maxpass=3 fullychecked=false ntrials=6
[TRIAL]   t1 slot=3 addr=register:0x10 sz=8 checked=0 active=0 used=0 defnouse=0 indcreate=0
          remformed=0 killedbycall=1 anc_real=0 anc_solid=0 condexe=0
          def=Some(CPUI_MULTIEQUAL) nin=3 in0=Some(CPUI_INDIRECT)[indcreate]
          in1=Some(CPUI_INT_ZEXT) in2=Some(CPUI_INDIRECT)[indcreate]
[TRIAL]   ANCESTOR(register) call@0x2ac1 t1 slot=3 killedbycall=1 -> realistic=1 solid=0 set_ind_create=1
[TRIAL]   AFTER call@0x2ac1 t1 slot=3 checked=1 active=1 defnouse=0 indcreate=1
...
[TRIAL] ENTER check_input_trial_use call@0x2ac1 passes=1 maxpass=0 fullychecked=false ntrials=14
[TRIAL]   t1 slot=2 addr=register:0x10 sz=8 checked=1 active=1 used=0 defnouse=0 indcreate=1
          remformed=0 killedbycall=1 anc_real=1 anc_solid=0 ...
[TRIAL] AFTER derive_input_map call@0x2ac1 ntrials=23
[TRIAL]   t8  slot=4 addr=register:0x38 used=1 indcreate=0 entry=Some(8)    <- RDI
[TRIAL]   t9  slot=3 addr=register:0x30 used=1 indcreate=0 entry=Some(9)    <- RSI
[TRIAL]   t10 slot=2 addr=register:0x10 used=1 indcreate=1 entry=Some(10)   <- RDX, kept
[TRIAL] AFTER build_input_from_trials call@0x2ac1 nin=4                     <- fspec + 3 args
```

`register:0x10` is RDX. Reading that trace against the brief's three candidate
causes:

**(a) The bit IS set, on the first pass.** `AncestorRealistic::enter_node`
(`substrate/funcdata_varnode.rs`, the `CPUI_INDIRECT` arm) sets
`set_ind_create_formed` and returns `PopSuccess` for the two indirect-creation
arms of the phi, `apply_trial` records `INDCREATE_FORMED` on the trial, and the
trial is `indcreate=1 active=1` from pass 0 onward.

**(b) The `is_checked()` guard is NOT the cause.** The three-input MULTIEQUAL
already exists when the trial is first checked (`passes=0`, the `def=` line
above), so the second pass skipping it (`p4_calls/funcdata_callsite.rs`, `if
trial.is_checked() { continue }`) changes nothing: re-checking would reach the
same conclusion on the same phi. The brief's leading hypothesis is refuted.

**(c) The refutation that killed `argindcreate` was mis-anchored.** The claim was
that `fspec.rs:3249/:3264-3267` — the `(is_rem_formed() || is_ind_create_formed())
&& !is_first_in_class()` -> `mark_no_use()` gate — lives inside
`fillin_map_standard` (the INPUT list) and therefore proves the bit is unset. In
the tree those lines are inside **`fillin_map_standard_out`** (fn at
`p4_calls/fspec.rs:3235`; `fillin_map_standard` spans `:3151-:3201` and contains
no such test). The gate is upstream's `ParamListStandardOut::fillinMap`, the
**return-value** list. The input list has no reader of the bit — which is exactly
what the D2 design claimed and the trace confirms.

So the chain is: `Heritage::guard_calls` plants an INDIRECT *creation* for a
killed-by-call register that is also a possible **output** location of the callee's
model (RDX is one on x86-64: a 16-byte value returns in RAX:RDX; with
`possibleoutput == false` the INDIRECT's `in0` would be flagged an *indirect zero*
and `enter_node` would answer `PopFailKill` instead) -> `AncestorRealistic` answers
realistic and records the bit -> `ancestor_op_use` finds no other use -> the trial
is active -> `fillin_map_standard` marks every active trial used -> three
arguments.

## 3. (a) Is this upstream-faithful?

**Yes.** Stock Ghidra's stored artifact for the *same binary*
(`decbench .../O2/coreutils/decompiled/ghidra_fmt.c`) emits:

```c
  ulong extraout_RDX;
  ulong extraout_RDX_00;
  ulong uVar11;
  ...
      uVar11 = extraout_RDX;
  ...
    bVar2 = FUN_00103700(stdin,&DAT_00108792,uVar11);   /* three arguments */
```

Same call, same three arguments, same two phantom values — Ghidra merely *names*
them `extraout_RDX`, which says out loud what kuna's `v9`/`v11` hide. kuna's
`fillin_map_standard` is a faithful port of `ParamListStandard::fillinMap`
(`build_trial_map` -> `force_exclusion_group` -> `separate_sections` ->
`force_no_use` -> `force_inactive_chain` -> mark active trials used), so this is
not a kuna scheduling divergence. IDA renders one argument at all three sites (its
own arity story, also not the DWARF two).

## 4. (b) The minimal sound fix

The seam is `build_input_from_trials`, immediately after
`kuna_calleearity::unify_with_sibling_call` — not `check_input_trial_use`, where
the trials are not yet sorted or entry-assigned and "trailing" is not computable
(`active.sort_trials()` runs at the tail of `fillin_map_standard`). Shipped as
`p4_calls/kuna_argclobber.rs`, option **`argclobber`, default off**, with seven
clauses:

1. **register trial, `is_ind_create_formed()`** — the upstream return-side test,
   applied to the input list;
2. **the clobber reaches the call directly** — the defining op is the indirect
   creation, or a MULTIEQUAL one of whose *immediate* inputs is one;
3. **every one of those creations is a creation of the argument register itself**
   (`clobber_of_this_register_reaches`);
4. **the callee's body does not read those bytes before writing them**
   (`CalleeEntryDead::proves_input`);
5. **trailing** — no used trial follows it, so no argument changes position;
6. **at least one argument remains**, since an empty list is `calleearityfwd`'s
   failure shape;
7. **a sibling that really used that storage wins** — no already-final call to the
   same callee entry in this function passed an argument there
   (`kuna_calleearity::best_witness_for`).

Clauses 3 and 4 were added in review round 2; §8 is what bought them.

**Ordering against the additive family.** `calleearity` runs first, in the same
function, so a slot it promoted from a sibling witness is visible to clause 7 and
declines the drop. The drop itself is `mark_no_use`, i.e. `DEFNOUSE`: every
deferred member of the family reads that as definitely-not-used
(`kuna_calleearitylive::capture_partial_call` skips such trials explicitly), so
none of them puts the argument back, and `plan_promotions` is aborted for that
slot by construction rather than by accident.

**Wrong-output risk (stated).** The INDIRECT creation exists *because* the register
is a possible return location. A callee that really returns a 16-byte value in
RAX:RDX and forwards the high half as the next call's trailing argument writes
nothing into RDX itself, so it satisfies clause 3 as well as clause 2; only the
callee probe (clause 4) can decline it, and only where the probe covers that
callee's entry. Default-off for that reason.

### Why clause 2 exists (a measured false positive)

Without it, the rule fires on tar `parse_datetime2` @0x48e10 -> `0x46550` and drops
a **real** argument: DWARF says that callee is `str_days(pc, buffer, n)`, three
parameters. Its RDX trial is `MULTIEQUAL(COPY, MULTIEQUAL)` — `n` is live from a
dominating block and the clobber merges in one phi deeper — while fmt's and grep's
are `MULTIEQUAL(INDIRECT[indcreate], ..., INDIRECT[indcreate])` with the clobber
immediate. Clause 2 keeps the correct drops and declines tar.

## 5. Measurements

Whole-corpus `decompile-all` sweep, option off vs on, **33 stripped binaries and
27,512 functions** from decbench `full_run_address_2026-09-11` — coreutils, grep,
gzip, bzip2, diffutils, findutils, tar, dpkg, e2fsprogs, bash, dash, cronie, kmod,
iproute2, libedit, rsyslog, shadow, zlib and openssh-portable on x86-64; u-boot,
betaflight and riot-os on ARM; mirai. Full artifact:
`docs/features/phantomargs/sweep-2026-09-16.txt`.

| | count |
|---|---:|
| functions scanned | 27,512 |
| **functions whose emitted C changes** | **45** |
| changed call groups | 47 |
| binaries byte-identical off vs on | 22 of 33 |
| call sites created or destroyed | 0 |
| statement moves / control-flow changes | 0 |

Every changed site loses exactly one **trailing** argument, `3 -> 2` on x86-64 and
`2 -> 1` on ARM, and at every one of them the arity it lands on is the arity of the
callee's **own recovered prototype**. The three sites whose callee is a named
import are confirmed against DWARF on the unstripped twins: `xrealloc(void *,
size_t)`, `ext2fs_dblist_sort2(dblist, sortfunc)`,
`e2fsck_use_inode_shortcuts(ctx, bool)`. u-boot's two are `get_ticks` (DWARF:
**0** formal parameters) and `sub_6086752c` (one parameter, and 60+ other sites in
the image pass one). betaflight's `sub_8032628` is called with one argument at ~40
other sites.

Per-function hunk classification on the two original witnesses:

* `fmt` `sub_26a0`: 3 declarations removed (`v9`/`v11`, the two phantoms, and
  `v10`, the fabricated argument's holder); the dead statements `v10 = v9;`,
  `v10 = v11;` and the now-unconsumed `(dat_17120 * 0xbb) % 200` remainder
  removed; `sub_3700(stdin,"-",v10)` -> `sub_3700(stdin,"-")`; and
  `sub_3700(v5)` -> **`sub_3700(v5,v6)`** — with the phantom gone, that call's
  recovered storage is two registers and `calleearity` reconciles the
  under-recovered site upward. All three sites now render 2 arguments, matching
  DWARF and the callee's own recovered prototype.
* `grep` `sub_cec0` (`kwsprep`): two `sub_b420(*v,&v,v)` -> `sub_b420(*v,&v)`
  (DWARF: 2 parameters) plus the two dead `v = v;` assignments they fed. Two
  further over-recovered sites in the same function are left alone by clauses 2
  and 7 — the rule is deliberately conservative, not complete.

### What decbench actually scores

The earlier record claimed the metric effect was *structurally* zero because
register locals are never exported. That was wrong, and the review was right to
falsify it: the pass can delete a `kind: arg` row when the dropped argument was
the enclosing function's own parameter's only use. Measured instead, on all 45
changed functions (`kuna decompile-all --addr <fn> --json`, `variables[]` rows
compared off vs on):

* **43 of 45 are row-for-row identical** in `(kind, stack_offset, size, type)`.
* u-boot `0x608715f0` loses a `kind: arg` row: its prototype narrows from
  `(uint4, unsigned int)` to `(uint4)`. `readelf` on the decbench twin says
  `0x608715f0` is `__udelay`, with **one** `DW_TAG_formal_parameter` (`usec`) —
  the second parameter kuna had was itself fabricated, so this is a type_match
  gain.
* bash `0xe30d0` (`expand_prompt`) has one `kind: stack` row at `-160` go
  `undefined8` -> `char *`. DWARF declares that function's `fbreg` locals at
  `-168` and `-176` only, so `-160` is a spill slot; the change replaces an
  untyped 8-byte slot with a typed pointer.

Speed, interleaved off/on pairs, minimum of 21, on a box running the workspace
suite throughout: fmt `main` 222.4 -> 226.9 ms (+2.03%), ls `main` (control, the
option never fires) 319.1 -> 320.4 ms (+0.40%). Budget +5%.

Gates on the shipped tree: `make test` PARITY OK 675/675, `make test-stages`
PARITY OK 1018/1018, `make test-cli` 172/172, `make check-spec` + `--strict` OK,
`kuna catalog --check` OK, `counters --check` no drift.

## 6. (c) Does `protoorder` (lane A3) make this unnecessary?

**No — they are complementary.**

* Parking the callee's own recovered prototype fixes all three `fmt` sites at once
  and is strictly better *there*. It is also whole-binary only: `kuna decompile
  <one function>` (the common agent path) has no callee pass to park from.
* grep's callee `sub_b420` is **self-recursive** (`sub_b420(*v4,a1)` inside its own
  body). A3's stated acceptance policy declines SCC members, so `protoorder` does
  not reach either grep site; `argclobber` does.
* A3 is a `[PROPOSAL]` awaiting go/no-go and costs ~+4% per function; this is a
  local, opt-in rule with no scheduling change.

If `protoorder` lands, the overlap should be re-measured: at a call whose callee
prototype is parked the input list is locked and this pass never runs (clause
`is_input_locked()`).

## 7. Deliverable 2 — where the `indirect_creation` flag is lost before naming

The flag is **not** lost in heritage or in the HighVariable.
`HighVariable::update_flags` (`p6_variables/variable.rs`) ORs every member
Varnode's flags, so `indirect_creation` reaches the high, and `is_extra_out`
(`indirect_creation && !addrtied`) is ported and correct.

What is missing is a *reader*. The only code that spells `extraout_` is
`Database::build_variable_name` (`p0_knowledge/database.rs`, the
`flags & indirect_creation` arm), reachable only from `build_default_name`, whose
sole production caller is `Database::assign_default_names` (`database.rs:4238`) and
which passes **`vn: None`** — exactly as upstream's
`ScopeInternal::assignDefaultNames` passes `(Varnode *)0`. With no representative
the flag-bearing branch cannot run, and the `num_entries()` branch below it
rebuilds `flags` as `addrtied` or `0`. Measured: with `KUNA_DBG_NAME`
instrumentation on `build_default_name`, decompiling `fmt::main` under `--option
namestyle ghidra` produces **zero** calls to it; the name `uVar9` comes from
`kuna_default_local_name` (`p6_variables/coreaction_cleanup.rs`), the `vN` /
`<t>VarN` allocator inside `name_local_highs_angr`, which serves *both* namestyles
and reads only `is_persist` / `is_addr_tied` / `is_input` / `is_constant` off the
name representative.

So the fix is: in that allocator, ask the name representative for
`indirect_creation` (and, for the same reason, the irregular-`input` and
`unaffected` cases that produce Ghidra's `in_EDI` / `unaff_R15`) and name the high
`extraout_<reg>` before falling through to the `vN` counter — or hand
`build_default_name` a `VnRepresentative` (the struct exists in `database.rs` with
no production constructor). **Size: S-M** (one arm in the allocator plus name
uniqueness) as a naming-only change, gated by an option because it renames
user-visible locals in the default rendering. It is *not* the 10-line arm the D2
design hoped for — the namer does not route through `build_variable_name` at all —
and it does not ride in this PR.

The two answers are alternatives to the same evidence: `argclobber` removes the
phantom, an `extraoutname` option would label it. Under `argclobber on` the fmt
phantoms disappear entirely, so the naming item only matters for the sites this
rule declines.

## 8. Review round 2 — the counterexample the first sweep missed, and what closed it

The first sweep covered 25 binaries and none of them was u-boot. Reviewing the
PR against decbench's `O2/u-boot/stripped/u-boot` (ARM, 1,727 functions) found
the rule deleting **live** arguments at five sites, of a class the PR did not
describe:

```
$ kuna decompile-all .../O2/u-boot/stripped/u-boot --addr 0x60827fa4 --option argclobber on
-   sub_60873270("%s %s: Invalid \"bus-width\" value %u!\n",v3,v4,v1);
+   sub_60873270("%s %s: Invalid \"bus-width\" value %u!\n",v3,v4);
```

`v1` is `ofnode_read_u32_default(dev->node,"bus-width",1)`, tested against 4, 8
and 1 three lines above, and it is the value `%u` prints. `sub_60873270` is
u-boot's `printf`, called from 1,924 sites and never recovered as variadic, so
`fc.is_dotdotdot()` could not have guarded it. A second site lost
`printf("Booting %pD\n",v3)`'s only argument, and a third
(`sub_6080b504 -> sub_6086609c`) took `if (*p == 0x2f) p = &p[1];` with it when
the argument it computed became dead.

**What the caller-side evidence actually was.** Instrumented on the first site,
`r3`'s defining MULTIEQUAL has two inputs and both are indirect creations — of
**`r0`**, not of `r3`, each with five readers. The ARM sequence is
`bl 0x60824d80; cmp; movne r3,r0; moveq r3,r0`: the value in `r3` is the previous
call's *return value*, moved there by the caller, and copy propagation had put
the creation of `r0` straight into `r3`'s join. Nothing in the trial says
"clobber" rather than "return value" — on every ABI the first return register is
also killed-by-call and a possible output location, so until the output seam
resolves it a call's result **is** an indirect creation.

That is clause 3, and it is the whole of the difference: require every creation
feeding the trial to be a creation of the argument register itself. It declines
four of the five u-boot sites, and it costs nothing elsewhere — a register the
caller *wrote* is a register the caller is passing, whatever the value in it came
from.

**Clause 4 is what scales past one function.** The fifth site,
`sub_6083af40 -> sub_6086b998(node,name,len)`, has `len` computed as `p - s` on
one path and left as `r2`'s own clobber on the other, so clause 3 passes. The
caller cannot settle it; the callee can, and `calleedeadarg` already decodes
every callee once per image to answer the opposite question. `proves_input` says
`sub_6086b998` reads `r2` before writing it, so the argument stays. The same
clause answers the whole variadic class at once — `printf`'s prologue spills
`r1`–`r3` into the `va_list` save area, which speaks for all 1,924 of its sites,
which no per-function sibling scan (the review's first suggestion) could do.

**What was NOT wrong.** The fifth u-boot drop, `sub_608715f0 -> sub_60871420`,
still fires, and `readelf` on the decbench twin settles it in the pass's favour:
`0x60871420` is `get_ticks`, **zero** `DW_TAG_formal_parameter`s, and
`0x608715f0` is `__udelay` with **one** (`usec`). The `kind: arg` row the review
saw deleted was the fabricated one.

**Re-sweep.** 33 binaries, 27,512 functions (§5), including u-boot, bash and
dpkg. u-boot now changes two functions, both verified correct against the callee
prototypes above.

**Refuted, with evidence.** The review asked for a program-wide version of the
sibling-witness clause, on the grounds that `best_witness_for` only sees the
current `Funcdata`. That is true, but the clause it would strengthen is not the
one that was failing, and a program-wide call-site witness would be
order-dependent (it can only see callees whose callers were decompiled earlier in
the run) where the callee-body probe is not. Clause 4 answers the same three
cases the review cited — `sub_6086609c`, and both `printf` sites — from evidence
that does not depend on decompilation order, and it is cached per image the same
way. `kuna decompile <one function>` gets it too, where a whole-binary witness
cache would be empty.

### Considered and rejected: a liveness test on the dropped value

The review's third suggestion — require the dropped Varnode to have no other
reader in the function — was implemented and measured before clause 3 was. It
does decline all five u-boot sites, but it is a proxy for the wrong thing and it
is expensive:

* On x86-64 the value has to be followed *backwards* through the width
  bookkeeping as well, because `mov %eax,%edx` lowers to `RDX = zext(EAX)` and
  that zext output genuinely reaches nothing but the join. So the rule is "no use
  of the value, or of anything it is a copy- or width-adjusted view of, is
  anything but a call argument".
* Even then it is not the invariant. openssh `ssh-keygen` `0x9b20` reaches
  `read_passphrase(prompt,flags)` with the other join input a `unique` COPY of a
  value the caller does use elsewhere — and the drop there is correct, because
  `read_passphrase` takes two parameters. Requiring deadness declined it, along
  with **55 of the 57** other correct drops on openssh, dpkg and bash: the whole
  sweep fell from 45 changed functions to 9.

Clause 3 answers the same five counterexamples by asking what the evidence
actually is (a clobber of *this* register versus a value moved in from another),
and costs nothing. Clause 4 then takes the case clause 3 cannot see. Every
changed call in the re-sweep is classified in
`docs/features/phantomargs/sweep-2026-09-16.txt` against the callee's own
recovered prototype, which is a stronger check than the per-callee arity
histogram the first round used and is what makes the five u-boot regressions
impossible to miss now: at each of them the site's *post*-drop arity would have
been below the callee's own recovered parameter count.

# 03 — SSA & simplification

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/p3_dataflow
```

This phase owns the **definition web**: the SSA linkage over the op-graph
(heritage — phi placement, renaming, call/return/load/store guards, the
dead-definition gate) and the **simplification fixpoint** that runs over it (the
rule pools, sub-variable flow, conditional-execution collapse, conditional
constants, and the kuna peephole rewrites). Nothing here runs as a standalone
stage: every pass in this chapter is a member of `mainloop`/`stackstall` or the
post-fullloop cleanup, scheduled and repeated exactly as §0.6 describes — SSA is
rebuilt incrementally each mainloop iteration and the pools re-fire between
rebuilds, until Band B reaches mutual quiescence.

Option metadata (defaults, tiers, symptoms, flip guidance) for every option
named below lives in the generated catalog ([docs/options.md](../options.md));
the rows are defined in `decompiler/crates/kuna-decomp/phases.toml` and the
default-divergence measurements are DIV-2/DIV-3 in `docs/history.md`.

## 3.1 Heritage

`decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs (Heritage)` is the
SSA construction engine — the port of the upstream `heritage.cc`. It is owned by
the function (`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs
(Funcdata::op_heritage_with_deadline)`) and driven once per mainloop iteration
by `decompiler/crates/kuna-decomp/src/p3_dataflow/coreaction_early.rs
(ActionHeritage)`. SSA is therefore built over **multiple passes**, not once: a
*free* Varnode (a value not yet linked to a defining op) becomes *heritaged*
when some pass collects its address range, and each pass increments the engine's
`pass` counter that everything else in this section keys on.

**Per-space staging.** Each address space carries a
`heritage.rs (HeritageInfo)`: a `delay` (how many passes to wait before
heritaging the space at all) and a `deadcodedelay` (how many passes to wait
before dead code may be removed there), both seeded from the processor spec's
per-space values. The registers heritage on pass 0; the stack space is typically
delayed one pass so that indirect references through the not-yet-renamed stack
pointer have a chance to materialize as located varnodes first (the
`heritage-staging` row in `decompiler/crates/kuna-decomp/phases.toml` — latent,
no user assertion). A space whose `delay` has not elapsed is skipped for the
round.

A spacebase space additionally carries `has_call_placeholders`, and reaching it
for the first time is the moment the call sites' stack-pointer placeholders come
back off (`heritage.rs (Heritage::clear_stack_placeholders)`, which strips every
call spec's placeholder input through
`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs
(FuncCallSpecs::abort_spacebase_relative)`). The placeholder exists only to let a
call's stack offset be read out of the data flow before the stack is in SSA
(§4.3); once the space is being heritaged it has either been resolved or never
will be, and leaving it on the op would put a spurious stack read in the CALL's
argument list.

**Join-space varnodes (kuna: split at the first piece's pass).** A varnode whose
storage is several pieces lives in the join space and is split into its pieces
before the ranges are collected (`heritage.rs (Heritage::process_joins)`): a free
join read becomes a PIECE of its pieces, and a written or input join defines each
piece with a SUBPIECE. Upstream splits a written join on the pass of its first
(most significant) piece's space. For a struct parameter split between registers
and the stack on a little-endian target, that piece is the stack word, so the
split waited for the stack's pass, after the registers were already heritaged:
every read of the register pieces had become a bare input register that no
parameter accounts for. A copy of those pieces to memory was then dead to
`ActionDeadCode`, and `use3(&s)` printed as `use3(&v1)` with only `v1.z = s.z`
stored. kuna splits the join on the first pass that heritages any of its pieces.

**Address-range worklists.** The unit of work is an address range, not a
varnode. Two disjoint-cover maps drive each pass
(`heritage.rs (LocationMap::add)`): `globaldisjoint` accumulates every range
ever heritaged (with the pass number it first appeared in), and `disjoint` holds
this pass's todo list. Adding a range returns an intersect code — `0` all-new,
`1` partially overlapping an older range, `2` wholly contained in one — and the
driver (`heritage.rs (Heritage::heritage)`) files the range under
`new_addresses`/`old_addresses` flags accordingly. That classification is
load-bearing twice over: only ranges with new addresses get call/return guards
(below), and an *old* overlap is the trigger for the dead-code-delay machinery.
`add` also hands back the size of the element the range ended up inside, which is
the other half of the C++ iterator its callers read, so classifying a range costs
one lookup rather than an add followed by a second search for the same entry; and
the walk that finds the candidate element asks for the *predecessor* of the range
first, which answers both the "step back from `lower_bound`" and the "already at
`begin()`" cases in one descent. Every varnode of every heritaged space is
re-offered to this map on every pass, so it is the most-called map in the phase.

**The simple case.** For each disjoint range, `heritage.rs (Heritage::collect)`
partitions the range's varnodes into reads (free), writes (defined), and
inputs. It walks the loc-tree's bounded half-open `[start,end)` slice in
location order (a wrapped end runs to the current space's end), rather than
scanning every varnode. Writes smaller than the range are widened through a
PIECE concatenation (`normalize_write_size`), reads smaller than the range are
served by a SUBPIECE (`normalize_read_size`), and input holes are filled and
concatenated (`guard_input`). Phi placement then runs the Bilardi–Pingali
augmented-dominator-tree algorithm (`heritage.rs (Heritage::build_adt)`,
`heritage.rs (Heritage::calc_multiequals)`) with a depth-keyed, LIFO-within-depth
priority queue (`heritage.rs (PriorityQueue)`) — the queue order decides
MULTIEQUAL placement order and is therefore observable output — and
`heritage.rs (Heritage::place_multiequals)` inserts a MULTIEQUAL with one free
input per in-edge at the head of every merge block. Renaming is the classic
Cytron et al. dominator-tree stack walk
(`heritage.rs (Heritage::rename_recurse)`): reads take the top of the
per-address `VariableStack`, writes push, and the walk pops on exit. A read
whose stack is *empty* has no reaching definition — it is materialized as a
formal **input varnode** of the function; this is how registers read before
being written become parameters-in-waiting for phase 04. One carve-out: an
INDIRECT and the op it wraps happen "at the same time", so an op whose renamed
read would resolve to its *own* INDIRECT output takes the next value down the
stack (or a fresh input) instead (`heritage.rs (op_from_const)`). After a block's
own ops are renamed the walk fills the in-edge slots of each successor's leading
MULTIEQUALs; it reads only that leading run (phi ops are always at the head of a
block and the walk stops at the first op that is not one), so it collects the run
off the block's intrusive op list rather than materializing every op of every
successor once per CFG edge per pass.

**Op properties on a heritage-created op.** The three op-codes heritage installs
(the phi, the read-size SUBPIECE, the write-size PIECE) take their property flags
from the same op-code table every other producer of p-code resolves through
(`decompiler/crates/kuna-decomp/src/p5_types/typeop.rs (seam_type_op_for)`), so a
heritage-created op is indistinguishable from the same op-code installed by a
simplification rule. The phi's entry carries `special`, `marker` and `nocollapse`
together, and each of the three is load-bearing somewhere: `marker` is what
`is_marker` reads, `nocollapse` keeps the constant folder off a phi whose inputs
happen to all be constants, and `special` — read as `get_eval_type() == special` —
is the test every pass that splices an op into a block, moves an op within one,
or gathers an expression uses to recognise that a MULTIEQUAL is not an ordinary
computation. `ScopeLocal::annotate_raw_stack_ptr` (chapter 06) is the sharpest
case: it splices a zero-offset `PTRSUB` placeholder in front of each op that
reads the raw input stack pointer, and skips `special` readers precisely so the
placeholder cannot land inside a block's leading phi run. A phi that fails that
test takes a `PTRSUB` in front of it, and both consumers of the leading run — the
renaming walk above, and `ConditionalJoin::cut_down_multiequals` (chapter 08),
which drops one in-edge slot from every phi of a joined exit block — stop at the
placeholder, leaving the phis behind it with one input more than their block has
in-edges. Skipping the phi is the only correct choice — a phi's input must be
defined on the incoming edge, so there is nowhere in the phi's own block the
placeholder could legally go — but it has a visible cost: `mov rax,rsp` merges
the unaffected input stack pointer into the same HighVariable as the register
that copies it, and `HighVariable::has_name` (chapter 06) refuses to name a high
carrying the stack pointer, so with no placeholder the leaf is printed from the
member's own storage and renders a bare register or `Unique<hex>` name rather
than `&Stack00000000`. Annotating the raw stack pointer on a phi's *incoming
edge* would recover the frame-relative spelling, but upstream does not do it and
it would be a new decision point, not part of this invariant.

**Materializing an input over existing pieces (kuna, DIV-50).** The input a
stack-empty read materializes may land on storage that already holds input
varnodes. Upstream refuses that outright — `Funcdata::set_input_varnode` raises
`Overlapping input varnodes` and the function is abandoned with no body at all.
The reachable case is `guard_input`'s own residue: it tiles a partially-input
range with input pieces, marks each piece *write-masked* so `collect` stops
seeing them, and represents the range by the PIECE concatenation instead. When
the rule pools later fold that PIECE away and a new free read of the full range
arrives on a subsequent pass, the read is asking for exactly the value those
pieces still hold. `kuna_inputtile.rs (new_tiled_input)` therefore
completes the tiling (creating an input for any gap, as `guard_input` does) and
folds it into one full-size input with
`decompiler/crates/kuna-decomp/src/substrate/funcdata_varnode.rs (Funcdata::combine_input_varnodes)`, which
destroys the pieces, rewrites each concatenating PIECE into a COPY, and repoints
every other reader at a SUBPIECE of the new whole. Only write-masked pieces
fully contained in the request are folded — a write-masked varnode is never
pushed onto a `VariableStack`, so no stack can be left holding a destroyed id —
and any other overlap still raises the upstream error.

**Phi-range granularity (refinement).** When a range is bigger than 4 bytes and
no single write covers it (`size > 4 && maxwritesize < size`), the range is
split at every varnode boundary observed inside it before phis are placed
(`heritage.rs (Heritage::refinement)`): ranges over 1024 bytes are never
refined, and a 1-byte/3-byte adjacent split is healed back to 4
(`remove13_refinement`). Refinement rewrites the disjoint covers (local and
global) in place and re-enters the walk at the first partition. Its inverse
exists too: when a *larger* range arrives over addresses already heritaged at a
smaller size, the stale MULTIEQUAL/INDIRECT/return-COPY markers from the earlier
pass are deleted and the old outputs re-derived as SUBPIECEs of the new full
range (`heritage.rs (Heritage::remove_revisited_markers)`).

**Refinement pieces keep the store mark** (`option splitstorekeep`,
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_splitstorekeep.rs
(keep_store_mark)`, default **on**). A frame store reaches this phase as a
direct `stackvn = COPY(value)` whose output `RuleStoreVarnode` marked
`stack_store` — "originally came from a CPUI_STORE". That mark is what keeps the
store printed: `ActionDirectWrite` calls a COPY into the frame a *direct write*
only when its output is a stack store, and dead-code elimination opens by
dropping the `addrforce` mark of anything that is not a direct write, after
which nothing consumes the store and the COPY is swept (§3.9, §9.2). The pieces
`refine_write` cuts a store into are fresh varnodes, so upstream they carry no
mark and every one of them is deleted — which is visible exactly where two
stack accesses overlap, since that is the only thing that makes refinement
split. The shape is the compiler's own idiom for copying an object whose size is
not a multiple of the word: a 31-byte copy done as four 8-byte moves at offsets
0, 8, 15 and 23 overlaps on byte 15, so the two middle stores are refined away
and the emitted C copies the first eight bytes and the last eight and nothing
between them. With the option on the mark is carried onto each partition cell,
which claims nothing new — a piece of a store is a store — and the copy prints
in full. `option splitstorekeep off` restores upstream's unmarked pieces.

**Call and return guards.** For ranges with new addresses, data-flow across
call sites is made explicit before renaming
(`heritage.rs (Heritage::guard_calls)`). Each call spec is asked what effect the
call has on the (callee-translated) range (`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs
(FuncProto::has_effect)`):

- *unknown effect* or *return address* → an INDIRECT op re-defines the range
  across the call, so its lifetime honestly spans the call site; if the range
  is address-tied the INDIRECT output is `addrforce`d (kept alive against
  dead-code) — the alias guard a call casts over memory it might touch through
  a pointer;
- *killed by call* → an INDIRECT *creation* (a definition from nothing) whose
  output is the potential return value, registered as an output trial when the
  call's output recovery is active;
- input-active call → a fresh varnode at the range is appended to the CALL as a
  tentative argument and an input trial registered — this is where register and
  stack arguments physically join the call op (chapter 04 judges the trials);
- a callee returning a struct into locked stack storage materializes the
  delayed CALL output and SUBPIECEs/PIECEs it into the range
  (`heritage.rs (Heritage::try_output_stack_guard)`).

**A range the call returns in.** Upstream skips a call whose output varnode
covers the heritaged range exactly: the call defines the range itself, so no
INDIRECT is needed. The skip also drops the input trial, which loses an
argument the call reads from the register it returns in. A declared
`double vr(int n, ...)` called as `vr(k, k + 1, 0.5)` gets its `0.5` in `xmm0`
on x86-64 (`d0` on AArch64, `f1` on 32-bit PowerPC), and the call printed as
`vr(k,k + 1)`. kuna still registers the input trial for such a range when the
call is variadic and its model passes a variadic argument there (chapter 04,
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_varargretreg.rs
(argument_in_own_output)`); the range still gets no INDIRECT.

**A counted floating-point register whose range is not the register.** On
x86-64 SysV a variadic caller puts the number of vector registers it filled in
`al` (chapter 04), and each of those carries an 8-byte `double`. The heritaged
range is often not that 8-byte `xmmN_Qa`. SLEIGH models `pxor`, `movq` from a
general register and `movdqa` as a write of the whole 16-byte register, so gcc
-O2's `pxor %xmm0,%xmm0; cvtsi2sd %edi,%xmm0` and gcc -O0's `movq %rax,%xmm0`
leave a 16-byte range: the input guard finds no entry that contains it, and the
argument was dropped (`printf("%f\n")`, `vr(k)`). SLEIGH models `movaps` as
four 4-byte lane copies, so clang's `movaps` swap of two doubles refines the
range into 4-byte lanes: the low lane is a justified piece of the entry, and the
argument was registered at 4 bytes (`vr(k,y,SUB84(x,0))`).
`heritage.rs (Heritage::guard_counted_float_input)` handles both for a register
that `decompiler/crates/kuna-decomp/src/p4_calls/kuna_varargretreg.rs
(counted_float_entry)` names: one of the first `al` floating-point entries of a
variadic call's model that the range overlaps without being it. A range that
contains the register passes its low 8 bytes through a SUBPIECE, as
`Heritage::guard_call_overlapping_input` does under `calloverlap`. A range that
starts the register and is narrower than it passes the PIECE of a read of
itself and of the ranges of this pass that cover the rest of the register (the
PIECE and any SUBPIECE that trims the last range write temporaries, so no new
register write enters the heritage), and the 4-byte trial is not registered.
When the rest is not covered by ranges of this pass, or a trial already overlaps
the register, the ordinary guard runs. Either way the trial is 8 bytes at a
counted register, so chapter 04 marks it active. A register past the count, or
a call whose block states no count, is unchanged.

**Narrowing the killed set to the callee's own writes.** A `<killedbycall>`
block in a compiler spec is a statement about the *convention*, not about any
particular callee, and there are callees the convention does not describe. The
one that costs a reader most is the i386 get-PC thunk gcc emits for every PIE
that reaches a global: `mov ebx,[esp]; ret`, four bytes that write `EBX` and
nothing else, called in place of a convention-abiding call precisely because it
is cheaper. `x86gcc.cspec` puts `ECX` and `EDX` in `<killedbycall>`, so a value
the caller loaded into `EDX` before that call becomes an INDIRECT creation and
every later read of it prints as a local the function never assigns.

Under `option calleepreserves` the call guard consults the callee's own
instructions instead. The evidence is the bounded body walk chapter 04 already
takes for the call-output seam (`decompiler/crates/kuna-decomp/src/p4_calls/kuna_rustabi.rs
(probe_callee_return_writes)`): from the callee's entry it follows fall-through
and resolved machine branch targets, ends a path at a `RETURN` or an
unconditional direct call with the same known no-return contract used by flow
construction, and declares itself *incomplete* — proving nothing — at a
returning or unknown nested call, an unresolved `BRANCHIND`, a user op (`CALLOTHER`),
an undecodable instruction, or its instruction budget. A complete
walk that records no write to the range downgrades *killed by call* to
*unaffected* for that one call, so no INDIRECT is planted and the caller's value
flows across (`decompiler/crates/kuna-decomp/src/p4_calls/kuna_calleepreserves.rs
(callee_preserves_range)`). The output-active arm is skipped for the same range:
a register the callee provably never writes cannot be carrying its return value
either, so registering an output trial for it would put the clobber straight
back.

Absence of a recorded write is not on its own enough to act on, and the second
half of the test is what keeps the rule off a body that is not really a body. A
summary with no writes at all is the maximal claim — *every* register survives
this call — drawn from the weakest possible reading, and a one-byte `ret` is
what a stub, a placeholder and a misidentified entry all decode to. So the
callee must also have written a register the model itself marks `<unaffected>`,
excluding the stack pointer, which every `RET` writes, and the program counter a
return instruction writes and then transfers through, which `ARM.cspec` lists as
`<unaffected>` and every `bx lr` writes
(`kuna_calleepreserves.rs (body_departs_from_convention)`). That is the
signature of the hand-rolled helper the rule exists for — the get-PC thunk's
`EBX` is callee-saved, so the convention is already not a description of it —
and it is a positive finding rather than an absence. Only a processor-space
range is ever answered, because a callee's memory writes are `STORE`s through an
address the walk cannot follow; only *killed by call* is downgraded, never
promoted; and a prototype carrying its own effect-record override has had a
deliberate statement made about it and is left alone.

**A well-behaved helper is invisible to that test.** Requiring a write the
model marks `<unaffected>` is a positive finding, but only for a callee that
breaks its convention. MSVC's out-of-line stack probe does not: it writes `RSP`,
`R10` and `R11`, and `x86-64-win.cspec` names none of them in either
`<unaffected>` or `<killedbycall>`. They are scratch, and clobbering scratch is
what the convention permits. So every large-frame prologue — `mov eax,SIZE; call
__chkstk; sub rsp,rax`, which MSVC emits for every frame over a page — lost the
size the caller loaded even though the probe *reads* `RAX` and never writes it,
and `sub rsp,rax` became `v1 = -v2` off an unassigned local with every frame
slot then indexed by it. That is the whole frame destroyed by one INDIRECT
creation, not one expression.

`option calleescratchbody` adds a second, independent way for a summary to count
as a body, and requires both of its marks
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_calleescratchbody.rs
(scratch_body_is_a_body)`). The body must have written **memory**: a
register-preserving helper's own save slot is why it can use a register at all
— `__chkstk` spills `R10`/`R11` into its frame before it touches them and
reloads them before it returns — while `ret`, `endbr64; ret`, a placeholder and
an entry decoded at the wrong address store nothing at all. And it must have
written a register other than the stack pointer, which every `RET` writes, so
the vacuous summary is still refused. Nothing the paragraphs above establish
moves: the walk must still be complete, the range must still be a register the
walk proves untouched, an explicit effect override still wins, and only *killed
by call* is ever downgraded.

**The return register is a different question.** That positive finding — a write
to a register the model marks `<unaffected>` — is the signature of a hand-rolled
helper, and a helper that clobbers only what its convention already allows never
produces it. MSVC's frame-cookie checker is the case: `x86-64-win.cspec` lists
`RAX` in `<killedbycall>`, `__security_check_cookie` writes `RCX` and the flags
and returns, and a `/GS` epilogue therefore prints `return
__security_check_cookie(v12);` — a return value the machine never computes,
although the caller set `EAX` to zero *before* the call precisely because the
checker leaves it alone. Declaring the callee void does not help; it turns the
invented call result into an uninitialised local.

For the call's return storage the evidence available is sharper than for a
scratch register, and `option calleeretpreserves` asks for that instead: a callee
that returns a value in `RAX` must *write* `RAX`, so a complete walk that records
no write to the return storage proves the call has no return value at all
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_calleeretpreserves.rs
(callee_preserves_return_storage)`). For an exact range the effect is downgraded
to *unaffected*, the output-active arm is skipped, and the caller's own
definition flows across the call.

Three conditions bound it. The range must characterize as the call's *output*.
For a locked `void` declaration, which intentionally has no concrete output
entry, `kuna_calleeretpreserves.rs (characterize_preserved_output)` consults the
prototype model's ABI output list solely to identify eligible return storage;
a locked non-void output remains authoritative, and scratch XMM registers never
become eligible through this fallback. Eligibility is exact even when heritage
tracks a wider register: Win64's 8-byte `XMM0_Qa` output inside a 16-byte `XMM0`
range is extracted from the reaching pre-call value, while killed INDIRECT
creations supply the flanking bytes before a post-call PIECE rejoins the range.
The upper `XMM0_Qb` scratch lane is therefore not preserved by a whole-range
`unaffected` downgrade. Every other killed range stays where the paragraph above
left it. A body that writes part of the return storage returns a value there,
and it answers only for a return register it never writes, one register at a
time: gcc's `-fipa-ra` keeps a caller's pointer in MIPS `$v1` across a callee
it knows writes only `$v0` (`move v1,a0; jal hk; ...; lw a0,0(v1)`), and
killing `$v1` at that call made the read the high word of a 64-bit result and
lost the parameter it came from (GH-878). That answer needs a recorded write to
the call's return storage, not a possible `STORE` into its space, and it is
given only for a register the model never passes an argument in
(`kuna_rustabi.rs (passes_arguments_in)`): MIPS `$v1`, but not ARM's `r1` or
x86-64's `RDX`, where a value kept across one call reaches the
next as a value the caller wrote, which the caller-side argument recovery reads
as one more argument than that callee takes. A locked non-void declaration, such
as DWARF's `int hk(int)`, names only `$v0` as the call's output; `$v1`, a register
the model returns values in that the declaration leaves out, gets the same
per-register answer (`kuna_calleeretpreserves.rs (undeclared_model_output)`) and
never the void helper's. A body that writes no return
storage at all is the void helper above,
and the body must be a body — more than one decoded instruction, and a write to a
register the convention itself names, either an argument register or one its
`<unaffected>`/`<killedbycall>` lists mention (`kuna_calleeretpreserves.rs
(body_is_a_body)`). `ret` and `endbr64; ret` are what a stub, a placeholder and
an entry decoded at the wrong address all decode to, and they write nothing but
the stack pointer and the program counter; reading one as a promise about `RAX`
deletes the call results the rest of a function is built on.

Every ARM return and every MIPS `jr ra` runs the `setISAMode` user op, the
marker of the instruction-set bit of the PC write the same instruction then
transfers through (`bx lr`, `pop {...,pc}`, `jr ra`), and the walk used to stop
at it like any other user op, so it completed no ARM or MIPS body at all. It now
reads through that user op, which has no output and writes nothing
(`kuna_rustabi.rs (ProbeEmit)`); the compiler specs replace it with a no-op for
the decompiler itself. A walk that only completed that way answers for no
register the call's model passes arguments in
(`kuna_rustabi.rs (CalleeReturnWrites::proves_untouched_for)`), in every rule
that reads it. On ARM those are the scratch registers `r0`-`r3`, and keeping
them across every leaf that leaves them alone put an extra argument at 101
calls over gcc-built ARM firmware (decbench chibios, crazyflie, nuttx,
betaflight, u-boot), each beyond what the callee's own recovered prototype
takes (`strlen(s,a1)` after a `strlcpy` that never writes `r1`). Walks that
needed no mode switch keep their answer for every register, as before.

The walk itself gained one fact to reach this callee at all. The checker's
failure path leaves by a direct `JMP` into a `__fastfail` stub, and x86 SLEIGH
lifts `INT imm8` to `intloc = swi(imm8); call [intloc]` (`ia.sinc:3671`), whose
`CALLIND` would make the walk incomplete and prove nothing. Under the gate
`fastfailnoreturn` already applies — the option, plus a Windows
compiler-spec id — a Windows `int 0x29` ends the path instead
(`kuna_rustabi.rs (ProbeEmit::note_fastfail_swi)`), which is the same statement
the flow builder makes about the same two ops.

`msvcstackguard` has a narrower algebraic answer for the late P7 cookie shape.
In its destructive arm the first exact direct, unread-output,
one-cookie-cancel match records that instruction address and requests a restart.
Independently, `calleeretpreserves on` collects every exact locked-`void` match,
records the full set, and requests one restart; marking the whole set at once
keeps duplicated epilogues from exhausting the outer eight-reflow limit. On
replay `Heritage::guard_calls` downgrades only an actual `KILLEDBYCALL` effect on
ABI output storage at a seeded site. An explicit prototype effect and every ABI
return-storage write recovered from the callee body veto the downgrade, even
when an ordinary nested call leaves that summary incomplete. The write probe
retains positive register writes and STORE spaces recovered before that
unresolved edge. A STORE into a processor space used by an ABI output entry is
a possible write to every output in that space and vetoes the downgrade even
when no direct written range was recovered; incompleteness disables only
conclusions from an absent write.
With
`msvcstackguard on`, replay is followed by deletion as before; preservation
alone leaves every call and all cookie algebra in the output. This caller-side
proof is deliberately narrower than relaxing the body walk. All unseeded calls
and all non-output scratch registers remain unchanged. The production stage
control makes the checker write XMM0 before its nested call, and separately
consumes the upper half of a 16-byte XMM0 range after an exact cookie call, so
both the positive-write veto and the killed scratch flank are observable. A
focused predicate control supplies the other retained production-summary
shape: incomplete, no direct ranges, and an output-processor-space STORE.

Heritage can also refine the logical 8-byte floating output itself into two
adjacent 4-byte cells. The low cell characterizes as `ContainsJustified`, while
the high cell characterizes as `ContainsUnjustified` even though both bytes are
inside the same ABI `float8` output. At an exact seeded cookie site only, the
second cell may use the same caller-side proof when the *caller* has a locked
output whose storage contains that cell. This is not added to the generic body
proof: an inferred caller output cannot gain the cell, the XMM bytes above the
locked output still characterize as scratch and remain killed, and every
direct-write, output-space STORE, and explicit-effect veto above still wins.
Thus a declared `double score(...)` keeps both halves of its division across
the checker without treating an entire XMM register as preserved.

**Partial-range call overlap.** A heritaged range can be strictly *larger* than
the ABI storage it contains — the characterization is `ContainedBy` rather than
`ContainsJustified`, so none of the whole-range arms above apply. This is
routine on x86-64: SLEIGH models `PXOR`, `POR`, `PAND`, `MOVDQA`, `MOVDQU`,
`MOVQ`-to-xmm and `ORPD` as a single 128-bit write to the whole XMM register, so
the range is never partitioned by refinement (whose gate is `size > 4 && maxw <
size`) and the 8-byte parameter and return entries inside it are invisible.
Under `option calloverlap` two dedicated guards recover them.

On the input side (`heritage.rs (Heritage::guard_call_overlapping_input)`), the
biggest input entry contained in the range is located, its address translated
from the callee's to the caller's perspective, and a SUBPIECE inserted before
the CALL that truncates a fresh whole-range varnode down to that entry; the
truncated varnode is registered as an input trial and appended to the CALL.
Chapter 04's trial machinery then judges it exactly as it judges a whole-register
trial — the guard *proposes* storage, it does not assert an argument.

On the output side (`heritage.rs (Heritage::try_output_overlap_guard)` and
`Heritage::guard_output_overlap`), the biggest contained return entry becomes an
INDIRECT *creation* at the call, and the bytes of the range on either side of it
become further INDIRECT creations that are PIECEd back around it, so the range
as a whole still has a definition at the call while the return entry alone
carries the output trial. When that succeeds the range's effect is downgraded to
*unaffected* so no second guard fires over the same bytes. Note this is not the
same construction as the locked-stack-output case above: the register form makes
every piece an indirect creation, where the stack form pulls the flanking pieces
off a value that already existed before the call.

The level selects how much of that runs: at `off` both branches are inert and a
partial-range slice of an argument or return register at a call gets no guard,
which is what kuna shipped before the option — the observable symptom is a call
rendered with missing arguments and a return value read from a stale pre-call
definition. At `in` only the input guard runs, which recovers the argument but
leaves the return value stale; at `full` both run, which is upstream Ghidra's
behavior.

`heritage.rs
(Heritage::guard_returns)` symmetrically appends output-trial varnodes to every
live RETURN when the range overlaps the recovered return storage (truncating
via SUBPIECE when the range is bigger, `guard_returns_overlapping`), and — for
*persist* ranges (globals) — inserts an `addrforce` COPY of the range before
each RETURN (`return_copy`), which is precisely what keeps a global store's
def-chain alive through dead-code elimination so `glob = ...` survives to the
output.

**LOAD/STORE guards.** Ranges in the stack space can be aliased by indexed
LOADs/STOREs (`stack[i]`). Once per space per function
(`heritage.rs (Heritage::discover_indexed_stack_pointers)`), the engine walks
the stack-pointer input's descendant tree — accumulating constant `INT_ADD`
offsets, passing through COPY/INDIRECT/SEGMENTOP, and flagging any traversal of
a *non-constant* add or a MULTIEQUAL — and records a guard
(`heritage.rs (LoadGuard)`) for every LOAD/STORE reached on a flagged path,
marking the op `spacebase_ptr`. A guard is born covering the **entire space**
(`LoadGuard::set`: minimum 0, maximum the space's highest offset). A STORE
whose pointer is still a free varnode cannot be classified yet: it is
conservatively marked and queued (`heritage.rs (Heritage::protect_free_stores)`),
and after the pass completes the discovery re-runs and strips the spurious
INDIRECTs from any STORE that turned out not to need a guard
(`heritage.rs (Heritage::reprocess_free_stores)`).

After renaming completes each pass, the value-set analysis narrows every newly
discovered guard to a real `[min,max,step]` window
(`heritage.rs (Heritage::analyze_new_load_guards)`, gated by
`option loadguardrange`, default on): the guards' pointer Varnodes become the
sinks of a `ValueSetSolver` system (the solver itself — constraint
generation, weak topological ordering, widening — is chapter
[05](05-types.md)'s machinery in
`decompiler/crates/kuna-decomp/src/p5_types/rangeutil.rs (ValueSetSolver)`),
one cheap `WidenerNone` solve seeds each guard
(`heritage.rs (LoadGuard::establish_range)`: minimum from the stable range
bound or the pointer base, step recorded only when the partial analysis shows
consistent iteration), and if any guard is still unresolved a full
`WidenerFull` solve finalizes it (`heritage.rs (LoadGuard::finalize_range)`:
a converged range of size in `(1, 0xffffff)` locks the guard —
`analysis_state == 2` — with `highind`-grade min/max/step; a range that wraps
past the stack parameters falls back to the whole space). A range-locked
store guard is what chapter [06](06-variables-and-merge.md)'s
`MapState::addGuard` loops turn into a real array index bound, and the
narrowed windows also shrink the merge tier's untied-call intersection test
to the addresses the op can actually touch. With the option off, guards keep
the maximally conservative whole-space window and are never range-locked —
the pre-port behavior. The guards' consumers are `option indexaliasguard`
below, the merge tier's untied-call intersection test (chapter 06), and
`RuleIndirectCollapse`'s store-guard branch.

**Index-alias guards.** The last arm of `heritage.rs (Heritage::guard)` runs
only where a pointer can reach the range being heritaged — upstream's
`Architecture::highPtrPossible`, which is every space but the internal *unique*
one save for the ranges a compiler spec names in `<nohighptr>` (only the PIC
families do, and kuna does not read that element). `option indexaliasguard`
selects how much of that arm runs.

At `load`, `heritage.rs (Heritage::guard_loads)` walks the guard
list built above and, for every still-live LOAD guard whose `[min,max]` window
covers the range's address, inserts an `addrforce` `CPUI_COPY` of the range
immediately before that LOAD and records the COPY as a load-copy sink. The
point is liveness, not value: a frame slot written by a direct store and read
only through a pointer derived from it has, before this, no reader at all in
the SSA, so `ActionDeadCode` deletes the store and the emitted C declares a
stack array, walks it with a pointer loop and never initialises it. The COPY
gives the slot a reader exactly where the pointer is dereferenced. It is
artificial and does not survive: once the pass finishes,
`heritage.rs (Heritage::handle_new_load_copies)` traces each sink to the
address-forcing boundary ops, marks those outputs `addrforce` when they fall
inside a guarded window, and propagates every load-guard COPY away again —
so what reaches the output is the mark the COPY earned, not the COPY.

At `global`, the default, the LOAD guard runs and so does the global half of
upstream's STORE guard (`heritage.rs (Heritage::guard_global_stores)`): every
live STORE into the range's own space (`kuna_indexaliasguard.rs (stores_into)`)
gets an `indirect_store` `CPUI_INDIRECT` of the range in front of it, unless the
range is read-only or the STORE's address is the stack pointer plus a constant
(a push, a call's return address, a spill: frame memory, not a global). The
STOREs are gathered once per heritage pass and space. The `INDIRECT`'s output is `addrforce`, as `guard_calls`
makes an address-tied range's, so it is not dead code when the global is stored
again right after the pointer store: `x = gi + 1; *p = k; gi = x;` keeps the
version it defines, and the read of `gi` from before the store conflicts with
it, which is what makes that read print as its own variable
(`v1 = gi; *a0 = a1; gi = v1 + 1;`, where `gi += 1;` after the store read `gi`
again). Without the guard a pointer STORE has no effect on a global in the
SSA, so three things go wrong whenever the pointer may point at the global. A
load of the global after the store reads the version from before it, and copy
propagation prints whatever was stored there: `gi = a; *p = k; return gi / 16;`
printed `a0` for the load, `gi = 5;` folded the load to `return 0;`, and a value
the function computed printed again. A value read before the store and used
after it prints as a read of the global after the store
(`int a = gi; *p = k; return a;` printed `*a0 = a1; return gi;`), and three
reads around two stores collapsed into one (`return gi * 0x6f;`). And a store to
the global whose value a later call's `INDIRECT` took drifted to that call,
behind the pointer store (`*a1 = a2; gi = a0; touch();`). With the `INDIRECT`,
the load reads a new version, the earlier value conflicts with it in chapter
06's Cover tests and keeps a variable of its own, and the store stays where the
binary makes it because `RulePropagateCopy` does not move a stored value into
one of these `INDIRECT`s
(`kuna_indexaliasguard.rs (keeps_store_guard_input)`): taking it would kill
the global's `COPY`, and `Merge` would rebuild the store in front of the effect
op or join the value with the global, whose type it would then take. Nor does
it move a stored value into a `MULTIEQUAL` or `INDIRECT` when a LOAD that may
read the global (one from its space; at a constant address, one that overlaps
it) follows the store before the next write of the global, a `MULTIEQUAL` on
the global not counting as a write
(`kuna_indexaliasguard.rs (keeps_store_before_load)`). With the `COPY` dead the
store would print where chapter 06's merge puts the value: where it is
computed, at the end of the loop body, or at the next call, so
`for (...) { gi += b; s += *p; }` would print the store after the load, and
`gi = a; x = *p; touch();` printed `v1 = *a1; gi = a0; touch();`. The same
holds for a `COPY` of the global into its own storage, which is what splitting
a return block that two paths share makes of the global's `MULTIEQUAL`: in
`if (c) { gi = v; return *p; } return 0;` at `-O0` the store's value moved to
that `COPY`, after the load, and printed `v1 = *a1; gi = a2; return v1;`. Each guard
is a varnode of the global's HighVariable that every later pass visits, and a
run of data decoded as code, with hundreds of stores and of addresses it reads,
reached a hundred thousand of them and tripled the time of a whole firmware
decompile, so a function stops guarding further ranges once it holds
`GLOBAL_STORE_BUDGET` (1024) of them. Heritage records each global range it
saw and whether it guarded it (`heritage.rs (Heritage::global_store_guarded)`),
and the refusals above and chapter 06's `keeps_apart` act only on a global
whose range it guarded.
At 2048 the guards still cost crazyflie's `cf2.elf` 6% of its whole-binary
time, at 1024 about 1%, and 23 of 15,942 functions over 20 binaries print
differently at the lower budget, 12 of them in that firmware. Chapter 06 adds the mirror image at this level: a value loaded through a
pointer stays ahead of a store to a global
(`kuna_indexaliasguard.rs (load_crosses_global_store)`, `kuna_loadorder.rs`).

At `global` heritage also gives a global the LOAD guard
(`heritage.rs (Heritage::guard_global_loads)`). Upstream builds load guards only for indexed stack
accesses, so a pointer LOAD reads no global at all in the SSA: in
`gi = gi + 1; s += *p; gi = gi - 1;` inside a loop, or `gi = b; if (c) { t = *p
+ 1; gi = t; }`, nothing reads the first store's COPY, `ActionDeadCode` deletes
it, and with `p` aimed at `gi` the printed C loads the value from before the
store. For a persistent, writable range, the guard puts an `addrforce` COPY of
the range in front of every live LOAD from the range's space that may read it
(`kuna_indexaliasguard.rs (loads_from)`): the LOAD's address is not a constant
outside the range, and it is not the stack pointer or an x86 segment base
(`FS_OFFSET`, `GS_OFFSET`, through which the stack canary is read) plus a
constant, through COPYs and constant offsets. These COPYs are load-copy sinks
like the stack ones: `handle_new_load_copies` marks the stores that reach them
`addrforce` within the guarded range and propagates the COPYs away, so each
store that a LOAD may read is kept where the binary makes it. A range is
guarded only when it has at least two writes other than `INDIRECT`s, since a
single write already reaches the return guard. A range written in smaller
pieces instead marks its original real writes `addrforce`
(`kuna_partialglobalload.rs (apply)`), preserving each write's
address and width. The synthetic full-width PIECEs created by width
normalization are not forced: doing so would add a write of the whole range
beside the original partial write. This keeps both halves in
`gs.a = b; gs.b = c; x = *p; gs = 0`, so a pointer aimed at `gs` reads the
new halves. This partial-range correction covers LOADs whose pointer derives
from a nonpersistent function data input through SSA operations, including
register and stack-passed pointer arguments. Architectural spacebase inputs
such as the stack pointer do not supply data-input provenance. Definite frame
addresses through copies, offsets, indexed addressing and frame-only phis
also stop the walk, so a local array's scalar index cannot establish pointer
provenance. A frame-address cycle requires a witnessed spacebase; a phi with
an incoming pointer arm remains eligible through that arm. Memory-fetched
and call-created pointers retain their existing behavior pending separate
alias provenance recovery. Input provenance requires a reachable input in a
bounded SSA graph walk; a cycle alone supplies no input provenance, and a
walk exceeding 1024 varnodes leaves the existing behavior. Partial guards
resolve pointer bounds after SSA
renaming and skip a LOAD proven disjoint from the global: unsigned intervals through
copies, extensions, byte concatenations, bounded carry bits and arithmetic
without wrap expose constant pointer prefixes. Unknown operations, cycles,
wrap and the bounded walk conservatively keep the stores. A word-addressed
space or a pointer width different from its space's address width also keeps
the stores, without attempting a byte-interval disjointness proof. This avoids
retaining zero-page pointer construction when that pointer can only read a
disjoint table. The same load selection and budget apply to partial ranges; one
heritage pass stops guarding further ranges at `GLOBAL_LOAD_BUDGET` (4096)
reads. Two rules respect the marks so that a kept
store prints once. `RulePropagateCopy` does not move a forced store's value
into a COPY of the global into its own storage, which is what a `MULTIEQUAL`
becomes when a shared return block is split
(`kuna_indexaliasguard.rs (keeps_forced_self_copy)`), and `RuleMultiCollapse`
does not match a forced store by functional equality when it collapses a
global's `MULTIEQUAL` (`kuna_indexaliasguard.rs (keeps_forced_join)`); either
would rewrite the join into another store of the same value. A kept store
stays where the binary makes it, so a LOAD whose value is live across a store
to a global prints as its own statement ahead of it (chapter
[06](06-variables-and-merge.md), `check_implied_cover`).

At `full`, `heritage.rs (Heritage::guard_stores)` runs instead: every STORE
whose space is the range's space, or is the range's container while the STORE
is marked `spacebase_ptr`, gets an `indirect_store` `CPUI_INDIRECT` of the
range in front of it, frame slots included, with no budget and with
`RulePropagateCopy` free to move a stored value into the `INDIRECT`. That is
what upstream always does; it is not kuna's default because on a frame slot the
INDIRECT chain survives into the emitted C as write-backs of values a slot
already holds. The global LOAD guard runs at `full` too.

At `off` none of them runs, which is what kuna shipped before the option.

`option stackstoreguard` (default on) also enables STORE guards at
`indexaliasguard load` and `global` for constant-initialized stack slots. Before normalizing
partial reads, it checks for a constant write (through COPY, SUBPIECE or
integer extension) and the processor's stack space. Only byte STOREs already
recorded by indexed stack-pointer discovery qualify; globals, unknown pointers
and wider stores retain the explicit `full` policy. The gate lives in
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_stackstoreguard.rs`.
A heritage pass also leaves the whole frame unguarded when one of those byte
STOREs, or any other LOAD or STORE whose address comes from the stack pointer,
has an address that does not resolve to the stack base plus constants plus
indices, or chooses between more than eight stack addresses or ones more than
256 bytes apart (`kuna_stackstoreguard.rs (frame_unresolved)`, decided once per
pass): the stack layout of chapter 06 cannot keep such a frame's guarded slots
in the one local the store writes through. Typical cases are an OR-formed
address (`(&v1 | 4) + i`) and a pointer walk with a variable step. When no
earlier pass guarded the frame, the option is turned off for the whole function
(`heritage.rs (store_frame_resolves)`), which then decompiles exactly as with
`stackstoreguard off`.

This bounded policy prevents an initializer from flowing unchanged across a
byte-fill loop into a later direct byte or word read. Otherwise constant
folding can delete a reachable condition and side-effecting call. Guards
conservatively describe a possible write; they do not invent its value or
prove its range. General store-alias recovery remains outside this policy:
applying every indexed store guard changes parameter and aggregate recovery
in existing cases, so it is not enabled by this option.

Turning `stackstoreguard off` restores the previous load-only behavior without
changing the explicit `full` policy. `indexaliasguard off` suppresses both
families, including the narrow stack-store guard.

Spilled pointers to fixed stack offsets also need a temporary STORE guard: the
stack read can acquire its initializer during renaming, while the pointer
reload resolves only in the following rule pool.
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_spillstoreguard.rs` guards
STOREs rooted in a free stack-slot read identified by `protect_free_stores`.
After renaming, it follows only COPYs and constant additions to the input stack
base with the stack pointer's full width on byte-addressed stacks, and retains
only guards overlapping that proven destination. Wrapping pointer arithmetic
uses the stack mask, but a memory access crossing the address-space boundary
is left to the existing policy. It removes
guards on other slots, and all guards for pointers whose destination remains
unresolved, including MULTIEQUALs and call effects. Removal repeats until no
more destinations resolve, so an unrelated provisional guard cannot prevent
a later spilled pointer from resolving. Closed frames with at least sixteen
spills can prefilter ranges using a whole-function proof: every located write
to each pointer slot supplies the same exact affine address, all remaining
STORE destinations are exact and disjoint from these slots, and no call,
partial slot write, INDIRECT, or unresolved pointer can change the proof. The
postrename proof still decides every retained guard. Other frames admit the
entire new guard set only when the conservative product of distinct stack
ranges and spilled STOREs is at most 4096; larger frames retain the previous
default policy, with broad guarding available through `indexaliasguard full`.
The retained guards keep later reads
linked to the write through its conversion to a located COPY; they do not
change the general indexed-store policy. Like the other pointer guards, this
repair is disabled by `indexaliasguard off`.

**The dead-code delay machinery and the dead-definition gate.** Dead-code
removal is only *allowed* in a space once heritage there is past the space's
dead-code delay: `heritage.rs (Heritage::dead_removal_allowed)` is the gate
(`pass > deadcodedelay`), consumed by
`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs (ActionDeadCode)`
and by `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_1.rs
(RuleEarlyRemoval)` — the checked variant (`dead_removal_allowed_seen`) also
records that removal has now *happened* (`deadremoved`). The reason the gate
exists: a free varnode can surface in pass N+1 at an address already heritaged
in pass N — most commonly a stack location whose aliasing access only became
visible after the stack pointer renamed — and if dead code was already removed
there, its defining stores may be gone. When the driver detects exactly that
(an old-range overlap, `deadremoved > 0`), it fires
`heritage.rs (Heritage::bump_deadcode_delay)`: install `deadcodedelay + 1` for
the space as a **persistent Override**
(`decompiler/crates/kuna-decomp/src/p0_knowledge/overrides.rs
(Override::insert_deadcode_delay)` — it survives `Funcdata::clear`), set the
restart-pending flag, and let the outer drive re-flow the function (§0.6); the
restarted run re-applies the persisted delay to the fresh per-space info before
its first pass (`funcdata.rs (Funcdata::op_heritage_with_deadline)`), so dead
code now waits one pass longer and the aliased store survives. The bump is
self-limiting: if the Override already carries a delay for the space, the bump
is suppressed rather than re-requested — that suppression is what makes the
restart converge instead of looping. The bump machinery records both events into a
throwaway per-call `RestartLog`
(`decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_restartlog.rs
(RestartLog)`) that is dropped on return — diagnostic plumbing not yet wired to
the Architecture-owned log — and neither fires during a jump-table
sub-decompilation (the `is_jumptable_recovery_on` guards at the call sites —
the sub-query must not mutate P0, §0.7). The console `deadcode delay` command
exists but is an unwired stub (`kuna-console/src/ifacedecomp.rs
(IfcDeadcodedelay)` returns engine-unavailable); the only live writer of the
Override is `Heritage::bump_deadcode_delay`.

**Free-varnode failure mode.** After `remove_revisited_markers`, a free read
being guarded must have exactly one reader; a free varnode with multiple reads
is an IR invariant violation and `heritage.rs (Heritage::guard)` deliberately
panics carrying the upstream error text ("kuna heritage: Free varnode with
multiple reads") — the
drive catches it at the per-function boundary and degrades to that function's
error record, exactly the route the C++ `LowlevelError` takes. (The port
history briefly downgraded this throw to a skip; with call-argument def-chains
kept alive by dead-code marking it fires zero times across the corpus, and the
faithful throw is restored.)

Two kuna extensions ride on the pass boundary: the per-function watchdog
deadline is probed at each address-space iteration (§0.6 — a stripped-binary
non-convergence spends its time inside heritage, so the pass bails here rather
than at the next action boundary; the abandoned partial pass is never
rendered), and after every pass `ActionHeritage` runs the lowered-switch input
repair (`decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs
(Funcdata::kuna_repair_lowered_switch_inputs)`), which re-points a synthetic
lowered-switch BRANCHIND whose input heritage normalized away (chapter 02). The
repair accepts written, input, *or heritage-known* varnodes as healthy — the
last category is what ended the condconst-vs-repair tug-of-war that once kept
mainloop reporting one change forever on certain stripped binaries
(`tests/hang-repro/README.md`).

## 3.2 The rule pools

A `decompiler/crates/kuna-decomp/src/infra/action.rs (Rule)` is a stateless
pattern→rewrite unit: `get_op_list` declares the opcodes it can fire on
(defaulting to *all* opcodes), and `apply_op(op, data)` either returns 0 (no
match — every guard along the way simply declines) or performs its whole
rewrite and returns 1. Rules are owned by an
`decompiler/crates/kuna-decomp/src/infra/action.rs (ActionPool)`, which indexes
them at registration into a flat per-opcode table (`perop`, insertion order
preserved). One pool sweep visits every op in the function in sequence-number
order through a resumable cursor that survives op deletion (§0.3) — it is
recorded as the last *consumed* `SeqNum`, so the next op is the first optree key
strictly greater than it, which stays valid when the op it named is destroyed.
Resolving that key is an optree search, and the sweep is the decompiler's
innermost loop, so the advance resolves the successor's id and the cursor read
returns it rather than searching a second time for the op the advance already
found; the memo is dropped on every `apply` exit, so a resumed or interleaved
sweep re-searches. For each op the sweep walks its opcode's rule list in
registration order
(`action.rs (ActionPool::process_op)`): disabled rules are skipped (the
upstream `option togglerule` surface writes that per-rule flag), a rule that
fires bumps the pool's change count, a rule that kills the op ends the walk,
and a rule that *changes the op's opcode* rewinds the walk to index 0 of the
new opcode's list — rules see each other's effects mid-op, and that rewind
order is part of the observable output (§0.6). A rule that mutates without
returning 1 is an invariant violation the pool reports as an engine error
message rather than silently absorbing. The **local fixpoint** comes from the
scheduler, not the pool: every pool node carries the repeat flag, so
`action.rs (Action::perform)` re-sweeps the whole function until a sweep makes
no change. There is no bound on the number of sweeps — quiescence is the
contract, and the only backstop against a rule pair that feeds itself forever
is the (kuna) cooperative deadline probed every 1024 op-visits
(`action.rs (POOL_DEADLINE_STRIDE)`, §0.6); exactly one such oscillation has
occurred in kuna's history (the lowered-switch repair, §3.1), and it presented
as mainloop reporting one change per iteration for good.

Three pools exist in the `decompile` tree
(`decompiler/crates/kuna-decomp/src/infra/universalaction.rs
(universal_sched)`): **oppool1**, 141 registered rules, sits inside the
`stackstall` repeat-group in mainloop — the main simplification bag;
**oppool2**, 5 rules (`RulePushPtr`, `RuleStructOffset0`, `RulePtrArith`,
`RuleLoadVarnode`, `RuleStoreVarnode`), runs after block structuring in
mainloop's tail — the pointer-arithmetic and stack-variable forms that need
type recovery started and a stable block structure; and the **cleanup** pool,
22 rules, runs once-per-drive after fullloop exits — presentation-form
rewrites that must not perturb the analysis fixpoint. The architecture may
append CPU-specific rules to oppool1 (`universalaction.rs (build_universal_action)`
takes `extra_pool_rules`); the engine currently always passes an empty list
(`decompiler/crates/kuna-decomp/src/infra/architecture.rs
(Architecture::build_action)`).

**Ordered floating-point condition negation.** The shared
`decompiler/crates/kuna-num/src/opcodes.rs (get_booleanflip)` table returns no
single-opcode complement for `FLOAT_LESS` or `FLOAT_LESSEQUAL`: when either
operand is NaN, both `a < b` and `b <= a` are false. Consequently
`Funcdata::op_normalize_flip` leaves the branch flip in place and
`RuleCondNegate` inserts an explicit `BOOL_NEGATE`; `RuleBoolNegate` cannot
fold it back into a swapped ordered comparison. This preserves SSE MINSS's
source selection on unordered and equal operands, including signed zero.
Floating equality and inequality remain complements. This correctness rule
applies regardless of `nanignore`, whose separate policy can still discard
explicit NaN tests; `nanignore none` preserves their semantics.

`kuna_floatnegation.rs (fold_guarded_negate)` preserves the compact ordered
form when the expression itself excludes unordered inputs:
`!(NAN(a) || NAN(b) || a <= b)` is exactly `b < a`. It requires a matching
NaN guard for each operand, or a constant the processor's float format proves
is not NaN. Same-width copies are followed when matching operands and proving
constants. A less-than plus matching equality can supply the `<=` term.
Missing or unrelated guards and extra boolean terms decline the fold. Only
the consumer changes, so shared comparisons retain their meaning. A CBRANCH
consuming the guarded expression receives a new complementary comparison and
flips its condition flag, preserving both branch destinations.
`RuleBoolNegate` applies this exact fold; `RuleIgnoreNan` also checks the
bounded boolean fan-out first, before dropping guards could hide the proof.
Thus ordinary UCOMISS conditions keep their ordered forms without granting
an unguarded MINSS comparison a false complement.

The upstream rule set is ported across eight files in C++ definition order —
`ruleaction.cc` split at class boundaries. The map, by dominant theme (named
rules are representative, not exhaustive; a rule's registration row in
`universalaction.rs (universal_sched)` is the authority for its pool and
group):

| File | Theme | Representative rules |
|---|---|---|
| `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_1.rs` | dead-op pruning, term ordering, bit-mask algebra, SUBPIECE motion through phis/INDIRECTs | `RuleEarlyRemoval` (the all-opcode dead-op reaper, gated by §3.1's dead-definition gate), `RuleCollectTerms`, `RuleAndMask`/`RuleShiftBitops`, `RulePullsubMulti`/`RulePushMulti`, `RuleIntLessEqual` (§3.5 compareform), `RuleRangeMeld`, `RulePiece2Zext` |
| `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_2.rs` | logical ops through extensions/pieces, double-op fusion, zext elimination | `RuleAndCommute`, `RuleAndCompare`, `RuleDoubleShift`, `RuleConcatShift`, `RuleLeftRight`, `RuleZextEliminate`, `RuleBooleanUndistribute`, `RuleFloatRange` |
| `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_3.rs` | boolean normalization, phi/INDIRECT collapse, constant folding, reassociation | `RuleMultiCollapse`, `RuleIndirectCollapse`, `RuleCollapseConstants` (the OpBehavior constant evaluator), `RulePropagateCopy`, `RuleAddMultCollapse`, `RuleSborrow`, `RuleShift2Mult` |
| `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_4.rs` | the SUBPIECE/ZEXT/CONCAT commuting family, piece reassembly, stack-var promotion | `RuleSubCommute`, `RuleConcatZext`, `RuleSubCancel`, `RuleHumptyDumpty`/`RuleDumptyHump`, `RuleLoadVarnode`/`RuleStoreVarnode` (oppool2, group `stackvars`), `RuleSwitchSingle`, `RuleCondNegate` |
| `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_5.rs` | comparisons against extremal constants, equation solving, the pointer-recovery trio | `RuleLess2Zero`, `RuleSLess2Zero`, `RuleEqual2Constant`, and oppool2's `RulePtrArith`/`RuleStructOffset0`/`RulePushPtr` (all no-ops until `ActionStartTypes` flips `has_type_recovery_started` — chapter 05 owns what they build) |
| `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_6.rs` | pointer-op undo, division strength-reduction inversion, cleanup arithmetic | `RulePtraddUndo`/`RulePtrsubUndo`, `RuleDivOpt`/`RuleDivTermAdd`/`RuleSubNormal` (recover `/`, `%` from magic-number multiplies), cleanup-pool `RuleMultNegOne`/`RuleAddUnsigned`/`RuleSubRight`/`RulePieceStructure` |
| `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_7.rs` | signed div/mod idioms, segments, pointer flow, predication, float compares | `RuleSignDiv2`, `RuleSignMod2nOpt`, `RuleModOpt`, `RuleSegment`, `RulePtrFlow`, `RuleConditionalMove` (group `conditionalexe`), `RuleFloatCast`, `RuleIgnoreNan` |
| `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_8.rs` | int↔float conversion recovery, bit-counting booleans, float sign ops, compare splitting | `RuleUnsigned2Float`, `RuleThreeWayCompare`, `RulePopcountBoolXor`, `RuleLzcountShiftBool`, `RuleFloatSign`, `RuleOrCompare`, `RuleFuncPtrEncoding`, cleanup-pool `RuleExpandLoad` |

`RulePullsubIndirect`
(`decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_1.rs (RulePullsubIndirect)`)
narrows an INDIRECT whose readers are all SUBPIECEs to the bytes they use, from
the lowest used byte to the highest. The new output piece starts at the lowest
used byte, and kuna slices the piece of the INDIRECT's input at that same byte,
as `RulePullsubMulti` does. Upstream slices the input at the offset of whichever
SUBPIECE triggered the rule, so when another reader uses a lower byte the input
piece holds the wrong bytes; on a big-endian space it also sits below the
original storage, and the emitted C copies a value that does not exist into the
read bytes. The stack STORE guards of §3.1 (`stackstoreguard`) are the common
source of such INDIRECTs: two direct byte reads of an initialized buffer after
an indexed byte store.

With `stackstoreguard` on, the rule leaves a stack STORE INDIRECT whole when one
of its SUBPIECE readers reads more than one byte (a SUBPIECE that only feeds
another INDIRECT, left behind by an earlier narrowing, does not count). The
narrowed piece would be a separate stack location from the slot the store
indexes, so the variable map would declare it as its own local: the read would
no longer see the store, and the store's base would become an address-only
local declared at pointer width and printed with that stride
(`long v1; (&v1)[i & 3] = j; return v2;`). Kept whole, the read stays a piece
of the slot the store indexes (`unsigned int v1; ((char *)&v1)[i] = j; return
v1._1_2_;`). Byte readers still narrow, because a byte piece maps as an element
of the byte array the store indexes. The check is `keeps_store_indirect_whole`
in `decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_stackstoreguard.rs`.
Keeping the slot whole does not by itself make the slot, its neighbours and the
read one local; the stack layout decides that, and with the option on it maps
every slot a bounded store may reach as one array and does not end an open range
inside a slot it absorbed (chapter 06, §6.2, "An indexed store's slots are one
local").

The pointer/division family in `ruleaction_6.rs` resolves opcode changes through
the canonical `TypeOp` table and applies them with `Funcdata::op_set_opcode`.
That mutation is infallible; its helper returns no `Result`, so rules do not
carry unreachable failure branches around it. Fresh unique outputs use
`Funcdata::new_unique_out` directly and retain their allocation-error handling.
These ownership changes do not alter rule guards, opcode flags, or rewrite order.

The boolean/arithmetic and bit-piece families in `ruleaction_3.rs` and
`ruleaction_4.rs` also allocate unique outputs through `Funcdata::new_unique_out`.
New outputs therefore receive high variables when high-level state is enabled,
including public rule calls after that transition, and matching lane-storage
records while lane collection is active. Rule matching and graph-edit order are
unchanged. Addressed outputs in `ruleaction_4.rs` likewise use
`Funcdata::new_varnode_out`: store promotion and extension shortening retain
the shared factory's high-variable, lane-storage and scope-property bookkeeping.
Shortening still adjusts the address for endianness and unsets the prior output
before allocating its replacement. Output reassignment uses
`Funcdata::op_set_output`, preserving old-definition unlinking and the bank's
reader-replacement callback. The shared property update supplies missing covers
in high-level state and marks high-variable cover information dirty. This also
applies when `RuleSubZext` turns a middle truncation, with or without a following
shift, into a full-width shift and mask.

For the 64-bit unsigned divide-by-three reciprocal, GCC can share one wide
multiply between the quotient and remainder. After `RuleDivOpt` recovers
`x / 3`, the sibling `(high64(x * 0xaaaaaaaaaaaaaaab) & ~1)` is exactly twice
that quotient. `RuleDivOpt` substitutes `(x / 3) * 2` only when the same
`x / 3` node precedes it in the same basic block; the ordinary term collection
and `RuleModOpt` rules can then recover `x % 3`. Other masks, reciprocals, widths, wide divisions, and
multiply-without-a-matching-quotient forms are declined.

The cleanup pool's `RuleExpandLoad` is upstream's rewrite of a LOAD narrower
than its pointer's pointee into a LOAD of the whole pointee, with the loaded
bytes taken back out either as the least-significant truncation or inside the
`(V & C) == D` compares that are the load's only uses. Both forms read bytes the
program never reads, and a program may pass an object that ends at the last
byte it reads. A record parameter whose 4-byte field at `+8` is read whole on
one path and by its low two bytes (`movzwl 8(%rdi)`) on another printed the
second read as `(unsigned short)a0->field_0x8`, and a byte test of `+9` as
`(a0->field_0x8 & 0x2000) != 0`; compiled, each reads four bytes and faults
when the caller's object ends at `+10` at the end of a page. The same held for a
2-byte read at `+4` through a pointer a callee reads as `unsigned int *`
(`(unsigned short)a0[1]`), for the low half of the `long` one past a walk
(`(int)a0[a1]`), and for a masked byte of an 8-byte element (`(q[1] & 0x30) !=
0x10`).

kuna never takes the truncation form: the read prints at its own width and
offset, `*(unsigned short *)&a0->field_0x8`, with the one cast the truncation
had. The AND form is taken only when the widened pointer is a field of a record
or union that a declaration laid out -- DWARF, a parsed header, a libc layout --
and not one `structsynth` minted, and only when that pointer reaches a Varnode
whose type is locked (a declared prototype's parameter, a declared global)
through pointer arithmetic, copies and casts alone
(`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_narrowload.rs (widens_into_declared_field)`).
There the declaration attests the whole field, and the compare can name the
field's own enum constants, `(p->flagfield & (HIGH_2|HIGH_1)) != 0`, which a
byte of the field cannot; that is what `enum.xml` #4 pins. A bare pointer's
target is attested only at the bytes the program reads, even when the pointer
is declared, and a synthesized record attests only the accesses it was built
from, so both keep the narrow read,
`((unsigned char *)&a0->field_0x8)[1] & 0x20`. So does a declared record that a
call returns, that a loop's phi carries, or whose pointer is read out of
memory: its type comes from propagation, not from a declaration of that
pointer, and `(src(k)->flags & 0x8100) == 0x8000` for a byte test of a 10-byte
object at a page end faults. A STORE was never widened:
`TypeOpStore::getInputCast` casts the pointer of a store narrower than its
pointee.

Over the 45 castbench binaries (stripped) the rule fired at 82 loads in 30
functions (852 in 276 with `elemptr` off, which had kept an inferred pointee's
loads narrow); it now fires at none, and the datatest corpus does not move. On
18 DWARF builds (coreutils, findutils, grep, tar, iproute2, gzip, diffutils,
bzip2, shadow; tar, grep and ip at O2 and O2-noinline) the truncation form fired
at 177 loads and now at none; the AND form still fires at 182 loads in 81
functions, exactly the loads it widened before. Those are the residual: a
declared record's field reached from a declared parameter or global, such as a
DWARF-typed `r` in `(r->flags & 0x81) == 0x80`. The printed C reads the
whole declared field there, which faults only for an object shorter than its
declared type. `kuna-narrowload.xml` pins the narrow spellings, the declared
parameter's whole-field compare, and the narrow read through a call's result
and a loop's phi. `decompile_all_cli.rs`'s
`a_narrow_read_round_trips_through_the_printed_c` and
`a_narrow_read_of_a_declared_record_round_trips_through_the_printed_c` (the
`-g` builds) run the printed functions on objects that end at a page boundary.
`kuna-tiedphitrim.xml` #13/#14 and `structsynth-locals.xml` #2 pin the narrow
spelling of a 4-byte field read through an inferred `int8 *`.

Under `namestyle angr`, copy propagation also preserves the eliminated
destination's machine storage on the unchanged source value, except for a
register copy used only to supply outgoing call arguments. These per-function
facts are diagnostics only: arithmetic operands and a load's address never
become the result's storage. Same-width replacement and cloning carry the facts;
dead Varnodes and a cleared function release them. P9 uses them to annotate a
local even when a SLEIGH temporary's copy into a register has disappeared.
An outgoing-argument transport proof survives removal of its readers, but a
different definition assigned to the same Varnode clears that proof.
Retained source facts identify their machine write. If stack heritage later
reveals that a load was an outgoing reload of an existing local, its register
fact can be withdrawn without removing a different write to that register.

**Keeping a frame store that only a marker still reads** (`option tiedstorekeep`,
default on). `RulePropagateCopy` rewrites a reader of a `COPY` output to read the
`COPY`'s input instead. When the reader is an ordinary op that is pure gain: the
value is the same, and the `COPY` stays alive for whoever else reads its
location. When the reader is a **marker** — an `INDIRECT` guarding an
address-tied range across a call (§3.1), or a `MULTIEQUAL` at a join — it is
not, because markers never print. Once the marker has swallowed the last
remaining reader, an address-tied `COPY` has no descendants at all and is reaped
as dead, and with it goes the only statement that said where the location's
value came from. `Merge` normally conceals that (chapter 06): it merges the
source's HighVariable into the tied location's, so both print under one name and
the store reads as an assignment to that name. When the merge is DECLINED —
covers intersect — nothing repairs it, and the local's last printed assignment
is whatever preceded the store, typically its initialiser. Upstream already
refuses the propagation when the `COPY` output is `addrforce` ("don't propagate
if we are keeping the `COPY` anyway"), but `addrforce` is set only on heritage's
own guard outputs (§3.1), never on an ordinary frame store. kuna widens that
refusal by one case: the marker is about to take the **last** reader of a
non-`persist` address-tied `COPY` whose input is not itself address-tied and
whose value comes from a call — a `CALL`/`CALLIND`/`CALLOTHER` output, or the
`INDIRECT` that carries the return register across the call site before chapter
04's output promotion rewrites it. Propagating there buys nothing, since the
marker is invisible either way, and costs the store. Every other propagation is
untouched, including into a marker that is not the last reader, out of a
constant, out of a same-location copy, and into any marker over a `persist`
global — a global already has heritage's persist `RETURN-COPY` (§3.1) keeping
its last store printed, so the brake has nothing to add there. `option
tiedstorekeep off` restores upstream's behavior exactly.

**Keeping a pointer stored to a global out of the global's markers**
(`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_pointeestorekeep.rs
(declines)`, a strict fix, no option, issue #767). When a register value is
stored to a persistent global and the value is also used as an address (the
address of a `LOAD` or `STORE`, or the base of a `PTRADD` or `PTRSUB`, directly
or through the `+` and `-` that offset it, following the `COPY`s, `INDIRECT`s
and `MULTIEQUAL`s that carry it unchanged), `RulePropagateCopy` leaves the
store's `COPY` as the input of the global's marker (a call's `INDIRECT`, a join's
`MULTIEQUAL`) and of a `COPY` into the same global. Without that the `COPY`
dies, chapter 06's forced marker merge joins the value with the global, and
`gc = q; x = q[1] + q[2]; touch();` printed `gc = &a0[a1]; v1 = gc[2];`: a
dereference in the pointee type of a global kuna never declares. The walk runs
before types exist, when `q + 1` is still an integer add, so every add on the way
to an address counts; after 256 varnodes it answers yes. Two stores keep
upstream's propagation. A value that is only the global read back through
`COPY`s, `INDIRECT`s and `MULTIEQUAL`s prints as the global because it is the
global. And when an earlier value of the global is still read after the store,
directly or through a copy into a register, the forced merge keeps the value
apart by trimming anyway, while keeping the store in place would make chapter 06
copy that earlier value at its definition, possibly above a pointer store that
changes the global (dash's `set_curjob` unlinks the list head through such a
pointer and then reads the head again). An earlier value also counts as read
after the store when the global's own `MULTIEQUAL` reads it on an edge from a
block the store dominates, or an `INDIRECT` after the store reads it
(`(old_value_read_after)`). Heritage never builds that; it is what remains when a
pointer `STORE` becomes the global's `COPY` after the global's heritage, and
keeping the store there made Merge restore the earlier value with copies (a
flex scanner's `yylex` at `-O2`).

The same function covers the loads the binary makes. Any other reader of the
global's `COPY` output is a load of the global after the store (`-O0` reloads a
global for every use). When heritage puts no `INDIRECT` on a global at a
pointer `STORE` (`indexaliasguard load`, or a function past the STORE guard's
budget) that load reads the `COPY` even
when a `STORE` between them may have changed the global, and rewritten to read
the stored value it prints the register where the binary reads memory: once the
value is kept apart from the global, `gi = q; x = q[2]; *pp = p; return gi[1] + x;`
printed `return v2 + v1[1];`, which returns 72 instead of 52 when `pp` points at
`gi`. So when the stored value is used as an address, `RulePropagateCopy` leaves
a load reading the global when a `STORE`, or a call heritage gave no `INDIRECT`
on the global, lies on a path from the store to the load (`(written_between)`,
walked backward from the load; past 2048 operations, or at a block the store
does not reach, the answer is yes), whether or not the load is used as an
address: `*a2 = a0; return v1 + gi[1];` at `-O0` and `-O2`, and
`*a2 = a0; return (gd - a0) + (long)v2;` for a load that is subtracted. Only a
store whose earlier value the global's own marker carries past it
(`(old_value_read_after)`, above) keeps upstream's propagation of its loads; an earlier
value read after the store through a register does not, so
`old = gi; gi = q; ... *pp = p; return gi[1] + x + old[0];` keeps its load.
`RulePushMulti` leaves a global's `MULTIEQUAL` in place when one of its inputs
is a store kept for the global's markers (`(keeps_join)`), rather than replace
it with an existing join of the stored values: at `-O0`,
`if (c) { q = p + k; gi = q; } else { q = p + k + 1; gi = q; }` stores a frame
variable's value in both arms, and the replacement made the load after
`*pp = p` read the frame variable. A register copy of
such a load is the same read: `RulePropagateCopy` does not put the global back
into a reader of the copy when a write lies between the copy and the reader, and
chapter 06 keeps the copy as its own variable, so
`r = gi; *pp2 = p + 1; return r[1];` keeps `r` where the binary loads it rather
than reading `gi` after the second store.

These questions are asked again every time the rule pool visits a reader
(issue #818). One search for an earlier value read after the store first sets
aside every value of the global written at or after the store. Each of those
questions walks up the dominator tree plainly for eight blocks
(`decompiler/crates/kuna-decomp/src/substrate/block.rs
(BlockGraph::dominates_within)`), and only a walk still going after that asks
one memo for the store's block (`block.rs (BlockGraph::dominates_memo)`), which
keeps the answer for every block it passes, so a long chain of blocks below the
store is climbed once rather than once per value, and a short walk pays nothing
for the memo. Only the earlier values' readers are read after that, and
each `MULTIEQUAL` among them is read once, against the earlier values that list
it, rather than once for every value it joins
(`kuna_pointeestorekeep.rs (Store::join_after)`); the walk through register
copies stays bounded by `WALK_BOUND` readers per value. Past the plain steps,
one search visits each block of the dominator tree at most once, and it
otherwise costs time linear in the global's values, the earlier values' readers
and those joins' inputs; but the rule pool still asks it once per reader, so the
total grows faster than the function. The answers are
the ones the per-value walk gave, and `RulePropagateCopy` still asks this
question before `kuna_globalstorekeep`'s below, for every op, so a marker this
one keeps never reaches that walk or its marking. Before this a function
storing one pointer to a global 600 times, with a pointer store between, took
about 38 s to decompile where it had taken 16 s before the store was kept.

When the stored value is not used as an address it joins the global in chapter
06, and a load that uses what it reads as an address, in the slot above or
through a `COPY` or an add leading to one, keeps reading the global, so the
dereference prints through the global the binary reads rather than through the
register it stored: `*--line_num_start = '1'` still prints as
`line_num_start = &line_num_start[-1]; *line_num_start = '1';` at `-O0`, and
`gi = &a0[a1]; *a2 = a0; return gi[1];` keeps its load where kuna printed
`return (&a0[a1])[1];`. Other loads of such a value, and the stores of a
parameter, a frame variable or a constant, are left to the STORE guard above:
at `indexaliasguard load` a load after a pointer store still prints the stored
value there (issue #792).

**Keeping a loop counter's write-back** (`option loopcounterstore`,
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_loopcounterstore.rs
(declines)`, default-on) is the same refusal for the same op, on a different
shape. A counter the compiler keeps in a frame slot is read, bumped in a
register and stored back — `MOV EAX,[RSP+0x84]` / `INC EAX` /
`MOV [RSP+0x84],EAX` — so after heritage the write-back is an address-tied
`COPY` whose only reader is the loop header's `MULTIEQUAL`, and that
`MULTIEQUAL`'s output is *the same frame slot*. Propagating the register into
that phi cannot make the slot's definition printable, because the phi already
names the slot; it can only orphan the `COPY`, which then dies as dead. What
prints instead is the register's own definition under the register's
HighVariable, so the emitted loop initialises and tests one variable and assigns
its increment to another — `for (v55 = 0; v55 <= 4; v7 = v55 + 1)`, an induction
variable that is never updated and a loop that cannot terminate. kuna refuses
the propagation when all of the following hold: the marker is a `MULTIEQUAL`
whose address-tied output has exactly the storage — address and size — of the
`COPY`'s output; the `COPY`'s input is a **register**, not a `unique`; the
stored value is computed from that same `MULTIEQUAL` output, following `COPY`
chains, which is what makes it a counter rather than an unrelated value that
happens to land in the slot; and the marker is the last reader of a non-`persist`
address-tied `COPY` whose input is not itself address-tied. The register clause
is what separates `MOV EAX,[m]` / `INC EAX` / `MOV [m],EAX` from
`ADD dword [m],1`, whose intermediate is a lifter temporary that never becomes a
named variable and whose store `Merge` always folds; keeping the temporary's
store alive instead displaces the loop's iterator statement and costs the `for`
form. The `COPY`-chain walk is not cosmetic either: the rule pool decides for
itself whether the counter's load side or its write-back is folded first, so a
predicate that only accepted a direct reference to the phi output would fire or
not depending on that order. `tiedstorekeep` does not reach this shape — its
predicate requires the stored value to come from a call, and a counter bump is an
`INT_ADD` — and `option loopcounterstore off` restores upstream's behavior
exactly. One printing consequence travels with the brake. Where `Merge` would
have succeeded anyway the kept `COPY` becomes the statement's root op, with the
arithmetic hanging off it as an implied expression, and chapter 09's in-place
render (`option inplaceops`, DIV-36) matches only a bare two-input op — so a
loop that was never broken came out as `v = v + 1` rather than `v += 1`. The
renderer therefore looks through a `COPY` of an **implied** two-input value and
decides on the inner op; a `COPY` of an *explicit* value is left alone, because
there the statement really is `out = <that name>`.

**Keeping a computed global store on its path**
(`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_globalorderkeep.rs
(declines)`, a strict fix under `indexaliasguard global`) prevents a forced
marker merge from bypassing chapter 06's global-write ordering check.
`RulePropagateCopy` leaves a guarded persistent COPY as the input of a marker
whose output has the same address and size when that COPY's value is computed
in another basic block, or across an observation in the same block. The shared
`kuna_globalorder.rs (write_moves)` predicate defines those barriers. The
value must be an ordinary computed temporary, with LOADs, calls, markers and
plain COPY roots left to the existing guards. Inputs, constants, address-tied
or persistent values and marked global loads are excluded, as are return
copies and stores with an earlier global value still read afterward, using
`(old_value_read_after)` below. A Clang `-O0` switch
computes a masked selector before dispatch and writes it to a volatile global
only in its case blocks. Propagating those writes into the return phi deleted
their COPYs and forced the selector to merge with the global, printing an
extra store before dispatch and reading the volatile global back in the cases.
Keeping the copies lets chapter 06 leave the selector separate and each store
on the path where the binary makes it.

**Keeping a global store whose value is read sign-sensitively**
(`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_globalstorekeep.rs
(declines)`, a strict fix, no option) is the same refusal once more, for a
persistent global. When a register value is stored to a global and an operation
also reads that value where its declared signedness decides the result — a
`>>`, a divide, remainder or ordered compare, an extension, an integer-to-float
conversion, or a sub-`int` `==`/`!=` that is not against a constant with the
operand's top bit clear — `RulePropagateCopy`
leaves the store's `COPY` as the input of the global's own markers, of a
`COPY` into the same global (what a duplicated join block leaves of the
global's `MULTIEQUAL`), and of every load of the global. The value is everything chapter 06 would join with it:
it is followed through the `COPY`s, `INDIRECT`s and `MULTIEQUAL`s that carry it
unchanged, in both directions, so a reader of a stack reload at `-O0` or of a
join counts. An expression computed from it is followed forward too when C
gives the result the operand's type: `+`, `-`, `*`, the bitwise operators, `~`,
unary `-` and the shifted operand of `<<` compute the same bits whatever the
signedness, but `sink + 1 >> 4` shifts the way `sink` is declared, so a
sign-sensitive reader of `u + 1` counts as a reader of `u`. Globals and
constants end the walk, and a walk that visits more than 256 varnodes answers
yes, which keeps the store and every load where the binary makes them.

The decision cannot wait for the other rules: once the `COPY` is gone, chapter
06's join of the value into the global's marker is forced. So the walk also
counts a reader that a rule running later turns sign-sensitive. A fold moves a
compare's constant onto the value: `RuleEqual2Constant`, `RuleEqual2Zero`,
`RuleXorCollapse` and `RuleShiftCompare` rewrite `u + 1 == 0` as `u == 0xffff`,
and do the same across `-`, `*`, `^`, `~`, unary `-` and `<<`. A sub-`int`
`==`/`!=` on an expression computed through one of those therefore counts
whatever its constant; through `&` and `|` alone no rule moves the constant,
so there it counts only as above. `RuleCarryElim` turns a carry into an ordered
compare (`carry(u, c)` is `-c <= u`), `RuleAndZext` turns the low half of a
concatenation into a zero extension, and `RuleRangeMeld` merges two compares of
the value against constants that a boolean `&&`/`||` (or a `&`/`|` of the two
results) combines into one ordered compare when their ranges join into one
(`u == 0 || u == 1` is `u < 2`; `u == 10 || u == -1` stays). All three count;
for the last the walk asks the rule's own question, pulling both compares back
to the value and combining their ranges. Every other rule that creates a sign-sensitive
operation (the divide, remainder and sign-test recognizers, the float
conversions, `RuleSborrow`, `RuleScarry`, `RuleSignShift`, `RuleTestSign`,
`RuleZextCommute`) starts from a shift, an extension, a divide or an ordered
compare that the walk already counts, and the rules that turn an ordered compare
into `==` only remove a reader.

Without the refusal the `COPY` dies, the store survives only as
chapter 06's join of the value into the global, and the reader prints as a read
of the global in the global's signedness; with it, chapter 06 keeps the value
apart and the `COPY` prints at the binary's own store, ahead of any later
pointer store or call.

The refusal holds only while no earlier value of the global is still used after
the store (`old_value_read_after`). Where heritage gives a pointer `STORE` no
effect on a global (`indexaliasguard load`, or a function past the STORE
guard's budget; at the default `global` level the load reads the `STORE`'s
`INDIRECT`, and the copy lands after the store), in `*p = k; old = g; n = old * 3 + inc; if (n >> 20)
return 7; g = n; return old == 5 ? 1 : old;` the binary's load of `old` after
`*p = k` reads the version of `g` the function entered with, and that version is
still used after `g = n`. With the store's `COPY` kept, both versions are live
after the store, and chapter 06 has to give the earlier one its own variable: it
copies it right where that version is defined, here the function entry (a call's
`INDIRECT` or a loop head's `MULTIEQUAL` elsewhere), above `*p = k`, so the
printed C reads the global before the pointer store may change it. Without the
refusal the `COPY` dies, the forced merge of the global's marker finds the stored
value and the earlier one live together and trims the stored value to its own
variable, and every read of the global prints after the pointer store, as
upstream prints it. A use counts when some path from the store reaches it
without passing the earlier version's definition, which only a version defined
in a block that dominates the store can have, since a definition dominates its
uses; a `MULTIEQUAL` uses its input at the end of the predecessor block it
comes from, and a use of a register `COPY` of the earlier version counts too,
since a later propagation folds the copy into it. A walk past 256 copies or
65,536 blocks and operations answers yes.
Placing the copy of the earlier value at the binary's own load, which would let
the store stay, is not done.

A load is any other reader of the store's `COPY`: an operation that writes
something other than the global itself (a `PIECE` that joins the stored part
into the whole of a wider global is left to upstream). kuna's heritage puts no
`INDIRECT` on a global at a pointer `STORE`, so after `gi = u; *p = k;` the
binary's load of `gi` is still the store's `COPY`, and propagating `u` into it
would stand the value in for memory that `*p` may have changed. While the
refusal above holds, the load keeps the `COPY` too, so it prints as the
global (`gi = a0 * 3; *a1 = a2; ... if (gi <= -1)`) while the value's own uses
keep the value. A load the refusal lets through marks the value and the store
(the `global_load` bit of the varnode's additional flags); the mark follows the
value into the global's markers and later stores, a marked store takes
upstream's propagation from then on, and chapter 06 never keeps a marked value
apart, so the load prints as the global exactly as before. A parameter's store
is left to upstream, since a parameter never merges with a global, and so is a
constant's. Every other propagation is upstream's.

**Resolving a direct store's effects**
(`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_storecopyeffects.rs`,
a strict fix, no option). A STORE can become a direct COPY only after a
spilled address is recovered. Its existing INDIRECT guards must then read the
new write immediately: waiting for `RuleIndirectCollapse` lets the next
`ActionDeadCode` delete the otherwise unread COPY before that rule runs. For
example, AArch64 clang `-O0` spills the address in `gi = b; touch(); gi = 9;
touch();`; without this repair the second store disappears.

`RuleStoreVarnode` resolves the contiguous guards that explicitly reference
that STORE as soon as it converts it. An equal range becomes a COPY of the
write; a contained range reads its own bytes; a partially overlapping range
joins the new bytes with the unaffected bytes of its previous value. Byte
selection and concatenation follow the address space's endianness. The
replacement computations are placed after the direct write and preserve the
guard's output, so existing call, load, return, and join uses receive the new
value. A disjoint guard is replaced by its previous input. Guards for another
operation are untouched, and a write with no surviving uses can still be
removed as dead.

**Retyping an op mid-rule.** A rule that rewrites an op in place usually changes
its op-code, and the op-code is not just a tag: `set_opcode` caches the
op-code's *property word* (`unary`/`binary`/`booloutput`/`commutative`/`marker`/
… ) into the op's flags, and every later guard — `is_bool_output`,
`is_commutative`, the pool's eval-type dispatch — reads it back off the op. The
upstream `Funcdata::opSetOpcode` therefore takes a bare op-code and looks up the
architecture's singleton property record (`glb->inst[opc]`); kuna's
`Funcdata::op_set_opcode` takes the already-resolved record, so each rule file
resolves it at the call site. Every one of those call sites goes through the
single canonical port of that table,
`decompiler/crates/kuna-decomp/src/p5_types/typeop.rs (seam_type_op_for)`, whose
per-op-code rows are transcribed field-for-field from the upstream `typeop.cc`
constructors. The seam is **total**: the table's `match` carries no wildcard arm
(so a new op-code cannot enter the enum without the compiler demanding its row),
every registered op-code answers with real property bits, and the one value with
no upstream record — the `CPUI_MAX` sentinel, which is not an operation — yields
a property-less skeleton rather than aborting. This totality is load-bearing
rather than cosmetic: the rule files previously each kept their own partial
whitelist of "op-codes this batch emits" with a `panic!` default arm, and the
copies drifted apart, so a rule that legitimately produced an op-code its file
had not enumerated (`INT_SRIGHT` out of `RuleBitUndistribute`, or a
`FLOAT_INT2FLOAT`/`FLOAT_LESS`/`FLOAT_ADD` phi collapsing through
`RuleMultiCollapse`) unwound the entire decompilation — the caller saw one error
record and no C at all for that function.

**Lowering a non-least-significant truncation.** A SUBPIECE carries a byte
offset, and only the offset-0 form has a C spelling: it is a cast. Every other
offset is a p-code-level slice with no operator in the language, so the printer
falls back to rendering the operation itself — `SUB81(v,7)`, an identifier no
emitted header declares. `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_6.rs
(RuleSubRight)` (cleanup pool, registered `subright`) is what keeps that
fallback unreachable in practice: it rewrites `sub(V,c)` into
`sub(V >> c*8, 0)`, synthesizing an `INT_RIGHT` by `c*8` bits ahead of the
SUBPIECE and zeroing the SUBPIECE's offset, so the result prints as the cast of
a shift — the ordinary arithmetic the source wrote. The shift's temporary is
typed `TYPE_UINT` at the input's width so the shift renders unsigned. Three
cases decline. `c == 0` is already least-significant and needs nothing. A
truncation whose input carries a composite (struct/union/array) read-facing
type is marked for special printing instead and rendered as a field extraction
(`sym._2_1_`), because there the slice *is* the source-level operation. And
when output and input are both address-tied and overlap at exactly `c`, the
SUBPIECE is a storage marker that
`decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs
(ActionCopyMarker)` will convert,
so rewriting it would destroy the partial-symbol rendering. One refinement
folds a level away: if the SUBPIECE takes the *high* end of its input
(`outsize + c == insize`) and its **only** reader is an `INT_RIGHT`/`INT_SRIGHT`
by a constant, the two shifts are lumped into one — the reader becomes the
least-significant SUBPIECE and the synthesized shift carries `c*8` plus the
reader's amount (so a 4-byte-offset SUBPIECE feeding `>> 5` becomes a single
`>> 0x25`). A lumped `INT_RIGHT` whose combined amount reaches the input width
would evaluate to zero and is declined outright; the arithmetic form clamps to
the sign bit instead, since that is a sign extraction.

Rules registered in the pools but implemented elsewhere: the sub-variable
triggers and split rules (§3.3, `subflow.rs`), `RuleOrPredicate` (§3.4,
`condexe.rs`), the kuna gated rules (§3.5), the double-precision family
(`decompiler/crates/kuna-decomp/src/p5_types/double.rs`, chapter 05), the
constant-sequence and bit-field cleanup rules
(`decompiler/crates/kuna-decomp/src/p5_types/constseq.rs`,
`decompiler/crates/kuna-decomp/src/p5_types/bitfield.rs`, chapter 05), and the
stack-probe-loop phi resolver
(`decompiler/crates/kuna-decomp/src/p2_lift/kuna_stackprobeloop.rs`, chapter
02). A note on the files themselves: their module headers still carry the
port-wave `STUB(...)` inventory from the mid-port merge; the live registration
and rule bodies are complete (the tree's action listing is byte-equal to the
C++ oracle dump, §0.6) — trust the code, not the header prose.

**Erasing the ISA-mode / alignment encoding on an indirect-call target.** On
processors that steal the low bits of a function pointer, the *instruction*
clears them before branching, so SLEIGH lifts the clear as a real p-code
`INT_AND` feeding the `CALLIND` target: ARM/Thumb `blx` goes through
`BXWritePC`, whose body is `local tmp = addr & 0xfffffffe`, and MIPS `jalr`
through `JXWritePC`, whose body is `tmp = -2 & addr`. That AND is machine
bookkeeping, not program semantics — the source performs no bit-clear — and
leaving it in place costs twice: the emitted C asserts an operation the program
never performs, and the mask stands between the `CALLIND` and its pointer
operand, so the pointer-to-code data-type never back-propagates onto the LOAD
that fetched the callee. A masked call renders as
`(*(code *)(*(uint4 *)(p + 0x44) & 0xfffffffe))(p)` where the un-masked form is
`(**(code **)(p + 0x44))(p)`.

`RuleFuncPtrEncoding` erases it. The width of the encoding is **not** a kuna
policy: it is declared per compiler spec by `<funcptr align="N"/>`, decoded into
`Architecture::funcptr_align` as the bit position of `N`'s first set bit
(chapter 00, the P0 knowledge plane), and read live by the rule. The rule fires
only on an exact match — the constant mask must equal `calc_mask(size) & (~0 <<
funcptr_align)`, i.e. all ones above the encoded bits — and rewrites the `INT_AND`
to a `COPY`, which is transparent, so any other reader of the masked value keeps
seeing it. A cspec that declares no `<funcptr>` leaves `funcptr_align == 0` and
the rule is inert, which is why x86/x86-64 keep every `& 0xfffffffe` their
programs really compute. The vendored specs declare `align="2"` (one mode bit)
for the four ARM cspecs, the nine MIPS cspecs, the four Loongarch cspecs and
8051, and `align="4"` (two word-alignment bits) for the five AARCH64 cspecs;
AARCH64's own `blr` masks nothing, so there the rule only fires on a mask the
program itself wrote. `funcptr_align` has two other live readers — the jump-table
model (chapter 02) and the `thumbfuncptr` const-pointer preservation (chapter 05,
§5) — and the three do not interact: this rule only ever removes an `INT_AND`
that a `CALLIND` consumes.

## 3.3 Sub-variable flow

`decompiler/crates/kuna-decomp/src/p3_dataflow/subflow.rs (SubvariableFlow)`
shrinks a logical value out of a larger container: given a *root* varnode and a
bit-mask identifying where the small value lives, it traces the value's flow
forward and backward through the data-flow graph, builds a parallel shadow
graph of placeholder varnodes/ops plus a patch list, and only if the **entire**
flow is expressible at the smaller size commits the rewrite
(`subflow.rs (SubvariableFlow::do_replacement)`) — replacing the wide ops with
logically-sized ones. It is all-or-nothing by construction: any placeholder the
trace cannot legalize aborts the whole transform with no IR change (marks are
cleared, `subflow.rs (SubvariableFlow::do_trace)`).

An aligned, contiguous byte slice also retains its original machine-storage
slice for P9 source comments. Its address respects the source space's byte
order. A bit-field extraction that cannot describe a byte slice does not claim
the entire containing register as its storage.

Six trigger rules in oppool1 (group `subvar`) seed it from ops that *prove* a
smaller logical value exists: `RuleSubvarAnd` (INT_AND by a low mask),
`RuleSubvarSubpiece` (SUBPIECE), `RuleSubvarCompZero` (INT_EQUAL/INT_NOTEQUAL
against a masked constant), `RuleSubvarShift` (INT_RIGHT bringing high bits
down), `RuleSubvarZext`, and `RuleSubvarSext` (the last arming the
sign-extension-invariant mode). The mask's bit-span picks the logical size
(`subflow.rs (SubvariableFlow::new)`): 1/2/3/4 bytes, 8 only when the caller
passes `big`, anything else — including a zero mask or a span over 64 bits —
constructs an invalid engine that traces nothing.

**When it refuses** (`subflow.rs (SubvariableFlow::set_replacement)`), roughly
in decision order (the constant-sext check actually sits in the constant arm
first; the sext size-mismatched-input refusal is bypassed in aggressive mode;
both type-lock refusals exempt `TYPE_PARTIALSTRUCT`): a varnode already visited with a *different* mask (two
inconsistent claims about where the logical value sits); any **free** varnode
(untraceable flow); an `addrforce` varnode of the wrong size (its full
container is pinned live); under sign-extension restrictions, a constant that
does not equal the sign-extension of its masked low part, and any
size-mismatched input or persistent varnode (their high bits cannot be assumed
to be extension); outside flag-sized traces (logical size ≥ 8 bits), a varnode
whose *consumed* bits extend beyond the mask — unless the caller is in
aggressive mode — because outside consumption means the container is probably
one real variable, not a packing; a type-locked varnode whose locked size
differs from the flow size; and for function inputs, no sub-byte flags and no
mask that is not anchored at bit 0 (either would fabricate an input register
slice the ABI cannot name). Terminal ops (CALL/RETURN/BRANCHIND boundaries) do
not refuse but *patch*: the trace records a pull/push patch at the boundary
(`try_call_pull`/`try_return_pull`/`try_switch_pull`/`try_call_return_push`),
and `do_trace` additionally refuses to commit when **zero pull points** were
found — a rewrite whose small value never actually escapes the shadow graph
would churn the IR for no output gain. (kuna) A call pull whose dropped bits
are known zero, because the wide input's non-zero mask lies inside the logical
mask, is recorded on the call's spec when the rewrite commits
(`decompiler/crates/kuna-decomp/src/p9_emit/kuna_truncarg.rs (note_trimmed_arg)`),
so emission can print the zero-extension C's promotion would otherwise lose
(chapter [09](09-emission.md)). (kuna) A RETURN pull is recorded on the RETURN
op the same way when its dropped bits are known zero and the calling convention
extends a value of the trimmed width by the sign of its type
(`decompiler/crates/kuna-decomp/src/p5_types/kuna_zextreturn.rs (unsigned_trim)`,
`note_trimmed_return`); a later trim of the same RETURN that drops bits not known
to be zero clears the record. Type inference reads it to type the returned value
unsigned (chapter [05](05-types.md)).

(kuna) The RETURN pull refuses to trim a 64-bit integer a 32-bit convention
returns in two registers
(`decompiler/crates/kuna-decomp/src/p3_dataflow/subflow.rs (returns_integer_pair)`):
a value no wider than eight bytes, wider than every single integer register of
the prototype model's output list, and laid out as one integer in the register
file's byte order. That covers the `join` return recovery builds for ARM `r1:r0`,
MIPS `$3:$2` or x86 `EDX:EAX`, and a register that names both halves, SPARC
`o0_1`. When the function writes the high register as zero the RETURN reads
`ZEXT(lo)`. Dead-code analysis counts only the possibly-nonzero bits of a RETURN
as consumed, so `RuleSubvarZext` used to trim the RETURN to the low register:
`u64 ins16(unsigned a, unsigned short b) { return ((a + 1) << 16) | b; }` (ARM
`orr r0,r1,r0,lsl #16; mov r1,#0; add r0,r0,#65536`) printed as
`int ins16(int a0,unsigned int a1)`, and a caller that widens the result
sign-extended the word the binary zero-extends. Kept whole, the function returns
`unsigned long long`, and the zero-extension is the conversion C performs at the
return, with a cast where the word's own C type is signed (`(unsigned int)(a1 + a0)`).
A pair joined against the register order keeps the trim: its high half is not
the convention's high word (on a big-endian target, `$3` above `$2`), so a zero
there says nothing about a 64-bit value. So does a 16-byte pair (x86-64
`RDX:RAX`): no standard C integer spans it, and a zeroed `RDX` there is far more
often the `-fzero-call-used-regs` scrub after an `int` function than an
`unsigned __int128`. Some 32-bit functions have exactly the
bytes of a zero-extended return and still mean a word, and now read as
`unsigned long long`: an i386 `int` function scrubbed by `-fzero-call-used-regs`
(`xor %edx,%edx; ret`), gcc -O0's `int f(u64 a) { return a >> 32; }`, which
copies the high word down and clears `EDX`, and compiler-rt's `rep_clz`.
`--option returnpair single` returns the first register alone for the run, which
restores the word for them.

(kuna) The RETURN pull also refuses to trim a zero-extended word where the
convention sign-extends every 32-bit return value whatever its sign
(`decompiler/crates/kuna-decomp/src/p5_types/kuna_zextreturn.rs (zero_extended_word)`):
the RISC-V and LoongArch LP64 rule of chapter [04](04-calls-and-prototypes.md)
(`kuna_narrowext::sign_extends_any`), and MIPS64 under `narrowext compiler`.
The trim must drop only known-zero bits, and the word's sign bit must be
possibly set; the check applies to the RETURN being pulled and to each constant
RETURN the pull patches directly. There no 32-bit C type is extended the way the
binary extends the value, so the function returns the whole register (GH-865):
RV64 `unsigned long z32m(unsigned long x) { return (x * 3) & 0xffffffffUL; }`
(`slliw; addw; slli a0,a0,32; srli a0,a0,32`) printed as `int z32m(int a0)`,
and a caller widening the result sign-extended what the binary zero-extends; it
now prints `unsigned long z32m(int a0) { return (unsigned int)(a0 * 3); }`. A
constant returned zero-extended, such as `0xfffffff0` beside such a value, no
longer reads as `-0x10`. A word whose sign bit is known clear keeps the trim,
since its zero and sign extensions agree, as do a word the function sign-extends
(`addw`) and a narrower value, which the rule extends by its type's sign
(`kuna_zextreturn`, chapter [05](05-types.md)). Under `narrowext off`, and on
x86 (32- and 64-bit) and AArch64, the function alone keeps the trim: those
conventions leave the bits above a narrow return unspecified, so the printed
`int` callee compiles to the same instructions. Its callers settle it. A caller that computes with the
whole register (`add $1,%rax` after the call) relies on the zero-extension the
callee performs, and its printed C sign-extended the `int` result. In
`decompile-all` such a function is decompiled again with the width its callers
compute with (`Funcdata::kuna_wide_return`, `voidret` in chapter
[04](04-calls-and-prototypes.md)). Where a caller computes with all eight bytes
the pull refuses the trim the same way (`zero_extended_word`); where callers
compute with at most four, a narrower value keeps its trim and is unsigned
instead (chapter [05](05-types.md)).
A convention that extends the value by its type's sign (`inttype`) leaves the
trim alone, since there a caller may read the whole register of an `unsigned
int`. Each trim the pull makes of a zero-extended value is noted on the
function, with whether its sign bit may be set (`Funcdata::kuna_note_zext_word`),
which is what `voidret` asks its callers about.

Three sibling engines share the file. `subflow.rs (SplitFlow)` (trigger
`RuleSplitFlow`, oppool1) splits a double-sized value into hi/lo lanes through
the `decompiler/crates/kuna-decomp/src/substrate/transform.rs
(TransformManager)` machinery when a SUBPIECE proves the halves live separate
lives. `subflow.rs (SubfloatFlow)` (trigger `RuleSubfloatConvert`, group
`floatprecision`) does the same for a float value carried in a wider float
container, converting constant encodings between formats along the way.
`subflow.rs (SplitDatatype)` (triggers `RuleSplitCopy`/`RuleSplitLoad`/
`RuleSplitStore`, cleanup pool) splits a whole-struct COPY/LOAD/STORE into
per-field transfers using recovered types — described with the type system in
chapter 05, as is lane division (`ActionLaneDivide` in stackstall, over
`subflow.rs (LaneDivide)` (built over `transform.rs (TransformManager)`)).

When `SplitDatatype` divides a constant, `kuna_constantbytes.rs` extracts
byte ranges from its 64-bit payload and returns zero beyond that payload.
For `PIECE` and `INT_ZEXT`, extraction respects the low operand's declared
byte width, including ranges that cross into the high operand. Both endian
orders use the same bounded extraction after mapping the field offset; no
shift wraps at the host word width. Output pieces wider than the host payload
still decline the extended-constant rewrite.

**Which copies the split declines** (`subflow.rs
(SplitDatatype::test_copy_constraints)`). Upstream refuses a COPY whose input is
a function input, whose input and output are address-tied at the *same* address
(the identity copy a heritage guard leaves behind), or whose input is the lone
output of a LOAD (handled by the LOAD split instead). kuna adds one more (DIV-55):
a COPY whose **output Varnode is read-only** — an address the load image reported
inside a non-writable section — is never split. A store into a read-only range is
not something the program performs, and the split is what makes such a copy
*visible*: whole, its input and output share a HighVariable and
`decompiler/crates/kuna-decomp/src/p6_variables/merge.rs
(Merge::mark_internal_copies)` marks it non-printing; split into one COPY per
array element, the pieces land in different HighVariables and P9 prints a block of
per-byte assignments into a `.rodata` string literal. The copies that reach the
gate in that shape are the `return_copy` guards of §3.1 after a block clone has
rewritten them: `substrate/funcdata_block.rs (CloneBlockOps::build_op_clone)`
copies only the upstream flag subset (plus chapter 05's `kuna_zextreturn`
record), which does not carry `return_copy`, and
`CloneBlockOps::patch_inputs` re-inputs the clone from a fresh COPY, so neither
the same-address test nor the flag can recognize the clone for what it is. The
read-only output test does, and it is the property that actually matters. The
same invariant is what `substrate/funcdata_varnode.rs (Funcdata::fillin_read_only)`
warns about (`Read-only address (ram,X) is written`) when `readonlypropagate` is
on; declining the split does not depend on that option.

**Which LOADs the split declines** (`subflow.rs (SplitDatatype::split_load)`).
When a loaded value's only use is a COPY, upstream builds the per-field LOADs at
the COPY and writes the fields straight into the COPY's output. The COPY can sit
after a STORE or a call, though. A 4-byte read at `s+7` that spans four `char`
fields, then a byte store to `s->c9`, then the COPY into the return register
printed the four field reads after the store, so the function returned the new
byte. kuna declines the split when a STORE or a call lies between the LOAD and
its COPY, or when the two sit in different blocks
(`subflow.rs (SplitDatatype::store_or_call_between)`). The LOAD then stays whole
and prints as its own statement ahead of the store (`v1 = *(uint4 *)&s->c7;`).
Splitting at the LOAD and keeping the COPY is not the answer: when the COPY writes
a global at the function's return, the pieces land in a temporary and the COPY
into the global stops printing, which drops the global's store. With nothing between the
two, the split still happens at the COPY, as upstream does. This is a strict fix
with no option (`tests/stages/kuna-aliasoverlap.xml` #5 to #8).

## 3.4 Conditional execution

`decompiler/crates/kuna-decomp/src/p3_dataflow/condexe.rs
(ActionConditionalExe)` (mainloop tail) removes a CBRANCH that re-tests a
condition an earlier block already decided. The candidate — the *iblock* — must
satisfy the two-block merge condition
(`condexe.rs (ConditionalExecution::verify)`), all read-only tests:

1. the iblock has exactly 2 in-edges and 2 out-edges and ends in a CBRANCH
   (`test_iblock`);
2. both in-paths, walked backward through any chain of single-in/single-out
   blocks, reach the **same** *initblock*, itself two-exit — so the iblock is
   purely a re-join of one earlier decision (`find_init_pre`);
3. the initblock also ends in a CBRANCH, and the two branch conditions are
   provably identical or complementary —
   `decompiler/crates/kuna-decomp/src/substrate/expression.rs
   (BooleanExpressionMatch::verify_condition)` matches the boolean expressions
   structurally (complement flips which path is "true");
4. every op in the iblock other than its branch is removable or movable
   (`test_removability`): no call, no flow-break, no LOAD/STORE/INDIRECT, no
   address-tied output; a MULTIEQUAL's readers must each tolerate the phi being
   pulled back into the predecessors (`test_multi_read` — a RETURN reader only
   in value position, an in-iblock reader only if COPY/SUBPIECE).

If verification passes, `condexe.rs (ConditionalExecution::execute)` rewires
the data-flow — each iblock op's output is replaced per consuming block, with
pulled-back MULTIEQUALs materialized in the post-blocks as needed
(`do_replacement`/`get_new_multi`) — deletes the iblock's ops in reverse order,
and splices the block out of the graph
(`decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs
(Funcdata::remove_from_flow_split)`). The action loops over all blocks until a
full round makes no change, and refuses to run at all while unreachable blocks
exist. One kuna conservatism: the per-space "has heritage run yet" array the
removability test consults is hard-wired to *false*
(`condexe.rs (ConditionalExecution::build_heritage_array)` — a port seam never
re-wired to the live `Funcdata::num_heritage_passes`), so an iblock op whose
output has **no readers** is always refused rather than trusted once its space
is heritaged; strictly conservative relative to upstream (a collapse is missed,
never wrongly taken). `condexe.rs (RuleOrPredicate)` (oppool1, group
`conditionalexe`) handles the value-form of the same redundancy: an INT_OR
(or INT_XOR) where one operand is provably zero along the path that reaches it (the
`MultiPredicate` zero-slot analysis) collapses to a COPY of the other operand.

**Conditional constants.** `decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionConditionalConst)` (mainloop tail, wrapper over
`decompiler/crates/kuna-decomp/src/p3_dataflow/condconst.rs (condconst_apply)`)
propagates the knowledge a CBRANCH creates: after `x == k` branches, `x` *is*
`k` on one out-edge (and a raw boolean is 0/1 down its two edges). Every read
of the varnode dominated by the constant edge is rewritten to the constant
(`condconst.rs (propagate_constant)`), constants are pushed through ops whose
other inputs are constant by direct evaluation (`condconst.rs (push_constant)`),
and — the phi case — a MULTIEQUAL input arriving on the constant edge is
replaced by a freshly-placed constant COPY in the edge's predecessor block,
but only when excising that edge leaves no alternate data-flow path rejoining
the original value (`condconst.rs (handle_phi_nodes)`; multiple disconnected
edges that flow together downstream get one shared placement).

A RETURN cannot take a constant input, so a dominated read by a RETURN gets
the constant through a COPY placed just before it, written at the varnode's own
storage. Upstream then stores that COPY in the RETURN's slot 1, which is the
return value only once return recovery has trimmed the RETURN to its address
and value. While the function's output trials are still open, a RETURN reads
every trial register — rax in slot 1, rdx in slot 2 on x86-64 — and a constant
known for rdx then replaced the rax slot: a function with a second RETURN
reached only when `rdx == 0` printed `return 0;` there instead of the entry
value. That happens whenever the output container outlives the first mainloop
iteration (a model with a delayed heritage space, or the `condexeret` extra
pass, 04 §4.4). kuna stores the COPY in the slot that actually reads the
varnode (`condconst.rs (propagate_constant)`); on a trimmed RETURN that is slot
1, as upstream.

A block whose last op is a CBRANCH is read as a two-way branch, so
`condconst.rs (condconst_apply)` skips one carrying fewer than two out-edges rather
than indexing off the end of its edge list. The `funcboundflow` truncation used to
produce one; the guard keeps a malformed graph from killing the process before the
pass that malformed it can be identified, but it does not by itself make the emitted
C right.

(kuna) **condexeplace** — GH-9203: that materialized COPY could land inside a
*loop* predecessor block, re-executing a supposedly loop-invariant `= 0` every
iteration and malforming the do/while. Under the gate,
`condconst.rs (handle_phi_nodes)` declines the placement when the predecessor
has a loop in-edge and leaves the phi edge untouched. Settable
`condexeplace` (`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_condexeplace.rs`
owns the option surface; the gate itself is the guarded block in
`handle_phi_nodes`); shipped default **on** per
`decompiler/crates/kuna-decomp/phases.toml` (DIV-3 — corpus-neutral, 0 of 675
assertions changed); `option condexeplace off` restores the upstream placement.
Catalog: [docs/options.md](../options.md).

## 3.5 kuna peephole rewrites

Kuna-added transforms live beside the upstream rules, each resolving an
open upstream issue (the sanctioned `(kuna)`-tag exception: their
`phases.toml` rows record `ghidra-upstream` as lineage because an upstream
*issue*, not upstream code, specified them — the GH number is the row's
`issue`). All share one wiring pattern: the rule is registered with its own baked-in
enable flag off (the pool still dispatches it), so each `apply_op` defers
per-op to the live gate on the per-function architecture snapshot (e.g. `kuna_booleanmask.rs (RuleBoolSignShift::apply_op)` testing
`fold_boolean_mask`) — which makes them subject to the flag-copy hazard of
§0.5 — and every gate's engine default is set in
`decompiler/crates/kuna-decomp/src/infra/architecture.rs
(reset_defaults_internal)`, mirrored by the `default` column of
`decompiler/crates/kuna-decomp/phases.toml` (the source quoted below; the
DIV-2/DIV-3 rows of `docs/history.md` carry the ablation evidence). With a
gate off, the rule returns 0 unconditionally and output is byte-identical to
upstream. Full option metadata: [docs/options.md](../options.md).

**addcarrychain** (GH-8913) —
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_addcarrychain.rs
(RuleAddCarryChain)`, oppool1, fires on PIECE. Pattern: the reassembly of an
8-bit carry-chained add, `PIECE(hi, lo)` where `lo = INT_ADD(a, b)` and
`hi = INT_ADD(hipart, carry)` with `carry` the carry of `(a, b)` — either a raw
INT_CARRY or its const-folded `INT_LESSEQUAL((-b) & mask, a)` form, matched
through CAST/COPY chains. Rewrite: one wide `INT_ADD(PIECE(hipart, b),
ZEXT(a))`, recovering the single 16-bit addition the 6502-class ADC pair
implements. Settable `addcarrychain`, shipped default **on** (DIV-2).

Carry-chain and array-stride helper outputs use
`decompiler/crates/kuna-decomp/src/substrate/funcdata_varnode.rs (new_unique_out)`
instead of private copies of the bank allocation sequence. The shared factory
links each definition and assigns its HighVariable when high-level variables
are already enabled; it also owns lane bookkeeping. Pattern guards, opcode
metadata, graph-edit order and option gates are unchanged. The ordinary
schedule applies these rules before high-level assignment, while direct rule
invocations must also preserve the function's existing high-level state.

**booleanmask** (GH-1282) —
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_booleanmask.rs
(RuleBoolSignShift)`, oppool1, fires on INT_SRIGHT. Pattern:
`(b << k) s>> k` with the same non-byte-aligned `k` on both shifts (the
byte-aligned case already belongs to `RuleLeftRight`), where the pre-shift
value's known-nonzero mask fits entirely below the shifted-out bits — i.e. `b`
is a boolean being smeared across the word. Rewrite: `INT_2COMP(b)` (`0 - b`,
giving 0 or all-ones), which the surrounding compare rules then clean to a
plain boolean test. Settable `booleanmask`, shipped default **on** (DIV-2).

**cancelbytearithmetic** (repipe `cancelling-byte-arithmetic-splits`) —
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_cancelbytearithmetic.rs`
owns the option and exact graph matcher; a small hook remains in the
upstream-derived `RuleSubCommute::apply_op` because the witness begins at its
`SUBPIECE(INT_LEFT(...),0)` arm. Pattern: an outer one-byte add whose inputs are
`low8(x << s)` and an inner add containing both an independent value and
`low8(x) * (-2^s mod 256)`, for constant `1 <= s < 8`. The two terms cancel for
every byte value, so the outer add is rewritten directly to a COPY of the
independent value. The matcher requires the same `x`, byte width and zero
offset in both SUBPIECEs, the exact modulo-256 coefficient, and a sole-consumer
chain from each intermediate result to the root. It accepts either operand
order for both adds and the multiply.

The filed witness additionally requires `x` to be the pure result of an
`INT_AND` with two nonconstant inputs. Constant masks and direct CALL/LOAD
producers decline. Wrong coefficients, sources, widths or offsets; shared
intermediates; variable or out-of-range shifts; and all unrelated
RuleSubCommute arms also decline. The fold does not push a SUBPIECE through the
shift and never creates narrow shift arithmetic, avoiding signed `char << s`
in emitted C. The original INT_ZEXT/PIECE route ignores this gate and remains
unchanged when the option is off. Settable `cancelbytearithmetic`, shipped
default **on** (DIV-174).

**simdlane** (repipe `simd-constant-string-initializer`) —
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_simdlane.rs
(RuleSimdShuffleLane)`, oppool1, fires on SUBPIECE. *Pattern:* a one-byte lane
read of a byte-shuffle user op whose mask is constant — `SUBPIECE(CALLOTHER
pshufb(src, m), k)` with `m` a constant Varnode. `pshufb` has no p-code
semantics (the x86 SLEIGH spec models it as an opaque CALLOTHER over a 16-byte
value), so after `ActionLaneDivide` splits the vector consumers into byte lanes
every lane read is a SUBPIECE of something nothing downstream can see through,
and neither `RuleSubExtComm`/`RuleSubZext` nor copy propagation can collapse
them. *Rewrite:* the instruction is a pure permutation with an exact per-lane
definition once the mask is known, `dst[i] = (m[i] & 0x80) ? 0 : src[m[i] &
(N-1)]`, so the lane read becomes `SUBPIECE(src, m[k] & (N-1))`, or a COPY of
the constant `0` for a zeroing mask byte. It is an identity, not a heuristic.
Once every lane read is re-anchored on the source the CALLOTHER loses its last
reader; for the standard byte-broadcast idiom (`pxor xmm2,xmm2; pshufb
xmm0,xmm2` — an all-zero mask) all sixteen lanes resolve to `SUBPIECE(src, 0)`
and collapse into one value. *Bounds/failure:* only a user op the architecture
registered under a shuffle name (`kuna_simdlane.rs (SHUFFLE_USEROP_NAMES)` =
`pshufb`, `vpshufb`; the ids are resolved once per program in
`Architecture::build_arch_handle` and carried on the `ArchContext`, since a Rule
cannot reach the userop table); only the three-input form whose two operands and
output all have the vector width; only widths 8 (MMX) and 16 (SSE); only a
ONE-BYTE lane read, because a wider SUBPIECE of a shuffle is a concatenation of
lanes and not another SUBPIECE. A mask wider than eight bytes does not fit a
`uintb` offset and is accepted only at value `0`, where the offset IS the whole
value and every lane byte is provably zero — the broadcast mask, and the only
wide constant mask the engine constructs. Settable `simdlane`, shipped default
**on**.

**constspaceload** (repipe `arm-neon-zero-initialization`) —
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_constspaceload.rs
(RuleConstSpaceLoad)`, oppool1, fires on LOAD. *Pattern:* a LOAD whose space
operand names the **constant** space and whose pointer is a defined Varnode
rather than a constant. That shape is what a SLEIGH constructor ending in
`export *[const]:N v` — a *dynamic* constant-space export — lowers to. The
defining property of the constant space is that an address IS its own value,
which is the identity `RuleLoadVarnode` applies when the pointer is already a
constant; it does not stop holding for a temporary, but nothing applied it
there. `RuleCollapseConstants` cannot supply the missing constant either,
because `PcodeOp::isCollapsible` declines any op wider than a `uintb`, so a
16-byte SIMD immediate (ARM `vmov.i32 q8,#0` is `ARMneon.sinc:549
simdExpImm_16`, whose body is `tmp:16 = 0; export *[const]:16 tmp`) survives as
a LOAD through a pointer. `ActionLaneDivide` then splits it into one lane LOAD
per word at `v`, `v+4`, `v+8`, `v+0xc` — invalid in the constant space, where
offsetting an address changes the value rather than selecting a word of it, so
lane 0 folds correctly and the rest read near-null memory. *Rewrite:*
`out:N = LOAD(const, p:N)` becomes `COPY p`, and only at equal widths. Applied
in oppool1 it lands before `ActionLaneDivide`, so each lane is split off the
*value*. *Bounds/failure:* only the constant space is matched; a width mismatch
is declined, because a dynamic `export *[const]:N tmp` gives the operand and
`tmp` the same size, so `N != S` is a different shape and resizing it would
invent a truncation rather than apply an identity — a `SUBPIECE`/`INT_ZEXT`
version of this rewrite was measured to re-render live AVX-512 `k` mask
registers on statically linked glibc, and in `__strlen_evex`-shaped code one
mask lost its reaching definition at a shared label, so the scan loop read a
stale earlier compare; a constant pointer is left to `RuleLoadVarnode`, which
also resolves the spacebase-placeholder tail that a bare COPY would drop; a
free pointer is declined. Settable `constspaceload`,
shipped default **on** (DIV-158).

**flagcompare** (GH-1276 / GH-8777) —
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_flagcompare.rs`, two rules
under one gate, for architectures that model condition flags as explicit bits.
`RuleBoolSignLess` (fires on INT_SLESS): a boolean shifted into the sign bit
and tested with `s< 0` — where the operand's nonzero mask is exactly the bit
landing in the sign position — becomes `b != 0`. `RuleSborrowGe` (fires on
BOOL_AND/BOOL_OR): the `N == V` signed-comparison idiom — the
XNOR of the result sign of `V - K` with `SBORROW(V, K)`, in either its
AND-of-ORs or OR-of-ANDs lowering — becomes `INT_SLESSEQUAL(K, V)` (`V >= K`
as the source wrote it). Settable `flagcompare`, shipped default **on**
(DIV-3).

**ovlesssimplify** (GH-7190) —
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_ovlesssimplify.rs
(RuleOvLessSimplify)`, oppool1, fires on INT_NOTEQUAL. Pattern: the explicit
S/OV-flag signed-less-than computation (V850-style),
`NE(SLESS(V+K, 0), BOOL_AND(signtest, SLESS(-1, V+K)))` — the sign flag XORed
with the overflow test spelled out in p-code. Rewrite: `INT_SLESS(V, -K)`.
Settable `ovlesssimplify`, shipped default **on** (DIV-2).

**compareform** (GH-558) — not a pool peephole but the canonicalization
round-trip for `<=`. The analysis wants one canonical compare form, so
`decompiler/crates/kuna-decomp/src/substrate/funcdata_op.rs
(Funcdata::replace_lessequal)` rewrites `V <= c` into `V < c+1` (and
`c-1 < V` from `c <= V`), with overflow guards, from exactly three sites: the
pool rule `ruleaction_1.rs (RuleIntLessEqual)` — carried in its own group
`canonicalcompare`, enabled in every root variant — and the two branch-flip
primitives in `funcdata_op.rs` (`op_normalize_flip` and the flip-in-place
path). Each rewrite stamps a provenance bit on the op
(`canonical_lessequal`). At the very end of the drive — after structuring's
last flips, before prototype/cast/naming fixation —
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_compareform.rs
(ActionPresentCompareForm)` (group `presentcompare`, `decompile` variant only)
inverts every still-marked op back to the source `<=` form, re-validating the
shape from scratch so an op reshaped by a later transform is simply left
alone. Settable `compareform canonical|original`, shipped default
**original** (restore `<=`; DIV-2 — the flip re-pinned 12 of 675 datatest
assertions); `option compareform canonical` leaves the analysis form standing,
reproducing upstream Ghidra's rendering.

**arraystride** (GH-8724) —
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_arraystride.rs
(RuleArrayStride)`, oppool1, fires on MULTIEQUAL. Pattern: a strength-reduced
array walk — a loop-header offset accumulator
`acc = MULTIEQUAL(#0, acc + STRIDE)` (STRIDE constant, neither 0 nor 1) with a
sibling unit-step counter phi `cnt = MULTIEQUAL(#0, cnt + 1)` in the *same*
block, lining up edge-for-edge. Rewrite: every other use of `acc` is replaced
by `INT_MULT(cnt, STRIDE)`, re-exposing `cnt` as the array index so the
pointer rules and the emitter can render `arr[i]` instead of
`iVar += 0x414`. Settable `arraystride`, shipped default **on** (DIV-3).

**mulblob** (kuna) —
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_mulblob.rs
(declines_zext)`, consulted by `RulePieceStructure` rather than registered as a
rule of its own. It answers one question: whether a widened operand of a wide
multiply is a *value* or an *aggregate*.

x86-64 `MUL r64` lowers to `tmp:16 = zext(RAX) * zext(rm64)` with `RDX` and
`RAX` read back out by SUBPIECE. A 16-byte Varnode has no primitive data-type,
so the type factory hands it the width fallback `undefined1[16]` — an array,
which `Datatype::is_piece_structured` accepts along with every struct. That is
enough for `RulePieceStructure` to treat the extension as the root of a
concatenation tree building a structured value: it rewrites the `INT_ZEXT` into
a `PIECE` of a zero constant and the operand, marks the output a partial root,
and §6 then gives that root a declaration of its own. Every unsigned wide
multiply therefore costs two extra locals whose entire content is the operand
(`v8._8_8_ = 0; v8._0_8_ = v29;`) before a single `SUB168` reads the product
back. The signed sibling never had this: `IMUL` lowers through `sext`, and
`INT_SEXT` is not in `RulePieceStructure`'s op list, so kuna has always printed
`SUB168(SEXT816(x) * SEXT816(y),8)` for it.

With the gate on, `declines_zext` makes the unsigned form behave like the
signed one: `RulePieceStructure` leaves the extension alone at all three of its
conversion sites (the direct `INT_ZEXT` entry, the extension found above a
CONCAT root, and each `INT_ZEXT` leaf inside a gathered tree), the operand stays
an implied varnode, and the multiply prints over the operands themselves. The
IR is otherwise untouched — no op is added, moved or deleted — so both halves of
the product are the same SUBPIECEs of the same product they were before.

The decline is narrow, because the same rewrite is load-bearing wherever a
concatenation really does build a structure. It requires all of: the extension's
output is in the unique space, and is not addr-tied, mapped, persistent or
already a proto-partial; it is wider than 8 bytes and no wider than 16; its
data-type is exactly the width fallback (an array of that many one-byte
`undefined` elements, not a recovered array); no local Symbol covers its
address; and every reader is a same-width integer arithmetic op whose own
result is read only through SUBPIECE. A Varnode backed by real storage, or one
whose array type came from type recovery, keeps upstream's structuring in
either arm.

One naming consequence is worth stating, because it is the only place the
decline is visible outside the multiply itself. With the blob gone, an operand
that is read by the multiply *and* by something else has two readers of its own,
and §6's `ActionMarkExplicit` gives a value with a second reader through an
extension its own name. A reciprocal division whose dividend is also returned
therefore reads `v1 = x; ... = v1 / N;` rather than repeating `x` on both sides
— which is exactly how the signed sibling, whose operands were never
structured, has always rendered the same code. With the blob present the copy
into it supplied that second name, so the repetition survived.

Settable `mulblob on|off`, shipped default **on**. Flipping it off restores
upstream Ghidra's rendering, which prints the same `undefined1 auVarN [16]`
blobs — this is a deliberate readability divergence, not a fidelity fix, and it
is inert on every function with no wide multiply.

## 3.6 Early passes

`decompiler/crates/kuna-decomp/src/p3_dataflow/coreaction_early.rs` holds the
setup and maintenance actions the schedule interleaves around heritage and the
pools (§0.6 places them; this section says what each computes).

**Setup one-shots** (in the restart group's prologue or at phase switches):
`ActionStart`/`ActionStop` are no-ops in kuna — the start/stop bookkeeping the
C++ did there happens in the drive, which follows flow before the tree runs
(§0.6) — and exist so the tree's listing stays oracle-identical.
`ActionConstbase` injects a `COPY #val` at the entry block for every *tracked
register* the context database pins to a constant at this function's address
(the console `set track` surface). `ActionStartTypes` flips the function's
type-recovery flag — the gate the oppool2 pointer rules and
`ActionInferTypes` key on (chapter 05) — and `ActionStartCleanUp` marks the
transition into the cleanup phase. `ActionNormalizeSetup` (normalize variant
only) strips prototype locks for the normalization style.

**Per-iteration maintenance** (mainloop): `ActionSpacebase` marks
stack-pointer varnodes and their types ahead of heritage; `ActionHeritage`
drives §3.1; `ActionNonzeroMask` recomputes the known-zero-bits fact
(`Funcdata::calc_nz_mask`) that dozens of rules consult (§3.5's booleanmask
and flagcompare among them); `ActionVarnodeProps` applies storage-derived
properties — after the first heritage pass it releases the `autolivehold`
pins (except on values still LOADed through a constant/read-only pointer or
a proven volatile address),
replaces *read-only* storage with its image constant when
`readonlypropagate` is set — or, with that program-wide switch off, when the
varnode lies in one of the loader's `dynrelocs` ranges, the `PT_GNU_RELRO`-frozen
dynamic-relocation slots whose value the linker itself computed (§1.2), which is
what turns a call through a relocated GOT slot back into a named call, or when the
read lies entirely inside the image's executable read-only memory (`litpoolconst`,
§1.2), which is what renders an ARM literal-pool constant as its value — expands
*volatile* access into its user-op form. An external-reference Varnode skips the
entire read-only fill branch, including program-wide `readonly`, `dynrelocs` and
`litpoolconst`: the mark says a loader supplies the run-time target, so file bytes
cannot replace its symbolic identity. This is load-bearing for a PE IAT placed in
an RX section, whose file word is a hint/name RVA rather than the function address
the Windows loader writes there. The action also folds to zero any varnode whose
consumed bits and nonzero mask are
disjoint (skipping constants and COPYs of nonzero constants, which would
recurse).

A read from a volatile range becomes a `read_volatile` user op only once its
address is a constant: `RuleLoadVarnode` turns the LOAD into a COPY of the
memory varnode, which carries the range's `volatile` property, and
`ActionVarnodeProps` rewrites that read. An address computed in registers (a
`movw`/`movt` pair feeding an ARM `ldm`, or an x86 base register plus
displacements) folds only in the rule pool, after `ActionDeadCode` has already
run, so a read whose value the program discards used to be deleted first when
the address sat deeper than the general `lastChanceLoad` lookahead (three
levels of binary operations). So before any removal path discards a LOAD,
`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_volatileload.rs` evaluates
its address through a bounded graph of constant integer operations and asks the
same local and global property query a new varnode at that address would get.
When the answer is `volatile`, the LOAD is kept: `lastChanceLoad` holds it on
every heritage pass, including a LOAD whose readers consume none of its bits
(which the sweep would otherwise replace with zero as never consumed),
`RuleEarlyRemoval` skips it, and `ActionVarnodeProps` keeps its `autolivehold`
pin. Each access therefore survives, in program order, until the ordinary
lowering above takes over.

The hardware reads a memory operand once per instruction, but SLEIGH lifts an
operand the flag macros re-load (x86 `add`/`or`/`and`/`inc` on memory, MSP430
`add src,x(Rn)`) as several LOADs of the same address, some of them after the
instruction's own store. So a volatile LOAD is neither held nor protected when
another live LOAD at the same instruction address reads the same storage and is
used, is already held, or was lifted first: one read per address per
instruction survives, the used one when there is one. Load-multiple
instructions read distinct addresses and keep every access. The key is the
instruction and the resolved address, not the operand, so two different
operands that resolve to the same volatile word also merge into one read when
the instruction's result is discarded (x86 `cmpsd` with `rsi` equal to `rdi`,
or MSP430 `cmp @r5,0(r5)`, with the flags unused); when the result is used,
both reads survive. The rule applies whichever path would have held the
LOAD, so it also removes the re-reads the eventual-constant hold kept for
shallow addresses. It does not reach an operand whose address is a constant at
lift time: those reads are memory varnodes from the start, not LOADs, and each
becomes its own `read_volatile`.

The evaluation respects operation widths, never reads memory, and declines phi
nodes, unknown inputs and cycles; constant leaves are free, and one proof may
evaluate at most 64 written varnodes beyond those `lastChanceLoad` already
proved in the same pass, so a long post-increment chain costs one step per LOAD.
An address that reaches the LOAD through a stack slot is not proven. Like the
lowering, the property is tested at the access's first byte. A range declared
both `readonly` and `volatile` over memory the image does not map prints each
kept read as a self-assignment at the function's exit. The volatile ranges
include those a processor specification declares (MSP430, AVR, 8051, PIC and
others), so default output changes on those processors; loads from any other
address keep the existing removal rules.

**Block-graph cleanup** (mainloop tail): `ActionUnreachable` deletes blocks
flow cannot reach (`Funcdata::remove_unreachable_blocks`); `ActionDoNothing`
(repeat-apply) and `ActionLateDoNothing` splice out empty do-nothing blocks
early and late; `ActionRedundBranch` removes a branch whose target join adds
nothing (the redundant-join splice); and `ActionDeterminedBranch` converts a
CBRANCH whose condition has simplified to a constant into an unconditional
branch, severing the dead edge
(`decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs
(Funcdata::remove_branch)`) — this is the in-loop feedback edge by which a
constant-propagation result (P5 facts) edits the P2 control-flow artifact
without any restart (§0.7): the next mainloop iteration simply re-heritages
the smaller graph.

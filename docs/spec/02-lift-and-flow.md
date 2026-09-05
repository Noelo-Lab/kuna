# 02 — Lift & flow recovery

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/p2_lift
```

This phase turns an entry address into a function: raw p-code ops for every
reachable instruction, a basic-block graph over them, a call-spec record per
call site, and — the centerpiece — a recovered `JumpTable` for every indirect
branch that is really a switch. The driver is
`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs (follow_flow_on_fd)`
(the C++ `Funcdata::followFlow`): it constructs a
`decompiler/crates/kuna-decomp/src/p2_lift/flow.rs (FlowInfo)` over the fresh
`Funcdata`, applies the architecture's flow options (default
`error_toomanyinstructions`, instruction budget 100000 —
`decompiler/crates/kuna-decomp/src/infra/architecture.rs (reset_defaults_internal)`),
runs op generation + block generation, maps each recovered jump table's
addresses onto block out-edges (`JumpTable::switch_over`), and finally computes
the dominator tree (`structure_reset`) the SSA phase requires. `FlowInfo`
reaches the architecture only through the
`decompiler/crates/kuna-decomp/src/p2_lift/flow.rs (FlowEnvironment)` trait; the
live implementation is
`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs (ArchFlowEnv)`.

Option defaults and flip guidance for every option named below live in the
generated catalog ([docs/options.md](../options.md)); the rows are defined in
`decompiler/crates/kuna-decomp/phases.toml` and the intentional
default-divergences are DIV-3/4/13/14 in `docs/history.md`.

> Scope note: `decompiler/crates/kuna-decomp/src/p2_lift/funcdata_resolveflow.rs
> (Funcdata::resolve_in_flow)` is, despite its name, the union-field
> `resolveInFlow`/`resolveTruncation` dispatch — type-recovery machinery
> described in chapter [05 — Types](05-types.md). It lives in this folder only
> for file-lineage reasons and plays no part in flow recovery.

## 2.1 Instruction following

**Two-phase model.** `flow.rs (FlowInfo::generate_ops)` produces every raw
p-code op into the op bank's *dead list*, instruction by instruction;
`flow.rs (FlowInfo::generate_blocks)` later organizes them into basic blocks
(§2.2). The simple case is a worklist: pop an address off the fall-through
stack, decode one instruction through the SLEIGH translator
(`flow.rs (FlowInfo::process_instruction)`), record it in the `visited` map
(address → first-op sequence number + byte length), classify the emitted ops'
control flow (`flow.rs (FlowInfo::xref_control_flow)`), and push the
fall-through successor plus any branch targets (`flow.rs
(FlowInfo::new_address)`). RETURN and BRANCH end fall-through; CALL and CALLIND
are treated as fall-through ops; a BRANCHIND is parked on `tablelist` for
jump-table recovery (§2.3). A branch whose target is a *constant* is a branch
internal to the instruction's own p-code (`flow.rs (FlowInfo::find_rel_target)`);
ops beyond the deepest internal branch time are dead and deleted
(`delete_remaining_ops`). The op-creation and classification order here is
observable — it fixes the SeqNum allocation every later phase keys on.

**The dead list is a list, and the walk must treat it as one.** The C++ holds
`std::list<PcodeOp *>::iterator`s directly (`insertiter` on each op), so
"the op after this one", "the op before this one" and "the last op" are all
constant-time. kuna's dead list is the same doubly-linked list, realized as
prev/next `OpId` links on each op (`op.rs (IntrusiveList)`), and
`op.rs (PcodeOpBank::dead_next / dead_prev / dead_front / dead_back)` read those
links. Position must never be re-derived by scanning `iter_dead()`: the marker
idiom in `process_instruction` (remember the tail, decode, then take the first
op after the marker) and the per-op walk in `xref_control_flow` run once per
instruction over a list that grows to the whole function, so a scan there is
quadratic in the function's op count. Membership is the `dead` flag *plus* a live
link — `destroy` retires an op to `deadandgone` with the flag still set, and the
alive list shares the same link pair — so the cursor reports a non-member as
`None`, which is what a scan for an absent op returned.

**Declared flow bounds.** A function normally follows flow wherever the code
goes: `FlowInfo`'s allowed range is initialized to the whole entry-point space, so
the extent is discovered, not asserted. A `Funcdata` that carries a non-zero byte
size instead restricts flow to `[entry, entry + size - 1]`
(`decompile_drive.rs (follow_flow_on_fd)` calls `flow.rs (FlowInfo::set_range)`
before op generation, the C++ `Funcdata::followFlow(baddr, eaddr)` shape). `eaddr`
is inclusive, so the last in-body byte is `entry + size - 1`. A branch target
outside the range is not followed (`flow.rs (FlowInfo::new_address)`), and
fall-through stops when it would leave it (`flow.rs (FlowInfo::fallthru)`); both
route through `flow.rs (FlowInfo::handle_out_of_bounds)`, which under the default
flow options continues rather than failing the decompile — `errorreinterpreted`-style
hard failure needs `error_outofbounds`.

Continuing quietly would be the wrong contract for a *declared* bound, because the
caller cannot otherwise tell a correct boundary from one that truncated the body:
both just produce a shorter function. So the out-of-bounds handler emits the C++
diagnostics — a `Funcdata::warning` at each cut edge and, once per function, the
`Function flows out of bounds` `Funcdata::warning_header` — which the emitter
renders as comments on the prototype and the offending statement. A correctly
declared function ends in a return and never leaves its range, so a warning here
means the declared end is wrong.

Warning is only half of continuing, though, because the cut edge still needs a
head: the target was never decoded, so it has no op, and until DIV-121 the block
build asked for one anyway and the whole function died with `Could not find op
at target address` — in every mode, so a declared end that cut any real edge
produced no C at all rather than a clipped body. §2.2 closes that: the halt
`fillin_branch_stubs` already planted at each out-of-extent address is now
registered as the instruction there, so the branch lands on it and the body ends
at the boundary carrying the warnings above. The registration is restricted to
the addresses `handle_out_of_bounds` recorded, so the diagnostic still means
what it says everywhere else. Size 0 is the unbounded default every caller took
until the boundary-declaration surface existed (chapter
[00 §0.4](00-overview.md)) and leaves the range at the whole entry-point space, so
both the bound and its diagnostics are inert for a run that declares nothing.

**Decode scratch storage.** Every SLEIGH translation checks out a parser
context from the engine-local pool
(`decompiler/crates/kuna-sleigh/src/sleigh.rs (Sleigh::checkout_context)`).
Checkout resets the parse state, addresses, context words, commit records, and
root node; each child node is reset before it is allocated. The state arena and
its allocation capacities are retained across instructions. Simultaneously
live main-instruction, `inst_next2`, and delay-slot resolutions hold distinct
contexts, and a guard returns each context after successful translation or an
error. This is scratch reuse only: instruction results and addresses are not
cached, and the bytes and painted context are resolved on every translation.

**Instruction-byte window.** A parser context buffers a fixed 16 bytes of the
instruction stream, the longest encoding any supported processor admits. Reads
out of that window are bounded by their *starting* byte: a read that begins at or
past the window is the architecture's instruction-length limit and raises bad
data, which the decode-error policy below turns into a truncating halt. A read
that begins inside the window but runs off its end is not an error, because
pattern matching walks a candidate pattern in fixed-width words however narrow
the pattern really is: matching an instruction at the length limit issues a
word-sized read straddling the end of the window. Those tail bytes are outside
the matcher's mask and cannot affect the verdict, so kuna reads them as zero
rather than rejecting the instruction (the C++ reads whatever memory follows the
buffer). Without this an x86-64 15-byte alignment NOP — clang's `-O2` loop
padding — failed to decode, truncating flow at the padding and dropping the loop
it preceded from the emitted function.

**Decode-error policy** (`flow.rs (FlowInfo::handle_decode_error)`). An
unimplemented instruction is, per the flags, treated as a NOP
(`ignore_unimplemented`), re-thrown (`error_unimplemented`), or replaced by an
*artificial halt* that truncates flow at that point. Undecodable bytes (bad
data) halt-truncate or throw; a branch outside the flow bounds is
recorded on the `unprocessed` list (`flow.rs (FlowInfo::new_address)`, via `handle_out_of_bounds` for the warn/throw policy); flow into the
middle of an already-decoded instruction is a *reinterpretation* (warn, or
throw under `error_reinterpreted`). An artificial halt
(`flow.rs (FlowInfo::artificial_halt)`) is a synthesized RETURN annotated with
its cause (`unimplemented`/`badinstruction`/`noreturn`/`missing`), so the CFG
always terminates cleanly and the printer can attribute the truncation.

**The instruction budget.** `max_instructions` (100000 by default, `option
maxinstruction N`) caps how many instructions one function's flow may decode.
Reaching it either throws — `option errortoomanyinstructions on`, upstream's
default and what the datatest harness and the interactive console take — or
truncates: a `badinstruction` artificial halt is planted at the address that
would have been decoded, registered in `visited` as the instruction there so a
branch arriving later resolves to it, marked as starting a basic block and an
instruction, and reported with no fall-through, which ends the walk down that
path. Each address still queued is halted the same way when it is popped, so
the decode stops at the budget rather than running to the end of the reachable
body; a function of 1.8 million instructions truncates in seconds instead of
exhausting memory. The truncated function carries a warning header naming the
budget and the option that raises it. The CLI's decompiling surfaces (`kuna
decompile`, `decompile-all`, `decompile-project`, `decompile-graph`) choose the
truncating policy (DIV-120) so an oversized function yields the body kuna did
decode; the engine default is still upstream's throw.

**Call sites.** Each CALL gets a `FuncCallSpecs` bound to the op
(`flow.rs (FlowInfo::setup_call_specs)`, shared body `build_call_specs`): the
callee entry is resolved through the symbol table (`FlowEnvironment::query_call`),
the call op's input 0 is replaced by an *fspec annotation* Varnode carrying the
spec's identity, per-call prototype overrides are applied before the query
(`override prototype`), and a declared callee prototype is copied onto the spec
(kuna performs the C++ `ActionDefaultParams`-time copy here, excluding inline
calls, with identical observable result — the analysis path locks callee
signatures before flow runs). A CALLIND keeps its computed target as input 0
(`setup_callind_specs`) unless a previous decompilation pass planted an
indirect override (de-indirection), which converts it to a direct CALL before
the spec is built. `flow.rs (FlowInfo::check_for_flow_modification)` then
applies the callee's flow effects: an *inline* callee queues the op for
injection; a *no-return* callee gets an artificial halt planted directly after
the call plus the `"Subroutine does not return"` warning, so flow never runs
off past the call (§2.4 covers who supplies the no-return facts).

**Inlining** (`flow.rs (FlowInfo::inline_sub_function)`). The callee's p-code
is generated by a nested `FlowInfo` over a fresh callee `Funcdata`
(`inline_flow`), then woven in by one of two models. The *EZ model* — the
callee is a straight-line leaf (no call or branch ops, `check_ez_model`) —
clones the body re-addressed to the call site and deletes the call: the inline
is invisible to addressing. The *hard model* clones the body at its original
addresses, replaces each callee RETURN with a BRANCH to the op after the call
site, and rewrites the CALL itself into a BRANCH to the callee entry; it is
refused (`test_hard_inline_restrictions`) when there is no op to return to or
the return address equals the call address. A recursion set
(`inline_recursion`, forwarded into nested flows) stops a function from being
inlined into itself; the failure mode of every refusal is the same — the call
stays a call — with a per-cause warning (`"Could not inline here"` for
recursion; distinct no-fallthrough / return-address messages from
`test_hard_inline_restrictions`; a missing callee body refuses silently).

**P-code injection.** Three substitution kinds run from a queue drained during op
generation (`flow.rs (FlowInfo::inject_pcode)`); the user-op and call-fixup kinds
share one weave (`flow.rs (FlowInfo::do_injection)`), while inlining uses its own
clone weave (above): emit the payload's p-code at the dead-list
tail, classify its control flow, optionally mark it *incidental copy*, splice
it after the original op, repoint the target map, and destroy the original op.

Classification queues an op the moment it is decoded, so the drain must run once
per flow-discovery round or the ops queued by a later round are dropped: the
queue is drained after the initial fall-thru phase (`generate_ops`) and again
after every jump-table round (§2.3), and a drain clears the queue. Which round
found a block therefore has no bearing on whether its injections are applied —
a spec-declared eraser such as ARM's `setISAMode` `<callotherfixup>` removes the
op uniformly, whether the block was reached by fall-thru or only through a
recovered switch table.

- **Injection library.** `decompiler/crates/kuna-decomp/src/p2_lift/pcodeinject.rs
  (PcodeInjectLibraryBase)` holds the payloads — `<callfixup>`,
  `<callotherfixup>`, and executable-p-code snippets — decoded from the
  compiler/processor specs; their SLEIGH source bodies are compiled to p-code
  templates by `decompiler/crates/kuna-decomp/src/p2_lift/inject_sleigh.rs
  (parse_inject)` and emitted at a concrete address through
  `inject_sleigh.rs (SleighInjectEngine)`.
- **User ops.** `decompiler/crates/kuna-decomp/src/p2_lift/userop.rs
  (UserOpManage)` manages the CALLOTHER black-box ops (unspecialized, datatype,
  volatile, segment, jump-assist, injected). A CALLOTHER whose user op is
  *injected* is queued during classification and replaced by its
  callother-fixup body (`flow.rs (FlowInfo::inject_user_op)`) — e.g. the ARM and
  MIPS `setISAMode` no-op, which dead-code elimination then removes. A user op
  with no declared fixup is not injected and survives as a black box the printer
  renders as a call; that is the intended rendering for unimplemented semantics
  (ARM `DataMemoryBarrier`, the coprocessor family), and only a *declared* fixup
  makes disappearance the correct outcome.
- **Call fixups.** A call spec carrying an inject id has its CALL/CALLIND
  replaced by the named call-fixup payload
  (`flow.rs (FlowInfo::inject_sub_function)`); the payload's parameter shift is
  transferred to the call spec created inside the woven body, and a nested call
  to the same fixup entry is cycle-broken (it must not re-inject).
- **kuna-owned call fixups.** Payloads are not required to come from a spec
  file. `decompiler/crates/kuna-decomp/src/p2_lift/kuna_msvcftol.rs` synthesizes
  a `<callfixup>` in Rust and registers it through the same
  `PcodeInjectLibraryBase::decode_inject` path immediately after the cspec's own
  (`architecture.rs (decode_kuna_call_fixups)`), so the vendored spec tree stays
  byte-identical to upstream while `parse_inject_all` still compiles every body
  together. Its subject is the MSVC x86-32 float-to-integer CRT helper family
  (`__ftol`, `__ftol2`, `__ftol2_sse`). MSVC passes that helper's argument in the
  x87 stack top `ST0` and returns the `__int64` in `EDX:EAX`, but no vendored
  x86 prototype model has an `<input>` `<pentry>` naming an x87 register — `ST0`
  appears only as an `<output>` pentry — so `FuncProto::characterize_as_input_param`
  answers `NoContainment`, `Heritage`'s call guard never appends the argument,
  nothing reads `ST0`, and the entire feeding `FLD` chain is dead-code eliminated
  along with every register it was based on (on a `__thiscall` method, the `ECX`
  `this` pointer). Widening a shared prototype model with an `ST0` input pentry
  is *not* the alternative: it invents a phantom `float10` first argument on
  every unrelated stack-passing callee. The fixup body pops the return address
  the `CALL` pushed (mandatory — x86 `CALL rel32` lifts as
  `push44(&:4 inst_next); call rel32`, so a replaced CALL otherwise leaks the
  pushed address into the next call's arguments), truncates `ST0` at the helper's
  real 64-bit width into `EDX:EAX`, and pops the x87 stack. Registration is
  guarded to x86-32 (every register the body names must resolve, at a 4-byte
  code space) because the body would not compile elsewhere and the helper exists
  nowhere else; it is unconditional rather than option-gated because the
  architecture bootstraps at `load file`, before the console's `option` lines.
  Registration is skipped outright in ghidra mode, exactly as the Cortex-M
  callother fixup below is and for the same reason: with no local `.sla` there
  is nothing to compile the body against, so the payload could never install.
  Skipping it also keeps the x86-32 language test — which asks whether this
  language has `ST0` — off the wire, since every 32-bit language reaches it (the
  test's cheap half is the code-space width, which ARM32/MIPS32/PPC32 all pass).
  The user-visible gate is on the *install* instead (`option msvcftol`, default
  on), where the analysis-tier call-fixup installer drops this one payload's
  targets from its match map.
- **kuna-owned callother fixups.** The same freedom applies to the user-op side.
  `decompiler/crates/kuna-decomp/src/p2_lift/kuna_cortexmpriv.rs` supplies a
  `<callotherfixup>` body for the ARM user op `isCurrentModePrivileged`, which
  the vendored Cortex-M SLEIGH raises at twelve places. Every VERSION_7M MRS/MSR
  constructor in `ARMTHUMBinstructions.sinc` models its special-register move as
  a *runtime* privilege test — `b:1 = isCurrentModePrivileged(); if (!b) goto
  <notPriv>; <effect>` — so lowering the model literally gives each such
  instruction one extra basic block and two extra CFG edges that the source it
  came from does not have; a Cortex-M `irq_disable`/`irq_restore` pair drops four
  phantom branches into an otherwise straight-line function. The fixup body is
  the constant `1` (its single output operand carries no declared size, so the
  injector binds it to the real CALLOTHER output varnode), which lets the guard
  condition constant-fold and its block and edges die while the real effect
  survives untouched. `architecture.rs (register_cortexmpriv_fixup)` installs it
  between the cspec `<callotherfixup>` dispatch and `parse_inject_all`, only on a
  language whose translator presents the user op and only when no compiler spec
  has already specialized it — so a spec-declared fixup always wins, and every
  non-ARM target is unaffected by construction. That "the core is privileged" is
  an assumption rather than a proof (Cortex-M Thread mode really can run
  unprivileged, and the moves really do read as zero there) is why it is
  `option cortexmpriv`, default off and on in the `aggressive` preset. Because
  the architecture bootstraps before any `option` line, the flag is read at the
  *consumption* seam instead — `decompile_drive.rs (is_injected_userop)`, the one
  live per-CALLOTHER predicate that sees the applied options — so with the option
  off the CALLOTHER is never queued, survives as an ordinary black box, and the
  emitted C is identical to a build carrying no such payload.

## 2.2 CFG construction

Blocks are built only after *all* ops exist — the deferred second phase
(`flow.rs (FlowInfo::generate_blocks)`). First every referenced-but-undecoded
address gets a `missing` artificial halt so branches have a landing op
(`fillin_branch_stubs`), and the halts planted for addresses flow reached by
*leaving the declared extent* are registered in `visited` as the instruction
there, so `collect_edges` can resolve the cut edge to them. The rest are not:
an undecoded address inside the extent means an op that should exist does not,
and resolving that to a halt would silently shorten a function instead of
reporting the defect, so it keeps upstream's "Could not find op at target
address". Then `collect_edges` walks the dead list
pairing branch ops with their target ops — a BRANCHIND contributes one edge per
recovered jump-table entry, deduplicated, and contributes *no* edges when no
table was recovered (the partial-flow "assume no branches out" rule);
`split_basic` cuts the list into blocks at the `startbasic` marks planted
during classification; `connect_basic` materializes the edges. If the entry
block acquired an in-edge (a loop back to the function start), a fresh empty
entry block is prepended so the entry is always in-degree 0. Jump-table
recovery runs *between* the phases (§2.3) precisely because it needs blocks and
SSA over a function whose blocks are not built yet — it gets them on a clone.

The same machinery serves the restart loop: when a late pass requests a
restart, `decompile_drive.rs (run_pipeline)` re-follows flow on the cleared
`Funcdata` (`refollow_flow`; per-function overrides survive the clear, recovered
jump tables do not — they are re-recovered) and re-runs the action pipeline, at
most 8 cross-flow restarts before keeping the last analyzed IR.

**(angr) Tail-call jumps — `option tailcalljump`, default on (DIV-14),
`decompiler/crates/kuna-decomp/src/p2_lift/kuna_tailcalljump.rs
(kuna_is_tail_call_branch)`.** At `-O2` a "call X then return" tail compiles to
a direct `jmp X`. Decision rule: a `CPUI_BRANCH` whose direct machine-address target is the
entry of a *known function* (including a PLT thunk) that is not the current
function's own entry is a tail call. The rewrite lives in the BRANCH arm of
`flow.rs (FlowInfo::xref_control_flow)`: the BRANCH becomes a CALL with a full
call spec, an artificial RETURN is planted after it (unless the callee is
no-return, whose halt was already planted), and a
`tailcalljump: recovered tail call` warning makes the introduced call
attributable. Without it the follower walks *into* the PLT thunk, whose body is
an indirect GOT jump; jump-table recovery fails on it and the function renders
a `(*dat_...)(...)` computed call with a `"Treating indirect jump as call"`
warning. Two datatests (Long double #1/#2) opt out per-test.

**(kuna) Frame-teardown tail jumps — `option tailcallframe`, default on
(DIV-109), `decompiler/crates/kuna-decomp/src/p2_lift/kuna_tailcallframe.rs
(kuna_is_frame_teardown_tail_call)`.** `tailcalljump` above resolves the callee
through `query_call`, so it only fires on a target some discovery oracle already
found. Every kuna oracle reaches a function from a symbol, from an unwind record,
or from a direct `call` the recursive-descent Listing walk arrived at; a callee
reached *only* through a code pointer in initialized data satisfies none of them,
because the walk never enters the callback that calls it. Decompiling such a
callback by address then follows its tail `jmp` as ordinary intraprocedural flow
and decodes the entire callee into it — on the round-2 RE-friction witness (a
stripped Wayland/xkb PIE) the keyboard callback at `0x6500` emitted 1,555 lines
instead of 427, with the renderer at `0x4610` inlined inside it *and* called by
name a few blocks later. Decision rule: the tail jump is the caller's last
instruction, so the compiler tears the frame down before it and the `jmp`
executes with the stack pointer exactly where `ret` would find it. Two constant
stack-pointer deltas are measured over the already-decoded raw p-code — the
*prologue*, accumulated forward from the function's entry address, and the
*epilogue*, accumulated backward from the branch — and the branch is a tail call
when the epilogue is a strictly positive teardown of exactly the prologue's
frame. Both scans stop at the first control-flow op (so neither leaves the block
it starts in), at a stack-pointer write that is not `SP = SP ± <const>` (a
`leave`-style `SP = FP` restore is not modelled and declines), at an instruction
address more than 16 bytes from its neighbour (so a hole in the decoded stream
cannot make two unrelated instructions look adjacent), and after 24 instructions.
A frameless leaf can never match: with no frame torn down there is no evidence,
and an ordinary unconditional intraprocedural jump would be indistinguishable
from a tail call. Neither can the function's own entry (self tail recursion stays
a back-edge, as in `tailcalljump`) nor an address this function has already
decoded (already-decoded blocks are live flow, whatever the stack looks like).
The rewrite is the one `tailcalljump` already drives in the BRANCH arm of
`flow.rs (FlowInfo::xref_control_flow)`; `flow.rs (FlowInfo::tail_call_kind)`
asks `tailcalljump` first, so a known target keeps that path and that warning
text, and a `tailcallframe: recovered tail call` warning attributes the calls
this rule introduces. Byte-identical on both parity corpora, whose bytechunks
carry no such shape. **Known limit:** the evidence is the frame, not the function
bound — a kuna `FunctionSymbol` has no extent, so the rule cannot ask whether
`dest` is still inside the caller, and a jump that tears the frame down
*completely* before branching to a shared return sequence in the same function is
recovered as a tail call. That shape is not optimizer output: a shared return
sequence must be shared including its teardown, so the jump is emitted part-way
through the epilogue and the exact-cancellation test rejects it (gcc and clang at
`-O1/-O2/-O3/-Os` emit `add rsp,0x68; jmp <shared tail>` against a `-0x70`
prologue; a 26,458-function LLVM `-O2` corpus carries 103 sites of the raw shape
and none of them fire). Deferring the decision until the flow work-stack drains,
and then asking whether the function decoded `dest` by another path, is the sound
fix; it is a change to the walk's ordering rather than to this predicate.

**(kuna) Fall-through function bound — `option funcboundflow`, default on
(DIV-67), `decompiler/crates/kuna-decomp/src/p2_lift/kuna_funcboundflow.rs
(kuna_should_bound_at_entry)`.** A kuna `FunctionSymbol` is an entry address with
no extent, and CALL/CALLIND are fall-through, so flow stops only at a RETURN or a
*known* no-return callee (`query_call_no_return`). A function whose last act is a
`call` to a routine kuna cannot prove no-return — in a stripped, statically-linked
binary the unnamed `exit`/`abort`/`__stack_chk_fail` bodies and the app-level
`die()`/`throw` wrappers built on them — compiles with no trailing `ret`, just
inter-function alignment padding, and kuna's follower runs the padding's
fall-through straight into the next function's entry, decoding *that* function's
body into the current one (the following function is then emitted twice: once
correctly, once as a garbage tail of its predecessor). Decision rule: a
fall-through whose target is the entry of another *known* function
(`query_call(next).is_some()`), and is not the current function's own entry, has
run off the end of the current function. The truncation lives at the fall-through
push of `flow.rs (FlowInfo::process_instruction)`: instead of pushing the target,
a no-return artificial RETURN is planted (mirroring the `check_for_flow_modification`
no-return-call halt) and a `funcboundflow` warning makes the truncation
attributable. The halt also *starts a basic block*: `flow.rs (FlowInfo::collect_edges)`
emits a fall-through edge for every CBRANCH whatever else the walk decided, so when
the truncated instruction is a conditional branch the edge otherwise resolves back
to the branch's own block. That self-edge is what the structurer renders as the
condition-less, syntactically invalid `} while ;`, and where the branch's taken edge
lands on that same block the two collapse into a single out-edge that the two-way
block readers index off the end of. Giving the halt its own block puts a real
no-return RETURN on the far end of the fall-through instead. The halt is deliberately
*not* marked as an instruction start: `flow.rs (FlowInfo::fallthru_op)` can only
resolve the fall-through to it as a same-instruction op, the truncated address never
having been decoded. This overlaps `noreturn_extern`/`noreturn_externmatch` (which stop
the same leak by callee *name*) but is name-independent — it bounds at the function
*boundary* whatever the callee. On real binaries ~36% of application functions in a
measured static-pie build end in such a call and were corrupted this way; IDA and
Ghidra both bound decompilation to the function body. The `longdouble` datatest
(which deliberately flows a tail `jmp` into its callee and on across adjacent
functions) and the `ghangr-noreturn_extern` test (which isolates the
`noreturn_extern` toggle) opt out per-test.

**(kuna) `__fastfail` is a no-return — `option fastfailnoreturn`, default on
(DIV-120), `decompiler/crates/kuna-decomp/src/p2_lift/kuna_fastfailnoreturn.rs
(is_fastfail_callind)`.** x86 SLEIGH lifts `INT imm8` to `intloc = swi(imm8);
call [intloc]` — a `call` with no matching push, unlike every other x86 `CALL`,
which lifts as `RSP = RSP - 8; push &next; call target`. Nothing downstream
distinguishes the two, so the interrupt is an ordinary modelled call and the
compiler spec's `extrapop` hands its bytes back after it: on `x86-64-win.cspec`'s
default `__fastcall` (`extrapop="8" stackshift="8"`) every interrupt raises the
stack pointer by eight, and the printer says so — `(*(void *)swi(0x29))(5);`
followed by `v62 = &v61[8];`. The damage is not local. Once two paths join
carrying stack-pointer values eight apart, the frame stops being a constant
offset from the spacebase: stack locals degenerate into offsets off a `char *`,
each `CALL`'s return-address push (normally a dead store into a slot nothing
maps) survives as an explicit store, and outgoing stack arguments go the same
way, so a Win32 call renders with stack blobs where it takes values. Decision
rule: on a Windows image, a CALLIND that reads the storage a `swi` CALLOTHER
wrote in the same instruction, from the one-byte constant vector `0x29`, is
`__fastfail` — the MSVC `/GS` and STL `_STL_VERIFY` failure path, which
terminates the process and by contract never returns. Its call spec is marked
no-return in `flow.rs (FlowInfo::setup_callind_specs)` and the artificial halt
`check_for_flow_modification` plants for a named no-return callee is planted
here too, so the block ends at the interrupt and the unbalanced stack pointer
reaches no join. Unlike that path no `"Subroutine does not return"` warning is
buffered: the divergence is definitional rather than a surprise, and one
function can hold a dozen sites. The vector is checked exactly — `int 0x80` is
a Linux syscall (`option linuxsyscall`) and `int1`/`int3`/`into` carry a
`return` in their own SLEIGH semantics and do return — and the gate is the
compiler-spec component of the resolved language id, since `int 0x29` is
`__fastfail` by Windows convention alone.

**(kuna) Overlapping branch target — `option overlapbranch`, default on
(DIV-106), `decompiler/crates/kuna-decomp/src/p2_lift/kuna_overlapbranch.rs
(kuna_overlaps_pending_branch)`.** A conditional branch pushes both successors and
the follower pops the fall-through first, so the fall-through instruction is
decoded before the branch target is ever looked at. The x86 anti-disassembly
overlap exploits exactly that ordering: a `75 01` short JNZ hops over a junk `e8`
lead byte, so the fall-through decodes as one long bogus `CALL` that *swallows*
the branch target and desynchronises everything downstream — an out-of-image
callee, stores through never-assigned pointers, and invented `dat_` globals, all
of them artefacts of operand bytes read as opcodes. `FlowInfo::set_fallthru_bound`
notices the clash only afterwards, in `reinterpreted`, by which time the losing
stream and its successors are already built. Decision rule: the instruction just
decoded is the fall-through of the previous instruction's conditional branch, and
that branch's own target lies **strictly inside** its encoding (`curaddr < target
< curaddr + step`). Both ends are strict — `target == curaddr` is a branch to its
own fall-through and `target == curaddr + step` is a branch over one instruction,
both ordinary compiler output. **One legitimate overlap is excluded first**
(`kuna_streams_reconverge`): glibc's compiler-generated conditional-`LOCK`
idiom (`JE` over a `LOCK` prefix byte, e.g. `malloc_consolidate` in a static
binary) is the same instruction with its prefix taken or skipped, so the two
decodes END AT THE SAME ADDRESS and both streams are real — truncating the
fall-through there would delete an atomic store on a live path. The branch
target's own instruction length is therefore decoded and the rule declines
whenever `target + target_len == curaddr + step`, and likewise whenever the
target does not decode at all (following it would gain nothing over the decode
that did work). The test is architecture-neutral — no prefix-byte table — and
runs only after the strict-interior test has already matched, which it never
does in ordinary code. Ownership policy: **the branch target wins**,
because a branch target is an address the program *encodes* while a fall-through
is only ever inferred from the previous instruction's length, and because two real
instruction starts cannot sit at `next` and strictly inside `next` — whenever the
rule fires at least one of the two decodes is already wrong. The loser is
truncated in place, in `flow.rs (FlowInfo::process_instruction)` right after the
decode: the ops that decode just emitted are dropped (`delete_remaining_ops`), an
artificial RETURN marked `badinstruction` is planted at the loser's own address,
its recorded size is set to 1, and an `overlapbranch` warning makes the truncation
attributable. The conditional stays a conditional and its fall-through edge stays
in the graph — the edge simply ends in a halt — and the pending target is then
decoded on its own boundary by the ordinary `addrlist` walk. The halt is marked
`badinstruction` rather than `noreturn` because a `noreturn` halt is folded into an
empty `if (cond) { }` by `kuna_ifnoexit`, which reads as though the fall-through
continues. Nothing already committed to is deleted or re-anchored: the loser is
the instruction *currently* being decoded and the winner is still pending, which
is what keeps this out of the "repair the flow graph afterwards" class of change.

**(kuna) Stack-probe loops — `option stackprobeloop`, default on (DIV-3),
`decompiler/crates/kuna-decomp/src/p2_lift/kuna_stackprobeloop.rs
(RuleStackProbeLoop)` (from Ghidra issues GH-8017/6858).** gcc's
stack-clash/`-fstack-check` prologue probes a large frame one page at a time,
leaving the post-loop stack pointer as a self-referential phi
(`PHI = PHI - page`) the spacebase tracker cannot resolve — every post-loop
local renders as `&pxVar[-0x1000]` noise and argument stores at unmatched
offsets vanish from calls. The rule (it runs in the simplification pools but is
stack-pointer normalization, so it is specified here) matches the exact shape —
a two-input stack-pointer `MULTIEQUAL` whose back edge subtracts the page
constant and whose loop exit compares against a stack-relative limit — and
rewrites the phi to the value the exit comparison pins:
`INT_ADD(SP_in, limit_const - page)`. Inert on functions without a probe loop.

**(kuna) V850 indirect branch — `option v850indirectbranch`, default off,
`decompiler/crates/kuna-decomp/src/p2_lift/kuna_v850indbranch.rs
(kuna_is_v850_indirect_jmp)` (from Ghidra issue GH-8817).** The V850 SLEIGH
spec lifts `jmp [reg]` to a CALLIND, so a compiler switch dispatch never
reaches jump-table recovery (which is gated on BRANCHIND). When on, a CALLIND
through a named non-PC hardware register is reclassified to BRANCHIND at the
top of `xref_control_flow`. Kept opt-in per program because the same pattern is
a genuine computed call on other architectures.

## 2.3 Jump tables & switch recovery

A switch exists in the output only if a BRANCHIND op carries a recovered
`decompiler/crates/kuna-decomp/src/p2_lift/jumptable.rs (JumpTable)` — a map
from index values to code targets, attached to the op, with per-target case
labels. Everything in this section exists to manufacture, verify, or rescue
that artifact; when all of it fails the BRANCHIND is demoted to a CALLIND and
the switch (and any loop containing it) is destroyed.

### The recovery stage: a reduced sub-decompilation at flow time

The BRANCHINDs parked on `tablelist` during op generation are recovered before
block building, in a loop that re-fills flow from each new table's targets
(`flow.rs (FlowInfo::generate_ops_with_jumptables)`). Each round of that loop
ends by draining the pending p-code injections (§2.1) — the newly reached blocks
queue their own, and an injected body can itself introduce indirect branches, so
the loop re-runs while `tablelist` is non-empty. The address computation
feeding a raw BRANCHIND is unusable as lifted — it must be simplified first, but
the function has no blocks or SSA yet. So each attempt runs as a **reduced-tree
sub-decompilation on a clone**
(`decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs
(Funcdata::stage_jump_table)`, C++ `stageJumpTable`): the raw ops and existing
tables are cloned into a partial `Funcdata` (`"@@jumprecovery"`), its blocks are
built against the parent flow's `visited` snapshot, and the reduced
`"jumptable"` action set — heritage plus the simplification core, no
structuring — is run over it (`decompile_drive.rs (run_jumptable_pipeline)`).
The model is then recovered against the *partial*'s BRANCHIND and the finished
table is written back keyed to the real op. **One partial is built per
`recoverJumpTables` batch and shared by every table in it**
(`jtsharepartial`, default on): the C++ guards the clone plus the reduced
pipeline behind `if (!partial.isJumptableRecoveryOn())`, so upstream pays for the
sub-decompilation once per function however many BRANCHINDs it has, and kuna now
does the same. Recovery itself still runs per table against that shared partial,
and it mutates it (the emulation walks its ops), exactly as upstream's does.
Turning the option off restores kuna's older per-table clone, in which a later
table's fresh partial re-clones the siblings recovered before it; that is what
`unrolledguard` (below) needs to see, and it is the only reason the per-table
shape is still reachable. Sharing is worth about 16% of the wall time of a
two-table function, and on the interleaved shape `unrolledguard` was written for
it recovers the same tables without any tolerance rule, because the shared
partial's edge collection runs once while every sibling table is still empty.
Two pre-checks bound the attempt:
`funcdata_block.rs (Funcdata::early_jump_table_fail)` backtracks up to 8 ops
through value-preserving arithmetic looking for a computation the recovery can
never emulate -- but its only failing arm (the uninjected-CALLOTHER
classification) is a stubbed loss, so in live kuna the check always passes, and `funcdata_block.rs (Funcdata::test_for_return_address)`
recognizes a BRANCHIND whose input chains back to the saved return address —
that is a return, not a switch (`RecoveryMode::FailReturn`).

Failure demotes: `flow.rs (FlowInfo::truncate_indirect_jump)` rewrites the
BRANCHIND to a RETURN (`fail_return`, warning `"Treating indirect jump as
return"`) or to a CALLIND with an artificial RETURN after it (`fail_normal`,
warning `"Treating indirect jump as call"`; `fail_thunk` silently; a
CALLOTHER-computed target additionally marks the halt no-return) (unreachable today: nothing produces the CALLOTHER failure, see above). This is the
failure mode every rescue below is fighting for: one unbounded table turns a
switch into an opaque computed call.

### The JumpBasic model

The base recovery (`jumptable.rs (JumpBasicModel::recover_model_basic)`) is
Ghidra's JumpBasic: derive a *normalized switch variable* plus a *value range*
such that emulating the address computation for each in-range value enumerates
the case targets.

1. **Path meld.** Starting from the BRANCHIND input, walk the defining
   expressions backwards, depth-first, pruning at calls/phis/constant-free ops
   (`jumptable.rs (JumpBasic::find_determining_varnodes)`). The
   `jumptable.rs (PathMeld)` intersects all paths into the sequence of Varnodes
   *common to every path* — the candidates for the switch variable — ordered
   from the branch backwards.
2. **Guards.** `jumptable.rs (JumpBasicModel::analyze_guards)` walks up the
   CFG from the branch through at most 2 dominating CBRANCHes; each guard's
   branch condition is pulled back through at most 2 defining ops
   (`jumptable.rs (circlerange_pull_back)`, the op-coupled `CircleRange`
   pull-back, non-zero-mask-refined) into a `jumptable.rs (GuardRecord)`: a
   circular value range the guarded Varnode must lie in for control to reach
   the switch. A guard applies to a candidate if they are literally the same
   Varnode, quasi-copies of one base value, duplicate calculations, or loads of
   the same location (`jumptable.rs (GuardRecord::value_match)`).
3. **Range.** For each meld candidate, `jumptable.rs
   (JumpBasicModel::calc_range)` seeds a range from what the Varnode itself
   proves (a constant; a boolean output = {0,1}; an AND-mask bound + power-of-2
   stride from the non-zero mask) and intersects every matching guard range;
   ranges still larger than 0x10000 are assumed positive.
   `find_smallest_normal` picks the candidate with the smallest reaching range
   as the normalized variable — refusing a 1-byte, 256-value candidate unless a
   table LOAD lies between it and the branch (a bare byte is not evidence of a
   switch). One special case: if the meld is a single read-only Varnode, the
   "switch" is a jump through a read-only pointer; its value is read from the
   load image and the table has one entry.
4. **Accept or rescue.** If the chosen range exceeds `max_jumptable_size`
   (1024, `architecture.rs (reset_defaults_internal)`), the four kuna bound
   extensions below get one chance each, in order; if none installs a bound the
   model is declined, model 2 is tried, and then recovery fails with
   `"Could not recover jumptable ... Too many branches"`
   (`jumptable.rs (JumpTable::recover_addresses)`).
5. **Enumeration.** `jumptable.rs (JumpBasicModel::build_addresses_basic)`
   emulates the meld path once per in-range value on the one-path syntax-tree
   emulator (`decompiler/crates/kuna-decomp/src/p2_lift/kuna_emulatefunction.rs
   (EmulateFunction::emulate_path)`), masking each result by the architecture's
   function-pointer alignment; with `option jumpload` the table LOADs are also
   recorded as `jumptable.rs (LoadTable)` entries. A sanity pass
   (`jumptable.rs (JumpTable::sanity_check)`) truncates the table at the first
   null target or far target (> 0xffff from the first) with no loaded data
   behind it, rejects it outright if the *first* entry is bad, and classifies a
   1-entry table whose target is null or > 0xffff from the branch as a thunk
   (`"Likely thunk"`). A BRANCHIND sitting behind an already-collapsed constant
   guard is marked *partial* — recovered as far as flow allows, revisited by
   `jumptable.rs (JumpTable::recover_multistage)` (re-recover, restoring the
   saved model and addresses on failure).

**Model 2 — the default-path variant** (`jumptable.rs
(JumpBasicModel::recover_model2)`, C++ `JumpBasic2`). Some compilers merge the
out-of-range path back into the switch by loading a constant "default" target
into the same variable: the failed model-1 meld ends at a 2-input MULTIEQUAL,
one input a COPY of a constant. Model 2 re-runs the model-1 analysis restricted
to the non-constant path and iterates the range *plus* that one extra value
(`jumptable.rs (JumpValuesRangeDefault)`), so the default becomes an explicit
last entry; a dominance check (`check_normal_dominance`) decides whether the
normalization walk can proceed past the join.

**What is deliberately absent.** The CALLOTHER-assisted `JumpAssisted` model
(the `jumpassist` user-op family) and the manual `JumpBasicOverride` model are
unported shells: `jumptable.rs (JumpTable::set_override)` and the
`<basicoverride>` arm of `jumptable.rs (JumpTable::decode)` return errors, and
`recover_model` walks only JumpBasic/JumpBasic2 (Trivial exists only as the label-time fallback). Likewise upstream's
multistage *restart* accounting — persisting a table whose size disagrees at
`matchModel` time and restarting the whole function — is a recorded loss: kuna
keeps the flow-recovered addresses instead (`jumptable.rs
(JumpTable::match_model)`).

### The late check: labels, normalization folding, guard folding

A table recovered mid-simplification may disagree with fully-simplified
dataflow, so the model is re-derived late, against the finished function, by
`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionSwitchNorm)`: for each unlabelled table, `match_model` saves the
flow-time model and recovers a fresh instance (preferring a variable whose
range size matches the known table size), then
`jumptable.rs (JumpTable::recover_labels)` computes the *case labels* by
reverse-emulating the normalization chain from the normalized variable back to
the unnormalized one (`jumptable.rs (JumpBasicModel::backup2_switch)`, exact
inversion of at most 1 add/sub and 1 extension per the table's caps); a
non-reversible value labels `NO_LABEL` (rendered as the default). If no model
can be recovered at all but addresses exist from flow, a trivial model labels
the targets by index (`jumptable.rs (JumpModelTrivial)` — each target labeled
with its own address; table size = the block's out-edge count). `fold_in_normalization` then re-points the BRANCHIND
input at the unnormalized variable — the whole address computation becomes dead
code and the header renders `switch(V)` — and records how many bits of `V` the
switch actually consumes. Finally `jumptable.rs
(JumpBasicModel::fold_in_one_guard)` folds each surviving guard CBRANCH into
the switch: its out-of-range edge becomes the switch's *default* edge (adding
the target as a new label-less destination, or marking an existing destination
as default and collapsing the CBRANCH to a constant predicate); a fold clears
the structuring so the new edge is re-structured, and the constant-predicate
residue is severed on the re-run by `ActionDeterminedBranch`. Before
structuring, any table still without a default marks its most-targeted
out-edge as the default (`decompiler/crates/kuna-decomp/src/p8_structure/blockaction.rs
(ActionBlockStructure)` via `funcdata_block.rs (Funcdata::install_switch_defaults)`).

**Table lifetime.** A `JumpTable` is only as alive as the BRANCHIND it points
at, so removing that op has to remove the table. `funcdata_block.rs
(Funcdata::block_remove_internal)` looks up the table of a removed block's
trailing BRANCHIND (`funcdata.rs (Funcdata::find_jump_table_index)`, matched by
op address) and drops it (`funcdata_block.rs (Funcdata::remove_jump_table)`)
before destroying the block's ops; a table that outlived its op would be
re-recovered later against nulled inputs. `ActionSwitchNorm` carries the same
guard `install_switch_defaults` already had and skips any table whose indirect
op is missing, dead, or parentless, and the model walk
`jumptable.rs (JumpBasic::find_determining_varnodes)` reports a stale op as an
error rather than dereferencing it, so a table that slips through abandons its
own recovery instead of the whole function.

### (angr) Lowered-cascade recovery — `option loweredswitch`, default on (DIV-4)

GCC lowers a dense switch over a small variable into a balanced binary-search
tree of compares — no BRANCHIND, so no switch, and Ghidra (like stock angr)
renders a deep if/else chain.
`decompiler/crates/kuna-decomp/src/p2_lift/kuna_loweredswitch.rs` (port of
SAILR's `LoweredSwitchSimplifier`) detects the cascade and *manufactures* the
artifact. The two halves straddle a restart because the CFG surgery must not
strand phi state:

- **Detect** (`kuna_loweredswitch.rs (ActionLowerSwitchDetect)`) runs late —
  scheduled directly after `ActionSwitchNorm` — on the fully simplified CFG,
  and only reads: it collects the pure-compare blocks, canonicalizes each
  compared variable, groups by variable and takes the most-compared one, finds
  the cascade head (skipping a leading getopt-style `V == -1` sentinel guard),
  and walks the compare tree carrying angr's binary-search interval
  (`kuna_loweredswitch.rs (recover_cascade)`) to collect case→target pairs and
  default votes. Acceptance is deliberately narrow: at least 3 cases and 2
  distinct targets, at most 16 cases, at least one *range* node (a purely
  linear equality chain is a hand-written if/else-if — without this guard the
  flip regressed 10 upstream assertions, DIV-4), exactly one independent
  default *sink* (candidates that flow into another candidate are paths into a
  shared default, counted by a bounded CFG walk — a hand-written cascade whose
  arms land on independent bodies keeps every arm a sink and is declined), and
  the variable must live in register/stack storage. A hit is recorded in a
  **restart-surviving side store** (keyed by function identity, addresses only
  — no IR handles; the store lives on the Action, outliving the `clear()`) and
  a restart is requested.
- **Install** (`kuna_loweredswitch.rs (ActionLowerSwitchInstall)`) runs on the
  restart, scheduled before `ActionHeritage` and gated to heritage pass 0 —
  the pre-SSA window, where edge surgery needs no phi patching.
  `funcdata_block.rs (Funcdata::kuna_install_lowered_switch)` replaces the
  cascade head's CBRANCH with a synthetic `BRANCHIND(V)`, rewires its
  out-edges to the case targets plus default, pushes a hand-built, pre-labelled
  `JumpTable` (signed labels recorded when the recovered variable is signed)
  carrying a `JumpModelTrivial`, and sweeps the orphaned compare spine via
  unreachable-block removal. Heritage then rebuilds SSA over the corrected CFG
  and the ordinary structurer/printer emit the switch.

  Install declines whenever the rewiring would take a genuine switch with it.
  Severing the head's out-edges makes everything reachable only through the
  compare spine unreachable, and the trailing sweep deletes it; when one of
  those blocks ends in a BRANCHIND that *already* owns a recovered `JumpTable`,
  the cascade was a guard chain standing in front of a real jump table, not a
  lowered switch of its own. `funcdata_block.rs
  (Funcdata::kuna_lowered_switch_strands_table)` walks reachability from the
  entry over the post-surgery successor relation before any edit and refuses the
  install in that case, so the real table's cases survive instead of being
  replaced by the (much narrower) cascade dispatch. The pre-existing guard —
  a table already registered at the head's own branch address — never fires
  here, because the cascade head and the real BRANCHIND are different
  instructions. Both coreutils shapes are covered: `comm`/`join`/`uniq` `main`
  (a getopt dispatch split between a compare cascade over the short options and
  a PIC relative-offset table over the dense long-option range) declines, while
  `mv`'s `main` — a cascade with no downstream table — still installs.

One repair hook closes the loop: heritage may widen the synthetic BRANCHIND's
storage read and null its input, so `funcdata_block.rs
(Funcdata::kuna_repair_lowered_switch_inputs)` (driven from
`decompiler/crates/kuna-decomp/src/p3_dataflow/coreaction_early.rs`) re-points
it at the live reaching SSA def of the recorded storage — accepting a
heritage-known input (written, function input, *or constant*) as healthy. The
constant case is load-bearing: `ActionConditionalConst` may legitimately prove
the switch variable constant on a guarded edge, and classifying that constant
as broken made repair and cond-const toggle the same input forever — the fixed
infinite-loop hang on stripped openssh/bash binaries (`tests/hang-repro/`).

### The selector the install cannot re-read — `option switchselector`, default off

Detection and installation do not see the same graph, and nothing checks that
the install's half of the bargain succeeded. `ActionLowerSwitchDetect` runs on
the fully simplified, SSA'd CFG and records the cascade's switch-variable
*storage*; `Funcdata::kuna_install_lowered_switch` runs pre-SSA on the
**re-lifted** raw p-code of the restart and has to re-find that variable. It has
two arms — recover the live Varnode from the head comparison
(`kuna_head_switch_var`), else fall back to a free read of the recorded storage
— and it commits the CFG surgery whatever it ends up holding. When both arms
fail, the emitted `switch` dispatches on something that is not the switch value.

On x86 the head arm is essentially never the productive one: `cmp`/`jcc` reaches
the install as flag arithmetic (`ja` is `!(CF|ZF)`, a `BOOL_NEGATE` over a
`BOOL_OR`), not as a comparison, so the recovery returns nothing and everything
rests on the fallback read. That read is re-linkable only when heritage can find
it a reaching definition at pass 0. A **register** always can. A **stack slot the
function writes** can, once stack-pointer normalization has run. A stack slot
that is a function **input** cannot: there is no definition to find — the
incoming value is still a `LOAD` through the frame pointer at that point, and the
input Varnode set is rebuilt by parameter recovery only *after* heritage. The
BRANCHIND's input collapses to a constant, and the surgery has already committed.

That is the Win32 callback shape. A `DialogProc` dispatching on its `uMsg`
parameter renders as `switch(0)`: every `WM_INITDIALOG`/`WM_COMMAND` case
present and unreachable, and the parameter the function dispatches on absent from
the recovered prototype, because after the collapse nothing reads it.

`kuna_loweredswitch.rs (install_can_reread)` is the guard: at detection time,
where the switch variable is a real SSA Varnode, refuse to record a cascade whose
variable is a function input outside the register space. Declining leaves the
comparison cascade exactly as the compiler lowered it — an `if`/`else if` chain
over the real variable, which is correct C that names the parameter. The test is
deliberately narrow: a stack local the function itself writes is still recorded,
because the fallback read does resolve for it (measured — the guard applied to
*every* stack-located switch variable also cost a genuine `switch` in the same
binary, and that is what narrowed it to the input case).

Measured: byte-identical on the 675-assertion datatest corpus, and across
seventeen crackmes-corpus binaries exactly one function changes — the witness,
where five unreachable cases become a five-branch `if`/`else if` chain over the
recovered parameter. It ships off only because turning it on is a DIV-registry
default change.

### Bound extensions: rescuing an unboundable table

Four gated extensions run, in this order, only when JumpBasic's range exceeds
the table cap; each installs a `[0, N)` bound on a chosen index Varnode and
hands back to the enumeration core — at most re-deriving the path meld and
seeding loop-invariant values (`switchsharedcase`); the enumeration core itself
is never changed, only what counts as a bound.

- **(kuna) `switchmodbound`, default off (from Ghidra issue GH-9191) —**
  `jumptable.rs (JumpBasicModel::kuna_try_modulo_bound_table)` (option shell:
  `decompiler/crates/kuna-decomp/src/p2_lift/kuna_switchmodbound.rs`). Accepts
  an *in-band* bound: the meld path from the BRANCHIND, through realigning ops
  and exactly one table LOAD, reaches an `INT_REM`/`INT_SREM` by a constant `N`
  or an `INT_AND` with a contiguous low mask (`bound = mask+1`); the index is
  re-bound to `[0, N)`, `N ∈ [2, max_jumptable_size]` (default 1024), starting emulation at the already-
  reduced modulo result. Opt-in: on a program whose indirect jump genuinely has
  no modulo bound it can over-bound an unrelated computed jump.
- **(angr) `switchguardbound`, default off —**
  `jumptable.rs (JumpBasicModel::kuna_try_guard_bound_table)` (option shell:
  `decompiler/crates/kuna-decomp/src/p2_lift/kuna_switchguardbound.rs`).
  Accepts an *out-of-band* CBRANCH range guard that guard analysis missed —
  the GCC `sub LOW; ja DEFAULT` dispatch where, on the early partial-function
  run, the guard is still unsimplified x86 flag arithmetic (the pull-back
  extracts no bound) and the index is spilled to the stack between test and
  load (so the guarded Varnode never value-matches). Rather than
  pattern-matching comparison constants, `scan_guard_tree` *evaluates* the
  guard's boolean as a function of a candidate meld index (sibling meld values
  resolved through a linear-offset map) for `v = 0, 1, …` and takes `N` = the
  first value whose routing flips from `v = 0`'s. Opt-in for the same
  over-bounding reason: the guard↔index correspondence is asserted across a
  memory round-trip dataflow cannot prove.
- **(angr) `switchsharedcase`, default on (DIV-14) —**
  `jumptable.rs (JumpBasicModel::kuna_try_loop_carried_guard_table)` (option
  shell: `decompiler/crates/kuna-decomp/src/p2_lift/kuna_switchsharedcase.rs`).
  Rescues the GCC PIC relative-offset table (`target = base +
  sext(load4(base + idx*4))`) whose `lea .rodata` base register is set *before*
  a getopt-style loop while the BRANCHIND sits inside it: the base reaches the
  jump through a loop-header phi, so the meld collapses to the final add and
  the index guard never bounds anything. The walk rebuilds a clean single path
  from the BRANCHIND down to a guard-bounded load index, identifies the base as
  the unique read-only-image constant reachable through the COPY/phi tree, and
  re-runs normalization with the base pre-seeded into the emulator
  (`kuna_emulatefunction.rs (EmulateFunction::seed_varnode_value)`). Slower on
  exactly the functions it rescues; declines restore the saved model.
- **(angr) `switchmultipred`, default on (DIV-14) —**
  `jumptable.rs (JumpBasicModel::kuna_try_multipred_guard_table)` (no
  dedicated module; the option row lives in
  `decompiler/crates/kuna-decomp/phases.toml`). Rescues the dispatch whose
  bound guard is duplicated — "unrolled" — across *multiple* predecessors of
  the BRANCHIND block, the per-path indices meeting in a MULTIEQUAL (angr's
  "abnormal switch case", e.g. an MSVC memmove small-count tail). The same gate
  also arms the upstream-faithful `jumptable.rs
  (JumpBasicModel::check_unrolled_guard)` inside guard analysis (a no-op when
  off), whose lockstep walk only fires when the *same* guard is duplicated on
  every path; when the per-path guards are semantically different (entry
  `count <= 16` vs back-copy `count & 7 != 0`) the fallback evaluates each
  predecessor's guard as a function of its MULTIEQUAL input (same
  first-routing-flip evaluation as `switchguardbound`, trampoline blocks
  peeled up to 4 deep) and re-binds the table to the *union* — the max — of
  the per-path prefixes.

**(angr) `unrolledguard`, default off** — despite the name, not a guard
analysis: a partial-flow tolerance in `flow.rs (FlowInfo::collect_edges)` for
the MSVC optimized-memcpy shape where several *interleaved* tables' case bodies
are only reachable as one another's case targets. With `jtsharepartial` **off**,
kuna recovers tables one at a time, each in a fresh partial clone that re-clones
already-recovered siblings; the clone's edge collection then hits a sibling case
body that was never decoded into *this* partial's `visited` and throws
`"Could not find op at target address"`, demoting a recoverable dispatch. With
the gate on, an unresolvable recovered-table case-target edge inside a recovery
clone is skipped instead (the same "assume no branches out" shape as the no-table
path). Opt-in because on a truly malformed table it would mask a real missing
target instead of declining. Under the shipped `jtsharepartial on` default the
condition it tolerates does not arise at all — there is one partial, built before
any sibling recovered — and the memcpy witness recovers all sixteen dispatches
with this gate off, so the two settings are paired: turn `unrolledguard` on only
together with `jtsharepartial off`.

## 2.4 No-return at lift time

The mechanism is §2.1's halt plant: if `check_for_flow_modification` believes a
call never returns, an artificial noreturn RETURN lands right after the CALL
and flow stops. The *facts* come from
`decompile_drive.rs (ArchFlowEnv::query_call_no_return)` as an OR of three
sources, checked in order:

1. the resolved callee symbol's no-return flag — set by `option noreturn`, by a
   declared prototype, or by the analysis-tier passes (`noreturn_known`, and
   the call-graph fixpoint `noreturn_propagate`) described in chapter
   [01 — Program preparation](01-program-prep.md);
2. **(angr) `noreturn_externmatch`, default on (DIV-13)** —
   `decompiler/crates/kuna-decomp/src/p2_lift/kuna_noreturn_externmatch.rs
   (is_known_noreturn_name)`: the callee *display name* matched against the
   same vendored list the analysis tier uses
   (`decompiler/crates/kuna-analysis/data/ElfFunctionsThatDoNotReturn`,
   build-time-included so the two matchers cannot drift), with the same
   all-leading-underscore strip, global/`std`-only namespace guard, and
   trailing-`*` wildcard-prefix support;
3. **(angr) `noreturn_extern`, default on (DIV-14)** —
   `decompiler/crates/kuna-decomp/src/p2_lift/kuna_noreturnextern.rs
   (matches_noreturn_extern_name)`: an exact-match check against a closed,
   hard-coded ELF no-return name set with the same namespace guard.

Both name matchers exist for the case the address-keyed analysis pass
structurally cannot reach: in an ET_REL `.o`, `__stack_chk_fail` is an
undefined extern — no definition, no address, no PLT — so no address-keyed fact
is ever emitted, and without the match flow runs off the function's end into
inter-function alignment padding (`00 00` decoding as `add byte ptr [rax], al`),
swallowing neighbour functions in garbage. On a normal dynamically-linked ELF
the proto flag is already set and the OR is a no-op. This *removes code* by
design — the fall-through past a matched call is dropped as unreachable — so
the match surface is kept deliberately narrow (exact names, closed lists, no
class methods): the failure mode of a false positive is truncating live code
after a returning callee that happens to share a listed name.

## 2.5 Arch quirks

**(kuna) SPARC struct return — `option sparcstructret`, default off,
`decompiler/crates/kuna-decomp/src/p2_lift/kuna_sparcstructret.rs
(kuna_is_sparc_struct_ret_trap)` (from Ghidra issue GH-6882).** The SPARC ABI
plants an `unimp <structsize>` word after a call to a struct-returning
function; the SLEIGH spec lifts it to an `IllegalInstructionTrap` CALLOTHER
feeding a BRANCHIND, which jump-table recovery can never resolve — so the
function loses its tail to a non-returning CALLIND. The predicate, consulted in
the BRANCHIND arm of `xref_control_flow`, identifies the idiom *positionally*
(pre-SSA the input is not def-linked): walk backwards over the dead list within
the same instruction looking for a CALLOTHER whose user op is named
`IllegalInstructionTrap`. On a match the BRANCHIND is destroyed and the
instruction falls through. Kept opt-in per program: globally it would convert a
*real* trap into silent fall-through on other targets.

**Emulate-function hooks.** `kuna_emulatefunction.rs (EmulateFunction)` is the
lightweight emulator behind every address enumeration in §2.3: a memory state
keyed by Varnode, constants read off the IR, RAM/register reads pulled from the
load image, and exactly *one* execution path — MULTIEQUAL inputs are selected
by which block the previous op came from, LOADs optionally collected as table
records, and any CALL/CALLOTHER is ignored while a nested branch op aborts the
path (the meld guarantees straight-line evaluation). Its one (kuna) extension
is the pre-seeding hook (`seed_varnode_value`) that lets `switchsharedcase`
inject the loop-carried table base — a register value that exists in no load
image — before each path walk.

## 2.6 Cleanup-call removal

**(oxidizer) `option cleanupcode`, default on (DIV-81),
`decompiler/crates/kuna-decomp/src/p2_lift/kuna_cleanupcode.rs
(ActionRemoveCleanupCode)`.** Rust's automatic resource management emits a
drop-glue call at every scope exit and every `?` early return. None of those
calls is in the source, none carries meaning for a reader, and on a real binary
they are the single largest source of emitted lines in a Rust function — the
showcase `FakeCrypt::fileops::encrypt_file` carries eight of them. This pass
deletes them, following SEFCOM Oxidizer's `CleanupCodeRemover`.

**What is deleted.** A *direct* CALL whose recovered callee display name
normalizes to one of exactly five paths: `core::ptr::drop_in_place`,
`core::ops::drop::Drop::drop`, `alloc::raw_vec::RawVecInner::deallocate`,
`__rust_dealloc`, `__rustc::__rust_dealloc`. Oxidizer's list also carries
`free`, `close` and `_close`; those are **deliberately not ported**, because
kuna's primary corpus is C binaries and deleting `free()` from C output would
be a catastrophically wrong answer. What remains cannot occur in a C program,
which is what makes the pass *structurally inert* on a C binary — the reason it
can default on without a compiler-detection channel from the loader to the
engine (there is none).

**Matching is normalize-then-compare-exactly, never `starts_with`.** kuna's
demangler leaves a legacy-mangled Rust symbol with its generic arguments in
escaped form, so the names that actually arrive look like
`core::ptr::drop_in_place$LT$std..fs..File$GT$`,
`alloc::raw_vec::RawVecInner$LT$A$GT$::deallocate` and
`_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$::drop`.
`kuna_cleanupcode.rs (normalize_rust_name)` reproduces Oxidizer's
`normalize(monopolize=True, use_trait_name=True)`: strip a trailing
`::h<16 hex>` disambiguator, un-escape the `$LT$`/`$GT$`/`$C$`/`$uXX$`
sequences and the `..` path separator, then repeatedly collapse the *innermost*
angle-bracket group — deleting a plain generic argument list (with the `::` of a
turbofish), and replacing a `<T as Trait>` qualified path with the **trait**
name. The three examples above reduce to the first, third and second list
entries. Innermost-first is what makes the ` as ` split safe: the inner groups
of a nested qualified path are already gone when it runs.

Normalization only ever shrinks a name and the comparison is exact, so nothing
outside those five paths can be reached. `FakeCrypt::fileops::drop_ransom_note`
is untouched (the reason a prefix test would be wrong), and so is
`<T as crossbeam_epoch::atomic::Pointable>::drop`, which normalizes to a
*different* trait's `drop`. Oxidizer's `smallvec::deallocate` is not carried:
no available binary witnesses that path, and an unverifiable entry in a delete
list is worse than a missing one.

**The seam is pre-SSA, and that is the whole point.** The Action is registered
at the top of `mainloop` (`infra/universalaction.rs (universal_sched)`) in the
`deadcode` group and self-gates on `get_heritage_pass() == 0` — the same
pre-SSA window `kuna_loweredswitch`'s install half and `kuna_outline` use, and
the analogue of Oxidizer's `STAGE = BEFORE_VARIABLE_RECOVERY`. Before heritage
there are no INDIRECT call-effect ops to unpick and no MULTIEQUAL to patch;
CALL is not a control-transfer op, so no edge moves and the CFG is untouched.
The payoff is dead code: the register and stack writes that existed only to set
up the drop's arguments lose their last reader and are collected by the ordinary
`ActionDeadCode` fixpoint, so the setup goes with the call instead of being left
behind as unexplained assignments. Removing the call after SSA would keep all of
it.

Each victim is removed with the stock pair `Funcdata::delete_call_specs` then
`Funcdata::op_destroy` — what `Funcdata::block_remove_internal` already does for
a CALL inside a deleted block. A call whose output Varnode still has a reader is
skipped rather than destroyed (a cleanup routine returns `()`, so in practice
none does); declining is always safe, freeing a read Varnode is not.

This *removes code with real side effects* — the drop really did release the
resource — which is why the option is marked destructive. Turn it off to audit
when a value is actually dropped or freed; with it off the rendering is
byte-identical to a build without the pass.

## 2.7 Linux `int 0x80` syscall naming

**(kuna) `option linuxsyscall`, default off,
`decompiler/crates/kuna-decomp/src/p2_lift/kuna_linuxsyscall.rs
(ActionLinuxSyscall)`.** x86 SLEIGH lowers `INT imm8` to a black-box userop
feeding an indirect call —
`tmp:1 = imm8; intloc:4 = swi(tmp); call [intloc];` — which is the honest
lifting of a *general* software interrupt: the spec cannot know which operating
system is behind the vector. Left alone it renders as
`(*(void *)swi(0x80))();`, a call through a pointer nothing assigns. The damage
is not only the missing name. The call has no recovered parameters, so the
register writes that set the syscall up have no reader, and the ordinary
dead-code fixpoint collects them: on a hand-written `int 0x80` program the
number *and* every argument leave the output together. On 32-bit Linux both the
vector and the ABI are fixed — the number is in `EAX`, the arguments are `EBX`,
`ECX`, `EDX`, `ESI`, `EDI`, `EBP` in that order, the result comes back in `EAX`
— so the information is recoverable, and this pass recovers it.

**Recognition is structural, not dataflow.** The Action runs in the pre-SSA
window (registered in `infra/universalaction.rs (universal_sched)` immediately
after `ActionConstbase`), where the `swi` output is a *free* Varnode with no
def-use edge to follow. So the lowering is identified by adjacency inside one
instruction: walking the op bank's CALLOTHER list, a two-input CALLOTHER whose
second input is the one-byte constant `0x80`, whose next op in the same block
and at the same instruction address is a CALLIND, and whose output storage is
that CALLIND's target. Nothing else in the x86 lifting has that shape.

**The number comes from a bounded backward walk.**
`kuna_linuxsyscall.rs (syscall_number_before)` walks the CALLOTHER's basic block
backwards and accepts exactly one thing: a full-width `EAX = <constant>` COPY.
It stops — declining — at the first op that writes any part of `EAX` by any
other means, at any call or branch (whose effect on `EAX` it cannot see), and at
the top of the block. `xor %eax,%eax; inc %eax; int $0x80` is therefore
*declined*, not folded: the fold would need the constant propagation that only
exists after SSA, and by then the argument setup this pass exists to save has
already been collected.

**The table is a subset by construction.**
`kuna_linuxsyscall.rs (SYSCALL_TABLE)` maps a number to a name *and an argument
count*; a number with no entry is declined. Names and numbers come from the
kernel's `asm/unistd_32.h`; the argument count is the arity of the syscall's
documented prototype read from its section-2 manual page, kept only where that
reading is unambiguous — 332 of the 438 `__NR_` names. Four numbers whose i386
entry point takes a different register set than the documented wrapper are
removed by hand (`select` and `mmap`, whose i386 entry points take one pointer
to an argument struct; `sigsuspend`, which carries two unused history words;
`ipc`, a multiplexer), and nineteen are set by hand from the kernel entry point
where the manual page documents the wrapper instead (`exit`, `open`, `ioctl`,
`clone`, `rt_sigaction`, the `*64` stat family, `openat`, …). The arity is not
optional detail: printing a known name with a *guessed* argument count states
something false about the call, where declining states nothing.

**The rewrite retargets the CALLIND in place.** It already owns a
`FuncCallSpecs` slot and a block position, so the pass sets its opcode to CALL,
replaces input 0 with a fresh fspec annotation, names the spec `sys_<name>`
(`FuncCallSpecs::set_funcdata`) and destroys the now-unread `swi` CALLOTHER. The
name carries the `sys_` prefix deliberately: the raw syscall returns `-errno`
where the libc wrapper returns `-1` and sets `errno`, so rendering it as
`write(...)` would assert a call that is not being made — and on a dynamically
linked image the real `write` PLT stub is in the same output.

Two properties of the synthesized prototype are load-bearing:

- **The inputs are locked** to the first N argument registers
  (`FuncProto::set_param` + `set_input_lock`). A locked prototype is what makes
  `ActionFuncLink` materialize the argument Varnodes itself, exactly as it does
  for a declared callee — which is why the `mov $1,%ebx` feeding the call keeps
  a reader and survives to the output. This is also why the Action is scheduled
  *before* `ActionFuncLink` rather than in `mainloop`.
- **`extrapop` is zero** (`FuncProto::set_extra_pop`). `int 0x80` pushes no
  return address, where a normal x86 `CALL rel32` lifts as `push &next; call
  target` and the default `__cdecl` `extrapop` of 4 is what
  `ActionExtraPopSetup` uses to hand `ESP` those four bytes back. Applying it
  here — which is what the plain indirect call gets today — moves `ESP` across
  every syscall and shifts every later `ESP`-relative reference in the function.
  With `extrapop == 0` `ActionExtraPopSetup` emits no adjustment at all.

**The language gate** is `kuna_linuxsyscall.rs (resolve_abi)`: every ABI
register must resolve at its full 32-bit width and the default code space must
be 4 bytes wide. x86-64 resolves `EAX`..`EBP` as sub-registers, so the
address-size test is what excludes it — there `int 0x80` is a compatibility
path, not the syscall ABI this models — and every non-x86 language is excluded
because the register names do not resolve. That resolution is the speculative
probe of §0, not the exact lookup, so in ghidra mode the gate sees only names
the register cache already holds; the seven it needs are there because the
compiler spec's `<prototype>` elements — which name all of them — are decoded
during `registerProgram`, before the first function is lifted.

**Why it ships off.** Naming the call asserts that the operating system behind
vector `0x80` is Linux. That is true of essentially every 32-bit x86 ELF, but
the vector alone does not prove it (on the original IBM PC the `0x80`–`0xF0`
range was reserved for BASIC) and the engine has no OS/ABI channel it could
consult at this seam. So the assertion is left to the operator, and
`linuxsyscall` is a documented exclusion from the `aggressive` preset
(`p0_knowledge/modes.rs`) rather than an unevaluated one.

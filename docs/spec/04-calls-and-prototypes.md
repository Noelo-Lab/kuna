# 04 — Calls & prototypes

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/p4_calls
```

This phase computes the **interface contract of every call**: which storage
locations carry parameters into each sub-function call, which location carries
each call's return value, and what the analyzed function's *own* prototype is.
Its artifacts are one `FuncCallSpecs` per CALL/CALLIND site and one `FuncProto`
for the function itself (`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs
(FuncCallSpecs, FuncProto)`). Everything runs in two directions over the same
storage model: the **assignment** direction (a declared prototype's data-types
are mapped to registers/stack per the calling convention, §4.1) and the
**recovery** direction (data-flow *trials* observed at the call are scored
against the convention until a parameter list emerges, §4.1–§4.2). Untagged
prose describes the Ghidra-derived port; scheduling is chapter 00 §0.6 — the
setup passes run once in the outer restart group, the trial passes co-evolve
with SSA/dead-code/types inside mainloop (Band B), and the one-shot prototype
fixation runs in the tail. Option metadata lives in the generated catalog,
[`docs/options.md`](../options.md), and is not repeated here.

## 4.1 Prototype models

### The model

A `ProtoModel` (`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs
(ProtoModel)`) is one named calling convention: an input and an output resource
list (`ParamListStandard`), the *extrapop* (how far the callee moves the stack
pointer past the return-address pop; `EXTRAPOP_UNKNOWN = 0x8000` means
"callee-cleanup, amount unknown"), the side-effect lists (§4.3), the
likely-trash and internal-storage register lists, the local/parameter stack
ranges, and optional entry/return p-code injections. Models are decoded from
the compiler spec at engine build: `decompiler/crates/kuna-decomp/src/infra/architecture.rs
(decode_default_proto, decode_pentry_list, decode_effect_block)` parses the
cspec's `<default_proto><prototype>` — its `<pentry>`/`<group>` storage
entries, `<rule>` model rules, a synthetic pointer-conversion rule when the
list carries a `pointermax` attribute, and the
`<unaffected>`/`<killedbycall>`/`<returnaddress>`/`<internal_storage>` effect
blocks — and registers the result as the default model
(`Architecture::register_model`).

The spec's **named** models are registered alongside it, in document order
(`decompiler/crates/kuna-decomp/src/infra/architecture.rs (decode_named_protos,
decode_resolve_proto)`, mirroring the `<prototype>`/`<resolveprototype>`/
`<modelalias>` arms of the C++ `parseCompilerConfig` dispatch). A top-level
`<prototype>` decodes through the same body as the default one, so a named
model carries identical storage and effect fidelity; it additionally reads the
`hasthis` and `constructor` attributes, and the name `__thiscall` forces
`hasThisPointer` whatever the attribute said. A `<resolveprototype>` builds a
`ProtoModelMerged` by folding in each `<model name=…>` constituent and
finalizing the merged input list; a `<modelalias>` registers a named copy of an
already-registered parent, which stays `isCompatible` with it. Unlike the C++,
which aborts the whole spec on a malformed element, a named model that fails to
decode (an unknown strategy, a `<pentry>` naming a register the language does
not have) is skipped: the vendored cspec corpus spans every processor, and one
undecodable named model must not cost the architecture its default one.

Upstream's post-parse invariant is honored at the tail: **every language has a
`__thiscall` model**. Most specs do not declare one (only the x86 family and a
handful of others do), so when the pass ends without one it is cloned off the
default under that name — and the name rule then gives the clone
`hasThisPointer`. Aliasing a merged model, or an alias of an alias, is refused
exactly as upstream refuses it.

Registration selects nothing. Which model a function is evaluated with is
unchanged by the presence of the named ones: nothing reads the registry except
`option defaultprototype` / `option protoeval`
(`decompiler/crates/kuna-decomp/src/p0_knowledge/options.rs (OptionDefaultPrototype,
OptionProtoEval)`) — the ABI-trust knob of the `abi-trust` sub-phase row in
`decompiler/crates/kuna-decomp/phases.toml`. Those options are what the registry
makes usable: on an x86 PE target `option defaultprototype __thiscall`
resolves and recovers the ECX `this` pointer as the first parameter, where
before it failed with "Unknown prototype model". Automatic
assignment of `__thiscall` to member functions (from the demangler or from DWARF
`DW_AT_object_pointer`) is not wired.

Registration is not the whole story, because a spec can also **nominate** one of
its registered models for evaluating a function's own unlocked prototype:
`<eval_current_prototype name=…>` (`evalcurrentproto`, default-on;
`decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_evalcurrentproto.rs
(eval_current_model_name)`). The nominated model is looked up at
`<default_proto>` time and handed to each function through the arch handle, where
`ActionPrototypeTypes` installs it on any prototype that is not model-locked —
the C++ `evalfp_current` slot. Six vendored specs nominate one, always a merged
model: `x86win` (`__fastcall/__thiscall/__stdcall`), `x86borland`, `x86gcc`
(`__cdecl/__regparm`), `CR16`, `HCS12` and `HCS12X`; every other language leaves
the slot empty and evaluates with `<default_proto>` as before. Nominating a model
outranks `option defaultprototype` for an unlocked prototype (that option sets
the *default* model, which the nomination replaces); an explicit
`option protoeval` outranks the nomination in turn, since both write the same
slot. Turning `evalcurrentproto` off restores `<default_proto>`-only evaluation.

What the nomination buys is the **merged-model machinery** — a
`ProtoModelMerged` union whose `FuncProto::resolveModel` picks the constituent
best fitting the observed trials via `ScoreProtoModel`
(`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs (ProtoModel::select_model,
ScoreProtoModel)`) — which was fully ported but had no live producer: `resolve_model`
short-circuits on a non-merged model, and the default model is never merged, so
before the nomination was read the union only ran when a merged model was named by
hand. That is what left an x86 Windows `__fastcall`/`__thiscall` function rendering
as `(void)` with its `ECX`/`EDX` arguments surviving as locals read before they are
written: `__stdcall`, the `<default_proto>`, has stack-only `<input>` entries, so a
register argument is not a *possible* parameter and never becomes a trial.
Resolution is per function, so a function that touches neither register still comes
out `__stdcall`.
The scorer itself is simple:
each trial is mapped to a resource slot; holes in slot coverage are penalized
16/10/7/5 for the first four missing slots and 3 thereafter, a duplicated slot
or an unmappable trial costs 20, lowest total wins, starting threshold 500
(`ScoreProtoModel::do_score`).

### ParamEntry: the storage atoms

A `ParamEntry` (`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs
(ParamEntry)`) is one range of memory usable for parameter passing. Two shapes:
an **exclusion** entry (`alignment == 0`) holds exactly one parameter (a
register — using EAX consumes the whole RAX group), and an **aligned resource**
is carved into slots (the stack parameter area). Each entry carries a storage
class (`decompiler/crates/kuna-decomp/src/substrate/dtype.rs (type_class)` —
general/float/pointer/hiddenret/vector, plus the reserved class1–class4), the group(s) it occupies, minimum and
maximum value sizes, endian-aware justification, and the extension the model
assumes for undersized values (zero/sign/float/int-dependent). The
output-determining queries are containment and justification: does a given
range lie in an entry covering the least-significant bytes
(`ContainsJustified`), cover more-significant bytes only
(`ContainsUnjustified`), contain the entry outright (`ContainedBy`), or miss
(`Containment::NoContainment`)? Entry lookup goes through a range-map resolver built once per
list (`ParamListStandard::populate_resolver`).

The `ParamListStandard` kinds (`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs
(ParamListKind)`) collapse the upstream subclass tree into one struct: `Standard`
(ordered input resources), `StandardOut` / `RegisterOut` (return-value storage),
`Register` (unordered register sets — order-free conventions), and `Merged`.

### Assignment: declared types → storage

`ProtoModel::assign_parameter_storage` maps a declared prototype
(`PrototypePieces`) to concrete storage: the output list first, then the input
list, each walk threading a per-group `status` array so an exclusion group one
parameter consumes blocks every later parameter in that list
(`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs
(ProtoModel::assign_parameter_storage, ParamListStandard::assign_map)`); the
output hands its verdict to the input walk through the result list itself (a
hidden-return marker there claims the first input slot). Per
parameter, `ParamListStandard::assign_address` tries each decoded `ModelRule`
in order — first non-fail response wins — and only when every rule fails falls
back to the classic algorithm: map the type's metatype to a storage class and
take the first unconsumed entry of that class (or a general one) that fits
(`assign_address_fallback`). A too-big return value degrades to the
**hidden-return** protocol: the output is rewritten as return-by-pointer
(`INDIRECTSTORAGE`) and a synthetic pointer parameter is prepended to the input
list, drawn from the dedicated hidden-return class or the normal pointer slots
(`assign_map_standard_out`, response codes `hiddenret_*`). A `__thiscall`-style
model then marks the right input as the `this` pointer, swapping markup when
the hidden-return pointer bumped it. Failure mode: an unassignable *input*
raises a hard error; an unassignable *output* is only survivable where the
caller opted into `ignore_output_error`, which degrades the return to `void`.

The rules themselves live in
`decompiler/crates/kuna-decomp/src/p4_calls/modelrules.rs (ModelRule,
AssignAction, DatatypeFilter, QualifierFilter)`. A `ModelRule` is a data-type
filter (size bounds, a metatype, or a homogeneous float aggregate of up to 4
primitives), an optional prototype qualifier (varargs position range, absolute
position, a data-type at a fixed position, or an AND of these), one primary
`AssignAction`, plus *precondition* actions applied to a scratch copy of the
group-status array (discarded if the primary fails) and *side-effect* actions
applied on success (`ModelRule::assign_address`). The ten `AssignAction`
variants cover the modern cspec vocabulary: `GotoStack`, `ConvertToPointer`,
`MultiSlotAssign` (join several registers, optionally spilling to stack),
`MultiMemberAssign` (one register per primitive), `MultiSlotDualAssign` (two
storage classes), `ConsumeAs`, `HiddenReturnAssign`, and the resource-burning
side-effects `ConsumeExtra`, `ExtraStack`, `ConsumeRemaining`.

### Recovery: trials → parameters

The recovery direction runs on `ParamTrial`/`ParamActive`
(`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs (ParamTrial,
ParamActive)`): one trial per storage location that *might* be a parameter,
carrying its life-cycle flags (checked, active, used, definitely-not-used,
unreferenced) and its evidence bits (killed-by-call — set heuristically at
registration for any non-stack location, since register contents rarely
survive a call; formed-by-remainder; formed-by-indirect-creation;
conditional-execution-affected; ancestor-realistic; ancestor-solid). Trials
are gathered by heritage (§4.2), then `fillin_map`
(`ParamListStandard::fillin_map`) converts the unordered set into a formal
parameter list. For the standard input list the decision sequence is:

1. **`build_trial_map`** — bind each trial to its justified containing entry
   (no entry → definitely-not-used), then *plug the holes*: a group no trial
   referenced gets a synthetic **unreferenced** trial (a formal parameter list
   cannot skip a slot), choosing a float or general entry by whichever class
   has more active trials; likewise unused slots inside a partially-used
   aligned entry. Trials then sort into formal parameter order
   (`ParamTrial::cmp` — group, then entry, then justified offset/address).
2. **`force_exclusion_group`** — inside one exclusion group an *active* trial
   evicts every overlapping trial. If a group has only inactive candidates,
   `mark_best_inactive` keeps the most plausible one: +5 for a realistic
   ancestor, +5 more for solid movement, +1 for the preferred storage class;
   multi-group entries are never chosen.
3. **`force_no_use`** (per resource section, after `separate_sections`) —
   parameters are allocated in order, so once an entire exclusion group is
   definitely-unused everything after it in the section is demoted to
   inactive: a hole proves the list ended before it.
4. **`force_inactive_chain`** (`maxchain = 2`) — the converse repairs: an
   active trial that sits past a run of more than two inactive slots is
   demoted (an isolated far register is more likely local state than a
   parameter), and during sub-call recovery an *unreferenced stack* slot ends
   the chain immediately (the callee never touched the stack area, so nothing
   beyond it is a parameter); finally every inactive slot *before* the last
   surviving active trial is promoted — interior holes are filled, because
   the list must be contiguous. (kuna) `inputparamgap` exempts an *active
   register* trial from that demotion when the trials are the function's
   **own** inputs rather than a call's — see below.
5. Whatever is still active is marked **used**.

Steps 3 and 4 both read a hole in a section as evidence that the argument list
ended, which is what makes a section the unit of scoring. That inference is
sound only while the arguments really do fill the resource in order, and at a
**variadic** call site on an ABI that passes the variable arguments on the stack
it does not. Apple's arm64 ABI is the case in point: a fixed parameter takes
`x0`, the varargs start at `[sp+0]`, and `x1`–`x7` are structurally empty —
seven slots, longer than either rule tolerates. Since AArch64 puts the general
registers and the outgoing stack area in **one** section, a stack trial
`check_input_trial_use` had already scored active is deactivated again here, and
the argument is dropped; whatever computed it then dies to dead-code
elimination, so the destination of a `scanf` is not merely unprinted but
unwritten. (kuna) `varargstackargs` (default-off,
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_varargstackargs.rs`) cuts such a
section in two at its first stack trial, so the register prefix and the stack
tail are scored independently and the ABI's hole stops being evidence about the
stack argument. The cut also keeps step 4's hole-filling promotion inside the
half that produced it — promoting across the boundary would fabricate `x1`–`x7`
as six invented register arguments. `ActionActiveParam` sets the flag on the
call's `ParamActive` and only for a callee whose prototype is variadic
(`FuncProto::is_dotdotdot`): with a fully known prototype a register hole *is*
evidence, and only `...` makes the hole a property of the ABI rather than of the
recovery. Nothing about trial scoring changes — a stack trial still has to reach
`fillin_map` active on its own evidence — so the option can keep an argument the
recovery already believed in but never invent one.

The same two rules are also what `ActionInputPrototype` runs the function's
**own** input Varnodes through, and there the premise behind step 4 does not
hold. At a call site an active trial is a caller-side inference — an argument
register holding a value the caller wrote and does not otherwise use — which is
genuinely ambiguous, so a long run of empty slots is fair evidence that the
recovery has walked past the end of the argument list. For the function's own
inputs an active trial is a fact about the body: this function reads that
caller-saved register before any definition of it, which on an argument register
has one explanation. The gap slots meanwhile carry no counter-evidence at all,
since an untouched argument register is exactly what an ignored parameter looks
like — and a callback whose signature is fixed by the API it is registered with
ignores parameters as a matter of course. So step 4 trades a fact for a
heuristic, and it fires hardest on the functions that need recovery most: a
handler reached only through a function-pointer table has no call site anywhere
in the image, so its body is the only evidence there is. The Wayland
`wl_keyboard_listener` key callback is the witness — `data` in `rdi`,
`wl_keyboard`/`serial`/`time` ignored, `key` and `state` arriving in `r8d`/`r9d`
behind a three-register hole, one past `maxchain` — and kuna rendered it as
`void sub_6500(long a0)` whose first statement branches on a local nothing ever
assigned. (kuna) `inputparamgap` (default-on,
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_inputparamgap.rs`) stops a gap
slot from ending the chain when the `ParamActive` is the one
`ActionInputPrototype` built, so the active trials past the hole survive and step
4's own promotion fills the interior with the unreferenced trials
`build_trial_map` had already synthesized — the full ABI signature, positions and
all. A two-slot hole was always tolerated, so the option moves only where the
limit sits.

Three clauses bound it, and the second was settled by measurement rather than
argument. The flag is carried on that `ParamActive` and nothing sets it at a call
site, so argument recovery everywhere else is untouched. Only an **active
exclusion (register)** trial is protected — a stack trial's fate is left exactly
to `seenchain`, because the evidence the option rests on is a register's: a
caller-saved argument register read live-in can only be carrying what the caller
placed there, while a positive-offset stack slot read live-in is much weaker,
since a Win64 home slot used as scratch and an over-wide or aliased read look the
same. A first design exempted any register *gap slot* instead; it fixed the
witness and left the datatest corpus byte-identical, and it also let one Win64
`sub_140010a57` span its four-register hole into the stack resource and promote
eleven scratch slots of the caller's argument area into a fifteen-parameter
signature. Because trials sort into formal parameter order, protecting only
register trials additionally keeps step 4's hole-filling inside the register file,
which is what bounds the recovered list to the ABI — six parameters on x86-64
SysV, four on Win64. And it never makes a trial active that was not already
active, so a register the body does not read before writing is still not a
parameter.

### `build_input_from_trials` — writing the argument list

Whatever is still `used` becomes the CALL op's input list, in prototype order
(`funcdata_callsite.rs (build_input_from_trials)`), a spacebase parameter's stack
range is marked unmapped, and the trials are dropped. What is written are the
argument *values*: after constant propagation a size argument is a constant
Varnode, not the register the ABI passes it in — so the storage each argument
occupied survives only if something records it. (kuna) `calleearity`
(default-on, `decompiler/crates/kuna-decomp/src/p4_calls/kuna_calleearity.rs`)
records exactly that, on the call spec, and uses it for one thing: when the same
callee is called more than once in the function, a call whose list is not yet
written is reconciled against a sibling whose list already is.

That reconciliation exists because nothing else in P4 does it. With an unlocked
callee prototype every call site recovers its arguments alone, so one allocator
wrapper renders as `sub_140008160(0x28)` at one site and `sub_140008160()` thirty
bytes later — the second site being the one where the argument is *also* the
operand of an overflow check, which `only_op_use` rejects on its `CPUI_CBRANCH`
descendant. Relaxing that rejection is not an option: `test rcx,rcx; jz; call` is
structurally identical and would gain an invented argument everywhere. The
sibling call is the only local evidence that settles it. The reconciliation is
register-storage only (a finalized call's stack arguments sit at caller-relative
addresses that differ per site), never promotes a synthetic unreferenced trial,
is all-or-nothing (parameters are positional), and never removes an argument.
`ActionActiveParam` finalizes each spec as soon as that spec is fully checked, so
that rule alone reconciles a call against the sites *before* it, and a callee
whose first call site is the broken one stays broken.

That direction is not a detail, because the shape the reconciliation exists for
routinely puts the loser first. MSVC's aligned `operator new` calls one allocator
from two arms of the same test: the large arm writes a fresh argument register
(`lea rax,[rcx+0x27]; cmp rax,rcx; jbe abort; mov rcx,rax; call`) and keeps its
argument, while the small arm passes the register live-in
(`test rcx,rcx; jz; call`) and loses it to the very `only_op_use` rejection
above. Flow order reaches the small arm's call spec first, so at the moment it
finalizes its witness is still `input_active` and has recovered nothing yet.
(kuna) `calleearityfwd` (default-on,
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_calleearityfwd.rs`) closes that
direction. Reordering the finalization would be the obvious way and is the wrong
one: `check_call_double_use` asks whether *another* call spec is still
`input_active` while scoring a trial, so deferring a spec past its neighbours'
`check_input_trial_use` changes argument recovery on every binary and not just
where two sites disagree. Instead a call that finalizes with an **empty**
argument list is set aside — together with the Varnodes its still-promotable
trials point at, read before `op_set_all_input` drops them, which is the only
moment they are reachable — and retried once at the end of the same
`ActionActiveParam::apply`, when every spec in the pass is final. The witness
search and every refusal are `calleearity`'s, unchanged, so the retry adds no new
way to promote a trial: it only lets the existing one see the sites that come
after. Two limits are its own. A captured Varnode wider than its trial is
declined rather than truncated, because the `SUBPIECE` the normal path would
insert needs the trials the retry no longer has; and nothing crosses an `apply`,
because the slot numbering the captured Varnodes came from does not survive
`delete_unused_trials`. It is inert unless `calleearity` is also on, so one
option still turns all sibling reconciliation off.

Both of those refuse a call that recovered *any* argument, and that refusal is
measured rather than cautious: without it the rule reads "same callee, same
arity", which the whole-corpus sweep showed is false for a variadic callee and
for a witness that itself over-recovered — `Sleep(200)` became `Sleep(200,0)`,
and a variadic internal logger `sub_1b11c(5,0,"Zip: empty archive?")` gained two
arguments its format string has no conversions for. A sibling call is simply not
evidence that a shorter argument list is a broken one. But a partial list can
still be wrong: one helper called fifteen times in one function renders eleven
times with five arguments and four times with three, from
instruction-for-instruction identical code, because `only_op_use` rejects the
last recovered trial on a competing use elsewhere in the function — a `CBRANCH`,
a `LOAD`, a `STORE` — and `fillin_map` then drops that argument and every
argument behind it.

(kuna) `calleearitylive` (default-on,
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_calleearitylive.rs`) extends a
partial list, and pays for the relaxation with evidence the sibling does not
carry: the **callee's own body**. It reuses the bounded entry decode
`calleedeadarg` takes for the subtractive direction
(`kuna_calleedeadarg.rs (probe_callee_entry_dead)`), which already records which
register bytes some path reads before writing, and asks two things of it. Every
register the witness claims beyond this site's own list must be read before
written by the callee, so it genuinely carries an input; and **no other argument
location of the prototype model may be**, so the witness's list is the callee's
whole register argument list and not a prefix of it. The second half is what
refuses the two shapes the sweep found: an import has no body to decode and
declines outright, while a variadic register-save prologue (`str x3,[sp,#136];
stp x4,x5,[sp,#144]; stp x6,x7,[sp,#160]`) reads argument registers a
five-argument witness does not claim. A fixed-arity callee reads exactly the
registers its prototype names.

Two limits are this rule's own, on top of `calleearity`'s. The site's own
recovered list must be exactly the **leading run** of the witness's, because
parameters are positional and a site whose arguments disagree with the witness
*in place* is a different call rather than a shorter one. And it is always
deferred, never in-order: on the witness the four short sites are the first four
and the first five-argument site is the fifth, so an in-order rule has no witness
at any of them. Like `calleearityfwd` it captures its candidate Varnodes in
`build_input_from_trials` and replays them at the end of the same
`ActionActiveParam::apply`, rather than moving when a spec finalizes. It is inert
unless `calleearity` is also on.

The `Register` (unordered) variant skips all ordering logic: every active
trial that lands justified in an entry is a parameter
(`fillin_map_register`). The output variant first lets the model rules claim
the trials (`ModelRule::fillin_output_map` — how a cspec `<join>` output rule
keeps a register *pair* alive as one return value), and otherwise runs the
fallback: try each output entry as the candidate return location, keep the one
where **all** active trials form a contiguous least-significant cover of at
least the entry's minimum size — rejecting remainder-formed and
indirect-creation-formed pieces at the positions the entry flags for extra
checks — kept when it has an earlier storage class *or* a wider cover
(`fillin_map_standard_out`, `fillin_map_fallback`). Failure mode: no candidate
survives → every trial is marked no-use and the call recovers as returning
nothing.

**The trial budget.** A `ParamActive` freezes when `numpasses > maxpass`.
`maxpass` is 0 (one look) unless the model's parameter registers have a
non-zero heritage delay, in which case it is fixed at 3 (a delay of 1 or 2 is raised, not capped)
(`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs
(FuncCallSpecs::init_active_input)`,
`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs
(Funcdata::init_active_output)`). This is the `trial-budget` sub-phase of
`decompiler/crates/kuna-decomp/phases.toml` — recorded there as LATENT: no
user surface sets it today.

## 4.2 Recovery passes

All drivers live in
`decompiler/crates/kuna-decomp/src/p4_calls/coreaction_protos.rs`; the
call-site trial mechanics they invoke are in
`decompiler/crates/kuna-decomp/src/p4_calls/funcdata_callsite.rs`. Placement
in the schedule is `decompiler/crates/kuna-decomp/src/infra/universalaction.rs
(universal_sched)`: setup before fullloop, the trial passes inside mainloop,
finalization in the one-shot tail (00 §0.6).

### Seeding: the prototype the function is decompiled against

Everything below is *recovery* — what runs when the function's prototype is
unknown. Before any of it, the drive
(`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs`) asks whether the
signature is already known, and locks it if so
(`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs
(Funcdata::apply_locked_prototype)`). Two sources, in precedence order: a
prototype the operator declared for this run (`parse line extern …` /
`map prototype <func> …`, 00 §0.2), then the
prototype parked on the function's own global `FunctionSymbol` — which is where
the DWARF pass's recovered `DW_TAG_subprogram` signature lands (01 §1.4) and
where the library-prototype table lands for a named libc function.

The parked prototype used to be read only by a *caller*: `ActionDefaultParams`
copies a callee's pieces into the call site, so a DWARF-described callee typed
its arguments correctly at every call while its own decompile ignored the
signature and re-derived it from data flow. Applying it to the function itself is
the difference between `undefined16 main(uint4 a0, void *a1)` and
`int main(int argc, char **argv)` on any `-g` binary. Storage assignment that
hits an unported seam degrades gracefully — the prototype is dropped and the
function decompiles unlocked, exactly as before.

Locking the output is also what collapses the bogus wide return described under
`ActionReturnRecovery` below: with no locked output, return recovery registers a
trial for every output register the model characterizes (x86-64 SysV: `RAX` *and*
`RDX`), the cspec's `join_dual_class` output rule accepts the consecutive pair as
one 16-byte return, and the result is a `char[16]` whose high half is whatever
uninitialized value `RDX` happened to hold. A known `int` return never enters
that machinery.

### Setup (once, before fullloop)

- **`ActionPrototypeTypes`** (`coreaction_protos.rs (ActionPrototypeTypes)`)
  attaches the evaluation model to the function's own prototype (the
  current-function evaluation model, falling back to the default), resets the
  local-variable discovery window from the model's stack ranges, replaces the
  non-constant first input of every RETURN with a constant 0 (the raw
  return-address reference never reaches high-level output), and starts
  return-value recovery (`Funcdata::init_active_output`) — or, for a locked
  output, plants the declared output Varnode on every live RETURN. Locked
  inputs are forced into existence as typed input Varnodes, with the model's
  assumed extension op materialized at the entry block (`extend_input`), so a
  partially-used wide parameter still exists to take a SUBPIECE.
- **`ActionDefaultParams`** (`coreaction_protos.rs (ActionDefaultParams)`)
  gives every call spec a model: a callee with a source-declared prototype
  gets a locked copy re-built from the pieces parked on its global symbol
  (`decompiler/crates/kuna-decomp/src/infra/architecture.rs
  (Architecture::callee_proto_pieces)`); everything else gets the
  called-function evaluation model with a void internal store. (kuna) A
  callee whose parked pieces contain *only* custom return storage — what the
  console `map return` plants — keeps model-driven input recovery and locks
  just the output on top. (kuna) The parked pieces describe *types*, and
  storage is otherwise re-derived from the model, so the two spellings that
  state storage explicitly carry it alongside: `output_storage` for the return
  and `input_storage` for individual parameter slots. Both are re-applied by
  `FuncProto::set_pieces` after the model-driven assignment, which is what lets
  a caller declare a non-default convention for a callee — the console `map
  param <func>::<i> <storage> <decl>` and `map return <func>::<storage>
  <decl>`, reached from the CLI as `--assert 'param <func>::<i> …'`. A slot no
  directive named is `undefined` of pointer width, so slots may be declared in
  any order.
- **`ActionExtraPopSetup`** (`coreaction_protos.rs (ActionExtraPopSetup)`)
  models the stack pointer across each call: a known extrapop becomes an
  explicit `INT_ADD sp, #extrapop` after the call; an unknown one becomes an
  INDIRECT, deferring the answer to the stack-pointer flow solver
  (`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
  (ActionStackPtrFlow)`, its home by port history) and, per-function, to
  `option extrapop`
  (`decompiler/crates/kuna-decomp/src/p0_knowledge/options.rs
  (OptionExtraPop)`).
- **`ActionFuncLink`** (`coreaction_protos.rs (ActionFuncLink)`) arms each
  call site. Unlocked or varargs prototype → input recovery on
  (`init_active_input`). Locked prototype → one pre-marked trial per declared
  parameter plus a stub input Varnode: plain register inserted directly, stack
  parameter materialized as a stack LOAD, and a stack+register `join`
  parameter reassembled with a PIECE. Output side: locked non-void output
  builds the output Varnode (plus the model's assumed extension after the
  call); a locked *stack* output is deferred to heritage
  (`set_stack_output_lock`); unlocked → `init_active_output`. When stack
  parameters may exist but the call-time stack offset is unknown, a
  **spacebase placeholder** input is appended
  (`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs
  (FuncCallSpecs::create_placeholder)`), resolved later in §4.3. The
  `jumptable` root variant swaps this for **`ActionFuncLinkOutOnly`** (group
  `noproto`): outputs are still guarded — otherwise callee return registers
  mis-heritage as locals — but no input recovery runs inside the reduced
  sub-decompilation.

### Trials are populated by heritage

The trial containers fill during SSA construction, not in P4 passes: when
heritage processes an address range, `decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs
(Heritage::guard_calls)` asks each call spec how the range relates to the
model. A justified input candidate registers an input trial *and appends the
Varnode to the CALL op*; an output candidate registers an output trial and —
where the effect says killed-by-call — seeds an INDIRECT *creation* whose
output is the would-be return value; the function's own RETURN sites get
output trials symmetrically (`Heritage::guard_returns`). Chapter 03 owns the
guard machinery; what matters here is that a CALL op's input list grows
speculatively during Band B and is *rewritten to the truth* by the passes
below.

### `ActionActiveParam` — does this argument exist?

Per call with active input recovery
(`coreaction_protos.rs (ActionActiveParam)`, mechanics in
`funcdata_callsite.rs (check_input_trial_use)`), each unchecked trial is
classified:

- **Stack trial**: aliased by local pointer arithmetic → no-use (a callee
  argument slot nobody else may touch); outside the caller's local stack range
  (the model's `localrange`, `FuncProto::get_local_range`) → no-use. If the
  callee demonstrably pops its own parameters (model extrapop unknown but the
  prototype's working extrapop, `get_extra_pop`, exceeds the return-address
  slot, > 4), the popped byte range is *hard evidence*:
  trials below it are active, at-or-above it no-use. Otherwise fall through
  to ancestor analysis.

  The local-range test probes **two different addresses for two different
  questions**, and the distinction decides whether stack-passed arguments are
  recovered at all. `guard_calls` registers the trial at the *callee*-relative
  address (`addr - stackoffset`, the callee's parameter frame) while creating
  the argument Varnode at the *caller*-relative one. `localrange` belongs to the
  caller's prototype, so the range test takes the **argument Varnode's**
  caller-relative address; only the `callee_pop` byte-range comparison, which
  reasons in the callee's frame, takes the trial's. Probing the callee-relative
  address against a caller-frame range rejects every outgoing-argument slot on a
  downward-growing stack — the offsets are positive, the range negative — so
  every call whose callee prototype is unlocked truncates at its register
  budget, and because a definitely-unused trial has its CALL input replaced by
  constant 0 (below), dead-code elimination then reaps whatever computed the
  argument. That is visible as deleted basic blocks, not merely as a shorter
  argument list. (kuna) `callsitestackargs` (default-on) selects which address
  is probed; `off` restores the truncating behavior for bisection.
- **Ancestor analysis** (`decompiler/crates/kuna-decomp/src/substrate/funcdata_varnode.rs
  (AncestorRealistic, Funcdata::ancestor_op_use)`): the trial is *active* only
  if the value reaching the call has a realistic def chain (not an INDIRECT
  fabrication, not uninitialized junk) **and** the Varnode's only role (within
  a recursion budget, `trim_recurse_max`, default 5 —
  `decompiler/crates/kuna-decomp/src/infra/architecture.rs
  (reset_defaults_internal)`) is feeding this call. A read by *another* call
  is admitted when that call provably takes the same value as the same
  parameter (`funcdata_varnode.rs (Funcdata::check_call_double_use)` — same
  direct target, or same function-pointer Varnode for CALLINDs; (kuna) two
  *sibling* CALLINDs through distinct function pointers are also admitted
  when the matched trial addresses agree, replacing an upstream
  restart-driven recovery whose override path is not ported — a documented,
  datatest-neutral divergence in that function's comments). Register trials
  that fail realism but are function inputs stay *inactive* (maybe a
  pass-through parameter); everything else is no-use.

  "Only role" is judged by `funcdata_varnode.rs (Funcdata::only_op_use)`, which
  walks every descendant of the value and classifies each use. A branch, a LOAD,
  a STORE, a non-matching call or a persistent output all mean *not exclusively
  a parameter*, and the trial goes inactive — permanently, because
  `mark_inactive` also sets CHECKED, so no later pass re-scores it and the
  argument's producer is reaped.

  The blanket STORE rejection exists to stop a value the caller writes to its
  own frame before a call from being mistaken for an argument. It also rejects
  the mirror-image idiom. On x86-64 SysV **no** xmm register is callee-saved, so
  a floating-point value that is both an argument and live across the call has
  to be spilled by the caller — and that spill is a second descendant of exactly
  the Varnode the trial is scoring. The argument is then dropped, and the
  producer with it. (kuna) `spillargtrial` (default-off,
  `decompiler/crates/kuna-decomp/src/p4_calls/kuna_spillargtrial.rs`) narrows
  the STORE arm: at `reload` a store stops rejecting when it writes the walked
  Varnode's own value — operand 2, never the pointer — into a caller-frame slot
  *and* a later LOAD reads that slot back at the same width, which is a genuine
  caller-save spill/reload pair; at `spill` the reload requirement is dropped and
  any caller-frame store of the value is tolerated.

  Two constraints shape how the frame slot is recognised. `ActionActiveParam`
  runs before `ActionStackPtrFlow`, so `RuleStoreVarnode` has not yet folded the
  frame STORE into a direct stack-space write and the pointer is still the raw
  `INT_ADD(<stack pointer register>, #const)`; and a caller-save reload by
  construction straddles the call, which re-defines the stack pointer, so the
  reload's constant is not directly comparable to the store's. The search
  therefore walks *forward* from the store's own base Varnode over the
  value-preserving and constant-displacing ops (INDIRECT, COPY, INT_ADD,
  INT_SUB), carrying the running offset delta; a pointer whose delta equals the
  store's offset addresses the same slot. MULTIEQUAL is not followed, since a
  phi's other arm may carry a different frame.

  This is a deliberate **divergence from upstream**, not a port repair:
  `only_op_use` is faithful to `funcdata_varnode.cc:1891`, and relaxing its
  STORE arm admits non-arguments. The failure mode is a *spurious trailing
  argument*, which no gate observes — the datatest corpus is prototype-declared
  and GED scores topology, not arity — which is why the option ships off by
  default and why `reload` is the recommended level over `spill`: on a clang
  `-O2` inlined 64-byte `memcpy`, the four `movaps` stores that fill the local
  buffer are never read back, so `reload` declines them while `spill` turns them
  into four invented leading arguments.
- **Callee-body evidence** (kuna, `decompiler/crates/kuna-decomp/src/p4_calls/kuna_calleedeadarg.rs`):
  every test above reasons on the *caller's* side of the call, and on that side
  a live argument register at an unprototyped callee is exactly what a real
  argument looks like. Where the return register and the first argument register
  coincide — `x0` on AArch64, `r0` on ARM — the previous call's result is
  therefore recovered as the next call's argument, and the same output that
  declares `int f(void);` calls `f(v3);`. That does not recompile, and it leaves
  the reader unable to tell whether the callee consumes the value.

  `calleedeadarg` (default-on) supplies the one piece of evidence the caller
  does not have: the callee's own body. Before the ancestor analysis runs, a
  bounded decode starting at the callee's entry answers, per register range,
  whether the callee **overwrites** those bytes on every path before ever
  reading them. Each path carries the register bytes already written on it; a
  read of a byte not in that set vetoes the range for the whole callee. Every
  path *ends* somewhere — at a `RETURN`, at a nested call, at an unresolved
  `BRANCHIND`, at a `LOAD`/`STORE` naming the register space, or at an
  undecodable instruction — and the range must already be written when it does,
  because past that point the walk is not reading the code that runs. That is
  what lets a body which overwrites `x0` and *then* calls `printf` still prove
  `x0` dead, while a body whose first act is a call proves nothing. A walk that
  records *no* terminator at all proves nothing either, and that case has to be
  named separately because the "written before every terminator" test is a
  conjunction and holds vacuously over an empty list — for every register at
  once. It arises whenever every path closes back onto an address the walk has
  already visited: a body that is one endless loop, and, in practice, a PE
  import whose entry address is its IAT slot, so the walk is decoding pointer
  bytes as instructions. An
  instruction whose p-code branches inside itself is scored against the set it
  was entered with and credits none of its writes, so a conditionally-executed
  write cannot hide a later read. A proven-dead register trial is scored
  `no-use` like any other definitely-unused trial.

  Requiring the *write* rather than merely the absence of a read is the whole
  safety margin. A callee whose entire body is `ret` reads nothing at all, so a
  "never read" rule would call every register dead there and delete the
  arguments of every stub and thunk in the image — which is precisely what the
  `stackreturn` datatest (three callees that are one `c3` byte each) catches.
  The claim the pass makes is the positive one: the callee demonstrably
  clobbers the register, so the value the caller left there cannot be reaching
  it. Only the `register` space is answered; a `ram`-space global trial would
  need the walk to model memory. Like `rustabi`'s call-*output* probe, the walk
  is taken from the driver right after the flow build — the per-function
  architecture handle the pipeline runs against carries the load image but no
  translator — and cached per callee entry, so each distinct body is decoded
  once per run. `off` restores the pre-option rendering.
- A definitely-unused trial has its dataflow **freed immediately** — the CALL
  input is replaced with constant 0 so dead-code elimination can reap the
  producer. This is why P4 must iterate with DCE inside mainloop.

Conditional-execution-affected actives set a *final-check* flag; when the
container freezes, `funcdata_callsite.rs (final_input_check)` re-runs realism
once more, since the condexe pass may have rewritten their ancestry. A
CALLIND's trials are deliberately not finalized on the container's first
frozen pass (`trimmable` requires a prior pass for CALLIND), giving
de-indirection (§4.3) one mainloop iteration to land the real callee's
prototype first. Finalization resolves the model, runs `fillin_map`
(§4.1), and `funcdata_callsite.rs (build_input_from_trials)` rewrites the
CALL's inputs to exactly the used trials in prototype order — truncating an
oversized Varnode with a SUBPIECE, translating stack trials into the caller's
frame and marking those ranges not-mapped, and materializing recovered but
unreferenced parameters as fresh Varnodes. For a locked varargs prototype the
fixed arguments sort to the front first (`ParamActive::sort_fixed_position`).

### `ActionActiveReturn` / `ActionReturnRecovery` — return values

For each call with active output (`coreaction_protos.rs (ActionActiveReturn)`,
fullloop tail): the INDIRECT-creation outputs planted by `guard_calls` are
collected (`funcdata_callsite.rs (collect_output_trial_varnodes)`), a trial is
active iff its Varnode exists, the model's output `fillin_map` picks the
winner, and `funcdata_callsite.rs (build_output_from_trials)` promotes the
single surviving Varnode to the CALL op's formal output, destroying the
scaffolding INDIRECTs. Documented seam: the multi-register call-return join
(two used output trials at one call site) currently leaves the trials in place
rather than building the concat — the shipped models recover single-register
call outputs; only the *function's own* return supports the join below.

The function's own return runs in mainloop
(`coreaction_protos.rs (ActionReturnRecovery)`): every live RETURN op's trial
Varnodes go through the same realism + sole-use tests, the container freezes
on the §4.1 budget, the output map is derived, and
`ActionReturnRecovery::build_return_output` rewrites each RETURN: zero or one
used trial passes through; **two pieces** are concatenated with a PIECE whose
output sits at the constructed join address (falling back to the first piece
if no join can be built); more pieces chain PIECEs over contiguous trials.
The (kuna) `returnpair` gate intercepts this join — §4.4.

The sole-use check has one narrow terminating-path exception, the (kuna)
`noreturnretuse` gate (`decompiler/crates/kuna-decomp/src/p4_calls/kuna_noreturnretuse.rs
(call_cannot_reach_return)`). When the use being matched is a RETURN, a candidate
return register may also feed a CALL/CALLIND whose immediately following and
block-final op is an artificial halt marked no-return. That call consumes the same
ABI register as its first argument on a failure path but cannot reach the RETURN,
so it does not disqualify the normal path's output trial. An ordinary call, a
non-adjacent halt, or a halt that may return still rejects the trial. The shape
needs the return register and the first argument register to be the same storage,
so it is an ARM/AArch64 finding in practice; with the gate off the check is
upstream's, rejecting on every competing call use.

### Fixating the function's own prototype

In the one-shot tail, after merge has built HighVariables:
**`ActionInputPrototype`** (`coreaction_protos.rs (ActionInputPrototype)`)
re-derives the function's own parameter list from its input Varnodes — each
input that the model admits as a possible parameter becomes a trial, active
iff it has readers; `fillin_map` orders them (with the (kuna) `inputparamgap`
gap-tolerance above, which applies only to this call of it); recovered-but-unreferenced
parameters get fresh input Varnodes unless something already overlaps the
slot; and the store is rewritten with each parameter typed from its
HighVariable (`update_input_types`). **`ActionOutputPrototype`**
(`coreaction_protos.rs (ActionOutputPrototype)`) sets the return storage and
type from the first RETURN's recovered value. Earlier, inside mainloop
between return recovery and dead-code elimination, **`ActionRestrictLocal`**
(`coreaction_protos.rs (ActionRestrictLocal)`, transcribed on the IR side as
`Funcdata::restrict_local`) marks locked callee argument stack ranges and
unaffected-register save slots as not-mapped so the local-variable phase
(chapter 06) cannot claim them.

Two registered passes are **documented no-ops** in the current port, kept in
the schedule so the materialized tree stays byte-equal to the upstream oracle
(00 §0.6): `ActionParamDouble` (double-precision split/join of call arguments;
its `apply` carries the transcribed upstream body as pseudocode and performs
no rewrites — the ported `FuncCallSpecs::check_input_join`/`do_input_join`
surface in `decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs` waits on this
driver) and `ActionPrototypeWarnings` (prototype-error headers; the warning
channel exists but nothing is emitted). Failure modes: a genuinely split
two-register argument is passed as two separate arguments, and a prototype
whose storage assignment failed degrades silently instead of warning. Two
S4-grouped passes live outside this folder by port history:
`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionDirectWrite)` — the `protorecovery_a` paint of Varnodes reachable from
legal parameter sources that ancestor realism consumes (the `decompile` root
enables the INDIRECT-propagating variant, `decompiler/crates/kuna-decomp/src/infra/action.rs
(build_default_groups)`) — and `decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionUnjustifiedParams)`, the fullloop-tail repair that re-justifies an
input recovered off-center in its containing entry.

## 4.3 Call-site ops

### How a CALL op carries its spec

Call specs are born at lift time: `decompiler/crates/kuna-decomp/src/p2_lift/flow.rs
(FlowInfo::setup_call_specs, FlowInfo::setup_callind_specs,
FlowInfo::build_call_specs)` creates a `FuncCallSpecs` per CALL/CALLIND and
pushes it onto the function's spec list (`decompiler/crates/kuna-decomp/src/substrate/funcdata.rs
(Funcdata::num_calls)`, the upstream `qlst`). A direct CALL's input 0 is
replaced by an **fspec annotation**: a Varnode in the reserved fspec address
space whose offset is a process-unique handle into a side table mapping back
to the spec (`decompiler/crates/kuna-decomp/src/p2_lift/flow.rs
(next_fspec_handle)`, `decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs
(FuncCallSpecs::register_in_fspec_space)`) — the arena-safe replacement for
the upstream pointer-cast-into-offset trick. A CALLIND keeps the computed
target Varnode in slot 0, which is exactly what the printer renders as
`(*fptr)(...)`. Spec construction consults P0 immediately: a call-site
prototype override is copied on first (before the callee-name query, so
inline/inject effects are not clobbered), then the callee symbol's
inline/no-return flow effects — inline queues the site for body injection, and
no-return plants an artificial halt after the call plus the "Subroutine does
not return" warning (`decompiler/crates/kuna-decomp/src/p2_lift/flow.rs
(FlowInfo::check_for_flow_modification)`; the fact-producing analyses are
chapter 01 §1.7, the lift-time behavior chapter 02 §2.4).

### Effect lists

Every call's data-flow shadow is the effect list: address-sorted
`EffectRecord`s of type **unaffected**, **killedbycall**, or
**return_address** (`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs
(EffectRecord, effect_type)`), decoded from the cspec per model, overridable
per call site (`FuncProto::effect_list` prefers the prototype's own list and
falls back to the model's; lookup is `ProtoModel::lookup_effect` — a
zero-size record blankets its whole space, and unique-space temporaries are
always unaffected). Heritage consumes the verdicts
(`decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs
(Heritage::guard_calls)`): *unaffected* ranges flow through the call
untouched; *killedbycall* ranges become INDIRECT creations (fabricated-value
markers — and return-value seeds when the range is an output candidate);
*unknown* and *return_address* ranges get a plain INDIRECT guard tying the
value across the call, so anything the callee might touch through a pointer
keeps a call-crossing cover. The wrong-list failure mode is structural: a
missing `<unaffected>` stack-pointer record makes every call guard the stack
pointer, skewing the entire frame layout.

(ida) One record kuna adds that the vendored specs leave implicit: the **x86
direction flag** (`decompiler/crates/kuna-decomp/src/p4_calls/kuna_dfunaffected.rs`).
Every x86 string instruction scales its pointer step by `1 − 2·DF`, and SLEIGH
lowers that faithfully, so the flag reaches emitted output as a live variable and
a `(uint8)df * -2 + 1` stride on every inlined `strcmp`/`memcpy`. The flag is not
unknown — the processor spec pins it to 0 at function entry (§1.3's tracked-value
seeding) and `ActionConstbase` materializes that — but the gcc prototype's
`<unaffected>` list omits `DF`, so a *call* forces the unknown-effect INDIRECT
guard and the constant never reaches the stride. Both x86 ABIs require the
direction flag clear at every function boundary, and the Microsoft prototype in
the same spec already records it, so kuna states the same guarantee for the models
that are silent, at model-decode time. A spec that mentions `DF` either way has
made a deliberate statement and is left alone, and a language with no such
register is a structural no-op — the assertion is keyed on the SLEIGH register
name and a lookup miss is the exit. That miss is a *speculative* question, so
the assertion takes a probe (`Option<VarnodeData>`) rather than the exact
by-name lookup: on a front-end that resolves register names by asking a host,
asking for a name the language does not define is an error the host reports
(§0's probe seam), and every non-x86 language would take that path on every
decoded prototype.

### The spacebase placeholder

A call that may take stack arguments cannot find them until the caller's
stack-pointer value *at that site* is known. `ActionFuncLink` appends a
placeholder input (§4.2); once simplification collapses the placeholder's
pointer to `spacebase + constant`, the rule-pool hook
`decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_4.rs
(RuleLoadVarnode)` fires `decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs
(FuncCallSpecs::resolve_spacebase_relative)`: the spec records the relative
stack offset, and stack trials can from then on be translated between callee-
and caller-relative addresses (`build_input_from_trials`,
`Heritage::guard_calls` both read it). The placeholder strip
(`FuncCallSpecs::abort_spacebase_relative`) happens on the *success* path of
`resolve_spacebase_relative` — the offset is recorded first, then the redundant
placeholder input is removed. When recovery ends *unresolved*, the placeholder
is silently dropped by the final input rewrite (`funcdata_callsite.rs
(build_input_from_trials)` via `op_set_all_input`), and stack arguments were
never registered as trials at all: `Heritage::guard_calls` skips spacebase
ranges while `get_spacebase_offset()` still reads `OFFSET_UNKNOWN`.

### De-indirection and the proto-change restart

`decompiler/crates/kuna-decomp/src/p9_emit/coreaction_render.rs
(ActionDeindirect)` (group `deindirect`, inside stackstall) watches every
CALLIND whose target Varnode — chased through COPYs — resolves to a known
function: an external-reference symbol, or a constant converted to a code
address (masked by `funcptr_align` when the architecture encodes bits in
function pointers). On a hit,
`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs (FuncCallSpecs::deindirect)`
rewrites the op to a direct CALL with a fresh fspec annotation and immediately
persists the lesson into P0:
`decompiler/crates/kuna-decomp/src/p0_knowledge/overrides.rs
(Override::insert_indirect_override)` keyed by the site address, so a restart
re-lifts the site as a direct call from the start
(`FlowInfo::setup_callind_specs` consults the override before building specs).
Then it tries to merge the discovered callee prototype **in place**:
`FuncCallSpecs::late_restriction` accepts when the site has no model yet, or
when the models are compatible (same or aliased `ProtoModel`, `is_compatible`),
varargs only while input recovery is still active, and — for locked callee
prototypes — when the existing argument Varnodes can be re-mapped onto the
locked storage (`transfer_locked_input`/`transfer_locked_output`). Success
commits the new input/output lists directly; failure sets the restart-pending
flag — the P4 → Band B feedback edge of 00 §0.7, bounded and executed by the
drive — (kuna) recording `ProtoDeindirect` in the restart log
(`decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_restartlog.rs
(RestartLog)`). The reasoning: by the time the target resolves, heritage has
already committed guards and trials under the wrong prototype; edits cannot be
made backwards, but the Override survives `Funcdata::clear`, so the re-run
lifts the truth.

The sibling `FuncCallSpecs::force_set` — forcing a *recovered* function-pointer
prototype onto a call site, upstream's other deindirect arm — carries the same
restart contract ((kuna) reason `ProtoForced`) and input-lock tail, but its
override-persist and success-commit halves are documented port seams, and the
`ActionDeindirect` arm that would invoke it (a typed function-pointer reaching
the CALLIND after type recovery starts) is not wired; such a site today keeps
its model-recovered argument list. Restarts triggered here are refused during
jump-table sub-decompilation like every other feedback edge (00 §0.7).

**The prototype wire encode.** The recovered prototype marshals out for the
ghidra-mode `decompileAt` response through the `FuncProto::encode` port
(`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs (FuncProto::encode)`):
the model name (a model-less fixture degrades to the `"default"` spelling
Java maps onto the program default), the extrapop (the reserved
`EXTRAPOP_UNKNOWN` spells the string `"unknown"`), the eight boolean flag
attributes, and the REQUIRED `<returnsym>` — the output parameter's sized
storage `<addr>` (blank for void) followed by its type reference.  Effect and
likely-trash overrides encode as model-diffs only
(`encode_effect`/`encode_likely_trash` + `EffectRecord::encode`, the
fspec.cc:3589/3631 ports); Java's `FunctionPrototype.decodePrototype` skips
them, so they matter only to a native decoder.  Input parameters are
deliberately NOT here: on the wire they travel as `<localdb>` category-0
symbols (chapter [06](06-variables-and-merge.md) §6.2), matching upstream's
symbol-backed `ProtoStoreSymbol::encode`, which writes nothing.

## 4.4 kuna extensions

### (kuna) `returnpair` — the register-pair return split

Provenance: upstream issue GH-6990, implemented kuna-side
(`decompiler/crates/kuna-decomp/phases.toml` records the row against P4 /
`trial-budget`). On ABIs whose output list joins a register pair (SPARC
`o0:o1` and relatives), a void or single-register function can *passively*
keep its second output register alive — a prologue value rides the
save/restore window to the RETURN untouched, its trial passes ancestor realism
(the movement is real, just not a return value), and §4.2's
`build_return_output` dutifully emits `return CONCAT44(...)` with a
double-width return type. The extension is a one-line gate at the join point:
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_returnpair.rs
(keep_single_return)`, read in `coreaction_protos.rs
(ActionReturnRecovery::build_return_output)` — when `option returnpair single`
is set, a gathered multi-register return is truncated to its first
(least-significant) register instead of joined. The flag rides
`decompiler/crates/kuna-decomp/src/infra/architecture.rs (Architecture)`
(`return_single`, default `false` = upstream `pair` behavior) and is copied
into the per-function snapshot per 00 §0.5.

It is a **destructive opt-in**, deliberately not flipped in the default-on
sweeps: the gate cannot distinguish a passively-live pair from a genuine
128-bit two-register return, and the DIV-2 ablation (`docs/history.md`)
found 3 of the 675 upstream assertions legitimately need the join — a global
`single` default would truncate real wide returns. Flip it per function on the
CONCAT-return symptom; the symptom table and flip guidance live in
[`docs/options.md`](../options.md#returnpair).

### (ida) The uncomputed half of a recovered return pair

The same passive-pair symptom, decided on evidence instead of by fiat, and
therefore default and unflagged
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_returnuncomputed.rs`). Two
shapes reach a RETURN carrying a register the function never meant to return: a
**callee-saved restore**, where the epilogue reloads a register from a frame slot
the function only ever read, and a **clobber at a synthesized return**, where the
flow model turns a call that never returns into one and the output registers hold
the callee's INDIRECT creations. Both are movement ancestor realism is right to
call realistic — it is asking whether a value could legitimately *reach* the
RETURN, not whether the function meant to return it — so the pair forms, and on
x86-64 SysV the result is `undefined16 main(…)` whose emitted body writes
`v[8] = <uninitialized stack slot>`: output that reads memory the function never
wrote.

The rule is that a half carrying no value the function computed is not a return
value. "Computed" is a bounded walk back through the operations that only *move* a
value — copies, phis, indirects, piece/subpiece reshaping — stopping at the first
one that produces one. An unwritten Varnode and an INDIRECT creation are
uncomputed; a constant is computed, because returning a literal is a real return;
anything the walk cannot classify is computed, so an unfamiliar shape keeps
today's answer. The RETURN is then rewritten to the surviving half and the dead
concatenation destroyed.

Timing is the load-bearing detail. At recovery time the restore is still
`COPY(LOAD(sp − k))` and indistinguishable from `return *p`, so there is nothing
to decide on; the repair therefore runs in the one-shot tail, just before
`ActionOutputPrototype` reads the storage and type off the RETURN, by which point
heritage has resolved that load into a bare unwritten Varnode. A genuine wide
return is safe from it twice over: both halves of a real struct return are
computed (built from constants, arithmetic, or loads through a pointer — a LOAD
is not a move, so the walk stops there), and the rule only ever edits an existing
*pair*, never a lone recovered return value. Where every half is uncomputed — the
synthesized-return case — the low, first-in-class register is kept so the
function's output storage still agrees across every RETURN.

This subsumes `returnpair` on the GH-6990 case it was written for (`tests/stages/
gh6990-returnpair.xml` now records both passes agreeing); the flag remains as the
blunt per-function instrument for a pair this rule judges genuine.

#### (kuna) The half that is an input parameter (`retinputhalf`)

The "unwritten means uncomputed" terminal is too coarse in one direction: a
**formal input parameter is unwritten by definition**. A returned pair whose high
half is a plain copy of an argument therefore looked exactly like the restore
phantom, and lost the half — and then the argument, which had no reader left,
disappeared from the recovered signature too. Two functions differing only in
whether the second returned half is `x` or `x*3+7` came out with different
arities: `unsigned long wide(long a0)` against `undefined16 w2(long a0,long a1)`,
for the same two-argument source (`tests/stages/kuna-retinputhalf.xml`).

`option retinputhalf` (default on, DIV-85) supplies the missing exception, on
storage evidence the prototype model already holds. An unwritten terminal is a
real return value when both of the following hold:

* **it is parameter storage.** `FuncProto::possibleInputParam` answers for the
  resolved model, and it is the same question input recovery itself asks — so the
  argument registers and the stack region above the return address qualify, while
  a *local frame slot*, the storage a callee-saved restore reads, does not. The
  clobber shape never reaches the test at all: an INDIRECT creation is rejected
  earlier in the walk.
* **the function put it there.** Parameter storage alone is not enough, because on
  most conventions an argument register is also a return register. The terminal's
  address is compared with the storage of the return half the walk started from: a
  *different* address means the function executed an instruction to move the
  argument into the return register, while the *same* address means the register
  was never touched and the caller's value is passing straight through — leftover,
  which is precisely what the sibling rule exists to drop.

A weaker version of the placement test was tried and rejected. It also rescued
the pair when *every* half was an untouched incoming argument, on the theory that
`double f(double x) { return x; }` on ARM returns its argument in the registers it
arrived in; that recovered three betaflight soft-float helpers and simultaneously
resurrected the GH-6990 SPARC symptom, because a *void* `main` that touches
nothing leaves `o0:o1` passing through and SPARC passes arguments in those same
registers. Nothing local to the pair separates the two, so the placement test is
applied per half with no exception.

The predicate runs inside `ActionOutputPrototype`, which is scheduled *before*
`ActionInputPrototype`, so the proto's own parameter list is not fixated yet and
the question goes to the model — the same fall-through
`possible_input_param` takes when no locked parameters exist.

#### (kuna, rustc) The two-register `ScalarPair` return (`rustabi`)

rustc returns a `Result`, an `Option`, a slice or a fat pointer whose layout it
classifies as **`ScalarPair`** in *two* registers — the variant discriminant in
the first return register, the payload in the second. On x86-64 that is
`RAX:RDX`, and the total size does not predict the choice: `Result<u32,u32>` is
8 bytes and is a pair, while `Result<Box<u64>,u32>` is 16 bytes and goes through
memory. The discriminator is the variant layout, which no compiler-spec rule
can express.

It does not have to. The storage rustc picks is exactly what the x86-64 cspec's
`<join_dual_class/>` output rule already describes, so kuna **already recovers
the pair**: both trials go active, the rule matches, and
`ActionReturnRecovery::build_return_output` builds the `join`-space
concatenation. Two later seams then throw it away, and both are invisible to a C
corpus, because a C function whose *first* returned register holds a one-bit
value is rare and a Rust `Result` is nothing else.

* **On the producer**, subvariable flow narrows a RETURN to the logical width of
  the value it is tracing (§03, `SubvariableFlow::tryReturnPull`). rustc
  materializes a two-variant discriminant as `xor %eax,%eax; setb %al`, so `RAX`
  is a *one-bit* logical value — and truncating the RETURN to it does not narrow
  the returned value, it deletes the other register. `Result<u32,u32>` recovers
  as `bool prod(uint4)` with no payload at all.
* **On the consumer**, `FuncCallSpecs::buildOutputFromTrials` handles one used
  output trial and returns early on two or more. A call whose model asked for a
  register pair therefore gets **no output at all**; the INDIRECT creations that
  stood for "the callee wrote something here" survive, and every read of the
  payload register after the call renders as a local the function never assigns
  — the phantom `int4 v3; // edx` that a `Result` guard tests.

`option rustabi off|auto|always`
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_rustabi.rs`) acts at both
seams. It does **not** answer them with one classification, because they are not
looking at the same thing, and a shared verdict would be a claim about evidence
that only one of them has.

**The producer's classification.** Here the concatenation's halves are values
*this* function computed, so their shape answers the question — taken from the
**observed register writes** rather than from a size. `classify_return_pair`
looks at the concatenation the ABI already built and reports:

* **`ScalarPair`** when the least-significant half is *discriminant-shaped* — a
  value whose known non-zero bits fit in a byte. That covers both forms rustc
  emits for the same source: the branchy `mov $0`/`mov $1` tag and the
  branchless `setCC`, which is the common one at `-C opt-level=2`. Asking about
  known bits rather than about "a constant per path" is what makes the
  recognition survive the optimizer's branchless lowering.
* **`Memory`** when that half traces back, through move-only operations, to the
  function's own incoming pointer argument — the sret epilogue, where the first
  return register carries the hidden result pointer and is not a tag. A veto,
  not an action: the pair must not form.
* **`Scalar`** otherwise, which is today's answer unchanged.

That verdict is what `holds_scalar_pair` reports to `tryReturnPull` before it
narrows, and it looks *through* the reshaping the rule pool applies to the
concatenation: `RuleConcatZext` rewrites `PIECE(ZEXT(V), W)` as
`ZEXT(PIECE(V, W))` as soon as the payload register is written 32-bit
(`lea 0x7(%rdi),%edx`), which is the overwhelmingly common rustc case, so
matching only a bare PIECE would miss it.

**The consumer's classification, and what it cannot prove.** At a call there are
no callee values in the IR at all: both halves are INDIRECT creations standing
for "the callee may have written this", so their shape says nothing and
`classify_return_pair` has nothing to read. `classify_call_output_pair` is a
different predicate over the three pieces of evidence a call site actually has:

1. **The prototype model** — the `join_dual_class` output rule already matched a
   justified, consecutive, first-in-class register pair, which is what put two
   *used* trials here rather than one.
2. **The caller's reads** — both halves are read out of the call, they are
   distinct non-overlapping registers, and the payload half has a descendant.
3. **The callee's body** — `probe_callee_return_writes` decodes the resolved
   direct callee, bounded, following fall-through and resolved machine branches
   until every path reaches a `RETURN`, and records the processor-space writes it
   observes. A nested call, an unresolved indirect branch, an undecodable
   instruction or the instruction budget makes the summary *incomplete*, which
   proves nothing. On a complete summary that never touches the payload
   register, the caller's read is a clobber and the pair is **vetoed**.

Evidence 3 is the only one that looks at the callee and it is one-sided: it can
refute a pair, never confirm one. So `ScalarPair` at the consumer means *no
counter-example*, not *the callee returns a pair* — that positive fact is not
derivable here. A recovered prototype is never written back to the symbol table,
so a caller has no recovered callee signature to consult, and what remains after
the veto is exactly the evidence upstream Ghidra ships this branch on unguarded.
The honest reading of the consumer half is: **complete the stubbed multi-trial
branch, and refuse it where the callee refutes it.**

The probe is the reason `Funcdata` carries a per-callee write summary at all. The
per-function `ArchContext` the pipeline runs against carries the load image but
no translator, so the callee's instructions cannot be read at the seam itself;
the driver takes the probe once the flow build has produced the call specs, and
caches it on the `Architecture` so each distinct callee body is decoded once per
run rather than once per caller. Nothing is probed unless the rule is live.

When the classification says `ScalarPair`, `build_call_output_pair` completes the
stubbed multi-trial branch: the CALL gains the `join`-space output covering both
registers and each half becomes a `SUBPIECE` of it, inserted after the call, with
the INDIRECT creations destroyed. The stub's recorded blocker — no
`constructJoinAddress` on the merged arch handle — was stale; the sibling
`build_return_output` calls it today.

Keeping the pair alive does not disable the phantom-killer above it. The
uncomputed-half repair still runs, later, on the pair this rule preserves, so a
half that is genuine leftover is still dropped — by the rule that can tell,
instead of by a width heuristic that cannot.

The gate is three-valued because the language fact and the forcing switch are
different questions. `auto` acts only on an image the loader's source-language
detection reported as rustc-produced (`Compiler::Rustc`, recorded on the
`Architecture` at `load file` and copied into the per-function snapshot per 00
§0.5); the XML `<binaryimage>` bootstrap never runs the analyzer tier, so `auto`
is inert on the datatest corpus **by construction**, not by luck. `always` drops
the language test, which is what `tests/stages/kuna-rustabi.xml` needs — a
`<bytechunk>` carries no `.comment` record to detect. The shipped default is
`off`: the pair this keeps alive is rendered as a raw fixed-size container until
a later pass gives it an enum type, so the option buys information at the cost of
polish, and that trade is the operator's to make.

What this deliberately does **not** do: it does not name anything `Result` or
`Option`, does not synthesize a union, struct or enum type, and does not touch
emission. Its entire deliverable is that the payload exists as a variable and is
connected to its producer. Spelling that value as a Rust enum is a chapter
[05](05-types.md) decision that cannot be made until the value survives to be
spelled.

### The ABI seam (`kuna_langabi.rs`)

**(kuna, output languages)** How a recovered calling convention *appears* is a
property of the output language, so it is a seam rather than a constant:
`p4_calls/kuna_langabi.rs` defines `LangAbi`, reached through
`OutLang::abi()`, with one method — the `extern "..."` marker a function's
signature must declare.

The axis is thin on purpose. The other two output-language axes are thick
because they have to be (every statement has a shape, every value has a type);
this one is thin because **`extern "Rust"` is unspecified**, and for the scalar
arguments a decompiler actually recovers it is System-V-shaped — the same
convention the cspec already describes. A `build_param_list("rust")` strategy
would encode a guess as an engine fact, which is precisely what `fspec.rs`'s
strategy allowlist (`""`/`"standard"`/`"register"`, error otherwise) exists to
prevent. Rust's genuinely distinct ABI surface — a niche-optimized
`Option<&T>` that is a nullable pointer, a `Result<T, E>` tagged across
`rax:rdx`, a slice passed as a `(ptr, len)` register pair — is an **enum and
discriminant inference** problem belonging to chapter [05](05-types.md), not a
convention problem belonging here. Modelling it as a convention would be
modelling the wrong thing. Measurement has since sharpened where the `rax:rdx`
half of that sits: the *storage* is not a Rust-specific convention at all (the
cspec's `join_dual_class` rule already describes it and kuna already recovers
it), so what P4 owns is keeping the recovered pair alive and connecting it at
the call — `option rustabi`, §4.2 — while naming and typing the value stays a
chapter 05 problem.

The one decision the seam does own is load-bearing rather than decorative: Rust
declares `extern "C"` exactly when the recovered prototype is variadic, because
a C-variadic parameter is only legal on an `unsafe extern "C" fn` and rustc
rejects it anywhere else. Every other function declares nothing, which means
`extern "Rust"` — the default, and unspellable. That is the honest answer rather
than a conservative one: marking every recovered function `extern "C"` would
assert a convention the recovery cannot support, and a Rust binary's own
functions are exactly the ones that are *not* `extern "C"`. C declares no
`extern` at all; its convention, when shown, is the `option conventionprinting`
keyword (`__cdecl`), a different token in a different position.

Nothing in this phase's recovery changes: the seam is consulted by the P9
prototype emitter (chapter [09](09-emission.md) §9.6), and no pass here reads it.
A third language is what would make it thicker — Go's ABI genuinely differs
(a register ABI since 1.17, multi-value returns, a two-word interface and slice
representation), and would add a preferred prototype model consulted where
`ActionPrototypeTypes` picks one, plus a multi-return form consulted where the
`RETURN` op is emitted. Neither is added ahead of a consumer: the `ArchContext`
that P4 action reads carries only `defaultfp`/`evalfp_current` and no named-model
registry, so a `preferred_model` hook today would be plumbing in service of a
function that returns `None` for both languages kuna emits.

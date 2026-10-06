# 04 — Calls & prototypes

`armfloatargs on` (default off) recovers scalar ARM32 VFP inputs only when
the container states the hard-float ABI and `armfloatreturn` is on. Alone it
changes nothing: the inputs it recovers rely on that option's widened
d-register model and on its rule that keeps an unused VFP parameter in its
slot. Floating operations and proven callee contracts supply the type evidence:
a value passed to a call takes the type the callee states for the slot it is
passed in, so a wrapper that moves its double from d2 to d1 for its callee still
reads one double. Same-width copies, contiguous pieces and truncations of a
right shift by whole bytes can carry that evidence back to an input. A double
consumed as a whole
stays one eight-byte input, including when an overlapping call result is only
four bytes. Unknown calls and ambiguous ABI attributes do not establish scalar
floating parameters; a floating aggregate or vector is seen only as the scalar
words or doubles its reads show.

A d-register input that the function reads only as its two words, each of
which a floating operation consumes or a callee states as a float, becomes the
two s-register inputs: heritage joined two single-precision parameters because
another access covers the whole register. Words read only as integers, such as
a double handed to a base-AAPCS helper in core registers, are left as
`armfloatreturn` alone leaves them.

The AAPCS-VFP back-fill slot is not a parameter. When a single-precision
parameter fills the low half of a d-register and every later VFP parameter is a
double, the high half is the slot the convention skips: `f(float, float, float,
double)` passes the double in d2 and leaves s3 empty. Recovery leaves that slot
out instead of filling it with an unused `float`, so the recovered list is the
one the convention assigns and a caller has no reason to read s3. A slot the
function's own body reads before writing is a parameter all the same: at -O0,
`f(float a, float unused, double d)` spills `unused` from s1, and a bounded
decode of the body from its entry sees that read, so s1 stays in the list and
its callers pass it.

At a call, the callee's stated single-precision parameters are taken word by
word out of a d-register range the call may not take whole, so values written
into s0 and s1 after an earlier double result reach the call as written. When
the callee's recovered contract is arity-sound, the call takes the VFP inputs
that contract states up to the last one the caller wrote for the call or the
callee's body, followed through its own calls, is seen to read, so caller and
callee agree on every VFP position in front of it. A contract that the callee's
own body contradicts shapes no call, and the call keeps the arguments it
recovered: when the body reads the high word of a d-register parameter slot but
not its low word, and the contract states neither, the convention allocated the
low word first, so the contract misses an input. A stated double that the call
holds as its two s-register words, because heritage split the d-register where
the caller writes only a single-precision half of it, is passed as one value
built from those two words where the callee's body, followed through its own
calls, reads both words: a float-returning wrapper that hands its double to its
callee in d0 and uses it again after the call passes one double, not two integer
words. Where the callee provably ignores a stated double the call holds, whole
or as words, and passing the caller's value would make the caller read an entry
register it does not itself use whole (its own float in s0 beside an s1 it never
writes, or two floats it reads apart), the argument is zero and the caller keeps
its float parameters: `float w(float a) { return f(8.25, a, a, 5.25) * 3 + a; }`
calls `double f(double, float, float, double)`, which never reads its first
parameter, as `f(0.0,a0,a0,5.25)`, not with a double `w` would then take. A
stated input the callee never reads is zero as well where the caller's value for
it is part of an earlier call's result left in the register, such as the s1
above a float an earlier call returned in s0: passing that word would turn the
earlier call's float result into a double. This holds for a whole earlier result
too, since a callee that never reads the register cannot tell, and the result
would otherwise take the slot's stated type: a float result in d0 handed to an
ignored stated double became 64-bit integer bits. A double the caller computed,
forwards untouched or also reads whole is passed as it is. Where the callee
never reads one word of a stated double and the caller left an earlier call's
leftover in that word, that word alone is zero: an -O0 callee that only spills
its leading float can be recovered as taking a double there, and its caller then
holds its own float beside the high word of a double an earlier call returned,
which would turn that result into integer bits. The callee never reads a
register when every path writes it first, or when it is a leaf that returns
without touching the register and every part of it inside the storage of the
callee's recovered result, whether or not every return computes that result, is
written on every path first. A leaf that returns early with its first parameter
untouched, `float f(float a, float b, int c) { if (c > 3) a = b * b; return a;
}` (`cmp r0,#4; bxlt lr; ...`), hands the caller's s0 back and so reads it: a
caller passing an earlier call's result there keeps it. Where the callee reads
the double but the caller's two words are not one double it forwards or reads
whole, the call's arguments stay as recovered. A stated input there is an
argument even where the positional rules ended the list before it: a constant
the caller left in d1 for its own arithmetic is the argument for an ignored
`double` the callee states there, and the used double after it stays in place. A
stated input there that the caller never wrote is passed as whatever value
reaches the call, which is how a wrapper forwards its own inputs, unless the
callee never reads that register: the argument is then zero, because a fresh
read of the register would only give the caller a parameter it never uses or
turn an earlier call's float result into a double. Stated inputs past that point
stay as the call recovered them, so a contract that lists more registers than
anything is seen to use adds no argument. A positional filler the contract skips
while stating a later VFP input, such as the back-fill slot, is dropped. Any
other unstated VFP input is dropped only where the callee's body, followed
through its own calls, neither reads nor forwards that register: an s-register
in which the caller happens to leave a value, such as a constant it used for its
own arithmetic or a stale half of an earlier double, is then not an argument,
while a callee whose body is cut short or reaches code no walk accounts for
keeps every argument its callers recover. A leaf callee that only returns
forwards nothing, including when its return switches the instruction set through
a user operation as ARM's `bx lr` does, so a register it never reads is dropped
there unless part of it lies in the callee's result storage and some return
leaves that part untouched: a direct caller does not pass the back-fill slot it
never wrote, and that slot does not become a parameter of the caller. A stated
input the caller's own scoring ruled out has already lost its value, so it is
kept, as zero, only where the callee's body provably ignores the register; where
the callee reads it, or where the call holds a stated input only in part or
under an unstated argument it keeps, the call's arguments stay as recovered. A
call that recovered no argument at all, to a callee that also states
core-register inputs, is left to the rescue that recovers both banks. A word the
caller forwards is never passed as the callee's stated double.

Every rule above reads the callee's body through the bounded decode from its
entry, which credits a conditionally executed write (ARM's `vmovgt s0,s1`, a
Thumb IT block) on both paths, and ends the path at a conditional return such
as `bxlt lr` without decoding the code after it. With the option on, the
paths that skip such a return are decoded too, separately and each from the
registers written before the return's condition, and only the leaf rule reads
them, so every other rule sees the decode as before. A register counts as
written on every path only where an instruction that always runs is the first
to write it on that path, so `vmov.f64 d0,d2` followed by `vmoveq.f64 d0,d1`
still writes d0 first, and the leaf rule needs a decode that, past every
conditional return as well, left no path behind, called nothing, and saw no
read of the register and no conditional first write to it. `float f(float a, float b, float c, float d, int k) { if (k > 6) c
= a; else if (k < 4) c = (d + a) / 2; return c; }` compiles to `cmp r0,#6;
bxgt lr; ...` and reads `b` on no path, so a caller that holds nothing of its
own in s1, only what an earlier call left beside its float result, passes zero
there (`f(5.5,0.0,v1,...)`) and that earlier call still returns a float.
`float pick(float a, float b, int c) { if
(c > 3) a = b; return a; }` compiles to `cmp r0,#3; vmovgt.f32 s0,s1; bx lr`,
which leaves s0 untouched when the move is skipped, so a caller's
`pick(v,2.5f,n)` keeps `v`. `calleedeadarg` asks the same question of a VFP
register with the option on, so it does not clear `v` before these rules see
the call. With the option off the decode and `calleedeadarg` are unchanged.

Declared prototypes keep their parameter order. For stripped functions whose
core and VFP banks do not reveal source order, recovery uses the model's VFP
then core order consistently. Whole-program callers use the established
`protoorder types` and `passthrough on` contract propagation; a single-function
run without a callee contract cannot recover an untouched forwarded argument.
Forwarding and wrapper-return claims are established before synthetic narrowing
inputs are inserted. If heritage splits a forwarded double around a float result,
same-width copies and contiguous incoming pieces still establish the original
input for call recovery. A computed value or an earlier call's result does not.
Resolved variadic format calls use explicit base AAPCS storage, including
their floating arguments, rather than the non-variadic VFP convention.
`formatstring off` disables that source of type evidence.

Nine call shapes stay incomplete. A double that the caller forwards untouched,
but of which only one word is live in the caller, is omitted from the call. A
wrapper that forwards a single-precision argument from the upper half of a
d-register without touching it, such as the third float of `w(float, float,
float)`, recovers none of its VFP arguments. When a call site recovers its
core-register arguments but not a forwarded VFP argument that the callee's
prototype lists first, the recovered arguments print from the first position, so
a core argument can stand in a VFP parameter's place. A floating argument that goes on the stack because the VFP
registers are full, such as the ninth parameter of `f(double, ..., double,
float)` after eight doubles, is not recovered on either side. A double that a
callee reads only as two integer words is outside this option. Single-precision
FPUs such as the Cortex-M4F hand every double to base-AAPCS helpers this way, so
such a callee and its callers keep what `armfloatreturn` alone recovers for
them. An unused trailing single-precision parameter that an -O0 body only
spills to its frame is left off the callee's recovered list, because the spill
is a dead store, while each caller still passes the value the callee reads
there, so such a call has one more argument than the callee lists:
`f(double, float, float, float)` with its last float unused is one. A function
whose only read of a d-register before writing it whole is the high word, such as
`f(double, double, float, float)` at -O2 when it ignores the first float and
then reuses `d2` as scratch, recovers neither word: the used float is read from
an uninitialized local, as with `armfloatreturn` alone, and its callers pass
it, since that contract shapes no call. A callee that returns a two-float aggregate in s0 and s1 is recovered as
returning one float, so a caller that reads the second member takes the call's
result as a 64-bit integer and converts its high word by value, as with
`armfloatreturn` alone; with the call's arguments now complete, that caller
compiles and computes the wrong member. A float-returning callee that computes
in d0 as a double on some path before narrowing into s0, such as `float f(float
a, double b, int k) { if (k > 4) { a = b * b + 1; a = a * b - 2; } return a;
}`, is recovered as taking and returning a double, as with `armfloatreturn`
alone, so its callers now compile and compute the wrong value.

With `stackaddrargtrial on` (default off), an existing register input trial can
use a bounded same-width copy/displacement chain to a specific stack-pointer
value as argument evidence. This retains a passed local address despite other
uses of the frame. The proven register trial also survives the inactive-prefix
length heuristic; normal ABI hole filling supplies its earlier register slots.
Known dead callee inputs, definitely-unused slots, non-register entries, unknown
bases, phi merges, width changes and call-clobbered pointer chains keep their
existing treatment. Ordinary call alias handling then preserves output loads
and their dependent conditions. A stack address may be incidental, so a known
interface should use a declared prototype instead of this inference.


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

The model's stack ranges come from the spec too, as upstream `ProtoModel::decode`
takes them (`option protoranges`, default on, §6.2): the parameter range is every stack `<pentry>` of `<input>` plus any
`<paramrange>`, and the local range is the `<localrange>` element
(`decompiler/crates/kuna-decomp/src/infra/architecture.rs (decode_proto_ranges)`).
Only a spec that states neither keeps `defaultParamRange` (stack 0..511) and
`defaultLocalRange` (the million bytes below the return address). The
difference is visible: x86-64-win.cspec declares the caller-allocated home
area (stack 8..39) a local range and its stack parameters as starting at 40,
and the default parameter range swallowed the home area, so no local was ever
mapped over a home slot. A value MSVC spills into one and passes by reference
printed as a store to one variable and a call on `&Stack0000000000000018`;
with the spec's ranges it is the local array the stock decompiler prints. MIPS
o32 says the same of its 0..15 register backup area, so a spill there that
nothing reads is dead and no longer makes an unused argument a parameter.

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

Registration selects nothing by itself. Which model a function is evaluated with
is unchanged by the presence of the named ones; three things read the registry.
The first is a **declaration that names its convention** — `--assert 'prototype
<func> void * __stdcall f(...)'` and the console `map prototype` / `parse line
extern` behind it (00 §0.2). The parser only classifies an identifier as a
convention because the registry names it, so the resolution cannot fail: the
resolved `ProtoModel` is recorded against the function
(`decompiler/crates/kuna-decomp/src/infra/architecture.rs
(Architecture::set_function_prototype_model)`) and used in both directions. The
function's own prototype is seeded under it rather than under `defaultfp`, and
the prototype-bearing `TypeCode` the declaration locks onto the symbol is built
under it too — which is the copy a CALLER reads (`ArchContext::query_callee_proto`
/ `ArchContext::callee_proto_model`,
`decompiler/crates/kuna-decomp/src/substrate/context.rs`), so declaring a callee
`__fastcall` moves where the caller's arguments come from. Without that second
half the keyword would parse and change nothing, which is the failure mode the
directive exists to avoid. The other two are
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

The bundled AArch64 cspecs (`AARCH64.cspec`, `AARCH64_apple.cspec` and
`AARCH64_win.cspec`) never split an argument wider than one general register
between x7 and the stack. AAPCS64 passes such an argument wholly in registers
when enough are left; otherwise it goes wholly on the stack and no later
argument uses a general register. Two rules ahead of the general
`<join align="true"/>` fallback say so for arguments of 9 bytes and up: a join
with `stackspill="false"`, then `<goto_stack/>` with
`<consume_remaining storage="general"/>`. With only x7 left, a 16-byte struct
therefore occupies the first two stack doublewords and a later `long` the
third, at the call site as in the callee. Arguments of 8 bytes or less, float
and homogeneous-float-aggregate arguments, and arguments that fit in the
registers left are assigned as before. The Windows variadic exception is kept:
Microsoft's ARM64 ABI lays out every argument of a variadic function as if on a
stack whose first 64 bytes are x0–x7, so a `<varargs/>` join ahead of the
no-spill rule still splits a composite between x7 and the stack there.
These rules model the C ABI only: a Rust-ABI two-scalar pair, which rustc may
split between x7 and the stack, is not modelled, as on x86-64.

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
   **own** inputs rather than a call's, and (kuna) `stackarggap` ends a call
   site's chain at an *unreferenced register* slot whose next credible slot is
   on the stack — both below.
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

Step 4's *promotion* half has a boundary problem of its own, and this one is a
statement about the ABI rather than about evidence. The rule fills interior holes
so that the recovered list is contiguous, which is right inside the register
file: a caller can pass an argument in `rdx` and leave `rcx` looking empty, and
reading that hole as the end of the list is how a call loses arguments the
disassembly plainly passes. It is not right across the register/stack boundary,
because a `ParamListStandard` model allocates in resource order and reaches the
stack only once the register file in front of it is exhausted. A Win64 call has a
fifth argument at `[rsp+0x20]` only if it has a fourth in `r9`; an x86-64 SysV
call spills only past `r9`/`xmm7`. So an argument-register slot with **no
Varnode at all** — one of the unreferenced fillers step 1 synthesized, meaning
nothing in the caller ever wrote that register — is not a hole in the middle of
an argument list but the end of one, and whatever sits in the outgoing-argument
area behind it is the frame's scratch.

The witness is a body-less IAT import in a Windows PE, which is the shape where
this bites hardest: with no body the callee cannot answer for its own arity, so
the call site's leftover state is all the recovery has. `EVP_DigestFinal_ex`,
whose signature takes three arguments, is called with `rcx`/`rdx`/`r8` loaded,
`r9` never written, and a `1` left in the fifth-argument slot by the frame's own
bookkeeping; the stack trial scores active, step 4 promotes the unreferenced
`r9` filler to reach it, and `build_input_from_trials` materializes the Varnode
that trial never had by reading `r9` at the call — which resolves to the
*caller's* untouched incoming `r9`. kuna emitted
`EVP_DigestFinal_ex(v3,v9,v7,a3,1)`: five arguments, of which the fourth is the
reader's own parameter and therefore looks load-bearing. (kuna) `stackarggap`
(default-on, `decompiler/crates/kuna-decomp/src/p4_calls/kuna_stackarggap.rs`)
sets `seenchain` at such a slot, so the chain ends there, the stack trial behind
it is deactivated with everything else past the cut, and the hole is never
filled. This is the same conclusion step 4 already draws one slot to the right,
where an unreferenced *stack* trial in sub-call recovery ends the chain
immediately; the option extends it to the register slot in front of the stack.

Four clauses bound it. It reads `ParamActive::is_recover_subcall`, so the
function's own input recovery — where an untouched argument register is an
ignored parameter, which is `inputparamgap`'s whole premise — is untouched. It
fires on **unreferenced** trials only: an inactive register trial still has a
Varnode, meaning the caller put something there and trial scoring merely could
not prove it was for the callee, and that ambiguous case still fills, so a
wrapper forwarding its own fourth parameter is unaffected. It fires only when the
next non-eliminated trial in the section is a stack slot, so a hole in the middle
of the register file with a written register behind it keeps the upstream fill.
And it only stops a trial from being marked active, never marks one — an argument
list can lose an invented tail, never gain a member. A variadic call site reaches
none of it, because `varargstackargs` has already cut its stack tail into its own
section and the register prefix is never scored against it.

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
carry. Its usual evidence is the **callee's own body**. It reuses the bounded entry decode
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

A stripped printf-like callee is the useful exception to that body test. Its
variadic register-save prologue reads every argument register, but two calls
with the same format still have a fixed arity. `kuna_formatwitness.rs` accepts
that alternate evidence only when both calls pass the same nonzero constant as
their first argument, the pointed-to bytes parse as a conservative printf
format, and the sibling recovered exactly one argument plus every conversion
and `*` operand in the format. Positional and malformed formats, unreadable
strings, different pointers, and mismatched arities decline. This recovers the
case where argument values are also stored to program state immediately before
the call and `only_op_use` rejects their trials for those stores.

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

All three of those rules are *sibling* reconciliation: each needs another call to
the same entry address in the same function. That leaves the callee called
exactly **once** with nothing to compare against — a thread entry point, a
one-shot payload, a handler reached from one place — and the family is inert
there by construction, not by choice. On a Win64 image a caller that passes its
own first parameter straight through (`mov rbx,rcx; cmp dword[rcx+0x238],edi;
je payload_call`, with `rcx` untouched to the `CALL`) renders `payload();`, while
decompiling that same callee on its own recovers `payload(unsigned int *)`. The
only other mention of the callee in that function is an address-taken `lea` for a
`CreateThread` argument, and an address-taken `lea` is not a call site, so the
witness search has nothing to find.

(kuna) `calleearitybody` (default-on,
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_calleearitybody.rs`) lets the
callee-body evidence stand **alone** as the witness in that quadrant, rather than
only extending a sibling's list. It takes the same summary
`calleearitylive` reads (`kuna_calleedeadarg.rs (probe_callee_entry_dead)`) and
walks the call's trials in prototype order: the recovered list is the leading run
of argument registers the callee is proven to **read before writing**, and that
run must end at a register the callee is proven to **overwrite before ever
reading**, on every path. The second half is the whole safety of the rule.
"Some path reads this register" is an existential and would happily run to the
end of the argument registers on its own; what bounds an argument list is a
register the callee is *proven* not to consume. It is also what refuses the shape
the family's sweep found — a variadic register-save prologue reads every argument
register there is, so no dead one follows the run and the rule declines rather
than inventing an argument per register. An import, a thunk, and a body past the
decode budget answer neither and decline the same way.

Everything else is `calleearity`'s, unchanged: register storage only, real
Varnodes only, all-or-nothing, never subtractive, and only a call that recovered
**nothing at all** — a site with a partial list is `calleearitylive`'s. It
captures its candidate Varnodes in `build_input_from_trials` like the other two
deferred rules and replays them at the end of the same `ActionActiveParam::apply`,
after both of them, so a site any sibling can speak for is already non-empty and
is left alone. It is inert unless `calleearity` is also on. Because its whole
subject is the callee called once, it is also the one reader for which
`seed_callee_entry_dead` probes a function with fewer than two calls.

(kuna) `calleearitycut` (default-on,
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_calleearitycut.rs`) widens that
dead-boundary test, because the boundary is unavailable far more often than the
run is. `probe_callee_entry_dead` ends **every** path at the callee's first
nested call, so a register the callee clobbers past that call is invisible to it,
and an entirely ordinary body proves a read run and no boundary at all. On an ELF
x86-64 image a caller that passes its own `rdi`/`rsi`/`rdx` straight through and
writes `mov ecx,0x1` before the `CALL` renders `sub_875e0();`, while its callee's
prologue reads all four (`lea rax,[rsi+rdx]`, `mov eax,ecx`, `add word ptr
[rdi+0x30],1`) and only then calls; `r8` is clobbered 0x33 bytes past that call.
The caller side cannot rescue it either, and for a reason that is upstream policy
rather than a gap: `AncestorRealistic::execute` refuses an input Varnode outright
— *if the parameter itself is an input, we don't consider this realistic, we
expect to see active movement into the parameter* — so the three pass-through
registers score inactive and `fillinMap` reads the hole in front of the one
written register as the end of the list.

A boundary the callee proves dead is not the only way to know a run is the whole
list; it is the only way to know it when the run reaches the **last** argument
register. When the run stops short, the register it stops at is itself the
boundary, and what has to be ruled out is that the register carries an argument
the cut walk could not see. Three conditions, and the rule is only as safe as
their conjunction. The run must be **contiguous** — no skipped register inside
it, so every emitted argument sits at the position the ABI assigns it and the
only reachable error is a *missing trailing* argument, never a misplaced one. It
must **stop short** — at least one register argument location must follow it,
unclaimed; a run that consumes every argument register has no boundary and is
refused, which is the variadic register-save prologue again. And the boundary
must be **quiet** — the caller must not have placed a value in that register,
neither a computed one nor a constant, since a caller that does is passing a
further argument whatever the callee decode can see. All of `calleearitybody`'s
own guards still answer first, and the rule is inert unless it is on.

(kuna) `calleearityscratch` (default-on,
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_calleearityscratch.rs`) widens
the last of those three, the **quiet** boundary, because "the caller wrote it"
and "the caller is passing it" are not the same claim and a register allocator
does not respect the difference. An ELF x86-64 crackme decodes a string with an
unrolled XOR loop and hands the buffer to its hint printer: `lea rdi,[rbp-0x30]`
sets up the argument, `movzx esi,byte ptr [rbp-0x1a]` puts the XOR key in the
*second* argument register, `xor byte ptr [r8+0x6],sil` consumes it, and the
`CALL` follows. The callee reads `rdi` at its first instruction, so the run is
`{rdi}`; its decode is cut three instructions later at a nested call, so nothing
proves `rsi` dead; `rsi` is the register the run stops at, the caller wrote it,
and the whole one-argument list is dropped.

Argument setup is a value the caller computes and then uses for **nothing but the
call**, which is `Funcdata::onlyOpUse` — and that question has already been asked
by the time the rule runs. `check_input_trial_use` puts every trial through
`ancestor_op_use` and marks it **active** only when the answer is yes, so the XOR
gives `rsi`'s trial an *inactive* score. An inactive boundary is scratch and is
quiet enough to end the run; an active one is upstream saying the value reaches
the call and nothing else, which is exactly the further argument `calleearitycut`
refuses to hide, and it stays refused. So does a **constant**, however the trial
scored: materializing a constant into the next argument register right before a
`CALL` is argument setup in its most literal form. The other two conditions are
untouched, so the reachable error is still a *missing trailing* argument and
never a misplaced one — and the site the rule fires on rendered no arguments at
all, so it trades an empty list for a prefix the callee's own body proves it
reads. Inert unless `calleearitycut` is also on.

Every rule above is additive: each one can only put an argument back. The
opposite error is just as real, and its evidence is already on the trial.
`Heritage::guard_calls` plants an INDIRECT *creation* for a killed-by-call
register that is also a possible **output** location of the callee's model — on
x86-64 `rdx` is one, because a 16-byte value returns in `rax:rdx` — and
`AncestorRealistic::enter_node` answers realistic for exactly that op, recording
`set_ind_create_formed` on the trial as it passes. With `only_op_use` satisfied
the trial scores active and `fillin_map` marks every active trial used, so a
register the caller never wrote for this call becomes its trailing argument and
the local it reads is assigned nowhere in the function. coreutils `fmt` -O2
renders `sub_3700(stdin,"-",v10)` against a DWARF prototype of two parameters,
with `unsigned long v9; // rdx` and `unsigned long v11; // rdx` never written;
stock Ghidra emits the same three-argument call and names the phantom
`extraout_RDX`. The `ind_create_formed` bit is read back on the **return** side
only (the remainder-formed and indirect-creation-formed rejection in
`fillin_map_standard_out`, below): the input list has no such test, so the
evidence that disqualifies a return value is ignored for an argument.

(kuna) `argclobber` (default **on**,
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_argclobber.rs`) applies that
same sentence to the input list, bounded by eight clauses.

The trial must be a **register** trial carrying `is_ind_create_formed` — the
upstream return-side test, read on the input list.

The clobber must reach the call **directly** — the trial's defining op is the
indirect creation, or a MULTIEQUAL one of whose immediate inputs is one — because
a clobber that merges in behind a further join says nothing about the argument:
tar's `str_days(pc,buffer,n)` is called with `n` live from a dominating block and
a clobber joining in one phi deeper, and that argument is real.

Every one of those indirect creations must be a creation of **the argument
register itself**. This is the clause that separates a clobber from a *return
value*, and the distinction is not visible in the bit: on every ABI the first
return register is also killed-by-call and a possible output location, so until
the output seam resolves it a preceding call's result is an indirect creation
exactly like a clobber is. When the caller then moves that result into an
argument register, copy propagation puts the creation straight into the
argument's join — at its own address, not the argument's. u-boot -O2
`sub_60827fa4` is that shape: `r0` holds
`ofnode_read_u32_default(dev->node,"bus-width",1)`, three `cmp`s test it, and a
pair of predicated `mov r3,r0` carry it into
`printf("%s %s: Invalid \"bus-width\" value %u!\n",dev,name,width)` as the value
`%u` prints. A register the caller wrote is a register the caller is passing,
whatever the value in it came from.

Every **other input of that join** must be a **division by-product** — the value
an `idiv` leaves in the remainder register while the caller goes on to use only
the quotient, reached through the width adjustments a compiler puts between the
`INT_REM` and the register (a `SUBPIECE` of the low half, a zero or sign
extension, a constant mask) and nothing else. This is the same statement as the
clause above, applied to a join rather than to a single def: one clobber among a
phi's incoming values says nothing about the other paths, so each of them has to
be positively classified as a value nobody placed either, and the by-product is
the only class that qualifies. Upstream is the authority for treating it that
way, on the return side: `fillin_map_standard_out` rejects a remainder-formed
piece exactly as it rejects an indirect-creation-formed one. Without this clause
a caller that forwards its own second parameter on the non-clobber path
(`mov %r12,%rdx`, joined with a clobber) loses both the argument and the
parameter, and `void caller(long,unsigned long)` renders as `void caller(long)` —
a deletion that reaches the JSON `variables[]` surface as a missing `kind: arg`
row, not just the C text.

The **callee's own recovered prototype** must exist, must have no parameter
overlapping those register bytes, and must *account for every argument the drop
leaves behind* — each recovered parameter covered by a surviving argument and
each surviving argument covered by a recovered parameter. This is the clause the
rule rests on, and the reason it can be a default. Nothing on the caller's side
can say whether the callee wanted the register; the parameter list kuna gets from
decompiling that function can, and `protoorder` (above) states exactly that list
for every callee it decompiles before the caller.

The second half is not bookkeeping. A recovery *short* of what the call passes is
not a statement that the tail is unwanted — it is a statement that the recovery
did not reach the tail, and from the caller's side the two read identically. A
forwarding thunk is the sharp case: `mov (%rdi),%rax; jmp *%rax` never names the
registers it passes through, so kuna recovers one parameter for it while the
function it tail calls consumes three, and `protoorder` states that short list in
*both* its modes. With only the free-bytes test the drop is admitted and a
forwarded argument is deleted; requiring the recovered list to account for the
two surviving arguments declines it, because one parameter cannot be two
arguments.

Requiring the prototype to **exist** carries the rest, because `protoorder`
states nothing for a callee with no recovered body such as a PLT import, one that
recovered no parameters at all (its statement, when it makes one, carries only the
return value, and an empty list is declined here), one whose prototype is already *declared* — that
case input-locks the call spec, which this rule declines at its first line — and,
under `types`, one inside a recursive component. Under the default `cycles` a
recursive callee states its list like any other; a member whose list is short
because it hands a register on to a partner is answered by the callee-body walk
below, which follows the partner's body and proves nothing about a cycle it
re-enters. "Nothing parked" and "cannot tell" are
therefore the same answer and both decline, which also makes the rule inert
wherever no callee was decompiled first: a single-function `kuna decompile` (it
forks one process per function), a `decompile-all` narrowed by `--addr` or
`--functions`, a `--jobs N` run, and `--option protoorder off`.

`protoorder` does **not** decline a variadic callee, and this rule must not be
read as resting on that. The variadic guards described above — the under-recovery
walk and the register-file boundary — belong to the branch that parks a prototype
in the symbol table, and the stating modes (`types` and the default `cycles`),
which are the ones this rule reads, return before them. The declared-`...` decline cannot fire on a stripped
image at all, because a declared prototype is rejected one branch earlier. A
stripped SysV variadic is therefore *stated*: `long vlog(int,long,...)` built
with `gcc -O2` and stripped states three parameters, and `KUNA_PROTOORDER_TRACE=1`
prints `state sub_11d0 params=3` for it. What declines the drop there is the list
itself — a register-save prologue reads every argument register the convention
has, so the recovered list carries the register the drop would take. Where a
variadic recovers short instead, the surviving-argument accounting and the
callee-body veto below are what answer for it.

The **callee's own body** is read twice more, and both readings are required.
Both come from the per-image entry-liveness summary `calleedeadarg` (below)
already builds. A bounded entry walk that positively sees those register bytes
read before they are written declines the drop (`proves_input`), and it settles
a parameter the callee's own recovery missed — bytes the callee reads at entry
are a parameter however the value got into the register. It is also what answers
for a callee kuna never recovered as variadic: u-boot's `printf` is called from
1,924 sites, and its prologue spilling `r1`–`r3` into the `va_list` save area
speaks for all of them at once, which no per-function sibling scan can.

The second reading is the one a prototype cannot give, and it does not stop at
the callee's own body. The entry walk ends every path at the callee's first
call, so what the register meets past one has to be asked of that call's target
(`resolve_forward_transfer`). Two answers let it through: a **declared,
non-variadic** prototype — a library signature, DWARF, a console declaration —
which is authoritative about what the target reads and is the only answer
available for a target with no body to read; or the target's own body, with the
same question put to it recursively. Everywhere else the callee must already
have written the register itself before control leaves, and the register is not
free if it has not: an indirect call or tail call, a `CALLOTHER`, an indexed
register-file access, an undecodable instruction, a recursive component, a walk
that ran out of budget, and an import whose PLT stub jumps through its GOT slot.

Naming a target is not accounting for it, and the difference is what two plain-C
programs are made of. `long fwd(struct box *o,long a,long b) { if (!a) return 0;
return o->fn(o,a,b); }` compiles under `gcc -O2` to `test %rsi,%rsi; je; jmp
*(%rdi)`, so kuna recovers `(rdi, rsi)` — honestly, the two registers the body
touches — while the function it jumps to consumes three. `long wrap(void *o,long
a,long b) { if (!a) return 0; return ext3(o,a,b); }` has no function pointer at
all, but `ext3` is an import nothing states a signature for, so nothing at that
call site reads `rdx` and kuna recovers two parameters again. In both, two
recovered parameters are exactly the two arguments a drop would leave behind, the
accounting above is satisfied, and the third argument is deleted although the
program reads it. `rdx` is unwritten at the `jmp` in the first and at the `call`
in the second, and that is what declines them — in the second only because the
call's target is asked and cannot answer. The programs are
`docs/features/argclobber/ce-forward-thunk-2param.c` and `ce-import-forward.c`
(with `ce-forward-thunk-2frame.c` putting an ordinary direct call in front of the
first), and `tests/cli/argclobber-keeps-an-argument-a-thunk-forwards.json` and
`…-an-import-reads.json` pin them. Neither reading can admit a drop the clauses
above refused.

The trial must be **trailing**, so the list keeps its positional shape and no
other argument moves. At least one argument must remain, since an empty list is
`calleearityfwd`'s failure shape and not this one's. And no already-final call to
the same callee entry in this function may have passed an argument in that
storage — the subtractive twin of `calleearity`'s witness rule, and the statement
of how this pass is ordered against the additive family: it runs immediately
after `unify_with_sibling_call` in `build_input_from_trials`, and a slot the
sibling rule promoted outranks the clobber evidence here. The drop is
`mark_no_use`, which every deferred member of the family reads as
definitely-not-used, so none of them puts the argument back.

Dropping the argument also **narrows the preceding call's return value** wherever
that call was the register's only writer: the `SUB168(v,8)` that extracted the
`rdx` half of a 16-byte return loses its only reader and goes with the argument.
No drop in the 770-binary corpus does that under the shipped rule. The witness
for it is `docs/features/argclobber/ce-forward-thunk-2param.c`, whose third
argument is real: the forwarding clause declines it here, and with that clause
ablated the drop takes the producing call's `SUB168(v,8)` with it. It is the
same mechanism that would delete a real struct half at a callee whose own
prototype is under-recovered.

What no clause can see is that the evidence is a *recovery*. A callee whose own
parameter list kuna under-recovers states a prototype that admits the drop, and
the argument goes. Two things bound how far that can reach. The
surviving-argument accounting requires the under-recovery to be exactly one slot
deep, at exactly the register the clobber wrote, with every other argument still
accounted for. And the forwarding reading requires the register to be dead at
every control transfer that cannot be followed or resolved through its target,
which is what closes the case where the recovery is short *because* it never saw
the code that reads the register.

The prototype clause and the body walk cover different halves of that, and
neither covers it alone. What the prototype clause removes is the class a bounded
body walk cannot see at all: a read past a jump table, a read beyond the walk's
instruction budget, and a callee that really returns a 16-byte value in
`rax:rdx` and forwards the high half as the next call's trailing argument — in
each of those the recovery reached far enough to name the parameter, so the
recovered prototype carries it and the drop is declined. Where the recovery does
*not* reach that far — a register forwarded to something the recovery could not
read, whether through a pointer, through an import, or through a chain of
ordinary calls ending in one of those — the prototype says nothing useful and the
forwarding reading is what declines the drop.

Together the clauses are narrow. Over 770 stripped decbench binaries (O0, O2 and
O2-noinline) the rule changes 19 functions, one per binary, and each loses
exactly one trailing argument at exactly one call site. Every drop lands on the
callee's true arity, checked against the unstripped twin: `__addvsi3` and
`__mulvsi3` at 15 openssh sites, `fmt(FILE *, char const *)` at the two coreutils
`fmt` mains, `ext2fs_dblist_sort2(dblist,sortfunc)` in `e2fsck`, and
`efi_create_handle` in u-boot. Two call sites *gain* an argument — the
under-recovered sibling site in each `fmt` main, which is what turns its 1/2/3
into 2/2/2. Every hunk in that sweep falls in one of seven documented classes;
`docs/features/argclobber/sweep-2026-09-20-forwarding.txt` has the
classification, and the two `bash` `xrealloc` drops it no longer takes: there the
callee reaches `call sbrk@plt` with `rdx` unwritten and kuna has no signature for
`sbrk`, so the import answers nothing and a correct drop is given up.

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
(Funcdata::apply_locked_prototype)`). The model it is locked under is the one the
declaration named when it named one (§4.1), else the architecture default, with
two exceptions: on an ARM image that states the soft-float convention and no
floating-point hardware, a declaration whose default layout would put a
parameter or the return value in a VFP register is laid out under the soft-float
model instead (see "Declared prototypes on a soft-float image" in §4.3), and on
an ARM image that states the VFP variant a variadic declaration returns under
the base standard (see "Declared variadic prototypes on a VFP image"). Two
sources, in precedence order: a
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
  partially-used wide parameter still exists to take a SUBPIECE. (kuna) Where
  the calling convention states the extension, `narrowext` (§4.4) supplies it
  in place of the model's.
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
  any order. A CALLIND through a literal global slot has one additional early
  source: when input 0 is either the unwritten memory Varnode at that exact
  address or the result of exactly `LOAD(constant-space, constant-address)`,
  the slot is queried before generic recovery. An exact data symbol contributes
  a prototype only when its full storage is a pointer to a prototype-bearing
  code type. Otherwise a loader FunctionSymbol at that exact address may supply
  its address-keyed parked pieces (its synthetic code-symbol size is one byte,
  so the LOAD is instead checked against the address-space pointer width).
  Scalars, pointers to data, code/code pointers without a prototype, interior
  aggregate addresses, truncated/wide loads, COPY chains, and computed addresses
  all stay generic. A later exact data declaration shadows loader metadata even
  when it is non-callable. The priority is call-site override, explicit typed
  data slot, loader FunctionSymbol prototype, then generic recovery; copying the
  current function's own prototype is never a fallback. A `FuncProto` taken
  from a typed data slot is copied intact, including its calling-convention
  model.
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
  call, or (kuna) the one `narrowext` states); a locked *stack* output is
  deferred to heritage
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

(kuna) **A variadic argument in the return register.** The guard skips a range
the call's own output covers exactly (chapter 03), so a variadic call that
passes an argument in the register it returns in lost that argument.
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_varargretreg.rs
(argument_in_own_output)` decides when the input trial is registered anyway. It
appends one variadic argument to the call's declared pieces, a `double` when the
range lies in a floating-point entry of the model and an integer of the range's
size otherwise, and asks the model where it goes. The trial is registered only
when that storage is the range itself. The model's own `<varargs>` rules keep
the floating-point registers out where a variadic value travels in integer
registers or on the stack (ARM, RISC-V, MIPS, Windows, Apple arm64). Three
conventions the models do not describe come from the language id
(`image_vararg_floats`). 64-bit PowerPC passes a variadic floating-point value
in general registers and only copies it to an FPR, so its FPRs are refused.
x86-64 SysV puts the number of vector registers that carry arguments in `al`,
and 32-bit PowerPC sets CR bit 6 when any FPR carries one (`crset 6`, cleared
by `crclr 6`). A constant zero written in the call's block (`xor %eax,%eax`,
`mov $0,%al`, `crclr 6`) refuses `xmm0` or `f1`. When the block does not state
a count, the model decides, as it always does on AArch64, which has none.
The trial then goes through the `ActionActiveParam` checks below like any
other.

A count above zero is also evidence for those checks. `ancestorOpUse` never
accepts a value that is another call's output ("a call is never a good
indication of a single op use"), so `vr(k, g(k))`, which leaves `g`'s result in
`xmm0` and passes it on, printed as `g(k); vr(k);`. That is the safe reading
when nothing says otherwise (on AArch64, `g(k); vr(k);` compiles to the same
instructions), but on x86-64 the `mov $1,%eax` before the call says otherwise.
`counted_float_arguments` lists the first `n` floating-point entries of the
model for a count of `n` (one for `crset 6`, which says only that some FPR is
used), and `check_input_trial_use` marks an 8-byte trial on one of them active
without the ancestor checks. This holds for any variadic call, not only one
whose own value returns in the register: `printf("%f", g())` keeps its `g()`
the same way. Such a trial exists only where the guard registered one, so a
counted register written whole (`pxor`, `movq`) or in 4-byte lanes (`movaps`)
gets its 8-byte trial from the guard in chapter 03
(`counted_float_entry`). Without a count (any AArch64 call, or a block that does not set
one), the ancestor checks still decide: a discarded call result or a parameter
the caller reads again after the call stays out, and so does a call result the
call really takes, which the instructions cannot tell apart. Only a declared
prototype settles that case: see `varargforward` in §4.4.

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
- **Jump-target register** (kuna, `tailcalljump on`): a tail call that flow
  recovered from a veneer's computed jump (chapter 02) records the register the
  jump read its target from (`fspec.rs (FuncCallSpecs::carries_jump_target)`).
  A register trial overlapping it is no-use: at the call it holds the callee's
  own address, so `ldr r1,=f; bx r1` never prints `f(a0,f)`.
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

  (kuna) A system call that `syscallregs` rewrote
  (`decompiler/crates/kuna-decomp/src/p2_lift/kuna_syscallregs.rs
  (reads_as_argument)`) is treated as a call site, not as an op the value flows
  through. The walk does not follow it to its output, because that output is the
  kernel's result. When the trial is a RETURN's, a value the system call reads as
  an argument is not exclusively a return value: the kernel preserves its
  argument registers, so after `svc; mov r0,#7` the argument still in `r1`
  reaches the RETURN, and the pair rule would join it. When the trial is
  another call's, the system call's read does not count against it, so an
  argument kept in `r1` across the `svc` stays the next call's second argument.
  The cost is a function that really returns, as the high half of a 64-bit
  value, a register it also handed to the kernel: that half is dropped.

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
  phi's other arm may carry a different frame. Frame-pointer aliases are resolved
  backwards through at most 64 same-width copies and constant additions or
  subtractions to a particular SP value. Both walks wrap offsets at pointer width,
  so a 32-bit subtraction and an addition of its two's-complement displacement
  identify the same slot. Unknown bases, width changes, and phi merges are declined.

  This is a deliberate **divergence from upstream**, not a port repair:
  `only_op_use` is faithful to `funcdata_varnode.cc:1891`, and relaxing its
  STORE arm admits non-arguments. The failure mode is a *spurious trailing
  argument*, which no gate observes — the datatest corpus is prototype-declared
  and GED scores topology, not arity — which is why the option ships off by
  default and why `reload` is the recommended level over `spill`: on a clang
  `-O2` inlined 64-byte `memcpy`, the four `movaps` stores that fill the local
  buffer are never read back, so `reload` declines them while `spill` turns them
  into four invented leading arguments.
  The descendant walk follows the value through arbitrary arithmetic, which is
  right for anything that can carry it and wrong for the one shape that cannot:
  a self-cancelling operation. `xor esi,esi` is how x86 clears a register, and
  after heritage it is `INT_XOR(SUBPIECE(rsi,0), SUBPIECE(rsi,0))` — a live
  descendant of whatever `rsi` last held. Inside a loop that puts a *fake*
  competing use on every call: the value passed at the bottom of the body
  reaches the next iteration's clear through the killed-by-call INDIRECT and the
  loop-head MULTIEQUAL, and from there the call that follows the clear, whose
  own trial for that register is active. `check_call_double_use` then rejects,
  both of the bottom call's arguments go inactive, and it renders `f()` while
  the two instructions before it plainly load its argument registers.

  (kuna) `zeroidiomuse` (default-on,
  `decompiler/crates/kuna-decomp/src/p4_calls/kuna_zeroidiomuse.rs`) skips such
  an op entirely — neither a rejection nor a step the walk continues through —
  when it is an `INT_XOR` or `INT_SUB` whose two operands are the same value.
  `INT_XOR(v,v)` is `0` whatever `v` is, so nothing downstream of it can observe
  the value being scored and the use it would veto on does not exist. Sameness
  has to be judged structurally rather than by Varnode identity: the two
  operands are still two distinct `SUBPIECE` Varnodes at this point in the
  schedule, because the common-subexpression elimination that would merge them
  (and let the constant fold fire) runs after `ActionActiveParam`. The test
  accepts identical Varnodes, equal constants, or the same pure reshaping op —
  `COPY`, `SUBPIECE`, `PIECE`, `INT_ZEXT`, `INT_SEXT` — over operands that are
  themselves the same value, to a depth of two. `INT_AND` and `INT_OR` are
  deliberately not in the set: `v & v` is `v`, so the value does survive them.
  The rule is one-directional, promoting no trial by itself and admitting no
  storage the upstream walk would not have admitted; `off` restores the
  upstream walk, in which the zeroing idiom is followed like any other
  arithmetic.

  The LOAD/STORE rejection asks the right question of the wrong set of ops. It
  is asking whether the value is used for something other than this call *on the
  execution that reaches this call*, and the walk has no notion of execution at
  all: it counts every descendant anywhere in the function. A container `append`
  compiled with the one-element fast path inlined and the grow path left as a
  call sets both of the call's arguments up **before** the capacity test,
  because both arms need them, and the fast path then dereferences exactly those
  Varnodes — `mov rdx,[rbx+8]; lea r8,[rsi+rdi]; cmp rdx,[rbx+0x10]; jz slow`,
  then `movzx eax,[r8]; mov [rdx],al` on one arm and `call append_slow` on the
  other. Both trials sink, and `append_slow(container, end, src)` renders
  `append_slow(container)` — the callee's own recovery having meanwhile settled
  on three parameters.

  (kuna) `exclusivearguse` (default-on,
  `decompiler/crates/kuna-decomp/src/p4_calls/kuna_exclusivearguse.rs`) skips a
  `CPUI_LOAD`/`CPUI_STORE` descendant when the matched op is a `CALL`/`CALLIND`,
  the walked Varnode is the **address** operand of the access rather than the
  stored datum, the walked Varnode is **defined in a block that both the
  access's block and the call's block are immediate successors of**, and the
  access and the call sit in distinct basic blocks with neither reachable from
  the other. The reachability test is the whole soundness
  of the rule: an execution path is one walk from the entry block, so a path
  holding both blocks would make the later one reachable from the earlier, and
  mutual unreachability therefore means no path holds both. It is conservative
  around loops for the same reason — two arms of an `if` inside a loop body do
  reach each other through the back edge, so the rule declines them.

  The definition test is what keeps the rule from inventing arguments, and it was
  written against a measured false positive rather than out of caution. Without
  it a register the compiler picked as a long-lived scratch copy qualifies:
  `phantomgate.exe`'s `random_device` constructor opens `mov r8,rcx` and then
  dereferences `r8` on the arms that succeed, so every `throw` call on an arm
  that fails gained a third argument the disassembly does not pass. The witness
  shape is the opposite of that — the pointers are computed in the two
  instructions before the test — and requiring the definition to sit in the very
  block that branches says exactly that: the value was set up *for* this branch,
  not merely still live when it was taken. A function input, which has no
  defining op at all, is declined for the same reason.

  The remaining restrictions keep it disjoint from its neighbours rather than
  merely narrow. The address-operand test hands the `STORE` value slot to
  `spillargtrial` and takes only the pointer slot, so the two options never
  decide the same descendant. The `CALL` match restricts it to caller-side input
  trials, leaving the function's own output trial to `noreturnretuse`. And
  `BRANCH`/`CBRANCH`/`BRANCHIND` keep the upstream rejection outright: the
  `calleearity` family below was built for a `CBRANCH` use in a block that
  *dominates* the call (`test rcx,rcx; jz; call`), and a dominating block always
  co-executes with the call, so relaxing that shape is the fabrication the
  family's own design notes warn against. `off` restores the upstream
  rejection, in which any LOAD or STORE of the value sinks the trial.
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
  write cannot hide a later read. ARM's conditional execution branches to the
  next instruction instead, so the walk credits such a write (`vmovgt s0,s1`, a
  Thumb IT block) on both paths. With `armfloatargs` on, a VFP range that
  some path writes that way before any instruction that always runs is not
  proven dead; an integer register keeps
  the walk's answer, so `int pick(int a, int b, int c) { if (c > 3) a = b;
  return a; }` (`movgt r0,r1`) still loses its first argument at a caller that
  holds an earlier call's result there. With `armfloatargs` on, the paths that
  skip a conditional return are also walked, separately, for that option's leaf
  rule alone; no answer described here reads them. A proven-dead register trial
  is scored `no-use` like any other definitely-unused trial.

  The same walk records a second, narrower fact for `argclobber` to read: for
  each terminator that leaves the callee, what was written on the way to it, and
  where it leaves for — a target the walk could not **name** (an indirect call or
  tail call, a `CALLOTHER`, an indexed register-file access, an undecodable
  instruction), or a named one, kept with its entry address. Naming a target is
  not accounting for what is forwarded there, so `argclobber` resolves each named
  one in turn (`resolve_forward_transfer`): a declared, non-variadic prototype
  answers for it, and so does its own body, walked the same way. A register
  written before every terminator that cannot be answered for cannot carry the
  caller's value out of the callee, and that is the one thing a recovered
  parameter list cannot say about a callee that forwards registers it never
  names.

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
- **Hidden-return register** (kuna, `decompiler/crates/kuna-decomp/src/p4_calls/kuna_hiddenretarg.rs`):
  every AArch64 cspec (Linux, Apple, Windows) lists `x8`, the indirect-result
  register, as the `hiddenret` input entry, and trial scoring treats it like
  any argument register. A value the caller wrote to `x8` and did not read
  again is active, and because the entry is its own resource section it sorts
  ahead of `x0` and prints as the call's first argument. `x8` is also an
  ordinary scratch register, so `mov x8,#0x38; mov x0,#3; bl foo` printed
  `foo(0x38,3)` for a one-argument `foo`, and every glibc syscall wrapper
  passed its syscall number to the helper it calls after the `svc`.

  `hiddenretarg` (default-on) rescores that trial no-use once scoring has
  made it active. A trial scoring left inactive keeps that answer, because
  freeing its dataflow as well would only reshape the caller's variables: a
  zlib loop flag kept in `w8` across a `memset` call changed its storage
  comment that way.

  The callee's own body decides first, read through the entry walk
  `calleedeadarg` caches. The callee never *takes* `x8` when no path reads it
  before writing it and no path leaves for code the walk did not read (a call,
  a `CALLOTHER` such as `svc`, an indirect branch, an undecodable instruction)
  with it still unwritten, and then the trial is vetoed. Unlike
  `calleedeadarg`, a `RETURN` needs no write first. That rule exists so that a
  stub's ignored source-level argument survives, and `x8` is no source-level
  argument. A callee whose walk shows it reading `x8` keeps the argument
  whatever the value: a struct return storing through it (`mov x8,sp; bl mk`),
  a tail call into one, or a hand-written helper that loops on it. So does a
  callee whose walk reaches a system call with `x8` still holding the caller's
  value (`helper: svc #0; ret`, or dietlibc's `__unified_syscall`), because
  the kernel reads it as the syscall number. The walk records those system
  calls by the user op the language's `svc` lowers to, the one `syscallregs`
  names.

  Where the body does not settle it, two facts about the value do. First, a
  value already vetoed from another call is vetoed here too. clang -O0
  computes a comparison into `w8` ahead of two calls on different paths
  (`if (pg < 1) return corrupt(4321); if (pagecount(bt) == 0) ...`). The value
  reaches both calls, so the double-use rule in trial scoring keeps it off
  each. Once the veto frees it at `pagecount`, `corrupt` becomes its only
  reader and would print `corrupt(0xffffffff,0x10e1)`. The vetoed value is
  recorded on the function as its leaves, through copies, zero extensions,
  phis and non-creation `INDIRECT`s, and a later trial whose value shares a
  leaf is vetoed as well. A leaf that is an address in the caller's frame is
  not recorded: a hoisted `add x8,sp,#16` can be a leftover at one call and the
  real result buffer at a later one it reaches through a phi. The record is
  cleared with the Varnodes on a restart. Second, a value whose every leaf is a constant below `0x1000` is in
  a page no operating system maps for a program, so it cannot be a result
  buffer. That answers for callees the walk cannot cover, such as an import or
  a glibc helper that reaches an `ldaxr`/`stxr` pair before it writes `x8`. It
  is also what keeps glibc `getcwd`'s syscall number off `__assert_fail`, whose
  body the walk cannot cover. A larger constant, such as the address of a
  global that a C++ static initializer constructs in place with
  `adrp x8,g; add x8,x8,:lo12:g; b make`, is left to the body evidence. The
  null-page rule cannot see a callee that takes a small number in `x8` by its
  own convention behind an import, such as the OCaml runtime's `caml_allocN`
  reached through a PLT stub. That number is dropped (`caml_allocN()`).

  Dropping the entry leaves no hole in `x0..x7`. Only a register entry is
  vetoed: not SPARC's `[sp+64]` stack slot, and not one that shares storage
  with an ordinary input entry, as tricore's `a4` does. `off` restores the
  upstream scoring.
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

That collection walks *backwards* from the CALL and stops at the first op that
is not a `CPUI_INDIRECT`, so it is exact only while the guard INDIRECTs form one
unbroken run immediately before the CALL. Upstream keeps that run whole from both
sides: `op_insert_before` skips back over it, and `op_insert_after`
(`decompiler/crates/kuna-decomp/src/substrate/funcdata_op.rs (Funcdata::op_insert_after)`)
redirects — asked to insert after an INDIRECT marker it decodes the iop annotation
in the marker's second input and inserts after the CALL (or STORE) that marker
speaks for. kuna carried that redirect as a stub, so an op inserted after a guard
INDIRECT landed between the guards and their CALL, the backward walk stopped
there, every output trial was marked no-use, and the call lost its return value
while its INDIRECT creation stayed behind as a local the emitted C reads and never
assigns. The (kuna) `indirectanchor` gate
(`decompiler/crates/kuna-decomp/src/p3_dataflow/kuna_indirectanchor.rs (anchor_of)`)
completes the redirect; with it off the op is inserted after the INDIRECT itself.
The shape that reaches it is `RulePullsubIndirect` pulling a SUBPIECE through the
guard INDIRECT of a frame slot whose upper half is read after the call. A `prev`
that is not an INDIRECT marker, an INDIRECT whose second input is not an iop
annotation, and an iop that decodes to a dead op are all left where they were
going.

The function's own return runs in mainloop
(`coreaction_protos.rs (ActionReturnRecovery)`): every live RETURN op's trial
Varnodes go through the same realism + sole-use tests, the container freezes
on the §4.1 budget, the output map is derived, and
`ActionReturnRecovery::build_return_output` rewrites each RETURN: zero or one
used trial passes through; **two pieces** are concatenated with a PIECE whose
output sits at the constructed join address (falling back to the first piece
if no join can be built); more pieces chain PIECEs over contiguous trials.
The (kuna) `returnpair` gate intercepts this join — §4.4.

A RETURN reached only along branch edges whose conditions literals already
decide the other way is not scored
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_bejoin.rs (never_reached)`).
SPARC's `call` pcode keeps one for a `restore` in its delay slot
(`didrestore = 0; call; if (didrestore == 0) goto next; return`), and return
recovery runs before the rule pool folds it away. What it reads in `%o1` is the
call's clobber, so scoring it would fail the `%o1` trial of a function that
returns a `long long`. It is still rewritten with the other RETURNs.

Which of the two pieces is the high half is the ABI's answer, not the trial
order's. The trials sort in storage order, first register first, and the output
rule that matched them records whether it consumes the most significant piece
first. A `<join>` rule does by default on every big-endian target, because a
register pair holds a wide value the way a load from memory would put it there,
lower address (the high word) in the first register. PowerPC returns a `long
long` in `r3:r4` with the high word in `r3`, MIPS o32 in `$2:$3`, SPARC in
`%o0:%o1`, ARM big-endian in `r0:r1`, and AArch64 big-endian an `__int128` in
`x0:x1`, each with the high half first. The flag is the rule's, not the target's
endianness: `reversesignif` flips it, and AVR's gcc spec, little-endian, lists
`R25` first and joins with `reversesignif="true"`, so its `int` comes back with
the high byte in `R25`, the first register, too
(`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs (ParamActive::join_pair_order)`).
Upstream joins the first register low everywhere, so on those ABIs a genuine
`long long` printed its halves swapped (`wide_mul` as `CONCAT44(low,high)`). A
little-endian rule without `reversesignif` never sets the flag, so its joins are
exactly upstream's. A cspec whose single join entry names the pieces itself (the
68000's `D0:D1`) never reaches the flag: its trials sort by their justified
offset, which already puts the low half first. Two-register *parameters* are
never joined at all (`ActionParamDouble` below is a no-op), so a `long long`
argument prints as its two registers in the ABI's order and no order assumption
applies there.

On those ABIs, with the (kuna) `bejoin` option on, the pair joins in the ABI's
order only when its second register holds a low word the function returns on
purpose
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_bejoin.rs (join_order)`).
The option is off by default (below): off, every pair joins first register
low, as upstream does, so default output on those ABIs is upstream's.
Ancestor realism makes a pair of any function whose second register reaches the
RETURN holding something, and a function returning one register often leaves
something there: SPARC's `restore` copies `%i1` back into `%o1` on every function
that opened a window, so `%o1` arrives holding the second argument, a scratch
value, or a byte the function stored; MIPS clang -O0 materializes a zero in `$3`
that nothing reads. Joined first register low, such a pair narrows back to the
first register wherever a later pass drops the phantom half (the
uncomputed-half repair below, a boolean or byte return, C's own truncation in a
narrow prototype), which is the right value. Joined in the ABI's order, every one
of those keeps the wrong register. So once the output map is derived, return
recovery walks the second register back at every live RETURN, through copies,
phis, indirects and the pieces heritage splits a register pair into (following
the byte offset of the low word), to where its value was made
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_bejoin.rs (classify)`):

* both registers the halves of one wider value (an 8-byte load, a product split
  by `mflo`/`mfhi`), which heritage did not merge from two: a wide value,
  whatever else the function does with it at that RETURN;
* the register's own entry value reached without an instruction that moves it,
  untouched or carried only by a register window's copies (a copy at an
  instruction that copies every general-purpose argument register, `save` and
  `restore`; `moves_register_window`): a leftover;
* a value also used for anything but the pair's RETURN slots (a store, a call's
  settled argument, a branch, an address, the stack pointer), one that reaches
  the first register
  other than through its sign (`sra 31`, the high half of its extension), a
  comparison (a carry or borrow) or as the other half of a wider value, or the
  first register's own value: scratch;
* what a comparison left behind: the first register holds a computed 0 or 1 (a
  comparison, a carry or borrow, a sign bit, a leading-zero count shifted down
  to its top bit, or 0s and 1s chosen between, by its possibly-nonzero bits), or
  a literal 0 or 1 at a function whose live RETURNs hold both (a boolean a
  branch chose), or the low word is a sum or difference (not a move spelled as
  adding zero, like SPARC's `restore %i0,%g0,%o1`) and the first register is
  more than one literal while no comparison, carry or borrow and none of the low
  word's own values reaches it. A 64-bit comparison computes the low words'
  difference or sum for its flags: ARM big-endian `return x >= 0x80000000ULL`
  leaves `0x7fffffff - lo` in `r1` beside the flag in `r0`, and a saturating
  `int` its clamped value in `r0` beside the same difference. SQLite's `v ==
  (int)v` on SPARC -O2 is `addcc %i1,%i2,%i1; addxcc %i0,0,%i0; cmp %i0,0; be`
  and then `restore %g0,1,%o0` on one path and `restore %g0,%g0,%o0` on the
  other, so `%o1` keeps `lo + 0x80000000`; PowerPC computes the same flag with
  `li 3,1` and `li 3,0` on two paths, or as `cntlzw 3,3; srwi 3,3,5`. The
  same check returns any literal, or a flag negated, shifted or offset, just as
  well: `if (v < INT_MIN || v > INT_MAX) return ERANGE; return 0;` on SPARC -O2
  is `addcc %i1,%i2,%i1; addxcc %i0,-1,%i0; cmp %i0,-1; bne` and then `restore
  %g0,34,%o0` or `restore %g0,%g0,%o0`, and PowerPC -O1 turns `v == (int)v ?
  -1 : 0` into `addc 4,4,5; addze 3,3; cntlzw 3,3; srwi 3,3,5; neg 3,3`. So a
  low word that is a sum or difference (`0 - x` counts; `x + 0` and `x - 0` are
  moves) is also a comparison's leftover when its carry or borrow -- a
  comparison of nothing but its operands and itself, through copies and
  truncations (`addcc`'s carry of `op1:4` and `op2:4`, MIPS's `sltu` of the sum
  against an operand) -- reaches a conditional branch or the first register,
  and the first register is worked out from comparisons and literals alone: a
  literal, a value with at most one bit that can be set, or such values
  negated, inverted, offset, shifted, masked, added or chosen between. A
  choice's input that the branch on its edge has just tested equal to a
  literal counts as that literal (ARM big-endian `adds r1,r1,#0x80000000; adcs
  r0,r0,#0; movne r0,#22` returns the `adcs` result only when it is zero). The
  sum's own carry is never one of those flags: a real 64-bit sum adds it into
  its high word (`(u64)n + ((n + 7) >> 3) + 5` is the sum of two carries
  there), while the range check only tests it. The halves of a real 64-bit sum
  or difference are tied by its carry, and a `long long` whose high word is
  only a carry (`(u64)a + b`, the same code as the `int` carry test) reads as
  the comparison
  (`decompiler/crates/kuna-decomp/src/p4_calls/kuna_bejoin.rs (computed_flag,
  carry_free, carries_of, tests_carry, of_flags)`);
* an `int` truncation of a stack temporary: both registers loaded from
  adjacent words of one stack object (the entry stack pointer plus literals,
  through a frame pointer), the first from its less significant word. clang
  -O0 spills a 64-bit value to two stack words and reloads both for `(int)(c ?
  a : b)`, `(int)MIN(len, INT_MAX)` or an absolute value truncated to `int`,
  the low word into the first register (`lwz 4,12(31); lwz 3,16(31)` on
  PowerPC); the `long long` reloads the same words the other way (`lwz
  3,12(31); lwz 4,16(31)`). Two words that are each the home a register's own
  argument was stored to and is reloaded from are two values, not one object:
  -O0 `((u64)a << 32) | b` reloads `a` and `b` from their homes, `b` below `a`
  (`decompiler/crates/kuna-decomp/src/p4_calls/kuna_bejoin.rs
  (loads_low_word_first)`). This and the comparison tests apply only where one
  register holds a whole `int`, so AVR's byte pairs never take them;
* a division by a constant: the low word is the low half of a product at least
  two words wide, and the product's high half reaches the first register
  through a right shift by a literal on some path. A compiler divides by most
  constants by multiplying by a magic number and shifting the product's high
  half down, and ARM's `umull` and `smull` write the low half too: big-endian
  `a / 10` at -O0 is `umull r1,r0,r2,r3; lsr r0,r0,#3`, `a / 7` on ARMv6 and
  v7 is `umull r1,r2,r0,r1; sub r0,r0,r2; add r0,r2,r0,lsr #1; lsr r0,r0,#2`,
  and the signed `a / 3` is `smull r1,r0,r2,r3; add r0,r0,r0,lsr #31`. A
  remainder or any other value built from such a quotient counts the same. A
  `long long` product returns its high half as it is or with more terms added
  (`(i64)a * 3` adds the high word's own product), never shifted down
  (`decompiler/crates/kuna-decomp/src/p4_calls/kuna_bejoin.rs (divided_down)`).
  A divisor whose magic number needs no shift (`a / 641` and `a / 6700417` are
  a bare `umull r1,r0,r0,r1`) leaves exactly a product's registers and is not
  told apart; it is one of the shapes listed below that read as the `long long`;
* the high half of a 64-bit temporary held the reverse way round: the low word
  is a right shift by a literal of a value whose dropped bits reach the first
  register. ARM big-endian computes an `int` that is a 64-bit sum, difference
  or product shifted right by one in the reverse register pair, low half in
  `r0`: `(u32)(((u64)a + b) >> 1)` is `adds r0,r1,r0; adc r1,r2,#0; lsrs
  r1,r1,#1; rrx r0,r0`, `((u64)a * b) >> 1` is `umull r0,r1,r1,r0; lsrs
  r1,r1,#1; rrx r0,r0`, and clang -O0 shifts a 64-bit argument the same way
  (`movs r1,r1,lsr #1; mov r0,r0,rrx`). Bit 0 of `r1` moves into `r0` and the
  stale `r1 >> 1` stays behind. A `long long` shifted right moves the bit the
  other way, from the first register into the second, so its low word is never
  the bare shift. The first register reads a dropped bit when it reads the
  shifted value (or a copy or the same truncation of it) other than shifted
  right by at least as much; the walk back from it follows a stack reload to
  every value the function stores at that offset (-O0 keeps the result in a
  local). A shift that leaves only the top bit or the sign (`x >> 31`) is a
  value of its own: a saturated `long long` fills its low word with it beside
  the high word it came from
  (`decompiler/crates/kuna-decomp/src/p4_calls/kuna_bejoin.rs (shifts_into_first)`);
* an `int` worked out from the high half of a 64-bit temporary, its low half
  left in the second register. When the low word is a sum or difference
  (through moves; MIPS's `move` is `or $3,$5,$zero`) whose carry or borrow
  reaches the first register, the first register must be that sum's high word
  itself: additions and subtractions of values no flag reaches and of no
  literal but zero (where the low word adds a literal too, the 64-bit
  literal's high half, as long as it is not added on top of a word that
  already holds the carry: `li 5,7; addic 4,4,5; adde 3,3,5` is `x +
  0x700000005`, `addic 4,4,5; addze 3,3; addi 3,3,7` is `(u32)((x + 5) >>
  32) + 7`, and a loop's accumulator, which holds an earlier turn's carry,
  may add one: coreutils dd's `records--`), each flag entering with the sign
  the low word's
  operation gives it, a carry added and a borrow subtracted (MIPS tests a
  difference's borrow as `a < a - b`, and a negation's may be spelled `b !=
  0`), read through copies, zero extensions, truncations (SPARC's `smul`
  truncates a 64-bit product), negated flags and the choices of a loop's
  accumulator. A shift, mask, product, literal or flag of the wrong sign on
  the way is a rework; a flag that reaches the first register only through
  a piece of a wider value counts for nothing. A flag is a comparison of nothing but the sum's operands and
  itself, read through copies and the pieces of a register pair heritage splits
  and joins (SPARC -O0 reloads a `long long` with `ldd` and adds its halves out
  of `%i2:%i3`). `(u32)((x + y) >> 48)` on PowerPC is `addc 4,6,4; adde 3,5,3;
  srwi 3,3,16` and leaves the low sum in `r4`; `-(int)((x + y) >> 32)` negates
  `adde`'s result, while a 64-bit `-x` (`subfic 4,4,0; subfze 3,3`, ARM's
  `rsbs; rsc`) negates a sum whose flag is a borrow and so keeps the sign
  right. With no carry between the two registers, the first register may not
  be a literal operation (a shift short of a sign fill, a mask, an offset, a
  negation, a product, an `or` or `xor` with a literal) on a value the low word
  never reads, nor on anything that value is worked out from by literal
  operations, rotates and stack reloads (PowerPC's `srwi` is `rlwinm`, a rotate
  then a mask; the first register's own value is followed into a stack slot
  only when the slot is stored once, not into an accumulator's earlier
  values): gcc -O0 on MIPS computes `(u32)((x ^ y) >> 32) & 0xffff` in
  `$2:$3` and masks `$2` alone, and ARM's `umull r1,r0` leaves the low half of
  `(u32)((x * k) >> 32) & 0xff` in `r1`, while `((u64)(v & 0xff) << 32) |
  (u32)(v * 7)` reads `v` in both words, and a complement or a mask, `or`
  or `xor` with a literal the low word gets too is one 64-bit operation
  (`~v`). Moves include PowerPC's `mr 3,5` (`or 3,5,5`). A first register that is the clean
  high word of a sum or difference the low word holds through moves, choices
  or stack reloads is not judged this way either (-O0 `i64 tabs64(i64 a)`
  returns from two locals, storing `a` and `-a`), nor is a literal low word
  (`((u64)(a ^ K) << 32) | K`). The two registers are followed through
  moves and through choices made in one block together, input by input, so a
  loop's 64-bit accumulator pairs its sum with the sum's own high word. When
  the low word is a choice the first register's value does not follow
  (choices in different blocks, an expression evaluator's operator cases),
  each sum the choice can return is judged against the first register, and a
  sum whose own high word it is counts as a 64-bit value
  (`decompiler/crates/kuna-decomp/src/p4_calls/kuna_bejoin.rs (reworked,
  carried_into, works_over)`);
* the high half of a product beside a first register that reads the product's
  low half, the reverse of a `long long`'s layout: clang -O0 on MIPS spills the
  `mfhi` of a 64-bit hash, reloads it into `$3`, and builds `$2`, `(u32)x`, from
  the `mflo`. The walk back follows stack reloads, and a stack address may come
  through a frame pointer set by `mr 31,1` or MIPS's `move $fp,$sp`, or
  SPARC's `%fp`, which a `save` that spills the window reaches through a
  chain of indirect effects and choices of the same address
  (`decompiler/crates/kuna-decomp/src/p4_calls/kuna_bejoin.rs (product_high_half,
  reads_low_half)`);
* a literal zero, which clang -O0 leaves behind and `(u64)x << 32` returns alike:
  proof of nothing;
* a callee's clobber or a location never written: nothing;
* anything else, a value or a nonzero literal nothing but the RETURNs read:
  returned on purpose.

A RETURN reached only along branch edges literals decide the other way is
skipped: SPARC's `call` pcode keeps one for a `restore` in its delay slot
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_bejoin.rs (never_reached)`).
A function with an indirect jump flow could not follow (`Treating indirect jump
as call`: a CALLIND and the artificial RETURN after it) has code return recovery
does not see, and a user-defined operation given no register inputs (an inline
system call kuna does not model: PowerPC's `sc` lifts to `syscall()`) may read
any register. A low word nothing visible reads is not returned on purpose there
when such a reader can read it: when one of the values the walk back reaches is
an input, a literal, or made in a block that dominates the reader. It then
vetoes the join like scratch; a value made after the reader, on the path that
returns it, still counts
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_bejoin.rs (hidden_readers,
reaches_jump)`). With `syscallregs` off, PowerPC `pair`'s argument registers
set up for `sc` come back as a pair joined first register low, as on main. gcc copies an argument into `$3` before a switch for the case
that passes it on (`move v1,a1`), and when the jump table is left to
relocations, as in an object file, that case is not part of the function:
iproute2's `rt_addr_n2a_r` on MIPS -O2 returns `"???"` in `$2` on the default
path with the copy beside it, the registers a `u64` function returning its
argument zero-extended leaves on purpose, and it printed `return a3;`.
The pair joins in the ABI's order when some RETURN holds a wide value or returns
its low word on purpose, and none holds a leftover, scratch, a comparison's
leftover, a truncation, a quotient's, a shift's, a reworked temporary's or a
product's leftover there. Otherwise it joins first register low, exactly as
upstream, and the
function's calls then join their own output pairs that way too
(`Funcdata::kuna_pairs_first_low`), so a pair a call hands back and the function
returns stays one value. An argument the function moves into the low word
itself is a value, not a leftover (ARM big-endian's `mov r4,r1; bl ext; mov
r1,r4` carries the second argument across a call into it), and so is the
function's own input read by a call whose inputs are still undecided trials: a
callee without a known prototype may not take it at all.

The rule is a prior, not a proof, which is why it is an option, and off by
default. The same registers can hold a `long long` or an `int`, and with the
option on each shape is read one way:

* read as the `int`, as upstream does, and still wrong for the `long long`:
  `(u64)x << 32` beside an `int` with a dead zero in the second register;
  `((u64)(x ^ K) << 32) | K`, whose `K` also builds the first register, beside
  `x ^ K`; a `long long` whose low word also feeds a call beside an `int` keeping
  scratch there; `(u64)a + b`, whose high word is only the carry, beside the
  `int` carry test; a `long long` whose high word is a literal 0 or 1 a branch
  chose, beside the boolean; a `long long` whose high word is a literal or a
  flag beside a low-word sum whose carry a branch tests (`u32 s = a + b; return
  s < a ? 0 : s;` returning `u64`), beside the `int` range check that leaves
  the same registers; -O0 `(v << 32) | (v >> 32)` of a 64-bit local,
  which reloads the local's two words in the same order as an `int` truncation
  of it; a `long long` whose low word is a right shift of a value whose
  dropped bits also build its high word, beside the reverse-pair `int`; a
  `long long` whose high word is a literal operation on a value its low word
  does not read (`*p | 0xffULL << 32`), or a reworked high word of the sum in
  its low word (`((s >> 33) << 32) | (u32)s`), beside the `int` worked out from
  the high half of a temporary; and a
  `long long` returned in a function with an indirect jump flow could not
  follow, whose low word nothing visible reads;
* read as the `long long`, and wrong for the `int`: an `int` holding the high
  word of a 64-bit temporary itself, whose stale low half stays in the second
  register, in the same registers the `long long` uses (one that goes on to
  shift, mask, offset, negate or multiply that word reads as the `int`,
  above, and so does gcc -O0 on MIPS, which moves the high word down with
  `srl $17,$2,0` for `>> 32`). `(int)((a + b) >> 32)` on PowerPC -O2 is
  `addc 4,4,6; addze 3,3; add 3,3,5`, the code for `a + b`, and
  `((u64)a * b) >> 32` leaves both halves of the product, on ARM big-endian at
  every optimization level (`umull r1,r0` writes both registers) and on MIPS
  gcc at -O0 (`multu` with `mflo`/`mfhi`), as does an unsigned division by a
  constant whose magic number needs no shift (`a / 641` is `umull r1,r0,r0,r1;
  bx lr` and prints as `a * 0x663d81`), `(u32)((x + K) >> 32) + M` on ARM
  big-endian, which folds the literal into the carry add (`adds r1,r1,#K; adc
  r0,r0,#M` is the code for `x + (M << 32 | K)`), and MIPS gcc -O0
  `(u32)(*p >> 32)` loads both words of `*p` into `$2:$3`, as `return *p`
  does. These print right by default and print the whole 64-bit value with
  the option on.

The default is off (GH-904). The second kind is rare in real code and the
first is common, but the rule decides from the instructions alone, and the
`int` that works over the high half of a 64-bit temporary is a family, not
one shape: each new look at it found another arithmetic form that compiles to
the `long long`'s registers. A default must not turn a correct `int` into a
wrong `long long`, so default output keeps upstream's join on every target, and
the option is for a reader who has checked a big-endian or AVR function's
return against its disassembly. A sound default needs positive evidence that
the value is 64 bits wide: a declared, DWARF or asserted 8-byte return type,
callers that read both registers after the call as one value, or a 64-bit
operation whose two halves both reach every RETURN without being narrowed.
With the option on, over 24,257 functions of nine corpora (e2fsprogs, coreutils and
gnulib, libexpat, libbsd, dpkg, diffutils, findutils, sysvinit, Lua,
libselinux, iproute2, rsyslog, openssh, bash, cronie, libedit, grep, kmod,
shadow, zlib, dash, gnutls, base-passwd and gzip) built with clang for
PowerPC, SPARC and ARM big-endian (its default architecture, ARMv6 and ARMv7)
and with gcc for MIPS big-endian at -O0 to -O2 and -Os, 1,707 change, and
every 64-bit return among them that returns both words comes out in the right
order (`docs/features/bejoin/record.json`). No function there whose DWARF
return type is `int` or a pointer changes value, but that is a measurement on
these corpora, not something the rule proves: an earlier form of it changed
one, iproute2's `rt_addr_n2a_r` (the unrecovered switch above), which now
prints as on main. A SPARC `int` function whose result folds
into the pair before the uncomputed-half repair can drop the leftover (`return
0` with the second argument still in `%i1`) prints the argument shifted into the
high word, as it did before.

When the two registers are contiguous in the joined order, the whole is built at
their parent register rather than in the join space (`constructJoinAddress`).
AVR maps its register file into data memory, so that parent, `R25R24`, is a
global: a value built there merged with the argument bytes it is made of, the
PIECE became internal to one variable, and `negate` printed `return R25R24;`, a
read of a register the printed C never assigns. With `bejoin` on, a pair whose parent
register is global storage therefore still gets a join record over its two
registers, which keeps it a value (`decompiler/crates/kuna-decomp/src/p4_calls/kuna_rustabi.rs
(pair_join_address)`); the call-output pair builds its whole the same way.
Register-space parents (`r3:r4` is not a named register, `EDX:EAX` is not
contiguous) are unaffected.

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

A second exception, (kuna) `condexeretuse`, covers uses that sit on a branch
no execution carrying the scored value can take. It is described with
`condexeret` in §4.4, because both come from the same merge blocks.

The output container normally gets a single pass (its budget is 0 unless the
model has a delayed heritage space), and that pass runs before
`ActionConditionalExe` in the same mainloop iteration. The (kuna) `condexeret`
gate adds at most one more pass for trials that failed only on a path that
pass can remove — §4.4.

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
chapter 01 §1.7, the lift-time behavior chapter 02 §2.4). Because that override
already gives the call spec a model, the later exact-slot query in
`ActionDefaultParams` declines, preserving the per-call-site priority for both
direct and indirect calls.

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

The mirror failure is an output register missing from the killed set. A model
whose `<output>` carries no `<rule>` kills its output registers on its own
(`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs
(ParamListStandard::initialize)`); one with rules relies on its `<killedbycall>`
list. The vendored SPARC specs have output rules (the `%o0:%o1` and float joins)
and listed no killed registers, so `%o0` crossed every call as an unknown
effect: no output trial was seeded, every call's result was dropped, and the
caller read the argument it had passed in its place. `SparcV9_32.cspec` now
kills `o0`, `o1` and `fd0` and `SparcV9_64.cspec` kills `o0`, `fd0` and `fd2`,
the registers each output list returns in. The other `%o` and float registers
are scratch too, but nothing reads them after a call expecting a value, and
they stay unknown effects.

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

That final rewrite only runs where the call spec recovered its own arguments. A
call whose callee carries a **declared** prototype is input-locked before
`ActionFuncLink` ever registers a trial (§4.2), so no later pass rewrites its
input list, and a placeholder that was never resolved would stay on the op
forever — rendering as a trailing argument past the declared arity, reading the
very stack slot the `call` pushed its return address into, and contradicting the
prototype the same output declares. The unconditional strip is therefore the
stack space's own heritage step: once
`decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs (Heritage::heritage)`
reaches a spacebase space it flagged `has_call_placeholders`, it calls
`abort_spacebase_relative` on **every** call spec in the function before
heritaging that space, whether or not the site resolved. The placeholder has
done its work by then — the offset, if it was recoverable, was recorded on the
success path above — and the LOAD it hung off is destroyed with it when nothing
else reads it.

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
commits the new input/output lists directly. For a locked variadic prototype,
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_varargtransfer.rs
(pending_trials)` preserves existing trials outside the fixed parameters before
`commit_new_inputs` rebuilds the fixed head. Their existing Varnodes, widths and
scoring flags survive, with slots remapped and model-entry associations cleared
for the new prototype. Unreferenced trials and the old stack placeholder are
excluded. This preserves an already active variadic argument and leaves an
inactive stored integer available for later format evidence; resolving an
imported target must not erase its unfinalized variadic trials. Ordinary trial
scoring still controls which preserved values become arguments. This also keeps
an already recovered promoted double copied from an XMM register to its variadic
GP slot; it needs no integer-format fallback and survives with `varargforward`
off as well. The format evidence parser remains restricted to promoted integers.
Failure sets the restart-pending
flag — the P4 → Band B feedback edge of 00 §0.7, bounded and executed by the
drive — (kuna) recording `ProtoDeindirect` in the restart log
(`decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_restartlog.rs
(RestartLog)`). The reasoning: by the time the target resolves, heritage has
already committed guards and trials under the wrong prototype; edits cannot be
made backwards, but the Override survives `Funcdata::clear`, so the re-run
lifts the truth.

`ActionDeindirect` has a second arm for a CALLIND whose target does not
resolve to a function but whose *type* says what it calls. Once type recovery
has started, if the target Varnode's read-facing type is a pointer to a code
type that carries a prototype — a function-pointer struct field, local or
parameter declared through the C grammar — and the call site's inputs are not
yet locked, the arm forces that prototype onto the site with
`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs (FuncCallSpecs::force_set)`.
The lock test is what makes the arm fire at most once per site, and it is also
why an existing lock wins: a user `override prototype` at the call address, or
a prototype an earlier pass already forced, is never replaced. A target with no
declared prototype (an untyped `void *` field, or a code type without one)
leaves the site on its model-recovered argument list.

A prototype whose storage puts a parameter or the return value in one of its
model's floating-point entries is forced only where the image states a
convention that keeps such values, doubles included, in floating-point
registers
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_typedcallabi.rs`). A
declarator that names no convention gets the default model, which passes
floating-point values in floating-point registers where it can. A soft-float
image passes them in integer registers, and a single-float one passes doubles
in integer registers, so forcing the default storage there would print
uninitialized floating-point register locals as the arguments and drop the ones
the caller set. A floating-point value the default model already places in
integer registers or on the stack (a MIPS `double` after an `int`, a variadic
argument) has the same storage under every convention, so such a prototype is
forced as declared everywhere. The loader reads what the container states
(`decompiler/crates/kuna-analysis/src/loader/kuna_floatabi.rs`): ARM's
`Tag_ABI_VFP_args` (an `aeabi` subsection without it means the core-register
base variant) or a linked EABI5 header's float flag, MIPS `.MIPS.abiflags` and
`Tag_GNU_MIPS_ABI_FP`, PowerPC `Tag_GNU_Power_ABI_FP`, and the RISC-V header's
float-ABI bits; x86 and AArch64 always use floating-point registers. Only a
double-precision hard-float convention counts as hard: MIPS `fp_abi` 1, 5, 6
or 7, PowerPC hard double, RISC-V lp64d/ilp32d, ARM's VFP variant. A hard-float
image gets the prototype as declared. A soft-float image gets it rebuilt from
the same types under the spec's soft-float model (ARM's `__stdcall_softfp`), or
not at all where the spec has none (MIPS, PowerPC, RISC-V), which leaves the
recovered arguments. A single-float image (RISC-V lp64f/ilp32f, MIPS `fp_abi`
2, PowerPC single-precision) and an image that states nothing (a raw image, a
clang PowerPC or attribute-less ARM object, a Mach-O, the Ghidra front-end)
keep the recovered arguments too. The instructions a function executes do not
decide it: ARM `softfp` and single-float code compute with floating-point
registers yet pass doubles, or every value, in integer registers. A
non-variadic prototype that names its own convention is forced as declared on
every image. A variadic prototype is forced only when its return value and
every fixed parameter each sit in one location outside the floating-point
entries; one with a floating-point entry or a join (a register pair such as
`long long` in `r0`/`r1`) is not forced on any image, whatever convention it
names, and the call keeps its recovered arguments. ARM's VFP variant passes
every value of a variadic call, the return value included, in core registers,
and a locked variadic prototype misreads the arguments past such storage: a
floating-point return drops the floating-point arguments recovered past the
fixed ones (x86-64 `xmm0`, AArch64 `d0`), and a register-pair parameter prints
a stale value for one of its words (32-bit ARM, PowerPC, RISC-V and MIPS).
A return value narrower than the register entry that holds it (a `char`,
`short` or `bool`, or an `int` on a 64-bit RISC-V) is forced only where the
caller extends it itself (x86, and AArch64 other than Apple's), the model's
output entry extends it by its type (`extension="inttype"`: ARM, PowerPC), or
`narrowext` states its extension (§4.4): RISC-V and LoongArch by default, MIPS
and Apple arm64 under `narrowext compiler`. An
Apple arm64 callee extends a return value narrower than 32 bits to 32 bits and
its caller reads `w0` as it is, so there only an `int` or wider return counts
as caller-extended. The loader reads the platform from the container
(`decompiler/crates/kuna-analysis/src/loader/kuna_returnext.rs`): a Mach-O
arm64 image is Apple's, and an ELF or PE one follows AAPCS64, whose caller
extends. The `AppleSilicon` language, and an arm64 image whose container says
neither (a raw image, the Ghidra front-end), are treated as Apple's, since the
same bytes are correct there only if the callee extended. Elsewhere (a plain
`char` or an undefined type, which `narrowext` leaves alone, or with it off)
the callee's extension is modelled wrongly and the recovered call is kept: a
MIPS model states no extension, so the rest of `v0` would print as an
unassigned piece of the result, and the RISC-V model and Apple's AArch64 one
state zero extension, which turns a negative `signed char`, `short` or 64-bit
RISC-V `int` result into a large positive one.

**Declared prototypes on a soft-float image.** A declaration that names no
convention — a `--assert prototype` / `map prototype` / `parse line extern`
directive, or a DWARF `DW_TAG_subprogram` — describes a function the image
calls with its own convention, and the default model is ARM's VFP variant. On an
image without floating-point hardware that states the base (soft-float)
convention, every function passes floating-point values in core registers, so
such a declaration is laid out under the soft-float model whenever its default
layout would put a parameter or the return value in a VFP register
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_typedcallabi.rs
(undeclared_model)`). Laid out under the default model, a `double` parameter
would be read from a VFP register the caller never set, and the core registers
that carry it would print as unassigned locals or as stray arguments. The same
choice is made wherever the pieces become storage: the function's own
decompile (`decompile_drive.rs`), a caller's copy in `ActionDefaultParams`, the
prototype-bearing code type the console locks onto the symbol (which flow reads
at a direct call and de-indirection reads at a resolved one), and the
canonical-storage checks `protoorder` and `callbacktype` compare a recovered or
declared list against, so a list is only stated to callers where they will
read it back at the same storage. A prototype without a floating-point or
aggregate type keeps the default model, whose storage for it is the same.

Floating-point hardware comes from the same attributes: `Tag_FP_arch`,
`Tag_Advanced_SIMD_arch` or `Tag_MVE_arch` naming an architecture
(`decompiler/crates/kuna-analysis/src/loader/kuna_armfloatabi.rs
(fp_hardware)`). An image that states an FPU next to the base convention
(`-mfloat-abi=softfp`) keeps the default layout: GCC and clang pass a static
function's floating-point values in VFP registers there when they see every
call, so the image's convention does not decide the function's. An image whose
attributes say nothing about hardware keeps it too. Neither case changes from
before this rule, and an exported function on a softfp image is still laid out
in VFP registers. MIPS, PowerPC and RISC-V specs have no soft-float model, so a
declaration on their soft-float images keeps the default layout as well.

The soft-float model is the spec's `__stdcall_softfp` with the default model's
`extension` copied onto its core-register input entries: the base standard
extends an argument narrower than a word as the VFP variant does, and the
spec's entries leave it unstated, which made a `bool` tested as a whole
register print as `CONCAT31` of an unassigned piece. Typed indirect calls above
use the same model. A declaration that names `__stdcall_softfp` itself gets the
spec's model unchanged.

**Declared variadic prototypes on a VFP image.** AAPCS-VFP applies only to a
function with a fixed parameter list: a variadic function is called under the
base standard, its result included, so `double vr(int n, ...)` returns in
`r0:r1`, a `float` in `r0`, a homogeneous floating-point aggregate of four bytes
in `r0` and a larger one in memory through a hidden pointer in `r0`. The spec's
default model already sends every argument of a variadic prototype to core
registers (its `<varargs/>` input rules), but its output list has no such rule,
so the declared return was read from `d0`: the call printed with no result, and
the caller's `vmov d0,r0,r1` read `r0` and `r1` as if the call had left them
alone. On an ARM image whose attributes state the VFP variant
(`Tag_ABI_VFP_args` = 1), a declaration that names no convention, has a variadic
tail and returns a floating-point value or an aggregate is laid out under the
default model with three output rules in front of its own: a `float` from the
core registers, a structure over four bytes through a hidden pointer, and a
homogeneous floating-point aggregate from the core registers
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_vfpvariadic.rs (model)`). The
derived model is chosen at the same four places as the soft-float one, so the
function's own decompile, its callers and the console's code type agree. A
non-variadic declaration, and a variadic one returning an integer or nothing,
keep the default model, whose storage for them is the same.

An image that states the base standard with an FPU (`softfp`), or no
convention, keeps the default layout. There a function's own declared
floating-point return is still laid out in a VFP register, so moving only its
variadic callee's would break the tail call `return vr(k, 2);` (`b vr`), which
prints right today because both sides read `d0`. The `protoorder lock` park
gives every recovered list a variable tail that is a floor for argument
recovery, not a variadic signature; on a VFP image those pieces state the
default model's return storage (`kuna_vfpvariadic::parked_output`), so a
recovered `float` return is still read from `s0` at its callers.

`force_set` first saves a copy of the full `FuncProto` — model, storage, locks
and all — into the function's Override store keyed by the call address
(`decompiler/crates/kuna-decomp/src/p0_knowledge/overrides.rs
(FuncProtoOverride::prototype)`). It then tries `late_restriction`, the same
in-place merge as above. On success it commits the new input and output lists
to the CALL op straight away, and the arm reports a change so the enclosing
stackstall loop runs its rules again over the new inputs. On failure it sets
restart-pending ((kuna) reason `ProtoForced`). Either way the site's input is
locked afterwards. On the re-run, `FlowInfo::build_call_specs` reads the saved
`FuncProto` before it looks for a parsed-pieces override, so the restarted call
site starts under the declared storage rather than one rebuilt from the bare
types under the default model, which would lose a non-default convention.

That is why the code type keeps what the declaration said. The grammar's
function modifier builds the pointed-to prototype under the calling convention
its declarator names (`void (__stdcall *cb)(int)`, or `MSABI` on a SysV image),
resolved through the model registry the architecture shares into the type
factory, and falls back to the default model when the name is not registered,
as upstream's `TypeDeclarator::getModel` does. A `...` in the field's parameter
list marks the prototype varargs, so trailing arguments the binary passes are
kept. Parameter names are not carried into the code type, as upstream: code
types compare equal regardless of names, so a name would be borrowed by
whichever matching pointer type was built first. Because the forced prototype
is locked, it also overrides the argument count the binary sets up, as in
Ghidra: a call that casts the pointer to a wider signature prints with the
declared arguments.

This does not include the literal import-slot case above: `ActionDefaultParams`
consumes that already-present global type before input trials start and needs
no target rewrite or restart. Restarts triggered here are refused during
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

### (kuna) `narrowext` — a narrow argument or return value extended as the ABI states

A compiler spec states one extension per register entry: `zero`, `sign`,
`inttype` (by the value's type) or none. The extension is applied in three
places: after a call whose return value is locked narrower than its register
(`ActionFuncLink`), when a return value committed from trials is narrower than
the register the caller reads (`FuncCallSpecs::commit_new_outputs`), and at the
entry of a function whose locked parameter is narrower than its register
(`extend_input`). One attribute per entry cannot say what the RISC-V and
LoongArch procedure-call standards say. RISC-V widens an integer narrower than
XLEN by the sign of its type up to 32 bits, then sign-extends it to XLEN, and
returns a value as it would pass the first argument of that type. LoongArch
zero-extends an unsigned integral value and sign-extends a signed one, except
that LP64 keeps an unsigned 32-bit value sign-extended, and returns a value the
same way. The RISC-V specs state `zero` and the LoongArch ones nothing, so on a
64-bit image a call to `int fi(int)` read as a 64-bit value printed
`(unsigned long)(unsigned int)fi(k)`, a callee that used its `int` parameter as
an index printed `(unsigned long)(unsigned int)x`, and a narrow LoongArch value
printed as `CONCAT44(v1,fi(k))` with an unassigned `v1`.

`narrowext` replaces the spec's answer with the convention's
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_narrowext.rs`). The value must
sit in the least significant bytes of a register entry (not a join, not a
floating-point entry, not the stack) and its type must state its sign: a signed
integer or enum is sign-extended, an unsigned one, an unsigned enum or a `bool`
is zero-extended. Under the rule for RISC-V, LoongArch and MIPS a value
narrower than 32 bits takes that extension to the whole register, and a 32-bit
value in a 64-bit register is sign-extended whatever its sign. Under Apple's
rule a value narrower than 32 bits takes it to the low 32 bits only, and the
rest of the register is left as the call or the entry leaves it. Plain `char`
keeps the spec's extension, since its sign is the platform's (unsigned on Arm,
AArch64 and RISC-V) while kuna's `char` is one signed type everywhere, and so
do the wide character types, an undefined type, a pointer, a float, a
structure, and every value on any other processor. `signed char` and
`unsigned char` are character types too, but not the target's own (core) one:
they state their sign, and the DWARF reader keeps them apart from plain `char`
(01). A typed
indirect call whose return value the rule extends is forced as declared (§4.3).
The rule is only as right as the sign the type states, so the type sources give
a narrow type the sign its source declared (01, DWARF and `cppsig`): `char16_t`
is unsigned whether DWARF (`DW_ATE_UTF`) or a mangled name (`Ds`) states it,
where it used to read as `short` and the rule sign-extended a value the caller
zero-extends; an enum takes the sign of the integer its `DW_AT_type` names when
it has no `DW_AT_encoding`, as clang emits it; and an anonymous enum narrower
than 32 bits falls back to an integer of its own sign, unsigned when DWARF
states none.

The value says which rules apply. `abi`, the default, applies a rule wherever
an ABI document states it: RISC-V and LoongArch arguments and return values,
the arguments of MIPS o32 (the System V MIPS supplement promotes every integer
argument to a 32-bit word) and n32/n64 (SGI's handbook promotes them to 64 bits
and sign-extends a 32-bit one whatever its sign), and the arguments of Apple
arm64, whose caller extends a value narrower than 32 bits to 32 bits. None of
those documents says how a return value is extended, nor how an EABI or o64
argument is. `compiler` adds those, as GCC and LLVM implement them: their MIPS
callees widen a return value like an argument and their callers read it as
it is, and Apple's clang callee extends a return value narrower than 32 bits
to 32 bits. A MIPS caller that reads such a result as a whole register printed
`CONCAT31(v1,fc(k)) * 3` with `v1` unassigned, and an Apple arm64 one
`(unsigned int)(unsigned char)esc(k) * 3`. `compiler` also gives the x86-64
System V arguments (compiler id `gcc`, which clang images load as) Apple's
argument rule. The psABI says nothing about the bits above a narrow argument,
but GCC and clang callers extend it to 32 bits by the sign of its type, and
clang callees rely on it: `long widen(unsigned char c) { return c; }` is
`mov eax,edi; ret`, which printed `(unsigned long)CONCAT31(v1,c)` with `v1`
unassigned and prints `(unsigned long)c` under `compiler`. A GCC callee extends
the value itself, so there the stated extension is almost always dead.
x86-64 return values, the Windows and Go conventions, and i386, whose arguments
travel on the stack, keep the spec's extension. `off` keeps every spec's
extension.

An extension the rule supplies is marked on its op. The spec's zero extension
was trimmed where an unprototyped call or the function's own return read it,
because dead-code analysis counts only the possibly-nonzero bits of such a
read as consumed (`ActionDeadCode::markConsumedParameters`,
`gatherConsumedReturn`) and sub-variable flow then narrows it. A sign extension
has no known-zero bits, so a marked one, or a copy or merge of marked ones of
the same value, counts only the value it widens as consumed
(`kuna_narrowext::implied_sign_extension`). An unprototyped call forwarding a
declared `int` parameter therefore still prints `rsc(k)` and a function
returning an `int` call's result still returns `int`. A compiler's own
sign-extending instruction is not marked and keeps upstream's treatment. A
narrow value the function compares or computes with as a whole register prints
the extension it performs, `(long)k` or `(int8)token`, where a zero extension
used to fold away.

The same rules answer the reverse question for a return value the function
recovers rather than declares: whether the convention extends a value of its
width by the sign of its type (`kuna_narrowext::extends_by_sign`), under the
rule where it speaks and otherwise where the spec states `inttype`. Where it
does, a return the function zero-extends is unsigned (chapter
[05](05-types.md), `kuna_zextreturn`). Where the rule sign-extends a value of
that width whatever its sign (`kuna_narrowext::sign_extends_any`: a 32-bit value
in a 64-bit register), a zero-extended return whose sign bit may be set is no
32-bit value at all, and the RETURN keeps its whole register (chapter
[03](03-ssa-and-simplification.md)).


### (kuna) `calleeprotostack` — the declared callee's stack contract

A locked prototype says two things about the stack, and until this option
existed the pipeline acted on neither.

**How much the callee pops.** `FuncProto::resolve_extra_pop` turns a locked
parameter list into the `4 + <stack argument bytes>` a callee-cleans convention
removes, and it is the only answer available for a model whose
`<prototype extrapop="unknown">` declines to state one — which on x86 Windows is
every call, because `x86win.cspec`'s default proto is `__stdcall`. Nothing called
it, so a locked prototype still reached the stack solver as extrapop-unknown and
`calleepop` (§6) guessed the cleanup from the push run in front of the call.
That run is not always the callee's: an image that stages one API's arguments,
calls a second argument-less API in the middle of the run, and pushes the rest
afterwards has its staged pushes credited to the wrong callee, and the solver
latches a stack pointer too high into every later reference. The option asks
`resolve_extra_pop` for the answer whenever the prototype is input-locked and
the model states no extrapop of its own; a model that states one already has the
exact value and is left alone, so `__cdecl`, x86-64 and every RISC spec are
inert.

The same park is read a second time, at de-indirection. `ActionDefaultParams`
runs once at the head of the function, when a `call dword ptr [IAT slot]` is
still a CALLIND with no callee to ask about, so a parked signature reached such a
call only when something else forced the action list to restart. Asking again in
`ActionDeindirect`, where the callee has just been resolved, makes the delivery
deterministic rather than incidental.

**How much of the caller's stack it can reach.** `Heritage::guardCalls` (§4.3,
*Effect lists*) asks the prototype what the call does to every heritaged range
and gets `unknown_effect` for the whole stack, so an INDIRECT guard is planted
over each of the caller's slots. That is the honest answer for an unknown callee.
For one with a locked, non-variadic prototype it is not: the callee owns the
return-address slot and its own parameter area, and nothing above them. A range
wholly above that floor — the prototype's extrapop, read as a signed offset in
the stack space's own width — is left unguarded, so a value staged before the
call reaches the call it was staged for as the constant or address-of the source
wrote, instead of a local assigned on the line before.

The claim is declined wherever it is not evidence: the prototype must be locked
and non-variadic, the floor must come from the rule above, and the range must be
one no pointer in the caller can reach. That last test is
`AliasChecker::has_local_alias` — the same one `FuncCallSpecs::checkInputTrialUse`
applies before it will call a stack slot a parameter. It is what keeps
`ReadFile(h,&buf,…)` correct: `buf`'s address is taken, the callee writes it
through the pointer, the guard is what models that write, and it stays.
`has_local_alias` answers only for locals below the return address, so the
same question is asked of the caller's own parameter area
(`AliasChecker::has_parameter_alias`): a slot there is reachable once any
address at or below it in that area is taken. MSVC x64 spills a value into the
home slot its caller allocated and passes the slot by reference (every Qt
`connect(sender, &Class::signal, …)` does), and without that test the store
before the call was dead. The gather is deferred and cached for the length of one heritage pass, so a function
that never reaches the locked branch never pays for it.

### (kuna) `callpush` — a call's own return-address push

An x86 `call` lifts as three p-code ops at one instruction address: the stack
pointer steps down one word, the fall-through address is STOREd at the new
stack pointer, and the CALL transfers. In a frame whose stack pointer stays a
constant offset from its entry value, `RuleStoreVarnode` (chapters 03 and 06) turns that STORE
into a COPY to a stack slot nothing reads, and dead-code elimination removes it,
so no push is ever printed. After an `alloca` (`sub rsp, rax`) the stack pointer
is the entry value minus a run-time size: the STORE keeps its pointer, and every
later call in the function printed its push as a statement of its own,
`*(unsigned long *)&v28[-8] = 0xbc79;` before `fstatat(...)`. Over the cast
census's 45 binaries at three optimization levels these were 1,607 statements
in 75 functions; on the functions kuna and IDA both emit they carried 856 of
the 1,252 `(unsigned long *)&v` casts kuna printed, and IDA prints no statement
for them.

With `callpush on` (the default), `RuleCallPush`
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_callpush.rs (RuleCallPush)`)
deletes such a STORE. It runs in `oppool2` directly after `RuleStoreVarnode`, so
a push in a tracked frame is that rule's COPY before this one sees it, and it
matches only when every one of these holds
(`kuna_callpush.rs (is_call_push)`): the stored value is a constant one
pointer word wide lying at most 15 bytes (the longest x86 instruction) past the
instruction's own address; a CALL or CALLIND follows the STORE in the same
basic block at the same instruction address, so both came from one `call`; that
call's destination is not the stored address (`call 1f; 1: pop` reads the
pushed word back, and is the one idiom whose caller does); the pointer, through
COPY and CAST, is the stack pointer register as that same instruction wrote it;
and `RuleLoadVarnode::check_spacebase` cannot place the pointer on the stack. A
callee that pops its return address as data is lifted as a BRANCH by
`callpopret` (chapter 02), so no CALL remains for the rule to match.

The deleted store writes the slot below the stack pointer that only the
callee's `ret` reads, which the C call already performs, so no value any printed
statement computes changes. What follows from the deletion is the ordinary
pipeline reacting to one fewer may-alias STORE, and each of these was read in
the whole-corpus diff: the INDIRECT guards the STORE planted over the stack
collapse (`RuleIndirectCollapse`, whose causing op is now dead), which also frees
tracked return-address slots those guards alone kept alive; a load or call
result held in a temporary across the push now prints at its use; a block that
held only the push beside a condition merges into the condition; a tail block
the push kept large is duplicated into its predecessors instead of reached by
`goto`; and a stack array whose lowest slot was the push is declared from its
next slot, with every reference shifted by the same amount. Stores of
stack-passed arguments through the same pointer, and a stack probe's
`*p = *p` touch, are not the call's push and stay. `callpush off` restores the
upstream statement.

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

### (kuna) `condexeret` — a return register tested on the same condition twice

`AncestorRealistic` fails a return trial as soon as one MULTIEQUAL input is
the function's own register on entry and not directwrite, so one path that
leaves the register untouched is enough to make the function `void`. A register
written under a condition and then branched on by the same condition again has
such a path — the first branch taken, the second taken too — that cannot run:

```text
twice:  cmp rdi,rsi; jae 1f; mov eax,0; 1: jb 2f; lea rax,[rdi+rsi]; 2: ret
```

`ActionConditionalExe` threads the merge block at `1:` and the path is gone,
but it runs at the end of mainloop, after `ActionReturnRecovery` has already
spent the output container's only pass, so `twice` printed `void twice(void)`
while the one-branch spelling of the same logic returned a value. Upstream has
the same order.

With `condexeret on` (the default), when the walk fails at a non-directwrite
function input read by op `m`
(`decompiler/crates/kuna-decomp/src/substrate/funcdata_varnode.rs
(AncestorRealistic::input_fail_reader)`), and `m` is a MULTIEQUAL in a block
`ActionConditionalExe` could thread — two in-edges that lead back through
straight-line blocks to one block ending in a CBRANCH, two out-edges, and a
CBRANCH on the same condition as that block or its complement
(`BooleanExpressionMatch`), i.e. `ConditionalExecution::verify` short of its
op-removability test
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_condexeret.rs
(threadable_block)`) — a second walk runs from the same RETURN
(`kuna_condexeret.rs (remember)`). That walk is put in *collect* mode
(`funcdata_varnode.rs (AncestorRealistic::collect_inputs)`): a non-directwrite
function input is recorded and passed instead of failing there, so one walk
reaches the end and sees every such input on every path. The trial is
remembered — with the merge blocks that read those inputs, and the inputs'
storage — only if that walk otherwise succeeds and every input it recorded is
read in a threadable block. So a failure that is *not* a same-condition merge,
anywhere on any path, stops the trial from being remembered at all: in the
counterexample below the sfp body's writes sit behind merges on unrelated bit
tests, so `comp` is never remembered.

At the end of the last normal pass (`kuna_condexeret.rs (end_pass)`), if a
remembered trial is still unchecked, the budget grows by one and the container
stays open into the next mainloop iteration
(`kuna_condexeret.rs (remembering)` marks that one pass). That pass re-walks a
remembered (trial, RETURN) pair only once *all* of its merge blocks are gone
from the graph (`kuna_condexeret.rs (check)`), and the re-walk runs in *strict*
mode: every function input overlapping the recorded storage fails the walk
whether or not it has become directwrite in the meantime
(`funcdata_varnode.rs (AncestorRealistic::forbid_inputs)`). Everything else the
first walk met already passed it, and the recorded inputs are exactly the
places it would have to pass now, so the re-walk can succeed only where
threading has left no path from the register's entry value to the RETURN. It
cannot pass for an unrelated reason — a later rewrite that makes the register
directwrite does not help, because the strict mode ignores directwrite on the
recorded storage. This is why a bare merge-gone gate is not enough: a fresh
whole-graph walk on the later IR can pass this function for a reason unrelated
to the removed path (kuna's own `__sfp_handle_exceptions` did), and the
collect/strict pair ties the retry to the threading itself.

The container then closes, so the extra pass happens at most once per recovery.
Call-site trials never get it. With the gate off the container closes after
its normal budget, as upstream.

Holding the container open one iteration longer means the mainloop tail runs
while every RETURN still reads all output trial registers. `ActionConditionalConst`
assumed a RETURN's slot 1 is its value and wrote a constant known for another
trial register there; that is fixed at its root in chapter 03 (Conditional
constants), for every open container, not only this one.

`passthrough` (§ above) claims a tail call's result before the ancestor walk
and skips the walk for that trial, so a claimed trial never reaches the
remember step; in the extra pass only remembered (trial, RETURN) pairs are
looked at, and `keep_tail_return_whole` still runs once, just before
`derive_output_map`, which with a pending retry happens after the extra pass.

### (kuna) `condexeretuse` — a use the re-test rules out

`condexeret` covers a return trial whose ancestor walk fails at an input. The
same merge blocks also fail return trials through the sole-use check. ARM
conditional execution writes the return value and returns under one
condition, and tail-calls under the other:

```text
pick:   cmp r0,#0; moveq r0,#7; bxeq lr; b getv
```

The `moveq` becomes a block that writes 7, the `bxeq` a merge block that
branches on Z again. In the graph the 7 flows through the merge's MULTIEQUAL
into both the RETURN and the tail call's `r0` argument slot, though the
execution that wrote it always takes the return. `ActionReturnRecovery`
scores output trials before `ActionConditionalExe` removes the merge, the
CALL counts as a competing use, and `int pick(int a) { if (a) return getv();
return 7; }` printed `void pick(int a0)`.

With `condexeretuse on` (the default), a sole-use walk matched at a RETURN
that fails, and that passed through a MULTIEQUAL in a merge block of the shape
`condexeret` uses, is walked once more
(`decompiler/crates/kuna-decomp/src/substrate/funcdata_varnode.rs
(Funcdata::only_op_use)`). In the repeated walk, a value that enters such a
MULTIEQUAL through exactly one in-edge fixes the merge's branch for that
execution: the edge leads back through straight-line blocks to one out-edge of
the init block, and the merge tests the same condition or its complement.
`ConditionalExecution::forced_out_block`
(`decompiler/crates/kuna-decomp/src/p3_dataflow/condexe.rs
(ConditionalExecution::forced_out_block)`) names the out block, using the
control-flow tests of `ConditionalExecution::verify` without its
op-removability test. The MULTIEQUAL's output carries the merge and that out
block, and so does every value computed from it inside the merge block
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_condexeretuse.rs (carried)`).
A use of such a value outside the merge block is skipped when the forced out
block cannot reach it (`kuna_condexeretuse.rs (cannot_arrive)`). The
reachability walk follows only the forced out-edge at any other such merge it
enters through one in-edge, so a 64-bit `moveq r0,#7; moveq r1,#0; bxeq lr`,
whose first value passes two merges that re-test Z, is handled too. Past 4096
visited (block, in-edge) states every block counts as reachable.

A skipped use is one the value being scored cannot reach. Skipping uses is
only half of the argument, though: the repeated walk passes only if it also
reaches the RETURN slot being matched through a use it did not skip
(`decompiler/crates/kuna-decomp/src/substrate/funcdata_varnode.rs
(Funcdata::only_op_use_walk)`). Without that, a value whose forced branch
leads away from the RETURN would pass with nothing checked: in
`cmp r0,#0; moveq r0,#7; beq 1f; str r0,[r1]; bx lr; 1: b 1b` the 7 goes
into the spin loop, the STORE and the RETURN are both out of its reach, and
the function must stay void; the same holds when the 7 is passed to a call
that does not return (`bleq exit`). With both conditions, the repeated walk
answers upstream's question for the executions that exist: the value reaches
the RETURN, and every use it can reach on the way was checked. It keeps the
upstream rejection when a value is reached both through such a merge and
along another route, since the uses it skipped would then be real.
Uses the scored value can reach still reject the trial: in `differ`
(`cmp r0,#0; moveq r0,#7; cmp r1,#0; bxeq lr; b getv`) the second test is on
r1, the 7 reaches `getv` when `a1 != 0`, and the function stays void. A STORE
of the value on the returning path (`streq r0,[r1]`) still rejects too. The
repeated walk runs only when the first walk fails after entering such a merge.
Call-site input trials are untouched, so the tail call still shows the phantom
`getv(a0)` argument the infeasible path gives its `r0` trial. With the gate off
the check is upstream's.

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
is not a move, so the walk stops there), and the rule only ever edits a value
concatenated from two halves, never a lone recovered return register. Where every
half is uncomputed — the synthesized-return case — the low, first-in-class
register is kept so the function's output storage still agrees across every
RETURN. A pair joined first register high is left alone: return recovery joins
one that way only when its low word is part of the value (see the join order
above; `decompiler/crates/kuna-decomp/src/p4_calls/kuna_bejoin.rs
(first_register_holds_high)`).

A high register the function set to zero is computed too, so the pair of a
zero-extended 64-bit return survives this repair. The earlier trim that narrowed
it to its low register, and printed `unsigned long long` as `int`, refuses a pair
no wider than eight bytes (chapter [03](03-ssa-and-simplification.md) §3.3).

This subsumes `returnpair` on the GH-6990 case it was written for (`tests/stages/
gh6990-returnpair.xml` now records both passes agreeing); the flag remains as the
blunt per-function instrument for a pair this rule judges genuine.

The same walk answers a second, stricter question for `passthrough`:
`every_return_computes` asks it of the whole function and of a lone return
register, where this repair cannot act, and requires *every* input of a phi or a
`PIECE` to be computed rather than any one of them, so a value merged from one
path that computes it and one that does not is not computed. It is what decides
whether a callee's recovered return may be stated to its callers at all; see
*The register a function forwards to a callee that reads it* below. It runs once
per decompiled function and only when that option is on, as a worklist over the
reachable move-only closure -- the recursive "every input" form revisits the same
Varnodes exponentially on a phi-rich -O0 body.

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
  address is compared with the storage the half occupies in the returned value
  (see *A value built in one return register* below): a *different* address means
  the function executed an instruction to move the argument into the return
  register, while the *same* address means the register was never touched and the
  caller's value is passing straight through — leftover, which is precisely what
  the sibling rule exists to drop, unless the function moved it out to another
  register and back (below).

A weaker version of the placement test was tried and rejected. It also rescued
the pair when *every* half was an untouched incoming argument, on the theory that
`double f(double x) { return x; }` on ARM returns its argument in the registers it
arrived in; that recovered three betaflight soft-float helpers and simultaneously
resurrected the GH-6990 SPARC symptom, because a *void* `main` that touches
nothing leaves `o0:o1` passing through and SPARC passes arguments in those same
registers. Nothing local to the pair separates the two, so the placement test is
applied per half with no exception.

One shape does reach the same address with the function's hand on it: an
argument carried across a call in a callee-saved register and moved back into its
own register to be returned. `unsigned long long own(unsigned a, unsigned b) {
ext(); return (unsigned long long)b << 32 | a; }` on ARM keeps `a` in `r5` and `b`
in `r4` across `bl ext`, then ends `mov r0,r5; mov r1,r4`. Once copy propagation
has collapsed the moves, `r1` reaches the RETURN as the caller's own `r1`, the
placement test read it as leftover, and the function printed `unsigned int
own(unsigned int a0) { ext(); return a0; }`: the high word and the parameter `b`
were gone. On AArch64 the high half was sometimes the one kept, returning `b`
alone in place of the whole value.

The moves are still in the IR when return recovery builds the pair, so it
records then (`decompiler/crates/kuna-decomp/src/p4_calls/kuna_retinputhalf.rs`,
`note_moved_back_returns`) each used output register whose value at some RETURN
walks back through moves, phis and non-creating indirects to its own input by way
of another register. The repair skips the placement test for a recorded register
at every RETURN; the half must still be parameter storage. The record is per
register because a function returns in one storage: once one path moves the
argument back to return it, the same register left untouched on an early return
is that argument passing through, and keeping the pair at one RETURN but not the
other would give the function two return widths. A move is a COPY, or the `or`
or `add` with zero and the self-`or` that MIPS, SPARC, RISC-V and PowerPC's `mr`
lift to before the rules reduce them.

Three kinds of copy are not moves. An incidental copy (Xtensa's register-window
swap around a `call8`) is walked through without counting. A move to another
storage made by an instruction that also writes the stack pointer ends the path:
SPARC's `save` and `restore` copy every `%o` register to its `%i` register and
back, so a SPARC function that never touches `%o1` has the moved-out-and-back
shape on every return, and both instructions move the stack pointer, as does the
`pop` of a pushed register. And a move back that a call or a CALLOTHER can follow
on some path to the RETURN ends the path too. An inline system call reads its
arguments from registers its p-code does not show: ARM `svc`, AArch64 `svc`,
RISC-V `ecall` and PowerPC `sc` lift to a CALLOTHER with no register inputs. So
`int sys2nr(int a, int b) { ext(); r0 = a; r1 = b; svc 0; return r0 + 1; }`
moves `b` back into `r1` for the kernel, not for its caller, and stays an `int`
function. The reachability is block-level: the move's block after the move, every
block reachable from it, and only those from which the RETURN's block can be
reached (the RETURN's own block counts up to the RETURN).

Moving a register back proves that the function returns it, not that it
returns that register alone. The high register can be moved back while the
low one is left untouched: compiler-rt's `__ashrdi3` on RISC-V 32 copies the high word aside at entry (`mv a3,a1`) and, when the shift is zero,
returns `a` with `mv a1,a3; ret`, leaving `a0` as it arrived. The recorded high
half is real and the low half fails the placement test, which would send the
RETURN down the keep-the-high-half arm and hand back the high word as the whole
value. Instead, when the high half is real only through the record and the low
half at that RETURN is the function's own argument passing through (real once
the placement test is dropped), the pair stays: the function returns the
argument whole. Any other low half takes the answer the strict rule gives, so
the record never returns the high register alone. A low register moved back
needs no such rule: the repair keeps the low half whenever the high one is not
real anyway.

`tests/stages/gh6990-returnpair.xml` keeps the SPARC function returning one
register, and `tests/stages/kuna-ownreturn.xml` pins the ARM shapes, the `svc`
control and a hand-written `__ashrdi3` shape; `kuna-cli/tests/own_register_returns.rs`
compiles the ARM, Thumb and RISC-V 32 output back and runs it against the
source. The same functions on a big-endian target keep both halves, in the
order the pair is joined in, which is swapped on big-endian until that join is
fixed.

The same keep-the-high-half arm met the low register holding the function's own
argument after a spill. clang `-O0` builds `u64 kc(unsigned a) { return a |
(5ULL << 32); }` as `str r0,[sp]; ldr r0,[sp]; mov r1,#5`: at recovery time `r0`
is a load and both trials are active, but by the repair heritage has resolved the
load to the caller's own `r0`, the placement test reads it as untouched, and the
function printed `unsigned int kc(void) { return 5; }`. A high register of a
little-endian pair is the second of its class and never a return value alone, so
when the low half is the function's own argument (real once the placement test is
dropped) the pair stays and `kc` returns `CONCAT44(5,a0)`. The function returns in
one storage, so this holds only while no other RETURN of the function is narrowed
to a single register; when one is, the argument is the return value at this
RETURN too. u-boot's ARM `memcpy` is that case: one exit leaves the advanced
source pointer in `r1` and `dest` untouched in `r0`, the others return `r0` alone,
and the exit that printed `return v2;` (the source pointer) now returns `a0`. On a
big-endian pair the high half is the first register, and the arm is unchanged.

The predicate runs inside `ActionOutputPrototype`, which is scheduled *before*
`ActionInputPrototype`, so the proto's own parameter list is not fixated yet and
the question goes to the model — the same fall-through
`possible_input_param` takes when no locked parameters exist.

#### A value built in one return register

A `PIECE` at the RETURN is not always the pair return recovery joined. The function
itself builds one when it assembles a value in its single return register:
`((u64)hi << 32) | lo` folds into `RAX = PIECE(ESI, EDI)`, whose halves are the
argument registers themselves, and heritage refines a partly written register into
`RAX = PIECE(RAX[4:4], EAX)`. The repair handles both with two rules.

* **Placement is measured against the bytes the half occupies.** For a pair each
  half occupies one register of the join, read from the join record; in a single
  register the low half sits at the register's address and the high half above it
  (the other way round on a big-endian register file). Measuring against the
  half's *own* address, as the repair once did, makes any argument folded straight
  into the value "arrive" where it already is, so `ESI` in RAX's high half read as
  the caller's leftover: the half was dropped, the return narrowed to four bytes,
  and the argument lost its only reader — `unsigned int join_lo_hi(unsigned int
  a0) { return a0; }` for `((u64)hi << 32) | lo` at -O0. The same measurement keeps
  a pair half the function moved from an argument register after copy propagation
  has replaced the move with the argument itself (two argument registers swapped
  into `r0:r1` on ARM).
* **The high half of one register is never returned alone.** Rewriting the RETURN
  to the high half of a single-register value hands back those bits as the whole
  value: `((u64)(a + 1) << 32) | b` at -O2 printed `return a0 + 1;`, and on AArch64
  a low half that is the first argument left in `w0` sits at its own slot and must
  read as untouched. So in one register only the low half can survive, and a value
  whose high half is computed keeps all its bytes. In a pair each register is its
  own location, and either half may survive as before.

A high half that really is leftover still goes: the upper half of RAX after a
callee that returns `int` in EAX sits at its own slot and carries nothing the
function computed, and the return narrows to EAX exactly as before. Over the
castbench corpus and 98 further binaries (60,037 functions) the change moves no
function; the shape it corrects is pinned by `tests/stages/kuna-returnpiece.xml`
and by the compiled round trip over the `piecehi_*` fixtures in
`kuna-cli/tests/decompile_all_cli.rs`.

#### (kuna) A call's result beside a computed second register

`unsigned long long full_or(unsigned a) { return full(a) | 0xff00000000ULL; }`
is, on 32-bit ARM, `bl full; orr r1,r1,#255; pop {r11,lr}; bx lr`: `r0` comes
back from `full` as it is and only `r1` is written. It printed as `void
full_or(int a0) { full(a0); }`, in `kuna decompile` and in `decompile-all` alike,
and so did a soft-float `double` negated (`eor r1,r1,#0x80000000`), an `int`
call result sign-extended (`bl g; asr r1,r0,#31`) and `((u64)7 << 32) | g(a)`.
At the RETURN `r0` is the call's INDIRECT creation, which upstream's
`ancestorOpUse` refuses as a return value; `r1` passes, but the output model
never returns the second register of a pair without the first, so the function
gets no output at all. The
`passthrough` tail claim does not help: it is made only when the function touches
neither register of the callee's stated result.

A register that is not the first of its class is never a return value of its own,
so a function that computes one and hands it only to the RETURN returns the pair
it belongs to. `kuna_retcallhalf::accept`
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_retcallhalf.rs`) runs once
`ActionReturnRecovery` has scored every trial for the last time, before the output
map is derived, and makes an inactive trial active when all of these hold:

* the model derives nothing from the trials as they stand, and exactly one trial
  is active;
* at every live RETURN the inactive trial's value is a call's untouched result:
  the INDIRECT creation of a CALL or CALLIND, read directly, through the injected
  no-op of an ARM or MIPS return, or through a phi of such values;
* the active trial's value was computed on purpose at every live RETURN and is not
  zero at all of them;
* both trials are the same width, the register file is little-endian, and the
  model, given the candidate, returns exactly those two registers with the
  candidate first.

"Computed on purpose" rules out the four kinds of write that reach a RETURN in a
function returning one register or none. The other output of an instruction that
writes two registers: an ARM `smull r1,r2,r2,r3` whose high word is a division by
a constant leaves the low word dead in `r1`, and cleanflight's `sub_802f412`
became `return CONCAT44(v2[0] * 0x51eb851f,v1)` without this limit. So the
instruction producing the value, and every instruction on a phi path to it, must
write no register wider than a flag besides its own output. A restore from the
frame: gcc's `pop {r1,r2,r4,pc}` releases stack space, and an `-O0` reload
`ldr r1,[sp,#4]` of a spilled argument is a half the late repair above drops
again, which would leave `int` where `void` was. So an instruction that writes the
stack pointer, a copy from a frame slot and a load through a stack-relative
pointer do not count. An argument set up for a later call or system call: an
inline `svc` lifts to a CALLOTHER that does not show which registers it reads, so
`bl g; mov r1,#5; mov r7,#4; svc #0; bx lr` would otherwise return
`CONCAT44(5,g())`, and the same holds for a call whose recovered arity is short.
So no call, CALLIND or CALLOTHER may be able to run between the producing
instruction and the RETURN, by the block-level reachability the `retinputhalf`
moved-back test uses. And the zero `-fzero-call-used-regs` leaves in every
call-used register a function does not return in, seen through the `PIECE` and
`SUBPIECE` heritage splits a 32-bit `xor` into before copy propagation runs:
x86-64 `long f(long a) { return g(a); }` built that way by gcc at `-O0` would
otherwise come back sixteen bytes wide. The equal-width rule keeps out
`or $0xff,%dl`, whose one-byte trial would join `RAX` into a nine-byte value with
the upper bytes of `RDX` missing; that write still prints as before (GH-852).

Three shapes stay as they were. A function that changes the low word and leaves
the high one untouched (`bl g; orr r0,r0,#255`) is byte for byte `int f(void) {
return (int)g() | 255; }` as well, and keeps upstream's answer. A first register
no instruction names gets no return trial at all, so i386 code that pushes its
arguments (`call g; or $0x12345,%edx; ret`) still prints `void` when the callee's
result is not known (GH-853). And on a big-endian target the pair is still joined
in little-endian order, so the rule refuses it there rather than return the
halves swapped. gcc's `-fipa-ra`, which can set `$3` in a `jal`'s delay slot
because it knows the callee leaves it alone, is outside what any per-function
rule can see. The second register's value is only as good as the call's own
output: when it is computed from a narrower piece of the call's second register,
as a Cortex-M `uxtb r1,r1` is, the call can still lose that register and leave it
an uninitialized local (GH-851).

`tests/stages/kuna-retcallhalf.xml` pins the ARM callers and the four controls
(the `smull` word, the hardening zero, the dummy pop and the `svc` argument) in
single-function mode, and `kuna-cli/tests/call_result_pair_returns.rs` compiles
the ARM, MIPS and i386 output of `decompile-all` back with gcc and clang and runs
it against the source.

#### (kuna) The argument left in place, or a low word that also makes the high word

Two more first halves are refused by upstream's scoring while the function
computes the second register for the RETURN alone. `long long sx(int a) { return
a + 3; }` is ARM `add r0,r0,#3; asr r1,r0,#31`, i386 `add $3,%eax; mov %eax,%edx;
sar $31,%edx` (gcc: `cltd`) and MIPS `addiu $2,$4,3; jr $ra; sra $3,$2,31`: the
low word is read by the RETURN and by the shift that makes the high word, and
`onlyOpUse` takes the second read for a competing use. `unsigned long long
zx(unsigned a) { return a; }` is ARM `mov r1,#0; bx lr`: the low word is the
caller's own `r0` passing through, which ancestor realism refuses. Both printed
`void f(void)`, losing the return value and the parameter, as did a constant or
copied high word beside the argument (`mov r1,#5`, `mov r1,r0`) and a sign
extension after a conditional move or a merge (`cmp r0,r1; movle r0,r1; asr
r1,r0,#31`, i386 `cmovl` then `cltd`, MIPS `movz` then `sra`).

`kuna_retcallhalf::accept` takes these too. At every live RETURN the first
register's value must be one the function produced or was handed: walked back
through phis, value-preserving INDIRECTs and the injected no-op of an ARM or MIPS
return, every value it merges is the function's own argument in that register,
a constant, a call's result, or the output of an operation other than a CALLOTHER
or a clobber. And at one RETURN at least there must be a sign the pair belongs
together: the first register is a call's untouched result (the rule above), the
function's own argument in place (`kuna_retcallhalf.rs (is_own_input)`, gated with
`retinputhalf`), or a value that, followed forward, is read only by the RETURN in
its own slot and by operations that turn it into the second register's value
there (`kuna_retcallhalf.rs (feeds_second)`): no branch, call, load, store or other
RETURN, nothing written to global memory. That walk starts at the merged value
itself and, when the merge comes after the high word is made (two `lea`/`cltd`
exits joining at one `ret`), at each computed value it merges.

Beside the argument in place the second register is the only evidence, so it
must also have been set by the function on every path
(`kuna_retcallhalf.rs (set_by_function)`): a constant, a computation, or a move
from another register. A load does not count. crazyflie's USB code reads a
peripheral register into `r1` for its side effect and returns nothing; with a
loaded high word allowed it became `unsigned long long f(int a0,unsigned int a1)`
returning the dropped read. A zero counts only when the function names the second
register nowhere else (`kuna_retcallhalf.rs (only_read_as_zero)`) and the pair is
no wider than eight bytes: `-fzero-call-used-regs` zeroes a call-used register
the body used, or every call-used register, and the latter would also scrub the
first register of a function that returns nothing. A function returning only
`int` that is scrubbed with `all-gpr` keeps its argument in `r0` and now reads
`unsigned long long`, the zero extension of the same word, where it printed
`void`.

Wherever the pair rests on anything but a call's untouched result, the second
register's value must also be read by nothing but the RETURN
(`kuna_retcallhalf.rs (read_only_by_returns)`): every value it merges, followed
forward through phis, INDIRECTs, the injected no-op and temporaries, may reach
only a RETURN or a flag. A register the function hands to anything else was set
for that use. Hand-written `mov r1,#0x3000000; vmsr fpscr,r1; bx lr` or `mov
r1,#0x13; msr cpsr_c,r1; bx lr` leaves the caller's `r0` in place and a constant
in `r1`, and without this test printed `CONCAT44(0x3000000,a0)` for a function
that returns nothing.

For both rules, "computed on purpose" judges the instruction that makes the
second register by the other registers it writes, and some ops sharing its
address are not its writes: the MULTIEQUALs and INDIRECTs heritage places at a
block's start, a write nothing reads (the program counter a MIPS `jr` sets beside
its delay slot), the injected no-op of the return's mode switch, and the
tracked-register values `ActionConstbase` copies in at the function's entry (ARM's
`spsr`, at the address of a leaf function's first instruction). These no longer
make the second register another instruction's by-product.

Not fixed here. A first register no instruction names gets no return trial, so
RISC-V's `li a1,0; ret` still prints `void`; ARM's `bx lr` and `pop {...,pc}`
name `r0` through their injected no-op. A loaded high word beside the argument
(`ldr r1,[r1]; bx lr` for `((u64)*p << 32) | a`) stays `void`. A high register
that feeds the low one (`lea 1(%eax),%edx; mov %edx,%eax` for a duplicated word)
is also an `int` division by a constant (`asr r1,r0,#2; add r0,r1,r0,lsr #31`)
and keeps upstream's answer. gcc `-O0` leaves the sign of a 64-bit intermediate in
`%edx` after an `int` function's `>> 32`, which now reads `long long` with the
same low word where it printed `void`. On 64-bit targets the same shapes make a
16-byte pair, printed `undefined16` (`SEXT816(a0 + 3)`) where it printed `void`.

`tests/stages/kuna-retpairhalf.xml` pins seven ARM functions (`sx`, `zx`, `sxa`,
`kc`, `dupc`, `sxsel` and the spilled `kc0`) and seven controls (`int div7`, a
stored sign word, a scrubbed `r1` the body used, an `svc` argument, and an `r1`
written to `fpscr` or `cpsr`), and
`kuna-cli/tests/pair_return_halves.rs` compiles the ARM, MIPS and i386 output of
`decompile-all` back with gcc and clang at `-O0` and `-O2` and runs it against
the source. Over 28 stripped decbench binaries (ARM firmware at O0 and O2, i386 PE,
x86-64; 26,103 functions) `decompile-all` changes 18 functions: libgcc's
`__aeabi_dadd` and `__aeabi_ddiv` in seven firmware images, where an exit that
hands back the argument's low word with a new high word printed the high word
alone; u-boot's `memcpy` and `__of_translate_address`, both right now; and two
copies of a `tbb` jump table decoded as code in betaflight. Lua 5.4 built for ARM,
Thumb, i386, MIPS and RISC-V 32 by clang and gcc at O0 and O2 (12,503 functions)
changes none.

#### (kuna) The register that was only ever pushed (`retpushedhalf`)

The placement test asks whether the terminal arrived from a *different* address
than the half it reaches, and reads a different address as "the function executed
an instruction to move the argument here". A stack-alignment `push` in the
prologue paired with a `pop` into another register in the epilogue satisfies that
reading by accident. A four-argument XOR decryptor returning the buffer it
allocated pushes `R8` to realign the stack and pops that slot into `RDX`; `RDX`
at the RETURN therefore traces back to `R8` at entry, the half is kept, keeping
the half is what gives `R8` a reader, and `R8` having a reader is what makes it a
parameter — so the function recovers `undefined16 f(long,int,long,int,unsigned
long)` with `v._8_8_ = a4`, a fifth argument that exists only to be the high half
of a return that does not exist either.

The evidence that separates the accident from a deliberate `mov %r8,%rdx` does
not survive to the repair. Copy propagation collapses the store and the load long
before `ActionOutputPrototype` runs, and what is left — `RDX = COPY(R8)` at the
RETURN — is byte-for-byte what the deliberate move leaves behind. So it is
gathered during the flow build instead, while the instructions are still being
lifted: a register is **push-only** when some stack-adjusting instruction stores
it to memory and no instruction in the function ever writes it. `option
retpushedhalf` (default on, DIV-156) makes a push-only register fail the
input-parameter terminal test, so the half is uncomputed and the pair collapses
to the register that carries a value.

The rule reaches nothing that has no stack-adjusting store: a genuine returned
fifth argument moved with `mov %r8,%rdx`, and the untouched `RAX:RDX` of a real
128-bit return, are decided exactly as before. The ordinary callee-saved
save/restore is excluded by the write test, because the pop writes the register
it pushed. What it cannot separate is a function that pushes an argument register
purely to preserve it across a call and pops it into a *different* register that
it then returns; nothing local distinguishes that from the alignment idiom, and
this rule reads it as maintenance.

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
   until each path reaches a `RETURN` or a direct call known to never return,
   and records the processor-space writes it observes. The no-return query is
   shared with flow construction (`kuna_calleenoreturn.rs (is_no_return)`), so
   explicit marks, loader facts and enabled name rules have the same meaning.
   The probe stops at that call without decoding its callee or fall-through.
   A returning or unknown nested call, an unresolved indirect branch, an
   undecodable instruction or the instruction budget makes the summary
   *incomplete*, which proves nothing. A p-code-relative branch in the call
   instruction also makes the summary incomplete: a predicated call may be
   skipped and does not establish terminal machine-level flow. On a complete
   summary that never touches the payload register, the caller's read is a clobber and the pair is **vetoed**.

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
run rather than once per caller. Cached summaries retain the no-return verdict
for each encountered direct call; reuse rechecks those facts and repeats the
bounded decode if a verdict changed. Marking or unmarking a leaf therefore cannot
reuse an absence claim made under a different flow contract. Writes recorded
before a terminal call remain conservative veto evidence, and explicit call
effect overrides still take precedence. No no-return fact is inferred from a
void declaration. Nothing is probed unless the rule is live.

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

#### The two-register CALL output on any image (`callretpair`)

**(kuna)** The consumer half above is not about Rust and neither is the gap it
closes. A sixteen-byte aggregate return in `RAX:RDX` is ordinary System V C — a
QuickJS `JSValue` is `{ JSValueUnion u; int64_t tag; }` and comes back in exactly
that pair — and `option rustabi auto` cannot see a GCC image at all, so on every
non-Rust binary the stubbed multi-trial branch stayed stubbed. The visible
consequence is the strongest argument against it: ask kuna about the callee and
it answers `undefined16 sub_875e0(int8,char *,int8,uint4)`; ask it about the
caller and the same call has no output, with both halves rendered as locals the
function never assigns.

`option callretpair on|off`
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_callretpair.rs`) opens the same
arm with the language test dropped, and nothing else: the classification is
`classify_call_output_pair` unchanged, the callee-body veto is the same one, and
`build_call_output_pair` is the same code — the two options simply both reach it.
`rustabi` keeps the producer-side pair (`holds_scalar_pair`, which `callretpair`
does not touch), so a rustc image behaves identically whichever is set.

Both classifiers reason about registers, not significance: the callee veto asks
about the *second* register, and the producer's tag test about the *first*,
where rustc puts the discriminant. `build_call_output_pair` alone decides which
register is the high half, from `ParamActive::join_pair_order` as return
recovery does, so a big-endian call output reads `r3`/`$2`/`%o0`/`r0` as the
high word, except in a function whose own return pair was joined first register
low (see the join order in §4.2), where its calls' pairs are joined that way too;
`holds_scalar_pair` maps the PIECE's halves back to register order with
`first_register_holds_high` before asking for the tag.

The reach is whatever the cspec's output model asks for, not an architecture
list: two used output trials arise wherever a convention describes its return
storage as two consecutive register pentries with a join rule over them. That is
`<join_dual_class/>` on x86-64 System V and MIPS64, and the plain `<join/>` rule
over `r0`/`r1` on 32-bit ARM, where it is how every soft-float `double` comes
back from `__aeabi_dmul`.

The shipped default is **on**, which is upstream's behaviour for this branch and
the reason the option is a completion rather than a feature. Its evidence is that
it only ever adds a definition: the arm replaces two INDIRECT creations — values
standing for "the callee wrote something here" — with two `SUBPIECE`s of a value
the CALL now produces, so no statement is removed that was not a read of an
undefined local, and no call can lose an argument. Set it off to restore the stub.

**A caller that reads only the second register.** `unsigned hib(unsigned a) {
return (unsigned char)(full(a) >> 32); }` compiles on 32-bit ARM to `bl full;
uxtb r0,r1`: `r0` is overwritten unread, so its output trial finds no Varnode,
and only the `r1` trial is active. No output rule returns the second register of
a pair without the first, so the model used nothing, the call kept no output, and
`r1` printed as a local nothing assigns. i386 reads `DL` (`movzbl %dl,%eax`),
which heritage plants as a killed INDIRECT creation with no trial at all because
it is narrower than the `EDX:EAX` entry's minimum; a Cortex-M `uxtb r1,r1` beside
a read of `r0` leaves the model returning `r0` alone and `r1:1` unassigned.

`build_partial_pair` (`decompiler/crates/kuna-decomp/src/p4_calls/kuna_callretpair.rs`)
runs first in `build_output_from_trials`, while the unused trials are still
present. A read register value the model left unused (an unused trial, or such a
killed creation) that lies in a general-purpose register the model never returns
a value in alone — `r1`, `EDX`, `$v1`, `RDX`, big-endian `r4` — names a pair: the
two pieces of the join entry holding it, or the first-in-class register entry of
its class beside it. A probe `ParamActive` holding just those two full registers
is run through the model's own `derive_output_map`; the pair is taken only when
the model uses exactly both, and the probe's join order says which is the high
half. A shift by 32 names the half it reads, so the pair is formed only when the
call's join order is the ABI's: on a big-endian ABI (`r3:r4`, `$v0:$v1`) that
takes `option bejoin on`, because the shipped first-register-low join (GH-904)
would print a read of the low word `r4` as the high one. The CALL then
gains the pair as a `join` output, each read becomes a `SUBPIECE` of it at its
own byte offset (`DL` is byte 4, `r1:1` byte 4, `r0` byte 0), and the INDIRECT
creations are destroyed. When one trial was already used, as in the Cortex-M
case, the pair replaces that single-register output.

The evidence here is stricter than the two-trial arm's, because a value the
callee leaves alone reads the same way: gcc's `-fipa-ra` keeps a caller's pointer
in `$v1` or `r1` across a static callee it knows never touches that register,
including through the `lw $t9,%got(f)($gp); jalr $t9` a MIPS PIC call to a static
function is. The callee-body decode cannot prove the absence on ARM or MIPS, where
it never completes (every return runs `setISAMode`, and MIPS returns through
`jr ra`), so the pair is formed only when that decode *recorded* a write to the
register the caller reads (the piece holding the read, whichever piece of the
join entry it is). An indirect call has no decoded callee and keeps the old
rendering. A write is still not a return value: a `void` helper's scratch
register, a get-PC thunk's `EDX`, the remainder an `idiv` leaves in `EDX` beside an
`int` result. So where the run holds the callee's own recovery — `decompile-all`
files it callee-first in `voidret`'s ledger (`kuna_callee_returns`,
`kuna_callee_return_storage`) — the callee must be recovered returning a value
whose storage, or a piece of whose join, holds that register; a callee recovered
`void` or returning only its first register declines. Every surface without
the callee-first pass has no such recovery and relies on the decode alone:
`kuna decompile`, `decompile-all --jobs N`, a run narrowed by `--filter`,
`--functions`, `--addr` or `--limit`, `--option protoorder off`,
`decompile-project --stream` and `decompile-graph`. There a get-PC thunk's `EDX`
or an `idiv` remainder read after the call still takes the pair: `decompile-all
--jobs 4` of a gcc `-m32 -fPIC` image prints `(int)((unsigned long long)
sub_10d3() >> 0x20)` for its get-PC thunk, beside `void sub_10d3(void)`. And an
op of the function itself must read the register — not a CALL or CALLIND, which may be a later call's phantom
argument, and not a RETURN or the PIECE return recovery joins a returned pair
with, which are the function's own unsettled return trial; phis, INDIRECTs and a
return's injected no-op copy are looked through. A caller whose second-register
read stays unexplained keeps the old rendering: the uninitialized local. The
shape is pinned by `tests/stages/kuna-callretpairhalf.xml` (ARM and Thumb, with
controls for a callee that never writes `r1` and for an indirect call) and by the
compiled round trip in `kuna-cli/tests/call_result_high_word_reads.rs` (ARM,
MIPS, i386, with a `void` callee and an `idiv` remainder as controls).

### (kuna) A resolved format call's open tail (`formatstring`)

**(kuna)** When the format string of a printf/scanf-family call is resolved
(chapter [01](01-program-prep.md), `formatstring`), the call gets a prototype
override with exactly the arguments the format consumes. How that override is
installed matters to every other call in the function, because of the veto in
`ActionActiveParam` above: a trial is kept only while its value is used by
nothing but its call, and a use by another call counts against it unless that
call is still recovering its own arguments and has not taken the value. An
open printf that claims a leftover stack slot as a phantom therefore also keeps
the slot away from the calls after it. Closed from the start, the resolved call
claims nothing past its declared arguments, and the slot goes to whichever open
call is scored next: in gnulib's `version_etc` the `va_list` in the lowest
outgoing stack slots became three extra arguments of each `"Written by ..."`
call whose format is not resolved. With the call closed, a local that clang
keeps in its `push rax` slot, the first outgoing argument slot, was also folded
across a `sscanf("%d", &a)` to the value stored before the call.

So the override is installed as its declared arguments followed by `...`
(`first_var_arg_slot` is the declared count; the flow build marks the call point
in the `Override` store and the call spec records the count). The call is then
offered the same trials past its declared arguments that it is offered without
the override, they are scored the same way, and the calls around it see the same
competition. Two steps in
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_formattail.rs` finish the
call. When it finalizes, `keep_declared_trials` marks every declared
(fixed-position) trial active and used, because `fillinMap`'s positional rules
can end a list at a gap the ABI makes on purpose: a ninth `double` goes on the
stack behind the four integer registers a `"%f ... %d"` call leaves unused, and
the chain rule reads that run as the end of the arguments. Then, once no live
call in the function has trials left, `shed_format_tails` removes every input
past the declared ones and clears the `...`. It runs at the end of the
`ActionActiveParam` pass that finalizes the last call, before the
`calleearity` retries, so those see the declared list. The call spec's
recorded storage is truncated to match.

The effect is that a resolved format call changes its own arguments and types,
and a neighbour only where a declared argument, now certain, vetoes the same
value at that neighbour. Calls without a resolved format never take this path.

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

### (kuna) `protoorder` — callee-first prototypes

Every prototype a caller can read about its callee is a **declared** one.
`TypeOpCall::getInputLocal`
(`decompiler/crates/kuna-decomp/src/p5_types/coreaction_infertypes.rs`) already
types a call argument from the callee's parameter, and `ActionDefaultParams`
already gives a call site its callee's whole signature — but both read the
`PrototypePieces` parked on the callee's `FunctionSymbol`, and the writers of
that slot are all statements of fact from outside the decompile: a libc table
entry, a DWARF or demangled signature, a console `parse line extern`, a CLI
`--assert prototype`. What a *recovery* found about a function has never been
written anywhere a caller reads, so an internal callee tells its callers nothing
and each caller types the call from its own local evidence alone. That is the
shape of `fmt -O2`: the callee at `0x3700` renders
`unsigned long sub_3700(FILE *a0, unsigned long a1)` where the ground truth is
`int fmt (FILE *fp, char const *file)`, and the `char *` its own callees already
proved never arrives.

The option closes that gap without adding a pass. `kuna decompile-all` already
loads and analyzes once and then loops over the entries; with the option on it
orders that loop by the program's call graph — the `kuna_analysis::listing::xrefs`
edges `kuna xrefs` answers with, not a second graph — so each callee is
decompiled before its callers, and after each function completes what its own
recovery found is recorded for the callers still ahead of it
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_protoorder.rs`). Results are
buffered and emitted in address order, so the ORDER of the run changes and the
order of the output does not.

Ordering is Tarjan's strongly-connected components over the direct-call edges,
whose output order is already reverse-topological. A component with more than one
member — and a function that calls itself — is recursion, where "callees first"
has no meaning. Under `types` and `lock` those functions decompile with nothing
stated; under `cycles` (the default, below) they state their types in an order of
the driver's own.

The callee-first loop ends with the same `structsynth` convergence sweep as an
address-order batch (chapter [00](00-overview.md), synthesized structures across
a batch): the results that name a structure a later, larger one superseded are
decompiled once more (`decompiler/crates/kuna-cli/src/decompile_all/callee_first.rs
(converge_callee_first)`). The redo walks the same plan, callees first, and each
function states its recovered types again where the plan let it, so a callee
moved onto the surviving structure states that one before its redone callers
read it. Without the sweep, a function kept the structure it was first given
even where the surviving one was in reach, which an address-order run would
have replaced. `lock` runs no sweep: a prototype it parked is declared, and a
second decompile would read that function's own first answer back.

Before it decompiles anything again, the sweep forgets every stated type list
that names a superseded structure, at any pointer depth
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_protoorder.rs
(forget_statements_naming)`). A redone callee states again before its redone
callers read it, so a callee planned first loses nothing, and a statement that is
still stale when a redo reaches the call is not read at all. Under `types` a
function's callees are planned before it (a call the graph misses is the
exception), so on the corpus measured this changes nothing there. Under `cycles`
it is what keeps a redo from reading a stale answer made after its own first
decompile: a function that calls itself, and a cycle member that calls a partner
planned after it, would otherwise type the argument with the structure the redo
exists to replace. On tar -O2 the recursive `make_hol` (`sub_3b0c0`) was redone
onto the survivor `struct_59 *a0` and still passed its child as
`(struct_54 *)*v22`, from its own first answer. A later partner's statement that
names no superseded structure is still read by the redo: it states a current
type, as any callee's does. A function is never offered its own statement
(`seed_protoorder_types`), so its call to itself reads nothing on any pass.
The types a function's callers stated for it (`calleevote`, below) are forgotten
input by input rather than as a whole: an input whose stated type names a
superseded structure loses its statement, and the function's other inputs keep
what their callers passed
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_calleevote.rs (forget_naming)`).
Forgetting the whole list made tar's `exclude_add_pattern_buffer (struct exclude
*, char *)`, redone for its record, print the buffer its callers pass as `char *`
as `unsigned long`. On the 45 castbench binaries at the defaults no function is
redone with such a list, so the output there is unchanged; `fieldtype on`, which
moves which records are superseded, reached the tar case.

The option has three live values, because there are two different things a
recovered prototype can be asked to say and only one of them is safe to say by
default, and one question the safe one can answer two ways: whether a function
in a recursive component says anything at all.

#### `types` — the callee's parameter types, never its arity

A recovered prototype is not a fact about the program; it is a summary of what
one decompile managed to recover, and recovery is wrong in both directions. It
under-counts on a variadic and on any function that forwards its arguments
without naming them; it over-counts wherever an argument register an earlier call
left live looked like a parameter. The types it recovered are a judgement about
values the program really passes. The COUNT it recovered is a claim about the
program's shape, and stating a wrong one rewrites what the emitted C says the
machine code does.

So `types`, like the default `cycles` below, states only the parameter types. Nothing is written to the
symbol table: the recovered parameter types and the storage they were recovered
in go into `Architecture::kuna_protoorder_types`, keyed by the callee's entry.
The type-inference seam that reads them cannot reach the `Architecture`
(`Funcdata::get_arch` is the per-function `ArchHandle`), so they are copied onto
each caller's `Funcdata` once after its flow build — the bridge `calleedeadarg`
and `rustabi` already take for their own callee-body probes, at the same two
points in `decompile_drive.rs`.

At the call site, argument `i` takes the callee's recovered type for parameter
`i` when the two recoveries agree about **where that argument lives**. The
caller's side of that agreement is `FuncCallSpecs::final_input_storage`, which
`build_input_from_trials` records as it writes the argument list — the storage
each recovered argument was passed in, in argument order, kept because the
written inputs hold values and not locations. Position and storage must both
agree: position alone would re-bind every later type whenever the two recoveries
disagree about how many arguments there are, and storage alone would let a
register the convention reuses carry a type across a slot boundary.

What arrives is a **vote** about the value, not a declaration the value must be
converted to. `Varnode::getLocalType` folds it against every other reader of the
Varnode by `Datatype::type_order`, type propagation afterwards replaces it with
any strictly more specific type the value carries, and a declared (type-locked)
parameter answers before it is asked. Casts are measured against the declared
answer only (`declared_input_type_local`): the vote is never the type a call
argument is *required* to have, so where it loses, the argument renders exactly
as it does with the option off — `sub_16da9(stderr)`, not
`sub_16da9((long)stderr)` — and where it wins, the value already carries it.
Winning still changes spellings, so a changed function has to be read rather
than assumed cosmetic: a constant the vote types a pointer prints with its cast
(`caller((unsigned char *)0x402000,3)`); a character pointee guessed for a
pointer can split one wide constant store through it into character stores of
the same bytes — the same memory contents at a different access width, which
matters on memory-mapped I/O, and printed as `builtin_strncpy` where the bytes
are text (a pointee of any other primitive type narrower than a constant
stored through it at a fixed place is refused, below); a pointee guessed for a
pointer can widen a narrow load into a read of the
wider element and a truncation (`*(short *)(a0 + 0xc)` becomes `(short)a0[3]`,
`*(char *)&v4[2]` becomes `(char)v4[2]`) — the same value, read at a different
width, again a difference on memory-mapped I/O; and an unsigned vote can make a caller's parameter
unsigned, with casts keeping its signed uses correct (`(int)a0 >> 2`).
"More specific" is the lattice's order, not a judgement of quality: a pointer
outranks an integer whatever the integer's source, so the fold alone would let a
recovered pointer overwrite an integer the caller had right. That is why a vote
is refused outright wherever the caller holds evidence the fold cannot weigh
(`kuna_protoorder::call_argument_vote`):

- **The argument is the address of a frame object**, a value computed from the
  stack pointer. `gatherOpen` turns a pointer's pointee at a frame address into
  a range hint, so a vote there re-lays the frame instead of typing a value: a
  `char buf[256]` whose elements are handed to `int *` and `short *` callees
  rendered as `char [20]` plus `unsigned int [61]` while the loop still wrote all
  256 bytes into the first.
- **The value is frame memory or declared.** The value is taken as its family:
  every varnode joined to the argument by COPY, CAST, MULTIEQUAL or INDIRECT in
  either direction — the edges type propagation crosses, and at -O0 the stack
  slot's phi-nodes that one parameter is reloaded through for every call. The
  vote is refused when a member is type-locked (a declared parameter) or global
  (`is_persist`); is itself a frame address (a phi that also carries `&buf`,
  through which the vote would reach the frame after all: dash's `int pip[2]`
  turned into a `long` read as `(int)v15`); is loaded from or stored to memory
  through a frame address; starts at a frame address the function takes; lies
  inside a stack struct or array the frame already holds; or lies in an indexed
  frame region: at or above a frame address the function indexes with a
  non-constant, below the next frame address it takes. That region is the open
  range `gatherOpen` builds an array from, and one typed element inside it cuts
  the array short: firmware's `char *argv[13]` split into a scalar, an
  `int [10]` indexed with `v4 - 1`, and a tail local nothing writes.
- **The value is loaded through itself** — `p = *p`, `p = p->next` — which is
  `T == ptr(T)`, the equation `ptrdepthcap` exists for: no finite type satisfies
  it, and a pointer vote seeds it, so every inference pass adds a level (tar's
  regex routines rendered `uint8 *******a0`). A vote deeper than the inferred
  cap `kuna_ptrdepth::MAX_INFERRED_PTR_DEPTH` is refused for the same reason.
- **A pointer vote lands on a constant inside a function's code.** A Thumb
  function address handed over as data — `target|1` — resolves, through the
  same global container lookup `ActionConstantPtr` uses, to the function's own
  symbol and prints as `&sub_8130[1]`: a subscript of a function, which is not
  C. The literal stays a number.
- **Another call reads or writes the same value as a different kind of thing**
  — pointer, integer or float — through a declared parameter, a declared return,
  or another callee's stated parameter. This is coreutils `tail_bytes`:
  `dump_remainder` recovers its byte count as `void *` (it is compared with the
  constant 0x2000, which is also the address `_DT_INIT`), `lseek` declares the
  same value `off_t`, and the pointer, had it won, would have reached
  `end_pos = stats.st_size`, split `struct stat` in two, and left the output
  reading an `st_blksize` local nothing writes.
- **Under `structheadless`, the vote is a synthesized record and a declared call
  gives the value a pointee.** When a call that returns the family's value, or
  takes it, is declared with a pointer to anything but `void` that the
  synthesizer did not mint -- a named record (`libctypes`' `group`, a DWARF
  struct), a `char *`, a `char **` -- a callee's `struct_N *` is refused
  (`kuna_structheadless::yields_to_a_declared_pointer`). shadow's `newgrp` holds
  `getgrnam`'s `group *` and hands it to a function that reads the group past its
  start, and coreutils `tail`'s `main` hands `getopt_long` the `char **argv` it
  also passes to `parse_obsolete_option`, which reads only `argv[1]` and
  `argv[2]`; either callee's own record otherwise retyped the caller's variable
  (`struct_2 *group`, `struct_19 *argv`). The rule is gated on the option
  because headless records are what made it reachable on the campaign corpus.
- **Under `structheadless`, the vote is a headless record that types as a word
  a member the caller reads through.** A load through the family at an offset
  where a record synthesized from reads past its start has a pointer-width
  integer or undefined word, whose value the function then loads or stores
  through, refuses the record (`kuna_structheadless::types_a_pointer_as_a_word`,
  checked where `pointee_refuses` walks the members). Such a record is another
  reader's partial view that only moved or compared the member: `tar`'s
  `wsnode_remove` copies `ws_head` as a word, and `wordsplit_varexp`, which walks
  the node list from it, took that record at the call it makes and declared its
  node pointer `int8`. The refusal is kept to headless records: applied to every
  synthesized record it also refused offset-0 records a function was right to
  take (grep `-O2` `kwsprep` printed eight more casts).
  The same option adds the one fallback in this list: a callee's headless
  record refused for any reason is offered again as `void *`, held to every
  refusal above (`kuna_structheadless::bare_pointer_for`). A headless record is
  the callee's partial view of what it was handed, and the caller's reads often
  disagree with it -- five of `fts_build`'s callees each read their own part of
  one `FTSENT` -- so refusing it outright left the caller's value, which the
  same callees state as `void *` when the option is off, with no pointer
  evidence at all: `v21 = a0->field_0x0` turned from `void *` into `long`. Any
  other refused record is refused with the option off too, and says nothing.
- **Float-ness disagrees**: the family is produced or read by a float op and the
  vote is not a float; a pointer vote on a value the caller multiplies, divides,
  shifts, masks or reads as a float; or an integer or pointer vote in a
  float-class argument register. `fabsf` recovered as `unsigned int` (it masks
  the sign bit) would otherwise print `(unsigned int)v7` on a float — a value
  conversion spelling a bit reinterpretation.
- **A float vote meets anything but a float or a copy of the bits.** Every other
  reader and writer of the family has to be a float op or pass the bits through
  untouched — a copy, a phi, a load. The vote is refused when any integer op
  computes with the family, including the ones a pointer vote ignores (addition,
  subtraction, the comparisons, the carry and borrow tests, zero or sign
  extension, truncation or byte extraction, concatenation), because a pointer is
  added to, compared and truncated but a float is not. It is also refused when
  the family is stored, or — outside a register the calling convention assigns
  to floats — returned by a function whose result is not declared a float,
  passed to another call that states no class for it, or produced by a call
  whose result is not declared. Each hands the value to a type decided
  elsewhere, and the printer bridges a float to an integer there with a value
  conversion; a float register is itself the float declaration, so `s0` on
  hard-float ARM or `xmm0` on x86-64 keeps the vote (the fixture
  `protoorder_floatreg_armhf.o` prints `1000.0`, not its bits `0x447a0000`).
  The case is an ABI that
  passes a float in a general register (MIPS
  o32, ARM soft-float, RISC-V ilp32): the caller hands a word's bits to a
  `float` parameter and also adds, compares, truncates or stores the same word as
  an integer, and the float vote printed `(int)v1 + 3`, `v1 == 1.5000001`,
  `(short)((unsigned int)v1 >> 0x10)` and `a2[1] = (int)v1` — value conversions
  where the machine works on bits, which compute different numbers (the fixture
  `protoorder_floatgpr_mipsel`). The store is not particular to those ABIs: on
  x86-64 a float loaded, passed in `xmm0` and stored beside an int in a struct
  kuna types as `int *` printed `v1[1] = (int)v3` (the fixture
  `protoorder_floatstore_x86_64`). Two more sources refuse a float vote: a
  constant whose bits are a NaN, because every NaN prints as `NAN` whatever its
  payload, and a parameter of the caller's own in a register the convention
  would not give a float at that position. The model answers that question
  rather than a register table: a float after an integer goes to `xmm0` on SysV,
  so `rsi` never holds one, while MIPS o32 passes it in `a1`.
- **What the caller does through a pointer disagrees with the pointee.** A
  pointer vote whose pointee is a float, a structure, an array or a union is
  checked against every load and store the caller makes through the value's
  family, following constant offsets, indexing (the stride of a multiply, a
  shift or a `PTRADD`), a pointer stepped around a loop and copies. An access
  outside a composite pointee refuses the vote, and so does an address the
  caller derives outside it: a callee's `struct_N` is only the part of the
  object that callee touched, and a caller that reads offset 0x19c of it printed
  `*(int **)&a0[0x33].field_0x4`, the array subscript of a structure that is not
  an array. So does a pointer the caller steps or indexes by other than exactly
  one element: grep's loop over 0x48-byte records printed `a0 = (unsigned long
  *)&a0[4].field_0x8` for a callee's 16-byte view, and sort's 32-byte records
  printed `v18[-2].field_0x8`. An index into an array member inside the
  structure is refused the same way, which costs a vote and prints what main
  prints. A pointer member the caller loads or stores through the pointee is
  checked against the caller's uses of it in turn, two levels deep. When the
  value itself was loaded from memory, every other load of the same field (the
  same base, offset and width) is checked with it: the vote types the field it
  was loaded from, and the field types those loads, so e2fsck's `getblk` printed
  `*(int8 *)&v2[4].field_0x4c[0x1c]` for a `v2 = a0->field_0x0` the call never
  saw. An access that is not exactly one member, whether it spans two
  members or part of one, refuses it: a 4-byte load over four byte members
  printed as four piece assignments (`v6._0_1_ = a4[1].field_0x0; ...`), and an
  8-byte struct copy over an `int` and a `float` was split into two 4-byte
  stores. An access landing on a float element or member is then held to the
  float-vote rule one level down: the value stored there, or the value loaded
  from there, is refused if any integer op computes with it, if it is a NaN
  constant or a caller parameter in a register no float arrives in, if it is
  handed on outside a float register, or, for a stored value, if it was loaded
  through another pointer and nothing reads it as a float. Without that, integer
  bits moved into memory a callee reads as floats printed as value conversions:
  `*a0 = (double)(a1 + 1)` for `u->l[0] = v + 1` through a union a callee sums
  as doubles, `a0[1] = NAN` for a signalling NaN's bits, `a0->field_0x4 =
  (float)v2` for a plain `*dst = *src`, and `double` parameters in `rsi` and
  `rdx` for a `memcpy` from two `long`s (the fixture
  `protoorder_floatpointee_x86_64`, whose six callers are compiled from the
  printed C and compared with the source). A pointee that is a non-character
  integer, a `bool` or a pointer refuses any constant the caller stores through
  the family wider than itself, wherever it lands: `SplitDatatype` reads such a
  pointer as an array of its pointee and splits a constant store into one store
  per element, so a callee's `unsigned char *` printed gzip's `".tar"` suffix as
  five byte stores, betaflight's `1.0f` in a 0x14-byte record as four and, with
  `ptrfromuse` on, bzip2's field write `*(unsigned long *)(a0 + 0x5c) = 0x100`
  as eight (the fixture `protoorder_narrowvote_x86_64` stores `"ustar  "` the
  same way). A buffer filled a word at a time is no exception: betaflight's
  sector fill `*(unsigned int *)(a1 + v1 * 4) = 0xefbeadde` printed as four byte
  stores per word, and a loop storing an eight-byte constant per word as eight
  (`fill_words` in `protoorder_widefill_x86_64`). A store of a computed value
  of another width prints a cast, not a conversion, and is not checked. Neither
  is a character pointee: the byte stores it produces are what the string-copy
  idiom prints as `builtin_strncpy`. The walk over the addresses derived from
  the family gives up after 511 steps, and every pointee check refuses the vote
  when it does, rather than taking one it could not check: `fill_many` in
  the same fixture stores 520 eight-byte constants, and a vote let through
  there printed 4,160 byte stores.

The callee's recovered RETURN type is not stated. It has no competing evidence at
the caller — the call's result is a new value — so a wrong one spreads through
every comparison it meets into the caller's own parameters: expr's `mbslen`
recovered as `char *` turned both `size_t` parameters of its caller into
`char *` and printed `&v3[1 - (long)a1]`. Over the 444-slice campaign corpus it
was worth one perfect function and accounted for four of the eight that scored
worse, `grep`'s `memchr_kwset` family among them.

No call spec is input-locked, so `ActionFuncLink` runs the caller's own argument
recovery at every call exactly as it does with the option off, and nothing about
the call's shape can move by this option. The one rule that reads a stated list as
evidence about arity is `argclobber` (above, on by default), which drops a
trailing argument only where the callee's stated list and its body both prove the
register free; the counts that follow were taken before it was on. That the
option itself moves no arity is checkable rather than merely arguable, and it
is checked: over 46 stripped binaries (25,038 functions, x86-64 userland
at -O0, -O2 and -O2-noinline plus ARM Cortex-M firmware) the two arms render
**the same 209,487 call arguments**, no function gains or loses a
`variables[]` argument row, no call is made or lost, and no `goto` or `return`
moves (`docs/features/protoorder/callsite.json`, `invariants.json`). Every
changed function is classified by `corpus-diff.py`, which also checks the frame
of every one of them for a split or merged stack object and for a stack local the
output reads without writing (`corpus-report.json`, `analysis.md`).

It is also why this value is cheap. Parking a prototype bumps
`Database::kuna_generation`, which drops the memoized
`build_callee_proto_pieces` snapshot that every later function rebuilds; this
table is the module's own and the symbol table does not move.

What the vote can still get wrong is a type the callee's own recovery got wrong,
where nothing at the call site contradicts it. Over the 444-slice campaign corpus
that costs three functions a lower score against 851 that gain, and
none that leave a perfect score against 115 that reach one.

#### `cycles` (the default) — recursive functions state their types too

Under `types`, a function in a recursive component states nothing, and that
stops the chain at the first recursion. Much of the character-level work in
coreutils and gnulib bottoms out in recursive functions — `quotearg_buffer_restyled`
calls itself, `copy_internal` and `copy_dir` call each other — so on cp -O0 the
quoting helper is recovered as taking `char *`, and every function that hands it a
file name keeps an integer: `emit_verbose` renders
`void sub_a6db(unsigned long a0,unsigned long a1,long a2)`.

`cycles` is `types` with one change: a member of a recursive component states
its recovered types too, as every other function does, through the same table
and the same refusals. What stays open is the order, because inside a cycle there
is no callee-first one (`decompiler/crates/kuna-cli/src/callgraph/plan.rs
(plan_from_components)`). A function that only calls itself is decompiled once,
like any other function, and its own call to itself reads nothing, in the
`structsynth` sweep's redo as well (above). The members of
a larger component are decompiled once each in a depth-first order over the
component's own edges (`cycle_order`), which emits each member after the partners
it reaches and starts from the members something outside the component calls.
Each member states its types as it finishes, so a member decompiled later reads
every partner decompiled before it, and a member called from outside — the one
the component's callers read — is decompiled after the partners it reaches. A
member decompiled before a partner it calls does not read that partner's
statement: what it states was recovered from its own body and the callees outside
the component, which is the same evidence a function whose callees all declined
has. The component's callers come after every member, so they read all of them.
The order is a function of the program alone (roots and edges in address order),
so the output is deterministic. On cp -O0 `emit_verbose` becomes
`void sub_a6db(char *a0,char *a1,char *a2)`.

A second round — decompiling each member that called a later partner once more,
after the whole component, so it too reads that partner — was measured and is not
taken: over the 444-slice corpus it adds 0.39 to the aggregate `type_match` (5
more functions improve, no more reach a perfect score), and on bash -O2 it costs
about +35%, because the parser and the command executor are the two largest members of
one component and each is decompiled twice.

`lock` keeps declining a recursive component: a parked prototype is declared, so
a member decompiled first would lock its partners' calls to a list recovered
without them, and a redone member would read its own first answer back.

`cycles` locks nothing, exactly as `types` does, so no call gains an argument and
no call spec is decided by a statement. One arity can move, and only by
`argclobber` (above, on by default), which reads the stated lists: a call to a
recursive callee can now lose a trailing clobbered argument where it could not
before, and only where the callee's stated list accounts for every surviving
argument and its body neither reads nor forwards the register. The body walk is
what answers for a member whose recovery is short because it hands a register on
to a partner: `resolve_forward_transfer` follows the partner's body, and a cycle it
re-enters proves nothing, so a register the callee has not written before its
recursive call can still reach a read and the drop is declined. The fixture
`protoorder_cycles_x86_64` shows both sides: `rtarget` writes `rdx` before calling
itself and its caller's clobbered third argument is dropped, `rkeep` forwards
`rdx` into its own recursion and its caller keeps it; C counterexamples where a
recursive forwarding thunk, and a two-member cycle feeding one, recover short keep
the argument too (`docs/features/protoscc/`).

Measured against `types` over the 444-slice campaign corpus: 46 functions reach a
perfect `type_match` and none leave it, 116 more improve, and one gets worse
(coreutils -O2 `install_file_in_file`, whose `to_relname` takes `stat *` from
`copy_internal`'s own recovery of the same argument — a wrong recovered type, the
known cost of any vote). The class that moves is `char *`. Over fifteen binaries
(x86-64 -O0, -O2 and -O2-noinline userland and three ARM Cortex-M images) both
values render the same 90,420 call arguments and 18,556 argument rows, no call is
made or lost, no `goto` or `return` moves, and no stack object is split, merged
or read without a write (`docs/features/protoscc/`).

#### `lock` — the arity claim, opt-in

The other value parks the recovered prototype in the symbol table, where
`ActionDefaultParams` reads a declared one from. That does decide the call's
arity, which is the point where a caller over-recovered — `ext2fs_mmp_start`
renders three arguments and takes one — and a defect where the callee
under-recovered. It gains 818 call arguments at 418 sites over the x86-64 half of
the corpus above and loses 28 at 21. It also FABRICATES parameters: 155 of 7,553
x86-64 functions and 630 of 6,589 ARM Cortex-M ones gain an argument row nothing
sets, which `type_match` cannot see at all, because a decompiled variable with no
ground-truth counterpart is never a false positive. It is therefore never the
default; the rest of this section is what makes it safe enough to offer at all.

The under-count is the dangerous direction, and it is dangerous because of what a
lock does. `ActionFuncLink::func_link_input` turns on a call site's own argument
recovery only when the call spec is *not* input-locked; a locked spec is answered
entirely from the parked list. So a callee parked with a list shorter than the
arguments its callers really pass does not merely fail to describe them — it
**deletes** them from the emitted call. Measured over twelve stripped binaries, a
closed parked list deleted 639 arguments at 216 call sites.

The shape of the park is what answers it: **the parked list is a floor, not a
ceiling.** The pieces name the first slot past the recovered parameters as the
start of a variable tail (`PrototypePieces::first_var_arg_slot`), which is the
one shape `func_link_input` treats as locked *and* input-active (`if !inputlocked
|| varargs`). The recovered types bind the slots that were recovered and the
caller's own recovery still runs for everything past them, so no call is
truncated to the parked list any more; over the same twelve binaries that class
goes from 639 arguments at 216 sites to zero.

It does not follow that an argument can no longer be lost. A parked prototype
still changes the caller's own dataflow, because every parameter it states is a
register the call site now reads: state a parameter the callee does not have and
the caller materialises a live-in that is nothing, which competes with the
caller's ordinary argument recovery — at a *different* call, earlier in the same
body, to a callee that has no prototype of its own. That is the residual: 28
arguments at 21 sites over twenty x86-64 userland binaries, each enumerated in
`docs/features/protoorder/deleted-arguments.json`. The open tail has a second
cost besides: a list with a variable tail makes the call spec variadic, and
`FuncProto::characterize_as_input_param` sends a variadic spec straight to the
model, so every argument register the convention has becomes a candidate at that
site again and a caller holding a live value in one of them renders it as an
argument the machine code never passes. How much that costs is a property of the
convention — AAPCS has four argument registers and firmware keeps all four busy,
which is why the ARM column is five times the x86-64 one. On an ARM image that
states the VFP variant a variable tail would also move a floating-point return
value to the core registers, so there the parked pieces name the return storage
themselves (see "Declared variadic prototypes on a VFP image").

The over-count is answered with evidence rather than with shape, because there is
no shape that expresses "this parameter may not exist". Both rules read the
callee's own machine code, through the entry-liveness walk `calleedeadarg`
already takes (`probe_callee_entry_dead`), which answers one question per
register range one-sidedly: does every path from the entry write these bytes
before reading them, or does some path read them first?

- A **trailing** parameter is stated only when the body reads the register it
  would arrive in, for a value that can reach something (the register-zeroing
  idiom `xor esi,esi` is a read in the p-code and not a use). The burden of
  proof is on the tail, not on the trim: a parameter the body neither reads nor
  writes is only *possible*, and `save_cwd` in `findutils/find -O2` — one
  parameter in the DWARF — is recovered with three exactly that way, because a
  nested call reads an `rdx` the body never writes. Under-stating a tail is the
  cheap direction and only because the parked list is a floor: the caller still
  passes what it passes, so the cost is a type on that slot, where an
  over-stated tail fabricates an argument and disturbs the caller's other
  calls. A stack parameter is dropped for the same reason — the entry walk
  cannot speak about it at all. Only the tail is trimmed: removing an interior
  parameter would leave a hole the convention re-packs, silently re-binding
  every later type.
- A callee that provably **reads** the register its next argument would arrive
  in — the storage the model assigns to one more parameter than were recovered —
  has under-recovered, and is declined outright. This is the rule that catches
  what the variadic test cannot: `FuncProto::is_dotdotdot` is only ever set from
  a *declared* signature, and a declared entry never reaches the policy at all
  (the declared check fires one branch earlier), so on a stripped image it can
  never fire. A SysV register-save prologue reads every argument register there
  is, and that read is visible in the body whether or not anything declared the
  function variadic.

Everything the walk cannot see — an incomplete walk, a register space it cannot
name, a body that is one endless loop — answers `false` to both, so both rules
decline to act rather than guess.

One case the walk cannot see at all is the one where it matters most. Asking
whether the callee reads its *next* argument register presumes there is a next
register to ask about. A recovered list that fills the convention's argument
registers — six on SysV, four on AAPCS, eight on AArch64 — has its next slot on
the stack, at an offset the entry walk never speaks about, so the question comes
back `false` for a variadic exactly as it does for a real six-argument function.
That is not evidence; it is the absence of a witness, and the population sitting
on that boundary is not neutral: a variadic that calls `va_start` spills the
argument registers past its named ones into the register save area, so every one
of them is read before it is written and recovery reports exactly as many
parameters as the convention has registers. So **a recovered list that ends on
the convention's last argument register is declined**, asked of the model (the
last parameter must be in a register and the next slot must not be) so that a
convention passing everything on the stack never trips it. The cost is the
genuine six-argument SysV function and the genuine four-argument AAPCS one,
which keep the option's off behaviour at their call sites.

None of these four rules applies under `types`, where a stated list cannot
fabricate an argument at all: a short list states fewer types, and a long one
states types for slots the caller does not have, which match no argument.

Closing the tail where the entry walk decoded the callee's whole body — every
path ending at a `RETURN`, so "no other argument register is read" is a
statement about the body rather than about its first few instructions — was
measured and rejected: over twenty-two binaries it left the fabricated-parameter
counters unmoved (785 functions gaining `variables[]` argument rows either way)
and deleted nineteen more arguments. The callees that fabricate are the ones
that call something, which is exactly where that walk stops.

#### What every value declines

Every value declines when

- the function's decompile errored, or left no parameter store;
- it says nothing: no parameters and no return value to state. Under `types`
  and `cycles` the return value is recorded only while `passthrough` is on, and
  then a function with no parameters states that value alone: an empty list is
  no arity claim (`arity_sound` is false), so the only reader it reaches is
  `passthrough`'s tail-call arm;
- model selection did not settle on a known model;
- a parameter is a hidden return pointer, an indirect-storage parameter or a
  `this` pointer, or carries no type or no real storage;
- under `types` and `lock`, the function is in a recursive component (a
  call-graph cycle of more than one member, or a function that calls itself);
- a prototype is already **declared** for that entry. A `--assert prototype`, a
  libc/`libctypes` signature, a DWARF (`cppproto`) or demangled (`cppsig`) one
  are all stated facts that outrank anything recovery found — and under `types`
  a type-locked parameter answers before the recovered vote in any case.

`lock` declines one thing more: **recovered parameters that are not where the
convention would put those types**. A parked prototype is re-bound from the model
(`set_pieces` → `update_all_types` → `assign_parameter_storage`), so a prototype
the model would re-bind cannot be parked faithfully at all. This catches the
variadic whose register-save prologue also spilled the XMM registers: recovery
reports fourteen parameters, the model would put the last eight on the stack, and
the mismatch declines it. `types` does not need the rule, because it round-trips
nothing through the model: it matches the recovered storage against the caller's
own recovered storage directly.

`types` states no return type at all. Under `lock`, a recovered **void return**
is never stated: `set_pieces` output-locks whatever the pieces name and
`call_output_type_local` then removes the call's result at every site, so a
caller reading the return register would lose the definition and go on reading a
value nothing writes. A recovered non-void return is parked, where it can only
type a result the caller already has.

Under `lock`, parameter names are carried as recovered (`a0`, `a1` on a stripped
image), which is what the callee's own decompile chose; on a `-g` binary that is
the DWARF name, and a caller's local can take it. `types` carries no names.

Two consequences are worth stating plainly. What is stated is a **caller-side
fact only**: the locked-input branch of `ActionPrototypeTypes` is stubbed, and
the callee has already been decompiled when its types are recorded, so nothing
re-reads it about itself. And `kuna decompile` — which forks one `decomp_dbg` per
function — cannot see what another function's decompile stated, so the two
surfaces may disagree about a call's argument types; the whole-binary surface is
the one that has the callee. `decompile-project` takes the same order, because
it keeps a `structsynth` ledger too and the ledger numbers a layout in visit
order: an export on its own schedule put a different record under the same
`struct_N` than `decompile-all` did (chapter
[00](00-overview.md), synthesized structures across a batch). A streamed export
and `decompile-graph` have their own schedules and are not callee-first; both
warn on stderr when the option is asked for explicitly, rather than producing
identical output silently.

A run that selects part of the program — `--functions`, `--addr`, a triage
filter — or a single function decompiles in address order and builds no call
graph, since the callees it would order are mostly not in it; naming the option
explicitly orders the selection anyway and says on stderr that a callee outside
it states nothing. An explicit `--option protoorder` is refused alongside
`--jobs N`, exactly like `--assert`: each worker process loads the binary into
its own `Architecture`, so a statement could not cross a chunk boundary and the
output would depend on how the chunks fell. A `--jobs` run under the default is
not refused; it states nothing and says so on stderr, since its call-argument
types can then differ from the serial run's, and `--option protoorder off` on
both makes them byte-identical.

### (kuna) `passthrough` — the register a function forwards to a callee that reads it

(kuna) `passthrough` (default on,
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_passthrough.rs`) reads the
statement `protoorder` makes in the one direction `types` never takes: it lets a
callee's recovered parameter list add an argument at a call site, and so a
parameter to the function making the call.

**The gap.** A call's argument trials exist only for storage heritage visits
(§4.2, *Trials are populated by heritage*), and heritage visits a range only when
some op of the function reads or writes it. A function that hands its own
incoming register straight to a callee names that register nowhere: gzip -O2
`gzip_base_name` is `endbr64; jmp last_component`, the call gets no `rdi` trial,
and kuna printed `void sub_d290(void) { sub_dfd0(); }` while printing the callee
in the same output as `char *sub_dfd0(char *a0)`. coreutils df `dir_name`
(`call mdir_name; test %rax,%rax; ...`) is the same shape without the tail call.
Where the register *is* heritaged — the function also reads it — upstream's
`AncestorRealistic::execute` refuses a trial whose Varnode is the function's own
input outright, because it expects to see a value moved into the register.

**The evidence.** Only the callee can say it wanted the register. A register `R`
at a direct, unlocked, non-variadic CALL is an argument when all of these hold:

- the callee's recovered prototype (`RecoveredTypes`, stated by `protoorder` for a
  callee decompiled first) has a parameter starting at `R`;
- that list is one `lock` would accept as an **arity claim**
  (`RecoveredTypes::arity_sound`, computed only while the option is on): every
  parameter is in a register, the callee's body does not read the register its
  next parameter would arrive in (a variadic's register-save prologue does,
  however much of it recovery kept — gnulib `rpl_fcntl(int,int,...)` recovers
  three parameters and saves three more registers), the list does not end on the
  last argument register, and the model would put those types where recovery
  found them. The next-slot read is asked without the register-zeroing idiom:
  `xor %esi,%esi` writes a constant;
- the callee's body reads `R` before writing it on some path, for a value that
  reaches something (`calleedeadarg`'s `proves_input`), so the parameter is not an
  artifact of the callee's own recovery. A callee that forwards by a call rather
  than a jump (`sub $8,%rsp; call narrow; add $8,%rsp; ret`, or a tail call the
  spec models as a call) reads nothing before that CALL, where the walk ends, so
  the read is also looked for in the target: `reads_through_calls` adds to the
  callee's reads what each direct call's target takes, for the register bytes
  nothing on the callee's path wrote before that call, following direct calls up
  to three deep. A target takes a read only where its own statement would pass
  this list's callee-side tests: an arity-sound list with a parameter the read
  overlaps, not as a variadic tail. Without that, a forwarder whose target calls
  a variadic function on an error path inherits the register-save prologue's
  8-byte reads of every argument register: dash's `or(int,union yystype *,int,
  int)` calls `and`, whose `binop` calls `sh_error(const char *,...)`, and
  `or`'s fourth parameter came out `unsigned long` with `(unsigned int)` casts at
  its uses. A read overlapping a byte written first is dropped whole, and so is
  one overlapping a byte the callee reads itself: that register is already
  proven an input, and a deeper read could only change how wide it is taken.
  openssh `channel_by_id` pushes all of `rsi` for a variadic `%d`, and without
  this `channel_send_open(ssh, int id)` and every other caller of
  `channel_lookup`, which reads `esi` itself, took a 64-bit `id`. A target the
  walk cannot fully cover, one that stated nothing, or an indirect call adds
  nothing. Only this check reads the widened set; `calleedeadarg`,
  `calleearitybody` and the arity claim above see the plain walk. The probes are
  cached for the run; the fold reads statements, which accumulate as the run
  goes, so it is memoized for one caller at a time, and a call cycle ends when
  the depth runs out. Over 266 stripped x86-64 -O2 binaries of 25 projects
  (92,275 functions) this gives 111 functions 179 parameters, every one with a
  DWARF twin confirmed (110; one has none), and 163 calls arguments their
  callee's DWARF prototype takes; nothing loses an argument. Parameter widths
  move toward DWARF's in 28 places and away in 2: gnulib `mkstemp_len` and
  `mkdtemp_len` hand their `size_t suff_len` to `gen_tempname_len`'s `int
  suffixlen`, and the argument is the `int` the callee reads;
- the caller did not set the call up as variadic. A register that carries no
  argument but the return value, written and not read again between the call and
  the call or block start before it, is SysV's vector-register count (`xor
  %eax,%eax`), and a variadic callee may state an optional tail register it saves
  (gnulib `open_safer(char const *,int,...)` saves only `rdx`);
- the calling function is not variadic itself: its entry block does not read that
  return-only register before writing it or calling anything (`test %al,%al`). Its
  own recovery would read every register its prologue saves, and one argument
  supplied here is enough to tip the saved tail into its list;
- the callee's read of `R` is for something other than a **variadic tail**
  (`kuna_varargtail.rs`). A stated parameter whose value, through
  value-preserving operations (copies, phis, width changes, a mask by a
  constant), only ever reaches an argument slot of a variadic call — one the
  callee set up as variadic, or a declared `...` prototype past its named
  parameters — is recorded on the statement and never claimed: the ABI lets a
  caller leave that register unset, so the callee's read of it says nothing
  about what its callers put there. openssh `xcalloc(size_t,size_t)` is
  recovered with a third parameter because the call to the variadic `sshfatal`
  is set up with `push %rdx; …; xor %eax,%eax`, and that push is gcc's
  stack-alignment filler; gnulib `open_safer` reads `rdx` only to pass it to
  `open`'s `mode`;
- where the function touches the register nowhere, so that this pass has to
  claim it into existence (*The claim*, below), the claim does not fill a
  **hole** (`no_hole_before`). Parameters are positional: a claim at the third
  stated register gives the function three parameters. Every earlier stated register must be one the function could be
  carrying — claimed here too, or one its own entry walk (the same
  `calleedeadarg` probe, asked of this function's entry) does not prove it
  WRITES before reading. Otherwise the register becomes a parameter nothing
  sets: the `protoorder` fixture's `overrec` sets `rsi` to 16 and forwards
  `rdx`, and rendered `void overrec(unsigned long *a0,unsigned long a1,long a2)`
  with `a1` in no statement, against the call site `overrec(v1)`;
- the value at the call is the function's own input Varnode for exactly the
  trial's storage.

The variadic-call test itself reads the writing instruction's own ops as its own:
`xor %eax,%eax` reads the register it zeroes, as its operand and again in `ZF =
(EAX == 0)`, so counting every read as a competing one made the test answer `no`
for the idiom gcc actually emits (0 calls recognized over ssh-keygen). A read at
the same instruction address as the write is that instruction's.

The argument is as wide as the callee's body reads it — the narrowest of the
trial, the stated parameter and the widest body read starting at `R` — so a
parameter recovered as `rdi` but read as `edi` is passed, and becomes the
caller's parameter, as an `int`, through a truncating `SUBPIECE`.

**The rule only adds.** Every other rule decides a call's argument list first,
exactly as with the option off, and `passthrough` extends the result. That order
is the design, not a detail: the list upstream scoring settles on is often
finished by a later rule — a call left empty because `onlyOpUse` rejected a
value that also feeds a sibling call is filled by `calleearityfwd` from that
sibling — and an argument supplied during scoring makes the list non-empty and
turns that rescue off. tar `sysinttostr` (`mov %rcx,%rsi; cmp %rdx,%rdi; ja;
jmp umaxtostr; jmp imaxtostr`) lost `buf` at its first tail call that way, and
the gnulib `xpalloc` copies lost `xrealloc`'s size. So the scoring of every
heritaged register is upstream's (the function's own input at a call slot is
*inactive*: a candidate for `forceInactiveChain` to fill a hole before a later
argument, never an argument on its own), and the pass acts twice outside it:

- `build_input_from_trials` first asks the pass which unused register trials
  stand on the function's own input Varnode for exactly the trial's storage and
  satisfy the evidence, and keeps those Varnodes;
- at the end of `ActionActiveParam::apply`, after `calleearityfwd`,
  `calleearitylive` and `calleearitybody`, each such call's final list is
  extended with the registers its callee's stated list names next, in order,
  stopping at the first one the function does not forward. The final list must
  be a leading run of the stated one, and a list holding a stack argument is
  left alone.

**Making the register visible.** At the end of `ActionFuncLink`, before the
first heritage, a register no Varnode of the function touches gets a trial and a
read on the CALL — the same thing the locked-prototype branch of `funcLinkInput`
does for a declared parameter — at each call whose callee satisfies the evidence
and that no other CALL or CALLIND can precede: the first call of a block every
path from the entry reaches without passing one. There heritage can only link
the read to the function's input; after another call the register may be that
call's clobber or its return value, and claiming it there gains nothing and
costs the claimed range's heritage everywhere else. The claim is recorded on the
`Funcdata`. Heritage then registers the visible range at every other call as it
would any heritaged range, argument and return-value trials alike, and only
`guard_returns` is kept off it, so the function gains no return value in a
claimed argument register. Registering rather than suppressing is what the
option-off run does one heritage pass later whenever a hole fill reads the
range, and it matters twice. An earlier call keeps the return value a later read
asks for: suppressing it cut a Cortex-M `double` returned in r0:r1 to r0.
And a later hole fill finds the trial already there instead of adding a read to
a range dead-code removal has visited, which forces a restart with a longer
dead-code delay and a different result.

A trial on a claimed range, at any call, is never scored: `check_input_trial_use`
marks it inactive, which is where the option-off run's trial for that slot
starts (an unreferenced one `fillinMap` adds to fill a hole before a later
argument, whose fresh Varnode heritage links to the same value). Scoring it
would be wrong in both directions. Marked active, it would pre-empt the rescues
above. Left to `AncestorRealistic`, a register that reaches the call through an
earlier call fails as "killed by call" and is marked definitely-not-used, and
`forceNoUse` then drops every argument after it: gcc -O2 does not re-save `rdi`
around a callee its IPA-RA knows leaves it alone, so `noop(); glob =
twoarg(p,3);` rendered `sub_11a0()` for `sub_11a0(a0,3)` and the function lost
`a0`. When the trial ends unused it is retired (marked definitely-not-used after
`fillinMap` has run, where that no longer propagates) before the sibling and body
rescues read the trials, so they see the candidates the option-off run gives
them and nothing more; the extension above then adds the register if the
function forwards it.

**The tail call's result.** When every live RETURN of the function is reached
from a direct CALL with no join, indirect call or user op in between (walking at
most four single-predecessor blocks back), every such callee states a non-void
return in the same register, the function's own output is not locked and no op
of the function touches that register, each RETURN gets a read of it and the
function's return trial is registered for it. Upstream's `ancestorOpUse` refuses
an INDIRECT creation at a RETURN ("an indication of an output trial"), so
`ActionReturnRecovery` marks the trial active when its Varnode is the creation
planted at one of those calls; that call keeps its return-value trial, and its
output takes the callee's recovered return type in `call_output_type_local`.
`gzip_base_name` becomes `char * sub_d290(char *a0) { return sub_dfd0(a0); }`.
This is the claim a declared callee already gets whenever the caller names the
return register; its cost is a void wrapper that tail-calls a value-returning
function, which is handed that value.

A callee with **no parameters** states its return the same way: `protoorder`
records the return value of a function whose list is empty, where it used to
decline the whole statement as saying nothing, so `int f(void) { return getv();
}` -- `jmp getv` -- no longer prints `void f(void) { getv(); }` beside `int
getv(void)` in the same output, on x86-64, ARM, AArch64, PowerPC or MIPS.

"No op touches the register" is asked of the whole function, but only the path
from each producing CALL to its RETURN can change what the RETURN reads; the
call overwrites the register, so a write before it does not. A register that
carries **no argument** under the function's own model (x86-64 and i386 `eax`,
MIPS `v0`) is therefore taken when it is written before the call -- gcc -O0
loads every argument through `rax`, and gcc -O2 `quotearg_buffer` saves `errno`
through it -- provided no op between the call and its RETURN reads or writes it
(`touched_after`, walking the same single-predecessor chain as
`producing_call`). Heritage then visits the wider range the function wrote, and
the RETURN's `eax` is the low SUBPIECE of the call's `rax` creation, which
`returns_tail_result` follows. An argument register is not relaxed: on
ARM `mov r0,#5; b g` is how a `void` function calls `g(5)` as often as how an
`int` one returns it. Over 24 debug-stripped decbench binaries (x86-64 O0, O2
and O2-noinline; ARM firmware at O2) the two changes give 126 functions a
return: DWARF confirms 97, says `void` for 4 and has no entry for 25; the
relaxation alone, tried on ARM `r0` as well, was confirmed for 31 of 97 there.
The claim takes the width the callee's recovery states, so two O0 wrappers that
`voidret` had returned as `unsigned int`, the width their callers read, now come
out `unsigned long` like their callees (gnulib `set_binary_mode` around a
`return 0;` the callee writes with `mov $0,%eax`). Taking a declared
callee's locked output the same way (`return strlen(s);`) was measured at 43 of
54 and is not done: a `void` wrapper ending in `putc` or `clock_gettime` is the
common shape there.

A return the callee's recovery put in a register pair (x86-64 `rdx:rax`, i386
`edx:eax`, ARM `r1:r0`) is stated as a join, and is claimed register by
register: every register of the pair must be one the function never touches,
each gets its own RETURN read and return trial, and return recovery joins the
trials again, as it joins the pair heritage would have registered. The high
register is not the first of its storage class, so `fillinMap` refuses a trial
for it that is formed by an INDIRECT creation; `ActionReturnRecovery` therefore
scores a claimed trial before the ancestor walk that sets that mark, and skips
the walk for it. Without this an x86-64 `jmp wide` to an `undefined16 wide(...)`
rendered `void fwd_wide(a0,a1) { wide(a0,a1); }`; it now renders `undefined16
fwd_wide(a0,a1) { return wide(a0,a1); }`, and a caller of `fwd_wide` reads the
pair it always read.

A pair is taken whole or not at all (`keep_tail_return_whole`, run once the
trials are scored for the last time): if any register of it was not accepted as
the call's result, none is. The x86-64 gcc model kills `rax`, `rdx` and `xmm0`
at a call but not `xmm1`, so the `xmm1` of a `struct { double, double }` return
reaches the RETURN through an ordinary INDIRECT, not the call's creation; keeping
`xmm0` alone rendered `unsigned long fwd_mkd(double a0,double a1)` -- half of the
callee's value, as an integer -- and spread to its callers. Such a forwarder
keeps the option-off `void`.

**A value the function computes itself is its return value.** The claim rests
on the function computing nothing of its own, and "no op touches the return
register" only says that for the claimed register. `float f(int x, float y) {
g(x); return y * 2.0f; }` leaves `g`'s result untouched in the integer return
register and computes its own in the float one: x86-64 gcc `call g; movss
4(%rsp),%xmm0; addss %xmm0,%xmm0; ret`, ARM hard-float `bl g; vadd.f32
s0,s16,s16; pop {r11,pc}`, MIPS `jal g; ...; add.s $f0,$f0,$f0; jr ra`. With
both trials active `fillinMap` finds no rule that joins two storage classes,
and its fallback prefers the more general entry, so the function rendered `int
f(int a0) { return g(a0); }` and lost its float parameter. At the same point as
the pair check, `returns_own_value` runs the output model's `fillinMap` on a
copy of the trials with the claimed ones inactive, which is the return value the
option-off run would give the function. Upstream accepts a trial there only for
a value the function wrote and hands to the RETURN alone (a float temporary
also stored to memory fails `ancestorOpUse`), and a convention returns a value
in one class, so when that derivation uses a register of another storage class
than a claimed one, the claim is dropped and the function returns what it
computed. Three things keep the claim. The zeroed upper lanes a `movss` load
leaves in `xmm0` are accepted trials but derive no return value. A register of
the same class can be the rest of the claimed value: ARM `bl g; mov r1,#0; pop
{r11,pc}` still returns `g`'s result. And a value that is zero at every RETURN
(`is_zero`: a zero constant, `x ^ x`, `x - x`, or a copy, extension,
truncation, concatenation or conversion of zeros) is not evidence, because it
is also what `-fzero-call-used-regs` writes into every call-used register the
function does not return in: openssh's forwarders end `call f; ...; pxor
%xmm0,%xmm0; ...; ret`, and `sshkey_certify` would otherwise lose the `int` it
forwards. The cost of that is a function that calls and then returns `0.0`,
which keeps the callee's result as it did before. The x86-64 `struct { long;
double; }` that the gcc model's `join_dual_class` rule returns in `rax` and
`xmm0` is the one shape the class test reads wrong; it is left as the
option-off run renders it. The rule composes with `armfloatreturn`, which only
adds the float entries the output model derives from. The witness is
`decompiler/crates/kuna-cli/tests/passthrough_own_return.rs`, which builds ARM, AArch64,
x86-64 and little- and big-endian MIPS objects in the test: a float computed
from the function's own float parameter after the call, a float constant
loaded after it (ARM, with `armfloatreturn` off and on) and an x86-64 double,
with a plain wrapper, the ARM `r1` write and an x86-64 forwarder ending in the
`-fzero-call-used-regs` scrub as controls that keep their callee's result.
Base against head `decompile-all` over the 274 ELF fixtures and 562
debug-stripped decbench binaries (98 ARM firmware images at O0, O2 and
O2-noinline, run again with `armfloatreturn on`, and 464 x86-64 binaries at O2
and O2-noinline) changes no function. Without the zero test the same run turned
the nine copies of openssh's `sshkey_certify` into `return 0;`.

**An injected no-op is not a touch of the returned register.** The ARM compiler
specs stand in for the `setISAMode` user op, the mode switch of every `bx lr`
and `pop {...,pc}`, with a p-code injection whose body is `r0 = r0`, marked
`incidentalcopy`; the MIPS specs do the same with `v0 = v0` for `jr ra`. That
COPY reads and writes the return register at every return, so by the test above
an ARM `push {r4,lr}; bl provider; pop {r4,pc}` touched `r0` and stayed `void
wrapper(unsigned int *a0) { provider(a0); }` while the x86-64 `call provider;
ret` it compiles from got `return provider(a0);`. Upstream marks an injection's
COPYs incidental so that parameter recovery walks through them, and a COPY of a
storage range onto itself moves nothing, so the tail-return rule does not count
either of its Varnodes, and `returns_tail_result` follows the RETURN's Varnode
back through it to the call's creation. Both architectures now take the
callee's result on the same terms as x86-64, with no caller involved.

The argument side keeps counting the no-op, and a returned register the body
touches, if only through it, does not hold call-site trials as a claim
otherwise does (`body_touches`): heritage visits it with the option off as
well, so its trials exist there, and they are scored as they are there. On ARM
`r0` is both the return register and the first argument, and in `bl g; bl f; pop
{r4,pc}` the `r0` at `f` is `g`'s result: it prints `v1 = g(a0); return
f(v1);`, where leaving that trial unscored printed `g(a0); return f();`. A wrapper that writes the
register itself before its call (`mov r0,#5; bl f; pop {r4,pc}`) still touches
it and stays `void`: `r0` is an argument register, which the before-the-call
relaxation above does not take. The witness is
`decompiler/crates/kuna-cli/tests/arm_wrapper_returns.rs`, which builds its ARM
object in the test: a wrapper ending in `pop {r4,pc}`, one ending in `bx lr`,
one whose caller ignores the result and a three-deep chain of them return their
callee's result, and `return provider(provider(a0))` keeps its inner call's
argument. Its controls keep `void`: a void callee, a two-function cycle, and a
declared `void` prototype on the wrapper or on its callee; a write after the call
returns what was written, and `store(provider(a0))` keeps its argument. An
indirect call and the argument write above are not this rule's evidence, but the
fixture's `consumer` reads `r0` after calling the wrapper, so `decompile-all`'s
`voidret` redo (below) returns `(*a3)()` and `provider((unsigned int *)0x5)`
there, and returns `provider(a0)` with the option off as well; a wrapper whose
result no caller reads stays `void` without the option. Thumb (`pop {r4,pc}`, `pop.w
{r4,lr}; bx lr`) and little- and big-endian MIPS (`jal provider; ...; jr ra`)
wrappers have their own cases there, the MIPS ones in a linked image because
kuna does not apply a MIPS object's relocations. The measured rates are under
**Default** below. Over the 272 ELF fixtures one function changes, the synthetic
Cortex-M reset handler of `cortexm_aifcorroborate_le32`, which ends in `bl` to a
helper recovered as returning `r1:r0`.

The witnesses are `passthroughpair_x86_64` and `passthroughpair_le32` under
`tests/cli/passthrough-returns-a-register-pair-a-tail-call-leaves.json` and its
ARM twin; a forwarder that writes `rdx` after the call and the `xmm0:xmm1`
forwarder are their controls. Over the decbench O2 and O2-noinline corpora (500
binaries, x86-64, i386 and ARM) the pair arm changes two coreutils wrappers,
each in three binaries (6 functions). `get_stat_btime`, a `jmp get_stat_mtime`,
now returns the `struct timespec` DWARF gives it. `strintcmp`, a `jmp
numcompare`, goes from `void` to the `undefined16` its callee is recovered
with, where DWARF says `int`: the callee's own recovery invents the `rdx` half,
and the wrapper repeats it, as it repeats any stated return. The bytes cannot
tell a second limit apart either: a wrapper that truncates the pair its callee
returns -- `int lo(int a,int b) { return (int)wide(a,b); }`, or `return
mkp(a,b).a;` -- compiles to the same `jmp` or `call; ret` as a real forwarder,
and is given the whole pair.

A callee states a return at all only where its own body computed one. `protoorder`
passes the callee's `Funcdata` to `recovered_output`, which asks
`kuna_returnuncomputed::every_return_computes`: each live RETURN must hand back a
value produced in every byte and on every path -- no terminal reachable through the
move-only operations may be an unwritten Varnode at the return register or an
INDIRECT creation standing for a callee's clobber. The pair repair of §4.9 asks the
relaxed form of the same question, computed if *any* input of a phi or a `PIECE` is,
because it is choosing between two halves of a wide return and must keep a genuine
one; a caller about to adopt the whole value needs the strict form. gnulib's `void
version_etc_arn` ends its fallthrough path in a `__fprintf_chk` and keeps that
call's `RAX` clobber, so kuna recovers it as returning `long`; the relaxed question
calls the `CONCAT44(<leftover>, __fprintf_chk(...))` it returns computed and the
strict one does not. Without the gate every `version_etc_ar` wrapper inherited that
wrong return: 212 of 4,267 gained returns over 444 decbench slices, against 6 of
4,087 with it (`docs/features/passthrough/dwarf-confirmation.md`). The walk's node
budget answers `false` when it runs out, because `true` there states a return on a
body it never finished reading.

Nothing is added where a callee stated nothing: a single-function `kuna
decompile`, a narrowed or sharded `decompile-all`, an import, `--option
protoorder off`, and under `--option protoorder types` a callee in a recursive
component, leave every call as the option-off run renders it. Under the default
`cycles` a recursive callee states its list like any other, and this rule reads
it on the same terms. `tests/stages/kuna-passthrough.xml` is the negative
control for that clause (the console states no prototype); the positive witness
is `decompiler/crates/kuna-analysis/tests/fixtures/passthrough_x86_64` under
`tests/cli/passthrough-gives-a-forwarding-function-its-parameter.json`, with a
clobbered forward and a variadic callee as its controls, and the three shapes
above (`noop(); twoarg(p,3)`, `vout(p); twoarg(p,3)` as a tail and as a plain
call, and `sysinttostr`) as controls that must keep every argument the
option-off run gives them. The vararg tail has its own fixture,
`varargtail_x86_64` under
`tests/cli/passthrough-declines-a-vararg-tail-parameter.json`: `xfail` pushes an
unloaded `rdx` as gcc's alignment filler before a variadic-style call and so
recovers a third parameter in every arm, and the function that forwards `rdx` to
it must keep two, while the `jmp`-forwarding control beside it still gains its
`char *`. The forwarder that calls its target has
`decompiler/crates/kuna-analysis/tests/fixtures/passthrough_callfwd_x86_64`
(generated by the `.py` beside it) under
`tests/cli/passthrough-a-caller-passes-its-inputs-to-a-call-forwarder.json` and
its option-off arm: the callers of a one-deep and a two-deep call forwarder pass
`rdi` and `rsi` on, the caller of one that writes `esi` first passes `rdi` alone,
and the callers of an indirect-call forwarder and of a two-function call cycle
gain nothing. `passthrough_widthfwd_x86_64` (built from the `.c` beside it) under
`tests/cli/passthrough-a-call-forwarder-keeps-the-width-it-reads.json` is the
openssh shape above: `send_open` and `use_send` gain the structure pointer their
chain of call forwarders hands to `by_id`, and keep `int id`.

**Default.** On, on the strength of the parameter arm. Every function that gains
something was checked against its unstripped twin's DWARF prototype over two
corpora: the 444-slice decbench corpus (8 GNU projects) and 130 slices of 17
projects disjoint from it (openssh, e2fsprogs, dpkg, kmod, dash, iproute2,
gnutls, zlib, …), 574 slices in all. **4,107 of 4,346 gained parameters are
confirmed, none contradicted, and the remaining 239 belong to forwarding thunks
the toolchain emitted with no debug entry at all**; no function and no call site
loses an argument, and nothing moves at -O0, where the register is already named
by an op. The two refusals above are what that costs: measured on the disjoint
corpus first, the rule contradicted DWARF on 159 of 1,691 checkable parameters
(9.4%, every one the openssh `xcalloc` shape), and removing them costs 208
confirmed parameters, 5% of the gain.

The **return** arm is a judgement rather than a proof, and it is where the
default costs something. 5,458 of 5,615 gained returns are confirmed, 157
contradicted — 6 on the decbench corpus but 151 on the disjoint one, all of one
shape: a source-`void` wrapper that tail-calls a value-returning function
compiles to exactly the `jmp` a wrapper that returns what it calls does, and
`rax` holds the callee's result at the RETURN either way
(`ext2fs_fast_mark_block_bitmap` is DWARF `void` and gets `unsigned long`;
gzip's `char *gzip_base_name` is the same code and is right). The rate follows
the project's style, not the optimisation level. The evidence is
`docs/features/passthrough/dwarf-confirmation.md`; set `off` to get upstream's
reading back, for the returns as much as the arguments.

On ARM and MIPS the return arm also takes a wrapper that calls and then returns
(`bl f; pop {r4,pc}`, `jal f; ...; jr ra`; the injected no-op above). Over 92
debug-stripped ARM firmware binaries of the decbench O0, O2 and O2-noinline
corpora, 223 functions gain a return that way: DWARF confirms 182 and says `void` for 39,
and 2 are entries kuna finds 0x12 bytes into nuttx's `vsyslog`, which has no
subprogram of its own there (182 of 221 checkable, 82.4%). Two projects carry
most of the misses: betaflight (49 of 74) and cleanflight (28 of 34) are built
with `-Og`, which turns off sibling calls, so each of their one-line `void`
wrappers ends in `bl f; pop {..,pc}` rather than a tail `b f`. Without them it is
105 of 113 (92.9%); the same corpus's tail-jump returns are 157 of 164 (95.7%),
and on 64 x86-64 binaries the tail and `call; ret` returns are 93.4% and 94.6%.
The misses are the one shape above: nuttx's `*outstream_putc` end in a call to
the matching `puts`, libgcc's `_Unwind_SetGR` in `_Unwind_VRS_Set`. A wrapper
also repeats its callee's recovered type, so the nuttx callers of `getopt`
compare against `0xffffffff` because `getopt_common` is recovered as returning
`unsigned int`. No MIPS corpus with DWARF was measured.

### (kuna) `varargforward` — a declared value forwarded in its own register to a variadic call

(kuna) `varargforward` (default on,
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_varargforward.rs
(forwards_declared_value)`) keeps a variadic argument that the caller does not
move at all. `int d2(const char *f, int a) { return pr(f, a); }` with
`pr(const char *, ...)` compiles to `xor %eax,%eax; jmp pr` on x86-64: `a`
arrives in `esi`, the first variadic integer register, and stays there. On
AArch64 `double c2(int k, double x) { return vr(k, x); }` is a bare `b vr` with
`x` in `d0`, and `vr(k, g(k))` leaves `g`'s result in `d0`. Upstream refuses
all three. `AncestorRealistic::execute` fails a trial whose value at the call
is the function's own input, because it expects to see a value moved into the
argument register, and `ancestorOpUse` fails one whose value is another
call's output. The trial is inactive, and with nothing active after it,
`fillinMap` drops it: the calls printed `pr(f)`, `vr(k)` and `g(k); vr(k)`.

The instructions are the same when the source did not pass the value: when
`a` is a parameter nothing uses, `pr(f)` compiles to the same `jmp`, and so does
`g(k); vr(k)` when `g`'s result is discarded. The rule therefore asks for a
declared prototype and a value nothing else reads. In `check_input_trial_use`,
next to the `al`/CR6 count, a trial in the variable part of a call whose
prototype ends in `...` is marked active when:

* the value at the call slot is the calling function's own input, that
  function's input is locked, and one of its declared parameters starts at the
  same storage (justified either way, so an `int` in `esi` matches an `rsi`
  trial); or the value is the output of a call whose return type is locked and
  not `void`, at exactly the storage of that output;
* the call slot is the value's only reader. An INDIRECT for some call's
  possible effect on the register does not count as a reader as long as
  nothing reads its own output;
* a floating-point trial is on an image without a count: where the caller
  states how many floating-point registers it filled (x86-64 SysV `al`, 32-bit
  PowerPC CR bit 6), the counted registers are the arguments and the others
  are not, and on 64-bit PowerPC the FPR only shadows a general register;
* the call's last fixed argument is not a printf format that consumes fewer
  arguments than the trial's place among the variadic slots of its class.
  The place is found by appending variadic arguments of the trial's class and
  size to the call's declared pieces until the model assigns the trial's
  storage. The format is read when the argument is a constant, a COPY or
  INT_ADD of constants, or a LOAD from read-only memory (an ARM literal pool),
  and a string with no `%` consumes nothing. This keeps
  `sqlite3_mprintf("... %s ...", zName)` from taking the `NotUsed2` that an
  sqlite callback declares but never uses in the next register.

A value with another reader normally remains ambiguous: `if (a > 5)
return 0; return pr(f, a);` with a dynamic `f` compiles the same with or without
`a`. `double a = g(k); return vr(k, a) + a;` on AArch64 is ambiguous too.
The exclusive-reader path still requires a declared caller prototype.

An immutable constant integer format gives separate evidence for a value with
another reader. `kuna_varargformat::arguments` requires a locked variadic callee
whose last fixed parameter is a character pointer. A callee name containing
`scanf` declines the proof because its conversions consume pointers. It resolves
the constant pointer through the bounded constant-value walk above and checks
memory-range read-only properties one byte at a time, including the NUL
terminator. A covering array symbol inherits properties at its first address;
its flags cannot establish that every byte is immutable. Named formats therefore
use the range property map independently of symbol flags. A
whole-range property query can fall back to properties at the starting address,
so it cannot prove that a writable conversion or terminator is immutable. It
accepts ordinary promoted integer conversions (`d/i/u/o/x/X/c`), `h/hh` integer lengths, star width and precision
operands, and literal percent escapes. Other classes, lengths, malformed or
positional conversions decline the entire proof; at most sixteen arguments are
assigned. The complete fixed prefix plus the format's integer types is assigned
through the call's model with its variadic slot preserved. This is storage
evidence for each consumed argument, not a conversion count applied to every
register class.

`kuna_varargformat::activate_trial` is a fallback after the existing declared-value
and ancestor-use paths reject a trial. Arguments those paths already retain keep
their original storage and scoring. A later pass can retry a checked inactive
trial when a variadic prototype or constant format becomes available after its
initial scoring. Definitely unused, unreferenced and passthrough-claimed trials
remain excluded; their inputs cannot supply new argument evidence.
It matches only an existing processor-register trial containing the assigned
argument. Its value must have realistic defined ancestry on every incoming path
(the ancestor walk disallows a failing path) or be a declared incoming parameter/return on the terms above. The declaration
must cover the consumed source bytes at their incoming storage, even if the
value now feeds a different argument register. Extensions can supply the other
bytes of a promoted integer.
A second bounded walk proves the consumed bytes through copies, concatenations,
subpieces, extensions, integer arithmetic, comparisons, carry/overflow flags,
boolean operations and all phi inputs. The consumed low bytes of addition,
subtraction, multiplication and negation depend only on the same low operand
bytes: unknown upper bytes of a wider temporary cannot change them. Bitwise
operations require the matching slice. Left shifts require all shift-count bytes
and the source bytes below the consumed end; right shifts and comparisons require
complete operands because upper bits can affect the result. Boolean expressions
require every consumed operand byte to be defined, just as arithmetic does.
An integer XOR with identical SSA operands defines zero without depending on
those operands' previous bits, including a register cleared after a call.
This covers a stored integer selected from a tag comparison and then clamped;
an undeclared comparison operand or an unknown phi arm still declines the proof.
Constants, loads and declared call returns provide defined values; undeclared input bytes decline
the proof. The walk stops after sixty-four visited nodes, which also bounds its
recursion. Copies, slices and extensions can form more than sixteen ancestry
levels before simplification; they share the node budget with the computations
instead of imposing a separate shorter depth limit.
A declared byte alone does not prove the other bytes of an integer. Assignments
with hidden or indirect parameter pieces are declined. The ancestor flags and
conditional execution recheck are preserved. The trial shrinks to the promoted
integer width and is marked active, so a store of
`*value + 1` does not hide the same value in EDX at `render_value("%i", ...)`.
Before activating a later argument, the consumed bytes of every preceding
format-assigned trial must also be defined. A missing, definitely unused,
unreferenced or passthrough-claimed predecessor declines the proof, including a
trial replaced by a synthetic zero.
This prevents parameter hole filling from inventing an uninitialized earlier
argument. No register past the format's argument list is supplied; an undeclared
live-in or call clobber is not supplied either. Computed and loaded values do not need
a declared caller prototype on this path. A dynamic, writable or unsupported
format leaves the exclusive-reader rule unchanged. With
`varargforward off`, upstream's scoring applies to both paths.

The upper half of a register the caller never received is a separate case,
issue 876. `int d4(const char *f, int a) { pr(f, a); return pr(f, a); }` keeps
a copy of `a` for the second call, and gcc and clang -O2 pass it to the first
call in `esi`, unmoved. Heritage pieces the 8-byte `rsi` trial together from
the declared 4-byte `esi` input and the undeclared upper half of `rsi`.
`AncestorRealistic` accepts a concatenation as wide as the trial without
looking at its pieces, and `ancestorOpUse` accepts a copy whose only use is the
same slot of another call to the same callee. The trial was active at 8 bytes,
and the call printed `pr(f,CONCAT44(v1,a))` with `v1` never assigned.

`int u5(const char *f, int a) { pr(f, a, a); return pr(f, a); }` is the same
call with a second reader: gcc also copies `a` into `edx` for the third
argument. `ancestorOpUse` then leaves the `rsi` trial inactive, and because the
`rdx` trial after it is active, the inactive-chain pass later fills the gap with
the 8-byte trial, which printed `pr(f,CONCAT44(v1,a),a)`.

`narrow_undeclared_upper`
(`decompiler/crates/kuna-decomp/src/p4_calls/kuna_varargforward.rs`) runs at
both points: right after the ancestor-use path activates a register trial in the
variable part of a `...` call, and right after it leaves one inactive where the
format proof does not activate it either. Shrinking an inactive trial that is
never filled in changes nothing; one that is filled in later keeps the narrowed
size. It shrinks a processor-register trial wider than an int to the int width,
keeping the register's low-order end on a big-endian image, when two bounded
walks agree. The low int bytes must pass the defined-bytes walk above. Every
byte above them must come, through copies, concatenations, subpieces and phi
nodes whose inputs all qualify, from an incoming processor register that no
parameter of the caller's locked prototype overlaps. Under the caller's own
prototype those bytes hold no value the source could have passed, so the
argument is the int and the calls print `pr(f,a)` and `pr(f,a,a)`. The rule
declines floating-point registers, callers with a parameter in join storage, and
callees without `...`. A declared `long` or pointer, or any value whose upper
bytes are computed, keeps its full width. A `char` or `short` that clang
forwards without extending it is not narrowed either, because bytes inside the
int are undeclared too. With `varargforward off` the trial keeps its 8 bytes.

What remains ambiguous is a declared parameter nothing uses that sits in a
variadic register, with no constant format to bound the call: `int d6(const
char *f, int a) { return pr(f); }` prints `pr(f,a)`, and an unused `double` in
`d0` joins an AArch64 variadic call. The register holds that value when the
callee starts, so the printed call compiles back to the same instructions.

### (kuna) `callbacktype` — the prototype of the slot a callback is passed to

`protoorder` carries a callee's types out to its callers and `calleevote`
carries the callers' types back in. Neither reaches a function that no call
site names. A `qsort` comparator, a `signal` handler, a `pthread_create` start
routine is reached only through a pointer, so its own body is all it has, and
the body of a comparator that never dereferences past the first word gives
`int sub_3a72(unsigned long *a0, unsigned long *a1)` where the program declares
`int (const void *, const void *)`; a handler that ignores the signal number
gives `void sub_1100(void)` where the program declares `void (int)`.

The library declares both, and kuna already resolves the library's own
prototype at the call site — the call renders `qsort(dat_d148, dat_d150, 0x10,
sub_3a72)`. `callbacktype` (values `on|off`, default `on`;
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_callbacktype.rs`) reads the
argument in that fourth slot as what it is: a declaration of `sub_3a72`. The
driver is `callback_park_round` in
`decompiler/crates/kuna-cli/src/decompile_all/callee_first.rs`, and like `protoorder` and
`calleevote` it lives only on the callee-first whole-binary pass.

**The slot table.** `SLOTS` names 23 library entry points that declare a
function-pointer parameter (`qsort`, `bsearch`, `lfind`, `lsearch`,
`signal`, `__sysv_signal`, `bsd_signal`, `sigset`, `atexit`, `on_exit`,
`pthread_create`, `pthread_once`, `pthread_key_create`, `pthread_atfork`,
`scandir`, `scandirat`, `ftw`, `nftw`, `tsearch`, `tfind`, `tdelete`,
`tdestroy`, `glob`), each with its OWN parameter list and which of those
parameters is the callback. The entry point's list is what the prototype model
is asked for storage (`kuna_protoorder::model_storage`), so the argument
register of the callback slot comes from the program's calling convention
rather than from an architecture table. Three names are deliberately absent:
`sigaction`, whose handler is a structure member and not an argument;
`__cxa_atexit`, because glibc's `<stdlib.h>` rewrites `atexit(f)` into
`__cxa_atexit(f, 0, __dso_handle)` with `f` cast from `void (*)(void)` — taking
that slot's declared `void (*)(void *)` literally would give every `atexit`
handler in a glibc program a parameter its source never wrote; and `qsort_r`,
whose C libraries disagree about where the callback goes. glibc and musl pass
the comparator fourth and its data pointer fifth, while macOS and FreeBSD
before 14 pass an opaque `thunk` fourth and the comparator fifth. The imported
name does not say which library the image links, and a declaration taken from
the wrong one would land on whatever the caller passed as its data pointer.

**Recording.** During the callee-first pass the driver sets
`Ledger::recording`, and after each decompile `record` files, per caller, every
constant a declared callback slot carried: the CALL's input at the slot's
storage, resolved through the forms a global address takes on the way to a call
(a plain constant, the `PTRSUB` off the constant spacebase a `&DAT_3a72` is
built as, and the copies, casts and zero-extensions between them). Anything
else — a load, a phi, arithmetic on a base — is not one address and is not
recorded. Of the addresses it did record, `record` also files the ones this
body used somewhere no callback argument accounts for: every operation that
consumes the address is such a use unless it merely carries the value along (a
copy, a cast, the `PTRSUB`, a phi, the `INDIRECT` a call leaves behind) or it is
one of the callback arguments just filed. The same decompile files what the
function's own body proved about ITSELF, once: whether it hands back a value it
computed, the storage of every input its own recovery found and of the output
it recovered, the low bytes of that output any returned value can set (from the
non-zero masks of what its live RETURNs hand back), and — when it hands back
nothing of its own — what the calls its RETURNs are reached from leave in the
return register, and the parameter and return types its signature printed.
Every decompile also files, for each direct call whose
returned value it uses, the part of the return storage that call site reads
(the low bytes holding every bit the body consumes of the call's output), and
for every direct call, its callee, how many arguments it passes and whether
anything consumes its result. The recording covers the whole run before the
park round -- the first pass, `calleevote`'s rounds and the convergence pass --
and is filed per caller: a function decompiled again replaces the callback
arguments its earlier body filed instead of counting them twice, and adds its
direct calls to the earlier ones, so whichever body the driver keeps, every
call it prints is on file.

**Deciding.** The park round runs LAST: after the callee-first pass,
`calleevote`'s rounds and the convergence pass, when every other function has
printed what it prints with the option off. `Ledger::decided` folds the run's
facts per address: which slot declared it, how many distinct call instructions
carried it, whether two slots disagreed, and whether any body that registered it
also used it elsewhere. The driver then parks the declaration
(`Architecture::set_function_prototype_pieces_at`, the seam `protoorder lock`
uses and `ActionDefaultParams` reads a declared prototype from) and decompiles
that function again -- and only that function. A park changes the parked
function and nothing else: no caller is decompiled again, so every caller keeps
its own parameter and return types, its casts and the call it printed. That
call was recovered without the declaration, so where it contradicts the
declaration the park is refused instead (below). A declaration that is exactly
the signature the body already printed -- the same parameter types in the same
order, the same return, no `...`, as a `pthread_once` routine recovered as
`void f(void)` is -- is not parked at all (trace token `body-agrees`): it would
change nothing but the cost of decompiling the function again. Unlike
`calleevote`, this moves the callback's ARITY, and it has to: a handler that
never reads `edi` has no parameter for a vote to retype.

The parked list is closed (`first_var_arg_slot = -1`), not the floor
`protoorder` parks, because it is a declaration rather than a partial recovery;
the function prints without a `...`.

On a successful park the statements the run made about that function leave the
tables its own redo would read them from: `protoorder`'s recovered parameter
types and `calleevote`'s decision would otherwise still type the parameters the
declaration now fixes, and a parked callback that calls another parked one (a
comparator that returns another comparator's result) reads both.

**What is refused.** A declared prototype outranks this one and the park is
declined for it — DWARF, a user `--assert`, the library tables, and, under
`--option protoorder lock`, the callee's own recovered prototype, which the
first pass parked in the same place. Two callback
slots that disagree about the same address decline it. So does an address that
reaches somewhere the recorded callback arguments do not explain. Two walks
answer that, because neither is enough alone. The driver asks the image: the
function must not be in `open_function_entries` (not exported, not the entry
point, not a pointer-width word of a loaded section, not a Mach-O chained-fixup
or dynamic-relocation target), every address-taking cross-reference
(`CallGraph::address_taken_refs`) must sit in a function that handed it to a
slot, and there must not be more of them than there were callback arguments — a
comparator also stored in a dispatch table is not declared by the `qsort` call.
That walk counts INSTRUCTIONS, and one hoisted `lea` can feed two registrations,
so the recorded bodies answer for the uses: an address a registering body also
used somewhere no slot accounts for is refused, which is what keeps
`signal(SIGINT, f); atexit(f);` from declaring `void (int)` on an `f` that takes
nothing. The value is followed from wherever it is materialized through
whatever only carries it — copies, casts, the `PTRSUB` a global address is built
as, a phi, the `INDIRECT` a call leaves behind — and every operation that then
consumes it counts. So a handler registered with `signal` and then called
through a phi that may also hold another function is refused, and so is one
whose register is also written into a global or an address-taken local, even
when that body never reads the copy back: memory the image walk cannot follow
is somewhere else the address went.

The body's own recovery is then held to the declaration in both directions
(`body_contradicts`), because a closed list and a declared return are exactly
right or they fabricate something.

- A recovered input list LONGER than the declaration refuses it: an input the
  body's own recovery found past the list would be dropped at every direct call
  site and read uninitialized in the body — which is what a three-argument
  function cast into `qsort`'s slot is. The one-sided entry walk `calleedeadarg`
  caches (`kuna_protoorder::reads_past_the_list`) then asks whether the body
  READS the argument register one past the declared list before writing it. That
  walk sees reads, not liveness: a body that forwards its arguments — a tail
  call, a jump — passes the register on without reading it and states nothing,
  which is why the recovered-arity refusal is the one that holds a forwarder.
- A recovered list SHORTER than the declaration refuses it when anything calls
  or tail-jumps to the function directly (`CallGraph::called_directly`: any call
  or jump cross-reference, from another function, the function itself, or code
  no function owns). The closed list materializes the missing argument at every
  such site out of whatever the register last held, so `signal(SIGALRM,
  (void (*)(int))cleanup); ... cleanup();` on a `void cleanup(void)` would print
  `cleanup(v2)` after an invented `v2 = 0x2006;`, and a one-pointer scorer cast
  into `qsort`'s slot and also called directly would pass `qsort`'s element size
  as a second argument.
- With NO direct call site the shorter list is parked, by design. The declared
  slot is then the only thing that ever calls the function, so it is the only
  evidence of how it is called, and the binary cannot tell `void h(int unused)`
  from `(void (*)(int))cleanup`: C handlers conventionally declare the parameter
  and ignore it, and on the 444 decbench slices every parameter this shape adds
  is one DWARF declares. The added parameter is read nowhere, so `--json`
  exports it as an `arg` with empty `line_numbers` and `addresses`.
- A recovered input that is not inside the storage the declaration gives its
  position refuses it, whether or not anything calls the function directly. The
  count can be right and the width wrong: a `struct ctx *` routine cast into
  `signal`'s `void (*)(int)` reads all eight bytes of the register the slot
  declares a four-byte `int` in. Declared, its body would rebuild the pointer as
  `CONCAT44` of the declared half and a register nothing set, and a direct call
  would print `cleanup((int)G)`; a `long` handler called directly with a value
  that does not fit in 32 bits would pass a different number from the
  program's. Each input is held to its own position, so a list whose registers
  are out of the declaration's order is refused as well.
- A `void` slot on a body whose every live RETURN hands back a value it computed
  refuses it.
- A value-returning slot on a body that computes no value refuses it: the
  register's leftover would print as an invented return expression — the high
  half of a `void *` that a tail-called `puts` never wrote reads as a global
  that is not in the image, and a body that never writes the register returns
  an unset local. What counts is the value, not the body's own recovered
  prototype. A comparator ending `return strcmp(a, b);` recovers `void` on its
  own because nothing in the program reads its result, yet `strcmp` leaves its
  declared `int` in the register for `qsort` to read. So each live RETURN is
  traced to the direct CALL it is reached from (`call_return`), and the machine
  code from that CALL to the RETURN is read one instruction at a time
  (`straight_to_return`) — from the image, because the p-code of a body that
  returns nothing has already dropped any write to the return register as dead.
  The path has to be a straight line (fall-through and unconditional jumps, no
  other call) that writes no byte of the callee's DECLARED output, and that
  output has to cover every byte of the storage the slot's return is given. A
  `jmp` to another function is the empty path; `call strcmp; leave; ret` and
  `call strcmp; addl $1, n(%rip); ret` are straight lines. `pthread_create`'s
  `void *` over a tail-called `puts`'s four-byte `int` is not covered and is
  refused. A value that reaches the RETURN through a join (`if (r) return r;`
  over two calls) or past a conditional branch (a stack protector's check) is
  not proved either, and is refused: the proof is kept to a shape that cannot
  be wrong. The rule is about the machine, not the source: a `void` function
  cast into `glob`'s `int` errfunc slot that ends in a tail-called `fprintf` is
  parked `int` and returns that call's value, which is what `glob` receives and
  what IDA prints for it.
- A value-returning slot refuses a value wider than the storage its declared
  return is given — the body's own computed output, or the declared output of
  the call the proof above traced it to. The computed output is measured by the
  bits its value can set, not by the storage its own recovery gave it: every
  32-bit write on x86-64 zero-extends into the whole register, so an `int`
  comparator ending `movzbl %al,%eax; cmovl %edx,%eax` (coreutils'
  `compare_ranges`, which DWARF declares `int`) recovers an 8-byte `rax`
  output whose upper half is provably zero, and it is parked. That exception
  holds only when nothing calls the function directly. A `long` comparator cast into
  `qsort`'s `int (*)(const void *, const void *)` would subtract in the low half
  only, and a direct caller that prints the whole result would read its high
  half back as `CONCAT44(dat_4, ...)`, a global that is not in the image. The
  same holds when the `long` is what a tail-called `strtol` leaves. A narrower
  value a CALL left is the case above: the callee's declared four bytes do not
  cover a `void *`, and the park is refused.
- A value-returning slot refuses a value the body COMPUTES that is narrower
  than the declared return, unless the bytes above it are provably zero at
  every RETURN. The declaration prints the whole declared return, so those
  bytes stop being invisible, and a byte write says nothing about them: clang
  compiles a `signed char` comparator to `mov (%rdi),%al; sub (%rsi),%al; ret`
  and a `bool` start routine to `cmpq $0,(%rdi); setg %al; ret`, and declared
  `int` and `void *` they printed
  `CONCAT31((undefined3)((unsigned int)v1 >> 8), ...)` and
  `(void *)CONCAT71(v1, ...)` with `v1` never set. gcc compiles the same
  comparator to `movzbl (%rdi),%eax; sub (%rsi),%al; ret`, whose upper bytes
  the `movzbl` cleared, and it is parked. The body's p-code cannot answer
  this: its own recovery returns the narrow value, and the write that cleared
  the rest was dead to it. So the machine code is walked from the entry over
  every path (`zero_at_every_return`), each byte of the declared return held
  zero only where every path into an instruction agrees: a constant with a
  zero byte, a zero-extension, an `and` with a zero byte, an `xor` of a
  register with itself, and every 32-bit write on x86-64, which the processor
  specification writes as a zero-extension into the whole register. A call
  clobbers the register, a call the body's own flow does not continue past
  ends the path, a conditional move (whose p-code branches to the next
  instruction) is walked both ways, a write inside an instruction whose p-code
  branches within itself (a `rep` prefix) is held to what the byte already was,
  and an indirect branch, an undecodable instruction or more than 4096
  instructions refuse the park. So
  an `int` routine cast into `pthread_create`'s slot is parked and prints
  `return (void *)(unsigned long)(puts(a0) + 1);`, and `xor %eax,%eax` before
  a `setg %al` is parked as well. Bytes above the value that the body sets to
  something other than zero refuse it too, unless its own recovery already
  returns them: `mov (%rdi),%eax; test %eax,%eax; setg %al` recovers a
  four-byte return that prints the same `CONCAT31` with the option off, and a
  `void *` slot adds only the upper half the 32-bit load cleared.
- A direct call that its caller already printed in a way the declaration
  contradicts refuses it (`Ledger::direct_calls_disagree`, trace token
  `direct-call-disagrees`), because that caller is never decompiled again: a
  call that passes another number of arguments than the declaration lists, or
  that uses a result the declaration does not return. A clang -O0 bsearch helper
  that leaves the `idiv` remainder in `rdx` before calling its comparator
  prints that remainder as a third argument; a clang -O2 wrapper that forwards
  its own two argument registers to a comparator untouched prints the call with
  none. Parking either would leave a printed call and a printed prototype that
  disagree. A call that ignores a result the declaration does return prints the
  same under either prototype and does not refuse it. Every function the image
  shows calling or tail-jumping to the callback (`CallGraph::direct_callers`,
  the callback itself included) must have been decompiled by the run, since
  what an undecompiled caller would print is unknown; a call from code no
  function owns refuses it the same way (`caller-not-seen`).
- A direct call site that reads more of the return register than the declared
  return holds refuses it, whatever the body says. A `long` comparator that
  returns a zero-extended comparison writes only `eax`, so its body is no wider
  than an `int`; the call that hands the whole `rax` to `printf("%ld")` is what
  says otherwise, and declared `int` that call would print
  `CONCAT44(dat_4,zcmp(..))`. The same read refuses a comparator whose result
  another comparator returns straight through its own `ret` (ptx's
  `compare_words` under `compare_occurs` at -O2): the outer one hands on the
  whole register, and declaring the inner one alone would leave the outer one
  returning a variable no path through the call sets. The outer one is refused
  in turn, because the value it hands back on that path is a call's full
  register, which nothing bounds.

A function this run never decompiled has no body facts at all, and nothing is
parked on it.

Because the escape question is answered by a cross-reference walk that reads one
instruction at a time, an architecture whose code builds an address from two
(AArch64 `adrp`+`add`, MIPS `lui`+`addiu`, ARM `movw`+`movt`, i386 PIC) has no
answer and nothing is parked. Everything is inert where there is no callee-first
pass: `kuna decompile`, a run narrowed by `--addr` or `--functions`,
`decompile-project --stream`, `--option protoorder off` and `--jobs N`. A
`--jobs N` run that does not name the option parks nothing, and its stderr note
says the run lacks the callee-first order this option rides; naming it
(`--jobs N --option callbacktype on`) is refused, the way `--option protoorder`
is. Of the other inert surfaces, two say so on stderr (`warn_protoorder_inert`,
shared with `protoorder`): `decompile-project --stream` and `decompile-graph`. `kuna decompile` does not,
and neither does `protoorder` there — it forks one `decomp_dbg` per function,
which is a surface neither option has ever reached.
`KUNA_CALLBACKTYPE_TRACE=1` prints every decision and its reason on stderr.

### (kuna) `calleevote` — the type every caller passes

`protoorder` carries what a callee's own recovery found to its callers. Nothing
carried anything the other way, so a function that only forwards, compares or
stores a pointer kept `void *` or `long` while every one of its callers held a
named record or a `char *` for the same value. `calleevote` (values
`off|types|fields`, default `fields`;
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_calleevote.rs`) closes that
direction on the one surface that decompiles every caller: the callee-first
whole-binary run `protoorder` drives (`decompile-all`, `decompile-project`). The
driver is `callee_vote_rounds` in `decompiler/crates/kuna-cli/src/decompile_all/callee_first.rs`.

**Recording.** During the callee-first pass the driver sets
`Ledger::recording`, and after each successful decompile `record` files two
things on `Architecture::kuna_calleevote`: the function's own recovered
parameters (storage and type), and for every direct CALL it makes the callee's
entry, the call instruction and, per argument slot, the storage the recovery
finalized (`FuncCallSpecs::final_input_storage`) with the HighVariable type the
caller gave the value. A function with a declared prototype, a `...`, an
input-locked prototype, or a hidden, indirect-storage, `this` or type-locked
parameter files no parameters and is never voted on. A later decompile of the
same function replaces the calls it recorded; its parameters stay those of the
first decompile (below).

**Knowing every caller.** The claim is about the whole program, so it is
checked against the call graph, not against what the decompiles happened to
see. `CallGraph::direct_call_sites` lists, for a function, the address of every
direct call and tail jump to it from another function, and answers "unknown"
when the reference walk sees its address taken in code (a data reference), or
when `open_function_entries` names it.

The reference walk reads one instruction at a time, so it sees an address
taken in code only where a single instruction carries all of it. That holds on
x86-64, where code takes a function's address with a RIP-relative `lea` or an
immediate. It does not hold elsewhere: AArch64 takes every function address in
code as `adrp`+`add`, MIPS as `lui`+`addiu`, ARM as `movw`+`movt` or a literal,
PowerPC and RISC-V as a high and a low half, and i386 position-independent code
as a base register plus an offset; the walk finds no reference from either
half, so a callback that is also called directly would count as closed. On
every architecture but x86-64 `open_function_entries` therefore names every
function, and nothing is stated. Two kinds of x86-64 image are open the same
way: a relocatable object, whose sections the loader lays out itself, so the
walk classifies no data reference and a stored callback is a relocation
against a zero word; and an image whose sections hold none of its entries (no
section headers).

Otherwise a function is open when its address is stored in the image: as a
pointer-width word at an aligned offset (a multiple of the pointer width) of
any section the image loads, code sections included (a const ops table placed
in `.text`); a Mach-O
chained-fixup rebase target; the target of a dynamic relocation; an exported
symbol; or the entry point. A section is one the image loads by its
`SHF_ALLOC` flag on ELF, never by its address, since firmware loads code at
address 0. A word that merely happens to equal an entry leaves that function
open, which only means nothing is stated about it. What the scan cannot see is
an address stored in any other shape: a pointer-width word at an unaligned
offset (a function pointer member of a packed struct, in an image with no
dynamic relocation for it; a position-independent image has one, which is
seen), or an offset added to another address at run time (a 32-bit offset
table, a relative C++ vtable, a self-relative pointer). A callback reached
only that way and also called directly can still be voted on. A function is decided only when its recorded
calls from other functions are exactly the listed addresses: a caller that
failed to decompile, or a call the walk found and no decompile reached, states
nothing. A self-recursive call is not a vote.

**The decision** (`decide_ledger`). A parameter is a candidate when the callee
typed it only as a pointer to nothing (`void *`, `undefined1 *`), a
pointer-width integer, or a one-field synthesized record (below). It takes a
type when every call passes the argument in exactly the storage the callee
recovered it in and every call passes the SAME committed pointer: a pointer to
a named record or union that carries its layout (a synthesized `struct_N` the
layout ledger shares, a record a program declares), a `char *` or a `char **`.
A `signed char *` or `unsigned char *` is a character pointer too (chapter 01
gives DWARF's those character types). Callers that disagree only in the sign of
that character, one passing a `char *` and another the same value cast to
`unsigned char *`, agree on `char *` (`same_but_character_sign`); with no
`char *` among them they state nothing.
A name with no layout behind it is not a commitment: the `FILE` shell
`libctypes` interns says no more about the object than `void *` does, and the
refusals below that read the pointee's members have nothing to read, so a
`FILE *` is voted only where the shell carries its fields
(`--option libctypes glibc`). Two types are the same when they are
one factory entry or have the same name and shape down the pointer chain; a
layout comparison alone would equate two records that merely have the same size.
Under `structheadless closed` a record the callee synthesized from reads past
its start is a candidate too, but only for a type the callers state from
outside the recovery -- a `char *`, a `char **`, a record a program declares --
and never for another function's synthesized record, which is one more partial
view of the object (`decide_ledger_under`). `tail`'s `parse_obsolete_option`
reads only `argv[1]` and `argv[2]`, so its first decompile under the option
takes a record; its one caller passes `main`'s `char **argv`, which replaces it.
Letting a caller's synthesized record replace a headless one too was measured
to spend the redo budget below on record-for-record swaps (tar `-O2
-fno-inline`: twelve declined redos against four) and cost `decode_timespec`
its `char **`.

**The vote.** Each function whose statement is new is decompiled again, in plan
order, so a redone callee states its types again before a redone caller reads
them. `seed` copies the statement onto the `Funcdata` at the same two seams
`protoorder` seeds from, and `ActionInferTypes::buildLocaltypes` offers it as one
more vote for the function input in that storage, ahead of the `ptrfromuse` and
`charptr` candidates (`input_vote`). It replaces the fold only where the fold
says no more than "a pointer-width value", and it is refused where the callee's
own uses disagree: the family refusals `protoorder` applies to its own votes
(`kuna_protoorder::input_refuses` — an integer operation on the value, a pointer
difference included; a family member that is type-locked, a global or frame
memory; a class conflict at another call; a load or store that does not land on
exactly one member of the record, which includes a read of bytes the caller's
record only covers with filler). Only the value's own family is checked: a
pointer the function derives from it (loaded from a field, or returned by a
call it is handed to) is not, so where the callers' record is smaller than
what the function reaches that way, the printer indexes past it as an array of
records (`*(unsigned long *)v1[1].field_0x0 = a1;` for a store at `v1 + 0x10`
into a 16-byte record), which is the same memory. A `char *` vote is also refused when the
callee stores a constant wider than a byte through the value: the printer
would spell `*(unsigned int *)(a0 + 0x34) = 0xffffffff` through a `char *` as
four character stores (`a0[0x34] = '\xff'; ...`), which is the same memory
but no longer reads as one store. A `char **` vote is refused the same way one
level down: every pointer-width value the callee loads through the value or
stores through it takes `char *`, so a constant wider than a byte stored
through any of those refuses it (`add(&cfg, path)` storing a new node into the
record and then writing the node's words; `kuna_protoorder::splits_a_wide_constant`).
A pointer to a pointer that some caller passes as the address of one of its
own frame objects (`&v4`) types that one object, not what follows it in the
frame: a caller keeping a record whose first member is a `char *` passes
`&cfg` as a `char **`. Such a vote is refused when the callee loads or stores
through the value anywhere but that one pointer at offset zero
(`reaches_past_the_pointee`), so `drop(&cfg)` reading `cfg->head` keeps its
own type, and `advance(&cursor)`, which only reads and writes `*cursor`, takes
`char **`; refusing every frame address instead was measured to lose 17
functions whose out-parameter is a real `char **` (`parse_line (char **keyword,
char **arg)`). A callee that only reads the first member (kmod
`cfg_kernel_matches` reading `cfg->kversion`) cannot tell the record from the
pointer and takes `char **`. Any vote is refused for a value the callee adds
to an address as an index (`((char *)0x1018)[a0]`, a constant the recovery
took for the pointer), where a pointer type would print as a cast at every
use (`indexes_a_pointer`). A value the callee only hands to a parameter
declared `void *` is NOT refused: that is what a wrapper around `memcpy` or
`fwrite` does with the `char *` its callers pass (`dired_outbuf`,
`samedir_template`), and refusing it was measured to lose 28 functions to gain 3.
The three it would have kept are gnulib's `xmemdup (void const *p, size_t s)`,
whose callers all happen to pass strings. The redone
functions record their calls again and the decision repeats, up to three
rounds, so a wrapper passes on what its own callers gave it. Every round
decides against the parameters a function's FIRST decompile recovered: a
redo's parameters carry the votes it took, and deciding against them would
leave an earlier vote out of the next statement, which replaces the old one,
so the next redo would lose it. A first statement that only repeats the types
the function already has (a one-field record its callers were handed back
through `protoorder`) is not made, since decompiling it again changes nothing.
A redo that fails keeps the first body.

**What a redo costs, and the budget for it.** The option's whole cost is the
second decompile, and a redo costs exactly what the first decompile of that
function cost: median 1.01x over the 55 functions `kmod -O2-noinline` redoes,
because a redo runs the same pipeline over the same bytes and only the type
lattice starts differently. A function's printed length is therefore what
redoing it charges.

The pass is given one budget for all three rounds,
`CALLEE_VOTE_BUDGET_PCT` (5) percent of the lines the first pass printed, and
every redo is charged the lines it reprints (`redo_charge`). Each round admits
from what it decided shortest first, ties to the lower (space, address) key
(`admit_within_budget`), so the cheapest bodies are bought first and the
admitted set is a function of the program rather than of the order the plan
visits it. A function printing at most `CALLEE_VOTE_MAX_LINES` (32) lines — the
shapes the vote exists for: a forwarder, a getter, a comparator — is admitted
even once the budget is gone, so a binary that uses the vote heavily keeps
every one of them and buys no long bodies at all, while one that barely uses it
redoes every candidate it has. A function the budget cannot reach is declined
for the run (`Ledger::decline`): the statement just decided is withdrawn, no
later round proposes it, the convergence sweep has nothing to apply to it, and
its parameters keep what its own body found — exactly what a function over the
old flat 32-line refusal used to get, now only on the binaries with no room for
it.

One function is declined differently: the one an earlier round already bought a
redo for and whose new body the driver kept (`Ledger::keep`). Withdrawing a
statement there would withdraw one a printed body was already printed with, and
the convergence sweep — which decompiles a function again after the rounds are
over and takes whatever the ledger states about it — would then print that
function without the vote the batch's own output used. So its statement goes
back to the one its kept body used rather than away, while the decision itself
still stops: nothing further is proposed about it and no more of the budget is
spent on it.

What that is worth is measurable without a stopwatch, because the redo pass is
a phase of its own: its share of a whole-binary run is 3.4% on
`kmod -O2-noinline`, 2.0% on `dpkg-divert -O2`, 1.3% on `cmp -O0` and 0.2% on
`fmt -O2` under the flat refusal. With no bound at all the same pass is 10.5%,
7.3%, 6.9% and 0.4%, and 30.1% on `mv -O0` — over the project's +5% budget on a
binary that redoes many long bodies, and far under it on one that redoes a
handful. The budget spends that room where it exists: `fmt -O2` reprints 96
of the 185 lines it is given and redoes every candidate it has, while `kmod`
reprints 708 lines (5.3% of its own output, its short functions being most of
it) and buys only the shortest of its long bodies.

**`fields`.** The same closed caller set decides one more thing. A function
whose callers are all known direct calls (at least one, none unknown) is marked
on its `Funcdata` (`kuna_calleevote_closed`), and `structsynth` then accepts a
pointer parameter whose pointee no declared prototype it is handed to
(`pipe (int *)`) gave it, and no callers' vote that the field fits (chapter
05), read at exactly one constant offset other than zero with an
access of four bytes or more as a one-field record (chapter
[05](05-types.md), `structsynth`). A one-field record is itself a candidate for
the vote above, so it gives way to the record every caller passes, and the
getter and its callers name one record. Its callers see the new type through
`protoorder`: where what a caller does with the value does not fit the one-field
record, that vote is refused, and a caller local that held the callee's old
`void *` only because the callee declared it keeps no pointer type; it is
declared as a pointer-width integer and may merge with another integer local
of its size (dash `sub_eb30`).

Everything is inert where there is no callee-first pass: `kuna decompile`
(one `decomp_dbg` per function), a run narrowed by `--addr`, `--functions` or a
triage filter, `--jobs N`, `decompile-project --stream`, a raw image and
`--option protoorder off`. The
variable rows a function exports keep their number; only their types move.
`KUNA_CALLEEVOTE_TRACE=1` prints every decision and its reason on stderr.

### (kuna) `callrettype` — a call returns the type its callee declares

`protoorder` carries a callee's recovered PARAMETER types to its call sites;
nothing carried its return. Upstream's `TypeOpCall::getOutputLocal` answers
only for a locked output, and a recovered prototype is never locked, so a call
to a function the same listing declares `char * sub_43ee(unsigned long *a0)`
produced an unknown of its width. Every caller that kept the result as a
`char *` then printed `v7 = (char *)sub_43ee(a0);`, a conversion from a type
the call does not have. `callrettype` (values `on|off`;
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_callrettype.rs`) carries the
return on the same callee-first surface (`decompile-all`, `decompile-project`).

**Recording.** After each function's final decompile in the callee-first order
(the same hook that parks `protoorder`'s statement, where
`park_recovered_proto` is set), `record` keeps the function's recovered return
value — storage, width and type — on `Architecture::kuna_callret_types`, keyed
by its entry, and a later decompile of the same function replaces it. Nothing
is kept for a function with a declared prototype (libc, `libctypes`, DWARF,
`--assert`: its callers already hold a locked output), for a `void` or
storage-less return, for a type other than a pointer, an integer wider than a
byte or a float (an unknown of its width is not a statement, and whether a byte
is a `bool` or a character is the caller's own call, `boolbyte` and `charbyte`),
for a pointer deeper than the inferred pointer cap, or when some live RETURN hands back a value the function never
computed (`kuna_returnuncomputed::every_return_computes_with`: a caller's
register left in place, a callee's clobber). A value read out of global
memory counts as computed here (`return stdout;` hands back the program's own
data), which the default walk does not grant. The `structsynth` convergence
sweep forgets a statement naming a superseded structure, as it does for
`protoorder`'s.

A function the run decompiles again after its callers (the `calleevote` redo,
the convergence sweep) records again, and its callers decompiled before that
keep the statement they read: nothing redoes a caller because its callee's
statement moved. A redo the run then discards (one that moves the arity, or
fails where the first decompile did not) puts the earlier statement back
(`kuna_callrettype::restore`), so the statement on record always describes the
body the run prints.

**The vote.** `seed` copies the statements for every callee a function calls
onto its `Funcdata` at the two seams `protoorder` seeds from; a function never
reads its own. `call_output_type_local` (the locked-output arm of
`TypeOpCall::getOutputLocal`) then answers for an unlocked CALL whose output
sits in exactly the storage and width the callee returns in
(`stated_return_type`), after `passthrough`'s tail-call arm. That one type is
the def-side vote of `Varnode::getLocalType`'s fold, so the caller's readers
still outrank it where their type is more specific, and it is the output token
`ActionSetCasts` compares the result's variable with: a caller that keeps the
result at the type the callee returns prints it without a cast, and one that
keeps it at another type prints the conversion from the callee's declared
type. Nothing is locked and no trial is touched, so the call keeps exactly the
arguments and the result it has with the option off; a `void` callee states
nothing and none of its calls starts being read.

The vote is refused where the caller holds evidence the fold cannot weigh:

- the family refusals `protoorder` applies to an argument vote, asked of the
  result's own uses (`kuna_protoorder::output_refuses`): a type-locked,
  global or frame-memory member, an integer operation on a pointer, a class
  conflict at another call, a float that is computed with as an integer, and a
  record the caller reads outside its members;
- a declaration the caller holds about the value (`declared_contradicts`). For
  an integer: an ordered comparison, a shift right, a division or remainder, an
  extension or a declared parameter that reads the value at the other sign, or
  another call writing the same variable whose declared or stated result has
  the other sign (`strcmp`'s `int` beside a recovered `unsigned int`); the vote
  would re-sign the variable and print a conversion at each of them. For a
  pointer: a declared parameter it is passed to, or another call writing the
  same variable whose declared or stated result is a pointer to a different
  known pointee. A declaration outranks a recovery (`getgrnam`'s `group *`
  beside a wrapper's synthesized `struct_8 *`), and two recoveries that
  disagree leave the variable to the caller's own fold: bash -O2 keeps one
  variable for `array_value`'s `struct_1 *` and `dequote_string`'s `char *`,
  and a vote for either one types the other's uses through the wrong pointee.
  A `void *` or a pointer to unknown bytes, on either side, says nothing about
  the pointee. Another call writing the same variable whose declared or stated
  result is of the other class at the same width, an integer beside a pointer,
  refuses the vote whatever the pointee: each would print the other as a
  conversion;
- for an integer, a widening at the other sign that the p-code no longer
  spells, of a value the function hands to a reader outside it: the result
  itself, or a value C computes from it at the same width (a sum, a product, a
  bit operation, a left shift, a negation), since C widens such a value by the
  sign of its own type (`widened_at_other_sign`). Two hand-offs lose their
  extension before any type is inferred. A call argument: the dead-bit
  trimming counts only the possibly-nonzero bits of a call input as consumed,
  so it narrows `RDI = ZEXT(x)` to `x` even for a callee that reads all of
  `RDI`, and records the slot (`kuna_truncarg`); a signed statement for a value
  that reaches such a slot is refused. `unsigned int r = neg32(x); return
  halve(r) + 1;` (`call neg32; mov %eax,%edi; call halve`) with an `int`
  statement would print `halve(v1)` with `int v1`, and C converts that to
  `halve`'s `unsigned long` by sign extension where the binary zero-extended
  it. The function's return: the return trimming (`RuleSubvarZext`,
  `RuleSubvarSext`) narrows a RETURN that reads a register through an
  extension back to the extension's input, leaving a plain copy; it records
  the sign and the width it narrowed from (`note_returned_extension`), and a
  statement of that width at the other sign is refused for a value the
  function returns. On x86-64 every 32-bit write zero-extends into the whole
  register, so the widening is not only the conversion a compiler spells in
  place (`return (unsigned short)s16(x)` in a function returning `long`:
  `call s16; movzwl %ax,%eax; ret`) but any last write of the returned value:
  the -O0 reload of the local holding the result, or the move back from the
  register it was kept in across another call (`unsigned int r = neg32(x);
  other(); return r;` in a function returning `unsigned long`: `mov
  %eax,%ebx; call other; mov %ebx,%eax`). An `int` statement would become the
  function's own return type, and `int f(...)` hands a caller that reads the
  whole register the value sign-extended where the binary hands it
  zero-extended. The same bytes are what `int f(x) { int r = neg32(x);
  other(); return r; }` compiles to, so that function keeps the unsigned
  return its own recovery gives it: from the function alone the two cannot be
  told apart, and only the unsigned spelling is right for both. A result the
  function returns with no write in between (`return neg32(x);`, `call neg32;
  ret`) is not widened here, and its statement stands. A store keeps its
  extension (a STORE consumes every byte it writes), so the extension stays a
  reader the rule above sees. A conversion spelled as a mask (AArch64 `and
  x0,x0,#0xffff`) is not recognized: an `INT_AND` is not a widening, and
  `RuleSubvarAnd` reports nothing;
- a pointer whose pointee the caller does not use as that pointee
  (`accesses_disagree`): a primitive pointee of N bytes read or written other
  than N bytes at a time, or offset or stepped by other than a multiple of N,
  and a `void *` offset at all. When the function returns the result, the other
  values it returns are counted too, since an undeclared return type follows
  the value to every RETURN (a word-at-a-time scanner stated `unsigned long *`
  must not become the return type of a function that steps its other result a
  byte at a time). A value the caller loads through the pointer that flows back
  into the pointer's own variable (`p = p[2]`) refuses it too: the variable
  would be both the pointer and what it points to.

**The audit.** The vote is taken while types are inferred, on the IR of that
moment, and the merge ties the return register whole-function after it in a
function that joins its returned values there (`mark_output_storage_addr_tied`,
`ActionMergeRequired`), so a call result the vote saw on its own can end up in
one variable with the function's own return. Whether the merge ties it is
decided on the IR propagation leaves, which does not exist yet when the vote is
taken (asked at inference time, the same predicate answered "no tie" for grep
`bmexec_trans`, which the merge then tied). After the caller's decompile,
`kuna-console`'s decompile step asks `kuna_callrettype::contradicted` for the
statements the finished function contradicts, withdraws them for that caller
(`Architecture::kuna_callret_refused`, read by `seed`) and decompiles the
caller once more. Only a statement that took is audited: the variable the
result ended up in carries the stated type, or, for a returned result, the
function's own return type became a pointer. A statement is contradicted when:

- the variable the result ended up in also holds another call's result of the
  other class, and beside a pointer a number that is not an address (a nonzero
  constant outside every data section the loader reported: `-1`, an error
  code), a value loaded through the pointer itself, or a sum or product of
  integers;
- the function hands back the result, directly or offset, and also hands back
  such a number, which would make its own return type the pointer and print
  every one of those numbers as one (`bmexec_trans` returns `-1` beside
  `p - buf`, and returns `ptrdiff_t`).

The second decompile costs what the first did, so a function over
`AUDIT_MAX_OPS` (1,000 live p-code ops) is not audited and keeps its first
decompile: on bash -O2 the audit redoes 26 functions for under a second of a
90-second run, where auditing every function would have cost 25 seconds, 24 of
them in twelve functions over that size, and dpkg-divert's one function of
1,992 ops alone cost 10% of its run. A vote a later inference pass refuses
while the result keeps the type (tar `sub_2ba80` shifts the result for a
`CONCAT71`) is left as it is: it costs a conversion, not a wrong type, and
auditing it redid 218 bash functions.

The witness is coreutils `du -O0` `map_inode_number`
(`uintmax_t map_inode_number (struct inode_map *, ino_t)`): its return register
holds `a1`, the `-1` sentinel, `ino_map_alloc`'s pointer on its way to a
field and `ino_map_insert`'s number, the merge ties all four, and a vote for
the pointer printed `long * sub_6b45(...)`, `v1 = (long *)0xffffffffffffffff;` and
`v1 = (unsigned long)sub_10624(...);` into the pointer. The audit withdraws
it, and the function prints exactly what it prints with the option off. The
fixture's `cached` is the same shape; its round trip asserts
`unsigned long cached(long *a0,unsigned long a1)` with the option on.

A caller whose other evidence types the result differently keeps a cast, now
from the callee's declared type: a pointer result stored into a field some
other access typed `long` prints `a0->field_0x48 = (long)sub_1563d(...)` (C
requires that conversion: the option-off `a0->field_0x48 = sub_1563d(...)`
beside `long * sub_1563d(...)` in the same listing assigns a pointer to an
integer without one), and a pointer result subtracted from
another pointer prints `(long)sub_10369(a0) - (long)a0`. Both compute what the
integer did. A function that returns a callee's result takes the callee's type
as its own return type.

A caller that refused the statement, or had it withdrawn, and keeps the result
as the other class converts it the same way
(`kuna_callrettype.rs (refused_token)`, which `ActionSetCasts` asks for a call's
output token, `decompiler/crates/kuna-decomp/src/p9_emit/coreaction_casts.rs
(get_output_token)`, where the call's own output type says nothing). The listing
declares the callee's return whether or not this caller took it, so find -O0's
`long v1 = sub_eecc(a0,v3)` beside `struct_56 * sub_eecc(..)` was not C, and
`*(unsigned int *)(sub_ef32(a0) + 0x24) = v` was pointer arithmetic C scales by
the `struct_56`. They print `v1 = (long)sub_eecc(a0,v3)` and
`((unsigned int *)sub_ef32(a0))[9] = v`. Only an integer beside a pointer of the
same width converts (a value-preserving conversion). A float statement beside an
integer or a pointer makes no token: no C conversion keeps a float's bits, and
gcc -O2's reader of a `struct { float, float }` returned in `xmm0` printed
`dat_4040 = (unsigned long)sub_11d0()` beside `double sub_11d0(void)`, which
stores 2 for the pair's 2.0000004. Such a reader withdraws the float return
instead (`voidret`, below). A float statement is the
token of a result the caller holds as a float or as raw bytes: nothing then
converts it, and a store of it through an untyped pointer takes the float's
type. Without it crazyflie printed `*(unsigned int *)((unsigned int)v2 * 4 +
a1) = sub_805bb84(..)` beside `float sub_805bb84(..)`, which C converts by value
(`floatret_cm4.o`'s `put2` stored 1.5 as 1); 29 of the 31 such stores on 27 binaries outside the cast
corpus (27,954 functions, most of them ARM firmware) print
`*(float *)`.

Measured on the 45-binary cast corpus (coreutils fmt/ls/sort/du/cp/tail/wc,
grep, gzip, diffutils cmp/diff/diff3/sdiff, tar, find at -O0, -O2 and
-O2-noinline), casts on the 4,815 functions kuna and IDA both emit go from
35,588 to 34,808 (0.941x to 0.920x IDA's count; 188.2 to 184.0 per thousand
lines, 29.9 to 29.3 per hundred statements): 393 functions fewer, 25 more.
The residue this leaves in `(char *)<call>` is dominated by callees kuna
recovers as `void` whose callers read the result (a wrapper ending in
`call; leave; ret` whose return is its callee's, `void sub_e8ca(...) {
sub_e7fc(0,a0,a1); }`): a `void` callee states nothing, and recovering such a
return is the callee's own prototype question, not this vote's.

Everything is inert where there is no callee-first pass: `kuna decompile`, a run
narrowed by `--addr`, `--functions` or a triage filter, `--jobs N`, a raw image
and `--option protoorder off`. A single-function decompile therefore still
prints the conversion that the whole-binary listing leaves out, the same
property `protoorder`'s argument types have.

### ARM scalar VFP contracts

The ARM default model (`ARM.cspec`) lists the VFP registers only as the 4-byte
`s0`-`s15`. A hard-float function returns a `double` in `d0`, which overlaps
`s0` and `s1`, so return recovery cuts it to its low word:
`vmov.f64 d0,#1.5; bx lr` prints `unsigned int fixed(void) { return 0; }`, and a
`double` parameter read from `d0` is never a parameter at all.

`armfloatreturn` (off by default; `decompiler/crates/kuna-decomp/src/p4_calls/kuna_armfloatreturn.rs`)
changes this only for an ARM ELF whose container states the VFP procedure-call
standard. The loader decides that once per image
(`decompiler/crates/kuna-analysis/src/loader/kuna_armfloatabi.rs`): a linked
EABI5 executable or shared object must carry the hard-float ABI flag in its
header, and any `.ARM.attributes` it has must agree; a relocatable object must
say `Tag_ABI_VFP_args=1` in `.ARM.attributes`. Soft-float, `softfp`,
conflicting, malformed, section- or symbol-scoped attributes, a raw image and
the Ghidra front-end all state nothing, and the option then changes nothing.

On such an image each function's copy of the default model gains an 8-byte
float entry for every `d0`-`d7` register over its two s-register entries, on
input, and one for `d0` on output; `d1`-`d3` are not return storage. A whole
double fills both single-register groups when holes are filled, so the
s-register it covers is not a missing parameter, and an unused `d` slot below a
used double parameter is filled by one 8-byte parameter rather than two 4-byte
ones (`double second(double x, double y) { return y; }` keeps `y` second). An
unused parameter that only fills a VFP slot below a float-typed one is typed
`float` or `double`, so the printed prototype still puts that parameter in its
register; below an integer-typed half it keeps its default type.

Each added `d` entry is one register, the double-width view of its two `s`
entries, not a join of two pieces: overlap resolution would give it the
per-piece checks of a containing entry, which drop a trial formed on some path
by what a call left behind, and a lower standing than the first float entry.
It gets neither, so a return that is a call's result on one path and computed
on another (`if (x > 1.0) return half(x); return x * 3.0;`) is judged as it is
in `s0`, and both paths keep their value instead of the function printing
`void`.

An 8-byte `d` input that no op reads whole -- every read, through casts, copies
or a right shift by 32, ends in a 4-byte piece -- holds two floats in `s0` and
`s1`, not a double, and is not made one 8-byte parameter. It keeps that slot only
when a later VFP input is read, so the later parameter keeps its position.

Return trials are scored as usual, and the wider `d0` entry then wins the
fill-in over a 4-byte `r0`. A value a call left in `d0` is therefore retired
first: when an integer return trial holds, at every RETURN, a value the
function computes and only returns (not a call's output), a VFP return trial
whose value at every RETURN is only what a call left in the register is marked
inactive. `half(x); return k + 1;` returns `k + 1` in `r0`, not `half`'s
double; a function that returns `half`'s result and merely uses `r0` as scratch
keeps the double. The machine code cannot tell every case apart: a `void`
function that calls `half` last and leaves `d0` alone, or an `int` stored as
well as returned, looks like one returning `half`'s double. After the output trials are scored, a single used trial in
`s0` or `d0` gives the returned value a float or double type before constant
folding can drop its storage.

A float result is written to `s0`, the low half of `d0`, so a function that
converts a double to float leaves `d0` holding the new low word and the old high
word. Before the return width is fixed, a bounded walk checks every normal
return: if each one returns such a piece -- a fresh 4-byte write joined to the
high word of an earlier `d0` value, through copies and joins, where a
predicated join may also carry that earlier value itself -- the trial shrinks
to `s0`. Whole doubles, mixed-width exits, reassembled halves of one double,
and incomplete or cyclic proofs keep the 8-byte trial. The walk cannot tell a
float result from code that deliberately edits the low word of a double; that
code needs an explicit `double` output contract, which skips the walk.

At a call, a live `d` register becomes an 8-byte argument only when the callee
says it takes one: a locked prototype with a parameter there, or a prototype
`protoorder` recovered (in a callee-first `decompile-all`) whose list is
arity-sound and has a float parameter in that register. A live `d0` alone at an
unknown call is not an argument. For that arity check a recovered VFP parameter
reads the recovered list as closed, because a variadic callee receives even its
fixed floats in `r0`-`r3`.

Declared prototypes and explicit return storage keep precedence throughout.
This is scalar inference, not aggregate or vector ABI reconstruction: a
homogeneous float aggregate returned in `d0`-`d3` (`struct { double a, b; }`,
`_Complex double`) prints as a `double` holding only its first member, and needs
a declared type. The stage test `tests/stages/kuna-arm-float-return.xml` runs a raw image,
a relocatable object and a linked PIE with the option off and on; the CLI tests
in `decompiler/crates/kuna-cli/tests/arm_float_returns.rs` cover narrowing,
widening, metadata controls and explicit contracts. ABI references:
[AAPCS32](https://github.com/ARM-software/abi-aa/blob/main/aapcs32/aapcs32.rst),
[AAELF32](https://github.com/ARM-software/abi-aa/blob/main/aaelf32/aaelf32.rst), and
[Addenda32](https://github.com/ARM-software/abi-aa/blob/main/addenda32/addenda32.rst).

### A function whose result a caller reads returns it (`kuna_voidret.rs`)

`call g; ret` is what both `void f(void) { g(); }` and `T f(void) { return
g(); }` compile to, and the function alone cannot tell them apart: the value in
the return register at its RETURN comes straight from a call, which
`ancestor_op_use` refuses ("a call is never a good indication of a single
parameter"), and a wrapper that never names the register gets no return trial
at all, because heritage registers one only for a range some op reads or writes.
So every such wrapper was recovered `void`, while its callers, compiled against
a declaration that returns a value, read the register after the call and
printed the read: `v6 = sub_18a0f(4,v22)` beside `void sub_18a0f(unsigned int
a0,char *a1)`, and for a float `v1 = (float)qnan()` of a `void qnan(void)`.
Neither is C. Over the 45-binary cast corpus main printed 2,126 such uses.

This is a correction, not an option, and it lives on the callee-first surface
(`decompile-all`, `decompile-project`). After each function's final decompile,
`decompiler/crates/kuna-decomp/src/p4_calls/kuna_voidret.rs (record)` files what
the function returns (`void`, a float, or another value; nothing for a declared
prototype) and, for every call it makes, the storage it reads of the call's
result: the bytes the result's uses consume, at the register's least significant
end (`kuna_voidret.rs (read_storage)`), so a `movss` of an `xmm0` result reads
four bytes whatever width the call's output was recovered at. Where
`ActionSetCasts` converted the result and left the call writing a temporary only
the conversion reads, the conversion's output holds the register, so the storage
and the type the reader holds the result as are read there
(`kuna_voidret.rs (holder)`); the temporary carries only the call's own type.
A wrapper every caller converts (`v7 = (char *)sub_a7d7(..)`) is redone like any
other. A joined pair
(`rustabi`'s `rax:rdx`) is filed whole. Every read is filed whatever the order,
because a caller is not always decompiled after its callee (a call the call
graph missed, a cycle). `kuna_voidret.rs (due)` then names the `void` functions
a caller reads a result from, each with the widest storage its callers read
(callers that read different registers refuse the function), and the driver
(`decompiler/crates/kuna-cli/src/decompile_all/callee_first.rs (VoidReads::settle)`)
decompiles them again in plan order. It settles after every function of the
callee-first plan, not once the plan is done: a wrapper is redone as soon as its
first reader is decompiled, so every later reader reads the wrapper's return
the first time it is decompiled. A reader decompiled later that reads the
wrapper wider forces it again at that width, and one that reads another
register withdraws it: it returns nothing again, as before.

A function recovered returning a value in storage that shares no byte with the
register its callers read is forced the same way (`kuna_voidret.rs (displaced)`).
gcc's `-fzero-call-used-regs` ends a function by zeroing every call-clobbered
register but the return register, so in `int put(int *p, int v) { int r =
check(p); if (r) return r; *p = v; return 0; }` the epilogue's `pxor
%xmm0,%xmm0` made `xmm0` a realistic constant return while `eax`, a call's
tested result, was refused: `put` printed `double put(..)` returning `0.0` on
both paths beside callers reading `eax`, and openssh's `xmalloc` family printed
`unsigned long xmalloc(..) { ... return 0; }`. Forced to `eax`, the trial there
is accepted and the model maps the output to it. The forced decompile comes
before the withdrawal of a float return the callers refuse
(`Ledger::displacing`); a function that still returns the float afterwards is
withdrawn as before.

In that decompile the function's return storage is seeded
(`kuna_voidret.rs (seed)`, the register pieces of a joined return such as a
`struct timespec` in `rax:rdx`), together with what each of its callees was last
recovered to return. Unless an op of the function names storage that reaches
outside it, `kuna_voidret.rs (plant)` gives every live RETURN a read of it and
registers the return trial itself, the way `passthrough` does for a claimed tail
call, and heritage's `guardReturns` leaves that piece alone
(`kuna_voidret.rs (planted_overlaps)`). A name inside the storage does not stop
it: heritage sizes its own trial by the range the function names, and gcc -O1's
`key_ssh_name`, `call key_type_plain; call type_name; ret`, names only the
`eax` of the first call's result, so it returned `unsigned int` around
`type_name`'s pointer while its caller read all of `rax`. Only the pieces planted are left out: in
i386 `call zsum; or $0xff,%edx; ret`, whose callers read `edx:eax`, `eax` is
planted and the `edx` the function writes keeps the trial heritage gives it.
Where paths meet before the RETURN, heritage refines a planted range wider
than every write at the narrower name the function uses and merges each piece
on its own, which no rule joins again; such a trial is refused
(`kuna_voidret.rs (merges_in_pieces)`). gcc -O1's `wrap`, `call pick; test
%eax,%eax` and then one of two `getname` calls before a shared `ret`, returned
`CONCAT44(dat_4,v2)` and stored the upper half of `getname`'s pointer into
`dat_4`, a register piece printed as a global, on each path; it stays `void`.
`ActionReturnRecovery` then marks a
trial on that storage active (`kuna_voidret.rs (score_forced)`) only when the
value is the return value at EVERY live RETURN: `AncestorRealistic` accepts it,
and `ancestor_op_use` finds it used only on its way to the RETURN, with two
changes. A call's result (or its INDIRECT creation) that nothing else uses
counts, where upstream refuses it outright
(`Funcdata::kuna_forced_scoring`). And the value may also be stored, branched
on, or handed back in another register (`kuna_voidret.rs (returned_use)`,
`kuna_voidret.rs (scoring_return)`): `gi = a * 3; return gi;` stores the `eax`
it returns, `if (r > 7) gj++; return r;` tests it, and `x = *p; gi = x + 1; gj
= x; return x;` leaves `x + 1` in `edx`, a register the callers do not read.
Upstream's `onlyOpUse` refuses all three, to tell a value passed in a register
from a variable that happens to sit there; with a caller reading the register
that question is settled, and the refusal printed `void f(int a0) { gi = a0 *
3; }` beside `printf("%d\n", f(2))`, and gnulib's `xmalloc` (`p = malloc(n);
if (!p) xalloc_die(); return p;`) as `void` beside every caller using its
pointer. Single-function decompiles have no caller to settle it and keep
upstream's answer. A load or store through the value and a call reading it still
refuse it. So a function that leaves its caller's register in place stays
`void`, and so does one that uses the register as scratch: a stream pointer in
a `getc` loop, or a message handed to an `error(nonzero, ...)` whose
fall-through is pruned into a RETURN.

The trial is returned no wider than the callers read and than every path sets
(`kuna_voidret.rs (defined_width)`). Heritage sizes the trial by the range the
function touches, so gcc -O0's `if (tz) return setenv(..); return unsetenv(..);`,
which loads a string address into `rax` before each call, gets an 8-byte `rax`
trial whose value at the RETURN is `PIECE(<killed by the call>, eax)`: taken
whole, the function returned `unsigned long` and printed `return v2;` of a
variable nothing assigns. Following the value back through copies, joins and
pieces, a register a call kills (an INDIRECT creation on an indirect-zero), the
function's entry value of a register no parameter arrives in, and the part of a
call's possible result outside the callee's declared or stated return storage
(`kuna_voidret.rs (returned_by_the_call)`: the rest of `xmm0` beside a `float`)
set nothing, and the least significant bytes every path sets bound the width.
A narrower width returns a `SUBPIECE` of the value in the narrower register
(`kuna_voidret.rs (narrow)`), which the subvariable rules pull back through the
joins to the calls, so the wrapper returns `int` and the value its calls
compute. That also keeps the register's other values out of the return
variable: taken at `rax`'s width, cp's `overwrite_ok` merged the `fprintf`
arguments it computes in `rax` with the `bool` it returns into one `char *`, and
tar's `argp_parse` branched on the `int` error through a variable the merge left
unassigned. A trial above the least significant end of the storage the callers
read (the upper half of a `double` split in two) is returned only beside an
accepted lower one. A return the callers read in several trials is returned
whole or not at all (`kuna_voidret.rs (whole_or_none)`, after the model maps the
trials): when the model leaves out a trial that every path sets -- the `edx` of
a 32-bit `call ins16; ret`, which upstream's multi-precision check takes for a
register the call clobbered -- every trial on the callers' storage is dropped and
the function stays `void`, since `eax` alone would hand its callers the low half
of a 64-bit value. A path that sets nothing refuses the trial; when what it
would return is the result of a callee still recovered `void`, the function files
a read of that callee (`kuna_voidret.rs (void_results)`), which makes the callee
due, and returns nothing until the callee does. The read is of all the storage
the function's own callers read, where the piece it hands on lies inside it
(`kuna_voidret.rs (as_wide_as)`): asked for `eax` alone, a callee handing on a
pointer returned `unsigned int` and its wrapper then took that width; asked for
`rax` in two pieces, it was asked for two registers and stayed `void`.

A trial narrower than what the callers read is refused, and the function stays
`void`, where a path returns what a callee returns wider, or of a width nothing
states (`kuna_voidret.rs (hands_on_unstated)`): the value is followed back
through copies, phi-nodes, pieces and the calls an INDIRECT guards to the calls
whose result it is, and the callee's declared output, its `callrettype`
statement, or the storage its own last decompile returned a value in (`record`
files it beside what the function returns; a `bool` is stated to no one) gives
the width (`kuna_voidret.rs (callee_return)`). This is the
net under the plant above, for a trial heritage sized when an op names storage
reaching outside the callers' read. A trial that may carry a
float is refused the same way where a path returns what a named callee nothing
declares returns: a floating-point register of the model's outputs, or any
return storage on an image that does not state a hard-float convention. A
relocatable object declares no `strtod`, and `double parse(const char *s) {
return strtod(s, 0); }` printed `unsigned long parse(..) { return
strtod(a0,0); }`, which converts the `double` by value once compiled against
`<stdlib.h>`, whether the reader keeps the bits as an integer or stores them as
a `double`; soft-float ARM printed `unsigned int parsef(..)` around `strtof`. A
call through a pointer is typed by the listing itself (an import stub's
`(*dat_4018)()`), so it does not count.

The value's type is whatever it is: the callee's stated return through `callrettype`, a float for a float
register (chapter 05). A forced function whose RETURN carries no value once the
returns are built is made `void` (`kuna_voidret.rs (void_unless_returned)`),
as upstream's `updateOutputTypes` would. The port's restart keeps the output an
earlier pass recovered, and `ActionOutputPrototype` leaves the output alone
when the RETURN has no value: a MinGW `-O0` function whose `mov eax,[iat];
call eax` resolves to `ExitProcess` restarts, the pass before the restart
returns `eax` joined with what the unresolved call left, the pass after it
refuses the bare handle where the returning path also loads through it, and the
function printed `int f(..)` around a bare `return;`.

A redone wrapper reads its own callee's result in turn, so each settling
repeats, up to ten rounds, reaching one function further down a chain of
wrappers (to the import stub at its end) and, once a wrapper returns, one caller
further up it. The redo keeps what the function's statement and recorded return
were before it (`callee_first.rs (VoidReads::redo_in_plan_order)`), and a
function whose redo changed either has every reader decompiled before that redo
decompiled again (`kuna_voidret.rs (stale_readers)`) when it now states a return
to `callrettype`, returns a float, or had its float return withdrawn; a callee
that states nothing has nothing to hand a reader, and only a wrapper still
waiting on it is redone. Such a reader typed the
call's result itself, against a callee it saw return nothing, and kept that
text: cp's callers printed `unsigned long v10 = sub_18a0f(4,a1)` beside `void
*sub_18a0f(..)`, which is not C, and find's printed `*(unsigned int
*)(sub_ef32(a0) + 0x24) = v`, which C scales by the `struct_56` the callee now
returns and so writes far past the field. Redone, the reader takes the
statement through `callrettype`, or, where it refuses it, converts it
explicitly (`callrettype`'s refused statements, below). A reader over
`AUDIT_MAX_OPS` (1,000 live ops) is redone only where its stale text computes a
wrong value: an offset taken in place from a result the callee now declares a
pointer to something wider than a byte, a float result held as something
else, or a float held from a callee whose float return was withdrawn. Its other stale text is an assignment between a pointer and an integer of
the same width, which the listing leaves unconverted: redoing every such reader
cost bash -O2 30 seconds, one `execute_command_internal` alone ten.

A float return a reader keeps as another type is withdrawn
(`kuna_voidret.rs (withdrawals)`). No C declaration serves a reader that holds
the result in an integer, `unsigned int v3 = clampf(..)` stored through an
`unsigned int *`, and one that uses it as a float: `v3 = clampf(..)` converts
the float by value where the machine moved its bits. `record` files the reader
(`kuna_voidret.rs (held_as_float)`), whatever storage the call's output sits in,
when the result, or any copy of it, is not
a float, is cast to something else, or reaches a place C converts it by value:
an argument whose parameter the call's declaration, or the callee's own
decompile, types as an integer or a pointer (`f2u(getf(p))` beside `unsigned int
f2u(unsigned int)`), a store through a pointer to an integer or a pointer, or a
RETURN of a function that does not return a float. The callee's parameter types
come from `protoorder`'s statement or, where it states nothing, from the
parameter classes `record` files for every function without a declared
prototype, handed to each caller by `seed` (`Funcdata::kuna_callee_param_float`).
A function whose own return converts the value it hands back from another type
(`return (float)a0[3];` of an `int *`) files itself. The function is then
decompiled once more without the float-register vote on its return
(`Funcdata::kuna_float_return_withdrawn`, chapter 05) and without a forced
return, so it declares what it did before this redo and the float vote (an
integer, or `void`), and its readers are decompiled again against that.

A function that hands on a callee's float return keeps returning a float once
withdrawn: gcc -O1's `double wrapd(p, i) { return getd(p, i); }` returns the
`double` `getd`'s own vote gave it, and the reader's `dat_40a0 = wrapd(..)`,
which copies the eight bytes into a `uint64_t`, converted 2.25 to 2. So `record`
files, for every function returning a float, the callees last recovered
returning one whose result reaches a live RETURN through copies, casts, pieces
and phi-nodes (`kuna_voidret.rs (float_sources)`), and `withdrawals` withdraws
those with the function, and theirs in turn, down the chain of wrappers. The
chain is redone in plan order, callee first, so each wrapper is decompiled
against the integer its callee now returns; a reader of the chain that used the
getter's result as a float directly goes back to what it printed before the
float vote. A callee whose float return is declared (a libc row) or computed by
its own float arithmetic is not withdrawn: its float is the type the machine
computes, and a reader holding it as an integer prints what it did without the
vote.

A forced function whose final decompile still returns, on some path, a register
a call only clobbers -- an INDIRECT creation the call's output never replaced --
is withdrawn the same way (`kuna_voidret.rs (returns_a_call_clobber)`, filed by
`record`). Scoring accepts a call's INDIRECT creation because the call normally
gains that Varnode as its output later, when `ActionActiveReturn` finds it among
the INDIRECT ops right before the call; gcc -O0's `call_f2u(float f) { return
f2u(f + 1.0f); }` builds `f2u`'s argument from two registers, the `PIECE` lands
between the creation and the call, the call never gains the output, and the
function printed `f2u(..); return v1;` of a `v1` nothing assigns. Withdrawn, it
is `void` again.

Two shapes stay outside it. A 16-byte return whose callee states only its low
half (sort's `dtotimespec` hands back `make_timespec`'s `rax:rdx`, and
`make_timespec` itself is recovered as `rax`) becomes an 8-byte return, not a
pair. And a call whose return register the caller heritages whole (gcc writes
all of `xmm0` with `movd`, `pxor` or `movq` before a call) has no output trial
and no read to file (`calloverlap`, chapter 03).

On the 45-binary cast corpus, uses of a `void` function's result, converted ones
included (`v7 = (char *)sub_a7d7(..)`), fall from 5,031 to 1,303, a pointer and
an integer assigned to each other without a conversion from 275 to 162, and no
function prints a new never-assigned return value, a new `CONCAT44` of a killed
register, a new explicit integer-to-pointer conversion, a new value conversion
of a float (tail's `main` keeps the four `(double)lseek(..)` it printed of a
`void lseek`, now of an `unsigned long` one), or new pointer arithmetic C would
scale. The 47 pointer and integer assignments that are new sit in 13 functions,
readers over the size cap or variables the merge left untyped (`xunknown8`,
printed `unsigned long`), and each reads a result of a function that was `void`
before, which was no more C. Most uses left are of functions whose return value
is also compared, indexed or passed on before it is returned (coreutils
`base_len` returns the length its loop tests, grep's `xmalloc` the pointer it
tests against `NULL`): upstream's sole-use rule, which this redo keeps, refuses
them, and relaxing it to accept a tested value brought the scratch-register
merges back.

The same ledger settles a narrow return the function zero-extends (GH-865). On
x86 (32- and 64-bit) and AArch64, `movzbl` or writing `eax` or `w0` clears the
rest of the register, and sub-variable flow trims such a return to the narrow
value
(`kuna_zextreturn`, chapter [05](05-types.md)); the conventions leave the bits
above it unspecified, so the function alone cannot tell `int z32m(..)` from
`unsigned long z32m(..) { return (x * 3) & 0xffffffff; }`. A caller that
computes with the register above the narrow value relies on the zero-extension:
gcc's `call z32m; add $1,%rax` printed `z32m(..) + 1` beside `int z32m(int a0)`,
which C sign-extends, and returned `0xffffffff80000002` where the binary returns
`0x80000002`. The RETURN pull notes each trim of a zero-extended value, and
whether its sign bit may be set (`Funcdata::kuna_note_zext_word`), and `record`
files the narrow storage when the final return is that trim
(`kuna_voidret.rs (file_word)`). For every call it files, besides the storage
its uses consume, the storage its operations compute with
(`kuna_voidret.rs (used_storage)`): the result is followed through copies,
joins, pieces, extensions, and shifts and masks by constants, which move bits
without computing with them, and a RETURN or a call's argument only hands the
register on, unless a shift on the way moved the upper bits where it takes them
(`return (long)z6(x) >> 1`, `sink((long)z8(x) >> 2)`). An unshifted 64-bit
result handed straight to a call (`sink(z9(x))` with a `long` parameter) stays
unread, so `z9` keeps its `int`. Consumption alone was no evidence: a `bool` function that tests a
comparator's result with `sete %al` and returns `rax` whole, its upper bytes
still the comparator's, consumed all of the comparator's register (grep's
`string_compare_ci` over `mbscasecmp`), as did every wrapper that returns an
`int` call's result. Where a function returns a callee's result as it is
(`kuna_voidret.rs (hands_on)`), what its own callers compute with is filed for
that callee too (`kuna_voidret.rs (file_use)`), so `unsigned long w(x) { return
z32m(x + 7); }` widens `z32m` once `w`'s caller adds to `w`'s result in 64 bits.
`due` then names each function whose noted value a caller computes with wider,
where that can change it (`kuna_voidret.rs (widens)`): with more than four
bytes, or with up to four of a value whose sign bit may be set and which is
typed a signed or unknown integer, since a `bool` or an `unsigned char` is
already promoted as the binary extends it. It is named with the widest such
storage, and `seed` hands that to its next decompile
(`Funcdata::kuna_wide_return`), where the pull keeps the whole register for a
caller computing with eight bytes and otherwise makes the narrow value unsigned
(chapters [03](03-ssa-and-simplification.md) and [05](05-types.md)). A
function whose return storage such a redo widened has every reader decompiled
before it decompiled again (`stale_readers`): a reader that took the result as
the narrow value prints `z32m(..) < 0` of an `int`, which the new `unsigned
long` declaration makes false. A reader over `AUDIT_MAX_OPS` is redone only
where it uses the result's sign or width (`kuna_voidret.rs (signed_use)`: an
extension, an ordering, a right shift, a division, a conversion to a float, or
a comparison with a negative constant, after copies and arithmetic that keeps
the low bits); e2fsck's `main`, which tests such a result for zero and passes
it on, spent three seconds on a redo that renamed one variable. A convention that extends a narrow return
by its type's sign (PowerPC64's `inttype`) is left alone: there a caller may read
the whole register of an `unsigned int`, which `kuna_zextreturn` already types.
The single-function `kuna decompile` has no callers to ask and keeps the trim.
Over 24 decbench x86-64 binaries and 190 AArch64 objects built from their
sources, it changes 15 functions, each toward its source type: kmod's
`kmod_module_dependency_symbol_get_bind` at three levels and dash's AArch64
`arith_prec`, a zero-extended byte their callers compare as an `int`, from
`char` to `unsigned char`; gnulib's `default_block_size`, a `uintmax_t` of 512 or
1024, from `int` to `unsigned long` in du, ls and find, with its reader
`humblock`'s `uintmax_t *` from `long *` to `unsigned long *` and du's `main`
passing it a typed global; find's `get_format_specifer_length`, a `size_t`,
from `char` to `unsigned long`, with its reader renumbered; and dash's
`hashvar`, which computes a pointer in 32 bits, from `int` to `unsigned long` on
both architectures.

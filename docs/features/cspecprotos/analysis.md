# cspecprotos - the prototype-model registry could only ever hold one model

## Symptom

`kuna decompile ... --option defaultprototype __thiscall` fails on every target:

```
Execution error: Unknown prototype model :__thiscall
Aborting process
```

even on an x86 PE binary whose compiler spec has *defined* `__thiscall` since the spec
was vendored (`specs/Ghidra/Processors/x86/data/languages/x86win.cspec:156`).

Downstream, every MSVC C++ member function loses its `this` pointer. On
`decompiler/crates/kuna-analysis/tests/fixtures/msvc_rtti_x86.exe`:

```c
int4 Box::vftable_0(void)
{
  int4 v1; // ecx
  return *(int4 *)(v1 + 4) * *(int4 *)(v1 + 4);
}
```

`v1` is read before it is ever assigned - not valid C - because ECX carries the `this`
pointer under `__thiscall`, and the model kuna evaluates the function with (`__stdcall`)
has no register `<pentry>` at all.

## Root cause

kuna's `this`-pointer machinery is **fully ported, wired and consumed**:

| Piece | Where |
|---|---|
| `ProtoModel::hasThisPointer` | `p4_calls/fspec.rs` (`has_this_pointer`) |
| `FuncProto::hasThisPointer` / `updateThisPointer` / `getThisPointerStorage` | `p4_calls/fspec.rs` |
| `ParameterPieces::isThisPointer` | `p4_calls/fspec.rs` |
| driver (`update_this_pointer` at the end of input recovery) | `p4_calls/coreaction_protos.rs` |
| consumer (a `this` param pointing at a `TYPE_STRUCT` is treated as effectively type-locked) | `p5_types/coreaction_infertypes.rs` |

All of it is dead, because **nothing can set `hasThis`**. `hasThis` comes from a model
whose name is `__thiscall` (or which carries `hasthis="true"`), and the only producer of
a `ProtoModel` in the live engine was `Architecture::build_default_proto`, which read
exactly one element: `<default_proto><prototype>`. The C++
`Architecture::parseCompilerConfig` (`architecture.cc:1253/1280-1284`) additionally
dispatches `<prototype>`, `<resolveprototype>`, `<modelalias>`,
`<eval_called_prototype>` and `<eval_current_prototype>`; kuna's reader dispatched none
of them. `ProtoModel::decode` (the marshaling-decoder entry point) is a hard `Err` stub.

So `Architecture::proto_models` held exactly **one** entry per program, and every lookup
into it - `option defaultprototype`, `option protoeval` - could only ever return the
default model or fail.

Secondary consequence: `ProtoModelMerged` (the union model whose `resolveModel` scores
each constituent against the observed trials) was ported, unit-tested, and had **no live
producer**, because its only producer is `<resolveprototype>`.

## What this change does

Registers what the spec declares, and nothing else:

* `<prototype>` - decoded through the *same* body as the default one
  (`decode_proto_model`), so a named model carries identical `<pentry>` storage and
  `<unaffected>`/`<killedbycall>`/`<returnaddress>`/`<internal_storage>` effect fidelity.
  It additionally reads the `hasthis` and `constructor` attributes, and applies upstream's
  `if (name == "__thiscall") hasThis = true` override (`fspec.cc:2595`) *after* them.
* `<resolveprototype>` - folds each `<model name=..>` constituent into a `ProtoModelMerged`
  and finalizes the merged input list (`ProtoModelMerged::decode`, `fspec.cc:2904`).
* `<modelalias>` - a named copy of an already-registered parent, which stays
  `isCompatible` with it (`architecture.cc:1310`); aliasing a merged model or an alias of
  an alias is refused, as upstream refuses it (`createModelAlias`, `architecture.cc:1137`).
* the post-parse invariant "we must have a `__thiscall` calling convention"
  (`architecture.cc:1342`): a spec that declares none gets one cloned off its default, so
  `getModel("__thiscall")` resolves on every language, not just the x86 family.

A named model that fails to decode is skipped rather than aborting the spec (upstream
throws). The vendored cspec corpus spans every processor kuna ships; one undecodable
named model must not cost the architecture its default one.

## What this change deliberately does NOT do

**Registration selects nothing.** No function's model changes, so emitted C is
byte-identical everywhere (measured: 675/675 datatests unmoved, 11 whole-binary
`decompile-all` sweeps byte-identical). The registry is the enabling substrate; the
selection decisions each need their own evidence and their own PR:

* `<eval_current_prototype>` / `<eval_called_prototype>` - honoring these changes which
  model **every** function on x86win / x86-64-win / x86gcc / HCS12 / CR16 is evaluated
  with. That is a default change, not registration, and belongs behind its own option
  with its own measurement.
* Automatic `__thiscall` assignment to member functions (from the MSVC/Itanium demangler,
  or from DWARF `DW_AT_object_pointer`).
* `FuncProto::decode`'s Java-tier `constructor`/`destructor` import, `prepareThisPointer`
  (which synthesizes a `this` parameter when the model has one and the recovery did not
  find it), and `hide_thisparam` (the printer's `this`-parameter suppression).

## Why no option

`AGENTS.md`: an option is required for anything that *can change emitted C*. This cannot.
The only new reachable state is a registry entry that nothing reads unless the user
explicitly names it via `option defaultprototype` / `option protoeval` - options that
already exist and that previously had exactly one legal value. Proven, not assumed:
675/675 datatest assertions unmoved and 11 byte-identical whole-binary sweeps (`plan.md`).

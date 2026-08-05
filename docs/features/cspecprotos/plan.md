# cspecprotos - implementation plan and measurements

Root cause and design rationale: `analysis.md`. This file is the plan as executed plus
the numbers behind it.

## What changed

| file | change |
|---|---|
| `decompiler/crates/kuna-decomp/src/infra/architecture.rs` | `decode_default_proto` split into a thin `<default_proto>` locator plus a reusable `decode_proto_model(<prototype>, root)` (which also gains the `hasthis`/`constructor` attribute reads and the `__thiscall` name rule); **new** `decode_named_protos` (the `<prototype>`/`<resolveprototype>`/`<modelalias>` dispatch plus the guaranteed-`__thiscall` tail) and `decode_resolve_proto` (the `ProtoModelMerged` fold-in); **new** free fns `decode_bool_attr` and `create_model_alias`; `build_default_proto` registers the named models after the default one |
| `decompiler/crates/kuna-decomp/tests/verify_cspecprotos.rs` | **new** - 5 registry assertions over four real vendored cspecs |
| `tests/stages/ghdec-cspecprotos.xml` | **new** - two-pass end-to-end witness, 6 assertions |
| `decompiler/crates/kuna-base/src/xml.rs` | corpus file count 191 to 192 (read off a live failing run on the rebased tree, not arithmetic) |
| `docs/baseline-stages.json` | re-recorded from a live run on the rebased tree: 394 to 400 keys, purely the 6 new ones |
| `docs/spec/04-calls-and-prototypes.md` | 4.1 rewritten: the named-model registration, the deliberate non-selection, and the merged-model producer |
| `docs/history.md` | Convergences row (no DIV: no default changes, no corpus output shift) |

No option, no `phases.toml` row, no catalog-count bump, no `docs/options.md` regeneration:
registration cannot change emitted C, so there is no decision to expose. The two options
that read the registry (`defaultprototype`, `protoeval`) already exist; this change is
what gives them more than one legal value.

## Decisions

1. **Lift the existing decode body rather than implement `ProtoModel::decode`.** The
   `<pentry>` storage resolution needs the engine `Translate` (register lookup) and the
   `AddrSpaceManager`, both of which live on `Architecture`; the marshaling-decoder entry
   point in `fspec.rs` has neither. Reusing `decode_proto_model` for *both* the default
   and the named models is what guarantees a named model has identical storage/effect
   fidelity - including the top-level `<returnaddress>` fallback and the
   `kuna_dfunaffected` DF assertion - rather than a second, weaker parser.
2. **`set_name` last.** Upstream reads all attributes, then applies
   `if (name == "__thiscall") hasThis = true`. kuna's `ProtoModel::set_name` already
   carries that rule, so the attribute reads are ordered *before* it; an explicit
   `hasthis="false"` on a model named `__thiscall` therefore loses, exactly as upstream.
3. **Skip a bad named model, do not abort the spec.** Upstream throws
   `LowlevelError` and loses the architecture. kuna's cspec corpus spans every vendored
   processor and the loader path already degrades gracefully on a failed default decode;
   an unknown `strategy` or a `<pentry>` naming a register some language lacks now costs
   that one model, not the program.
4. **Do not honor `<eval_current_prototype>`/`<eval_called_prototype>`.** Both x86win and
   x86gcc declare one, and honoring it would hand *every* function on those targets the
   merged `__fastcall/__thiscall/__stdcall` (resp. `__cdecl/__regparm`) model. That is a
   default change with its own evidence burden, not registration. Deferred with its own
   option and its own measurement.
5. **Register `<modelalias>` too**, even though only three cspecs use it. It is the same
   `parseCompilerConfig` arm, it is one `copy_named` call, and leaving it out would make
   x86-64-win's `__cdecl`/`__stdcall` the only *named* conventions a user cannot select
   on a Windows x64 target.
6. **Port the guaranteed `__thiscall` too** (`parseCompilerConfig` tail,
   architecture.cc:1342): when a spec declares no `__thiscall`, upstream clones one off
   the default under that name. Most languages are in that case, and the follow-up
   (demangler / DWARF-driven `__thiscall` assignment) needs `getModel("__thiscall")` to
   resolve on more than the x86 family. It is still pure registration - nothing selects
   it - and it is what makes `option defaultprototype __thiscall` work on an ELF target.
7. **One cspec parse, not three.** The first cut left `decode_named_protos` parsing the
   document independently, on top of the return-address parse and the default-proto parse
   that already existed. That measured +2.8% / +4.3% on the two smallest PE loads - inside
   budget, but paying it for no reason. `build_default_proto` now parses once and hands the
   root to all three readers, which is a net *removal* of one parse versus the pre-PR tree.
8. **No option.** `AGENTS.md`'s "a strict bug fix that only corrects wrong output needs no
   flag" is the weaker case; this is stronger still - nothing about default output moves.
   Proven by the datatest ablation and the byte-identical sweep below, not assumed.

## Evidence

### Gates

| gate | result |
|---|---|
| `make test` | PARITY OK, 675/675 |
| `make test-stages` | PARITY OK, 400/400 (394 + the 6 new keys) |
| `make rust-test` | green |
| `make check-spec` | check-spec OK |
| `kuna catalog --check` | catalog OK (no option added) |

### Byte-identical sweep (`kuna decompile-all`, before vs after)

Both arms captured on the SAME rebased tree (`f2c15037`, i.e. after #262/#263/#264, which
themselves move C++ output). Twelve binaries across four compiler specs, 73,173 lines:

| binary | spec | lines | result |
|---|---|---|---|
| `/bin/ls` | x86-64-gcc | 14,762 | IDENTICAL |
| `/usr/bin/grep` | x86-64-gcc | 22,054 | IDENTICAL |
| `/usr/bin/gzip` | x86-64-gcc | 9,157 | IDENTICAL |
| `/usr/bin/du` | x86-64-gcc | 18,823 | IDENTICAL |
| `/usr/bin/xxd` | x86-64-gcc | 1,033 | IDENTICAL |
| `cpp_demo` (g++ -O1, vtables + libstdc++) | x86-64-gcc | 436 | IDENTICAL |
| `cppproto_x86_64` (the #264 C++ DWARF fixture) | x86-64-gcc | 216 | IDENTICAL |
| `msvc_rtti_x86.exe` | x86win | 16 | IDENTICAL |
| `pe_imports.exe` | x86win | 5,217 | IDENTICAL |
| `pe_dwarf.exe` | x86win | 1,425 | IDENTICAL |
| `pdb_prog.exe` | x86win | 13 | IDENTICAL |
| `msvc_rtti_x64.exe` | x86-64-win | 21 | IDENTICAL |

stderr is byte-identical too. Byte-identical is the *expected* result here and the sweep
is the proof, not a sanity check: any diff at all would mean registration is selecting
something.

### Registry census (every language kuna ships)

Bootstrapping every `language:compiler` pair in the vendored `.ldefs` and counting
`Architecture::proto_models` after `init_post_engine`:

| | before | after |
|---|---|---|
| pairs bootstrapped | 294 / 294 | 294 / 294 |
| pairs with more than one model | **0** | **294** |
| pairs where `getModel("__thiscall")` resolves | **0** | **294** |
| models registered in total | 294 | **790** |

Zero pairs fail to bootstrap, so the tolerant skip is not masking a decode failure
anywhere in the corpus. In the spec text: 110 cspec files declare 186 `<prototype>`
elements, of which only the 110 inside `<default_proto>` were ever decoded - 76 named
models, 6 `<resolveprototype>` and 3 `<modelalias>` were dead spec text.

### Registry contents (the point of the change)

`decompiler/crates/kuna-decomp/tests/verify_cspecprotos.rs` asserts, against the real
vendored specs:

| spec | default | additionally registered |
|---|---|---|
| x86win (`x86:LE:32:default:windows`) | `__stdcall` | `__cdecl`, `__fastcall`, `__thiscall`, merged `__fastcall/__thiscall/__stdcall` (3 constituents) |
| x86-64-win (`x86:LE:64:default:windows`) | `__fastcall` | `__thiscall`, aliases `__cdecl` + `__stdcall` |
| x86gcc (`x86:LE:32:default:gcc`) | `__cdecl` | `__thiscall`, `__regparm1/2/3`, `syscall`, `processEntry`, merged `__cdecl/__regparm` |
| x86-64-gcc (`x86:LE:64:default:gcc`) | `__stdcall` | `MSABI`, `syscall`, `processEntry`, cloned `__thiscall` |

plus: `__thiscall.hasThisPointer()` is true and `__stdcall`'s is false; `__thiscall`
accepts ECX as an input parameter and `__stdcall` does not; `defaultfp` is still the same
`Rc` as `getModel("__stdcall")` with the same extrapop and effect list as before; and
`getModel("__thiscall")` resolves on `x86:LE:64:default:gcc`, `AARCH64:LE:64:v8A` and
`ARM:LE:32:v7` as well, whose specs declare none (upstream's cloned-off-the-default
invariant).

### End-to-end witness

`tests/stages/ghdec-cspecprotos.xml` - two MSVC-shaped member-function bodies with the
`this` pointer in ECX, on `x86:LE:32:default:windows`.

Pass 1 (default `__stdcall`) - the `this` pointer is lost:

```c
unsigned int getfield(void)          int4 addfield(int4 a0)
{                                    {
  int4 v1; // ecx                      int4 *v1; // ecx
  return *(unsigned int *)(v1+0xc);    return *v1 + a0;
}                                    }
```

Pass 2 (`option defaultprototype __thiscall`, previously "Unknown prototype model"):

```c
unsigned int getfield(int4 a0)       int4 addfield(int4 *a0,int4 a1)
{                                    {
  return *(unsigned int *)(a0+0xc);    return *a0 + a1;
}                                    }
```

Same effect on the real MSVC PE fixture `msvc_rtti_x86.exe`:
`int4 Box::vftable_0(void) { int4 v1; // ecx ... }` becomes
`int4 Box::vftable_0(int4 a0) { return *(int4 *)(a0 + 4) * *(int4 *)(a0 + 4); }`.

### Speed

Parsing the extra cspec elements happens once per program load, so **load** time is the
metric that matters, not per-function decompile time. `kuna functions --json` is a full
load + analysis with no decompilation; interleaved before/after on the same tree, n=31,
minimum of the run (medians in `record.json`):

| target | spec | before | after | delta |
|---|---|---|---|---|
| `msvc_rtti_x86.exe` | x86win (5 models registered) | 115.1 ms | 115.6 ms | +0.42% |
| `msvc_rtti_x64.exe` | x86-64-win (5 models) | 134.6 ms | 135.3 ms | +0.55% |
| `pe_imports.exe` | x86win | 236.8 ms | 237.2 ms | +0.15% |
| `/usr/bin/gzip` | x86-64-gcc | 317.3 ms | 317.2 ms | -0.05% |

Whole-binary `decompile-all`: gzip -1.89% (n=5), `pe_imports.exe` +0.63% (n=9),
`cpp_demo` +0.89% (n=5). Worst case +0.89%, inside the 5% budget.

The first cut cost +2.8% / +4.3% on the two smallest PE loads, because
`decode_named_protos` parsed the cspec document a *third* time. Folding the
return-address, default-proto and named-model readers onto ONE parse in
`build_default_proto` removed two of the three parses that existed even before this PR,
and that is what brought the delta to +0.5% worst case.

# `peimportcall` -- design

## The one option, and why both halves sit behind it

`peimportcall on|off`, P1 / external-refinement, default-**ON** (DIV-57).

The feature has three moving parts and they are useless apart:

1. **`externref` over the IAT slots** (`kuna-analysis/src/loader/kuna_peimportcall.rs`
   + `pe_iat::resolve_pe_import_slots`, committed by `engine.rs` as
   `Database::set_property_range(varnode_flags::externref, ...)`). Binds the call.
2. **`GlobalEntry::func_no_return`** (`p0_knowledge/database.rs::build_global_query`,
   read by `substrate/context.rs::query_function`). Gives the bound call its
   no-return flow effect, which is what fixes the boundary.
3. **Upstream's `PEFunctionsThatDoNotReturn`**, vendored verbatim and matched against
   a PE/COFF object only. Supplies the names that make (2) fire on a real PE.

(1) alone renames the call and leaves the overrun. (2) alone is inert without (1) on a
PE and, ungated, would change a MIPS/GOT-const-fold ELF the moment a deindirect
resolved to a known no-return callee. (3) alone is inert without (1) for slot calls
but would change a MinGW PE's *thunked* direct calls. One gate over all three buys the
reviewable guarantee **"option off => byte-identical, on every target"**, which the
measurements confirm.

## Why the property map, not a second symbol

Upstream maps an `ExternRefSymbol` at the slot: a pointer-to-code type of pointer
width carrying `externref|typelock`, whose `refaddr` points at the real function, and
`queryExternalRefFunction` resolves `refaddr` through `queryFunction`. Porting that
shape would mean two symbols mapped at one address (the import `FunctionSymbol` kuna
already registers there, plus the ExternRef) and would change naming and typing at the
slot. kuna's `query_function` keys on the Varnode's **own** address instead of
upstream's `refaddr` indirection, so the slot-VA registration already lines up; all
that is missing is the flag.

`Scope::queryProperties` consults the property map exactly when no covering Symbol is
found, and the import `FunctionSymbol`'s map entry is `min_funcsymbol_size = 1` byte,
so a 4/8-byte slot read never finds it as a container and always takes the property
branch. That makes `set_property_range` the minimal, surgical carrier -- and
`externref` is read in exactly one place in the whole workspace
(`coreaction_render.rs:1177`), so the blast radius is bounded by construction.

## Why the PE and Mach-O no-return lists stay separate

Upstream ships `PEFunctionsThatDoNotReturn` and `MachOFunctionsThatDoNotReturn` as two
files selected by `noReturnFunctionConstraints.xml`'s per-executable-format arms; kuna
merged them into one `PeMacFunctionsThatDoNotReturn` carrying only the shared CRT
names. Widening that merged file with `ExitProcess`/`KeBugCheck`/`?_Xbad_alloc@std@@YAXXZ`
would match Windows names against a Mach-O. The vendored PE list therefore stays its
own file on its own format arm (`BinaryFormat::Pe | Coff`), which is *more* faithful
than the merge, not less. kuna's Mach-O list is still missing upstream's Mach-O names
(`objc_exception_throw`, `cxa_throw`, `pthread_exit`, `longjmp`, the Swift family) --
a separate, out-of-scope gap, recorded here.

## Gating mechanics

The load-time-flag trap (an `Architecture` flag read inside a load-time pass silently
does nothing, because load-time passes run before `--option` is applied) is avoided
without an env-var bridge: kuna-analysis passes compute their facts at load and the
console **merges them per-pass-id at `read symbols`**, after options are in effect
(`engine.rs::analysis_pass_enabled`). So a new pass id is the gate. The decompiler-core
half (2) is an ordinary `Architecture` flag copied into the ArchSeam in
`build_arch_handle` (`ctx.peimportcall`), read at decompile time.

## Rejected alternatives

- **A new P2 pass matching the CALLIND shape** (the filed proposal): refuted -- raw
  flow IR has no def-use links, see `analysis.md`.
- **Constant-folding the IAT slot** (`readonly`): wrong -- the on-disk slot holds an
  RVA into `IMAGE_IMPORT_BY_NAME`, not a function address.
- **Registering an `ExternRefSymbol`**: two symbols at one address, and
  `query_function` requires `e.is_function`, so a straight swap would break the very
  lookup `deindirect` then performs.

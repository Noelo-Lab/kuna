# [PROPOSAL] Apply DWARF function prototypes (`undefined16 main(uint4,void*)` → `int main(int argc, char **argv)`)

Part of the **kuna → IDA Pro parity** program. Addresses gaps #6 (argument types/names) and the
DWARF-having case of #2 (return type) in one feature.

## The problem

`fmt/main`:

```c
// kuna
undefined16 main(uint4 a0,void *a1)
// IDA Pro
int __fastcall main(int argc, const char **argv, const char **envp)
```

kuna recovers neither the return type (`undefined16` — a 16-byte value, see the sibling
`ida-return-storage-inference` proposal), nor the parameter types (`uint4`/`void*` instead of
`int`/`char**`), nor the parameter names (`a0`/`a1` instead of `argc`/`argv`).

**Root cause.** The binary's `.debug_info` fully describes `main` — verified:

```
<1a17> DW_AT_name : main
<1a1f> DW_AT_type : <0x5f>                     # return type (int)
<2><1a39> DW_TAG_formal_parameter
<1a3a>   DW_AT_name : argc                      # + DW_AT_type
         DW_TAG_formal_parameter  DW_AT_name : argv  ...
```

kuna's DWARF pass (`s1_dwarf/mod.rs`) already resolves `DW_TAG_variable` (globals — PR #151)
and stack locals (`dwarf_locals`), but it does **not** consume the `DW_TAG_subprogram`
return type or its `DW_TAG_formal_parameter` children. So the recovered prototype is never
applied to the function symbol / `Funcdata` proto.

## The IDA / Ghidra reference

Both apply the DWARF (or PDB) subprogram prototype: return type, parameter types + names,
`typelock` so propagation preserves them. Ghidra's `DWARFFunctionImporter`; IDA's DWARF plugin.

## Proposed implementation

1. **DWARF pass** (`s1_dwarf/mod.rs`): for each `DW_TAG_subprogram` with a low-pc, resolve
   `DW_AT_type` → return `Datatype` (via the existing `build_datatype`), and each
   `DW_TAG_formal_parameter`'s `DW_AT_type`+`DW_AT_name`. Emit a `FuncProtoFact { addr,
   ret_type, params: Vec<(name, type)> }`.
2. **Engine commit**: apply the recovered prototype to the function's symbol / stored proto
   with `typelock|namelock` (the same lock discipline `dwarf_locals` uses for stack symbols).
   The prototype must be installed **before** decompile so the `Funcdata` proto seeds from it
   (mirror how `dwarf_locals_for` threads into the decompile drive).
3. **Calling-convention interplay**: the ABI (`__fastcall`/SysV) already maps params to storage;
   applying the DWARF *types/names* to those slots is the delta. Return type narrows the
   return storage (fixing `undefined16 main` → `int main` for DWARF binaries — the fmt case of
   #2).

## Default policy

Default-on, no flag. Re-pin baselines only if a DWARF datatest binary changes prototype render
(the corpus is largely DWARF-less bytechunk/`.o`; expect none).

## Speed / risk

- **Speed:** load-time DWARF walk (already performed for globals/locals) + a per-function proto
  install. Negligible decompile impact.
- **Risk:** MEDIUM. Prototype application interacts with the ABI/param-storage recovery and
  type propagation; a wrong `typelock` could fight the recovered type. Mitigate by matching the
  existing `dwarf_locals` lock convention and gating on a successfully-sized `build_datatype`
  (fall through to today's behavior otherwise). Struct-by-value / varargs params are edge cases
  to handle or skip conservatively.

## Testing

`verify_*.rs`: a DWARF fixture (extend #151's `dwarf_globals_x86_64.c`, or the vendored
`regglobal_fmt_x86_64`) asserting `main` renders `int main(int argc, char **argv)` (return type
+ arg types + names). Gates: all three.

## Effort

**Medium.** Builds directly on the existing DWARF pass + the #151/`dwarf_locals` install
machinery. ~1–2 PRs (return type + params can split). High readability payoff (every
DWARF-built function gets its real signature).

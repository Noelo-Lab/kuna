# [PROPOSAL] DEX loader and the Dalvik decompiler pieces

## Problem

kuna vendors Ghidra's Dalvik SLEIGH specs (`specs/Ghidra/Processors/Dalvik`), but
it cannot open a `.dex` file. A user can only feed raw bytecode with
`--raw-image --target Dalvik:LE:32:DEX_Android12 --base 0`, and the result shows
what is missing below the loader too:

```c
int sub_31c(void)              // static int add(int a, int b)
{
  int v1; // v0
  int v2; // v1
  return v1 + v2;
}
unsigned int sub_438(void)     // static String greet(String who)
{
  v1 = NEW(CPOOLREF(0,6,5));
  (*(void (*)(int,unsigned int))CPOOLREF(v1,0xf,0))(v1,CPOOLREF(0,1,4));
  ...
}
```

* No method names, classes or prototypes: the image has no symbols.
* Parameters are unread locals: the Dalvik cspec's `uponentry` payload is
  `dynamic` (Ghidra builds it in Java from the method's code item), and kuna has
  no source for it.
* `CPOOLREF` is never resolved: the engine has no constant pool, and
  `RuleTransformCpool`, `TypeOpCpoolref::getOutputLocal` and
  `PrintC::opCpoolRefOp` are stubs or missing.
* `invoke-*/range` uses the `moveRangeToIV` callother-fixup, also `dynamic`.
* `sparse-switch` lowers to the `switchAssist` jump-assist user-op; kuna has no
  `JumpAssisted` jump-table model, so the switch has no cases.
* There is no Java back end (`printjava.rs` is a stub).

## Design

Everything follows the upstream split: the loader and the constant-pool/inject
contents come from Ghidra's Java side (`DexLoader`, `ConstantPoolDex`,
`InjectPayloadDexParameters`, `InjectPayloadDexRange`, Apache-2.0) and the
DEX format document (source.android.com/docs/core/runtime/dex-format); the
engine pieces are ports of `ruleaction.cc`, `typeop.cc`, `jumptable.cc`,
`printc.cc` and `printjava.cc`.

1. **Parser** — `kuna-analysis/src/loader/dex.rs`: header, string/type/proto/
   field/method ids, class defs, `class_data_item` and `code_item`. Plain data,
   bounds-checked, no allocation per string until asked.
2. **Load** — `kuna-console` recognises `dex\n0NN\0`, maps the whole file
   read-only at `ram:0` (one segment, as Ghidra's `.dex` block), selects
   `Dalvik:LE:32:DEX_Android12` (all DEX files `d8` emits decode with it;
   `--target` overrides), and installs one function symbol per method with
   code at `code_off + 16`, named `pkg.Class.method` (`_<addr>` added to
   overloads, as Ghidra's uniquifier does).
3. **DEX model on the architecture** — `kuna-decomp/src/p0_knowledge/kuna_dex.rs`
   holds what the engine asks: per-entry register layout, method/field/type/
   string tables. It answers three upstream questions:
   * the constant pool (`ConstantPoolDex::getRecord`) — records are built on
     first use and cached, so a 65k-method file costs nothing until a function
     references a method;
   * the `uponentry` payload (`InjectPayloadDexParameters`): `COPY iv_i -> v_j`
     for `this` and each parameter, wide types taking 8 bytes;
   * the `moveRangeToIV` payload (`InjectPayloadDexRange`).
   Both payloads are reached through the existing "no compiled template" seam
   (`emit_inject`, `ActionConstbase`).
4. **Engine ports** — `RuleTransformCpool` (body), `TypeOpCpoolref`
   output type, `JumpAssisted` (with a small evaluator for the `<case_pcode>`,
   `<addr_pcode>`, `<default_pcode>` snippets), `opCpoolRefOp`, `opNewOp`.
5. **Java output** — `OutLang::Java` (`option setlanguage java-language`):
   the C profile with Java's differences (`.` member access, `>>>`, `null`,
   `instanceof`, array/class type spelling, hidden `this`), following
   `printjava.cc`. A DEX image selects it by default; `setlanguage c-language`
   still gives C.

## Options

* `dexmodel` (P0, default on): use the DEX model (constant pool, entry and
  range payloads, method prototypes). Off gives today's raw-image output.
* `jumpassist` (P2, default on): recover switches from a `<jumpassist>` user-op.
  Only the Dalvik and JVM specs declare one; the datatest corpus has none.

## Tests and evidence

* A stage test over a small DEX built from Java source with `d8` (two passes:
  `dexmodel off` = the raw output above, default = named, typed Java).
* Unit tests for the parser on the same file.
* Speed: no cost to non-DEX images (the model is absent); DEX load time
  measured on a large real `classes.dex`.

## Out of scope

CDEX/ODEX/OAT/VDEX, APK unzipping (`unzip` the `classes*.dex` first), multi-dex
cross-file references, annotations, debug-info local names, try/catch handler
recovery, class/field struct layouts.

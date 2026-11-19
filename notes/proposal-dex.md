# [PROPOSAL] DEX, APK and Java: loader, constant pool, try/catch, `new T()`, on-device containers

## Problem

kuna vendors Ghidra's Dalvik SLEIGH specs (`specs/Ghidra/Processors/Dalvik`), but
cannot open a `.dex` file. A user can only feed raw bytecode with
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
* There is no Java back end (`printjava.rs` was a stub).
* Nothing in the instruction stream reaches a catch handler, so every catch
  body is unreachable code.

## Design

The loader and the constant-pool/inject contents follow Ghidra's Java side
(`DexLoader`, `ConstantPoolDex`, `InjectPayloadDexParameters`,
`InjectPayloadDexRange`, Apache-2.0) and the DEX format document
(source.android.com/docs/core/runtime/dex-format); the engine pieces are ports of
`ruleaction.cc`, `typeop.cc`, `jumptable.cc`, `printc.cc` and `printjava.cc`.
The try/catch recovery and the `new T(args)` form are kuna's own.

1. **Parser** — `kuna-decomp/src/p0_knowledge/kuna_dex/format.rs`: header,
   string/type/proto/field/method ids, class defs, `class_data_item`,
   `code_item` and its try table; compact DEX (`cdex`) through the same reader.
   Plain data, bounds-checked.
2. **Load** — `kuna-analysis/src/loadimage_dex.rs` + `kuna-console`: a DEX file
   is mapped read-only at `ram:0`; an APK's `classes*.dex` are inflated and
   mapped one after another at 64 KiB boundaries; ODEX, VDEX 027 and OAT
   (Android ≤ 7) files are unwrapped to the DEX files they hold. The language is
   `Dalvik:LE:32:DEX_Android12` (`ODEX_KitKat` for an ODEX; `--target`
   overrides), and one function symbol per method with code is installed, named
   `pkg::Class::method` (`_<addr>` added to overloads).
3. **DEX model** — `kuna_dex.rs` holds, per file, the method/field/type/string
   tables and the per-entry register layout, and answers the constant pool
   (records built on first use), the `uponentry` payload and the
   `moveRangeToIV` payload. `kuna_dex/links.rs` resolves method references
   across an APK's files, through the superclass chain, for `kuna xrefs`.
4. **Engine ports** — `RuleTransformCpool`, `TypeOpCpoolref` output type,
   `JumpAssisted` (with an evaluator for the `<case_pcode>`, `<addr_pcode>`,
   `<default_pcode>`, `<size_pcode>` scripts), `opCpoolRefOp`, `opNewOp`.
5. **Java output** — `OutLang::Java` (`option setlanguage java-language`,
   `--language java`): the C profile with Java's differences (`.` member
   access, `>>>`, `null`, `instanceof`, array/class type spelling, hidden
   `this`), after `printjava.cc`. A DEX image selects it by default.
6. **try/catch** — `p2_lift/kuna_javatry.rs`: each handler is dispatched once
   from the start of the first range that names it (`java_try`), each throwing
   instruction in a range gets an exceptional edge (`java_throws`, data flow
   only), `move-exception` is defined by `java_caught`, `throw` is a halt.
   `ActionJavaThrowPrune` drops the edges a handler learns nothing from;
   structuring keeps a dispatch out of conditions and loops;
   `printc/kuna_javatryprint.rs` prints the dispatch as `try { } catch (T e) { }`.
7. **`new T(args)`** — `p9_emit/kuna_javanew.rs` folds a NEW into the `<init>`
   call that consumes it; constructors are declared by class name and print
   `this(args)` / `super(args)`.

## Options

* `dexmodel` (P1, default on): use the DEX model. Off gives the raw lifting.
* `jumpassist` (P2, default on): recover switches from a `<jumpassist>` user-op.
  Only the Dalvik and JVM specs declare one; the datatest corpus has none.
* `javatry` (P2, default on, DEX input only): recover try/catch.
* `javanew` (P9, default on, java-language output only): `new T(args)`,
  `this()`/`super()`.

Loading APKs and the on-device containers has no option: a file either is one
of them or it is not.

## Slices

The PR is one branch with one commit per slice, each with its own stage test:

1. DEX loader, constant pool, entry and range payloads, `JumpAssisted`, Java
   output (`tests/stages/kuna-dex.xml`).
2. `new T(args)` (`kuna-dex-javanew.xml`).
3. try/catch, `throw`, constructors (`kuna-dex-javatry.xml`,
   `kuna-dex-javactor.xml`).
4. APK, compact DEX, ODEX, VDEX, OAT; cross-DEX method references and
   `kuna xrefs` (`kuna-dex-apk.xml`, `kuna-dex-containers.xml`).

## Evidence

* Stage tests over small DEX files built from Java source with `d8`
  (`tests/stages/fixtures/dex/*.java`), two passes per option.
* Unit tests for the parser and the containers on the same files.
* Speed: no cost to non-DEX images (the model is absent, the `<jumpassist>`
  table is empty, and the new builtins are never emitted).

## Out of scope

Annotations, debug-info local names, class/field struct layouts, OAT files
from Android 8 and later (their DEX files live in the VDEX next to them),
`finally` regions beyond the compiler's duplicated copies.

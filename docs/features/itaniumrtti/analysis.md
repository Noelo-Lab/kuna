# itaniumrtti — analysis

## 1. The gap, and why it is worth a PR

**Ghidra has no Itanium RTTI analyzer at all.** Its `RttiAnalyzer` lives in
`Ghidra/Features/MicrosoftCodeAnalyzer` and reads only the MSVC
`CompleteObjectLocator` graph; its GCC/Clang class recovery is *script*-tier
(`RecoverClassesFromRTTIScript.java` / `RTTIGccClassRecoverer.java`) and never runs
in auto-analysis. Measured on a stripped `g++ -O0` probe, Ghidra 12.1 resolves the
`Shape` vtable to `DAT_00107a80` and every virtual method to `FUN_<addr>`.

IDA Pro **does** reconstruct it — `_ZTV6Widget`, `_ZTI6Widget`, `_ZTS8Drawable` come
back from `.rodata` + `.data.rel.ro` alone. kuna had **zero** `_ZTV` / `_ZTI` /
`__class_type_info` / `__cxxabiv1` hits anywhere in `decompiler/crates` before this
PR: the Itanium side was entirely absent while the MSVC side had been ported.

So this is a place kuna can beat Ghidra outright rather than catch up.

## 2. The scoping decision, and the measurement that forced it

Two capabilities were separable and only one belonged in the first PR.

**(a) Itanium typeinfo/class recovery** — general, exact, and the thing Ghidra
lacks. **(b) Independent vtable detection** from `_ZTV` symbols and/or
code-pointer-run scanning — the half that could in principle touch the benchmark,
since `libleveldb.so` exports 18 `_ZTV` symbols that survive `strip`.

(b) was measured before it was scoped, and it is a dead end on that corpus:

| leveldb `-O0` stripped, the vtable half | measured |
|---|---|
| `_ZTV` symbols defined in `.dynsym` | 17 (+1 undefined import) |
| total slots across them | 126 |
| slots relocated to `__cxa_pure_virtual` | **59** |
| header words with no relocation (offset-to-top / typeinfo) | 31 |
| slots naming a real method | 36, **every one already a `.dynsym` FUNC symbol** |
| recoverable-but-unnamed virtual methods | **0** |

The 17 exported vtables are the *abstract interface* classes (`Comparator`,
`Iterator`, `Env`, `DB`, `Cache`, …); their slots are pure-virtual stubs, and the
concrete implementations' vtables carry no symbol. Worse, the file bytes are all
zero — a PIE's vtable slots are written by relocations, so a naive read of
`_ZTVN7leveldb10ComparatorE` returns eight NUL words. And an *independent*
code-pointer-run scan of `.data.rel.ro` reports 37 runs / 231 "functions", none of
which correspond to a `.dynsym` symbol: on a PIE that scan is measuring relocation
residue, which is precisely the "mistyping a non-vtable pointer array" failure mode.

So (a) shipped first. It is exact rather than heuristic, it is the Ghidra-beating
capability, and — decisively — **it delivers vtable recovery anyway**, reached from
the typeinfo back-pointer instead of guessed. The heuristic scanner stays out.

## 3. The mechanism — an anchor, not a pattern scan

The MSVC sibling pass has to guess: it byte-searches `.rdata` for `.?A` strings and
treats `ref − 12` as a candidate `CompleteObjectLocator`. The Itanium graph does not
require guessing, because the three `__cxxabiv1` typeinfo vtables live in
**libstdc++**:

```
$ readelf -rW probe_O0.stripped | grep cxxabiv
0000000000007bf8  R_X86_64_64  _ZTVN10__cxxabiv121__vmi_class_type_infoE + 10
0000000000007c30  R_X86_64_64  _ZTVN10__cxxabiv117__class_type_infoE     + 10
0000000000007c40  R_X86_64_64  _ZTVN10__cxxabiv117__class_type_infoE     + 10
0000000000007c50  R_X86_64_64  _ZTVN10__cxxabiv120__si_class_type_infoE  + 10
0000000000007c68  R_X86_64_64  _ZTVN10__cxxabiv120__si_class_type_infoE  + 10
0000000000007c80  R_X86_64_64  _ZTVN10__cxxabiv117__class_type_infoE     + 10
```

Six relocations, six typeinfo objects, zero false positives — and the *symbol* names
the flavour, which fixes the object's layout past `[vptr][name ptr]`. The addend is
`0x10` = `2 x ptr`, because a typeinfo's vptr points two words into the ABI vtable.
`.rela.dyn` is a loader input, so `strip --strip-all` cannot remove it.

This matters for a reason that is easy to get wrong: **in a PIE or shared object the
typeinfo's vptr word is 0 in the file** (the loader writes it). A "find the common
vptr the typeinfo objects agree on" approach — the exact shape of the MSVC pass's
`MIN_TYPE_INFO_AGREEMENT` heuristic — therefore recovers **nothing** on a modern
dynamically linked binary. The relocation is not a convenience; it is the only
anchor that works.

From there the graph walks itself: `_ZTS…` name → demangled class (by wrapping the
bare mangled component back into the `_ZTS` symbol form, the exact analog of the
MSVC `??_R0…@8` wrap, adding no demangler code); `__si_`/`__vmi_` base lists →
inheritance graph **with byte displacements**; one scan for pointer slots holding a
typeinfo address → every sub-vtable in the image.

## 4. The two ABI constraints that make the vtable scan sound

A pointer slot holding a typeinfo address is a *candidate* sub-vtable header+8, but
base-class pointers inside other typeinfo objects hold typeinfo addresses too. Two
exact constraints separate them, and on the probe they reject 4/4 false positives:

1. **`offset-to-top <= 0`** — 0 for a primary sub-vtable, negative for a secondary,
   never positive. The four false candidates read +2, +8589934592, +20856, +20864.
2. **at least one slot pointing into an executable section** — all four false
   candidates walk zero slots.

A third detail is what makes the extent right on real binaries: a slot whose file
word is 0 *but which carries a dynamic relocation* is an imported virtual method
(`__cxa_pure_virtual`, a base method from another image). Terminating there would
truncate every abstract interface's vtable; the walk steps over it instead, keeping
the ABI slot indices faithful while emitting no name.

## 5. Results — measured against `nm` ground truth

On the stripped probe (`g++ -O0`, `strip`), **23 of 23** virtual methods across six
classes are recovered, and every one matches the unstripped `nm -C` output:

| recovered | ground truth |
|---|---|
| `Shape::vtable_0/1` | `Shape::~Shape()` x2 (complete + deleting) |
| `Shape::vtable_2` | `Shape::area(int) const` |
| `Shape::vtable_3` | `Shape::perimeter() const` |
| `Circle::vtable_2` | `Circle::area(int) const` |
| `Square::vtable_2` | `Square::area(int) const` |
| `Widget::vtable_2/3` | `Widget::emit(int)` / `Widget::render(int)` |
| `Widget::vtable_for_Drawable_0/1/2` | **`non-virtual thunk to Widget::~Widget()` x2, `non-virtual thunk to Widget::render(int)`** |

Two of those rows are, as far as the five-decompiler pane capture shows, things no
other tool produces:

- **The secondary sub-vtable is named for the base subobject it serves.** `Widget`
  inherits `Loggable` at +0 and `Drawable` at +16; the sub-vtable with
  `offset-to-top = -16` is matched against the recovered `__vmi_class_type_info`
  base displacements and labelled `vtable_for_Drawable`. That displacement is
  exactly the datum the MSVC path throws away with its `pmd` fields.
- **An inherited slot is attributed to the class that defines it.** `Shape::perimeter`
  (`0x3316`) appears at slot 3 of `Shape`'s, `Circle`'s and `Square`'s vtables.
  Scan-order attribution would call it `Square::vtable_3`; the recovered base graph
  picks the claimant that is an ancestor of all the others, so it is named once, for
  `Shape`.

The most legible single line is a multiple-inheritance constructor:

```c
/* off */                            /* on */
sub_34d4(a0,a1);                     sub_34d4(a0,a1);
sub_356a(&a0[2],a2);                 sub_356a(&a0[2],a2);
*a0 = 0x7a90;                        *a0 = Widget_vptr;
a0[2] = 0x7ac0;                      a0[2] = Widget_vptr_for_Drawable;
```

## 6. What it does NOT do — stated plainly

**It does not move leveldb, and it cannot.** `libleveldb.so` is built `-fno-rtti`:
18 `_ZTV` symbols, **zero** `_ZTI`, **zero** `_ZTS`, **zero** `__cxxabiv1`
relocations. No anchor exists, so the pass emits nothing. The whole-binary sweep
confirms it exactly — leveldb `-O0` stripped, 3,962 functions, **0 changed**;
leveldb `-O2` stripped, 919 functions, **0 changed**. There is no benchmark delta to
report and none is invented. The PR is justified on capability plus the two
non-leveldb witnesses.

**It does not devirtualize.** The vtable slot arrays are marked read-only, which is
the precondition, but a virtual call through a base pointer of unknown dynamic type
still renders `(**(code **)(*a0 + 0x10))(a0,a1)`. Measured on the probe: no rival
devirtualizes it either. Real devirtualization needs the vptr store to reach the
call through the type system; that is a separate, larger feature.

**It does not recover `this->field`.** No decompiler does on a stripped binary.

**It does not scan for code-pointer runs.** Section 2 explains why that would be the
wrongness axis rather than a feature.

## 7. What an adversarial probe found that the witness could not

The probe and the shipped fixture both use plain, well-named classes. A second,
deliberately hostile binary — two instantiations of one template, a three-deep
namespace, and an anonymous-namespace class — found **two defects the good-citizen
witness could never surface**, and fixing them took recovery on that binary from
**7 of 14 virtual methods to 14 of 14**:

1. **Template instantiations collapsed into one class.** `Vec<int>` and `Vec<double>`
   are two classes with two vtables, but the module-wide name-only demangling
   (`strip_bracket_groups`, a hard requirement of the `::`-scope splitter) reduces
   both to `Vec`. Because the symbol commit is an idempotent ADD, the second
   instantiation then loses the name race and every one of its methods stays
   `sub_<addr>` — silently, with no error. The fix is to *fold* template arguments
   into the identifier rather than delete them (`Vec_int`, `Vec_double`), with a
   depth-aware `::` split so a separator inside an argument list is not mistaken for
   a scope boundary.

2. **Every TU-local class was invisible.** The Itanium ABI (§2.9.1) prefixes the
   type-name string of a type whose identity is local to one translation unit with
   `*`, telling the runtime to compare `type_info`s by pointer rather than by string.
   That byte is not part of the mangled name, so `_ZTS*N12_GLOBAL__N_16HiddenE` does
   not demangle and the whole class was dropped. This is not a corner case: an
   anonymous-namespace class is how most C++ codebases spell a concrete
   implementation of an exported interface — leveldb's `PosixEnv` is one — so the
   classes worth recovering most were precisely the ones being lost.

Both cases are now in the shipped fixture and asserted by
`template_instantiations_and_tu_local_classes_are_recovered`.

## 8. The wrongness axis, measured

The axis is **attaching a wrong class name / naming a non-method**. Three structural
properties bound it, and all three were checked by sweep rather than argued:

1. A candidate is rejected unless its name string **demangles** as `_ZTS<name>` — a
   coincidental address almost never does.
2. Every base pointer must resolve to an already-accepted typeinfo.
3. The function-symbol commit is an idempotent `find_function`-gated ADD, so a real
   `.dynsym`/`.symtab` name is never overwritten (asserted by
   `the_function_set_and_real_names_are_untouched`).

**Plain C is completely untouched.** On the real default path (`--mode aggressive`,
the baseline being the pre-PR default path `--mode aggressive --option itaniumrtti
off`): **14 binaries / 357 functions across x86-64, aarch64, arm, riscv64, PIE and
non-PIE — 0 renamed, 0 body-changed, 0 new errors.** At the option level, above the
500 KiB `auto` threshold where the preset does not apply, the same holds on the two
largest available binaries: `mcount_x86_64` (static C, 1,073 functions) and leveldb
`-O0` stripped (3,962) / `-O2` (919), all 0 changed. This is structural — a C binary
emits no typeinfo, so there is no relocation to anchor on.

On C++ — 8 binaries / 364 functions on the default path, plus leveldb `-O0` (3,962)
and `-O2` (919) at the option level — the only non-probe fixture that changes at all
is `cppproto_x86_64`, and its single hunk is strictly better:

```c
/* off */ *(unsigned long *)this = 0x402028;
/* on  */ *(code **)this = Shape_vptr;
```

**0 new decompile errors anywhere**, and the function ENTRY set is byte-identical on
every binary in the sweep — the pass names things, it never discovers them.

## 9. A rendering decision worth recording

Data labels join the class to the kind with `_` (`Shape_vptr`), not `::`. kuna's
scope splitter splits on every `::` and the C printer emits a global by its **leaf**
name, so `Shape::vptr` and `Circle::vptr` both render as a bare, ambiguous `vptr` —
verified on the probe before the naming was chosen. Function symbols keep the `::`
form, because a callee's qualified name *is* rendered at the call site
(`cppcallnames`, DIV-59): `shapes::Circle::vtable_0(v3)`.

## 10. Reachability — the feature was shipped inert, and that is a defect

A default-off option is only half a decision. `auto` selects `aggressive` for any
input under 500 KiB (DIV-40), which is the default path for `decompile-all`, the
`kuna` CLI, the web front-end and the benchmark — so an option outside
`AGGRESSIVE_OVERRIDES` **never runs for anyone**. Measured on the shipped fixture,
before the preset change:

| `decompile-all` invocation | vtable-method names recovered |
|---|---|
| default (`auto` → `aggressive`) | **0** |
| `--mode aggressive` | **0** |
| `--mode reliable` | 0 |
| explicit `--option itaniumrtti on` | 29 |

The whole recovery existed and nothing reached it. After adding the option to the
preset:

| `decompile-all` invocation | vtable-method names recovered |
|---|---|
| default (`auto` → `aggressive`) | **29** |
| `--mode aggressive` | **29** |
| `--mode reliable` | 0 *(shipped defaults — unchanged, by design)* |
| explicit `--option itaniumrtti on` | 29 |

Membership is not a judgement call here: the preset's own documented contract is
already *"every off-by-default option flipped ON except `v850indirectbranch` and
`dwarf_lines`"*, and that clause explicitly names the format-gated no-ops
(`rtti`/`pdb` = PE, `objc`/`macho-arm64e` = Mach-O, `sparcstructret` = SPARC) as the
safe class. `itaniumrtti` is that same shape with a **strictly narrower** firing
condition than `rtti`'s: inert off ELF, and additionally inert on any ELF whose
`.rela.dyn` carries no `__cxxabiv1` typeinfo relocation.

### The invariant was sampled, not enforced

`aggressive_excludes_v850_and_dwarf_lines_but_includes_the_rest` checks five
*representative* members, so it passes whether or not a newly added option was ever
put in the list — which is exactly how this shipped inert and how `formatstring`
shipped inert for a different reason. `aggressive_carries_every_default_off_option`
now enforces the real contract, and immediately found **four further default-off
options in the same trap**: `cortexmvectors`, `ptrentry`, `tailcallentry`,
`paramcopyhoist`.

Those four are recorded in an explicit `UNEVALUATED` list rather than silently
flipped on. Preset membership changes emitted C on the binaries an option fires for,
so each needs its own sweep and speed measurement — `paramcopyhoist`'s own catalog
row already records that its flip regresses a stage assertion and was deferred.
Shrinking that list is the follow-up; the test now makes each one a deliberate
decision instead of an oversight.

### Why no `_LOADTIME_GATE_ENV` entry is needed

#268 added a gate table to `scripts/decompile.py` because `typedepth`'s DWARF types
are *consumed* inside `load file`, upstream of every console `option` line, so both
arms of a before/after demo rendered identically. `itaniumrtti` computes its facts at
load too, but they are **committed** at `read symbols`
(`commit_pending_analysis` → `engine.rs::analysis_pass_enabled`), and the script
emits every `option` line *before* `read symbols`. So the console line reaches the
gate — verified, not assumed: the generated demo renders `sub_14b8` off and
`shapes::Shape::vtable_2` on. The rule is recorded next to the table: **consumed at
load ⇒ needs an entry; committed at `read symbols` ⇒ does not.**

## 11. Speed as a preset member

Now that the option is a preset member, the number that matters is what it costs on
the binaries where it **does not fire** — that is the cost every user pays. Baseline
is the pre-PR default path (`--mode aggressive --option itaniumrtti off`), treatment
is the post-PR default path. Interleaved, **minimum of 15** pairs (31 for the fire
row), on a box shared with other agents.

| case | off | on | delta |
|---|---|---|---|
| **does not fire** — `functions` mcount_x86_64 (static C, 1,969 syms) | 171.2 ms | 171.6 ms | **+0.24%** |
| **does not fire** — `functions` leveldb-O0 stripped (1,546 relocs) | 189.0 ms | 189.5 ms | **+0.26%** |
| **does not fire** — `functions` regglobal_fmt_x86_64 (224 KiB C) | 180.3 ms | 180.8 ms | **+0.28%** |
| **does not fire** — `decompile-all` agg regglobal_fmt_x86_64 (C) | 2476.0 ms | 2482.1 ms | **+0.25%** |
| **does not fire** — `decompile-all` agg fmt_x86_64 (C) | 110.4 ms | 109.9 ms | **-0.38%** |
| **does not fire** — `decompile-all` agg fauxware (C) | 123.4 ms | 123.5 ms | **+0.06%** |
| **does not fire** — `decompile-all` agg cppsig_x86_64.so (C++, no RTTI) | 115.1 ms | 114.7 ms | **-0.32%** |
| **fires** — `decompile-all` agg itaniumrtti_x86_64.so (n=31) | 151.1 ms | 152.6 ms | **+0.99%** |

**Worst no-fire cost +0.28%; worst overall +0.99%; budget 5%.** The two rows the
coordinator asked for specifically — the `.rela.dyn`/symbol scan on a large binary
with no typeinfo at all — are `mcount_x86_64` (896 KiB static C, 1,969 symbols) at
+0.24% and stripped leveldb (1,546 dynamic relocations) at +0.26%. The no-fire path
is one relocation-table walk plus one symbol walk, then an early return; discovery
now runs *before* any other structure is built precisely so that path stays cheap.

The fire row is **+0.99% here versus +2.13% measured pre-preset in isolation** — same
work, larger denominator, because under `aggressive` the baseline already carries
`listing`, `aif`, `funcstart_patterns` and the rest. The preset-member figure is the
honest one for a default-path user.

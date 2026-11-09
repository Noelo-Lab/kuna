# vftrace: pass-by-pass differential tracer

kuna-jvm ports Vineflower 1.12.0 and aims at byte-identical Java output (in compat mode: kuna's own
fixes of Vineflower's failures are off, as the kuna side of this tool runs unless given
`--opt jvm_vineflower_compat=0`). When a class comes out different, this tool tells you **which pass
first went wrong, in which method**. Both engines call
`decompileRecord.add("<StepName>", root)` after every pass of the per-method pipeline
(`MethodProcessor.codeToJava`, plus `ClassWriter`'s late `FinalSimplifySwitches`/`FinalPrettifyIfs`);
at each call the tracer writes a canonical dump of the method's statement tree. Two traces are
compared method by method; the first step whose dump differs names the pass to look at.

| Piece | Where |
|---|---|
| Java tracer | `src/org/.../main/rels/DecompileRecord.java` and `src/org/.../modules/decompiler/vars/VarProcessor.java` (shadows of the released classes: `add` / `rerunClashing` also call the tracer), `src/kuna/vftrace/{Dumper,Trace,TraceDriver,HashMode}.java`, `gen_shadow.py`, `build.sh`, `run.sh` |
| Rust tracer | `src/infra/kuna_trace.rs`, called from `DecompileRecord::add` (`src/infra/decompile_record.rs`) and `VarProcessor::rerun_clashing`; `examples/jvmtrace.rs` |
| Comparer | `compare.py` (two traces); `sweep_compare.py` (two `corpus_trace.py` directories); `trace-diff.sh` runs both engines and the comparer |
| Corpus runner | `corpus_trace.py --engine vineflower\|kuna` over the Java oracle corpus (`$KUNA_JVM_ORACLE/corpus/manifest.json`, `../oracle/README.md`) or a class directory; `sweep.sh` = both engines + `sweep_compare.py` |
| Tests | `tests/trace_vectors.rs` + `tests/data/trace/` (both dumpers on the same hand-built trees and variable table; `gen.sh` regenerates), `tests/trace_file.rs` (trace file byte-identical to the Java one), `tests/trace_sweep.rs` (final-text regressions for bugs the sweep found) |

## The sweep (one command)

```sh
tools/vftrace/sweep.sh [--corpus DIR] [--jobs N] [--out DIR] [--kuna BIN] [--strict-text]
tools/vftrace/sweep.sh --classes DIR [--set NAME] [--opt k=v]... [--lib JAR]... [--sample N] ...
```

Traces every runnable case of the oracle corpus (or, with `--classes`, every top-level class of a
directory together with its nested classes, run with the `--opt`/`--lib` given and no fixture) with
both engines into `OUT/vineflower/` and `OUT/kuna/`, compares every method of every case, and prints
the summary and the first-divergence histogram (details: `OUT/compare.txt`, `OUT/compare.json`).
Exit status 1 when any method diverges or was traced by one engine only; final-text differences
(vs the oracle for corpus cases, between the engines for `--classes`) are listed and fail only with
`--strict-text` (text parity is the corpus runner's gate). A difference confined to the stack-trace
lines of a `// $VF: Couldn't be decompiled` comment is reported as `TEXT-EXC` and never fails: kuna
has no Java stack trace to print. The Java tracer is rebuilt when its sources are newer than
`$BUILD/built.stamp`; the kuna binary is built (`--release`) unless `--kuna` is given.
Vineflower runs on **JDK 21** here (`VF_JAVA`): it escapes string characters by its JVM's
`Character` tables, and kuna's `rt::jchar` is generated from JDK 21's (Unicode 15). On JDK 17
(the corpus oracle JVM, Unicode 13) characters added in Unicode 14/15 print raw in kuna and
`\uXXXX` in Vineflower (seen in Guava's `CharMatcher`, `PublicSuffixPatterns`); traces are not affected.
`--opt` on a corpus sweep overlays every case's options (the corpus under another option map).

Stage-3 sweep results (after the fixes; "OK" = every step identical, variable tables included):

| Input | Cases | Methods | Steps | Result |
|---|---|---|---|---|
| corpus, fixture options | 598 | 3,479 | 72,000 | all OK; text: the 4 known corpus failures |
| corpus x 13 option overlays (`decompile-enums=0`, `lambda-to-anonymous-class=1`, `decompile-generics=0`, `decompile-finally=0`, `use-method-parameters=0`, `rename-members=1`, ...) | 598 each | 3,464-3,479 | 69-72 k each | all OK |
| JDK 21 `java.base` (`include-runtime` = JDK 21) | 3,481 | 62,791 | 1.28 M | all OK (4 methods need Vineflower `VF_XMX` of 6-24 GB) |
| Guava 33.8 (`--lib` its jar) | 594 | 15,557 | 284 k | all OK, text identical |
| spring-boot / spring-framework (500 sampled) / elasticsearch (500 sampled) | 493 / 500 / 500 | 5,192 / 5,115 / 9,083 | 101 / 102 / 178 k | all OK |

The traces are hash-stable: the corpus traced by Vineflower in `stock` and `random:1` mode equals the
`index` trace. Commands (scratch paths are examples):

```sh
J=$KUNA_JVM_ORACLE/jdks/jdk21
G=$JAVABENCH_WORKSPACE/dataset/guava/classes.jar      # the javabench dataset (docs/jvm-benchmark.md)
$J/bin/jimage extract --include 'regex:/java.base/.*\.class' --dir jdk $J/lib/modules
VF_XMX=1g tools/vftrace/sweep.sh --classes jdk/java.base --set jdk21 --opt include-runtime=$J --jobs 8 --out OUT1
unzip -q $G -d guava
tools/vftrace/sweep.sh --classes guava --set guava --opt include-runtime=$J --lib $G --out OUT2
tools/vftrace/sweep.sh --opt decompile-enums=0 --opt remove-synthetic=0 --out OUT3    # an overlay
```

A Vineflower `OutOfMemoryError` (its trace "ends early in A") is a heap-size artifact: re-run the case
with a larger `VF_XMX` (`--only`). Ghidra (500 sampled groups, 6,587 methods) agrees everywhere except
the ANTLR-generated `SleighParser`, whose `tokendef()` exceeds both engines' budgets: Vineflower
OOMs at 2 GB and is still in its finally loop after 28 min at 12 GB (9.3 GB RSS); kuna passes the
600 s case timeout (untraced: 17.6 GB RSS after 14 min). The 9 steps both reach agree; the class's
other methods come after it and are never traced by kuna.

## Running

```sh
tools/vftrace/build.sh        # once; outputs go to $KUNA_JVM_ORACLE/vftrace-build ($BUILD)

# Vineflower: same arguments as the corpus oracle's VfDriver (--fixture, --opt k=v, inputs, -o)
tools/vftrace/run.sh --hash index --trace vf.vft --fixture --opt ternary-in-if=1 Foo.class 'Foo$1.class'

# kuna: same command line (cargo run -p kuna-jvm --example jvmtrace -- ...)
target/debug/examples/jvmtrace --trace kuna.vft --fixture --opt ternary-in-if=1 Foo.class 'Foo$1.class'
KUNA_JVM_TRACE=kuna.vft <anything that decompiles>      # env switch, read once per thread

tools/vftrace/compare.py vf.vft kuna.vft                 # per method: OK or first divergence + diff
tools/vftrace/trace-diff.sh Foo.class --fixture --opt ternary-in-if=1   # all of the above

# a whole corpus case set, one engine per run (traces stay in --out), then compare the two directories
python3 -I tools/vftrace/corpus_trace.py --out VF --hash index
python3 -I tools/vftrace/corpus_trace.py --out KUNA --engine kuna --kuna target/release/examples/jvmtrace
python3 -I tools/vftrace/sweep_compare.py VF KUNA --json cmp.json     # or simply: tools/vftrace/sweep.sh
```

`jvmtrace` runs on one 512 MiB-stack engine thread (as the corpus runner does) and, like VfDriver's
default `--save legacy`, writes each class to `-o` by file name (`--save folder` keeps the package path).

`run.sh --hash` picks Vineflower's identity hashes: `stock` (released classes), `index` (default of
`trace-diff.sh`: shadow classes hash by creation index, the way kuna's arenas do) or `random:SEED`.
`--filter S` traces only method keys containing `S`. The Java driver prepends
`--opt thread-count=1` so every class runs on one worker thread (a later `--opt` overrides it).
`jvmtrace --initial-only` runs `codeToJava` only up to `DomHelper.parseGraph` and records `Initial`;
compare such a trace with `compare.py --prefix` (a shorter trace whose steps all agree is `OK-PREFIX`).
`compare.py --quiet` prints only the verdicts and the summary, `--json` writes machine-readable
results, `--ignore REGEX` blanks a pattern before comparing (e.g. `--ignore ' id=[0-9]+'`).

Rust API: `kuna_trace::start(path, filter)` / `finish() -> Option<TraceSummary>` on the current
thread, `kuna_trace::dump(root) -> String`, `kuna_trace::enabled()` (one thread-local flag read;
tracing is off unless started or `KUNA_JVM_TRACE` is set; `KUNA_JVM_TRACE_FILTER` filters; a second
tracing thread writes `<file>.1`, ...).

## Trace format

```
#vftrace 1 engine=vineflower hash=index shift=0     header (comparer ignores # lines)
=M pkg/Foo#bar(I)V                                  method key: class#name+descriptor
@ 0 Initial                                         step n (per method, call order) and name
<dump>
@ 1 CondenseSequences
= 0                                                 dump identical to step 0
@ 7 Loop_2_Start #2                                 repeated step name: k-th occurrence
```

Steps of a method are numbered across all of its records (the main pipeline, then the late
record), so a method's block can appear twice in the file (`=M` repeats whenever the method changes);
the comparer merges by key. Names carry Vineflower's own `Loop_N_` / `Merge_N_` prefixes.

`RerunClashing` steps come from the end of `VarProcessor.rerunClashing`, which both engines call twice
per decompiled method: from `setDebugVarNames` right after `codeToJava` (`RerunClashing`), and as the
last per-method act of `ClassWriter.invokeProcessors` (`RerunClashing #2`), i.e. after
`NestedClassProcessor`, `NestedMemberAccess`, `InitializerProcessor`, `EnumProcessor`, the record
fix-ups and the late passes, on the tree that is then rendered. Their dump is the tree followed by the
method's **variable table**:

```
vars nested                         ("nested": VarProcessor.nestedProcessed)
  v 1.0 "a" t=I FINAL clash="b" param this="a/B" ext inh="x" src="lambda$0":2.0 lvt="a":I:sig@4-14
  clashing "b" "q2"                 clashingNames() values, sorted (absent when empty)
  semaphores [2, 4]                 syntheticSemaphores, sorted (absent when empty)
```

One `v` line per pair that is a key of `mapVarNames` (`~` = a null name), a parameter, a this-var,
external or inherited, sorted by (var, version): name, `getVarType` (`~` = null), `getVarFinal`, then
only what is set: clashing name, `param`, this-var class, `ext`, inherited name, var source, LVT
(name, descriptor, signature, start-end).

A dump lists the statements in pre-order over each statement's `stats` list (indent = depth), then
the root's dummy exit `x`, then orphans:

```
s0 Root id=37 flags=TLS errcomment                   canonical id, kind, raw id, flags
  cont [s2 s4]          continueSet (sorted; "cont ~" = null)
  labels s2.1 *s5Rs2    labelEdges (sorted edge refs)
  > B s3 c=s1 unlabeled implicit noinline phantomcont exc=[a b]   successor edges, in list order
  < s2.0 s4.1           predecessor edges (sorted edge refs)
  >B [2 0]              a per-type successor list that is not the ALL list filtered by type
  def (var 7.0 t=I)     varDefinitions
  exprents 2            exprents (absent when null), one per line:
  - (assign@3-5 (var 1.0 t=I) (const X 2))
  head / init / cond / inc / caseedges / casevalues / guard / catch / catchvar / resource / monitor / comment / insn
o57 orphan Seq          a statement referenced from the tree but not in it
```

* **Statements**: `sN` is the pre-order index; `x` the root's `DummyExitStatement`; `o<raw id>`
  anything referenced but not in the tree (named by id, not by encounter order). Header fields:
  `id=` (raw `Statement.id`, which reaches the output as `label<id>`), `first=` (only when it is not
  `stats[0]`), `parent=` (only when it is not the tree parent), `post=`, `lbt=` (non-GENERAL
  `lastBasicType`), `monenter athrow monexit copied phantom`, and per type: Root `flags=` (T try, L
  loops, S switch) `errcomment`, comment lines; Block `rmexit`, `insn [offset:opcode ...]` while
  `exprents` is still null; General `placeholder`; If `iftype ifstat elsestat ifedge elseedge negated
  patmatched ppmm head`; Do `loop= init cond inc`; Switch `cases= default= scoped=`, head, case
  edges, case values (`default` for a null value), guards; Sync `body=` head; Try `catch i [types]`,
  `catchvar i`, `resource`; CatchAll `handler= finally`, `monitor`, `catchvar`; Exit `bc=` offsets.
* **Edges**: type letter R E B C F (D = the DIRECT_ALL list), destination, `c=` closure, and only
  the non-default flags. An edge is referenced as `sK.n` = n-th edge of sK's successor (ALL) list;
  `*sKTsJ` is an edge that is not in its source's list. `src=` / `!dst=` flag an edge stored in the
  list of a statement other than its source / destination.
* **Exprents**: `(kind@bytecode-offsets fields children)`. Kinds: annotation array assert assign
  const exit field func if invoke monitor new pattern switch switchhead var yield; enum values are the
  Java constant names (`BOOL_NOT`, `DYNAMIC`, ...). Variables print `index.version`, the raw `t=`
  type, the processor's `pt=` lower bound (`noproc` without a processor) and every flag; constants
  print int / `nL` / `f:bits` / `d:bits` (raw IEEE bits) / Java-escaped UTF-16 strings / `null`
  (strings are printed, and sorted, by their Java UTF-16 units: kuna keeps an unpaired surrogate as
  the private-use scalar `rt::jstring::SURROGATE_BASE + unit`, which `JavaStr::utf16` maps back). An
  exprent object reachable twice is printed once as `(&k ...)`, later as `^k`.
* **Types**: descriptor text (`I`, `[[La/B;`, `TT;` for a GENVAR), other code types by name
  (`INT:Q`); a `!f=`/`!s=` suffix when family or stack size differ from what the type implies;
  generic types add `<args>` (`?` unbound), a `+ - *` wildcard prefix and `{outer}`.
* Not `toJava()` text anywhere: rendering is itself being ported and depends on import state.

## Canonicalization, and why (the identity-hash problem)

Vineflower iterates `HashSet`/`HashMap`s keyed by identity-hashed objects; kuna cannot reproduce
JVM identity hashes and hashes by arena creation index instead. To find out which parts of the
intermediate state that touches, the Java tracer was run over the 598-case corpus (3,479 method
traces, 65,512 steps; 10 runs) and 14 JDK classes with their nested classes (2,649 methods; 9 runs)
under these hash assignments: stock twice, stock with the decompiler thread's identity-hash sequence shifted by
starting 1/2/3/5 throwaway threads first (`--shift`, which perturbs *every* identity hash, enums
included), `index`, and `random:1..4` (shadow classes: `Statement`, `StatEdge`, `Exprent`,
`BasicBlock`, `Instruction`, `ExceptionRangeCFG`, `VarVersionNode`, one counter per kuna arena).
With every element printed in raw order, two things differed between assignments, never anything else:

1. **The order of a statement's predecessor-edge list after `EliminateLoops`**: `EliminateLoopsHelper`
   re-targets `new ArrayList<>(loop.getLabelEdges())` (a `HashSet<StatEdge>`) with
   `changeDestination`, appending to the new destination's predecessor lists in hash order. Affected:
   4 corpus methods (`TestComplexIfElseChain.testInLoop1/2/3`, `TestExceptionElse.test`) and 2 JDK
   methods (`ConcurrentHashMap.get`, `ConcurrentHashMap$TreeBin.contendedLock`), under some or all
   non-stock assignments. The difference stays confined to that one `<` line through the last step.
2. **Names of orphan statements**, when they were numbered in encounter order: encounter order
   followed hash-set iteration (label edges), which showed up first in `BuildAsserts`
   (`TestAssertsEnabled.test10`) and `ProcessSwitchPatternMatch` (`TestSwitchPatternMatchingJ25.test6`).
   That was the dump's own artifact, not Vineflower state.

Raw statement ids, child order, successor order, per-type list order, exprent trees and the step
sequence were identical under all assignments. So the dump **sorts predecessor lists** (and prints
per-type predecessor deviations as edge refs, not list positions), **names orphans by raw id**, and
sorts every set it prints (label edges, continue sets, scoped case statements, `genericsMap`);
everything else stays in raw order, because a difference there was never hash noise and is a real
divergence. With these rules all nine corpus runs and all seven JDK runs of the final format produce
identical traces.
Stock Vineflower is not stable run to run either (two stock runs once differed in
`TestComplexIfElseChain`, probably because compiler threads started on demand draw from the
`os::random` sequence that seeds each new thread's identity hashes). `--hash index` additionally pins the
shadowed classes to kuna's arena order, so even raw predecessor order agrees whenever both engines
allocate in the same order; `trace-diff.sh` uses it. `-Dvftrace.rawpreds=1` (via `VF_JAVA_OPTS`)
turns predecessor sorting off for such experiments.

## Cost

Java side, whole oracle corpus (`corpus_trace.py`, 598 cases, 16 JVMs at `-Xmx1g`): 31-39 s wall,
~500 CPU-s, 43.9 MB of traces (65,512 steps over 3,479 methods; largest case 1.5 MB,
`TestComplexIfElseChain`), every produced `.java` byte-identical to the oracle's. Dumping takes 9%
of the in-JVM decompile time (33 s of 364 s; `= n` back-references make 80 MB of dumps 44 MB on
disk). The 14 heavier JDK classes: 67 MB, dumping 21% of decompile time. kuna with tracing off pays
one thread-local flag read per `DecompileRecord::add`.

## Blind spots

* Only state reachable from the root statement is dumped, plus the variable table of the
  `RerunClashing` steps: no CFG (`ControlFlowGraph`) between steps, no `DirectGraph`/SSA/
  `VarVersionsGraph`, no `VarProcessor` state between `codeToJava`'s passes except each variable's
  lower-bound type (`pt=`), no class-level state (field initializers moved out of `<clinit>`/`<init>`
  show up only as their absence from the `RerunClashing #2` tree; hidden members, imports, the
  `ClassWrapper` field-initializer lists and `MethodWrapper.setOuterVarNames` are not dumped).
* Rendering (`toJava`, `ClassWriter`) is not traced: what it computes on the fly (cast insertion,
  `isVarArgCall`, import shortening, parameter-name choice) is checked only by the final-text
  comparison. `sweep_compare.py` reports that comparison next to the traces.
* Not printed: exprent ids (`EXPRESSION_COUNTER`), `InvocationExprent.descriptor` (only the string
  descriptor), `ExitExprent.methodDescriptor`, `FieldExprent.descriptor` beyond its string,
  `AnnotationExprent.didWriteAlready`, `Statement.endpoints` (a cache), `RootStatement.mt`, the
  contents of orphan statements, instruction operands, `DummyExit` predecessor ownership details.
* A method that throws mid-pipeline simply stops; the comparer reports where the traces end.
* `--hash index` emulates kuna's arena index by Java constructor-entry order; where the port
  allocates an object after the children its Java constructor builds, the two orders differ
  (harmless for the statement trace given the measurements above).
* Traces are per JVM run; the Java side runs with `thread-count=1`. `run.sh` builds the Java tracer
  only when `$BUILD` has none; after editing `src/` run `build.sh` (or let `sweep.sh` do it).
* The Rust dumper is checked against the Java one on hand-built trees covering every statement
  type, exprent kind, `FunctionType`, the type printer, unpaired surrogates and the variable table
  (`tests/trace_vectors.rs`); real-method traces agree on the whole corpus, `java.base` and Guava
  (`sweep.sh`).

# vftrace: pass-by-pass differential tracer

kuna-jvm ports Vineflower 1.12.0 and aims at byte-identical Java output. When a class comes out
different, this tool tells you **which pass first went wrong, in which method**. Both engines call
`decompileRecord.add("<StepName>", root)` after every pass of the per-method pipeline
(`MethodProcessor.codeToJava`, plus `ClassWriter`'s late `FinalSimplifySwitches`/`FinalPrettifyIfs`);
at each call the tracer writes a canonical dump of the method's statement tree. Two traces are
compared method by method; the first step whose dump differs names the pass to look at.

| Piece | Where |
|---|---|
| Java tracer | `src/org/.../main/rels/DecompileRecord.java` (shadow of the released class: `add` also calls the tracer), `src/kuna/vftrace/{Dumper,Trace,TraceDriver,HashMode}.java`, `gen_shadow.py`, `build.sh`, `run.sh` |
| Rust tracer | `src/infra/kuna_trace.rs`, called from `DecompileRecord::add` (`src/infra/decompile_record.rs`); `examples/jvmtrace.rs` |
| Comparer | `compare.py`; `trace-diff.sh` runs both engines and the comparer |
| Corpus runner | `corpus_trace.py` (Java side over `~/kwt/jvm/corpus/manifest.json`) |
| Tests | `tests/trace_vectors.rs` + `tests/data/trace/` (both dumpers on the same hand-built trees), `tests/trace_file.rs` (trace file byte-identical to the Java one) |

## Running

```sh
tools/vftrace/build.sh        # once; outputs go to ~/kwt/jvm/scratch/s2-trace/vftrace-build ($BUILD)

# Vineflower: same arguments as the corpus oracle's VfDriver (--fixture, --opt k=v, inputs, -o)
tools/vftrace/run.sh --hash index --trace vf.vft --fixture --opt ternary-in-if=1 Foo.class 'Foo$1.class'

# kuna: same command line (cargo run -p kuna-jvm --example jvmtrace -- ...)
target/debug/examples/jvmtrace --trace kuna.vft --fixture --opt ternary-in-if=1 Foo.class 'Foo$1.class'
KUNA_JVM_TRACE=kuna.vft <anything that decompiles>      # env switch, read once per thread

tools/vftrace/compare.py vf.vft kuna.vft                 # per method: OK or first divergence + diff
tools/vftrace/trace-diff.sh Foo.class --fixture --opt ternary-in-if=1   # all of the above

# a whole corpus case set, Java side only (traces stay in --out)
python3 -I tools/vftrace/corpus_trace.py --out ~/kwt/jvm/scratch/s2-trace/corpus --hash index
```

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
  print int / `nL` / `f:bits` / `d:bits` (raw IEEE bits) / Java-escaped UTF-16 strings / `null`. An
  exprent object reachable twice is printed once as `(&k ...)`, later as `^k`.
* **Types**: descriptor text (`I`, `[[La/B;`, `TT;` for a GENVAR), other code types by name
  (`INT:Q`); a `!f=`/`!s=` suffix when family or stack size differ from what the type implies;
  generic types add `<args>` (`?` unbound), a `+ - *` wildcard prefix and `{outer}`.
* Not `toJava()` text anywhere: rendering is itself being ported and depends on import state.

## Canonicalization, and why (the identity-hash problem)

Vineflower iterates `HashSet`/`HashMap`s keyed by identity-hashed objects; kuna cannot reproduce
JVM identity hashes and hashes by arena creation index instead. To find out which parts of the
intermediate state that touches, the Java tracer was run over the 598-case corpus (3,479 method
traces, 65,512 steps) and 14 JDK classes with their nested classes (2,649 methods) under ten hash
assignments each: stock twice, stock with the decompiler thread's identity-hash sequence shifted by
starting 1/2/3/5 throwaway threads first (`--shift`, which perturbs *every* identity hash, enums
included), `index`, and `random:1..4` (shadow classes: `Statement`, `StatEdge`, `Exprent`,
`BasicBlock`, `Instruction`, `ExceptionRangeCFG`, `VarVersionNode`, one counter per kuna arena).
With every element printed in raw order, the only difference ever seen was **the order of a
statement's predecessor-edge list**, first produced by three passes:

| pass | why | methods affected (any assignment vs. stock) |
|---|---|---|
| `EliminateLoops` | `EliminateLoopsHelper` re-targets `new ArrayList<>(loop.getLabelEdges())` (a `HashSet<StatEdge>`) with `changeDestination` | corpus 4 (TestComplexIfElseChain ×3, TestExceptionElse), JDK 2 (`ConcurrentHashMap.get`, `ConcurrentHashMap$TreeBin.contendedLock`) |
| `BuildAsserts` | `AssertProcessor` edge rewiring in label-edge order | corpus 1 (TestAssertsEnabled.test10) |
| `ProcessSwitchPatternMatch` | iterates `stat.getParent().getLabelEdges()` while adding edges | corpus 1 (TestSwitchPatternMatchingJ25.test6) |

In every case the difference stayed confined to that one predecessor line through the last step;
raw statement ids, child order, successor order, exprents and the step sequence never changed. The
dump therefore **sorts predecessor lists** (and prints per-type predecessor deviations as edge refs,
not list positions) and sorts every set it prints (label edges, continue sets, scoped case
statements, `genericsMap`); everything else is kept in raw order, because a difference there was
never hash noise and is a real divergence. Stock Vineflower is not even stable run to run (two
stock runs differed in 2 corpus cases: JIT compiler threads started on demand shift the decompiler
thread's hash seed), which is one more reason not to compare against stock dumps unsorted.
`--hash index` additionally pins the shadowed classes to kuna's arena order, so the traces agree on
predecessor order whenever both engines allocate in the same order; `trace-diff.sh` uses it.

## Blind spots

* Only state reachable from the root statement is dumped: no CFG (`ControlFlowGraph`) between steps,
  no `DirectGraph`/SSA/`VarVersionsGraph`, no `VarProcessor` state except each variable's lower-bound
  type (`pt=`), no variable names (assigned after `codeToJava`), no class-level state.
* Not printed: exprent ids (`EXPRESSION_COUNTER`), `InvocationExprent.descriptor` (only the string
  descriptor), `ExitExprent.methodDescriptor`, `FieldExprent.descriptor` beyond its string,
  `AnnotationExprent.didWriteAlready`, `Statement.endpoints` (a cache), `RootStatement.mt`, the
  contents of orphan statements, instruction operands, `DummyExit` predecessor ownership details.
* A method that throws mid-pipeline simply stops; the comparer reports where the traces end.
* `--hash index` emulates kuna's arena index by Java constructor-entry order; where the port
  allocates an object after the children its Java constructor builds, the two orders differ
  (harmless for the statement trace given the measurements above).
* Traces are per JVM run; the Java side runs with `thread-count=1`.
* The Rust dumper is checked against the Java one on hand-built trees covering every statement
  type, exprent kind, `FunctionType` and the type printer (`tests/trace_vectors.rs`); real-method
  traces could not be compared yet because the port's pipeline stops in unported pass bodies.

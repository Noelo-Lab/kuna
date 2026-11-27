# 10 — The JVM engine (vineflower)

```yaml
Anchors:
  - decompiler/nonnative/kuna-jvm/src
  - decompiler/nonnative/kuna-jvm/options.toml
  - decompiler/crates/kuna-cli/src/nonnative
  - decompiler/nonnative/kuna-managed
```

Chapters 01-09 describe the native pipeline: machine code lifted through SLEIGH to
p-code and recovered by `kuna-decomp`. This chapter describes the second, non-native
engine, `kuna-jvm`, which decompiles JVM class files and jars to Java, and classes
compiled from Kotlin to Kotlin, and shares none of that pipeline. JVM bytecode is typed and stack-based, and the class file already carries
what native recovery has to infer: the class structure, member signatures, generic
signatures, exception tables, nesting attributes and bootstrap methods for lambdas. The
engine therefore works over a Java-level IR of its own (statements and expressions, in
`decompiler/nonnative/kuna-jvm/src/substrate`) instead of over p-code. Why it is a
separate engine, how it is tested and how it is maintained is `docs/jvm.md`; this chapter
is what it computes.

**Lineage.** The engine is a line-by-line Rust port of Vineflower 1.12.0, itself a fork
of JetBrains' Fernflower. In this chapter, untagged prose describes the Vineflower port
and `(kuna)` marks what kuna added or replaced. Upstream names are given as
parenthetical provenance (`MethodProcessor.codeToJava`); every claim is anchored to the
Rust file that implements it. Option names are the `jvm_*` rows of
`decompiler/nonnative/kuna-jvm/options.toml`; their defaults, tiers, symptoms and flip
guidance are in the generated catalog ([docs/options-jvm.md](../options-jvm.md)).

**The fidelity contract** shapes everything below: on the same class files and options,
the engine prints the text Vineflower prints, byte for byte, except where one of kuna's
fixes of Vineflower's defects applies (section 10.16). The port therefore keeps
Vineflower's pass order, its iteration orders (including the iteration order of Java's
hash collections, which reaches the output), its id counters (which reach the output as
`var10000` and `label7`) and its failure behaviour. Where this chapter describes a
heuristic that looks arbitrary, it is Vineflower's, kept for that reason. kuna's source forms
choose the source's form of equivalent code over Vineflower's (source locals and source
patterns, section 10.10).

## 10.1 The phase folders and the schedule

The source folders reuse kuna's phase taxonomy so a reader can find a stage by its
phase, but the order of execution is the engine's own:

| Folder | Holds |
|---|---|
| `p0_knowledge/` | options and the decompilation context, the class universe and where classes are read from (jars, directories, the JDK), the input front door (kuna) |
| `p1_classfile/` | the class-file model (constant pool, members, attributes), class nesting, the per-class method loop |
| `p2_lift/` | bytecode decoding, the control-flow graph and its repairs, the stack-to-expression lift |
| `p3_dataflow/` | SSA and SSAU forms over the expression IR and the simplifications they license |
| `p4_calls/` | lambdas and method references, synthetic accessors, dynamic constants, JDK varargs |
| `p5_types/` | the variable type lattice, generic signatures and casts |
| `p6_variables/` | variable versions, declaration placement, naming |
| `p7_regions/` | dominator-based recovery of the statement tree, irreducible flow, `finally` |
| `p8_structure/` | refinement of the statement tree into Java constructs |
| `p9_emit/` | the Java writer, imports, line wrapping, line mapping |
| `substrate/` | the shared IR: statements, expressions, edges, types and the set/map containers |
| `rt/` | (kuna) the Java-runtime kernel the port stands on (section 10.13) |
| `infra/` | the per-method schedule, pass hooks, the parallel driver and the debugging aids |
| `plugins/` | Vineflower's VariableRenaming and IdeaNotNull plugins (this section, below) |
| `kotlin/` | Vineflower's Kotlin plugin: metadata decoding, its pass schedule, expression and statement layer and writer (section 10.15) |
| `fixes/` | (kuna) the fixes of Vineflower's defects, one module per family, called from the folders above (section 10.16) |

The schedule has two levels. **Per input**, `decompiler/nonnative/kuna-jvm/src/p0_knowledge/fernflower.rs
(Fernflower::decompile_context)` loads the class universe and nests classes (section
10.2), then saves every own unit (`decompiler/nonnative/kuna-jvm/src/p0_knowledge/struct_context.rs
(StructContext::save_context)`). Each unit is saved in two phases
(`decompiler/nonnative/kuna-jvm/src/p0_knowledge/context_unit.rs (ContextUnit::save_selected)`):
first every root class of the unit is *processed* (its methods decompiled, its nested
classes attached), then every root class is *written*. The split is load-bearing: writing
one class reads state that processing other classes left behind (the constructor masks of
another class's nested classes, the switch maps of synthetic classes), so no class is
written before all of its unit is processed. **Per method**,
`decompiler/nonnative/kuna-jvm/src/infra/method_processor.rs (MethodProcessor::code_to_java)`
runs the pass schedule of sections 10.4-10.10 from bytecode to a structured statement
tree. Every pass is recorded by name through
`decompiler/nonnative/kuna-jvm/src/infra/decompile_record.rs (DecompileRecord::add)`.

Vineflower's plugin API is ported: the pass locations (`CfgConstruction`, `BeforeMain`,
`InLoopDecomp`, `MainLoop`, `AfterMain`, `AtEnd`, `LateProcessing`) are kept in the
schedule, and `decompiler/nonnative/kuna-jvm/src/infra/plugin_context.rs (PluginContext)`
registers each plugin's passes there and asks it for a language spec, which takes over the
method pipeline and the class writer for the classes it claims. Plugins are
not discovered on a class path: the engine carries the plugins `java -jar vineflower.jar`
bundles, in its registration order, in
`decompiler/nonnative/kuna-jvm/src/infra/kuna_plugins.rs`, and the custom property
`jvm_plugins` (on unless set; the corpus runner turns it off for Vineflower's test fixture,
which loads none) decides whether they load. It is not a catalog option: off gives what
`jvm_kt_enable off` gives, without the IdeaNotNull pass. Each plugin's options join the option
map with the plugin's defaults when the plugins initialize, and an option a caller set
keeps its value. Of the two Java plugins, IdeaNotNull is ported, in
`decompiler/nonnative/kuna-jvm/src/plugins`; VariableRenaming (JAD-style and tiny-style
renamers of locals) is not, and every name stays as Vineflower's own pipeline gives it.
**IdeaNotNull** registers one
`MainLoop` pass that removes the
null checks IntelliJ IDEA's compiler instruments: an `if (p == null) $$$reportNull$$$0(n);`
heading the method body for a parameter whose invisible parameter annotations carry
`org/jetbrains/annotations/NotNull` (the annotation lists counted from the last parameter
when javac recorded fewer lists than parameters) is replaced, head block and all, by an
empty block; and, in a method annotated `@NotNull`, an `if (v == null) $$$reportNull$$$0(n);`
right before `return v;` is removed, its head block's expressions moving into the return's
block, as is the older `if (v == null) <exit>; else return v;` shape. The third bundled
plugin, **Kotlin**, is a language plugin: it claims the classes the Kotlin compiler
emitted and decompiles and writes them as Kotlin (section 10.15). Every other class is
written as Java; Vineflower has no other language support.

## 10.2 The class universe and nesting

**Inputs (kuna).** `decompiler/nonnative/kuna-jvm/src/p0_knowledge/kuna_input.rs (classify, load)`
names an input's kind and reads it into an `InputSet`: a class file (`CA FE BA BE` and a
major version from 45 to 255 at bytes 6-7, which is what tells it from a fat Mach-O with
the same magic), an archive (a zip local or empty-archive header, or a `.jar`/`.war`/`.ear`/`.zip`
file with a zip end record in its last KiB, for launcher-script jars), or a directory. A
`.jmod` and a jimage are recognized and refused; a DEX file, and a zip without `.class`
entries (an APK), are refused as not JVM bytecode. A *library* is read the same way by
`load_library`, except that an archive or a directory without class files is an empty set
rather than an error, since Vineflower adds such a library without complaint (a Kotlin
Multiplatform root jar holds only metadata). The `InputSet` holds the class bytes as
shared read-only buffers and hands each engine thread its own context-source views, which
list exactly what Vineflower's `JarContextSource`, `DirectoryContextSource` and
`SingleFileContextSource` list for the same path, in the same order. For a single class
file, `jvm_siblings` also loads the `Name$*.class` files next to it, each as its own
source after the class, as Vineflower's test fixture and IDE integration do; without them
a class's nested, local and anonymous classes are missing. Archive entries keep their
names: the multi-release variants under `META-INF/versions/<n>/` are skipped, and a
prefixed entry such as `BOOT-INF/classes/a/B.class` is placed by its class name, not its
path. A large archive's class entries are inflated on up to eight threads, at most one
per 256 entries, each with its own reader of the archive; the results keep entry order,
and a part whose reader fails is read again on the calling thread.

**Sources and libraries.** `decompiler/nonnative/kuna-jvm/src/p0_knowledge/struct_context.rs
(StructContext)` holds the session's units: *own* units (what is decompiled) and library
units (read for context only: the user's libraries and the JDK class library). A class
name is looked up in the unit that lists it, and lookups are cached, failed ones
included; a class found under an entry path that does not match its name is re-keyed by
its real name. The class
reader is `decompiler/nonnative/kuna-jvm/src/p0_knowledge/context_source.rs`, with
`decompiler/nonnative/kuna-jvm/src/substrate/zip_reader.rs (ZipReader)`, which follows JDK
21's `java.util.zip.ZipFile` (end-record search, central-directory order, prefix stubs
absorbed through the central-directory offset, stored and deflated entries only).

**The JDK (kuna).** Vineflower reads the Java standard library through the `jrt:` file
system of the JVM it runs on. kuna has no JVM, so
`decompiler/nonnative/kuna-jvm/src/p0_knowledge/jdk_runtime.rs (JdkRuntime)` locates a
JDK home and reads its modules itself: from the `lib/modules` jimage, else from
`jmods/*.jmod`, else (Java 8) from the jars in `jre/lib` and `lib`. Module entries are
listed in the order the `jrt:` walk produces, including the duplicate `module-info.class`
entry a JDK 17 or older `jrt-fs.jar` reports. `jvm_include_runtime` selects the JDK: `on`
(or `current`) searches `JAVA_HOME`, then the `java` on `PATH`, then the usual install
directories (newest version first); a path names a home; `off` uses none. The JDK only
supplies context. Without it, the engine still decompiles every class, but whatever needs
a JDK class's members or hierarchy degrades: `@Override` on a method that overrides a JDK
method is not written, a diamond `new ArrayList<>()` prints raw, and casts and generic
arguments that depend on JDK signatures can differ. The vendored test corpus runs without
a JDK.

**Renaming.** With `jvm_rename_members`,
`decompiler/nonnative/kuna-jvm/src/p0_knowledge/identifier_converter.rs (IdentifierConverter)`
renames classes, fields and methods whose names are not valid Java identifiers, are
reserved words or are short obfuscated names, before anything is loaded, rewriting every
constant-pool reference through `decompiler/nonnative/kuna-jvm/src/p0_knowledge/pool_interceptor.rs`.
The naming policy is `decompiler/nonnative/kuna-jvm/src/p0_knowledge/converter_helper.rs
(ConverterHelper)`; a user renamer class (Vineflower loads one by name) cannot be loaded
without a JVM, so the default policy is always used.

**Nesting.** `decompiler/nonnative/kuna-jvm/src/p1_classfile/classes_processor.rs
(ClassesProcessor::load_classes)` turns the own classes into a forest of class nodes. A
class whose qualified name was already loaded is skipped with an error log. Every own class
joins the forest: which roots are written is the driver's selection (`DriverConfig::select`,
section 10.14), and the others still give context. With `jvm_decompile_inner`
(the default), the `InnerClasses` entries of every class are read into records of type
*member*, *local* or *anonymous* (an entry with no simple name is anonymous; one whose
`EnclosingMethod` names a method is local), each remembering which class declared it.
Classes often disagree about a nested class; the record from the nested class itself
wins over its enclosing class's, which wins over a third class's, and each disagreement
is logged. A member entry is believed only when its name is the
outer name, `$`, and its simple name. Then, from each root, a breadth-first walk attaches
nested classes to their enclosing class: an anonymous class loses `static` and records
the supertype it instantiates, a local class keeps only the modifiers a local declaration
can carry, and `jvm_verify_anonymous_classes` first checks that a supposedly anonymous
class has one supertype and is created at most once by its enclosing class (never cast
to, tested, or read statically); a class that fails is written as a local class instead.
With `jvm_decompile_inner` off, no class is nested and every class file is written as its
own top-level class. Lambdas get nodes too, but later (section 10.11).

## 10.3 The class-file model

`decompiler/nonnative/kuna-jvm/src/p1_classfile/struct_class.rs (StructClass)`,
`struct_method.rs` and `struct_field.rs` (in the same folder) model a class: access
flags, the constant pool (`decompiler/nonnative/kuna-jvm/src/p1_classfile/constant_pool.rs`),
members, and the attributes Vineflower reads, each a view over one attribute object
(`decompiler/nonnative/kuna-jvm/src/p1_classfile/struct_general_attribute.rs`).
`decompiler/nonnative/kuna-jvm/src/p1_classfile/bytecode_version.rs (BytecodeVersion)`
turns the major version into the feature predicates the passes test: `jsr` is legal up to
Java 6, lambdas from Java 8, `invokedynamic` string concatenation from Java 9, `instanceof`
patterns and switch expressions from Java 16, sealed classes from Java 17, record
patterns from Java 21; preview versions count from the release that previewed them.

Two lifetimes coexist (kuna). Classes, members, attributes and the class-node forest live
for the whole session; the IR a class's decompilation builds lives only until that root
class is finished. Method bodies are therefore kept as raw bytes and expanded on demand
(`decompiler/nonnative/kuna-jvm/src/p1_classfile/struct_method.rs (StructMethod::expand_data)`),
and annotations, which Vineflower parses straight into expressions, are parsed at load
time into a session-lived element tree and built into expressions on first use in each
class scope (`decompiler/nonnative/kuna-jvm/src/p1_classfile/struct_annotation_attribute.rs`).
A session object never hands out a class-scope object of an earlier decompilation, which an
engine the driver reuses for another batch would otherwise read (section 10.14): an expanded
sequence whose arena was reset counts as released
(`decompiler/nonnative/kuna-jvm/src/p1_classfile/struct_method.rs (StructMethod::get_instruction_sequence)`
does not return it, `expand_data` rebuilds it; the Kotlin plugin's methods stay expanded after
they are decompiled, as in Java, and so does a method whose decompilation threw), and the class
nodes drop their wrappers and an anonymous class's kept super call when a decompilation ends
(`decompiler/nonnative/kuna-jvm/src/p1_classfile/classes_processor.rs (ClassesProcessor::end_class_scope)`).
A recogniser that took the stale sequence of a constructor accessor for an expanded one failed
the method it was decompiling (`TimeMark.DefaultImpls.plus-LRDsOJo` of kotlin-stdlib, with
"stale FullInstructionSequence handle", whenever the batch holding `AdjustedTimeMark` had run
on the same engine first).
Reading is lazy where no output can tell (kuna). Unless a pool interceptor is set
(renaming), a class is built from its header, interfaces and class attributes only: its
constant pool records where each entry lies and reads an entry the first time it is
asked for, resolving the entries it names first, and its fields and methods are read on
the first `StructClass::get_fields`/`get_methods`
(`decompiler/nonnative/kuna-jvm/src/p1_classfile/constant_pool.rs (ConstantPool::new)`).
Resolving on demand gives the values of Java's two resolution passes whenever every
reference points at an entry of the kind the pass expects and every method descriptor a
link constant checks is well formed; a construction-time scan checks exactly that and
reads any other pool eagerly, so a malformed pool fails where Java's does. Laziness
changes only the creation order, hence the identity hashes, of member and attribute
objects, and those hash only into sets the engine tests for membership. Class bytes are
shared with the input, not copied. A member whose content (not its framing) is malformed
fails when its class is first used rather than when the input is added.
`jvm_decompile_generics` decides whether `Signature` attributes are parsed into generic
descriptors (`decompiler/nonnative/kuna-jvm/src/p5_types/generic_main.rs`); off, every
type prints erased. Debug attributes feed naming later: the `LocalVariableTable` and
`LocalVariableTypeTable` (`jvm_use_lvt_names`) and `MethodParameters`.

## 10.4 Bytecode to a control-flow graph

**Decode.** `StructMethod::expand_data` decodes the code attribute into an instruction
sequence with its exception table
(`decompiler/nonnative/kuna-jvm/src/p2_lift/full_instruction_sequence.rs`,
`decompiler/nonnative/kuna-jvm/src/p2_lift/instruction.rs`), one instruction per opcode
with resolved operands; the comment of a failed method later lists this sequence. The method's local-variable count seeds the variable counter, so every
variable the decompiler invents is numbered after the method's own locals.

**Blocks.** `decompiler/nonnative/kuna-jvm/src/p2_lift/control_flow_graph.rs (ControlFlowGraph::new)`
splits the sequence at leaders (jump and switch targets, the instruction after a jump,
a return or a switch, handler starts and the bounds of every protected range), links
the blocks by their regular successors, gives each block of a protected range an
exception edge to its handler, and links `jsr` subroutines. The first pass then removes
every block not reachable from the entry over regular or exception edges
(`decompiler/nonnative/kuna-jvm/src/p2_lift/dead_code_helper.rs (DeadCodeHelper::remove_dead_blocks)`).

**JSR.** For class files of Java 6 and older, or any class file under
`jvm_force_jsr_inline`, `ControlFlowGraph::inline_jsr` removes subroutines: each
subroutine body is copied for every `jsr` that reaches it, exception ranges are split to
follow the copies, and the `jsr`/`ret` instructions are deleted. After that the graph has
only regular, exception and exit edges.

**Exits and gotos.** Every block that leaves the method is connected to one dummy exit
block (`DeadCodeHelper::connect_dummy_exit_block`). `goto` instructions are then deleted,
since the edges already carry the flow, except in cycles made only of gotos, and the
blocks this empties are removed (`DeadCodeHelper::remove_gotos`).

**Exception-range repairs.** Real compilers and obfuscators produce exception tables the
structurer cannot use directly, so a fixed sequence of repairs runs in
`decompiler/nonnative/kuna-jvm/src/p2_lift/exception_deobfuscator.rs (ExceptionDeobfuscator)`
and `DeadCodeHelper`:

1. `remove_circular_ranges` - a range whose handler lies inside the range it protects
   (an obfuscator pattern) loses the blocks the handler reaches by itself.
2. `restore_pop_ranges` - javac leaves the first instruction of a handler, the one that
   stores or discards the caught exception (`astore`, `pop`), outside an enclosing range
   that protects the same code. When a range's handler starts that way and another range
   covers its protected blocks, that instruction is split into a block of its own and put
   back into the outer range, so a nested `try` stays inside the outer one. Ranges with
   the same handler and exception types are considered together.
3. `remove_empty_ranges` - a range that protects no
   instructions is dropped together with its exception edges, and the method gets a
   `$VF: Removed empty exception range` comment.
4. `DeadCodeHelper::extend_synchronized_range_to_monitorexit` - when some path out of a `monitorenter` region
   has no `monitorexit` inside the protected range, the range is extended to it, so the
   region can still become a `synchronized` block.
5. `DeadCodeHelper::incorporate_value_returns` - a block
   that only returns (a bare `return`, or one load and a return) is added to the
   exception ranges that protect all of its predecessors and taken out of ranges that
   protect none of them, so `return x;` stays inside the `try` or `catch` it belongs to
   instead of being hoisted after the `try` through a temporary.
6. `insert_empty_exception_handler_blocks` - every handler gets an empty block of its
   own in front of it.
7. `DeadCodeHelper::merge_basic_blocks` - straight-line chains of blocks are merged.
8. If a range has several entry blocks (`has_obfuscated_exceptions`; javac never emits
   one), `handle_multiple_entry_exception_ranges` splits off, entry by entry, the blocks
   of the range that entry reaches and dominates as a range of their own (comment
   `$VF: Handled exception range with multiple entry points by splitting it`). Then
   `insert_dummy_exception_handler_blocks` gives each range of a handler shared by
   several ranges a handler of its own: a copy of the handler block by default
   (`$VF: Duplicated exception handlers to handle obfuscated exceptions`), or with
   `jvm_old_try_dedup` a dummy block in front of the shared handler. A range that cannot
   be split leaves the method with `$VF: Could not handle exception ranges with multiple
   entries`, and structuring is attempted anyway.

## 10.5 Statement recovery

`decompiler/nonnative/kuna-jvm/src/p7_regions/dom_helper.rs (DomHelper::parse_graph)`
turns the graph into a statement tree. Each block becomes a basic-block statement inside
one general statement (an unstructured region), under a root statement with a dummy exit;
regular edges become `REGULAR` edges, an edge back to the first block becomes `CONTINUE`,
an edge to the method exit becomes `BREAK` to the dummy exit, and each protected block
gets an `EXCEPTION` edge to its handler.

**The simple case.** `DomHelper::find_simple_statements` walks the region's statements in
post-reverse-post-order and asks each, as a head, whether it starts one of six statement
shapes, in this order: a loop (`DoStatement::is_head`), a switch, an if, a sequence, a
`try`/`catch` (`CatchStatement::is_head`) and a catch-all (`CatchAllStatement::is_head`,
the shape of every `finally` and `synchronized`). The first match collapses its nodes into
one statement and the walk restarts; the region is done when one statement covers it.
Everything is first recognized in its most general form - every loop is an infinite loop
with breaks, every `try` a plain `try`, every catch-all an unprocessed `finally` - and the
later passes refine it. The `try` and catch-all shapes are refused when their head would
merge an `if` into a `try` inside a loop (`decompiler/nonnative/kuna-jvm/src/p7_regions/dec_helper.rs
(DecHelper::invalid_head_merge)`), which otherwise builds malformed
loops around `try`/`catch`.

**When the simple case is stuck.** `DomHelper::find_general_statement` carves out a
single-entry subregion bounded by a postdominator of its head and recurses into it: the
candidates come from the region's postdominator tree on the first attempt, then from its
*extended* postdominators
(`decompiler/nonnative/kuna-jvm/src/p7_regions/fast_extended_postdominance_helper.rs`),
and a candidate is accepted only if it has no entry besides its head, does not cut an
exception range in half, and does not split a `monitorenter` from its successor. The
recursion is the reason the engine can structure nested loops with breaks and continues
that no single-level pattern would match: it solves the inner region first and uses the
result as one node outside.

**Irreducible flow.** If a region still does not reduce,
`decompiler/nonnative/kuna-jvm/src/p7_regions/irreducible_cfg_deobfuscator.rs` checks
whether it is irreducible and, if so, splits one node and tries again: among the
statements with several predecessors it copies the one with the fewest successors, then
the smallest size times its extra predecessors, and moves one predecessor's edges to the
copy. There are up to five such rounds per region, and the whole procedure is retried once
with a freshly computed postdominance map. A split adds
`$VF: Irreducible bytecode was duplicated to produce valid code`; exhausting the rounds
adds an error comment that the flow was not entirely decomposed. A region that is
reducible but still cannot be decomposed, or whose irreducible node cannot be split, fails
the method (`parsing failure!`, section 10.12).

After the tree is built, `parse_graph` lowers `continue` labels, condenses sequences,
computes the monitor flags and rebuilds `synchronized` blocks:
`DomHelper::build_synchronized` turns a `monitorenter` followed by a catch-all whose
handler exits the monitor into a synchronized statement and marks the duplicated
`monitorexit` instructions dead.

## 10.6 `finally`

javac compiles `try { A } finally { F }` by copying `F` onto every exit of `A` and of
each `catch`, plus a catch-all handler that runs `F` and rethrows.
`decompiler/nonnative/kuna-jvm/src/p7_regions/finally_processor.rs (FinallyProcessor::iterate_graph)`
undoes that, one catch-all at a time, on the control-flow graph, and the schedule
re-runs `DomHelper::parse_graph` after each success: the copies are removed from the
graph, so the statement tree has to be rebuilt from it.

For each unprocessed catch-all, it first reads the handler's shape (where the exception
is stored, where the rethrow is, whether it ends in a return of a saved value). With
`jvm_decompile_finally`, it then verifies that every exit of the protected region is
followed by an exact copy of the handler body, comparing the blocks instruction by
instruction (`FinallyProcessor::verify_finally_ex`, `compare_subgraphs_ex`), where
local-variable operands may differ when an SSA analysis shows they hold the same value.
When every copy matches, the copies are deleted and the catch-all becomes a `finally`.
When they do not match, or
the option is off, the engine keeps the code correct instead of pretty: it adds a
synthetic *semaphore* variable, set on entry to the `try` and cleared on every edge that
leaves it normally, and the `finally` body runs only while the semaphore is set
(`FinallyProcessor::insert_semaphore`); the copies on the normal exits stay where they
are. The method gets the comment
`$VF: Could not verify finally blocks. A semaphore variable has been added to preserve control flow.`
(under `jvm_decompiler_comments`). A handler whose shape cannot be read at all stays a
catch-all with the comment `$VF: Could not inline inconsistent finally blocks`. Each
change also re-runs dead-block removal and block merging on the graph.

The schedule then rebuilds `synchronized` statements once more, removes their exception
handlers (`DomHelper::remove_synchronized_handler`), condenses sequences and clears
structure-only statements
(`decompiler/nonnative/kuna-jvm/src/p7_regions/clear_struct_helper.rs`).

## 10.7 The stack-to-expression lift

`decompiler/nonnative/kuna-jvm/src/p2_lift/expr_processor.rs (ExprProcessor::process_statement)`
converts each basic block's instructions into expressions. The tree is flattened into a
direct graph of blocks
(`decompiler/nonnative/kuna-jvm/src/p3_dataflow/flatten_statements_helper.rs`), and the
operand stack is simulated along it: every value pushed is assigned to a *stack variable*
(numbered from 10000, `VarExprent::STACK_BASE` in
`decompiler/nonnative/kuna-jvm/src/substrate/var_exprent.rs`), every instruction that
consumes the stack reads those variables, and a block's stack on entry is taken from its
predecessors. `dup` and `swap` families copy variables rather than expressions, and a
catch handler starts with its exception on the stack. The result is correct and
unreadable - `var10000 = System.out; var10001 = "hi"; var10000.println(var10001);` - and
the next stage folds it back. Expression nodes are in
`decompiler/nonnative/kuna-jvm/src/substrate/exprent.rs` (one kind per Java expression
form: assignment, constant, field, function and operator, invocation, `new`, array,
`instanceof` pattern, switch, exit, monitor, and so on).

When a constructor call names a constructor its class does not declare (invalid or
tampered bytecode), the method fails with `Constructor .. not found`; with
`jvm_ignore_invalid_bytecode` the call is printed without hiding synthetic arguments
instead (`decompiler/nonnative/kuna-jvm/src/substrate/expr_util.rs`).

## 10.8 SSA, SSAU and stack-variable inlining

The lifted tree goes through a fixed point
(`MethodProcessor::code_to_java`): simplify stack variables, recompute variable versions
(section 10.10), and look for `++`/`--` shapes
(`decompiler/nonnative/kuna-jvm/src/p3_dataflow/p_pand_mm_helper.rs (PPandMMHelper::find_p_pand_mm)`);
repeat while the last step finds one.

`decompiler/nonnative/kuna-jvm/src/p3_dataflow/stack_vars_processor.rs (StackVarsProcessor::simplify_stack_vars)`
alternates two views of the variables:

- **SSA** (`decompiler/nonnative/kuna-jvm/src/p3_dataflow/ssa_constructor_sparse_ex.rs`):
  every assignment of a variable gets its own version. On it,
  `decompiler/nonnative/kuna-jvm/src/p3_dataflow/simplify_exprents_helper.rs
  (SimplifyExprentsHelper::simplify_stack_vars_statement)` runs peephole rewrites to a
  fixed point: a stack variable assigned and then used once next to it is inlined; array
  creation followed by element stores becomes an array initializer; `i = i + 1` shapes
  become `++`/`--`; `new` followed by the constructor call becomes one constructor
  expression; the two-branch assignment of a ternary becomes `?:`; the javac null check
  `outer.getClass()` before a qualified `outer.new Inner()` is dropped
 ; the pre-Java-5 `class$` idiom is folded into a class literal
  (finished at class level by section 10.11).
- **SSAU** (`decompiler/nonnative/kuna-jvm/src/p3_dataflow/ssau_constructor_sparse_ex.rs`):
  versions also change at every use, which yields a graph of definitions and uses. On
  the first round it marks effectively-final variables; on every round
  `StackVarsProcessor::iterate_statements` walks the blocks and inlines a variable's
  definition into its use when no intervening write can change the value. Each block is
  swept twice; the second sweep, which also inlines across the boundaries between
  expressions on the operand stack, is what
  turns chains of temporaries back into `a = b = c`, `arr[i++] = v` and chained calls.

Both forms are reverted (all versions back to zero) after each use, so each round
re-derives them from the current tree. Then, once: `++`/`--` inside conditions is inlined
(`PPandMMHelper::inline_pp_iand_mmi_if`); for Java 9+ class files,
`decompiler/nonnative/kuna-jvm/src/p3_dataflow/concatenation_helper.rs` turns
`invokedynamic` string concatenation back into `+`; and
`decompiler/nonnative/kuna-jvm/src/p8_structure/assert_processor.rs` rebuilds `assert`
statements from the `$assertionsDisabled` check and the `throw new AssertionError(..)`
that javac emits.

## 10.9 The main structuring loop

The tree now holds correct but generic statements. The main loop of
`MethodProcessor::code_to_java` refines it; each pass returns whether it changed the tree,
and most changes restart the loop from the top, so the order below is a priority order.

1. `LabelHelper::clean_up_edges`
   (`decompiler/nonnative/kuna-jvm/src/p8_structure/label_helper.rs`) normalizes the
   edges: every `break`/`continue` points at the statement that really closes it.
2. **The loop-merge loop**, only when the method has loops, repeated until nothing
   changes:
   `decompiler/nonnative/kuna-jvm/src/p8_structure/eliminate_loops_helper.rs` removes loops
   that never loop; `decompiler/nonnative/kuna-jvm/src/p8_structure/merge_helper.rs
   (MergeHelper::enhance_loops)` turns infinite loops with a leading or trailing exit test
   into `while`, `do`-`while` and `for` loops, and iterator and array loops into for-each;
   `decompiler/nonnative/kuna-jvm/src/p8_structure/loop_extract_helper.rs` moves code that
   always leaves the loop out of it; and
   `decompiler/nonnative/kuna-jvm/src/p8_structure/if_helper.rs (IfHelper::merge_all_ifs)`
   merges nested `if`s into `&&`/`||` conditions and builds `if`-`else`. It
   also collapses the nested ifs a ternary inside a condition
   compiles to (`if (a ? b : c)`).
3. Stack variables are simplified and variable versions recomputed again, and
   `LabelHelper::identify_labels` decides which jumps need an explicit label.
4. `decompiler/nonnative/kuna-jvm/src/p3_dataflow/secondary_functions_helper.rs
   (SecondaryFunctionsHelper::identify_secondary_functions)` rewrites operator shapes:
   an `lcmp`/`fcmpl`/`dcmpg` result compared with zero becomes a direct comparison (one
   used as a value is spelled out as a ternary), `c ? true : false` and `c ? false : true`
   fold to `c` and `!c`, negations are pushed into comparisons, and an `if`-`else` with an
   empty `if` branch is inverted. The Kotlin plugin extends the folding to every ternary
   with one boolean constant branch (`c ? true : x` to `c || x`).
5. `decompiler/nonnative/kuna-jvm/src/p5_types/intersection_cast_processor.rs` builds
   intersection casts (`(A & B) x`).
6. With `jvm_pattern_matching`, on Java 16+ class files,
   `decompiler/nonnative/kuna-jvm/src/p8_structure/if_pattern_match_processor.rs
   (IfPatternMatchProcessor::match_instanceof)` turns `instanceof` followed by a cast into
   an `instanceof` pattern, record patterns included. (kuna) Only from javac's compiled
   pattern (fix `pattern_binding_shape`, section 10.16), and not where the line table shows
   the cast as a statement (source patterns, section 10.10).
7. When the method has switches:
   `decompiler/nonnative/kuna-jvm/src/p8_structure/switch_pattern_match_processor.rs`
   rebuilds pattern switches from `SwitchBootstraps.typeSwitch` dispatch (also gated by
   `jvm_pattern_matching`), and
   `decompiler/nonnative/kuna-jvm/src/p8_structure/switch_expression_helper.rs` rebuilds
   switch expressions with `->` arms and `yield` from the statement switch and temporary
   javac emits (`jvm_decompile_switch_expressions`).
8. When the method has a `try`:
   `decompiler/nonnative/kuna-jvm/src/p8_structure/try_helper.rs (TryHelper::enhance_try_stats)`
   rebuilds try-with-resources
   (`decompiler/nonnative/kuna-jvm/src/p8_structure/try_with_resources_processor.rs`; the
   Java 11+ shape from a plain `try`/`catch`, the older one from a `finally`), then folds
   a try-with-resources that is the whole body of another one into that statement's
   resource list (`try (A a = ..; B b = ..)`).
9. `decompiler/nonnative/kuna-jvm/src/p8_structure/inline_single_block_helper.rs` moves a
   block reached only by a labeled `break` to where the break is, which removes most of
   the labels the initial recovery left.
10. `MergeHelper::make_do_while_loops` and `MergeHelper::condense_infinite_loops_with_return`
    finish loop shapes.
11. Outside static initializers,
    `decompiler/nonnative/kuna-jvm/src/p8_structure/exit_helper.rs (ExitHelper::condense_exits)`
    gives an `if` whose branch jumps to a shared `return` or `throw` block a copy of that
    exit as its body, and removes the blocks this leaves unreachable.

When a full pass of the loop changes nothing, the tail runs once, in this order:

- `decompiler/nonnative/kuna-jvm/src/p8_structure/switch_helper.rs (SwitchHelper::simplify_switches)`
  turns the javac lowering of an enum switch (a switch over a synthetic `$SwitchMap$`
  array, read from the `<clinit>` of the class that holds it) back into a switch over the
  enum constants, and the lowering of a string switch (a switch over `hashCode()` followed
  by `equals` tests into a second switch) back into one switch over the strings; the
  helper statements this leaves are marked hidden and are not written. If that produced a
  switch expression, switch expressions, stack
  variables and versions are processed once more.
- `ExitHelper::adjust_return_type` and `ExitHelper::remove_redundant_returns` (a trailing
  `return;` of a `void` method is dropped).
- secondary functions once more, then
  `decompiler/nonnative/kuna-jvm/src/p8_structure/synchronized_helper.rs` folds the
  temporary that holds a `synchronized` block's monitor into `synchronized (expr)`, and
  assigns a monitor that is a non-string constant to a fresh variable first
  (`$VF: Added assignment to ensure synchronized validity`), since `synchronized` on a
  literal is not valid Java.
- variable definitions are placed (section 10.10), compound assignments (`x += y`) are
  formed (`SecondaryFunctionsHelper::update_assignments`), and empty `default` arms are
  hidden.
- `decompiler/nonnative/kuna-jvm/src/p5_types/generics_processor.rs (GenericsProcessor::qualify_chains)`
  qualifies generic call chains, and
  `decompiler/nonnative/kuna-jvm/src/p2_lift/expr_processor.rs (ExprProcessor::canonicalize_casts)`
  collapses a cast of a cast to the same type.
- `LabelHelper::replace_continue_with_break`, and finally
  `ExprProcessor::mark_expr_oddities`, which does not change the tree but adds an error
  comment for every shape that means the result is not trustworthy: an invalid label, a
  leftover `monitorenter`, an `if` destroyed into an expression, a variable whose type
  could not be determined, an `assert` that could not be resugared.

Two more passes run when the class is written, before any text is produced
(`decompiler/nonnative/kuna-jvm/src/p9_emit/class_writer.rs (ClassWriter::invoke_processors)`):
switch simplification once more (`FinalSimplifySwitches`) and the removal of an `else` after a branch that always returns or throws
(`IfHelper::prettify_ifs`).

**javac's block layout (kuna).** The `if` and exit structuring of a method javac compiled (its
class carries no `kotlin.Metadata` and names no `.kt` source file, the one test of
`decompiler/nonnative/kuna-jvm/src/p6_variables/kuna_source_locals.rs (kotlinc_class)` that the
options reading javac's code shapes and the fix `switch_case_order` share) follows the order javac
laid its blocks out in. javac
emits an `if`'s then-branch right after the condition's last jump and the else-branch, or
the code after the `if`, at the jump's target; a `return` or `throw` the source has is an
instruction at its place, a `continue` or `break` a `goto` right after the condition, and a
ternary that is returned one return instruction reached by a `goto`. Vineflower decides
without that order: `IfHelper::reorder_if` turns an `if` whose body ends in an exit, and
the code after it, into an if-else (whose trailing `return;` `remove_redundant_returns` then
drops, and whose emptied branch `identify_secondary_functions` inverts, so `if (s == null)
return; add(s);` prints `if (s != null) { add(s); }`), turns `if (c) continue; rest` into
`if (!c) { rest }`, and keeps the condition of the block it took as the body even when that
block is the jump's target (`if (o == null || ..) return false;` prints `} else if (o !=
null && ..) { .. } else { return false; }`); `SimplifyExprentsHelper::build_iff` merges an
if-else of two returns into one return of a ternary, which secondary functions fold
further (`if (c) return true; return false;` to `return c;`).
`decompiler/nonnative/kuna-jvm/src/p8_structure/kuna_javac_layout.rs (reorder)` makes the
decision of `reorder_if` from the offsets of the blocks' first instructions instead: the
branch that follows the condition (after the binding of a negated `instanceof` pattern,
which belongs to the condition) is the then-branch. When the graph has merged that binding
(`aload; checkcast; astore`, then a `goto` over the then-branch) into the code after the
then-branch, and taken the result for the `if`'s body, the code after the `if` is javac's
then-branch: the `if` is negated, so `if (!(o instanceof T v)) { return ..; } rest` stays a
guard instead of becoming `if (o instanceof T v) { rest } else { return ..; }`. An `if` whose then-branch is first
stays an `if` when that branch ends in a return or throw placed before the code after it,
and becomes an if-else when the branch reaches the code after the sequence over it (a
`goto`; the method's shared last `return`, which the graph merges into the branch, counts
as one); an `if` whose then-branch is the code after it is negated with the two swapped,
and becomes an if-else when that code also ends where the sequence ends. A then-branch
that was a `goto` (`if (c) continue;`, `if (c) break;`, read from the method's bytecode)
keeps the jump: `collapse_else` and `if_else_chain_denesting` leave such an `if` and an
`if` whose body javac placed first (`keeps_jump`, `keeps_denesting_out`), and
`reorder_last` turns `if (!c) { body }` that ends a loop body back into `if (c) continue;
body` once loop recovery has made the jump a `continue` or `break`. `build_iff` merges two
returns only when they are one instruction (`same_return`). Vineflower's choice stands for
the head of an infinite loop that is the loop's condition (the condition
`MergeHelper::match_while` looks for), for an if-else whose else-branch would end a loop body
where `match_for` may still find a loop's update or `match_do_while` its condition (below),
for a positive `instanceof` pattern test that starts an else-branch, or whose else-branch
declares a local in a slot after the binding's (javac gives the bindings of an if-else, and
of an else-if chain, their slots for the whole statement, where code after an `if` reuses
them, so the slots show which form the source had), and for an `if` it has laid out three
times, so that a later pass that changes it back cannot alternate with it without end. Every change is a transformation
`reorder_if` already makes, applied to the other branch, or one it skips: the
statements, edges and conditions stay those of the bytecode.

The option also covers comparisons with a boolean literal. javac compiles `x == false`,
`x != false` and `!x` to one `ifeq`/`ifne` on the value, but a literal on the left (`false
== x`: `iconst_0` first, then `if_icmp..`), a comparison with `true` (`iconst_1`, `if_icmp..`)
and a condition compared with `false` (`(a || b) == false`, the condition's value computed as
0 or 1 and then tested) to code of their own. `SecondaryFunctionsHelper` folds every
comparison of two booleans one of which is a constant into the other operand or its
negation. With the option on,
`decompiler/nonnative/kuna-jvm/src/p3_dataflow/kuna_javac_boolean_compare.rs (keeps)` keeps
the comparison when the constant is the left operand (the comparison came from an
`if_icmp..` with the literal pushed first), when it is `true`, or when the other operand is a
relational comparison, `&&` or `||` (a condition only reaches a comparison as a computed
value); a constant `false` on the right of any other operand is the `ifeq`/`ifne` form and
still folds to `x` or `!x`.

**javac's layout in loop bodies (kuna).** The same option lays out the ifs and exits of loop
bodies and the code around them. Loops are recovered (`MergeHelper::enhance_loops`) before
and inside the main loop of `MethodProcessor::code_to_java`, and an if-else made before a
loop is recovered would swallow its update or its `do`-`while` condition, so an if-else whose
else-branch ends a loop body is made only once the main loop has started
(`kuna_javac_layout::loops_built`), not when the statements after the `if` end with the last
`if` of an infinite loop (the condition `match_do_while` takes), and, when the body is the
`if`'s jump target placed after the statements after it (its branches swapped), only when
those statements hold no instruction javac placed after the body (an exprent moved into the
loop header leaves its instructions in the block, so only the instructions of the exprents
still there count: the update of a `for` loop that the graph merged into the then-branch's
block). A then-branch javac ended with a `continue` or `break` keeps the jump (`jumps_out`):
its last statement continues or leaves a loop, and its last `goto` goes right to that loop's
head or past the loop's code, where javac's jump over an else-branch goes to the if-else's
end inside the loop. javac also sends that jump to the loop's head when the if-else ends the
loop body and no local goes out of scope there (it resolves pending jumps when it closes a
scope, and otherwise chains them into the back edge), so a `goto` to the head counts only
when a local declared in the loop body goes out of scope at the end of the statements after
the `if` (from the LocalVariableTable); the `continue` of a `for` loop goes to its update,
where an if-else ending its body goes too, and stays Vineflower's. Such an `if` is not made an
if-else, and an if-else the graph builder made of it (both branches continue the loop) is
split into the `if` and the statements after it (`splits_else`, `split_else`).
`collapse_if_if`, `collapse_if_else` and `collapse_else` do not join two `if`s that each jump
on their own into one condition: the first one's condition is followed by a `goto` (a
then-branch the source wrote as a jump, `separate_test`), or the first is the head of an
infinite loop, a bare condition, and jumps where the `goto` of the second one's then-branch
goes (javac's loop condition and an `if (..) break;` after it, `separate_jumps`; `if (a || b)
break;` jumps there the same way and is not the loop condition either). `if (c) continue;` is
recognised by its condition jumping over the `goto` right after it even when the code after
it lost its first instructions to a loop header (a `for` loop's init, `jumps_over_goto`). The
body of a loop's first or last `if` stays in the loop when javac placed it before the loop's
last jump back to its start (`inside_loop`; `LoopExtractHelper::extract_first_if` and
`extract_last_if` would move it after the loop, where javac puts the code after a loop), and
the head of an infinite loop is laid out like any `if` when its condition jumps forward inside
the loop and its body stays there (`body_kept_in_loop`: a `do`-`while` body that starts with
an `if`). `MergeHelper::condense_loop` then leaves an infinite loop whose body has more than
its first `if` before the statements it would move out: it makes that `if` the loop's whole
body, which drops the others, and javac's layout keeps `if (c) { ..; continue; }` statements
there that Vineflower would have nested in else-branches. Before the main loop, `place_tails`
moves the last statement of a try body, or of the branch of an if-else javac placed first,
out after the try-catch or if-else when javac placed it at the target of the `goto` that ends
that branch, after all of their code (`DomHelper` makes code only one branch reaches part of
that branch: `try { ..; return x; } catch (..) { throw ..; }` for the source's `try { .. }
catch (..) { throw ..; } return x;`, which the protected range shows; the last catch or
else-branch runs into the code after it, so its end does not show and it is left alone, as
are a bare `return;` and a negated `instanceof` pattern test, whose binding and `goto` belong
to the condition). The edges into the moved statement become breaks out of the try-catch or
if-else, which `InlineSingleBlockHelper` does not inline again. None of these rules applies
inside a pattern `switch` or the loop javac wraps around one (`typeSwitch`, `enumSwitch`),
whose layout `SwitchPatternMatchProcessor` recovers. Code the graph merged into one basic
block with a branch across javac's `goto` (`DeadCodeHelper::merge_basic_blocks`) stays in
that branch. Every decision is bounded the same way: an `if` is laid out at most three times,
`split_else` counts as one, and the other rules only keep a transformation from happening.

**javac's layout of tails, `for` headers and protected exits (kuna).** The same option places five
more shapes where javac placed them. *Code after a loop or switch*:
`InlineSingleBlockHelper::is_inlineable` moves a statement that one `break` reaches to that break;
`decompiler/nonnative/kuna-jvm/src/p8_structure/kuna_javac_layout.rs (keeps_tail)` keeps it after
an infinite loop or a switch when javac placed it after all of their code and reached it by the
break's `goto`, or, for a loop, by the jump of the loop's condition (`exit_jump`: a loop's code
ends after its last jump back to its start or at its condition's exit, whichever is later; code a
switch's last case runs into, or a branch an if-else chain placed after a loop's last jump back,
follows its test without a `goto` and is still inlined). When the graph builder already took such
code into the loop as the body of an `if` of the loop body (in its sequences, or in those of a loop
inside it), `LoopExtractHelper::extract_tail_if` moves the body after the outermost loop whose
code it follows (`tail_if`), the `if` becoming `if (c) break;`, labelled when it leaves an outer
loop. *`for` headers*: `MergeHelper::match_for` makes a `for` of a `while` loop when the statement
before it assigns a variable of the loop and the body ends with an assignment or call; when that
statement is straight-line code, `for_header` decides instead: a local the assignment declares
whose LocalVariableTable scope ends where the loop's code ends was declared in the header
(Vineflower's choice stands), and otherwise the assignment and the last statement are the
header's only when the LineNumberTable gives them the line of the loop's condition, as javac does
for a header's parts and not for a statement before or in a `while` loop. So `X x = a; while (x !=
null) { ..; x = x.next; }` stays a `while` loop, `for (; i < n; i++)` with `i` declared before it
is a `for` without init, and a last statement that the end of an if-else reaches is not taken for
a `for` update when it has a line of its own. *A value returned after a try whose handlers exit, or
after a `synchronized` block*: `DeadCodeHelper::incorporate_value_returns` adds a `load; xreturn`
block to the protected ranges of its predecessors, which puts the `return` into the try; it does
not when javac reached the block by a `goto` from every predecessor (`jumped_to`: the block does
not start where a predecessor ends); a bare `return;` is still taken in. *`if (a || b) continue;`*
(or `break`): javac emits a jump to code that starts with a `goto` as a jump to the `goto`'s
target, so the first test of `a || b` jumps right to the loop; `or_jump` recognises a test whose
jump goes where the `goto` after the last of a chain of tests goes (each test between jumping there
too), `reorder_last` negates it into `if (a) continue;` and `keeps_jump` keeps it, which
`collapse_else` then joins with the next test (an `if` of its own ending a loop body jumps there
too, but the code after its test is not such a chain). *An exit in a try-with-resources, `finally`
or `synchronized` block* (`copied_exit`): an `if` without a body there whose condition jumps to the
statements after it, with javac's then-branch between: code no statement holds any longer (the
copies of the close call, finally block or `monitorexit` the structurer removed), then a return, a
throw or the `goto` of a `continue` or `break` of the loop around, and no other jump into it (the
then-branch of `a || b` is the target of `a`'s jump); `keeps_jump` keeps it, and once
`ExitHelper::integrate_exits` has copied the exit into the `if` (a block without instructions,
`copied_exit_start`) `reorder` reads its position from the exit's expression. Two guards keep
Vineflower steps from printing wrong code on shapes only these rules leave: `integrate_exits` makes
the exit after the statement that follows an `if` without a body (or after the `if`'s own break)
the `if`'s body, which runs it where the `if`-edge went and is right only for the `switch` on
strings it was written for, so it does that only in a switch (`keeps_if_edge`; a test of `return a
|| b && !c` kept as an `if` of its own returned `false` on both paths); and
`LoopExtractHelper::extract_first_if` makes a loop's first `if`, whose body continues the loop, the
loop's exit to the successor of the nearest statement around that has one, which a loop whose tail
was moved out has, so it leaves an `if` whose other path continues the loop (`else_continues`: an
empty `while (cmp(++i, v) < 0) {}` sharing the start of a `for (;;)` became `if (cmp(++i, v) < 0)
return;`; the same guard corrects Vineflower's inverted exit of that shape in an infinite loop
that already had a successor).

## 10.10 Variables, types, declarations and names

**Versions and types.** `decompiler/nonnative/kuna-jvm/src/p6_variables/var_processor.rs
(VarProcessor::set_var_versions)` runs
`decompiler/nonnative/kuna-jvm/src/p6_variables/var_versions_processor.rs
(VarVersionsProcessor::set_var_versions)` each time the schedule asks for it: an SSA split,
a merge of the versions a phi joins, type inference, and a renumbering that gives each
remaining version its own variable index. Type inference is
`decompiler/nonnative/kuna-jvm/src/p5_types/var_type_processor.rs (VarTypeProcessor::calculate_var_types)`:
each variable version gets a lower and an upper bound in the type lattice of
`decompiler/nonnative/kuna-jvm/src/substrate/var_type.rs`, seeded from the method
descriptor and from every expression that defines or uses it, and the bounds are
propagated over the direct graph to a fixed point. Integer constants start at the
narrowest type that can hold them; the constant `1` (not only
`0`) may also become `boolean`, so flags stored with `iconst_1`/`iconst_0` print as
`true`/`false`. Before renumbering, the lattice's internal types become Java types: an
integer-constant type becomes `byte`, `short` or `char`, and the null type `Object`.

**Declarations.** `VarProcessor::set_var_definitions` runs
`decompiler/nonnative/kuna-jvm/src/p6_variables/var_definition_helper.rs
(VarDefinitionHelper::set_var_definitions)`: each variable is declared at the innermost
statement that contains all its uses, folded into its first assignment where that
assignment comes first there; variables of the same slot whose lives do not overlap are
merged into one where the merge cannot change what a read sees; and local types from the
`LocalVariableTypeTable` are applied. `jvm_verify_merges` and `jvm_verify_pre_post_merges`
are experimental, stricter merge checks: the second computes, with an extra SSA pass,
which assignments each read can see, and refuses any merge that would change it.

**Source locals (kuna).**
`decompiler/nonnative/kuna-jvm/src/p6_variables/kuna_source_locals.rs`
keeps the local variables the class file records for the source. javac writes one
`LocalVariableTable` entry per local of the source and none for the temporaries it adds
itself (a `finally`'s or `synchronized` block's saved return value, a for-each's iterator,
array, length and index), and gives a local the next free slot when its declaration runs.
Vineflower folds a variable that is assigned and then read once into the read and declares
each variable at its first use, which loses named locals and renumbers slots. With the
option, a variable the table names at the instruction that reads it (`is_source_local`;
a name holding `$` is a compiler's, as javac before 8 listed `i$`, `arr$` and `len$`) is not
folded: `SimplifyExprentsHelper::is_assignment_return` keeps `x = e; return x;`,
`is_array_initializer` and `add_array_initializer` keep a named array's element stores,
and `decompiler/nonnative/kuna-jvm/src/p8_structure/merge_helper.rs
(MergeHelper::match_for_each)` keeps a named iterable local, and writes a loop whose
iterator or index the table names as the `while`/`for` loop it is. `IfPatternMatchProcessor`
does not fold a local assigned just before an `if` into the subject of the pattern made from
it (`Entry e = find(k); if (e instanceof Group g)`), when the table names it
(`is_pattern_subject_local`); without a table, when its slot comes before the binding's: javac
gives a pattern's binding its slot before the condition runs, so the temporary it stores a
subject expression in (`find(k) instanceof Group g`) has a higher slot, a local of the source a
lower one. In a method without a
table, in a class kotlinc did not compile, a store read back by the very next instruction
counts as a local of the source: javac stores a value it then returns only around a
`finally` or a monitor exit, which sit in between (kotlinc stores its array literals that
way, so its classes rely on the table alone). Then, in Java output only,
`order_declarations` (from `VarDefinitionHelper::set_var_definitions`, before clashing
names are renamed) visits each block's top-level declarations in printed order and moves a
declaration without an initializer before the first earlier one whose variable has a
higher original slot, or splits a declaration with an initializer there into `T v;` and
`v = e;` (not one printed `var`), never past a local class; a variable whose slot is a
parameter's, one of the indices Vineflower gives variables it makes (a `finally`'s
semaphore), or that of a variable still in scope (versions Vineflower split off one slot)
stays where it is. `Exprent::sort_indexed` orders a run of declarations without an
initializer by original slot. javac then gives each local the slot the class file has, so
`int nRead; long total = 0;` keeps its order instead of becoming `long total = 0L;` and a
later `int nRead;`.

**Source locals: scopes and increments (kuna).** The same option, in Java output, also undoes
the narrowing of `VarDefinitionHelper::set_var_definitions`, which declares a variable in the
innermost statement holding all its uses: before `order_declarations` orders each block,
`decompiler/nonnative/kuna-jvm/src/p6_variables/kuna_source_scopes.rs (hoist)` walks the
declarations in printed order with the blocks they are in scope in. javac gives a local the next
free slot where it is declared and frees a block's slots at its end, so two locals in scope at once
have slots in declaration order, and a `catch` parameter takes the first slot after the locals in
scope at its `try`. A variable the LocalVariableTable names (the entries that cover its
instructions agree on a name without `$`), declared in a block inside the scope of a named variable
`w` from an enclosing block (a `for` initializer's included) whose slot is higher, while one of its
entries overlaps one of `w`'s, was declared before `w`: its declaration moves right before `w`'s,
split into `T v;` and `v = e;` when it has an initializer (not one printed `var`; a `for`
initializer becomes `for (v = e; ..)`). For a `catch` parameter whose slot is above a slot no
variable in scope at the `try` holds, the one local the table names at that slot inside the `try`
body is in scope at the `try` (two locals cannot share a slot while both are in scope), and its
declaration moves before the `try`, before any declaration in scope there with a higher slot.
Other declarations of the moved variable's slot and name in its new scope are merged into it; and a
declaration whose slot, name and table entry are those of a variable in scope, or of a parameter, is
merged into that variable (Vineflower splits `it = new Wrapper(it)` or `origin =
origin.getParent()` into a new `var3`). Every merge is refused when a read of one variable could see
a write of the other (`fixes::var_merge::var_merge_liveness::overwrites_live`, both ways), and a variable a
lambda or a local or anonymous class captures, or a `final` one, is neither moved nor merged.
`order_declarations` moves only variables the table names. `VarDefinitionHelper::merge_check`
refuses to merge two variables the table names differently (`kuna_source_locals::named_apart`,
`properties` and a later block's `subFields`). Increments
(`decompiler/nonnative/kuna-jvm/src/p3_dataflow/kuna_source_increments.rs`): javac emits a
statement's code in source order and starts a LineNumberTable entry where a statement starts on a
new line. `SimplifyExprentsHelper::is_ipp_or_imm` does not fold `x = a; a++;` into `x = a++` when the
assignment's own store instruction precedes the increment's code (`store_first`; a stack variable's
assignment, javac's `x = a++`, has no store), and `inline_ppi_and_mmi` and
`PPandMMHelper::inline_pp_iand_mmi_if` do not fold `++x;` into the next statement or `if` condition
when all of the increment's code comes before the next statement's and the table puts the two on
different lines (`own_statement`). Classes kotlinc compiled are left alone.

**Source patterns (kuna).**
`IfPatternMatchProcessor::check_branch` makes no pattern from an
`instanceof` and the cast stored after it when the class file's `LineNumberTable` shows the cast
as a statement of its own
(`decompiler/nonnative/kuna-jvm/src/p8_structure/kuna_source_patterns.rs (keeps_cast)`), in a
class kotlinc did not compile. For a local subject javac compiles `if (x instanceof T v) S` and
`if (x instanceof T) { T v = (T) x; S }` to the same instructions; it starts a line table entry
at each statement, so the cast's `aload` begins a line, and the statement after the cast another,
while a pattern's binding belongs to the condition's line. The cast stays when the table has an
entry at the binding's first instruction, for a line after the one in effect at the
`instanceof`, and another at the instruction after the binding's store. A pattern whose
multi-line condition starts its last line with the binding, the body on the next line, reads
the same and is printed as the test and the cast (the same code). javac's own temporary subject,
whose slot comes after the binding's, shows a pattern and keeps it. Otherwise Vineflower's
pattern stays. In an else-if chain the source's casts free their slot at the end of each
branch, where a chain of patterns keeps every binding's slot to the end, so the recompiled
chain takes the class file's slots.

**Names.** After each method is processed
(`decompiler/nonnative/kuna-jvm/src/p1_classfile/class_wrapper.rs (ClassWrapper::init)`),
variables get `varN`-style names that avoid the class's field names
(`decompiler/nonnative/kuna-jvm/src/p6_variables/var_names_collector.rs`), then, with
`jvm_use_lvt_names`, the debug names from the `LocalVariableTable` replace them wherever
the name is a valid identifier (`VarProcessor::set_debug_var_names`). With the same
option, a `MethodParameters` attribute names (and marks `final`) the parameters. A method without code still gets parameter names. Names that clash within a
scope - two variables, a variable and a captured variable, a variable and a field it
shadows - are renamed apart (`VarDefinitionHelper::remap_clashing_names`), once at
processing time and again at writing time (`VarProcessor::rerun_clashing`), when nested
classes and lambdas have been processed. A parameter or lambda argument whose type stays
undetermined prints as `Object`.

## 10.11 Class-level processing

Processing a root class (`ClassesProcessor::process_class`) runs, in order:

1. **Lambdas.** `decompiler/nonnative/kuna-jvm/src/p4_calls/lambda_processor.rs
   (LambdaProcessor::process_class)` reads the `BootstrapMethods` of every class of the
   tree (Java 8+) and gives each `invokedynamic` to `LambdaMetafactory.metafactory` or
   `altMetafactory` a lambda class node, which is written later as a lambda or a method
   reference.
2. **Methods.** Every class of the tree gets a wrapper whose `init` decompiles each of its
   methods with the schedule of sections 10.4-10.10. Synthetic classes that hold only enum
   switch maps are initialized first, so the switches of their users can be read back.
3. **Nested classes.** `decompiler/nonnative/kuna-jvm/src/p8_structure/nested_class_processor.rs
   (NestedClassProcessor::process_class)` finds, for each anonymous class, local class
   and lambda, the enclosing variables it captures (the `val$x` fields and synthetic
   constructor parameters javac adds), rewrites their uses into uses of the enclosing
   method's variables, and places each local class's definition in the statement where it
   is first used. `jvm_synthetic_not_set` recognizes compiler-generated members (outer-instance
   and captured-variable fields, accessors, `class$` caches) by their shape even without
   the `ACC_SYNTHETIC` flag, which some compilers and obfuscators omit.
4. **Synthetic accessors.** `decompiler/nonnative/kuna-jvm/src/p4_calls/nested_member_access.rs
   (NestedMemberAccess::propagate_member_access)` replaces calls to the `access$NNN`
   methods javac generates for private members of nested classes with the field access or
   call they stand for.

Writing a root class (`ClassesProcessor::write_class`) first runs, for each class of the
tree, the late method passes of section 10.9 and then the class-level rewrites of
`ClassWriter::invoke_processors`:
`decompiler/nonnative/kuna-jvm/src/p8_structure/initializer_processor.rs` moves field
initializers out of `<clinit>` and the constructors into the field declarations and drops
a constructor's leading argument-less `super()`;
`decompiler/nonnative/kuna-jvm/src/p4_calls/class_reference14_processor.rs` turns the
Java 1.4 class-literal cache into `X.class`;
`decompiler/nonnative/kuna-jvm/src/p8_structure/enum_processor.rs` removes `$VALUES`,
`values()`, `valueOf()` and the hidden name and ordinal constructor parameters; and
`decompiler/nonnative/kuna-jvm/src/p8_structure/record_helper.rs` restores a record's
canonical constructor and hides the accessors, `equals`, `hashCode` and `toString` that
match the generated ones. A failure here writes the class as `$VF: Couldn't be decompiled`
(section 10.12).

Two class-level facts come from outside the class. **Varargs (kuna).**
`decompiler/nonnative/kuna-jvm/src/substrate/invocation_exprent.rs` needs to know
whether a called method is varargs, to print `f(a, b)` instead of
`f(new Object[]{a, b})`. It asks the class universe first; for a class the universe lacks,
Vineflower uses reflection on the JVM it runs on, which kuna cannot.
`decompiler/nonnative/kuna-jvm/src/p4_calls/classpath_helper.rs (ClasspathHelper)`
answers from an embedded table, `classpath_helper_jdk.txt` in the same folder: every
varargs method of OpenJDK 21 such a lookup can find, so the answer is the same whether or
not a JDK runtime is loaded. **Dynamic constants.**
`decompiler/nonnative/kuna-jvm/src/p4_calls/condy_helper.rs` writes the
`ConstantBootstraps` constants `nullConstant`, `primitiveClass`, `enumConstant` and
`getStaticFinal` as source expressions; a `VarHandle` constant
keeps its raw constant-dynamic form (Vineflower's `decompile-complex-constant-dynamic`
lookup expression runs eagerly, which the constant does not, and is not ported).

## 10.12 Emission

`decompiler/nonnative/kuna-jvm/src/p9_emit/class_writer.rs (ClassWriter::write_class)`
writes a root class and everything nested in it into a text buffer, and
`ClassesProcessor::write_class` puts the header in front: the package and
the imports `decompiler/nonnative/kuna-jvm/src/p9_emit/import_collector.rs (ImportCollector)`
collected while the body was written. The collector decides, per class name, whether
the short name can be used in this file without ambiguity, and lists the imports that
requires. A `package-info` class and a
`module-info` class have writers of their own.

**What is written.** For each class: annotations, modifiers, type parameters, `extends`,
`implements` and `permits`, and for a record its components; then the enum constants,
the other fields with their initializers, the methods and the nested classes. Members are
left out by policy: synthetic members (`jvm_remove_synthetic`), bridge methods
(`jvm_remove_bridge`), a lone default constructor, and
whatever the class-level rewrites hid. kuna adds `@Override` where an
instance method of a Java 5+ class file overrides or implements a method of a supertype
the engine can see;
`jvm_mark_corresponding_synthetics` names the synthetic method behind each lambda and the
class behind each anonymous class in a comment; `jvm_decompiler_comments` controls every `$VF:` note
the passes above attached to a method. Lambdas are written inline at their use, without
braces when the body is one expression. A generic method call carries explicit type
arguments only where inference needs them. Constants print as source
does: `Double.NaN`, `Long.MAX_VALUE` and `Math.PI` rather than their values unless
`jvm_keep_literals` is on, and non-ASCII characters escaped only under
`jvm_ascii_strings` (`decompiler/nonnative/kuna-jvm/src/substrate/const_exprent.rs`).
Numbers print with the JDK's own algorithms (section 10.13).

**Failed methods.** A method whose schedule threw is written as its signature and a body
of comments (`ClassWriter::dump_error`): `$VF: Couldn't be decompiled`, then the
`error-message` note (a request to report the class to kuna's tracker; section 10.16), the
exception text and its throw location, and a listing of
the method's bytecode. The class around it is written normally. (Vineflower's
`dump-exception-on-error`, which the corpus fixture turns off, drops the note and the
exception.)

**Layout.** The buffer is a tree of line groups; after a class is written,
`decompiler/nonnative/kuna-jvm/src/p9_emit/text_buffer.rs (TextBuffer::reformat)` breaks
argument lists, chained calls and long operator chains at their recorded break points
wherever a line exceeds 160 columns, and indents with three spaces (Vineflower's
`preferred-line-length` and `indent-string`, fixed at their defaults). Offsets and widths are counted in UTF-16
units, as Java counts them. Every reference and declaration in the text is recorded as a text token
(Vineflower's hook for IDE consumers; kuna reads none), and kuna's zero-width span marks
(`kuna_spans`) ride the same mechanism, following every later edit.

**Line mapping.** While expressions are written,
`decompiler/nonnative/kuna-jvm/src/p9_emit/bytecode_mapping_tracer.rs` records which
bytecode offsets each output line came from, and
`decompiler/nonnative/kuna-jvm/src/p9_emit/bytecode_source_mapper.rs` relates them to the
class file's `LineNumberTable`. (kuna) `decompiler/nonnative/kuna-jvm/src/infra/kuna_spans.rs`
records, with zero-width marks that ride through re-wrapping and are never printed, where
each class and each method starts and ends in the final text, and groups the tracer's
offsets by method and line. These spans are what lets a front end slice one method out of
a class, report its line mapping in bytecode offsets, and mark a method as failed, without
changing a byte of the output. The recorder also keeps the stack of spans open while the
writers recurse. When a body printed inside another one is written as a failure comment (a
lambda, a method of an anonymous or local class, such a class itself), the writers report
it (`method_failed` from each writer's `dump_error` and the inline-lambda and Kotlin-lambda
error paths, `class_failed` from `invoke_processors` and the Kotlin writer's failed-lambda
path), and every open method span records it as a nested failure, as does the innermost
open class span when no method of it is open (a field initializer); the failing body's own
span keeps it as its own error. A closed span also closes those a throw left open inside
it. A method span further carries the method's `$VF:` comments when its wrapper's
`add_error_comment` is set, the comments Vineflower follows with its report-this note.

**Failure granularity.** A method failure becomes the comment block above and the class
survives. A class whose processing throws (section 10.11) is written as
`// $VF: Couldn't be decompiled` followed by the exception, in place of its body. A class
whose writing throws is written as a `/* $VF: Unable to decompile class ... */` comment
(not at all under the corpus fixture's `dump-exception-on-error=0`). Nothing
else stops the run: the other classes of the input are still written. A Java
`RuntimeException` and an ordinary Rust panic (an index out of bounds, an `unwrap` on
`None`) are caught at these boundaries alike (section 10.13), and so is recursion deeper
than the engine thread's guarded stack, as a `StackOverflowError` (section 10.13), which
fails the method or class it unwinds to as Vineflower's JVM fails it; an allocation
failure is not caught, and ends the process.

**Time limit (kuna).** Vineflower enforces `max-time-per-method` by running each method on a
watchdog thread and killing it. kuna instead arms a per-thread budget
(`decompiler/nonnative/kuna-jvm/src/infra/cancelation_manager.rs (CancelationManager)`)
that the schedule polls at the heads of its fixed-point loops - the `finally` loop, the
stack-variable loop, the main and merge loops, the SSA fixed point, statement recovery and
irreducible-node splitting. The budget is a count of polls, `ClassWrapper::POLLS_PER_SECOND`
(200,000, the rate measured on the slowest methods, Ghidra's ANTLR-generated parsers) per
second of the limit, so whether a method runs out depends only on its input and options,
not on the machine or how loaded it is; Vineflower's wall-clock limit, and kuna's before
it, stopped more methods on a busy machine than on an idle one. A wall-clock deadline at
twice the limit backs the count up for a method whose polls are rare. A method that runs
out fails like a killed one: an error log and a `$VF: Couldn't be decompiled` body whose
exception is a `TimeoutException` (`decompiler/nonnative/kuna-jvm/src/p1_classfile/class_wrapper.rs
(ClassWrapper::code_to_java_on_watchdog)`). A method stuck between two polls is not
interrupted.

## 10.13 The runtime kernel (kuna)

Vineflower is written against Java's object model, and the port keeps its control flow
line for line, so `decompiler/nonnative/kuna-jvm/src/rt` gives each Java facility the
port depends on one faithful Rust shape:

- **Objects with identity** (`decompiler/nonnative/kuna-jvm/src/rt/arena.rs`). Every
  ported class with identity or shared mutable state lives in a thread-local arena and is
  referred to by a `Copy` handle, so `==` on handles is Java's reference `==`. Class-scope
  arenas (statements, expressions, edges, graphs, wrappers) are emptied when a decompile
  ends; session-scope arenas (the class model, the class forest) live as long as the
  engine instance. Hash collections keyed by such objects order them by creation index,
  where Java would use identity hashes.
- **Reclaiming garbage** (`decompiler/nonnative/kuna-jvm/src/infra/kuna_reclaim.rs
  (ReclaimWindow)`). Java's collector frees what the passes discard (replaced
  expressions, copies, the trees each `finally` re-parse throws away); the arenas would
  hold all of it until the decompile ends. A window opens when a method's schedule starts;
  when the method returns or throws, and at points inside its loops where no pass is in
  flight once the arenas have grown enough, every object allocated since the window opened
  that is not reachable from the handles the schedule still holds is released. When a root
  class has been processed or written, the decompile records, variable-definition helpers
  and mapping tracers it allocated are released too. A released slot keeps its index, so
  creation order, and with it every hash order above, is unchanged, and a wrongly
  released object fails on its next use instead of being reused.
- **Hash collections in JDK order** (`decompiler/nonnative/kuna-jvm/src/rt/jhash.rs`).
  Vineflower iterates `HashMap` and `HashSet` in places where the order reaches the
  output, so the port reproduces JDK 21's table: hash spreading, power-of-two buckets,
  tail insertion, order-preserving splits on resize, and capacity kept across `clear`.
  Plain `std` hash maps are banned in the workspace for the same reason.
- **Shared collections** (`decompiler/nonnative/kuna-jvm/src/rt/jlist.rs`): a list or map
  that several holders read and mutate, as a Java reference would be.
- **Strings** (`decompiler/nonnative/kuna-jvm/src/rt/jstring.rs`). A Java string is UTF-16
  and may hold unpaired surrogates; they are kept in private-use scalars and mapped back
  for every UTF-16 view, so lengths, indices and escapes are the JVM's.
- **Characters and numbers** (`decompiler/nonnative/kuna-jvm/src/rt/jchar.rs`,
  `decompiler/nonnative/kuna-jvm/src/rt/jnum.rs`): `Character` classification from JDK
  21's tables, and `Double.toString`/`Float.toString` as JDK 19+ print them.
- **Exceptions** (`decompiler/nonnative/kuna-jvm/src/rt/exception.rs`). A Java `throw`
  unwinds as a panic carrying the exception, and `try_catch` is the
  `catch (Throwable)` that receives it at the method and class boundaries of section 10.12.
- **Stack overflow** (`decompiler/nonnative/kuna-jvm/src/rt/stack.rs`). The port recurses
  where Vineflower does, and a JVM turns a recursion that outgrows its stack into a
  `StackOverflowError` that those boundaries catch; a Rust stack overflow aborts the
  process and cannot be caught. So every engine thread declares its stack when it starts
  (the driver's workers, `with_engine_stack`), and a guard on every arena access and at the
  expression and statement writers compares the stack pointer with a limit:
  `rt::stack::DEPTH_LIMIT` (32 MiB) below the thread's start, or all but an eighth of a
  smaller stack. Past it the guard throws `StackOverflowError`, which unwinds like any
  other exception (never while the thread is already unwinding). Every recursion in the
  engine touches the arenas at each level, so none can pass the limit unseen by more than
  a few frames. The deepest real input measured used 150 KiB; the limit only stops
  adversarial nesting (expressions thousands deep, or lambdas whose bodies create each
  other, which Vineflower nests without end). The guard costs nothing
  measurable (Guava, single instance, paired runs).

## 10.14 Front-end support (kuna)

**Inventory.** `decompiler/nonnative/kuna-jvm/src/p0_knowledge/kuna_inventory.rs` lists an
input's classes and methods (names, nesting, access flags, source file, sizes, bytecode
length per method) from the class files alone, without decompiling.

**Parallel instances.** `decompiler/nonnative/kuna-jvm/src/infra/kuna_driver.rs (run)`
decompiles one input on several threads with output identical to a single instance's.
Every worker loads its engine as it starts; the first also plans: it lists the root
classes in the order a single run writes them. The single run's only cross-class dependency that changes output is a class
reading another root class's processing results while it is written (most visibly the
constructor masks of that root's nested classes, read when `new Outer.Inner(..)` or
`super(..)` is written), so `decompiler/nonnative/kuna-jvm/src/infra/kuna_partition.rs`
reads from the class files whose nested-class constructors each root calls. Roots
are grouped into batches that also *process* (without writing) the roots they depend on.
A root is first joined with the roots it depends on, costliest first, while the joined
unit stays within the first batch's size, so that few batches process a root another
batch writes; units are packed in root order into batches that shrink as the grouping goes
(each about the cost not yet grouped over twice the worker count, down to the total over
eight times the worker count, and closed before a unit would overfill it), so the last
batches are short. Worker threads take batches from a shared queue,
largest first, preferring among the next few one their engine can run without reloading
(it processes no root this engine already processed), and each runs its batch with
`Fernflower::decompile_selected`, in the single run's order. A detector watches every
read of another root's state during the run; a read whose answer would differ from the
single run's (*a divergent read*) makes the driver repeat that batch with the missing root
added, so results are delivered only once clean.
(kuna) A selected run writes only the roots it delivers, so a root that the single run had
already written when the read was made is added to the repeat's roots to write without
delivering (`decompiler/nonnative/kuna-jvm/src/infra/kuna_partition.rs (owner_written_in_full_run)`);
otherwise the repeat read the same unwritten state again and the batch ended in the last resort,
every root processed, which the detector does not watch and which still wrote only the delivered
roots (kotlin-stdlib's `SequencesKt___SequencesKt`, whose `sortedBy` creates the `compareBy`
object of `ComparisonsKt__ComparisonsKt`, written before it in the single run).
The last resort now processes and writes every root: the single run.
(kuna) An engine renames once, as a Vineflower instance does, however many batches it runs, and
is kept only while its batches succeed: an engine that fails to load or whose batch unwinds is
dropped and the next batch starts a fresh one, and a worker that unwinds outside the engine's
catch reports the batch it held as failed, so the caller does not wait for it
(`decompiler/nonnative/kuna-jvm/src/infra/kuna_driver.rs (InFlight)`). With renaming, the plan
reads class files under their new names (`decompiler/nonnative/kuna-jvm/src/p0_knowledge/fernflower.rs
(renamed_class)`), and a selection may name a root by either name. Writing a Kotlin class marks
members it reads again (its companion's overrides and forwarders, the `INSTANCE` and companion
fields); a single run processes every class before writing any, so an engine kept for another batch
takes those marks back when its batch ends (`decompiler/nonnative/kuna-jvm/src/kotlin/kt_attrs.rs
(undo_marks)`): otherwise `r.size` on a class whose companion overrides `size` could come out
`r.getSize()` once that class's batch had run on the same engine. `ClassWrapper::init` pairs
the logger's `start_method` with its `end_method` also when the method throws before its body
(a malformed descriptor), which Vineflower leaves unpaired; the CLI's stall watch reads the pairs.
The detector exempts `getSyntheticParametersMask` reading a root class's own node except when
the root does not declare the constructor asked for: that call throws while the root is
processed and answers no mask otherwise (the Kotlin plugin asks for a superclass's
default-arguments constructor without its marker parameter).
Results reach the caller in root order,
whatever the number of workers, and the call returns after the last delivery while the
workers drop their engines. Each engine instance runs on a thread with a 512 MiB
stack, of which the recursion guard (section 10.13) lets it use 32 MiB. The driver reads
Vineflower's three-letter option names as their long names (`mpm` is
`max-time-per-method`) before setting the keys it owns.

**Options.** `decompiler/nonnative/kuna-jvm/src/p0_knowledge/kuna_options.rs (resolve)`
turns a `jvm_*` name, or Vineflower's long or three-letter name, and a value as a user
writes it into the Vineflower key and value; the table is generated from `options.toml` by
the crate's build script. Keys the front end sets itself (thread count, time limit, method
selection, logging, line separator, line mapping, class-path keys a JVM would serve), and
the formatting and failure-comment keys kuna fixes at Vineflower's defaults, are listed
there as `[[driver_owned]]` and refused as user options. kuna's own rows resolve to
their own names; `Fernflower::new` takes `jvm_plugins` (no row) off the property list before the rest becomes Vineflower's option map (section
10.16), so that map is the one Vineflower would build. Any other key, a `jvm_fix_*` name included, is not a
switch: `resolve` refuses it as an unknown option, and given to the engine directly it
stays in Vineflower's option map like any key Vineflower does not read.

**The CLI seam.** `kuna-cli` routes an input to this engine before any native code runs
when the input operand is a class file, an archive or a directory (chapter 00, section
0.2). The CLI meets the engine through a contract of its own crate, `kuna-managed` (this
chapter's fourth anchor), which any managed-bytecode engine implements: an `Engine` sniffs,
classifies and opens its inputs and owns its option rows, and an opened `Program` lists
its types and members, resolves its selection grammar to a target and runs a selection,
delivering each root's unit (text, path, language, and the spans of the types and members
written into it) as the driver finishes it. The JVM side is
`decompiler/nonnative/kuna-jvm/src/infra/kuna_engine.rs`: it turns the resolved `--option`
pairs into the driver configuration (the JDK of `jvm_include_runtime` included), installs a
silent engine logger that marks each method's start and end on the CLI's stall watch and
arms its memory guard around the method, and converts each delivered class result into a
unit. The CLI's non-native module (this chapter's third anchor) is the same for every
engine: it slices types and members out of the delivered text by their spans and renders
the native CLI's text and JSON shapes, tagging each unit with its language (`java`, or
`kotlin` for a unit the Kotlin plugin wrote, whose file name ends in `.kt`). Before a
command that decompiles (not `functions`) it sets glibc's heap policy for the process
(`decompiler/crates/kuna-cli/src/nonnative/heap.rs (tune)`, Linux with glibc only): a 256
MiB top pad, no trimming, and blocks up to 32 MiB served from the heaps instead of fresh
mappings. The engine threads allocate and free many small objects, and under the default
policy each heap's growth and trimming are memory-map calls under one process-wide lock
that serialize the threads; the native path never sets it. For the same reason it arms the
per-method memory limit (chapter 00, `kuna_fnmemory`) without the heap release the native
drivers make before each function
(`decompiler/crates/kuna-cli/src/nonnative/run.rs (arm_memory_limit)`). The CLI surface is
`docs/cli.md` ("JVM inputs").

## 10.15 Kotlin

Vineflower's Kotlin plugin, ported in `decompiler/nonnative/kuna-jvm/src/kotlin`, is the
one bundled language plugin. A class it claims is decompiled by a method pipeline and
written by a writer of its own, as Kotlin source in a `.kt` unit. Everything before the
point where it takes over is the Java engine's: loading and nesting (section 10.2), the
class-file model (10.3), the control-flow graph up to the exception-range repairs (10.4),
lambda discovery and the nested-class and accessor processing of a root class (10.11).

**Which classes are Kotlin.** `decompiler/nonnative/kuna-jvm/src/kotlin/kotlin_chooser.rs
(KotlinChooser::is_language)` claims a class when `jvm_kt_enable` is on (the default) and
the class carries a `kotlin/Metadata` annotation, visible or invisible at run time: the
annotation the Kotlin compiler puts on every class it emits. Nothing else counts, so a Java
class is never written as Kotlin and a Kotlin class whose annotation was stripped is
written as Java. The plugin context asks the chooser of every loaded language plugin in
turn (`decompiler/nonnative/kuna-jvm/src/infra/plugin_context.rs
(PluginContext::get_language_spec)`), and the first that claims a class supplies its
language spec (`decompiler/nonnative/kuna-jvm/src/infra/language_spec.rs`): a CFG pass, a
statement parser, a main pass, a root-class processor, a writer and a file extension. The
question is asked for the root class at four places - when it is processed
(`decompiler/nonnative/kuna-jvm/src/p1_classfile/classes_processor.rs
(ClassesProcessor::process_class)`), which hands the spec to the wrapper of every class
nested in it; when it is written (`ClassesProcessor::write_class`), which picks the writer;
and when its unit's file name is formed (`decompiler/nonnative/kuna-jvm/src/p0_knowledge/fernflower.rs`),
which takes the spec's extension, `kt`. A nested class therefore follows its root. (The
chooser is asked once more while statements are written, to decide whether they end in a
semicolon.) With `jvm_kt_enable`
off, a Kotlin class takes the Java pipeline and is written as Java, the `@Metadata`
annotation printed like any other annotation, and its metadata is not parsed.
With no plugin loaded (`jvm_plugins=0`, the corpus fixture) every class is Java.

**Metadata.** Asking the chooser also parses the class's metadata, once
(`KotlinChooser::parse_metadata_for`). The annotation's `k` (the kind), `d1` (the data)
and `d2` (the strings) are decoded by `decompiler/nonnative/kuna-jvm/src/kotlin/kt_metadata.rs`:
`d1` is protobuf bytes spread over Java strings, either one byte per character (after a
leading NUL) or, by default, seven bits per character
(`decompiler/nonnative/kuna-jvm/src/kotlin/bit_encoding.rs`). Its first message is a string
table whose records `decompiler/nonnative/kuna-jvm/src/kotlin/metadata_name_resolver.rs`
expands into the names the rest refers to: each record a `d2` string or one of a fixed list
of predefined names, optionally cut to a substring or with one character replaced, `$`
read as `.` (including Vineflower's slip of testing the substring list but reading the
replacement list, kept for fidelity). The second message is the one `k` names: a class
(1); a file of top-level declarations, compiled to a `...Kt` class (2); a synthetic class,
such as a lambda compiled to a class, described by one function (3); a multifile facade,
whose `d1` lists its part classes (4); or a multifile part, a file whose facade is named by
the annotation's `xs` (5). The messages follow the Kotlin compiler's metadata schema, which
the plugin ships as a shaded protobuf-lite build; `decompiler/nonnative/kuna-jvm/src/kotlin/kt_proto.rs`
ports that runtime and schema as one schema-driven reader with the same semantics: unknown
fields and enum numbers are skipped, repeated fields accept packed and unpacked encodings,
a scalar given twice keeps the last value, a sub-message given twice is merged, and a
missing required field fails the parse. The flag words (visibility, modality, class kind, `data`,
`value`, `inline`, `suspend`, `operator`, `infix`, `const`, `lateinit`, accessor flags,
parameter defaults) are unpacked by `decompiler/nonnative/kuna-jvm/src/kotlin/kt_flags.rs`.
A parse that throws is logged as a warning and leaves the class with what was attached
before the failure; a class without a usable `k` or `d1` gets no model, and its writer
says `// $VF: Class flags could not be determined`.

The decoded message becomes the plugin's model of the class
(`decompiler/nonnative/kuna-jvm/src/kotlin/k_element.rs`): properties with their getters,
setters, backing fields and delegates (`k_property.rs`), functions with receivers, type
parameters, parameters, return types and contracts (`k_function.rs`), constructors, one
marked primary (`k_constructor.rs`), and types carrying nullability, type arguments and
variance (`k_type.rs`), all in the same folder. Each element is attached to the
class-file members it describes, found through the JVM names and signatures the metadata
records (or, for a function, built from its Kotlin signature). Members that exist only
for the compiler get a hidden marker instead: the `name$default` stub of a function with
default arguments, a companion object's members mirrored as static methods on the outer
class, a property's `$annotations` holder method and a delegated property's delegate
field, and, when the class is written, the `Companion` and `INSTANCE` fields. Vineflower stores all of these in the
members' attribute maps; (kuna) the port keeps them in a session-scope side table keyed by
member (`decompiler/nonnative/kuna-jvm/src/kotlin/kt_attrs.rs`), which the plugin clears
when it closes. Parsing one class can mark members of another (a companion's outer class),
so the order of parsing matters, and it is Vineflower's: before any class is processed,
`decompiler/nonnative/kuna-jvm/src/p0_knowledge/context_unit.rs` forms every class's file
name, which parses the metadata of every class in entry order.
Writing a class with a companion object reads the companion again with the class as its parent
and marks hidden every member of the class that has the JVM signature of a companion member,
in place of the element the class's own metadata gave it (`kotlin.random.Random`'s
`nextInt(until: Int)`, which its companion `Default` overrides). (kuna) The elements the class
declared are kept first (`decompiler/nonnative/kuna-jvm/src/kotlin/kuna_declared_element.rs`),
and the fixes that ask which members another class declares (`kt_extension_call`) read those, so
their answer does not depend on whether that class was written first: in the input's order or,
in the driver, on which batch an engine ran before (kotlin-stdlib's `RangesKt___RangesKt` wrote
`random.nextInt(this)` or `RandomKt.nextInt(random, this)` by the worker count). With the fix
`kt_companion_static_copy` only a static method is marked, and the class is written with its own members.

**The pass schedule.** For a Kotlin class,
`decompiler/nonnative/kuna-jvm/src/infra/method_processor.rs (MethodProcessor::code_to_java)`
builds the graph, removes dead blocks, inlines `jsr`, connects the exit and removes gotos
as for Java, then
hands the method to the spec's passes, declared in
`decompiler/nonnative/kuna-jvm/src/kotlin/kotlin_plugin.rs`, and returns; none of the Java
schedule after that point runs, nor any plugin pass registered at the Java pass locations
(IdeaNotNull's, for one). The CFG pass is the exception-range repairs of section 10.4 in
the same order, step 8 as `decompiler/nonnative/kuna-jvm/src/kotlin/obfuscated_exceptions_pass.rs`.
The statement tree comes from the same dominator-based recovery (10.5), and the
main pass is the Java schedule of sections 10.6-10.10 with these differences: the
`finally` loop, the initial stack-variable loop and the loop-merge step are plugin passes
(`java_finally_pass.rs`, `stack_var_initial_pass.rs`, `k_merge_pass.rs`, which recognizes
`while`, `for` and for-each loops but not `do`-`while`); secondary functions always fold
ternaries with a boolean constant branch; each round of the main loop also removes
assignments of `null`, zero or `Unit` to a variable that no read can see, found on the SSAU
graph (`eliminate_dead_vars_pass.rs`); the last stack-variable pass inlines ordinary local
variables as well as stack variables; and the Java main loop's pattern-matching,
switch-expression, try-with-resources and intersection-cast passes, and its tail's
switch-map simplification, `synchronized`-monitor folding, generic-chain qualification,
cast canonicalization and oddity comments, are not part of it. After variable definitions
are placed, five passes turn the tree into Kotlin. `replace_stats_pass.rs` and
`replace_exprents_pass.rs` (in `decompiler/nonnative/kuna-jvm/src/kotlin`) replace every
sequence, loop, switch, `if` and `try` statement and every expression by its Kotlin
counterpart, and mark each variable declaration, catch variable and loop variable as a
Kotlin declaration. `resugar_kotlin_methods_pass.rs` matches calls of the Kotlin runtime:
the `Intrinsics.checkNotNull...` checks the compiler inserts for non-null parameters and
values are removed, `Intrinsics.areEqual(a, b)` becomes `a == b`, `x != null ? x : y`
(with `x` a variable) becomes `x ?: y`, and `Reflection.getOrCreateKotlinClass(C.class)`
becomes `C::class`. `collapse_string_concat_pass.rs`
flattens nested string concatenations into one string template (`"count=${c.count}"`), and
the last pass replaces `continue` with `break` where Java's tail does.

**The Kotlin expression and statement layer.** Vineflower writes Kotlin syntax by
subclassing the Java expression and statement classes: `KVarExprent` prints `val`/`var`
declarations with their type (`k_var_exprent.rs`), `KFunctionExprent` the Kotlin
operators (`is`/`!is`, `as`, `===`, `?:`, `::class`, ranges and string templates,
`k_function_exprent.rs`), `KInvocationExprent` property syntax for getter and setter
calls and the `infix` and operator call forms (`k_invocation_exprent.rs`), `KNewExprent`
lambdas (`{ x: Int -> ... }`), `object : T { ... }` expressions and array constructors
(`k_new_exprent.rs`), `KExitExprent` labelled returns (`return@label`, `k_exit_exprent.rs`),
and the statement subclasses `if` as statement or expression, `while`/`do`/`for (x in
xs)` loops, `when`, `try`/`catch` and labelled blocks (`k_if_statement.rs`,
`k_do_statement.rs`, `k_switch_statement.rs`, `k_catch_statement.rs`,
`k_sequence_statement.rs`), all in `decompiler/nonnative/kuna-jvm/src/kotlin`. JVM types
are written as Kotlin types (`int` as `Int`, `java.util.List` as `List` or `MutableList`
by the metadata, nullable types with `?`; `k_types.rs`). (kuna) A Kotlin expression or
statement in the port is its Java object with a Kotlin flag and the subclass's extra data,
and every dispatcher of a method a Kotlin subclass overrides checks the flag first
(`decompiler/nonnative/kuna-jvm/src/kotlin/k_dispatch.rs`): the object stays an instance of
its Java kind for every core helper that still runs on it, which is what subclassing gives
Vineflower.

**The Kotlin writer.** `decompiler/nonnative/kuna-jvm/src/kotlin/kotlin_writer.rs
(KotlinWriter::write_class)` writes a Kotlin root class and everything nested in it. The
unit header has the file annotations (`@file:JvmName` and `@file:JvmMultifileClass` for a
multifile part), the package and imports without semicolons, and the class's type aliases;
the import collector (`kotlin_import_collector.rs`) leaves out the packages Kotlin imports
by default (`kotlin`, `kotlin.collections`, `kotlin.io`, `kotlin.text` and the other
`kotlin.*` defaults), and a Java type that a Kotlin type of the same simple name would
shadow (`java.lang.String`, `java.util.List`) is written qualified. Before writing, the
class-level rewrites run: field initializers move into their declarations, Java 1.4 class
literals are restored, an enum's synthetic members are hidden (`k_enum_processor.rs`), and
the instance fields of `object`s and companion objects are found
(`kotlin_object_processors.rs`). What a class is written as comes from its metadata, not
from its modifiers: the declaration's keyword and modifiers (`class`, `interface`, `enum
class`, `annotation class`, `object`, `companion object`, `data`, `value`, `sealed`,
`inner`, `fun interface`, visibility, modality, `expect`/`external`) from the class flags,
the primary constructor's parameters in the header, the supertypes; a file class (kind 2
or 5) has no declaration, only its top-level members; a lambda class (kind 3) is written
as a lambda where it is created; a multifile facade (kind 4) is written as a comment listing
its parts. The members
are walked in class-file order, enum constants first, then fields, then methods, then
member classes, but each is written as the metadata element it belongs to: a field or an
accessor as its property, once, at the first of them (`val` or `var`, type, initializer,
`const`, `lateinit`, delegate, and the accessors' own visibility and bodies when they are
not the default ones); a method as its function, with `fun` modifiers (`operator`,
`infix`, `inline`, `tailrec`, `suspend`, `override`), receiver, parameters with their
default values and return type; a constructor as a secondary `constructor(..) : this(..)`
or, for the primary one, the `init` block of its body. Hidden members and the bodies of
lambdas compiled to methods (`name$lambda$N`, written inline where they are used) are
skipped, as are synthetic and bridge members under `jvm_remove_synthetic` and
`jvm_remove_bridge`. A member the metadata does not describe is written from its JVM
signature as a plain `fun`, and a warning is logged. Every declaration carries its
visibility, `public` included, which Kotlin source leaves implicit (Vineflower's
`kt-show-public` default, fixed). A parameter default is recovered from
the `$default` stub's body (`decompiler/nonnative/kuna-jvm/src/kotlin/default_args_map.rs`):
the value it assigns when the parameter's mask bit is set; when no value can be read, the
parameter is written with `= ...` (the plugin's `kt-unknown-defaults`, fixed).

**What stays Java-shaped.** The plugin rewrites syntax, not semantics: the statement tree
and every expression in it are what the Java pipeline recovers from the bytecode. Calls
keep their JVM targets (`System.out.println(x)`, `text.length()` rather than
`text.length`, `StringsKt.repeat(..)` for an extension function), a value boxed or unboxed
by the compiler keeps the conversion (`(f(x) as java.lang.Number).intValue()`), a captured
local that a lambda mutates stays the compiler's `IntRef`/`ObjectRef` holder, the body of an
inline function (`let`, `apply`, `forEach`) appears expanded where it was called, since the
compiler copied it there, and a function reference compiled to a singleton class prints as
`<unrepresentable>.INSTANCE`. Local variables carry
the types the Java pipeline infers, so a local whose type has no Kotlin counterpart in the
metadata is written with its JVM type. A method or class that fails is written with
Vineflower's `// $VF: Couldn't be decompiled` comment inside its Kotlin declaration, as
for Java (section 10.12).
By default kuna's fixes write the JDK members and classes Kotlin maps by their Kotlin names
instead (`text.length`, `(f(x) as Number).toInt()`; fixes `kt_mapped_java_member` and
`kt_mapped_java_type`, section 10.16).

**Fidelity and front-end support (kuna).** The engine's Kotlin output is Vineflower's: the
Kotlin plugin's own tests and core's Kotlin tests are corpus cases (`tests/jvm/kotlin.json`),
compared with the released jar's text. `decompiler/nonnative/kuna-jvm/src/infra/kuna_spans.rs`
spans are recorded by the Kotlin writer too: a property's getter and setter share the span
of the property's declaration, a primary constructor's span is its `init` block (it has
none when the body is empty; it is opened before the constructor is stringified, so a
failure inside it is the constructor's), and members the writer folds away have none.
`DecompiledClass::language` (`decompiler/nonnative/kuna-jvm/src/infra/kuna_plugins.rs`)
reports `kotlin` for a unit whose file name ends in `.kt`, which is how the CLI labels
units and writes `.kt` files in a project export; the CLI's `--language java` is
`jvm_kt_enable off`. The Kotlin schedule polls the per-method deadline (section 10.12) at
the heads of its own loops: each round of a looping pass, the `finally` loop and the
initial stack-variable loop.

## 10.16 Kuna-only fixes (kuna)

Vineflower 1.12.0 throws on some methods, and the port, which keeps its failure behaviour,
throws there too and writes the method as `// $VF: Couldn't be decompiled` (section 10.12).
A *fix* is a deliberate departure from the Java at the place of such a defect, or of code
Vineflower prints wrong. The table in `docs/jvm.md` (*Kuna-only fixes*) describes each fix,
by its id, in one row: the defect and what kuna does instead. This section describes the
gate every fix goes through, the invariants the fixes keep, and the mechanisms groups of
them share. A fix longer than a few lines lives in its family's module,
`decompiler/nonnative/kuna-jvm/src/fixes/<family>.rs`, or, in a family with a folder, in
`decompiler/nonnative/kuna-jvm/src/fixes/<family>/<id>.rs`.

**No switch.** A fix is a correction, not a judgement call, so it has no option and no
gate: its code takes the place of the Java's at its site, and the engine has one mode.
`Fernflower::new` (`decompiler/nonnative/kuna-jvm/src/p0_knowledge/fernflower.rs`) sets the
`error-message` default to kuna's note (`report_to_kuna`). The regression corpus's runner
(`decompiler/nonnative/kuna-jvm/src/infra/kuna_corpus.rs (engine_options)`) runs every case
with the defaults: its Vineflower-oracle cases against a digest of the text kuna printed when
last reviewed, and each fix's case (`tests/jvm/kuna-fixes/`) against kuna's reviewed text.

**Invariants.**

- A shape a fix does not recognise keeps Vineflower's text, or fails as Vineflower does;
  where the evidence a fix needs is missing (a class outside the input, a type kuna cannot
  resolve), Vineflower's choice stands.
- The failure marker line never changes: the CLI and the benchmark count failed methods by
  it. `report_to_kuna` changes only the `error-message` note under it, putting kuna's
  request (`KUNA_ERROR_MESSAGE`) in place of the default before the caller's options are
  applied, so an `error-message` the caller passes (the corpus fixture's empty one) still
  wins; it reads the set being built, since `Fernflower::new` has installed no context yet.
- The `kt_*` fixes change Kotlin output only: in a pass or writer that Java output shares,
  they act only on a class written as Kotlin (a class with Kotlin metadata, the Kotlin
  plugin on).
- A rewrite keeps what the code does. A control-flow rewrite (a handler copy, a split node)
  is exact, and a fix's evidence includes recompiling its case's default text to the
  original's instructions (`docs/jvm.md`, *Fixing a Vineflower bug*).

**Exception ranges, handler copies and `finally`.** When a protected range has several
entries (kotlinc's `try` around a suspension point, whose resume label jumps into the
range; javac's record patterns inside a `try`),
`decompiler/nonnative/kuna-jvm/src/p2_lift/exception_deobfuscator.rs (ExceptionDeobfuscator::handle_multiple_entry_exception_ranges)`
splits it and gives every part but the first a copy of the handler's first block, with the
handler's instructions and successors but no bytecode offsets, placed in every range that
protects the original. `handler_copy_offsets`, `nested_handler_copies`,
`handler_copy_shared_body` and `copied_finally` restore the nesting right after the
duplication, give each copy its own body, let `FinallyProcessor`
(`decompiler/nonnative/kuna-jvm/src/p7_regions/finally_processor.rs`) accept copies without
offsets, and mark a copied catch-all that traces back to a processed `finally` as a
`finally`; a part split off at a resume label prints as a `try`/`finally` guarded by a
semaphore variable. `DomHelper` (`decompiler/nonnative/kuna-jvm/src/p7_regions/dom_helper.rs`)
builds a `try` from the blocks one handler protects, so ranges must be disjoint or nested,
and reduces a region by forming simple statements, by isolating a general statement between
a head and one of its extended postdominators, or by splitting irreducible nodes; a region
none of these reduces fails the method. `catch_loop_back_successor` and
`irreducible_with_exception_edges` steer such shapes onto one of those paths. `FinallyProcessor` checks a `finally`'s copies and
removes them, or falls back to a semaphore variable; `DomHelper.buildSynchronized` builds a
`synchronized` from a verified catch-all, and `SynchronizedHelper.cleanSynchronizedVar`
removes the lock variable (`e; dup; astore n; monitorenter` from javac, `e; astore n; aload
n; monitorenter` from kotlinc). `overlapping_ranges`, `protected_back_edge`,
`inline_marker_finally_copy`, `synchronized_break_exit` and `synchronized_lock_assignment`
each adjust one recognition step, so the built statement runs the `finally` or the release
exactly where the bytecode does.

**Variables: merging, scopes and names.**
`decompiler/nonnative/kuna-jvm/src/p6_variables/var_definition_helper.rs (VarDefinitionHelper::merge_vars)`
merges a definition into an earlier variable of the same slot that is no longer read,
under the type `getMergedType` gives. `var_merge_narrowing`, `var_merge_liveness` and
`var_merge_declared_type` are asked about every proposed merge and may refuse it; a refused
merge is denylisted like one `remapVar` could not do, and leaves two variables, which are
always correct since SSA split them (`var_merge_lost_definition` keeps the declaration
then). `iterateClashingNames` tracks which declarations are live through a scope and braces
the scopes or renames a variable on a clash; `switch_case_scope`,
`switch_expression_arm_scope` and `head_definition_scope` correct which scopes it visits,
in what order, and when it forgets a declaration. `NestedClassProcessor` renames a nested
body's variables whose names the enclosing method uses, but the new name reaches only some
of the places that print them; `lambda_catch_var_rename` and `nested_param_rename` give the
declaration the name its uses print with.

**Generic inference and casts.**
`decompiler/nonnative/kuna-jvm/src/substrate/invocation_exprent.rs (InvocationExprent::get_inferred_expr_type)`
and `decompiler/nonnative/kuna-jvm/src/substrate/new_exprent.rs (NewExprent::get_inferred_expr_type)`
infer a call's type arguments from the expected type the context passes down and the
argument types, record `GenericType::DUMMY_VAR` (printed as the diamond) where a
constructor's type arguments follow from the expected type, and decide from the result
which argument casts `appendParamList` writes and whether a `checkcast` is redundant. Their
view of what fixes a type variable differs from javac's (JLS 15.12.2.2, 18.4, 18.5.2) for
implicitly typed lambdas, inexact method references, lambdas passed for a bare type
variable and a functional argument's return type. The inference fixes
(`lambda_wildcard_return`, `external_class_diamond`, `inner_method_argument_shift`,
`raw_receiver_erasure`, `lambda_pertinence`, `lambda_type_variable_target`,
`diamond_argument_conflict`, `lambda_return_type_argument`) model javac only where they can
decide: they add or withhold an explicit type-argument list, write a raw `new` instead of a
diamond, or cast a lambda to the interface Vineflower mapped it to, always with
Vineflower's inferred values. Where a value meets a target type,
`decompiler/nonnative/kuna-jvm/src/p2_lift/expr_processor.rs (ExprProcessor::get_casted_exprent_full)`
casts a value not known to fit. `unchecked_generic_cast`, `unchecked_call_result_cast`,
`disjoint_generic_cast`, `boxed_type_variable_cast`, `external_supertype_arguments`,
`bounded_inference_cast`, `receiver_type_arguments` and `argument_generic_cast` compare the
type arguments the value's type gives the target's class with the target's own, and where
javac would reject the conversion write the source's unchecked cast, through the raw class
when the two parameterizations are provably distinct (JLS 4.5), or give the chain's first
call explicit type arguments; bounds and types kuna cannot resolve count as satisfied. `appendParamList` spreads a trailing array and writes
casts or boxing calls only where `getAmbiguousParameters` finds another overload, while
javac resolves the written call afresh (JLS 15.12.2): `vararg_array_length`,
`vararg_spread_resolution`, `unseen_class_overload`, `ambiguous_type_variable_argument`,
`unboxed_argument_type` and `reference_unboxing` correct the type each argument is written
as wherever javac would otherwise pick another overload, box to another type, or keep a box
or an identity comparison the bytecode lacks. For a class outside the input, the
descriptors the input calls stand in for its methods.

**Expression types.** A switch expression's or conditional's type is the join of its arms
(their nearest common ancestor, or `Object` when the class universe knows none). Writing
the arms or a cast against that join breaks Java's typing: a literal cannot have the join
type, and a cast makes the expression standalone, so numeric promotion applies.
`switch_yield_const_retype`, `switch_expression_poly_cast` and `ternary_numeric_arms` write
the arms so that each keeps the type the bytecode gives it. Vineflower also leaves widening
conversions unwritten; `implicit_widening_arithmetic`, `mixed_type_reassociation` and
`kt_implicit_widening` define a value's type as written (an unwritten conversion has its
operand's type, arithmetic the promotion of its operands', a conditional its arms') and
write the conversion back where that is narrower than the operation's or the target's
type: a cast or kept parentheses in Java, `.toInt()`, `.code` or `.toLong()` in Kotlin,
which never widens implicitly.

**Resugaring only javac's shape.** Vineflower resugars try-with-resources, compound
assignments, compact record constructors, pattern bindings, the null check of an explicit
outer instance and switch layouts from loose tests, so hand-written code of a similar form
prints as the sugar and behaves differently or does not compile. `twr_desugaring_shape`,
`twr_unresolved_resource`, `twr_class_version`, `compound_assignment_shape`,
`compact_record_constructor`, `pattern_binding_shape`, `synthetic_null_check_shape`,
`implicit_outer_instance`, `qualified_super_call`, `switch_case_order`,
`switch_table_fillers` and `switch_synthetic_default` accept the sugar only in the shape
javac emits, judged by offsets, operand-stack identity, LocalVariableTable names,
LineNumberTable lines and the switch instruction's targets (javac sorts keys but lays
bodies out in source order, and points a `tableswitch`'s gaps at the default by
`Gen.visitSwitch`'s cost rule); otherwise the code is printed as the bytecode has it.
Three recognisers get the same treatment.
`decompiler/nonnative/kuna-jvm/src/p8_structure/if_pattern_match_processor.rs (IfPatternMatchProcessor::check_branch)`
makes a pattern only under the preconditions of `pattern_variable_scope` and
`unchecked_type_pattern`.
`decompiler/nonnative/kuna-jvm/src/p8_structure/assert_processor.rs (AssertProcessor::replace_assertion)`
finds an assert's `throw new AssertionError` in the `if`'s body, its else or the statement
the body breaks to, and with `assert_dead_throw` and `assert_throw_after_if` consumes it
only when its other incoming edges are accounted for.
`decompiler/nonnative/kuna-jvm/src/p8_structure/switch_pattern_match_processor.rs (SwitchPatternMatchProcessor::process_statement)`
turns the index constants of a `SwitchBootstraps.typeSwitch` back into labels, one defect
of which each of `type_switch_enum_label`, `type_switch_pattern_type` and
`type_switch_label_lookup` corrects.

**Heap reads copied into several uses.**
`decompiler/nonnative/kuna-jvm/src/p3_dataflow/stack_vars_processor.rs (StackVarsProcessor)`
copies a value with several uses into each use, and the Kotlin plugin's schedule adds a
final pass that inlines every local (`INLINE_ALL_VARS`), a second `SimplifyStack` and
`EliminateDeadVarsPass`, which rely on what holds for stack variables but not for locals (a
value's use position is its place in evaluation order; two versions of one slot are never
live at once). A field or array-element read copied into several uses is read once per use,
and another thread may store between the reads. `field_read_reuse`, `kt_field_read_reuse`,
`kt_partial_inline`, `kt_inline_heap_read`, `kt_inline_lock_local`, `field_read_all_paths`,
`kt_inline_evaluation_order`, `var_merge_liveness` and `kt_semaphore_store_liveness` keep
such a value in its variable, or restrict the Kotlin passes for locals, so the output reads
the location once, as the bytecode does; a single use still moves the read.

**Kotlin members without metadata, and generated members.** The Kotlin writer
(`decompiler/nonnative/kuna-jvm/src/kotlin/kotlin_writer.rs`) writes declarations from the
metadata. `KConstructor` (`decompiler/nonnative/kuna-jvm/src/kotlin/k_constructor.rs`)
writes only the constructors the metadata gives it, and every other member goes through
`writeMethod`/`writeField`, ports of the Java writer that add `fun`/`open`. `writeMethod`
writes every method without metadata (`@JvmOverloads` overloads, `$$forInline` copies,
`DefaultImpls` bodies, `-impl` methods, bridges) with its receiver and return type from the
class file, and installs it as the current method wrapper, the state that also marks a
lambda body, so `KExitExprent` writes its `return` as a bare value (`kt_return_outside_lambda`
and its neighbours correct that for exits that are not a lambda's tail). kotlinc emits
members the source does not declare: forwarding stubs, `$$forInline` templates,
`@JvmStatic` copies, synthesized data-class members, read-only-collection stubs,
constructors for default arguments, an interface companion's field. The fixes of this
group recognise such a member by its exact code (loads of its parameters in order, perhaps
after null checks or marker stores, one invoke, a return) together with its name, flags and
descriptor, and leave it out or write its calls as calls of the member it forwards to; they
write constructors and fields left without metadata from the class file alone (secondary
constructors, `init`, `@JvmField` properties); they read metadata as Kotlin's reader does
where `KConstructor.parse` misreads it; and they restore the delegation call
`writePrimaryConstructor` and `stringify` take from a constructor's first statement where
Kotlin 1.3's parameter checks, expression checks or `hideEmptySuper` displaced it.
`KProperty` (`decompiler/nonnative/kuna-jvm/src/kotlin/k_property.rs`) writes a property
from its metadata and attributes only; the property fixes default absent accessor flags to
the property's, find the initializer where kotlinc put it, write a never-stored value as
the field's JVM default, reach the backing field as `field`, drop a modality Kotlin allows
none of, and read the flags the source kept only in the class file (an accessor's
`synchronized`, a field's `ACC_VOLATILE`, `ACC_TRANSIENT` or `ACC_SYNTHETIC`, a backing
field that is not private) to write the annotation, or the missing setter line, that
compiles back to them.

**Anonymous classes, object expressions and references.** The Kotlin writer prints an
anonymous class at its `new` as an `object` expression whose supertypes come from its
Class metadata, or as a lambda for Function metadata; a class with only synthetic-class
metadata (a lambda kotlinc did not compile to `invokedynamic`, a continuation, a function
or property reference) makes `writeClassDefinition` throw `Anonymous class does not have
Class Kotlin metadata`.
`decompiler/nonnative/kuna-jvm/src/p8_structure/nested_class_processor.rs (NestedClassProcessor)`
takes the superclass constructor call out of an anonymous class's constructor and keeps it
on the node, and with `verify-anonymous-classes` off (the default) reference classes count
as anonymous; it maps a nested class's captured values from the `new`s in the class it is
nested in, accepting only plain variables. `ClassesProcessor.loadClasses` nests a class
only through its encloser's `InnerClasses` entry, which does not list kotlinc's
per-call-site copies. `kt_synthetic_anonymous_object`, `kt_object_super_call`,
`kt_synthetic_object_super_call`, `kt_reference_anonymous_super_call`,
`kt_property_reference`, `anonymous_class_created_elsewhere`, `captured_var_clash_name`,
`captured_var_through_cast` and `unlisted_anonymous_class` rebuild `object` headers,
arguments and references from the kept call, nest such copies into the class their
`EnclosingMethod` names, and widen what counts as a capture. Each writes only shapes Kotlin
can express and otherwise fails as Vineflower does; `kt_anonymous_class_failure_reason`
gives the rest a named reason at the same throw site.

**Default arguments.** kotlinc compiles a function's defaults into a synthetic
default-arguments member, `<JVM name>$default` (for an interface function, in
`DefaultImpls`), whose parameters are the function's JVM parameters, one `int` mask per 32
value parameters and an `Object` marker: a caller passes a placeholder for each left-out
argument and sets bit `i % 32` of mask `i / 32`, numbering the value parameters, which for
an inner class follow the outer instance.
`decompiler/nonnative/kuna-jvm/src/kotlin/default_args_map.rs (DefaultArgsMap::from)` reads
each default back from the member's `if ((mask & bit) != 0)` branch, and a default it
cannot read prints as `= ...`; `KConstructor.parse` finds a constructor's
default-arguments constructor by its descriptor. `kt_default_args_super_call`,
`kt_default_args_call`, `kt_default_arg_compound_branch`, `kt_default_arg_assigned_param`,
`kt_default_args_ctor_descriptor`, `kt_default_args_bridge_lookup`,
`kt_default_value_names`, `kt_default_value_coercion_cast` and
`kt_delegated_member_default` read this layout correctly and write a call through it as
the call the source makes.

**Types and names in Kotlin output.** `KTypes`
(`decompiler/nonnative/kuna-jvm/src/kotlin/k_types.rs`) translates only `Integer`, `Object`
and the `java.util` collections and drops wildcards; other types fall back to Java's
`getCastTypeName`, and the header writers shared with Java print Java syntax. The type
syntax fixes (`kt_java_cast_syntax`, `kt_java_reference_cast`, `kt_java_type_syntax`,
`kt_explicit_type_arguments`, `kt_mapped_java_member`, `kt_mapped_java_type` and others)
write Kotlin's forms: projections, `Array<T>` and the primitive arrays, `Any`, Kotlin's
names for the mapped JDK classes and members, the read-only collection interfaces, casts as
`x as T`. A local is declared with its JVM type, so it is never nullable, raw for a generic
class (kotlinc writes no LocalVariableTypeTable) and `Byte` or `Short` when it holds only
constants; `kt_nullable_local`, `kt_nullable_cast`, `kt_raw_generic_type` and
`kt_narrow_constant_local` infer its type from its stores and reads, through the metadata
and signatures of fields, calls and parameters, to a fixed point, and a local whose
evidence is missing or disagrees keeps Vineflower's type. The JVM `Signature` lacks
variance, nullability, `Nothing` and Kotlin's collection names, while metadata can name type
parameters by ids no scope declares; `kt_class_header_metadata`,
`kt_enclosing_type_parameter`, `kt_type_parameter_id` and `kt_supertype_type_arguments`
write headers from the metadata when every name resolves, naming an unnamed id after the
`Signature`'s variable at the same position. `KInvocationExprent` reads metadata only from
the class a call names; `kt_facade_call`, `kt_extension_call`, `kt_jvm_name_call` and
`kt_synthetic_accessor_call` resolve the callee to the declaration Kotlin sees and write it
by its Kotlin name, bare only where Kotlin would resolve the bare name to it.
`KotlinWriter.toValidKotlinIdentifier` backticks names Kotlin cannot write bare, which
`kt_unescaped_name`, `kt_reserved_underscore_name` and `kt_java_keyword_variable_name`
extend to the writers shared with Java. A body takes its names from the LocalVariableTable
but a header from metadata, so while a body or default is written a variable at a
parameter's slot is written with the metadata's name, and an extension receiver as `this`
(`kt_function_parameter_name`, `kt_setter_parameter_name`, `kt_extension_receiver`).

**Control flow and operators in Kotlin output.**
`decompiler/nonnative/kuna-jvm/src/kotlin/k_do_statement.rs (KDoStatement::to_java)`
writes an integer `for (V = A; L < R; V++)` as a range (`..`, `downTo`, `until`) or
`repeat`, and any other `for` as its initializer and a `while (true)` loop that tests the
condition and runs the update at its end; the `kt_for_*` fixes and
`kt_range_operand_precedence` correct the range test, its bounds and the fallback. Kotlin's
`break@` and `continue@` need a loop and a `when` has no `break`: `kt_jump_to_non_loop_label`,
`kt_when_break`, `kt_java_statement_jump` and `kt_catch_block_jump` send every jump through
`KExprProcessor.jmpWrapper`, wrap a statement a written jump leaves in `run labelN@{ .. }`
with the jump as `return@labelN`, and drop a `break` that only ends a `when` branch; `run`,
`repeat` and `synchronized` are inline, so this compiles to the same jumps. The plugin
writes Java operators as Kotlin infix or member functions but keeps Java's precedence, its
implicit promotions and the compound operators Kotlin lacks; the operator fixes
parenthesise by Kotlin's levels, write promotions as explicit conversions that compile to
no instruction or to Java's widening, and expand `x op= y` where rereading `x` is
equivalent. `KFunctionExprent` makes Java's `==` of non-null operands Kotlin's `===` and
keeps a resugared `Intrinsics.areEqual` as `==`; the equality fixes keep that distinction
through negation, `copy` and `areEqual(..) ^ 1`.

**Checks, inlined code and static initializers in Kotlin output.** kotlinc inserts checks:
the `Intrinsics` null checks, in Kotlin 1.3's and 1.4's spellings, and a `throw
KotlinNothingValueException()` after a call that returns `Nothing`. `kt_not_null_assertion`,
`kt_expression_value_check`, `kt_legacy_parameter_checks`,
`kt_legacy_constructor_parameter_checks` and `kt_nothing_value_throw` turn a check back into
the source construct (`x!!`) or remove one the declared type implies, since kotlinc emits it
again; a check that alone carries a non-null contract stays. kotlinc also copies an inline
function's body into each call site; `kt_inlined_to_typed_array`, `kt_inlined_stdlib_call`
and `kt_reused_inline_object` write a copy that is exactly the function's body as the call
and leave anything else. `InitializerProcessor`
(`decompiler/nonnative/kuna-jvm/src/p8_structure/initializer_processor.rs`) moves each
`<clinit>` statement it can into its field's initializer and stops at the first it cannot,
leaving a remainder the Kotlin writer cannot express; `kt_object_instance_local`,
`kt_static_initializer`, `kt_property_initializer_key`, `kt_file_static_initializer` and
`enum_constant_in_values_array` remove the obstacle or write the remainder as an `init`
block or `run { }` initializers that run each statement where it ran.

**Suspend functions.** A method is in the suspend calling convention when its descriptor
ends in a `Continuation` and it returns `Object`. kotlinc lowers a suspending body to a
state machine whose resumed path recreates the direct path's frame, so the direct path is
the source's body. The `SuspendStateMachine` pass
(`decompiler/nonnative/kuna-jvm/src/fixes/kt_suspend/kt_suspend_state_machine.rs`), first
among the Kotlin plugin's graph passes, matches the machine instruction by instruction,
rewrites the graph to the direct path and rejoins the try ranges kotlinc split; any other
shape keeps Vineflower's machine. `kt_suspend_call_continuation` marks the calls that pass
`$completion`, so the continuation argument and the result cast are not written, and
`kt_coroutine_boxing`, `kt_suspend_function_type`, `kt_suspend_without_metadata`,
`kt_suspend_lambda` and `kt_suspend_impl` write the surface forms: boxing, `suspend`
function types, the suspend functions of objects without function metadata, suspend
lambdas rebuilt from `invokeSuspend`, and an open suspend function's `$suspendImpl` body.

The other fixes each correct one step of one Vineflower pass, and their table row is their
description.

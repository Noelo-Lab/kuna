# 10 — The JVM engine (vineflower)

```yaml
Anchors:
  - decompiler/nonnative/kuna-jvm/src
  - decompiler/nonnative/kuna-jvm/options.toml
  - decompiler/crates/kuna-cli/src/nonnative
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
the engine prints the text Vineflower prints, byte for byte. The port therefore keeps
Vineflower's pass order, its iteration orders (including the iteration order of Java's
hash collections, which reaches the output), its id counters (which reach the output as
`var10000` and `label7`) and its failure behaviour. Where this chapter describes a
heuristic that looks arbitrary, it is Vineflower's, kept for that reason. The one
deliberate exception is kuna's fixes of Vineflower's defects (section 10.16): each is a
named option, on by default, that changes only what Vineflower gets wrong, and
`jvm_vineflower_compat` turns them all off.

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
`decompiler/nonnative/kuna-jvm/src/infra/decompile_record.rs (DecompileRecord::add)`, which
is also where the trace differ hooks in (section 10.14).

Vineflower's plugin API is ported: the pass locations (`CfgConstruction`, `BeforeMain`,
`InLoopDecomp`, `MainLoop`, `AfterMain`, `AtEnd`, `LateProcessing`) are kept in the
schedule, and `decompiler/nonnative/kuna-jvm/src/infra/plugin_context.rs (PluginContext)`
registers each plugin's passes there and asks it for a language spec, which takes over the
method pipeline and the class writer for the classes it claims. Plugins are
not discovered on a class path: the engine carries the plugins `java -jar vineflower.jar`
bundles, in its registration order, in
`decompiler/nonnative/kuna-jvm/src/infra/kuna_plugins.rs`, and `jvm_plugins` (on by
default, as with `java -jar`; off is Vineflower's test fixture, which loads none) decides
whether they load. Each plugin's options join the option map with the plugin's defaults
when the plugins initialize, and an option a caller set keeps its value. Two are ported, in
`decompiler/nonnative/kuna-jvm/src/plugins`. **VariableRenaming** registers no pass; it
supplies the variable naming factory, which every method's local names pass through when
`VarDefinitionHelper` settles them and every parameter name passes through when it is
written. `jvm_variable_renaming` picks a renamer by name: `jad` names a variable after its
type (`i`, `j`, `k`, `l`, `i1`... for `int` and `long`, `flag`, `s`, `file1`, the lowercased
class name, `a` + the element type for arrays), counting each name per method; `tiny` gives
primitives one letter and steps through `a`..`z`, `ba`... on a clash, and an object the
class name with a lowercase first letter (plural for arrays), numbered on a clash, with `_`
after a keyword. A lambda body's method, an anonymous class's methods and a local class's
methods take over the enclosing method's naming state (`jad`'s counters, `tiny`'s used
names) once the enclosing method has been renamed. With no renamer named, the deprecated
`jvm_jad_style_variable_naming` selects `jad`; an unknown name selects none, and the names
stay as Vineflower's own pipeline gives them. `jvm_rename_parameters` (and, for `jad` only,
the deprecated `jvm_jad_style_parameter_naming`) renames parameters too; with a
LocalVariableTable, a parameter's entry carries no type at that point, so `tiny`, which
needs one, fails the method as Vineflower's provider does. **IdeaNotNull** registers one
`MainLoop` pass that, while `jvm_resugar_idea_notnull` is on (the default), removes the
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
class whose qualified name was already loaded is skipped with an error log; a class whose
internal name matches `jvm_excluded_classes`, or does not match `jvm_included_classes`
(both full-match Java regular expressions, run by the small backtracking engine in the
same file), is left out of the forest but still gives context. With `jvm_decompile_inner`
(the default), the `InnerClasses` entries of every class are read into records of type
*member*, *local* or *anonymous* (an entry with no simple name is anonymous; one whose
`EnclosingMethod` names a method is local), each remembering which class declared it.
Classes often disagree about a nested class; the record from the nested class itself
wins over its enclosing class's, which wins over a third class's, and
`jvm_warn_inconsistent_inner_attributes` logs each disagreement. With
`jvm_validate_inner_classes_names`, a member entry is believed only when its name is the
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
`LocalVariableTypeTable` (`jvm_use_lvt_names`) and `MethodParameters`
(`jvm_use_method_parameters`).

## 10.4 Bytecode to a control-flow graph

**Decode.** `StructMethod::expand_data` decodes the code attribute into an instruction
sequence with its exception table
(`decompiler/nonnative/kuna-jvm/src/p2_lift/full_instruction_sequence.rs`,
`decompiler/nonnative/kuna-jvm/src/p2_lift/instruction.rs`), one instruction per opcode
with resolved operands; `jvm_dump_bytecode_on_error` later prints this sequence inside a
failed method. The method's local-variable count seeds the variable counter, so every
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
3. `remove_empty_ranges` (with `jvm_remove_empty_try_catch`) - a range that protects no
   instructions is dropped together with its exception edges, and the method gets a
   `$VF: Removed empty exception range` comment.
4. `DeadCodeHelper::extend_synchronized_range_to_monitorexit` (with
   `jvm_ensure_synchronized_monitors`) - when some path out of a `monitorenter` region
   has no `monitorexit` inside the protected range, the range is extended to it, so the
   region can still become a `synchronized` block.
5. `DeadCodeHelper::incorporate_value_returns` (with `jvm_incorporate_returns`) - a block
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
(DecHelper::invalid_head_merge)`, `jvm_try_loop_fix`), which otherwise builds malformed
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
  (`jvm_remove_getclass`); the pre-Java-5 `class$` idiom is folded into a class literal
  (`jvm_decompile_java4`, finished at class level by section 10.11).
- **SSAU** (`decompiler/nonnative/kuna-jvm/src/p3_dataflow/ssau_constructor_sparse_ex.rs`):
  versions also change at every use, which yields a graph of definitions and uses. On
  the first round it marks effectively-final variables; on every round
  `StackVarsProcessor::iterate_statements` walks the blocks and inlines a variable's
  definition into its use when no intervening write can change the value. Each block is
  swept twice; the second sweep, which also inlines across the boundaries between
  expressions on the operand stack, runs only under `jvm_simplify_stack`, and it is what
  turns chains of temporaries back into `a = b = c`, `arr[i++] = v` and chained calls.

Both forms are reverted (all versions back to zero) after each use, so each round
re-derives them from the current tree. Then, once: `++`/`--` inside conditions is inlined
(`PPandMMHelper::inline_pp_iand_mmi_if`); for Java 9+ class files,
`decompiler/nonnative/kuna-jvm/src/p3_dataflow/concatenation_helper.rs` turns
`invokedynamic` string concatenation back into `+`; and with `jvm_decompile_assert`,
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
   merges nested `if`s into `&&`/`||` conditions and builds `if`-`else`. With
   `jvm_ternary_in_if` it also collapses the nested ifs a ternary inside a condition
   compiles to (`if (a ? b : c)`).
3. Stack variables are simplified and variable versions recomputed again, and
   `LabelHelper::identify_labels` decides which jumps need an explicit label.
4. `decompiler/nonnative/kuna-jvm/src/p3_dataflow/secondary_functions_helper.rs
   (SecondaryFunctionsHelper::identify_secondary_functions)` rewrites operator shapes:
   an `lcmp`/`fcmpl`/`dcmpg` result compared with zero becomes a direct comparison (one
   used as a value is spelled out as a ternary), `c ? true : false` and `c ? false : true`
   fold to `c` and `!c`, negations are pushed into comparisons, and an `if`-`else` with an
   empty `if` branch is inverted. `jvm_ternary_constant_simplification` extends the
   folding to every ternary with one boolean constant branch (`c ? true : x` to
   `c || x`).
5. `decompiler/nonnative/kuna-jvm/src/p5_types/intersection_cast_processor.rs` builds
   intersection casts (`(A & B) x`).
6. With `jvm_pattern_matching`, on Java 16+ class files,
   `decompiler/nonnative/kuna-jvm/src/p8_structure/if_pattern_match_processor.rs
   (IfPatternMatchProcessor::match_instanceof)` turns `instanceof` followed by a cast into
   an `instanceof` pattern, record patterns included.
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
  helper statements this leaves are marked hidden, and `jvm_show_hidden_statements`
  prints them anyway. If that produced a switch expression, switch expressions, stack
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
switch simplification once more (`FinalSimplifySwitches`) and, with `jvm_prettify_ifs`,
the removal of an `else` after a branch that always returns or throws
(`IfHelper::prettify_ifs`).

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
narrowest type that can hold them; with `jvm_boolean_as_int` the constant `1` (not only
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

**Names.** After each method is processed
(`decompiler/nonnative/kuna-jvm/src/p1_classfile/class_wrapper.rs (ClassWrapper::init)`),
variables get `varN`-style names that avoid the class's field names
(`decompiler/nonnative/kuna-jvm/src/p6_variables/var_names_collector.rs`), then, with
`jvm_use_lvt_names`, the debug names from the `LocalVariableTable` replace them wherever
the name is a valid identifier (`VarProcessor::set_debug_var_names`). With
`jvm_use_method_parameters`, a `MethodParameters` attribute names (and marks `final`) the
parameters. A method without code still gets parameter names. Names that clash within a
scope - two variables, a variable and a captured variable, a variable and a field it
shadows - are renamed apart (`VarDefinitionHelper::remap_clashing_names`), once at
processing time and again at writing time (`VarProcessor::rerun_clashing`), when nested
classes and lambdas have been processed. A parameter or lambda argument whose type stays
undetermined prints as `Object` under `jvm_undefined_as_object` and as `<undefinedtype>`
without it.

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
   is first used. `jvm_lambda_to_anonymous_class` writes every lambda as an anonymous class
   instead. `jvm_synthetic_not_set` recognizes compiler-generated members (outer-instance
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
a constructor's leading argument-less `super()` (`jvm_hide_empty_super`);
`decompiler/nonnative/kuna-jvm/src/p4_calls/class_reference14_processor.rs` turns the
Java 1.4 class-literal cache into `X.class` (`jvm_decompile_java4`);
`decompiler/nonnative/kuna-jvm/src/p8_structure/enum_processor.rs` removes `$VALUES`,
`values()`, `valueOf()` and the hidden name and ordinal constructor parameters
(`jvm_decompile_enums`); and
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
`decompiler/nonnative/kuna-jvm/src/p4_calls/condy_helper.rs` writes constant-dynamic
`VarHandle` constants as the equivalent `MethodHandles` lookup expression under
`jvm_decompile_complex_constant_dynamic`, with a comment because the expression runs
eagerly; otherwise the raw constant-dynamic form is kept.

## 10.12 Emission

`decompiler/nonnative/kuna-jvm/src/p9_emit/class_writer.rs (ClassWriter::write_class)`
writes a root class and everything nested in it into a text buffer, and
`ClassesProcessor::write_class` puts the header in front: `jvm_banner`, the package, and
the imports `decompiler/nonnative/kuna-jvm/src/p9_emit/import_collector.rs (ImportCollector)`
collected while the body was written. The collector decides, per class name, whether
the short name can be used in this file without ambiguity, and lists the imports that
requires; `jvm_remove_imports` writes no import statements. A `package-info` class and a
`module-info` class have writers of their own.

**What is written.** For each class: annotations, modifiers, type parameters, `extends`,
`implements` and `permits`, and for a record its components; then the enum constants,
the other fields with their initializers, the methods and the nested classes. Members are
left out by policy: synthetic members (`jvm_remove_synthetic`), bridge methods
(`jvm_remove_bridge`), a lone default constructor (`jvm_hide_default_constructor`), and
whatever the class-level rewrites hid. `jvm_override_annotation` adds `@Override` where an
instance method of a Java 5+ class file overrides or implements a method of a supertype
the engine can see;
`jvm_mark_corresponding_synthetics` names the synthetic method behind each lambda and the
class behind each anonymous class in a comment; `jvm_sourcefile_comments` writes the
`SourceFile` name above each class; `jvm_decompiler_comments` controls every `$VF:` note
the passes above attached to a method. Lambdas are written inline at their use, without
braces when the body is one expression (`jvm_inline_simple_lambdas`). Generic method calls
carry explicit type arguments under `jvm_explicit_generics`. Constants print as source
does: `Double.NaN`, `Long.MAX_VALUE` and `Math.PI` rather than their values unless
`jvm_keep_literals` is on, and non-ASCII characters escaped only under
`jvm_ascii_strings` (`decompiler/nonnative/kuna-jvm/src/substrate/const_exprent.rs`).
Numbers print with the JDK's own algorithms (section 10.13).

**Failed methods.** A method whose schedule threw is written as its signature and a body
of comments (`ClassWriter::dump_error`): `$VF: Couldn't be decompiled`, then with
`jvm_dump_exception_on_error` the `jvm_error_message` note (by default a request to report
the class to kuna's tracker, Vineflower's in compat mode; section 10.16), the exception
text and its throw location, and with `jvm_dump_bytecode_on_error` a listing of the
method's bytecode. The class around it is written normally.

**Layout.** The buffer is a tree of line groups; after a class is written,
`decompiler/nonnative/kuna-jvm/src/p9_emit/text_buffer.rs (TextBuffer::reformat)` breaks
argument lists, chained calls and long operator chains at their recorded break points
wherever a line exceeds `jvm_preferred_line_length` (160), and indents with
`jvm_indent_string` (three spaces). Offsets and widths are counted in UTF-16 units, as Java
counts them. Every reference and declaration in the text is a text token;
`jvm_dump_text_tokens` appends a dump of them with their positions.

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
whose writing throws is written, with `jvm_dump_exception_on_error`, as a
`/* $VF: Unable to decompile class ... */` comment, and without it not at all. Nothing
else stops the run: the other classes of the input are still written. A Java
`RuntimeException` and an ordinary Rust panic (an index out of bounds, an `unwrap` on
`None`) are caught at these boundaries alike (section 10.13); a stack overflow or an
allocation failure is not, and ends the process.

**Time limit (kuna).** Vineflower enforces `max-time-per-method` by running each method on a
watchdog thread and killing it. kuna instead arms a per-thread deadline
(`decompiler/nonnative/kuna-jvm/src/infra/cancelation_manager.rs (CancelationManager)`)
that the schedule polls at the heads of its fixed-point loops - the `finally` loop, the
stack-variable loop, the main and merge loops, the SSA fixed point, statement recovery and
irreducible-node splitting. A method past its deadline fails like a killed one: an error
log and a `$VF: Couldn't be decompiled` body whose exception is a `TimeoutException`
(`decompiler/nonnative/kuna-jvm/src/p1_classfile/class_wrapper.rs
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
added, so results are delivered only once clean. Results reach the caller in root order,
whatever the number of workers, and the call returns after the last delivery while the
workers drop their engines. Each engine instance runs on a thread with a 512 MiB
stack, since the port recurses as deeply as Vineflower does on the JVM.

**Options.** `decompiler/nonnative/kuna-jvm/src/p0_knowledge/kuna_options.rs (resolve)`
turns a `jvm_*` name, or Vineflower's long or three-letter name, and a value as a user
writes it into the Vineflower key and value; the table is generated from `options.toml` by
the crate's build script. Keys the front end sets itself (thread count, time limit, method
selection, logging, line separator, line mapping, class-path keys a JVM would serve) are
listed there as `[[driver_owned]]` and refused as user options. kuna's own rows resolve to
their own names; `Fernflower::new` takes `jvm_plugins` and `jvm_vineflower_compat` off the
property list before the rest becomes Vineflower's option map (section 10.16), so that map
is the one Vineflower would build. Any other key, a `jvm_fix_*` name included, is not a
switch: `resolve` refuses it as an unknown option, and given to the engine directly it
stays in Vineflower's option map like any key Vineflower does not read.

**Tracing.** With `KUNA_JVM_TRACE=<file>`,
`decompiler/nonnative/kuna-jvm/src/infra/kuna_trace.rs` writes a canonical dump of each
method's statement tree after every recorded pass, in the format the Java-side tracer of
`decompiler/nonnative/kuna-jvm/tools/vftrace` prints for Vineflower, so the two can be
compared pass by pass (`docs/jvm.md`).

**The CLI seam.** `kuna-cli` routes an input to this engine before any native code runs
when the input operand is a class file, an archive or a directory (chapter 00, section
0.2); its non-native module (this chapter's third anchor) builds a driver configuration
from the command line, slices classes and methods out of the delivered text by their
spans, and renders the native CLI's text and JSON shapes, tagging each unit with its
language (`java`, or `kotlin` for a unit the Kotlin plugin wrote, whose file name ends in
`.kt`). Before a command that decompiles (not `functions`) it sets glibc's heap policy
for the process (`decompiler/crates/kuna-cli/src/nonnative/jvm/heap.rs (tune)`, Linux
with glibc only): a 256 MiB top pad, no trimming, and blocks up to 32 MiB served from the
heaps instead of fresh mappings. The engine threads allocate and free many small objects,
and under the default policy each heap's growth and trimming are memory-map calls under
one process-wide lock that serialize the threads; the native path never sets it. The CLI
surface is `docs/cli.md` ("JVM inputs").

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
annotation printed like any other annotation; `jvm_kt_export_metadata` then still parses
the metadata, for other plugins (kuna bundles none that read it), and changes no output.
With `jvm_plugins off` no plugin is loaded and every class is Java.

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

**The pass schedule.** For a Kotlin class,
`decompiler/nonnative/kuna-jvm/src/infra/method_processor.rs (MethodProcessor::code_to_java)`
builds the graph, removes dead blocks, inlines `jsr`, connects the exit and removes gotos
as for Java, then
hands the method to the spec's passes, declared in
`decompiler/nonnative/kuna-jvm/src/kotlin/kotlin_plugin.rs`, and returns; none of the Java
schedule after that point runs, nor any plugin pass registered at the Java pass locations
(IdeaNotNull's, for one). The CFG pass is the exception-range repairs of section 10.4 in
the same order, step 8 as `decompiler/nonnative/kuna-jvm/src/kotlin/obfuscated_exceptions_pass.rs`,
except that steps 3-5 always run, whatever `jvm_remove_empty_try_catch`,
`jvm_ensure_synchronized_monitors` and `jvm_incorporate_returns` say. The statement tree comes from the same dominator-based recovery (10.5), and the
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
becomes `C::class`. `collapse_string_concat_pass.rs`, under `jvm_kt_collapse_string_concat`,
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
its parts, and with `jvm_kt_decompile_facades` also its forwarding functions. The members
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
signature as a plain `fun`, and a warning is logged. `jvm_kt_show_public` writes the `public` that Kotlin source
leaves implicit (on by default, as in Vineflower). A parameter default is recovered from
the `$default` stub's body (`decompiler/nonnative/kuna-jvm/src/kotlin/default_args_map.rs`):
the value it assigns when the parameter's mask bit is set; when no value can be read, the
parameter is written with `jvm_kt_unknown_defaults` (`= ...`), or with no default when
that text is empty.

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
Vineflower prints wrong.

**The gate.** `decompiler/nonnative/kuna-jvm/src/p0_knowledge/kuna_fixes.rs (KunaFix)`
names each fix; `KunaFix::id` gives its id (the name each paragraph below is headed by, and
the `fix` field of `tests/jvm/kuna-fixes/cases.json`). A fix is a correction, not a
judgement call, so it has no option of its own and is always on. The one switch is
`jvm_vineflower_compat` (off by default), which turns every fix off at once; with it on,
the engine is the plain port and prints Vineflower's text, failures included: methods
Vineflower fails on are written as its failure comment, code it prints wrong is printed as
it prints it, and the failure note names Vineflower's tracker. Every site where a fix
changes behaviour asks `kuna_fixes::enabled(KunaFix::..)` and keeps the Java's behaviour
verbatim in the other branch. `Fernflower::new`
(`decompiler/nonnative/kuna-jvm/src/p0_knowledge/fernflower.rs`) takes the switch off the
custom property list, resolves it into the set of active fixes (every fix, or none when
the last `jvm_vineflower_compat` pair is on) and stores it in the decompilation context;
installing a context (`DecompilerContext::set_current_context`) makes its set the one
`enabled` answers from on that thread, which costs one thread-local read per question. The
per-class contexts of `ContextUnit::set_context` copy the set from the root context, and no
context means every fix on. Unit tests that must isolate one fix build the set
`KunaFixes::DEFAULT.without(KunaFix::X)` and run with it through `Fernflower::new_with_fixes`
(or a corpus `Case`'s `fixes`); no option reaches that set. The regression corpus's runner
(`decompiler/nonnative/kuna-jvm/src/infra/kuna_corpus.rs (engine_options)`) turns compat
on unless a case says otherwise, so its Vineflower-oracle cases test the plain port, and
each fix has a case that runs in both modes (`tests/jvm/kuna-fixes/`): in compat mode it
must print the released Vineflower jar's text, failure comment included, and by default
kuna's reviewed text.

**`report_to_kuna`.** Vineflower's error comments (`$VF: Couldn't be decompiled`,
and the other `$VF:` errors a method collects, such as an `assert` that could not be
resugared) carry the `error-message` note, whose default asks the reader to report the
class to Vineflower's issue tracker. With this fix, `Fernflower::new` puts kuna's request
(to report to kuna's tracker, `KUNA_ERROR_MESSAGE`) in place of that default before the
caller's options are applied, so a `jvm_error_message` the caller sets still wins. The
fix is read from the set being built, since no context is installed yet at that point.
The marker line itself never changes: the CLI and the benchmark count failed methods by
it.

**`lambda_wildcard_return`.** When a lambda or method reference is an argument of
a generic call, `decompiler/nonnative/kuna-jvm/src/substrate/new_exprent.rs
(NewExprent::get_inferred_expr_type)` infers the type arguments of its functional
interface: from the call's expected type (the upper bound) it gathers a first mapping of
the interface's type variables, and for a method reference it then maps the interface
method's return variable to the referenced method's return type, remapped through the
receiver's type arguments. A guard Vineflower added in 1.10 (upstream commit `13d4112e`,
unchanged on `develop/1.13.0`) keeps an already inferred, finished generic mapping when
the method's return type is a subtype of it, and reads that return type's class name. When
the receiver's type argument is an unbounded wildcard (`Instantiator<?> i; ...
map(i::instantiateType)`, where `instantiateType` returns the class's `T`), the receiver's
mapping sends `T` to Fernflower's encoding of `?`, which is null, so the remapped return type
is null; the code before the guard allows that (it then adds the mapping), but the guard
dereferences it, and the method fails with `NullPointerException: returnType` while it is
written. With the fix, an unknown return type cannot contradict the mapping inferred from
the context, so the guard keeps that mapping, as it does for a known subtype. Only the
branch that threw changes: without an inferred generic mapping, the null return type is
added as before. spring-boot's `StructuredLoggingJsonProperties.customizers` is the case that
showed it; with the fix it prints `this.customizer().stream().map(instantiator::instantiateType).toList()`,
the source without its raw `(List)` cast, which javac does not record, exactly as Vineflower
prints the same call on a receiver without a wildcard.

**`twr_unary_condition`.** In a Java 11+ class, javac compiles a try-with-resources
to a plain `try` with a `catch (Throwable t)` handler that closes the resource and rethrows,
and the close sits inside `if (r != null)` when the resource may be null.
`decompiler/nonnative/kuna-jvm/src/p8_structure/try_with_resources_processor.rs (TryWithResourcesProcessor::make_try_with_resource_j11)`
recognizes that shape from the handler: when the handler is a sequence that starts with an
`if`, it strips the negations off the `if`'s condition and takes the result for the null
check, reading its first operand (which must be a variable) and its second (which must be
`null`). A handler written by hand that starts with `if (!flag)` on a boolean local
(spring-framework's `ConfigurationClassEnhancer.createClass`:
`catch (Throwable ex) { if (!fallback) throw ...; ... }`) leaves a one-operand negation
after the stripping, whose operand is a variable, so Vineflower reads the second operand
that a negation does not have and fails the method with an index out of bounds (still so on
`develop/1.13.0`). With the fix, a condition with fewer than two operands is not a null
check: the handler is not a try-with-resources, and the matcher leaves the statement alone,
as it does when the operand is not a variable; the `try`/`catch` is written as it is.
**`if_denest_fallthrough`.** `IfHelper::merge_ifs`
(`decompiler/nonnative/kuna-jvm/src/p8_structure/if_helper.rs (IfHelper::if_else_chain_denesting)`)
rewrites `if (a) { if (b) X; ...; Y } Z`, where the body and `Z` both jump to the end of
the enclosing sequence, as `if (!a) { Z } else { if (b) X; ...; Y }`: it negates the
condition and swaps the branches, which moves `Z` (and everything after the `if` in its
sequence) into the then-branch. Vineflower 1.12.0 (from upstream `91ff9353`, unchanged on
`develop/1.13.0`) checks that the body *has* a jump to the end (`hasDirectEndEdge`), which
any exit of any statement in it satisfies: a `return` in a `catch`, the rethrow edge of a
`finally` handler, one branch of an inner `if`-`else`. It does not check what `reorderIf`
checks before the same swap, that no jump from inside the `if` reaches the statements it
moves. When the body can also complete normally (a `try` whose body falls through to `Z`),
those jumps are left pointing into the then-branch of an `if` they are no longer inside,
an invalid tree (`ValidationHelper` reports `Break edge with closure with invalid direct
path` right after the swap). What happens next depends on `Z`: when it is a lone `return`,
`ExitHelper::condense_exits` later copies it into each jump and the method comes out
correct but restructured; otherwise the path through the body loses `Z`, and Vineflower
prints a `break` past it (a skipped statement, a missing `return`). With an `assert`
before the `try`, `AssertProcessor` moves the stale jump onto the new `assert` block,
which no later pass repairs; `ExitHelper::remove_redundant_returns` empties `Z`,
`SecondaryFunctionsHelper` drops the then-branch it has become, and the method fails with
`Could not find destination nodes for stat id` when `VarDefinitionHelper` flattens the
tree for the jump that still points at it. With the fix,
the swap is skipped when an edge from inside the `if` reaches a statement after it in the
sequence, and the method keeps its source shape. On the benchmark inputs it decompiles
Elasticsearch's `InternalEngine.closeNoLock` and changes two methods that Vineflower prints
correctly through the healed path (Ghidra's `DataTypeManagerDB.resolveBuiltIn`, Spring's
`AnnotationTypeFilter.hasAnnotation`); all three recompile to the original bytecode.
**`switch_yield_const_retype`.** A switch expression replaces a switch statement
whose arms all store one stack variable, and takes that variable's type, which type
inference computes as the join of the values stored in it: for two unrelated classes their
nearest common ancestor (`Serializable` for `String` and `Character`), or `Object` when the
class universe does not know one. When
`decompiler/nonnative/kuna-jvm/src/substrate/switch_exprent.rs (SwitchExprent::to_java)`
writes an arm that is a single `yield` of a constant, it first gives the constant the
switch's type, so that a `char` or `boolean` switch writes its arms' integer constants as
`'a'` or `true`. A literal can only have a primitive type, a box, `String`, `Class` or
`MethodType`, so when a constant arm sits next to an arm of an unrelated reference type
(`use(switch (c) { case 34 -> "\\\""; default -> (char) c; })`, a `String` and a boxed
`char`; a `String` and a `Class` literal, a `Boolean`, a `StringBuilder`) the constant
gets the join, which `ConstExprent::to_java` cannot write, and Vineflower fails the method
with `RuntimeException: invalid constant type: Ljava/io/Serializable; with value ...`.
Vineflower guards the same retyping where variables are merged (`canConstTypeMerge`,
upstream commit `aea5fd12`) but not here, and `develop/1.13.0` still has it. With the fix
the constant takes the switch's type only when
`decompiler/nonnative/kuna-jvm/src/substrate/kuna_fix_switch_yield_const_retype.rs (ConstExprent::can_be_written_as)`
accepts it, which is exactly when `to_java` can write it; otherwise the constant keeps its
own type. That type is a subtype of the switch's, because the join is an upper bound of
every arm, and the assignment or invocation context the switch expression sits in widens
it the way javac did when it compiled the arm. A switch whose type a literal can have is
written as before, so only methods Vineflower fails change. spring-framework's
`BasicJsonWriter.escape` is the case that showed it: its lambda now prints
`builder.append(switch (c) { case 8 -> "\\b"; ... default -> c <= 31 ? String.format("\\u%04x", c) : (char)c; })`.
Case labels, which `to_java` gives the selector's type, are left alone: the one failure
seen there (Vineflower's own `TestUnnamedVar1.patternSwitch3`) is a pattern switch whose
labels were never turned into patterns, and writing them would print wrong code.
**`catch_loop_back_successor`.** `CatchStatement::is_head`
(`decompiler/nonnative/kuna-jvm/src/substrate/catch_statement.rs (CatchStatement::is_head)`)
turns a node and the handlers only it reaches into a `try`/`catch` (section 10.5) and
chooses the statement that follows it, `next`: the node's own regular successor, else the
first handler's; a handler with another regular successor is left out, and the statement
is built only when the number of members that reach `next` is not one (the try block
counts two). In a `while (true)` loop whose body is a `try` that leaves the loop only by
`return` or `throw`, with two or more `catch` blocks that all go back to the top of the
loop, every handler's successor is the try block itself, so `next` is the try block. When
code precedes the loop, the loop's header is not the first statement of its region, so
those back edges are still regular edges (an edge back to the first block would already be
a `CONTINUE`), and the `try`/`catch` is built at the level of the region with `next` among
its own members: `collapseNodesToStatement` turns the back edges into `CONTINUE`s and then
adds a regular edge from the new statement to `next`, its own child. Nothing that leaves
the region reaches that statement backwards any more, the post-reverse-post-order of the
region leaves it out, and `DomHelper::find_simple_statements` fails the method with
`computing post reverse post order failed!` (Vineflower 1.12.0, and `develop/1.13.0`).
With a single `catch` block the count is one, the statement is not built at that level,
`DomHelper::find_general_statement` isolates the loop first, inside it the back edges are
`CONTINUE`s and the `try`/`catch` has no successor, and the loop becomes a `do` statement
around it. With the fix, `is_head` builds no `try`/`catch` whose `next` is one of its own
members (the try block or a kept handler), so the multi-catch loop takes the single-catch
path. On the benchmark's five inputs, okio, okhttp and kotlin-stdlib, `next` is a member in
13 methods: in 11 (one `catch` block, as in Guava's `Uninterruptibles`) Vineflower already
declines to build the statement, and the other two are Elasticsearch's
`RetryingInputStream.read()` and `read(byte[], int, int)`, retry loops of this shape, which
Vineflower fails and which now print their source.
**`kt_synthetic_anonymous_object`.** The Kotlin writer prints an anonymous class
where its `new` stands, as an `object` expression
(`decompiler/nonnative/kuna-jvm/src/kotlin/k_new_exprent.rs` calls
`KotlinWriter::write_class`), and takes the object's supertypes from its Kotlin Class
metadata (`decompiler/nonnative/kuna-jvm/src/kotlin/kotlin_writer.rs
(KotlinWriter::write_class_definition)`); a class with Function metadata is written as a
lambda instead. kotlinc 2.x gives a third kind of anonymous class only synthetic-class
metadata (`k = 3`) with no data: a SAM conversion or function-type lambda it cannot compile
to `invokedynamic` because it sits in an inline function and uses a `crossinline`
parameter (kotlin-stdlib's `compareBy`, `thenBy`, `AutoCloseable { }`; the inline functions
that call those, `sortedBy`, `sortBy`, create the same class), a suspend function's
continuation, and a reference to a local function. For such a class `writeClassDefinition`
throws `IllegalStateException: Anonymous class does not have Class Kotlin metadata` and the
method that creates it fails (unchanged on `develop/1.13.0`). With the fix,
`decompiler/nonnative/kuna-jvm/src/kotlin/kuna_fix_kt_synthetic_anonymous_object.rs`
writes the object from the class file: `object : ` and the supertypes other than `Object`,
from the generic signature when there is one, as the writer prints the supertypes of a named
class without metadata, and no "Class flags could not be determined" comment. Its methods
have no metadata either and go through `write_method`, with two differences that put them
where an object's functions stand: a `return` is printed (`write_method` installs the method
as the current method wrapper, and `KExitExprent` drops the `return` of a lambda body, which
it recognises by a current method wrapper; with the fix the method being written counts as
none), and a method whose body fails keeps its failure comment (`write_method_body` hides
such a method, which would print the object without it). An `object` expression has no
superclass constructor call, takes no constructor arguments and refers to what it captures by
name, so the fix writes only an object that needs none of that: its superclass is `Object`,
every constructor parameter is a captured variable, and the `new` passes each one the variable
of the name the object's body uses for it (`NestedClassProcessor` names the captures after the
enclosing method's variables, which is right where the class is created in that method, and
where an inline function that calls it passes a variable of the same name). A continuation, a
function reference or a class created with other arguments still fails as in Vineflower.
The 37 kotlin-stdlib methods of this failure now print `object : Comparator { fun compare(a: T,
b: T): Int { return .. } }` (or `AutoCloseable`, `Function1<T, Integer>`).

**`kt_local_as_extension_receiver`.** For a method without metadata,
`decompiler/nonnative/kuna-jvm/src/kotlin/kotlin_writer.rs (KotlinWriter::write_method)`
decides that it is an extension function when the variable in its first parameter slot is
named `$this$..`, and then reads the first parameter's type. In a method without
parameters that slot holds no parameter but can still hold such a name: `NestedClassProcessor`
gives every method of an anonymous class a variable, in its first free slot, for each
variable the class captures, named after it (`$this$iterator` when it captures the enclosing
function's extension receiver), and kotlinc names the receiver of a lambda with receiver it
inlines into the method `$this$..` too. So `operator fun <T> Enumeration<T>.iterator() =
object : Iterator<T> { .. }` fails while the `remove()` kotlinc adds to an object implementing
a read-only `Iterator` is written, and spring-framework's `BeanDefinitionDsl$bean$$inlined$bean$N`
classes (a `Supplier` whose `get()` runs an inlined `BeanSupplierContext.() -> T`) fail whole,
with `ArrayIndexOutOfBoundsException` (kuna: `index out of bounds`; unchanged on
`develop/1.13.0`). With the fix only a method with a parameter can be an extension function;
the others are written as plain methods.
**`kt_default_args_super_call`.** For a constructor with default arguments the Kotlin
compiler also emits a synthetic default-arguments constructor `<init>(P1..Pn, int mask..,
DefaultConstructorMarker)`: a caller passes a placeholder (null or zero) for each argument it
leaves out and sets bit `i % 32` of mask `i / 32` for it, and that constructor computes the
defaults and delegates. A subclass whose supertype call leaves arguments out (`class Sub(a: Int) :
Base(a)` with `Base(a: Int, b: String = "b")`) calls it. When the Kotlin writer prints the primary
constructor's supertype call,
`decompiler/nonnative/kuna-jvm/src/kotlin/k_constructor.rs (KConstructor::write_primary_constructor)`
removes the marker argument and its type from the descriptor (`KUtils::remove_arguments`) but
keeps the placeholders and the masks, so the call names `<init>(P1..Pn, int)`, a constructor the
superclass does not have. `InvocationExprent::append_param_list` asks the class processor for
that constructor's synthetic-parameter mask; when the superclass is decompiled in the same run
and its wrapper is still alive (it is nested in the same top-level class, or its own top-level
class has not been written yet), `ExprUtil::get_synthetic_parameters_mask` throws
`RuntimeException: Constructor ... not found` outside any method's guard, and the whole
top-level class fails (kotlinx-coroutines' `BroadcastChannelImpl`, kotlin-reflect's `CallerImpl`).
Otherwise the call is printed with its placeholders and masks, `: Base(a, null, 2)`, which is not
the call the bytecode makes. With the fix,
`decompiler/nonnative/kuna-jvm/src/kotlin/kuna_fix_kt_default_args_super_call.rs (resugar)`
rewrites such a call into the call it stands for when all of these hold: the descriptor has the
default-arguments shape (n >= 1 parameters, n / 32 + 1 `int` masks, the marker); the masks are
`int` constants that default only parameters of the constructor, at least one and only trailing
ones; each left-out argument is a constant placeholder; the superclass has the synthetic
default-arguments constructor and `<init>(P1..Pn)`, and its Kotlin metadata declares
`<init>(P1..Pn)` with n value parameters and a default for each left-out one; and no other
constructor of the superclass could take the arguments that remain (one with that many
parameters, a varargs one, or one with defaults of its own). The call then keeps its leading
arguments and the descriptor of `<init>(P1..Pn)`, and is printed `: Base(a)`, which Kotlin resolves
to that constructor with the same defaults. Every other shape keeps Vineflower's code: a call that
leaves out an argument before one it passes needs named arguments, which the parameter-list
printer cannot write, and with a competing constructor the shorter call could select it. As the
defect is the rewriting and not the lookup, the fix also changes the supertype calls Vineflower
prints with placeholders without failing: okhttp 4.7.2's `RealWebSocket.WriterTask` and
kotlin-stdlib 2.2.20's `TerminateException` lose their `false, 2` and `null, null, 6` and read
`: Task("${...} writer")` and `: FileSystemException(file)`, as in their sources. It also takes
the write order out of the result, which with Vineflower's code decides between failing and
printing the placeholders (and so differs between kuna's single-instance and parallel runs).

**`kt_default_arg_compound_branch`.** To print default values,
`decompiler/nonnative/kuna-jvm/src/kotlin/default_args_map.rs (DefaultArgsMap::from)` reads them
back from the default-arguments method or constructor: for each `if ((mask & bit) != 0)` test it
takes the first expression of the branch and, when that is an assignment, records its right-hand
side as the default of the parameter the bit names; any other test or branch records nothing, and
the parameter is printed with `jvm_kt_unknown_defaults` (`= ...`). Vineflower takes the branch's
expression list without checking that the branch is a basic block. When a default is computed by
several statements (`names.map { ... }`, which the compiler inlines into a loop), the branch is a
sequence, which has no expression list, and the `NullPointerException` fails the whole top-level
class, because the defaults are read while the member headers are written (kotlin-reflect's
`AnnotationConstructorCaller` and `AnnotationConstructorCallerKt`). With the fix, a branch that is
not a basic block with at least one expression records nothing, as a branch that is not an
assignment does, and the parameter is printed `= ...`. Reading the sequence's first statement
instead would take the first temporary of the inlined code for the default value.

**`generic_parent_without_signature`.** To type a field read or a call on a value of
a generic type, `decompiler/nonnative/kuna-jvm/src/p5_types/generic_type.rs (GenericType::map_gen_vars_to)`
maps the type variables of the member's class (the generic type its `Signature` attribute
declares) onto the value's type, argument by argument, and then onto the value's enclosing
type: when the value's enclosing type is generic and the declaration's is not, it looks up
the enclosing class and maps through the generic type that class's signature declares. A
non-generic inner class of a generic class (`Outer<E>.Itrs`, with no type parameters and a
plain superclass) has no `Signature` attribute, yet a type nested in it is written with
`Outer`'s arguments (`Outer<E>.Itrs.Node`), so Vineflower dereferences the missing signature
and fails the method with a `NullPointerException` (the JDK's
`ArrayBlockingQueue$Itrs.doSomeSweeping` and three more methods of it; still so on
`develop/1.13.0`). With the fix, such a class maps as the generic type its signature would
declare, its name without type arguments (`Outer$Itrs<>`, what `GenericMain` builds for a
class whose signature has no formal parameters); its arguments match the value's empty
ones, and the mapping goes on to the next enclosing type, so `Outer`'s `E` still maps to the
value's argument (`Outer<String>.Itrs.Node` gives `E` to `String`). That is what Vineflower
does when every class on the way has a signature; the other choice, skipping the enclosing
types, would lose the mapping and print redundant casts.

**`self_referencing_lambda`** (the fix `self_referencing_lambda`). A lambda whose
implementation is a synthetic method of its own class is written in place, as a lambda
expression whose body is that method's code, and
`decompiler/nonnative/kuna-jvm/src/p4_calls/lambda_processor.rs (LambdaProcessor::process_class)`
nests every lambda a method creates under the lambda that method implements, so that the
inner lambda is written inside the outer one's body. When an implementation method creates,
directly or through the bodies of other lambdas, a lambda that it implements itself (no
compiler does this; obfuscators and hand-written bytecode do, as Vineflower's
`TestRecursiveLambda` does), the nesting has a cycle and writing it in place never ends.
Vineflower throws `Lambda X is recursive?!` for the whole class when the lookup returns the
lambda itself, which happens for one ordering of the one-method cycle only (upstream
`bfa4a794`, unchanged on `develop/1.13.0`), and overflows the stack for every other ordering
and for longer cycles, where kuna, whose stack overflow is not recoverable, aborts. With the
fix, `decompiler/nonnative/kuna-jvm/src/p4_calls/kuna_fix_self_referencing_lambda.rs (write_recursive_lambdas_as_method_references)`
runs before the nesting is built: it finds the implementation methods that reach themselves
through the lambdas they create, writes every lambda one of them implements as a method
reference to it (`this::body`, `Owner::body` for a static one), which calls the same method
with the same receiver, and nests nothing under those lambdas; the methods stay methods of
the class, hidden like every synthetic method under `jvm_remove_synthetic`. A recursive
lambda that captures values besides its receiver has no method-reference form, and the class
fails with a message that says so instead of the overflow. Lambdas whose methods are not on
such a cycle are written in place as before.

**The fix `external_class_diamond`** (`KunaFix::ExternalClassDiamond`) corrects output that Vineflower prints
wrongly without failing. Vineflower prints `new C<>(..)` only when the inference of
`decompiler/nonnative/kuna-jvm/src/substrate/new_exprent.rs (NewExprent::get_inferred_expr_type)`
finds `C` in the class universe with a `Signature` that declares type parameters; it then
infers the instance's type arguments from the expected type (the upper bound that a field
initializer, a declaration, an assignment, a `return` or a parameter of a known method
passes down) and gives the constructor the diamond. A class that is not in the universe,
such as a library that was not given to the decompiler, is taken to be non-generic, and
`new C(..)` is printed raw even where the expected type is `C<A, B>`. Raw, a lambda
argument loses its parameter types and the class does not recompile (Elasticsearch's
`PARSER = new ConstructingObjectParser("task_info", true, objs -> … (String)objs[i++] …)`,
which javac rejects with `array required, but java.lang.Object found`; in the benchmark this
keeps 106 Elasticsearch classes from recompiling). The expected type itself proves the class
generic when it is a parameterization of `C` itself (`C<..>` with type arguments, not a
supertype of `C`, not an array, not a wildcard). With the fix,
`decompiler/nonnative/kuna-jvm/src/substrate/kuna_fix_external_class_diamond.rs (infer)`
takes the step that `InvocationExprent::get_inferred_expr_type` takes for a known generic
class, with the class's formal parameters read off that type: each type argument that
names a type usable at the site (not `?`, not a type variable out of scope) maps its
parameter; when at least one does, the constructor gets the diamond, and the expression's
type is the expected type when every argument mapped (otherwise the raw type, as for a
known class). Anonymous classes (written from their own signature), arrays, a class in
the universe (generic or not), and Kotlin output (where Kotlin has no diamond) are left
to Vineflower's code. In the benchmark's output the change is only `new C(` becoming
`new C<>(` (with a reflow where the longer line wraps): 3 sites in spring-boot, 133 in
Ghidra and 353 in Elasticsearch, none in Guava, spring-framework or the Kotlin libraries.

**The fix `statement_as_expression`**. Unlike the fixes above, this one corrects output
Vineflower prints without failing, so the default text differs from Vineflower's on code it
prints wrongly. Vineflower models `assert`, `return`, `throw`, `yield`, a monitor and a local
variable declaration as exprents, like expressions, and an `assert` that
`decompiler/nonnative/kuna-jvm/src/p8_structure/assert_processor.rs (AssertProcessor)`
resugars from the `$assertionsDisabled` check is a lone `AssertExprent` in a basic block. Two
places put such a lone exprent where Java requires an expression, and each excludes the
statement-only kinds it knew of by hand. `decompiler/nonnative/kuna-jvm/src/p9_emit/class_writer.rs (ClassWriter::class_lambda_to_java)`,
under `inline-simple-lambdas`, writes a lambda whose body is one basic block with one exprent
as an expression lambda unless the exprent is a declaration or a `throw` (a `return` is written
as its value), so `forEach((k, v) -> assert v.equals(k) : k)` comes out. `decompiler/nonnative/kuna-jvm/src/p8_structure/merge_helper.rs (MergeHelper::match_for)`
takes the last exprent of a loop body for the `for` update when the loop has an initializer
before it and that exprent is alone in a block reached by more than one edge (the `issingle`
path accepts any kind), and refuses only an exit, so a `while (more) { ...; assert x; }` whose
last statement follows an `if` becomes `for (...; more; assert x)`. javac rejects both
(`assert.as.identifier`, `not.stmt`); no upstream commit changes either check through
`develop/1.13.0`. With the fix both ask one predicate,
`decompiler/nonnative/kuna-jvm/src/substrate/kuna_fix_statement_as_expression.rs (is_statement_only)`:
an `assert`, an exit, a `yield`, a monitor, a statement head or a declaration is never written
as an expression. The lambda then gets a block body, the text Vineflower itself prints for it
with `inline-simple-lambdas` off, and the loop stays the `while` loop with the `assert` as its
last statement, as `match_for` leaves a loop that ends in a `throw`. The checks Vineflower
already made give the same answers; a `return` without a value, a monitor and a declaration
never reach either position from javac's bytecode (the trailing `return;` is removed first, a
monitor moves into its `synchronized` head, declarations are set after loops are built), so
on javac output the fix changes exactly the `assert` cases.

The next three fixes correct code that Vineflower prints without a failure comment but that is
not valid Java; the default output differs from Vineflower's text on exactly those classes.

**The fix `enum_body_separator`**. Java's enum body is `{ [constants] [; members] }`: the `;` is
required before the members even when there are no constants. `ClassWriter.writeClass`
(`decompiler/nonnative/kuna-jvm/src/p9_emit/class_writer.rs (ClassWriter::write_class)`) writes
the `;` only after the last constant it wrote, so an enum used as a namespace of static
members (`public enum Util { ; public static ... }`) is printed `public enum Util { public
static ...`, which javac rejects (`enum constant expected`; still so on `develop/1.13.0`). With
the fix, a class declared with the `enum` keyword (the test of `write_class_definition`; not an
enum constant's body) whose first written member has no constant before it gets the `;` on a
line of its own first, followed by the blank line that separates constants from members
(`decompiler/nonnative/kuna-jvm/src/p9_emit/kuna_fix_enum_body_separator.rs`). An enum with
neither constants nor members stays `enum E {}`.

**The fix `anonymous_static_qualifier`**. Since Java 16 an anonymous class may declare static
methods. A class nested in it (another anonymous class, a local class) calls one by its simple
name, which finds it through the enclosing classes (JLS 15.12.1), and javac compiles the call
as `invokestatic` of the anonymous class (JLS 13.1). `InvocationExprent.toJava`
(`decompiler/nonnative/kuna-jvm/src/substrate/invocation_exprent.rs (InvocationExprent::to_java)`)
qualifies every static call whose owner is not the class being written with the owner's short
name, which `ImportCollector.getShortName` gives as `<unrepresentable>` for an anonymous class,
so the call is printed `<unrepresentable>.m(...)` and the file does not parse (Elasticsearch's
`AbstractTSDBDocValuesProducer`; still so on `develop/1.13.0`). With the fix
(`decompiler/nonnative/kuna-jvm/src/substrate/kuna_fix_anonymous_static_qualifier.rs`), such a
call is written unqualified when the anonymous owner lexically encloses the class being
written and no class on the way out (the writer's class and its enclosing classes, with their
supertypes in the class context) has a method of that name, which would capture the simple
name; otherwise Vineflower's text stays, since Java has no other way to name the method. A
static field of an anonymous class read from a nested class has the same qualifier and is not
changed: a simple field name can also be shadowed by the locals of every enclosing method.

**The fix `assert_dead_throw`**. `AssertProcessor` runs before the bodies of the `if`s are
inlined, so the `throw new AssertionError(...)` of an assert is often not the `if`'s body but
a statement the `if` breaks to, placed after the loop or labeled block the break leaves.
`AssertProcessor.replaceAssertion`
(`decompiler/nonnative/kuna-jvm/src/p8_structure/assert_processor.rs (AssertProcessor::replace_assertion)`;
the 1.12 assert rework, upstream `d00eec94`, unchanged on `develop/1.13.0`) builds the `assert`
from that statement, removes the `if`'s edge to it and its own exit edge, and leaves it in the
tree. It is then dead code after a statement that cannot complete normally (javac:
`unreachable statement`, or `return outside method` when the `return;` printed before it sits
in a static initializer), and it keeps the variables of its message alive outside their loop,
so they are declared before the loop: a for-each over such a variable is written
`for (key : keys)`, which does not parse, and a later local sharing its slot is merged into it.
With the fix
(`decompiler/nonnative/kuna-jvm/src/p8_structure/kuna_fix_assert_dead_throw.rs`), when the
throw was reached through the `if`'s break, it is removed from its sequence once the assert is
built, provided every other edge into it is the structural edge from the statement before it,
a sequence or an infinite loop (all loops are still infinite at this point), whose normal
completion would show as a break edge into the throw, and none is left. The assignment-assert
and switch-expression shapes, whose conversion replaces more than the `if`, keep Vineflower's
behaviour.

**The fix `handler_copy_offsets`**. A basic block keeps, next to its instructions, their original
bytecode offsets (`instrOldOffsets`), which only feed source positions: expression offsets,
line mapping and local-variable-table lookups. When a protected range has several entries,
`decompiler/nonnative/kuna-jvm/src/p2_lift/exception_deobfuscator.rs (ExceptionDeobfuscator::handle_multiple_entry_exception_ranges)`
splits it, and `insert_dummy_exception_handler_blocks` gives every part but the first its own
copy of the handler block; the copy has the handler's instructions but no offsets. A Kotlin
suspend function produces this whenever a `try` spans a suspension point: the state
machine's resume label jumps into the middle of the protected range. If the handler is a
`finally`, `decompiler/nonnative/kuna-jvm/src/p7_regions/finally_processor.rs (FinallyProcessor::remove_exception_instructions_ex)`
strips the handler copy's leading `astore` and trailing `aload`/`athrow`, both when the inlined
copies are verified and removed and when a semaphore variable is inserted instead, and it
removes the matching offsets as it goes (added upstream in `d88f743c`, from ForgeFlower). On a
handler copy there is no offset to remove, so Vineflower fails the method with an index out of
bounds (`-1` on the verified path, `0` on the semaphore path). The defect is in this
bookkeeping only: the instructions were already stripped, and the structure the processor
builds does not depend on the offsets. With the fix, on a handler copy that holds the whole handler
(a block without offsets whose only successor is the method's exit, because it ends in the
rethrow), the processor removes an offset only when the block has one at that index, which is
how the rest of the engine treats a block without offsets (`BasicBlock::get_old_offset` answers
-1, and the block splitting of `compare_basic_blocks_ex` already guards its removal). Wherever
Vineflower does not throw, the removals are the same as Vineflower's. Two shapes keep failing,
because getting past the throw would print wrong code. When the `finally` body spans several
Vineflower does not throw, the removals are the same as Vineflower's. One shape keeps failing,
because getting past the throw would print wrong code: when the `finally` body spans several
blocks (`if (n.decrementAndGet() == 0) channel.close()`), the deobfuscator copies only its first
block and every copy shares the rest, so stripping the shared rethrow for one part would leave
the other parts without it; the copy is then not a whole handler and the method fails as in
Vineflower (kotlinx-coroutines' `combineInternal` lambda, which `develop/1.13.0` prints with
undefined `throw var2_2` statements). A catch-all that irreducible-flow splitting copied in such a method, which `iterate_graph` leaves alone and which would print as a `catch` that swallows the exception (kotlinx-coroutines' `consumeEach`, printed that way by `develop/1.13.0`), is made a `finally` by the fix `copied_finally` below. Upstream fixed the same failure on `develop/1.13.0`
(commit `5c360738`, #597) by copying the offsets into the handler copy and no longer editing
offsets in `FinallyProcessor`. kuna does not port that: a copy with offsets changes the
local-variable names of handler copies in methods that decompile today (kotlinx-coroutines'
`TimeoutKt`, `catch (TimeoutCancellationException var12)` becoming `e`), and no longer trimming
the offsets shifts the source positions of every `finally` handler. The structure kuna prints
for the methods the fix decompiles is the one `develop/1.13.0` prints: the `try` whose exit
holds the inlined `finally` copy becomes a plain `try`/`finally`, and each part split off at a
resume label becomes a `try`/`finally` guarded by a semaphore variable, because its normal exit
leads to the suspension check rather than to a copy of the `finally`.

**Fix `nested_handler_copies`**. The same duplication puts each copy into every range that
protects the original handler, under that range's handler. When the handler is itself protected,
as the `catch` of a `try`/`catch`/`finally` is by the `finally`'s handler, and the outer handler
was duplicated too (it has one range for the try body and one for the catch, and more when the
body was split), a try body ends up protected by one copy of the outer handler and its catch by
another. `DomHelper` builds a `try`/`catch` only when the body and the catch share their other
handlers (`DecHelper.checkStatementExceptions`), so the method fails with `parsing failure!`; the
original outer handler, moreover, now protects several separate copies, the multiple-entry range
the pass exists to remove. javac produces the shape with record deconstruction patterns inside a
`try`: each accessor call gets a `catch (Throwable)` that throws `MatchException`, one handler with
a range per call, which sets the duplication off and with it the duplication of the user's
`catch`; kotlinc produces it with `try`/`catch`/`finally`, `use` and `consume` around a
suspension point. kuna restores the nesting right after the duplication
(`decompiler/nonnative/kuna-jvm/src/p2_lift/kuna_fix_nested_handler_copies.rs
(NestedHandlerCopies::nest)`), for each outer handler whose copies break it: when that handler's
ranges together have a single entry, its copies were not needed and their ranges go back to the
handler itself, which gives the source's single `try`; otherwise each catch, with the blocks only
it runs, moves to the copy of the outer handler that protects its try body. Both changes are
exact, since the copies of a handler have its code, successors and protection. Not fixed on
develop/1.13.0.

**Fix `copied_finally`**. A suspend function's loop is entered at its head and, from the resume
path, in its middle. `IrreducibleCFGDeobfuscator.splitIrreducibleNode` copies statements to give
it a single entry, and once the try regions inside it are built the copies can include a
`try`/`finally`. `FinallyProcessor.iterateGraph` processes only catch-alls that are not copies, and
the copies are made anew by every re-parse before their original is known to be a `finally`, so a
copy stays a `catch (Throwable)` around the handler the processing has stripped of its `astore`
and rethrow, and the output swallows the exception. With the fix,
`BasicBlockStatement.getSimpleCopy` records the block a copy's block comes from
(`decompiler/nonnative/kuna-jvm/src/p2_lift/basic_block.rs (BasicBlock::get_copy_of)`), and
`iterateGraph` marks a copied catch-all whose handler block traces back to a processed `finally`
as a `finally`, with its original's semaphore variable. Without the two fixes above no method
reaches this; with them, every suspend loop inside a `try`/`finally` does.

**Pattern switches: `type_switch_enum_label`, `type_switch_pattern_type`, `type_switch_label_lookup`.**
javac compiles a pattern switch to `SwitchBootstraps.typeSwitch(selector, restartIndex)`, whose
static arguments are the case labels in order (an `Integer`, a `String`, a class, or, for a
qualified enum constant, a dynamic constant), followed by a `lookupswitch` on the returned index;
a guard that fails sets the restart index and loops. `decompiler/nonnative/kuna-jvm/src/p8_structure/switch_pattern_match_processor.rs (SwitchPatternMatchProcessor::process_statement)`
turns the index constants among the case values back into labels: a pattern from the case
body's cast assignment, and, from the bootstrap arguments, constants and type patterns. Three
defects of that step have one fix each. *Enum labels* (`type_switch_enum_label`): a `case E.X`
beside type patterns is passed as `ConstantBootstraps.invoke(EnumDesc.of, ClassDesc.of("pkg.E"),
"X")`, which Vineflower 1.12.0 does not read, so the raw index stays a label and the switch head's
type check fails the method (`Invalid switch case set`). With the fix,
`decompiler/nonnative/kuna-jvm/src/p8_structure/kuna_fix_type_switch_enum_label.rs (resolve_label)` resolves such a constant to the enum
field, as upstream's `CondyHelper` does on `develop/1.13.0` (`36ac62ec`), reading `ClassDesc`'s
binary name in internal form, and marks the field a dynamic constant as upstream does; the
switch writers print such a label qualified (`case E.X:`, without the dynamic-constant comment)
unless the switch is on the enum's own type, since an unqualified constant does not compile
there. Upstream qualifies every enum label whose type differs from the switch's, which also
changes `enumSwitch` labels built from a widened selector type (its own
`TestSwitchPatternMatching19` now prints `case TestSwitchPatternMatching19.E.X1` for `XXX.X1`);
kuna leaves those as Vineflower 1.12.0 prints them. *Pattern types* (`type_switch_pattern_type`): when a
pattern's type is the selector's static type there is no `checkcast`, and Vineflower builds the
pattern's type operand as a class literal, of type `java.lang.Class`, where its two other paths
use the type itself. The switch head's bound computation
(`decompiler/nonnative/kuna-jvm/src/substrate/switch_head_exprent.rs (SwitchHeadExprent::check_expr_type_bounds)`)
joins the selector's type with each label's, so `Integer` joined with `Class` widens the selector
to `Serializable` (with the JDK loaded) or `Object`, which no integer constant label fits: the
method fails, either in that computation or when the constant label is printed against the
widened type, and without debug information the pattern variable itself is declared
`Class<Integer>`. With the fix the operand is typed by the label's class. Upstream's change on
`develop/1.13.0` (`4cad12c6`) instead stops the bound computation at the contradiction, which
leaves the widened type and moves the failure to printing when there is no LocalVariableTable;
kuna does not take it. *Label lookup* (`type_switch_label_lookup`): Vineflower looks for index
`i` as an integer constant equal to `i` in the first label of each case and otherwise uses case
number `i`, so a case with several labels keeps the raw index of every label after the first
(`case 5, 7` prints `case 5: case 1:`; `case E.X, E.Y` and `case Integer _, String _` fail on
printing the index against a reference selector, or index past the last case), and an integer
label already substituted is taken for an index (`case 1 -> ...` beside a type pattern at index
1 becomes `case Integer var5 -> ...`). With the fix,
`decompiler/nonnative/kuna-jvm/src/p8_structure/kuna_fix_type_switch_label_lookup.rs (replace_labels)` records every index constant before
substituting and replaces each, by identity, in whichever case holds it. A case with several
type patterns, which Java (22+) allows only unnamed, javac compiles to a guard that tests the
selector against those types again (`sel instanceof Integer || sel instanceof String`, then
`&&` the source's guard if any); `strip_multi_pattern_guards` removes exactly that test, and the
switch statement and expression writers print such a case as one label of unnamed patterns
(`case Integer _, String _:`). Each fix's case recompiles to the original bytecode.

The next seven fixes correct generic types that Vineflower prints without a failure comment but
that javac rejects, the largest remaining reasons the benchmark's classes do not recompile; the
default output differs from Vineflower's text only where Vineflower's is not valid Java (or, for
`inner_method_argument_shift`, an argument's cast changes with the corrected match). None of
them is touched on `develop/1.13.0` (whose only overload change, `1123be5b`, is about varargs).
Each fix's case in `tests/jvm/kuna-fixes/` recompiles to the original bytecode.

**The fix `unchecked_type_pattern`**. `IfPatternMatchProcessor.findPatternMatchingInstanceof`
turns `if (x instanceof C) { T v = (T) x; ... }` into the type pattern `x instanceof T v`, and
when the variable's type is generic it writes that type into the pattern. Java allows a type
pattern only where the cast it stands for is checked (JLS 15.20.2): `T` is reifiable, or `x`'s
static type already fixes `T`'s type arguments. A pattern of a type variable (`sym instanceof NT
nt`) or of a parameterization the operand does not determine (`value instanceof List<String>
list` on an `Object`) is rejected (`instanceof.reifiable.not.safe`). With the fix,
`decompiler/nonnative/kuna-jvm/src/p8_structure/if_pattern_match_processor.rs (IfPatternMatchProcessor::check_branch)`
asks `decompiler/nonnative/kuna-jvm/src/p8_structure/kuna_fix_unchecked_type_pattern.rs (is_checked_cast)`
first, and makes no pattern when the cast is unchecked: the `instanceof` and the declaration
from the cast stay as they are in the bytecode. A cast is checked when the target is reifiable
(no type arguments but unbounded wildcards), when the operand's type is a subtype of it, or when
the target's class extends the operand's and every one of its type parameters is passed through
as one of the operand's type arguments, equal to the target's (`Box<String>` to
`Leaf<String>` for `Leaf<T> implements Box<T>`). The operand's static type is the type of the
value assigned to it just before the `if` (which Vineflower then moves into the `instanceof`),
or a method parameter's declared type, whose variable has no local-variable-table entry yet.

**The fix `inner_method_argument_shift`**. `InvocationExprent.getInferredExprType`
(`decompiler/nonnative/kuna-jvm/src/substrate/invocation_exprent.rs (InvocationExprent::get_inferred_expr_type)`)
maps the method's generic parameter types to the argument types to infer type variables. For a
method of a class in the input that is nested and not static, it starts at the second argument
(`start = 1`, since `c43c76f7` of 2019), as if the method took the outer instance, which only an
inner class's constructor does, and the constructor path handles it apart. So every argument is
matched to the next parameter's type: `subMap(K, boolean, K, boolean)` maps `K` to `boolean`,
`put(K, V, long)` maps `K` to `V` and `V` to `long`, and `append_param_list` casts the
arguments to those types, `this.subMap((boolean)fromKey, true, (boolean)toKey, false)`, which
javac rejects. With the fix the arguments are matched from the first; as Java does for any
instance method, only the method's own type variables are inferred from them, since the class
type variables come from the receiver (the receiver of such a call is often an outer instance
read through a synthetic field, which carries no type arguments, and Vineflower would otherwise
infer the class's type variable from an argument and drop a cast the call needs).

**The fix `raw_receiver_erasure`**. The members of a raw type are erased (JLS 4.8): on a raw
`Map`, `keySet()` is a raw `Set` and `set.toArray(new String[0])` is an `Object[]`. Vineflower
infers a call's generic result whatever its receiver, finds `String[]`, and takes the
`checkcast` the bytecode holds for redundant; and it writes a for-each over a raw collection
with the typed loop variable javac then sees as `Object`. With the fix,
`decompiler/nonnative/kuna-jvm/src/substrate/kuna_fix_raw_receiver_erasure.rs (has_raw_receiver)`
finds the calls whose receiver's static type, as the decompiled program declares it, is raw (a
class with type parameters, written without type arguments): a variable by its parameter or
local-variable-table type (not `this`, not a lambda's parameter or a captured variable, whose
types javac infers), a field of a known class, a written cast, or a call whose declared result
is raw. Such a call's inferred type is its erased descriptor type, so the cast is written
(`(String[])map.keySet().toArray(...)`), and its arguments are cast to the erased parameter
types. `decompiler/nonnative/kuna-jvm/src/substrate/do_statement.rs (DoStatement::to_java)`
casts a raw for-each iterable to its class parameterized by the loop variable's (boxed) type
when the class has one type parameter and passes it to `Iterable`, `(List<MethodInfo>)methods`,
replacing the iterable's own raw cast when it has one (`(List<String>)entry.v2()` for
`(List)entry.v2()`); the cast adds no `checkcast`.

**The fix `boxed_type_variable_cast`**. Java casts a primitive to a reference type only by boxing
it and widening the box (JLS 5.5), so `(T) 0` does not compile for a type variable `T`.
Vineflower hides a boxing call (`Integer.valueOf(x)`) wherever Java boxes implicitly, and so
writes `(T) Integer.valueOf(0)` as `(T)0`. With the fix, a boxing call cast to a type variable
keeps its call, whether the cast comes from the bytecode (`decompiler/nonnative/kuna-jvm/src/substrate/function_exprent.rs (keep_boxing_under_cast)`)
or from the cast `decompiler/nonnative/kuna-jvm/src/p2_lift/expr_processor.rs (ExprProcessor::get_casted_exprent_full)`
adds for an assignment, a return or an argument.

**The fix `lambda_argument_overload`**. An implicitly typed lambda, or a method reference whose
target is chosen by overload resolution, is compatible with every overload whose parameter at
its place is a functional interface of the same arity whose result fits the lambda (JLS
15.12.2.1); two such overloads whose interfaces are unrelated make the call ambiguous:
`spliterator.forEachRemaining(this::add)` on a `Spliterator.OfInt`, which has
`forEachRemaining(IntConsumer)` and `forEachRemaining(Consumer<? super Integer>)`.
`InvocationExprent.getAmbiguousParameters` compares the lambda's own interface type, the one
the bytecode created, with each overload's parameter and finds only the overload called, so no
cast is written. With the fix,
`decompiler/nonnative/kuna-jvm/src/substrate/kuna_fix_lambda_argument_overload.rs (ambiguous_arguments)`
marks such an argument ambiguous when another overload of the same arity takes, at its place,
a known functional interface with as many parameters whose result kind (void or a value) the
lambda can produce (a method reference both when its method returns a value, a lambda what its
implementation method returns), is not a supertype of the called overload's interface, and
accepts the other arguments; the argument is then cast to its interface,
`(IntConsumer)(this::add)`. Overloads that take a type variable at the lambda's place are not
considered.

**The fix `static_type_variable_scope`**. A class's type variables are not in scope in its
static methods and initializers (JLS 6.3, 8.1.3). `Exprent.getNamedGenerics`
(`decompiler/nonnative/kuna-jvm/src/substrate/exprent.rs (Exprent::get_named_generics)`)
counts them everywhere inside the class, so a type variable of a static method that a call
leaves unresolved, named like one of the class's, is taken for the class's and written in a
cast (`getUninterruptibly((Future<V>)future)` in Guava's `AbstractFuture`), which javac rejects
in a static context. With the fix the walk out from the code stops counting class type
variables once it has passed a static method
(`decompiler/nonnative/kuna-jvm/src/substrate/kuna_fix_static_type_variable_scope.rs (is_static_method)`;
a synthetic method, such as a lambda's static implementation method, says nothing about the
context the lambda is written in); the method's own type variables, and those of the methods
enclosing a local or anonymous class, stay. With the class's type variable out of the way,
Vineflower also finds the diamond of a generic `new` in a static method
(`new ElementOrder<>(...)` for `new ElementOrder(...)`). Static nested classes, whose bodies
are not in their enclosing classes' scope either, are not treated: there the walk's other
defect, that an enclosing class's type variable overrides a same-named inner one, decides
whether Vineflower writes a diamond its inference cannot check (Guava's
`MultimapBuilderWithKeys.enumSetValues` would get a diamond javac rejects).

**The fix `unchecked_generic_cast`**. A cast between two parameterizations of one class, such as
`(ReferenceEntry<K, V>) NullEntry.INSTANCE` for an enum that implements
`ReferenceEntry<Object, Object>`, `(CacheBuilder<K1, V1>) this` or `(Comparator<Object>)
comparator` on a `Comparator<? super E>`, is unchecked and compiles to no instruction, so the
bytecode keeps no trace of it. `ExprProcessor.getCastedExprent` writes a cast where the erased
types need one or where `doGenericTypesCast` recognises a few generic shapes, and otherwise
writes the value bare, which javac rejects when its generic type does not fit the target
(`incompatible types`). With the fix,
`decompiler/nonnative/kuna-jvm/src/p2_lift/kuna_fix_unchecked_generic_cast.rs (cast_for)`,
asked by `decompiler/nonnative/kuna-jvm/src/p2_lift/expr_processor.rs (ExprProcessor::get_casted_exprent_full)`
for a return, an assignment or a field initializer (not an argument), takes a value whose
static type the program declares: `this` (not of an inner class, whose type does not show the
enclosing instance's type arguments), a static field or a field of `this` of a known class,
or a variable with a local-variable-table entry or a parameter's declared type. It views
that type as the target's class (the target being a declared parameterization: the method's
generic return type, the variable's or the field's type) and, when a type argument of the
target that is not a wildcard differs from the value's at its place, writes the cast: to
the target when at every such place one side is a type variable or the value's is a
wildcard, else, the two parameterizations being provably distinct (JLS 4.5), to the raw
class, which javac converts unchecked (`(ImmutableList) delegates`, as the source has it).
Values of other kinds, a call's result above all, are left alone: their types are
Vineflower's inference, not a declaration.

# 11 — The JVM engine (vineflower)

```yaml
Anchors:
  - decompiler/managed/kuna-jvm/src
  - decompiler/managed/kuna-jvm/options.toml
  - decompiler/crates/kuna-cli/src/managed
  - decompiler/managed/kuna-managed
  - decompiler/managed/kuna-dex
```

Chapters 01-09 describe the native pipeline: machine code lifted through SLEIGH to
p-code and recovered by `kuna-decomp`. This chapter describes the second, non-native
engine, `kuna-jvm`, which decompiles JVM class files and jars to Java, and classes
compiled from Kotlin to Kotlin, and shares none of that pipeline. JVM bytecode is typed and stack-based, and the class file already carries
what native recovery has to infer: the class structure, member signatures, generic
signatures, exception tables, nesting attributes and bootstrap methods for lambdas. The
engine therefore works over a Java-level IR of its own (statements and expressions, in
`decompiler/managed/kuna-jvm/src/substrate`) instead of over p-code. Why it is a
separate engine and how it is used is `docs/jvm.md`, its regression corpus is
`decompiler/managed/kuna-jvm/corpus/README.md`; this chapter is what it computes.

**Lineage.** The engine is a line-by-line Rust port of Vineflower 1.12.0, itself a fork
of JetBrains' Fernflower. In this chapter, untagged prose describes the Vineflower port
and `(kuna)` marks what kuna added or replaced. Upstream names are given as
parenthetical provenance (`MethodProcessor.codeToJava`); every claim is anchored to the
Rust file that implements it. Option names are the `jvm_*` rows of
`decompiler/managed/kuna-jvm/options.toml`; their defaults, tiers, symptoms and flip
guidance are in the generated catalog ([docs/options-jvm.md](../options-jvm.md)).

**Order reaches the output.** The port keeps Vineflower's pass order, its iteration orders
(including the iteration order of Java's hash collections, which reaches the output), its id
counters (which reach the output as `var10000` and `label7`) and its failure behaviour, apart
from kuna's fixes of Vineflower's defects (section 11.16). Where this chapter describes a
heuristic that looks arbitrary, it is Vineflower's, kept for that reason. Where two forms are equivalent, kuna writes the source's (section 11.10).
**The fidelity contract** shapes everything below: on the same class files and options,
the engine prints the text Vineflower prints, byte for byte, except where one of kuna's
fixes of Vineflower's defects applies (section 11.16). The port therefore keeps
Vineflower's pass order, its iteration orders (including the iteration order of Java's
hash collections, which reaches the output), its id counters (which reach the output as
`var10000` and `label7`) and its failure behaviour. Where this chapter describes a
heuristic that looks arbitrary, it is Vineflower's, kept for that reason. kuna's source forms
choose the source's form of equivalent code over Vineflower's (source locals and source
patterns, section 11.10).

**Code conventions.** Every ported file starts with `//! Port of \`org/jetbrains/java/decompiler/.../X.java\`.`
(`org/vineflower/...` for the plugins), and the port follows the original line by line,
so a change upstream can be found and applied in the same place:

- **One Java top-level class, one module**, named after the class in snake case
  (`IfHelper` is `if_helper`, `SSAUConstructorSparseEx` is `ssau_constructor_sparse_ex`;
  acronyms split oddly: `IFernflowerPreferences` is `i_fernflower_preferences`,
  `PPandMMHelper` is `p_pand_mm_helper`). Every ported module is re-exported flat from the
  crate root (`kuna_jvm::if_helper`), so code never names a phase folder; a fix is named by
  its family (`crate::fixes::switches`, `crate::fixes::generic_casts::receiver_type_arguments`). Types keep their Java names,
  nested Java types live in their outer class's module, and methods are the Java names in
  snake case with prefixes kept (`getParent` is `get_parent`). A class of static methods
  is a unit struct, so a call reads like the Java (`IfHelper::merge_all_ifs(root)`).
  Overloads carry a suffix and a `/// Java: name(ParamTypes)` line.
- **Three shapes of class.** Anything with identity or shared mutable state is an *arena
  object*: a `Copy` handle whose `==` is Java's reference `==`, with Java subclasses as
  typed views over one `kind` enum (`x.is_if()`, `x.as_if()`, `x.try_as_if()`) and virtual
  methods as dispatchers on the base type. Immutable values that Java compares both by
  identity and by `equals` (`VarType`, `MethodDescriptor`, constants) are `Rc`-backed,
  with explicit `equals` and `ptr_eq` and no `PartialEq`, so every comparison says which
  one it means. Small records and Java enums are plain Rust values.
- **Java semantics, not Rust defaults.** A nullable reference is an `Option`; integers
  keep Java's widths and use wrapping arithmetic wherever Java can overflow; strings that
  may be non-ASCII are indexed in UTF-16 units; numbers and characters are formatted and
  classified through `rt/`; `HashMap`/`HashSet` are always the kernel's JDK-order
  collections (the workspace's Clippy configuration bans the `std` ones); a Java `throw`
  is `jthrow!` and a `catch` is `rt::try_catch`.
- **Counter discipline.** Ids come from counters and reach the output (`var10000`,
  `label7`), so an object is allocated exactly where the Java constructor runs, never as
  an extra temporary.
- **kuna's own code** lives in `kuna_*` modules (`kuna_input`, `kuna_inventory`,
  `kuna_options`, `kuna_driver`, `kuna_engine`, `kuna_partition`, `kuna_spans`,
  `kuna_corpus`, `kuna_reclaim`), in `src/fixes/`, in the modules that replace JVM
  facilities (`context_source`, `jdk_runtime`, `zip_reader`, `byte_cursor`), and in the
  Kotlin plugin's seams (`kt_metadata`, `kt_attrs`, `k_dispatch`). Changes inside ported
  files are kept to the seams those modules need (span marks in the writer, a deadline
  poll in the cancellation manager, partial runs in `fernflower.rs`) and to the sites of
  kuna's fixes (section 11.16), noted in the file headers.
- **Comments are minimal**, as everywhere in kuna: Java comments and Javadoc are not
  transcribed.

## 11.1 The phase folders and the schedule

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
| `rt/` | (kuna) the Java-runtime kernel the port stands on (section 11.13) |
| `infra/` | the per-method schedule, pass hooks, the parallel driver and the debugging aids |
| `plugins/` | Vineflower's IdeaNotNull plugin's pass (this section, below) |
| `kotlin/` | Vineflower's Kotlin plugin: metadata decoding, its pass schedule, expression and statement layer and writer (section 11.15) |
| `fixes/` | (kuna) the fixes of Vineflower's defects, one module per family, called from the folders above (section 11.16) |

The schedule has two levels. **Per input**, `decompiler/managed/kuna-jvm/src/p0_knowledge/fernflower.rs
(Fernflower::decompile_context)` loads the class universe and nests classes (section
11.2), then saves every own unit (`decompiler/managed/kuna-jvm/src/p0_knowledge/struct_context.rs
(StructContext::save_context)`). Each unit is saved in two phases
(`decompiler/managed/kuna-jvm/src/p0_knowledge/context_unit.rs (ContextUnit::save_selected)`):
first every root class of the unit is *processed* (its methods decompiled, its nested
classes attached), then every root class is *written*. The split is load-bearing: writing
one class reads state that processing other classes left behind (the constructor masks of
another class's nested classes, the switch maps of synthetic classes), so no class is
written before all of its unit is processed. **Per method**,
`decompiler/managed/kuna-jvm/src/infra/method_processor.rs (MethodProcessor::code_to_java)`
runs the pass schedule of sections 11.4-11.10 from bytecode to a structured statement
tree. Every pass is recorded by name through
`decompiler/managed/kuna-jvm/src/infra/decompile_record.rs (DecompileRecord::add)`.

Vineflower loads plugins that register passes at named locations of this schedule and
may supply a language spec, which takes over the method pipeline and the class writer for
the classes it claims. The engine has no plugin loading: of the plugins `java -jar
vineflower.jar` bundles, IdeaNotNull and Kotlin are ordinary parts of the pipeline, always
present, and VariableRenaming (JAD-style and tiny-style renamers of locals) is not ported,
so every name stays as Vineflower's own pipeline gives it. `Fernflower::new` installs the
Kotlin spec and puts the Kotlin options' defaults into the option map, keeping a value a
caller set. **IdeaNotNull**
(`decompiler/managed/kuna-jvm/src/plugins/idea_not_null_pass.rs`) is a step of the Java
schedule's main loop, after the loop condensing, that removes the
null checks IntelliJ IDEA's compiler instruments: an `if (p == null) $$$reportNull$$$0(n);`
heading the method body for a parameter whose invisible parameter annotations carry
`org/jetbrains/annotations/NotNull` (the annotation lists counted from the last parameter
when javac recorded fewer lists than parameters) is replaced, head block and all, by an
empty block; and, in a method annotated `@NotNull`, an `if (v == null) $$$reportNull$$$0(n);`
right before `return v;` is removed, its head block's expressions moving into the return's
block, as is the older `if (v == null) <exit>; else return v;` shape; a change restarts
the loop. **Kotlin** is the language spec: it claims the classes the Kotlin compiler
emitted and decompiles and writes them as Kotlin (section 11.15). Every other class is
written as Java; Vineflower has no other language support.

## 11.2 The class universe and nesting

**Inputs (kuna).** `decompiler/managed/kuna-jvm/src/p0_knowledge/kuna_input.rs (classify, load)`
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

**Sources and libraries.** `decompiler/managed/kuna-jvm/src/p0_knowledge/struct_context.rs
(StructContext)` holds the session's units: *own* units (what is decompiled) and library
units (read for context only: the user's libraries and the JDK class library). A class
name is looked up in the unit that lists it, and lookups are cached, failed ones
included; a class found under an entry path that does not match its name is re-keyed by
its real name. The class
reader is `decompiler/managed/kuna-jvm/src/p0_knowledge/context_source.rs`, with
`decompiler/managed/kuna-jvm/src/substrate/zip_reader.rs (ZipReader)`, which follows JDK
21's `java.util.zip.ZipFile` (end-record search, central-directory order, prefix stubs
absorbed through the central-directory offset, stored and deflated entries only).

**The JDK (kuna).** Vineflower reads the Java standard library through the `jrt:` file
system of the JVM it runs on. kuna has no JVM, so
`decompiler/managed/kuna-jvm/src/p0_knowledge/jdk_runtime.rs (JdkRuntime)` locates a
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
`decompiler/managed/kuna-jvm/src/p0_knowledge/identifier_converter.rs (IdentifierConverter)`
renames classes, fields and methods whose names are not valid Java identifiers, are
reserved words or are short obfuscated names, before anything is loaded, rewriting every
constant-pool reference through `decompiler/managed/kuna-jvm/src/p0_knowledge/pool_interceptor.rs`.
The naming policy is `decompiler/managed/kuna-jvm/src/p0_knowledge/converter_helper.rs
(ConverterHelper)`; a user renamer class (Vineflower loads one by name) cannot be loaded
without a JVM, so the default policy is always used.

**Nesting.** `decompiler/managed/kuna-jvm/src/p1_classfile/classes_processor.rs
(ClassesProcessor::load_classes)` turns the own classes into a forest of class nodes. A
class whose qualified name was already loaded is skipped with an error log. Every own class
joins the forest: which roots are written is the driver's selection (`DriverConfig::select`,
section 11.14), and the others still give context. With `jvm_decompile_inner`
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
own top-level class. Lambdas get nodes too, but later (section 11.11).

## 11.3 The class-file model

`decompiler/managed/kuna-jvm/src/p1_classfile/struct_class.rs (StructClass)`,
`struct_method.rs` and `struct_field.rs` (in the same folder) model a class: access
flags, the constant pool (`decompiler/managed/kuna-jvm/src/p1_classfile/constant_pool.rs`),
members, and the attributes Vineflower reads, each a view over one attribute object
(`decompiler/managed/kuna-jvm/src/p1_classfile/struct_general_attribute.rs`).
`decompiler/managed/kuna-jvm/src/p1_classfile/bytecode_version.rs (BytecodeVersion)`
turns the major version into the feature predicates the passes test: `jsr` is legal up to
Java 6, lambdas from Java 8, `invokedynamic` string concatenation from Java 9, `instanceof`
patterns and switch expressions from Java 16, sealed classes from Java 17, record
patterns from Java 21; preview versions count from the release that previewed them.

Which compiler wrote a class (kuna). The rules that read javac's code shapes (sections 11.9
and 11.10) also read the Eclipse compiler's code, which lays a few constructs out its own way, so
a class records whether ECJ wrote it, decided once from the raw constant pool when the class is
read (`decompiler/managed/kuna-jvm/src/p1_classfile/kuna_compiler.rs (written_by_ecj)`). ECJ's
`ClassFile` enters the class's own name first and its superclass's second, each
`CONSTANT_Class` right before the `CONSTANT_Utf8` it names, then the names and descriptors of the
fields and methods in member order, a method's `Code` attribute name before any constant its
code uses. javac generates all method code before it writes the class, so the member references
of that code come before `Code` (javac 8 to 12 put them at the start of the pool, javac 13 and
later as the code uses them, the class itself included); ASM and `kuna-dex` enter a name's
`CONSTANT_Utf8` before the `CONSTANT_Class` that names it. A class is ECJ's when entries 1 to 4
are that header (entries 3 and 4 only when it has a superclass) and the first `Code` comes before
the first field or method reference; a pool without either decides nothing, and the class is
treated as javac's. On classes compiled from the same sources by javac 8, 11, 17, 21 and 25 and
by ECJ 3.46 (`-1.8`, `-17`), every ECJ class with code and member references is recognised and
no javac class is. ECJ's other marks (a `$SWITCH_TABLE$` method for an enum switch, one
`hashCode` switch for a string) are not read.

Two lifetimes coexist (kuna). Classes, members, attributes and the class-node forest live
for the whole session; the IR a class's decompilation builds lives only until that root
class is finished. Method bodies are therefore kept as raw bytes and expanded on demand
(`decompiler/managed/kuna-jvm/src/p1_classfile/struct_method.rs (StructMethod::expand_data)`),
and annotations, which Vineflower parses straight into expressions, are parsed at load
time into a session-lived element tree and built into expressions on first use in each
class scope (`decompiler/managed/kuna-jvm/src/p1_classfile/struct_annotation_attribute.rs`).
A session object never hands out a class-scope object of an earlier decompilation, which an
engine the driver reuses for another batch would otherwise read (section 11.14): an expanded
sequence whose arena was reset counts as released
(`decompiler/managed/kuna-jvm/src/p1_classfile/struct_method.rs (StructMethod::get_instruction_sequence)`
does not return it, `expand_data` rebuilds it; the Kotlin plugin's methods stay expanded after
they are decompiled, as in Java, and so does a method whose decompilation threw), and the class
nodes drop their wrappers and an anonymous class's kept super call when a decompilation ends
(`decompiler/managed/kuna-jvm/src/p1_classfile/classes_processor.rs (ClassesProcessor::end_class_scope)`).
A recogniser that took the stale sequence of a constructor accessor for an expanded one failed
the method it was decompiling (`TimeMark.DefaultImpls.plus-LRDsOJo` of kotlin-stdlib, with
"stale FullInstructionSequence handle", whenever the batch holding `AdjustedTimeMark` had run
on the same engine first).
Reading is lazy where no output can tell (kuna). Unless a pool interceptor is set
(renaming), a class is built from its header, interfaces and class attributes only: its
constant pool records where each entry lies and reads an entry the first time it is
asked for, resolving the entries it names first, and its fields and methods are read on
the first `StructClass::get_fields`/`get_methods`
(`decompiler/managed/kuna-jvm/src/p1_classfile/constant_pool.rs (ConstantPool::new)`).
Resolving on demand gives the values of Java's two resolution passes whenever every
reference points at an entry of the kind the pass expects and every method descriptor a
link constant checks is well formed; a construction-time scan checks exactly that and
reads any other pool eagerly, so a malformed pool fails where Java's does. Laziness
changes only the creation order, hence the identity hashes, of member and attribute
objects, and those hash only into sets the engine tests for membership. Class bytes are
shared with the input, not copied. A member whose content (not its framing) is malformed
fails when its class is first used rather than when the input is added.
`jvm_decompile_generics` decides whether `Signature` attributes are parsed into generic
descriptors (`decompiler/managed/kuna-jvm/src/p5_types/generic_main.rs`); off, every
type prints erased. Debug attributes feed naming later: the `LocalVariableTable` and
`LocalVariableTypeTable` (`jvm_use_lvt_names`) and `MethodParameters`.

## 11.4 Bytecode to a control-flow graph

**Decode.** `StructMethod::expand_data` decodes the code attribute into an instruction
sequence with its exception table
(`decompiler/managed/kuna-jvm/src/p2_lift/full_instruction_sequence.rs`,
`decompiler/managed/kuna-jvm/src/p2_lift/instruction.rs`), one instruction per opcode
with resolved operands; the comment of a failed method later lists this sequence. The method's local-variable count seeds the variable counter, so every
variable the decompiler invents is numbered after the method's own locals.

**Blocks.** `decompiler/managed/kuna-jvm/src/p2_lift/control_flow_graph.rs (ControlFlowGraph::new)`
splits the sequence at leaders (jump and switch targets, the instruction after a jump,
a return or a switch, handler starts and the bounds of every protected range), links
the blocks by their regular successors, gives each block of a protected range an
exception edge to its handler, and links `jsr` subroutines. The first pass then removes
every block not reachable from the entry over regular or exception edges
(`decompiler/managed/kuna-jvm/src/p2_lift/dead_code_helper.rs (DeadCodeHelper::remove_dead_blocks)`).

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
`decompiler/managed/kuna-jvm/src/p2_lift/exception_deobfuscator.rs (ExceptionDeobfuscator)`
and `DeadCodeHelper`:

1. `remove_circular_ranges` - a range whose handler lies inside the range it protects
   (an obfuscator pattern) loses the blocks the handler reaches by itself.
2. `restore_pop_ranges` - javac leaves the first instruction of a handler, the one that
   stores or discards the caught exception (`astore`, `pop`), outside an enclosing range
   that protects the same code. When a range's handler starts that way and another range
   covers its protected blocks, that instruction is split into a block of its own and put
   back into the outer range, so a nested `try` stays inside the outer one. Ranges with
   the same handler and exception types are considered together. Vineflower takes a range
   for the outer one only when it protects more blocks: over the same blocks two handlers
   are as often the `catch` clauses of one `try`. (kuna) The class file decides it when the
   later handler catches a type the earlier one already catches (the same class or a
   subclass): javac rejects that as a second `catch` of one `try` (`exception Exception has
   already been caught`), and compiles `try { try { A } catch (E e) {} } catch (E f) { B }`
   with both ranges over `A` alone, so the later range is the outer one
   (`decompiler/managed/kuna-jvm/src/fixes/exception_ranges/already_caught.rs`). Without the
   classes in the input or the runtime, only the same class is decided.
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

## 11.5 Statement recovery

`decompiler/managed/kuna-jvm/src/p7_regions/dom_helper.rs (DomHelper::parse_graph)`
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
merge an `if` into a `try` inside a loop (`decompiler/managed/kuna-jvm/src/p7_regions/dec_helper.rs
(DecHelper::invalid_head_merge)`), which otherwise builds malformed
loops around `try`/`catch`.

**When the simple case is stuck.** `DomHelper::find_general_statement` carves out a
single-entry subregion bounded by a postdominator of its head and recurses into it: the
candidates come from the region's postdominator tree on the first attempt, then from its
*extended* postdominators
(`decompiler/managed/kuna-jvm/src/p7_regions/fast_extended_postdominance_helper.rs`),
and a candidate is accepted only if it has no entry besides its head, does not cut an
exception range in half, and does not split a `monitorenter` from its successor. The
recursion is the reason the engine can structure nested loops with breaks and continues
that no single-level pattern would match: it solves the inner region first and uses the
result as one node outside.

**Irreducible flow.** If a region still does not reduce,
`decompiler/managed/kuna-jvm/src/p7_regions/irreducible_cfg_deobfuscator.rs` checks
whether it is irreducible and, if so, splits one node and tries again: among the
statements with several predecessors it copies the one with the fewest successors, then
the smallest size times its extra predecessors, and moves one predecessor's edges to the
copy. There are up to five such rounds per region, and the whole procedure is retried once
with a freshly computed postdominance map. A split adds
`$VF: Irreducible bytecode was duplicated to produce valid code`; exhausting the rounds
adds an error comment that the flow was not entirely decomposed. A region that is
reducible but still cannot be decomposed, or whose irreducible node cannot be split, fails
the method (`parsing failure!`, section 11.12).

After the tree is built, `parse_graph` lowers `continue` labels, condenses sequences,
computes the monitor flags and rebuilds `synchronized` blocks:
`DomHelper::build_synchronized` turns a `monitorenter` followed by a catch-all whose
handler exits the monitor into a synchronized statement and marks the duplicated
`monitorexit` instructions dead.

## 11.6 `finally`

javac compiles `try { A } finally { F }` by copying `F` onto every exit of `A` and of
each `catch`, plus a catch-all handler that runs `F` and rethrows.
`decompiler/managed/kuna-jvm/src/p7_regions/finally_processor.rs (FinallyProcessor::iterate_graph)`
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

A handler of one block is an empty `finally` when it only stores and rethrows the
exception: `athrow` alone, or `astore n` followed by a load and an `athrow`. Vineflower
decided that from the number of instructions, so `finally { throw t; }` (`astore_3;
aload_1; athrow`) and `finally { return x; }` (`astore_2; iload_1; ireturn`) were read as
empty: the handler's code was deleted with the rethrow and its copies left in the `try`,
where a `catch (Throwable)` then caught `t`, or an exception escaped that the `finally`'s
`return` swallowed. kuna (`get_finally_information`) also requires that the block's exit
is the rethrow of the stored exception, which the same pass already records per exit block.

The handler's shape is read from the blocks of the catch-all's handler statement, but the
structurer can place the rethrow outside it: in `while (..) { try { .. } catch (..) { .. }
finally { if (c) break; } .. }` the handler's `aload; athrow` came after the loop body.
The pass then saw a handler without a rethrow, matched the copies without one, found no
code to continue at after them and sent the end of the `try` to the method's exit, which
deleted the rest of the loop, and left `throw var3` with its store removed (Vineflower
prints the same). A handler whose stored exception is rethrown from a block outside the
handler statement is now read as inconsistent: it stays a `catch (Throwable)` with its copies
where javac placed them.

The schedule then rebuilds `synchronized` statements once more, removes their exception
handlers (`DomHelper::remove_synchronized_handler`), condenses sequences and clears
structure-only statements
(`decompiler/managed/kuna-jvm/src/p7_regions/clear_struct_helper.rs`).

## 11.7 The stack-to-expression lift

`decompiler/managed/kuna-jvm/src/p2_lift/expr_processor.rs (ExprProcessor::process_statement)`
converts each basic block's instructions into expressions. The tree is flattened into a
direct graph of blocks
(`decompiler/managed/kuna-jvm/src/p3_dataflow/flatten_statements_helper.rs`), and the
operand stack is simulated along it: every value pushed is assigned to a *stack variable*
(numbered from 10000, `VarExprent::STACK_BASE` in
`decompiler/managed/kuna-jvm/src/substrate/var_exprent.rs`), every instruction that
consumes the stack reads those variables, and a block's stack on entry is taken from its
predecessors. `dup` and `swap` families copy variables rather than expressions, and a
catch handler starts with its exception on the stack. The result is correct and
unreadable - `var10000 = System.out; var10001 = "hi"; var10000.println(var10001);` - and
the next stage folds it back. Expression nodes are in
`decompiler/managed/kuna-jvm/src/substrate/exprent.rs` (one kind per Java expression
form: assignment, constant, field, function and operator, invocation, `new`, array,
`instanceof` pattern, switch, exit, monitor, and so on).

When a constructor call names a constructor its class does not declare (invalid or
tampered bytecode), the method fails with `Constructor .. not found`; with
`jvm_ignore_invalid_bytecode` the call is printed without hiding synthetic arguments
instead (`decompiler/managed/kuna-jvm/src/substrate/expr_util.rs`).

## 11.8 SSA, SSAU and stack-variable inlining

The lifted tree goes through a fixed point
(`MethodProcessor::code_to_java`): simplify stack variables, recompute variable versions
(section 11.10), and look for `++`/`--` shapes
(`decompiler/managed/kuna-jvm/src/p3_dataflow/p_pand_mm_helper.rs (PPandMMHelper::find_p_pand_mm)`);
repeat while the last step finds one.

`decompiler/managed/kuna-jvm/src/p3_dataflow/stack_vars_processor.rs (StackVarsProcessor::simplify_stack_vars)`
alternates two views of the variables:

- **SSA** (`decompiler/managed/kuna-jvm/src/p3_dataflow/ssa_constructor_sparse_ex.rs`):
  every assignment of a variable gets its own version. On it,
  `decompiler/managed/kuna-jvm/src/p3_dataflow/simplify_exprents_helper.rs
  (SimplifyExprentsHelper::simplify_stack_vars_statement)` runs peephole rewrites to a
  fixed point: a stack variable assigned and then used once next to it is inlined; array
  creation followed by element stores becomes an array initializer; `i = i + 1` shapes
  become `++`/`--`; `new` followed by the constructor call becomes one constructor
  expression; the two-branch assignment of a ternary becomes `?:`; the javac null check
  `outer.getClass()` before a qualified `outer.new Inner()` is dropped
 ; the pre-Java-5 `class$` idiom is folded into a class literal
  (finished at class level by section 11.11).
- **SSAU** (`decompiler/managed/kuna-jvm/src/p3_dataflow/ssau_constructor_sparse_ex.rs`):
  versions also change at every use, which yields a graph of definitions and uses. On
  the first round it marks effectively-final variables; on every round
  `StackVarsProcessor::iterate_statements` walks the blocks and inlines a variable's
  definition into its use when no intervening write can change the value. Each block is
  swept twice; the second sweep, which also inlines across the boundaries between
  expressions on the operand stack, is what
  turns chains of temporaries back into `a = b = c`, `arr[i++] = v` and chained calls.
  A stack variable's value is copied into a use further than the next statement only when
  it is a constant, a variable or a field read whose evaluation has no other effect
  (kuna: Vineflower also moved a field read whose receiver calls a method, so
  `step().sink.accept(s, count, count)` with an `int` `count` passed as `float` printed
  `float var10002 = this.count;` before the call, reading `count` before `step()` runs;
  `stack_var_inlining::has_effect`).

Both forms are reverted (all versions back to zero) after each use, so each round
re-derives them from the current tree. Then, once: `++`/`--` inside conditions is inlined
(`PPandMMHelper::inline_pp_iand_mmi_if`), but only into a read the condition always
evaluates: not under the right operand of `&&` or `||` or an arm of `?:` (kuna,
`always_evaluated`; Vineflower refused any `&&` or `||` below the condition's top operator and
none at it, so without a LineNumberTable `j -= 1; if (i > 3) { if (j == -2) .. }` became `if
(i > 3 && --j == -2)`, which decrements only when `i > 3`, while `while (--i >= start &&
index == -1)` stays folded); for Java 9+ class files,
`decompiler/managed/kuna-jvm/src/p3_dataflow/concatenation_helper.rs` turns
`invokedynamic` string concatenation back into `+`; and
`decompiler/managed/kuna-jvm/src/p8_structure/assert_processor.rs` rebuilds `assert`
statements from the `$assertionsDisabled` check and the `throw new AssertionError(..)`
that javac emits.

## 11.9 The main structuring loop

The tree now holds correct but generic statements. The main loop of
`MethodProcessor::code_to_java` refines it; each pass returns whether it changed the tree,
and most changes restart the loop from the top, so the order below is a priority order.

1. `LabelHelper::clean_up_edges`
   (`decompiler/managed/kuna-jvm/src/p8_structure/label_helper.rs`) normalizes the
   edges: every `break`/`continue` points at the statement that really closes it.
2. **The loop-merge loop**, only when the method has loops, repeated until nothing
   changes:
   `decompiler/managed/kuna-jvm/src/p8_structure/eliminate_loops_helper.rs` removes loops
   that never loop; `decompiler/managed/kuna-jvm/src/p8_structure/merge_helper.rs
   (MergeHelper::enhance_loops)` turns infinite loops with a leading or trailing exit test
   into `while`, `do`-`while` and `for` loops, and iterator and array loops into for-each;
   `decompiler/managed/kuna-jvm/src/p8_structure/loop_extract_helper.rs` moves code that
   always leaves the loop out of it; and
   `decompiler/managed/kuna-jvm/src/p8_structure/if_helper.rs (IfHelper::merge_all_ifs)`
   merges nested `if`s into `&&`/`||` conditions and builds `if`-`else`. It
   also collapses the nested ifs a ternary inside a condition
   compiles to (`if (a ? b : c)`).
3. Stack variables are simplified and variable versions recomputed again, and
   `LabelHelper::identify_labels` decides which jumps need an explicit label.
4. `decompiler/managed/kuna-jvm/src/p3_dataflow/secondary_functions_helper.rs
   (SecondaryFunctionsHelper::identify_secondary_functions)` rewrites operator shapes:
   an `lcmp`/`fcmpl`/`dcmpg` result compared with zero becomes a direct comparison (one
   used as a value is spelled out as a ternary), `c ? true : false` and `c ? false : true`
   fold to `c` and `!c`, negations are pushed into comparisons, and an `if`-`else` with an
   empty `if` branch is inverted. The Kotlin plugin extends the folding to every ternary
   with one boolean constant branch (`c ? true : x` to `c || x`).
5. `decompiler/managed/kuna-jvm/src/p5_types/intersection_cast_processor.rs` builds
   intersection casts (`(A & B) x`).
6. With `jvm_pattern_matching`, on Java 16+ class files,
   `decompiler/managed/kuna-jvm/src/p8_structure/if_pattern_match_processor.rs
   (IfPatternMatchProcessor::match_instanceof)` turns `instanceof` followed by a cast into
   an `instanceof` pattern, record patterns included. (kuna) Only from javac's compiled
   pattern (fix `pattern_binding_shape`, section 11.16), and not where the line table shows
   the cast as a statement (source patterns, section 11.10). (kuna) A record component matched
   by a nested type test (`R(String s)` for `record R(Object o)`: javac tests the accessor's
   value with `instanceof String`, and on failure goes on to the next case) is bound with the
   tested type, not the component's: without a LocalVariableTable, or for an unnamed `_`,
   the binding had the type `Object`, so the pattern printed as `R(Object var4)`, which
   matches every `R`. (kuna) A copy of a record pattern keeps those tested types
   (`decompiler/managed/kuna-jvm/src/substrate/pattern_exprent.rs (PatternExprent::copy)`);
   Vineflower's copy dropped them, and the passes that copy a condition (joining ifs into
   `&&`, a returned condition) turned `return o instanceof Box(Integer i) && i > 0` into
   `Box(Object var1) && var1 > 0`.
7. When the method has switches:
   `decompiler/managed/kuna-jvm/src/p8_structure/switch_pattern_match_processor.rs`
   rebuilds pattern switches from `SwitchBootstraps.typeSwitch` dispatch (also gated by
   `jvm_pattern_matching`), and
   `decompiler/managed/kuna-jvm/src/p8_structure/switch_expression_helper.rs` rebuilds
   switch expressions with `->` arms and `yield` from the statement switch and temporary
   javac emits (`jvm_decompile_switch_expressions`).
8. When the method has a `try`:
   `decompiler/managed/kuna-jvm/src/p8_structure/try_helper.rs (TryHelper::enhance_try_stats)`
   rebuilds try-with-resources
   (`decompiler/managed/kuna-jvm/src/p8_structure/try_with_resources_processor.rs`; the
   Java 11+ shape from a plain `try`/`catch`, the older one from a `finally`), then folds
   a try-with-resources that is the whole body of another one into that statement's
   resource list (`try (A a = ..; B b = ..)`).
9. `decompiler/managed/kuna-jvm/src/p8_structure/inline_single_block_helper.rs` moves a
   block reached only by a labeled `break` to where the break is, which removes most of
   the labels the initial recovery left.
10. `MergeHelper::make_do_while_loops` and `MergeHelper::condense_infinite_loops_with_return`
    finish loop shapes.
11. Outside static initializers,
    `decompiler/managed/kuna-jvm/src/p8_structure/exit_helper.rs (ExitHelper::condense_exits)`
    gives an `if` whose branch jumps to a shared `return` or `throw` block a copy of that
    exit as its body, and removes the blocks this leaves unreachable.

When a full pass of the loop changes nothing, the tail runs once, in this order:

- `decompiler/managed/kuna-jvm/src/p8_structure/switch_helper.rs (SwitchHelper::simplify_switches)`
  turns the javac lowering of an enum switch (a switch over a synthetic `$SwitchMap$`
  array, read from the `<clinit>` of the class that holds it) back into a switch over the
  enum constants, and the lowering of a string switch (a switch over `hashCode()` followed
  by `equals` tests into a second switch) back into one switch over the strings; the
  helper statements this leaves are marked hidden and are not written. If that produced a
  switch expression, switch expressions, stack
  variables and versions are processed once more. (kuna) javac compiles `switch (e) {}` on
  an enum of another class to a read of the switch map at `e.ordinal()` whose value it
  discards, with no switch instruction, which Vineflower wrote as
  `int var10000 = <unrepresentable>.$SwitchMap$..[e.ordinal()];`; such a read
  (`empty_enum_switches`: a stack variable nothing reads, set from an array of a
  `$SwitchMap` field indexed by `ordinal()`) becomes the head of a switch on `e` in the
  block's expression list, and `ExprProcessor::list_to_java` writes a switch head there
  with an empty body: `switch (e) {` `}`, which javac compiles back to the same read.
- `ExitHelper::adjust_return_type` and `ExitHelper::remove_redundant_returns` (a trailing
  `return;` of a `void` method is dropped).
- secondary functions once more, then
  `decompiler/managed/kuna-jvm/src/p8_structure/synchronized_helper.rs` folds the
  temporary that holds a `synchronized` block's monitor into `synchronized (expr)`, and
  assigns a monitor that is a non-string constant to a fresh variable first
  (`$VF: Added assignment to ensure synchronized validity`), since `synchronized` on a
  literal is not valid Java.
- variable definitions are placed (section 11.10), compound assignments (`x += y`) are
  formed (`SecondaryFunctionsHelper::update_assignments`), and empty `default` arms are
  hidden.
- `decompiler/managed/kuna-jvm/src/p5_types/generics_processor.rs (GenericsProcessor::qualify_chains)`
  qualifies generic call chains, and
  `decompiler/managed/kuna-jvm/src/p2_lift/expr_processor.rs (ExprProcessor::canonicalize_casts)`
  collapses a cast of a cast to the same type.
- `LabelHelper::replace_continue_with_break`, and finally
  `ExprProcessor::mark_expr_oddities`, which does not change the tree but adds an error
  comment for every shape that means the result is not trustworthy: an invalid label, a
  leftover `monitorenter`, an `if` destroyed into an expression, a variable whose type
  could not be determined, an `assert` that could not be resugared.

Two more passes run when the class is written, before any text is produced
(`decompiler/managed/kuna-jvm/src/p9_emit/class_writer.rs (ClassWriter::invoke_processors)`):
switch simplification once more (`FinalSimplifySwitches`) and the removal of an `else` after a branch that always returns or throws
(`IfHelper::prettify_ifs`).

**javac's block layout (kuna).** The `if` and exit structuring of a method javac compiled (its
class carries no `kotlin.Metadata` and names no `.kt` source file, the one test of
`decompiler/managed/kuna-jvm/src/p6_variables/kuna_source_locals.rs (kotlinc_class)` that the
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
`decompiler/managed/kuna-jvm/src/p8_structure/kuna_javac_layout.rs (reorder)` makes the
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
`decompiler/managed/kuna-jvm/src/p3_dataflow/kuna_javac_boolean_compare.rs (keeps)` keeps
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
then-branch the source wrote as a jump, `separate_test`; not when the condition ends with an
`instanceof` pattern's binding, whose `goto` goes over the then-branch or, in `a || b`, to the
code the condition guards: `while (o instanceof String x || o.hashCode() < 0)` printed as a
`while (true)` around nested ifs, which javac compiles to other code), or the first is the head of an
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
`decompiler/managed/kuna-jvm/src/p8_structure/kuna_javac_layout.rs (keeps_tail)` keeps it after
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

**An else-branch that held a loop's update (kuna).** `MergeHelper::match_for` takes a loop's
update from the last statement of its body, which can end the else-branch of an if-else that ends
the body (`if (a[i] != 0) { .. } else { i++; }`, as the rules above make it);
`remove_last_empty_statement` then removed the emptied branch but left the statement an if-else,
and `IfNode` failed on it (`called Option::unwrap() on a None value`, the one method kuna failed
on the second held-out benchmark).
`decompiler/managed/kuna-jvm/src/p8_structure/merge_helper.rs (MergeHelper::drop_else_edge)`
makes it an if: the head's edge into the branch, which now continues the loop, leaves the if, or
is dropped when the if already has a successor, which goes where the branch went.

**The increment of a for-each over an array (kuna).** `MergeHelper::match_for_each` makes a
for-each of `T[] c = a; int n = c.length; int i = 0; while (i < n) { T x = c[i]; ..; i++; }` and
deletes the `i++` with the other parts. Vineflower took that increment from the last statement in
the body's statement order and checked its operand only when it was a variable: inside a `switch`,
whose cases are not kept in flow order, it took the `a[0][0]++` of a `catch` block for the
increment and deleted it, leaving the real `i++` behind undeclared; and without a
LocalVariableTable it took the `i++` of `if (c) { i++; }`, so a loop that stays on one element
while `c` is false iterated over all of them. The increment is now the last statement of the one
block every `continue` edge of the loop leaves from, so it runs before every jump back, and it
must be `i++` or `++i` on the index variable itself; otherwise the loop stays a `while`.

**An exit copied into a `try` (kuna).** `ExitHelper::integrate_exits` replaces a `break` to a
block that holds only a `return` or `throw` with a copy of that exit, at the place of the `break`.
When the `break` leaves a `try`, a `finally` or a `synchronized` body, the copy lands in it, which
Vineflower did not check: `for (..) { try { switch (..) { .. } break; } catch (Exception e) {} }
return Arrays.asList(ret);` printed the `return` in both cases of the switch, where the `catch`
covers `Arrays.asList` and continues the loop when it throws, and a `return n` copied into a `try`
whose `finally` assigns `n` returned the value from before the `finally`.
`decompiler/managed/kuna-jvm/src/p8_structure/exit_helper.rs (ExitHelper::moves_into_protected)`
walks from the place of the copy to the statement the jump leaves and refuses the copy when it
passes a protected body the exit could run differently in: a `try` with a `catch` or a
`synchronized` body unless the exit is a `return` of nothing, a constant or a variable (which
cannot throw), and the `try` of a `finally` unless it returns nothing, a constant, or a local
the `finally` does not assign (the `finally` runs after the value is computed, and nothing
else can change a local). The `break` then stays, labelled where it leaves an outer statement.

**ECJ's layout (kuna).** In a class the Eclipse compiler wrote (section 11.3) the rules above
read ECJ's code, which places an `if`'s branches, its exits and its `continue` and `break`
jumps as javac does, with two differences.
*A returned condition*: ECJ compiles `return x <= y;` (a comparison, `&&` or `||`) as the
condition's jumps, `iconst_1; ireturn` where it holds and `iconst_0; ireturn` at their target
(or the reverse), where javac pushes the value and returns once, and the source's `if (c)
return true; return false;` is the same code. ECJ starts a LineNumberTable entry at each
statement on a line of its own and none at the second return of a returned condition, so
`decompiler/managed/kuna-jvm/src/p8_structure/kuna_javac_layout.rs (ecj_returned_condition)`
takes two returns of boolean constants for one statement when no entry starts at the second:
`reorder` makes them the if-else Vineflower makes and `same_return` lets `build_iff` merge it
(`return c ? true : false`, which secondary functions fold to `return c;`); with an entry they
stay two statements, and without a LineNumberTable the code is either form and Vineflower's one
return stands. The block of the merged return holds no instructions, so an `if` around it reads
where its body starts from the instructions of its expressions (`ecj_merged_return`, which knows
the merged return by the conditional jump of its condition).
*A switch on a string*: ECJ switches once on the string's `hashCode()`, each case testing
`equals` and jumping to the code of its case, which ECJ places after all the tests, so the last
case's test is followed by the first case's code, the shape of javac's `if (c) break;`.
`ecj_string_switch` (the switch instruction follows `invokevirtual String.hashCode()`) puts such a
switch with the pattern switches, inside which no rule applies, and `reorder_last` leaves its
ifs: the `if (!equals) break;` it made of a case was then turned by `ExitHelper.integrate_exits`
(written for this switch) into a return that dropped the other cases' returns, which is wrong code.
Each other decision above was measured off for ECJ's classes alone, one at a time, on the ECJ
project of the second held-out benchmark, and none was better off.

## 11.10 Variables, types, declarations and names

**Versions and types.** `decompiler/managed/kuna-jvm/src/p6_variables/var_processor.rs
(VarProcessor::set_var_versions)` runs
`decompiler/managed/kuna-jvm/src/p6_variables/var_versions_processor.rs
(VarVersionsProcessor::set_var_versions)` each time the schedule asks for it: an SSA split,
a merge of the versions a phi joins, type inference, and a renumbering that gives each
remaining version its own variable index. Type inference is
`decompiler/managed/kuna-jvm/src/p5_types/var_type_processor.rs (VarTypeProcessor::calculate_var_types)`:
each variable version gets a lower and an upper bound in the type lattice of
`decompiler/managed/kuna-jvm/src/substrate/var_type.rs`, seeded from the method
descriptor and from every expression that defines or uses it, and the bounds are
propagated over the direct graph to a fixed point. Integer constants start at the
narrowest type that can hold them; the constant `1` (not only
`0`) may also become `boolean`, so flags stored with `iconst_1`/`iconst_0` print as
`true`/`false`. Before renumbering, the lattice's internal types become Java types: an
integer-constant type becomes `byte`, `short` or `char`, and the null type `Object`.

**Constant locals are `int` (kuna).** A JVM slot holds an `int`, and the narrow type a
constant fits says nothing about the source's declaration, so without a LocalVariableTable
`byte type = 15;` printed for `int type = DOUBLE;`. In a method without one,
`constant_local_int` (`decompiler/managed/kuna-jvm/src/fixes/numeric_types/constant_local_int.rs`)
looks at each slot all of whose stores are integer constants (parameters excluded) and at
every read of it: a read as an operand, a condition, an index or an argument, store or
return of `int` or wider accepts `int`; an argument, field or array store, return or array
element of `byte`, `short` or `char` requires that type. An argument is also a value a
creation passes to its constructor and a value a lambda or a local or anonymous class
captures, whose type is the parameter's in the constructor's or the `invokedynamic` call
site's descriptor: `int count = 7; return () -> count;` printed `byte var0 = 7;`, and the
recompiled lambda boxed a `Byte` where the class file boxes an `Integer`. When the reads
accept `int` the slot's variables are `int`; when they also require one narrow type that every stored constant
fits, they have that type (`char c = 'q'` for a `char` field store, which kept a cast
before); reads that require two types, or a read in any other context (a conditional, a
string concatenation, a copy to another variable), leave Vineflower's types. A slot an
increment or decrement writes (`b++` on a `byte` is `iadd`, `i2b`, `istore`, folded into the
expression before types are computed) is not constant-only and keeps its narrow type:
`int b = 0; take(b++)` would not select `take(byte)`.

**A local that only holds `null` (kuna).** Vineflower types a variable whose every value is
`null` as `Object` (`eliminateNonJavaTypes`), so without a LocalVariableTable `Source s =
null; take(s);` printed `Object var0 = null; take((Source)var0);`. Such a variable now takes
its upper bound when that is a class or array type: the type an argument, a store or a return
of it requires, which the descriptors in the class file give. A variable that a reference cast
reads keeps `Object`: javac writes a `checkcast` only where the value's type is not already
the target's, so `Object o = null; accept((Integer) o)` must keep both. The receiver of a
call and the instance of a field access whose value is `null` are bounded by the class the
instruction names (`InvocationExprent::check_expr_type_bounds`,
`FieldExprent::check_expr_type_bounds`), and a `null` value stored into a field by the
field's type (`AssignmentExprent::check_expr_type_bounds`): Vineflower bounded a receiver
by its own type, which for a `null` local is the null type and erased every other bound, so
`Cursor c = null; c.read()` printed `Object var0 = null; ((Cursor)var0).read()` and
`int[] v = null; v.clone()` printed `var0.clone()` on an `Object`, which does not compile.
The verifier checks the class an `invokevirtual`, `getfield` or `putfield` names; it checks
an interface owner (`invokeinterface`, an interface-typed field) only loosely, so such a
bound is evidence from javac, which names the receiver's compile-time type (JLS 13.1); a
`null` local has no other type the call could compile with. The receiver of `arraylength`
names no element type and keeps the `Object[]` cast.

**A `var` local of an anonymous class (kuna).** An anonymous class creation has the type of
the class it extends or implements (`anonymous_class_type`), so a local assigned one is
declared with that type. Java 10's `var` declares the local with the anonymous class
itself, and its members are then reachable through it: `var c = new Object() { int n; void
bump() {..} }; c.bump();`, which javac compiles to an `invokevirtual` naming the anonymous
class. Without a LocalVariableTable that printed `Object var0 = ..; var0.bump();` and
`((<unrepresentable>)var0).n`, neither of which compiles. In a class file of version 54 or
later, `VarVersionsProcessor::type_anonymous_receivers` gives a variable that a call or a
field access reads as an instance of an anonymous class (the class the instruction names
is one) that class as its type, which prints as `var`; such a variable can hold no other
value, since no other type could be the call's qualifying type (JLS 13.1). Only a member
the anonymous class declares and no supertype does counts (`declared_above`; a supertype
that is not known counts as declaring it, `Object` by its fixed members): a member it
inherits or overrides is reachable through the supertype the local already has. The class
file version does not tell a DEX-derived class (kuna-dex raises a nest member to 55), where
d8 holds `new TypeVisitor() {..}.visit(from)` in a register: `var` there printed text the
project's Java 8 does not compile (guava's `TypeResolver.populateTypeMappings`, release
DEX).

**An `int` conditional assigned to a local (kuna).** Vineflower lowers the `int` type of a
conditional on the right of an assignment to `bytechar`, so that `byte v = b ? 0 : 1` keeps its
narrow variable: the arms are constants and the conditional is `int` only by their default.
The lowering applies only when every arm, through nested conditionals, is a constant
(`ternary_numeric_arms::constant_arms`). An arm that is a value, such as an `int` variable,
gives the conditional the type of what it can yield: without a LocalVariableTable,
`int result = logical != 0 ? logical : sum` after a loop printed `byte var4 = (byte)(..)`,
truncating a result the class file stores and returns with no `i2b`.

**A compound assignment to an `int` local (kuna).** javac stores an integer operation's
result into a `byte`, `short` or `char` only through `i2b`, `i2s` or `i2c`, and `x op= y` on
such a local keeps that conversion; on an `int` it has none. Folding `x = x op y` into
`x op= y` (`SecondaryFunctionsHelper`) drops the store's shape, so without a
LocalVariableTable a local initialised with a small constant and then only updated by
compound assignments was typed by the constant: `int res = 0; ... res += sz;` printed `byte
var2 = 0; ... var2 += var3;`, which truncates. The fold now marks an assignment it makes
from an operation stored with no narrowing conversion (`AssignmentExprent::mark_int_result`),
and the type pass gives such a variable `int` as its lower bound.

**Generic locals without a LocalVariableTable (kuna).** Without the table a local has its
erased type, so `Set keys = settings.keySet(); keys.removeIf(k -> k.startsWith(p))` did not
compile (the lambda's parameter is an `Object`) and a for-each over a raw `List` cast it.
Before a Java class is written,
`decompiler/managed/kuna-jvm/src/fixes/generic_casts/initializer_local_type.rs` declares a
local of a generic class with the type of its value when the method has no
LocalVariableTable, the local is assigned exactly once and is no parameter, that value's
inferred type is a parameterization of the local's class whose arguments are all class
types (no wildcard or type variable, to any depth), and every read is either the receiver
of a call whose arguments the class's type variables do not type (a lambda or method
reference may take one) or the iterable of a for-each whose variable's class the element
type extends. Any other read (an argument, a store, a capture by a lambda, a call such as
`add(x)` whose argument the type arguments would constrain) keeps the erased type, since
the source may have declared it raw; the class file cannot tell, but the declared type is
the value's own, so every accepted read compiles as it did. The type is the local's in
every inference that reads it (`raw_receiver_erasure` included). A local (not compounded)
every value of which has one type variable's type (or is `null`, the local itself or a
conditional of such), and the variable of a for-each whose iterable's element type is a
type variable, is declared with that variable when it is in scope (the method's or, for an
instance method, the class's) and erases to the local's type: `for (N item : source)
map.put(item.toString(), item)` for a `List<N> source` and a `Map<String, N> map` printed
`for (Number var3 : var1)`, which does not compile. A variable of a type variable accepts
every read its erasure did, its bound's members and conversions included, so no read is
checked. A name that both the method and its class declare is not used (the class's
variable is hidden there, and a value of its type is not of the type the name denotes), nor,
in a lambda's body, a name that any method of the class declares, since the body's method
does not say which method encloses it. A value's type counts only where declarations fix it
(`generic_type`): a parameter's from the method's signature, `this`, an element of such an
array, a field's (`field_type`: its signature with the type variables of the class that
declares it replaced by the arguments the type of its instance gives that class; none when
the instance is of a raw or undeclared type, whose field has the erased type (JLS 4.8), or
when the signature names a variable of an enclosing class: `Object var1 = this.raw.value`
and `Object var3 = box.value` for a `Box<String> box` stay as they are, where the name `T`
of `Box<T>`'s variable would otherwise be read as the reading class's `T`), a variable this
pass typed, and a call on one of these whose result is a type
variable of the method's class, given by the receiver's type arguments, when no argument
is of a raw type for a parameter with type arguments (that unchecked conversion erases the
result, JLS 15.12.2.6: `V v = this.operate(k, v, rawDecisionMaker)` does not compile);
javac's `checkcast` of such a value to its erasure is a value of the variable
(`U u = (U)io.mutate(t)`).
The pass, like every fix that asks which method a call names before the call is written,
looks the method up without recording it (`InvocationExprent::find_desc`). The ported
`get_desc` records what it finds, and a call with a recorded method is written without the
inference of its type arguments (`append_param_list` runs it only for a call that has
none), so asking through it changed what a call whose value nothing uses printed: in a
method without a LocalVariableTable, `elements.accept(x, e -> ..)` on a captured receiver
came out as `elements.accept(x, (Consumer<Object>) e -> ..)`, which does not compile
against the receiver's `Consumer<E>`.

**Declarations.** `VarProcessor::set_var_definitions` runs
`decompiler/managed/kuna-jvm/src/p6_variables/var_definition_helper.rs
(VarDefinitionHelper::set_var_definitions)`: each variable is declared at the innermost
statement that contains all its uses, folded into its first assignment where that
assignment comes first there; variables of the same slot whose lives do not overlap are
merged into one where the merge cannot change what a read sees; and local types from the
`LocalVariableTypeTable` are applied. `jvm_verify_merges` and `jvm_verify_pre_post_merges`
are experimental, stricter merge checks: the second computes, with an extra SSA pass,
which assignments each read can see, and refuses any merge that would change it.
(kuna) `populate_type_bounds`, which bounds a variable a method returns by the class the method
returns, read a method returning an array as returning the class its element descriptor names:
`byte[]` gave a class `B`, which the merge of `bytes = null` and `bytes = s.getBytes(..)` then
took for their type (`B var2 = null;` in a method without a LocalVariableTable); a returned array
bounds nothing.

**Source locals (kuna).**
`decompiler/managed/kuna-jvm/src/p6_variables/kuna_source_locals.rs`
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
and `decompiler/managed/kuna-jvm/src/p8_structure/merge_helper.rs
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
`decompiler/managed/kuna-jvm/src/p6_variables/kuna_source_scopes.rs (hoist)` walks the
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
A merged variable carries one table entry on every occurrence, its target's or else the merged
declaration's: a variable gets its entry from the store the entry starts after, so in a loop
(`long acc; do { acc = ..; acc ^= ..; } while (acc < 0);`) only the first store's version has it,
and the version declared before the loop, which the first store's declaration is merged into,
printed `var7` beside an undeclared `acc` (and `var7 ^= ..` read the unset `var7`).
A variable none of whose instructions an entry covers takes the name of an entry that starts
right after one of its stores (the offsets an occurrence records can lie up to four bytes before
its store instruction), unless another variable of the slot stores in between: javac starts a
local's entry after the store that declares it and writes no entry for a local never read, so in
`{ int[] counts = new int[n]; } String text = "n=";` the entry `text` starts after the second
store, and the unread array stays a variable of its own instead of being merged into `text`
(`int[] text = new int[n]; text = "n=";`, which javac rejects).
`order_declarations` moves only variables the table names. `VarDefinitionHelper::merge_check`
refuses to merge two variables the table names differently (`kuna_source_locals::named_apart`,
`properties` and a later block's `subFields`). Increments
(`decompiler/managed/kuna-jvm/src/p3_dataflow/kuna_source_increments.rs`): javac emits a
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
(`decompiler/managed/kuna-jvm/src/p8_structure/kuna_source_patterns.rs (keeps_cast)`), in a
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
(`decompiler/managed/kuna-jvm/src/p1_classfile/class_wrapper.rs (ClassWrapper::init)`),
variables get `varN`-style names that avoid the class's field names
(`decompiler/managed/kuna-jvm/src/p6_variables/var_names_collector.rs`), then, with
`jvm_use_lvt_names`, the debug names from the `LocalVariableTable` replace them wherever
the name is a valid identifier (`VarProcessor::set_debug_var_names`). With the same
option, a `MethodParameters` attribute names (and marks `final`) the parameters. A method without code still gets parameter names. A parameter the header declares by its
`MethodParameters` name keeps that name where the other variables are renamed away from
field names and from the names of the methods around a nested body
(`decompiler/managed/kuna-jvm/src/fixes/nested_class_vars/mod.rs`, `header_parameter_names`):
javac's default (`-parameters` without `-g`) writes the attribute and no variable table,
and the header and the reads must agree (`this.items = items`, not `itemsx`). A static
field of the class being written is printed qualified when the method's variable table
has its name or a variable of the method has it, and inside a lambda body also when a
variable of a method around it has it, captured or not
(`FieldExprent::is_ambiguous`): `() -> label` would read the parameter `label` of the
method around it, not the field. Names that clash within a
scope - two variables, a variable and a captured variable, a variable and a field it
shadows - are renamed apart (`VarDefinitionHelper::remap_clashing_names`), once at
processing time and again at writing time (`VarProcessor::rerun_clashing`), when nested
classes and lambdas have been processed. A parameter or lambda argument whose type stays
undetermined prints as `Object`.

## 11.11 Class-level processing

Processing a root class (`ClassesProcessor::process_class`) runs, in order:

1. **Lambdas.** `decompiler/managed/kuna-jvm/src/p4_calls/lambda_processor.rs
   (LambdaProcessor::process_class)` reads the `BootstrapMethods` of every class of the
   tree (Java 8+) and gives each `invokedynamic` to `LambdaMetafactory.metafactory` or
   `altMetafactory` a lambda class node, which is written later as a lambda or a method
   reference. (kuna) `altMetafactory` also takes flags: `FLAG_SERIALIZABLE` and
   `FLAG_MARKERS` add `Serializable` and marker interfaces to the lambda, which the source
   wrote as an intersection cast (`(Comparator & Serializable) (a, b) -> 1`). The node
   records the interfaces its functional interface is not known to extend
   (`lambdas::added_interfaces`), and the lambda is written under the intersection cast,
   naming its inferred parameterized type, or the interface itself when that is known not to
   be generic; otherwise it is written as before. Without the cast the recompiled lambda is
   not serializable. A lambda or method reference that is the receiver of a call has no
   target type in Java, so it is written cast to the class of the invoked method
   (`lambdas::receiver_cast`): `((Func) () -> 1).get()` had printed `(() -> 1).get()`.
   The cast names the parameterized type the lambda was inferred to have when there is one
   (`lambdas::receiver_cast_type`), because the call's result is typed by it and its own
   cast is dropped: `String s = ((Supplier<String>) () -> "x").get()` under the raw
   `(Supplier)` did not compile wherever the interface's signature is known (always, with a
   Java runtime found). An inferred type that names a type variable out of scope is not
   stored on the lambda, and the cast stays the raw class. (kuna) Under a raw cast, or as
   the value of a raw local without a LocalVariableTable, a lambda's parameters are typed
   `Object` and its body may not compile (`((Mapper) (s -> s.get())).apply(source)`,
   `Function f = s -> s.length() + 1`). The `invokedynamic`'s instantiated method type is
   the erasure of the function type of the lambda's target (JLS 15.27.3), so
   `lambdas::instantiated_parameterization` gives each type variable of the interface that
   is a parameter or the return of its method's signature the instantiated type at that
   place: `Mapper<Source, Object>`, `Function<String, Object>`. The lambda's body is written
   from its implementation method, whose parameters have exactly these erased types, so it
   compiles against them, and the parameterization's method erases to the descriptor the
   call site names. It is used only when every type variable is determined and a parameter
   is more specific than `Object` (where the raw type would serve), for the receiver cast
   when no inferred type is stored, and by `initializer_local_type` as the type of a lambda
   value. (kuna) Vineflower writes a lambda's parameters without types, so a lambda whose
   target leaves a parameter's type open printed text whose parameter is an `Object`:
   `g(s -> s.length())` for `g(Function<?, ?>)` (the wildcard's bound, JLS 9.9),
   `h(s -> s.length())` for `<T> h(Function<T, ?>)` (an implicitly typed lambda does not take
   part in inferring `T`), and with a LocalVariableTable `Function<?, ?> f = s -> s.length()`.
   `explicit_lambda_parameters` marks a lambda passed to a call or assigned to a variable or
   field whose target type gives a parameter that is a type variable of the interface
   another type than the instantiated one, through a wildcard or a type variable of the
   called method that neither another parameter nor the result mentions; the lambda is
   then written with every parameter declared with its implementation method's type
   (`(String s) -> s.length()`), which inference from explicit parameter types (JLS 18.5.3)
   accepts for these targets. A parameter whose target type argument is a class with type
   arguments, or whose type in the interface method is generic, leaves the lambda as it
   was: a declared type must equal the target's exactly, and the class file holds only its
   erasure.
2. **Methods.** Every class of the tree gets a wrapper whose `init` decompiles each of its
   methods with the schedule of sections 11.4-11.10. Synthetic classes that hold only enum
   switch maps are initialized first, so the switches of their users can be read back.
3. **Nested classes.** `decompiler/managed/kuna-jvm/src/p8_structure/nested_class_processor.rs
   (NestedClassProcessor::process_class)` finds, for each anonymous class, local class
   and lambda, the enclosing variables it captures (the `val$x` fields and synthetic
   constructor parameters javac adds), rewrites their uses into uses of the enclosing
   method's variables, and places each local class's definition in the statement where it
   is first used. `jvm_synthetic_not_set` recognizes compiler-generated members (outer-instance
   and captured-variable fields, accessors, `class$` caches) by their shape even without
   the `ACC_SYNTHETIC` flag, which some compilers and obfuscators omit. A captured variable
   is declared `final`, except (kuna, `nested_class_vars::unmark_constant_capture`) in a
   Java 8+ class when its initializer is a constant and its type a primitive or `String`:
   `final String s = ""` is a constant variable, which javac inlines rather than captures,
   so the recompiled class lost the `val$s` field and its constructor parameter; the
   capture in the class file proves the local was not one.
4. **Synthetic accessors.** `decompiler/managed/kuna-jvm/src/p4_calls/nested_member_access.rs
   (NestedMemberAccess::propagate_member_access)` replaces calls to the `access$NNN`
   methods javac generates for private members of nested classes with the field access or
   call they stand for.

Writing a root class (`ClassesProcessor::write_class`) first runs, for each class of the
tree, the late method passes of section 11.9 and then the class-level rewrites of
`ClassWriter::invoke_processors`:
`decompiler/managed/kuna-jvm/src/p8_structure/initializer_processor.rs` moves field
initializers out of `<clinit>` and the constructors into the field declarations and drops
a constructor's leading argument-less `super()`;
`decompiler/managed/kuna-jvm/src/p4_calls/class_reference14_processor.rs` turns the
Java 1.4 class-literal cache into `X.class`;
`decompiler/managed/kuna-jvm/src/p8_structure/enum_processor.rs` removes `$VALUES`,
`values()`, `valueOf()` and the hidden name and ordinal constructor parameters; and
`decompiler/managed/kuna-jvm/src/p8_structure/record_helper.rs` restores a record's
canonical constructor and hides the accessors, `equals`, `hashCode` and `toString` that
match the generated ones. A failure here writes the class as `$VF: Couldn't be decompiled`
(section 11.12).

**Static initializers keep their order (kuna).** Field initializers and `static` blocks run
in the order the source declares them, and `<clinit>` runs them in that order. The class is
written with its fields, in class-file order, before the `static` block that holds what was
not moved, so a moved assignment runs before every statement that stays in the block and
before the moved assignments to the fields declared after its own. Vineflower moved any
assignment whose value reads no local: past a call (`static { init(); b = a + 1; }` became
`static int b = a + 1;`, run before `init()` set `a`), past an assignment that stayed
(`sizes = new int[names.length]` ahead of `names = {..}`, which then read `null`), and ahead
of an earlier one to a later field (`next = base + 1; base = 5;` moved as `base = 5` first).
`extract_static_initializers` now moves an assignment of the first block only while every
statement before it moved and its field is declared after every field moved so far, so the
written order is the order of `<clinit>`; the rest stays in the `static` block, a final field
there being assigned once in it. Synthetic fields (`$assertionsDisabled`, an enum's
`$VALUES`) are not written, so their position does not count, and an enum's constants are
written first whatever their field index (a class translated from DEX lists its fields by
name), as `<clinit>` builds them first: they always move and do not bound the other fields. An interface, which cannot
hold a `static` block, and a class written as Kotlin, whose writer places an object's or
companion's instance and the properties that read it itself, keep Vineflower's rule. The
first block is followed through switch expressions (`straight_line_blocks`): a switch
expression leaves its backing switch in the tree, phantom, writing only its head block, so
`<clinit>`'s straight-line code continues in the first block of the statement after it;
Vineflower read only the first block, so enum constants built with a switch expression as an
argument stayed in a `static` block (`$VF: Failed to inline enum fields`, which javac rejects)
and so did every static field after such an initializer.

**A compound assignment is no initializer (kuna).** `count += 2` in `<clinit>` is an assignment
whose right side is the `2` alone, and Vineflower moved it into the declaration as
`static int count = 2;`, dropping the read of `count`: after an initializer that calls a method
setting `count` to 10, the field ends at 2 instead of 12. `extract_static_initializers` moves
only a plain assignment; a compound one stays a statement. The constructors had the same defect:
an instance block `{ count += 2; }` after `int calls = this.bump();` became `int count = 2;`, and
`extract_dynamic_initializers` too moves only a plain assignment. It takes the initializers
from the front of the statements every constructor that calls `super(..)` shares, one at a
time, and stops at the first it does not move, so no statement kept in a constructor runs
before an initializer it moved.

**Static blocks written where they run (kuna).** With one `static` block after the fields,
the order rule keeps every initializer after the first statement that stays, though the
source most often has its block *before* a field whose initializer follows (Ghidra's
`UnsignedLeb128DataType` calls `ClassTranslator.put(..)` and then declares
`dataType = new UnsignedLeb128DataType()`), so such initializers read far from their
declarations. `extract_static_initializers` gathers the statements that stay into the runs
between the initializers it moves (`HeadRuns`): a run that `<clinit>` executes before a moved
initializer is written as a `static { }` block immediately before that field's declaration
(`ClassWriter::write_static_block`), and what follows the last moved initializer stays in the
block after the fields. The moved initializers are still in increasing field order and each
block lies between the declarations whose initializers run just before and just after it, so
the text runs every statement in `<clinit>`'s order. A run becomes a block only where the class
file shows it is a scope of its own: none of its locals is mentioned after it, in the head or
anywhere else in `<clinit>` (two blocks whose locals share a slot and that the variable analysis
merged into one local stay one block), it holds no switch expression, condition, `return`,
`throw` or monitor, and no local class is declared in it or before it, or outside the head;
otherwise the initializer stays in the run, as before. Since every statement kept earlier is
written before a moved initializer, the initializer may read a field that a kept statement
assigns. The class file does not tell a field's initializer from a last assignment of the block
before it (Vineflower's `TestStaticBlockFinalField` assigns `dum = field.substring(1)` at the end
of its block); the assignment is written as the initializer, as kuna writes every assignment it
can move. A block written before a
declaration may not read a later field by its simple name (JLS 8.3.3), which a class file not
compiled by javac from this layout can hold (ECJ, other compilers, obfuscators) and javac's
own source had to qualify: every read of a static field declared at or after the block's place,
in the block and in the bodies of the lambdas it holds, is written qualified (`C.f`), which
compiles to the same `getstatic`, while the target of a plain assignment keeps its simple name,
which the rule allows and a final field needs. Enum constants are written first and do not
close a run; synthetic fields, interfaces, classes written as Kotlin and the single-method
mode keep the rules above. A class translated from DEX lists its fields by name, and its
blocks stand among the fields in that order, which keeps the order of execution. The
statements stay in `<clinit>` through the class-level passes, which type and name their locals,
and leave it only when the class is written (`InitializerProcessor::take_static_blocks`).

**A lambda reads its own field qualified (kuna).** A moved initializer's reads of later fields
are qualified, in the bodies of its lambdas too, but in a lambda only the fields declared after
its own were: `static final Runnable SELF = () -> { if (SELF == null) .. }` is a self-reference
javac rejects (JLS 8.3.3 counts the field's own initializer), though the class file holds just a
`getstatic` of `SELF` (its source wrote `C.SELF`). The field itself is now qualified as well.

**A qualified empty `super()` (kuna).** A superclass constructor call that passes only the
outer instance of an inner member superclass prints as `super()`, the outer instance being
a synthetic argument, so `hideEmptySuper` took it for the implicit call and dropped it. It
keeps the call, written `o.super()` by `qualified_super_call`, when the class file proves the
qualifier: javac's null check of the instance, which it emits only for a written qualifier,
or an instance that is no class's `this`, which is all an unqualified call ever passes. An
enclosing `this` without the check stays implicit even where a supertype outside the input
hides whether the call's class is a member of that enclosing class.

**No `return` in a static initializer (kuna).** A `<clinit>` has one `return` at its end,
which the structuring may place inside the last statement: a loop that leaves only through
that block ends in `return;`, which Java rejects in an initializer. Before the class is
written, `initializer_return_break` (in `decompiler/managed/kuna-jvm/src/fixes/control_flow.rs`)
turns each `return;` that is not at the end of the body into a `break` out of the innermost
enclosing statement that is the last of a sequence ending the body (through the last
statements of sequences and both branches of an `if`), into an empty block placed after it;
the `break` is labeled unless that statement is a loop or switch with none in between. It
jumps where the `return` did, so the class compiles to the same instructions; a `return` with
no such statement stays.

Two class-level facts come from outside the class. **Varargs (kuna).**
`decompiler/managed/kuna-jvm/src/substrate/invocation_exprent.rs` needs to know
whether a called method is varargs, to print `f(a, b)` instead of
`f(new Object[]{a, b})`. It asks the class universe first; for a class the universe lacks,
Vineflower uses reflection on the JVM it runs on, which kuna cannot.
`decompiler/managed/kuna-jvm/src/p4_calls/classpath_helper.rs (ClasspathHelper)`
answers from an embedded table, `classpath_helper_jdk.txt` in the same folder: every
varargs method of OpenJDK 21 such a lookup can find, so the answer is the same whether or
not a JDK runtime is loaded. **Dynamic constants.**
`decompiler/managed/kuna-jvm/src/p4_calls/condy_helper.rs` writes the
`ConstantBootstraps` constants `nullConstant`, `primitiveClass`, `enumConstant` and
`getStaticFinal` as source expressions; a `VarHandle` constant
keeps its raw constant-dynamic form (Vineflower's `decompile-complex-constant-dynamic`
lookup expression runs eagerly, which the constant does not, and is not ported).

## 11.12 Emission

`decompiler/managed/kuna-jvm/src/p9_emit/class_writer.rs (ClassWriter::write_class)`
writes a root class and everything nested in it into a text buffer, and
`ClassesProcessor::write_class` puts the header in front: the package and
the imports `decompiler/managed/kuna-jvm/src/p9_emit/import_collector.rs (ImportCollector)`
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
the engine can see (an interface's superclass in the class file is `Object`, but an interface
has only `Object`'s public methods, so its `Object clone();` overrides nothing and gets no
`@Override`, which javac would reject: `decompiler/managed/kuna-jvm/src/fixes/class_writing.rs
(overrides)`);
`jvm_mark_corresponding_synthetics` names the synthetic method behind each lambda and the
class behind each anonymous class in a comment; `jvm_decompiler_comments` controls every `$VF:` note
the passes above attached to a method. Lambdas are written inline at their use, without
braces when the body is one expression. A generic method call carries explicit type
arguments only where inference needs them. Constants print as source
does: `Double.NaN`, `Long.MAX_VALUE` and `Math.PI` rather than their values unless
`jvm_keep_literals` is on, and non-ASCII characters escaped only under
`jvm_ascii_strings` (`decompiler/managed/kuna-jvm/src/substrate/const_exprent.rs`).
Numbers print with the JDK's own algorithms (section 11.13).

**Failed methods.** A method whose schedule threw is written as its signature and a body
of comments (`ClassWriter::dump_error`): `$VF: Couldn't be decompiled`, then the
note asking for the class to be reported to kuna's tracker (section 11.16), the
exception text and its throw location, and a listing of
the method's bytecode. The class around it is written normally.

**Layout.** The buffer is a tree of line groups; after a class is written,
`decompiler/managed/kuna-jvm/src/p9_emit/text_buffer.rs (TextBuffer::reformat)` breaks
argument lists, chained calls and long operator chains at their recorded break points
wherever a line exceeds 160 columns, and indents with three spaces (Vineflower's
`preferred-line-length` and `indent-string`, fixed at their defaults). Offsets and widths are counted in UTF-16
units, as Java counts them. kuna's zero-width span marks (`kuna_spans`) are the buffer's
only tokens: they follow every later edit the way Vineflower's text tokens (its hook for
IDE consumers, which kuna does not record) do.

**Line mapping.** While expressions are written, the text buffer records which bytecode
offsets each output line came from
(`decompiler/managed/kuna-jvm/src/p9_emit/text_buffer.rs (TextBuffer::kuna_bytecode_lines)`).
(kuna) `decompiler/managed/kuna-jvm/src/infra/kuna_spans.rs`
records, with zero-width marks that ride through re-wrapping and are never printed, where
each class and each method starts and ends in the final text, and groups those
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
it, and a writer that catches a body's failure (`write_method`'s body, the inline and block
lambda bodies) first drops the spans the throw left open since it started (`depth`, `unwind`),
so a field initializer's lambda whose anonymous class fails at a method header is recorded on
the class span, not on the dangling span of that header. A method span further carries the method's `$VF:` comments when its wrapper's
`add_error_comment` is set, the comments Vineflower follows with its report-this note.

**Failure granularity.** A method failure becomes the comment block above and the class
survives. A lambda or a method of an anonymous class that fails while it is processed (a stack
underflow) has no statement tree; the variable naming of the enclosing method
(`VarDefinitionHelper::iterate_clashing_exprent`) skips such a body instead of failing the
enclosing class, so the body is written as its failure comment like any other. A class whose processing throws (section 11.11) is written as
`// $VF: Couldn't be decompiled` followed by the exception, in place of its body. A class
whose writing throws is written as a `/* $VF: Unable to decompile class ... */` comment.
Nothing
else stops the run: the other classes of the input are still written. A Java
`RuntimeException` and an ordinary Rust panic (an index out of bounds, an `unwrap` on
`None`) are caught at these boundaries alike (section 11.13), and so is recursion deeper
than the engine thread's guarded stack, as a `StackOverflowError` (section 11.13), which
fails the method or class it unwinds to as Vineflower's JVM fails it; an allocation
failure is not caught, and ends the process.

**Time limit (kuna).** Vineflower enforces `max-time-per-method` by running each method on a
watchdog thread and killing it. kuna instead arms a per-thread budget
(`decompiler/managed/kuna-jvm/src/infra/cancelation_manager.rs (CancelationManager)`)
that the schedule polls at the heads of its fixed-point loops - the `finally` loop, the
stack-variable loop, the main and merge loops, the SSA fixed point, statement recovery and
irreducible-node splitting. The budget is a count of polls, `ClassWrapper::POLLS_PER_SECOND`
(200,000, the rate measured on the slowest methods, Ghidra's ANTLR-generated parsers) per
second of the limit, so whether a method runs out depends only on its input and options,
not on the machine or how loaded it is; Vineflower's wall-clock limit, and kuna's before
it, stopped more methods on a busy machine than on an idle one. A wall-clock deadline at
twice the limit backs the count up for a method whose polls are rare. A method that runs
out fails like a killed one: an error log and a `$VF: Couldn't be decompiled` body whose
exception is a `TimeoutException` (`decompiler/managed/kuna-jvm/src/p1_classfile/class_wrapper.rs
(ClassWrapper::code_to_java_on_watchdog)`). A method stuck between two polls is not
interrupted.

## 11.13 The runtime kernel (kuna)

Vineflower is written against Java's object model, and the port keeps its control flow
line for line, so `decompiler/managed/kuna-jvm/src/rt` gives each Java facility the
port depends on one faithful Rust shape:

- **Objects with identity** (`decompiler/managed/kuna-jvm/src/rt/arena.rs`). Every
  ported class with identity or shared mutable state lives in a thread-local arena and is
  referred to by a `Copy` handle, so `==` on handles is Java's reference `==`. Class-scope
  arenas (statements, expressions, edges, graphs, wrappers) are emptied when a decompile
  ends; session-scope arenas (the class model, the class forest) live as long as the
  engine instance. Hash collections keyed by such objects order them by creation index,
  where Java would use identity hashes.
- **Reclaiming garbage** (`decompiler/managed/kuna-jvm/src/infra/kuna_reclaim.rs
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
- **Hash collections in JDK order** (`decompiler/managed/kuna-jvm/src/rt/jhash.rs`).
  Vineflower iterates `HashMap` and `HashSet` in places where the order reaches the
  output, so the port reproduces JDK 21's table: hash spreading, power-of-two buckets,
  tail insertion, order-preserving splits on resize, and capacity kept across `clear`.
  Plain `std` hash maps are banned in the workspace for the same reason.
- **Shared collections** (`decompiler/managed/kuna-jvm/src/rt/jlist.rs`): a list or map
  that several holders read and mutate, as a Java reference would be.
- **Strings** (`decompiler/managed/kuna-jvm/src/rt/jstring.rs`). A Java string is UTF-16
  and may hold unpaired surrogates; they are kept in private-use scalars and mapped back
  for every UTF-16 view, so lengths, indices and escapes are the JVM's.
- **Characters and numbers** (`decompiler/managed/kuna-jvm/src/rt/jchar.rs`,
  `decompiler/managed/kuna-jvm/src/rt/jnum.rs`): `Character` classification from JDK
  21's tables, and `Double.toString`/`Float.toString` as JDK 19+ print them.
- **Exceptions** (`decompiler/managed/kuna-jvm/src/rt/exception.rs`). A Java `throw`
  unwinds as a panic carrying the exception, and `try_catch` is the
  `catch (Throwable)` that receives it at the method and class boundaries of section 11.12.
- **Stack overflow** (`decompiler/managed/kuna-jvm/src/rt/stack.rs`). The port recurses
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

## 11.14 Front-end support (kuna)

**Inventory.** `decompiler/managed/kuna-jvm/src/p0_knowledge/kuna_inventory.rs` lists an
input's classes and methods (names, nesting, access flags, source file, sizes, bytecode
length per method) from the class files alone, without decompiling.

**Parallel instances.** `decompiler/managed/kuna-jvm/src/infra/kuna_driver.rs (run)`
decompiles one input on several threads with output identical to a single instance's.
Every worker loads its engine as it starts; the first also plans: it lists the root
classes in the order a single run writes them. The single run's only cross-class dependency that changes output is a class
reading another root class's processing results while it is written (most visibly the
constructor masks of that root's nested classes, read when `new Outer.Inner(..)` or
`super(..)` is written), so `decompiler/managed/kuna-jvm/src/infra/kuna_partition.rs`
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
delivering (`decompiler/managed/kuna-jvm/src/infra/kuna_partition.rs (owner_written_in_full_run)`);
otherwise the repeat read the same unwritten state again and the batch ended in the last resort,
every root processed, which the detector does not watch and which still wrote only the delivered
roots (kotlin-stdlib's `SequencesKt___SequencesKt`, whose `sortedBy` creates the `compareBy`
object of `ComparisonsKt__ComparisonsKt`, written before it in the single run).
The last resort now processes and writes every root: the single run.
(kuna) An engine renames once, as a Vineflower instance does, however many batches it runs, and
is kept only while its batches succeed: an engine that fails to load or whose batch unwinds is
dropped and the next batch starts a fresh one, and a worker that unwinds outside the engine's
catch reports the batch it held as failed, so the caller does not wait for it
(`decompiler/managed/kuna-jvm/src/infra/kuna_driver.rs (InFlight)`). With renaming, the plan
reads class files under their new names (`decompiler/managed/kuna-jvm/src/p0_knowledge/fernflower.rs
(renamed_class)`), and a selection may name a root by either name. Writing a Kotlin class marks
members it reads again (its companion's overrides and forwarders, the `INSTANCE` and companion
fields); a single run processes every class before writing any, so an engine kept for another batch
takes those marks back when its batch ends (`decompiler/managed/kuna-jvm/src/kotlin/kt_attrs.rs
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
stack, of which the recursion guard (section 11.13) lets it use 32 MiB. The driver sets
the keys it owns (runtime, thread count, time limit) over the caller's.

**Options.** `decompiler/managed/kuna-jvm/src/p0_knowledge/kuna_options.rs (resolve)`
turns a `jvm_*` name and a value as a user writes it into the engine key (Vineflower's long
name) and value; the table is generated from `options.toml` by the crate's build script,
and each option has that one name. The keys the front end sets itself (thread count, time
limit, method selection) and the formatting keys kuna fixes at Vineflower's defaults have no
row, so `resolve` refuses them, like any other name, as unknown options. kuna's own rows
resolve to their own names; the rest becomes Vineflower's option map (section 11.16).
`decompiler/managed/kuna-jvm/src/p0_knowledge/i_fernflower_preferences.rs` tables the keys
the engine reads and their defaults; given to the engine directly, any other key stays in
the option map unread.

**The CLI seam.** `kuna-cli` routes an input to this engine before any native code runs
when the input operand is a class file, an archive or a directory (chapter 00, section
0.2). The CLI meets the engine through a contract of its own crate, `kuna-managed` (this
chapter's fourth anchor), which any managed-bytecode engine implements: an `Engine` sniffs,
classifies and opens its inputs and owns its option rows, and an opened `Program` lists
its types and members, resolves its selection grammar to a target and runs a selection,
delivering each root's unit (text, path, language, and the spans of the types and members
written into it) as the driver finishes it. The JVM side is
`decompiler/managed/kuna-jvm/src/infra/kuna_engine.rs`: it turns the resolved `--option`
pairs into the driver configuration (the JDK of `jvm_include_runtime` included), installs a
silent engine logger that marks each method's start and end on the CLI's stall watch and
arms its memory guard around the method, and converts each delivered class result into a
unit. The CLI's non-native module (this chapter's third anchor) is the same for every
engine: it slices types and members out of the delivered text by their spans and renders
the native CLI's text and JSON shapes, tagging each unit with its language (`java`, or
`kotlin` for a unit the Kotlin plugin wrote, whose file name ends in `.kt`). Before a
command that decompiles (not `functions`) it sets glibc's heap policy for the process
(`decompiler/crates/kuna-cli/src/managed/heap.rs (tune)`, Linux with glibc only): a 256
MiB top pad, no trimming, and blocks up to 32 MiB served from the heaps instead of fresh
mappings. The engine threads allocate and free many small objects, and under the default
policy each heap's growth and trimming are memory-map calls under one process-wide lock
that serialize the threads; the native path never sets it. For the same reason it arms the
per-method memory limit (chapter 00, `kuna_fnmemory`) without the heap release the native
drivers make before each function
(`decompiler/crates/kuna-cli/src/managed/run.rs (arm_memory_limit)`). The CLI surface is
`docs/cli.md` ("JVM inputs").

## 11.15 Kotlin

Vineflower's Kotlin plugin, ported in `decompiler/managed/kuna-jvm/src/kotlin`, is the
one bundled language plugin. A class it claims is decompiled by a method pipeline and
written by a writer of its own, as Kotlin source in a `.kt` unit. Everything before the
point where it takes over is the Java engine's: loading and nesting (section 11.2), the
class-file model (11.3), the control-flow graph up to the exception-range repairs (11.4),
lambda discovery and the nested-class and accessor processing of a root class (11.11).

**Which classes are Kotlin.** `decompiler/managed/kuna-jvm/src/kotlin/kotlin_chooser.rs
(KotlinChooser::is_language)` claims a class when `jvm_kt_enable` is on (the default) and
the class carries a `kotlin/Metadata` annotation, visible or invisible at run time: the
annotation the Kotlin compiler puts on every class it emits. Nothing else counts, so a Java
class is never written as Kotlin and a Kotlin class whose annotation was stripped is
written as Java. A class the chooser claims gets the session's Kotlin language spec
(`decompiler/managed/kuna-jvm/src/infra/language_spec.rs (LanguageSpec::of)`): a CFG pass, a
statement parser, a main pass, a root-class processor, a writer and a file extension. The
question is asked for the root class at four places - when it is processed
(`decompiler/managed/kuna-jvm/src/p1_classfile/classes_processor.rs
(ClassesProcessor::process_class)`), which hands the spec to the wrapper of every class
nested in it; when it is written (`ClassesProcessor::write_class`), which picks the writer;
and when its unit's file name is formed (`decompiler/managed/kuna-jvm/src/p0_knowledge/fernflower.rs`),
which takes the spec's extension, `kt`. A nested class therefore follows its root. (The
chooser is asked once more while statements are written, to decide whether they end in a
semicolon.) With `jvm_kt_enable`
off, a Kotlin class takes the Java pipeline and is written as Java, the `@Metadata`
annotation printed like any other annotation, and its metadata is not parsed.

**Metadata.** Asking the chooser also parses the class's metadata, once
(`KotlinChooser::parse_metadata_for`). The annotation's `k` (the kind), `d1` (the data)
and `d2` (the strings) are decoded by `decompiler/managed/kuna-jvm/src/kotlin/kt_metadata.rs`:
`d1` is protobuf bytes spread over Java strings, either one byte per character (after a
leading NUL) or, by default, seven bits per character
(`decompiler/managed/kuna-jvm/src/kotlin/bit_encoding.rs`). Its first message is a string
table whose records `decompiler/managed/kuna-jvm/src/kotlin/metadata_name_resolver.rs`
expands into the names the rest refers to: each record a `d2` string or one of a fixed list
of predefined names, optionally cut to a substring or with one character replaced, `$`
read as `.` (including Vineflower's slip of testing the substring list but reading the
replacement list, kept for fidelity). The second message is the one `k` names: a class
(1); a file of top-level declarations, compiled to a `...Kt` class (2); a synthetic class,
such as a lambda compiled to a class, described by one function (3); a multifile facade,
whose `d1` lists its part classes (4); or a multifile part, a file whose facade is named by
the annotation's `xs` (5). The messages follow the Kotlin compiler's metadata schema, which
the plugin ships as a shaded protobuf-lite build; `decompiler/managed/kuna-jvm/src/kotlin/kt_proto.rs`
ports that runtime and schema as one schema-driven reader with the same semantics: unknown
fields and enum numbers are skipped, repeated fields accept packed and unpacked encodings,
a scalar given twice keeps the last value, a sub-message given twice is merged, and a
missing required field fails the parse. The flag words (visibility, modality, class kind, `data`,
`value`, `inline`, `suspend`, `operator`, `infix`, `const`, `lateinit`, accessor flags,
parameter defaults) are unpacked by `decompiler/managed/kuna-jvm/src/kotlin/kt_flags.rs`.
A parse that throws is logged as a warning and leaves the class with what was attached
before the failure; a class without a usable `k` or `d1` gets no model, and its writer
says `// $VF: Class flags could not be determined`.

The decoded message becomes the plugin's model of the class
(`decompiler/managed/kuna-jvm/src/kotlin/k_element.rs`): properties with their getters,
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
member (`decompiler/managed/kuna-jvm/src/kotlin/kt_attrs.rs`), which the struct context
clears when it closes. Parsing one class can mark members of another (a companion's outer class),
so the order of parsing matters, and it is Vineflower's: before any class is processed,
`decompiler/managed/kuna-jvm/src/p0_knowledge/context_unit.rs` forms every class's file
name, which parses the metadata of every class in entry order.
Writing a class with a companion object reads the companion again with the class as its parent
and marks hidden every member of the class that has the JVM signature of a companion member,
in place of the element the class's own metadata gave it (`kotlin.random.Random`'s
`nextInt(until: Int)`, which its companion `Default` overrides). (kuna) The elements the class
declared are kept first (`decompiler/managed/kuna-jvm/src/kotlin/kuna_declared_element.rs`),
and the fixes that ask which members another class declares (`kt_extension_call`) read those, so
their answer does not depend on whether that class was written first: in the input's order or,
in the driver, on which batch an engine ran before (kotlin-stdlib's `RangesKt___RangesKt` wrote
`random.nextInt(this)` or `RandomKt.nextInt(random, this)` by the worker count). With the fix
`kt_companion_static_copy` only a static method is marked, and the class is written with its own members.

**The pass schedule.** For a Kotlin class,
`decompiler/managed/kuna-jvm/src/infra/method_processor.rs (MethodProcessor::code_to_java)`
builds the graph, removes dead blocks, inlines `jsr`, connects the exit and removes gotos
as for Java, then
hands the method to the spec's passes, declared in
`decompiler/managed/kuna-jvm/src/kotlin/kotlin_plugin.rs`, and returns; none of the Java
schedule after that point runs, IdeaNotNull included. The CFG pass is the exception-range repairs of section 11.4 in
the same order, step 8 as `decompiler/managed/kuna-jvm/src/kotlin/obfuscated_exceptions_pass.rs`.
The statement tree comes from the same dominator-based recovery (11.5), and the
main pass is the Java schedule of sections 11.6-11.10 with these differences: the
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
`replace_exprents_pass.rs` (in `decompiler/managed/kuna-jvm/src/kotlin`) replace every
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
`k_sequence_statement.rs`), all in `decompiler/managed/kuna-jvm/src/kotlin`. JVM types
are written as Kotlin types (`int` as `Int`, `java.util.List` as `List` or `MutableList`
by the metadata, nullable types with `?`; `k_types.rs`). (kuna) A Kotlin expression or
statement in the port is its Java object with a Kotlin flag and the subclass's extra data,
and every dispatcher of a method a Kotlin subclass overrides checks the flag first
(`decompiler/managed/kuna-jvm/src/kotlin/k_dispatch.rs`): the object stays an instance of
its Java kind for every core helper that still runs on it, which is what subclassing gives
Vineflower.

**The Kotlin writer.** `decompiler/managed/kuna-jvm/src/kotlin/kotlin_writer.rs
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
the `$default` stub's body (`decompiler/managed/kuna-jvm/src/kotlin/default_args_map.rs`):
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
for Java (section 11.12).
By default kuna's fixes write the JDK members and classes Kotlin maps by their Kotlin names
instead (`text.length`, `(f(x) as Number).toInt()`; fixes `kt_mapped_java_member` and
`kt_mapped_java_type`, section 11.16).

**Front-end support (kuna).** The Kotlin plugin's own tests and core's Kotlin tests are
corpus cases (sets *Kotlin ...* of `decompiler/managed/kuna-jvm/corpus/cases.tsv`). `decompiler/managed/kuna-jvm/src/infra/kuna_spans.rs`
spans are recorded by the Kotlin writer too: a property's getter and setter share the span
of the property's declaration, a primary constructor's span is its `init` block (it has
none when the body is empty; it is opened before the constructor is stringified, so a
failure inside it is the constructor's), and members the writer folds away have none.
`DecompiledClass::language` (`decompiler/managed/kuna-jvm/src/p0_knowledge/fernflower.rs`)
reports `kotlin` for a unit whose file name ends in `.kt`, which is how the CLI labels
units and writes `.kt` files in a project export; the CLI's `--language java` is
`jvm_kt_enable off`. The Kotlin schedule polls the per-method deadline (section 11.12) at
the heads of its own loops: each round of a looping pass, the `finally` loop and the
initial stack-variable loop.

## 11.16 Kuna-only fixes (kuna)

Vineflower 1.12.0 throws on some methods, and the port, which keeps its failure behaviour,
throws there too and writes the method as `// $VF: Couldn't be decompiled` (section 11.12).
A *fix* is a deliberate departure from the Java at the place of such a defect, or of code
Vineflower prints wrong. This section describes the invariants the fixes keep and the
mechanisms groups of them share. A fix longer than a few lines lives in its family's module,
`decompiler/managed/kuna-jvm/src/fixes/<family>.rs`, or, in a family with a folder, in
`decompiler/managed/kuna-jvm/src/fixes/<family>/<id>.rs`.

**No switch.** A fix is a correction, not a judgement call, so it has no option and no
gate: its code takes the place of the Java's at its site, and the engine has one mode.
The regression corpus's runner
(`decompiler/managed/kuna-jvm/src/infra/kuna_corpus.rs (execute)`) runs every case
with the defaults: the cases from Vineflower's own test suite against a digest of the text kuna
printed when last reviewed, and kuna's cases (`decompiler/managed/kuna-jvm/corpus/kuna-fixes/`) against kuna's
reviewed text.

**Invariants.**

- A shape a fix does not recognise keeps Vineflower's text, or fails as Vineflower does;
  where the evidence a fix needs is missing (a class outside the input, a type kuna cannot
  resolve), Vineflower's choice stands.
- The failure marker line never changes: the CLI and the benchmark count failed methods by
  it. Only the note under it is kuna's: a request to report the class to kuna's tracker
  (`decompiler/managed/kuna-jvm/src/p0_knowledge/fernflower.rs (KUNA_ERROR_MESSAGE)`) where
  Vineflower names its own.
- The `kt_*` fixes change Kotlin output only: in a pass or writer that Java output shares,
  they act only on a class written as Kotlin (a class with Kotlin metadata, the Kotlin
  plugin on).
- A rewrite keeps what the code does. A control-flow rewrite (a handler copy, a split node)
  is exact, and a fix's evidence includes recompiling its case's text to the original's
  instructions.

**Exception ranges, handler copies and `finally`.** When a protected range has several
entries (kotlinc's `try` around a suspension point, whose resume label jumps into the
range; javac's record patterns inside a `try`),
`decompiler/managed/kuna-jvm/src/p2_lift/exception_deobfuscator.rs (ExceptionDeobfuscator::handle_multiple_entry_exception_ranges)`
splits it and gives every part but the first a copy of the handler's first block, with the
handler's instructions and successors but no bytecode offsets, placed in every range that
protects the original. `handler_copy_offsets`, `nested_handler_copies`,
`handler_copy_shared_body` and `copied_finally` restore the nesting right after the
duplication, give each copy its own body, let `FinallyProcessor`
(`decompiler/managed/kuna-jvm/src/p7_regions/finally_processor.rs`) accept copies without
offsets, and mark a copied catch-all that traces back to a processed `finally` as a
`finally`; a part split off at a resume label prints as a `try`/`finally` guarded by a
semaphore variable. `DomHelper` (`decompiler/managed/kuna-jvm/src/p7_regions/dom_helper.rs`)
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
`decompiler/managed/kuna-jvm/src/p6_variables/var_definition_helper.rs (VarDefinitionHelper::merge_vars)`
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
`decompiler/managed/kuna-jvm/src/substrate/invocation_exprent.rs (InvocationExprent::get_inferred_expr_type)`
and `decompiler/managed/kuna-jvm/src/substrate/new_exprent.rs (NewExprent::get_inferred_expr_type)`
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
`decompiler/managed/kuna-jvm/src/p2_lift/expr_processor.rs (ExprProcessor::get_casted_exprent_full)`
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
descriptors the input calls stand in for its methods. An unboxing call left unwritten
whose instance is an assignment (`++a` on an `Integer` is `(a = a + 1).intValue()`) has the
assignment's precedence (kuna, `numeric_types::bare_unboxed_assignment`), so an operator
around it parenthesizes it: `++a + ++a` printed `a = a + 1 + Integer.valueOf(a + 1)`,
which assigns the whole sum, and `--b * 2` printed `b = b - 1 * 2`.

**Casts that select an overload (kuna).** `getAmbiguousParameters` finds no ambiguity when
no overload takes the argument types exactly, so `f(m)` for a `Member` that is both a `Decl`
and a `Named`, with `f(Decl)` and `f(Named)` declared, or `put(d, null)` beside `put(Decl,
String)` and `put(Decl, Integer)`, printed without the cast that picks the invoked method,
and javac reports the call ambiguous. For a method of a class in the input,
`selecting_cast` resolves the written call as javac does (JLS 15.12.2): the candidates are
the methods of that name of the class and its known supertypes the current class can
access, less bridges and synthetic methods; when the invoked method is applicable by strict
invocation to the written argument types and some other candidate also is but is not less
specific, each argument whose type differs from its parameter's at a position where such a
candidate's parameter differs is cast to its parameter type. Every candidate still
applicable then takes a supertype of each parameter, so the invoked method is the most
specific. A call whose candidates have generic parameter types, whose arguments include a
lambda or a type kuna cannot resolve, or whose method is varargs or not strictly
applicable keeps Vineflower's casts.

**A callee's unbound class variable (kuna).** `appendParamList` writes each argument
against its parameter's generic type, remapped through the type arguments the receiver's
type and the class hierarchy give the callee's class. When neither binds a type variable of
that class (a receiver whose parameterization kuna does not see, such as a local an
enclosing method declares and a lambda captures), the remapped type kept the variable's
name, and a cast to it named whatever variable of that name is in scope at the call:
`bag.add(check(t))` printed as `bag.add((E)((Task)check((E)t)))` inside a method declaring
its own `E`. `raw_receiver_erasure` writes such an argument of a method call (a constructor's
class variables are inferred from its arguments) against the parameter's erasure,
the type the descriptor passes it as, so no cast names a variable of another declaration; a
class whose variables are in scope at the call (the current class or an enclosing one) keeps
the remapped type.

**A field of a raw receiver (kuna).** A field read through a receiver whose static type is raw has
the erasure of its declared type (JLS 4.8), as the result of a call on one has, but its inferred
type kept its class's type variable; where the reading class has a variable of that name, the
argument looked like the parameter's type and lost the cast it needs:
`fill(into.data, ((UnitStore)from).data)` for `fill(N[], N[])`, which javac rejects. Such a field
read passed for a parameter of a method whose type is a type variable or an array of one (the
erasure converts to a parameterized type unchecked, not to these) is written with the cast to the
parameter's type
(`decompiler/managed/kuna-jvm/src/fixes/generic_casts/raw_receiver_erasure.rs (has_raw_field_receiver)`,
read by `InvocationExprent`'s argument list): `fill(into.data, (N[])((UnitStore)from).data)`. Its
inferred type stays, since a constructor infers its class's type arguments from it (a diamond
inferred from the erasure would not compile).
The declared type of a call looks past a bridge (`desc_of`): javac gives a public class a public
forwarder, without a signature, for each public method it inherits from a superclass that is not
public, and its erased result made a for-each over the call one over a raw iterable, cast to its
class parameterized by the loop variable, which javac rejects for a wildcard receiver
(`(Set<Entry<K, ? extends Comparable<?>>>)series.entrySet()`); the superclass's method it
forwards to has the same name and descriptor and the signature.

**A varargs spread beside a generic overload (kuna).** `vararg_spread_resolution` keeps a
varargs array written as an array when another overload is applicable to the spread
arguments, judged by erasure, so `of(c)` for `of(Chromosome<G>, Chromosome<G>...)` printed
as `of(c, new Chromosome[0])` beside `of(Iterable<? extends Chromosome<G>>)`, whose erasure
takes any `Iterable`; the raw array then makes the call unchecked. Such an overload is not a
rival when its generic signature provably rejects an argument: the argument's class gives
the parameter's class known type arguments (through the class signatures of its supertypes),
and one of them is not contained by the parameter's (JLS 4.5.1): for `? extends B` a class
that is not a subclass of `B`'s, for `? super B` one `B`'s class is not a subclass of, for
`B` another class. Subtyping implies these relations of the erasures, so the test is sound
only when the classes compared have every supertype but `Object` in the class context; when
one is missing the array stays.

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
For `x.CONST`, a constant instance field read through an expression, javac emits the null
check of `x` and then loads the constant: removing the check alone printed `return 3;` for
`return b.k;`, which no longer throws for a null `b`. `synthetic_null_check_shape` records
the offset of the loaded constant, and `restore_field_read` writes the constant at that
offset as the read of the first instance field of `x`'s class (or a superclass) that holds
its value; where it finds no such constant, the check stays written.
(kuna) A creation `Outer.this.new Inner(..)` whose outer instance javac checks with a
`requireNonNull` the creation's next argument follows keeps its qualifier also when the check
already reads the outer instance and the creation still the stack variable holding it
(`implicit_outer_instance::mark_checked`, which matched only an equal argument): it printed
`new Base.Inner(..)`, which javac compiles without the check.
(kuna) A creation of an inner member class whose class is not in the input (a nested class of
another top-level class decompiled alone, or of a library) printed its outer instance as the
first constructor argument, `new Cursor(Outer.this, false)`, which does not compile: the
outer-instance parameter is known only from the created class's own node. The InnerClasses
attribute of the class being written lists every nested class its constant pool names, with
its outer class, simple name and flags (JVMS 4.7.6); `constructor_calls::listed_inner_member`
reads a non-static member entry from it, and when the constructor's first parameter has that
outer class's type (`listed_outer_instance_mask`) the argument is the outer instance: it is
left out of the argument list and printed as the qualifier, or not at all, by the rules
`implicit_outer_instance` applies to a class of the input (`lib.new Entry(2)`,
`Outer.this.new Entry(3)` after javac's check, `new Entry(1)`). The superclass-call path of
`qualified_super_call` reads the same entry.
Three recognisers get the same treatment.
`decompiler/managed/kuna-jvm/src/p8_structure/if_pattern_match_processor.rs (IfPatternMatchProcessor::check_branch)`
makes a pattern only under the preconditions of `pattern_variable_scope` and
`unchecked_type_pattern`; the latter takes a cast to a parameterized subtype as checked (JLS
5.1.6.2) when the operand's type arguments fix every type argument of the target, matching the
argument the target class gives the operand's class against the operand's argument by
structure, so `Factory<Genotype<G>> f` tested as `f instanceof Genotype<G> gt` (where
`Genotype<G>` implements `Factory<Genotype<G>>`) stays a pattern; a bare parameter is bound
directly, a wildcard or array argument only when it names no parameter and is equal.
`decompiler/managed/kuna-jvm/src/p8_structure/assert_processor.rs (AssertProcessor::replace_assertion)`
finds an assert's `throw new AssertionError` in the `if`'s body, its else or the statement
the body breaks to, and with `assert_dead_throw` and `assert_throw_after_if` consumes it
only when its other incoming edges are accounted for.
`decompiler/managed/kuna-jvm/src/p8_structure/switch_pattern_match_processor.rs (SwitchPatternMatchProcessor::process_statement)`
turns the index constants of a `SwitchBootstraps.typeSwitch` back into labels, one defect
of which each of `type_switch_enum_label`, `type_switch_pattern_type` and
`type_switch_label_lookup` corrects.

**A case's own `break` (kuna).** `LabelHelper::set_explicit_edges` takes a case followed by an
empty case that only breaks (labels whose keys jump to a `goto` past the switch, javac's `case
'a': case 'b': break;`, or the default javac points past the switch) as running into the
statement that empty case breaks to, so the case's own break became implicit and printed as a
fall-through into the empty labels, or vanished with a hidden empty default. The control-flow graph
drops every `goto`, so only the instructions tell the two apart: javac lays cases out in order, and a
case ending with `break;` ends with a `goto` right before the instruction the empty labels' keys jump
to, going where they go (past the switch). `decompiler/managed/kuna-jvm/src/fixes/switches.rs
(falls_into)` keeps the edge explicit exactly when that `goto` ends one of the case's blocks, the
empty case breaks this switch and the case's breaks to there close the switch itself (one closing a
statement inside the case would print as a labeled block); an empty case that leaves another way
(code of the next case that the structuring moved out of the switch, a loop's `break label`) keeps
Vineflower's rule.

**Heap reads copied into several uses.**
`decompiler/managed/kuna-jvm/src/p3_dataflow/stack_vars_processor.rs (StackVarsProcessor)`
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
(`decompiler/managed/kuna-jvm/src/kotlin/kotlin_writer.rs`) writes declarations from the
metadata. `KConstructor` (`decompiler/managed/kuna-jvm/src/kotlin/k_constructor.rs`)
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
`KProperty` (`decompiler/managed/kuna-jvm/src/kotlin/k_property.rs`) writes a property
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
`decompiler/managed/kuna-jvm/src/p8_structure/nested_class_processor.rs (NestedClassProcessor)`
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
`decompiler/managed/kuna-jvm/src/kotlin/default_args_map.rs (DefaultArgsMap::from)` reads
each default back from the member's `if ((mask & bit) != 0)` branch, and a default it
cannot read prints as `= ...`; `KConstructor.parse` finds a constructor's
default-arguments constructor by its descriptor. `kt_default_args_super_call`,
`kt_default_args_call`, `kt_default_arg_compound_branch`, `kt_default_arg_assigned_param`,
`kt_default_args_ctor_descriptor`, `kt_default_args_bridge_lookup`,
`kt_default_value_names`, `kt_default_value_coercion_cast` and
`kt_delegated_member_default` read this layout correctly and write a call through it as
the call the source makes.

**Types and names in Kotlin output.** `KTypes`
(`decompiler/managed/kuna-jvm/src/kotlin/k_types.rs`) translates only `Integer`, `Object`
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
`Signature`'s variable at the same position. Such a class hides the members kotlinc adds for
a read-only collection interface that only Java's interface declares, recognised by their five
instructions (`new`, `dup`, `ldc` of the read-only-collection message, `invokespecial` of
`UnsupportedOperationException.<init>`, `athrow`); the constant pool is read only after the five
opcodes match, so any other five-instruction member (a `DefaultImpls` bridge ending in a
`checkcast`) is written instead of failing its class. `KInvocationExprent` reads metadata only from
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
`decompiler/managed/kuna-jvm/src/kotlin/k_do_statement.rs (KDoStatement::to_java)`
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
(`decompiler/managed/kuna-jvm/src/p8_structure/initializer_processor.rs`) moves each
`<clinit>` statement it can into its field's initializer and stops at the first it cannot,
leaving a remainder the Kotlin writer cannot express; `kt_object_instance_local`,
`kt_static_initializer`, `kt_property_initializer_key`, `kt_file_static_initializer` and
`enum_constant_in_values_array` remove the obstacle or write the remainder as an `init`
block or `run { }` initializers that run each statement where it ran.

**Suspend functions.** A method is in the suspend calling convention when its descriptor
ends in a `Continuation` and it returns `Object`. kotlinc lowers a suspending body to a
state machine whose resumed path recreates the direct path's frame, so the direct path is
the source's body. The `SuspendStateMachine` pass
(`decompiler/managed/kuna-jvm/src/fixes/kt_suspend/kt_suspend_state_machine.rs`), first
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

## 11.17 DEX input (kuna)

A DEX file or an APK reaches the engine as class files. The loader
(`decompiler/managed/kuna-jvm/src/p0_knowledge/kuna_input.rs (dex_unit)`) parses each
`classes*.dex` in order, translates every class definition with `kuna-dex`, each file with
the others as its *peers* (d8 may call a synthetic class placed in another file), keeps the
first definition of each class name, leaves out the classes `kuna-dex` marks as omitted (a
d8 helper whose every use it wrote back), and lists and serves the bytes as a jar unit's
entries; a class that cannot be translated at all is an entry whose read fails with the
reason. The rest of this section is `kuna-dex`, which depends on nothing and translates one
class without reading any other class's code (only the d8 synthetic classes of the file and
its peers are recognised, once per file).

**Parsing.** `decompiler/managed/kuna-dex/src/dex.rs` validates the header (versions 035
to 039, little-endian, the declared file size) and the bounds of every id table once, and
reads items on demand: class data and encoded fields and methods (LEB128), code items with
their try items and catch handlers, encoded values and arrays, annotation directories, sets
and parameter lists, call sites and method handles (from the map list). Every offset and
count is checked when read, counts never size an allocation beyond what the remaining bytes
can hold, and nested values are depth-limited, so a malformed file is an error.
`decompiler/managed/kuna-dex/src/insn.rs` decodes every instruction format and the three
payloads; `decompiler/managed/kuna-dex/src/debug.rs` runs the debug-info state machine
into line positions and local ranges (parameters, `this` included, start at address 0).

**Ops and blocks.** `decompiler/managed/kuna-dex/src/code.rs` turns each instruction
into an op with its register uses in JVM push order and its def, each with a category
mask (int-like, float, long, double, reference): an invoke absorbs the `move-result` that
follows it, `fill-array-data` reads and redefines its array (so `new int[]{...}` keeps the
array on the stack), and a `new-instance` whose reference every path from it carries, within
a few blocks and without another read, to one `<init>` call on it becomes one op at that
call defining the constructed object (`new T(...)`, arguments computed before it). The
reference may travel through `move-object` copies d8 makes to line up a range call; those
copies vanish, and the op defines whichever of the reference's registers is read after the
call (it fails over when more than one is). Blocks start at branch targets, after branches,
returns, throws and the monitor ops, at try boundaries and at handlers; a block in a try range has its handlers as exception successors, and
reachability follows normal edges and the handlers of blocks with a throwing instruction.
Unreachable blocks are not translated, and a reachable block that falls off the code is an
error.

**Values and webs.** Liveness is computed per register with exception edges taken at each
throwing op (the state before the op flows to the handler). A forward pass then numbers
values: every def is a value, and where a live register receives two different values at a
block entry the block gets a phi for that register; a use names the one value reaching it.
Webs are the union-find classes of values joined through phis. A web of a single constant
def with no phi is *inline*: the constant is written at each of its uses, typed by that use,
so one register can carry one literal (`0` as int, float and `null`) to differently-typed
uses, and no store is emitted. After typing, a `move` that d8 wrote to line up a range call
is forwarded: when its source is a parameter or a value read only by the move, and its copy
is read once later in the same block, that reader reads the source instead and the move
vanishes, provided no other op writes the source's local in between; a chain of such moves
(d8's register shuffles) resolves to the first source. The source can then fold into its
reader as an expression, so the arguments of a `super(...)` or `new T(...)` that d8
evaluated out of order, or parked in other registers, are written inside the call.

**Types.** Each web is a type variable starting from all categories; the method's
parameter types, every typed opcode, field type, invoke parameter and return type, and
`move-result` kind narrow it, and moves, `check-cast`, fills and the two operands of
`if-eq`/`if-ne` join variables. Reference descriptors are inferred for webs from
parameters, allocations, casts, members, results and element reads, iterated to a fixpoint,
and an `aget`/`aput` value takes the element category of its array's descriptor. A
variable left ambiguous resolves int over float over reference (long over double); an empty
one is a method failure. An array whose type stays unknown (always `null`) is accessed with
the opcode its value's category picks, and `aconst_null` is accepted by every array opcode.

**Emission.** `decompiler/managed/kuna-dex/src/emit.rs` writes one JVM local per
(register, category), parameters in their JVM slots; a parameter's register that d8 reuses
for another variable gets a second local for it, so the parameter's own local holds the
parameter throughout and a copy of it can always read it. A value is the parameter's own
when it is in the parameter's web, when its instruction (as d8 wrote it) reads the
register's own value and writes back one of the parameter's type (an update in place:
arithmetic, `n--`, or another instruction but a cast whose value is read more than once,
`n = Math.min(n, k)`), or when debug info names it as the parameter; everything else in that
register is another variable (`char[] cs = s.toCharArray()` in `s`'s register). An
`add-int/lit` becomes `iinc` only when the value it reads is in the local it writes. A
parameter debug info does not name (a release build) gets a `LocalVariableTable` entry over
the whole method with its `MethodParameters` name, as javac's `-g` writes one; the engine
otherwise names a read of it apart from its declaration.

**Release builds.** d8's release build also gives each `return` of a boolean method its own
block where javac shares one `ireturn` for `return c;`: two blocks that only return `true`
and `false`, on the same source line, share one `ireturn` again (the constant pushed in each,
the first jumping to the second), so the engine prints the condition; returns on different
lines were written as such and stay. And it reads `Collections.EMPTY_LIST`, `EMPTY_SET` and
`EMPTY_MAP` where the code called `emptyList()`, `emptySet()` and `emptyMap()`; such a read
is written as the call. In a method without debug locals, references of different types
that d8 put in one register get one local per type (the type inferred for each value's web),
so the engine does not type them as one variable of their common supertype. And a value
captured by a lambda body (a synthetic implementation method) stays in a local unless it is a
field read (a capture is a variable in source; folded into the `invokedynamic`, the engine
drops the computation); a method reference's bound receiver is an expression and folds. Blocks are laid
out in address order, a fall-through successor right after its block, and a block whose
only predecessor jumps to it unconditionally right after that predecessor, with the `goto`
dropped (so a handler's code is contiguous, as javac writes it); the regions below then
move blocks, and a block whose fall-through successor no longer follows it gets a `goto`. Within a block, a value defined once and used once
later in the same block, not named by debug info there, is not stored: its op is pushed on a
pending stack, and an op folds pending values into its operands only when they form the top
of that stack in the operand order the JVM evaluates (`aput`, `iput` and `sput` push their
operands in a different order than Dalvik lists them); every other pending value is flushed
first, in Dalvik order, as a store, so no side effect, load or exception moves past another.
A statement flushes everything pending before it is written. A copy of a parameter (`move`, as d8 writes
for methods with many registers, `this` included) whose parameter register nothing writes is
not stored: each use reads the parameter's own local, so `super(...)` is called on `this`
and parameters keep their names. A join whose phi receives a
constant (or a copy of one) on every incoming edge (a materialised boolean, `c ? 5 : 7`; d8 often hoists one
constant above the branch) gets the constant pushed on each edge instead: a predecessor that
jumps or falls into the join pushes its constant last, and a conditional or switch edge
lands on a stub that pushes it, the stubs laid out right before the join as javac's arms are.
The value then stays on the stack when it is read once, as the first operand of the join's
first statement, and is stored into its local at the join otherwise; the constants themselves
are not written. A register some exception handler reads is excluded, since its local would
then be unwritten where the JVM merges handler states. `add-int/lit` on one local
becomes `iinc`, `not-int` becomes `^ -1`, `rsub-int` pushes its literal first,
`filled-new-array` becomes `newarray` and a `dup`/store per element, comparisons become
`lcmp`/`fcmpl`/`fcmpg`/`dcmpl`/`dcmpg`, the switches `tableswitch` and `lookupswitch`
(default: the fall-through block), `if-eqz` on a reference `ifnull`, `invoke-polymorphic`
`invokevirtual` with the call-site descriptor, and `invoke-custom` `invokedynamic` with a
`BootstrapMethods` entry. A handler block starting with `move-exception` is the JVM handler
and stores the exception; any other handler is reached through a stub that pops it.

**Regions.** d8 keeps a try item only over instructions that can throw, places blocks by
its own order (a handler among the code it protects, a `throw` after the method's return),
and cannot list a typed handler after a catch-all, so javac's one `finally` range arrives in
pieces and an outer handler does not cover the inner `finally` it encloses; the engine's
`finally` processing (11.9) matches neither. `decompiler/managed/kuna-dex/src/region.rs`
therefore lays out each catch-all handler that throws the exception it caught again (a
`finally`, or a `synchronized` block's release) as javac does: its *region* is the blocks
it protects plus the regions of the handlers whose first throwing code it protects (the
nested ones), excluding its own handler code (the blocks only its entry reaches). Regions
are arranged innermost first: within the span of layout positions the region occupies, a
block that cannot throw, is no handler and is entered only from the region stays and is
covered, and every other block moves right after the region in its order. A region whose
handler releases a monitor also takes in the `monitor-exit` blocks of that monitor entered
only from it (the normal exits) and the blocks between the `monitor-enter` and its first
protected block, and its handler protects its own code up to the release, as javac writes
`synchronized`. Each region is one exception-table entry, after the per-try-item entries of
the other handlers and innermost first, plus one for the handler code it protects. The
table is kept only if every block that can throw still meets, entry by entry, exactly the
handlers its try item lists, in that order, up to a catch-all; otherwise every try item
protects the pc intervals of its own blocks, as without regions. A catch-all handler that
does not throw its exception again is javac's `catch (Throwable t)`, which DEX writes as a
catch-all, and is written with the catch type `java/lang/Throwable`.
`LineNumberTable` maps each statement's first pc to the line of its earliest Dalvik
instruction; `LocalVariableTable` maps each debug local to the pc intervals of the ops in
its address range. Branches are 16-bit unless one overflows, when the method is assembled
again with `goto_w` and inverted conditionals over it. A method over 65,535 bytes, a
register read before any write, or a type conflict fails that method only: it gets a body
that throws and a `KunaDexError` attribute (a constant-pool string, laid out as
`SourceFile`), which the engine turns into its failure comment
(`decompiler/managed/kuna-jvm/src/infra/method_processor.rs (code_to_java_schedule)`).
A panic in the translation of one method is caught and fails that method the same way.

**Lambdas.** d8 desugars every `invokedynamic` to `LambdaMetafactory` into a class
`X$$ExternalSyntheticLambdaN`. `decompiler/managed/kuna-dex/src/lambda.rs` recognises
such a class once per file: it implements one interface, its constructor stores its
captures in its fields in order, and its one method loads them, adapts its parameters
(casts, unboxing), calls one implementation method (or `new`), and returns the result,
boxed if needed. A `new` of it becomes the `invokedynamic` javac wrote: the interface
method's name, the captures' types to the interface, and `metafactory` with the interface
method's type, a handle to the implementation (static, virtual, interface, special or
constructor reference, from the call) and the instantiated type (the implementation's
parameter types after the captures, refined by the casts, boxed where the interface
method takes references, and its return type, likewise). A method whose whole work is the
unboxing call is the method reference to it (`Long::longValue`). d8 reaches a private
instance method (a lambda body, or a method reference's target) through a public static
synthetic bridge of the same class (`$r8$lambda$...`) that passes its parameters on
unchanged; the handle then names the method itself, virtual (interface in an interface), as
javac's does (the bridge stays, synthetic, for the lambda class). The engine then prints the lambda or
method reference. A lambda class of any other shape (d8 inlined the body into it) is left as
it is; the lambda class of a `new` in a peer file is found there.

**Backports.** Below the API level that added a JDK method, d8 calls a static method of a
synthetic class (`X$$ExternalSyntheticBackportN.m`) holding its own implementation.
`decompiler/managed/kuna-dex/src/backport.rs` recognises such a method by its body: a hash
(FNV-1a) of its descriptor, its opcodes, literals, branch offsets, strings, and the JDK
members and types it names, any other class counting as one name, and a call into another
synthetic method of the file standing for that method's own hash (d8 nests backports), with
a second hash that names such calls only, for a nested method in a peer file. The hashes
are looked up in `backports.tsv`, recorded by
`decompiler/managed/kuna-jvm/fixtures/dex/backports/regen.sh`: it turns d8's own list of
backported methods (`BackportedMethodList`, at min-api 1 to 35) into one probe method per
JDK method, compiles it with d8 at each level, debug and release, and hashes every
synthetic method a probe calls instead (`examples/backports.rs`); a hash two JDK methods
share is kept for `Math` over the identical `StrictMath` and dropped otherwise. A call to a
recognised method, here or in a peer file, is written as the JDK call (static, virtual, or
interface, the receiver first), and a synthetic class holding only recognised methods is
omitted. An unrecognised synthetic (another d8 version, `sun.misc` methods) stays as it is.

**Nest access.** javac 11 and later read a private member of a nestmate directly; d8
reaches a private field or method through a static synthetic accessor of the member's class
(`-$$Nest$fget...`, `-$$Nest$m...`) whose body is that one access on its parameters, and a
private constructor through a synthetic one taking an extra argument of an empty class
`X-IA` (always `null`). `decompiler/managed/kuna-dex/src/nest.rs` recognises both by their
bodies; before ops are built, a call of an accessor in the same file becomes the access
itself (a field read takes over the `move-result`, which becomes a `nop`) and a call of such
a constructor the call of the private one, without its last argument. The empty `-IA`
classes are omitted, the accessors stay (synthetic, for calls from other files), and every
class with nested classes gets `NestMembers` (all of them, at any depth) and every nested
class `NestHost` (its outermost class), at version 55, so the accesses are legal again.

**Records.** Below API 34 d8 gives a record the superclass `com.android.tools.r8.RecordTag`,
drops the `Record` attribute, and writes `toString` (`$record$getFieldsAsObjects`, the
class, the `;`-joined component names, a helper in `$$ExternalSyntheticRecordN`),
`hashCode` (the component fields to a helper) and `equals` (a private `$record$equals`) in
its own way. `decompiler/managed/kuna-dex/src/record.rs` recognises those three shapes; the
component order is the order of the names given to the `toString` helper or, when the
record writes its own `toString`, the order in which the canonical constructor's parameters
reach the fields (or match one field each by type). A recognised record gets
`java/lang/Record` back as its superclass (references to `RecordTag` in code too), a
`Record` attribute with each component's name, descriptor, `Signature` and annotations from
its field, and javac's methods: one `invokedynamic` to `ObjectMethods.bootstrap` with the
class, the names and a getter handle per component. The private helpers are left out, a
helper class only recognised records call is omitted, and `RecordTag` too when every class
of the file extending it was recognised.

**Classes.** `decompiler/managed/kuna-dex/src/class.rs` writes the members and the
attributes of 11.3 from the system annotations, and `decompiler/managed/kuna-dex/src/classfile.rs`
the constant pool and bytes. A class lists in `InnerClasses` itself and its outer chain, its
member classes, and every class of the file whose `EnclosingClass` or `EnclosingMethod`
names it (the engine links anonymous and local classes through their encloser's
attribute). Static values become `ConstantValue` for primitives and strings when non-zero
or the field is final; a final primitive field with no static value (d8 drops trailing
defaults) that `<clinit>` never writes gets its type's default. Since DEX does not mark a method's class as an interface, a static,
`super` or direct call is an interface call when the class is an interface defined in the
file or one of a short list of JDK interfaces. The class-file version is the lowest that
expresses the class: 50 (or the `min_version` option, down to 49), 51 for method-handle and
method-type constants, 52 for `invokedynamic`, interfaces with code, and interface calls
other than `invokeinterface`. Its `verifiable` option keeps classes at their minimum by
calling library interface methods through `Methodref` (verifiable, but failing if run) and
keeping d8's lambda, backport and record classes as d8 wrote them, for checking
translations with the JVM's own verifier.


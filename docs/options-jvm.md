# kuna JVM option catalog

**Generated** by `kuna catalog --markdown --engine jvm` from `decompiler/nonnative/kuna-jvm/options.toml` -- do not edit by hand; edit the table row and regenerate.

These options steer the JVM engine (`kuna-jvm`, a port of Vineflower 1.12.0), which decompiles class files, jars and directories of classes to Java, and classes compiled from Kotlin to Kotlin. They apply to those inputs only, as the options of `docs/options.md` apply to native inputs only. Set any of them per run on `decompile`, `decompile-all`, `decompile-project` and `functions`: `kuna decompile app.jar com.ex.Main --option <name> <value>` (repeatable). Every Vineflower option also answers to Vineflower's long and three-letter names (its `aliases`). Booleans take `on|off` (also `1|0|true|false`). Defaults are Vineflower 1.12.0's, its bundled plugins' included. kuna's own rows have no Vineflower name.

Vineflower keys kuna sets itself, so they are not options:

- `thread-count` -- --jobs: the driver runs one engine per thread
- `max-time-per-method` -- --max-fn-seconds: the driver's cooperative per-method deadline
- `method-to-decompile` -- method selection (decompile <class>.<method>) slices the class by its method spans
- `log-level` -- kuna's own logger
- `new-line-separator` -- output always uses \n
- `bytecode-source-mapping` -- line mappings are always collected (method spans)
- `user-renamer-class` -- not supported: a renamer class needs a JVM to load
- `include-classpath` -- not supported: there is no JVM class path; use --lib and jvm_include_runtime
- `dump-code-lines` -- line data of Vineflower's archive saver; kuna writes no archives
- `skip-extra-files` -- kuna copies no resources
- `decompile-preview` -- read nowhere in Vineflower 1.12.0: it has no effect
- `__unit_test_mode__` -- corpus runner only
- `__dump_original_lines__` -- corpus runner only
- `indent-string` -- fixed at Vineflower's default, three spaces
- `preferred-line-length` -- fixed at Vineflower's default, 160 columns; the corpus runner sets it for Vineflower's own tests
- `error-message` -- fixed: the note under a failure comment asks for reports to kuna's tracker; the corpus fixture sets it empty
- `dump-bytecode-on-error` -- fixed on: a failed method's comment lists its bytecode
- `dump-exception-on-error` -- fixed on: a failure comment names the exception; the corpus fixture turns it off
- `kt-unknown-defaults` -- fixed at the Kotlin plugin's default: ... stands for a default argument the $default stub does not reveal

Three tiers:

- **Toggleable transforms** -- restructure, remove, or insert code; the on/off control surface.
- **Input & analysis** -- what the engine reads: the class universe, libraries, the JDK.
- **Rendering defaults** -- how the text is written: Java or Kotlin, and how it is spelled.

## Symptom index

| If the output shows... | Try |
|---|---|
| method javap lists as bridge is missing from the output | [`jvm_remove_bridge`](#jvm_remove_bridge----on--off-default-on) |
| need the generated Object-typed forwarder of a generic override | [`jvm_remove_bridge`](#jvm_remove_bridge----on--off-default-on) |
| an obfuscated method marked bridge vanished | [`jvm_remove_bridge`](#jvm_remove_bridge----on--off-default-on) |
| code javap shows is missing from the decompiled class | [`jvm_remove_synthetic`](#jvm_remove_synthetic----on--off-default-on) |
| lambda$main$0 or access$000 body needed but hidden | [`jvm_remove_synthetic`](#jvm_remove_synthetic----on--off-default-on) |
| obfuscator flagged real methods synthetic and they disappeared | [`jvm_remove_synthetic`](#jvm_remove_synthetic----on--off-default-on) |
| access$000(this) calls left in the output | [`jvm_synthetic_not_set`](#jvm_synthetic_not_set----on--off-default-off) |
| this$0 or val$name fields printed in inner classes | [`jvm_synthetic_not_set`](#jvm_synthetic_not_set----on--off-default-off) |
| class$ helper calls remain in old class files | [`jvm_synthetic_not_set`](#jvm_synthetic_not_set----on--off-default-off) |
| "Could not verify finally blocks" comment with a semaphore variable | [`jvm_decompile_finally`](#jvm_decompile_finally----on--off-default-on) |
| cleanup code duplicated after every return | [`jvm_decompile_finally`](#jvm_decompile_finally----on--off-default-on) |
| catch (Throwable var) { ...; throw var; } instead of finally (option is off) | [`jvm_decompile_finally`](#jvm_decompile_finally----on--off-default-on) |
| $VF: Couldn't be decompiled with RuntimeException: Constructor ... not found | [`jvm_ignore_invalid_bytecode`](#jvm_ignore_invalid_bytecode----on--off-default-off) |
| method fails on a new expression in a patched class | [`jvm_ignore_invalid_bytecode`](#jvm_ignore_invalid_bytecode----on--off-default-off) |
| String s = (String)o; after an instanceof test (option is off) | [`jvm_pattern_matching`](#jvm_pattern_matching----on--off-default-on) |
| typeSwitch bootstrap call instead of a pattern switch | [`jvm_pattern_matching`](#jvm_pattern_matching----on--off-default-on) |
| pattern variable reconstructed wrongly | [`jvm_pattern_matching`](#jvm_pattern_matching----on--off-default-on) |
| switch statement assigning a temporary in every case where a switch expression was written | [`jvm_decompile_switch_expressions`](#jvm_decompile_switch_expressions----on--off-default-on) |
| yield statements misplaced | [`jvm_decompile_switch_expressions`](#jvm_decompile_switch_expressions----on--off-default-on) |
| one variable reused for two unrelated values | [`jvm_verify_merges`](#jvm_verify_merges----on--off-default-off) |
| variable read before its assignment in the output | [`jvm_verify_merges`](#jvm_verify_merges----on--off-default-off) |
| decompiled code does not recompile because of variable scoping | [`jvm_verify_merges`](#jvm_verify_merges----on--off-default-off) |
| i++ applied to the wrong variable after merging | [`jvm_verify_pre_post_merges`](#jvm_verify_pre_post_merges----on--off-default-off) |
| loop counter merged with another variable | [`jvm_verify_pre_post_merges`](#jvm_verify_pre_post_merges----on--off-default-off) |
| "Could not handle exception ranges with multiple entries" comment | [`jvm_old_try_dedup`](#jvm_old_try_dedup----on--off-default-off) |
| duplicated try blocks in an obfuscated method | [`jvm_old_try_dedup`](#jvm_old_try_dedup----on--off-default-off) |
| method with jsr/ret fails to decompile in a modern class file | [`jvm_force_jsr_inline`](#jvm_force_jsr_inline----on--off-default-off) |
| finally logic of a modern class built from subroutines | [`jvm_force_jsr_inline`](#jvm_force_jsr_inline----on--off-default-off) |
| a facade class prints only This is a multifile facade and a list of files | [`jvm_kt_decompile_facades`](#jvm_kt_decompile_facades----on--off-default-off) |
| functions of a @JvmMultifileClass facade are missing | [`jvm_kt_decompile_facades`](#jvm_kt_decompile_facades----on--off-default-off) |
| Outer$Inner written as a separate class instead of nested | [`jvm_decompile_inner`](#jvm_decompile_inner----on--off-default-on) |
| anonymous class body lands in the wrong method | [`jvm_decompile_inner`](#jvm_decompile_inner----on--off-default-on) |
| need each class file decompiled on its own | [`jvm_decompile_inner`](#jvm_decompile_inner----on--off-default-on) |
| anonymous class body inlined at the wrong new expression | [`jvm_verify_anonymous_classes`](#jvm_verify_anonymous_classes----on--off-default-off) |
| class used twice printed as an anonymous class | [`jvm_verify_anonymous_classes`](#jvm_verify_anonymous_classes----on--off-default-off) |
| "Inconsistent anonymous class" warnings wanted | [`jvm_verify_anonymous_classes`](#jvm_verify_anonymous_classes----on--off-default-off) |
| extra casts on calls into java.util or java.lang | [`jvm_include_runtime`](#jvm_include_runtime----on--off--current---default-on) |
| @Override missing on JDK interface methods | [`jvm_include_runtime`](#jvm_include_runtime----on--off--current---default-on) |
| generic arguments of JDK types erased | [`jvm_include_runtime`](#jvm_include_runtime----on--off--current---default-on) |
| output differs between machines with different JDKs | [`jvm_include_runtime`](#jvm_include_runtime----on--off--current---default-on) |
| no JDK found note on stderr | [`jvm_include_runtime`](#jvm_include_runtime----on--off--current---default-on) |
| class or method named like a Java keyword | [`jvm_rename_members`](#jvm_rename_members----on--off-default-off) |
| identifiers that are not valid Java (obfuscated names) | [`jvm_rename_members`](#jvm_rename_members----on--off-default-off) |
| many classes named a, b, c that clash | [`jvm_rename_members`](#jvm_rename_members----on--off-default-off) |
| inner and anonymous classes missing when decompiling one .class file | [`jvm_siblings`](#jvm_siblings----on--off-default-on) |
| unrelated Name$x.class files nested into the class | [`jvm_siblings`](#jvm_siblings----on--off-default-on) |
| nonsense generic types from a tampered Signature attribute | [`jvm_decompile_generics`](#jvm_decompile_generics----on--off-default-on) |
| need erased types that match the method descriptors | [`jvm_decompile_generics`](#jvm_decompile_generics----on--off-default-on) |
| raw List where List<String> was expected (option is off) | [`jvm_decompile_generics`](#jvm_decompile_generics----on--off-default-on) |
| Math.PI where the source had a numeric literal | [`jvm_keep_literals`](#jvm_keep_literals----on--off-default-off) |
| Long.MAX_VALUE or Double.MAX_VALUE instead of the number | [`jvm_keep_literals`](#jvm_keep_literals----on--off-default-off) |
| need raw digits of a float constant | [`jvm_keep_literals`](#jvm_keep_literals----on--off-default-off) |
| string literal shows odd or invisible characters | [`jvm_ascii_strings`](#jvm_ascii_strings----on--off-default-off) |
| homoglyphs in a compared string | [`jvm_ascii_strings`](#jvm_ascii_strings----on--off-default-off) |
| terminal mangles non-ASCII text in decompiled strings | [`jvm_ascii_strings`](#jvm_ascii_strings----on--off-default-off) |
| variables named like keywords or with misleading names from debug info | [`jvm_use_lvt_names`](#jvm_use_lvt_names----on--off-default-on) |
| LocalVariableTable names contradict their use | [`jvm_use_lvt_names`](#jvm_use_lvt_names----on--off-default-on) |
| var1 var2 names although debug info exists (option is off) | [`jvm_use_lvt_names`](#jvm_use_lvt_names----on--off-default-on) |
| parameter names differ from the debug-info names | [`jvm_use_method_parameters`](#jvm_use_method_parameters----on--off-default-on) |
| unexpected final on parameters | [`jvm_use_method_parameters`](#jvm_use_method_parameters----on--off-default-on) |
| /* $VF: ... */ and // $VF: comments clutter the output | [`jvm_decompiler_comments`](#jvm_decompiler_comments----on--off-default-on) |
| need to know where the decompiler approximated (option is off) | [`jvm_decompiler_comments`](#jvm_decompiler_comments----on--off-default-on) |
| stack trace names lambda$main$0 or Main$1 and the matching code is hard to find | [`jvm_mark_corresponding_synthetics`](#jvm_mark_corresponding_synthetics----on--off-default-off) |
| need the class file of an anonymous class | [`jvm_mark_corresponding_synthetics`](#jvm_mark_corresponding_synthetics----on--off-default-off) |
| Kotlin classes come out as Java with @Metadata(mv = ..., d1 = ...) | [`jvm_kt_enable`](#jvm_kt_enable----on--off-default-on) |
| a Kotlin class is written as .kt but Java output is wanted | [`jvm_kt_enable`](#jvm_kt_enable----on--off-default-on) |
| Kotlin output of a class is wrong or failed | [`jvm_kt_enable`](#jvm_kt_enable----on--off-default-on) |

## Toggleable transforms

Each of these restructures, removes or inserts code, so each stays flippable.

### `jvm_remove_bridge` -- on | off, default `on`

- **Symptoms:** method javap lists as bridge is missing from the output; need the generated Object-typed forwarder of a generic override; an obfuscated method marked bridge vanished.
- **What it does:** Leave out methods flagged ACC_BRIDGE: the forwarders javac generates for covariant return types and erased generic overrides.
- **When to flip:** Turn off to see the bridge forwarders a class really carries, e.g. when checking which erased signatures a library exposes or when an obfuscator hid code in a method marked bridge.
- **Where / provenance:** p9_emit/class_writer · vineflower · vineflower-option
- **Also spelled:** `remove-bridge`, `rbr`
- **Example:** `kuna decompile app.jar com.ex.Box --option jvm_remove_bridge off`

### `jvm_remove_synthetic` -- on | off, default `on`

- **Symptoms:** code javap shows is missing from the decompiled class; lambda$main$0 or access$000 body needed but hidden; obfuscator flagged real methods synthetic and they disappeared.
- **What it does:** Leave out synthetic fields, methods and member classes (this$0, val$x, access$000, lambda$main$0, switch-map classes) once their uses have been folded back into source form.
- **When to flip:** Turn off when reversing obfuscated or non-javac bytecode that keeps real code in members flagged synthetic, or to read the accessor and lambda implementation methods themselves.
- **Where / provenance:** p9_emit/class_writer · vineflower · vineflower-option
- **Also spelled:** `remove-synthetic`, `rsy`
- **Example:** `kuna decompile app.jar com.ex.Main --option jvm_remove_synthetic off`

### `jvm_synthetic_not_set` -- on | off, default `off`

- **Symptoms:** access$000(this) calls left in the output; this$0 or val$name fields printed in inner classes; class$ helper calls remain in old class files.
- **What it does:** Recognize compiler-generated members by their shape even without the ACC_SYNTHETIC flag: static accessor methods, outer-instance and captured-variable fields, and Java 1.4 class$ caches.
- **When to flip:** Turn on for class files from compilers or obfuscators that drop the synthetic flag, so access$NNN calls and this$0/val$x fields still fold back into source form.
- **Where / provenance:** p8_structure/nested_class_processor, nested_member_access, class_reference14_processor · vineflower · vineflower-option
- **Also spelled:** `synthetic-not-set`, `nns`
- **Example:** `kuna decompile obf.jar a.b --option jvm_synthetic_not_set on`

### `jvm_decompile_finally` -- on | off, default `on`

- **Symptoms:** "Could not verify finally blocks" comment with a semaphore variable; cleanup code duplicated after every return; catch (Throwable var) { ...; throw var; } instead of finally (option is off).
- **What it does:** Rebuild try/finally: find the finally code javac copies onto every exit of the try, keep one copy in a finally block and drop the catch-all handler that rethrows.
- **When to flip:** Turn off when the reconstructed finally is wrong or adds a semaphore variable, to see the duplicated copies and the catch (Throwable) handler as compiled.
- **Where / provenance:** p7_regions/finally_processor · vineflower · vineflower-option
- **Also spelled:** `decompile-finally`, `fdi`
- **Example:** `kuna decompile app.jar com.ex.Io --option jvm_decompile_finally off`

### `jvm_ignore_invalid_bytecode` -- on | off, default `off`

- **Symptoms:** $VF: Couldn't be decompiled with RuntimeException: Constructor ... not found; method fails on a new expression in a patched class.
- **What it does:** When a constructor call names a constructor its class does not declare (invalid or tampered bytecode), print the call without hiding synthetic arguments instead of failing the method with "Constructor .. not found".
- **When to flip:** Turn on for obfuscated or patched jars whose methods fail with a Constructor not found error.
- **Where / provenance:** substrate/expr_util · vineflower · vineflower-option
- **Also spelled:** `ignore-invalid-bytecode`, `iib`
- **Example:** `kuna decompile obf.jar a.b --option jvm_ignore_invalid_bytecode on`

### `jvm_pattern_matching` -- on | off, default `on`

- **Symptoms:** String s = (String)o; after an instanceof test (option is off); typeSwitch bootstrap call instead of a pattern switch; pattern variable reconstructed wrongly.
- **What it does:** Rebuild instanceof patterns (if (o instanceof String s)) and pattern switches (case Point(int x, int y) ->) from the checkcast and SwitchBootstraps.typeSwitch code of Java 16+ class files.
- **When to flip:** Turn off to see the explicit casts and typeSwitch bootstrap calls, or for output that older Java versions can compile.
- **Where / provenance:** p8_structure/if_pattern_match_processor, switch_pattern_match_processor, method_processor · vineflower · vineflower-option
- **Also spelled:** `pattern-matching`, `pam`
- **Example:** `kuna decompile app.jar com.ex.Shapes --option jvm_pattern_matching off`

### `jvm_decompile_switch_expressions` -- on | off, default `on`

- **Symptoms:** switch statement assigning a temporary in every case where a switch expression was written; yield statements misplaced.
- **What it does:** Rebuild Java 14+ switch expressions (var x = switch (k) { case A -> 1; default -> { yield 2; } }) from the statement switch and temporary javac emits.
- **When to flip:** Turn off to see the statement switch and assignments, or for output that pre-14 Java can compile.
- **Where / provenance:** p8_structure/switch_expression_helper · vineflower · vineflower-option
- **Also spelled:** `decompile-switch-expressions`, `swe`
- **Example:** `kuna decompile app.jar com.ex.Codes --option jvm_decompile_switch_expressions off`

### `jvm_verify_merges` -- on | off, default `off`

- **Symptoms:** one variable reused for two unrelated values; variable read before its assignment in the output; decompiled code does not recompile because of variable scoping.
- **What it does:** Before merging two variable versions into one variable, check that the merge cannot change which value a read sees (experimental).
- **When to flip:** Turn on when one variable seems to hold two unrelated values or a value is read before it could have been assigned.
- **Where / provenance:** p6_variables/var_definition_helper · vineflower · vineflower-option
- **Also spelled:** `verify-merges`, `vvm`
- **Example:** `kuna decompile app.jar com.ex.Loop --option jvm_verify_merges on`

### `jvm_verify_pre_post_merges` -- on | off, default `off`

- **Symptoms:** i++ applied to the wrong variable after merging; loop counter merged with another variable.
- **What it does:** Compute, with an extra SSA pass, which assignments each variable read can see, and refuse variable merges that would change it (experimental).
- **When to flip:** Turn on when merged variables around ++/-- or loops produce wrong values in the output.
- **Where / provenance:** p6_variables/var_definition_helper · vineflower · vineflower-option
- **Also spelled:** `verify-pre-post-merges`, `pvm`
- **Example:** `kuna decompile app.jar com.ex.Loop --option jvm_verify_pre_post_merges on`

### `jvm_old_try_dedup` -- on | off, default `off`

- **Symptoms:** "Could not handle exception ranges with multiple entries" comment; duplicated try blocks in an obfuscated method.
- **What it does:** For methods with obfuscated exception ranges (several entry points), insert dummy exception handlers instead of duplicating the blocks the ranges share (experimental).
- **When to flip:** Turn on when an obfuscated method's duplicated try blocks blow up the output or fail to structure.
- **Where / provenance:** p2_lift/exception_deobfuscator · vineflower · vineflower-option
- **Also spelled:** `old-try-dedup`
- **Example:** `kuna decompile obf.jar a.b --option jvm_old_try_dedup on`

### `jvm_force_jsr_inline` -- on | off, default `off`

- **Symptoms:** method with jsr/ret fails to decompile in a modern class file; finally logic of a modern class built from subroutines.
- **What it does:** Inline jsr/ret subroutines in every class file; normally only class files of Java 6 or older, where they are legal, get this.
- **When to flip:** Turn on for hand-assembled or obfuscated Java 7+ class files that use jsr/ret anyway and fail to decompile.
- **Where / provenance:** infra/method_processor · vineflower · vineflower-option
- **Also spelled:** `force-jsr-inline`, `fji`
- **Example:** `kuna decompile obf.jar a.b --option jvm_force_jsr_inline on`

### `jvm_kt_decompile_facades` -- on | off, default `off`

- **Symptoms:** a facade class prints only This is a multifile facade and a list of files; functions of a @JvmMultifileClass facade are missing.
- **What it does:** Kotlin plugin: also write the wrapper functions of a multifile facade class (@file:JvmMultifileClass); off writes only the comment that lists the part classes holding the real implementations.
- **When to flip:** Turn on to see a facade's forwarding functions, for example when callers in Java code call the facade and you need its exact signatures.
- **Where / provenance:** kotlin/kotlin_writer · vineflower · vineflower-option
- **Also spelled:** `kt-decompile-facades`
- **Example:** `kuna decompile app.jar com.ex.UtilsKt --option jvm_kt_decompile_facades on`

## Input & analysis

What the engine loads and infers before any class is written.

### `jvm_decompile_inner` -- on | off, default `on`

- **Symptoms:** Outer$Inner written as a separate class instead of nested; anonymous class body lands in the wrong method; need each class file decompiled on its own.
- **What it does:** Nest member, local and anonymous classes inside their enclosing class, as the InnerClasses and EnclosingMethod attributes describe; off writes every class file as its own top-level class.
- **When to flip:** Turn off when InnerClasses attributes are stripped or forged and nesting goes wrong, or to get Outer$Inner as a separate compilation unit.
- **Where / provenance:** p1_classfile/classes_processor · vineflower · vineflower-option
- **Also spelled:** `decompile-inner`, `din`
- **Example:** `kuna decompile-project app.jar -o out --option jvm_decompile_inner off`

### `jvm_verify_anonymous_classes` -- on | off, default `off`

- **Symptoms:** anonymous class body inlined at the wrong new expression; class used twice printed as an anonymous class; "Inconsistent anonymous class" warnings wanted.
- **What it does:** Before treating a class as anonymous, check that it has one supertype and that its enclosing class creates it with new exactly once and never casts to, tests for or reads statics of it; a class failing the check is written as a local class.
- **When to flip:** Turn on when anonymous-class bodies end up inlined in the wrong place, typically with obfuscated InnerClasses attributes.
- **Where / provenance:** p1_classfile/classes_processor · vineflower · vineflower-option
- **Also spelled:** `verify-anonymous-classes`, `vac`
- **Example:** `kuna decompile obf.jar a.b --option jvm_verify_anonymous_classes on`

### `jvm_include_runtime` -- on | off | current | <path>, default `on`

- **Symptoms:** extra casts on calls into java.util or java.lang; @Override missing on JDK interface methods; generic arguments of JDK types erased; output differs between machines with different JDKs; no JDK found note on stderr.
- **What it does:** Give the decompiler a JDK's class library as context: on/current finds the current JDK (JAVA_HOME, then java on PATH, then usual install directories), a path names a JDK home, off uses none.
- **When to flip:** Point it at the JDK the code was built for when casts, generics or overloads on java.* types come out wrong; use off for reproducible output that does not depend on the machine's JDK.
- **Where / provenance:** p0_knowledge/fernflower, jdk_runtime · vineflower · vineflower-option
- **Also spelled:** `include-runtime`, `jrt`
- **Example:** `kuna decompile app.jar com.ex.Main --option jvm_include_runtime /usr/lib/jvm/java-21`

### `jvm_rename_members` -- on | off, default `off`

- **Symptoms:** class or method named like a Java keyword; identifiers that are not valid Java (obfuscated names); many classes named a, b, c that clash.
- **What it does:** Rename classes, fields and methods whose names are not valid Java identifiers, are reserved words, or are short obfuscated names, to class_N/field_N/method_N, rewriting every reference and noting the original name.
- **When to flip:** Turn on for obfuscated jars whose names (if, do, a, ., unicode garbage) make the output unreadable or uncompilable.
- **Where / provenance:** p0_knowledge/fernflower, identifier_converter, converter_helper · vineflower · vineflower-option
- **Also spelled:** `rename-members`, `ren`
- **Example:** `kuna decompile-project obf.jar -o out --option jvm_rename_members on`

### `jvm_siblings` -- on | off, default `on`

- **Symptoms:** inner and anonymous classes missing when decompiling one .class file; unrelated Name$x.class files nested into the class.
- **What it does:** For a single .class input, also load the Name$*.class files next to it, so nested, local and anonymous classes are decompiled inside it.
- **When to flip:** Turn off to decompile exactly the given class file, e.g. when a directory holds unrelated classes with the same prefix.
- **Where / provenance:** p0_knowledge/kuna_input · kuna · kuna-input
- **Example:** `kuna decompile Main.class --option jvm_siblings off`

## Rendering defaults

How the text is written: its language (Java, or Kotlin for Kotlin classes), formatting, literals, comments.

### `jvm_decompile_generics` -- on | off, default `on`

- **Symptoms:** nonsense generic types from a tampered Signature attribute; need erased types that match the method descriptors; raw List where List<String> was expected (option is off).
- **What it does:** Read Signature and LocalVariableTypeTable attributes so classes, methods, fields and locals print with their generic types (List<String>, <T extends Comparable<T>>); off prints erased types.
- **When to flip:** Turn off when signatures are corrupt or forged (obfuscators) and generic types come out wrong or cause failures; erased types then follow the descriptors exactly.
- **Where / provenance:** p1_classfile/struct_class, struct_method, struct_field, var_exprent · vineflower · vineflower-option
- **Also spelled:** `decompile-generics`, `dgs`
- **Example:** `kuna decompile app.jar com.ex.Repo --option jvm_decompile_generics off`

### `jvm_keep_literals` -- on | off, default `off`

- **Symptoms:** Math.PI where the source had a numeric literal; Long.MAX_VALUE or Double.MAX_VALUE instead of the number; need raw digits of a float constant.
- **What it does:** Print floating-point and long constants as plain literals (0.0F / 0.0F for NaN, 1.0 / 0.0 for infinity, 3.141592653589793) instead of resugaring them to Double.NaN, Double.POSITIVE_INFINITY, Long.MAX_VALUE or Math.PI.
- **When to flip:** Turn on when the exact numeric value matters more than the named constant, e.g. when a magic number happens to equal Math.PI or Long.MAX_VALUE.
- **Where / provenance:** substrate/const_exprent · vineflower · vineflower-option
- **Also spelled:** `keep-literals`, `lit`
- **Example:** `kuna decompile app.jar com.ex.Math2 --option jvm_keep_literals on`

### `jvm_ascii_strings` -- on | off, default `off`

- **Symptoms:** string literal shows odd or invisible characters; homoglyphs in a compared string; terminal mangles non-ASCII text in decompiled strings.
- **What it does:** Escape every non-ASCII character of string and char literals as \uXXXX.
- **When to flip:** Turn on for strings with invisible, right-to-left or homoglyph characters (obfuscated keys, spoofed names) or when the output must be pure ASCII.
- **Where / provenance:** substrate/const_exprent · vineflower · vineflower-option
- **Also spelled:** `ascii-strings`, `asc`
- **Example:** `kuna decompile app.jar com.ex.Keys --option jvm_ascii_strings on`

### `jvm_use_lvt_names` -- on | off, default `on`

- **Symptoms:** variables named like keywords or with misleading names from debug info; LocalVariableTable names contradict their use; var1 var2 names although debug info exists (option is off).
- **What it does:** Name parameters and locals from the LocalVariableTable (debug info) and take local generic types from it, instead of var1/var2-style names.
- **When to flip:** Turn off when the debug names are misleading (obfuscated or forged LocalVariableTable) or clash with keywords.
- **Where / provenance:** p1_classfile/class_wrapper, var_exprent · vineflower · vineflower-option
- **Also spelled:** `use-lvt-names`, `udv`
- **Example:** `kuna decompile obf.jar a.b --option jvm_use_lvt_names off`

### `jvm_use_method_parameters` -- on | off, default `on`

- **Symptoms:** parameter names differ from the debug-info names; unexpected final on parameters.
- **What it does:** Name parameters, and mark them final, from the MethodParameters attribute (javac -parameters, records) when a method has one.
- **When to flip:** Turn off when the MethodParameters names are forged or worse than the LocalVariableTable ones.
- **Where / provenance:** p9_emit/class_writer, var_definition_helper · vineflower · vineflower-option
- **Also spelled:** `use-method-parameters`, `ump`
- **Example:** `kuna decompile app.jar com.ex.Api --option jvm_use_method_parameters off`

### `jvm_decompiler_comments` -- on | off, default `on`

- **Symptoms:** /* $VF: ... */ and // $VF: comments clutter the output; need to know where the decompiler approximated (option is off).
- **What it does:** Write $VF: comments where the decompiler had to bend the code: irreducible flow, unverifiable finally blocks, MethodType constants, constructors it could not resugar, ambiguous variable types.
- **When to flip:** Turn off for output free of decompiler remarks; keep on to know which parts of the code to distrust.
- **Where / provenance:** p9_emit/class_writer, finally_processor, assignment_exprent, const_exprent, invocation_exprent · vineflower · vineflower-option
- **Also spelled:** `decompiler-comments`, `dec`
- **Example:** `kuna decompile app.jar com.ex.Main --option jvm_decompiler_comments off`

### `jvm_mark_corresponding_synthetics` -- on | off, default `off`

- **Symptoms:** stack trace names lambda$main$0 or Main$1 and the matching code is hard to find; need the class file of an anonymous class.
- **What it does:** Write a comment naming the synthetic construct behind each lambda (its lambda$name$N method) and each anonymous or local class (its Outer$N class).
- **When to flip:** Turn on to map lambdas and anonymous classes in the output back to the class-file members and stack-trace names they compile to.
- **Where / provenance:** p9_emit/class_writer · vineflower · vineflower-option
- **Also spelled:** `mark-corresponding-synthetics`, `mcs`
- **Example:** `kuna decompile app.jar com.ex.Handlers --option jvm_mark_corresponding_synthetics on`

### `jvm_kt_enable` -- on | off, default `on`

- **Symptoms:** Kotlin classes come out as Java with @Metadata(mv = ..., d1 = ...); a Kotlin class is written as .kt but Java output is wanted; Kotlin output of a class is wrong or failed.
- **What it does:** Kotlin plugin: write classes that carry @kotlin.Metadata as Kotlin source (.kt) through the plugin's own pipeline and writer; off writes them as Java (.java) with the @Metadata annotation, like any other class.
- **When to flip:** Turn off to see the Java view of a Kotlin class (accessor methods, $default bridges, Intrinsics checks, every JVM method on its own), or when the Kotlin output of a class is wrong or fails and the Java output is enough; the CLI's --language java turns it off for the run.
- **Where / provenance:** kotlin/kotlin_chooser · vineflower · vineflower-option
- **Also spelled:** `kt-enable`
- **Example:** `kuna decompile app.jar com.ex.UserKt --option jvm_kt_enable off`

## Programmatic use

```bash
# discover (machine-readable; includes tier, symptoms, aliases):
kuna catalog --engine jvm --json

# transforms only:
kuna catalog --engine jvm --tier transform

# decompile with an option flipped (repeatable):
kuna decompile app.jar com.ex.Main --option jvm_decompile_finally off
```

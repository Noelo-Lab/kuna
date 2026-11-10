# kuna JVM option catalog

**Generated** by `kuna catalog --markdown --engine jvm` from `decompiler/nonnative/kuna-jvm/options.toml` -- do not edit by hand; edit the table row and regenerate.

These options steer the JVM engine (`kuna-jvm`, a port of Vineflower 1.12.0), which decompiles class files, jars and directories of classes to Java, and classes compiled from Kotlin to Kotlin. They apply to those inputs only, as the options of `docs/options.md` apply to native inputs only. Set any of them per run on `decompile`, `decompile-all`, `decompile-project` and `functions`: `kuna decompile app.jar com.ex.Main --option <name> <value>` (repeatable). Every Vineflower option also answers to Vineflower's long and three-letter names (its `aliases`). Booleans take `on|off` (also `1|0|true|false`). Defaults are Vineflower 1.12.0's, its bundled plugins' included. kuna's own rows have no Vineflower name; among them, `jvm_vineflower_compat on` turns off kuna's fixes of Vineflower's defects (which have no options of their own) for Vineflower's exact text.

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

Three tiers:

- **Toggleable transforms** -- restructure, remove, or insert code; the on/off control surface.
- **Input & analysis** -- what the engine reads: the class universe, libraries, the JDK.
- **Rendering defaults** -- how the text is written: Java or Kotlin, and how it is spelled.

## Symptom index

| If the output shows... | Try |
|---|---|
| method javap lists as bridge is missing from the output | [`jvm_remove_bridge`](#jvm_remove_bridge) |
| need the generated Object-typed forwarder of a generic override | [`jvm_remove_bridge`](#jvm_remove_bridge) |
| an obfuscated method marked bridge vanished | [`jvm_remove_bridge`](#jvm_remove_bridge) |
| code javap shows is missing from the decompiled class | [`jvm_remove_synthetic`](#jvm_remove_synthetic) |
| lambda$main$0 or access$000 body needed but hidden | [`jvm_remove_synthetic`](#jvm_remove_synthetic) |
| obfuscator flagged real methods synthetic and they disappeared | [`jvm_remove_synthetic`](#jvm_remove_synthetic) |
| class$("java.lang.String") calls instead of String.class | [`jvm_decompile_java4`](#jvm_decompile_java4) |
| static Class class$java$lang$String fields in old class files | [`jvm_decompile_java4`](#jvm_decompile_java4) |
| need the raw Class.forName cache logic | [`jvm_decompile_java4`](#jvm_decompile_java4) |
| if (!$assertionsDisabled && !cond) throw new AssertionError() instead of assert cond | [`jvm_decompile_assert`](#jvm_decompile_assert) |
| $assertionsDisabled field visible | [`jvm_decompile_assert`](#jvm_decompile_assert) |
| assert hides a check that must be read literally | [`jvm_decompile_assert`](#jvm_decompile_assert) |
| constructor shows no super() call | [`jvm_hide_empty_super`](#jvm_hide_empty_super) |
| need to see the implicit superclass constructor invocation | [`jvm_hide_empty_super`](#jvm_hide_empty_super) |
| public Foo() {} constructor missing from the output | [`jvm_hide_default_constructor`](#jvm_hide_default_constructor) |
| class seems to have no constructor | [`jvm_hide_default_constructor`](#jvm_hide_default_constructor) |
| var = x; inside try followed by return var after the catch | [`jvm_incorporate_returns`](#jvm_incorporate_returns) |
| return inside try reads differently from the bytecode | [`jvm_incorporate_returns`](#jvm_incorporate_returns) |
| synchronized block covers more code than expected | [`jvm_ensure_synchronized_monitors`](#jvm_ensure_synchronized_monitors) |
| MonitorEnter/MonitorExit pseudo-calls instead of synchronized | [`jvm_ensure_synchronized_monitors`](#jvm_ensure_synchronized_monitors) |
| unbalanced monitor in handwritten bytecode | [`jvm_ensure_synchronized_monitors`](#jvm_ensure_synchronized_monitors) |
| enum written as class extends Enum with $VALUES | [`jvm_decompile_enums`](#jvm_decompile_enums) |
| need the generated values()/valueOf() bodies | [`jvm_decompile_enums`](#jvm_decompile_enums) |
| enum constant initializers hide tampered code | [`jvm_decompile_enums`](#jvm_decompile_enums) |
| stray outer.getClass(); before outer.new Inner() | [`jvm_remove_getclass`](#jvm_remove_getclass) |
| Objects.requireNonNull-like check before an inner class instantiation | [`jvm_remove_getclass`](#jvm_remove_getclass) |
| access$000(this) calls left in the output | [`jvm_synthetic_not_set`](#jvm_synthetic_not_set) |
| this$0 or val$name fields printed in inner classes | [`jvm_synthetic_not_set`](#jvm_synthetic_not_set) |
| class$ helper calls remain in old class files | [`jvm_synthetic_not_set`](#jvm_synthetic_not_set) |
| exception handler code missing from the output | [`jvm_remove_empty_try_catch`](#jvm_remove_empty_try_catch) |
| empty try { } catch blocks wanted for inspection | [`jvm_remove_empty_try_catch`](#jvm_remove_empty_try_catch) |
| "Could not verify finally blocks" comment with a semaphore variable | [`jvm_decompile_finally`](#jvm_decompile_finally) |
| cleanup code duplicated after every return | [`jvm_decompile_finally`](#jvm_decompile_finally) |
| catch (Throwable var) { ...; throw var; } instead of finally (option is off) | [`jvm_decompile_finally`](#jvm_decompile_finally) |
| need the interface method a lambda implements | [`jvm_lambda_to_anonymous_class`](#jvm_lambda_to_anonymous_class) |
| output must compile with Java 7 | [`jvm_lambda_to_anonymous_class`](#jvm_lambda_to_anonymous_class) |
| x -> ... where an anonymous class is wanted | [`jvm_lambda_to_anonymous_class`](#jvm_lambda_to_anonymous_class) |
| $VF: Couldn't be decompiled with RuntimeException: Constructor ... not found | [`jvm_ignore_invalid_bytecode`](#jvm_ignore_invalid_bytecode) |
| method fails on a new expression in a patched class | [`jvm_ignore_invalid_bytecode`](#jvm_ignore_invalid_bytecode) |
| a ? true : b where a logical or was written | [`jvm_ternary_constant_simplification`](#jvm_ternary_constant_simplification) |
| nested ternaries with true and false branches | [`jvm_ternary_constant_simplification`](#jvm_ternary_constant_simplification) |
| x ? false : true instead of !x | [`jvm_ternary_constant_simplification`](#jvm_ternary_constant_simplification) |
| String s = (String)o; after an instanceof test (option is off) | [`jvm_pattern_matching`](#jvm_pattern_matching) |
| typeSwitch bootstrap call instead of a pattern switch | [`jvm_pattern_matching`](#jvm_pattern_matching) |
| pattern variable reconstructed wrongly | [`jvm_pattern_matching`](#jvm_pattern_matching) |
| while (true) with a try inside that does not match the bytecode | [`jvm_try_loop_fix`](#jvm_try_loop_fix) |
| loop condition moved into a try block | [`jvm_try_loop_fix`](#jvm_try_loop_fix) |
| if (a ? b : c) condition hard to read | [`jvm_ternary_in_if`](#jvm_ternary_in_if) |
| duplicated if bodies where a ternary condition was compiled (option is off) | [`jvm_ternary_in_if`](#jvm_ternary_in_if) |
| switch statement assigning a temporary in every case where a switch expression was written | [`jvm_decompile_switch_expressions`](#jvm_decompile_switch_expressions) |
| yield statements misplaced | [`jvm_decompile_switch_expressions`](#jvm_decompile_switch_expressions) |
| var10000 temporaries everywhere (option is off) | [`jvm_simplify_stack`](#jvm_simplify_stack) |
| compound assignment merges side effects in the wrong order | [`jvm_simplify_stack`](#jvm_simplify_stack) |
| arr[i++] = v reads differently from the bytecode | [`jvm_simplify_stack`](#jvm_simplify_stack) |
| one variable reused for two unrelated values | [`jvm_verify_merges`](#jvm_verify_merges) |
| variable read before its assignment in the output | [`jvm_verify_merges`](#jvm_verify_merges) |
| decompiled code does not recompile because of variable scoping | [`jvm_verify_merges`](#jvm_verify_merges) |
| i++ applied to the wrong variable after merging | [`jvm_verify_pre_post_merges`](#jvm_verify_pre_post_merges) |
| loop counter merged with another variable | [`jvm_verify_pre_post_merges`](#jvm_verify_pre_post_merges) |
| "Could not handle exception ranges with multiple entries" comment | [`jvm_old_try_dedup`](#jvm_old_try_dedup) |
| duplicated try blocks in an obfuscated method | [`jvm_old_try_dedup`](#jvm_old_try_dedup) |
| condy bootstrap call with ConstantBootstraps.fieldVarHandle in the output | [`jvm_decompile_complex_constant_dynamic`](#jvm_decompile_complex_constant_dynamic) |
| VarHandle constant shown as an unresolved dynamic constant | [`jvm_decompile_complex_constant_dynamic`](#jvm_decompile_complex_constant_dynamic) |
| method with jsr/ret fails to decompile in a modern class file | [`jvm_force_jsr_inline`](#jvm_force_jsr_inline) |
| finally logic of a modern class built from subroutines | [`jvm_force_jsr_inline`](#jvm_force_jsr_inline) |
| else branch missing after an if that returns | [`jvm_prettify_ifs`](#jvm_prettify_ifs) |
| deep else chains after returns (option is off) | [`jvm_prettify_ifs`](#jvm_prettify_ifs) |
| a local variable of the source is missing | [`jvm_source_locals`](#jvm_source_locals) |
| x = e; return x; printed as return e; | [`jvm_source_locals`](#jvm_source_locals) |
| declarations reordered against the source | [`jvm_source_locals`](#jvm_source_locals) |
| recompiled bytecode uses other local slots | [`jvm_source_locals`](#jvm_source_locals) |
| want Vineflower's folded form of locals | [`jvm_source_locals`](#jvm_source_locals) |
| if condition negated against the source with the branches swapped | [`jvm_javac_layout`](#jvm_javac_layout) |
| early return or continue missing and the rest of the method nested in an if | [`jvm_javac_layout`](#jvm_javac_layout) |
| return c ? a : b where the source has two returns | [`jvm_javac_layout`](#jvm_javac_layout) |
| return c; where the source has if (c) return true; return false; | [`jvm_javac_layout`](#jvm_javac_layout) |
| else branch after an if that returns | [`jvm_javac_layout`](#jvm_javac_layout) |
| locals named var1 var2 var3 with no hint of their type | [`jvm_variable_renaming`](#jvm_variable_renaming) |
| obfuscated or meaningless local names from a forged LocalVariableTable | [`jvm_variable_renaming`](#jvm_variable_renaming) |
| names differ from a Vineflower run with --variable-renaming=jad or tiny | [`jvm_variable_renaming`](#jvm_variable_renaming) |
| locals renamed but parameters still named var1 or paramN | [`jvm_rename_parameters`](#jvm_rename_parameters) |
| parameter names do not follow the renamer's scheme | [`jvm_rename_parameters`](#jvm_rename_parameters) |
| an old Vineflower command line with --jad-style-variable-naming=1 gives JAD-style names | [`jvm_jad_style_variable_naming`](#jvm_jad_style_variable_naming) |
| locals named var1 var2 where JAD-style names were expected | [`jvm_jad_style_variable_naming`](#jvm_jad_style_variable_naming) |
| an old Vineflower command line with --jad-style-parameter-naming=1 renames parameters | [`jvm_jad_style_parameter_naming`](#jvm_jad_style_parameter_naming) |
| parameters keep their names under the jad renamer | [`jvm_jad_style_parameter_naming`](#jvm_jad_style_parameter_naming) |
| if (x == null) { $$$reportNull$$$0(0); } at the top of methods | [`jvm_resugar_idea_notnull`](#jvm_resugar_idea_notnull) |
| explicit null checks on @NotNull parameters or before returns (option is off) | [`jvm_resugar_idea_notnull`](#jvm_resugar_idea_notnull) |
| every Kotlin declaration starts with public | [`jvm_kt_show_public`](#jvm_kt_show_public) |
| public modifiers that Kotlin source would omit | [`jvm_kt_show_public`](#jvm_kt_show_public) |
| parameters written as x: Int = ... | [`jvm_kt_unknown_defaults`](#jvm_kt_unknown_defaults) |
| default argument values missing from a Kotlin function signature | [`jvm_kt_unknown_defaults`](#jvm_kt_unknown_defaults) |
| "a" + b + "c" in Kotlin output where a template was written | [`jvm_kt_collapse_string_concat`](#jvm_kt_collapse_string_concat) |
| string templates hide the concatenation order | [`jvm_kt_collapse_string_concat`](#jvm_kt_collapse_string_concat) |
| a facade class prints only This is a multifile facade and a list of files | [`jvm_kt_decompile_facades`](#jvm_kt_decompile_facades) |
| functions of a @JvmMultifileClass facade are missing | [`jvm_kt_decompile_facades`](#jvm_kt_decompile_facades) |
| Outer$Inner written as a separate class instead of nested | [`jvm_decompile_inner`](#jvm_decompile_inner) |
| anonymous class body lands in the wrong method | [`jvm_decompile_inner`](#jvm_decompile_inner) |
| need each class file decompiled on its own | [`jvm_decompile_inner`](#jvm_decompile_inner) |
| anonymous class body inlined at the wrong new expression | [`jvm_verify_anonymous_classes`](#jvm_verify_anonymous_classes) |
| class used twice printed as an anonymous class | [`jvm_verify_anonymous_classes`](#jvm_verify_anonymous_classes) |
| "Inconsistent anonymous class" warnings wanted | [`jvm_verify_anonymous_classes`](#jvm_verify_anonymous_classes) |
| extra casts on calls into java.util or java.lang | [`jvm_include_runtime`](#jvm_include_runtime) |
| @Override missing on JDK interface methods | [`jvm_include_runtime`](#jvm_include_runtime) |
| generic arguments of JDK types erased | [`jvm_include_runtime`](#jvm_include_runtime) |
| output differs between machines with different JDKs | [`jvm_include_runtime`](#jvm_include_runtime) |
| no JDK found note on stderr | [`jvm_include_runtime`](#jvm_include_runtime) |
| class or method named like a Java keyword | [`jvm_rename_members`](#jvm_rename_members) |
| identifiers that are not valid Java (obfuscated names) | [`jvm_rename_members`](#jvm_rename_members) |
| many classes named a, b, c that clash | [`jvm_rename_members`](#jvm_rename_members) |
| "Inconsistent inner class entries" warnings flood stderr | [`jvm_warn_inconsistent_inner_attributes`](#jvm_warn_inconsistent_inner_attributes) |
| nested class appears under an unexpected outer class | [`jvm_warn_inconsistent_inner_attributes`](#jvm_warn_inconsistent_inner_attributes) |
| decompiling a fat jar spends time on bundled libraries | [`jvm_excluded_classes`](#jvm_excluded_classes) |
| output includes generated or third-party classes | [`jvm_excluded_classes`](#jvm_excluded_classes) |
| only one package of a large jar is of interest | [`jvm_included_classes`](#jvm_included_classes) |
| decompile-project takes too long on the whole jar | [`jvm_included_classes`](#jvm_included_classes) |
| member classes of an obfuscated jar written as separate top-level classes | [`jvm_validate_inner_classes_names`](#jvm_validate_inner_classes_names) |
| renamed inner classes not nested | [`jvm_validate_inner_classes_names`](#jvm_validate_inner_classes_names) |
| inner and anonymous classes missing when decompiling one .class file | [`jvm_siblings`](#jvm_siblings) |
| unrelated Name$x.class files nested into the class | [`jvm_siblings`](#jvm_siblings) |
| a Vineflower command line to replay sets --kt-export-metadata=1 (the output is the same either way) | [`jvm_kt_export_metadata`](#jvm_kt_export_metadata) |
| nonsense generic types from a tampered Signature attribute | [`jvm_decompile_generics`](#jvm_decompile_generics) |
| need erased types that match the method descriptors | [`jvm_decompile_generics`](#jvm_decompile_generics) |
| raw List where List<String> was expected (option is off) | [`jvm_decompile_generics`](#jvm_decompile_generics) |
| Math.PI where the source had a numeric literal | [`jvm_keep_literals`](#jvm_keep_literals) |
| Long.MAX_VALUE or Double.MAX_VALUE instead of the number | [`jvm_keep_literals`](#jvm_keep_literals) |
| need raw digits of a float constant | [`jvm_keep_literals`](#jvm_keep_literals) |
| int variable printed as boolean true/false | [`jvm_boolean_as_int`](#jvm_boolean_as_int) |
| 1 and 0 where true and false were expected (option is off) | [`jvm_boolean_as_int`](#jvm_boolean_as_int) |
| string literal shows odd or invisible characters | [`jvm_ascii_strings`](#jvm_ascii_strings) |
| homoglyphs in a compared string | [`jvm_ascii_strings`](#jvm_ascii_strings) |
| terminal mangles non-ASCII text in decompiled strings | [`jvm_ascii_strings`](#jvm_ascii_strings) |
| <undefinedtype> in a parameter list (option is off) | [`jvm_undefined_as_object`](#jvm_undefined_as_object) |
| lambda parameter typed Object where a type is expected | [`jvm_undefined_as_object`](#jvm_undefined_as_object) |
| variables named like keywords or with misleading names from debug info | [`jvm_use_lvt_names`](#jvm_use_lvt_names) |
| LocalVariableTable names contradict their use | [`jvm_use_lvt_names`](#jvm_use_lvt_names) |
| var1 var2 names although debug info exists (option is off) | [`jvm_use_lvt_names`](#jvm_use_lvt_names) |
| parameter names differ from the debug-info names | [`jvm_use_method_parameters`](#jvm_use_method_parameters) |
| unexpected final on parameters | [`jvm_use_method_parameters`](#jvm_use_method_parameters) |
| string switch reconstructed wrongly and the hidden hashCode switch is needed | [`jvm_show_hidden_statements`](#jvm_show_hidden_statements) |
| debugging which switch the decompiler discarded | [`jvm_show_hidden_statements`](#jvm_show_hidden_statements) |
| @Override missing on toString or interface methods (supertype not loaded) | [`jvm_override_annotation`](#jvm_override_annotation) |
| @Override that javac did not write is unwanted | [`jvm_override_annotation`](#jvm_override_annotation) |
| decompiled code fails to recompile with an inference error | [`jvm_explicit_generics`](#jvm_explicit_generics) |
| need the type argument of a generic method call | [`jvm_explicit_generics`](#jvm_explicit_generics) |
| x -> { return x + 1; } where x -> x + 1 is wanted (option is off) | [`jvm_inline_simple_lambdas`](#jvm_inline_simple_lambdas) |
| one-line lambdas hard to diff | [`jvm_inline_simple_lambdas`](#jvm_inline_simple_lambdas) |
| three-space indentation does not match the project style | [`jvm_indent_string`](#jvm_indent_string) |
| output too wide when nested deeply | [`jvm_indent_string`](#jvm_indent_string) |
| very long lines in builder chains | [`jvm_preferred_line_length`](#jvm_preferred_line_length) |
| expressions wrapped across lines unexpectedly | [`jvm_preferred_line_length`](#jvm_preferred_line_length) |
| decompiled files need a provenance or license header | [`jvm_banner`](#jvm_banner) |
| failure comment asks to report to an issue tracker | [`jvm_error_message`](#jvm_error_message) |
| failure comment should carry a project-specific note | [`jvm_error_message`](#jvm_error_message) |
| failed method shows a long // Bytecode: listing | [`jvm_dump_bytecode_on_error`](#jvm_dump_bytecode_on_error) |
| need the instructions of a method that failed to decompile (option is off) | [`jvm_dump_bytecode_on_error`](#jvm_dump_bytecode_on_error) |
| exception text and at src/... lines inside a failed method | [`jvm_dump_exception_on_error`](#jvm_dump_exception_on_error) |
| need to know why a method failed (option is off) | [`jvm_dump_exception_on_error`](#jvm_dump_exception_on_error) |
| /* $VF: ... */ and // $VF: comments clutter the output | [`jvm_decompiler_comments`](#jvm_decompiler_comments) |
| need to know where the decompiler approximated (option is off) | [`jvm_decompiler_comments`](#jvm_decompiler_comments) |
| need the original .java file name of a class | [`jvm_sourcefile_comments`](#jvm_sourcefile_comments) |
| repackaged or shaded classes of unknown origin | [`jvm_sourcefile_comments`](#jvm_sourcefile_comments) |
| need the source positions of identifiers for navigation | [`jvm_dump_text_tokens`](#jvm_dump_text_tokens) |
| debugging which identifier a name in the output refers to | [`jvm_dump_text_tokens`](#jvm_dump_text_tokens) |
| long import blocks at the top of every file | [`jvm_remove_imports`](#jvm_remove_imports) |
| imports are noise for the task at hand | [`jvm_remove_imports`](#jvm_remove_imports) |
| stack trace names lambda$main$0 or Main$1 and the matching code is hard to find | [`jvm_mark_corresponding_synthetics`](#jvm_mark_corresponding_synthetics) |
| need the class file of an anonymous class | [`jvm_mark_corresponding_synthetics`](#jvm_mark_corresponding_synthetics) |
| output differs from Vineflower's test fixture but matches java -jar | [`jvm_plugins`](#jvm_plugins) |
| a class is written in a plugin's language instead of Java | [`jvm_plugins`](#jvm_plugins) |
| a bundled plugin's pass changes the Java output | [`jvm_plugins`](#jvm_plugins) |
| output differs from Vineflower 1.12.0 | [`jvm_vineflower_compat`](#jvm_vineflower_compat) |
| need Vineflower's exact text, failure comments included | [`jvm_vineflower_compat`](#jvm_vineflower_compat) |
| a method decompiles in kuna but fails in Vineflower | [`jvm_vineflower_compat`](#jvm_vineflower_compat) |
| failure comment names the kuna issue tracker instead of Vineflower's | [`jvm_vineflower_compat`](#jvm_vineflower_compat) |
| a kuna fix is suspected of changing a method | [`jvm_vineflower_compat`](#jvm_vineflower_compat) |
| Kotlin classes come out as Java with @Metadata(mv = ..., d1 = ...) | [`jvm_kt_enable`](#jvm_kt_enable) |
| a Kotlin class is written as .kt but Java output is wanted | [`jvm_kt_enable`](#jvm_kt_enable) |
| Kotlin output of a class is wrong or failed | [`jvm_kt_enable`](#jvm_kt_enable) |

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

### `jvm_decompile_java4` -- on | off, default `on`

- **Symptoms:** class$("java.lang.String") calls instead of String.class; static Class class$java$lang$String fields in old class files; need the raw Class.forName cache logic.
- **What it does:** Turn the pre-Java-5 class-literal idiom (a synthetic static class$(String) helper, a cached Class field and Class.forName) back into Foo.class.
- **When to flip:** Turn off to see the literal class$ helper calls and cache fields in Java 1.4-and-older class files, e.g. when the helper was tampered with.
- **Where / provenance:** p3_dataflow/simplify_exprents_helper, class_writer · vineflower · vineflower-option
- **Also spelled:** `decompile-java4`, `dc4`
- **Example:** `kuna decompile legacy.jar com.ex.Old --option jvm_decompile_java4 off`

### `jvm_decompile_assert` -- on | off, default `on`

- **Symptoms:** if (!$assertionsDisabled && !cond) throw new AssertionError() instead of assert cond; $assertionsDisabled field visible; assert hides a check that must be read literally.
- **What it does:** Rebuild assert statements from the $assertionsDisabled check and the throw new AssertionError(..) javac compiles them to.
- **When to flip:** Turn off to see the explicit $assertionsDisabled test and throw, e.g. when the static initializer computing the flag was changed.
- **Where / provenance:** p8_structure/assert_processor · vineflower · vineflower-option
- **Also spelled:** `decompile-assert`, `das`
- **Example:** `kuna decompile app.jar com.ex.Checks --option jvm_decompile_assert off`

### `jvm_hide_empty_super` -- on | off, default `on`

- **Symptoms:** constructor shows no super() call; need to see the implicit superclass constructor invocation.
- **What it does:** Drop a constructor's leading super() call when it passes no arguments, as source code leaves it implicit.
- **When to flip:** Turn off to show every super() call, e.g. to check which superclass constructor runs first.
- **Where / provenance:** p8_structure/initializer_processor · vineflower · vineflower-option
- **Also spelled:** `hide-empty-super`, `hes`
- **Example:** `kuna decompile app.jar com.ex.Child --option jvm_hide_empty_super off`

### `jvm_hide_default_constructor` -- on | off, default `on`

- **Symptoms:** public Foo() {} constructor missing from the output; class seems to have no constructor.
- **What it does:** Leave out a class's only constructor when it takes no parameters, throws nothing, has an empty body and the class's own access (the one javac adds).
- **When to flip:** Turn off to list every constructor the class file declares, e.g. when counting entry points.
- **Where / provenance:** p9_emit/class_writer · vineflower · vineflower-option
- **Also spelled:** `hide-default-constructor`, `hdc`
- **Example:** `kuna decompile app.jar com.ex.Util --option jvm_hide_default_constructor off`

### `jvm_incorporate_returns` -- on | off, default `on`

- **Symptoms:** var = x; inside try followed by return var after the catch; return inside try reads differently from the bytecode.
- **What it does:** Move value returns into the try and catch blocks they follow, instead of storing the value in a temporary and returning after the try.
- **When to flip:** Turn off to see the return placement the bytecode literally has, e.g. when a return-inside-try reading looks wrong.
- **Where / provenance:** infra/method_processor · vineflower · vineflower-option
- **Also spelled:** `incorporate-returns`, `ner`
- **Example:** `kuna decompile app.jar com.ex.Io --option jvm_incorporate_returns off`

### `jvm_ensure_synchronized_monitors` -- on | off, default `on`

- **Symptoms:** synchronized block covers more code than expected; MonitorEnter/MonitorExit pseudo-calls instead of synchronized; unbalanced monitor in handwritten bytecode.
- **What it does:** When a monitorenter has no matching monitorexit on some path, extend the synchronized range to where the exit should be so the block still becomes synchronized (..) { }.
- **When to flip:** Turn off when the inferred extent of a synchronized block looks wrong and the raw monitor operations are wanted.
- **Where / provenance:** infra/method_processor · vineflower · vineflower-option
- **Also spelled:** `ensure-synchronized-monitors`, `esm`
- **Example:** `kuna decompile app.jar com.ex.Lock --option jvm_ensure_synchronized_monitors off`

### `jvm_decompile_enums` -- on | off, default `on`

- **Symptoms:** enum written as class extends Enum with $VALUES; need the generated values()/valueOf() bodies; enum constant initializers hide tampered code.
- **What it does:** Write enum classes as enum declarations: constants with their arguments and bodies, no $VALUES, values() or valueOf(), and no hidden name/ordinal constructor parameters.
- **When to flip:** Turn off to see an enum as the plain class javac generates, e.g. when the generated values()/valueOf() or the constant initializers were modified.
- **Where / provenance:** p9_emit/class_writer, invocation_exprent, method_descriptor · vineflower · vineflower-option
- **Also spelled:** `decompile-enums`, `den`
- **Example:** `kuna decompile app.jar com.ex.Color --option jvm_decompile_enums off`

### `jvm_remove_getclass` -- on | off, default `on`

- **Symptoms:** stray outer.getClass(); before outer.new Inner(); Objects.requireNonNull-like check before an inner class instantiation.
- **What it does:** Drop the outer.getClass() null check javac inserts before a qualified inner-class creation (outer.new Inner()).
- **When to flip:** Turn off to keep the synthetic getClass() call, e.g. when the null check itself matters to the analysis.
- **Where / provenance:** p3_dataflow/simplify_exprents_helper · vineflower · vineflower-option
- **Also spelled:** `remove-getclass`, `rgn`
- **Example:** `kuna decompile app.jar com.ex.Outer --option jvm_remove_getclass off`

### `jvm_synthetic_not_set` -- on | off, default `off`

- **Symptoms:** access$000(this) calls left in the output; this$0 or val$name fields printed in inner classes; class$ helper calls remain in old class files.
- **What it does:** Recognize compiler-generated members by their shape even without the ACC_SYNTHETIC flag: static accessor methods, outer-instance and captured-variable fields, and Java 1.4 class$ caches.
- **When to flip:** Turn on for class files from compilers or obfuscators that drop the synthetic flag, so access$NNN calls and this$0/val$x fields still fold back into source form.
- **Where / provenance:** p8_structure/nested_class_processor, nested_member_access, class_reference14_processor · vineflower · vineflower-option
- **Also spelled:** `synthetic-not-set`, `nns`
- **Example:** `kuna decompile obf.jar a.b --option jvm_synthetic_not_set on`

### `jvm_remove_empty_try_catch` -- on | off, default `on`

- **Symptoms:** exception handler code missing from the output; empty try { } catch blocks wanted for inspection.
- **What it does:** Drop exception-table ranges that protect no instructions (and the handlers they alone reach) before the control flow is structured.
- **When to flip:** Turn off to keep every exception range of the class file, e.g. when an obfuscator uses empty ranges and their handlers to hide code.
- **Where / provenance:** infra/method_processor, dead_code_helper · vineflower · vineflower-option
- **Also spelled:** `remove-empty-try-catch`, `rer`
- **Example:** `kuna decompile obf.jar a.b --option jvm_remove_empty_try_catch off`

### `jvm_decompile_finally` -- on | off, default `on`

- **Symptoms:** "Could not verify finally blocks" comment with a semaphore variable; cleanup code duplicated after every return; catch (Throwable var) { ...; throw var; } instead of finally (option is off).
- **What it does:** Rebuild try/finally: find the finally code javac copies onto every exit of the try, keep one copy in a finally block and drop the catch-all handler that rethrows.
- **When to flip:** Turn off when the reconstructed finally is wrong or adds a semaphore variable, to see the duplicated copies and the catch (Throwable) handler as compiled.
- **Where / provenance:** p7_regions/finally_processor · vineflower · vineflower-option
- **Also spelled:** `decompile-finally`, `fdi`
- **Example:** `kuna decompile app.jar com.ex.Io --option jvm_decompile_finally off`

### `jvm_lambda_to_anonymous_class` -- on | off, default `off`

- **Symptoms:** need the interface method a lambda implements; output must compile with Java 7; x -> ... where an anonymous class is wanted.
- **What it does:** Write each lambda and method reference as an anonymous class implementing its functional interface (new Runnable() { public void run() { .. } }) instead of -> or :: syntax.
- **When to flip:** Turn on when the functional interface and method a lambda implements matter, or for Java 7-style output.
- **Where / provenance:** p8_structure/nested_class_processor, new_exprent, class_writer · vineflower · vineflower-option
- **Also spelled:** `lambda-to-anonymous-class`, `lac`
- **Example:** `kuna decompile app.jar com.ex.Handlers --option jvm_lambda_to_anonymous_class on`

### `jvm_ignore_invalid_bytecode` -- on | off, default `off`

- **Symptoms:** $VF: Couldn't be decompiled with RuntimeException: Constructor ... not found; method fails on a new expression in a patched class.
- **What it does:** When a constructor call names a constructor its class does not declare (invalid or tampered bytecode), print the call without hiding synthetic arguments instead of failing the method with "Constructor .. not found".
- **When to flip:** Turn on for obfuscated or patched jars whose methods fail with a Constructor not found error.
- **Where / provenance:** substrate/expr_util · vineflower · vineflower-option
- **Also spelled:** `ignore-invalid-bytecode`, `iib`
- **Example:** `kuna decompile obf.jar a.b --option jvm_ignore_invalid_bytecode on`

### `jvm_ternary_constant_simplification` -- on | off, default `off`

- **Symptoms:** a ? true : b where a logical or was written; nested ternaries with true and false branches; x ? false : true instead of !x.
- **What it does:** Fold ternaries with a boolean constant branch into logic: c ? true : x to c || x, c ? x : false to c && x, c ? false : true to !c.
- **When to flip:** Turn on to read compiled boolean logic as && and || instead of nested ternaries.
- **Where / provenance:** p3_dataflow/secondary_functions_helper · vineflower · vineflower-option
- **Also spelled:** `ternary-constant-simplification`, `tcs`
- **Example:** `kuna decompile app.jar com.ex.Rules --option jvm_ternary_constant_simplification on`

### `jvm_pattern_matching` -- on | off, default `on`

- **Symptoms:** String s = (String)o; after an instanceof test (option is off); typeSwitch bootstrap call instead of a pattern switch; pattern variable reconstructed wrongly.
- **What it does:** Rebuild instanceof patterns (if (o instanceof String s)) and pattern switches (case Point(int x, int y) ->) from the checkcast and SwitchBootstraps.typeSwitch code of Java 16+ class files.
- **When to flip:** Turn off to see the explicit casts and typeSwitch bootstrap calls, or for output that older Java versions can compile.
- **Where / provenance:** p8_structure/if_pattern_match_processor, switch_pattern_match_processor, method_processor · vineflower · vineflower-option
- **Also spelled:** `pattern-matching`, `pam`
- **Example:** `kuna decompile app.jar com.ex.Shapes --option jvm_pattern_matching off`

### `jvm_try_loop_fix` -- on | off, default `on`

- **Symptoms:** while (true) with a try inside that does not match the bytecode; loop condition moved into a try block.
- **What it does:** Refuse a loop-head merge that would pull an if statement into a try block nested in a while loop, which otherwise produces malformed loops around try/catch.
- **When to flip:** Turn off only to reproduce older Fernflower structuring when comparing outputs.
- **Where / provenance:** p7_regions/dec_helper · vineflower · vineflower-option
- **Also spelled:** `try-loop-fix`, `tlf`
- **Example:** `kuna decompile app.jar com.ex.Retry --option jvm_try_loop_fix off`

### `jvm_ternary_in_if` -- on | off, default `on`

- **Symptoms:** if (a ? b : c) condition hard to read; duplicated if bodies where a ternary condition was compiled (option is off).
- **What it does:** Collapse nested if statements that a ternary inside a condition compiles to (if (a ? b : c)) back into one if with the ternary condition.
- **When to flip:** Turn off when a collapsed condition reads wrong; the nested ifs are what the branches really are.
- **Where / provenance:** p8_structure/if_helper · vineflower · vineflower-option
- **Also spelled:** `ternary-in-if`, `tco`
- **Example:** `kuna decompile app.jar com.ex.Rules --option jvm_ternary_in_if off`

### `jvm_decompile_switch_expressions` -- on | off, default `on`

- **Symptoms:** switch statement assigning a temporary in every case where a switch expression was written; yield statements misplaced.
- **What it does:** Rebuild Java 14+ switch expressions (var x = switch (k) { case A -> 1; default -> { yield 2; } }) from the statement switch and temporary javac emits.
- **When to flip:** Turn off to see the statement switch and assignments, or for output that pre-14 Java can compile.
- **Where / provenance:** p8_structure/switch_expression_helper · vineflower · vineflower-option
- **Also spelled:** `decompile-switch-expressions`, `swe`
- **Example:** `kuna decompile app.jar com.ex.Codes --option jvm_decompile_switch_expressions off`

### `jvm_simplify_stack` -- on | off, default `on`

- **Symptoms:** var10000 temporaries everywhere (option is off); compound assignment merges side effects in the wrong order; arr[i++] = v reads differently from the bytecode.
- **What it does:** Run the second stack-variable stage, which inlines stack temporaries across statement boundaries and resugars compound expressions (a = b = c, arr[i++] = v, chained calls).
- **When to flip:** Turn off when a merged expression is wrong or reorders side effects, to see each stack temporary as a var assignment.
- **Where / provenance:** p3_dataflow/stack_vars_processor · vineflower · vineflower-option
- **Also spelled:** `simplify-stack`, `ssp`
- **Example:** `kuna decompile app.jar com.ex.Parser --option jvm_simplify_stack off`

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

### `jvm_decompile_complex_constant_dynamic` -- on | off, default `off`

- **Symptoms:** condy bootstrap call with ConstantBootstraps.fieldVarHandle in the output; VarHandle constant shown as an unresolved dynamic constant.
- **What it does:** Write constant-dynamic VarHandle constants (fieldVarHandle, staticFieldVarHandle, arrayVarHandle) as the equivalent MethodHandles lookup expression, marked with a comment because it runs eagerly; off keeps the raw condy form.
- **When to flip:** Turn on to read code using condy VarHandles in plain Java terms, accepting that the expression is not lazily evaluated.
- **Where / provenance:** p4_calls/condy_helper · vineflower · vineflower-option
- **Also spelled:** `decompile-complex-constant-dynamic`, `dcc`
- **Example:** `kuna decompile app.jar com.ex.Atomic --option jvm_decompile_complex_constant_dynamic on`

### `jvm_force_jsr_inline` -- on | off, default `off`

- **Symptoms:** method with jsr/ret fails to decompile in a modern class file; finally logic of a modern class built from subroutines.
- **What it does:** Inline jsr/ret subroutines in every class file; normally only class files of Java 6 or older, where they are legal, get this.
- **When to flip:** Turn on for hand-assembled or obfuscated Java 7+ class files that use jsr/ret anyway and fail to decompile.
- **Where / provenance:** infra/method_processor · vineflower · vineflower-option
- **Also spelled:** `force-jsr-inline`, `fji`
- **Example:** `kuna decompile obf.jar a.b --option jvm_force_jsr_inline on`

### `jvm_prettify_ifs` -- on | off, default `on`

- **Symptoms:** else branch missing after an if that returns; deep else chains after returns (option is off).
- **What it does:** Before writing, turn if (c) { ..; return; } else { rest } into the if followed by rest: the else after a branch that always returns or throws is dropped (unless the else exits too).
- **When to flip:** Turn off to keep the if/else pairs as structured from the control flow, e.g. when comparing with older Fernflower output.
- **Where / provenance:** p9_emit/class_writer, if_helper · vineflower · vineflower-option
- **Also spelled:** `prettify-ifs`
- **Example:** `kuna decompile app.jar com.ex.Main --option jvm_prettify_ifs off`

### `jvm_source_locals` -- on | off, default `on`

- **Symptoms:** a local variable of the source is missing; x = e; return x; printed as return e;; declarations reordered against the source; recompiled bytecode uses other local slots; want Vineflower's folded form of locals.
- **What it does:** Keep the local variables the class file records for the source. A local the LocalVariableTable names is not folded into its one use: x = e; return x; stays two statements, a named array keeps its element stores instead of becoming new T[]{..}, and a named iterable, iterator or index is not folded into a for-each. In Java output, a declaration without an initializer moves up, or a declaration is split into T v; and v = e;, so that javac numbers the locals' slots as the class file does. Vineflower folds such locals and declares each variable at its first use; jvm_vineflower_compat on turns this off.
- **When to flip:** Leave on to read a method with the locals its author wrote and to recompile it to the same bytecode; turn off for Vineflower's shorter form (return e; for x = e; return x;) without the compat switch's other effects.
- **Where / provenance:** p6_variables/kuna_source_locals, simplify_exprents_helper, merge_helper, var_definition_helper · kuna · kuna-form
- **Example:** `kuna decompile app.jar com.ex.Main --option jvm_source_locals off`

### `jvm_javac_layout` -- on | off, default `on`

- **Symptoms:** if condition negated against the source with the branches swapped; early return or continue missing and the rest of the method nested in an if; return c ? a : b where the source has two returns; return c; where the source has if (c) return true; return false;; else branch after an if that returns.
- **What it does:** Structure ifs and exits of classes javac compiled (no kotlin.Metadata), written as Java, the way javac laid out the bytecode: an if's then-branch is the code javac placed right after the condition, so its condition keeps the source's polarity; an early return, throw, break or continue that the bytecode has stays where it is instead of the rest of the code being nested in a negated if; and two return instructions stay two returns instead of becoming one return of a ternary (if (c) return true; return false; is not folded to return c;). Off gives Vineflower's structuring; jvm_vineflower_compat on turns it off.
- **When to flip:** Turn off to get Vineflower's if/else orientation, nested ifs instead of early exits and ternary returns, e.g. to compare with Vineflower's output on Java classes, or where the condensed form reads better.
- **Where / provenance:** p8_structure/kuna_javac_layout, if_helper, simplify_exprents_helper · kuna · kuna-form
- **Example:** `kuna decompile app.jar com.ex.Main --option jvm_javac_layout off`

### `jvm_variable_renaming` -- jad | tiny | <name>, default ``

- **Symptoms:** locals named var1 var2 var3 with no hint of their type; obfuscated or meaningless local names from a forged LocalVariableTable; names differ from a Vineflower run with --variable-renaming=jad or tiny.
- **What it does:** Rename locals with a renamer of the VariableRenaming plugin (needs jvm_plugins on, the default): jad names them after their type JAD-style (i, j, k, l, s, flag, file1, aobject), tiny the way tiny-remapper does (i, l, bl, the class name lowercased, plural for arrays); unset or an unknown name keeps the LocalVariableTable or var names.
- **When to flip:** Set jad or tiny on obfuscated or debug-info-free code whose var1, var2 or forged names say nothing about the variables, or to match a Vineflower run that used the same renamer.
- **Where / provenance:** plugins/variable_renaming_plugin, renamers, jad_name_provider, tiny_name_provider · vineflower · vineflower-option
- **Also spelled:** `variable-renaming`
- **Example:** `kuna decompile obf.jar a.b --option jvm_variable_renaming jad`

### `jvm_rename_parameters` -- on | off, default `off`

- **Symptoms:** locals renamed but parameters still named var1 or paramN; parameter names do not follow the renamer's scheme.
- **What it does:** With a jvm_variable_renaming renamer, rename method and lambda parameters too, not only locals.
- **When to flip:** Turn on with jvm_variable_renaming when the parameter names are as meaningless as the locals (obfuscated code, no MethodParameters or LocalVariableTable).
- **Where / provenance:** plugins/jad_name_provider, tiny_name_provider · vineflower · vineflower-option
- **Also spelled:** `rename-parameters`
- **Example:** `kuna decompile obf.jar a.b --option jvm_variable_renaming tiny --option jvm_rename_parameters on`

### `jvm_jad_style_variable_naming` -- on | off, default `off`

- **Symptoms:** an old Vineflower command line with --jad-style-variable-naming=1 gives JAD-style names; locals named var1 var2 where JAD-style names were expected.
- **What it does:** Deprecated in Vineflower: when jvm_variable_renaming is unset, use the jad renamer.
- **When to flip:** Only to replay Vineflower command lines that still pass the deprecated flag; prefer jvm_variable_renaming jad.
- **Where / provenance:** plugins/variable_renaming_plugin · vineflower · vineflower-option
- **Also spelled:** `jad-style-variable-naming`
- **Example:** `kuna decompile obf.jar a.b --option jvm_jad_style_variable_naming on`

### `jvm_jad_style_parameter_naming` -- on | off, default `off`

- **Symptoms:** an old Vineflower command line with --jad-style-parameter-naming=1 renames parameters; parameters keep their names under the jad renamer.
- **What it does:** Deprecated in Vineflower: like jvm_rename_parameters, but for the jad renamer only.
- **When to flip:** Only to replay Vineflower command lines that still pass the deprecated flag; prefer jvm_rename_parameters.
- **Where / provenance:** plugins/jad_name_provider · vineflower · vineflower-option
- **Also spelled:** `jad-style-parameter-naming`
- **Example:** `kuna decompile obf.jar a.b --option jvm_variable_renaming jad --option jvm_jad_style_parameter_naming on`

### `jvm_resugar_idea_notnull` -- on | off, default `on`

- **Symptoms:** if (x == null) { $$$reportNull$$$0(0); } at the top of methods; explicit null checks on @NotNull parameters or before returns (option is off).
- **What it does:** Remove the null checks IntelliJ IDEA's compiler inserts for @NotNull parameters and return values (if (x == null) { $$$reportNull$$$0(n); }), leaving the @NotNull annotations (IdeaNotNull plugin; needs jvm_plugins on, the default).
- **When to flip:** Turn off to see the checks IDEA instrumented into a method, e.g. to confirm which parameters are checked at run time.
- **Where / provenance:** plugins/idea_not_null_pass, idea_not_null_plugin · vineflower · vineflower-option
- **Also spelled:** `resugar-idea-notnull`, `inn`
- **Example:** `kuna decompile app.jar com.ex.Service --option jvm_resugar_idea_notnull off`

### `jvm_kt_show_public` -- on | off, default `on`

- **Symptoms:** every Kotlin declaration starts with public; public modifiers that Kotlin source would omit.
- **What it does:** Kotlin plugin: write the public modifier on public classes, functions, constructors and properties of Kotlin classes, which Kotlin source normally leaves implicit (needs jvm_plugins and jvm_kt_enable on, the defaults).
- **When to flip:** Turn off for idiomatic Kotlin with public implied, so that only internal, protected and private members carry a visibility keyword.
- **Where / provenance:** kotlin/kotlin_writer, k_function, k_constructor, k_utils · vineflower · vineflower-option
- **Also spelled:** `kt-show-public`
- **Example:** `kuna decompile app.jar com.ex.Service --option jvm_kt_show_public off`

### `jvm_kt_unknown_defaults` -- <text>, default `...`

- **Symptoms:** parameters written as x: Int = ...; default argument values missing from a Kotlin function signature.
- **What it does:** Kotlin plugin: the text written after = for a parameter that has a default value the plugin could not recover from the $default bridge (fun f(x: Int = ...)); empty writes no default at all.
- **When to flip:** Set a marker of your own (for example /* default */) to make unrecovered defaults easy to find, or set it empty to drop them.
- **Where / provenance:** kotlin/default_args_map · vineflower · vineflower-option
- **Also spelled:** `kt-unknown-defaults`
- **Example:** `kuna decompile app.jar com.ex.ApiKt --option jvm_kt_unknown_defaults "/* default */"`

### `jvm_kt_collapse_string_concat` -- on | off, default `on`

- **Symptoms:** "a" + b + "c" in Kotlin output where a template was written; string templates hide the concatenation order.
- **What it does:** Kotlin plugin: turn string concatenations (StringBuilder chains, + chains, invokedynamic makeConcatWithConstants) in Kotlin methods into string templates "a $b ${c.d}".
- **When to flip:** Turn off to see the concatenation exactly as compiled, for example to check which values are converted with toString or how a template was built.
- **Where / provenance:** kotlin/collapse_string_concat_pass · vineflower · vineflower-option
- **Also spelled:** `kt-collapse-string-concat`
- **Example:** `kuna decompile app.jar com.ex.FormatKt --option jvm_kt_collapse_string_concat off`

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

### `jvm_warn_inconsistent_inner_attributes` -- on | off, default `on`

- **Symptoms:** "Inconsistent inner class entries" warnings flood stderr; nested class appears under an unexpected outer class.
- **What it does:** Log a warning when classes' InnerClasses entries disagree about a nested class, with both entries and the one chosen; output text is unchanged.
- **When to flip:** Turn off to quiet the log on jars with many conflicting entries; keep on to learn why a class was nested where it was.
- **Where / provenance:** p1_classfile/classes_processor · vineflower · vineflower-option
- **Also spelled:** `warn-inconsistent-inner-attributes`, `win`
- **Example:** `kuna decompile-all obf.jar --option jvm_warn_inconsistent_inner_attributes off`

### `jvm_excluded_classes` -- <regex>, default ``

- **Symptoms:** decompiling a fat jar spends time on bundled libraries; output includes generated or third-party classes.
- **What it does:** Do not decompile classes whose internal name (a/b/C$D) fully matches this Java regular expression; they still give context to the others.
- **When to flip:** Set to skip bundled libraries or generated code in a fat jar, e.g. ^com/google/.* or .*\$\$Lambda.*.
- **Where / provenance:** p1_classfile/classes_processor · vineflower · vineflower-option
- **Also spelled:** `excluded-classes`
- **Example:** `kuna decompile-project app.jar -o out --option jvm_excluded_classes "^org/slf4j/.*"`

### `jvm_included_classes` -- <regex>, default ``

- **Symptoms:** only one package of a large jar is of interest; decompile-project takes too long on the whole jar.
- **What it does:** Decompile only classes whose internal name (a/b/C$D) fully matches this Java regular expression; the rest still give context.
- **When to flip:** Set to focus on one package of a large jar, e.g. ^com/ex/core/.*.
- **Where / provenance:** p1_classfile/classes_processor · vineflower · vineflower-option
- **Also spelled:** `included-classes`
- **Example:** `kuna decompile-project app.jar -o out --option jvm_included_classes "^com/ex/core/.*"`

### `jvm_validate_inner_classes_names` -- on | off, default `on`

- **Symptoms:** member classes of an obfuscated jar written as separate top-level classes; renamed inner classes not nested.
- **What it does:** Nest a member class only when its name is its outer class's name, $, and its simple name (Outer$Inner); a member entry with any other name is ignored and the class stays top-level.
- **When to flip:** Turn off for renamed or obfuscated jars whose inner class names do not follow the Outer$Inner scheme, so they still nest.
- **Where / provenance:** p1_classfile/classes_processor · vineflower · vineflower-option
- **Also spelled:** `validate-inner-classes-names`
- **Example:** `kuna decompile obf.jar a --option jvm_validate_inner_classes_names off`

### `jvm_siblings` -- on | off, default `on`

- **Symptoms:** inner and anonymous classes missing when decompiling one .class file; unrelated Name$x.class files nested into the class.
- **What it does:** For a single .class input, also load the Name$*.class files next to it, so nested, local and anonymous classes are decompiled inside it.
- **When to flip:** Turn off to decompile exactly the given class file, e.g. when a directory holds unrelated classes with the same prefix.
- **Where / provenance:** p0_knowledge/kuna_input · kuna · kuna-input
- **Example:** `kuna decompile Main.class --option jvm_siblings off`

### `jvm_kt_export_metadata` -- on | off, default `off`

- **Symptoms:** a Vineflower command line to replay sets --kt-export-metadata=1 (the output is the same either way).
- **What it does:** Kotlin plugin: parse the @kotlin.Metadata of Kotlin classes even when jvm_kt_enable is off, so that the parsed metadata is available to other plugins; it does not change what is written (needs jvm_plugins on, the default).
- **When to flip:** Only together with jvm_kt_enable off, to replay a Vineflower command line that passes it; kuna has no other consumer of the parsed metadata, so the Java output is the same either way.
- **Where / provenance:** kotlin/kotlin_chooser · vineflower · vineflower-option
- **Also spelled:** `kt-export-metadata`
- **Example:** `kuna decompile app.jar com.ex.UserKt --option jvm_kt_enable off --option jvm_kt_export_metadata on`

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

### `jvm_boolean_as_int` -- on | off, default `on`

- **Symptoms:** int variable printed as boolean true/false; 1 and 0 where true and false were expected (option is off).
- **What it does:** Let the int constant 1, not only 0, take boolean type during type inference, so flags stored with iconst_1/iconst_0 print as true and false.
- **When to flip:** Turn off when booleans are being inferred where the code really works with the ints 0 and 1 (handwritten or non-Java bytecode).
- **Where / provenance:** substrate/const_exprent · vineflower · vineflower-option
- **Also spelled:** `boolean-as-int`, `bto`
- **Example:** `kuna decompile app.jar com.ex.Flags --option jvm_boolean_as_int off`

### `jvm_ascii_strings` -- on | off, default `off`

- **Symptoms:** string literal shows odd or invisible characters; homoglyphs in a compared string; terminal mangles non-ASCII text in decompiled strings.
- **What it does:** Escape every non-ASCII character of string and char literals as \uXXXX.
- **When to flip:** Turn on for strings with invisible, right-to-left or homoglyph characters (obfuscated keys, spoofed names) or when the output must be pure ASCII.
- **Where / provenance:** substrate/const_exprent · vineflower · vineflower-option
- **Also spelled:** `ascii-strings`, `asc`
- **Example:** `kuna decompile app.jar com.ex.Keys --option jvm_ascii_strings on`

### `jvm_undefined_as_object` -- on | off, default `on`

- **Symptoms:** <undefinedtype> in a parameter list (option is off); lambda parameter typed Object where a type is expected.
- **What it does:** Print a lambda or method parameter whose type could not be inferred as Object instead of <undefinedtype>.
- **When to flip:** Turn off to see exactly where type inference gave up (the <undefinedtype> marker).
- **Where / provenance:** p9_emit/class_writer · vineflower · vineflower-option
- **Also spelled:** `undefined-as-object`, `uto`
- **Example:** `kuna decompile app.jar com.ex.Streams --option jvm_undefined_as_object off`

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

### `jvm_show_hidden_statements` -- on | off, default `off` (destructive opt-in)

- **Symptoms:** string switch reconstructed wrongly and the hidden hashCode switch is needed; debugging which switch the decompiler discarded.
- **What it does:** Debug aid: also print the switch statements the decompiler marked phantom (scaffolding left over after rebuilding string, enum and pattern switches).
- **When to flip:** Turn on only to debug the decompiler's switch reconstruction; the output no longer reads as source.
- **Where / provenance:** substrate/switch_statement · vineflower · vineflower-option
- **Also spelled:** `show-hidden-statements`, `shs`
- **Example:** `kuna decompile app.jar com.ex.Codes --option jvm_show_hidden_statements on`

### `jvm_override_annotation` -- on | off, default `on`

- **Symptoms:** @Override missing on toString or interface methods (supertype not loaded); @Override that javac did not write is unwanted.
- **What it does:** Add @Override to instance methods (Java 5+ class files) that override or implement a method found in a superclass or interface the decompiler can see.
- **When to flip:** Turn off for output without inferred annotations; leave on and give a runtime or libraries so supertypes are known.
- **Where / provenance:** p9_emit/class_writer · vineflower · vineflower-option
- **Also spelled:** `override-annotation`, `ovr`
- **Example:** `kuna decompile app.jar com.ex.Impl --option jvm_override_annotation off`

### `jvm_explicit_generics` -- on | off, default `off`

- **Symptoms:** decompiled code fails to recompile with an inference error; need the type argument of a generic method call.
- **What it does:** Write the type arguments of generic method calls explicitly (Collections.<String>emptyList(), this.<T>cast(x)) even where javac would infer them.
- **When to flip:** Turn on when the decompiled code fails to compile on type inference, or to see which type a generic call was instantiated with.
- **Where / provenance:** substrate/invocation_exprent · vineflower · vineflower-option
- **Also spelled:** `explicit-generics`, `ega`
- **Example:** `kuna decompile app.jar com.ex.Repo --option jvm_explicit_generics on`

### `jvm_inline_simple_lambdas` -- on | off, default `on`

- **Symptoms:** x -> { return x + 1; } where x -> x + 1 is wanted (option is off); one-line lambdas hard to diff.
- **What it does:** Write a lambda whose body is one expression without braces and return (x -> x + 1).
- **When to flip:** Turn off for block-bodied lambdas everywhere, e.g. to diff lambda bodies line by line.
- **Where / provenance:** p9_emit/class_writer · vineflower · vineflower-option
- **Also spelled:** `inline-simple-lambdas`, `isl`
- **Example:** `kuna decompile app.jar com.ex.Streams --option jvm_inline_simple_lambdas off`

### `jvm_indent_string` -- <text>, default `   `

- **Symptoms:** three-space indentation does not match the project style; output too wide when nested deeply.
- **What it does:** The text written once per indentation level (three spaces by default).
- **When to flip:** Set to four spaces or a tab to match a code base's style, or two spaces for compact output.
- **Where / provenance:** p9_emit/text_buffer, text_util · vineflower · vineflower-option
- **Also spelled:** `indent-string`, `ind`
- **Example:** `kuna decompile app.jar com.ex.Main --option jvm_indent_string "    "`

### `jvm_preferred_line_length` -- <n>, default `160`

- **Symptoms:** very long lines in builder chains; expressions wrapped across lines unexpectedly.
- **What it does:** Break argument lists, chained calls and long operator chains at their possible break points when a line would run past this many characters.
- **When to flip:** Lower it for narrow terminals or diffs, raise it to keep each expression on one line.
- **Where / provenance:** p9_emit/text_buffer · vineflower · vineflower-option
- **Also spelled:** `preferred-line-length`, `pll`
- **Example:** `kuna decompile app.jar com.ex.Main --option jvm_preferred_line_length 100`

### `jvm_banner` -- <text>, default ``

- **Symptoms:** decompiled files need a provenance or license header.
- **What it does:** Text written verbatim at the top of every decompiled file (include your own trailing newline).
- **When to flip:** Set to mark generated sources, e.g. a // Decompiled by kuna from app.jar header.
- **Where / provenance:** p0_knowledge/fernflower · vineflower · vineflower-option
- **Also spelled:** `banner`, `ban`
- **Example:** `kuna decompile-project app.jar -o out --option jvm_banner "// decompiled\n"`

### `jvm_error_message` -- <text>, default `Please report this to the Vineflower issue tracker, at https://github.com/Vineflower/vineflower/issues with a copy of the class file (if you have the rights to distribute it!)`

- **Symptoms:** failure comment asks to report to an issue tracker; failure comment should carry a project-specific note.
- **What it does:** The note (one comment line per text line) written after $VF: Couldn't be decompiled when exception dumps are on; unset, it points to kuna's issue tracker, or with jvm_vineflower_compat on to Vineflower's (the default shown, Vineflower's own).
- **When to flip:** Replace it with your own text (it then wins over both trackers), or set it empty to drop the note.
- **Where / provenance:** p9_emit/class_writer · vineflower · vineflower-option
- **Also spelled:** `error-message`, `erm`
- **Example:** `kuna decompile app.jar com.ex.Main --option jvm_error_message "see kuna docs jvm"`

### `jvm_dump_bytecode_on_error` -- on | off, default `on`

- **Symptoms:** failed method shows a long // Bytecode: listing; need the instructions of a method that failed to decompile (option is off).
- **What it does:** Inside a method that could not be decompiled, list its bytecode as comments (offset, opcode, operands, constants).
- **When to flip:** Turn off for shorter output when failures are expected and the bytecode is not needed.
- **Where / provenance:** p9_emit/class_writer · vineflower · vineflower-option
- **Also spelled:** `dump-bytecode-on-error`, `dbe`
- **Example:** `kuna decompile obf.jar a.b --option jvm_dump_bytecode_on_error off`

### `jvm_dump_exception_on_error` -- on | off, default `on`

- **Symptoms:** exception text and at src/... lines inside a failed method; need to know why a method failed (option is off).
- **What it does:** Inside a method or class that could not be decompiled, write the error-message note and the exception with its throw location; off leaves just $VF: Couldn't be decompiled (a class that fails entirely then has no output).
- **When to flip:** Turn off for clean output when failures are expected; keep on to learn why a method failed.
- **Where / provenance:** p9_emit/class_writer, fernflower · vineflower · vineflower-option
- **Also spelled:** `dump-exception-on-error`, `dee`
- **Example:** `kuna decompile obf.jar a.b --option jvm_dump_exception_on_error off`

### `jvm_decompiler_comments` -- on | off, default `on`

- **Symptoms:** /* $VF: ... */ and // $VF: comments clutter the output; need to know where the decompiler approximated (option is off).
- **What it does:** Write $VF: comments where the decompiler had to bend the code: irreducible flow, unverifiable finally blocks, MethodType constants, constructors it could not resugar, ambiguous variable types.
- **When to flip:** Turn off for output free of decompiler remarks; keep on to know which parts of the code to distrust.
- **Where / provenance:** p9_emit/class_writer, finally_processor, assignment_exprent, const_exprent, invocation_exprent · vineflower · vineflower-option
- **Also spelled:** `decompiler-comments`, `dec`
- **Example:** `kuna decompile app.jar com.ex.Main --option jvm_decompiler_comments off`

### `jvm_sourcefile_comments` -- on | off, default `off`

- **Symptoms:** need the original .java file name of a class; repackaged or shaded classes of unknown origin.
- **What it does:** Write // $VF: Compiled from <name> above each class whose SourceFile attribute names its source file.
- **When to flip:** Turn on to see the original file names, which reveal class origins in shaded or repackaged jars.
- **Where / provenance:** p9_emit/class_writer · vineflower · vineflower-option
- **Also spelled:** `sourcefile-comments`, `sfc`
- **Example:** `kuna decompile app.jar com.ex.Main --option jvm_sourcefile_comments on`

### `jvm_dump_text_tokens` -- on | off, default `off` (destructive opt-in)

- **Symptoms:** need the source positions of identifiers for navigation; debugging which identifier a name in the output refers to.
- **What it does:** Debug aid: append to every file a dump of its text tokens (each class, field, method and variable reference or declaration with its line:column range).
- **When to flip:** Turn on only to debug token positions; the appended dump is not Java.
- **Where / provenance:** p0_knowledge/fernflower, text_token_dump_visitor · vineflower · vineflower-option
- **Also spelled:** `dump-text-tokens`, `dtt`
- **Example:** `kuna decompile app.jar com.ex.Main --option jvm_dump_text_tokens on`

### `jvm_remove_imports` -- on | off, default `off`

- **Symptoms:** long import blocks at the top of every file; imports are noise for the task at hand.
- **What it does:** Write no import statements; names keep the short form an import would allow, so the file reads like the class body alone and no longer compiles.
- **When to flip:** Turn on to save space when only method bodies are read, e.g. when feeding classes to a model.
- **Where / provenance:** p9_emit/import_collector · vineflower · vineflower-option
- **Also spelled:** `remove-imports`, `rim`
- **Example:** `kuna decompile app.jar com.ex.Main --option jvm_remove_imports on`

### `jvm_mark_corresponding_synthetics` -- on | off, default `off`

- **Symptoms:** stack trace names lambda$main$0 or Main$1 and the matching code is hard to find; need the class file of an anonymous class.
- **What it does:** Write a comment naming the synthetic construct behind each lambda (its lambda$name$N method) and each anonymous or local class (its Outer$N class).
- **When to flip:** Turn on to map lambdas and anonymous classes in the output back to the class-file members and stack-trace names they compile to.
- **Where / provenance:** p9_emit/class_writer · vineflower · vineflower-option
- **Also spelled:** `mark-corresponding-synthetics`, `mcs`
- **Example:** `kuna decompile app.jar com.ex.Handlers --option jvm_mark_corresponding_synthetics on`

### `jvm_plugins` -- on | off, default `on`

- **Symptoms:** output differs from Vineflower's test fixture but matches java -jar; a class is written in a plugin's language instead of Java; a bundled plugin's pass changes the Java output.
- **What it does:** Load the plugins bundled with the engine (Vineflower's VariableRenaming, Kotlin and IdeaNotNull), as java -jar vineflower.jar does; off runs the bare Java pipeline, like Vineflower's test fixture.
- **When to flip:** Turn off to see what the core Java decompiler makes of a class a plugin would claim, or to compare with Vineflower's test fixture, which loads no plugin.
- **Where / provenance:** p0_knowledge/fernflower, kuna_plugins · kuna · kuna-plugins
- **Example:** `kuna decompile app.jar com.ex.Main --option jvm_plugins off`

### `jvm_vineflower_compat` -- on | off, default `off`

- **Symptoms:** output differs from Vineflower 1.12.0; need Vineflower's exact text, failure comments included; a method decompiles in kuna but fails in Vineflower; failure comment names the kuna issue tracker instead of Vineflower's; a kuna fix is suspected of changing a method.
- **What it does:** Turn off every kuna-only fix and print exactly what Vineflower 1.12.0 prints, its failures included: a method Vineflower fails on is written as its // $VF: Couldn't be decompiled comment instead of being decompiled, code Vineflower prints wrong (such as a statement skipped after a denested if, or a Kotlin supertype call with default-argument placeholders) is printed as Vineflower prints it, and the note under a failure comment asks for reports to Vineflower's tracker instead of kuna's. The fixes have no options of their own (docs/jvm.md lists them).
- **When to flip:** Turn on to compare with Vineflower byte for byte, to check whether a difference from Vineflower comes from a kuna fix, or to reproduce a Vineflower failure for an upstream report; the regression corpus's Vineflower cases run this way. Off, every fix is on; each only replaces a failure or code Vineflower gets wrong.
- **Where / provenance:** p0_knowledge/kuna_fixes, fernflower · kuna · kuna-compat
- **Example:** `kuna decompile app.jar com.ex.Main --option jvm_vineflower_compat on`

### `jvm_kt_enable` -- on | off, default `on`

- **Symptoms:** Kotlin classes come out as Java with @Metadata(mv = ..., d1 = ...); a Kotlin class is written as .kt but Java output is wanted; Kotlin output of a class is wrong or failed.
- **What it does:** Kotlin plugin: write classes that carry @kotlin.Metadata as Kotlin source (.kt) through the plugin's own pipeline and writer; off writes them as Java (.java) with the @Metadata annotation, like any other class (needs jvm_plugins on, the default).
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

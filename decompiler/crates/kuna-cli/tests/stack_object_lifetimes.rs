//! Shared storage keeps incompatible object views and old escaped pointers coherent.
mod common;

use std::process::Command;
use std::time::Duration;

fn decompile(function: &str, enabled: bool) -> String {
    decompile_options(function, enabled, &[])
}

fn decompile_options(function: &str, enabled: bool, options: &[(&str, &str)]) -> String {
    decompile_annotated(function, enabled, options, &[])
}

fn decompile_annotated(
    function: &str,
    enabled: bool,
    options: &[(&str, &str)],
    assertions: &[&str],
) -> String {
    decompile_surface(function, enabled, options, assertions, false)
}

fn decompile_surface(
    function: &str,
    enabled: bool,
    options: &[(&str, &str)],
    assertions: &[&str],
    json: bool,
) -> String {
    let fixture = common::fixture(
        if matches!(
            function,
            "xor_view" | "double_bits" | "constant_bits" | "effect_bits"
        ) {
            "lifetimes_stack_representations_x86_64"
        } else if function == "group_fields" {
            "lifetimes_pointer_fields_x86_64"
        } else if matches!(function, "frame_alias" | "frame_seed" | "frame_cached") {
            "lifetimes_frame_events_x86_64"
        } else if matches!(function, "cached_stack_word" | "publish_frame_word") {
            "lifetimes_stack_snapshots_x86_64"
        } else if function == "rgb_reuse" {
            "lifetimes_partial_windows_x64"
        } else if function.starts_with("win_") {
            "lifetimes_windows_x64.exe"
        } else if function.starts_with("obj_") {
            "lifetimes_objects_x86_64"
        } else if matches!(
            function,
            "bounded_index_store"
                | "bounded_index_load"
                | "bounded_byte_store"
                | "bounded_reverse_store"
                | "unbounded_index_store"
        ) {
            "lifetimes_index_x86_64"
        } else {
            "lifetimes_stack_x86_64"
        },
    );
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command
        .args([
            "decompile",
            &fixture,
            function,
            "--mode",
            "reliable",
            "--assert-strict",
            "--assert",
        ])
        .arg(format!("@{fixture}.kuna"))
        .args([
            "--option",
            "stackviews",
            if enabled { "on" } else { "off" },
            "--option",
            "stackalias",
            "off",
            "--option",
            "structdefs",
            "on",
            "--sleighpath",
        ])
        .arg(common::repo_root().join("specs"));
    if function == "rgb_reuse" {
        command.args(["--target", "x86:LE:64:default:windows"]);
    }
    for (option, value) in options {
        command.args(["--option", option, value]);
    }
    for directive in assertions {
        command.args(["--assert", directive]);
    }
    if json {
        command.arg("--json");
    }
    let output = common::process::output_with_timeout(
        &mut command,
        Duration::from_secs(15),
        Duration::from_millis(25),
    )
    .expect("bounded lifetime decompilation");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    if json {
        let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert!(document["functions"][0]["error"].is_null(), "{document}");
        document["functions"][0]["code"]
            .as_str()
            .unwrap()
            .to_owned()
    } else {
        String::from_utf8(output.stdout).unwrap()
    }
}

const PRELUDE: &str = r#"
#include <stdint.h>
#include <stdbool.h>
#define CONCAT11(hi,lo) ((uint2)(((uint2)(uint1)(hi) << 8) | (uint1)(lo)))
typedef uint8_t undefined1; typedef float float4; typedef double float8;
typedef int8_t int1; typedef uint8_t uint1;
typedef int16_t int2; typedef uint16_t uint2;
typedef int32_t int4; typedef uint32_t uint4;
typedef int64_t int8; typedef uint64_t uint8;
struct LifetimePair; struct LifetimeHandle;
int4 datum = 55, win_datum = 55; int4 *escaped_pointer; float float_datum = 1.5f;
int4 read_pair(const struct LifetimePair *);
int4 read_handle(const struct LifetimeHandle *);
int4 same_address(const struct LifetimePair *, const struct LifetimeHandle *);
int4 read_saved_handle(void);
int4 read_escaped(void);
void write_saved(void);
void set_seven(int4 *);
int4 win_read_pair(const struct LifetimePair *);
int4 win_read_handle(const struct LifetimeHandle *);
int4 read_float(const float *);
int4 read_integer(const int4 *);
const int4 *get_pointer(void);
"#;
const HELPERS: &str = r#"
int4 read_pair(const struct LifetimePair *p) { return p->x + p->y; }
int4 read_handle(const struct LifetimeHandle *p) { return *p->value; }
int4 same_address(const struct LifetimePair *a, const struct LifetimeHandle *b) {
    return (const void *)a == (const void *)b ? *b->value + 1 : 0;
}
int4 read_saved_handle(void) { return **(int4 **)escaped_pointer; }
int4 read_escaped(void) { return *escaped_pointer; }
void write_saved(void) { *escaped_pointer = 7; }
void set_seven(int4 *p) { *p = 7; }
int4 win_read_pair(const struct LifetimePair *p) { return p->x + p->y; }
int4 win_read_handle(const struct LifetimeHandle *p) { return *p->value; }
int4 read_float(const float *p) { return (int4)*p; }
int4 read_integer(const int4 *p) { return *p; }
const int4 *get_pointer(void) { return &datum; }
int4 pair_callback(struct LifetimePair *p) { return read_pair(p); }
int4 handle_callback(struct LifetimeHandle *p) { return read_handle(p); }
"#;

fn native_control(function: &str, expected: i32) {
    native_arguments(function, "", expected);
}

fn native_arguments(function: &str, arguments: &str, expected: i32) {
    native_code(function, arguments, expected, decompile(function, true));
}

fn native_code(function: &str, arguments: &str, expected: i32, code: String) {
    native_cases(function, &[(arguments, expected)], code);
}

fn native_cases(function: &str, cases: &[(&str, i32)], code: String) {
    assert!(!code.contains("&Stack"), "{code}");
    if matches!(
        function,
        "stack_reuse" | "simultaneous_views" | "escaped_object_reuse"
    ) {
        assert!(code.contains("union stack_views_"), "{code}");
    }
    execute_emitted_cases(function, cases, code);
}

fn execute_emitted_cases(function: &str, cases: &[(&str, i32)], code: String) {
    let mut context = PRELUDE.to_string();
    if !code.contains("struct LifetimePair {") {
        context.push_str("struct LifetimePair { int4 x, y; };\n");
    }
    if !code.contains("struct LifetimeHandle {") {
        context.push_str("struct LifetimeHandle { int4 *value; };\n");
    }
    execute_with_context(function, cases, &context, &code, HELPERS);
}

fn execute_with_context(
    function: &str,
    cases: &[(&str, i32)],
    context: &str,
    code: &str,
    helpers: &str,
) {
    let compilers: Vec<_> = ["clang", "gcc"]
        .into_iter()
        .filter(|compiler| {
            common::process::optional_output(Command::new(compiler).arg("--version")).is_some()
        })
        .collect();
    if compilers.is_empty() {
        eprintln!("no C compiler available; skipping emitted-C execution");
        return;
    }
    let source = common::scratch_file(function, "c");
    let checks = cases
        .iter()
        .map(|(arguments, expected)| format!("{function}({arguments}) != {expected}"))
        .collect::<Vec<_>>()
        .join(" || ");
    std::fs::write(
        &source,
        format!("{context}\n{code}\n{helpers}\nint main(void) {{ return {checks}; }}\n"),
    )
    .unwrap();
    for compiler in compilers {
        for level in ["-O0", "-O2"] {
            let binary = source.with_extension(format!("{compiler}{}", &level[1..]));
            let strict: &[&str] = if function.starts_with("obj_global")
                || function.starts_with("obj_volatile")
                || function == "obj_indirect_reuse"
                || matches!(
                    function,
                    "check_snapshot"
                        | "check_publication"
                        | "group_fields"
                        | "check_frame_events"
                        | "check_frame_cache"
                ) {
                &[
                    "-Werror=int-conversion",
                    "-Werror=incompatible-pointer-types",
                ]
            } else {
                &[]
            };
            let initialized: &[&str] =
                if matches!(function, "check_frame_events" | "check_frame_cache") {
                    &["-Werror=uninitialized"]
                } else {
                    &[]
                };
            common::process::required_output(
                Command::new(compiler)
                    .args(["-std=c11", level, "-fstrict-aliasing"])
                    .args(strict)
                    .args(initialized)
                    .arg(&source)
                    .arg("-o")
                    .arg(&binary),
            );
            common::process::required_output(&mut Command::new(binary));
        }
    }
}

#[test]
fn frame_bookkeeping_preserves_native_writes_and_real_snapshots() {
    let fixture = common::fixture("lifetimes_frame_events_x86_64");
    common::process::required_output(&mut Command::new(&fixture));
    let context = r#"
#include <stdint.h>
#include <string.h>
typedef uint8_t uint1; typedef uint8_t undefined1;
typedef uint16_t uint2; typedef uint32_t uint4; typedef uint64_t uint8;
typedef int32_t int4;
struct ChoiceItem;
void expose_input(const struct ChoiceItem *item);
void read_key(const int4 *key);
void read_item(struct ChoiceItem *const *item);
void advance_word(uint8 *word);
extern struct ChoiceItem item_two;
"#;
    for function in ["frame_alias", "frame_seed"] {
        let code = decompile(function, true);
        assert!(!code.contains("SUB84("), "{code}");
        assert!(!code.contains("0xffffffff00000000"), "{code}");
        let helpers = format!(
            r#"
struct ChoiceItem item_two = {{2,14}};
static struct ChoiceArray *active_array;
static int alias_enabled, key_calls, observed_keys[8];
static struct ChoiceItem *observed_item;
void read_key(const int4 *key) {{ observed_keys[key_calls++] = *key; }}
void expose_input(const struct ChoiceItem *item) {{
    read_key(&item->kind);
    if (alias_enabled) active_array->items[0] = (struct ChoiceItem *)(void *)item;
}}
void read_item(struct ChoiceItem *const *item) {{ observed_item = *item; }}
int check_frame_events(int scenario) {{
    struct ChoiceItem one={{1,23}},ignored={{0,100}};
    struct ChoiceItem *items[]={{&one,&ignored,&item_two,0}};
    struct ChoiceItem *slots[2]={{0,0}};
    struct ChoiceQueue queue={{slots,slots,slots+2}};
    struct ChoiceArray array={{4,{{0}},items}};
    int initial = {initial};
    alias_enabled = scenario == 1;
    key_calls = 0; observed_item = 0; active_array = &array;
    if (scenario == 0) array.count = 0;
    if (scenario == 3) queue.capacity = slots;
    {function}(&array,&queue);
    if (observed_keys[0] != initial || key_calls != (scenario == 0 ? 1 : 2)) return 1;
    if (scenario == 0) return queue.end != slots;
    if (observed_keys[1] != 23) return 2;
    if (scenario == 3) return observed_item != &item_two || queue.end != slots;
    return slots[0] != &item_two || queue.end != slots+1;
}}
"#,
            initial = if function == "frame_alias" { 1 } else { 0 }
        );
        let cases = if function == "frame_alias" {
            vec![("0", 0), ("1", 0), ("2", 0), ("3", 0)]
        } else {
            vec![("0", 0), ("2", 0), ("3", 0)]
        };
        execute_with_context("check_frame_events", &cases, context, &code, &helpers);
    }
    let code = decompile("frame_cached", true);
    let helpers = r#"
struct ChoiceItem item_two = {2,14};
static int observed_key; static struct ChoiceItem *observed_item;
void advance_word(uint8 *word) { *word += 2; }
void read_key(const int4 *key) { observed_key = *key; }
void read_item(struct ChoiceItem *const *item) { observed_item = *item; }
int check_frame_cache(uint8 input) {
    uint8 saved = frame_cached(input);
    return saved != input+2 || observed_key != 5 || observed_item != &item_two;
}
"#;
    execute_with_context(
        "check_frame_cache",
        &[("13", 0), ("0x12345678fffffffeULL", 0)],
        context,
        &code,
        helpers,
    );
}

#[test]
fn object_names_preserve_pointer_field_dereferences() {
    let fixture = common::fixture("lifetimes_pointer_fields_x86_64");
    common::process::required_output(&mut Command::new(&fixture));
    let context = r#"
#include <stdint.h>
#include <stdbool.h>
typedef uint8_t uint1; typedef uint8_t undefined1;
typedef uint16_t uint2; typedef uint32_t uint4; typedef uint64_t uint8;
typedef int32_t int4; typedef float float4;
struct FieldSet;
bool construct_set(struct FieldSet *set);
"#;
    let helpers = r#"
static struct FieldNode second_node = { .value = 31 };
static struct FieldNode first_node = { .next = &second_node, .value = 7 };
bool construct_set(struct FieldSet *set) { set->head = &first_node; return true; }
"#;
    for assertions in [
        &[][..],
        &["name group_fields::object_401007_1 target_set"][..],
    ] {
        let code = decompile_annotated("group_fields", true, &[], assertions);
        assert!(!code.contains("PTRSUB("), "{code}");
        assert!(code.contains("->next->value"), "{code}");
        execute_with_context("group_fields", &[("", 31)], context, &code, helpers);
    }
}

#[test]
fn typed_address_publications_preserve_old_reads_and_object_views() {
    let fixture = common::fixture("lifetimes_objects_x86_64");
    common::process::required_output(&mut Command::new(&fixture));
    let context = r#"
#include <stdint.h>
typedef uint8_t uint1; typedef uint8_t undefined1;
typedef uint16_t uint2; typedef uint32_t uint4; typedef uint64_t uint8;
typedef int32_t int4;
struct LifetimePair; struct LifetimeHandle; struct LifetimeSink;
int4 object_datum = 55;
struct LifetimePair *object_pair;
struct LifetimeHandle *object_handle;
struct LifetimePair *volatile volatile_pair;
struct LifetimeHandle *volatile volatile_handle;
"#;
    for function in [
        "obj_global_reuse",
        "obj_volatile_reuse",
        "obj_global_shifted",
        "obj_volatile_shifted",
        "obj_indirect_reuse",
    ] {
        let (arguments, helpers) = if function == "obj_indirect_reuse" {
            ("&indirect_sink", "struct LifetimeSink indirect_sink;")
        } else {
            ("", "")
        };
        let code = decompile(function, true);
        assert!(code.contains("view_LifetimePair"), "{code}");
        assert!(code.contains("view_LifetimeHandle"), "{code}");
        execute_with_context(function, &[(arguments, 67)], context, &code, helpers);
        let output = common::process::required_output(
            Command::new(env!("CARGO_BIN_EXE_kuna"))
                .args([
                    "decompile",
                    &fixture,
                    function,
                    "--json",
                    "--mode",
                    "reliable",
                    "--assert-strict",
                    "--assert",
                    &format!("@{fixture}.kuna"),
                    "--option",
                    "stackviews",
                    "on",
                    "--sleighpath",
                ])
                .arg(common::repo_root().join("specs")),
        );
        let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert!(document["functions"][0]["error"].is_null(), "{document}");
        let selector = document["functions"][0]["stack_objects"][0]["id"]
            .as_str()
            .unwrap();
        let name = format!("name {function}::{selector} pair_phase");
        let retype = format!("type {function}::pair_phase struct SemanticPair");
        let code = decompile_annotated(
            function,
            true,
            &[],
            &[
                "typedef struct SemanticPair { int left; int right; };",
                &name,
                &retype,
            ],
        );
        assert!(code.contains(".pair_phase"), "{code}");
        execute_with_context(function, &[(arguments, 67)], context, &code, helpers);
    }
    for (function, cases) in [
        ("obj_volatile_branch", vec![("0", 14), ("1", 16)]),
        ("obj_volatile_partial", vec![("", 0x9900)]),
        ("obj_volatile_loop", vec![("", 8)]),
    ] {
        let code = decompile(function, true);
        execute_with_context(function, &cases, context, &code, "");
    }
}

#[test]
fn same_layout_incarnations_accept_independent_names_and_types() {
    let code = decompile_annotated(
        "obj_reuse",
        true,
        &[],
        &[
            "typedef struct SemanticPair { int left; int right; };",
            "name obj_reuse::object_40104b_1 initial",
            "name obj_reuse::object_401064_1 replacement",
            "type obj_reuse::initial struct SemanticPair",
        ],
    );
    assert!(
        code.lines()
            .any(|line| line.contains("read_pair(") && line.contains(".initial")),
        "{code}"
    );
    assert!(
        code.lines()
            .any(|line| line.contains("read_pair(") && line.contains(".replacement")),
        "{code}"
    );
    assert!(code.contains("initial.left = 5"), "{code}");
    assert!(code.contains("replacement.x = 9"), "{code}");
    native_code("obj_reuse", "", 32, code);
}

#[test]
fn names_over_locked_layouts_share_native_storage_and_reuse_one_pointer() {
    for (function, backing, names, expected) in [
        (
            "obj_repeat",
            "stack_views_401006_m18",
            vec!["name obj_repeat::object_40101d_1 repeated"],
            24,
        ),
        (
            "obj_reuse",
            "stack_views_401034_m18",
            vec![
                "name obj_reuse::object_40104b_1 initial",
                "name obj_reuse::object_401064_1 replacement",
            ],
            32,
        ),
    ] {
        for reverse in [false, true] {
            let lock = format!("type {function}::v2 {backing}");
            let mut order = names.clone();
            if reverse {
                order.reverse();
            }
            let mut assertions = vec![lock.as_str()];
            assertions.extend(order);
            let code = decompile_annotated(function, true, &[], &assertions);
            assert!(code.contains(&format!("{backing} v2;")), "{code}");
            for directive in &names {
                let name = directive.split_whitespace().last().unwrap();
                assert_eq!(code.matches(&format!("*{name}_view;")).count(), 1, "{code}");
                assert_eq!(code.matches(&format!("{name}_view =")).count(), 1, "{code}");
                assert_eq!(
                    code.matches(&format!("read_pair({name}_view)")).count(),
                    if function == "obj_repeat" { 2 } else { 1 },
                    "{code}",
                );
            }
            native_code(function, "", expected, code);
        }
    }
}

#[test]
fn locked_pointer_views_keep_interiors_retypes_and_name_collisions_readable() {
    let interior = decompile_annotated(
        "shifted_views",
        true,
        &[],
        &[
            "type shifted_views::v3 stack_views_401324_m18_1",
            "name shifted_views::object_40133b_1 original",
            "name shifted_views::object_401355_1 updated",
            "name shifted_views::object_401361_1 interior",
        ],
    );
    assert!(interior.contains("read_pair(interior_view)"), "{interior}");
    native_code("shifted_views", "", 52, interior);

    let collision = decompile_annotated(
        "obj_reuse",
        true,
        &[],
        &[
            "type obj_reuse::v2 stack_views_401034_m18",
            "name obj_reuse::v1 initial_view",
            "name obj_reuse::object_40104b_1 initial",
            "name obj_reuse::object_401064_1 replacement",
        ],
    );
    assert!(collision.contains("int4 initial_view;"), "{collision}");
    assert!(collision.contains("read_pair(initial_view_"), "{collision}");
    native_code("obj_reuse", "", 32, collision);

    let retyped = decompile_annotated(
        "obj_reuse",
        true,
        &[],
        &[
            "typedef struct SemanticPair { int left; int right; };",
            "type obj_reuse::v2 stack_views_401034_m18",
            "name obj_reuse::object_40104b_1 initial",
            "type obj_reuse::initial struct SemanticPair",
            "name obj_reuse::object_401064_1 replacement",
        ],
    );
    assert!(retyped.contains("SemanticPair *initial_view;"), "{retyped}");
    assert!(
        retyped.contains("read_pair((LifetimePair *)initial_view)"),
        "{retyped}",
    );
    native_code("obj_reuse", "", 32, retyped);
}

#[test]
fn differently_typed_incarnations_keep_names_and_their_physical_address() {
    let code = decompile_annotated(
        "stack_reuse",
        true,
        &[],
        &[
            "name stack_reuse::object_401017_1 coordinates",
            "name stack_reuse::object_40102c_1 handle",
        ],
    );
    assert!(code.contains(".coordinates"), "{code}");
    assert!(code.contains(".handle"), "{code}");
    assert!(code.contains("handle.value = &datum"), "{code}");
    native_code("stack_reuse", "", 67, code);
}

#[test]
fn named_objects_keep_aliases_interior_views_and_uncertain_effects() {
    for (function, cases, names) in [
        (
            "simultaneous_views",
            vec![("", 56)],
            vec![("object_40115b_1", "shared")],
        ),
        (
            "escaped_object_reuse",
            vec![("", 122)],
            vec![("object_401193_1", "earlier"), ("object_4011af_1", "later")],
        ),
        (
            "shifted_views",
            vec![("", 52)],
            vec![
                ("object_40133b_1", "initial"),
                ("object_401355_1", "middle"),
                ("object_401361_1", "interior"),
            ],
        ),
        (
            "float_reuse",
            vec![("", 10)],
            vec![("object_401387_1", "real"), ("object_401398_1", "integer")],
        ),
        (
            "typed_indirect",
            vec![("pair_callback,handle_callback", 67)],
            vec![("object_40142e_1", "pair"), ("object_401441_1", "handle")],
        ),
        (
            "obj_repeat",
            vec![("", 24)],
            vec![("object_40101d_1", "pair")],
        ),
        (
            "obj_branch",
            vec![("0", 20), ("1", 12)],
            vec![("object_40109c_1", "pair")],
        ),
        (
            "obj_loop",
            vec![("", 28)],
            vec![("object_4010c8_1", "pair")],
        ),
        (
            "obj_partial",
            vec![("", 43544)],
            vec![
                ("object_4010fd_1", "initial"),
                ("object_40110c_1", "updated"),
            ],
        ),
    ] {
        let directives: Vec<_> = names
            .iter()
            .map(|(selector, name)| format!("name {function}::{selector} {name}"))
            .collect();
        let assertions: Vec<_> = directives.iter().map(String::as_str).collect();
        let code = decompile_annotated(function, true, &[], &assertions);
        for (_, name) in names {
            assert!(code.contains(&format!(".{name}")), "{function}: {code}");
        }
        native_cases(function, &cases, code);
    }
}

#[test]
fn logical_object_reuse_joins_loops_and_partial_updates_match_native() {
    common::process::required_output(&mut Command::new(common::fixture(
        "lifetimes_objects_x86_64",
    )));
    for (function, cases) in [
        ("obj_repeat", vec![("", 24)]),
        ("obj_reuse", vec![("", 32)]),
        ("obj_branch", vec![("0", 20), ("1", 12)]),
        ("obj_loop", vec![("", 28)]),
        ("obj_partial", vec![("", 43544)]),
    ] {
        native_cases(function, &cases, decompile(function, true));
    }
}

#[test]
fn named_objects_survive_indexed_store_guard_retries() {
    for guard in ["on", "off"] {
        let code = decompile_annotated(
            "bounded_byte_store",
            true,
            &[("stackstoreguard", guard)],
            &[
                "name bounded_byte_store::object_401092_1 initial",
                "name bounded_byte_store::object_4010a1_1 updated",
            ],
        );
        assert!(code.contains("read_pair(&v2.initial)"), "{code}");
        assert!(code.contains("read_pair(&v2.updated)"), "{code}");
        native_cases("bounded_byte_store", &[("0", 28), ("1", 26)], code);
    }
}

#[test]
fn bounded_word_and_byte_indices_use_the_whole_eight_byte_object() {
    common::process::required_output(&mut Command::new(common::fixture("lifetimes_index_x86_64")));
    for (function, expected) in [
        ("bounded_index_store", [28, 26]),
        ("bounded_index_load", [17, 19]),
        ("bounded_byte_store", [28, 26]),
        ("bounded_reverse_store", [26, 28]),
    ] {
        let baseline = decompile(function, false);
        assert!(
            baseline.contains("[3]") || baseline.contains("._0_4_"),
            "{baseline}"
        );
        let code = decompile(function, true);
        assert!(code.contains("bytes[8]"), "{code}");
        assert!(code.contains("__builtin_memcpy"), "{code}");
        assert!(!code.contains("[3]"), "{code}");
        native_cases(function, &[("0", expected[0]), ("1", expected[1])], code);
    }
}

#[test]
fn unresolved_index_bounds_do_not_invent_an_object_extent() {
    let baseline = decompile("unbounded_index_store", false);
    let enabled = decompile("unbounded_index_store", true);
    assert_eq!(baseline, enabled);
    assert!(!enabled.contains("union stack_views_"), "{enabled}");
}

#[test]
fn a_renamed_indexed_backing_survives_assertion_replay() {
    let code = decompile_annotated(
        "bounded_reverse_store",
        true,
        &[],
        &["name bounded_reverse_store::v2 frame_bytes"],
    );
    assert!(code.contains("(uint1 *)&frame_bytes"), "{code}");
    native_cases("bounded_reverse_store", &[("0", 26), ("1", 28)], code);
}

#[test]
fn reused_pair_and_handle_share_their_storage() {
    let baseline = decompile("stack_reuse", false);
    assert!(!baseline.contains("union stack_views_"), "{baseline}");
    native_control("stack_reuse", 67);
}

#[test]
fn simultaneous_call_views_keep_one_address() {
    native_control("simultaneous_views", 56);
}

#[test]
fn an_old_escaped_pointer_observes_the_new_object() {
    native_control("escaped_object_reuse", 122);
}

#[test]
fn partial_writes_control_flow_and_unknown_memory_effects_match_native() {
    for (function, arguments, expected) in [
        ("escaped_alias", "", 7),
        ("suffix_reinit", "123", 123),
        ("suffix_initial", "", 0x11223344),
        ("suffix_callee_update", "", 7),
        ("suffix_callee_reinit", "", 7),
        ("escaped_two_reads", "", 12),
        ("escaped_partial", "", 0x9900),
        ("escaped_branch", "0", 7),
        ("escaped_branch", "1", 5),
        ("escaped_loop", "", 3),
        ("escaped_indirect_write", "", 7),
        ("escaped_indirect_partial", "", 0x1122aa44),
        ("callee_updates_saved", "", 7),
        ("callback_rewrite", "set_seven", 7),
        (
            "indexed_bytes",
            "(const uint1[]){0xaa,0xbb,0xcc,0xdd}",
            0x11ccbbaa,
        ),
        (
            "indexed_words",
            "(const uint2[]){0xbbaa,0xddcc}",
            0xddccbbaa_u32 as i32,
        ),
        ("indexed_load", "0", 0x3344),
        ("indexed_load", "1", 0x1122),
    ] {
        native_arguments(function, arguments, expected);
    }
    common::process::required_output(&mut Command::new(common::fixture("lifetimes_stack_x86_64")));
}

#[test]
fn explicit_stack_effects_survive_disabling_generic_index_guards() {
    for (function, expected) in [
        ("escaped_indirect_write", 7),
        ("escaped_indirect_partial", 0x1122aa44),
        ("callee_updates_saved", 7),
    ] {
        native_code(
            function,
            "",
            expected,
            decompile_options(function, true, &[("indexaliasguard", "off")]),
        );
    }
}

#[test]
fn aliased_suffix_writes_keep_only_observable_initial_bytes() {
    for (function, arguments, expected) in [
        ("suffix_reinit", "123", 123),
        ("suffix_initial", "", 0x11223344),
    ] {
        native_code(
            function,
            arguments,
            expected,
            decompile_options(function, false, &[("stackalias", "on")]),
        );
    }
}

#[test]
fn records_at_different_offsets_keep_shared_interior_bytes() {
    native_control("shifted_views", 52);
}

#[test]
fn windows_home_space_keeps_the_reused_object_above_arguments() {
    native_control("win_stack_reuse", 67);
}

#[test]
fn floating_and_integer_incarnations_preserve_storage_bits() {
    native_control("float_reuse", 10);
}

#[test]
fn numeric_conversion_targets_the_scalar_view() {
    native_arguments("converted_view", "7", 16);
    native_arguments("converted_view", "-7", 2);
}

#[test]
fn declared_stack_layouts_preserve_incompatible_stores() {
    common::process::required_output(&mut Command::new(common::fixture("lifetimes_stack_x86_64")));
    let code = decompile_surface(
        "stack_reuse",
        true,
        &[],
        &[
            "type stack_reuse::v2 struct LifetimePair protected_pair",
            "name stack_reuse::object_401017_1 initial_pair",
            "name stack_reuse::object_40102c_1 replacement_handle",
        ],
        true,
    );
    assert!(code.contains("LifetimePair protected_pair;"), "{code}");
    assert!(!code.contains("protected_pair = &datum"), "{code}");
    execute_emitted_cases("stack_reuse", &[("", 67)], code);
    for (function, cases) in [
        ("float_reuse", vec![("", 10)]),
        ("converted_view", vec![("7", 16), ("-7", 2)]),
    ] {
        let directive = format!("type {function}::v2 struct LifetimeFloatStorage protected_float",);
        let code = decompile_surface(
            function,
            true,
            &[],
            &[
                "typedef struct LifetimeFloatStorage { float value; };",
                &directive,
            ],
            true,
        );
        assert!(
            code.contains("LifetimeFloatStorage protected_float;"),
            "{code}"
        );
        execute_emitted_cases(function, &cases, code);
    }
}

#[test]
fn partial_color_initialization_preserves_reused_stack_bytes() {
    common::process::required_output(&mut Command::new(common::fixture(
        "lifetimes_partial_windows_x64",
    )));
    let context = r#"
#include <stdint.h>
#include <stdbool.h>
typedef uint8_t undefined1, uint1; typedef uint16_t uint2;
typedef uint32_t uint4; typedef uint64_t uint8; typedef int32_t int4;
struct RGB; struct Manager;
int4 datum = 55, rgb_result;
void read_rgb(const struct RGB *);
struct Manager *make_manager(void);
int4 read_manager(const struct Manager *);
"#;
    let helpers = r#"
void read_rgb(const struct RGB *p) { rgb_result = p->red + p->green + p->blue; }
struct Manager *make_manager(void) { return (struct Manager *)&datum; }
int4 read_manager(const struct Manager *p) { return p->value; }
"#;
    for assertions in [
        vec![],
        vec!["name rgb_reuse::object_40102c_1 recorded_color"],
    ] {
        let code = decompile_surface("rgb_reuse", true, &[], &assertions, true);
        assert!(!code.contains("._3_5_"), "{code}");
        assert!(!code.contains("._0_3_"), "{code}");
        execute_with_context(
            "rgb_reuse",
            &[
                ("0, false, 0", 460),
                ("0, false, 255", 460),
                ("0, true, 0", 510),
                ("0, true, 1", 511),
                ("0, true, 255", 765),
            ],
            context,
            &code,
            helpers,
        );
    }
}

#[test]
fn returned_pointer_bytes_match_the_handle_view() {
    native_control("returned_pointer", 55);
}

#[test]
fn typed_indirect_calls_select_their_declared_object_views() {
    let code = decompile("typed_indirect", true);
    assert!(code.contains("view_LifetimePair"), "{code}");
    assert!(code.contains("view_LifetimeHandle"), "{code}");
    assert_eq!(code.matches("\nunion stack_views_").count(), 1, "{code}");
    native_code("typed_indirect", "pair_callback,handle_callback", 67, code);
}

#[test]
fn complete_and_conditional_cfg_overwrites_match_native() {
    for (function, first, first_expected, second, second_expected, keeps_initial) in [
        ("all_paths_overwrite", 0, 7, 1, 5, false),
        ("one_path_overwrites", 0, 1, 1, 5, true),
        ("loop_overwrites", 0, 7, 2, 7, false),
        ("loop_may_skip", 0, 1, 2, 7, true),
    ] {
        let code = decompile(function, true);
        assert_eq!(code.contains(" = 1;"), keeps_initial, "{function}: {code}");
        let control = format!(
            "{code}\nint4 cfg_native_control(void) {{ return \
             {function}({first}) == {first_expected} && \
             {function}({second}) == {second_expected}; }}\n"
        );
        native_code("cfg_native_control", "", 1, control);
    }
}

#[test]
fn declared_float_members_keep_bits_and_call_effects_are_not_stores() {
    common::process::required_output(&mut Command::new(common::fixture(
        "lifetimes_stack_representations_x86_64",
    )));
    let context = r#"
#include <stdint.h>
typedef uint8_t uint1; typedef uint16_t uint2; typedef uint32_t uint4; typedef uint64_t uint8;
typedef int32_t int4; typedef float float4; typedef double float8;
uint4 observed_word; uint8 observed_quad;
int4 read_float(const float4 *); int4 read_double(const float8 *);
void observe_word(const uint4 *); void observe_quad(const uint8 *);
"#;
    let helpers = r#"
int4 read_float(const float4 *p) { return (int4)(*p * 2.0f); }
int4 read_double(const float8 *p) { return (int4)(*p * 2.0); }
void observe_word(const uint4 *p) { __builtin_memcpy(&observed_word,p,4); }
void observe_quad(const uint8 *p) { __builtin_memcpy(&observed_quad,p,8); }
void write_word(uint4 *p) { uint4 bits=0x40400000; __builtin_memcpy(p,&bits,4); }
"#;
    for (function, datatype, name, cases, control) in [
        (
            "xor_view", "FloatStorage { float value; };", "protected_float",
            vec![
                ("0x3f800000u,5,0x3fc00000u",0),
                ("0xbf800000u,-5,0xbfc00000u",0),
                ("0x3fc00000u,5,0x3f800000u",0),
                ("0x40400000u,10,0x40000000u",0),
                ("0xc0400000u,-10,0xc0000000u",0),
                ("0u,0,0x400000u",0),
                ("0x80000000u,0,0x80400000u",0),
            ],
            "int4 check_bits(uint4 bits,int4 want,uint4 word) { float4 initial; \
             __builtin_memcpy(&initial,&bits,4); return xor_view(initial)!=want || observed_word!=word; }",
        ),
        (
            "double_bits", "DoubleStorage { double value; };", "protected_double",
            vec![
                ("0x3ff0000000000000ULL,5,0x3ff8000000000000ULL",0),
                ("0xbff0000000000000ULL,-5,0xbff8000000000000ULL",0),
                ("0ULL,0,0x8000000000000ULL",0),
                ("0x8000000000000000ULL,0,0x8008000000000000ULL",0),
            ],
            "int4 check_bits(uint8 bits,int4 want,uint8 word) { float8 initial; \
             __builtin_memcpy(&initial,&bits,8); return double_bits(initial)!=want || observed_quad!=word; }",
        ),
    ] {
        let declaration = format!("typedef struct {datatype}");
        let ty = datatype.split_whitespace().next().unwrap();
        let directive = format!("type {function}::v2 struct {ty} {name}");
        let code = decompile_surface(function,true,&[],&[&declaration,&directive],true);
        assert!(code.contains(&format!("{ty} {name};")),"{code}");
        let helpers = format!("{helpers}\n{control}");
        execute_with_context("check_bits",&cases,context,&code,&helpers);
    }
    for (function, expected, word, call) in [
        ("constant_bits", 5, "0x3fc00000", "constant_bits()"),
        ("effect_bits", 6, "0x40400000", "effect_bits(write_word)"),
    ] {
        let code = decompile_surface(function, true, &[], &[], true);
        assert!(!code.contains(".bytes._"), "{code}");
        let control = format!("{helpers}\nint4 check_bits(void) {{ return {call}!={expected} || observed_word!={word}; }}");
        execute_with_context("check_bits", &[("", 0)], context, &code, &control);
    }
}

#[test]
fn cached_frame_words_keep_scalar_types_across_callbacks_and_reuse() {
    common::process::required_output(&mut Command::new(common::fixture(
        "lifetimes_stack_snapshots_x86_64",
    )));
    let context = r#"
#include <stdint.h>
typedef uint8_t uint1; typedef uint16_t uint2;
typedef uint32_t uint4; typedef uint64_t uint8; typedef int32_t int4;
struct LifetimeHandle;
int4 datum = 55;
int4 read_integer(const int4 *); int4 read_handle(const struct LifetimeHandle *);
"#;
    let helpers = r#"
int4 read_integer(const int4 *p) { return *p; }
int4 read_handle(const struct LifetimeHandle *p) { return *p->value; }
void callback_full(void *p) {
    uint8 word; __builtin_memcpy(&word,p,8); word += 2; __builtin_memcpy(p,&word,8);
}
void callback_half(void *p) {
    uint4 word; __builtin_memcpy(&word,p,4); word += 2; __builtin_memcpy(p,&word,4);
}
int4 check_snapshot(uint8 input,void (*callback)(void *),uint8 expected) {
    return cached_stack_word(input,callback) != expected;
}
"#;
    let cases = [
        ("13,callback_full,75", 0),
        ("(uint8)-7,callback_full,55", 0),
        (
            "0x12345678fffffffeULL,callback_full,0x123456790000003cULL",
            0,
        ),
        (
            "0x12345678fffffffeULL,callback_half,0x123456780000003cULL",
            0,
        ),
        ("0xfffffffffffffffeULL,callback_full,60", 0),
        (
            "0xfffffffffffffffeULL,callback_half,0xffffffff0000003cULL",
            0,
        ),
    ];
    for json in [false, true] {
        for assertions in [
            vec![],
            vec!["name cached_stack_word::v1 cached_word"],
            vec![
                "name cached_stack_word::v1 cached_word",
                "type cached_stack_word::cached_word unsigned long long",
            ],
        ] {
            let code = decompile_surface("cached_stack_word", true, &[], &assertions, json);
            let name = if assertions.is_empty() {
                "v1"
            } else {
                "cached_word"
            };
            assert!(code.contains(&format!("uint8 {name};")), "{code}");
            assert!(!code.contains(".bytes._"), "{code}");
            execute_with_context("check_snapshot", &cases, context, &code, helpers);
        }
    }
}

#[test]
fn frame_word_publications_preserve_declared_queue_pointer_types() {
    let opaque = [
        "prototype read_integer int read_integer(void *word)",
        "prototype read_handle int read_handle(void *handle)",
    ];
    let context = r#"
#include <stdint.h>
typedef uint8_t uint1; typedef uint16_t uint2;
typedef uint32_t uint4; typedef uint64_t uint8; typedef int32_t int4;
int4 datum = 55, other = -7;
int4 read_integer(void *); int4 read_handle(void *);
"#;
    let helpers = r#"
int4 read_integer(void *p) { int4 value; __builtin_memcpy(&value,p,4); return value; }
int4 read_handle(void *p) { int4 *value; __builtin_memcpy(&value,p,8); return *value; }
int4 check_publication(uint4 capacity,int4 *value) {
    int4 *slots[3] = {0,0,0};
    LifetimeQueue queue = {slots,slots,slots+capacity};
    for (uint4 attempt=0; attempt<3; ++attempt) {
        uint4 count = attempt+1 < capacity ? attempt+1 : capacity;
        if (publish_frame_word(&queue,&value) != 5+*value ||
            queue.begin != slots || queue.end != slots+count ||
            queue.capacity != slots+capacity || slots[capacity] != 0) return 1;
        for (uint4 slot=0; slot<count; ++slot) if (slots[slot] != value) return 1;
    }
    return 0;
}
"#;
    let baseline = decompile_surface("publish_frame_word", true, &[], &opaque, true);
    let backing = baseline
        .lines()
        .find_map(|line| {
            line.strip_prefix("union stack_views_")
                .map(|rest| format!("stack_views_{}", rest.split_whitespace().next().unwrap()))
        })
        .expect("generated frame backing");
    let retype = format!("type publish_frame_word::publication_frame {backing}");
    for json in [false, true] {
        for names in [
            vec![],
            vec!["name publish_frame_word::v4 publication_frame"],
            vec![
                "name publish_frame_word::v4 publication_frame",
                retype.as_str(),
            ],
        ] {
            let assertions: Vec<_> = opaque
                .iter()
                .copied()
                .chain(names.iter().copied())
                .collect();
            let code = decompile_surface("publish_frame_word", true, &[], &assertions, json);
            assert!(code.contains("int4 **v1;"), "{code}");
            assert!(
                !code.contains(".bytes._") && !code.contains("(uint1[8])"),
                "{code}"
            );
            execute_with_context(
                "check_publication",
                &[("0,&datum", 0), ("1,&datum", 0), ("2,&other", 0)],
                context,
                &code,
                helpers,
            );
        }
    }
}

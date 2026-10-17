//! Source-value calls retain their aggregate fields and distinguish pointer parameters.
use crate::common;
use common::process;
use std::process::Command;

#[test]
fn aggregate_calls_and_function_pointers_compile_and_execute() {
    let xml = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/stages/kuna-indirect-value-arguments.xml"
    ));
    let chunk =
        regex::Regex::new(r#"<bytechunk[^>]*offset="0x([0-9a-f]+)"[^>]*>([0-9a-f]+)</bytechunk>"#)
            .unwrap();
    let mut bytes = Vec::new();
    for captures in chunk.captures_iter(xml) {
        let offset = usize::from_str_radix(&captures[1], 16).unwrap() - 0x1000;
        let hex = &captures[2];
        bytes.resize(offset, 0);
        bytes.extend(
            (0..hex.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()),
        );
    }
    let image = common::scratch_file("indirect-values", "bin");
    std::fs::write(&image, bytes).unwrap();
    let declarations = [
        "typedef struct Pair { int a; int b; } Pair;",
        "prototype pointer_sink void pointer_sink(Pair *input)",
        "prototype value_sink void value_sink(Pair value)",
        "prototype stack_value void stack_value(int a,int b)",
        "prototype stack_pointer void stack_pointer(int a,int b)",
        "prototype pointer_value void pointer_value(Pair *input)",
        "prototype indirect_value void indirect_value(int a,int b,void (*fn)(Pair value))",
        "prototype value_reader int value_reader(Pair value)",
        "typedef struct Link { struct Link *next; int count; } Link;",
        "prototype advanced int advanced(Pair value)",
        "prototype spilled int spilled(Pair value)",
        "prototype updated int updated(Pair value)",
        "prototype cursor int cursor(Link value)",
        "prototype indirect_pointer void indirect_pointer(int a,int b,void (*ptr_fn)(Pair *))",
        "typedef struct PairReverse { int a; int b; } PairReverse;",
        "prototype value_sink_reverse void value_sink_reverse(PairReverse value)",
        "prototype pointer_sink_reverse void pointer_sink_reverse(PairReverse *input)",
        "prototype stack_value_reverse void stack_value_reverse(int a,int b)",
        "prototype stack_pointer_reverse void stack_pointer_reverse(int a,int b)",
        "prototype stack_cursor int stack_cursor(int a,int b,int c,int d,int e,int f,Link value)",
        "prototype collision_cursor int collision_cursor(Link value,int value_addr)",
    ];
    let functions = [
        "0x1000-0x1020=stack_value",
        "0x1020-0x1040=stack_pointer",
        "0x1040-0x1058=pointer_value",
        "0x1058-0x1078=indirect_value",
        "0x1078-0x1090=value_reader",
        "0x1200-0x1208=value_sink",
        "0x1210-0x1218=pointer_sink",
        "0x1300-0x1320=advanced",
        "0x1320-0x1340=spilled",
        "0x1340-0x1360=updated",
        "0x1370-0x13a0=cursor",
        "0x1400-0x1420=indirect_pointer",
        "0x1500-0x1520=stack_value_reverse",
        "0x1540-0x1560=stack_pointer_reverse",
        "0x1600-0x1608=value_sink_reverse",
        "0x1610-0x1618=pointer_sink_reverse",
        "0x1700-0x1734=stack_cursor",
        "0x1740-0x1770=collision_cursor",
    ];
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command.args([
        "decompile-all",
        image.to_str().unwrap(),
        "--raw-image",
        "--entry",
        "0x1000",
        "--base",
        "0x1000",
        "--target",
        "sparc:BE:32:default:default",
        "--addr",
        "0x1000",
        "--addr",
        "0x1020",
        "--addr",
        "0x1040",
        "--addr",
        "0x1058",
        "--addr",
        "0x1078",
        "--addr",
        "0x1300",
        "--addr",
        "0x1320",
        "--addr",
        "0x1340",
        "--addr",
        "0x1370",
        "--addr",
        "0x1400",
        "--addr",
        "0x1500",
        "--addr",
        "0x1540",
        "--addr",
        "0x1700",
        "--addr",
        "0x1740",
        "--assert-strict",
        "--json",
    ]);
    for declaration in declarations {
        command.args(["--assert", declaration]);
    }
    for function in functions {
        command.args(["--define-function", function]);
    }
    let output = process::required_output(&mut command);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(document["assertions"]
        .as_array()
        .unwrap()
        .iter()
        .all(|a| a["status"] == "applied"));
    let mut source = String::from(
        "typedef struct Pair { int a; int b; } Pair;\n\
        typedef int int4;\n\
        typedef struct Link { struct Link *next; int count; } Link;\nvoid value_sink(Pair); void pointer_sink(Pair *);\n\
        typedef struct PairReverse { int a; int b; } PairReverse;\n\
        void value_sink_reverse(PairReverse); void pointer_sink_reverse(PairReverse *);\n",
    );
    for function in document["functions"].as_array().unwrap() {
        assert!(function["error"].is_null(), "{function}");
        source.push_str(function["code"].as_str().unwrap());
        source.push('\n');
    }
    source.push_str(
        r#"
static int seen_a, seen_b, calls;
void value_sink(Pair value) {
    seen_a = value.a; seen_b = value.b; ++calls;
}
void pointer_sink(Pair *value) {
    seen_a = value->a; seen_b = value->b; ++calls;
    value->a = 999;
}
void value_sink_reverse(PairReverse value) {
    seen_a = value.a; seen_b = value.b; ++calls;
}
void pointer_sink_reverse(PairReverse *value) {
    seen_a = value->a; seen_b = value->b; ++calls;
}
int main(void) {
    stack_value(-17, 12345);
    if (calls != 1 || seen_a != -17 || seen_b != 12345) return 1;
    stack_pointer(31, -19);
    if (calls != 2 || seen_a != 31 || seen_b != -19) return 2;
    Pair input = {81, -43};
    pointer_value(&input);
    if (calls != 3 || seen_a != 81 || seen_b != -43) return 3;
    indirect_value(71, -61, value_sink);
    if (calls != 4 || seen_a != 71 || seen_b != -61) return 4;
    if (value_reader(input) != 38) return 5;
    if (advanced(input) != 38 || spilled(input) != 38 || updated(input) != 39) return 6;
    Link tail = {0, 7}, head = {&tail, 8};
    if (cursor(head) != 15 || stack_cursor(1,2,3,4,5,6,head) != 15
        || collision_cursor(head, 93) != 15) return 7;
    indirect_pointer(9, 17, pointer_sink);
    if (calls != 5 || seen_a != 9 || seen_b != 17) return 8;
    stack_value_reverse(-3, 11);
    if (calls != 6 || seen_a != -3 || seen_b != 11) return 9;
    stack_pointer_reverse(15, -21);
    if (calls != 7 || seen_a != 15 || seen_b != -21) return 10;
    return 0;
}
"#,
    );
    let file = common::scratch_file("indirect-values", "c");
    std::fs::write(&file, source).unwrap();
    let mut found = 0;
    for cc in ["gcc", "clang"] {
        if process::optional_output(Command::new(cc).arg("--version")).is_none() {
            continue;
        }
        found += 1;
        for optimization in ["-O0", "-O2"] {
            let binary = common::scratch_file("indirect-values", "exe");
            let compiled = process::required_output(
                Command::new(cc)
                    .args([
                        optimization,
                        "-Werror=incompatible-pointer-types",
                        "-Werror=implicit-function-declaration",
                    ])
                    .arg(&file)
                    .arg("-o")
                    .arg(&binary),
            );
            assert!(
                compiled.status.success(),
                "{cc} {optimization}: {}",
                String::from_utf8_lossy(&compiled.stderr)
            );
            let result = process::required_output(&mut Command::new(&binary));
            assert!(
                result.status.success(),
                "{cc} {optimization}: {:?}",
                result.status
            );
        }
    }
    assert!(found > 0, "a C compiler is required");
}

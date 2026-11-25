//! (kuna `boolbranch return`) A function that only tests a short-circuit
//! condition returns it.  The gcc -O0 image is printed with `boolbranch keep`
//! and with the default `return`; the functions the default re-spells must
//! print the form each value gives, the neighbours that keep their `if` must
//! print it under both, and the printed C is compiled beside the fixture
//! source (functions renamed `s_<name>`, the helper kept) under gcc and clang
//! at -O0 and -O2: every printed function must return what the source returns
//! and leave `knob` as the source does.
use crate::common;
use common::process;
use std::process::Command;

const SOURCE: &str = include_str!("../../kuna-analysis/tests/fixtures/boolret.c");

const FUNCS: &[&str] = &["is_abs", "is_full", "probe_ok", "last_match", "same", "either", "bump_and_test"];

/// The fixture's `struct node` as the printed C names it.
const PRELUDE: &str = "#include <stdio.h>\n#define BOOLRET_NO_MAIN\n\
typedef struct { int field_0x0; int field_0x4; void *field_0x8; } struct_0;\n";

/// Calls each printed function and its source twin through prototype-less
/// pointers on the same inputs and compares what each returns and leaves
/// behind.
const MAIN: &str = r#"
#define F(f) ((unsigned long (*)())(void (*)())f)
static int bad;
static void check(const char *what, unsigned i, long long got, long long want) {
  if (got != want) { printf("%s [%u]: %llx, want %llx\n", what, i, got, want); bad++; }
}
int main(void) {
  static const char *P[] = {0, "", "/", "/x", "x", "xy", "ab"};
  static const int V[] = {-1, 0, 1, 2, 3, 4, 9, 255, 256, 300};
  for (unsigned i = 0; i < 70; i++) {
    const char *p = P[i % 7];
    int a = V[i % 10], b = V[(i / 7) % 10];
    check("is_abs", i, (int)F(is_abs)(p), s_is_abs(p));
    if (p) check("probe_ok", i, (int)F(probe_ok)(p), s_probe_ok(p));
    struct run r = {{0}, (unsigned)a, b};
    check("is_full", i, (int)F(is_full)(&r), s_is_full(&r));
    struct node tail = {0, 0, 0};
    struct node n = {a, b, (i % 3 == 0) ? &tail : 0};
    check("last_match", i, (int)F(last_match)(i % 5 == 0 ? 0 : &n), s_last_match(i % 5 == 0 ? 0 : &n));
    check("same", i, (int)F(same)(a, b), s_same(a, b));
    check("either", i, (int)F(either)(a, b), s_either(a, b));
    int v = b;
    knob = 0;
    long r1 = (int)F(bump_and_test)(i % 4 == 0 ? 0 : &v, a);
    int k1 = knob;
    knob = 0;
    long r2 = s_bump_and_test(i % 4 == 0 ? 0 : &v, a);
    check("bump_and_test", i, r1, r2);
    check("bump_and_test knob", i, k1, knob);
  }
  return bad != 0;
}
"#;

/// Each re-spelled function and the text printed with `keep` and with `return`.
const CHANGED: &[(&str, &str, &str)] = &[
    ("is_abs", "  if ((a0 != NULL) && (*a0 == '/'))\n    return 1;\n  return 0;", "  return ((a0 != NULL) && (*a0 == '/'));"),
    (
        "is_full",
        "    return 0;\n  return 1;",
        "  return !((((unsigned int *)a0)[0x17] <= 0xff) && (1 <= ((int *)a0)[0x18]));",
    ),
    ("probe_ok", "  if ((!probe(a0)) && (((char *)a0)[1]))", "  return ((!probe(a0)) && (((char *)a0)[1]));"),
    ("last_match", "    return 1;\n  return 0;", "  return ((a0 != NULL) && ("),
];

/// The neighbours, which print the same `if` under both values.
const KEPT: &[(&str, &str)] = &[
    ("same", "  if (a0 == a1)\n    return 1;\n  return 0;"),
    ("either", "  if (a1 == 9)\n    return 1;\n  return 0;"),
    ("bump_and_test", "  if ((a0 != NULL) && (a1 == *a0))\n    return 1;\n  return 0;"),
];

/// The printed function `name`, from its header comment to the next.
fn function<'a>(text: &'a str, name: &str) -> &'a str {
    let start = text.find(&format!("// Function: {name} @")).unwrap_or_else(|| panic!("no {name}:\n{text}"));
    let rest = &text[start + 1..];
    &text[start..start + 1 + rest.find("// Function:").unwrap_or(rest.len())]
}

/// The source with every fixture function renamed `s_<name>`.
fn reference() -> String {
    let mut src = SOURCE.to_string();
    for name in FUNCS {
        src = src.replace(&format!(" {name}("), &format!(" s_{name}("));
    }
    src
}

#[test]
fn a_returned_short_circuit_test_round_trips_through_the_printed_c() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let reference = reference();
    let funcs = FUNCS.join(",");
    let path = common::fixture("boolret_gcc_O0_x86_64");
    for value in ["keep", "return"] {
        let args = ["decompile-all", path.as_str(), "--functions", funcs.as_str(), "--option", "boolbranch", value];
        let (text, err, rc) = common::run_kuna(&args);
        assert_eq!(rc, 0, "boolbranch {value}: {err}");
        for &(name, keep, ret) in CHANGED {
            let body = function(&text, name);
            let (want, not) = if value == "return" { (ret, keep) } else { (keep, ret) };
            assert!(body.contains(want) && !body.contains(not), "{name} boolbranch {value}:\n{body}");
        }
        for &(name, kept) in KEPT {
            let body = function(&text, name);
            assert!(body.contains(kept), "{name} boolbranch {value}:\n{body}");
        }
        let program = format!("{PRELUDE}{reference}\n{text}\n{MAIN}");
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let src = common::scratch_file(&format!("boolret-{value}-{cc}{level}"), "c");
                let exe = src.with_extension("exe");
                std::fs::write(&src, &program).unwrap();
                let out = Command::new(cc)
                    .args(["-std=gnu11", "-w", "-fwrapv", level, "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                    .args(common::CC_GCC15_DEMOTE)
                    .output()
                    .expect("spawn the C compiler");
                assert!(
                    out.status.success(),
                    "{cc} {level} rejected the printed C (boolbranch {value}):\n{}\n{text}",
                    String::from_utf8_lossy(&out.stderr)
                );
                let run = Command::new(&exe).output().expect("run the round trip");
                assert!(
                    run.status.success(),
                    "the image printed with boolbranch {value} and built by {cc} {level} computes a different value:\n{}\n{text}",
                    String::from_utf8_lossy(&run.stdout)
                );
            }
        }
    }
}

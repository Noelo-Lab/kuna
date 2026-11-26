//! (kuna `returndup auto`) Adjacent guards folded into one `||` guard and a
//! merged return split per arm, in an optimized function.  The image is
//! printed with `returndup record` and with the default `auto`, the functions
//! the default re-spells must print the form each value gives, and the printed
//! C is compiled beside the fixture source (functions renamed `s_<name>`, the
//! helpers kept) under gcc and clang at -O0 and -O2: every printed function
//! must return what the source returns and leave `knob` and its buffers as the
//! source does.
use crate::common;
use common::process;
use std::process::Command;

const SOURCE: &str = include_str!("../../kuna-analysis/tests/fixtures/retshape.c");

const FUNCS: &[&str] = &["mutable_ptr", "settle", "describe", "wrap_add"];

/// The fixture's `struct buf` as the printed C names it.
const PRELUDE: &str = "#include <stdio.h>\n#define RETSHAPE_NO_MAIN\n\
typedef struct { long field_0x0; long field_0x8; int field_0x10; unsigned int field_0x14; } struct_0;\n";

/// Calls each printed function and its source twin through prototype-less
/// pointers from the same global state and compares what each returns and
/// leaves behind.
const MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
static int bad;
static void check(const char *what, unsigned i, long long got, long long want) {
  if (got != want) { printf("%s [%u]: %llx, want %llx\n", what, i, got, want); bad++; }
}
int main(void) {
  static const long V[] = {-2, -1, 0, 1, 2, 3};
  for (unsigned i = 0; i < 36; i++) {
    long a = V[i % 6], c = V[i / 6];
    struct buf b = { a == 0 ? 0 : 0x1000 + a, c, (int)(i % 3 == 0), (unsigned)(c < 0 ? 0 : c) };
    check("mutable_ptr", i, F(long, mutable_ptr)(&b), s_mutable_ptr(&b));
    for (int k = 0; k < 2; k++) {
      char o1[4] = {0}, o2[4] = {0};
      knob = k * (int)(i % 2);
      F(void, settle)((int)a + 3, o1);
      int k1 = knob;
      knob = k * (int)(i % 2);
      s_settle((int)a + 3, o2);
      check("settle knob", i, k1, knob);
      check("settle out", i, *(int *)o1, *(int *)o2);
      char e1[32] = {0}, e2[32] = {0};
      knob = k;
      long r1 = F(int, describe)((int)a, "fixture", e1, (size_t)(16 + c));
      long r2 = s_describe((int)a, "fixture", e2, (size_t)(16 + c));
      check("describe", i, (int)r1, (int)r2);
      check("describe err", i, e1[0] + e1[8], e2[0] + e2[8]);
      check("wrap_add", i, F(unsigned long, wrap_add)((unsigned long)a, (unsigned long)c), s_wrap_add((unsigned long)a, (unsigned long)c));
    }
  }
  return bad != 0;
}
"#;

/// Each re-spelled function and the text printed with `record` and with `auto`.
const CHANGED: &[(&str, &str, &str)] = &[
    (
        "mutable_ptr",
        "  if (sane(a0))\n    return 0;\n  if (a0->field_0x10)\n    return 0;",
        "  if (((sane(a0)) || (a0->field_0x10)) || (2 <= a0->field_0x14))\n    return 0;",
    ),
    ("describe", "  return v1;", "    return measure(a1,v1,a3 & 0xffffffff);"),
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
fn a_re_spelled_return_shape_round_trips_through_the_printed_c() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let reference = reference();
    let funcs = FUNCS.join(",");
    let path = common::fixture("retshape_gcc_O2_x86_64");
    for opt in ["record", "auto"] {
        let args = ["decompile-all", path.as_str(), "--functions", funcs.as_str(), "--option", "returndup", opt];
        let (text, err, rc) = common::run_kuna(&args);
        assert_eq!(rc, 0, "returndup {opt}: {err}");
        for &(name, record, auto) in CHANGED {
            let body = function(&text, name);
            let (want, not) = if opt == "auto" { (auto, record) } else { (record, auto) };
            assert!(body.contains(want) && !body.contains(not), "{name} returndup {opt}:\n{body}");
        }
        let program = format!("{PRELUDE}{reference}\n{text}\n{MAIN}");
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let src = common::scratch_file(&format!("retshape-{opt}-{cc}{level}"), "c");
                let exe = src.with_extension("exe");
                std::fs::write(&src, &program).unwrap();
                let out = Command::new(cc)
                    .args(["-std=gnu11", "-w", "-fwrapv", level, "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                    .args(common::CC_GCC15_DEMOTE)
                    .output()
                    .expect("spawn the C compiler");
                assert!(
                    out.status.success(),
                    "{cc} {level} rejected the printed C (returndup {opt}):\n{}\n{text}",
                    String::from_utf8_lossy(&out.stderr)
                );
                let run = Command::new(&exe).output().expect("run the round trip");
                assert!(
                    run.status.success(),
                    "the image printed with returndup {opt} and built by {cc} {level} computes a different value:\n{}\n{text}",
                    String::from_utf8_lossy(&run.stdout)
                );
            }
        }
    }
}

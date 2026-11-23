//! (kuna `tailform`) Function-tail `if`s re-spelled by the size of their
//! conditional part.  Each image is printed with the option off and on, the
//! functions it re-spells must print the form each setting gives, and the printed C is
//! compiled beside the fixture source (functions renamed `s_<name>`, the `note`
//! and `record` helpers kept) under gcc and clang at -O0 and -O2: every printed
//! function must return what the source returns and leave `hits`, `ntrail` and
//! `trail` as the source does.
use crate::common;
use common::process;
use std::process::Command;

const SOURCE: &str = include_str!("../../kuna-analysis/tests/fixtures/tailform.c");

const FUNCS: &[&str] = &[
    "note_if_zero",
    "record_pair",
    "record_above",
    "record_positive",
    "keep_first",
    "pass_through",
    "replay",
    "walk_list",
    "walk_after",
    "first_value",
];

const PRELUDE: &str = "#include <stdio.h>\n#define TAILFORM_NO_MAIN\n";

/// Calls each printed function and its source twin through prototype-less
/// pointers from the same global state and compares the result and the state
/// each leaves.
const MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
static int bad;
static void check(const char *what, unsigned i, long long got, long long want) {
  if (got != want) { printf("%s [%u]: %llx, want %llx\n", what, i, got, want); bad++; }
}
#define RUN(call, twin, use_ret) do { int h0 = hits, n0 = ntrail; \
    int r1 = (int)(call); int h1 = hits, n1 = ntrail, t1 = trail[(ntrail - 1) & 63]; \
    hits = h0; ntrail = n0; \
    int r2 = (int)(twin); int h2 = hits, n2 = ntrail, t2 = trail[(ntrail - 1) & 63]; \
    if (use_ret) check(#call, i, r1, r2); \
    check(#call " hits", i, h1, h2); check(#call " ntrail", i, n1, n2); check(#call " trail", i, t1, t2); } while (0)
int main(void) {
  static const int V[] = {-2, -1, 0, 1, 2, 5};
  for (unsigned i = 0; i < 36; i++) {
    int a = V[i % 6], b = V[i / 6];
    struct item it = {a, 0}, *p = (i % 7 == 3) ? 0 : &it;
    RUN(F(int, note_if_zero)(a), (s_note_if_zero(a), 0), 0);
    RUN(F(int, record_pair)(a, b), s_record_pair(a, b), 1);
    RUN(F(int, record_above)(p, b), s_record_above(p, b), 1);
    RUN(F(int, record_positive)(p), (s_record_positive(p), 0), 0);
    RUN(F(int, keep_first)(a, b), s_keep_first(a, b), 1);
    RUN(F(int, pass_through)(a, b), s_pass_through(a, b), 1);
    RUN(F(int, replay)(p), (s_replay(p), 0), 0);
    RUN(F(int, first_value)(p), s_first_value(p), 1);
    struct item chain[3] = {{a, &chain[1]}, {b, &chain[2]}, {a + b, 0}}, *q = (i % 5 == 2) ? 0 : chain;
    RUN(F(int, walk_list)(q), (s_walk_list(q), 0), 0);
    RUN(F(int, walk_after)(q, b), (s_walk_after(q, b), 0), 0);
  }
  return bad != 0;
}
"#;

/// The replay body behind a guard clause, and wrapped.
const REPLAY: (&str, &str, &str) = ("replay", "  if (a0) {\n    record(*a0);", "  if (!a0)\n    return;\n  record(*a0);");

/// Each image, and for the functions the option re-spells there the text printed
/// with it off and with it on.
const IMAGES: &[(&str, &[(&str, &str, &str)])] = &[
    (
        "tailform_gcc_O2_x86_64",
        &[
            ("note_if_zero", "  if (a0)\n    return;\n  note();", "  if (!a0)\n    note();"),
            ("record_pair", "    return 0;\n  record(a0);", "  if ((int)a0) {\n    record(a0);\n    record(a1);\n  }\n  return 0;"),
            ("record_above", "    record(*a0);\n    return 0;\n  }\n  return 0;", "    record(*a0);\n  return 0;"),
            REPLAY,
        ],
    ),
    ("tailform_gcc_O0_x86_64", &[REPLAY]),
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
fn a_re_spelled_function_tail_round_trips_through_the_printed_c() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let reference = reference();
    let funcs = FUNCS.join(",");
    for &(image, changed) in IMAGES {
        let path = common::fixture(image);
        for opt in ["off", "on"] {
            let args = ["decompile-all", path.as_str(), "--functions", funcs.as_str(), "--option", "tailform", opt];
            let (text, err, rc) = common::run_kuna(&args);
            assert_eq!(rc, 0, "{image} tailform {opt}: {err}");
            for &(name, off, on) in changed {
                let body = function(&text, name);
                let (want, not) = if opt == "on" { (on, off) } else { (off, on) };
                assert!(body.contains(want) && !body.contains(not), "{image} {name} tailform {opt}:\n{body}");
            }
            let program = format!("{PRELUDE}{reference}\n{text}\n{MAIN}");
            for cc in &compilers {
                for level in ["-O0", "-O2"] {
                    let src = common::scratch_file(&format!("tailform-{image}-{opt}-{cc}{level}"), "c");
                    let exe = src.with_extension("exe");
                    std::fs::write(&src, &program).unwrap();
                    let out = Command::new(cc)
                        .args(["-std=gnu11", "-w", "-fwrapv", level, "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                        .args(common::CC_GCC15_DEMOTE)
                        .output()
                        .expect("spawn the C compiler");
                    assert!(
                        out.status.success(),
                        "{cc} {level} rejected the printed C ({image}, tailform {opt}):\n{}\n{text}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    let run = Command::new(&exe).output().expect("run the round trip");
                    assert!(
                        run.status.success(),
                        "{image} printed with tailform {opt} and built by {cc} {level} computes a different value:\n{}\n{text}",
                        String::from_utf8_lossy(&run.stdout)
                    );
                }
            }
        }
    }
}

//! (kuna `tailform`) The no-return form: `if (c) { X } T..; f();`, where `f`
//! does not return and `X` leaves, prints as `if (!c) { T..; f(); } X`.  Each
//! image is printed with the option off and on, the re-spelled functions must
//! print the form each setting gives, and the printed C is compiled beside the
//! fixture source (functions renamed `s_<name>`, `probe` kept) under gcc and
//! clang at -O0 and -O2, with `abort` and `exit` turned into a `longjmp` that
//! records which one ran: every printed function must return, store and stop
//! where the source does. `guardarm` is off: its `auto` default already prints
//! the -O0 `lookup` guard flat with `tailform` off. `loopcondhoist` is off: its
//! default keeps `lookup`'s fatal test inside the loop, away from the tail.
use crate::common;
use common::process;
use std::process::Command;

const SOURCE: &str = include_str!("../../kuna-analysis/tests/fixtures/tailnoret.c");

const FUNCS: &[&str] = &["checked", "scaled", "pick", "lookup", "store"];

const PRELUDE: &str = r#"#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <setjmp.h>
#define TAILNORET_NO_MAIN
static jmp_buf jb;
static volatile int stopped;
static int slot;
#define abort() (stopped = 1, longjmp(jb, 1))
#define exit(c) (stopped = 2 + (c), longjmp(jb, 1))
"#;

/// Calls each printed function and its source twin through prototype-less
/// pointers and compares the result, the stop and the state each leaves.
const MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
static int bad;
static void check(const char *what, unsigned i, long long got, long long want) {
  if (got != want) { printf("%s [%u]: %llx, want %llx\n", what, i, got, want); bad++; }
}
#define RUN(call, twin) do { volatile long long r1 = 0, r2 = 0; int k1, k2, s1, w1, s0 = sink, w0 = slot; \
    stopped = 0; if (!setjmp(jb)) r1 = (long long)(call); k1 = stopped; s1 = sink; w1 = slot; \
    sink = s0; slot = w0; \
    stopped = 0; if (!setjmp(jb)) r2 = (long long)(twin); k2 = stopped; \
    check(#call, i, r1, r2); check(#call " stop", i, k1, k2); \
    check(#call " sink", i, s1, sink); check(#call " slot", i, w1, slot); } while (0)
int main(void) {
  static const int V[] = {-2, 0, 1, 3, 4, 7};
  for (unsigned i = 0; i < 36; i++) {
    int a = V[i % 6], b = V[i / 6];
    int v[4] = {a, b, a + b, a - b}, *pv = (i % 7 == 3) ? 0 : v;
    struct node n2 = {0, 2, b}, n1 = {&n2, 1, a}, *list = (i % 5 == 4) ? 0 : &n1;
    int *ps = (i % 4 == 1) ? 0 : &slot;
    RUN(F(int, checked)(a), s_checked(a));
    RUN(F(long, scaled)((long)a), s_scaled((long)a));
    RUN(F(int, pick)(pv, (unsigned)b, 4u), s_pick(pv, (unsigned)b, 4u));
    RUN(F(int, lookup)(list, a), s_lookup(list, a));
    RUN(F(int, lookup)(list, b), s_lookup(list, b));
    RUN((F(int, store)(ps, a), 0), (s_store(ps, a), 0));
  }
  return bad != 0;
}
"#;

/// Each image, and for the functions the option re-spells there the text printed
/// with it off and with it on.
const IMAGES: &[(&str, &[(&str, &str, &str)])] = &[
    (
        "tailnoret_gcc_O2_x86_64",
        &[
            ("checked", "  if (0 <= v1)\n    return v1;\n  abort();", "  if (0 > v1)\n    abort(); // no-return\n  return v1;"),
            ("scaled", "  if (v2)\n    return v2 + 7;", "  if (!v2) {\n    __fprintf_chk("),
            ("store", "  if (a0) {\n    *a0 = a1;\n    return;\n  }", "  if (!a0)\n    abort(); // no-return\n  *a0 = a1;"),
        ],
    ),
    (
        "tailnoret_gcc_O0_x86_64",
        &[(
            "lookup",
            "    return ((unsigned int *)v1)[3];\n  abort();",
            "    abort(); // no-return\n  return ((unsigned int *)v1)[3];",
        )],
    ),
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
fn a_no_return_tail_round_trips_through_the_printed_c() {
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
            let args = [
                "decompile-all", path.as_str(), "--functions", funcs.as_str(), "--option", "tailform", opt, "--option",
                "guardarm", "off", "--option", "loopcondhoist", "off",
            ];
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
                    let src = common::scratch_file(&format!("tailnoret-{image}-{opt}-{cc}{level}"), "c");
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
                        "{image} printed with tailform {opt} and built by {cc} {level} behaves differently:\n{}\n{text}",
                        String::from_utf8_lossy(&run.stdout)
                    );
                }
            }
        }
    }
}

//! (kuna `condfold`) A chain of tests that all branch to one call that does not
//! return prints as one `if` over the `||` chain with the call as its body and
//! the rest after it; under the default `auto` an optimized function also folds a
//! chain into any block only it enters.  Each image is printed with the setting
//! that leaves the chains alone and with the default, the chains must print the
//! form each setting gives, and the printed C is compiled beside the fixture
//! source (functions renamed `s_<name>`, the helpers kept) under gcc and clang at
//! -O0 and -O2.  `put` fails at each of its calls in turn and `exit` (inside
//! `fatal`) is a `longjmp`; every printed function must return, store, stop and
//! pass on the value the source does, and make the same `put` calls in the same
//! order.
use crate::common;
use common::process;
use std::process::Command;

const SOURCE: &str = include_str!("../../kuna-analysis/tests/fixtures/fatalchain.c");

const FUNCS: &[&str] = &["reply", "compose", "twice", "wide"];

const PRELUDE: &str = r#"#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <setjmp.h>
#define FATALCHAIN_NO_MAIN
static jmp_buf jb;
static volatile int stopped;
static int last_r, buf;
static const char *last_what;
#define fprintf(f, fmt, w, r) (last_what = (w), last_r = (r))
#define exit(c) (stopped = 2 + (c), longjmp(jb, 1))
"#;

/// Calls each printed function and its source twin through prototype-less
/// pointers, with `put` failing at call `i / 2` (never for 0).
const MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
static int bad;
static void check(const char *what, int i, long long got, long long want) {
  if (got != want) { printf("%s [%d]: %llx, want %llx\n", what, i, got, want); bad++; }
}
static int differ(const char *a, const char *b) { return (a == 0 || b == 0) ? a != b : strcmp(a, b) != 0; }
#define RESET() (buf = b0, sink = 0, steps = 0, order = 0, last_r = 0, last_what = 0, stopped = 0)
#define RUN(call, twin) do { volatile long long r1 = 0, r2 = 0; int k1, b1, s1, l1; unsigned o1; const char *w1; \
    RESET(); if (!setjmp(jb)) r1 = (long long)(call); \
    k1 = stopped; b1 = buf; s1 = sink; o1 = order; l1 = last_r; w1 = last_what; \
    RESET(); if (!setjmp(jb)) r2 = (long long)(twin); \
    check(#call, i, r1, r2); check(#call " stop", i, k1, stopped); check(#call " order", i, o1, order); \
    check(#call " fatal value", i, l1, last_r); check(#call " fatal text", i, differ(w1, last_what), 0); \
    check(#call " buf", i, b1, buf); check(#call " sink", i, s1, sink); } while (0)
int main(void) {
  for (int i = 0; i < 20; i++) {
    int b0 = (i & 1) ? 90 : 0;
    fail_at = i / 2;
    RUN((F(void, reply)(&buf, 1, 2, 3), 0), (s_reply(&buf, 1, 2, 3), 0));
    RUN(F(int, compose)(&buf, 5, 6), s_compose(&buf, 5, 6));
    RUN(F(int, twice)(&buf, 4), s_twice(&buf, 4));
    RUN(F(int, twice)((int *)0, 4), s_twice((int *)0, 4));
    RUN((F(void, wide)(&buf, 3), 0), (s_wide(&buf, 3), 0));
  }
  return bad != 0;
}
"#;

/// Each image, and for the functions the option re-spells there the text printed
/// with `condfold off` and with the default.
const IMAGES: &[(&str, &[(&str, &str, &str)])] = &[
    (
        "fatalchain_gcc_O2_x86_64",
        &[
            ("reply", "  if (!v1) {\n    v1 = put(a0,", "  if (((v1) || (v1 = put(a0,"),
            ("twice", "      fatal(v1,\"second\"); // no-return\n", "    fatal(v1,\"second\"); // no-return\n  return *a0;"),
        ],
    ),
    (
        "fatalchain_gcc_O0_x86_64",
        &[
            ("reply", "  if (!v1) {\n    v1 = put(a0,", "  if (((v1) || (v1 = put(a0,"),
            ("compose", "        sink = *a0;", "    fatal(v1,\"compose\"); // no-return\n  sink = *a0;"),
        ],
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
fn a_chain_into_a_no_return_call_round_trips_through_the_printed_c() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let reference = reference();
    let funcs = FUNCS.join(",");
    for &(image, changed) in IMAGES {
        let path = common::fixture(image);
        for opt in ["off", "default"] {
            let mut args = vec!["decompile-all", path.as_str(), "--functions", funcs.as_str()];
            if opt == "off" {
                args.extend(["--option", "condfold", "off"]);
            }
            let (text, err, rc) = common::run_kuna(&args);
            assert_eq!(rc, 0, "{image} condfold {opt}: {err}");
            for &(name, off, on) in changed {
                let body = function(&text, name);
                let (want, not) = if opt == "off" { (off, on) } else { (on, off) };
                assert!(body.contains(want) && !body.contains(not), "{image} {name} condfold {opt}:\n{body}");
            }
            let program = format!("{PRELUDE}{reference}\n{text}\n{MAIN}");
            for cc in &compilers {
                for level in ["-O0", "-O2"] {
                    let src = common::scratch_file(&format!("fatalchain-{image}-{opt}-{cc}{level}"), "c");
                    let exe = src.with_extension("exe");
                    std::fs::write(&src, &program).unwrap();
                    let out = Command::new(cc)
                        .args(["-std=gnu11", "-w", "-fwrapv", level, "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                        .args(common::CC_GCC15_DEMOTE)
                        .output()
                        .expect("spawn the C compiler");
                    assert!(
                        out.status.success(),
                        "{cc} {level} rejected the printed C ({image}, condfold {opt}):\n{}\n{text}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    let run = Command::new(&exe).output().expect("run the round trip");
                    assert!(
                        run.status.success(),
                        "{image} printed with condfold {opt} and built by {cc} {level} behaves differently:\n{}\n{text}",
                        String::from_utf8_lossy(&run.stdout)
                    );
                }
            }
        }
    }
}

const CHAIN_SOURCE: &str = include_str!("../../kuna-analysis/tests/fixtures/chainfold.c");

const CHAIN_FUNCS: &[&str] = &["send", "store", "commit"];

const CHAIN_PRELUDE: &str = r#"#include <stdio.h>
#include <stdlib.h>
#define CHAINFOLD_NO_MAIN
static int buf;
"#;

const CHAIN_MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
static int bad;
static void check(const char *what, int i, long long got, long long want) {
  if (got != want) { printf("%s [%d]: %llx, want %llx\n", what, i, got, want); bad++; }
}
#define RESET() (buf = b0, sink = 0, steps = 0, order = 0, errors = 0)
#define RUN(call, twin) do { long long r1, r2; int b1, s1, e1; unsigned o1; \
    RESET(); r1 = (long long)(call); b1 = buf; s1 = sink; o1 = order; e1 = errors; \
    RESET(); r2 = (long long)(twin); \
    check(#call, i, r1, r2); check(#call " order", i, o1, order); check(#call " errors", i, e1, errors); \
    check(#call " buf", i, b1, buf); check(#call " sink", i, s1, sink); } while (0)
int main(void) {
  for (int i = 0; i < 12; i++) {
    int b0 = (i & 1) ? 90 : 0;
    fail_at = i / 2;
    RUN(F(int, send)(&buf, 3, 4), s_send(&buf, 3, 4));
    RUN((F(void, store)(&buf, 5), 0), (s_store(&buf, 5), 0));
    RUN((F(void, commit)(&buf, 7), 0), (s_commit(&buf, 7), 0));
  }
  return bad != 0;
}
"#;

/// Each image, and for the functions the default re-spells there the text printed
/// with `condfold noreturn` and with the default.
const CHAIN_IMAGES: &[(&str, &[(&str, &str, &str)])] = &[
    (
        "chainfold_gcc_O2_x86_64",
        &[
            ("store", "  if (!v1) {\n    v1 = put(a0,(int)a1 * 2);", "  if ((!v1) && (v1 = put(a0,(int)a1 * 2), !v1)) {"),
            ("commit", "  if (!v1) {\n    order = 1;", "  if ((!v1) && (order = 1, v1 = put(a0,(int)a1 + 3), !v1)) {"),
        ],
    ),
    ("chainfold_gcc_O0_x86_64", &[("send", "      v1 = put(a0,a1 + a2);", "  if ((")]),
];

#[test]
fn a_chain_into_an_ordinary_block_round_trips_through_the_printed_c() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let mut reference = CHAIN_SOURCE.to_string();
    for name in CHAIN_FUNCS {
        reference = reference.replace(&format!(" {name}("), &format!(" s_{name}("));
    }
    let funcs = CHAIN_FUNCS.join(",");
    for &(image, changed) in CHAIN_IMAGES {
        let path = common::fixture(image);
        for opt in ["noreturn", "default"] {
            let mut args = vec!["decompile-all", path.as_str(), "--functions", funcs.as_str()];
            if opt == "noreturn" {
                args.extend(["--option", "condfold", "noreturn"]);
            }
            let (text, err, rc) = common::run_kuna(&args);
            assert_eq!(rc, 0, "{image} condfold {opt}: {err}");
            for &(name, old, new) in changed {
                let body = function(&text, name);
                let unoptimized = image.contains("_O0_");
                let (want, not) = if opt == "noreturn" || unoptimized { (old, new) } else { (new, old) };
                assert!(body.contains(want) && !body.contains(not), "{image} {name} condfold {opt}:\n{body}");
            }
            let program = format!("{CHAIN_PRELUDE}{reference}\n{text}\n{CHAIN_MAIN}");
            for cc in &compilers {
                for level in ["-O0", "-O2"] {
                    let src = common::scratch_file(&format!("chainfold-{image}-{opt}-{cc}{level}"), "c");
                    let exe = src.with_extension("exe");
                    std::fs::write(&src, &program).unwrap();
                    let out = Command::new(cc)
                        .args(["-std=gnu11", "-w", "-fwrapv", level, "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                        .args(common::CC_GCC15_DEMOTE)
                        .output()
                        .expect("spawn the C compiler");
                    assert!(
                        out.status.success(),
                        "{cc} {level} rejected the printed C ({image}, condfold {opt}):\n{}\n{text}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    let run = Command::new(&exe).output().expect("run the round trip");
                    assert!(
                        run.status.success(),
                        "{image} printed with condfold {opt} and built by {cc} {level} behaves differently:\n{}\n{text}",
                        String::from_utf8_lossy(&run.stdout)
                    );
                }
            }
        }
    }
}

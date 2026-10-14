//! (kuna `castnarrow`) A narrowing whose value lands whole in an assignment, a
//! store, a prototyped argument or a `return` declared with exactly its type
//! prints no cast, because C performs that conversion there itself; one read by
//! arithmetic, by a comparison or by a destination of another type keeps it.
//! The fixture is printed with the option off and on, each printed function is
//! compiled beside the fixture source (renamed `s_<name>`), and both must
//! compute the same values, under gcc and clang at -O0 and -O2.
use crate::common;
use common::process;
use std::process::Command;

const SOURCE: &str = include_str!("../../kuna-analysis/tests/fixtures/castnarrow_x86_64.c");

const FUNCS: &[&str] = &[
    "trunc_local",
    "cond_narrow",
    "pair_low",
    "scanned_low",
    "libc_args",
    "narrow_short",
    "sign_change",
    "put_bytes",
    "ret_narrow",
    "ret_short",
    "keep_store",
    "keep_arith",
    "keep_cmp",
];

const PROTOTYPES: &[&str] = &[
    "prototype ret_narrow int ret_narrow(long x,long *out)",
    "prototype ret_short short ret_short(long x,long *out)",
];

const PRELUDE: &str = "#include <ctype.h>\n#include <stdbool.h>\n#include <stdio.h>\n#include <stdlib.h>\n\
                       #include <string.h>\nint __isoc99_sscanf(const char *, const char *, ...);\n\
                       #define CASTNARROW_NO_MAIN\n";

/// Calls the printed functions through prototype-less pointers, since their
/// recovered signatures need not match the source's, and compares each result
/// and out-parameter with the source function's at the narrowest width a
/// printed signature returns it at (`keep_cmp` may print as `bool`).
const MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
static const char *const HEX[] = {"fedcba9876543261", "0", "1", "3e9", "ffffffff", "100000000",
                                  "7fffffff80000005", "8000000000000013", "abc", "123456789abc"};
static const unsigned long VAL[] = {0, 1, 0xff, 0x7fffffff, 0x80000000UL, 0xffffffffUL, 0x100000001UL,
                                    0xfedcba9876543261UL, 0x8000000000000010UL, 0xffffffff00000061UL, 0x12345fedcbUL};
#define CHECK(got, want) do { long long g_ = (long long)(got), w_ = (long long)(want); if (g_ != w_) { \
    printf("%s [%u]: %llx, want %llx\n", #got, i, g_, w_); bad++; } } while (0)
int main(void) {
  int bad = 0;
  for (unsigned i = 0; i < sizeof HEX / sizeof HEX[0]; i++) {
    unsigned long o1 = 0, o2 = 0;
    char line[64];
    CHECK(F(unsigned long, cond_narrow)(HEX[i], &o1), s_cond_narrow(HEX[i], &o2)); CHECK(o1, o2);
    snprintf(line, sizeof line, "%s 3", HEX[i]);
    o1 = o2 = 0;
    CHECK(F(unsigned int, pair_low)(line, &o1), (unsigned int)s_pair_low(line, &o2)); CHECK(o1, o2);
    o1 = o2 = 0;
    CHECK(F(unsigned int, scanned_low)(HEX[i], &o1), (unsigned int)s_scanned_low(HEX[i], &o2)); CHECK(o1, o2);
  }
  for (unsigned i = 0; i < sizeof VAL / sizeof VAL[0]; i++) {
    unsigned long x = VAL[i], o1 = 0, o2 = 0;
    long l1 = 0, l2 = 0;
    unsigned short h1[3] = {0}, h2[3] = {0};
    unsigned int u1 = 0, u2 = 0;
    char b1[5] = {0}, b2[5] = {0};
    CHECK(F(unsigned long, trunc_local)(x, &o1), s_trunc_local(x, &o2)); CHECK(o1, o2);
    CHECK(F(int, libc_args)(x, &o1), s_libc_args(x, &o2)); CHECK(o1, o2);
    CHECK(F(unsigned long, narrow_short)(x, h1), s_narrow_short(x, h2));
    CHECK(h1[0], h2[0]); CHECK(h1[1], h2[1]); CHECK(h1[2], h2[2]);
    CHECK(F(int, sign_change)((unsigned long)(unsigned int)x, &u1), s_sign_change((unsigned int)x, &u2)); CHECK(u1, u2);
    F(void, put_bytes)(x, b1); s_put_bytes(x, b2);
    CHECK(b1[0], b2[0]); CHECK(b1[2], b2[2]); CHECK(b1[4], b2[4]);
    CHECK(F(int, ret_narrow)((long)x, &l1), s_ret_narrow((long)x, &l2)); CHECK(l1, l2);
    CHECK(F(short, ret_short)((long)x, &l1), s_ret_short((long)x, &l2)); CHECK(l1, l2);
    CHECK(F(unsigned long, keep_store)(x, &u1), s_keep_store(x, &u2)); CHECK(u1, u2);
    CHECK(F(unsigned int, keep_arith)(x, 0x80000000u, &o1), (unsigned int)s_keep_arith(x, 0x80000000u, &o2));
    CHECK(o1, o2);
    CHECK(F(unsigned char, keep_cmp)(x, &o1), (unsigned char)s_keep_cmp(x, &o2)); CHECK(o1, o2);
  }
  return bad != 0;
}
"#;

/// Each image and the lines it prints with the option off and on: a local
/// (`v1 = (unsigned int)a0;`), the low piece of a wider stack variable
/// (`v1 = (int)v3;`), a sign change, a `short` and a byte, a store through
/// `char *`, the arguments of `toupper` and `abs`, and a `return` from a
/// function asserted to return `int` or `short`.  Every image keeps the `(int)`
/// of a store into `unsigned int *` and of an operand of an addition.
const IMAGES: &[(&str, &[(&str, &str)])] = &[
    (
        "castnarrow_gcc_O0_x86_64",
        &[
            ("v2 = (unsigned int)v1;", "v2 = v1;"),
            ("v2 = (int)a0;", "v2 = a0;"),
            ("v2 = (short)a0;", "v2 = a0;"),
            ("v1 = (unsigned char)(a0 >> 3);", "v1 = a0 >> 3;"),
            ("*a1 = (char)a0;", "*a1 = a0;"),
            ("a1[4] = (char)((unsigned long)a0 >> 0x20);", "a1[4] = (unsigned long)a0 >> 0x20;"),
            ("return (int)x;", "return x;"),
            ("return (short)x;", "return x;"),
        ],
    ),
    (
        "castnarrow_gcc_O2_x86_64",
        &[
            ("v1 = (unsigned int)a0;", "v1 = a0;"),
            ("v3 = (int)a0;", "v3 = a0;"),
            ("v2 = (short)((a0 & 0x7f8) >> 3);", "v2 = (a0 & 0x7f8) >> 3;"),
            ("*a1 = (char)a0;", "*a1 = a0;"),
            ("return (int)x;", "return x;"),
        ],
    ),
    (
        "castnarrow_clang_O0_x86_64",
        &[
            ("v4 = (unsigned int)v1;", "v4 = v1;"),
            ("v1 = toupper((int)a0);", "v1 = toupper(a0);"),
            ("return v1 + abs((int)a0);", "return v1 + abs(a0);"),
            ("v3 = (short)a0;", "v3 = a0;"),
            ("a1[2] = (char)((unsigned long)a0 >> 0x10);", "a1[2] = (unsigned long)a0 >> 0x10;"),
            ("return (short)x;", "return x;"),
        ],
    ),
    (
        "castnarrow_clang_O2_x86_64",
        &[
            ("v5 = (unsigned int)a0;", "v5 = a0;"),
            ("v1 = (int)v3;", "v1 = v3;"),
            ("v1 = (int)a0;", "v1 = a0;"),
            ("v1 = (short)a0;", "v1 = a0;"),
            ("return (int)x;", "return x;"),
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
fn a_narrowing_into_a_destination_of_its_type_round_trips_through_the_printed_c() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let reference = reference();
    let funcs = FUNCS.join(",");
    for &(image, lines) in IMAGES {
        let path = common::fixture(image);
        for opt in ["off", "on"] {
            let mut args = vec!["decompile-all", path.as_str(), "--functions", funcs.as_str()];
            for p in PROTOTYPES {
                args.extend_from_slice(&["--assert", p]);
            }
            args.extend_from_slice(&["--option", "castnarrow", opt]);
            let (text, err, rc) = common::run_kuna(&args);
            assert_eq!(rc, 0, "{image} castnarrow {opt}: {err}");
            for &(off, on) in lines {
                let (want, gone) = if opt == "on" { (on, Some(off)) } else { (off, None) };
                assert!(text.contains(want), "{image} castnarrow {opt} does not print `{want}`:\n{text}");
                if let Some(gone) = gone {
                    assert!(!text.contains(gone), "{image} castnarrow on still prints `{gone}`:\n{text}");
                }
            }
            for (name, kept) in [("keep_store", "*a1 = (int)a0;"), ("keep_arith", "(int)a0")] {
                assert!(function(&text, name).contains(kept), "{image} castnarrow {opt} lost `{kept}`:\n{text}");
            }
            let program = format!("{PRELUDE}{reference}\n{text}\n{MAIN}");
            for cc in &compilers {
                for level in ["-O0", "-O2"] {
                    let src = common::scratch_file(&format!("castnarrow-{image}-{opt}-{cc}{level}"), "c");
                    let exe = src.with_extension("exe");
                    std::fs::write(&src, &program).unwrap();
                    let out = Command::new(cc)
                        .args(["-std=gnu11", "-w", "-fwrapv", level, "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                        .args(common::CC_GCC15_DEMOTE)
                        .output()
                        .expect("spawn the C compiler");
                    assert!(
                        out.status.success(),
                        "{cc} {level} rejected the printed C ({image}, castnarrow {opt}):\n{}\n{text}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    let run = Command::new(&exe).output().expect("run the round trip");
                    assert!(
                        run.status.success(),
                        "{image} printed with castnarrow {opt} and built by {cc} {level} computes a different value:\n{}\n{text}",
                        String::from_utf8_lossy(&run.stdout)
                    );
                }
            }
        }
    }
}

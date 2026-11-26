//! (kuna `rangetest`) A merged unsigned range test prints as the two
//! comparisons it stands for. The gcc -O2 image is printed under each value;
//! the splits each value prints, and the tests every value keeps merged (a
//! wrapping range, a signed 16-bit value whose range crosses its sign bit, a
//! switch), must print as expected, a split `||` heading a nested `&&` must
//! keep its own parentheses, and the printed C is compiled beside the
//! fixture source (functions renamed `s_<name>`) under gcc and clang at -O0
//! and -O2: every printed function must return what the source returns and
//! leave `knob` as the source does, at each range's boundaries, 0 and the
//! type's extremes.
use crate::common;
use common::process;
use std::process::Command;

const SOURCE: &str = include_str!("../../kuna-analysis/tests/fixtures/rangetest.c");

const FUNCS: &[&str] =
    &["printable", "bits", "reject", "digit", "lower_after", "isdig", "is_port", "scaled", "small", "number", "pick", "ctl"];

const PRELUDE: &str = "#include <stdbool.h>\n#define RANGETEST_NO_MAIN\n";

/// Calls each printed function and its source twin through prototype-less
/// pointers on the same inputs and compares what each returns, read at the
/// width the printed C returns (a `bool` sets only the low byte), and leaves
/// in `knob`.
const MAIN: &str = r#"
#define F(f) ((long (*)())(void (*)())f)
static int bad;
static void check(const char *what, unsigned long long in, long long got, long long want) {
  if (got != want) { printf("%s(%llx): %llx, want %llx\n", what, in, got, want); bad++; }
}
#define PAIR(name, T, R, x) do { \
    knob = -7; long long g = (R)F(name)((T)(x)); int gk = knob; \
    knob = -7; long long w = (R)s_##name((T)(x)); \
    check(#name, (unsigned long long)(x), g, w); check(#name " knob", (unsigned long long)(x), gk, knob); \
  } while (0)
#define VOID(name, T, x) do { \
    knob = -7; F(name)((T)(x)); int gk = knob; \
    knob = -7; s_##name((T)(x)); \
    check(#name " knob", (unsigned long long)(x), gk, knob); \
  } while (0)
int main(void) {
  static const unsigned long long V[] = {
    0, 1, 2, 3, 5, 8, 9, 0xa, 0xb, 0xf, 0x10, 0x1e, 0x1f, 0x20, 0x21, 0x2f, 0x30, 0x39, 0x3a, 0x60, 0x61, 0x7a, 0x7b,
    0x7e, 0x7f, 0x80, 0xff, 0x100, 0x3ff, 0x400, 0xbfff, 0xc000, 0x7fff, 0x8000, 0xffff, 0x10000,
    0x7fffffff, 0x80000000, 0xffffffe0, 0xffffffef, 0xfffffff0, 0xffffffff, 0x100000000ULL,
    0x1fffffffffffffffULL, 0x2000000000000000ULL, 0x7fffffffffffffffULL, 0x8000000000000000ULL,
    0xfffffffffffffff0ULL, 0xffffffffffffffffULL};
  for (unsigned i = 0; i < sizeof V / sizeof V[0]; i++) {
    unsigned long long v = V[i];
    VOID(printable, unsigned char, v);
    PAIR(bits, int, int, v);
    VOID(reject, int, v);
    VOID(digit, unsigned char, v);
    PAIR(isdig, int, unsigned char, v);
    PAIR(is_port, unsigned short, unsigned char, v);
    PAIR(scaled, unsigned long, long, v);
    PAIR(small, int, unsigned char, v);
    PAIR(pick, int, int, v);
    for (int a = 2; a <= 3; a++)
      check("lower_after", v, (unsigned char)F(lower_after)(a, (char)v), s_lower_after(a, (char)v));
    for (int b = 0; b <= 9; b += 9)
      check("ctl", v, (int)F(ctl)(b, (int)v), s_ctl(b, (int)v));
    char s[4] = {(char)v, (char)(v >> 8), '7', 0};
    check("number", v, (int)F(number)(s), s_number(s));
  }
  return bad != 0;
}
"#;

/// The values that print each line of a function.
const SPELLED: &[(&[&str], &str, &str)] = &[
    (&["merged"], "printable", "  if ((unsigned char)(a0 - 0x20) <= 0x5e)"),
    (&["auto", "split"], "printable", "  if (0x20 <= a0 && a0 <= 0x7e)"),
    (&["merged"], "bits", "  if ((unsigned int)(a0 - 1U) <= 0x1d) {"),
    (&["auto", "split"], "bits", "  if (1 <= a0 && a0 <= 0x1e) {"),
    (&["merged"], "reject", "  if (0x1e <= (unsigned int)(a0 - 1U))"),
    (&["auto", "split"], "reject", "  if (a0 < 1 || 0x1e < a0)"),
    (&["merged", "auto"], "digit", "  if ((unsigned char)(a0 - 0x30) <= 9)"),
    (&["split"], "digit", "  if (0x30 <= a0 && a0 <= 0x39)"),
    (&["merged"], "lower_after", "  return 2 < a0 && (unsigned char)(a1 + 0x9fU) <= 0x19;"),
    (&["auto", "split"], "lower_after", "  return 2 < a0 && (0x61 <= a1 && a1 <= 0x7a);"),
    (&["merged"], "isdig", "  return (unsigned int)(a0 - 0x30U) <= 9;"),
    (&["auto", "split"], "isdig", "  return 0x30 <= a0 && a0 <= 0x39;"),
    (&["merged"], "scaled", "  if ((unsigned long)(a0 - 1U) <= 0x1ffffffffffffffe)"),
    (&["auto", "split"], "scaled", "  if (1 <= a0 && a0 <= 0x1fffffffffffffff)"),
    (&["merged", "auto"], "number", "  while ((unsigned char)(v1 - 0x30U) <= 9) {"),
    (&["split"], "number", "  while (0x30 <= v1 && v1 <= 0x39) {"),
    (&["merged"], "ctl", "    if ((a0 == 7) || ((2 <= (unsigned int)(a1 - 9U) && (a1 != 5))))"),
    (&["auto", "split"], "ctl", "    if ((a0 == 7) || (((a1 < 9 || 10 < a1) && (a1 != 5))))"),
];

/// The tests no value splits.
const KEPT: &[(&str, &str)] = &[
    ("is_port", "  return (unsigned short)(a0 - 0x400U) <= 0xbbff;"),
    ("small", "  return a0 + 0x10U <= 0x1f;"),
    ("pick", "  switch(a0) {"),
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
fn a_split_range_test_round_trips_through_the_printed_c() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let reference = reference();
    let funcs = FUNCS.join(",");
    let path = common::fixture("rangetest_gcc_O2_x86_64");
    for value in ["merged", "auto", "split"] {
        let args = ["decompile-all", path.as_str(), "--functions", funcs.as_str(), "--option", "rangetest", value];
        let (text, err, rc) = common::run_kuna(&args);
        assert_eq!(rc, 0, "rangetest {value}: {err}");
        for &(values, name, line) in SPELLED {
            let body = function(&text, name);
            assert_eq!(body.contains(line), values.contains(&value), "{name} rangetest {value}, {line:?}:\n{body}");
        }
        for &(name, kept) in KEPT {
            let body = function(&text, name);
            assert!(body.contains(kept), "{name} rangetest {value}:\n{body}");
        }
        let program = format!("{PRELUDE}{reference}\n{text}\n{MAIN}");
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let src = common::scratch_file(&format!("rangetest-{value}-{cc}{level}"), "c");
                let exe = src.with_extension("exe");
                std::fs::write(&src, &program).unwrap();
                let out = Command::new(cc)
                    .args(["-std=gnu11", "-w", "-fwrapv", level, "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                    .args(common::CC_GCC15_DEMOTE)
                    .output()
                    .expect("spawn the C compiler");
                assert!(
                    out.status.success(),
                    "{cc} {level} rejected the printed C (rangetest {value}):\n{}\n{text}",
                    String::from_utf8_lossy(&out.stderr)
                );
                let run = Command::new(&exe).output().expect("run the round trip");
                assert!(
                    run.status.success(),
                    "the image printed with rangetest {value} and built by {cc} {level} computes a different value:\n{}\n{text}",
                    String::from_utf8_lossy(&run.stdout)
                );
            }
        }
    }
}

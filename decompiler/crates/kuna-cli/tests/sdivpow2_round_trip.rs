//! A signed division by a power of two whose correction of a negative dividend
//! the compiler made with a conditional move prints as the division, with no
//! branch.  Each printed function is compiled beside the fixture source
//! (renamed `s_<name>`) and must compute what the source computes at the most
//! negative value, zero, the largest value and around each power, under gcc
//! and clang at -O0 and -O2.
use crate::common;
use common::process;
use std::process::Command;
use crate::common::process::Capped;

const SOURCE: &str = include_str!("../../kuna-analysis/tests/fixtures/sdivpow2_x86_64.c");

const FUNCS: &[&str] = &[
    "div2_i", "div8_i", "div1024_i", "div_top_i", "div2_l", "div8_l", "div1024_l", "div_top_l", "scaled", "round8",
    "split", "word_bit",
];

const LINES: &[&str] = &[
    "return a0 / 8;",
    "return a0 / 0x400;",
    "return a0 / 0x40000000;",
    "return a0 / 0x4000000000000000;",
    "return (a0 + 0x200) / 0x400;",
    "*a1 = a0 % 0x10;",
    "return a0 / 0x10;",
    "a0[a1 / 0x40]",
];

const MAIN: &str = r#"
#include <limits.h>
#include <stdio.h>
#define F(ret, f) ((ret (*)())(void (*)())f)
#define CHECK(got, want) do { long long g_ = (long long)(got), w_ = (long long)(want); if (g_ != w_) { \
    printf("%s [%lld]: %lld, want %lld\n", #got, (long long)x, g_, w_); bad++; } } while (0)
static const long VAL[] = {LONG_MIN, LONG_MIN + 1, -0x4000000000000001L, -0x4000000000000000L, INT_MIN, INT_MIN + 1,
                           -0x40000001L, -0x40000000L, -1025, -1024, -1023, -513, -512, -511, -65, -64, -63, -17, -16,
                           -15, -9, -8, -7, -2, -1, 0, 1, 2, 7, 8, 9, 15, 16, 17, 511, 512, 1023, 1024, 1025,
                           0x3fffffff, 0x40000000, INT_MAX - 512, INT_MAX - 511, INT_MAX, 0x4000000000000000L, LONG_MAX};
int main(void) {
  int bad = 0;
  for (unsigned i = 0; i < sizeof VAL / sizeof VAL[0]; i++) {
    long x = VAL[i];
    int w = (int)x, r1 = 0, r2 = 0;
    CHECK(F(int, div2_i)(w), s_div2_i(w));
    CHECK(F(int, div8_i)(w), s_div8_i(w));
    CHECK(F(int, div1024_i)(w), s_div1024_i(w));
    CHECK(F(int, div_top_i)(w), s_div_top_i(w));
    CHECK(F(long, div2_l)(x), s_div2_l(x));
    CHECK(F(long, div8_l)(x), s_div8_l(x));
    CHECK(F(long, div1024_l)(x), s_div1024_l(x));
    CHECK(F(long, div_top_l)(x), s_div_top_l(x));
    CHECK(F(int, scaled)(w), s_scaled(w));
    CHECK(F(int, round8)(w), s_round8(w));
    CHECK(F(int, split)(w, &r1), s_split(w, &r2)); CHECK(r1, r2);
  }
  static const unsigned long BITS[4] = {0x8000000000000001UL, 0x0123456789abcdefUL, 0, ~0UL};
  for (long x = 0; x < 256; x++)
    CHECK(F(unsigned int, word_bit)(BITS, (int)x), s_word_bit(BITS, (int)x));
  return bad != 0;
}
"#;

const IMAGES: &[&str] = &["sdivpow2_gcc_O0_x86_64", "sdivpow2_gcc_O2_x86_64", "sdivpow2_clang_O2_x86_64"];

/// The source with every fixture function renamed `s_<name>`.
fn reference() -> String {
    let mut src = SOURCE.to_string();
    for name in FUNCS {
        src = src.replace(&format!(" {name}("), &format!(" s_{name}("));
    }
    src
}

#[test]
fn a_conditionally_corrected_power_of_two_division_round_trips_through_the_printed_c() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let reference = reference();
    let funcs = FUNCS.join(",");
    for image in IMAGES {
        let path = common::fixture(image);
        let (text, err, rc) = common::run_kuna(&["decompile-all", path.as_str(), "--functions", funcs.as_str()]);
        assert_eq!(rc, 0, "{image}: {err}");
        for line in LINES {
            assert!(text.contains(line), "{image} does not print `{line}`:\n{text}");
        }
        assert!(!text.contains("if ("), "{image} keeps a branch:\n{text}");
        let program = format!("{reference}\n{text}\n{MAIN}");
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let src = common::scratch_file(&format!("sdivpow2-{image}-{cc}{level}"), "c");
                let exe = src.with_extension("exe");
                std::fs::write(&src, &program).unwrap();
                let out = Command::new(cc)
                    .args(["-std=gnu11", "-w", "-fwrapv", level, "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                    .args(common::CC_GCC15_DEMOTE)
                    .output()
                    .expect("spawn the C compiler");
                assert!(
                    out.status.success(),
                    "{cc} {level} rejected the printed C ({image}):\n{}\n{text}",
                    String::from_utf8_lossy(&out.stderr)
                );
                let run = Command::new(&exe).capped_output().expect("run the round trip");
                assert!(
                    run.status.success(),
                    "{image} printed and built by {cc} {level} computes a different value:\n{}\n{text}",
                    String::from_utf8_lossy(&run.stdout)
                );
            }
        }
    }
}

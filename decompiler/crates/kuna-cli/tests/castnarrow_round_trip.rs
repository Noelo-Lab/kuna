//! Narrowed values round-trip through both castnarrow settings, including
//! arithmetic, comparisons, calls and exact-width stores. Each printed
//! function is compiled beside the renamed fixture source, and both must
//! compute the same values and preserve surrounding bytes under gcc and
//! clang at -O0 and -O2. Cast spelling and recovered local names are not part
//! of the native behavior contract.
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
    struct { unsigned short value[3]; unsigned short guard; } hs1 = {{0}, 0xa55a}, hs2 = {{0}, 0xa55a};
    struct { unsigned int value, guard; } us1 = {0, 0xa55aa55a}, us2 = {0, 0xa55aa55a};
    struct { char before, value[5], after; } bs1 = {0x55, {0x33, 0x33, 0x33, 0x33, 0x33}, 0x66}, bs2 = bs1;
    CHECK(F(unsigned long, trunc_local)(x, &o1), s_trunc_local(x, &o2)); CHECK(o1, o2);
    CHECK(F(int, libc_args)(x, &o1), s_libc_args(x, &o2)); CHECK(o1, o2);
    CHECK(F(unsigned long, narrow_short)(x, hs1.value), s_narrow_short(x, hs2.value));
    for (unsigned j = 0; j < 3; j++) CHECK(hs1.value[j], hs2.value[j]);
    CHECK(hs1.guard, 0xa55a);
    CHECK(F(int, sign_change)((unsigned long)(unsigned int)x, &us1.value), s_sign_change((unsigned int)x, &us2.value));
    CHECK(us1.value, us2.value); CHECK(us1.guard, 0xa55aa55a);
    F(void, put_bytes)(x, bs1.value); s_put_bytes(x, bs2.value);
    CHECK(memcmp(&bs1, &bs2, sizeof bs1), 0);
    CHECK(F(int, ret_narrow)((long)x, &l1), s_ret_narrow((long)x, &l2)); CHECK(l1, l2);
    CHECK(F(short, ret_short)((long)x, &l1), s_ret_short((long)x, &l2)); CHECK(l1, l2);
    CHECK(F(unsigned long, keep_store)(x, &us1.value), s_keep_store(x, &us2.value));
    CHECK(us1.value, us2.value); CHECK(us1.guard, 0xa55aa55a);
    CHECK(F(unsigned int, keep_arith)(x, 0x80000000u, &o1), (unsigned int)s_keep_arith(x, 0x80000000u, &o2));
    CHECK(o1, o2);
    CHECK(F(unsigned char, keep_cmp)(x, &o1), (unsigned char)s_keep_cmp(x, &o2)); CHECK(o1, o2);
  }
  return bad != 0;
}
"#;

const IMAGES: &[&str] = &[
    "castnarrow_gcc_O0_x86_64",
    "castnarrow_gcc_O2_x86_64",
    "castnarrow_clang_O0_x86_64",
    "castnarrow_clang_O2_x86_64",
];

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
    for &image in IMAGES {
        let path = common::fixture(image);
        for opt in ["off", "on"] {
            let mut args = vec!["decompile-all", path.as_str(), "--functions", funcs.as_str()];
            args.extend_from_slice(&["--option", "castnarrow", opt]);
            let (text, err, rc) = common::run_kuna(&args);
            assert_eq!(rc, 0, "{image} castnarrow {opt}: {err}");
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

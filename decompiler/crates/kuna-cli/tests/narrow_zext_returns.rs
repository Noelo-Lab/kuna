//! A narrow integer a callee returns zero-extended is printed unsigned where the
//! calling convention extends a return by the sign of its type, so C promotes it
//! the way the binary extends it. Typed by its own ops instead, `k * 300` or a
//! byte of unknown type read as `short` or `char`, and `u16_inc() * b >> 16`
//! sign-extended the 0x8000 the binary returns as 32768. A RETURN that block
//! duplication copies into each exit keeps the record (`h_loop` on PowerPC).
use crate::common;
use common::process;
use std::process::Command;

const SOURCE: &str = include_str!("../../kuna-analysis/tests/fixtures/zextreturn.c");

const CALLEES: &[&str] = &["u16_mul", "u8_add", "u8_hi", "u16_inc", "u16_sum", "s16_mul", "s8_add", "h_loop"];
const CALLERS: &[&str] = &["c_u16_mul", "c_u8_add", "c_u8_hi", "c_u16_inc", "c_u16_sum", "c_s16_mul", "c_s8_add"];

/// The array `h_loop` sums is static and the program is built without PIE, so
/// a 32-bit target's pointer arithmetic through `int` keeps its address.
const MAIN: &str = r#"
static short hl_arr[5];
static const int V[] = {0, 1, -1, 3, 99, 100, 101, 109, 110, 127, 128, 155, 156, 218, 219, 227, 228,
                        0x1b4e, 0x1b4f, 0x7fff, 0x8000, 0xffff, 0x10000, 0x7fffffff, (int)0x80000000, -7,
                        0x12345678, (int)0xdeadbeef};
#define CHECK(got, want) do { long long g = (got), w = (want); if (g != w) { \
    printf("%s v=%#x: %llx, want %llx\n", #got, v, g, w); bad++; } } while (0)
int main(void) {
  int bad = 0;
  for (unsigned i = 0; i < sizeof V / sizeof V[0]; i++) {
    int v = V[i];
    unsigned short x = v, y = v;
    hl_arr[0] = v, hl_arr[1] = v >> 3, hl_arr[2] = v * 7, hl_arr[3] = 0x4000, hl_arr[4] = v >> 16;
    CHECK(u16_mul(v), s_u16_mul(v));
    CHECK(u8_add(v), s_u8_add(v));
    CHECK(u8_hi(v), s_u8_hi(v));
    CHECK(u16_inc((void *)&x), s_u16_inc(&y));
    CHECK(x, y);
    CHECK(u16_sum(v & 0xff), s_u16_sum(v & 0xff));
    CHECK(s16_mul(v), s_s16_mul(v));
    CHECK(s8_add(v), s_s8_add(v));
    CHECK(h_loop(i % 6, hl_arr), s_h_loop(i % 6, hl_arr));
#ifdef CALLERS
    CHECK(c_u16_mul(v), s_c_u16_mul(v));
    CHECK(c_u8_add(v), s_c_u8_add(v));
    CHECK(c_u8_hi(v), s_c_u8_hi(v));
    CHECK(c_u16_inc((void *)&x, v), s_c_u16_inc(&y, v));
    CHECK(x, y);
    CHECK(c_u16_sum(v), s_c_u16_sum(v));
    CHECK(c_s16_mul(v), s_c_s16_mul(v));
    CHECK(c_s8_add(v), s_c_s8_add(v));
#endif
  }
  return bad != 0;
}
"#;

/// Each image, the options it is decompiled with, and whether its callers are
/// checked too: the loader resolves the calls of ARM objects only.
const IMAGES: &[(&str, &[&str], bool)] = &[
    ("zextreturn_arm.o", &[], true),
    ("zextreturn_arm_O0.o", &[], true),
    ("zextreturn_armv7.o", &[], true),
    ("zextreturn_thumb.o", &[], true),
    ("zextreturn_ppc.o", &[], false),
    ("zextreturn_rv32.o", &[], false),
    ("zextreturn_rv64.o", &[], false),
    ("zextreturn_mipsel.o", &["--option", "narrowext", "compiler"], false),
];

/// The printed function `name`, from its header comment to the next.
fn function<'a>(text: &'a str, name: &str) -> &'a str {
    let start = text.find(&format!("// Function: {name} @")).unwrap_or_else(|| panic!("no {name}:\n{text}"));
    let rest = &text[start + 1..];
    &text[start..start + 1 + rest.find("// Function:").unwrap_or(rest.len())]
}

/// The source with every function renamed `s_<name>`.
fn reference() -> String {
    let mut src = SOURCE.to_string();
    for name in CALLERS.iter().chain(CALLEES) {
        src = src.replace(&format!(" {name}("), &format!(" s_{name}("));
    }
    src
}

#[test]
fn a_zero_extended_narrow_return_round_trips_through_the_printed_c() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let reference = reference();
    for &(fixture, options, callers) in IMAGES {
        let path = common::fixture(fixture);
        let mut args = vec!["decompile-all", path.as_str()];
        args.extend_from_slice(options);
        let (text, err, rc) = common::run_kuna(&args);
        assert_eq!(rc, 0, "{fixture}: {err}");
        let names = if callers { [CALLEES, CALLERS].concat() } else { CALLEES.to_vec() };
        let printed: String = names.iter().map(|n| function(&text, n)).collect();
        let define = if callers { "#define CALLERS\n" } else { "" };
        let program = format!("#include <stdio.h>\n#include <stdbool.h>\n{define}{reference}\n{printed}\n{MAIN}");
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let src = common::scratch_file(&format!("zextreturn-{fixture}-{cc}{level}"), "c");
                let exe = src.with_extension("exe");
                std::fs::write(&src, &program).unwrap();
                let out = Command::new(cc)
                    .args(["-std=gnu11", "-w", "-fwrapv", "-no-pie", level, "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                    .args(common::CC_GCC15_DEMOTE)
                    .output()
                    .expect("spawn the C compiler");
                assert!(
                    out.status.success(),
                    "{cc} {level} rejected the printed C ({fixture}):\n{}\n{printed}",
                    String::from_utf8_lossy(&out.stderr)
                );
                let run = Command::new(&exe).output().expect("run the round trip");
                assert!(
                    run.status.success(),
                    "{fixture} printed and built by {cc} {level} computes a different value:\n{}\n{printed}",
                    String::from_utf8_lossy(&run.stdout)
                );
            }
        }
    }
}

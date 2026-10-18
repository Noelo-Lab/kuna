use crate::common;
use crate::common::process;
use std::process::Command;

const MAIN: &str = r#"
#include <stdio.h>
#include <stdint.h>
int main(void) {
  uint32_t xs[] = {0, 1, 0x7f, 0x80, 0xff, 0x17f, 0x7f7f, 0x8080, 0xffff, 0x2aab, 0x5555, 0x12345678, 0xfffffffe};
  unsigned long long h = 0;
  for (unsigned i = 0; i < sizeof xs / sizeof xs[0]; i++)
    for (unsigned j = 0; j < sizeof xs / sizeof xs[0]; j++) {
      uint32_t x = xs[i], y = xs[j];
      h = h * 31 + (uint32_t)larger_byte(x) + 7 * (uint32_t)halfword_pair(y) + 13 * (uint32_t)count_above(x, y)
          + 17 * (uint32_t)keeps_sign(x ^ y) + (uint32_t)keeps_width(y) + 3 * (uint32_t)returns_word(x)
          + 5 * (uint32_t)use_pick(x, y) + 11 * (uint32_t)either_half(x ^ (y << 1));
    }
  printf("%llu\n", h);
  return 0;
}
"#;

const WANT: &str = "3693803628968453501\n";

fn decompile(fixture: &str, option: &str) -> String {
    let bin = common::fixture(fixture);
    let (stdout, stderr, code) = common::run_kuna(&["decompile-all", &bin, "--option", "narrowret", option]);
    assert_eq!(code, 0, "kuna decompile-all {fixture} failed: {stderr}");
    stdout
}

/// The function bodies of a `decompile-all` listing, without its banners.
fn printed_c(listing: &str) -> String {
    listing
        .lines()
        .filter(|l| !l.starts_with("// Function: "))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The callers of an AAPCS callee that returns a `uint8_t` or a `bool` read
/// r0 at full width; with the option on, the locals holding those results are
/// declared at the callee's width, and the sign-extended and word-wide
/// controls keep theirs, as do a truth value merged with a byte truncated from
/// a word (`pick` is no `bool`) and a byte merged with a sign-extended
/// half-word.  On x86-64 the 32-bit `whole` is declared `int`, so it states
/// nothing and its caller keeps `%rax` whole.
#[test]
fn a_call_result_takes_the_width_its_callee_returns() {
    let on = decompile("narrowret_thumb_O2.o", "on");
    let off = decompile("narrowret_thumb_O2.o", "off");
    for (text, byte, flag) in [
        (&on, "unsigned char v1; // r0 | r5", "bool v1; // r0 | r6"),
        (&off, "unsigned int v1; // r0 | r5", "unsigned int v1; // r0 | r6"),
    ] {
        assert!(text.contains(byte), "{text}");
        assert!(text.contains(flag), "{text}");
    }
    assert!(on.contains("return mix(v2 || v1,a0) + 1;"), "{on}");
    for text in [&on, &off] {
        assert!(
            text.contains("  v1 = signed_byte(a0);\n  v2 = signed_byte(a0 >> 8);"),
            "{text}"
        );
        assert!(text.contains("int keeps_sign("), "{text}");
        assert!(text.contains("unsigned char pick(unsigned int a0,unsigned int a1)"), "{text}");
        assert!(text.contains("unsigned int either_half(unsigned int a0)"), "{text}");
        assert!(!text.contains("(bool)"), "{text}");
    }
    for option in ["on", "off"] {
        let text = decompile("narrowret_gcc_O2_x86_64.o", option);
        assert!(text.contains("unsigned long returns_word(int a0)"), "{text}");
    }
}

/// The printed C computes what the source does with the option on and off.
#[test]
fn a_narrowed_call_result_round_trips_through_the_printed_c() {
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the narrowret round trip requires a C compiler");
    for fixture in ["narrowret_thumb_O2.o", "narrowret_gcc_O2_x86_64.o"] {
        for option in ["on", "off"] {
            let printed = printed_c(&decompile(fixture, option));
            for cc in &compilers {
                let src = common::scratch_file("narrow-ret", "c");
                let exe = common::scratch_file("narrow-ret", "bin");
                std::fs::write(&src, format!("#include <stdbool.h>\n{printed}\n{MAIN}")).unwrap();
                let out = Command::new(cc)
                    .args(["-std=gnu11", "-w", "-O2"])
                    .args(common::CC_GCC15_DEMOTE)
                    .args(["-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                    .output()
                    .expect("spawn the C compiler");
                assert!(
                    out.status.success(),
                    "{cc} rejected the printed C ({fixture}, option {option}):\n{}\n{printed}",
                    String::from_utf8_lossy(&out.stderr)
                );
                let run = process::required_output(&mut Command::new(&exe));
                assert_eq!(
                    String::from_utf8_lossy(&run.stdout),
                    WANT,
                    "{cc} ({fixture}, option {option}):\n{printed}"
                );
            }
        }
    }
}

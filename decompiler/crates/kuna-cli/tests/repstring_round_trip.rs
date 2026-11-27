//! (kuna `repstring`) An x86 `rep movs`, `rep stos` or `repz cmpsb` prints as
//! a copy, fill or compare builtin.  Each image is printed with the option
//! off and on; the x86-64 one is compiled under gcc and clang at -O0 and -O2
//! beside a `main` that states what each function computes, so the printed
//! builtins must write the same bytes and return the same values as the loops.
use crate::common;
use common::process;
use std::process::Command;
use crate::common::process::Capped;

const FUNCS: &[&str] =
    &["fill32", "zero32", "fill8", "copy32", "copy_end", "isdash", "order", "below", "same_n", "same5", "mismatch"];
/// The function whose final pointer is read, which keeps its loop.
const LOOP: &str = "mismatch";

const PRELUDE: &str = r#"
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#define builtin_memcpy memcpy
#define builtin_memset memset
#define builtin_memcmp memcmp
static void builtin_memset32(unsigned int *d, unsigned int v, unsigned long n) { while (n--) *d++ = v; }
"#;

const MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
#define CHECK(got, want) do { long long g_ = (long long)(got), w_ = (long long)(want); if (g_ != w_) { \
    printf("%s [%u]: %llx, want %llx\n", #got, n, g_, w_); bad++; } } while (0)
static const unsigned char TEXT[][16] = {"-", "", "-x", "abcdefgh", "abcdefgx", "abcd\xffgh", "abcd\x01gh",
    "\x80", "\x7f", "abcdefg", "-\0y", "abcdexgh"};
#define TEXTS (sizeof TEXT / sizeof TEXT[0])
static int sign(int v) { return (v > 0) - (v < 0); }
int main(void) {
  int bad = 0;
  unsigned int src[40], a[40], b[40];
  for (unsigned i = 0; i < 40; i++) src[i] = i * 2654435761u + 7;
  for (unsigned n = 0; n < 34; n += 3) {
    memset(a, 0xaa, sizeof a); memset(b, 0xaa, sizeof b);
    F(void, fill32)(a + 1, n);
    for (unsigned k = 0; k < n; k++) b[1 + k] = 0x01020304;
    CHECK(memcmp(a, b, sizeof a), 0);
    F(void, zero32)(a + 3, n);
    for (unsigned k = 0; k < n; k++) b[3 + k] = 0;
    CHECK(memcmp(a, b, sizeof a), 0);
    F(void, fill8)((char *)a + 5, 0x5a + n, n * 3);
    for (unsigned k = 0; k < n * 3; k++) ((char *)b)[5 + k] = 0x5a + n;
    CHECK(memcmp(a, b, sizeof a), 0);
    F(void, copy32)(a + 2, src + 1, n);
    for (unsigned k = 0; k < n; k++) b[2 + k] = src[1 + k];
    CHECK(memcmp(a, b, sizeof a), 0);
    CHECK(F(unsigned int *, copy_end)(a + 4, src, n) - a, 4 + n);
    for (unsigned k = 0; k < n; k++) b[4 + k] = src[k];
    CHECK(memcmp(a, b, sizeof a), 0);
  }
  for (unsigned n = 0; n < TEXTS * TEXTS; n++) {
    const char *x = (const char *)TEXT[n / TEXTS], *y = (const char *)TEXT[n % TEXTS];
    unsigned first = 0;
    while (first < 8 && x[first] == y[first]) first++;
    CHECK(F(unsigned char, isdash)(x), strcmp(x, "-") == 0);
    CHECK(sign(F(int, order)(x, y)), sign(memcmp(x, y, 5)));
    CHECK(F(unsigned int, below)(x, y), memcmp(x, y, 7) < 0);
    for (unsigned long k = 0; k < 10; k++) CHECK(F(unsigned char, same_n)(x, y, k), memcmp(x, y, k) == 0);
    CHECK(F(unsigned char, same5)(x, y), memcmp(x, y, 5) == 0);
    CHECK(F(const char *, mismatch)(x, y) - x, first < 8 ? first + 1 : 8);
  }
  return bad != 0;
}
"#;

/// The printed function `name`, from its header comment to the next.
fn function<'a>(text: &'a str, name: &str) -> &'a str {
    let start = text.find(&format!("// Function: {name} @")).unwrap_or_else(|| panic!("no {name}:\n{text}"));
    let rest = &text[start + 1..];
    &text[start..start + 1 + rest.find("// Function:").unwrap_or(rest.len())]
}

#[test]
fn repeated_string_instructions_round_trip_through_the_printed_builtins() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let funcs = FUNCS.join(",");
    for (image, compile) in [("repstring_x86_64", true), ("repstring_i386", false)] {
        let path = common::fixture(image);
        for opt in ["off", "on"] {
            let args = ["decompile-all", path.as_str(), "--functions", funcs.as_str(), "--option", "repstring", opt];
            let (text, err, rc) = common::run_kuna(&args);
            assert_eq!(rc, 0, "{image} repstring {opt}: {err}");
            for name in FUNCS {
                let body = function(&text, name);
                let (builtin, looped) = (body.contains("builtin_mem"), body.contains("while ("));
                let want = opt == "on" && *name != LOOP;
                assert!(builtin == want && looped != want, "{image} {name} with repstring {opt}:\n{body}");
            }
            if !compile {
                continue;
            }
            let program = format!("{PRELUDE}\n{text}\n{MAIN}");
            for cc in &compilers {
                for level in ["-O0", "-O2"] {
                    let src = common::scratch_file(&format!("repstring-{image}-{opt}-{cc}{level}"), "c");
                    let exe = src.with_extension("exe");
                    std::fs::write(&src, &program).unwrap();
                    let out = Command::new(cc)
                        .args(["-std=gnu11", "-w", "-fwrapv", level, "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                        .args(common::CC_GCC15_DEMOTE)
                        .output()
                        .expect("spawn the C compiler");
                    assert!(
                        out.status.success(),
                        "{cc} {level} rejected the printed C ({image}, repstring {opt}):\n{}\n{text}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    let run = Command::new(&exe).capped_output().expect("run the round trip");
                    assert!(
                        run.status.success(),
                        "{image} printed with repstring {opt} and built by {cc} {level} computes a different value:\n{}\n{text}",
                        String::from_utf8_lossy(&run.stdout)
                    );
                }
            }
        }
    }
}

/// A `rep stosd` and a `rep movsd` of 32-bit code on the 64-bit model
/// (`x86:LE:64:compat32`) step the program's own 4-byte pointers, so they are
/// still a fill and a copy; the same `rep stosd` with an x86-64 address-size
/// override steps EDI, which wraps at 4 GiB, and keeps its loop.
#[test]
fn a_pointer_as_wide_as_the_programs_pointers_still_matches() {
    let cases: [(&str, &[u8], bool); 3] = [
        ("x86:LE:64:compat32:gcc", &[0xf3, 0xab, 0xc3], true),
        ("x86:LE:64:compat32:gcc", &[0xf3, 0xa5, 0xc3], true),
        ("x86:LE:64:default:gcc", &[0xf3, 0x67, 0xab, 0xc3], false),
    ];
    for (target, code, builtin) in cases {
        let path = common::scratch_file("repstring-width", "bin");
        std::fs::write(&path, code).unwrap();
        let path = path.to_str().unwrap();
        let args = ["decompile", path, "--raw-image", "--target", target, "--base", "0x1000", "--addr", "0x1000"];
        let (text, err, rc) = common::run_kuna(&args);
        assert_eq!(rc, 0, "{target} {code:02x?}: {err}");
        assert!(
            text.contains("builtin_mem") == builtin && text.contains("while (") != builtin,
            "{target} {code:02x?}:\n{text}"
        );
    }
}

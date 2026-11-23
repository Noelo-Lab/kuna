//! (kuna `strlenscan`) gcc 9's inline `repnz scasb` string length prints as
//! `strlen(s)`, taken where the scan was: before a store, a call or a free that
//! follows it, once per inlined `strcat`, and once on each side of a branch that
//! joins.  A scan for another byte and a scan whose raw count is used keep their
//! loops.  Each image is printed with the option off and on; the x86-64 ones are
//! compiled beside the fixture source (functions renamed `s_<name>`) under gcc
//! and clang at -O0 and -O2, and every printed function must compute what the
//! source computes.
use crate::common;
use common::process;
use std::process::Command;

const SOURCE: &str = include_str!("../../kuna-analysis/tests/fixtures/strlenscan.c");

const FUNCS: &[&str] = &[
    "name_len",
    "has_suffix",
    "dup_text",
    "last_char",
    "weighted_len",
    "wipe",
    "len_then_clear",
    "len_then_wipe",
    "len_then_free",
    "add_slash",
    "len_join",
    "scan_slash",
    "raw_count",
];
const SCANS: &[&str] = &["scan_slash", "raw_count"];
/// Functions whose length must not be taken after the store, call or free that
/// follows the scan.
const AFTER: &[&str] = &["len_then_clear", "len_then_wipe", "len_then_free"];

const PRELUDE: &str = "#include <stdbool.h>\n#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\n\
                       #define STRLENSCAN_NO_MAIN\n#define builtin_memcpy memcpy\n";

/// Calls the printed functions through prototype-less pointers and compares
/// each result, and each buffer they write, with the source function's.  A
/// path is extended inside a buffer whose bytes past the string are not zero,
/// so a length taken after a store reads past the first terminator.
const MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
static const char *const TEXT[] = {"", "a", "log", ".log", "notes.log", "dir/x.log", "ab/cd", "x/",
    "a much longer line of text that ends in .log", "/"};
#define CHECK(got, want) do { long long g_ = (long long)(got), w_ = (long long)(want); if (g_ != w_) { \
    printf("%s [%u]: %llx, want %llx\n", #got, i, g_, w_); bad++; } } while (0)
static void junk(char *a, char *b, const char *s) {
  memset(a, 'X', 96); memset(b, 'X', 96); a[95] = b[95] = 0;
  memcpy(a, s, strlen(s) + 1); memcpy(b, s, strlen(s) + 1);
}
int main(void) {
  int bad = 0;
  for (unsigned i = 0; i < sizeof TEXT / sizeof TEXT[0]; i++) {
    const char *s = TEXT[i], *t = TEXT[(i + 3) % (sizeof TEXT / sizeof TEXT[0])];
    char a[96], b[96];
    unsigned long o1[2], o2[2];
    CHECK(F(unsigned long, name_len)(s), s_name_len(s));
    CHECK(F(unsigned char, has_suffix)(s, ".log"), (unsigned char)s_has_suffix(s, ".log"));
    CHECK(F(unsigned char, has_suffix)(s, t), (unsigned char)s_has_suffix(s, t));
    CHECK(F(int, last_char)(s), s_last_char(s));
    CHECK(F(long, weighted_len)(s, t), s_weighted_len(s, t));
    CHECK(F(unsigned long, raw_count)(s), s_raw_count(s));
    if (strchr(s, '/'))
      CHECK(F(long, scan_slash)(s), s_scan_slash(s));
    junk(a, b, s);
    CHECK(F(unsigned long, len_then_clear)(a), s_len_then_clear(b));
    CHECK(memcmp(a, b, sizeof a), 0);
    junk(a, b, s);
    CHECK(F(unsigned long, len_then_wipe)(a), s_len_then_wipe(b));
    CHECK(memcmp(a, b, sizeof a), 0);
    CHECK(F(unsigned long, len_then_free)(s), s_len_then_free(s));
    junk(a, b, s);
    F(void, add_slash)(a, "lib");
    s_add_slash(b, "lib");
    CHECK(memcmp(a, b, sizeof a), 0);
    for (int k = 0; k < 2; k++) {
      junk(a, b, s);
      o1[0] = o1[1] = o2[0] = o2[1] = 7;
      CHECK(F(unsigned long, len_join)(a, k, &o1[0], &o1[1]), s_len_join(b, k, &o2[0], &o2[1]));
      CHECK(o1[0], o2[0]);
      CHECK(o1[1], o2[1]);
      CHECK(memcmp(a, b, sizeof a), 0);
    }
#ifdef CHECK_DUP
    char *d1 = F(char *, dup_text)(s), *d2 = s_dup_text(s);
    CHECK(strcmp(d1, d2), 0);
    free(d1);
    free(d2);
#endif
  }
  return bad != 0;
}
"#;

/// Each image, the functions it inlines a scan into, whether its printed C is
/// compiled here, and whether its printed `dup_text` returns the copy.
const IMAGES: &[(&str, &[&str], bool, bool)] = &[
    (
        "strlenscan_gcc9_O1_x86_64",
        &["name_len", "has_suffix", "dup_text", "last_char", "weighted_len", "len_then_clear", "len_then_wipe",
          "len_then_free", "add_slash", "len_join"],
        true,
        true,
    ),
    (
        "strlenscan_gcc9_Os_x86_64",
        &["name_len", "has_suffix", "dup_text", "last_char", "weighted_len", "len_then_clear", "len_then_wipe",
          "len_then_free", "len_join"],
        true,
        false,
    ),
    (
        "strlenscan_gcc9_O1_i386",
        &["name_len", "has_suffix", "dup_text", "last_char", "weighted_len", "len_then_clear", "len_then_wipe",
          "len_then_free", "add_slash", "len_join"],
        false,
        false,
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
        src = src.replace(&format!("*{name}("), &format!("*s_{name}("));
    }
    src
}

#[test]
fn an_inline_string_length_round_trips_through_the_printed_strlen() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let reference = reference();
    let funcs = FUNCS.join(",");
    for &(image, lengths, compile, dup) in IMAGES {
        let path = common::fixture(image);
        for opt in ["off", "on"] {
            let args = ["decompile-all", path.as_str(), "--functions", funcs.as_str(), "--option", "strlenscan", opt];
            let (text, err, rc) = common::run_kuna(&args);
            assert_eq!(rc, 0, "{image} strlenscan {opt}: {err}");
            for name in lengths {
                let body = function(&text, name);
                if opt == "on" {
                    assert!(body.contains("strlen(") && !body.contains("do {"), "{image} {name} is no strlen:\n{body}");
                } else {
                    assert!(!body.contains("strlen(") && body.contains("do {"), "{image} {name} lost its scan loop:\n{body}");
                }
            }
            for name in AFTER {
                let body = function(&text, name);
                assert!(!body.contains("return strlen("), "{image} {name} takes its length late:\n{body}");
            }
            if opt == "on" && lengths.contains(&"add_slash") {
                let body = function(&text, "add_slash");
                assert_eq!(body.matches("strlen(").count(), 1, "{image} add_slash takes its length twice:\n{body}");
            }
            for name in SCANS {
                let body = function(&text, name);
                assert!(!body.contains("strlen(") && body.contains("do {"), "{image} {name} lost its scan loop:\n{body}");
            }
            if !compile {
                continue;
            }
            let check_dup = if dup { "#define CHECK_DUP\n" } else { "" };
            let program = format!("{PRELUDE}{check_dup}{reference}\n{text}\n{MAIN}");
            for cc in &compilers {
                for level in ["-O0", "-O2"] {
                    let src = common::scratch_file(&format!("strlenscan-{image}-{opt}-{cc}{level}"), "c");
                    let exe = src.with_extension("exe");
                    std::fs::write(&src, &program).unwrap();
                    let out = Command::new(cc)
                        .args(["-std=gnu11", "-w", "-fwrapv", level, "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                        .args(common::CC_GCC15_DEMOTE)
                        .output()
                        .expect("spawn the C compiler");
                    assert!(
                        out.status.success(),
                        "{cc} {level} rejected the printed C ({image}, strlenscan {opt}):\n{}\n{text}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    let run = Command::new(&exe).output().expect("run the round trip");
                    assert!(
                        run.status.success(),
                        "{image} printed with strlenscan {opt} and built by {cc} {level} computes a different value:\n{}\n{text}",
                        String::from_utf8_lossy(&run.stdout)
                    );
                }
            }
        }
    }
}

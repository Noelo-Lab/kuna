//! (kuna `rettriangle`) A one-block `if` before a constant return prints as
//! `if (c) f(); return 0;` instead of the split `if (!c) return 0; f(); return 0;`.
//! Each image is printed with the option off and on, compiled beside the fixture
//! source (functions renamed `s_<name>`) under gcc and clang at -O0 and -O2, and
//! every printed function must return what the source returns and call
//! `celebrate` as often.
use crate::common;
use common::process;
use std::process::Command;

const SOURCE: &str = include_str!("../../kuna-analysis/tests/fixtures/rettriangle.c");

const FUNCS: &[&str] = &["celebrate", "finish", "check_name", "guard_first", "two_way"];

const PRELUDE: &str = "#include <stdio.h>\n#include <string.h>\n#define RETTRIANGLE_NO_MAIN\n";

/// Calls each printed function and its source twin through prototype-less
/// pointers and compares the result, the number of `celebrate` calls (the
/// source's `hits` counter, which the printed `celebrate` also bumps) and the
/// words `guard_first` stores.
const MAIN: &str = r#"
#define F(ret, f) ((ret (*)())(void (*)())f)
#define CHECK(got, want) do { long long g_ = (long long)(got), w_ = (long long)(want); if (g_ != w_) { \
    printf("%s [%u]: %llx, want %llx\n", #got, i, g_, w_); bad++; } } while (0)
#define SAME(call, twin) do { int h0 = hits; int r1 = (call); int d1 = hits - h0; \
    h0 = hits; int r2 = (twin); int d2 = hits - h0; CHECK(r1, r2); CHECK(d1, d2); } while (0)
int main(void) {
  int bad = 0;
  static const char *const NAME[] = {"go", "no", "", "gopher"};
  for (unsigned i = 0; i < 4; i++) {
    int a = i & 1, b = (i >> 1) & 1;
    int o1[2] = {7, 7}, o2[2] = {7, 7};
    SAME(F(int, finish)(a), s_finish(a));
    SAME(F(int, check_name)(NAME[i]), s_check_name(NAME[i]));
    SAME(F(int, guard_first)(a, o1), s_guard_first(a, o2));
    CHECK(o1[0], o2[0]);
    CHECK(o1[1], o2[1]);
    SAME(F(int, two_way)(a, b), s_two_way(a, b));
  }
  return bad != 0;
}
"#;

/// Each image and the function whose join it prints as a one-block `if` when
/// the option is on.
const IMAGES: &[(&str, &[&str])] = &[
    ("rettriangle_gcc_O0_x86_64", &["finish", "check_name", "guard_first"]),
    ("rettriangle_gcc_O2_x86_64", &["check_name", "guard_first"]),
    ("rettriangle_clang_O0_x86_64", &["finish", "check_name", "guard_first"]),
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
fn a_one_block_if_before_a_constant_return_round_trips_through_the_printed_c() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let reference = reference();
    let funcs = FUNCS.join(",");
    for &(image, triangles) in IMAGES {
        let path = common::fixture(image);
        for opt in ["off", "on"] {
            let args = ["decompile-all", path.as_str(), "--functions", funcs.as_str(), "--option", "rettriangle", opt];
            let (text, err, rc) = common::run_kuna(&args);
            assert_eq!(rc, 0, "{image} rettriangle {opt}: {err}");
            for name in triangles {
                let body = function(&text, name);
                let returns = body.matches("return ").count();
                if opt == "on" {
                    assert_eq!(returns, 1, "{image} {name} still splits its return:\n{body}");
                } else {
                    assert_eq!(returns, 2, "{image} {name} lost the split it is compared against:\n{body}");
                }
            }
            let program = format!("{PRELUDE}{reference}\n{text}\n{MAIN}");
            for cc in &compilers {
                for level in ["-O0", "-O2"] {
                    let src = common::scratch_file(&format!("rettriangle-{image}-{opt}-{cc}{level}"), "c");
                    let exe = src.with_extension("exe");
                    std::fs::write(&src, &program).unwrap();
                    let out = Command::new(cc)
                        .args(["-std=gnu11", "-w", "-fwrapv", level, "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                        .args(common::CC_GCC15_DEMOTE)
                        .output()
                        .expect("spawn the C compiler");
                    assert!(
                        out.status.success(),
                        "{cc} {level} rejected the printed C ({image}, rettriangle {opt}):\n{}\n{text}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    let run = Command::new(&exe).output().expect("run the round trip");
                    assert!(
                        run.status.success(),
                        "{image} printed with rettriangle {opt} and built by {cc} {level} computes a different value:\n{}\n{text}",
                        String::from_utf8_lossy(&run.stdout)
                    );
                }
            }
        }
    }
}

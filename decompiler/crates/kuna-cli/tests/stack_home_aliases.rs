//! Reused Windows x64 home and argument slots keep the keys a callee reads (#1121).
use crate::common;

use serde_json::Value;
use std::process::Command;
use std::time::Duration;

const FIXTURE: &str = "stackhome_alias_x86_64";

fn decompile() -> String {
    let fixture = common::fixture(FIXTURE);
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command
        .args(["decompile", &fixture, "reuse", "--json"])
        .args([
            "--target",
            "x86:LE:64:default:windows",
            "--mode",
            "reliable",
        ])
        .args([
            "--option",
            "structdefs",
            "on",
            "--option",
            "stackviews",
            "on",
        ])
        .args(["--assert-strict", "--assert"])
        .arg(format!("@{fixture}.kuna"))
        .arg("--sleighpath")
        .arg(common::repo_root().join("specs"));
    let output = common::process::output_with_timeout(
        &mut command,
        Duration::from_secs(15),
        Duration::from_millis(25),
    )
    .expect("bounded home-slot decompilation");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document: Value = serde_json::from_slice(&output.stdout).unwrap();
    let function = &document["functions"][0];
    assert!(function["error"].is_null(), "{function}");
    function["code"].as_str().unwrap().to_owned()
}

#[test]
fn reused_home_and_argument_slots_keep_call_keys() {
    common::process::required_output(&mut Command::new(common::fixture(FIXTURE)));
    let code = decompile();
    let source = common::scratch_file("stack-home-aliases", "c");
    std::fs::write(
        &source,
        format!(
            "#include <stdbool.h>\n#include <stdint.h>\n\
             typedef int8_t int1; typedef uint8_t uint1;\n\
             typedef int16_t int2; typedef uint16_t uint2;\n\
             typedef int32_t int4; typedef uint32_t uint4;\n\
             typedef int64_t int8; typedef uint64_t uint8;\n\
             typedef uint8_t undefined1;\n\
             struct Context {{ int4 first; int4 count; int4 log[6]; }};\n\
             __attribute__((noinline)) int4 observe(void *context, const int4 *key) {{\n\
                 struct Context *c = context;\n\
                 if (c->count < 6) c->log[c->count] = *key;\n\
                 return c->count++, *key;\n\
             }}\n\
             {code}\n\
             static int differs(bool flag, int4 count) {{\n\
                 static const int4 keys[] = {{ 12, 10, 11 }};\n\
                 struct Context c = {{ 0 }};\n\
                 reuse(&c, 2, 3, 4, flag);\n\
                 if (c.first != 5 || c.count != count) return 1;\n\
                 for (int4 i = 0; i < count; ++i) if (c.log[i] != keys[i]) return 1;\n\
                 return 0;\n\
             }}\n\
             int main(void) {{ return differs(false, 3) || differs(true, 1); }}\n"
        ),
    )
    .unwrap();
    for compiler in ["gcc", "clang"] {
        if common::process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        for level in ["-O0", "-O2"] {
            let binary = source.with_extension(format!("{compiler}{}", &level[1..]));
            common::process::required_output(
                Command::new(compiler)
                    .args(["-std=c11", level, "-fstrict-aliasing"])
                    .arg(&source)
                    .arg("-o")
                    .arg(&binary),
            );
            let ran = Command::new(&binary).output().unwrap();
            assert!(
                ran.status.success(),
                "{compiler} {level} lost a key store:\n{code}"
            );
        }
    }
}
